use lox_gc::{Gc, Trace, Tracer};

use crate::value::Value;

#[derive(Clone, Copy)]
pub struct BoundMethod {
	pub receiver: Gc<()>,
	pub method: Value,
}

unsafe impl Trace for BoundMethod {
	fn trace(&self, tracer: &mut Tracer<'_>) {
		self.receiver.trace(tracer);
		self.method.trace(tracer);
	}
}
