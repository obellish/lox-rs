use std::ops::{Deref, DerefMut};

pub type Value = f64;

#[derive(Debug, Clone)]
pub struct Chunk {
	pub(crate) code: Vec<OpCode>,
	pub(crate) constants: ValueArray,
	lines: Vec<usize>,
}

impl Chunk {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			code: Vec::new(),
			constants: ValueArray::new(),
			lines: Vec::new(),
		}
	}

	pub fn write_opcode(&mut self, code: OpCode, line: usize) {
		self.code.push(code);
		self.lines.push(line);
	}

	pub fn write_constant(&mut self, value: Value, line: usize) {
		let constant = self.add_constant(value);
		self.write_opcode(OpCode::Constant(constant), line);
	}

	pub fn add_constant(&mut self, value: Value) -> usize {
		self.constants.write_value(value);
		self.constants.values.len() - 1
	}
}

impl Default for Chunk {
	fn default() -> Self {
		Self::new()
	}
}

impl Deref for Chunk {
	type Target = [OpCode];

	fn deref(&self) -> &Self::Target {
		&self.code
	}
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct ValueArray {
	values: Vec<Value>,
}

impl ValueArray {
	#[must_use]
	pub const fn new() -> Self {
		Self { values: Vec::new() }
	}

	pub fn write_value(&mut self, value: Value) {
		self.values.push(value);
	}
}

impl Default for ValueArray {
	fn default() -> Self {
		Self::new()
	}
}

impl Deref for ValueArray {
	type Target = [Value];

	fn deref(&self) -> &Self::Target {
		&self.values
	}
}

impl DerefMut for ValueArray {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.values
	}
}

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
	Constant(usize),
	Negate,
	Add,
	Subtract,
	Divide,
	Multiply,
	Return,
}
