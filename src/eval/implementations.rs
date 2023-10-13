use std::cell::RefCell;
use std::rc::Rc;

use super::{Eval, ReamType, ReamValue, Scope};
use crate::ast::{Datum, Expression, Identifier, Literal};
use crate::EvalError;

impl<'s, 'r> Eval<'s, 'r> for Expression<'s> {
	fn eval(self, scope: Rc<RefCell<Scope<'s>>>) -> Result<ReamValue<'s>, EvalError> {
		match self {
			Self::Identifier(Identifier { span, id }) => {
				match scope.borrow().get(id) {
					Some(v) => Ok(v),
					None => Err(EvalError::UnknownIdentifier { loc: span, id: id.to_owned() }),
				}
			},
			Self::Literal(lit) => lit.eval(scope),
			Self::Definition { span, target, value } => {
				let value = value.eval(scope.clone())?;
				scope.borrow_mut().set(target.id, value);

				Ok(ReamValue { span, t: ReamType::Unit })
			},
			Self::Sequence { span, seq } => {
				let sequence_scope = Scope::extend(scope.to_owned());

				let values = seq
					.into_iter()
					.map(|e| e.eval(sequence_scope.clone()))
					.collect::<Result<Vec<ReamValue<'s>>, EvalError>>()?;

				let ret_value = values.last().cloned().map(|v| v.t).unwrap_or(ReamType::Unit);

				Ok(ReamValue { span, t: ret_value })
			},
			Self::ProcedureCall { span, operator, operands } => {
				// let arguments = operands
				// 	.into_iter()
				// 	.map(|o| o.eval(scope.clone()))
				// 	.collect::<Result<Vec<ReamValue<'s>>, EvalError>>()?;

				let value = operator.apply(operands, scope)?;

				Ok(ReamValue { span, t: value })
			},
			Self::LambdaExpression { span, formals, body } => {
				let enclosed_scope = Scope::close(scope.to_owned());

				Ok(ReamValue { span, t: ReamType::Closure { formals, body, enclosed_scope } })
			},
			Self::Conditional { span, test, consequent, alternate } => {
				let test_value = test.eval(scope.clone())?;

				if test_value.t.is_truthy() {
					let cons_value = consequent.eval(scope)?;

					return Ok(ReamValue { span, t: cons_value.t });
				}

				if let Some(alternate) = alternate {
					let alt_value = alternate.eval(scope)?;

					Ok(ReamValue { span, t: alt_value.t })
				} else {
					Ok(ReamValue { span, t: ReamType::Unit })
				}
			},

			_ => todo!(),
		}
	}
}

impl<'s, 'r> Eval<'s, 'r> for Literal<'s> {
	fn eval(self, scope: Rc<RefCell<Scope<'s>>>) -> Result<ReamValue<'s>, EvalError> {
		match self {
			Self::Quotation { span, q } => {
				let value = q.eval(scope).map(|v| v.t)?;

				Ok(ReamValue { span, t: value })
			},
			Self::Boolean { span, b } => Ok(ReamValue { span, t: ReamType::Boolean(b) }),
			Self::Integer { span, i } => Ok(ReamValue { span, t: ReamType::Integer(i) }),
			Self::Float { span, f } => Ok(ReamValue { span, t: ReamType::Float(f) }),
			Self::Character { span, c } => Ok(ReamValue { span, t: ReamType::Character(c) }),
			Self::String { span, s } => Ok(ReamValue { span, t: ReamType::String(s) }),
			Self::Atom { span, a } => Ok(ReamValue { span, t: ReamType::Atom(a) }),
		}
	}
}

impl<'s, 'r> Eval<'s, 'r> for Datum<'s> {
	fn eval(self, _scope: Rc<RefCell<Scope<'s>>>) -> Result<ReamValue<'s>, EvalError> {
		match self {
			Self::Identifier { span, id } => Ok(ReamValue { span, t: ReamType::Identifier(id) }),
			Self::Boolean { span, b } => Ok(ReamValue { span, t: ReamType::Boolean(b) }),
			Self::Integer { span, i } => Ok(ReamValue { span, t: ReamType::Integer(i) }),
			Self::Float { span, f } => Ok(ReamValue { span, t: ReamType::Float(f) }),
			Self::Character { span, c } => Ok(ReamValue { span, t: ReamType::Character(c) }),
			Self::String { span, s } => Ok(ReamValue { span, t: ReamType::String(s) }),
			Self::Atom { span, a } => Ok(ReamValue { span, t: ReamType::Atom(a) }),
			Self::List { span, l } => {
				let datum_vec = Vec::<Datum<'s>>::from(l.to_owned());
				let rvalue_vec = datum_vec
					.into_iter()
					.map(|d| d.eval(_scope.clone()))
					.collect::<Result<Vec<ReamValue<'s>>, EvalError>>()?;

				Ok(ReamValue { span, t: ReamType::List(rvalue_vec) })
			},
		}
	}
}
