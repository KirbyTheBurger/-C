#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::{ir::{Instr, builder::IRBuilder}, parser::{Statement, Expression}, Spanned};

    fn spanned<T: Debug + PartialEq>(e: T) -> Spanned<T> {
        Spanned { element: e, span: 0..0 }
    }

    fn num_expr(n: u16) -> Spanned<Expression> {
        spanned(Expression::Number(n))
    }

    fn print_stmt(n: u16) -> Spanned<Statement> {
        spanned(Statement::Print(Box::new(num_expr(n))))
    }

    fn expr_stmt(n: u16) -> Spanned<Statement> {
        spanned(Statement::Expression(Box::new(num_expr(n))))
    }

    fn run(statements: Vec<Spanned<Statement>>) -> Vec<Instr> {
        IRBuilder::new(statements).build().unwrap()
            .into_iter().map(|i| i.element).collect()
    }

    #[test]
    fn single_print() {
        let ir = run(vec![print_stmt(5)]);
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(0, 5),
                Instr::Print(0),
            ]
        );
    }

    #[test]
    fn multiple_prints() {
        let ir = run(vec![print_stmt(1), print_stmt(2), print_stmt(3)]);
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(0, 1),
                Instr::Print(0),
                Instr::LoadImm(1, 2),
                Instr::Print(1),
                Instr::LoadImm(2, 3),
                Instr::Print(2),
            ]
        );
    }

    #[test]
    fn standalone_expression_single_number() {
        let ir = run(vec![expr_stmt(5)]);
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(0, 5),
            ]
        );
    }

    #[test]
    fn multiple_standalone_expressions() {
        let ir = run(vec![expr_stmt(5), expr_stmt(7), expr_stmt(9)]);
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(0, 5),
                Instr::LoadImm(1, 7),
                Instr::LoadImm(2, 9),
            ]
        );
    }

    #[test]
    fn prints_and_standalone_expressions_mixed() {
        let ir = run(vec![
            print_stmt(1),
            expr_stmt(5),
            print_stmt(2),
            expr_stmt(7),
        ]);
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(0, 1),
                Instr::Print(0),
                Instr::LoadImm(1, 5),
                Instr::LoadImm(2, 2),
                Instr::Print(2),
                Instr::LoadImm(3, 7),
            ]
        );
    }

    #[test]
    fn empty_program() {
        let ir = run(vec![]);
        assert_eq!(ir, vec![]);
    }
}
