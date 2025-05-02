#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

use std::alloc::{Layout, alloc, dealloc};

pub struct MemoryMap {
	size: usize,
	data: *mut u8,
}

impl MemoryMap {
	#[must_use]
	pub fn new(size: usize) -> Self {
		unsafe {
			let layout = Layout::array::<u8>(size).unwrap().align_to(4096).unwrap();
			let data = alloc(layout);

			Self { size, data }
		}
	}

	#[must_use]
	pub const fn data(&self) -> *mut u8 {
		self.data
	}
}

impl Drop for MemoryMap {
	fn drop(&mut self) {
		let layout = Layout::array::<u8>(self.size)
			.unwrap()
			.align_to(4096)
			.unwrap();

		unsafe { dealloc(self.data, layout) }
	}
}
