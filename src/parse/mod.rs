#![allow(dead_code)]

use std::iter::Peekable;

use miette::{Error, SourceSpan};

use crate::{ast, Combine, Lexer, ParseError, Token, TokenType, EOF_TOKEN};

mod annotation;

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
	/// Returns an [`EndOfFile`](TokenType::EndOfFile) if no tokens are left
	fn peek(&mut self) -> Result<&Token<'s>, Error> {
		match self.tokens.peek() {
			Some(res) => Ok(res.as_ref().map_err(|e| e.clone())?),
			None => Ok(&EOF_TOKEN),
		}
	}

	/// Consume and return the next [`Token`]
	///
	/// Returns an [`EndOfFile`](TokenType::EndOfFile) if no tokens are left
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
		let initial_span: SourceSpan = (0, 0).into();
		let mut exprs = vec![];

		while self.peek()?.t != TokenType::EndOfFile {
			let expr = self.parse_expression(initial_span)?;

			exprs.push(expr);
		}

		Ok(ast::Program(exprs))
	}

	/// Parse any expression
	fn parse_expression(&mut self, initial_span: SourceSpan) -> Result<ast::Expression<'s>, Error> {
		let token = self.next()?;

		let new_span = initial_span.combine(&token.span);

		match token.t {
			TokenType::Identifier(_) => Ok(ast::Expression::Identifier(token.into())),
			TokenType::Boolean(_) => Ok(ast::Expression::Literal(token.into())),
			TokenType::Integer(_) => Ok(ast::Expression::Literal(token.into())),
			TokenType::Float(_) => Ok(ast::Expression::Literal(token.into())),
			TokenType::Character(_) => Ok(ast::Expression::Literal(token.into())),
			TokenType::String(_) => Ok(ast::Expression::Literal(token.into())),
			TokenType::Atom(_) => Ok(ast::Expression::Literal(token.into())),

			TokenType::LeftParen => self.parse_parenthesized_expression(new_span),

			// EndOfFile is unreachable as it's filtered out in the loop in `self.parse()`
			TokenType::EndOfFile => unreachable!(),

			tt => {
				Err(ParseError::UnexpectedToken {
					loc:      token.span,
					found:    tt.to_string(),
					expected: vec![
						"Identifier".to_string(),
						"Boolean".to_string(),
						"Integer".to_string(),
						"Float".to_string(),
						"Character".to_string(),
						"String".to_string(),
						"Atom".to_string(),
						"(".to_string(),
					],
				}
				.into())
			},
		}
	}

	/// Parse any expression that starts with an opening parenthesis
	fn parse_parenthesized_expression(
		&mut self,
		initial_span: SourceSpan,
	) -> Result<ast::Expression<'s>, Error> {
		let token = self.next()?;

		let new_span = initial_span.combine(&token.span);

		match token.t {
			TokenType::Atom(annotation_type) => {
				let annotation = self.parse_annotation(new_span, annotation_type)?;

				Ok(ast::Expression::Annotation(annotation))
			},

			TokenType::Backtick => todo!(),
			TokenType::KwQuote => todo!(),
			TokenType::KwLet => todo!(),
			TokenType::KwBegin => todo!(),
			TokenType::KwLambda => todo!(),
			TokenType::KwIf => todo!(),
			TokenType::KwInclude => todo!(),

			tt => {
				let token = self.next().unwrap();
				Err(ParseError::UnexpectedToken {
					loc:      token.span,
					found:    tt.to_string(),
					expected: vec!["Atom".to_string(), "Keyword".to_string()],
				}
				.into())
			},
		}
	}
}
