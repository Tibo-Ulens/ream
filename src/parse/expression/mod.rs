use crate::lex::Token;

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
#[derive(Clone)]
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
#[derive(Clone)]
pub(crate) struct IdentifierExpr(pub(crate) Token);

/// An AST expression which evaluates to the last expression in its sequence
#[derive(Clone)]
pub(crate) struct SequenceExpr(pub(crate) Vec<Expression>);
