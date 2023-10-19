//! Virtual machine implementation

#![allow(dead_code)]

use miette::{Error, SourceCode};

use crate::bytecode::{Chunk, OpCode, Value};
use crate::InterpretError;

const STACK_SIZE: usize = 1024;

/// A virtual machine which executes bytecode
#[derive(Clone, Debug)]
pub struct ReamVirtualMachine<S: SourceCode + 'static> {
	chunk: Chunk<S>,
	ip:    usize,

	stack: [Value; STACK_SIZE],
	sp:    usize,
}

impl<S: SourceCode + 'static> ReamVirtualMachine<S> {
	/// Create a new VM
	pub fn new(chunk: Chunk<S>) -> Self {
		Self { chunk, ip: 0, stack: [const { Value::Integer(0) }; STACK_SIZE], sp: 0 }
	}

	/// Execute the given chunk
	pub fn execute_chunk(&mut self, chunk: Chunk<S>, trace: bool) -> Result<(), Error> {
		self.chunk = chunk;
		self.ip = 0;

		self.run(trace)
	}

	fn push(&mut self, value: Value) {
		self.stack[self.sp] = value;
		self.sp += 1;
	}

	fn pop(&mut self) -> Value {
		self.sp -= 1;
		self.stack[self.sp].clone()
	}

	/// Start the VM
	pub fn run(&mut self, trace: bool) -> Result<(), Error> {
		while self.ip < self.chunk.instructions.len() {
			let instruction = self.chunk.instructions[self.ip];

			if trace {
				print!("[");
				for i in 0..self.sp {
					print!("{} ", self.stack[i]);
				}
				println!("]");
				println!("{}", instruction.disassemble(self.ip, &self.chunk))
			}

			self.ip += 1;

			match instruction {
				OpCode::Return => {
					println!("{}", self.pop());
					return Ok(());
				},
				OpCode::LoadImmediate { imm } => {
					self.push(Value::Integer(imm));
				},
				OpCode::LoadConstant { idx } => {
					self.push(self.chunk.constants[idx].clone());
				},
				OpCode::Negate => {
					let v = self.pop();
					let new_v = match v {
						Value::Integer(i) => Value::Integer(-i),
						Value::Float(f) => Value::Float(-f),
						t => {
							return Err(InterpretError::WrongType {
								loc:      self.chunk.spans[self.ip],
								expected: &[Value::Integer(0), Value::Float(0.)],
								found:    t.type_name(),
							}
							.into());
						},
					};

					self.push(new_v);
				},
				OpCode::Add => {
					let a = self.pop();
					let b = self.pop();

					let result = match (a, b) {
						(Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
						(Value::Float(a), Value::Float(b)) => Value::Float(a + b),
						(Value::Integer(_), t) | (t, Value::Integer(_)) => {
							return Err(InterpretError::WrongType {
								loc:      self.chunk.spans[self.ip],
								expected: &[Value::Integer(0)],
								found:    t.type_name(),
							}
							.into());
						},
						(Value::Float(_), t) | (t, Value::Float(_)) => {
							return Err(InterpretError::WrongType {
								loc:      self.chunk.spans[self.ip],
								expected: &[Value::Float(0.)],
								found:    t.type_name(),
							}
							.into());
						},
						(t, _) => {
							return Err(InterpretError::WrongType {
								loc:      self.chunk.spans[self.ip],
								expected: &[Value::Integer(0), Value::Float(0.)],
								found:    t.type_name(),
							}
							.into());
						},
					};

					self.push(result);
				},
				OpCode::Sub => {
					let a = self.pop();
					let b = self.pop();

					let result = match (a, b) {
						(Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
						(Value::Float(a), Value::Float(b)) => Value::Float(a - b),
						(Value::Integer(_), t) | (t, Value::Integer(_)) => {
							return Err(InterpretError::WrongType {
								loc:      self.chunk.spans[self.ip],
								expected: &[Value::Integer(0)],
								found:    t.type_name(),
							}
							.into());
						},
						(Value::Float(_), t) | (t, Value::Float(_)) => {
							return Err(InterpretError::WrongType {
								loc:      self.chunk.spans[self.ip],
								expected: &[Value::Float(0.)],
								found:    t.type_name(),
							}
							.into());
						},
						(t, _) => {
							return Err(InterpretError::WrongType {
								loc:      self.chunk.spans[self.ip],
								expected: &[Value::Integer(0), Value::Float(0.)],
								found:    t.type_name(),
							}
							.into());
						},
					};

					self.push(result);
				},
				OpCode::Mul => {
					let a = self.pop();
					let b = self.pop();

					let result = match (a, b) {
						(Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
						(Value::Float(a), Value::Float(b)) => Value::Float(a * b),
						(Value::Integer(_), t) | (t, Value::Integer(_)) => {
							return Err(InterpretError::WrongType {
								loc:      self.chunk.spans[self.ip],
								expected: &[Value::Integer(0)],
								found:    t.type_name(),
							}
							.into());
						},
						(Value::Float(_), t) | (t, Value::Float(_)) => {
							return Err(InterpretError::WrongType {
								loc:      self.chunk.spans[self.ip],
								expected: &[Value::Float(0.)],
								found:    t.type_name(),
							}
							.into());
						},
						(t, _) => {
							return Err(InterpretError::WrongType {
								loc:      self.chunk.spans[self.ip],
								expected: &[Value::Integer(0), Value::Float(0.)],
								found:    t.type_name(),
							}
							.into());
						},
					};

					self.push(result);
				},
				OpCode::Div => {
					let a = self.pop();
					let b = self.pop();

					let result = match (a, b) {
						(Value::Integer(a), Value::Integer(b)) => Value::Integer(a / b),
						(Value::Float(a), Value::Float(b)) => Value::Float(a / b),
						(Value::Integer(_), t) | (t, Value::Integer(_)) => {
							return Err(InterpretError::WrongType {
								loc:      self.chunk.spans[self.ip],
								expected: &[Value::Integer(0)],
								found:    t.type_name(),
							}
							.into());
						},
						(Value::Float(_), t) | (t, Value::Float(_)) => {
							return Err(InterpretError::WrongType {
								loc:      self.chunk.spans[self.ip],
								expected: &[Value::Float(0.)],
								found:    t.type_name(),
							}
							.into());
						},
						(t, _) => {
							return Err(InterpretError::WrongType {
								loc:      self.chunk.spans[self.ip],
								expected: &[Value::Integer(0), Value::Float(0.)],
								found:    t.type_name(),
							}
							.into());
						},
					};

					self.push(result);
				},
			}
		}

		Ok(())
	}
}

#[cfg(test)]
mod test {
	use miette::{NamedSource, SourceSpan};

	use super::ReamVirtualMachine;
	use crate::bytecode::{Chunk, OpCode};

	#[test]
	fn test_vm() {
		let source = NamedSource::new("test_source", "foo\nfoo\nfoo\nfoo\nfoo\n");
		let mut chunk = Chunk::new("main".into(), source);

		chunk.push_instruction(OpCode::LoadImmediate { imm: 42 }, SourceSpan::new(0.into(), 3));
		chunk.push_instruction(OpCode::LoadImmediate { imm: 69 }, SourceSpan::new(4.into(), 3));
		chunk.push_instruction(OpCode::Add, SourceSpan::new(8.into(), 3));
		chunk.push_instruction(OpCode::Return, SourceSpan::new(12.into(), 3));

		println!("{chunk}");

		let mut vm = ReamVirtualMachine::new(chunk);

		assert!(vm.run(true).is_ok())
	}
}
