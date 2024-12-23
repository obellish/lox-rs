mod scanner;

use std::{
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult, Write},
};

pub use self::scanner::*;
use super::Chunk;

pub struct Compiler {
	scanner: Scanner,
	previous: Option<Token<'static>>,
	current: Option<Token<'static>>,
}

impl Compiler {
	#[must_use]
	pub const fn new(text: String) -> Self {
		Self {
			scanner: Scanner::new(text),
			previous: None,
			current: None,
		}
	}

	pub fn compile(&mut self) -> Result<Chunk, CompilerError> {
		let mut line = 0usize;
		loop {
			let token = self.scanner.scan_token()?;

			if token.line() == line {
				print!("   | ");
			} else {
				print!("{:>4} ", token.line());
				line = token.line();
			}

			println!("{:>2} '{}'", token.kind() as u8, token.lexeme());

			if matches!(token.kind(), TokenType::Eof) {
				break;
			}
		}

		Ok(Chunk::new())
	}

	fn advance(&mut self) -> Result<(), CompilerError> {
		self.previous = self.current.clone();

		let token = self.scanner.scan_token()?.into_static();

		self.current.replace(token);

		Ok(())
	}
}

#[derive(Debug)]
pub enum CompilerError {
	UnexpectedCharacter(char),
	UnterminatedString,
}

impl Display for CompilerError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::UnexpectedCharacter(c) => {
				f.write_str("Unexpected character: ")?;
				f.write_char(*c)?;
				f.write_char('.')
			}
			Self::UnterminatedString => f.write_str("Unterminated string."),
		}
	}
}

impl StdError for CompilerError {}
