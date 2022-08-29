use super::Expression;

/// An AST expression which evaluates to its `consequent` or `alternate` depending on `test`
#[derive(Clone)]
pub(crate) struct IfExpr {
	pub(crate) test:       IfTest,
	pub(crate) consequent: IfConsequent,
	pub(crate) alternate:  Option<IfAlternate>,
}

#[derive(Clone)]
pub(crate) struct IfTest(pub(crate) Box<Expression>);

#[derive(Clone)]
pub(crate) struct IfConsequent(pub(crate) Box<Expression>);

#[derive(Clone)]
pub(crate) struct IfAlternate(pub(crate) Box<Expression>);
