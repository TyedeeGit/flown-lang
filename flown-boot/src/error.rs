use thiserror::Error;

use crate::lexer::error::LexError;
use crate::parser::error::ParseError;
use crate::semantic::error::SemanticError;

#[derive(Error, Debug, Clone)]
pub enum CompileError {
    #[error("Lexical error: {0}")]
    Lex(#[from] LexError),
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),
    #[error("Semantic error: {0}")]
    Semantic(#[from] SemanticError),
}