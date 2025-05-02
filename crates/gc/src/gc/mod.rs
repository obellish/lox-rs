mod vtable;

use std::{
	any::TypeId,
	cell::{Cell, RefCell},
	ops::Deref,
	ptr::{self, NonNull},
};

use super::heap::Heap;

#[repr(transparent)]
pub struct Tracer<'heap> {
	heap: &'heap ManagedHeap,
}

impl Tracer<'_> {
	pub unsafe fn mark(&self, ptr: *const u8) {
		unsafe { self.heap.heap.mark(ptr) };
	}
}

pub struct ManagedHeap {
	pub(crate) heap: Heap,
	finalizers: RefCell<Vec<Gc<()>>>,
	threshold: Cell<usize>,
}

#[repr(transparent)]
pub struct Gc<T: ?Sized> {
	ptr: NonNull<Allocation<T>>,
}

impl<T: ?Sized> Gc<T> {
	pub fn ptr_eq(a: Self, b: Self) -> bool {
		std::ptr::addr_eq(a.ptr.as_ptr(), b.ptr.as_ptr())
	}

	const fn allocation(&self) -> &Allocation<T> {
		unsafe { self.ptr.as_ref() }
	}

	fn dyn_data(&self) -> &dyn Trace {
		let ptr = self.ptr.as_ptr() as *const Allocation<()>;
		unsafe {
			let data = ptr::addr_of!((*ptr).data);
			self::vtable::construct(data, self.allocation().vtable)
		}
	}

	#[expect(clippy::mut_from_ref)]
	fn dyn_data_mut(&self) -> &mut dyn Trace {
		let ptr = self.ptr.as_ptr().cast::<Allocation<()>>();
		unsafe {
			let data = ptr::addr_of_mut!((*ptr).data);
			self::vtable::construct_mut(data, self.allocation().vtable)
		}
	}
}

#[allow(clippy::trivially_copy_pass_by_ref)]
impl<T> Gc<T> {
	#[must_use]
	pub fn is_same_type(a: &Self, b: &Self) -> bool {
		a.allocation().tag == b.allocation().tag
	}

	#[must_use]
	pub const fn erase(self) -> Gc<()> {
		Gc {
			ptr: unsafe { NonNull::new_unchecked(self.ptr.as_ptr().cast::<Allocation<()>>()) },
		}
	}

	#[must_use]
	pub fn to_bits(self) -> u64 {
		self.ptr.as_ptr() as u64
	}

	#[must_use]
	pub const unsafe fn from_bits(value: u64) -> Self {
		Self {
			ptr: unsafe { NonNull::new_unchecked(value as *mut Allocation<T>) },
		}
	}
}

impl<T: ?Sized> Clone for Gc<T> {
	fn clone(&self) -> Self {
		*self
	}
}

impl<T: ?Sized> Copy for Gc<T> {}

impl<T: ?Sized> Deref for Gc<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		let allocation = unsafe { self.ptr.as_ref() };

		&allocation.data
	}
}

unsafe impl<T: ?Sized> Trace for Gc<T> {
	fn trace(&self, tracer: &mut Tracer<'_>) {
		let ptr = self.ptr.as_ptr() as *const u8;

		if !tracer.heap.heap.is_marked(ptr) {
			unsafe {
				tracer.heap.heap.mark(ptr);
			}

			self.dyn_data().trace(tracer);
		}
	}
}

#[repr(C)]
struct Allocation<T: ?Sized> {
	tag: TypeId,
	vtable: *mut (),
	data: T,
}

pub unsafe trait Trace {
	fn trace(&self, tracer: &mut Tracer<'_>);
}
