//! Bytecode instructions, values, and chunks

use std::{fmt, str};

use miette::{NamedSource, SourceCode, SourceSpan};

/// A single instruction
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpCode {
	/// Return from a function call
	Return,
	/// Load an immediate straight from the instruction
	LoadImmediate {
		/// The value to load
		imm: i64,
	},
	/// Load a constant from the constant table
	LoadConstant {
		/// The index of the constant to load
		idx: usize,
	},
	/// Negate the value at the top of the stack
	Negate,
	/// Add the top two values of the stack
	Add,
	/// Subtract the top two values of the stack
	Sub,
	/// Multiply the top two values of the stack
	Mul,
	/// Divide the top two values of the stack
	Div,
}

impl OpCode {
	/// Disassemble an instruction to a string containing all relevant info
	pub fn disassemble<S: SourceCode + 'static>(&self, idx: usize, chunk: &Chunk<S>) -> String {
		let inst_formatted = match self {
			Self::LoadConstant { idx } => {
				let c = &chunk.constants[*idx];
				format!("{self} = {c}")
			},
			_ => self.to_string(),
		};

		let span = chunk.spans[idx];
		let span_contents = chunk.source.read_span(&span, 0, 0).unwrap();
		let line_info = format!(
			"[{:03} {:02}] {}",
			span_contents.line(),
			span_contents.column(),
			str::from_utf8(span_contents.data()).unwrap(),
		);

		format!("{idx:04} | {inst_formatted:32} | {line_info}")
	}
}

impl fmt::Display for OpCode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Return => write!(f, "Return"),
			Self::LoadImmediate { imm } => write!(f, "LoadImmediate {imm}"),
			Self::LoadConstant { idx } => write!(f, "LoadConstant {idx}"),
			Self::Negate => write!(f, "Negate"),
			Self::Add => write!(f, "Add"),
			Self::Sub => write!(f, "Sub"),
			Self::Mul => write!(f, "Mul"),
			Self::Div => write!(f, "Div"),
		}
	}
}

/// A single bytecode value
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
	Boolean(bool),
	Integer(i64),
	Float(f64),
	Character(char),
	String(String),
}

impl Value {
	/// Get the name of the type of this value
	pub fn type_name(&self) -> String {
		match &self {
			Self::Boolean(_) => "Boolean".into(),
			Self::Integer(_) => "Integer".into(),
			Self::Float(_) => "Float".into(),
			Self::Character(_) => "Character".into(),
			Self::String(_) => "String".into(),
		}
	}
}

impl fmt::Display for Value {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			Self::Boolean(b) => write!(f, "{b}"),
			Self::Integer(i) => write!(f, "{i}"),
			Self::Float(fl) => write!(f, "{fl}"),
			Self::Character(c) => write!(f, "\'{c}\'"),
			Self::String(s) => write!(f, "\"{s}\""),
		}
	}
}

/// A single chunk of bytecode with its associated metadata
#[derive(Clone, Debug)]
pub struct Chunk<S: SourceCode + 'static> {
	pub(crate) name:         String,
	pub(crate) instructions: Vec<OpCode>,
	pub(crate) constants:    Vec<Value>,
	pub(crate) spans:        Vec<SourceSpan>,
	pub(crate) source:       NamedSource<S>,
}

impl<S: SourceCode + 'static> Chunk<S> {
	/// Create a new chunk of bytecode
	pub fn new(name: String, source: NamedSource<S>) -> Self {
		Self { name, instructions: vec![], constants: vec![], spans: vec![], source }
	}

	/// Push an instruction to the chunk
	pub fn push_instruction(&mut self, inst: OpCode, span: SourceSpan) {
		self.instructions.push(inst);
		self.spans.push(span);
	}

	/// Push a constant to the constant table
	///
	/// Returns the index into the table for use in a LoadConstant instruction
	pub fn push_constant(&mut self, constant: Value) -> usize {
		self.constants.push(constant);

		self.constants.len() - 1
	}
}

impl<S: SourceCode> fmt::Display for Chunk<S> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "== {} ==", self.name)?;

		for (idx, inst) in self.instructions.iter().enumerate() {
			let formatted = inst.disassemble(idx, self);

			writeln!(f, "{formatted}")?;
		}

		Ok(())
	}
}
