use crate::error::{Error, ParseError};
use crate::lex::{Token, TokenType};

mod expression;

pub(crate) use expression::*;

/// The root node of the type-checked AST
#[derive(Clone)]
pub(crate) struct Root {
	pub(crate) exprs: Vec<Expression>,
}

pub(crate) struct Parser {
	tokens:    Vec<Token>,
	token_len: usize,
	idx:       usize,
}

impl Parser {
	pub(crate) fn new(tokens: Vec<Token>) -> Self {
		let len = tokens.len();
		Self { tokens, token_len: len, idx: 0 }
	}

	pub(crate) fn parse(&mut self) -> Result<Root, Error> {
		let mut exprs = vec![];

		while self.idx < self.token_len {
			let expr = self.try_expression()?;
			exprs.push(expr);
		}

		Ok(Root { exprs })
	}

	/// Return the value of the current token
	fn peek(&self) -> Token { self.tokens[self.idx].clone() }

	/// Advance one token and return its value
	fn next(&mut self) -> Token {
		let next = self.tokens[self.idx].clone();
		self.idx += 1;
		next
	}

	/// Try to match an expression
	fn try_expression(&mut self) -> Result<Expression, Error> {
		let token = self.next();
		match token.ty {
			TokenType::Identifier(i) => Ok(Expression::Identifier(IdentifierExpr(i))),
			TokenType::Bool(b) => Ok(Expression::Literal(LiteralExpr::Bool(b))),
			TokenType::Number(n) => Ok(Expression::Literal(LiteralExpr::Number(n))),
			TokenType::String(s) => Ok(Expression::Literal(LiteralExpr::String(s))),
			TokenType::Nil => Ok(Expression::Literal(LiteralExpr::Nil)),
			TokenType::LeftParenthesis => {
				match self.peek().ty {
					TokenType::QuoteKW => {
						let expr = Expression::Literal(self.try_quote()?);

						// Match closing paren
						self.try_right_paren()?;

						Ok(expr)
					},
					TokenType::BeginKW => Ok(Expression::Sequence(self.try_sequence()?)),
					TokenType::LambdaKW => Ok(Expression::Lambda(self.try_lambda()?)),
					TokenType::IfKW => Ok(Expression::If(self.try_if()?)),
					TokenType::DefineKW => Ok(Expression::Define(self.try_define()?)),
					TokenType::SetKW => Ok(Expression::Assign(self.try_assign()?)),
					TokenType::Identifier(_) => Ok(Expression::Call(self.try_call()?)),
					_ => Err(ParseError::UnexpectedToken(token).into()),
				}
			},
			_ => Err(ParseError::UnexpectedToken(token).into()),
		}
	}

	fn try_quote(&mut self) -> Result<LiteralExpr, Error> {
		// Skip the QuoteKW token
		self.next();

		let datum = self.try_datum()?;

		Ok(LiteralExpr::Quotation(datum))
	}

	fn try_datum(&mut self) -> Result<Datum, Error> {
		let next = self.next();
		match next.ty {
			TokenType::BeginKW => {
				Ok(Datum::IdentDatum(Box::new(IdentifierExpr("begin".to_string()))))
			},
			TokenType::LambdaKW => {
				Ok(Datum::IdentDatum(Box::new(IdentifierExpr("lambda".to_string()))))
			},
			TokenType::IfKW => Ok(Datum::IdentDatum(Box::new(IdentifierExpr("if".to_string())))),
			TokenType::DefineKW => {
				Ok(Datum::IdentDatum(Box::new(IdentifierExpr("define".to_string()))))
			},
			TokenType::SetKW => Ok(Datum::IdentDatum(Box::new(IdentifierExpr("set!".to_string())))),

			TokenType::Bool(b) => Ok(Datum::LitDatum(Box::new(LiteralExpr::Bool(b)))),
			TokenType::Number(n) => Ok(Datum::LitDatum(Box::new(LiteralExpr::Number(n)))),
			TokenType::String(s) => Ok(Datum::LitDatum(Box::new(LiteralExpr::String(s)))),
			TokenType::Nil => Ok(Datum::LitDatum(Box::new(LiteralExpr::Nil))),

			TokenType::Identifier(i) => Ok(Datum::IdentDatum(Box::new(IdentifierExpr(i)))),

			TokenType::LeftParenthesis => {
				let mut data = vec![];
				while self.peek().ty != TokenType::RightParenthesis {
					let datum = self.try_datum()?;
					data.push(datum);
				}

				// Match closing paren
				self.try_right_paren()?;

				Ok(Datum::ListDatum(data))
			},
			_ => Err(ParseError::UnexpectedToken(next).into()),
		}
	}

	/// Try to match a sequence expression
	fn try_sequence(&mut self) -> Result<SequenceExpr, Error> {
		// Skip the BeginKW token
		self.next();

		let mut exprs = vec![];
		while self.peek().ty != TokenType::RightParenthesis {
			let expr = self.try_expression()?;
			exprs.push(expr);
		}

		// Match closing paren
		self.try_right_paren()?;

		Ok(SequenceExpr(exprs))
	}

	/// Try to match a lambda expression
	fn try_lambda(&mut self) -> Result<LambdaExpr, Error> {
		// Skip the LambdaKW token
		self.next();

		let mut formals = vec![];
		let next = self.next();
		match next.ty {
			TokenType::LeftParenthesis => {
				while self.peek().ty != TokenType::RightParenthesis {
					let formal = self.next();
					match formal.ty {
						TokenType::Identifier(i) => formals.push(IdentifierExpr(i)),
						_ => {
							return Err(ParseError::Expected {
								expected: "`Identifier`".to_string(),
								found:    formal,
							}
							.into());
						},
					}
				}

				// Skip closing paren
				self.next();
			},
			TokenType::Identifier(i) => formals.push(IdentifierExpr(i)),
			TokenType::Nil => (),
			_ => {
				return Err(ParseError::Expected {
					expected: "`(` or `Identifier`".to_string(),
					found:    next,
				}
				.into());
			},
		}

		let mut body = vec![];
		while self.peek().ty != TokenType::RightParenthesis {
			let expr = self.try_expression()?;
			body.push(expr);
		}

		// Match closing paren
		self.try_right_paren()?;

		Ok(LambdaExpr { formals: LambdaFormals(formals), body: LambdaBody(body) })
	}

	/// Try to match a lambda expression
	fn try_if(&mut self) -> Result<IfExpr, Error> {
		// Skip the IfKW token
		self.next();

		let test = self.try_expression()?;
		let consequent = self.try_expression()?;

		match self.peek().ty {
			TokenType::RightParenthesis => {
				// Match closing paren
				self.try_right_paren()?;

				Ok(IfExpr {
					test:       IfTest(Box::new(test)),
					consequent: IfConsequent(Box::new(consequent)),
					alternate:  None,
				})
			},
			_ => {
				let alternate = self.try_expression()?;

				// Match closing paren
				self.try_right_paren()?;

				Ok(IfExpr {
					test:       IfTest(Box::new(test)),
					consequent: IfConsequent(Box::new(consequent)),
					alternate:  Some(IfAlternate(Box::new(alternate))),
				})
			},
		}
	}

	/// Try to match a lambda expression
	fn try_define(&mut self) -> Result<DefineExpr, Error> {
		// Skip the DefineKW token
		self.next();

		let next = self.next();
		let target = match next.ty {
			TokenType::Identifier(i) => IdentifierExpr(i),
			_ => {
				return Err(ParseError::Expected {
					expected: "`Identifier`".to_string(),
					found:    next,
				}
				.into());
			},
		};

		let value = self.try_expression()?;

		// Match closing paren
		self.try_right_paren()?;

		Ok(DefineExpr { target: DefineTarget(target), value: DefineValue(Box::new(value)) })
	}

	/// Try to match a lambda expression
	fn try_assign(&mut self) -> Result<AssignExpr, Error> {
		// Skip the SetKW token
		self.next();

		let next = self.next();
		let target = match next.ty {
			TokenType::Identifier(i) => IdentifierExpr(i),
			_ => {
				return Err(ParseError::Expected {
					expected: "`Identifier`".to_string(),
					found:    next,
				}
				.into());
			},
		};

		let value = self.try_expression()?;

		// Match closing paren
		self.try_right_paren()?;

		Ok(AssignExpr { target: AssignTarget(target), value: AssignValue(Box::new(value)) })
	}

	/// Try to match a lambda expression
	fn try_call(&mut self) -> Result<CallExpr, Error> {
		let op = match self.next().ty {
			TokenType::Identifier(i) => IdentifierExpr(i),
			_ => unimplemented!(),
		};

		let operator = CallOperator(op);

		let mut operands = vec![];
		while self.peek().ty != TokenType::RightParenthesis {
			let expr = self.try_expression()?;
			operands.push(expr);
		}

		// Match closing paren
		self.try_right_paren()?;

		Ok(CallExpr { operator, operands: CallOperands(operands) })
	}

	/// Try to match a right parenthesis
	fn try_right_paren(&mut self) -> Result<(), Error> {
		let next = self.next();
		match next.ty {
			TokenType::RightParenthesis => Ok(()),
			_ => Err(ParseError::Expected { expected: "`)`".to_string(), found: next }.into()),
		}
	}
}
