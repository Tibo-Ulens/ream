use super::{Expression, IdentifierExpr};

/// An AST expression which introduces a new identifier
#[derive(Clone, Debug)]
pub(crate) struct DefineExpr {
	pub(crate) target: DefineTarget,
	pub(crate) value:  DefineValue,
}

#[derive(Clone, Debug)]
pub(crate) struct DefineTarget(pub(crate) IdentifierExpr);

#[derive(Clone, Debug)]
pub(crate) struct DefineValue(pub(crate) Box<Expression>);
