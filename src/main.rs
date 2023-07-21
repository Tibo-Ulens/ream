use std::borrow::Cow;
use std::fs::File;
use std::io::Read;

use clap::Parser as ArgParser;
use miette::NamedSource;
use ream::{Error, Lexer, Parser};

#[derive(ArgParser, Clone)]
#[command(author, version, about, long_about=None)]
struct Args {
	/// The source file
	source_file: String,

	/// How verbose the output should be
	#[arg(short='v', long="verbose", action=clap::ArgAction::Count)]
	verbosity: u8,

	/// Whether or not to show the output of the lexer
	#[arg(short = 'l', long = "lex")]
	show_lex: bool,
}

fn main() -> miette::Result<()> {
	let args = Args::parse();

	let mut source_file = File::open(args.source_file.clone()).map_err(Error::from)?;
	let mut source = String::new();
	source_file.read_to_string(&mut source).map_err(Error::from)?;

	let source: Cow<str> = source.into();

	let named_source = NamedSource::new(args.source_file.clone(), source.clone());

	process_file(&source, &args).map_err(|err| err.with_source_code(named_source))
}

/// Separate function that actually does all the work because miette decided
/// that [`NamedSource`] didn't need to be [`Copy`] or [`Clone`] for some
/// reason
fn process_file(source: &str, args: &Args) -> miette::Result<()> {
	let lexer = Lexer::new(source);

	if args.show_lex {
		let tokens = lexer.clone().collect::<Result<Vec<_>, _>>()?;

		println!("{}", tokens.iter().map(|t| format!("{t:?}")).collect::<Vec<_>>().join("\n"));
	}

	let token_iterator = lexer.peekable();

	let parser = Parser::new(source, token_iterator);

	let _root = parser.parse()?;

	Ok(())
}
