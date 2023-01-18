pub mod ast;
mod lexer;
mod parser;
mod token;

use crate::parser::Parser;
use alc_command_option::CommandOptions;
use alc_diagnostic::{FileId, Files, Result, Span, Spanned};
use log::debug;

pub fn parse(command_options: &CommandOptions, files: &Files, file_id: FileId) -> Result<ast::Ast> {
    let parser = Parser::new(command_options, files, file_id)?;
    let mut items = reserved_items();
    for item in parser {
        items.push(item?);
    }
    let ast = ast::Ast { items };
    debug!("{:#?}", ast);
    Ok(ast)
}

fn reserved_items() -> Vec<Spanned<ast::Item>> {
    vec![
        spanned(ast::Item::Struct(ast::Struct {
            name: spanned(String::from("SockAddrIn")),
            fields: vec![
                spanned(ast::Binding {
                    binder: spanned(String::from("family")),
                    ty: spanned(ast::Ty::I16),
                }),
                spanned(ast::Binding {
                    binder: spanned(String::from("port")),
                    ty: spanned(ast::Ty::I16),
                }),
                spanned(ast::Binding {
                    binder: spanned(String::from("addr")),
                    ty: spanned(ast::Ty::TyName(String::from("InAddr"))),
                }),
                spanned(ast::Binding {
                    binder: spanned(String::from("buf")),
                    ty: spanned(ast::Ty::Array(Box::new(ast::Ty::I8), 8)),
                }),
            ],
        })),
        spanned(ast::Item::Struct(ast::Struct {
            name: spanned(String::from("InAddr")),
            fields: vec![spanned(ast::Binding {
                binder: spanned(String::from("s_addr")),
                ty: spanned(ast::Ty::I32),
            })],
        })),
    ]
}

fn spanned<T>(target: T) -> Spanned<T> {
    Span::span(Span::dummy(), target)
}
