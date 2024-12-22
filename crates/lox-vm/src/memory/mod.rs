mod bound_method;
mod class;

use std::fmt::{Formatter, Result as FmtResult};

use lox_gc::Gc;

pub use self::{bound_method::*, class::*};

pub fn print(value: Gc<()>, f: &mut Formatter<'_>) -> FmtResult {
	f.write_str("<unknown>")
}
