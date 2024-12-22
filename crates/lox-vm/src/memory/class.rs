use std::cell::UnsafeCell;

use crate::{interner::Symbol, string::LoxString, table::Table, value::Value};

#[derive(Debug)]
pub struct Class {
	pub name: LoxString,
	methods: UnsafeCell<Table>,
}

impl Class {
	pub fn new(name: impl Into<LoxString>) -> Self {
		Self {
			name: name.into(),
			methods: UnsafeCell::default(),
		}
	}

	#[inline]
	pub fn method(&self, symbol: Symbol) -> Option<Value> {
		self.methods().get(symbol)
	}

	pub fn set_method(&self, symbol: Symbol, closure: Value) {
		let methods = unsafe { &mut *self.methods.get() };
		methods.set(symbol, closure);
	}

	fn methods(&self) -> &Table {
		unsafe { &*self.methods.get() }
	}
}
