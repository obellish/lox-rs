use std::mem;

use super::Trace;

#[repr(C)]
struct Object {
	data: *const (),
	vtable: *mut (),
}

pub fn extract<T: Trace>(data: *const T) -> *mut () {
	unsafe {
		let obj = data as *const dyn Trace;
		mem::transmute::<*const dyn Trace, Object>(obj).vtable
	}
}

pub unsafe fn construct<'a>(data: *const (), vtable: *mut ()) -> &'a dyn Trace {
	unsafe {
		let object = Object { data, vtable };

		mem::transmute(object)
	}
}

pub unsafe fn construct_mut<'a>(data: *mut (), vtable: *mut ()) -> &'a mut dyn Trace {
	unsafe {
		let object = Object { data, vtable };

		mem::transmute(object)
	}
}
