mod scanner;

use std::{
	borrow::Cow,
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult, Write as _},
};

pub use self::scanner::*;
use super::Chunk;

pub struct Compiler {
	scanner: Scanner,
	previous: Option<Token<'static>>,
	current: Option<Token<'static>>,
	compiling_chunk: Chunk,
}

impl Compiler {
	#[must_use]
	pub const fn new(text: String) -> Self {
		Self {
			scanner: Scanner::new(text),
			previous: None,
			current: None,
			compiling_chunk: Chunk::new(),
		}
	}

	pub fn compile(&mut self) -> Result<Chunk, CompilerError> {
		self.advance()?;

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
	ErrorAt(usize, Option<Cow<'static, str>>),
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
			Self::ErrorAt(line, token) => {
				f.write_str("Error at line ")?;
				Display::fmt(&line, f)?;
				f.write_str(" at ")?;
				if let Some(token) = token {
					f.write_str(token)?;
				} else {
					f.write_str("end")?;
				}
				f.write_char('.')
			}
		}
	}
}

impl StdError for CompilerError {}
