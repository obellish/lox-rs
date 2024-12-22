use std::ptr;

use lox_gc::{Trace, Tracer};

use super::value::Value;

pub struct StackBlock {
	stack: *mut u8,
}

impl StackBlock {
	pub fn new(size: usize) -> Self {
		let stack = unsafe { lox_gc::alloc(std::alloc::Layout::array::<Value>(size).unwrap()) };

		Self { stack }
	}
}

unsafe impl Trace for StackBlock {
	fn trace(&self, tracer: &mut Tracer<'_>) {
		unsafe {
			tracer.mark(self.stack);
		}
	}
}

#[derive(Clone, Copy)]
pub struct Stack {
	top: *mut Value,
	bottom: *mut Value,
}

#[allow(clippy::cast_ptr_alignment)]
impl Stack {
	pub const fn with_block(block: &StackBlock) -> Self {
		Self {
			top: block.stack.cast::<Value>(),
			bottom: block.stack.cast::<Value>(),
		}
	}

	#[inline]
	pub const fn len(&self) -> usize {
		unsafe { self.top.offset_from(self.bottom) as usize }
	}

	#[inline]
	pub fn truncate(&mut self, top: usize) {
		unsafe {
			self.top = self.bottom.add(top);
		}
	}

	#[inline]
	pub fn push(&mut self, value: Value) {
		unsafe {
			ptr::write(self.top, value);
			self.top = self.top.add(1);
		}
	}

	#[inline]
	pub fn pop(&mut self) -> Value {
		unsafe {
			self.top = self.top.sub(1);
			ptr::read(self.top)
		}
	}

	#[inline]
	pub fn rset(&mut self, n: usize, value: Value) {
		unsafe {
			let ptr = self.top.sub(n + 1);
			ptr::write(ptr, value);
		}
	}

	#[inline]
	pub fn set(&mut self, index: usize, value: Value) {
		unsafe {
			let ptr = self.bottom.add(index);
			ptr::write(ptr, value);
		}
	}

	#[inline]
	pub const fn get(&self, index: usize) -> Value {
		unsafe {
			let ptr = self.bottom.add(index);
			ptr::read(ptr)
		}
	}

	#[inline]
	pub const fn peek_n(&self, n: usize) -> Value {
		unsafe {
			let ptr = self.top.sub(n + 1);
			ptr::read(ptr)
		}
	}

	#[inline]
	pub fn pop_n(&mut self, n: usize) -> Vec<Value> {
		unsafe {
			self.top = self.top.sub(n);
			let slice = std::ptr::slice_from_raw_parts(self.top, n);
			(*slice).to_owned()
		}
	}
}

unsafe impl Trace for Stack {
	fn trace(&self, tracer: &mut Tracer<'_>) {
		for i in 0..self.len() {
			self.get(i).trace(tracer);
		}
	}
}
