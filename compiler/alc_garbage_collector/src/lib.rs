use alc_ast_lowering::{idx_vec::IdxVec, ir, ty};
use alc_command_option::{CommandOptions, Gc};
use alc_diagnostic::{FileId, Result};

pub fn collect(
    command_options: &CommandOptions,
    _file_id: FileId,
    _ty_sess: &ty::TySess,
    ir: ir::Ir,
) -> Result<ir::Ir> {
    match command_options.gc {
        Gc::OwnRc => {
            let mut ctx = OwnRcCtx::new();
            ctx.collect_ir(&ir)?;
            Ok(ctx.ir)
        }
        Gc::None => Ok(ir),
    }
}

struct OwnRcCtx {
    ir: ir::Ir,
}

impl<'gc> OwnRcCtx {
    fn new() -> OwnRcCtx {
        OwnRcCtx {
            ir: ir::Ir { defs: IdxVec::new() },
        }
    }

    fn collect_ir(&mut self, ir: &'gc ir::Ir) -> Result<()> {
        for def in ir.defs.values() {
            self.ir.defs.push(LocalOwnRcCtx::new(self).collect_def(def)?);
        }
        Ok(())
    }
}

struct LocalOwnRcCtx<'tcx> {
    _global_ctx: &'tcx OwnRcCtx,
}

impl<'tcx> LocalOwnRcCtx<'tcx> {
    fn new(global_ctx: &'tcx OwnRcCtx) -> LocalOwnRcCtx<'tcx> {
        LocalOwnRcCtx {
            _global_ctx: global_ctx,
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

    fn collect_entry(&self, entry: &ir::Entry) -> ir::Entry {
        ir::Entry {
            owner: entry.owner,
            param_bindings: entry.param_bindings.clone(),
            body: self.collect_block(&entry.body),
        }
    }

    fn collect_block(&self, block: &ir::Block) -> ir::Block {
        ir::Block {
            owner: block.owner,
            block_idx: block.block_idx,
            span: block.span,
            instructions: self.collect_instructions(&block.instructions),
            terminator: self.collect_terminator(&block.terminator),
        }
    }

    fn collect_instructions(&self, instructions: &[ir::Instruction]) -> Vec<ir::Instruction> {
        instructions
            .iter()
            .map(|instruction| self.collect_instruction(instruction))
            .collect()
    }

    fn collect_terminator(&self, terminator: &ir::Terminator) -> ir::Terminator {
        match terminator {
            ir::Terminator::Return(local_idx) => ir::Terminator::Return(*local_idx),
            ir::Terminator::Match { source, arms } => ir::Terminator::Match {
                source: *source,
                arms: arms.iter().map(|arm| self.collect_arm(arm)).collect(),
            },
        }
    }

    fn collect_instruction(&self, instruction: &ir::Instruction) -> ir::Instruction {
        instruction.clone()
    }

    fn collect_arm(&self, arm: &ir::Arm) -> ir::Arm {
        ir::Arm {
            span: arm.span,
            pattern: arm.pattern.clone(),
            target: self.collect_block(&arm.target),
        }
    }
}
