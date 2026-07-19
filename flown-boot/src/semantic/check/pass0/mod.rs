use crate::parser::ast::{Program, Block, Expression};
use super::super::error::SemanticError;

pub fn check_program(prog: &Program) -> Result<(), SemanticError> {
    Ok(())
}

pub fn check_expression(expr: &Expression) -> Result<(), SemanticError> {
    Ok(())
}