pub mod idx;
pub mod idx_vec;
pub mod ir;
mod lowering;
mod lowering_ctx;
pub mod ty;
mod ty_lowering;

use crate::ty_lowering::TyLowering;
use alc_command_option::CommandOptions;
use alc_diagnostic::{FileId, Result, Spanned};
use alc_parser::ast;
use std::ops::Deref;

pub const ENTRY_NAME: &str = "main";
pub const RESERVED_NAMES: &[&str] = &[
    "malloc",
    "GC_malloc",
    "free",
    "alc_reset",
    "alc_free",
    "alc_alloc",
];

pub fn lower(
    command_options: &CommandOptions,
    file_id: FileId,
    ast: &ast::Ast,
) -> Result<(ir::Ir, ty::TySess)> {
    let mut ty_lowering = TyLowering::new(command_options, file_id);
    ty_lowering.register(ast.items.iter().map(Spanned::deref))?;
    ty_lowering.lower(ast.items.iter().map(Spanned::deref))?;
    let mut lowering = lowering::Lowering::new(command_options, file_id, ty_lowering);
    lowering.register(ast.items.iter().map(Spanned::deref))?;
    lowering.lower(ast.items.iter().map(Spanned::deref))?;
    Ok(lowering.complete())
}
