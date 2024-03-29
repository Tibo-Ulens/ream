use std::fmt;
use std::sync::OnceLock;

use miette::SourceSpan;

/// Placeholder EndOfFile token that can be inserted by the parser
pub static EOF_TOKEN: OnceLock<Token> = OnceLock::new();

/// A single source code token
#[derive(Clone, Copy, Debug)]
pub struct Token<'t> {
	/// The region of source code wrapped by this token
	pub span: SourceSpan,
	/// The type of the token
	pub t:    TokenType<'t>,
}

impl<'t> Token<'t> {
	/// Change the span of self
	pub fn with_span(mut self, span: SourceSpan) -> Self {
		self.span = span;
		self
	}
}

/// All possible types of [`Token`]s
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType<'t> {
	TypeKwBottom,
	TypeKwTuple,
	TypeKwList,
	TypeKwFunction,
	TypeKwSum,
	TypeKwProduct,

	KwQuote,
	KwLet,
	KwFn,
	KwLambda,
	KwSeq,
	KwIf,
	KwInclude,

	Identifier(&'t str),
	Boolean(bool),
	Integer(u64),
	Float(f64),
	Character(char),
	String(&'t str),
	Atom(&'t str),

	LeftParen,
	RightParen,
	Period,
	Backtick,

	EndOfFile,
}

impl<'t> fmt::Display for TokenType<'t> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::TypeKwBottom => write!(f, "Bottom"),
			Self::TypeKwTuple => write!(f, "Tuple"),
			Self::TypeKwList => write!(f, "List"),
			Self::TypeKwFunction => write!(f, "Function"),
			Self::TypeKwSum => write!(f, "Sum"),
			Self::TypeKwProduct => write!(f, "Product"),
			Self::KwQuote => write!(f, "quote"),
			Self::KwLet => write!(f, "let"),
			Self::KwFn => write!(f, "fn"),
			Self::KwLambda => write!(f, "lambda"),
			Self::KwSeq => write!(f, "seq"),
			Self::KwIf => write!(f, "if"),
			Self::KwInclude => write!(f, "include"),
			Self::Identifier(id) => write!(f, "{id}"),
			Self::Boolean(b) => write!(f, "{b}"),
			Self::Integer(i) => write!(f, "{i}"),
			Self::Float(fl) => write!(f, "{fl}"),
			Self::Character(c) => write!(f, "{c}"),
			Self::String(s) => write!(f, "{s}"),
			Self::Atom(a) => write!(f, "{a}"),
			Self::LeftParen => write!(f, "("),
			Self::RightParen => write!(f, ")"),
			Self::Period => write!(f, "."),
			Self::Backtick => write!(f, "`"),
			Self::EndOfFile => write!(f, "EOF"),
		}
	}
}

impl<'t> TokenType<'t> {
	/// Get the name of this [`TokenType`]
	pub fn name(&self) -> String {
		match self {
			Self::TypeKwBottom => "Bottom".to_string(),
			Self::TypeKwTuple => "Tuple".to_string(),
			Self::TypeKwList => "List".to_string(),
			Self::TypeKwFunction => "Function".to_string(),
			Self::TypeKwSum => "Sum".to_string(),
			Self::TypeKwProduct => "Product".to_string(),
			Self::KwQuote => "quote".to_string(),
			Self::KwLet => "let".to_string(),
			Self::KwFn => "fn".to_string(),
			Self::KwLambda => "lambda".to_string(),
			Self::KwSeq => "begin".to_string(),
			Self::KwIf => "if".to_string(),
			Self::KwInclude => "include".to_string(),
			Self::Identifier(_) => "Identifier".to_string(),
			Self::Boolean(_) => "Boolean".to_string(),
			Self::Integer(_) => "Integer".to_string(),
			Self::Float(_) => "Float".to_string(),
			Self::Character(_) => "Character".to_string(),
			Self::String(_) => "String".to_string(),
			Self::Atom(_) => "Atom".to_string(),
			Self::LeftParen => "(".to_string(),
			Self::RightParen => ")".to_string(),
			Self::Period => ".".to_string(),
			Self::Backtick => "`".to_string(),
			Self::EndOfFile => "EndOfFile".to_string(),
		}
	}
}
