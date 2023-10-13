//! AST node evaluation

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ast::{Expression, Identifier, Program};
use crate::EvalError;

mod implementations;
mod primitives;
mod value;

use value::{ReamType, ReamValue};

use self::primitives::ADD;

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

impl<'s, 'r> Identifier<'s> {
	fn apply(
		&'r self,
		args: Vec<Expression<'s>>,
		scope: Rc<RefCell<Scope<'s>>>,
	) -> Result<ReamType<'s>, EvalError> {
		// if let Some(primitive) = self.try_primitive() {
		// 	return primitive(self.span, self.id.to_string(), args);
		// }

		let Some(ReamValue { span: _, t: v_type }) = scope.borrow().get(self.id) else {
			return Err(EvalError::UnknownIdentifier { loc: self.span, id: self.id.to_string() });
		};

		if let ReamType::Primitive(prim) = v_type {
			return prim(self.span, self.id.to_string(), args, scope);
		}

		let ReamType::Closure { formals, body, enclosed_scope } = v_type else {
			return Err(EvalError::NotAFunction { loc: self.span, name: self.id.to_string() });
		};

		if formals.len() != args.len() {
			return Err(EvalError::WrongArgumentCount {
				loc:      self.span,
				callee:   self.id.to_string(),
				expected: formals.len(),
				found:    args.len(),
			});
		}

		let arg_values = args
			.into_iter()
			.map(|o| o.eval(scope.clone()))
			.collect::<Result<Vec<ReamValue<'s>>, EvalError>>()?;

		// Create a new scope with the formals set to their respective argument
		let execution_scope = Scope::extend(enclosed_scope);
		formals
			.iter()
			.map(|f| f.id)
			.zip(arg_values)
			.for_each(|(k, v)| execution_scope.borrow_mut().set(k, v));

		let values = body
			.into_iter()
			.map(|e| e.eval(execution_scope.clone()))
			.collect::<Result<Vec<ReamValue<'s>>, EvalError>>()?;

		Ok(values.last().cloned().map(|v| v.t).unwrap_or(ReamType::Unit))
	}
}

impl<'s> Program<'s> {
	/// Run the program
	pub fn run(self) -> Result<(), EvalError> {
		let mut scope_inner = Scope::default();

		scope_inner.set("+", ReamValue { span: (0, 0).into(), t: ADD::<'s> });

		let global_scope = Rc::new(RefCell::new(scope_inner));

		for expr in self.0 {
			println!("{:?}", expr.eval(global_scope.clone())?);
		}

		Ok(())
	}
}
