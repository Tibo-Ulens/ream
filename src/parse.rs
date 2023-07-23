#![allow(dead_code)]

use std::iter::Peekable;

use miette::{Error, SourceSpan};

use crate::{ast, Lexer, ParseError, Token, TokenType};

/// A parser for a single source file
#[allow(missing_docs)]
pub struct Parser<'s> {
	source: &'s str,
	tokens: Peekable<Lexer<'s>>,

	prev_span: SourceSpan,
}

impl<'s> Parser<'s> {
	/// Create a new [`Parser`]
	pub fn new(source: &'s str, tokens: Peekable<Lexer<'s>>) -> Self {
		Self { source, tokens, prev_span: (0, 0).into() }
	}

	/// Peek at the next [`Token`]
	///
	/// Returns [`None`] if no tokens are left
	fn peek(&mut self) -> Result<&Token<'s>, Error> {
		match self.tokens.peek() {
			Some(res) => Ok(res.as_ref().map_err(|e| e.clone())?),
			None => Err(ParseError::UnexpectedEof { loc: self.prev_span }.into()),
		}
	}

	/// Consume and return the next [`Token`]
	///
	/// Returns [`None`] if no tokens are left
	fn next(&mut self) -> Result<Token<'s>, Error> {
		let token_result = match self.tokens.next() {
			Some(t) => t,
			None => return Err(ParseError::UnexpectedEof { loc: self.prev_span }.into()),
		};

		match token_result {
			Ok(t) => {
				self.prev_span = t.span;

				Ok(t)
			},
			Err(e) => Err(e.into()),
		}
	}

	/// Consume and return the next [`Token`] if it has the given [`TokenType`]
	fn expect(&mut self, t: TokenType<'s>) -> Result<Token<'s>, Error> {
		let token = match self.peek() {
			Ok(t) => t,
			Err(e) => return Err(e),
		};

		if std::mem::discriminant(&token.t) == std::mem::discriminant(&t) {
			// Unwrap is safe as peek returned a token
			Ok(self.next().unwrap())
		} else {
			Err(ParseError::UnexpectedToken {
				loc:      token.span,
				found:    token.t.to_string(),
				expected: vec![t.to_string()],
			}
			.into())
		}
	}

	/// Parse the entire input
	pub fn parse(&mut self) -> Result<ast::Program<'s>, Error> {
		self.expect(TokenType::LeftParen)?;

		Ok(ast::Program(vec![]))
	}
}
