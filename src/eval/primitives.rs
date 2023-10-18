use super::value::ReamType;
use crate::eval::Eval;
use crate::EvalError;

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! generate_primitive {
	($prim_vis:vis $prim_name:ident ($($argument:ident),*) => {
		$(
			($( $argument_matcher:pat ),+) => Ok($result:expr)
		),+

		$(
			( $( $error_matcher:pat_param ),+ ) => Err($err_result:expr)
		),*
	}) => {
		// #[rustfmt::skip]
		$prim_vis const $prim_name<'s>: ReamType<'s> =  ReamType::Primitive::<'s>(|l, i, a, s| {
			const __EXPECTED_ARG_COUNT: usize = count!($( $argument )*);
			let __given_arg_count = a.len();

			if __EXPECTED_ARG_COUNT != a.len() {
				return Err(EvalError::WrongArgumentCount {
					loc:      l,
					callee:   i,
					expected: __EXPECTED_ARG_COUNT,
					found:    __given_arg_count,
				});
			}

			let [$( $argument ),*]: [_; __EXPECTED_ARG_COUNT] = a.try_into().unwrap();

			$(
				let $argument = $argument.eval(s.clone())?;
			)*

			#[allow(unused_parens)]
			match ($( $argument.t ),*) {
				$(
					($( $argument_matcher ),+) => {
						Ok::<ReamType, EvalError>($result)
					},
				)+

				$(
					($( $error_matcher ),+ ) => {
						Err::<ReamType, EvalError>($err_result)
					},
				)*
			}
		});
	};
}

generate_primitive! {
	pub(super) ADD (a, b) => {
		(ReamType::Integer(a), ReamType::Integer(b)) => Ok(ReamType::Integer(a + b)),
		(ReamType::Float(a), ReamType::Float(b)) => Ok(ReamType::Float(a + b))

		(a_t @ ReamType::Integer(_), b_t) => Err(EvalError::WrongType {
			loc: b.span,
			expected: a_t.type_name(),
			found: b_t.type_name(),
		}),
		(a_t @ ReamType::Float(_), b_t) => Err(EvalError::WrongType {
			loc: b.span,
			expected: a_t.type_name(),
			found: b_t.type_name(),
		}),

		(a_t, _) => Err(EvalError::WrongType {
			loc: b.span,
			expected: "Integer or Float".to_string(),
			found: a_t.type_name(),
		})
	}
}

generate_primitive! {
	pub(super) SUB (a, b) => {
		(ReamType::Integer(a), ReamType::Integer(b)) => Ok(ReamType::Integer(a - b)),
		(ReamType::Float(a), ReamType::Float(b)) => Ok(ReamType::Float(a - b))

		(a_t @ ReamType::Integer(_), b_t) => Err(EvalError::WrongType {
			loc: b.span,
			expected: a_t.type_name(),
			found: b_t.type_name(),
		}),
		(a_t @ ReamType::Float(_), b_t) => Err(EvalError::WrongType {
			loc: b.span,
			expected: a_t.type_name(),
			found: b_t.type_name(),
		}),

		(a_t, _) => Err(EvalError::WrongType {
			loc: b.span,
			expected: "Integer or Float".to_string(),
			found: a_t.type_name(),
		})
	}
}

generate_primitive! {
	pub(super) MUL (a, b) => {
		(ReamType::Integer(a), ReamType::Integer(b)) => Ok(ReamType::Integer(a * b)),
		(ReamType::Float(a), ReamType::Float(b)) => Ok(ReamType::Float(a * b))

		(a_t @ ReamType::Integer(_), b_t) => Err(EvalError::WrongType {
			loc: b.span,
			expected: a_t.type_name(),
			found: b_t.type_name(),
		}),
		(a_t @ ReamType::Float(_), b_t) => Err(EvalError::WrongType {
			loc: b.span,
			expected: a_t.type_name(),
			found: b_t.type_name(),
		}),

		(a_t, _) => Err(EvalError::WrongType {
			loc: b.span,
			expected: "Integer or Float".to_string(),
			found: a_t.type_name(),
		})
	}
}

generate_primitive! {
	pub(super) DIV (a, b) => {
		(ReamType::Integer(a), ReamType::Integer(b)) => Ok(ReamType::Integer(a / b)),
		(ReamType::Float(a), ReamType::Float(b)) => Ok(ReamType::Float(a / b))

		(a_t @ ReamType::Integer(_), b_t) => Err(EvalError::WrongType {
			loc: b.span,
			expected: a_t.type_name(),
			found: b_t.type_name(),
		}),
		(a_t @ ReamType::Float(_), b_t) => Err(EvalError::WrongType {
			loc: b.span,
			expected: a_t.type_name(),
			found: b_t.type_name(),
		}),

		(a_t, _) => Err(EvalError::WrongType {
			loc: b.span,
			expected: "Integer or Float".to_string(),
			found: a_t.type_name(),
		})
	}
}

generate_primitive! {
	pub(super) EQU (a, b) => {
		(ReamType::Boolean(a), ReamType::Boolean(b)) => Ok(ReamType::Boolean(a == b)),
		(ReamType::Integer(a), ReamType::Integer(b)) => Ok(ReamType::Boolean(a == b)),
		(ReamType::Float(a), ReamType::Float(b)) => Ok(ReamType::Boolean(a == b)),
		(ReamType::Character(a), ReamType::Character(b)) => Ok(ReamType::Boolean(a == b)),
		(ReamType::String(a), ReamType::String(b)) => Ok(ReamType::Boolean(a == b)),
		(ReamType::Identifier(a), ReamType::Identifier(b)) => Ok(ReamType::Boolean(a == b)),
		(ReamType::Atom(a), ReamType::Atom(b)) => Ok(ReamType::Boolean(a == b)),
		(ReamType::Unit, ReamType::Unit) => Ok(ReamType::Boolean(true))

		(a_t, _) => Err(EvalError::WrongType {
			loc: b.span,
			expected: "Boolean or Integer or Float or Character or String or Identifier or Atom \
					   or Unit".to_string(),
			found: a_t.type_name(),
		})
	}
}

generate_primitive! {
	pub(super) NEQ (a, b) => {
		(ReamType::Boolean(a), ReamType::Boolean(b)) => Ok(ReamType::Boolean(a != b)),
		(ReamType::Integer(a), ReamType::Integer(b)) => Ok(ReamType::Boolean(a != b)),
		(ReamType::Float(a), ReamType::Float(b)) => Ok(ReamType::Boolean(a != b)),
		(ReamType::Character(a), ReamType::Character(b)) => Ok(ReamType::Boolean(a != b)),
		(ReamType::String(a), ReamType::String(b)) => Ok(ReamType::Boolean(a != b)),
		(ReamType::Identifier(a), ReamType::Identifier(b)) => Ok(ReamType::Boolean(a != b)),
		(ReamType::Atom(a), ReamType::Atom(b)) => Ok(ReamType::Boolean(a != b)),
		(ReamType::Unit, ReamType::Unit) => Ok(ReamType::Boolean(false))

		(a_t, _) => Err(EvalError::WrongType {
			loc: b.span,
			expected: "Boolean or Integer or Float or Character or String or Identifier or Atom \
					   or Unit".to_string(),
			found: a_t.type_name(),
		})
	}
}

generate_primitive! {
	pub(super) GT (a, b) => {
		(ReamType::Boolean(a), ReamType::Boolean(b)) => Ok(ReamType::Boolean(a & !b)),
		(ReamType::Integer(a), ReamType::Integer(b)) => Ok(ReamType::Boolean(a > b)),
		(ReamType::Float(a), ReamType::Float(b)) => Ok(ReamType::Boolean(a > b)),
		(ReamType::Character(a), ReamType::Character(b)) => Ok(ReamType::Boolean(a > b)),
		(ReamType::String(a), ReamType::String(b)) => Ok(ReamType::Boolean(a > b)),
		(ReamType::Identifier(a), ReamType::Identifier(b)) => Ok(ReamType::Boolean(a > b)),
		(ReamType::Atom(a), ReamType::Atom(b)) => Ok(ReamType::Boolean(a > b)),
		(ReamType::Unit, ReamType::Unit) => Ok(ReamType::Boolean(false))

		(a_t, _) => Err(EvalError::WrongType {
			loc: b.span,
			expected: "Boolean or Integer or Float or Character or String or Identifier or Atom \
					   or Unit".to_string(),
			found: a_t.type_name(),
		})
	}
}

generate_primitive! {
	pub(super) GTE (a, b) => {
		(ReamType::Boolean(a), ReamType::Boolean(b)) => Ok(ReamType::Boolean(a >= b)),
		(ReamType::Integer(a), ReamType::Integer(b)) => Ok(ReamType::Boolean(a >= b)),
		(ReamType::Float(a), ReamType::Float(b)) => Ok(ReamType::Boolean(a >= b)),
		(ReamType::Character(a), ReamType::Character(b)) => Ok(ReamType::Boolean(a >= b)),
		(ReamType::String(a), ReamType::String(b)) => Ok(ReamType::Boolean(a >= b)),
		(ReamType::Identifier(a), ReamType::Identifier(b)) => Ok(ReamType::Boolean(a >= b)),
		(ReamType::Atom(a), ReamType::Atom(b)) => Ok(ReamType::Boolean(a >= b)),
		(ReamType::Unit, ReamType::Unit) => Ok(ReamType::Boolean(false))

		(a_t, _) => Err(EvalError::WrongType {
			loc: b.span,
			expected: "Boolean or Integer or Float or Character or String or Identifier or Atom \
					   or Unit".to_string(),
			found: a_t.type_name(),
		})
	}
}

generate_primitive! {
	pub(super) LT (a, b) => {
		(ReamType::Boolean(a), ReamType::Boolean(b)) => Ok(ReamType::Boolean(a & !b)),
		(ReamType::Integer(a), ReamType::Integer(b)) => Ok(ReamType::Boolean(a > b)),
		(ReamType::Float(a), ReamType::Float(b)) => Ok(ReamType::Boolean(a > b)),
		(ReamType::Character(a), ReamType::Character(b)) => Ok(ReamType::Boolean(a > b)),
		(ReamType::String(a), ReamType::String(b)) => Ok(ReamType::Boolean(a > b)),
		(ReamType::Identifier(a), ReamType::Identifier(b)) => Ok(ReamType::Boolean(a > b)),
		(ReamType::Atom(a), ReamType::Atom(b)) => Ok(ReamType::Boolean(a > b)),
		(ReamType::Unit, ReamType::Unit) => Ok(ReamType::Boolean(false))

		(a_t, _) => Err(EvalError::WrongType {
			loc: b.span,
			expected: "Boolean or Integer or Float or Character or String or Identifier or Atom \
					   or Unit".to_string(),
			found: a_t.type_name(),
		})
	}
}

generate_primitive! {
	pub(super) LTE (a, b) => {
		(ReamType::Boolean(a), ReamType::Boolean(b)) => Ok(ReamType::Boolean(a >= b)),
		(ReamType::Integer(a), ReamType::Integer(b)) => Ok(ReamType::Boolean(a >= b)),
		(ReamType::Float(a), ReamType::Float(b)) => Ok(ReamType::Boolean(a >= b)),
		(ReamType::Character(a), ReamType::Character(b)) => Ok(ReamType::Boolean(a >= b)),
		(ReamType::String(a), ReamType::String(b)) => Ok(ReamType::Boolean(a >= b)),
		(ReamType::Identifier(a), ReamType::Identifier(b)) => Ok(ReamType::Boolean(a >= b)),
		(ReamType::Atom(a), ReamType::Atom(b)) => Ok(ReamType::Boolean(a >= b)),
		(ReamType::Unit, ReamType::Unit) => Ok(ReamType::Boolean(false))

		(a_t, _) => Err(EvalError::WrongType {
			loc: b.span,
			expected: "Boolean or Integer or Float or Character or String or Identifier or Atom \
					   or Unit".to_string(),
			found: a_t.type_name(),
		})
	}
}

generate_primitive! {
	pub(super) PRINT (a) => {
		(a) => Ok({
			println!("{a}");
			ReamType::Unit
		})
	}
}
