#[allow(dead_code)]
pub struct MemoryMap {
	size: usize,
	data: *mut u8,
}

impl MemoryMap {
	#[must_use]
	pub const fn data(&self) -> *mut u8 {
		self.data
	}
}

#[cfg(not(all(not(miri), unix)))]
impl MemoryMap {
	#[must_use]
	pub fn new(size: usize) -> Self {
		// SAFETY: idk
		unsafe {
			let layout = std::alloc::Layout::array::<u8>(size)
				.unwrap()
				.align_to(4096)
				.unwrap();

			let data = std::alloc::alloc(layout);

			Self { size, data }
		}
	}
}

#[cfg(not(all(not(miri), unix)))]
impl Drop for MemoryMap {
	fn drop(&mut self) {
		let layout = std::alloc::Layout::array::<u8>(self.size)
			.unwrap()
			.align_to(4096)
			.unwrap();

		unsafe {
			std::alloc::dealloc(self.data, layout);
		}
	}
}

#[cfg(all(not(miri), unix))]
impl MemoryMap {
	pub fn new(size: usize) -> Self {
		let addr = unsafe {
			libc::mmap(
				ptr::null_mut(),
				size,
				libc::PROT_READ | libc::PROT_WRITE,
				libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_NORESERVE,
				-1,
				0,
			)
		};

		assert!(!addr.is_null() && addr != libc::MAP_FAILED, "mmap failed");

		Self {
			data: addr as _,
			size,
		}
	}
}

#[cfg(all(not(miri), unix))]
impl Drop for MemoryMap {
	fn drop(&mut self) {
		unsafe {
			libc::munmap(self.data as _, self.size);
		}
	}
}
