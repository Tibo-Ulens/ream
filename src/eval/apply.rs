use std::cell::RefCell;
use std::rc::Rc;

use miette::SourceSpan;

use super::value::ReamValue;
use super::Scope;
use crate::ast::Identifier;
use crate::eval::Eval;
use crate::EvalError;

type Primitive<'s> =
	fn(loc: SourceSpan, id: String, Vec<ReamValue<'s>>) -> Result<ReamValue<'s>, EvalError>;

macro_rules! expect_args {
	($loc:expr, $cl:expr, $exp:expr, $gvn:expr) => {
		if $exp != $gvn {
			return Err(EvalError::WrongArgumentCount {
				loc:      $loc,
				callee:   $cl,
				expected: $exp,
				found:    $gvn,
			});
		}
	};
}

fn add(loc: SourceSpan, id: String, v: Vec<ReamValue>) -> Result<ReamValue, EvalError> {
	expect_args!(loc, id, 2, v.len());

	let [a, b]: [_; 2] = v.try_into().unwrap();

	match (a, b) {
		(ReamValue::Integer(a), ReamValue::Integer(b)) => Ok(ReamValue::Integer(a + b)),
		(ReamValue::Float(a), ReamValue::Float(b)) => Ok(ReamValue::Float(a + b)),
		_ => unimplemented!(),
	}
}

impl<'s, 'r> Identifier<'s> {
	fn try_primitive(&'r self) -> Option<Primitive<'s>> {
		match self.id {
			"+" => Some(add),
			_ => None,
		}
	}

	pub(super) fn apply(
		&'r self,
		args: Vec<ReamValue<'s>>,
		scope: Rc<RefCell<Scope<'s>>>,
	) -> Result<ReamValue<'s>, EvalError> {
		if let Some(primitive) = self.try_primitive() {
			return primitive(self.span, self.id.to_string(), args);
		}

		let Some(value) = scope.borrow().get(self.id) else {
			return Err(EvalError::UnknownIdentifier { loc: self.span, id: self.id.to_string() });
		};

		let ReamValue::Closure { formals, body, enclosed_scope } = value else {
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

		// Create a new scope with the formals set to their respective argument
		let execution_scope = Scope::extend(enclosed_scope);
		formals
			.iter()
			.map(|f| f.id)
			.zip(args)
			.for_each(|(k, v)| execution_scope.borrow_mut().set(k, v));

		let values = body
			.into_iter()
			.map(|e| e.eval(execution_scope.clone()))
			.collect::<Result<Vec<ReamValue<'s>>, EvalError>>()?;

		Ok(values.last().cloned().unwrap_or(ReamValue::Unit))
	}
}
