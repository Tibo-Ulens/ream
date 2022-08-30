mod assign;
mod call;
mod define;
mod if_;
mod lambda;
mod literal;

pub(crate) use assign::*;
pub(crate) use call::*;
pub(crate) use define::*;
pub(crate) use if_::*;
pub(crate) use lambda::*;
pub(crate) use literal::*;

/// Generic AST expression node
#[derive(Clone, Debug)]
pub(crate) enum Expression {
	Identifier(IdentifierExpr),
	Literal(LiteralExpr),
	Sequence(SequenceExpr),
	Call(CallExpr),
	Lambda(LambdaExpr),
	If(IfExpr),
	Define(DefineExpr),
	Assign(AssignExpr),
}

/// An AST expression which evaluates to an identifier
#[derive(Clone, Debug)]
pub(crate) struct IdentifierExpr(pub(crate) String);

/// An AST expression which evaluates to the last expression in its sequence
#[derive(Clone, Debug)]
pub(crate) struct SequenceExpr(pub(crate) Vec<Expression>);
