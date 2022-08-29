#![feature(iter_collect_into)]

use std::env;
use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate thiserror;

use error::Error;
use ptree::print_tree;

mod error;
mod lex;
mod parse;
mod tree;

use lex::Lexer;

use crate::parse::Parser;
use crate::tree::ToNode;

fn main() -> Result<(), Error> {
	let args: Vec<String> = env::args().collect();

	let file: &str = &args[1];
	let mut file = File::open(file)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;

	let chars: Vec<char> = contents.chars().collect();
	let mut lexer = Lexer::new(&chars);
	let tokens = lexer.lex()?;

	for t in tokens.iter() {
		println!("{}", t);
	}

	let mut parser = Parser::new(tokens);
	let ast = parser.parse()?;

	let tree = ast.to_node();

	print_tree(&tree).unwrap();

	Ok(())
}
