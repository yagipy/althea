use alc_ast_lowering::{idx_vec::IdxVec, ir, ir::LocalIdx, ty};
use alc_command_option::{CommandOptions, Gc};
use alc_diagnostic::{FileId, Result};
use std::collections::HashMap;

pub fn collect(
    command_options: &CommandOptions,
    _file_id: FileId,
    ty_sess: &ty::TySess,
    ir: ir::Ir,
) -> Result<ir::Ir> {
    match command_options.gc {
        Gc::OwnRc => {
            let mut ctx = OwnRcCtx::new(ty_sess);
            ctx.collect_ir(&ir)?;
            Ok(ctx.ir)
        }
        Gc::None => Ok(ir),
    }
}

struct OwnRcCtx<'gc> {
    ir: ir::Ir,
    ty_sess: &'gc ty::TySess,
}

impl<'gc> OwnRcCtx<'gc> {
    fn new(ty_sess: &'gc ty::TySess) -> OwnRcCtx {
        OwnRcCtx {
            ir: ir::Ir { defs: IdxVec::new() },
            ty_sess,
        }
    }

    fn collect_ir(&mut self, ir: &'gc ir::Ir) -> Result<()> {
        for def in ir.defs.values() {
            self.ir
                .defs
                .push(LocalOwnRcCtx::new(self, None).collect_def(def)?);
        }
        Ok(())
    }
}

pub type RefCount = i32;

struct LocalOwnRcCtx<'gc> {
    global_ctx: &'gc OwnRcCtx<'gc>,
    instructions: Vec<ir::Instruction>,
    malloc_map: HashMap<LocalIdx, (ty::Ty, RefCount)>,
    _parent: Option<&'gc LocalOwnRcCtx<'gc>>,
}

impl<'gc> LocalOwnRcCtx<'gc> {
    fn new(global_ctx: &'gc OwnRcCtx, parent: Option<&'gc LocalOwnRcCtx<'gc>>) -> LocalOwnRcCtx<'gc> {
        LocalOwnRcCtx {
            global_ctx,
            instructions: vec![],
            malloc_map: HashMap::new(),
            _parent: parent,
        }
    }

    fn collect_def(&mut self, def: &ir::Def) -> Result<ir::Def> {
        Ok(ir::Def {
            def_idx: def.def_idx,
            name: def.name.clone(),
            ty: def.ty,
            span: def.span,
            entry: self.collect_entry(&def.entry),
            local_idxr: def.local_idxr.clone(),
        })
    }

    fn collect_entry(&mut self, entry: &ir::Entry) -> ir::Entry {
        ir::Entry {
            owner: entry.owner,
            // TODO: 対応必要か確認
            param_bindings: entry.param_bindings.clone(),
            body: self.collect_block(&entry.body),
        }
    }

    fn collect_block(&mut self, block: &ir::Block) -> ir::Block {
        self.push_instructions(&block.instructions);
        let terminator = self.collect_terminator(&block.terminator);
        ir::Block {
            owner: block.owner,
            block_idx: block.block_idx,
            span: block.span,
            instructions: self.instructions.clone(),
            terminator,
        }
    }

    fn push_instructions(&mut self, instructions: &[ir::Instruction]) {
        for instruction in instructions {
            let instruction = self.collect_instruction(instruction);
            self.instructions.push(instruction);
        }
    }

    fn collect_terminator(&mut self, terminator: &ir::Terminator) -> ir::Terminator {
        match terminator {
            ir::Terminator::Return(local_idx) => {
                self.release_malloc_map();
                ir::Terminator::Return(*local_idx)
            }
            ir::Terminator::Match { source, arms } => {
                // TODO: 対応必要か確認
                ir::Terminator::Match {
                    source: *source,
                    arms: arms.iter().map(|arm| self.collect_arm(arm)).collect(),
                }
            }
        }
    }

    fn collect_instruction(&mut self, instruction: &ir::Instruction) -> ir::Instruction {
        match instruction {
            ir::Instruction {
                kind:
                    ir::InstructionKind::Let {
                        binding: _,
                        ty: _,
                        expr:
                            ir::Expr {
                                local_idx,
                                span: _,
                                kind: ir::ExprKind::Record { ty, fields },
                            },
                    },
                span: _,
            } => {
                self.retain_malloc_map(*local_idx, *ty);
                let fields_by_ty_sess = self
                    .global_ctx
                    .ty_sess
                    .ty_kind(*ty)
                    .as_struct()
                    .unwrap()
                    .fields
                    .clone();
                for (field_idx, local_idx) in fields.iter() {
                    if self
                        .global_ctx
                        .ty_sess
                        .ty_kind(*fields_by_ty_sess.get(field_idx).unwrap())
                        .is_struct()
                    {
                        self.retain_malloc_map(*local_idx, *fields_by_ty_sess.get(field_idx).unwrap());
                    }
                }
            }
            ir::Instruction {
                kind:
                    ir::InstructionKind::Let {
                        binding: _,
                        ty: Some(ty),
                        expr:
                            ir::Expr {
                                local_idx: _,
                                span: _,
                                kind: ir::ExprKind::Var(local_idx_in_var, _),
                            },
                    },
                span: _,
            } => {
                if self.global_ctx.ty_sess.ty_kind(*ty).is_struct() {
                    self.retain_malloc_map(*local_idx_in_var, *ty);
                }
            }
            _ => {}
        }
        instruction.clone()
    }

    fn collect_arm(&mut self, arm: &ir::Arm) -> ir::Arm {
        ir::Arm {
            span: arm.span,
            pattern: arm.pattern.clone(),
            target: LocalOwnRcCtx::new(self.global_ctx, Some(self)).collect_block(&arm.target),
        }
    }

    fn retain_malloc_map(&mut self, local_idx: LocalIdx, ty: ty::Ty) {
        if let Some((_, count)) = self.malloc_map.get_mut(&local_idx) {
            *count += 1;
            self.instructions.push(ir::Instruction {
                kind: ir::InstructionKind::IncrementRc(local_idx, ty),
                span: local_idx.span(),
            });
            return;
        }

        self.malloc_map.insert(local_idx, (ty, 1));
    }

    fn release_malloc_map(&mut self) {
        let mut new_malloc_map = HashMap::new();
        for (malloc_idx, (ty, count)) in self.malloc_map.iter_mut() {
            *count -= 1;
            if count <= &mut 0 {
                self.instructions.push(ir::Instruction {
                    kind: ir::InstructionKind::Free(*malloc_idx, *ty),
                    span: malloc_idx.span(),
                });
                continue;
            } else {
                self.instructions.push(ir::Instruction {
                    kind: ir::InstructionKind::DecrementRc(*malloc_idx, *ty),
                    span: malloc_idx.span(),
                });
            }

            new_malloc_map.insert(*malloc_idx, (*ty, *count));
        }
        self.malloc_map = new_malloc_map;
    }
}
