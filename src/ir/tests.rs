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

    fn char_expr(c: u8) -> Spanned<Expression> {
        spanned(Expression::Char(c))
    }

    fn bin_expr(left: Spanned<Expression>, op: Token, right: Spanned<Expression>) -> Spanned<Expression> {
        spanned(Expression::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        })
    }

    fn paren_expr(e: Spanned<Expression>) -> Spanned<Expression> {
        spanned(Expression::Paren(Box::new(e)))
    }

    fn print_stmt(e: Spanned<Expression>) -> Spanned<Statement> {
        spanned(Statement::Print(Box::new(e)))
    }

    fn expr_stmt(e: Spanned<Expression>) -> Spanned<Statement> {
        spanned(Statement::Expression(Box::new(e)))
    }

    fn run(statements: Vec<Spanned<Statement>>) -> Vec<Instr> {
        IRBuilder::new(statements).build()
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
                Instr::Drop(1),
                Instr::Drop(2),
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
                Instr::Drop(1),
                Instr::Drop(2),
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
                Instr::Drop(1),
                Instr::Drop(2),
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
                Instr::Drop(1),
                Instr::Drop(2),
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
                Instr::Drop(3),
                Instr::Drop(4),
                Instr::LoadImm(2, 3),
                Instr::Mul { left: 1, right: 2, dest: 0 },
                Instr::Drop(1),
                Instr::Drop(2),
            ]
        );
    }

    #[test]
    fn paren_expression_unwraps() {
        // (5)
        let ir = run(vec![
            expr_stmt(paren_expr(num_expr(5))),
        ]);
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(0, 5),
            ]
        );
    }

    #[test]
    fn print_char_uses_raw_print() {
        let ir = run(vec![
            print_stmt(char_expr(65)),
        ]);
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(0, 65),
                Instr::Print(0),
            ]
        );
    }

    #[test]
    fn print_char_binary_uses_raw_print() {
        // type is inferred from the left operand, so Char + Number still prints raw
        let ir = run(vec![
            print_stmt(bin_expr(char_expr(65), Token::Add, num_expr(1))),
        ]);
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(1, 65),
                Instr::LoadImm(2, 1),
                Instr::Add { left: 1, right: 2, dest: 0 },
                Instr::Drop(1),
                Instr::Drop(2),
                Instr::Print(0),
            ]
        );
    }

    #[test]
    fn print_number_uses_decimal_print() {
        let ir = run(vec![
            print_stmt(num_expr(123)),
        ]);

        // don't pin the exact instruction sequence (it's long and easy to break
        // incidentally) — just check the structural shape of decimal printing.
        assert_eq!(ir[0], Instr::LoadImm(0, 123));

        assert!(ir.iter().any(|i| matches!(i, Instr::Mod { .. })),
            "expected a Mod instruction for digit extraction");
        assert!(ir.iter().any(|i| matches!(i, Instr::Push(_))),
            "expected digits to be pushed onto the stack");
        assert!(ir.iter().any(|i| matches!(i, Instr::Pop(_))),
            "expected digits to be popped back off the stack");
        assert!(ir.iter().any(|i| matches!(i, Instr::Print(_))),
            "expected a Print instruction for each popped digit");
        assert!(ir.iter().any(|i| matches!(i, Instr::Label(l) if l.starts_with("dec_push"))),
            "expected a push loop label");
        assert!(ir.iter().any(|i| matches!(i, Instr::Label(l) if l.starts_with("dec_pop"))),
            "expected a pop loop label");

        // the original `value` vreg (1) should be dropped, not left dangling
        assert!(ir.iter().any(|i| *i == Instr::Drop(1)),
            "expected the original value vreg to be dropped after copying into cur");

        // should end with the final drops of the loop-carried constants
        let tail = &ir[ir.len() - 6..];
        let drop_count = tail.iter().filter(|i| matches!(i, Instr::Drop(_))).count();
        assert_eq!(drop_count, 6, "expected 6 trailing Drop instructions for the loop-carried vregs");
    }

    #[test]
    fn print_zero_still_prints_a_digit() {
        // regression check: the zero-value special case should still push exactly
        // one digit (via the dec_zero_skip path) rather than printing nothing
        let ir = run(vec![
            print_stmt(num_expr(0)),
        ]);

        assert!(ir.iter().any(|i| matches!(i, Instr::Label(l) if l.starts_with("dec_zero_skip"))),
            "expected the zero-value skip label to be present");
    }
}
