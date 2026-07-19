use crate::parser::ast::{Program, Block, Expression};
use super::error::SemanticError;

mod pass0;

pub fn check_program(prog: &Program) -> Result<(), SemanticError> {
    pass0::check_program(prog)?;
    Ok(())
}

pub fn check_expression(expr: &Expression) -> Result<(), SemanticError> {
    pass0::check_expression(expr)?;
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
    fn test_check_expression() {
        let expr = Expression::Block(Spanning {
            span: Span {
                start: Location { line: 0, col: 0 },
                end: Location { line: 0, col: 0 },
                sl: "",
            },
            inner: Block { stmts: vec![], ret: None },
        });
        check_expression(&expr).unwrap();
    }

}