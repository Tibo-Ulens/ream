use super::{Expression, IdentifierExpr};

/// An AST expression which evaluates to the result of a function call
#[derive(Clone, Debug)]
pub(crate) struct CallExpr {
	pub(crate) operator: CallOperator,
	pub(crate) operands: CallOperands,
}

#[derive(Clone, Debug)]
pub(crate) struct CallOperator(pub(crate) IdentifierExpr);

#[derive(Clone, Debug)]
pub(crate) struct CallOperands(pub(crate) Vec<Expression>);
