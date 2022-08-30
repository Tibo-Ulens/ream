use super::{Expression, IdentifierExpr};

/// An AST expression which evaluates to a new anonymous function
#[derive(Clone, Debug)]
pub(crate) struct LambdaExpr {
	pub(crate) formals: LambdaFormals,
	pub(crate) body:    LambdaBody,
}

#[derive(Clone, Debug)]
pub(crate) struct LambdaFormals(pub(crate) Vec<IdentifierExpr>);

#[derive(Clone, Debug)]
pub(crate) struct LambdaBody(pub(crate) Vec<Expression>);
