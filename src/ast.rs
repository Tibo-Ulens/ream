//! AST type definitions and QOL implementations

#![allow(dead_code)]

use miette::SourceSpan;

use crate::{Token, TokenType};

/// A single ream program
#[derive(Clone, Debug)]
pub struct Program<'s>(pub Vec<Expression<'s>>);

/// A single expression
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum Expression<'s> {
	TypeAlias {
		span:   SourceSpan,
		target: Identifier<'s>,
		spec:   TypeSpec<'s>,
	},
	AlgebraicTypeDefintion {
		span:   SourceSpan,
		target: Identifier<'s>,
		spec:   TypeSpec<'s>,
	},
	Annotation(Annotation<'s>),
	Literal(Literal<'s>),
	Identifier(Identifier<'s>),
	VariableDefinition {
		span:   SourceSpan,
		target: Identifier<'s>,
		value:  Box<Expression<'s>>,
	},
	FunctionDefinition {
		span:    SourceSpan,
		target:  Identifier<'s>,
		formals: Vec<Identifier<'s>>,
		body:    Vec<Expression<'s>>,
	},
	ClosureDefintion {
		span:    SourceSpan,
		formals: Vec<Identifier<'s>>,
		body:    Vec<Expression<'s>>,
	},
	Sequence {
		span: SourceSpan,
		seq:  Vec<Expression<'s>>,
	},
	ProcedureCall {
		span:     SourceSpan,
		operator: Box<Expression<'s>>,
		operands: Vec<Expression<'s>>,
	},
	Conditional {
		span:       SourceSpan,
		test:       Box<Expression<'s>>,
		consequent: Box<Expression<'s>>,
		alternate:  Option<Box<Expression<'s>>>,
	},
	Inclusion {
		span:  SourceSpan,
		files: Vec<&'s str>,
	},
}

impl<'s> From<Identifier<'s>> for Expression<'s> {
	fn from(value: Identifier<'s>) -> Self { Self::Identifier(value) }
}

impl<'s> From<Literal<'s>> for Expression<'s> {
	fn from(value: Literal<'s>) -> Self { Self::Literal(value) }
}

impl<'s> From<Annotation<'s>> for Expression<'s> {
	fn from(value: Annotation<'s>) -> Self { Self::Annotation(value) }
}

/// A single identifier
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct Identifier<'s> {
	pub span: SourceSpan,
	pub id:   &'s str,
}

impl<'s> From<Token<'s>> for Identifier<'s> {
	fn from(value: Token<'s>) -> Self {
		match value.t {
			TokenType::Identifier(id) => Self { span: value.span, id },
			_ => unreachable!(),
		}
	}
}

/// A literal value
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum Literal<'s> {
	Quotation { span: SourceSpan, q: Datum<'s> },
	Boolean { span: SourceSpan, b: bool },
	Integer { span: SourceSpan, i: u64 },
	Float { span: SourceSpan, f: f64 },
	Character { span: SourceSpan, c: char },
	String { span: SourceSpan, s: &'s str },
	Atom { span: SourceSpan, a: &'s str },
}

impl<'s> Token<'s> {
	/// Convert the token to a quotation [`Literal`]
	pub fn to_quotation(self) -> Literal<'s> {
		Literal::Quotation { span: self.span, q: self.into() }
	}
}

impl<'s> From<Token<'s>> for Literal<'s> {
	fn from(value: Token<'s>) -> Self {
		match value.t {
			TokenType::Boolean(b) => Self::Boolean { span: value.span, b },
			TokenType::Integer(i) => Self::Integer { span: value.span, i },
			TokenType::Float(f) => Self::Float { span: value.span, f },
			TokenType::Character(c) => Self::Character { span: value.span, c },
			TokenType::String(s) => Self::String { span: value.span, s },
			TokenType::Atom(a) => Self::Atom { span: value.span, a },
			_ => unreachable!(),
		}
	}
}

/// A datum
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum Datum<'s> {
	Identifier { span: SourceSpan, id: &'s str },
	Boolean { span: SourceSpan, b: bool },
	Integer { span: SourceSpan, i: u64 },
	Float { span: SourceSpan, f: f64 },
	Character { span: SourceSpan, c: char },
	String { span: SourceSpan, s: &'s str },
	Atom { span: SourceSpan, a: &'s str },
	List { span: SourceSpan, l: ConsList<'s> },
}

impl<'s> From<Token<'s>> for Datum<'s> {
	fn from(value: Token<'s>) -> Self {
		match value.t {
			TokenType::Identifier(id) => Self::Identifier { span: value.span, id },
			TokenType::Boolean(b) => Self::Boolean { span: value.span, b },
			TokenType::Integer(i) => Self::Integer { span: value.span, i },
			TokenType::Float(f) => Self::Float { span: value.span, f },
			TokenType::Character(c) => Self::Character { span: value.span, c },
			TokenType::String(s) => Self::String { span: value.span, s },
			TokenType::Atom(a) => Self::Atom { span: value.span, a },
			_ => unreachable!(),
		}
	}
}

/// A linked list of [`ConsCell`]s
#[derive(Clone, Debug)]
pub struct ConsList<'s> {
	/// The head of the linked list
	head: Option<Box<ConsCell<'s>>>,
}

/// A Cons cell used to define lists
#[derive(Clone, Debug)]
pub struct ConsCell<'s> {
	/// The head/car of the cell
	head: Datum<'s>,
	/// The tail/cdr of the cell, can be empty
	tail: Option<Box<ConsCell<'s>>>,
}

impl<'s> From<Vec<Datum<'s>>> for ConsList<'s> {
	fn from(value: Vec<Datum<'s>>) -> Self {
		let iter = value.into_iter();
		let head = vec_to_cons_helper(iter);

		Self { head }
	}
}

/// Recursive function to change an iterator to a linked list
fn vec_to_cons_helper<'s, I>(mut iter: I) -> Option<Box<ConsCell<'s>>>
where
	I: Iterator<Item = Datum<'s>>,
{
	if let Some(head) = iter.next() {
		let tail = vec_to_cons_helper(iter);

		Some(Box::new(ConsCell { head, tail }))
	} else {
		None
	}
}

impl<'s> From<ConsList<'s>> for Vec<Datum<'s>> {
	fn from(value: ConsList<'s>) -> Self {
		let collector = vec![];
		let Some(boxed_head) = value.head else {
			return collector;
		};

		let head = *boxed_head;

		cons_to_vec_helper(collector, head)
	}
}

/// Recursive function to change a linked list to vec
fn cons_to_vec_helper<'s>(mut collector: Vec<Datum<'s>>, list: ConsCell<'s>) -> Vec<Datum<'s>> {
	collector.push(list.head);

	match list.tail {
		Some(cell) => cons_to_vec_helper(collector, *cell),
		None => collector,
	}
}

/// An annotation for an item
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum Annotation<'s> {
	TypeAnnotation { span: SourceSpan, target: Identifier<'s>, spec: TypeSpec<'s> },
	DocAnnotation { span: SourceSpan, target: Identifier<'s>, doc: &'s str },
}

/// A type specification
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum TypeSpec<'s> {
	Identifier(Identifier<'s>),
	Constructor(TypeConstructor<'s>),
}

impl<'s> From<Identifier<'s>> for TypeSpec<'s> {
	fn from(value: Identifier<'s>) -> Self { Self::Identifier(value) }
}

impl<'s> From<TypeConstructor<'s>> for TypeSpec<'s> {
	fn from(value: TypeConstructor<'s>) -> Self { Self::Constructor(value) }
}

/// A type constructor
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum TypeConstructor<'s> {
	Bottom { span: SourceSpan },
	Tuple { span: SourceSpan, fields: Vec<TypeSpec<'s>> },
	List { span: SourceSpan, t: Box<TypeSpec<'s>> },
	Vector { span: SourceSpan, t: Box<TypeSpec<'s>> },
	Function { span: SourceSpan, arguments: Vec<TypeSpec<'s>>, values: Vec<TypeSpec<'s>> },
	Sum { span: SourceSpan, fields: Vec<NamedTypeSpec<'s>> },
	Product { span: SourceSpan, fields: Vec<NamedTypeSpec<'s>> },
}

/// A named (labeled) type specification
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct NamedTypeSpec<'s> {
	span: SourceSpan,
	name: Literal<'s>,
	spec: Option<TypeSpec<'s>>,
}
