mod bound_method;
mod class;
mod closure;
mod import;
mod upvalue;

use std::fmt::{Formatter, Result as FmtResult};

use lox_gc::Gc;

pub use self::{bound_method::*, class::*, closure::*, import::*, upvalue::*};

pub fn print(value: Gc<()>, f: &mut Formatter<'_>) -> FmtResult {
	f.write_str("<unknown>")
}
