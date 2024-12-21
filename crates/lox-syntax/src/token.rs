use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::position::WithSpan;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	// Single-character tokens.
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	LeftBracket,
	RightBracket,
	Comma,
	Dot,
	Minus,
	Plus,
	Semicolon,
	Slash,
	Star,

	// One or two character tokens.
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

	// Literals.
	Identifier(String),
	String(String),
	Number(f64),

	// Keywords.
	And,
	Class,
	Else,
	False,
	Fun,
	For,
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
	Import,

	// Other.
	Eof,
	UnterminatedString,
	Unknown(char),
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let kind: TokenKind = self.into();
		Display::fmt(&kind, f)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
	// Single-character tokens.
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	LeftBracket,
	RightBracket,
	Comma,
	Dot,
	Minus,
	Plus,
	Semicolon,
	Slash,
	Star,

	// One or two character tokens.
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

	// Literals.
	Identifier,
	String,
	Number,

	// Keywords.
	And,
	Class,
	Else,
	False,
	Fun,
	For,
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
	Import,

	// Other.
	Eof,
	UnterminatedString,
	Unknown,
}

impl Display for TokenKind {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(
			f,
			"{}",
			match self {
				Self::LeftParen => "'('",
				Self::RightParen => "')'",
				Self::LeftBrace => "'{'",
				Self::RightBrace => "'}'",
				Self::LeftBracket => "'['",
				Self::RightBracket => "']'",
				Self::Comma => "','",
				Self::Dot => "'.'",
				Self::Minus => "'-'",
				Self::Plus => "'+'",
				Self::Semicolon => "';'",
				Self::Slash => "'/'",
				Self::Star => "'*'",
				Self::Bang => "'!'",
				Self::BangEqual => "'!='",
				Self::Equal => "'='",
				Self::EqualEqual => "'=='",
				Self::Greater => "'>'",
				Self::GreaterEqual => "'>='",
				Self::Less => "'<'",
				Self::LessEqual => "'<='",
				Self::Identifier => "identifier",
				Self::String => "string",
				Self::Number => "number",
				Self::And => "'and'",
				Self::Class => "'class'",
				Self::Else => "'else'",
				Self::False => "'false'",
				Self::Fun => "'fun'",
				Self::For => "'for'",
				Self::If => "'if'",
				Self::Nil => "nil",
				Self::Or => "'or'",
				Self::Print => "'print'",
				Self::Return => "'return'",
				Self::Super => "'super'",
				Self::This => "'this'",
				Self::True => "'true'",
				Self::Var => "'var'",
				Self::While => "'while'",
				Self::Import => "'import'",
				Self::Eof => "<EOF>",
				Self::UnterminatedString => "<Unterminated String>",
				Self::Unknown => "<Unknown>",
			}
		)
	}
}

impl From<&Token> for TokenKind {
	fn from(token: &Token) -> Self {
		match token {
			Token::LeftParen => Self::LeftParen,
			Token::RightParen => Self::RightParen,
			Token::LeftBrace => Self::LeftBrace,
			Token::RightBrace => Self::RightBrace,
			Token::LeftBracket => Self::LeftBracket,
			Token::RightBracket => Self::RightBracket,
			Token::Comma => Self::Comma,
			Token::Dot => Self::Dot,
			Token::Minus => Self::Minus,
			Token::Plus => Self::Plus,
			Token::Semicolon => Self::Semicolon,
			Token::Slash => Self::Slash,
			Token::Star => Self::Star,
			Token::Bang => Self::Bang,
			Token::BangEqual => Self::BangEqual,
			Token::Equal => Self::Equal,
			Token::EqualEqual => Self::EqualEqual,
			Token::Greater => Self::Greater,
			Token::GreaterEqual => Self::GreaterEqual,
			Token::Less => Self::Less,
			Token::LessEqual => Self::LessEqual,
			Token::Identifier(_) => Self::Identifier,
			Token::String(_) => Self::String,
			Token::Number(_) => Self::Number,
			Token::And => Self::And,
			Token::Class => Self::Class,
			Token::Else => Self::Else,
			Token::False => Self::False,
			Token::Fun => Self::Fun,
			Token::For => Self::For,
			Token::If => Self::If,
			Token::Nil => Self::Nil,
			Token::Or => Self::Or,
			Token::Print => Self::Print,
			Token::Return => Self::Return,
			Token::Super => Self::Super,
			Token::This => Self::This,
			Token::True => Self::True,
			Token::Var => Self::Var,
			Token::While => Self::While,
			Token::Import => Self::Import,
			Token::Eof => Self::Eof,
			Token::UnterminatedString => Self::UnterminatedString,
			Token::Unknown(_) => Self::Unknown,
		}
	}
}

impl From<&WithSpan<Token>> for TokenKind {
	fn from(value: &WithSpan<Token>) -> Self {
		Self::from(&value.value)
	}
}
