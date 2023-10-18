//! AST node evaluation

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::Program;
use crate::EvalError;

mod implementations;
mod primitives;
mod value;

use value::{ReamType, ReamValue};

use self::primitives::*;

#[derive(Debug, Clone, Default)]
struct Scope<'s> {
	parent:  Option<Rc<RefCell<Self>>>,
	symbols: HashMap<&'s str, ReamValue<'s>>,
}

impl<'s> Scope<'s> {
	/// Get a value in the current scope
	fn get(&self, key: &'s str) -> Option<ReamValue<'s>> {
		match self.symbols.get(key) {
			Some(v) => Some(v.clone()),
			None => self.parent.as_ref().and_then(|p| p.borrow().get(key)),
		}
	}

	/// Set a value in the current scope
	fn set(&mut self, key: &'s str, value: ReamValue<'s>) { self.symbols.insert(key, value); }

	/// Extend a new scope
	fn extend(parent: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
		let symbols = HashMap::new();

		Rc::new(RefCell::new(Self { parent: Some(parent), symbols }))
	}

	/// Close over the given scope
	fn close(scope: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
		Rc::new(RefCell::new(scope.borrow().clone()))
	}
}

trait Eval<'s, 'r> {
	fn eval(self, scope: Rc<RefCell<Scope<'s>>>) -> Result<ReamValue<'s>, EvalError>;
}

impl<'s> Program<'s> {
	/// Run the program
	pub fn run(self) -> Result<(), EvalError> {
		let mut scope_inner = Scope::default();

		scope_inner.set("+", ReamValue { span: (0, 0).into(), t: ADD });
		scope_inner.set("-", ReamValue { span: (0, 0).into(), t: SUB });
		scope_inner.set("*", ReamValue { span: (0, 0).into(), t: MUL });
		scope_inner.set("/", ReamValue { span: (0, 0).into(), t: DIV });

		scope_inner.set("==", ReamValue { span: (0, 0).into(), t: EQU });
		scope_inner.set("!=", ReamValue { span: (0, 0).into(), t: NEQ });
		scope_inner.set(">", ReamValue { span: (0, 0).into(), t: GT });
		scope_inner.set(">=", ReamValue { span: (0, 0).into(), t: GTE });
		scope_inner.set("<", ReamValue { span: (0, 0).into(), t: LT });
		scope_inner.set("<=", ReamValue { span: (0, 0).into(), t: LTE });

		scope_inner.set("print", ReamValue { span: (0, 0).into(), t: PRINT });

		let global_scope = Rc::new(RefCell::new(scope_inner));

		for expr in self.0 {
			expr.eval(global_scope.clone())?;
		}

		Ok(())
	}
}
