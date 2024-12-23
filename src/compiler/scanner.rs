use std::borrow::Cow;

use super::CompilerError;

#[derive(Debug, Clone)]
pub struct Scanner {
	text: String,
	start: usize,
	current: usize,
	line: usize,
}

impl Scanner {
	#[must_use]
	pub const fn new(text: String) -> Self {
		Self {
			text,
			start: 0,
			current: 0,
			line: 1,
		}
	}

	pub fn scan_token(&mut self) -> Result<Token<'_>, CompilerError> {
		self.skip_whitespace();
		self.start = self.current;

		if self.is_at_end() {
			return Ok(Token::new(
				TokenType::Eof,
				&self.text[self.start..self.current],
				self.line,
			));
		}

		let c = self.advance();

		Ok(match c {
			'(' => Token::new(TokenType::LeftParen, self.current_lexeme(), self.line),
			')' => Token::new(TokenType::RightParen, self.current_lexeme(), self.line),
			'{' => Token::new(TokenType::RightBrace, self.current_lexeme(), self.line),
			'}' => Token::new(TokenType::LeftBrace, self.current_lexeme(), self.line),
			';' => Token::new(TokenType::Semicolon, self.current_lexeme(), self.line),
			',' => Token::new(TokenType::Comma, self.current_lexeme(), self.line),
			'.' => Token::new(TokenType::Dot, self.current_lexeme(), self.line),
			'-' => Token::new(TokenType::Minus, self.current_lexeme(), self.line),
			'+' => Token::new(TokenType::Plus, self.current_lexeme(), self.line),
			'/' => Token::new(TokenType::Slash, self.current_lexeme(), self.line),
			'*' => Token::new(TokenType::Star, self.current_lexeme(), self.line),
			'!' => {
				if self.r#match('=') {
					Token::new(TokenType::BangEqual, self.current_lexeme(), self.line)
				} else {
					Token::new(TokenType::Bang, self.current_lexeme(), self.line)
				}
			}
			'=' => {
				if self.r#match('=') {
					Token::new(TokenType::EqualEqual, self.current_lexeme(), self.line)
				} else {
					Token::new(TokenType::Equal, self.current_lexeme(), self.line)
				}
			}
			'<' => {
				if self.r#match('=') {
					Token::new(TokenType::LessEqual, self.current_lexeme(), self.line)
				} else {
					Token::new(TokenType::Less, self.current_lexeme(), self.line)
				}
			}
			'>' => {
				if self.r#match('=') {
					Token::new(TokenType::GreaterEqual, self.current_lexeme(), self.line)
				} else {
					Token::new(TokenType::Greater, self.current_lexeme(), self.line)
				}
			}
			'"' => self.string()?,
			c if c.is_ascii_digit() => self.number(),
			c if Self::is_alpha(c) => self.identifier(),
			c => return Err(CompilerError::UnexpectedCharacter(c)),
		})
	}

	fn skip_whitespace(&mut self) {
		loop {
			let c = self.peek();
			match c {
				Some(' ' | '\r' | '\t') => {
					self.advance();
				}
				Some('\n') => {
					self.line += 1;
					self.advance();
				}
				Some('/') => {
					if matches!(self.peek_next(), Some('/')) {
						while !matches!(self.peek(), Some('\n')) && !self.is_at_end() {
							self.advance();
						}
					} else {
						return;
					}
				}
				_ => return,
			}
		}
	}

	fn number(&mut self) -> Token<'_> {
		while self.peek().is_some_and(|c| c.is_ascii_digit()) {
			self.advance();
		}

		if matches!(self.peek(), Some('.')) && self.peek_next().is_some_and(|c| c.is_ascii_digit())
		{
			self.advance();

			while self.peek().is_some_and(|c| c.is_ascii_digit()) {
				self.advance();
			}
		}

		Token::new(TokenType::Number, self.current_lexeme(), self.line)
	}

	fn identifier(&mut self) -> Token<'_> {
		while self
			.peek()
			.is_some_and(|c| Self::is_alpha(c) || c.is_ascii_digit())
		{
			self.advance();
		}

		Token::new(self.identifier_type(), self.current_lexeme(), self.line)
	}

	fn identifier_type(&self) -> TokenType {
		match self.text.chars().nth(self.start) {
			Some('a') => self.check_keyword(1, 2, "nd", TokenType::And),
			Some('c') => self.check_keyword(1, 4, "lass", TokenType::Class),
			Some('e') => self.check_keyword(1, 3, "lse", TokenType::Else),
			Some('f') if self.current - self.start > 1 => {
				match self.text.chars().nth(self.start + 1) {
					Some('a') => self.check_keyword(2, 3, "lse", TokenType::False),
					Some('o') => self.check_keyword(2, 1, "r", TokenType::For),
					Some('u') => self.check_keyword(2, 1, "n", TokenType::Fun),
					_ => TokenType::Identifier,
				}
			}
			Some('i') => self.check_keyword(1, 1, "f", TokenType::If),
			Some('n') => self.check_keyword(1, 2, "il", TokenType::Nil),
			Some('o') => self.check_keyword(1, 1, "r", TokenType::Or),
			Some('p') => self.check_keyword(1, 4, "rint", TokenType::Print),
			Some('r') => self.check_keyword(1, 5, "eturn", TokenType::Return),
			Some('s') => self.check_keyword(1, 4, "uper", TokenType::Super),
			Some('t') if self.current - self.start > 1 => {
				match self.text.chars().nth(self.start + 1) {
					Some('h') => self.check_keyword(2, 2, "is", TokenType::This),
					Some('r') => self.check_keyword(2, 2, "ue", TokenType::True),
					_ => TokenType::Identifier,
				}
			}
			Some('v') => self.check_keyword(1, 2, "ar", TokenType::Var),
			Some('w') => self.check_keyword(1, 4, "hile", TokenType::While),
			_ => TokenType::Identifier,
		}
	}

	fn check_keyword(&self, start: usize, length: usize, rest: &str, kind: TokenType) -> TokenType {
		if self.current - self.start == start + length && &self.current_lexeme()[start..] == rest {
			kind
		} else {
			TokenType::Identifier
		}
	}

	fn string(&mut self) -> Result<Token<'_>, CompilerError> {
		while !matches!(self.peek(), Some('"')) && !self.is_at_end() {
			if matches!(self.peek(), Some('\n')) {
				self.line += 1;
			}

			self.advance();
		}

		if self.is_at_end() {
			Err(CompilerError::UnterminatedString)
		} else {
			self.advance();
			Ok(Token::new(
				TokenType::String,
				self.current_lexeme(),
				self.line,
			))
		}
	}

	const fn is_alpha(c: char) -> bool {
		matches!(c, 'A'..='Z' | 'a'..='z' | '_')
	}

	fn current_lexeme(&self) -> &str {
		&self.text[self.start..self.current]
	}

	fn advance(&mut self) -> char {
		self.current += 1;
		self.text.chars().nth(self.current - 1).unwrap()
	}

	fn is_at_end(&self) -> bool {
		self.text.len() == self.current
	}

	fn peek_next(&self) -> Option<char> {
		if self.is_at_end() {
			None
		} else {
			self.text.chars().nth(self.current + 1)
		}
	}

	fn peek(&self) -> Option<char> {
		self.text.chars().nth(self.current)
	}

	fn r#match(&mut self, c: char) -> bool {
		if self.is_at_end() {
			return false;
		}

		if self.peek() != Some(c) {
			return false;
		}

		self.advance();
		true
	}
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
	kind: TokenType,
	lexeme: Cow<'a, str>,
	line: usize,
}

impl<'a> Token<'a> {
	const fn new(kind: TokenType, lexeme: &'a str, line: usize) -> Self {
		Self {
			kind,
			lexeme: Cow::Borrowed(lexeme),
			line,
		}
	}

	#[must_use]
	pub const fn kind(&self) -> TokenType {
		self.kind
	}

	#[must_use]
	pub fn lexeme(&self) -> &str {
		&self.lexeme
	}

	#[must_use]
	pub const fn line(&self) -> usize {
		self.line
	}

	pub fn into_static(self) -> Token<'static> {
		Token {
			kind: self.kind,
			lexeme: Cow::Owned(self.lexeme.into_owned()),
			line: self.line,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	Comma,
	Dot,
	Minus,
	Plus,
	Semicolon,
	Slash,
	Star,
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,
	Identifier,
	String,
	Number,
	And,
	Class,
	Else,
	False,
	For,
	Fun,
	If,
	Nil,
	Or,
	Print,
	Return,
	Super,
	This,
	True,
	Var,
	While,
	Error,
	Eof,
}
