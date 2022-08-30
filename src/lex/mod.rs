use regex::Regex;

use crate::error::{Error, LexError};

mod token;

pub(crate) use token::*;

lazy_static! {
	static ref NUMBER_REGEX: Regex = Regex::new(r"^[0-9]+(?:\.[0-9]+)?$").unwrap();
	static ref IDENTIFIER_REGEX: Regex =
		Regex::new(r"(?:[a-zA-Z!$%&*/:<=>?^_~][a-zA-Z!$%&*/:<=>?^_~0-9+\-.@]*)|[+-]").unwrap();
}

impl std::fmt::Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let loc_repr = format!("[{}:{}]", self.line, self.col);
		write!(f, "{:<9} {} `{}`", loc_repr, self.ty, self.repr)
	}
}

pub(crate) struct Lexer<'s> {
	source:     &'s [char],
	source_len: usize,
	tokens:     Vec<Token>,

	/// Start of the current token
	start: usize,
	/// Index of the current grapheme
	idx:   usize,

	line: usize,
	col:  usize,
}

impl<'s> Lexer<'s> {
	/// Create a new lexer for a given list of graphemes
	pub(crate) fn new(source: &'s [char]) -> Self {
		Self { source, source_len: source.len(), tokens: vec![], start: 0, idx: 0, line: 1, col: 1 }
	}

	fn make_token(&self, ty: TokenType) -> Token {
		let repr = self.get_string();

		Token { ty, repr, line: self.line, col: self.col }
	}

	/// Lexes the entire source
	pub(crate) fn lex(&mut self) -> Result<Vec<Token>, Error> {
		while self.idx < self.source_len {
			self.start = self.idx;

			let next = self.next();
			if self.skip_whitespace_and_comments(next) {
				continue;
			}

			let token = match next {
				'(' => {
					match self.peek() {
						')' => {
							// Skip the closing paren
							self.idx += 1;

							let t = self.make_token(TokenType::Nil);

							self.col += 1;

							t
						},
						_ => self.make_token(TokenType::LeftParenthesis),
					}
				},
				')' => self.make_token(TokenType::RightParenthesis),
				'#' => {
					match self.next() {
						't' => {
							let t = self.make_token(TokenType::Bool(true));
							self.col += 1;

							t
						},
						'f' => {
							let t = self.make_token(TokenType::Bool(false));
							self.col += 1;

							t
						},
						c => {
							return Err(LexError::InvalidConstant {
								constant: c.to_string(),
								line:     self.line,
								col:      self.col,
							}
							.into());
						},
					}
				},
				'"' => {
					let i = self.idx;
					while self.peek() != '"' && self.idx < self.source_len {
						self.idx += 1;
					}

					// Skip the opening quote
					self.start += 1;
					let raw = self.get_string();

					let t = self.make_token(TokenType::String(raw));

					// Skip the closing quote
					self.idx += 1;
					self.col += self.idx - i;

					t
				},
				_ => {
					let i = self.idx;
					while !(Self::is_delimiter(self.peek())) {
						self.idx += 1;
					}

					let raw = self.get_string();

					if NUMBER_REGEX.is_match(&raw) {
						let num: f64 = raw.parse().unwrap_or_else(|_| {
							panic!("invalid number at [{}:{}]", self.line, self.col)
						});

						let t = self.make_token(TokenType::Number(num));

						self.col += self.idx - i;

						t
					} else if IDENTIFIER_REGEX.is_match(&raw) {
						let t = self.match_identifier(&raw);

						self.col += self.idx - i;

						t
					} else {
						return Err(LexError::InvalidIdentifier {
							identifier: raw,
							line:       self.line,
							col:        self.col,
						}
						.into());
					}
				},
			};

			self.col += 1;

			self.tokens.push(token);
		}

		Ok(self.tokens.to_owned())
	}

	/// Return the value of the current char
	fn peek(&self) -> char { self.source[self.idx] }

	/// Advance one char and return its value
	fn next(&mut self) -> char {
		let next = self.source[self.idx];
		self.idx += 1;
		next
	}

	/// See if an identifier is a keyword or not and return the appropriate
	/// token
	fn match_identifier(&self, id: &str) -> Token {
		match id {
			"quote" => self.make_token(TokenType::QuoteKW),
			"begin" => self.make_token(TokenType::BeginKW),
			"lambda" => self.make_token(TokenType::LambdaKW),
			"if" => self.make_token(TokenType::IfKW),
			"define" => self.make_token(TokenType::DefineKW),
			"set!" => self.make_token(TokenType::SetKW),
			i => self.make_token(TokenType::Identifier(i.to_string())),
		}
	}

	/// See if the current char is whitespace/comment, update self accordingly
	/// and return whether or not further matching should occur
	fn skip_whitespace_and_comments(&mut self, c: char) -> bool {
		match c {
			'\n' => {
				self.line += 1;
				self.col = 1;
				true
			},
			' ' | '\t' => {
				self.col += 1;
				true
			},
			';' => {
				while self.next() != '\n' && self.idx < self.source_len {}
				true
			},
			_ => false,
		}
	}

	fn is_delimiter(c: char) -> bool {
		c == '\n' || c == ' ' || c == '\t' || c == '(' || c == ')' || c == ';'
	}

	fn get_string(&self) -> String { self.source[self.start..self.idx].iter().collect() }
}
