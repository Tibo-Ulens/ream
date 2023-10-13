use std::cell::RefCell;
use std::rc::Rc;

use miette::SourceSpan;

use super::Scope;
use crate::ast::{Expression, Identifier};
use crate::EvalError;

type Primitive<'s> = fn(
	operator_location: SourceSpan,
	operator_id: String,
	arguments: Vec<Expression<'s>>,
	scope: Rc<RefCell<Scope<'s>>>,
) -> Result<ReamType<'s>, EvalError>;

#[derive(Debug, Clone)]
pub(super) struct ReamValue<'s> {
	pub span: SourceSpan,
	pub t:    ReamType<'s>,
}

#[derive(Debug, Clone)]
pub(super) enum ReamType<'s> {
	Boolean(bool),
	Integer(u64),
	Float(f64),
	Character(char),
	String(&'s str),
	Identifier(&'s str),
	Atom(&'s str),
	List(Vec<ReamValue<'s>>),

	Primitive(Primitive<'s>),
	Closure {
		formals:        Vec<Identifier<'s>>,
		body:           Vec<Expression<'s>>,
		enclosed_scope: Rc<RefCell<Scope<'s>>>,
	},

	Unit,
}

impl<'s> ReamType<'s> {
	pub(super) fn type_name(&self) -> String {
		match self {
			Self::Boolean(_) => "Boolean".to_string(),
			Self::Integer(_) => "Integer".to_string(),
			Self::Float(_) => "Float".to_string(),
			Self::Character(_) => "Character".to_string(),
			Self::String(_) => "String".to_string(),
			Self::Identifier(_) => "Identifier".to_string(),
			Self::Atom(_) => "Atom".to_string(),
			Self::List(_) => "List".to_string(),
			Self::Primitive(_) => "Primitive".to_string(),
			Self::Closure { formals: _, body: _, enclosed_scope: _ } => "Closure".to_string(),
			Self::Unit => "Unit".to_string(),
		}
	}
}
