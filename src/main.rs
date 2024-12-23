use std::{
	fs::read_to_string,
	io::{stdin, stdout, Write},
	path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use lox::{Args, Chunk, OpCode, Vm, VmError};

fn main() -> Result<()> {
	let args = match Args::try_parse() {
		Ok(args) => args,
		Err(e) => {
			eprintln!("{e}");
			return Ok(());
		}
	};

	let mut vm = Vm::new();

	if let Some(input_file) = args.input_file {
		run_file(input_file, &mut vm)
	} else {
		run_repl(&mut vm)
	}
}

fn run_repl(vm: &mut Vm) -> Result<()> {
	loop {
		print!("> ");
		stdout().flush()?;
		let value = stdin().lines().next();
		if let Some(Ok(line)) = value {
			if line.trim().is_empty() {
				break;
			}

			vm.interpret(line.trim().to_owned())?;
		}
	}

	Ok(())
}

fn run_file(path: PathBuf, vm: &mut Vm) -> Result<()> {
	let data = read_to_string(path)?;

	vm.interpret(data)?;

	Ok(())
}
