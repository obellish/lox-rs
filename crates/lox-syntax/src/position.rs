use std::cmp;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BytePos(pub u32);

impl BytePos {
	#[must_use]
	pub const fn shift(self, ch: char) -> Self {
		Self(self.0 + ch.len_utf8() as u32)
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
	pub start: BytePos,
	pub end: BytePos,
}

impl Span {
	#[must_use]
	pub const fn new(start: BytePos, end: BytePos) -> Self {
		Self { start, end }
	}

	#[must_use]
	pub const unsafe fn new_unchecked(start: u32, end: u32) -> Self {
		Self {
			start: BytePos(start),
			end: BytePos(end),
		}
	}

	#[must_use]
	pub const fn empty() -> Self {
		// SAFETY: can't be used to index
		unsafe { Self::new_unchecked(0, 0) }
	}

	#[must_use]
	pub fn union_span(a: Self, b: Self) -> Self {
		Self {
			start: cmp::min(a.start, b.start),
			end: cmp::max(a.end, b.end),
		}
	}

	pub fn union<A, B>(a: &WithSpan<A>, b: &WithSpan<B>) -> Self {
		Self::union_span(a.into(), b.into())
	}
}

impl<T> From<WithSpan<T>> for Span {
	fn from(value: WithSpan<T>) -> Self {
		value.span
	}
}

impl<T> From<&WithSpan<T>> for Span {
	fn from(value: &WithSpan<T>) -> Self {
		value.span
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WithSpan<T> {
	pub value: T,
	pub span: Span,
}

impl<T> WithSpan<T> {
	pub const fn new(value: T, span: Span) -> Self {
		Self { value, span }
	}

	pub const fn empty(value: T) -> Self {
		Self::new(value, Span::empty())
	}

	pub const unsafe fn new_unchecked(value: T, start: u32, end: u32) -> Self {
		Self::new(value, Span::new_unchecked(start, end))
	}

	pub const fn as_ref(&self) -> WithSpan<&T> {
		WithSpan {
			span: self.span,
			value: &self.value,
		}
	}
}

impl<T> AsRef<T> for WithSpan<T> {
	fn as_ref(&self) -> &T {
		&self.value
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Diagnostic {
	pub span: Span,
	pub message: String,
}

#[derive(Debug, Clone)]
pub struct LineOffsets {
	offsets: Vec<u32>,
	len: u32,
}

impl LineOffsets {
	#[must_use]
	pub fn new(data: &str) -> Self {
		let mut offsets = vec![0];
		let len = data.len() as u32;

		for (i, val) in data.bytes().enumerate() {
			if val == b'\n' {
				offsets.push((i + 1) as u32);
			}
		}

		Self { offsets, len }
	}

	#[must_use]
	pub fn line(&self, pos: BytePos) -> usize {
		let offset = pos.0;
		assert!(offset <= self.len);

		match self.offsets.binary_search(&offset) {
			Ok(line) => line + 1,
			Err(line) => line,
		}
	}
}
