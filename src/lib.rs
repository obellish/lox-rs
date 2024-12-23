mod chunk;
mod compiler;
mod vm;

use std::path::PathBuf;

use clap::Parser;

pub use self::{chunk::*, compiler::*, vm::*};

#[derive(Debug, Clone, Parser)]
#[command()]
#[repr(transparent)]
pub struct Args {
	pub input_file: Option<PathBuf>,
}
