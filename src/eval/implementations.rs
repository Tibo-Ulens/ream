use std::cell::RefCell;
use std::rc::Rc;

use super::{Eval, ReamValue, Scope};
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
			Self::Definition { span: _, target, value } => {
				let value = value.eval(scope.clone())?;
				scope.borrow_mut().set(target.id, value);

				Ok(ReamValue::Unit)
			},
			Self::Sequence { span: _, seq } => {
				let sequence_scope = Scope::extend(scope.to_owned());

				let values = seq
					.into_iter()
					.map(|e| e.eval(sequence_scope.clone()))
					.collect::<Result<Vec<ReamValue<'s>>, EvalError>>()?;

				Ok(values.last().cloned().unwrap_or(ReamValue::Unit))
			},
			Self::ProcedureCall { span: _, operator, operands } => {
				let arguments = operands
					.into_iter()
					.map(|o| o.eval(scope.clone()))
					.collect::<Result<Vec<ReamValue<'s>>, EvalError>>()?;

				operator.apply(arguments, scope.clone())
			},
			Self::LambdaExpression { span: _, formals, body } => {
				let enclosed_scope = Scope::close(scope.to_owned());

				Ok(ReamValue::Closure { formals, body, enclosed_scope })
			},

			_ => todo!(),
		}
	}
}

impl<'s, 'r> Eval<'s, 'r> for Literal<'s> {
	fn eval(self, scope: Rc<RefCell<Scope<'s>>>) -> Result<ReamValue<'s>, EvalError> {
		match self {
			Self::Quotation { span: _, q } => q.eval(scope),
			Self::Boolean { span: _, b } => Ok(ReamValue::Boolean(b)),
			Self::Integer { span: _, i } => Ok(ReamValue::Integer(i)),
			Self::Float { span: _, f } => Ok(ReamValue::Float(f)),
			Self::Character { span: _, c } => Ok(ReamValue::Character(c)),
			Self::String { span: _, s } => Ok(ReamValue::String(s)),
			Self::Atom { span: _, a } => Ok(ReamValue::Atom(a)),
		}
	}
}

impl<'s, 'r> Eval<'s, 'r> for Datum<'s> {
	fn eval(self, _scope: Rc<RefCell<Scope<'s>>>) -> Result<ReamValue<'s>, EvalError> {
		match self {
			Self::Identifier { span: _, id } => Ok(ReamValue::Identifier(id)),
			Self::Boolean { span: _, b } => Ok(ReamValue::Boolean(b)),
			Self::Integer { span: _, i } => Ok(ReamValue::Integer(i)),
			Self::Float { span: _, f } => Ok(ReamValue::Float(f)),
			Self::Character { span: _, c } => Ok(ReamValue::Character(c)),
			Self::String { span: _, s } => Ok(ReamValue::String(s)),
			Self::Atom { span: _, a } => Ok(ReamValue::Atom(a)),
			Self::List { span: _, l } => {
				let datum_vec = Vec::<Datum<'s>>::from(l.to_owned());
				let rvalue_vec = datum_vec
					.into_iter()
					.map(|d| d.eval(_scope.clone()))
					.collect::<Result<Vec<ReamValue<'s>>, EvalError>>()?;

				Ok(ReamValue::List(rvalue_vec))
			},
		}
	}
}
