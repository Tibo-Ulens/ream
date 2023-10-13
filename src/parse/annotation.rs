use miette::{Error, SourceSpan};

use crate::{ast, Combine, ParseError, Parser, TokenType};

impl<'s> Parser<'s> {
	/// Parse an annotation of the form `(<atom> <target> ...)
	/// where target is `<identifier>`
	///
	/// `(` and `<atom>` already consumed
	pub(super) fn parse_annotation(
		&mut self,
		initial_span: SourceSpan,
		annotation_type: &'s str,
	) -> Result<ast::Annotation<'s>, Error> {
		match annotation_type {
			":type" => self.parse_type_annotation(initial_span),
			":doc" => self.parse_doc_annotation(initial_span),
			_ => {
				Err(ParseError::InvalidAnnotation {
					loc:   initial_span,
					found: annotation_type.to_string(),
				}
				.into())
			},
		}
	}

	/// Parse a type annotation of the form `(:type <target> <typespec>)`
	/// where target is `<identifier>`
	/// and docstring is `<string>`
	///
	/// `(` and `:type` already consumed
	fn parse_type_annotation(
		&mut self,
		_initial_span: SourceSpan,
	) -> Result<ast::Annotation<'s>, Error> {
		todo!()
	}

	/// Parse a doc annotation of the form `(:doc <target> <docstring>)`
	/// where target is `<identifier>`
	/// and docstring is `<string>`
	///
	/// `(` and `:doc` already consumed
	fn parse_doc_annotation(
		&mut self,
		initial_span: SourceSpan,
	) -> Result<ast::Annotation<'s>, Error> {
		let target = self.expect(TokenType::Identifier(""))?;

		let doc_str_token = self.expect(TokenType::String(""))?;
		let TokenType::String(doc_str) = doc_str_token.t else { unreachable!() };

		let right_paren = self.expect(TokenType::RightParen)?;

		let span = [&target, &doc_str_token, &right_paren]
			.iter()
			.map(|t| t.span)
			.fold(initial_span, |acc, s| acc.combine(&s));

		Ok(ast::Annotation::DocAnnotation { span, target: target.into(), doc: doc_str })
	}
}
