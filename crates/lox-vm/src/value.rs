use std::fmt::{Display, Formatter, Result as FmtResult};

use lox_gc::{Gc, Trace, Tracer};

const QNAN: u64 = 0x7ffc_0000_0000_0000;
const SIGN_BIT: u64 = 0x8000_0000_0000_0000;
const TAG_NIL: u64 = 1;
const TAG_FALSE: u64 = 2;
const TAG_TRUE: u64 = 3;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Value(u64);

impl Value {
	pub const FALSE: Self = Self(QNAN | TAG_FALSE);
	pub const NIL: Self = Self(QNAN | TAG_NIL);
	pub const TRUE: Self = Self(QNAN | TAG_TRUE);

	#[inline]
	#[must_use]
	pub fn from_object<T>(value: Gc<T>) -> Self {
		let bits = value.to_bits();
		Self(SIGN_BIT | QNAN | bits)
	}

	#[inline]
	#[must_use]
	pub const fn is_object(self) -> bool {
		self.0 & (SIGN_BIT | QNAN) == (SIGN_BIT | QNAN)
	}

	#[inline]
	#[must_use]
	pub const fn as_object(self) -> Gc<()> {
		unsafe {
			let bits = self.0 & (!(SIGN_BIT | QNAN));
			Gc::from_bits(bits)
		}
	}

	#[inline]
	#[must_use]
	pub const fn is_number(self) -> bool {
		self.0 & QNAN != QNAN
	}

	#[inline]
	#[must_use]
	pub const fn is_bool(self) -> bool {
		self.0 == Self::TRUE.0 || self.0 == Self::FALSE.0
	}

	#[inline]
	#[must_use]
	pub const fn is_nil(self) -> bool {
		self.0 == Self::NIL.0
	}

	#[inline]
	#[must_use]
	pub fn as_number(self) -> f64 {
		f64::from_bits(self.0)
	}

	#[inline]
	#[must_use]
	pub const fn to_bits(self) -> u64 {
		self.0
	}

	#[inline]
	#[must_use]
	pub const fn is_falsey(self) -> bool {
		if self.0 == Self::FALSE.0 {
			true
		} else {
			self.is_nil()
		}
	}

	#[inline]
	#[must_use]
	pub fn is_same_type(a: Self, b: Self) -> bool {
		if (a.is_number() && b.is_number())
			|| (a.is_nil() && b.is_nil())
			|| (a.is_object() && b.is_object())
		{
			true
		} else if a.is_object() && b.is_object() {
			Gc::is_same_type(&a.as_object(), &b.as_object())
		} else {
			false
		}
	}

	#[inline]
	#[must_use]
	pub fn is_object_of_type<T>(self) -> bool
	where
		T: 'static,
	{
		self.is_object() && self.as_object().is::<T>()
	}

	#[inline]
	#[must_use]
	pub fn try_cast<T>(self) -> Option<Gc<T>>
	where
		T: 'static,
	{
		if self.is_object_of_type::<T>() {
			Some(self.as_object().cast::<T>())
		} else {
			None
		}
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		if self.is_nil() {
			f.write_str("nil")
		} else if self.0 == Self::TRUE.0 {
			f.write_str("true")
		} else if self.0 == Self::FALSE.0 {
			f.write_str("false")
		} else if self.is_object() {
			super::memory::print(self.as_object(), f)
		} else {
			Display::fmt(&self.as_number(), f)
		}
	}
}

impl From<f64> for Value {
	#[inline]
	fn from(value: f64) -> Self {
		Self(value.to_bits())
	}
}

impl From<bool> for Value {
	#[inline]
	fn from(value: bool) -> Self {
		if value {
			Self::TRUE
		} else {
			Self::FALSE
		}
	}
}

impl PartialEq for Value {
	#[allow(clippy::op_ref)]
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		use crate::string::LoxString;

		if self.is_number() && other.is_number() {
			self.as_number() == other.as_number()
		} else if self.is_object_of_type::<LoxString>() && other.is_object_of_type::<LoxString>() {
			*self.as_object().cast::<LoxString>() == *other.as_object().cast::<LoxString>()
		} else if self.is_object() && other.is_object() {
			self.as_object() == other.as_object()
		} else {
			self.0 == other.0
		}
	}
}

unsafe impl Trace for Value {
	fn trace(&self, tracer: &mut Tracer<'_>) {
		if self.is_object() {
			self.as_object().trace(tracer);
		}
	}
}
