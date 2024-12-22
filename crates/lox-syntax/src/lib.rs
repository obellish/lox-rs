pub mod ast;
mod common;
mod expr_parser;
mod parser;
pub mod position;
mod stmt_parser;
mod token;
mod tokenizer;

use parser::Parser;

use self::{ast::Ast, position::Diagnostic, tokenizer::tokenize_with_context};

pub fn parse(code: &str) -> Result<Ast, Vec<Diagnostic>> {
	let tokens = tokenize_with_context(code);
	let mut parser = Parser::new(&tokens);
	match stmt_parser::parse(&mut parser) {
		Some(ast) if parser.diagnostics().is_empty() => Ok(ast),
		Some(_) | None => Err(parser.diagnostics().to_owned()),
	}
}
