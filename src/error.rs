use crate::lex::Token;

#[derive(Debug, Error)]
pub(crate) enum Error {
	#[error(transparent)]
	Io(#[from] std::io::Error),
	#[error(transparent)]
	Lex(#[from] LexError),
	#[error(transparent)]
	Parse(#[from] ParseError),
}

#[derive(Debug, Error)]
pub(crate) enum LexError {
	#[error("[{line}:{col}]: invalid constant {constant}")]
	InvalidConstant { constant: String, line: usize, col: usize },
	#[error("[{line}:{col}]: invalid identifier `{identifier}` ")]
	InvalidIdentifier { identifier: String, line: usize, col: usize },
}

#[derive(Debug, Error)]
pub(crate) enum ParseError {
	#[error("[{}:{}]: unexpected token `{}`", get_line(.0), get_col(.0), get_repr(.0))]
	UnexpectedToken(Token),
	#[error(
		"[{}:{}]: expected {expected} but found `{}`",
		get_line(.found),
		get_col(.found),
		get_repr(.found)
	)]
	Expected { expected: String, found: Token },
}

fn get_line(t: &Token) -> usize { t.line }

fn get_col(t: &Token) -> usize { t.col }

fn get_repr(t: &Token) -> String { t.repr.clone() }
