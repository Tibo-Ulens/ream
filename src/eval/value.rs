use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;

use super::Scope;
use crate::ast::{Expression, Identifier};

#[derive(Debug, Clone)]
pub(super) enum ReamValue<'s> {
	Boolean(bool),
	Integer(u64),
	Float(f64),
	Character(char),
	String(&'s str),
	Identifier(&'s str),
	Atom(&'s str),
	List(Vec<ReamValue<'s>>),

	Closure {
		formals:        Vec<Identifier<'s>>,
		body:           Vec<Expression<'s>>,
		enclosed_scope: Rc<RefCell<Scope<'s>>>,
	},

	Unit,
}

impl<'s> Add for ReamValue<'s> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Self::Integer(a), Self::Integer(b)) => Self::Integer(a + b),
			(Self::Float(a), Self::Float(b)) => Self::Float(a + b),
			_ => unimplemented!(),
		}
	}
}
