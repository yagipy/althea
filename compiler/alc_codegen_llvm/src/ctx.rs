use crate::{CodegenLLVM, BIND, LISTEN, PRINTF, SOCKET};
use alc_ast_lowering::{ir, ty};
use alc_diagnostic::{Diagnostic, Label, Result};
use inkwell::{
    values::{ArrayValue, BasicValue, BasicValueEnum, FunctionValue, IntValue, VectorValue},
    AddressSpace,
    IntPredicate,
};
use std::{collections::HashMap, ops::Deref};

macro_rules! local {
    ( $idx:expr ) => {{
        use alc_ast_lowering::idx::Idx;
        &format!("alc_{}", $idx.index())
    }};
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum MatchCase<'ctx> {
    Wild,
    Record,
    StringLiteral(VectorValue<'ctx>),
    ArrayLiteral(ArrayValue<'ctx>),
    Literal(IntValue<'ctx>),
    Variant(ty::Ty, IntValue<'ctx>),
}

pub struct CodegenLLVMCtx<'gen, 'ctx> {
    sess: &'gen CodegenLLVM<'gen, 'ctx>,
    ir: &'gen ir::Def,
    llvm: FunctionValue<'ctx>,
    bindings: HashMap<ir::LocalIdx, BasicValueEnum<'ctx>>,
}

impl<'gen, 'ctx> CodegenLLVMCtx<'gen, 'ctx> {
    fn bind(&mut self, idx: ir::LocalIdx, value: BasicValueEnum<'ctx>) {
        self.bindings.insert(idx, value);
    }

    fn lookup(&self, idx: ir::LocalIdx) -> Result<BasicValueEnum<'ctx>> {
        if let Some(value) = self.bindings.get(&idx) {
            Ok(*value)
        } else {
            Err(Box::from(Diagnostic::new_bug(
                "reference to unbound local index",
                Label::new(
                    self.file_id,
                    idx.span(),
                    &format!("'{:?}' not bound in this scope", idx),
                ),
            )))
        }
    }

    fn compile_unop(
        &self,
        idx: ir::LocalIdx,
        kind: ir::UnopKind,
        operand: ir::LocalIdx,
    ) -> Result<BasicValueEnum<'ctx>> {
        let operand = self.lookup(operand)?.into_int_value();
        Ok(match kind {
            ir::UnopKind::Not => self.builder.build_not(operand, local!(idx)).into(),
        })
    }

    fn compile_int_predicate(&self, kind: ir::BinopKind) -> Option<IntPredicate> {
        Some(match kind {
            ir::BinopKind::Less => IntPredicate::ULT,
            ir::BinopKind::Leq => IntPredicate::ULE,
            ir::BinopKind::Greater => IntPredicate::UGT,
            ir::BinopKind::Geq => IntPredicate::UGE,
            ir::BinopKind::Eq => IntPredicate::EQ,
            ir::BinopKind::Neq => IntPredicate::NE,
            _ => return None,
        })
    }

    fn compile_binop(
        &self,
        idx: ir::LocalIdx,
        kind: ir::BinopKind,
        left: ir::LocalIdx,
        right: ir::LocalIdx,
    ) -> Result<BasicValueEnum<'ctx>> {
        let left = self.lookup(left)?.into_int_value();
        let right = self.lookup(right)?.into_int_value();
        Ok(match kind {
            ir::BinopKind::Plus => self.builder.build_int_add(left, right, local!(idx)).into(),
            ir::BinopKind::Minus => self.builder.build_int_sub(left, right, local!(idx)).into(),
            ir::BinopKind::Mul => self.builder.build_int_mul(left, right, local!(idx)).into(),
            ir::BinopKind::Div => self
                .builder
                .build_int_unsigned_div(left, right, local!(idx))
                .into(),
            ir::BinopKind::And => self.builder.build_and(left, right, local!(idx)).into(),
            ir::BinopKind::Or => self.builder.build_or(left, right, local!(idx)).into(),
            ir::BinopKind::Xor => self.builder.build_xor(left, right, local!(idx)).into(),
            ir::BinopKind::LShift => self.builder.build_left_shift(left, right, local!(idx)).into(),
            ir::BinopKind::RShift => self
                .builder
                .build_right_shift(left, right, false, local!(idx))
                .into(),
            comparison => {
                let comparison = self.builder.build_int_compare(
                    self.compile_int_predicate(comparison).unwrap(),
                    left,
                    right,
                    local!(idx),
                );
                self.builder
                    .build_int_z_extend::<IntValue>(comparison, self.context.i64_type(), "cast_tmp")
                    .into()
            }
        })
    }

    fn compile_expr(&mut self, expr: &ir::Expr) -> Result<BasicValueEnum<'ctx>> {
        match &expr.kind {
            ir::ExprKind::I8Literal(literal) => Ok(self.compile_i8_literal(*literal).into()),
            ir::ExprKind::I16Literal(literal) => Ok(self.compile_i16_literal(*literal).into()),
            ir::ExprKind::I32Literal(literal) => Ok(self.compile_i32_literal(*literal).into()),
            ir::ExprKind::U64Literal(literal) => Ok(self.compile_i64_literal(*literal).into()),
            ir::ExprKind::ArrayLiteral { element_ty, elements } => {
                Ok(self.compile_array_literal(*element_ty, elements.to_vec()).into())
            }
            ir::ExprKind::StringLiteral(literal) => Ok(self.compile_string_literal(literal).into()),
            ir::ExprKind::Var(local_idx, _) => self.lookup(*local_idx),
            ir::ExprKind::Unop { kind, operand } => self.compile_unop(expr.local_idx, *kind, *operand),
            ir::ExprKind::Binop { kind, left, right } => {
                self.compile_binop(expr.local_idx, *kind, *left, *right)
            }
            ir::ExprKind::Call { target, args } => {
                let target_fn = self.lookup_def(*target, expr.span)?;
                let mut compiled_args = Vec::with_capacity(args.len());
                for arg in args.values() {
                    compiled_args.push(self.lookup(*arg)?.into());
                }
                self.builder
                    .build_call(target_fn, compiled_args.as_slice(), local!(expr.local_idx))
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })
            }
            ir::ExprKind::Variant {
                ty,
                discriminant,
                body,
            } => {
                let variant = self.build_alloc(*ty, local!(expr.local_idx), expr.span)?;
                self.write_enum_discriminant(variant, *ty, *discriminant);
                self.write_enum_body(variant, *ty, *discriminant, self.lookup(*body)?);
                // TODO: GC
                Ok(variant.into())
            }
            ir::ExprKind::Record { ty, fields } => {
                let record = self.build_alloc(*ty, local!(expr.local_idx), expr.span)?;
                for (field_idx, local_idx) in fields.iter() {
                    self.write_struct_field(record, *ty, field_idx, self.lookup(*local_idx)?);
                }
                // TODO: GC
                Ok(record.into())
            }
            ir::ExprKind::Socket { domain, ty, protocol } => {
                let domain = self.lookup(*domain)?.into_int_value();
                let ty = self.lookup(*ty)?.into_int_value();
                let protocol = self.lookup(*protocol)?.into_int_value();
                let socket = self
                    .builder
                    .build_call(
                        self.module.get_function(SOCKET).unwrap(),
                        &[domain.into(), ty.into(), protocol.into()],
                        "socket",
                    )
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })?;
                Ok(socket)
            }
            ir::ExprKind::Bind {
                socket_file_descriptor,
                address,
                address_length,
            } => {
                let socket_file_descriptor = self.lookup(*socket_file_descriptor)?.into_int_value();
                let address = self.lookup(*address)?.into_pointer_value();
                let address_length = self.lookup(*address_length)?.into_int_value();
                let casted_address = self.builder.build_bitcast(
                    address,
                    self.context
                        .struct_type(
                            &[
                                self.context.i16_type().into(),
                                self.context.i8_type().array_type(14).into(),
                            ],
                            false,
                        )
                        .ptr_type(AddressSpace::Generic),
                    "cast_tmp",
                );
                let bind = self
                    .builder
                    .build_call(
                        self.module.get_function(BIND).unwrap(),
                        &[
                            socket_file_descriptor.into(),
                            casted_address.into(),
                            address_length.into(),
                        ],
                        "bind",
                    )
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })?;
                Ok(bind)
            }
            ir::ExprKind::Listen {
                socket_file_descriptor,
                backlog,
            } => {
                let socket_file_descriptor = self.lookup(*socket_file_descriptor)?.into_int_value();
                let backlog = self.lookup(*backlog)?.into_int_value();
                let listen = self
                    .builder
                    .build_call(
                        self.module.get_function(LISTEN).unwrap(),
                        &[socket_file_descriptor.into(), backlog.into()],
                        "listen",
                    )
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })?;
                Ok(listen)
            }
        }
    }

    fn compile_instruction(&mut self, instruction: &ir::Instruction) -> Result<()> {
        match &instruction.kind {
            ir::InstructionKind::Let { binding, expr, .. } => {
                let compiled_expr = self.compile_expr(expr)?;
                self.bind(*binding, compiled_expr);
            }
            ir::InstructionKind::Println { idx } => {
                let value = self.lookup(*idx).unwrap();
                let const_ref = self.builder.build_alloca(value.get_type(), "printf_tmp");
                self.builder.build_store(const_ref, value);
                let ptr;
                unsafe {
                    ptr = self.sess.gep(const_ref, 0, "printf_tmp");
                }
                self.builder.build_call(
                    self.module.get_function(PRINTF).unwrap(),
                    &[ptr.as_basic_value_enum().into()],
                    "printf",
                );
            }
            ir::InstructionKind::Mark(idx, ty) => {
                // %a.mark := 1;
                self.write_mark(self.lookup(*idx)?.into_pointer_value(), *ty, true);
            }
            ir::InstructionKind::Unmark(idx, ty) => {
                // %a.mark := 0;
                self.write_mark(self.lookup(*idx)?.into_pointer_value(), *ty, false);
            }
            ir::InstructionKind::Free(idx, ty) => {
                // if marked(%a) {} else { @alc_free(%a); }
                let ptr = self.lookup(*idx)?.into_pointer_value();
                let free_block = self.context.append_basic_block(self.llvm, "free");
                let merge_block = self.context.append_basic_block(self.llvm, "merge");
                self.builder.build_conditional_branch(
                    self.read_mark(ptr, *ty).into_int_value(),
                    merge_block,
                    free_block,
                );
                self.builder.position_at_end(free_block);
                self.build_free(ptr);
                self.builder.build_unconditional_branch(merge_block);
                self.builder.position_at_end(merge_block);
            }
            ir::InstructionKind::RTReset => {
                // @alc_reset();
                self.build_reset();
            }
        }
        Ok(())
    }

    fn compile_block(&mut self, block: &ir::Block) -> Result<()> {
        for instruction in block.instructions.iter() {
            self.compile_instruction(instruction)?;
        }
        self.compile_terminator(&block.terminator)
    }

    fn compile_pattern(
        &mut self,
        source: BasicValueEnum<'ctx>,
        pattern: &ir::PatternKind,
    ) -> MatchCase<'ctx> {
        match pattern {
            ir::PatternKind::I8Literal(literal) => MatchCase::Literal(self.compile_i8_literal(*literal)),
            ir::PatternKind::I16Literal(literal) => MatchCase::Literal(self.compile_i16_literal(*literal)),
            ir::PatternKind::I32Literal(literal) => MatchCase::Literal(self.compile_i32_literal(*literal)),
            ir::PatternKind::U64Literal(literal) => MatchCase::Literal(self.compile_i64_literal(*literal)),
            ir::PatternKind::ArrayLiteral { element_ty, elements } => {
                MatchCase::ArrayLiteral(self.compile_array_literal(*element_ty, elements.to_owned()))
            }
            ir::PatternKind::StringLiteral(literal) => {
                MatchCase::StringLiteral(self.compile_string_literal(literal))
            }
            ir::PatternKind::Ident(binding) => {
                self.bind(*binding, source);
                MatchCase::Wild
            }
            ir::PatternKind::Variant {
                ty,
                discriminant,
                binding,
            } => {
                self.bind(
                    *binding,
                    self.read_enum_body(source.into_pointer_value(), *ty, *discriminant),
                );
                MatchCase::Variant(*ty, self.compile_variant_idx(*discriminant))
            }
            ir::PatternKind::Record { fields, ty } => {
                let source = source.into_pointer_value();
                for (field_idx, binding) in fields.iter() {
                    self.bind(*binding, self.read_struct_field(source, *ty, field_idx));
                }
                MatchCase::Record
            }
        }
    }

    fn compile_terminator(&mut self, terminator: &ir::Terminator) -> Result<()> {
        match terminator {
            ir::Terminator::Return(local_idx) => {
                self.builder.build_return(Some(&self.lookup(*local_idx)?));
            }
            ir::Terminator::Match { source, arms } => {
                let source = self.lookup(*source)?;
                let origin = self.builder.get_insert_block().unwrap();
                let mut source_ty = None;
                let mut else_block = None;
                let mut cases = vec![];
                for (i, arm) in arms.iter().enumerate() {
                    let block = if let ir::PatternKind::Record { .. } = &arm.pattern {
                        origin
                    } else {
                        self.context.append_basic_block(self.llvm, &format!("arm_{}", i))
                    };
                    self.builder.position_at_end(block);
                    match self.compile_pattern(source, &arm.pattern) {
                        MatchCase::Wild => {
                            else_block = Some(block);
                            self.compile_block(&arm.target)?;
                            break;
                        }
                        MatchCase::Record => {
                            self.compile_block(&arm.target)?;
                            return Ok(());
                        }
                        MatchCase::Literal(case) => {
                            cases.push((case, block));
                            self.compile_block(&arm.target)?;
                        }
                        MatchCase::StringLiteral(_) => {
                            // TODO: support
                            // cases.push((case, block));
                            self.compile_block(&arm.target)?;
                        }
                        MatchCase::ArrayLiteral(_) => {
                            // TODO: support
                            // cases.push((case, block));
                            self.compile_block(&arm.target)?;
                        }
                        MatchCase::Variant(ty, case) => {
                            source_ty = Some(ty);
                            cases.push((case, block));
                            self.compile_block(&arm.target)?;
                        }
                    }
                }
                let else_block = match else_block {
                    Some(block) => block,
                    _ => {
                        let block = self.context.append_basic_block(self.llvm, "unreachable_else");
                        self.builder.position_at_end(block);
                        self.builder.build_unreachable();
                        block
                    }
                };
                self.builder.position_at_end(origin);
                let source = if let Some(ty) = source_ty {
                    self.read_enum_discriminant(source.into_pointer_value(), ty)?
                } else {
                    source
                }
                .into_int_value();
                self.builder.build_switch(source, else_block, cases.as_slice());
            }
        }
        Ok(())
    }

    fn compile_entry(&mut self, entry: &ir::Entry) -> Result<()> {
        let entry_block = self.context.append_basic_block(self.llvm, "entry");
        self.builder.position_at_end(entry_block);
        for (param_idx, binding) in entry.param_bindings.iter() {
            self.bind(*binding, self.read_param(self.llvm, param_idx));
        }
        self.compile_block(&entry.body)
    }

    fn compile(mut self) -> Result<FunctionValue<'ctx>> {
        self.compile_entry(&self.ir.entry)?;
        Ok(self.llvm)
    }

    pub(super) fn compile_def(
        sess: &'gen CodegenLLVM<'gen, 'ctx>,
        def: &'gen ir::Def,
    ) -> Result<FunctionValue<'ctx>> {
        let ctx = CodegenLLVMCtx {
            sess,
            ir: def,
            llvm: sess.lookup_def(def.def_idx, def.span)?,
            bindings: HashMap::new(),
        };
        ctx.compile()
    }
}

impl<'gen, 'ctx> Deref for CodegenLLVMCtx<'gen, 'ctx> {
    type Target = CodegenLLVM<'gen, 'ctx>;

    fn deref(&self) -> &Self::Target {
        self.sess
    }
}
