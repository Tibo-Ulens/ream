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
			None => {
				Ok(EOF_TOKEN.get_or_init(|| {
					Token { span: self.prev_span.increment(), t: TokenType::EndOfFile }
				}))
			},
		}
	}

	/// Consume and return the next [`Token`]
	///
	/// Returns an [`EndOfFile`](TokenType::EndOfFile) if no tokens are left
	fn next(&mut self) -> Result<Token<'s>, Error> {
		let token_result = match self.tokens.next() {
			Some(t) => t,
			None => {
				return Err(ParseError::UnexpectedEof { loc: self.prev_span.increment() }.into());
			},
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

	/// Parse any expression that starts with a `(`
	///
	/// `(` already consumed
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

			TokenType::Identifier(_) => {
				Ok(self.parse_procedure_call(expression_span, token.into())?)
			},

			TokenType::KwQuote => Ok(self.parse_quote(expression_span)?.into()),
			TokenType::KwLet => Ok(self.parse_definition(expression_span)?),
			TokenType::KwBegin => Ok(self.parse_sequence(expression_span)?),
			TokenType::KwLambda => Ok(self.parse_lambda(expression_span)?),
			TokenType::KwIf => Ok(self.parse_conditional(expression_span)?),
			TokenType::KwInclude => Ok(self.parse_inclusion(expression_span)?),

			tt => {
				Err(ParseError::UnexpectedToken {
					loc:      token.span,
					found:    tt.to_string(),
					expected: vec![
						"Atom".to_string(),
						"Keyword".to_string(),
						"Identifier".to_string(),
					],
				}
				.into())
			},
		}
	}

	/// Parse a procedure call of the form `(<operator> <operands>)`
	/// where operator is `<identifier>
	/// and operands is `<expression>*`
	///
	/// `(` and `<operator>` already consumed
	fn parse_procedure_call(
		&mut self,
		initial_span: SourceSpan,
		operator: ast::Identifier<'s>,
	) -> Result<ast::Expression<'s>, Error> {
		let mut operands = vec![];
		let mut procedure_span = initial_span;

		while self.peek()?.t != TokenType::RightParen {
			let operand = self.parse_expression()?;
			operands.push(operand);
			procedure_span = procedure_span.combine(&self.prev_span);
		}

		// Unwrap is safe as RightParen is selected for in the loop
		let right_paren = self.expect(TokenType::RightParen).unwrap();
		procedure_span = procedure_span.combine(&right_paren.span);

		Ok(ast::Expression::ProcedureCall { span: procedure_span, operator, operands })
	}

	/// Parse a definition of the form `(let <target> <value>)`
	/// where target is `<identifier>`
	/// and value is `<expression>`
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

	/// Parse a sequence of the form `(begin <sequence>)`
	/// where sequence is `<expression>+`
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

	/// Parse a conditional of the form `(if <test> <consequent> [<alternate>])`
	/// where test is `<expression>`
	/// consequent is `<expression>`
	/// and alternate is `<expression>`
	///
	/// `(` and `if` already consumed
	fn parse_conditional(
		&mut self,
		initial_span: SourceSpan,
	) -> Result<ast::Expression<'s>, Error> {
		let test = self.parse_expression()?;
		let mut conditional_span = initial_span.combine(&self.prev_span);

		let consequent = self.parse_expression()?;
		conditional_span = conditional_span.combine(&self.prev_span);

		let alternate = if self.peek()?.t == TokenType::RightParen {
			// Unwrap is safe as peek is some
			let right_paren = self.next().unwrap();
			conditional_span = conditional_span.combine(&right_paren.span);

			None
		} else {
			let expr = self.parse_expression()?;
			conditional_span = conditional_span.combine(&self.prev_span);

			let right_paren = self.expect(TokenType::RightParen)?;
			conditional_span = conditional_span.combine(&right_paren.span);

			Some(Box::new(expr))
		};

		Ok(ast::Expression::Conditional {
			span: conditional_span,
			test: Box::new(test),
			consequent: Box::new(consequent),
			alternate,
		})
	}

	/// Parse an inclusion of the form `(include <string>+)`
	///
	/// `(` and `include` already consumed
	fn parse_inclusion(&mut self, initial_span: SourceSpan) -> Result<ast::Expression<'s>, Error> {
		let first_file_token = self.expect(TokenType::String(""))?;
		let TokenType::String(first_file) = first_file_token.t else { unreachable!() };
		let mut inclusion_span = initial_span.combine(&first_file_token.span);

		let mut files = vec![first_file];

		while self.peek()?.t != TokenType::RightParen {
			let file_token = self.expect(TokenType::String(""))?;
			let TokenType::String(file) = file_token.t else { unreachable!() };
			inclusion_span = inclusion_span.combine(&file_token.span);

			files.push(file);
		}

		// Unwrap is safe as RightParen is selected for in the loop
		let right_paren = self.expect(TokenType::RightParen).unwrap();
		inclusion_span = inclusion_span.combine(&right_paren.span);

		Ok(ast::Expression::Inclusion { span: inclusion_span, files })
	}
}
