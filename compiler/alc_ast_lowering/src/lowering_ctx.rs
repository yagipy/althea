use crate::{
    idx::Idxr,
    idx_vec::{IdxVec, IntoIdxVec},
    ir,
    lowering::Lowering,
    ty,
};
use alc_diagnostic::{Diagnostic, Label, Result, Span};
use alc_parser::ast;
use std::collections::HashMap;

#[derive(Debug)]
pub(super) struct LoweringCtx<'lcx, 'ast> {
    sess: &'lcx Lowering<'ast>,
    local_idxr: &'lcx Idxr<ir::LocalIdx>,
    block_idxr: &'lcx Idxr<ir::BlockIdx>,
    def_idx: ir::DefIdx,
    parent: Option<&'lcx LoweringCtx<'lcx, 'ast>>,
    local_map: HashMap<&'ast ast::Ident, (ir::LocalIdx, Option<ty::Ty>)>,
    field_map: HashMap<ir::LocalIdx, IdxVec<ty::FieldIdx, ir::LocalIdx>>,
    instructions: Vec<ir::Instruction>,
}

impl<'lcx, 'ast> LoweringCtx<'lcx, 'ast> {
    pub(super) fn new(
        sess: &'lcx Lowering<'ast>,
        local_idxr: &'lcx Idxr<ir::LocalIdx>,
        block_idxr: &'lcx Idxr<ir::BlockIdx>,
        def_idx: ir::DefIdx,
    ) -> LoweringCtx<'lcx, 'ast> {
        LoweringCtx {
            sess,
            local_idxr,
            block_idxr,
            def_idx,
            parent: None,
            local_map: HashMap::new(),
            field_map: HashMap::new(),
            instructions: vec![],
        }
    }

    pub(super) fn lower_entry(
        self,
        param_bindings: IdxVec<ty::ParamIdx, ir::LocalIdx>,
        term: &'ast ast::Term,
        span: Span,
    ) -> Result<ir::Entry> {
        Ok(ir::Entry {
            owner: self.def_idx,
            param_bindings,
            body: self.lower_term_to_block(term, span)?,
        })
    }

    fn mk_child(&'lcx self) -> LoweringCtx<'lcx, 'ast> {
        LoweringCtx {
            sess: self.sess,
            local_idxr: self.local_idxr,
            block_idxr: self.block_idxr,
            def_idx: self.def_idx,
            parent: Some(self),
            local_map: HashMap::new(),
            field_map: HashMap::new(),
            instructions: vec![],
        }
    }

    fn lookup(&self, ident: &'ast ast::Ident, span: Span) -> Result<(ir::LocalIdx, Option<ty::Ty>)> {
        if let Some(local_idx) = self.local_map.get(ident) {
            Ok(*local_idx)
        } else if let Some(parent) = self.parent {
            parent.lookup(ident, span)
        } else {
            Err(Box::from(Diagnostic::new_error(
                "reference to unbound variable",
                Label::new(
                    self.sess.file_id,
                    span,
                    &format!("'{}' is not bound here (while lowering)", ident),
                ),
            )))
        }
    }

    fn lookup_field(
        &self,
        local_idx: ir::LocalIdx,
        field_idx: ty::FieldIdx,
        span: Span,
    ) -> Result<ir::LocalIdx> {
        if let Some(field_map) = self.field_map.get(&local_idx) {
            if let Some(field_local_idx) = field_map.get(field_idx) {
                Ok(field_local_idx.with_span(span))
            } else {
                Err(Box::from(Diagnostic::new_error(
                    "reference to unbound field",
                    Label::new(
                        self.sess.file_id,
                        span,
                        &format!("'{:?}' is not bound here (while lowering)", field_idx),
                    ),
                )))
            }
        } else if let Some(parent) = self.parent {
            parent.lookup_field(local_idx, field_idx, span)
        } else {
            Err(Box::from(Diagnostic::new_error(
                "reference to unbound field",
                Label::new(
                    self.sess.file_id,
                    span,
                    &format!("'{:?}' is not bound here (while lowering)", field_idx),
                ),
            )))
        }
    }

    fn lookup_fields(
        &self,
        local_idx: ir::LocalIdx,
        field_idxes: Vec<ty::FieldIdx>,
        span: Span,
    ) -> Result<ir::LocalIdx> {
        let mut root_local_idx = self
            .lookup_field(local_idx, *field_idxes.first().unwrap(), span)
            .unwrap();
        for field_idx in field_idxes.into_iter().skip(1) {
            root_local_idx = self.lookup_field(root_local_idx, field_idx, span).unwrap();
        }
        Ok(root_local_idx)
    }

    #[inline]
    pub(super) fn bind(
        &mut self,
        ident: &'ast ast::Ident,
        local_idx: ir::LocalIdx,
        ty: Option<ty::Ty>,
    ) -> Option<(ir::LocalIdx, Option<ty::Ty>)> {
        self.local_map.insert(ident, (local_idx, ty))
    }

    #[inline]
    fn lower_unop_kind(&self, kind: ast::UnopKind) -> ir::UnopKind {
        match kind {
            ast::UnopKind::Not => ir::UnopKind::Not,
        }
    }

    #[inline]
    fn lower_binop_kind(&self, kind: ast::BinopKind) -> ir::BinopKind {
        match kind {
            ast::BinopKind::Plus => ir::BinopKind::Plus,
            ast::BinopKind::Minus => ir::BinopKind::Minus,
            ast::BinopKind::Mul => ir::BinopKind::Mul,
            ast::BinopKind::Div => ir::BinopKind::Div,
            ast::BinopKind::Less => ir::BinopKind::Less,
            ast::BinopKind::Leq => ir::BinopKind::Leq,
            ast::BinopKind::Greater => ir::BinopKind::Greater,
            ast::BinopKind::Geq => ir::BinopKind::Geq,
            ast::BinopKind::Eq => ir::BinopKind::Eq,
            ast::BinopKind::Neq => ir::BinopKind::Neq,
            ast::BinopKind::And => ir::BinopKind::And,
            ast::BinopKind::Or => ir::BinopKind::Or,
            ast::BinopKind::Xor => ir::BinopKind::Xor,
            ast::BinopKind::LShift => ir::BinopKind::LShift,
            ast::BinopKind::RShift => ir::BinopKind::RShift,
        }
    }

    fn lower_expr_kind(
        &mut self,
        ty: Option<ty::Ty>,
        expr: &'ast ast::Expr,
        span: Span,
    ) -> Result<ir::ExprKind> {
        Ok(match expr {
            ast::Expr::NumberLiteral(literal) => {
                if let Some(ty) = ty {
                    if self.sess.tys.ty_sess().make_i8() == ty {
                        return Ok(ir::ExprKind::I8Literal(*literal as i8));
                    } else if self.sess.tys.ty_sess().make_i16() == ty {
                        return Ok(ir::ExprKind::I16Literal(*literal as i16));
                    } else if self.sess.tys.ty_sess().make_i32() == ty {
                        return Ok(ir::ExprKind::I32Literal(*literal as i32));
                    } else if self.sess.tys.ty_sess().make_u64() == ty {
                        return Ok(ir::ExprKind::U64Literal(*literal as u64));
                    }
                }
                ir::ExprKind::I32Literal(*literal as i32)
            }
            ast::Expr::U64Literal(literal) => ir::ExprKind::U64Literal(*literal),
            ast::Expr::StringLiteral(literal) => ir::ExprKind::StringLiteral(literal.clone()),
            ast::Expr::Var(stream) => {
                let (local_idx, ty) = self.lookup(stream.first().unwrap(), span)?;
                if stream.len() == 1 {
                    return Ok(ir::ExprKind::Var(local_idx, vec![]));
                }
                let mut field_idxes = vec![];
                for field in stream.iter().skip(1) {
                    let field_idx = self.sess.tys.lookup_field(ty.unwrap(), field, field.span())?;
                    field_idxes.push(field_idx);
                }
                ir::ExprKind::Var(local_idx, field_idxes)
            }
            ast::Expr::Unop { kind, operand } => ir::ExprKind::Unop {
                kind: self.lower_unop_kind(**kind),
                operand: self.lower_expr(None, operand, operand.span())?,
            },
            ast::Expr::Binop { kind, left, right } => ir::ExprKind::Binop {
                kind: self.lower_binop_kind(**kind),
                left: self.lower_expr(None, left, left.span())?,
                right: self.lower_expr(None, right, right.span())?,
            },
            ast::Expr::Call { target, args } => {
                let mut lowered_args = IdxVec::new();
                for arg in args.iter() {
                    lowered_args.push(self.lower_expr(None, arg, arg.span())?);
                }
                ir::ExprKind::Call {
                    target: self.sess.lookup(target, target.span())?,
                    args: lowered_args,
                }
            }
            ast::Expr::Variant {
                enum_name,
                discriminant,
                body,
            } => {
                let enum_ty = self.sess.tys.lookup(enum_name, enum_name.span())?;
                ir::ExprKind::Variant {
                    ty: enum_ty,
                    discriminant: self
                        .sess
                        .tys
                        .lookup_variant(enum_ty, discriminant, discriminant.span())?,
                    body: self.lower_expr(None, body, body.span())?,
                }
            }
            ast::Expr::Record { struct_name, fields } => {
                let struct_ty = self.sess.tys.lookup(struct_name, struct_name.span())?;
                let mut field_bindings = HashMap::new();
                for (field, body) in fields.iter() {
                    let field_idx = self.sess.tys.lookup_field(struct_ty, field, field.span())?;
                    let field_ty = self.sess.tys.ty_sess().ty_kind(struct_ty).field_ty(field_idx);
                    let lowered = self.lower_expr(field_ty, body, body.span())?;
                    if let Some(idx) = field_bindings.insert(
                        self.sess.tys.lookup_field(struct_ty, field, field.span())?,
                        lowered,
                    ) {
                        return Err(Box::from(
                            Diagnostic::new_error(
                                "malformed struct initializer",
                                Label::new(
                                    self.sess.file_id,
                                    span,
                                    "attempted to initialise the same field twice",
                                ),
                            )
                            .with_secondary_labels(vec![
                                Label::new(
                                    self.sess.file_id,
                                    lowered.span(),
                                    &format!("attempted to initialise '{}' here", &**field),
                                ),
                                Label::new(
                                    self.sess.file_id,
                                    idx.span(),
                                    "but it was already initialised here",
                                ),
                            ]),
                        ));
                    }
                }
                if let Some(fields) = field_bindings.into_idx_vec() {
                    ir::ExprKind::Record {
                        ty: struct_ty,
                        fields,
                    }
                } else {
                    return Err(Box::from(Diagnostic::new_error(
                        "malformed struct initializer",
                        Label::new(self.sess.file_id, span, "not all fields initialised"),
                    )));
                }
            }
            ast::Expr::Socket { domain, ty, protocol } => {
                // TODO: Noneやめる
                let domain = self.lower_expr(None, domain, domain.span())?;
                let ty = self.lower_expr(None, ty, ty.span())?;
                let protocol = self.lower_expr(None, protocol, protocol.span())?;
                ir::ExprKind::Socket { domain, ty, protocol }
            }
        })
    }

    fn lower_expr(&mut self, ty: Option<ty::Ty>, expr: &'ast ast::Expr, span: Span) -> Result<ir::LocalIdx> {
        let kind = self.lower_expr_kind(ty, expr, span)?;
        Ok(match kind {
            ir::ExprKind::Var(local_idx, field_idxes) => {
                if field_idxes.is_empty() {
                    return Ok(local_idx);
                }
                self.lookup_fields(local_idx, field_idxes, span)?
            }
            ir::ExprKind::Record {
                ty: record_ty,
                fields,
            } => {
                let idx = self.local_idxr.next();
                self.field_map.insert(idx, fields.clone());
                self.instructions.push(ir::Instruction {
                    span,
                    kind: ir::InstructionKind::Let {
                        binding: idx,
                        ty,
                        expr: ir::Expr {
                            local_idx: idx,
                            span,
                            kind: ir::ExprKind::Record {
                                ty: record_ty,
                                fields,
                            },
                        },
                    },
                });
                idx
            }
            _ => {
                let idx = self.local_idxr.next();
                self.instructions.push(ir::Instruction {
                    span,
                    kind: ir::InstructionKind::Let {
                        binding: idx,
                        ty,
                        expr: ir::Expr {
                            local_idx: idx,
                            span,
                            kind,
                        },
                    },
                });
                idx
            }
        }
        .with_span(span))
    }

    fn lower_arm(
        &mut self,
        pattern: &'ast ast::Pattern,
        body: &'ast ast::Term,
        pattern_span: Span,
        body_span: Span,
    ) -> Result<ir::Arm> {
        let mut ctx = self.mk_child();
        let pattern = match pattern {
            ast::Pattern::NumberLiteral(literal) => {
                // TODO: 型のハンドリング
                ir::PatternKind::I32Literal(*literal as i32)
            }
            ast::Pattern::U64Literal(literal) => ir::PatternKind::U64Literal(*literal),
            ast::Pattern::StringLiteral(literal) => ir::PatternKind::StringLiteral(literal.clone()),
            ast::Pattern::Ident(ident) => {
                let local_idx = self.local_idxr.next().with_span(pattern_span);
                ctx.bind(ident, local_idx, None);
                ir::PatternKind::Ident(local_idx)
            }
            ast::Pattern::Variant {
                enum_name,
                discriminant,
                bound,
            } => {
                let local_idx = self.local_idxr.next().with_span(bound.span());
                ctx.bind(bound, local_idx, None);
                let ty = self.sess.tys.lookup(enum_name, enum_name.span())?;
                let discriminant = self
                    .sess
                    .tys
                    .lookup_variant(ty, discriminant, discriminant.span())?;
                ir::PatternKind::Variant {
                    ty,
                    discriminant,
                    binding: local_idx,
                }
            }
            ast::Pattern::Record { struct_name, fields } => {
                let ty = self.sess.tys.lookup(struct_name, struct_name.span())?;
                let mut field_bindings = HashMap::new();
                for (field, bound) in fields {
                    let local_idx = self.local_idxr.next().with_span(bound.span());
                    ctx.bind(bound, local_idx, Some(ty));
                    let field = self.sess.tys.lookup_field(ty, field, field.span())?;
                    field_bindings.insert(field, local_idx);
                }
                if let Some(fields) = field_bindings.into_idx_vec() {
                    ir::PatternKind::Record { ty, fields }
                } else {
                    return Err(Box::from(Diagnostic::new_error(
                        "malformed match arm",
                        Label::new(self.sess.file_id, pattern_span, "not all fields are matched"),
                    )));
                }
            }
        };
        Ok(ir::Arm {
            span: pattern_span.merge(body_span),
            pattern,
            target: Box::new(ctx.lower_term_to_block(body, body_span)?),
        })
    }

    fn lower_term(&mut self, term: &'ast ast::Term, span: Span) -> Result<ir::Terminator> {
        match term {
            ast::Term::Let {
                binder,
                annotation,
                expr,
                body,
            } => {
                let ty = match annotation {
                    Some(ty) => Some(self.sess.tys.lookup_ty(ty, ty.span())?),
                    _ => None,
                };
                let idx = self.lower_expr(ty, expr, expr.span())?;
                self.bind(binder, idx, ty);
                self.lower_term(body, body.span())
            }
            ast::Term::Println { expr, body } => {
                let idx = self.lower_expr(None, expr, expr.span())?;
                self.instructions.push(ir::Instruction {
                    span,
                    kind: ir::InstructionKind::Println { idx },
                });
                self.lower_term(body, body.span())
            }
            ast::Term::Match { source, arms } => {
                let source = self.lower_expr(None, source, source.span())?;
                let mut lowered_arms = vec![];
                for (pattern, body) in arms.iter() {
                    lowered_arms.push(self.lower_arm(pattern, body, pattern.span(), body.span())?);
                }
                Ok(ir::Terminator::Match {
                    source,
                    arms: lowered_arms,
                })
            }
            ast::Term::If {
                source,
                then,
                otherwise,
            } => {
                let source = self.lower_expr(None, source, source.span())?;
                Ok(ir::Terminator::Match {
                    source,
                    arms: vec![
                        ir::Arm {
                            span: otherwise.span(),
                            pattern: ir::PatternKind::U64Literal(0),
                            target: Box::new(
                                self.mk_child().lower_term_to_block(otherwise, otherwise.span())?,
                            ),
                        },
                        ir::Arm {
                            span: then.span(),
                            pattern: ir::PatternKind::Ident(self.local_idxr.next().with_span(source.span())),
                            target: Box::new(self.mk_child().lower_term_to_block(then, then.span())?),
                        },
                    ],
                })
            }
            ast::Term::Return(expr) => Ok(ir::Terminator::Return(self.lower_expr(None, expr, span)?)),
        }
    }

    fn lower_term_to_block(mut self, term: &'ast ast::Term, span: Span) -> Result<ir::Block> {
        let block_idx = self.block_idxr.next();
        let terminator = self.lower_term(term, span)?;
        Ok(ir::Block {
            owner: self.def_idx,
            block_idx,
            span,
            instructions: self.instructions,
            terminator,
        })
    }
}
