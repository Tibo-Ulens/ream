//! AST node evaluation

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::Program;
use crate::EvalError;

mod apply;
mod implementations;
mod value;

use value::ReamValue;

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
		let global_scope = Rc::new(RefCell::new(Scope::default()));

		for expr in self.0 {
			println!("{:?}", expr.eval(global_scope.clone())?);
		}

		Ok(())
	}
}
