use std::{
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult},
};

use super::{Chunk, CompilerError, OpCode, Value};
use crate::Compiler;

const STACK_MAX: usize = 256;

#[derive(Debug, Clone, Copy)]
pub struct Vm<'chunk> {
	chunk: Option<&'chunk Chunk>,
	ip: usize,
	stack: [Value; STACK_MAX],
	stack_top: usize,
}

impl<'chunk> Vm<'chunk> {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			chunk: None,
			ip: 0,
			stack: [0.0; STACK_MAX],
			stack_top: 0,
		}
	}

	pub fn interpret(&mut self, source: String) -> Result<(), VmError> {
		let mut compiler = Compiler::new(source);
		compiler.compile()?;

		self.run()
	}

	pub fn push(&mut self, value: Value) {
		self.stack[self.stack_top] = value;
		self.stack_top += 1;
	}

	pub fn pop(&mut self) -> Value {
		self.stack_top -= 1;
		self.stack[self.stack_top]
	}

	fn pop_two(&mut self) -> (Value, Value) {
		let b = self.pop();
		let a = self.pop();

		(a, b)
	}

	fn run(&mut self) -> Result<(), VmError> {
		let Some(chunk) = self.chunk else {
			return Err(VmError::NoChunkPresent);
		};

		loop {
			let index = self.ip;
			self.ip += 1;
			let code = chunk[index];

			match code {
				OpCode::Return => {
					let value = self.pop();
					println!("{value}");
					break;
				}
				OpCode::Add => {
					let (a, b) = self.pop_two();
					self.push(a + b);
				}
				OpCode::Subtract => {
					let (a, b) = self.pop_two();
					self.push(a - b);
				}
				OpCode::Multiply => {
					let (a, b) = self.pop_two();
					self.push(a * b);
				}
				OpCode::Divide => {
					let (a, b) = self.pop_two();
					self.push(a / b);
				}
				OpCode::Negate => {
					let value = self.pop();
					self.push(-value);
				}
				OpCode::Constant(constant) => {
					let value = chunk.constants[constant];
					self.push(value);
				}
			}
		}

		Ok(())
	}
}

impl Default for Vm<'_> {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug)]
pub enum VmError {
	NoChunkPresent,
	Runtime(RuntimeError),
	Compiler(CompilerError),
}

impl Display for VmError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::NoChunkPresent => f.write_str("No chunk was present in the VM"),
			Self::Runtime(r) => Display::fmt(r, f),
			Self::Compiler(c) => Display::fmt(c, f),
		}
	}
}

impl From<CompilerError> for VmError {
	fn from(value: CompilerError) -> Self {
		Self::Compiler(value)
	}
}

impl From<RuntimeError> for VmError {
	fn from(value: RuntimeError) -> Self {
		Self::Runtime(value)
	}
}

impl StdError for VmError {
	fn source(&self) -> Option<&(dyn StdError + 'static)> {
		match self {
			Self::NoChunkPresent => None,
			Self::Runtime(r) => Some(r),
			Self::Compiler(c) => Some(c),
		}
	}
}

#[derive(Debug)]
pub enum RuntimeError {}

impl Display for RuntimeError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match *self {}
	}
}

impl StdError for RuntimeError {}
