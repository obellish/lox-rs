#[derive(Debug)]
pub struct Local {
	name: String,
	depth: usize,
	slot: usize,
	initialized: bool,
	is_upvalue: bool,
}

impl Local {
	pub const fn slot(&self) -> usize {
		self.slot
	}

	pub const fn initialized(&self) -> bool {
		self.initialized
	}

	pub const fn captured(&self) -> bool {
		self.is_upvalue
	}
}

#[derive(Debug)]
pub struct Locals {
	stack: Vec<Local>,
	scope_depth: usize,
}

impl Locals {
	pub const fn new() -> Self {
		Self {
			stack: Vec::new(),
			scope_depth: 0,
		}
	}

	pub const fn scope_depth(&self) -> usize {
		self.scope_depth
	}

	pub fn begin_scope(&mut self) {
		self.scope_depth += 1;
	}

	pub fn end_scope(&mut self) -> Vec<Local> {
		self.scope_depth -= 1;
		let index = self
			.stack
			.iter()
			.enumerate()
			.find_map(|(i, l)| {
				if l.depth > self.scope_depth {
					Some(i)
				} else {
					None
				}
			})
			.unwrap_or(self.stack.len());

		self.stack.split_off(index)
	}

	pub fn get(&self, identifier: &str) -> Option<&Local> {
		self.stack.iter().rev().find(|l| l.name == identifier)
	}

	pub fn get_at_current_depth(&self, identifier: &str) -> Option<&Local> {
		self.get_at_depth(identifier, self.scope_depth)
	}

	pub fn mark_captured(&mut self, slot: usize) {
		let local = self.stack.iter_mut().find(|l| l.slot == slot);
		if let Some(local) = local {
			local.is_upvalue = true;
		}
	}

	pub fn get_at_depth(&self, identifier: &str, depth: usize) -> Option<&Local> {
		self.stack
			.iter()
			.rev()
			.find(|l| l.depth == depth && l.name == identifier)
	}

	pub fn mark_initialized(&mut self) {
		let index = self.stack.len() - 1;
		self.stack[index].initialized = true;
	}

	pub fn insert(&mut self, identifier: String) -> Option<&Local> {
		if self.get_at_depth(&identifier, self.scope_depth).is_some() {
			None
		} else {
			self.stack.push(Local {
				name: identifier,
				depth: self.scope_depth,
				slot: self.stack.len(),
				initialized: false,
				is_upvalue: false,
			});
			self.stack.last()
		}
	}
}
