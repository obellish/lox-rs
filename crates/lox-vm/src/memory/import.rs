use std::cell::UnsafeCell;

use lox_bytecode::bytecode::{Chunk, ChunkIndex, ClassIndex, ClosureIndex, ConstantIndex, Module};
use lox_gc::{Gc, Trace, Tracer};

use crate::{
	array::Array,
	interner::{Interner, Symbol},
	string::LoxString,
	table::Table,
	value::Value,
};

pub struct Import {
	pub name: LoxString,
	module: Module,
	globals: UnsafeCell<Table>,
	symbols: Array<Symbol>,
	strings: Array<Gc<LoxString>>,
}

impl Import {
	pub fn new(name: impl Into<LoxString>) -> Self {
		Self {
			name: name.into(),
			module: Module::new(),
			globals: UnsafeCell::default(),
			symbols: Array::new(),
			strings: Array::new(),
		}
	}

	pub(crate) fn with_module(
		name: impl Into<LoxString>,
		module: Module,
		interner: &mut Interner,
	) -> Self {
		let symbols = module
			.identifiers()
			.iter()
			.map(|identifier| interner.intern(identifier.clone()))
			.collect();

		let strings: Array<Gc<LoxString>> = module
			.strings
			.iter()
			.map(|value| lox_gc::manage(value.into()))
			.collect();

		Self {
			name: name.into(),
			module,
			globals: UnsafeCell::default(),
			symbols,
			strings,
		}
	}

	pub fn copy_to(&self, other: &Self) {
		let dst = unsafe { &mut *other.globals.get() };
		self.globals().copy_to(dst);
	}

	fn globals(&self) -> &Table {
		unsafe { &*self.globals.get() }
	}

	#[inline]
	pub(crate) fn symbol(&self, index: ConstantIndex) -> Symbol {
		unsafe { *self.symbols.get_unchecked(index) }
	}

	pub(crate) fn chunk(&self, index: ChunkIndex) -> &Chunk {
		self.module.chunk(index)
	}

	#[inline]
	pub(crate) fn number(&self, index: ConstantIndex) -> f64 {
		self.module.number(index)
	}

	#[inline]
	pub(crate) fn string(&self, index: ConstantIndex) -> Gc<LoxString> {
		unsafe { *self.strings.get_unchecked(index) }
	}

	pub(crate) fn class(&self, index: ClassIndex) -> &lox_bytecode::bytecode::Class {
		self.module.class(index)
	}

	pub(crate) fn closure(&self, index: ClosureIndex) -> &lox_bytecode::bytecode::Closure {
		self.module.closure(index)
	}

	pub fn set_global(&self, key: Symbol, value: Value) {
		let globals = unsafe { &mut *self.globals.get() };
		globals.set(key, value);
	}

	pub fn has_global(&self, key: Symbol) -> bool {
		self.globals().has(key)
	}

	#[inline]
	pub fn global(&self, key: Symbol) -> Option<Value> {
		self.globals().get(key)
	}
}

unsafe impl Trace for Import {
	fn trace(&self, tracer: &mut Tracer<'_>) {
		self.name.trace(tracer);
		self.globals.trace(tracer);
		self.symbols.mark(tracer);
		self.strings.trace(tracer);
	}
}
