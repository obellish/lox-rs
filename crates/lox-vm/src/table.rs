use lox_gc::{Trace, Tracer};

use super::{array::Array, interner::Symbol, value::Value};

#[derive(Clone, Copy)]
struct Entry {
	key: Symbol,
	value: Value,
}

impl Entry {
	pub const fn new() -> Self {
		Self {
			key: Symbol::invalid(),
			value: Value::NIL,
		}
	}
}

unsafe impl Trace for Entry {
	fn trace(&self, tracer: &mut Tracer<'_>) {
		self.value.trace(tracer);
	}
}

pub struct Table {
	count: usize,
	capacity: usize,
	max_capacity: usize,
	entries: Array<Entry>,
}

impl Table {
	const INITIAL_CAPACITY: usize = 8;
	const MAX_LOAD: f32 = 0.75;

	// Needs to be a power of two.

	pub fn new() -> Self {
		let entries = Array::with_contents(
			Entry {
				key: Symbol::invalid(),
				value: Value::NIL,
			},
			Self::INITIAL_CAPACITY,
		);

		Self {
			count: 0,
			capacity: Self::INITIAL_CAPACITY,
			max_capacity: (Self::INITIAL_CAPACITY as f32 * Self::MAX_LOAD) as usize,
			entries,
		}
	}

	#[inline]
	pub const fn capacity(&self) -> usize {
		self.capacity
	}

	pub fn copy_to(&self, other: &mut Self) {
		for entry in self.entries.iter() {
			if entry.key != Symbol::invalid() {
				other.set(entry.key, entry.value);
			}
		}
	}

	#[inline]
	pub fn has(&self, key: Symbol) -> bool {
		if self.count == 0 {
			return false;
		}

		let index = find_entry(self.capacity, &self.entries, key);
		let entry = unsafe { self.entries.get_unchecked(index) };

		entry.key != Symbol::invalid()
	}

	#[inline]
	pub fn set(&mut self, key: Symbol, value: Value) -> bool {
		if self.count + 1 > self.max_capacity {
			self.adjust_capacity();
		}

		let index = find_entry(self.capacity, &self.entries, key);
		let entry = unsafe { self.entries.get_unchecked_mut(index) };

		let is_new = entry.key == Symbol::invalid();

		if is_new {
			self.count += 1;
		}

		entry.key = key;
		entry.value = value;

		is_new
	}

	#[inline]
	pub fn get(&self, key: Symbol) -> Option<Value> {
		if self.count == 0 {
			return None;
		}

		let index = find_entry(self.capacity, &self.entries, key);
		let entry = unsafe { self.entries.get_unchecked(index) };

		if entry.key == Symbol::invalid() {
			None
		} else {
			Some(entry.value)
		}
	}

	fn adjust_capacity(&mut self) {
		let new_capacity = if self.capacity < 8 {
			8
		} else {
			self.capacity * 2
		};

		let mut new_entries = Array::with_contents(Entry::new(), new_capacity);

		for entry in self.entries.iter() {
			let new_index = find_entry(new_capacity, &new_entries, entry.key);
			let new_entry = unsafe { new_entries.get_unchecked_mut(new_index) };

			new_entry.key = entry.key;
			new_entry.value = entry.value;
		}
		self.max_capacity = (new_capacity as f32 * Self::MAX_LOAD) as usize;
		self.capacity = new_capacity;
		self.entries = new_entries;
	}
}

impl Default for Table {
	fn default() -> Self {
		Self::new()
	}
}

unsafe impl Trace for Table {
	fn trace(&self, tracer: &mut Tracer<'_>) {
		self.entries.trace(tracer);
	}
}

fn find_entry(capacity: usize, entries: &[Entry], key: Symbol) -> usize {
	let mut index = key.0 as usize & (capacity - 1);

	loop {
		let entry = unsafe { entries.get_unchecked(index) };

		if entry.key == key || entry.key == Symbol::invalid() {
			return index;
		}

		index = (index + 1) & (capacity - 1);
	}
}
