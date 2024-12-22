mod bettercompiler;

use lox_bytecode::bytecode::Module;
pub use lox_syntax::position::{Diagnostic, LineOffsets};

pub fn compile(code: &str) -> Result<Module, Vec<Diagnostic>> {
	let ast = lox_syntax::parse(code)?;

	bettercompiler::compile(&ast)
}
