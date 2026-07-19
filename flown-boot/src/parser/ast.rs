use crate::lexer::token::{Identifier, Literal};
use crate::lexer::info::Spanning;

#[derive(Debug, Clone)]
pub struct Program<'a> {
    pub stmts: Vec<Spanning<'a, GlobalStatement<'a>>>,
}

#[derive(Debug, Clone)]
pub enum GlobalStatement<'a> {
    Empty,
    Let(Spanning<'a, LetStatement<'a>>),
}

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Bind(Spanning<'a, Binding<'a>>),
    Lit(Spanning<'a, Literal>),
    Call(Spanning<'a, FunctionCall<'a>>),
    Mut(Spanning<'a, MutableExpression<'a>>),
    Share(Spanning<'a, ShareExpression<'a>>),
    Borrow(Spanning<'a, BorrowExpression<'a>>),
    ShareTy(Spanning<'a, ShareType<'a>>),
    BorrowTy(Spanning<'a, BorrowType<'a>>),
    Typed(Spanning<'a, TypedExpression<'a>>),
    FuncTy(Spanning<'a, FunctionType<'a>>),
    Lambda(Spanning<'a, LambdaDef<'a>>),
    Block(Spanning<'a, Block<'a>>),
    Intr(Spanning<'a, Intrinsic<'a>>),
}

#[derive(Debug, Clone)]
pub struct Intrinsic<'a> {
    pub name: Spanning<'a, Identifier>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Binding<'a> {
    pub ns: Vec<Spanning<'a, Identifier>>,
    pub ident: Spanning<'a, Identifier>,
}

#[derive(Debug, Clone)]
pub struct MutableExpression<'a> {
    pub val: Spanning<'a, Box<Expression<'a>>>,
}

#[derive(Debug, Clone)]
pub struct ShareExpression<'a> {
    pub val: Spanning<'a, Box<Expression<'a>>>,
}

#[derive(Debug, Clone)]
pub struct BorrowExpression<'a> {
    pub val: Spanning<'a, Box<Expression<'a>>>,
}

#[derive(Debug, Clone)]
pub enum LetModifiers {
    None,
}

#[derive(Debug, Clone)]
pub struct LetStatement<'a> {
    pub mods: LetModifiers,
    pub denoter: Spanning<'a, Box<Expression<'a>>>,
    pub denotee: Spanning<'a, Box<Expression<'a>>>,
}

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    Empty,
    Expr(Spanning<'a, Box<Expression<'a>>>),
    Let(Spanning<'a, LetStatement<'a>>),
}

#[derive(Debug, Clone)]
pub struct Block<'a> {
    pub stmts: Vec<Spanning<'a, Statement<'a>>>,
    pub ret: Option<Spanning<'a, Box<Expression<'a>>>>,
}

#[derive(Debug, Clone)]
pub struct ShareType<'a> {
    pub reg: Option<Spanning<'a, Box<Expression<'a>>>>,
    pub ty: Spanning<'a, Box<Expression<'a>>>,
}

#[derive(Debug, Clone)]
pub struct BorrowType<'a> {
    pub reg: Option<Spanning<'a, Box<Expression<'a>>>>,
    pub ty: Spanning<'a, Box<Expression<'a>>>,
}

#[derive(Debug, Clone)]
pub struct FunctionCall<'a> {
    pub callee: Spanning<'a, Box<Expression<'a>>>,
    pub args: Vec<Spanning<'a, Box<Expression<'a>>>>,
}

#[derive(Debug, Clone)]
pub struct TypedExpression<'a> {
    pub val: Spanning<'a, Box<Expression<'a>>>,
    pub ty: Spanning<'a, Box<Expression<'a>>>,
}

#[derive(Debug, Clone)]
pub struct FunctionType<'a> {
    pub params: Vec<Spanning<'a, TypedExpression<'a>>>,
    pub ret: Spanning<'a, Box<Expression<'a>>>,
}

#[derive(Debug, Clone)]
pub struct LambdaDef<'a> {
    pub ty: Spanning<'a, FunctionType<'a>>,
    pub body: Spanning<'a, Box<Expression<'a>>>,
}
