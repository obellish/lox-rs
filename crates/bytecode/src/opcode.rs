pub const TRUE: u8 = 1;
pub const FALSE: u8 = 2;
pub const NIL: u8 = 3;

pub const NEGATE: u8 = 4;
pub const ADD: u8 = 5;
pub const SUBTRACT: u8 = 6;
pub const MULTIPLY: u8 = 7;
pub const DIVIDE: u8 = 8;

pub const NOT: u8 = 9;
pub const EQUAL: u8 = 10;
pub const GREATER: u8 = 11;
pub const LESS: u8 = 12;

pub const POP: u8 = 13;

pub const RETURN: u8 = 14;
pub const PRINT: u8 = 15;

pub const DEFINE_GLOBAL: u8 = 16;
pub const GET_GLOBAL: u8 = 17;
pub const SET_GLOBAL: u8 = 18;
pub const GET_LOCAL: u8 = 19;
pub const SET_LOCAL: u8 = 20;
pub const GET_UPVALUE: u8 = 21;
pub const SET_UPVALUE: u8 = 22;
pub const SET_PROPERTY: u8 = 23;
pub const GET_PROPERTY: u8 = 24;

pub const JUMP: u8 = 25;
pub const JUMP_IF_FALSE: u8 = 26;
pub const CALL: u8 = 27;
pub const INVOKE: u8 = 28;
pub const CLOSE_UPVALUE: u8 = 29;

pub const CLASS: u8 = 30;
pub const CLOSURE: u8 = 31;
pub const METHOD: u8 = 32;

pub const IMPORT: u8 = 33;
pub const IMPORT_GLOBAL: u8 = 34;

pub const LIST: u8 = 35;
pub const GET_INDEX: u8 = 36;
pub const SET_INDEX: u8 = 37;

pub const NUMBER: u8 = 38;
pub const STRING: u8 = 39;

pub const RETURN_TOP: u8 = 40;

pub struct OpCodeIterator<T> {
	offset: usize,
	inner: T,
}

impl<T> OpCodeIterator<T> {
	pub const fn new(inner: T) -> Self {
		Self { offset: 0, inner }
	}
}

impl<T> OpCodeIterator<T>
where
	T: Iterator<Item = u8>,
{
	fn next_u8(&mut self) -> Option<u8> {
		self.offset += 1;
		self.inner.next()
	}

	fn next_u32(&mut self) -> Option<u32> {
		let bytes = [
			self.next_u8()?,
			self.next_u8()?,
			self.next_u8()?,
			self.next_u8()?,
		];
		Some(u32::from_le_bytes(bytes))
	}

	fn next_i16(&mut self) -> Option<i16> {
		let bytes = [self.next_u8()?, self.next_u8()?];
		Some(i16::from_le_bytes(bytes))
	}

	fn next_u16(&mut self) -> Option<u16> {
		let bytes = [self.next_u8()?, self.next_u8()?];
		Some(u16::from_le_bytes(bytes))
	}
}

impl<T> Iterator for OpCodeIterator<T>
where
	T: Iterator<Item = u8>,
{
	type Item = (usize, OpCode);

	fn next(&mut self) -> Option<Self::Item> {
		let offset = self.offset;
		self.offset += 1;
		let opcode = self.inner.next()?;

		let opcode = match opcode {
			TRUE => OpCode::True,
			FALSE => OpCode::False,
			NIL => OpCode::Nil,

			NEGATE => OpCode::Negate,
			ADD => OpCode::Add,
			SUBTRACT => OpCode::Subtract,
			MULTIPLY => OpCode::Multiply,
			DIVIDE => OpCode::Divide,

			NOT => OpCode::Not,
			EQUAL => OpCode::Equal,
			GREATER => OpCode::Greater,
			LESS => OpCode::Less,

			POP => OpCode::Pop,

			RETURN => OpCode::Return,
			PRINT => OpCode::Print,

			DEFINE_GLOBAL => OpCode::DefineGlobal(self.next_u32()?),
			GET_GLOBAL => OpCode::GetGlobal(self.next_u32()?),
			SET_GLOBAL => OpCode::SetGlobal(self.next_u32()?),
			GET_LOCAL => OpCode::GetLocal(self.next_u32()?),
			SET_LOCAL => OpCode::SetLocal(self.next_u32()?),
			GET_UPVALUE => OpCode::GetUpvalue(self.next_u32()?),
			SET_UPVALUE => OpCode::SetUpvalue(self.next_u32()?),
			GET_PROPERTY => OpCode::GetProperty(self.next_u32()?),
			SET_PROPERTY => OpCode::SetProperty(self.next_u32()?),

			JUMP => OpCode::Jump(self.next_i16()?),
			JUMP_IF_FALSE => OpCode::JumpIfFalse(self.next_i16()?),
			CALL => OpCode::Call(self.next_u8()?),
			INVOKE => OpCode::Invoke(self.next_u8()?, self.next_u32()?),
			CLOSE_UPVALUE => OpCode::CloseUpvalue,

			CLASS => OpCode::Class(self.next_u8()?),
			CLOSURE => OpCode::Closure(self.next_u32()?),
			METHOD => OpCode::Method(self.next_u32()?),

			IMPORT => OpCode::Import(self.next_u32()?),
			IMPORT_GLOBAL => OpCode::ImportGlobal(self.next_u32()?),

			LIST => OpCode::List(self.next_u8()?),
			GET_INDEX => OpCode::GetIndex,
			SET_INDEX => OpCode::SetIndex,

			NUMBER => OpCode::Number(self.next_u16()?),
			STRING => OpCode::String(self.next_u16()?),

			RETURN_TOP => OpCode::ReturnTop,

			_ => return None,
		};

		Some((offset, opcode))
	}
}
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
	True,
	False,
	Nil,

	Negate,
	Add,
	Subtract,
	Multiply,
	Divide,

	Not,
	Equal,
	Greater,
	Less,

	Pop,

	Return,
	Print,

	DefineGlobal(u32),
	GetGlobal(u32),
	SetGlobal(u32),
	GetLocal(u32),
	SetLocal(u32),
	GetUpvalue(u32),
	SetUpvalue(u32),
	GetProperty(u32),
	SetProperty(u32),

	Jump(i16),
	JumpIfFalse(i16),
	Call(u8),
	Invoke(u8, u32),
	CloseUpvalue,

	Class(u8),
	Closure(u32),
	Method(u32),

	Import(u32),
	ImportGlobal(u32),

	List(u8),
	GetIndex,
	SetIndex,

	Number(u16),
	String(u16),

	ReturnTop,
}
