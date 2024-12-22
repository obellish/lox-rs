mod gc;
mod heap;

use self::gc::ManagedHeap;
pub use self::gc::{Gc, Trace, Tracer};

thread_local! {
	pub static HEAP: ManagedHeap = ManagedHeap::new();
}

pub fn manage<T>(data: T) -> Gc<T>
where
	T: Trace + 'static,
{
	HEAP.with(|heap| heap.manage(data))
}

#[must_use]
pub unsafe fn alloc(layout: std::alloc::Layout) -> *mut u8 {
	HEAP.with(|heap| heap.alloc(layout))
}

pub fn collect(roots: &[&dyn Trace]) {
	HEAP.with(|heap| {
		heap.collect(roots);
	});
}
