use std::borrow::Cow;
use std::io::Write;

use ptree::{Style, TreeItem};

mod impls;

/// A non type-checked AST node
#[derive(Clone)]
pub(crate) struct Node {
	pub(crate) repr:     String,
	pub(crate) children: Vec<Node>,
}

impl TreeItem for Node {
	type Child = Self;

	fn write_self<W: Write>(&self, f: &mut W, style: &Style) -> std::io::Result<()> {
		write!(f, "{}", style.paint(self.repr.clone()))
	}

	fn children(&self) -> Cow<[Self::Child]> { Cow::from(self.children.clone()) }
}

/// Converting to a non type-checked node
pub(crate) trait ToNode {
	/// Perform the conversion
	fn to_node(&self) -> Node;
}
