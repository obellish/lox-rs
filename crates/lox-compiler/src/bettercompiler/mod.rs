mod compiler;
mod locals;
mod statements;
#[cfg(test)]
mod tests;

use lox_bytecode::{bytecode::Module, opcode};
use lox_syntax::{ast::BorrowedAst, position::Diagnostic};

use self::{
	compiler::{Compiler, ContextType},
	statements::compile_ast,
};

pub fn compile(ast: BorrowedAst<'_>) -> Result<Module, Vec<Diagnostic>> {
	let mut compiler = Compiler::new();

	let _ = compiler.with_context(ContextType::TopLevel, |compiler| {
		compile_ast(compiler, ast);
		compiler.add_u8(opcode::RETURN_TOP);
	});

	if compiler.has_errors() {
		Err(compiler.into_errors())
	} else {
		Ok(compiler.into_module())
	}
}
