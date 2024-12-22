use std::{
	cell::Cell,
	fmt::{Debug, Formatter, Result as FmtResult},
};

use lox_bytecode::bytecode::ChunkIndex;
use lox_gc::Gc;

use super::{Import, Upvalue};
use crate::array::Array;

pub struct Closure {
	pub function: Function,
	pub upvalues: Array<Gc<Cell<Upvalue>>>,
}

pub struct Function {
	pub name: String,
	pub chunk_index: ChunkIndex,
	pub import: Gc<Import>,
	pub arity: usize,
}

impl Function {
	pub(crate) fn new(value: &lox_bytecode::bytecode::Function, import: Gc<Import>) -> Self {
		Self {
			name: value.name.as_str().into(),
			chunk_index: value.chunk_index,
			arity: value.arity,
			import,
		}
	}
}

impl Debug for Function {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_struct("Function")
			.field("name", &self.name)
			.finish_non_exhaustive()
	}
}
