use super::{bytecode::Module, opcode::Opcode};
use crate::opcode::OpcodeIterator;

pub fn disassemble_module(module: &Module) {
	println!("=== Start of Dump ===");
	println!();

	for (index, chunk) in module.chunks().iter().enumerate() {
		println!("=== Chunk {index} ===");
		disassemble_chunk(chunk, module);
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
	for (index, constant) in module.numbers.iter().enumerate() {
		println!("{index} {constant:?}");
	}
	println!();

	println!("=== Strings ===");
	for (index, constant) in module.strings.iter().enumerate() {
		println!("{index} {constant:?}");
	}
	println!();

	println!("=== End of Dump ===");
	println!();
}

pub fn disassemble_chunk(chunk: &[u8], module: &Module) {
	let chunk = OpcodeIterator::new(chunk.iter().copied());
	for (offset, opcode) in chunk {
		let instruction = format!("{opcode:?}");
		match opcode {
			Opcode::Jump(relative) | Opcode::JumpIfFalse(relative) => println!(
				"{offset:04X} {instruction:<18} {:04X}",
				absolute(offset, relative)
			),
			Opcode::DefineGlobal(index) | Opcode::GetGlobal(index) | Opcode::SetGlobal(index) => {
				println!(
					"{offset:04X} {instruction:<18} {}",
					module.identifier(index as _)
				);
			}
			Opcode::Number(index) => println!(
				"{offset:04X} {instruction:<18} {}",
				module.number(index as _)
			),
			Opcode::String(index) => println!(
				"{offset:04X} {instruction:<18} {}",
				module.string(index as _)
			),
			Opcode::Invoke(_arity, index) => println!(
				"{offset:04X} {instruction:<18} {}",
				module.identifier(index as _)
			),
			Opcode::GetProperty(index) | Opcode::SetProperty(index) => println!(
				"{offset:04X} {instruction:<18} {}",
				module.identifier(index as _)
			),
			_ => println!("{offset:04X} {instruction:<18}"),
		}
	}
}

const fn absolute(offset: usize, relative: i16) -> usize {
	let offset = offset as i64;
	let relative = relative as i64;
	let absolute = offset + relative + 3;
	absolute as _
}
