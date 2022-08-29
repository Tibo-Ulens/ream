
/// All possible types of token
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TokenType {
	// Delimiters
	LeftParenthesis,
	RightParenthesis,

	// Keywords
	QuoteKW,
	BeginKW,
	LambdaKW,
	IfKW,
	DefineKW,
	SetKW,

	// Literals
	Bool(bool),
	Number(f64),
	String(String),
	Nil,

	// Identifiers
	Identifier(String),
}

impl std::fmt::Display for TokenType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::LeftParenthesis => write!(f, "{:<16}", "LeftParenthesis"),
			Self::RightParenthesis => write!(f, "{:<16}", "RightParenthesis"),
			Self::QuoteKW => write!(f, "{:<16}", "QuoteKW"),
			Self::BeginKW => write!(f, "{:<16}", "BeginKW"),
			Self::LambdaKW => write!(f, "{:<16}", "LambdaKW"),
			Self::IfKW => write!(f, "{:<16}", "IfKW"),
			Self::DefineKW => write!(f, "{:<16}", "DefineKW"),
			Self::SetKW => write!(f, "{:<16}", "SetKw"),
			Self::Bool(_) => write!(f, "{:<16}", "Bool"),
			Self::Number(_) => write!(f, "{:<16}", "Number"),
			Self::String(_) => write!(f, "{:<16}", "String"),
			Self::Nil => write!(f, "{:<16}", "Nil"),
			Self::Identifier(_) => write!(f, "{:<16}", "Identifier"),
		}
	}
}

/// A single lexeme
#[derive(Debug, Clone)]
pub(crate) struct Token {
	pub(crate) ty:   TokenType,
	pub(crate) repr: String,
	pub(crate) line: usize,
	pub(crate) col:  usize,
}
