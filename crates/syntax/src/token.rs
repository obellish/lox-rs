use std::{
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult},
	num::ParseFloatError,
};

use logos::Logos;

#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(error = ParseTokenError)]
pub enum Token {
	#[token("(")]
	LeftParen,
	#[token(")")]
	RightParen,
	#[token("{")]
	LeftBrace,
	#[token("}")]
	RightBrace,
	#[token("[")]
	LeftBracket,
	#[token("]")]
	RightBracket,
	#[token(",")]
	Comma,
	#[token(".")]
	Dot,
	#[token("-")]
	Minus,
	#[token("+")]
	Plus,
	#[token(";")]
	Semicolon,
	#[token("/")]
	Slash,
	#[token("*")]
	Star,
	#[token("!")]
	Bang,
	#[token("!=")]
	BangEqual,
	#[token("=")]
	Equal,
	#[token("==")]
	EqualEqual,
	#[token(">")]
	Greater,
	#[token(">=")]
	GreaterEqual,
	#[token("<")]
	Less,
	#[token("<=")]
	LessEqual,
	#[regex(r"-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>())]
	Number(f64),
	#[regex("\".+\"", |lex| lex.slice().to_owned())]
	String(String),
	#[regex("[A-Za-z_][A-Za-z0-9_]+", |lex| lex.slice().to_owned())]
	Identifier(String),
	#[token("and")]
	And,
	#[token("class")]
	Class,
	#[token("else")]
	Else,
	#[token("false")]
	False,
	#[token("fun")]
	Fun,
	#[token("for")]
	For,
	#[token("if", priority = 1)]
	If,
	#[token("nil")]
	Nil,
	#[token("or", priority = 1)]
	Or,
	#[token("print")]
	Print,
	#[token("return")]
	Return,
	#[token("super")]
	Super,
	#[token("this")]
	This,
	#[token("true")]
	True,
	#[token("var")]
	Var,
	#[token("while")]
	While,
	#[token("import")]
	Import,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum ParseTokenError {
	#[default]
	Unknown,
	ParseFloat(ParseFloatError),
}

impl Display for ParseTokenError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Unknown => f.write_str("unknown parsing error"),
			Self::ParseFloat(e) => Display::fmt(&e, f),
		}
	}
}

impl StdError for ParseTokenError {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		match self {
			Self::Unknown => None,
			Self::ParseFloat(e) => Some(e),
		}
	}
}

impl From<ParseFloatError> for ParseTokenError {
	fn from(value: ParseFloatError) -> Self {
		Self::ParseFloat(value)
	}
}
