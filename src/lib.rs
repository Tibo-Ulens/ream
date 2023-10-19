//! Ream language library

#![warn(missing_docs)]
#![allow(incomplete_features)]
#![feature(assert_matches)]
#![feature(generic_const_items)]
#![feature(inline_const)]

use miette::SourceSpan;

pub mod ast;
pub mod bytecode;
mod error;
mod eval;
mod lex;
mod parse;
mod token;
pub mod vm;

pub use error::*;
pub use lex::*;
pub use parse::*;
pub use token::*;

trait Combine {
	/// Combine two items into one
	fn combine(&self, other: &Self) -> Self;

	/// Increment an item
	fn increment(&self) -> Self;
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

	fn increment(&self) -> Self {
		let start = self.offset() + self.len();
		let len = 1;

		(start, len).into()
	}
}
