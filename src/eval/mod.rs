use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::error::{Error, EvalError};
use crate::parse::Expression;

mod impls;

/// Base types that an expression may evaluate to
#[derive(Clone, Debug)]
pub(crate) enum Type {
	Bool(bool),
	Number(f64),
	String(String),
	Nil,

	Symbol(String),
	List(Vec<Type>),

	Lambda((Vec<String>, Vec<Expression>)),
}

impl Display for Type {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Bool(b) => write!(f, "{}", b),
			Self::Number(n) => write!(f, "{}", n),
			Self::String(s) => write!(f, "{}", s),
			Self::Nil => write!(f, "Nil"),
			Self::Symbol(s) => write!(f, "{}", s),
			Self::List(l) => {
				write!(
					f,
					"({})",
					l.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(" ")
				)
			},
			Self::Lambda((fs, b)) => write!(f, "({}) -> {:?}", fs.join(" "), b),
		}
	}
}

impl Type {
	fn render_type(&self) -> String {
		match self {
			Self::Bool(_) => "bool".to_string(),
			Self::Number(_) => "number".to_string(),
			Self::String(_) => "string".to_string(),
			Self::Nil => "nil".to_string(),
			Self::Symbol(_) => "symbol".to_string(),
			Self::List(_) => "list".to_string(),
			Self::Lambda(_) => "lambda".to_string(),
		}
	}
}

/// Environment representing the current program state
#[derive(Clone, Default)]
pub(crate) struct Env {
	parent:  Option<Rc<RefCell<Self>>>,
	symbols: HashMap<String, Type>,
}

impl Env {
	fn get(&self, key: &str) -> Option<Type> {
		match self.symbols.get(key) {
			Some(v) => Some(v.clone()),
			None => self.parent.as_ref().and_then(|o| o.borrow().get(key).clone()),
		}
	}

	fn set(&mut self, key: &str, val: Type) { self.symbols.insert(key.to_owned(), val); }

	fn extend(parent: Rc<RefCell<Self>>) -> Self {
		let mut symbols = HashMap::new();
		symbols.extend(parent.borrow().symbols.iter().map(|(k, v)| (k.clone(), v.clone())));

		Self { parent: Some(parent), symbols }
	}
}

/// Evaluating to a base type
pub(crate) trait Eval {
	/// Perform the evaluation
	fn eval(&self, env: &mut Rc<RefCell<Env>>) -> Result<Type, Error>;
}

/// See if an operator is a built-in, and evaluate it if so
fn try_eval_std_operator(op: &str, operands: Vec<Type>) -> Result<Type, Error> {
	match op {
		"+" => {
			if operands.len() != 2 {
				return Err(EvalError::WrongArgCount(op.to_string(), 2, operands.len()).into());
			}

			match (&operands[0], &operands[1]) {
				(Type::Number(l), Type::Number(r)) => Ok(Type::Number(l + r)),
				(l, r) => {
					Err(EvalError::InvalidTypes(
						op.to_string(),
						vec!["number".to_string(), "number".to_string()],
						vec![Type::render_type(l), Type::render_type(r)],
					)
					.into())
				},
			}
		},
		"-" => {
			if operands.len() != 2 {
				return Err(EvalError::WrongArgCount(op.to_string(), 2, operands.len()).into());
			}

			match (&operands[0], &operands[1]) {
				(Type::Number(l), Type::Number(r)) => Ok(Type::Number(l - r)),
				(l, r) => {
					Err(EvalError::InvalidTypes(
						op.to_string(),
						vec!["number".to_string(), "number".to_string()],
						vec![Type::render_type(l), Type::render_type(r)],
					)
					.into())
				},
			}
		},
		"*" => {
			if operands.len() != 2 {
				return Err(EvalError::WrongArgCount(op.to_string(), 2, operands.len()).into());
			}

			match (&operands[0], &operands[1]) {
				(Type::Number(l), Type::Number(r)) => Ok(Type::Number(l * r)),
				(l, r) => {
					Err(EvalError::InvalidTypes(
						op.to_string(),
						vec!["number".to_string(), "number".to_string()],
						vec![Type::render_type(l), Type::render_type(r)],
					)
					.into())
				},
			}
		},
		"/" => {
			if operands.len() != 2 {
				return Err(EvalError::WrongArgCount(op.to_string(), 2, operands.len()).into());
			}

			match (&operands[0], &operands[1]) {
				(Type::Number(l), Type::Number(r)) => Ok(Type::Number(l / r)),
				(l, r) => {
					Err(EvalError::InvalidTypes(
						op.to_string(),
						vec!["number".to_string(), "number".to_string()],
						vec![Type::render_type(l), Type::render_type(r)],
					)
					.into())
				},
			}
		},
		"%" => {
			if operands.len() != 2 {
				return Err(EvalError::WrongArgCount(op.to_string(), 2, operands.len()).into());
			}

			match (&operands[0], &operands[1]) {
				(Type::Number(l), Type::Number(r)) => Ok(Type::Number(l % r)),
				(l, r) => {
					Err(EvalError::InvalidTypes(
						op.to_string(),
						vec!["number".to_string(), "number".to_string()],
						vec![Type::render_type(l), Type::render_type(r)],
					)
					.into())
				},
			}
		},
		"<" => {
			if operands.len() != 2 {
				return Err(EvalError::WrongArgCount(op.to_string(), 2, operands.len()).into());
			}

			match (&operands[0], &operands[1]) {
				(Type::Number(l), Type::Number(r)) => Ok(Type::Bool(l < r)),
				(l, r) => {
					Err(EvalError::InvalidTypes(
						op.to_string(),
						vec!["number".to_string(), "number".to_string()],
						vec![Type::render_type(l), Type::render_type(r)],
					)
					.into())
				},
			}
		},
		">" => {
			if operands.len() != 2 {
				return Err(EvalError::WrongArgCount(op.to_string(), 2, operands.len()).into());
			}

			match (&operands[0], &operands[1]) {
				(Type::Number(l), Type::Number(r)) => Ok(Type::Bool(l > r)),
				(l, r) => {
					Err(EvalError::InvalidTypes(
						op.to_string(),
						vec!["number".to_string(), "number".to_string()],
						vec![Type::render_type(l), Type::render_type(r)],
					)
					.into())
				},
			}
		},
		"<=" => {
			if operands.len() != 2 {
				return Err(EvalError::WrongArgCount(op.to_string(), 2, operands.len()).into());
			}

			match (&operands[0], &operands[1]) {
				(Type::Number(l), Type::Number(r)) => Ok(Type::Bool(l <= r)),
				(l, r) => {
					Err(EvalError::InvalidTypes(
						op.to_string(),
						vec!["number".to_string(), "number".to_string()],
						vec![Type::render_type(l), Type::render_type(r)],
					)
					.into())
				},
			}
		},
		">=" => {
			if operands.len() != 2 {
				return Err(EvalError::WrongArgCount(op.to_string(), 2, operands.len()).into());
			}

			match (&operands[0], &operands[1]) {
				(Type::Number(l), Type::Number(r)) => Ok(Type::Bool(l >= r)),
				(l, r) => {
					Err(EvalError::InvalidTypes(
						op.to_string(),
						vec!["number".to_string(), "number".to_string()],
						vec![Type::render_type(l), Type::render_type(r)],
					)
					.into())
				},
			}
		},
		"==" => {
			if operands.len() != 2 {
				return Err(EvalError::WrongArgCount(op.to_string(), 2, operands.len()).into());
			}

			match (&operands[0], &operands[1]) {
				(Type::Bool(l), Type::Bool(r)) => Ok(Type::Bool(l == r)),
				(Type::Number(l), Type::Number(r)) => Ok(Type::Bool(l == r)),
				(Type::String(l), Type::String(r)) => Ok(Type::Bool(l == r)),
				(Type::Nil, Type::Nil) => Ok(Type::Bool(true)),
				(l, r) => {
					Err(EvalError::InvalidTypes(
						op.to_string(),
						vec!["number".to_string(), "number".to_string()],
						vec![Type::render_type(l), Type::render_type(r)],
					)
					.into())
				},
			}
		},
		"!=" => {
			if operands.len() != 2 {
				return Err(EvalError::WrongArgCount(op.to_string(), 2, operands.len()).into());
			}

			match (&operands[0], &operands[1]) {
				(Type::Bool(l), Type::Bool(r)) => Ok(Type::Bool(l != r)),
				(Type::Number(l), Type::Number(r)) => Ok(Type::Bool(l != r)),
				(Type::String(l), Type::String(r)) => Ok(Type::Bool(l != r)),
				(Type::Nil, Type::Nil) => Ok(Type::Bool(false)),
				(l, r) => {
					Err(EvalError::InvalidTypes(
						op.to_string(),
						vec!["number".to_string(), "number".to_string()],
						vec![Type::render_type(l), Type::render_type(r)],
					)
					.into())
				},
			}
		},
		"and" => {
			if operands.len() != 2 {
				return Err(EvalError::WrongArgCount(op.to_string(), 2, operands.len()).into());
			}

			match (&operands[0], &operands[1]) {
				(Type::Bool(l), Type::Bool(r)) => Ok(Type::Bool(*l && *r)),
				(l, r) => {
					Err(EvalError::InvalidTypes(
						op.to_string(),
						vec!["number".to_string(), "number".to_string()],
						vec![Type::render_type(l), Type::render_type(r)],
					)
					.into())
				},
			}
		},
		"or" => {
			if operands.len() != 2 {
				return Err(EvalError::WrongArgCount(op.to_string(), 2, operands.len()).into());
			}

			match (&operands[0], &operands[1]) {
				(Type::Bool(l), Type::Bool(r)) => Ok(Type::Bool(*l || *r)),
				(Type::Nil, Type::Nil) => Ok(Type::Bool(false)),
				(l, r) => {
					Err(EvalError::InvalidTypes(
						op.to_string(),
						vec!["number".to_string(), "number".to_string()],
						vec![Type::render_type(l), Type::render_type(r)],
					)
					.into())
				},
			}
		},
		"nil?" => {
			if operands.len() != 1 {
				return Err(EvalError::WrongArgCount(op.to_string(), 1, operands.len()).into());
			}

			match &operands[0] {
				Type::Nil => Ok(Type::Bool(true)),
				Type::List(l) => Ok(Type::Bool(l.len() == 0)),
				_ => Ok(Type::Bool(false)),
			}
		},
		"print" => {
			if operands.len() != 1 {
				return Err(EvalError::WrongArgCount(op.to_string(), 1, operands.len()).into());
			}

			println!("{}", operands[0]);

			Ok(Type::Nil)
		},
		"cons" => {
			if operands.len() != 2 {
				return Err(EvalError::WrongArgCount(op.to_string(), 2, operands.len()).into());
			}

			match (&operands[0], &operands[1]) {
				(l, r) => Ok(Type::List(vec![l.to_owned(), r.to_owned()])),
			}
		},
		"car" => {
			if operands.len() != 1 {
				return Err(EvalError::WrongArgCount(op.to_string(), 1, operands.len()).into());
			}

			match &operands[0] {
				Type::List(l) => Ok(l[0].clone()),
				t => {
					Err(EvalError::InvalidTypes(
						op.to_string(),
						vec!["list".to_string()],
						vec![Type::render_type(t)],
					)
					.into())
				},
			}
		},
		"cdr" => {
			if operands.len() != 1 {
				return Err(EvalError::WrongArgCount(op.to_string(), 1, operands.len()).into());
			}

			match &operands[0] {
				Type::List(l) => {
					let list = l[1..].to_vec();

					if list.len() == 1 {
						let elem = list[0].clone();
						match elem {
							Type::List(_) => Ok(elem),
							_ => Ok(Type::List(vec![elem])),
						}
					} else {
						let mut v = vec![];
						for e in list {
							v.push(e);
						}

						Ok(Type::List(v))
					}
				},
				t => {
					Err(EvalError::InvalidTypes(
						op.to_string(),
						vec!["list".to_string()],
						vec![Type::render_type(t)],
					)
					.into())
				},
			}
		},
		_ => Err(EvalError::UnknownSymbol(op.to_string()).into()),
	}
}
