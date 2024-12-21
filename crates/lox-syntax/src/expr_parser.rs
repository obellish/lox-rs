use super::{
	ast::{BinaryOperator, Expr, LogicalOperator, UnaryOperator},
	common::expect_identifier,
	parser::Parser,
	position::{Span, WithSpan},
	token::{Token, TokenKind},
};

type ExprWithSpan = WithSpan<Expr>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
	#[default]
	None,
	Assign, // =
	Or,
	And,
	Equality,   // == !=
	Comparison, // < <= > >=
	Term,       // + -
	Factor,     // * /
	Unary,      // ! -
	Call,       // ()
	List,       // []
	Primary,
}

impl From<TokenKind> for Precedence {
	fn from(value: TokenKind) -> Self {
		match value {
			TokenKind::Equal => Self::Assign,
			TokenKind::Or => Self::Or,
			TokenKind::And => Self::And,
			TokenKind::BangEqual | TokenKind::EqualEqual => Self::Equality,
			TokenKind::Less
			| TokenKind::LessEqual
			| TokenKind::Greater
			| TokenKind::GreaterEqual => Self::Comparison,
			TokenKind::Plus | TokenKind::Minus => Self::Term,
			TokenKind::Star | TokenKind::Slash => Self::Factor,
			TokenKind::Bang => Self::Unary,
			TokenKind::LeftParen | TokenKind::Dot => Self::Call,
			TokenKind::LeftBracket => Self::List,
			_ => Self::None,
		}
	}
}

fn parse_expr(it: &mut Parser<'_>, precedence: Precedence) -> Option<ExprWithSpan> {
	let mut expr = parse_prefix(it)?;
	while !it.is_eof() {
		let next_precedence = Precedence::from(it.peek());
		if precedence >= next_precedence {
			break;
		}

		expr = parse_infix(it, expr)?;
	}

	Some(expr)
}

fn parse_infix(it: &mut Parser<'_>, left: ExprWithSpan) -> Option<ExprWithSpan> {
	match it.peek() {
		TokenKind::BangEqual
		| TokenKind::EqualEqual
		| TokenKind::LessEqual
		| TokenKind::Less
		| TokenKind::Greater
		| TokenKind::GreaterEqual
		| TokenKind::Plus
		| TokenKind::Minus
		| TokenKind::Star
		| TokenKind::Slash => parse_binary(it, left),
		TokenKind::Or | TokenKind::And => parse_logical(it, left),
		TokenKind::Equal => parse_assign(it, left),
		TokenKind::LeftParen => parse_call(it, left),
		TokenKind::LeftBracket => parse_list_get(it, left),
		TokenKind::Dot => parse_get(it, left),
		_ => {
			it.error(
				format!("Unexpected {}", it.peek_token().value),
				it.peek_token().span,
			);
			None
		}
	}
}

fn parse_prefix(it: &mut Parser<'_>) -> Option<ExprWithSpan> {
	match it.peek() {
		TokenKind::Number
		| TokenKind::Nil
		| TokenKind::This
		| TokenKind::True
		| TokenKind::False
		| TokenKind::Identifier
		| TokenKind::Super
		| TokenKind::String => parse_primary(it),
		TokenKind::Bang | TokenKind::Minus => parse_unary(it),
		TokenKind::LeftParen => parse_grouping(it),
		TokenKind::LeftBracket => parse_list(it),
		_ => {
			it.error(
				format!("Unexpected {}", it.peek_token().value),
				it.peek_token().span,
			);
			None
		}
	}
}

fn parse_list_get(it: &mut Parser<'_>, left: ExprWithSpan) -> Option<ExprWithSpan> {
	it.expect(TokenKind::LeftBracket)?;
	let right = parse_expr(it, Precedence::None)?;
	let end = it.expect(TokenKind::RightBracket)?;
	let span = Span::union(&left, end);

	Some(WithSpan::new(
		Expr::ListGet(Box::new(left), Box::new(right)),
		span,
	))
}

fn parse_get(it: &mut Parser<'_>, left: ExprWithSpan) -> Option<ExprWithSpan> {
	it.expect(TokenKind::Dot)?;
	let tc = it.advance();
	if let Token::Identifier(ref i) = &tc.value {
		let span = Span::union(&left, tc);
		Some(WithSpan::new(
			Expr::Get(Box::new(left), WithSpan::new(i.clone(), tc.span)),
			span,
		))
	} else {
		it.error(format!("Expected identifier got {}", tc.value), tc.span);
		None
	}
}

fn parse_call(it: &mut Parser<'_>, left: ExprWithSpan) -> Option<ExprWithSpan> {
	it.expect(TokenKind::LeftParen)?;
	let args = parse_arguments(it)?;
	let most_right = it.expect(TokenKind::RightParen)?;
	let span = Span::union(&left, most_right);
	Some(WithSpan::new(Expr::Call(Box::new(left), args), span))
}

fn parse_arguments(it: &mut Parser<'_>) -> Option<Vec<ExprWithSpan>> {
	let mut args = Vec::new();
	if !it.check(TokenKind::RightParen) {
		args.push(parse_expr(it, Precedence::None)?);
		while it.check(TokenKind::Comma) {
			it.expect(TokenKind::Comma)?;
			args.push(parse_expr(it, Precedence::None)?);
		}
	}

	Some(args)
}

fn parse_assign(it: &mut Parser<'_>, left: ExprWithSpan) -> Option<ExprWithSpan> {
	it.expect(TokenKind::Equal)?;
	let right = parse_expr(it, Precedence::None)?;
	let span = Span::union(&left, &right);
	match &left.value {
		Expr::Variable(i) => Some(WithSpan::new(
			Expr::Assign(i.clone(), Box::new(right)),
			span,
		)),
		Expr::Get(l, i) => Some(WithSpan::new(
			Expr::Set(l.clone(), i.clone(), Box::new(right)),
			span,
		)),
		Expr::ListGet(l, i) => Some(WithSpan::new(
			Expr::ListSet(l.clone(), i.clone(), Box::new(right)),
			span,
		)),
		_ => {
			it.error("Invalid left value".to_owned(), left.span);
			None
		}
	}
}

fn parse_logical(it: &mut Parser<'_>, left: ExprWithSpan) -> Option<ExprWithSpan> {
	let precedence = Precedence::from(it.peek());
	let operator = parse_logical_op(it)?;
	let right = parse_expr(it, precedence)?;
	let span = Span::union(&left, &right);
	Some(WithSpan::new(
		Expr::Logical(Box::new(left), operator, Box::new(right)),
		span,
	))
}

fn parse_list_items(it: &mut Parser<'_>) -> Option<Vec<ExprWithSpan>> {
	let mut args = Vec::new();
	if !it.check(TokenKind::RightBracket) {
		args.push(parse_expr(it, Precedence::None)?);
		while it.check(TokenKind::Comma) {
			it.expect(TokenKind::Comma)?;
			args.push(parse_expr(it, Precedence::None)?);
		}
	}
	Some(args)
}

fn parse_list(it: &mut Parser<'_>) -> Option<ExprWithSpan> {
	let left_bracket = it.expect(TokenKind::LeftBracket)?;
	let items = parse_list_items(it)?;
	let right_bracket = it.expect(TokenKind::RightBracket)?;

	let span = Span::union(left_bracket, right_bracket);
	Some(WithSpan::new(Expr::List(items), span))
}

fn parse_grouping(it: &mut Parser<'_>) -> Option<ExprWithSpan> {
	let left_paren = it.expect(TokenKind::LeftParen)?;
	let expr = parse_expr(it, Precedence::None)?;
	let right_paren = it.expect(TokenKind::RightParen)?;

	let span = Span::union(left_paren, right_paren);
	Some(WithSpan::new(Expr::Grouping(Box::new(expr)), span))
}

fn parse_binary(it: &mut Parser<'_>, left: ExprWithSpan) -> Option<ExprWithSpan> {
	let precedence = Precedence::from(it.peek());
	let operator = parse_binary_op(it)?;
	let right = parse_expr(it, precedence)?;
	let span = Span::union(&left, &right);
	Some(WithSpan::new(
		Expr::Binary(Box::new(left), operator, Box::new(right)),
		span,
	))
}

fn parse_unary(it: &mut Parser<'_>) -> Option<ExprWithSpan> {
	let operator = parse_unary_op(it)?;
	let right = parse_expr(it, Precedence::Unary)?;
	let span = Span::union(&operator, &right);

	Some(WithSpan::new(Expr::Unary(operator, Box::new(right)), span))
}

fn parse_logical_op(it: &mut Parser<'_>) -> Option<WithSpan<LogicalOperator>> {
	let tc = it.advance();
	match tc.value {
		Token::And => Some(WithSpan::new(LogicalOperator::And, tc.span)),
		Token::Or => Some(WithSpan::new(LogicalOperator::Or, tc.span)),
		_ => {
			it.error(
				format!("Expected logical operator got {}", tc.value),
				tc.span,
			);
			None
		}
	}
}

fn parse_unary_op(it: &mut Parser<'_>) -> Option<WithSpan<UnaryOperator>> {
	let tc = it.advance();
	match tc.value {
		Token::Bang => Some(WithSpan::new(UnaryOperator::Bang, tc.span)),
		Token::Minus => Some(WithSpan::new(UnaryOperator::Minus, tc.span)),
		_ => {
			it.error(format!("Expected unary operator got {}", tc.value), tc.span);
			None
		}
	}
}

fn parse_binary_op(it: &mut Parser<'_>) -> Option<WithSpan<BinaryOperator>> {
	let tc = it.advance();
	match tc.value {
		Token::BangEqual => Some(WithSpan::new(BinaryOperator::BangEqual, tc.span)),
		Token::EqualEqual => Some(WithSpan::new(BinaryOperator::EqualEqual, tc.span)),
		Token::Less => Some(WithSpan::new(BinaryOperator::Less, tc.span)),
		Token::LessEqual => Some(WithSpan::new(BinaryOperator::LessEqual, tc.span)),
		Token::Greater => Some(WithSpan::new(BinaryOperator::Greater, tc.span)),
		Token::GreaterEqual => Some(WithSpan::new(BinaryOperator::GreaterEqual, tc.span)),
		Token::Plus => Some(WithSpan::new(BinaryOperator::Plus, tc.span)),
		Token::Minus => Some(WithSpan::new(BinaryOperator::Minus, tc.span)),
		Token::Star => Some(WithSpan::new(BinaryOperator::Star, tc.span)),
		Token::Slash => Some(WithSpan::new(BinaryOperator::Slash, tc.span)),
		_ => {
			it.error(
				format!("Expected binary operator got {}", tc.value),
				tc.span,
			);
			None
		}
	}
}

fn parse_primary(it: &mut Parser<'_>) -> Option<ExprWithSpan> {
	let tc = it.advance();
	match tc.value {
		Token::Nil => Some(WithSpan::new(Expr::Nil, tc.span)),
		Token::This => Some(WithSpan::new(Expr::This, tc.span)),
		Token::Number(n) => Some(WithSpan::new(Expr::Number(n), tc.span)),
		Token::True => Some(WithSpan::new(Expr::Boolean(true), tc.span)),
		Token::False => Some(WithSpan::new(Expr::Boolean(false), tc.span)),
		Token::String(ref s) => Some(WithSpan::new(Expr::String(s.clone()), tc.span)),
		Token::Identifier(ref s) => Some(WithSpan::new(
			Expr::Variable(WithSpan::new(s.clone(), tc.span)),
			tc.span,
		)),
		Token::Super => parse_super(it, tc),
		_ => {
			it.error(format!("Expected primary got {}", tc.value), tc.span);
			None
		}
	}
}

fn parse_super(it: &mut Parser<'_>, keyword: &WithSpan<Token>) -> Option<ExprWithSpan> {
	it.expect(TokenKind::Dot)?;
	let name = expect_identifier(it)?;
	let span = Span::union(keyword, &name);
	Some(WithSpan::new(Expr::Super(name), span))
}

pub fn parse(it: &mut Parser<'_>) -> Option<ExprWithSpan> {
	parse_expr(it, Precedence::None)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		position::{Diagnostic, Span},
		tokenizer::tokenize_with_context,
	};

	#[allow(clippy::undocumented_unsafe_blocks)]
	mod make {
		use std::ops::Range;

		use super::*;
		use crate::ast::Identifier;

		pub const fn ws<T>(value: T, range: Range<u32>) -> WithSpan<T> {
			unsafe { WithSpan::new_unchecked(value, range.start, range.end) }
		}

		pub const fn n(value: f64) -> Expr {
			Expr::Number(value)
		}

		pub const fn wsn(value: f64, range: Range<u32>) -> ExprWithSpan {
			ws(n(value), range)
		}

		#[allow(clippy::range_plus_one)]
		pub fn wsmn(value: f64, range: Range<u32>) -> ExprWithSpan {
			ws(
				Expr::Unary(
					ws(UnaryOperator::Minus, range.start..range.start + 1),
					Box::new(ws(n(value), range.start + 1..range.end)),
				),
				range,
			)
		}

		pub const fn s(value: String) -> Expr {
			Expr::String(value)
		}

		pub const fn v(value: String, range: Range<u32>) -> Expr {
			Expr::Variable(ws(value, range))
		}

		pub const fn wsb(value: bool, range: Range<u32>) -> ExprWithSpan {
			ws(Expr::Boolean(value), range)
		}

		pub fn uo(
			operator: UnaryOperator,
			operator_range: Range<u32>,
			expr: Expr,
			expr_range: Range<u32>,
		) -> Expr {
			Expr::Unary(ws(operator, operator_range), Box::new(ws(expr, expr_range)))
		}

		pub fn wsbo(
			left: ExprWithSpan,
			op: WithSpan<BinaryOperator>,
			right: ExprWithSpan,
		) -> ExprWithSpan {
			let span = Span::union(&left, &right);
			WithSpan::new(Expr::Binary(Box::new(left), op, Box::new(right)), span)
		}

		pub fn wslo(
			left: ExprWithSpan,
			op: WithSpan<LogicalOperator>,
			right: ExprWithSpan,
		) -> ExprWithSpan {
			let span = Span::union(&left, &right);
			WithSpan::new(Expr::Logical(Box::new(left), op, Box::new(right)), span)
		}

		pub fn wsg(expr: ExprWithSpan, range: Range<u32>) -> ExprWithSpan {
			ws(Expr::Grouping(Box::new(expr)), range)
		}

		pub fn wsa(left: WithSpan<Identifier>, right: ExprWithSpan) -> ExprWithSpan {
			let span = Span::union(&left, &right);
			WithSpan::new(Expr::Assign(left, Box::new(right)), span)
		}

		pub fn wsi(value: &str, range: Range<u32>) -> WithSpan<Identifier> {
			ws(value.into(), range)
		}

		pub fn wscall(
			left: ExprWithSpan,
			args: Vec<ExprWithSpan>,
			range: Range<u32>,
		) -> ExprWithSpan {
			ws(Expr::Call(Box::new(left), args), range)
		}

		pub fn wsget(left: ExprWithSpan, right: WithSpan<Identifier>) -> ExprWithSpan {
			let span = Span::union(&left, &right);
			WithSpan::new(Expr::Get(Box::new(left), right), span)
		}

		pub fn wsset(
			left: ExprWithSpan,
			right: WithSpan<Identifier>,
			set: ExprWithSpan,
		) -> ExprWithSpan {
			let span = Span::union(&left, &set);
			WithSpan::new(Expr::Set(Box::new(left), right, Box::new(set)), span)
		}
	}

	mod help {
		use std::ops::Range;

		use super::*;

		pub fn assert(expr: &str, expected: ExprWithSpan) {
			assert_eq!(parse_str(expr), Ok(expected));
		}

		pub fn assert2(expr: &str, expected: Expr, range: Range<u32>) {
			use super::make::ws;
			assert_eq!(parse_str(expr), Ok(ws(expected, range)));
		}

		#[allow(clippy::range_plus_one)]
		pub fn simple_binary2(op: BinaryOperator, op_len: u32, start: u32) -> Expr {
			use super::make::*;

			let left = ws(n(1.0), start..1 + start);
			let op = ws(op, 1 + start..1 + start + op_len);
			let right = ws(n(2.0), 1 + start + op_len..2 + start + op_len);

			Expr::Binary(Box::new(left), op, Box::new(right))
		}

		pub fn simple_binary(op: BinaryOperator, op_len: u32) -> Expr {
			simple_binary2(op, op_len, 0)
		}
	}

	fn parse_str(data: &str) -> Result<ExprWithSpan, Vec<Diagnostic>> {
		let tokens = tokenize_with_context(data);
		let mut parser = Parser::new(&tokens);
		parse(&mut parser).map_or_else(|| Err(parser.diagnostics().to_owned()), Ok)
	}

	fn assert_errors(data: &str, errors: &[&str]) {
		let x = parse_str(data);
		assert!(x.is_err());
		let diagnostics = x.unwrap_err();
		for diag in diagnostics {
			assert!(errors.contains(&diag.message.as_str()), "{}", diag.message);
		}
	}

	#[test]
	fn primary() {
		use help::assert;
		use make::*;
		assert("nil", ws(Expr::Nil, 0..3));
		assert("1.0", ws(n(1.0), 0..3));
		assert("1", ws(n(1.0), 0..1));
		assert("true", ws(Expr::Boolean(true), 0..4));
		assert("false", ws(Expr::Boolean(false), 0..5));
		assert("\"iets\"", ws(s("iets".to_owned()), 0..6));
		assert("iets", ws(v("iets".to_owned(), 0..4), 0..4));
		assert("this", ws(Expr::This, 0..4));
		assert(
			"super.iets",
			ws(Expr::Super(ws("iets".into(), 6..10)), 0..10),
		);
	}

	#[test]
	fn unary() {
		use help::assert2;
		use make::*;
		assert2(
			"-nil",
			uo(UnaryOperator::Minus, 0..1, Expr::Nil, 1..4),
			0..4,
		);
		assert2("!nil", uo(UnaryOperator::Bang, 0..1, Expr::Nil, 1..4), 0..4);
		assert2(
			"!!nil",
			uo(
				UnaryOperator::Bang,
				0..1,
				uo(UnaryOperator::Bang, 1..2, Expr::Nil, 2..5),
				1..5,
			),
			0..5,
		);
		assert2(
			"!-nil",
			uo(
				UnaryOperator::Bang,
				0..1,
				uo(UnaryOperator::Minus, 1..2, Expr::Nil, 2..5),
				1..5,
			),
			0..5,
		);
		assert2(
			"-!nil",
			uo(
				UnaryOperator::Minus,
				0..1,
				uo(UnaryOperator::Bang, 1..2, Expr::Nil, 2..5),
				1..5,
			),
			0..5,
		);
	}

	#[test]
	fn binary() {
		use help::{assert2, simple_binary};
		assert2("1+2", simple_binary(BinaryOperator::Plus, 1), 0..3);
		assert2("1-2", simple_binary(BinaryOperator::Minus, 1), 0..3);
		assert2("1>2", simple_binary(BinaryOperator::Greater, 1), 0..3);
		assert2("1<2", simple_binary(BinaryOperator::Less, 1), 0..3);
		assert2("1*2", simple_binary(BinaryOperator::Star, 1), 0..3);
		assert2("1/2", simple_binary(BinaryOperator::Slash, 1), 0..3);

		assert2("1!=2", simple_binary(BinaryOperator::BangEqual, 2), 0..4);
		assert2("1==2", simple_binary(BinaryOperator::EqualEqual, 2), 0..4);
		assert2("1>=2", simple_binary(BinaryOperator::GreaterEqual, 2), 0..4);
		assert2("1<=2", simple_binary(BinaryOperator::LessEqual, 2), 0..4);
	}

	#[test]
	fn binary_precedence() {
		use help::assert;
		use make::*;

		let expr = wsbo(
			wsbo(wsn(1., 0..1), ws(BinaryOperator::Star, 1..2), wsn(2., 2..3)),
			ws(BinaryOperator::Plus, 3..4),
			wsbo(wsn(3., 4..5), ws(BinaryOperator::Star, 5..6), wsn(4., 6..7)),
		);
		assert("1*2+3*4", expr);

		let expr = wsbo(
			wsmn(1., 0..2),
			ws(BinaryOperator::Star, 2..3),
			wsmn(2., 3..5),
		);
		assert("-1*-2", expr);
	}

	#[test]
	fn errors() {
		use help::{assert2, simple_binary};

		// Test infinite loops and extra tokens
		assert2("1+2 3", simple_binary(BinaryOperator::Plus, 1), 0..3);

		// assert!(matches!(parse_str("1+"), Err(SyntaxError::Unexpected(_))));
		assert_errors("1+", &["Unexpected <EOF>"]);
	}

	#[test]
	fn grouping() {
		use help::assert;
		use make::*;

		let expr = wsg(wsn(1., 1..2), 0..3);
		assert("(1)", expr);

		let expr = wsg(
			wsbo(wsn(1., 1..2), ws(BinaryOperator::Plus, 2..3), wsn(2., 3..4)),
			0..5,
		);
		assert("(1+2)", expr);

		assert_errors("(1", &["Expected ')' got <EOF>"]);
		assert_errors("(1}", &["Expected ')' got '}'"]);
	}

	#[test]
	fn logical() {
		use help::assert;
		use make::*;

		let expr = wslo(
			wsb(true, 0..4),
			ws(LogicalOperator::Or, 5..7),
			wsb(false, 8..13),
		);
		assert("true or false", expr);

		let expr = wslo(
			wsb(true, 0..4),
			ws(LogicalOperator::And, 5..8),
			wsb(false, 9..14),
		);
		assert("true and false", expr);
	}

	#[test]
	fn logical_precedence() {
		use help::assert;
		use make::*;

		let left = wslo(wsn(1., 0..1), ws(LogicalOperator::And, 2..5), wsn(2., 6..7));
		let right = wslo(
			wsn(3., 11..12),
			ws(LogicalOperator::And, 13..16),
			wsn(4., 17..18),
		);
		let expr = wslo(left, ws(LogicalOperator::Or, 8..10), right);
		assert("1 and 2 or 3 and 4", expr);
	}

	#[test]
	fn assignment() {
		use help::{assert, simple_binary2};
		use make::*;

		let expr = wsa(wsi("a", 0..1), wsn(3., 2..3));
		assert("a=3", expr);
		let expr = wsa(wsi("a", 0..1), wsa(wsi("b", 2..3), wsn(3., 4..5)));
		assert("a=b=3", expr);
		let expr = wsa(
			wsi("a", 0..1),
			ws(simple_binary2(BinaryOperator::Plus, 1, 2), 2..5),
		);
		assert("a=1+2", expr);

		assert_errors("a=", &["Unexpected <EOF>"]);
		assert_errors("3=3", &["Invalid left value"]);
	}

	#[test]
	fn call() {
		use help::assert;
		use make::*;

		let expr = wscall(ws(v("a".to_owned(), 0..1), 0..1), vec![], 0..3);
		assert("a()", expr);

		let expr = wscall(ws(v("a".to_owned(), 0..1), 0..1), vec![wsn(3., 2..3)], 0..4);
		assert("a(3)", expr);

		let expr = wscall(
			ws(v("a".to_owned(), 0..1), 0..1),
			vec![wsn(3., 2..3), wsn(4., 4..5)],
			0..6,
		);
		assert("a(3,4)", expr);

		let expr = wscall(ws(v("a".to_owned(), 1..2), 1..2), vec![], 1..4);
		let expr = ws(
			Expr::Unary(ws(UnaryOperator::Minus, 0..1), Box::new(expr)),
			0..4,
		);
		assert("-a()", expr);

		let left = wscall(ws(v("a".to_owned(), 0..1), 0..1), vec![], 0..3);
		let right = wscall(ws(v("b".to_owned(), 4..5), 4..5), vec![], 4..7);
		let expr = wsbo(left, ws(BinaryOperator::Plus, 3..4), right);
		assert("a()+b()", expr);

		assert_errors("a(3,)", &["Unexpected ')'"]);
	}

	#[test]
	fn get() {
		use help::assert;
		use make::*;

		let left = wsget(ws(v("a".to_owned(), 0..1), 0..1), wsi("b", 2..3));
		let expr = wsget(left, wsi("c", 4..5));
		assert("a.b.c", expr);
	}

	#[test]
	fn set() {
		use help::assert;
		use make::*;

		let expr = wsset(
			ws(v("a".to_owned(), 0..1), 0..1),
			wsi("b", 2..3),
			wsn(3., 4..5),
		);
		assert("a.b=3", expr);
	}

	#[test]
	fn list() {
		use help::assert;
		use make::*;

		let expr = ws(Expr::List(Vec::new()), 0..2);
		assert("[]", expr);

		let num = ws(n(1.0), 1..2);
		let nil = ws(Expr::Nil, 4..7);
		let expr = ws(Expr::List(vec![num, nil]), 0..8);
		assert("[1, nil]", expr);

		let left = ws(v("x".to_owned(), 0..1), 0..1);
		let right = ws(n(0.0), 2..3);
		let expr = ws(Expr::ListGet(Box::new(left), Box::new(right)), 0..4);
		assert("x[0]", expr);

		let left = ws(v("x".to_owned(), 0..1), 0..1);
		let right = ws(n(0.0), 2..3);
		let value = ws(n(1.0), 5..6);
		let expr = ws(
			Expr::ListSet(Box::new(left), Box::new(right), Box::new(value)),
			0..6,
		);
		assert("x[0]=1", expr);
	}
}
