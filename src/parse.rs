use miette::Error;

/// A parser for a single source file
#[allow(missing_docs)]
pub struct Parser<'s> {
	source: &'s str,
}

impl<'s> Parser<'s> {
	/// Create a new [`Parser`]
	pub fn new(source: &'s str) -> Self { Self { source } }

	/// Parse the entire input
	pub fn parse(&self) -> Result<(), Error> { Ok(()) }
}
