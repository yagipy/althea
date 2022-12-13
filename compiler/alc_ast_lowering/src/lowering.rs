use crate::{idx::Idxr, idx_vec::IdxVec, ir, lowering_ctx::LoweringCtx, ty, TyLowering, RESERVED_NAMES};
use alc_diagnostic::{Diagnostic, FileId, Label, Result, Span};
use alc_parser::ast;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Lowering<'ast> {
    #[allow(unused)]
    command_options: &'ast alc_command_option::CommandOptions,
    pub file_id: FileId,
    pub tys: TyLowering<'ast>,
    ir: ir::Ir,
    bind_points: IdxVec<ir::DefIdx, Span>,
    global_map: HashMap<&'ast ast::Ident, ir::DefIdx>,
}

impl<'ast> Lowering<'ast> {
    pub fn new(
        command_options: &'ast alc_command_option::CommandOptions,
        file_id: FileId,
        tys: TyLowering<'ast>,
    ) -> Lowering<'ast> {
        Lowering {
            command_options,
            file_id,
            tys,
            ir: ir::Ir { defs: IdxVec::new() },
            bind_points: IdxVec::new(),
            global_map: HashMap::new(),
        }
    }

    pub fn register<T>(&mut self, items: T) -> Result<()>
    where T: Iterator<Item = &'ast ast::Item> {
        for item in items {
            if let ast::Item::Fn(def) = item {
                self.bind(&def.name, def.name.span())?;
            }
        }
        Ok(())
    }

    pub fn lower<T>(&mut self, items: T) -> Result<()>
    where T: Iterator<Item = &'ast ast::Item> {
        for item in items {
            if let ast::Item::Fn(decl) = item {
                let def = self.lower_decl(decl)?;
                self.ir.defs.push(def);
            }
        }
        Ok(())
    }

    pub fn complete(self) -> (ir::Ir, ty::TySess) {
        (self.ir, self.tys.into_ty_sess())
    }

    pub fn lookup(&self, ident: &'ast ast::Ident, span: Span) -> Result<ir::DefIdx> {
        if let Some(def_idx) = self.global_map.get(ident) {
            Ok(*def_idx)
        } else {
            Err(Box::from(Diagnostic::new_error(
                "reference to unbound function symbol",
                Label::new(
                    self.file_id,
                    span,
                    &format!("'{}' is not bound as a function symbol", ident),
                ),
            )))
        }
    }

    fn bind(&mut self, ident: &'ast ast::Ident, span: Span) -> Result<ir::DefIdx> {
        if let Some(def_idx) = self.global_map.get(ident) {
            Err(Box::from(
                Diagnostic::new_error(
                    "attempt to rebind function name",
                    Label::new(
                        self.file_id,
                        span,
                        &format!("'{}' is already bound to a function", ident),
                    ),
                )
                .with_secondary_labels(vec![Label::new(
                    self.file_id,
                    self.bind_points[*def_idx],
                    "previously bound here",
                )]),
            ))
        } else {
            let def_idx = self.bind_points.push(span);
            self.global_map.insert(ident, def_idx);
            Ok(def_idx)
        }
    }

    fn lower_decl(&mut self, decl: &'ast ast::FnDecl) -> Result<ir::Def> {
        if RESERVED_NAMES.contains(&&**decl.name) {
            return Err(Box::from(Diagnostic::new_error(
                "use of reserved name",
                Label::new(
                    self.file_id,
                    decl.name.span(),
                    &format!("'{}' is reserved for use by the compiler", &**decl.name),
                ),
            )));
        }
        let def_idx = self.lookup(&decl.name, decl.name.span())?;
        let local_idxr = Idxr::new();
        let block_idxr = Idxr::new();
        let mut lcx = LoweringCtx::new(self, &local_idxr, &block_idxr, def_idx);
        let mut param_tys = IdxVec::new();
        let mut param_bindings: IdxVec<ty::ParamIdx, ir::LocalIdx> = IdxVec::new();
        for binding in decl.params.iter() {
            let local_idx = local_idxr.next().with_span(binding.span());
            if lcx.bind(&binding.binder, local_idx, None).is_some() {
                return Err(Box::from(Diagnostic::new_error(
                    "attempted to rebind formal parameter",
                    Label::new(
                        self.file_id,
                        binding.span(),
                        "a formal parameter with this name already exists",
                    ),
                )));
            }
            param_bindings.push(local_idx.with_span(binding.span()));
            param_tys.push(self.tys.lookup_ty(&binding.ty, binding.ty.span())?);
        }
        let return_ty = self.tys.lookup_ty(&decl.return_ty, decl.return_ty.span())?;
        Ok(ir::Def {
            def_idx,
            name: decl.name.clone(),
            span: decl.name.span(),
            ty: self.tys.ty_sess().make_fn(return_ty, param_tys),
            entry: lcx.lower_entry(param_bindings, &decl.body, decl.body.span())?,
            local_idxr,
        })
    }
}
