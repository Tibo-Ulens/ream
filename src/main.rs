use std::borrow::Cow;
use std::fs::File;
use std::io::Read;

use clap::Parser as ArgParser;
use miette::NamedSource;
use ream::{Error, Lexer};

#[derive(ArgParser)]
#[command(author, version, about, long_about=None)]
struct Args {
	/// The source file
	pub source_file: String,

	/// How verbose the output should be
	#[arg(short='v', long="verbose", action=clap::ArgAction::Count)]
	pub verbosity: u8,
}

fn main() -> miette::Result<()> {
	let args = Args::parse();

	let mut source_file = File::open(args.source_file.clone()).map_err(Error::from)?;
	let mut source = String::new();
	source_file.read_to_string(&mut source).map_err(Error::from)?;

	let source: Cow<str> = source.into();

	let named_source = NamedSource::new(args.source_file, source.clone());

	let lexer = Lexer::new(&source);

	let tokens = lexer
		.into_iter()
		.collect::<Result<Vec<_>, _>>()
		.map_err(|err| err.with_source_code(named_source))?;

	println!("{}", tokens.iter().map(|t| format!("{t:?}")).collect::<Vec<_>>().join("\n"));

	// let parser = Parser::new(&source);

	// let _root = parser.parse().map_err(|err| err.with_source_code(named_source))?;

	Ok(())
}
