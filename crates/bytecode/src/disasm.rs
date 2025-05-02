use super::{
	Module,
	opcode::{OpCode, OpCodeIterator},
};

pub fn disassemble_module(module: &Module) {
	println!("=== Start of Dump ===");
	println!();

	for (index, chunk) in module.chunks().iter().enumerate() {
		println!("=== Chunk {index} ===");
		disassemble_chunk(chunk.as_bytes(), module);
		println!();
	}

	println!("=== Classes ===");
	for (index, class) in module.classes().iter().enumerate() {
		println!("{index} {class}");
	}
	println!();

	println!("=== Closures ===");
	for (index, closure) in module.closures().iter().enumerate() {
		println!("{index} {closure:?}");
	}
	println!();

	println!("=== Identifiers ===");
	for (index, identifier) in module.identifiers().iter().enumerate() {
		println!("{index} {identifier}");
	}
	println!();

	println!("=== Numbers ===");
	for (index, constant) in module.numbers.iter().copied().enumerate() {
		println!("{index} {constant}");
	}
	println!();

	println!("=== Strings ===");
	for (index, constant) in module.strings.iter().enumerate() {
		println!("{index} {constant}");
	}
	println!();

	println!("=== End of Dump ===");
	println!();
}

pub fn disassemble_chunk(chunk: &[u8], module: &Module) {
	let chunk = OpCodeIterator::new(chunk.iter().copied());

	for (offset, opcode) in chunk {
		let instruction = format!("{opcode:?}");

		match opcode {
			OpCode::JumpIfFalse(relative) | OpCode::Jump(relative) => println!(
				"{offset:04X} {instruction:<18} {:04X}",
				absolute(offset, relative)
			),
			OpCode::DefineGlobal(index)
			| OpCode::GetGlobal(index)
			| OpCode::SetGlobal(index)
			| OpCode::Invoke(_, index)
			| OpCode::GetProperty(index)
			| OpCode::SetProperty(index) => {
				println!("{offset:04X} {instruction:<18} {}", unsafe {
					module.identifier_unchecked(index as _)
				});
			}
			OpCode::Number(index) => println!("{offset:04X} {instruction:<18} {}", unsafe {
				module.number_unchecked(index as _)
			}),
			OpCode::String(index) => println!("{offset:04X} {instruction:<18} {}", unsafe {
				module.string_unchecked(index as _)
			}),
			_ => println!("{offset:04X} {instruction:<18}"),
		}
	}
}

const fn absolute(offset: usize, relative: i16) -> usize {
	let offset = offset as i64;
	let relative = relative as i64;
	let absolute = offset + relative + 3;
	absolute as usize
}
