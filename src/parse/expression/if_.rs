use super::Expression;

/// An AST expression which evaluates to its `consequent` or `alternate` depending on `test`
#[derive(Clone, Debug)]
pub(crate) struct IfExpr {
	pub(crate) test:       IfTest,
	pub(crate) consequent: IfConsequent,
	pub(crate) alternate:  Option<IfAlternate>,
}

#[derive(Clone, Debug)]
pub(crate) struct IfTest(pub(crate) Box<Expression>);

#[derive(Clone, Debug)]
pub(crate) struct IfConsequent(pub(crate) Box<Expression>);

#[derive(Clone, Debug)]
pub(crate) struct IfAlternate(pub(crate) Box<Expression>);
