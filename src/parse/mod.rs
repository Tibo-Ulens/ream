#![allow(dead_code)]

use std::iter::Peekable;

use miette::{Error, SourceSpan};

use crate::{ast, Combine, Lexer, ParseError, Token, TokenType, EOF_TOKEN};

mod annotation;
mod quote;

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
				found:    token.t.name(),
				expected: vec![t.name()],
			}
			.into())
		}
	}

	/// Parse the entire input
	pub fn parse(&mut self) -> Result<ast::Program<'s>, Error> {
		// let initial_span: SourceSpan = (0, 0).into();
		let mut exprs = vec![];

		while self.peek()?.t != TokenType::EndOfFile {
			let expr = self.parse_expression()?;

			exprs.push(expr);
		}

		Ok(ast::Program(exprs))
	}

	/// Parse any expression
	fn parse_expression(&mut self) -> Result<ast::Expression<'s>, Error> {
		let token = self.next()?;

		let expression_span = token.span;

		match token.t {
			TokenType::Identifier(_) => Ok(ast::Expression::Identifier(token.into())),
			TokenType::Boolean(_) => Ok(ast::Expression::Literal(token.into())),
			TokenType::Integer(_) => Ok(ast::Expression::Literal(token.into())),
			TokenType::Float(_) => Ok(ast::Expression::Literal(token.into())),
			TokenType::Character(_) => Ok(ast::Expression::Literal(token.into())),
			TokenType::String(_) => Ok(ast::Expression::Literal(token.into())),
			TokenType::Atom(_) => Ok(ast::Expression::Literal(token.into())),

			TokenType::Backtick => Ok(self.parse_shorthand_quote(expression_span)?.into()),

			TokenType::LeftParen => self.parse_parenthesized_expression(expression_span),

			// EndOfFile is unreachable as it's filtered out in the loop in `self.parse()`
			TokenType::EndOfFile => unreachable!(),

			tt => {
				Err(ParseError::InvalidExpression {
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

		let expression_span = initial_span.combine(&token.span);

		match token.t {
			TokenType::Atom(annotation_type) => {
				Ok(self.parse_annotation(expression_span, annotation_type)?.into())
			},

			TokenType::KwQuote => Ok(self.parse_quote(expression_span)?.into()),
			TokenType::KwLet => Ok(self.parse_definition(expression_span)?),
			TokenType::KwBegin => Ok(self.parse_sequence(expression_span)?),
			TokenType::KwLambda => Ok(self.parse_lambda(expression_span)?),
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

	/// Parse a definition of the form `(let <identifier> <expression>)`
	///
	/// `(` and `let` already consumed
	fn parse_definition(&mut self, initial_span: SourceSpan) -> Result<ast::Expression<'s>, Error> {
		let target_token = self.expect(TokenType::Identifier(""))?;
		let mut definition_span = initial_span.combine(&target_token.span);

		let value = self.parse_expression()?;
		definition_span = definition_span.combine(&self.prev_span);

		let right_paren = self.expect(TokenType::RightParen)?;
		definition_span = definition_span.combine(&right_paren.span);

		Ok(ast::Expression::Definition {
			span:   definition_span,
			target: target_token.into(),
			value:  Box::new(value),
		})
	}

	/// Parse a sequence of the form `(begin <expression>+)`
	///
	/// `(` and `begin` already consumed
	fn parse_sequence(&mut self, initial_span: SourceSpan) -> Result<ast::Expression<'s>, Error> {
		let mut exprs = vec![self.parse_expression()?];
		let mut sequence_span = initial_span.combine(&self.prev_span);

		while self.peek()?.t != TokenType::RightParen {
			let expr = self.parse_expression()?;
			exprs.push(expr);
			sequence_span = sequence_span.combine(&self.prev_span);
		}

		// Unwrap is safe as RightParen is selected for in the loop
		let right_paren = self.expect(TokenType::RightParen).unwrap();
		sequence_span = sequence_span.combine(&right_paren.span);

		Ok(ast::Expression::Sequence { span: sequence_span, seq: exprs })
	}

	/// Parse a lambda of the form `(lambda <formals> <body>)`
	/// where formals is `<identifier>` or `(<identifier>*)`
	/// and body is `<expression>+`
	///
	/// `(` and `lambda` already consumed
	fn parse_lambda(&mut self, initial_span: SourceSpan) -> Result<ast::Expression<'s>, Error> {
		let mut formals = vec![];
		let next_token = self.next()?;
		let mut lambda_span = initial_span.combine(&next_token.span);

		match next_token.t {
			TokenType::Identifier(_) => formals.push(next_token.into()),
			TokenType::LeftParen => {
				while self.peek()?.t != TokenType::RightParen {
					let formal = self.expect(TokenType::Identifier(""))?;
					lambda_span = lambda_span.combine(&formal.span);
					formals.push(formal.into());
				}

				// Unwrap is safe as RightParen is selected for in the loop
				let right_paren = self.expect(TokenType::RightParen).unwrap();
				lambda_span = lambda_span.combine(&right_paren.span);
			},
			tt => {
				return Err(ParseError::InvalidLambdaFormals {
					loc:   next_token.span,
					found: tt.to_string(),
				}
				.into());
			},
		}

		let mut body = vec![];

		while self.peek()?.t != TokenType::RightParen {
			let expr = self.parse_expression()?;
			body.push(expr);
			lambda_span = lambda_span.combine(&self.prev_span);
		}

		// Unwrap is safe as RightParen is selected for in the loop
		let right_paren = self.expect(TokenType::RightParen).unwrap();
		lambda_span = lambda_span.combine(&right_paren.span);

		Ok(ast::Expression::LambdaExpression { span: lambda_span, formals, body })
	}
}
