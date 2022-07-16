pub mod ast;
mod lexer;
mod parser;
mod token;

use crate::parser::Parser;
use alc_command_option::CommandOptions;
use alc_diagnostic::{FileId, Files, Result};
use log::debug;

pub fn parse(command_options: &CommandOptions, files: &Files, file_id: FileId) -> Result<ast::Ast> {
    let parser = Parser::new(command_options, files, file_id)?;
    let mut items = vec![];
    for item in parser {
        items.push(item?);
    }
    let ast = ast::Ast { items };
    debug!("{:#?}", ast);
    Ok(ast)
}
