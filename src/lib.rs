//! Ream language library

#![warn(missing_docs)]
#![feature(assert_matches)]
#![feature(let_chains)]
#![feature(lazy_cell)]
#![feature(type_alias_impl_trait)]

pub mod ast;
mod error;
mod lex;
mod parse;
mod token;

pub use error::*;
pub use lex::*;
use miette::SourceSpan;
pub use parse::*;
pub use token::*;

trait Combine {
	fn combine(&self, other: &Self) -> Self;
}

impl Combine for SourceSpan {
	fn combine(&self, other: &Self) -> Self {
		// Start at the first span
		let start = self.offset() as isize;

		// Keep going for the length of the first span, compensate for a
		// potential gap between the first and second span, and keep going for
		// the length of the second span
		let first_len = self.len() as isize;
		let len =
			first_len + (other.offset() as isize - (start + first_len)) + other.len() as isize;

		(start as usize, len as usize).into()
	}
}
