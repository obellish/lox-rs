use super::{
	parser::Parser,
	position::{Span, WithSpan},
};
use crate::ast::Stmt;

type StmtWithSpan = WithSpan<Stmt>;

fn parse_program(it: &mut Parser<'_>) -> Option<Vec<StmtWithSpan>> {
	todo!()
}

fn parse_declaration(it: &mut Parser<'_>) -> Option<StmtWithSpan> {
	todo!()
}
