use lox_gc::{Trace, Tracer};

use crate::value::Value;

#[derive(Clone, Copy)]
pub enum Upvalue {
	Open(usize),
	Closed(Value),
}

impl Upvalue {
	#[must_use]
	pub const fn is_open_with_range(self, index: usize) -> Option<usize> {
		match self {
			Self::Open(i) => {
				if i >= index {
					Some(i)
				} else {
					None
				}
			}
			Self::Closed(_) => None,
		}
	}

	#[must_use]
	pub const fn is_open_with_index(self, index: usize) -> bool {
		match self {
			Self::Open(i) => i == index,
			Self::Closed(_) => false,
		}
	}

	#[must_use]
	pub const fn is_open(self) -> bool {
		matches!(self, Self::Open(..))
	}
}

unsafe impl Trace for Upvalue {
	fn trace(&self, tracer: &mut Tracer<'_>) {
		match self {
			Self::Open(_) => (),
			Self::Closed(value) => value.trace(tracer),
		}
	}
}
