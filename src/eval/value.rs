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
	pub(super) span: SourceSpan,
	pub(super) t:    ReamType<'s>,
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
	/// Render the name of this type as a string
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

	/// Check if the value is truthy
	pub(super) fn is_truthy(&self) -> bool {
		match self {
			Self::Boolean(b) => *b,
			Self::Integer(i) => *i != 0,
			Self::Float(f) => *f != 0.0,
			Self::Character(_) => true,
			Self::String(s) => !s.is_empty(),
			Self::Identifier(_) => true,
			Self::Atom(_) => true,
			Self::List(l) => !l.is_empty(),
			Self::Primitive(_) => true,
			Self::Closure { formals: _, body: _, enclosed_scope: _ } => true,
			Self::Unit => true,
		}
	}
}
