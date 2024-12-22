#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
	Done,
	More,
	RuntimeError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmError {
	Unknown,
	StackEmpty,
	FrameEmpty,
	StringConstantExpected,
	GlobalNotDefined,
	InvalidCallee,
	IncorrectArity,
	UnexpectedConstant,
	ClosureConstantExpected,
	UnexpectedValue,
	UndefinedProperty,
	Unimplemented,
	UnknownImport,
	IndexOutOfRange,
}
