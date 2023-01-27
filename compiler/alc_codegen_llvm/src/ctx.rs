use crate::{CodegenLLVM, ACCEPT, BIND, CLOSE, HTONS, LISTEN, PRINTF, RECV, SEND, SNPRINTF, SOCKET, STRLEN};
use alc_ast_lowering::{ir, ty};
use alc_command_option::Gc;
use alc_diagnostic::{Diagnostic, Label, Result};
use inkwell::{
    types::BasicType,
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
                    format!("'{:?}' not bound in this scope", idx),
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
                    .build_int_z_extend::<IntValue>(comparison, self.context.i32_type(), "cast_tmp")
                    .into()
            }
        })
    }

    fn compile_expr(&mut self, expr: &ir::Expr) -> Result<BasicValueEnum<'ctx>> {
        match &expr.kind {
            ir::ExprKind::I8Literal(literal) => Ok(self.compile_i8_literal(*literal).into()),
            ir::ExprKind::I16Literal(literal) => Ok(self.compile_i16_literal(*literal).into()),
            ir::ExprKind::I32Literal(literal) => Ok(self.compile_i32_literal(*literal).into()),
            ir::ExprKind::I64Literal(literal) => Ok(self.compile_i64_literal(*literal).into()),
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
            ir::ExprKind::Accept {
                socket_file_descriptor,
            } => {
                let socket_file_descriptor = self.lookup(*socket_file_descriptor)?.into_int_value();
                let accept = self
                    .builder
                    .build_call(
                        self.module.get_function(ACCEPT).unwrap(),
                        &[
                            socket_file_descriptor.into(),
                            self.context
                                .struct_type(
                                    &[
                                        self.context.i16_type().as_basic_type_enum(),
                                        self.context.i8_type().array_type(14).as_basic_type_enum(),
                                    ],
                                    false,
                                )
                                .ptr_type(AddressSpace::Generic)
                                .const_zero()
                                .into(),
                            self.context
                                .i32_type()
                                .ptr_type(AddressSpace::Generic)
                                .const_zero()
                                .into(),
                        ],
                        "accept",
                    )
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })?;
                Ok(accept)
            }
            ir::ExprKind::Recv {
                socket_file_descriptor,
                buffer,
                buffer_length,
                flags,
            } => {
                let socket_file_descriptor = self.lookup(*socket_file_descriptor)?.into_int_value();
                let buffer = self.lookup(*buffer)?.into_array_value();
                let buffer_length = self.lookup(*buffer_length)?.into_int_value();
                let flags = self.lookup(*flags)?.into_int_value();
                let allocated_buffer = self.builder.build_alloca(buffer.get_type(), "allocated_buffer");
                let buffer_ptr = unsafe { self.gep(allocated_buffer, 0, "buffer_ptr") };
                let recv = self
                    .builder
                    .build_call(
                        self.module.get_function(RECV).unwrap(),
                        &[
                            socket_file_descriptor.into(),
                            buffer_ptr.into(),
                            buffer_length.into(),
                            flags.into(),
                        ],
                        "recv",
                    )
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })?;
                Ok(recv)
            }
            ir::ExprKind::Send {
                socket_file_descriptor,
                buffer,
                buffer_length,
                content,
                flags,
            } => {
                let socket_file_descriptor = self.lookup(*socket_file_descriptor)?.into_int_value();
                let buffer = self.lookup(*buffer)?.into_array_value();
                let buffer_length = self.lookup(*buffer_length)?.into_int_value();
                let content = self.lookup(*content)?.into_vector_value();
                let flags = self.lookup(*flags)?.into_int_value();

                let allocated_buffer = self.builder.build_alloca(buffer.get_type(), "allocated_buffer");
                let buffer_ptr = unsafe { self.gep(allocated_buffer, 0, "buffer_ptr") };
                let allocated_content = self.builder.build_alloca(content.get_type(), "allocated_content");
                self.builder.build_store(allocated_content, content);
                let content_ptr = unsafe { self.gep(allocated_content, 0, "content_ptr") };

                self.builder.build_call(
                    self.module.get_function(SNPRINTF).unwrap(),
                    &[buffer_ptr.into(), buffer_length.into(), content_ptr.into()],
                    "snprintf",
                );
                let buffer_ptr_with_content = unsafe { self.gep(allocated_buffer, 0, "buffer_ptr") };
                let content_length = self
                    .builder
                    .build_call(
                        self.module.get_function(STRLEN).unwrap(),
                        &[buffer_ptr_with_content.into()],
                        "content_length",
                    )
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })?;
                let send = self
                    .builder
                    .build_call(
                        self.module.get_function(SEND).unwrap(),
                        &[
                            socket_file_descriptor.into(),
                            buffer_ptr_with_content.into(),
                            content_length.into(),
                            flags.into(),
                        ],
                        "send",
                    )
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })?;
                Ok(send)
            }
            ir::ExprKind::Close {
                socket_file_descriptor,
            } => {
                let socket_file_descriptor = self.lookup(*socket_file_descriptor)?.into_int_value();
                let close = self
                    .builder
                    .build_call(
                        self.module.get_function(CLOSE).unwrap(),
                        &[socket_file_descriptor.into()],
                        "close",
                    )
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })?;
                Ok(close)
            }
            ir::ExprKind::ListenAndServe {
                domain,
                ty,
                protocol,
                address,
                address_length,
                backlog,
                recv_buffer,
                recv_buffer_length,
                recv_flags,
                send_buffer,
                send_buffer_length,
                send_flags,
                format_string,
                http_header,
                call_handler,
            } => {
                let domain = self.lookup(*domain)?.into_int_value();
                let ty = self.lookup(*ty)?.into_int_value();
                let protocol = self.lookup(*protocol)?.into_int_value();
                let socket_file_descriptor = self
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
                    })?
                    .into_int_value();
                let address = self.lookup(*address)?.into_pointer_value();
                let port_ptr = unsafe { self.gep(address, 1, "port") };
                let port = self.builder.build_load(port_ptr, "port").into_int_value();
                let port = self
                    .builder
                    .build_call(self.module.get_function(HTONS).unwrap(), &[port.into()], "port")
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })?
                    .into_int_value();
                self.builder.build_store(port_ptr, port);
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
                let _bind = self
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
                let backlog = self.lookup(*backlog)?.into_int_value();
                let _listen = self
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
                let recv_buffer = self.lookup(*recv_buffer)?.into_array_value();
                let recv_buffer_length = self.lookup(*recv_buffer_length)?.into_int_value();
                let recv_flags = self.lookup(*recv_flags)?.into_int_value();
                let loop_block = self.context.append_basic_block(self.llvm, "loop");
                let allocated_recv_buffer = self
                    .builder
                    .build_alloca(recv_buffer.get_type(), "allocated_buffer");
                let recv_buffer_ptr = unsafe { self.gep(allocated_recv_buffer, 0, "buffer_ptr") };
                let send_content_ptr = self.lookup(*call_handler)?.into_pointer_value();
                let send_buffer = self.lookup(*send_buffer)?.into_array_value();
                let send_buffer_length = self.lookup(*send_buffer_length)?.into_int_value();
                let http_header = self.lookup(*http_header)?.into_vector_value();
                let format_string = self.lookup(*format_string)?.into_vector_value();
                let send_flags = self.lookup(*send_flags)?.into_int_value();
                let allocated_send_buffer = self
                    .builder
                    .build_alloca(send_buffer.get_type(), "allocated_buffer");
                let allocated_http_header = self
                    .builder
                    .build_alloca(http_header.get_type(), "allocated_http_header");
                self.builder.build_store(allocated_http_header, http_header);
                let allocated_format_string = self
                    .builder
                    .build_alloca(format_string.get_type(), "allocated_http_header");
                self.builder.build_store(allocated_format_string, format_string);
                let send_buffer_ptr = unsafe { self.gep(allocated_send_buffer, 0, "buffer_ptr") };
                let http_header_ptr = unsafe { self.gep(allocated_http_header, 0, "content_ptr") };
                let format_string_ptr = unsafe { self.gep(allocated_format_string, 0, "content_ptr") };
                self.builder.build_call(
                    self.module.get_function(SNPRINTF).unwrap(),
                    &[
                        send_buffer_ptr.into(),
                        send_buffer_length.into(),
                        format_string_ptr.into(),
                        http_header_ptr.into(),
                        send_content_ptr.into(),
                    ],
                    "snprintf",
                );
                let send_buffer_ptr_with_content =
                    unsafe { self.gep(allocated_send_buffer, 0, "buffer_ptr") };
                let send_content_length = self
                    .builder
                    .build_call(
                        self.module.get_function(STRLEN).unwrap(),
                        &[send_buffer_ptr_with_content.into()],
                        "content_length",
                    )
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })?;
                self.builder.build_unconditional_branch(loop_block);
                self.builder.position_at_end(loop_block);
                let accept_file_descriptor = self
                    .builder
                    .build_call(
                        self.module.get_function(ACCEPT).unwrap(),
                        &[
                            socket_file_descriptor.into(),
                            self.context
                                .struct_type(
                                    &[
                                        self.context.i16_type().as_basic_type_enum(),
                                        self.context.i8_type().array_type(14).as_basic_type_enum(),
                                    ],
                                    false,
                                )
                                .ptr_type(AddressSpace::Generic)
                                .const_zero()
                                .into(),
                            self.context
                                .i32_type()
                                .ptr_type(AddressSpace::Generic)
                                .const_zero()
                                .into(),
                        ],
                        "accept",
                    )
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })?
                    .into_int_value();
                let _recv = self
                    .builder
                    .build_call(
                        self.module.get_function(RECV).unwrap(),
                        &[
                            accept_file_descriptor.into(),
                            recv_buffer_ptr.into(),
                            recv_buffer_length.into(),
                            recv_flags.into(),
                        ],
                        "recv",
                    )
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })?;
                let _send = self
                    .builder
                    .build_call(
                        self.module.get_function(SEND).unwrap(),
                        &[
                            accept_file_descriptor.into(),
                            send_buffer_ptr_with_content.into(),
                            send_content_length.into(),
                            send_flags.into(),
                        ],
                        "send",
                    )
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })?;
                let _accept_close = self
                    .builder
                    .build_call(
                        self.module.get_function(CLOSE).unwrap(),
                        &[accept_file_descriptor.into()],
                        "close",
                    )
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })?;
                self.builder.build_unconditional_branch(loop_block);
                let end_block = self.context.append_basic_block(self.llvm, "end");
                self.builder.position_at_end(end_block);
                let socket_close = self
                    .builder
                    .build_call(
                        self.module.get_function(CLOSE).unwrap(),
                        &[socket_file_descriptor.into()],
                        "close",
                    )
                    .try_as_basic_value()
                    .left()
                    .ok_or_else(|| {
                        Box::from(Diagnostic::new_bug(
                            "attempted to return non-basic value from function call",
                            Label::new(self.file_id, expr.span, "this call returns a non-basic value"),
                        ))
                    })?;
                Ok(socket_close)
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
                let value = self.lookup(*idx)?.into_vector_value();
                let const_ref = self.builder.build_alloca(value.get_type(), "printf_tmp");
                self.builder.build_store(const_ref, value);
                let ptr = unsafe { self.sess.gep(const_ref, 0, "printf_tmp") };
                self.builder.build_call(
                    self.module.get_function(PRINTF).unwrap(),
                    &[ptr.as_basic_value_enum().into()],
                    "printf",
                );
            }
            ir::InstructionKind::Mark(idx, ty) => {
                self.write_mark(self.lookup(*idx)?.into_pointer_value(), *ty, true);
            }
            ir::InstructionKind::Unmark(idx, ty) => {
                self.write_mark(self.lookup(*idx)?.into_pointer_value(), *ty, false);
            }
            ir::InstructionKind::Free(idx, _) => match self.command_options.gc {
                Gc::OwnRc => {
                    let ptr = self.lookup(*idx)?.into_pointer_value();
                    self.build_free(ptr);
                }
                Gc::None => {}
            },
            ir::InstructionKind::IncrementRc(idx, ty) => {
                let ptr = self.lookup(*idx)?.into_pointer_value();
                let rc_ptr = self.mark_ptr(ptr, *ty);
                let current_rc = self.builder.build_load(rc_ptr, "rc").into_int_value();
                let new_rc = self.builder.build_int_add(
                    current_rc,
                    self.context.i32_type().const_int(1, false),
                    "increment_rc",
                );
                self.builder.build_store(rc_ptr, new_rc);
            }
            ir::InstructionKind::DecrementRc(idx, ty) => {
                let ptr = self.lookup(*idx)?.into_pointer_value();
                let rc_ptr = self.mark_ptr(ptr, *ty);
                let current_rc = self.builder.build_load(rc_ptr, "rc").into_int_value();
                let new_rc = self.builder.build_int_sub(
                    current_rc,
                    self.context.i32_type().const_int(1, false),
                    "decrement_rc",
                );
                self.builder.build_store(rc_ptr, new_rc);
                let free_block = self.context.append_basic_block(self.llvm, "free");
                let else_block = self.context.append_basic_block(self.llvm, "else");
                self.builder.build_conditional_branch(
                    self.builder.build_int_compare(
                        IntPredicate::SLE,
                        new_rc,
                        self.context.i32_type().const_int(0, false),
                        "is_zero",
                    ),
                    free_block,
                    else_block,
                );
                self.builder.position_at_end(free_block);
                self.build_free(ptr);
                self.builder.build_unconditional_branch(else_block);
                self.builder.position_at_end(else_block);
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
            ir::PatternKind::I64Literal(literal) => MatchCase::Literal(self.compile_i64_literal(*literal)),
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
                if self.lookup(*local_idx)?.is_vector_value() {
                    let value = self.lookup(*local_idx)?.into_vector_value();
                    // TODO: 回収と修正
                    // let const_ref = self.builder.build_alloca(value.get_type(), "return_tmp");
                    let const_ref = self
                        .builder
                        .build_malloc(value.get_type(), "return_tmp")
                        .map_err(|err| {
                            Box::from(Diagnostic::new_bug(
                                "failed to build malloc call",
                                Label::new(self.file_id, local_idx.span(), err),
                            ))
                        })?;
                    self.builder.build_store(const_ref, value);
                    let ptr = unsafe { self.sess.gep(const_ref, 0, "return_tmp") };
                    self.builder.build_return(Some(&ptr));
                } else {
                    self.builder.build_return(Some(&self.lookup(*local_idx)?));
                }
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
