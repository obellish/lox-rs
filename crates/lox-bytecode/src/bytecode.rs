use std::{
	fmt::{Display, Formatter, Result as FmtResult},
	ops::Deref,
};

use serde::{Deserialize, Serialize};

pub type InstructionIndex = usize;
pub type ConstantIndex = usize;
pub type StackIndex = usize;
pub type ChunkIndex = usize;
pub type ArgumentCount = usize;
pub type UpvalueIndex = usize;
pub type ClosureIndex = usize;
pub type ClassIndex = usize;
pub type IdentifierIndex = usize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Class {
	pub name: String,
}

impl Display for Class {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(&self.name)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Closure {
	pub function: Function,
	pub upvalues: Vec<Upvalue>,
}

impl From<Function> for Closure {
	fn from(value: Function) -> Self {
		Self {
			function: value,
			upvalues: Vec::new(),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Upvalue {
	Local(StackIndex),
	Upvalue(UpvalueIndex),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
	pub name: String,
	pub chunk_index: ChunkIndex,
	pub arity: usize,
}

impl Display for Function {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(&self.name)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Chunk {
	instructions: Vec<u8>,
}

impl Chunk {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			instructions: Vec::new(),
		}
	}

	pub fn add_u8(&mut self, value: u8) -> InstructionIndex {
		self.instructions.push(value);
		self.instructions.len() - 1
	}

	pub fn add_u32(&mut self, value: u32) -> InstructionIndex {
		let bytes = value.to_le_bytes();
		for byte in bytes {
			self.add_u8(byte);
		}

		self.instructions.len() - 4
	}

	pub fn add_i16(&mut self, value: i16) -> InstructionIndex {
		let bytes = value.to_le_bytes();

		for byte in bytes {
			self.add_u8(byte);
		}

		self.instructions.len() - 2
	}

	pub fn add_u16(&mut self, value: u16) -> InstructionIndex {
		let bytes = value.to_le_bytes();
		for byte in bytes {
			self.add_u8(byte);
		}

		self.instructions.len() - 2
	}

	pub fn set_i16(&mut self, index: InstructionIndex, value: i16) {
		let bytes = value.to_le_bytes();
		self.instructions[index..(2 + index)].copy_from_slice(&bytes);
	}

	pub fn set_u32(&mut self, index: InstructionIndex, value: u32) {
		let bytes = value.to_le_bytes();
		self.instructions[index..(4 + index)].copy_from_slice(&bytes);
	}

	#[must_use]
	pub fn instruction_index(&self) -> InstructionIndex {
		self.instructions.len()
	}

	pub fn patch_instruction(&mut self, index: InstructionIndex) {
		let current = self.instruction_index();
		self.patch_instruction_to(index, current);
	}

	pub fn patch_instruction_to(&mut self, index: InstructionIndex, to: InstructionIndex) {
		let offset = (to as isize) - (index as isize) - 2;
		self.set_i16(index, offset as _);
	}

	#[must_use]
	pub fn as_slice(&self) -> &[u8] {
		self
	}

	#[must_use]
	pub fn as_ptr(&self) -> *const u8 {
		self.instructions.as_ptr()
	}

	#[must_use]
	pub fn get_u8(&self, pc: usize) -> u8 {
		self.instructions[pc]
	}

	#[must_use]
	pub fn get_u32(&self, pc: usize) -> u32 {
		let bytes = &self.instructions[pc..pc + 4];
		u32::from_le_bytes(bytes.try_into().unwrap())
	}
}

impl Default for Chunk {
	fn default() -> Self {
		Self::new()
	}
}

impl Deref for Chunk {
	type Target = [u8];

	fn deref(&self) -> &Self::Target {
		&self.instructions
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
	pub chunks: Vec<Chunk>,
	pub closures: Vec<Closure>,
	pub classes: Vec<Class>,
	pub identifiers: Vec<String>,
	pub numbers: Vec<f64>,
	pub strings: Vec<String>,
}

impl Module {
	#[must_use]
	pub const fn new() -> Self {
		Self {
			chunks: Vec::new(),
			closures: Vec::new(),
			classes: Vec::new(),
			identifiers: Vec::new(),
			numbers: Vec::new(),
			strings: Vec::new(),
		}
	}

	#[inline]
	#[must_use]
	pub fn chunk(&self, index: ChunkIndex) -> &Chunk {
		&self.chunks[index]
	}

	pub fn chunk_mut(&mut self, index: ChunkIndex) -> &mut Chunk {
		&mut self.chunks[index]
	}

	pub fn add_chunk(&mut self) -> ChunkIndex {
		self.chunks.push(Chunk::new());
		self.chunks.len() - 1
	}

	pub fn add_closure(&mut self, closure: Closure) -> ClosureIndex {
		self.closures.push(closure);
		self.closures.len() - 1
	}

	pub fn add_class(&mut self, class: Class) -> ClassIndex {
		self.classes.push(class);
		self.classes.len() - 1
	}

	pub fn add_identifier(&mut self, identifier: String) -> IdentifierIndex {
		self.identifiers.push(identifier);
		self.identifiers.len() - 1
	}

	pub fn add_number(&mut self, n: f64) -> ConstantIndex {
		self.numbers.push(n);
		self.numbers.len() - 1
	}

	pub fn add_string(&mut self, value: String) -> ConstantIndex {
		self.strings.push(value);
		self.strings.len() - 1
	}

	#[must_use]
	pub fn closures(&self) -> &[Closure] {
		&self.closures
	}

	#[must_use]
	pub fn classes(&self) -> &[Class] {
		&self.classes
	}

	#[must_use]
	pub fn identifiers(&self) -> &[String] {
		&self.identifiers
	}

	#[must_use]
	pub fn number(&self, index: ConstantIndex) -> f64 {
		self.numbers[index]
	}

	#[must_use]
	pub fn string(&self, index: ConstantIndex) -> &str {
		&self.strings[index]
	}

	#[must_use]
	pub fn closure(&self, index: ClosureIndex) -> &Closure {
		&self.closures[index]
	}

	#[must_use]
	pub fn class(&self, index: ClassIndex) -> &Class {
		&self.classes[index]
	}

	#[must_use]
	pub fn identifier(&self, index: IdentifierIndex) -> &str {
		&self.identifiers[index]
	}

	#[must_use]
	pub fn chunks(&self) -> &[Chunk] {
		&self.chunks
	}
}

impl Default for Module {
	fn default() -> Self {
		Self::new()
	}
}
