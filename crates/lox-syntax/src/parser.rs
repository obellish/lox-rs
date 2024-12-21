use super::{
	position::{Diagnostic, Span, WithSpan},
	token::{Token, TokenKind},
};

static EOF_TOKEN: WithSpan<Token> = WithSpan::empty(Token::Eof);

pub struct Parser<'a> {
	tokens: &'a [WithSpan<Token>],
	cursor: usize,
	diagnostics: Vec<Diagnostic>,
}

impl<'a> Parser<'a> {
	pub const fn new(tokens: &'a [WithSpan<Token>]) -> Self {
		Self {
			tokens,
			cursor: 0,
			diagnostics: Vec::new(),
		}
	}

	pub fn diagnostics(&self) -> &[Diagnostic] {
		&self.diagnostics
	}

	pub fn error(&mut self, message: String, span: Span) {
		self.diagnostics.push(Diagnostic { span, message });
	}

	pub fn is_eof(&self) -> bool {
		self.check(TokenKind::Eof)
	}

	pub fn check(&self, match_token: TokenKind) -> bool {
		self.peek() == match_token
	}

	pub fn peek(&self) -> TokenKind {
		self.peek_token().into()
	}

	pub fn peek_token(&self) -> &'a WithSpan<Token> {
		self.tokens.get(self.cursor).map_or(&EOF_TOKEN, |t| t)
	}

	pub fn advance(&mut self) -> &'a WithSpan<Token> {
		let token = self.tokens.get(self.cursor);
		let Some(token) = token else {
			return &EOF_TOKEN;
		};

		self.cursor += 1;
		token
	}

	pub fn expect(&mut self, expected: TokenKind) -> Option<&'a WithSpan<Token>> {
		let token = self.advance();

		if TokenKind::from(token) == expected {
			Some(token)
		} else {
			self.error(
				format!("Expected {expected} got {}", token.value),
				token.span,
			);
			None
		}
	}

	pub fn optionally(&mut self, expected: TokenKind) -> Option<bool> {
		let token = self.peek();
		if TokenKind::from(token) == expected {
			self.expect(expected)?;
			Some(true)
		} else {
			Some(false)
		}
	}
}
