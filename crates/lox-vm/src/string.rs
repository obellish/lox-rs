use std::{
	borrow::{Borrow, BorrowMut},
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	hash::Hash,
	ops::{Deref, DerefMut},
	str,
};

use lox_gc::{Trace, Tracer};

use super::array::Array;

pub struct LoxString {
	vec: Array<u8>,
}

impl LoxString {
	pub const fn new() -> Self {
		Self { vec: Array::new() }
	}

	pub fn as_bytes(&self) -> &[u8] {
		&self.vec
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			vec: Array::with_capacity(capacity),
		}
	}

	pub fn as_str(&self) -> &str {
		self
	}

	pub fn push_str(&mut self, string: &str) {
		self.vec.extend_from_slice(string.as_bytes());
	}
}

impl AsRef<[u8]> for LoxString {
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl AsRef<str> for LoxString {
	fn as_ref(&self) -> &str {
		self
	}
}

impl Borrow<str> for LoxString {
	fn borrow(&self) -> &str {
		&self[..]
	}
}

impl BorrowMut<str> for LoxString {
	fn borrow_mut(&mut self) -> &mut str {
		&mut self[..]
	}
}

impl Clone for LoxString {
	fn clone(&self) -> Self {
		Self {
			vec: self.vec.clone(),
		}
	}
}

impl Debug for LoxString {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Debug::fmt(&**self, f)
	}
}

impl Default for LoxString {
	fn default() -> Self {
		Self::new()
	}
}

impl Deref for LoxString {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		unsafe { str::from_utf8_unchecked(&self.vec) }
	}
}

impl DerefMut for LoxString {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { str::from_utf8_unchecked_mut(&mut self.vec) }
	}
}

impl Display for LoxString {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Display::fmt(&**self, f)
	}
}

impl Eq for LoxString {}

impl From<String> for LoxString {
	fn from(value: String) -> Self {
		let mut str = Self::with_capacity(value.len());
		str.push_str(&value);
		str
	}
}

impl From<&String> for LoxString {
	fn from(value: &String) -> Self {
		let mut str = Self::with_capacity(value.len());
		str.push_str(value);
		str
	}
}

impl From<&str> for LoxString {
	fn from(value: &str) -> Self {
		let mut str = Self::with_capacity(value.len());
		str.push_str(value);
		str
	}
}

impl Hash for LoxString {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		(**self).hash(state);
	}
}

impl PartialEq for LoxString {
	fn eq(&self, other: &Self) -> bool {
		PartialEq::eq(&self[..], &other[..])
	}
}

unsafe impl Trace for LoxString {
	fn trace(&self, tracer: &mut Tracer<'_>) {
		self.vec.mark(tracer);
	}
}
