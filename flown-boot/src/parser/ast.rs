use crate::lexer::token::{Identifier, Literal};
use crate::lexer::info::Spanning;

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Spanning<GlobalStatement>>,
}

#[derive(Debug, Clone)]
pub enum GlobalStatement {
    Empty,
    Let(Spanning<LetStatement>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Bind(Spanning<Binding>),
    Lit(Spanning<Literal>),
    Call(Spanning<FunctionCall>),
    Mut(Spanning<MutableExpression>),
    Share(Spanning<ShareExpression>),
    Borrow(Spanning<BorrowExpression>),
    ShareTy(Spanning<ShareType>),
    BorrowTy(Spanning<BorrowType>),
    Typed(Spanning<TypedExpression>),
    FuncTy(Spanning<FunctionType>),
    Lambda(Spanning<LambdaDef>),
    Block(Spanning<Block>),
}

#[derive(Debug, Clone)]
pub struct Binding {
    pub ns: Vec<Spanning<Identifier>>,
    pub ident: Spanning<Identifier>,
}

#[derive(Debug, Clone)]
pub struct MutableExpression {
    pub val: Spanning<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub struct ShareExpression {
    pub val: Spanning<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub struct BorrowExpression {
    pub val: Spanning<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub enum LetModifiers {
    None,
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub mods: LetModifiers,
    pub denoter: Spanning<Box<Expression>>,
    pub denotee: Spanning<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Empty,
    Expr(Spanning<Box<Expression>>),
    Let(Spanning<LetStatement>),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Spanning<Statement>>,
    pub ret: Option<Spanning<Box<Expression>>>,
}

#[derive(Debug, Clone)]
pub struct ShareType {
    pub reg: Option<Spanning<Box<Expression>>>,
    pub ty: Spanning<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub struct BorrowType {
    pub reg: Option<Spanning<Box<Expression>>>,
    pub ty: Spanning<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub callee: Spanning<Box<Expression>>,
    pub args: Vec<Spanning<Box<Expression>>>,
}

#[derive(Debug, Clone)]
pub struct TypedExpression {
    pub val: Spanning<Box<Expression>>,
    pub ty: Spanning<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub struct FunctionType {
    pub params: Vec<Spanning<TypedExpression>>,
    pub ret: Spanning<Box<Expression>>,
}

#[derive(Debug, Clone)]
pub struct LambdaDef {
    pub ty: Spanning<FunctionType>,
    pub body: Spanning<Box<Expression>>,
}
