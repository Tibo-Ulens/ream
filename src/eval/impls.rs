use std::cell::RefCell;
use std::rc::Rc;

use super::{try_eval_std_operator, Env, Eval, Type};
use crate::error::{Error, EvalError};
use crate::parse::{
	AssignExpr,
	CallExpr,
	Datum,
	DefineExpr,
	Expression,
	IdentifierExpr,
	IfExpr,
	LambdaExpr,
	LiteralExpr,
	Root,
	SequenceExpr,
};

impl Eval for Root {
	fn eval(&self, env: &mut Rc<RefCell<Env>>) -> Result<Type, Error> {
		let list = self.exprs.iter().map(|e| e.eval(env)).collect::<Result<Vec<Type>, Error>>()?;

		Ok(Type::List(list))
	}
}

impl Eval for Expression {
	fn eval(&self, env: &mut Rc<RefCell<Env>>) -> Result<Type, Error> {
		match self {
			Self::Identifier(i) => i.eval(env),
			Self::Literal(l) => l.eval(env),
			Self::Sequence(s) => s.eval(env),
			Self::Define(d) => d.eval(env),
			Self::Assign(a) => a.eval(env),
			Self::Lambda(l) => l.eval(env),
			Self::Call(c) => c.eval(env),
			Self::If(i) => i.eval(env),
		}
	}
}

impl Eval for IdentifierExpr {
	fn eval(&self, env: &mut Rc<RefCell<Env>>) -> Result<Type, Error> {
		match env.borrow().get(&self.0) {
			Some(t) => Ok(t),
			None => Err(EvalError::UnknownSymbol(self.0.clone()).into()),
		}
	}
}

impl Eval for LiteralExpr {
	fn eval(&self, env: &mut Rc<RefCell<Env>>) -> Result<Type, Error> {
		match self {
			Self::Bool(b) => Ok(Type::Bool(*b)),
			Self::Number(n) => Ok(Type::Number(*n)),
			Self::String(s) => Ok(Type::String((*s).clone())),
			Self::Nil => Ok(Type::Nil),
			Self::Quotation(d) => d.eval(env),
		}
	}
}

impl Eval for Datum {
	fn eval(&self, env: &mut Rc<RefCell<Env>>) -> Result<Type, Error> {
		match self {
			Datum::IdentDatum(i) => Ok(Type::Symbol(i.0.clone())),
			Datum::LitDatum(l) => l.eval(env),
			Datum::ListDatum(l) => {
				let list = l.iter().map(|d| d.eval(env)).collect::<Result<Vec<Type>, Error>>()?;

				Ok(Type::List(list))
			},
		}
	}
}

impl Eval for SequenceExpr {
	fn eval(&self, env: &mut Rc<RefCell<Env>>) -> Result<Type, Error> {
		let mut seq_env = Rc::new(RefCell::new(Env::extend(env.to_owned())));

		let list =
			self.0.iter().map(|e| e.eval(&mut seq_env)).collect::<Result<Vec<Type>, Error>>()?;

		Ok(list.last().unwrap_or_else(|| &Type::Nil).clone())
	}
}

impl Eval for DefineExpr {
	fn eval(&self, env: &mut Rc<RefCell<Env>>) -> Result<Type, Error> {
		let target = self.target.0.0.clone();
		let value = self.value.0.eval(env)?;

		if let Some(_) = env.borrow().get(&target) {
			return Err(EvalError::RedefinedSymbol(target).into());
		}

		env.borrow_mut().set(&target, value);

		Ok(Type::Nil)
	}
}

impl Eval for AssignExpr {
	fn eval(&self, env: &mut Rc<RefCell<Env>>) -> Result<Type, Error> {
		let target = self.target.0.0.clone();
		let value = self.value.0.eval(env)?;

		if let None = env.borrow().get(&target) {
			return Err(EvalError::UnknownSymbol(target).into());
		}

		env.borrow_mut().set(&target, value);

		Ok(Type::Nil)
	}
}

impl Eval for LambdaExpr {
	fn eval(&self, _: &mut Rc<RefCell<Env>>) -> Result<Type, Error> {
		let formals = self.formals.0.iter().map(|i| i.0.clone()).collect::<Vec<String>>();

		let body = self.body.0.clone();

		Ok(Type::Lambda((formals, body)))
	}
}

impl Eval for CallExpr {
	fn eval(&self, env: &mut Rc<RefCell<Env>>) -> Result<Type, Error> {
		let operands =
			self.operands.0.iter().map(|e| e.eval(env)).collect::<Result<Vec<Type>, Error>>()?;

		let operator_raw = &self.operator.0;
		let operator = match operator_raw.eval(env) {
			Ok(t) => t,
			Err(_) => {
				return try_eval_std_operator(&operator_raw.0, operands);
			},
		};

		if let Type::Lambda((formals, body)) = operator {
			if formals.len() != operands.len() {
				return Err(EvalError::WrongArgCount(
					self.operator.0.0.clone(),
					formals.len(),
					operands.len(),
				)
				.into());
			}

			// Create new env with the lambda formals pointing to their values
			let mut call_env = Rc::new(RefCell::new(Env::extend(env.to_owned())));
			formals
				.iter()
				.zip(operands.iter())
				.for_each(|(k, v)| call_env.borrow_mut().set(k, v.to_owned()));

			let results =
				body.iter().map(|e| e.eval(&mut call_env)).collect::<Result<Vec<Type>, Error>>()?;

			Ok(results.last().unwrap_or_else(|| &Type::Nil).clone())
		} else {
			return Err(EvalError::NotAProcedure(self.operator.0.0.clone()).into());
		}
	}
}

impl Eval for IfExpr {
	fn eval(&self, env: &mut Rc<RefCell<Env>>) -> Result<Type, Error> {
		let test = self.test.0.eval(env)?;
		let cond = match test {
			Type::Bool(b) => b,
			_ => true,
		};

		if cond {
			self.consequent.0.eval(env)
		} else {
			if let Some(alternate) = &self.alternate {
				alternate.0.eval(env)
			} else {
				Ok(Type::Nil)
			}
		}
	}
}
