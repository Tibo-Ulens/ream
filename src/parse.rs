#![allow(dead_code)]

use std::iter::Peekable;

use miette::Error;

use crate::{ast, Lexer, Token};

/// A parser for a single source file
#[allow(missing_docs)]
pub struct Parser<'s> {
	source: &'s str,
	tokens: Peekable<Lexer<'s>>,
}

impl<'s> Parser<'s> {
	/// Create a new [`Parser`]
	pub fn new(source: &'s str, tokens: Peekable<Lexer<'s>>) -> Self { Self { source, tokens } }

	/// Peek at the next [`Token`]
	///
	/// Returns [`None`] if no tokens are left
	fn peek(&mut self) -> Option<&Result<Token<'s>, Error>> { self.tokens.peek() }

	/// Consume and return the next [`Token`]
	///
	/// Returns [`None`] if no tokens are left
	fn next(&mut self) -> Option<Result<Token<'s>, Error>> { self.tokens.next() }

	/// Parse the entire input
	pub fn parse(&self) -> Result<ast::Program<'s>, Error> { todo!() }
}
