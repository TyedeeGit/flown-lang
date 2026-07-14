use crate::parser::ast::{Program, Block, Expression};
use super::error::SemanticError;

pub fn check_program(prog: &Program) -> Result<(), SemanticError> {
    Ok(())
}

pub fn check_block(block: &Block) -> Result<(), SemanticError> {
    Ok(())
}

pub fn check_expression(expr: &Expression) -> Result<(), SemanticError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lexer::info::{Location, Span, Spanning};
    use super::*;

    #[test]
    fn test_check_program() {
        let prog = Program { stmts: vec![] };
        check_program(&prog).unwrap();
    }

    #[test]
    fn test_check_block() {
        let block = Block { stmts: vec![], ret: None };
        check_block(&block).unwrap();
    }

    #[test]
    fn test_check_expression() {
        let expr = Expression::Block(Spanning {
            span: Span {
                start: Location { line: 0, column: 0 },
                end: Location { line: 0, column: 0 }
            },
            inner: Block { stmts: vec![], ret: None },
        });
        check_expression(&expr).unwrap();
    }

}