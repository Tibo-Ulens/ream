use super::{Expression, IdentifierExpr};

/// An AST expression which assigns a new value to an identifier
#[derive(Clone)]
pub(crate) struct AssignExpr {
	pub(crate) target: AssignTarget,
	pub(crate) value:  AssignValue,
}

#[derive(Clone)]
pub(crate) struct AssignTarget(pub(crate) IdentifierExpr);

#[derive(Clone)]
pub(crate) struct AssignValue(pub(crate) Box<Expression>);
