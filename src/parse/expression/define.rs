use super::{Expression, IdentifierExpr};

/// An AST expression which introduces a new identifier
#[derive(Clone)]
pub(crate) struct DefineExpr {
	pub(crate) target: DefineTarget,
	pub(crate) value:  DefineValue,
}

#[derive(Clone)]
pub(crate) struct DefineTarget(pub(crate) IdentifierExpr);

#[derive(Clone)]
pub(crate) struct DefineValue(pub(crate) Box<Expression>);
