use std::cell::Cell;

use lox_gc::{Gc, Trace, Tracer};

use crate::memory::*;

pub struct CallFrame {
	pub base_counter: usize,
	pub closure: Gc<Closure>,
	ip: Cell<*const u8>,
}

#[allow(clippy::inline_always)]
impl CallFrame {
	pub fn new(object: Gc<Closure>, base_counter: usize) -> Self {
		let ip = object
			.function
			.import
			.chunk(object.function.chunk_index)
			.as_ptr();
		Self {
			base_counter,
			closure: object,
			ip: Cell::new(ip),
		}
	}

	#[inline(always)]
	pub fn load_ip(&self) -> *const u8 {
		self.ip.get()
	}

	#[inline(always)]
	pub fn store_ip(&self, ip: *const u8) {
		self.ip.set(ip);
	}
}

unsafe impl Trace for CallFrame {
	#[inline]
	fn trace(&self, tracer: &mut Tracer<'_>) {
		self.closure.trace(tracer);
	}
}
