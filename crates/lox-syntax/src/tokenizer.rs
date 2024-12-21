use std::{
	iter::Peekable,
	str::{self, Chars},
};

use super::{
	position::{BytePos, Span, WithSpan},
	token::Token,
};

struct Scanner<'a> {
	current_position: BytePos,
	it: Peekable<Chars<'a>>,
}

impl<'a> Scanner<'a> {
	pub fn new(buf: &'a str) -> Self {
		Self {
			current_position: BytePos::default(),
			it: buf.chars().peekable(),
		}
	}

	pub fn peek(&mut self) -> Option<char> {
		self.it.peek().copied()
	}

	pub fn consume_if<F>(&mut self, f: F) -> bool
	where
		F: FnOnce(char) -> bool,
	{
		self.peek().is_some_and(|ch| {
			if f(ch) {
				self.next().unwrap();
				true
			} else {
				false
			}
		})
	}

	pub fn consume_if_next<F>(&mut self, f: F) -> bool
	where
		F: FnOnce(char) -> bool,
	{
		let mut it = self.it.clone();
		if it.next().is_none() {
			return false;
		}

		it.peek().is_some_and(|&ch| {
			if f(ch) {
				self.next().unwrap();
				true
			} else {
				false
			}
		})
	}

	pub fn consume_while<F>(&mut self, f: F) -> Vec<char>
	where
		F: Fn(char) -> bool,
	{
		let mut chars = Vec::new();

		while let Some(ch) = self.peek() {
			if f(ch) {
				self.next().unwrap();
				chars.push(ch);
			} else {
				break;
			}
		}

		chars
	}
}

impl Iterator for Scanner<'_> {
	type Item = char;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.it.next();
		if let Some(c) = next {
			self.current_position = self.current_position.shift(c);
		}

		next
	}
}

#[repr(transparent)]
struct Lexer<'a> {
	it: Scanner<'a>,
}

impl<'a> Lexer<'a> {
	pub fn new(buf: &'a str) -> Self {
		Self {
			it: Scanner::new(buf),
		}
	}

	fn match_token(&mut self, ch: char) -> Option<Token> {
		match ch {
			'=' => Some(self.either('=', Token::EqualEqual, Token::Equal)),
			'!' => Some(self.either('=', Token::BangEqual, Token::Bang)),
			'<' => Some(self.either('=', Token::LessEqual, Token::Less)),
			'>' => Some(self.either('=', Token::GreaterEqual, Token::Greater)),
			' ' | '\n' | '\t' | '\r' => None,
			'/' => {
				if self.it.consume_if(|ch| ch == '/') {
					self.it.consume_while(|ch| ch != '\n');
					None
				} else {
					Some(Token::Slash)
				}
			}
			'"' => {
				let string: String = self.it.consume_while(|ch| ch != '"').into_iter().collect();
				// Skip last "
				match self.it.next() {
					None => Some(Token::UnterminatedString),
					_ => Some(Token::String(string)),
				}
			}
			x if x.is_numeric() => self.number(x),
			x if x.is_ascii_alphabetic() || x == '_' => self.identifier(x),
			'.' => Some(Token::Dot),
			'(' => Some(Token::LeftParen),
			')' => Some(Token::RightParen),
			'{' => Some(Token::LeftBrace),
			'}' => Some(Token::RightBrace),
			'[' => Some(Token::LeftBracket),
			']' => Some(Token::RightBracket),
			',' => Some(Token::Comma),
			'-' => Some(Token::Minus),
			'+' => Some(Token::Plus),
			';' => Some(Token::Semicolon),
			'*' => Some(Token::Star),
			c => Some(Token::Unknown(c)),
		}
	}

	fn either(&mut self, to_match: char, matched: Token, unmatched: Token) -> Token {
		if self.it.consume_if(|ch| ch == to_match) {
			matched
		} else {
			unmatched
		}
	}

	#[allow(clippy::unused_self)]
	fn keyword(&self, identifier: &str) -> Option<Token> {
		use std::collections::HashMap;
		let mut keywords: HashMap<&str, Token> = HashMap::new();
		keywords.insert("and", Token::And);
		keywords.insert("class", Token::Class);
		keywords.insert("else", Token::Else);
		keywords.insert("false", Token::False);
		keywords.insert("for", Token::For);
		keywords.insert("fun", Token::Fun);
		keywords.insert("if", Token::If);
		keywords.insert("nil", Token::Nil);
		keywords.insert("or", Token::Or);
		keywords.insert("print", Token::Print);
		keywords.insert("return", Token::Return);
		keywords.insert("super", Token::Super);
		keywords.insert("this", Token::This);
		keywords.insert("true", Token::True);
		keywords.insert("var", Token::Var);
		keywords.insert("while", Token::While);
		keywords.insert("import", Token::Import);

		keywords.get(identifier).cloned()
	}

	fn number(&mut self, x: char) -> Option<Token> {
		let mut number = String::new();
		number.push(x);
		let num = self
			.it
			.consume_while(char::is_numeric)
			.into_iter()
			.collect::<String>();
		number.push_str(&num);
		if self.it.peek() == Some('.') && self.it.consume_if_next(char::is_numeric) {
			let num2 = self
				.it
				.consume_while(char::is_numeric)
				.into_iter()
				.collect::<String>();

			number.push('.');
			number.push_str(&num2);
		}

		Some(Token::Number(number.parse::<f64>().unwrap()))
	}

	fn identifier(&mut self, x: char) -> Option<Token> {
		let mut identifier = String::new();
		identifier.push(x);

		let rest = self
			.it
			.consume_while(|a| a.is_ascii_alphanumeric() || a == '_')
			.into_iter()
			.collect::<String>();

		identifier.push_str(&rest);
		self.keyword(&identifier)
			.map_or(Some(Token::Identifier(identifier)), Some)
	}

	pub fn tokenize_with_context(&mut self) -> Vec<WithSpan<Token>> {
		let mut tokens = Vec::new();
		loop {
			let initial_position = self.it.current_position;
			let Some(ch) = self.it.next() else {
				break;
			};

			if let Some(token) = self.match_token(ch) {
				tokens.push(WithSpan::new(
					token,
					Span::new(initial_position, self.it.current_position),
				));
			}
		}

		tokens
	}
}

pub fn tokenize_with_context(buf: &str) -> Vec<WithSpan<Token>> {
	let mut t = Lexer::new(buf);
	t.tokenize_with_context()
}

#[cfg(test)]
mod tests {
	use super::tokenize_with_context;
	use crate::token::Token;

	fn tokenize(buf: &str) -> Vec<Token> {
		tokenize_with_context(buf)
			.iter()
			.map(|tc| tc.value.clone())
			.collect()
	}

	#[test]
	#[allow(clippy::approx_constant)]
	fn errors() {
		assert_eq!(tokenize("\"test"), [Token::UnterminatedString]);
		assert_eq!(tokenize("&"), [Token::Unknown('&')]);
		assert_eq!(tokenize("&&"), [Token::Unknown('&'), Token::Unknown('&')]);
		assert_eq!(
			tokenize("& 3.14"),
			[Token::Unknown('&'), Token::Number(3.14)]
		);
	}

	#[test]
	fn tokenize_works() {
		assert_eq!(tokenize(""), []);
		assert_eq!(tokenize("="), [Token::Equal]);
		assert_eq!(tokenize("=="), [Token::EqualEqual]);
		assert_eq!(
			tokenize("== = =="),
			[Token::EqualEqual, Token::Equal, Token::EqualEqual]
		);
		assert_eq!(tokenize("//test"), []);
		assert_eq!(tokenize("=//test"), [Token::Equal]);
		assert_eq!(
			tokenize(
				"=//test
        ="
			),
			[Token::Equal, Token::Equal]
		);
		assert_eq!(tokenize("\"test\""), [Token::String("test".to_owned())]);
		assert_eq!(tokenize("12.34"), [Token::Number(12.34)]);
		assert_eq!(tokenize("99"), [Token::Number(99.00)]);
		assert_eq!(tokenize("99."), [Token::Number(99.00), Token::Dot]);
		assert_eq!(
			tokenize("99.="),
			[Token::Number(99.00), Token::Dot, Token::Equal]
		);
		assert_eq!(tokenize("!"), [Token::Bang]);
		assert_eq!(tokenize("!="), [Token::BangEqual]);
		assert_eq!(tokenize("test"), [Token::Identifier("test".to_owned())]);
		assert_eq!(tokenize("orchid"), [Token::Identifier("orchid".to_owned())]);
		assert_eq!(tokenize("or"), [Token::Or]);
		assert_eq!(tokenize("["), [Token::LeftBracket]);
		assert_eq!(tokenize("]"), [Token::RightBracket]);
	}
}
