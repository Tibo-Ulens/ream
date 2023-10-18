use std::iter::Peekable;
use std::str::Chars;

use miette::SourceSpan;
use unicode_xid::UnicodeXID;

use crate::{LexError, Token, TokenType};

static NON_DECIMAL_FLOAT_LITERAL: &str =
	"this number appears to be a float, however floats can only be created using decimal notation";

/// A lexer for a single source file
#[allow(missing_docs)]
#[derive(Clone)]
pub struct Lexer<'s> {
	source: &'s str,
	chars:  Peekable<Chars<'s>>,
	len:    usize,

	/// The start of the current token
	start: usize,
	/// The current index into the character list
	idx:   usize,
}

impl<'s> Iterator for Lexer<'s> {
	type Item = Result<Token<'s>, LexError>;

	fn next(&mut self) -> Option<Self::Item> { self.lex_token() }
}

impl<'s> Lexer<'s> {
	/// Create a new lexer
	pub fn new(source: &'s str) -> Self {
		let chars = source.chars().peekable();
		let len = source.chars().count();

		Self { source, chars, len, start: 0, idx: 0 }
	}

	/// Peek at the next [`char`]
	///
	/// Returns [`None`] if no characters are left
	fn peek(&mut self) -> Option<&char> { self.chars.peek() }

	/// Consume and return the next [`char`]
	///
	/// Returns [`None`] if no characters are left
	fn next(&mut self) -> Option<char> {
		self.idx += 1;
		self.chars.next()
	}

	/// Check if a character can start an identifier
	fn is_id_start(c: char) -> bool {
		UnicodeXID::is_xid_start(c)
			|| c == '!' || c == '$'
			|| c == '%' || c == '&'
			|| c == '*' || c == '/'
			|| c == '<' || c == '='
			|| c == '>' || c == '?'
			|| c == '^' || c == '_'
			|| c == '~' || c == ':'
			|| c == '+' || c == '-'
	}

	/// Check if a character can continue an identifier
	fn is_id_continue(c: char) -> bool {
		Self::is_id_start(c)
			|| UnicodeXID::is_xid_continue(c)
			|| c.is_numeric()
			|| c == '.' || c == '@'
	}

	/// Check if a character is a delimiter
	fn is_delimiter(c: char) -> bool {
		c.is_whitespace() || c == '(' || c == ')' || c == '"' || c == '\'' || c == ';' || c == '`'
	}

	/// Lex a single token
	pub fn lex_token(&mut self) -> Option<Result<Token<'s>, LexError>> {
		// Consume any leading whitespace
		self.trim()?;

		// take_whitespace updates self.idx, so self.start should be updated
		// accordingly to mark the start of a new token
		self.start = self.idx;

		match self.next()? {
			'(' => Some(Ok(Token { span: (self.start, 1).into(), t: TokenType::LeftParen })),
			')' => Some(Ok(Token { span: (self.start, 1).into(), t: TokenType::RightParen })),
			'.' => Some(Ok(Token { span: (self.start, 1).into(), t: TokenType::Period })),
			'`' => Some(Ok(Token { span: (self.start, 1).into(), t: TokenType::Backtick })),
			':' => Some(self.make_atom_token()),
			'#' => {
				match self.peek()? {
					't' | 'f' => Some(self.make_boolean_token()),
					&c => {
						Some(Err(LexError::UnexpectedSymbol {
							loc:      (self.start, 1).into(),
							found:    c,
							expected: vec!['t', 'f'],
						}))
					},
				}
			},
			'\'' => Some(self.make_character_token()),
			'"' => Some(self.make_string_token()),
			n if n.is_ascii_digit() => Some(self.make_number_token()),
			c if Self::is_id_start(c) => Some(self.make_identifier_token()),
			c => Some(Err(LexError::UnknownSymbol { loc: (self.start, 1).into(), found: c })),
		}
	}

	/// Consume any available whitespace characters and/or comments, updating
	/// the [`Lexer`]s state as it goes along
	///
	/// Returns [`None`] if no characters are left
	fn trim(&mut self) -> Option<()> {
		match self.peek()? {
			';' => {
				let _ = self.take_chars_while(|c| c != '\n');

				self.trim()
			},
			' ' | '\t' | '\n' => {
				// Unwrap is safe as peek is some
				self.next().unwrap();

				self.trim()
			},
			_ => Some(()),
		}
	}

	/// Keep taking characters while a predicate holds true
	///
	/// Returns the slice of characters that satisfied the predicate, from the
	/// start of the current token up to, and including, the last character
	/// that satisfied the predicate
	fn take_chars_while<F>(&mut self, pred: F) -> Result<&'s str, LexError>
	where
		F: Fn(char) -> bool,
	{
		// Return early if the immediately following character is None
		let mut peek = match self.peek() {
			Some(p) => *p,
			None => return Err(LexError::UnexpectedEof { loc: (self.idx, 1).into() }),
		};

		while pred(peek) {
			// Unwrap is safe as the previous iteration of the loop assures
			// there is a character
			self.next().unwrap();

			if self.idx >= self.len {
				return Err(LexError::UnexpectedEof { loc: (self.idx, 1).into() });
			}

			// Unwrap is safe as idx < len
			peek = *self.peek().unwrap();
		}

		Ok(&self.source[self.start..self.idx])
	}

	/// Attempt to make an atom starting from the lexers current position
	/// in the source
	fn make_atom_token(&mut self) -> Result<Token<'s>, LexError> {
		let atom = self.take_chars_while(|c| !Self::is_delimiter(c))?;

		Ok(Token { span: (self.start, atom.len()).into(), t: TokenType::Atom(atom) })
	}

	/// Attempt to make a boolean starting from the lexers current position
	/// in the source
	fn make_boolean_token(&mut self) -> Result<Token<'s>, LexError> {
		let raw = self.take_chars_while(|c| !Self::is_delimiter(c))?;

		if raw == "#t" || raw == "#true" {
			Ok(Token { span: (self.start, raw.len()).into(), t: TokenType::Boolean(true) })
		} else if raw == "#f" || raw == "#false" {
			Ok(Token { span: (self.start, raw.len()).into(), t: TokenType::Boolean(false) })
		} else {
			Err(LexError::InvalidBoolean {
				loc:   (self.start, raw.len()).into(),
				found: raw.to_string(),
			})
		}
	}

	/// Convert a string with a 2 character escape code into its corresponding character
	fn unescape_string_to_char(&self, string: &str, loc: SourceSpan) -> Result<char, LexError> {
		match string {
			"\\n" => Ok('\n'),
			"\\r" => Ok('\r'),
			"\\t" => Ok('\t'),
			"\\\\" => Ok('\\'),
			"\\0" => Ok('\0'),
			"\\'" => Ok('\''),
			_ => Err(LexError::InvalidEscape { loc, found: string.to_string() }),
		}
	}

	/// Attempt to make a character starting from the lexers current position
	/// in the source
	///
	/// Supported escape sequences:
	///  - `\n` - line feed
	///  - `\r` - carriage return
	///  - `\t` - htab
	///  - `\\` - backslash
	///  - `\0` - null
	///  - `\'` - single quote
	fn make_character_token(&mut self) -> Result<Token<'s>, LexError> {
		// Return early if the immediately following character is None
		let chr = match self.next() {
			Some(c) => c,
			None => {
				return Err(LexError::UnexpectedEof { loc: (self.start + 1, 1).into() });
			},
		};

		if chr == '\\' {
			let escaped = match self.next() {
				Some(c) => c,
				None => {
					return Err(LexError::UnexpectedEof { loc: (self.start + 2, 1).into() });
				},
			};

			let close = match self.next() {
				Some(c) => c,
				None => {
					return Err(LexError::UnexpectedEof { loc: (self.start + 3, 1).into() });
				},
			};

			if close != '\'' {
				return Err(LexError::UnexpectedSymbol {
					loc:      (self.start + 3, 1).into(),
					found:    close,
					expected: vec!['\''],
				});
			}

			let mut unescaped_str = String::from(chr);
			unescaped_str.push(escaped);

			let escaped_char =
				self.unescape_string_to_char(&unescaped_str, (self.start + 1, 2).into())?;

			return Ok(Token {
				span: (self.start, 4).into(),
				t:    TokenType::Character(escaped_char),
			});
		}

		let close = match self.next() {
			Some(c) => c,
			None => {
				return Err(LexError::UnexpectedEof { loc: (self.start + 2, 1).into() });
			},
		};

		if close != '\'' {
			return Err(LexError::UnexpectedSymbol {
				loc:      (self.start + 2, 1).into(),
				found:    close,
				expected: vec!['\''],
			});
		}

		Ok(Token { span: (self.start, 3).into(), t: TokenType::Character(chr) })
	}

	/// Attempt to make a string starting from the lexers current position
	/// in the source until a non-escaped " is found"
	fn make_string_token(&mut self) -> Result<Token<'s>, LexError> {
		// Return early if the immediately following character is None
		let mut peek = match self.peek() {
			Some(c) => *c,
			None => {
				return Err(LexError::UnexpectedEof { loc: (self.start + 1, 1).into() });
			},
		};

		let mut i = 0;
		let mut prev = ' ';
		// Keep looping until a `"` without a preceding `\` is found
		while !(peek == '"' && prev != '\\') {
			// Unwrap is safe as the previous iteration of the loop assures
			// there is a character
			self.next().unwrap();

			if self.idx >= self.len {
				return Err(LexError::UnexpectedEof { loc: (self.start + i + 2, 1).into() });
			}

			prev = peek;
			// Unwrap is safe as idx < len
			peek = *self.peek().unwrap();
			i += 1;
		}

		// Take the closing quote
		//
		// Unwrap is safe as the last iteration of the loop assures the next
		// character is `"`
		self.next().unwrap();

		// + and - 1 to ignore the quotes
		let string_literal = &self.source[self.start + 1..self.idx - 1];

		Ok(Token {
			span: (self.start, string_literal.len()).into(),
			t:    TokenType::String(string_literal),
		})
	}

	/// Attempt to make a number starting from the lexers current position
	/// in the source
	///
	/// Can make decimal, hex, octal, or binary integers, or decimal floats.
	fn make_number_token(&mut self) -> Result<Token<'s>, LexError> {
		let raw = self.take_chars_while(|c| {
			c.is_ascii_hexdigit()
				|| c == 'x' || c == 'X'
				|| c == 'o' || c == 'O'
				|| c == '_' || c == '.'
		})?;

		let raw = raw.replace('_', "");

		if (raw.starts_with("0x") || raw.starts_with("0o") || raw.starts_with("0b"))
			&& raw.contains('.')
		{
			return Err(LexError::InvalidNumber {
				loc:   (self.start, raw.len()).into(),
				help:  Some(NON_DECIMAL_FLOAT_LITERAL.to_string()),
				found: raw,
			});
		}

		if raw.contains('.') {
			let float = raw.parse::<f64>().map_err(|_| {
				LexError::InvalidNumber {
					loc:   (self.start, raw.len()).into(),
					help:  None,
					found: raw.to_string(),
				}
			})?;

			return Ok(Token {
				span: (self.start, raw.len()).into(),
				t:    TokenType::Float(float),
			});
		}

		let num = if raw.starts_with("0x") {
			u64::from_str_radix(raw.trim_start_matches("0x"), 16).map_err(|_| {
				LexError::InvalidNumber {
					loc:   (self.start, raw.len()).into(),
					help:  None,
					found: raw.to_string(),
				}
			})?
		} else if raw.starts_with("0o") {
			u64::from_str_radix(raw.trim_start_matches("0o"), 8).map_err(|_| {
				LexError::InvalidNumber {
					loc:   (self.start, raw.len()).into(),
					help:  None,
					found: raw.to_string(),
				}
			})?
		} else if raw.starts_with("0b") {
			u64::from_str_radix(raw.trim_start_matches("0b"), 2).map_err(|_| {
				LexError::InvalidNumber {
					loc:   (self.start, raw.len()).into(),
					help:  None,
					found: raw.to_string(),
				}
			})?
		} else {
			raw.parse::<u64>().map_err(|_| {
				LexError::InvalidNumber {
					loc:   (self.start, raw.len()).into(),
					help:  None,
					found: raw.to_string(),
				}
			})?
		};

		Ok(Token { span: (self.start, raw.len()).into(), t: TokenType::Integer(num) })
	}

	/// Attempt to make an identifier starting from the lexers current position
	///
	/// Recognizes keywords
	fn make_identifier_token(&mut self) -> Result<Token<'s>, LexError> {
		let raw = match self.take_chars_while(Self::is_id_continue) {
			Ok(id) => id,
			Err(e) => return Err(e),
		};

		Ok(self.match_identifier(raw))
	}

	/// Attempt to recognize identifiers as keywords
	fn match_identifier(&self, id: &'s str) -> Token<'s> {
		match id {
			"Bottom" => {
				Token { span: (self.start, id.len()).into(), t: TokenType::TypeKwBottom }
			},
			"Tuple" => Token { span: (self.start, id.len()).into(), t: TokenType::TypeKwTuple },
			"List" => Token { span: (self.start, id.len()).into(), t: TokenType::TypeKwList },
			"Function" => {
				Token { span: (self.start, id.len()).into(), t: TokenType::TypeKwFunction }
			},
			"Sum" => Token { span: (self.start, id.len()).into(), t: TokenType::TypeKwSum },
			"Product" => {
				Token { span: (self.start, id.len()).into(), t: TokenType::TypeKwProduct }
			},

			"quote" => Token { span: (self.start, id.len()).into(), t: TokenType::KwQuote },
			"let" => Token { span: (self.start, id.len()).into(), t: TokenType::KwLet },
			"fn" => Token { span: (self.start, id.len()).into(), t: TokenType::KwFn },
			"lambda" => Token { span: (self.start, id.len()).into(), t: TokenType::KwLambda },
			"seq" => Token { span: (self.start, id.len()).into(), t: TokenType::KwSeq },
			"if" => Token { span: (self.start, id.len()).into(), t: TokenType::KwIf },
			"include" => Token { span: (self.start, id.len()).into(), t: TokenType::KwInclude },

			_ => Token { span: (self.start, id.len()).into(), t: TokenType::Identifier(id) },
		}
	}
}
