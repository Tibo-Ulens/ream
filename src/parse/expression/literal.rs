use super::IdentifierExpr;

/// An AST expression which evaluates to a literal
#[derive(Clone, Debug)]
pub(crate) enum LiteralExpr {
	Quotation(Datum),
	Bool(bool),
	Number(f64),
	String(String),
	Nil,
}

/// A less specific type equivalent to an [`Expression`] used in Quotations
#[derive(Clone, Debug)]
pub(crate) enum Datum {
	IdentDatum(Box<IdentifierExpr>),
	LitDatum(Box<LiteralExpr>),
	ListDatum(Vec<Datum>),
}
