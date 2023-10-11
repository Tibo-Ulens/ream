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
#[derive(Clone, Debug, Diagnostic, Error)]
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
#[derive(Clone, Debug, Diagnostic, Error)]
pub enum ParseError {
	/// Error for testing
	#[allow(missing_docs)]
	#[error("test {loc:?}")]
	#[diagnostic(code(ream::test))]
	Test {
		#[label = "here"]
		loc: SourceSpan,
	},

	/// Unexpected end-of-file
	#[allow(missing_docs)]
	#[error("Unexpected end-of-file")]
	#[diagnostic(code(ream::parse_error::unexpected_eof))]
	UnexpectedEof {
		#[label = "here"]
		loc: SourceSpan,
	},

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

	/// Invalid expression
	#[allow(missing_docs)]
	#[error("Invalid Expression: found `{found}`, expected {}", format_expected_tokens(expected))]
	#[diagnostic(code(ream::parse_error::invalid_expression))]
	InvalidExpression {
		#[label = "here"]
		loc: SourceSpan,

		found:    String,
		expected: Vec<String>,
	},

	/// Invalid annotation type
	#[allow(missing_docs)]
	#[error("Invalid Annotation Type: found `{found}`, expected one of `:type`, `:doc`")]
	#[diagnostic(code(ream::parse_error::invalid_annotation))]
	InvalidAnnotation {
		#[label = "here"]
		loc: SourceSpan,

		found: String,
	},

	/// Invalid Datum
	#[allow(missing_docs)]
	#[error(
		"Invalid Datum: found `{found}`, expected one of `Identifier`, `Boolean`, `Integer`, \
		 `Float`, `Character`, `String`, `Atom`, `(`"
	)]
	#[diagnostic(code(ream::parse_error::invalid_datum))]
	InvalidDatum {
		#[label = "here"]
		loc: SourceSpan,

		found: String,
	},

	/// Invalid Lambda Formals
	#[allow(missing_docs)]
	#[error("Invalid Lambda Formals: found `{found}`, expected one of `Identifier`, `(`")]
	#[diagnostic(code(ream::parse_error::invalid_lambda_formals))]
	InvalidLambdaFormals {
		#[label = "here"]
		loc: SourceSpan,

		found: String,
	},
}

/// Any error related to evaluation
#[derive(Clone, Debug, Diagnostic, Error)]
pub enum EvalError {
	#[allow(missing_docs)]
	#[error("Could not find value for `{id}` in this scope")]
	#[diagnostic(code(ream::eval_error::unknown_identifier))]
	UnknownIdentifier {
		#[label = "here"]
		loc: SourceSpan,
		id:  String,
	},

	#[allow(missing_docs)]
	#[error("`{name}` is not a function")]
	#[diagnostic(code(ream::eval_error::not_a_function))]
	NotAFunction {
		#[label = "here"]
		loc:  SourceSpan,
		name: String,
	},

	#[allow(missing_docs)]
	#[error("`{callee}` takes {expected} arguments, got {found}")]
	#[diagnostic(code(ream::eval_error::wrong_argument_count))]
	WrongArgumentCount {
		#[label = "here"]
		loc:      SourceSpan,
		callee:   String,
		expected: usize,
		found:    usize,
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
