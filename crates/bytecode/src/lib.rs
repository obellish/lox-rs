#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

pub mod disasm;
pub mod opcode;

use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{Deserialize, Serialize};

pub type InstructionIndex = usize;
pub type ConstantIndex = usize;
pub type StackIndex = usize;
pub type ChunkIndex = usize;
pub type ArgumentCount = usize;
pub type UpValueIndex = usize;
pub type ClosureIndex = usize;
pub type ClassIndex = usize;
pub type IdentifierIndex = usize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(transparent)]
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
	pub up_values: Vec<UpValue>,
}

impl From<Function> for Closure {
	fn from(value: Function) -> Self {
		Self {
			function: value,
			up_values: Vec::new(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Function {
	pub name: String,
	pub chunk_index: ChunkIndex,
	pub arity: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(transparent)]
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
			self.instructions.push(byte);
		}

		self.instructions.len() - 4
	}

	pub fn add_i16(&mut self, value: i16) -> InstructionIndex {
		self.add_2_byte_value(value.to_le_bytes())
	}

	pub fn add_u16(&mut self, value: u16) -> InstructionIndex {
		self.add_2_byte_value(value.to_le_bytes())
	}

	pub fn set_i16(&mut self, index: InstructionIndex, value: i16) {
		self.set_value(index, value.to_le_bytes());
	}

	pub fn set_u32(&mut self, index: InstructionIndex, value: u32) {
		self.set_value(index, value.to_le_bytes());
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
	pub fn as_bytes(&self) -> &[u8] {
		&self.instructions
	}

	#[must_use]
	pub fn as_ptr(&self) -> *const u8 {
		self.instructions.as_ptr()
	}

	#[must_use]
	pub unsafe fn get_unchecked(&self, pc: usize) -> u8 {
		unsafe { *self.instructions.get_unchecked(pc) }
	}

	#[must_use]
	pub unsafe fn get_u32_unchecked(&self, pc: usize) -> u32 {
		let bytes = unsafe { self.instructions.get_unchecked(pc..pc + 4) };
		u32::from_le_bytes(bytes.try_into().unwrap())
	}

	fn add_2_byte_value(&mut self, value: [u8; 2]) -> InstructionIndex {
		for byte in value {
			self.instructions.push(byte);
		}

		self.instructions.len() - 2
	}

	fn set_value(&mut self, index: InstructionIndex, value: impl IntoIterator<Item = u8>) {
		for (i, byte) in value.into_iter().enumerate() {
			self.instructions[index + i] = byte;
		}
	}
}

impl Default for Chunk {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
			classes: Vec::new(),
			closures: Vec::new(),
			identifiers: Vec::new(),
			numbers: Vec::new(),
			strings: Vec::new(),
		}
	}

	#[must_use]
	pub fn chunk(&self, index: ChunkIndex) -> Option<&Chunk> {
		self.chunks.get(index)
	}

	pub fn chunk_mut(&mut self, index: ChunkIndex) -> Option<&mut Chunk> {
		self.chunks.get_mut(index)
	}

	#[must_use]
	pub unsafe fn chunk_unchecked(&self, index: ChunkIndex) -> &Chunk {
		unsafe { self.chunks.get_unchecked(index) }
	}

	pub unsafe fn chunk_unchecked_mut(&mut self, index: ChunkIndex) -> &mut Chunk {
		unsafe { self.chunks.get_unchecked_mut(index) }
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

	pub fn add_number(&mut self, value: f64) -> ConstantIndex {
		self.numbers.push(value);
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
	pub fn number(&self, index: ConstantIndex) -> Option<f64> {
		self.numbers.get(index).copied()
	}

	#[must_use]
	pub unsafe fn number_unchecked(&self, index: ConstantIndex) -> f64 {
		unsafe { *self.numbers.get_unchecked(index) }
	}

	#[must_use]
	pub fn string(&self, index: ConstantIndex) -> Option<&str> {
		self.strings.get(index).map(String::as_str)
	}

	#[must_use]
	pub unsafe fn string_unchecked(&self, index: ConstantIndex) -> &str {
		unsafe { self.strings.get_unchecked(index) }
	}

	#[must_use]
	pub fn closure(&self, index: ClosureIndex) -> Option<&Closure> {
		self.closures.get(index)
	}

	#[must_use]
	pub unsafe fn closure_unchecked(&self, index: ClosureIndex) -> &Closure {
		unsafe { self.closures.get_unchecked(index) }
	}

	#[must_use]
	pub fn class(&self, index: ClassIndex) -> Option<&Class> {
		self.classes.get(index)
	}

	#[must_use]
	pub unsafe fn class_unchecked(&self, index: ClassIndex) -> &Class {
		unsafe { self.classes.get_unchecked(index) }
	}

	#[must_use]
	pub fn identifier(&self, index: IdentifierIndex) -> Option<&str> {
		self.identifiers.get(index).map(String::as_str)
	}

	#[must_use]
	pub unsafe fn identifier_unchecked(&self, index: IdentifierIndex) -> &str {
		unsafe { self.identifiers.get_unchecked(index) }
	}

	#[must_use]
	pub fn chunks(&self) -> &[Chunk] {
		&self.chunks
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpValue {
	Local(StackIndex),
	UpValue(UpValueIndex),
}
