use super::{
	ast::Identifier,
	parser::Parser,
	position::WithSpan,
	token::{Token, TokenKind},
};

pub fn expect_identifier(p: &mut Parser<'_>) -> Option<WithSpan<Identifier>> {
	let token = &p.advance();

	if let Token::Identifier(ident) = &token.value {
		Some(WithSpan::new(ident.clone(), token.span))
	} else {
		p.error(
			format!("Expected {} got {}", TokenKind::Identifier, token.value),
			token.span,
		);
		None
	}
}

pub fn expect_string(p: &mut Parser<'_>) -> Option<WithSpan<String>> {
	let token = p.advance();
	if let Token::String(ident) = &token.value {
		Some(WithSpan::new(ident.clone(), token.span))
	} else {
		p.error(
			format!("Expected {} got {}", TokenKind::String, token.value),
			token.span,
		);
		None
	}
}
