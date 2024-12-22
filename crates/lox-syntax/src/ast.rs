use super::position::WithSpan;

pub type Identifier = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
	Bang,
	Minus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
	Slash,
	Star,
	Plus,
	Minus,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,
	BangEqual,
	EqualEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperator {
	And,
	Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
	Binary(
		Box<WithSpan<Expr>>,
		WithSpan<BinaryOperator>,
		Box<WithSpan<Expr>>,
	),
	Grouping(Box<WithSpan<Expr>>),
	Number(f64),
	Boolean(bool),
	Nil,
	This,
	Super(WithSpan<Identifier>),
	String(String),
	Unary(WithSpan<UnaryOperator>, Box<WithSpan<Expr>>),
	Variable(WithSpan<Identifier>),
	Logical(
		Box<WithSpan<Expr>>,
		WithSpan<LogicalOperator>,
		Box<WithSpan<Expr>>,
	),
	Assign(WithSpan<Identifier>, Box<WithSpan<Expr>>),
	Call(Box<WithSpan<Expr>>, Vec<WithSpan<Expr>>),
	Get(Box<WithSpan<Expr>>, WithSpan<Identifier>),
	Set(
		Box<WithSpan<Expr>>,
		WithSpan<Identifier>,
		Box<WithSpan<Expr>>,
	),
	List(Vec<WithSpan<Expr>>),
	ListGet(Box<WithSpan<Expr>>, Box<WithSpan<Expr>>),
	ListSet(
		Box<WithSpan<Expr>>,
		Box<WithSpan<Expr>>,
		Box<WithSpan<Expr>>,
	),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
	Expression(Box<WithSpan<Expr>>),
	Print(Box<WithSpan<Expr>>),
	Var(WithSpan<Identifier>, Option<Box<WithSpan<Expr>>>),
	If(
		Box<WithSpan<Expr>>,
		Box<WithSpan<Stmt>>,
		Option<Box<WithSpan<Stmt>>>,
	),
	Block(Vec<WithSpan<Stmt>>),
	While(Box<WithSpan<Expr>>, Box<WithSpan<Stmt>>),
	Return(Option<Box<WithSpan<Expr>>>),
	Function(
		WithSpan<Identifier>,
		Vec<WithSpan<Identifier>>,
		Vec<WithSpan<Stmt>>,
	),
	Class(
		WithSpan<Identifier>,
		Option<WithSpan<Identifier>>,
		Vec<WithSpan<Stmt>>,
	),
	Import(WithSpan<String>, Option<Vec<WithSpan<String>>>),
}

pub type Ast = Vec<WithSpan<Stmt>>;

pub type BorrowedAst<'a> = &'a [WithSpan<Stmt>];
