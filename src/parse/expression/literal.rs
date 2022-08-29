use super::IdentifierExpr;
use crate::lex::Token;

/// An AST expression which evaluates to a literal
#[derive(Clone)]
pub(crate) enum LiteralExpr {
	Quotation(Datum),
	Bool(Token),
	Number(Token),
	String(Token),
	Nil(Token),
}

/// A less specific type equivalent to an [`Expression`] used in Quotations
#[derive(Clone)]
pub(crate) enum Datum {
	IdentDatum(Box<IdentifierExpr>),
	LitDatum(Box<LiteralExpr>),
	ListDatum(Vec<Datum>),
}
