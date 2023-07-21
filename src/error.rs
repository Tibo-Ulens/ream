//! Error definition

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

/// Any possible error
#[allow(missing_docs)]
#[derive(Debug, Diagnostic, Error)]
pub enum Error {
	#[error(transparent)]
	#[diagnostic(code(ream::io_error))]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	#[diagnostic(code(ream::lex_error))]
	Lex(#[from] LexError),

	#[error(transparent)]
	#[diagnostic(code(ream::parse_error))]
	Parse(#[from] ParseError),
}

/// Any error related to lexing
#[derive(Debug, Diagnostic, Error)]
pub enum LexError {
	/// Unexpected end-of-file
	#[allow(missing_docs)]
	#[error("Unexpected end-of-file")]
	#[diagnostic(code(ream::lex_error::unexpected_eof))]
	UnexpectedEof {
		#[label = "here"]
		loc: SourceSpan,
	},

	/// Expected one symbol, found another
	#[allow(missing_docs)]
	#[error("Unexpected Symbol: found {found:?}, expected {}", format_expected_symbols(expected))]
	#[diagnostic(code(ream::lex_error::unexpected_symbol))]
	UnexpectedSymbol {
		#[label = "here"]
		loc: SourceSpan,

		found:    char,
		expected: Vec<char>,
	},

	/// Invalid boolean
	#[allow(missing_docs)]
	#[error("Invalid Boolean: {found:?}")]
	#[diagnostic(help("valid boolean literals are `#t`, `#true`, `#f`, and `#false`"))]
	#[diagnostic(code(ream::lex_error::invalid_boolean))]
	InvalidBoolean {
		#[label = "here"]
		loc: SourceSpan,

		found: String,
	},

	/// Invalid escape sequence
	#[allow(missing_docs)]
	#[error("Invalid Escape Sequence: {found:?}")]
	#[diagnostic(code(ream::lex_error::invalid_escape))]
	InvalidEscape {
		#[label = "here"]
		loc: SourceSpan,

		found: String,
	},

	/// Invalid number
	#[allow(missing_docs)]
	#[error("Invalid Number: {found:?}")]
	#[diagnostic(code(ream::lex_error::invalid_number))]
	InvalidNumber {
		#[label = "here"]
		loc:  SourceSpan,
		#[help]
		help: Option<String>,

		found: String,
	},

	/// Unknown symbol
	#[allow(missing_docs)]
	#[error("Unknown Symbol: {found:?}")]
	#[diagnostic(code(ream::lex_error::unexpected_symbol))]
	UnknownSymbol {
		#[label = "here"]
		loc: SourceSpan,

		found: char,
	},
}

/// Any error related to parsing
#[derive(Debug, Diagnostic, Error)]
pub enum ParseError {
	/// Expected one token, found another
	#[allow(missing_docs)]
	#[error("Unexpected Token: found `{found}`, expected {}", format_expected_tokens(expected))]
	#[diagnostic(code(ream::parse_error::unexpected_token))]
	UnexpectedToken {
		#[label = "here"]
		loc: SourceSpan,

		found:    String,
		expected: Vec<String>,
	},
}

fn format_expected_symbols(ex: &[char]) -> String {
	if ex.len() == 1 {
		format!("`{}`", ex[0])
	} else {
		format!("one of {}", ex.iter().map(|e| format!("`{}`", e)).collect::<Vec<_>>().join(", "))
	}
}

fn format_expected_tokens(ex: &[String]) -> String {
	if ex.len() == 1 {
		format!("`{}`", ex[0])
	} else {
		format!("one of {}", ex.iter().map(|e| format!("`{}`", e)).collect::<Vec<_>>().join(", "))
	}
}
