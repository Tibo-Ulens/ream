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

	KwBottom,
	KwTuple,
	KwList,
	KwVector,
	KwFuntion,
	KwSum,
	KwProduct,

	KwQuote,
	KwLet,
	KwBegin,
	KwLambda,
	KwIf,

	Boolean(bool),
	Integer(u64),
	Float(f64),
	Character(char),
	String(&'t str),
	Atom(&'t str),

	LeftParen,
	RightParen,
	VecParen,
	Period,
}
