use std::{
	alloc::Layout,
	mem,
	ops::{Deref, DerefMut},
	ptr::{self, NonNull},
};

use lox_gc::{Trace, Tracer};

pub struct Array<T> {
	ptr: NonNull<T>,
	len: usize,
	cap: usize,
}

impl<T> Array<T> {
	pub const fn new() -> Self {
		assert!(mem::size_of::<T>() != 0, "must not be ZST");

		Self {
			ptr: NonNull::dangling(),
			len: 0,
			cap: 0,
		}
	}

	pub fn with_capacity(capacity: usize) -> Self {
		if capacity == 0 {
			Self::new()
		} else {
			let ptr = unsafe {
				NonNull::new_unchecked(
					lox_gc::alloc(Layout::array::<T>(capacity).unwrap()).cast::<T>(),
				)
			};

			Self {
				cap: capacity,
				len: 0,
				ptr,
			}
		}
	}

	pub fn push(&mut self, value: T) {
		if self.len == self.cap {
			self.grow();
		}

		unsafe {
			ptr::write(self.ptr.as_ptr().add(self.len), value);
		}

		self.len += 1;
	}

	pub fn pop(&mut self) -> Option<T> {
		if self.len == 0 {
			None
		} else {
			self.len -= 1;

			unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
		}
	}

	pub fn swap_remove(&mut self, index: usize) -> T {
		fn assert_failed(index: usize, len: usize) -> ! {
			panic!("swap_remove index (is {index}) should be < len (is {len})");
		}

		let len = self.len;
		if index >= len {
			assert_failed(index, len);
		}

		unsafe {
			let value = ptr::read(self.as_ptr().add(index));
			let base_ptr = self.as_mut_ptr();
			ptr::copy(base_ptr.add(len - 1), base_ptr.add(index), 1);
			self.len -= 1;
			value
		}
	}

	fn grow(&mut self) {
		let (new_cap, new_layout) = if self.cap == 0 {
			(1, Layout::array::<T>(1).unwrap())
		} else {
			let new_cap = 2 * self.cap;
			let new_layout = Layout::array::<T>(new_cap).unwrap();
			(new_cap, new_layout)
		};

		assert!(
			isize::try_from(new_layout.size()).is_ok(),
			"Allocation too large"
		);

		let new_ptr = if self.cap == 0 {
			unsafe { lox_gc::alloc(new_layout) }
		} else {
			let old_layout = Layout::array::<T>(self.cap).unwrap();
			let old_ptr = self.ptr.as_ptr().cast::<u8>();
			let new_ptr = unsafe { lox_gc::alloc(new_layout) };

			unsafe {
				ptr::copy_nonoverlapping(old_ptr, new_ptr, old_layout.size());
			}

			new_ptr
		};

		self.ptr = unsafe { NonNull::new_unchecked(new_ptr.cast::<T>()) };
		self.cap = new_cap;
	}

	pub fn mark(&self, tracer: &mut Tracer<'_>) {
		if self.cap == 0 {
			return;
		}

		unsafe {
			tracer.mark(self.ptr.as_ptr() as *const u8);
		}
	}
}

impl<T: Clone> Array<T> {
	pub fn extend_from_slice(&mut self, other: &[T]) {
		for elem in other {
			self.push(elem.clone());
		}
	}
}

impl<T: Copy> Array<T> {
	pub fn with_contents(elem: T, size: usize) -> Self {
		let mut array = Self::with_capacity(size);

		for _ in 0..size {
			array.push(elem);
		}

		array
	}
}

impl<T> Clone for Array<T> {
	fn clone(&self) -> Self {
		unsafe {
			let ptr = lox_gc::alloc(Layout::array::<T>(self.cap).unwrap()).cast::<T>();
			ptr::copy_nonoverlapping(self.ptr.as_ptr(), ptr, self.cap);

			Self {
				cap: self.cap,
				len: self.len,
				ptr: NonNull::new_unchecked(ptr),
			}
		}
	}
}

impl<T> Default for Array<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> Deref for Array<T> {
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
	}
}

impl<T> DerefMut for Array<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
	}
}

impl<T> FromIterator<T> for Array<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		let iter = iter.into_iter();

		let mut array = Self::new();

		for elem in iter {
			array.push(elem);
		}

		array
	}
}

unsafe impl<T> Trace for Array<T>
where
	T: Trace,
{
	fn trace(&self, tracer: &mut Tracer<'_>) {
		self.mark(tracer);
	}
}
