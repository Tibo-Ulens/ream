use miette::SourceSpan;

/// A single source code token
#[derive(Clone, Copy, Debug)]
pub struct Token<'t> {
	/// The region of source code wrapped by this token
	pub span: SourceSpan,
	/// The type of the token
	pub t:    TokenType<'t>,
}

/// All possible types of [`Token`]s
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType<'t> {
	Identifier(&'t str),

	KwQuote,
	KwLambda,
	KwIf,
	KwLet,

	Boolean(bool),
	Integer(u64),
	Float(f64),
	Character(char),
	String(&'t str),

	LeftParen,
	RightParen,
	VecParen,
	Colon,
	Period,
}
