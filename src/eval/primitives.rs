use super::value::ReamType;
use crate::eval::Eval;
use crate::EvalError;

macro_rules! check_arg_count {
	($location:expr, $operator:expr, $expected_arg_count:expr, $given_arg_count:expr) => {
		if $expected_arg_count != $given_arg_count {
			return Err(EvalError::WrongArgumentCount {
				loc:      $location,
				callee:   $operator,
				expected: $expected_arg_count,
				found:    $given_arg_count,
			});
		}
	};
}

pub(super) const ADD<'s>: ReamType<'s> = ReamType::Primitive(|l, i, a, s| {
	check_arg_count!(l, i, 2, a.len());

	let [a, b]: [_; 2] = a.try_into().unwrap();

	let a = a.eval(s.clone())?;
	let b = b.eval(s)?;

	match (a.t, b.t) {
		(ReamType::Integer(a), ReamType::Integer(b)) => Ok(ReamType::Integer(a + b)),
		(ReamType::Integer(_), b_t) => {
			Err(EvalError::WrongType {
				loc:      b.span,
				expected: "Integer".to_string(),
				found:    b_t.type_name(),
			})
		},

		(ReamType::Float(a), ReamType::Float(b)) => Ok(ReamType::Float(a + b)),
		(ReamType::Float(_), b_t) => {
			Err(EvalError::WrongType {
				loc:      b.span,
				expected: "Float".to_string(),
				found:    b_t.type_name(),
			})
		},

		(a_t, _) => {
			Err(EvalError::WrongType {
				loc:      a.span,
				expected: "Integer or Float".to_string(),
				found:    a_t.type_name(),
			})
		},
	}
});
