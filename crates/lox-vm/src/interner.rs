use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Symbol(pub u32);

impl Symbol {
	#[must_use]
	pub const fn invalid() -> Self {
		Self(0)
	}
}

pub struct Interner {
	next: u32,
	map: HashMap<String, Symbol>,
}

impl Interner {
	#[must_use]
	pub fn new() -> Self {
		Self {
			next: 1,
			map: HashMap::new(),
		}
	}

	pub fn intern(&mut self, string: String) -> Symbol {
		if let Some(symbol) = self.map.get(&string).copied() {
			symbol
		} else {
			let symbol = Symbol(self.next);
			self.next += 1;
			self.map.insert(string, symbol);
			symbol
		}
	}
}

impl Default for Interner {
	fn default() -> Self {
		Self::new()
	}
}
