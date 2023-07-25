use miette::{Error, SourceSpan};

use crate::{ast, Combine, ParseError, Parser, TokenType};

impl<'s> Parser<'s> {
	/// Parse a shorthand quote of the form '`<datum>'
	///
	/// '`' already consumed
	pub(super) fn parse_shorthand_quote(
		&mut self,
		initial_span: SourceSpan,
	) -> Result<ast::Literal<'s>, Error> {
		let (datum, datum_span) = self.parse_datum()?;

		let quote_span = initial_span.combine(&datum_span);

		Ok(ast::Literal::Quotation { span: quote_span, q: datum })
	}

	/// Parse a quote of the form `(quote <datum>)`
	///
	/// `(` and `quote` already consumed
	pub(super) fn parse_quote(
		&mut self,
		initial_span: SourceSpan,
	) -> Result<ast::Literal<'s>, Error> {
		let (datum, datum_span) = self.parse_datum()?;

		let right_paren = self.expect(TokenType::RightParen)?;
		let quote_span = initial_span.combine(&datum_span).combine(&right_paren.span);

		Ok(ast::Literal::Quotation { span: quote_span, q: datum })
	}

	/// Parse a datum and return it alongside its span
	fn parse_datum(&mut self) -> Result<(ast::Datum<'s>, SourceSpan), Error> {
		let token = self.next()?;

		let span = token.span;

		match token.t {
			TokenType::Identifier(_) => Ok((token.into(), token.span)),
			TokenType::Boolean(_) => Ok((token.into(), token.span)),
			TokenType::Integer(_) => Ok((token.into(), token.span)),
			TokenType::Float(_) => Ok((token.into(), token.span)),
			TokenType::Character(_) => Ok((token.into(), token.span)),
			TokenType::String(_) => Ok((token.into(), token.span)),
			TokenType::Atom(_) => Ok((token.into(), token.span)),

			TokenType::LeftParen => {
				let (data, data_span) = self.parse_datum_list(span)?;

				Ok((ast::Datum::List { span: data_span, l: data }, data_span))
			},

			tt => Err(ParseError::InvalidDatum { loc: token.span, found: tt.to_string() }.into()),
		}
	}

	/// Parse a datum list of the form `(<datum>*)` or `(<datum> . <list>)`
	///
	/// `(` already consumed
	fn parse_datum_list(
		&mut self,
		initial_span: SourceSpan,
	) -> Result<(Vec<ast::Datum<'s>>, SourceSpan), Error> {
		let mut data = vec![];
		let mut span = initial_span;

		// If the next token is a `)` then this is an empty list `()` and
		// there's nothing left to parse
		if self.peek()?.t == TokenType::RightParen {
			// Unwrap is safe as peek is some
			let right_paren = self.next().unwrap();
			span = span.combine(&right_paren.span);

			return Ok((data, span));
		}

		loop {
			let (datum, next_span) = self.parse_datum()?;
			span = span.combine(&next_span);
			data.push(datum);

			let peek = self.peek()?;
			span = span.combine(&peek.span);

			match peek.t {
				TokenType::RightParen => {
					// Unwrap is safe as peek is some
					self.next().unwrap();
					return Ok((data, span));
				},
				TokenType::Period => {
					// Unwrap is safe as peek is some
					self.next().unwrap();

					let left_paren = self.expect(TokenType::LeftParen)?;
					// span.combine(&left_paren.span);

					let (rec_data, rec_span) = self.parse_datum_list(left_paren.span)?;

					let rec_list = ast::Datum::List { span: rec_span, l: rec_data };

					data.push(rec_list);
					span.combine(&rec_span);

					let right_paren = self.expect(TokenType::RightParen)?;
					span = span.combine(&right_paren.span);

					return Ok((data, span));
				},

				_ => (),
			}
		}
	}
}
