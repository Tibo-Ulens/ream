//! Ream language library

#![warn(missing_docs)]
#![feature(assert_matches)]
#![feature(let_chains)]
#![feature(type_alias_impl_trait)]

mod error;
mod lex;
mod parse;
mod token;

pub use error::*;
pub use lex::*;
pub use parse::*;
pub use token::*;
