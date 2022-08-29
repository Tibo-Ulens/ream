use super::{Expression, IdentifierExpr};

/// An AST expression which evaluates to the result of a function call
#[derive(Clone)]
pub(crate) struct CallExpr {
	pub(crate) operator: CallOperator,
	pub(crate) operands: CallOperands,
}

#[derive(Clone)]
pub(crate) struct CallOperator(pub(crate) IdentifierExpr);

#[derive(Clone)]
pub(crate) struct CallOperands(pub(crate) Vec<Expression>);
