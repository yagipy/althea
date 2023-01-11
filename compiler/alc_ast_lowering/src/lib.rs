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
use log::debug;
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
    // println!("----------lowering.lower----------");
    // println!("tys: {:#?}", lowering.tys);
    // println!("ir: {:#?}", lowering.ir);
    // println!("bind_points: {:#?}", lowering.bind_points);
    // println!("global_map: {:#?}", lowering.global_map);
    let (ir, ty_sess) = lowering.complete();
    debug!("{:#?}", ir);
    debug!("{:#?}", ty_sess);
    Ok((ir, ty_sess))
}
