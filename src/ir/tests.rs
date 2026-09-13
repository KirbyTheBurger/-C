#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::{ir::{Instr, builder::IRBuilder}, parser::{Statement, Expression}, lexer::Token, Spanned};

    fn spanned<T: PartialEq + Debug>(e: T) -> Spanned<T> {
        Spanned { element: e, span: 0..0 }
    }

    fn num_expr(n: u16) -> Spanned<Expression> {
        spanned(Expression::Number(n))
    }

    fn bin_expr(left: Spanned<Expression>, op: Token, right: Spanned<Expression>) -> Spanned<Expression> {
        spanned(Expression::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        })
    }

    fn print_stmt(e: Spanned<Expression>) -> Spanned<Statement> {
        spanned(Statement::Print(Box::new(e)))
    }

    fn expr_stmt(e: Spanned<Expression>) -> Spanned<Statement> {
        spanned(Statement::Expression(Box::new(e)))
    }

    fn run(statements: Vec<Spanned<Statement>>) -> Vec<Instr> {
        IRBuilder::new(statements).build().unwrap()
            .into_iter().map(|i| i.element).collect()
    }

    #[test]
    fn single_add() {
        let ir = run(vec![
            expr_stmt(bin_expr(num_expr(1), Token::Add, num_expr(2))),
        ]);
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(1, 1),
                Instr::LoadImm(2, 2),
                Instr::Add { left: 1, right: 2, dest: 0 },
            ]
        );
    }

    #[test]
    fn single_sub() {
        let ir = run(vec![
            expr_stmt(bin_expr(num_expr(5), Token::Sub, num_expr(3))),
        ]);
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(1, 5),
                Instr::LoadImm(2, 3),
                Instr::Sub { left: 1, right: 2, dest: 0 },
            ]
        );
    }

    #[test]
    fn single_mul() {
        let ir = run(vec![
            expr_stmt(bin_expr(num_expr(4), Token::Mul, num_expr(6))),
        ]);
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(1, 4),
                Instr::LoadImm(2, 6),
                Instr::Mul { left: 1, right: 2, dest: 0 },
            ]
        );
    }

    #[test]
    fn single_div() {
        let ir = run(vec![
            expr_stmt(bin_expr(num_expr(8), Token::Div, num_expr(2))),
        ]);
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(1, 8),
                Instr::LoadImm(2, 2),
                Instr::Div { left: 1, right: 2, dest: 0 },
            ]
        );
    }

    #[test]
    fn print_arithmetic_result() {
        let ir = run(vec![
            print_stmt(bin_expr(num_expr(1), Token::Add, num_expr(2))),
        ]);
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(1, 1),
                Instr::LoadImm(2, 2),
                Instr::Add { left: 1, right: 2, dest: 0 },
                Instr::Print(0),
            ]
        );
    }

    #[test]
    fn nested_binary_expression() {
        // (1 + 2) * 3
        let ir = run(vec![
            expr_stmt(bin_expr(
                bin_expr(num_expr(1), Token::Add, num_expr(2)),
                Token::Mul,
                num_expr(3),
            )),
        ]);
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(3, 1),
                Instr::LoadImm(4, 2),
                Instr::Add { left: 3, right: 4, dest: 1 },
                Instr::LoadImm(2, 3),
                Instr::Mul { left: 1, right: 2, dest: 0 },
            ]
        );
    }
}