#[cfg(test)]
mod tests {
    use crate::lexer::Token;
    use crate::parser::{Parser, Statement, Expression};
    use crate::Spanned;

    fn tok(token: Token) -> Spanned<Token> {
        Spanned {
            element: token,
            span: 0..0,
        }
    }

    fn run(tokens: Vec<Token>) -> Vec<Statement> {
        let tokens = tokens.into_iter().map(tok).collect();
        let mut parser = Parser::new(tokens);
        parser.parse().unwrap().into_iter().map(|s| s.element).collect()
    }

    #[test]
    fn parses_number_literal_as_expression_statement() {
        let result = run(vec![Token::Number(5)]);
        match &result[0] {
            Statement::Expression(e) => assert_eq!(e.element, Expression::Number(5)),
            _ => panic!("expected Statement::Expression, got {:?}", result[0]),
        }
    }

    #[test]
    fn parses_print_with_number() {
        let result = run(vec![Token::Print, Token::Number(5)]);
        match &result[0] {
            Statement::Print(e) => assert_eq!(e.element, Expression::Number(5)),
            _ => panic!("expected Statement::Print, got {:?}", result[0]),
        }
    }

    #[test]
    fn parses_multiple_statements() {
        let result = run(vec![Token::Number(5), Token::Print, Token::Number(10)]);
        assert_eq!(result.len(), 2);

        match &result[0] {
            Statement::Expression(e) => assert_eq!(e.element, Expression::Number(5)),
            _ => panic!("expected Statement::Expression, got {:?}", result[0]),
        }

        match &result[1] {
            Statement::Print(e) => assert_eq!(e.element, Expression::Number(10)),
            _ => panic!("expected Statement::Print, got {:?}", result[1]),
        }
    }

    #[test]
    fn parses_chained_expression_statements() {
        let result = run(vec![Token::Number(1), Token::Number(2), Token::Number(3)]);
        assert_eq!(result.len(), 3);

        for (i, expected) in [1, 2, 3].iter().enumerate() {
            match &result[i] {
                Statement::Expression(e) => assert_eq!(e.element, Expression::Number(*expected)),
                _ => panic!("expected Statement::Expression, got {:?}", result[i]),
            }
        }
    }

    #[test]
    fn parses_chained_print_statements() {
        let result = run(vec![
            Token::Print, Token::Number(1),
            Token::Print, Token::Number(2),
        ]);
        assert_eq!(result.len(), 2);

        match &result[0] {
            Statement::Print(e) => assert_eq!(e.element, Expression::Number(1)),
            _ => panic!("expected Statement::Print, got {:?}", result[0]),
        }

        match &result[1] {
            Statement::Print(e) => assert_eq!(e.element, Expression::Number(2)),
            _ => panic!("expected Statement::Print, got {:?}", result[1]),
        }
    }

    #[test]
    fn parses_hex_and_binary_numbers() {
        let result = run(vec![
            Token::Print, Token::Number(255),
            Token::Print, Token::Number(10),
        ]);
        assert_eq!(result.len(), 2);

        match &result[0] {
            Statement::Print(e) => assert_eq!(e.element, Expression::Number(255)),
            _ => panic!("expected Statement::Print, got {:?}", result[0]),
        }

        match &result[1] {
            Statement::Print(e) => assert_eq!(e.element, Expression::Number(10)),
            _ => panic!("expected Statement::Print, got {:?}", result[1]),
        }
    }

    #[test]
    #[should_panic]
    fn print_without_expression_errors() {
        run(vec![Token::Print]);
    }

    fn num(n: u16) -> Expression {
        Expression::Number(n)
    }

    fn spanned(e: Expression) -> Spanned<Expression> {
        Spanned { element: e, span: 0..0 }
    }

    fn bin(left: Expression, op: Token, right: Expression) -> Expression {
        Expression::Binary {
            left: Box::new(spanned(left)),
            op,
            right: Box::new(spanned(right)),
        }
    }

    fn paren(e: Expression) -> Expression {
        Expression::Paren(Box::new(spanned(e)))
    }

    #[test]
    fn parses_addition() {
        let result = run(vec![Token::Number(1), Token::Add, Token::Number(2)]);
        match &result[0] {
            Statement::Expression(e) => assert_eq!(e.element, bin(num(1), Token::Add, num(2))),
            _ => panic!("expected Statement::Expression, got {:?}", result[0]),
        }
    }

    #[test]
    fn parses_subtraction() {
        let result = run(vec![Token::Number(5), Token::Sub, Token::Number(3)]);
        match &result[0] {
            Statement::Expression(e) => assert_eq!(e.element, bin(num(5), Token::Sub, num(3))),
            _ => panic!("expected Statement::Expression, got {:?}", result[0]),
        }
    }

    #[test]
    fn parses_multiplication() {
        let result = run(vec![Token::Number(4), Token::Mul, Token::Number(6)]);
        match &result[0] {
            Statement::Expression(e) => assert_eq!(e.element, bin(num(4), Token::Mul, num(6))),
            _ => panic!("expected Statement::Expression, got {:?}", result[0]),
        }
    }

    #[test]
    fn parses_division() {
        let result = run(vec![Token::Number(8), Token::Div, Token::Number(2)]);
        match &result[0] {
            Statement::Expression(e) => assert_eq!(e.element, bin(num(8), Token::Div, num(2))),
            _ => panic!("expected Statement::Expression, got {:?}", result[0]),
        }
    }

    #[test]
    fn mul_binds_tighter_than_add() {
        // 2 + 3 * 4  =>  2 + (3 * 4)
        let result = run(vec![
            Token::Number(2), Token::Add,
            Token::Number(3), Token::Mul, Token::Number(4),
        ]);
        match &result[0] {
            Statement::Expression(e) => {
                assert_eq!(e.element, bin(num(2), Token::Add, bin(num(3), Token::Mul, num(4))));
            }
            _ => panic!("expected Statement::Expression, got {:?}", result[0]),
        }
    }

    #[test]
    fn div_binds_tighter_than_sub() {
        // 10 - 8 / 2  =>  10 - (8 / 2)
        let result = run(vec![
            Token::Number(10), Token::Sub,
            Token::Number(8), Token::Div, Token::Number(2),
        ]);
        match &result[0] {
            Statement::Expression(e) => {
                assert_eq!(e.element, bin(num(10), Token::Sub, bin(num(8), Token::Div, num(2))));
            }
            _ => panic!("expected Statement::Expression, got {:?}", result[0]),
        }
    }

    #[test]
    fn addition_is_left_associative() {
        // 1 + 2 + 3  =>  (1 + 2) + 3
        let result = run(vec![
            Token::Number(1), Token::Add,
            Token::Number(2), Token::Add,
            Token::Number(3),
        ]);
        match &result[0] {
            Statement::Expression(e) => {
                assert_eq!(e.element, bin(bin(num(1), Token::Add, num(2)), Token::Add, num(3)));
            }
            _ => panic!("expected Statement::Expression, got {:?}", result[0]),
        }
    }

    #[test]
    fn division_is_left_associative() {
        // 8 / 4 / 2  =>  (8 / 4) / 2
        let result = run(vec![
            Token::Number(8), Token::Div,
            Token::Number(4), Token::Div,
            Token::Number(2),
        ]);
        match &result[0] {
            Statement::Expression(e) => {
                assert_eq!(e.element, bin(bin(num(8), Token::Div, num(4)), Token::Div, num(2)));
            }
            _ => panic!("expected Statement::Expression, got {:?}", result[0]),
        }
    }

    #[test]
    fn parens_override_precedence() {
        // (2 + 3) * 4
        let result = run(vec![
            Token::LParen, Token::Number(2), Token::Add, Token::Number(3), Token::RParen,
            Token::Mul, Token::Number(4),
        ]);
        match &result[0] {
            Statement::Expression(e) => {
                assert_eq!(e.element, bin(paren(bin(num(2), Token::Add, num(3))), Token::Mul, num(4)));
            }
            _ => panic!("expected Statement::Expression, got {:?}", result[0]),
        }
    }

    #[test]
    fn nested_parens() {
        // ((1 + 2))
        let result = run(vec![
            Token::LParen, Token::LParen,
            Token::Number(1), Token::Add, Token::Number(2),
            Token::RParen, Token::RParen,
        ]);
        match &result[0] {
            Statement::Expression(e) => {
                assert_eq!(e.element, paren(paren(bin(num(1), Token::Add, num(2)))));
            }
            _ => panic!("expected Statement::Expression, got {:?}", result[0]),
        }
    }

    #[test]
    fn print_with_arithmetic() {
        let result = run(vec![Token::Print, Token::Number(2), Token::Add, Token::Number(3)]);
        match &result[0] {
            Statement::Print(e) => assert_eq!(e.element, bin(num(2), Token::Add, num(3))),
            _ => panic!("expected Statement::Print, got {:?}", result[0]),
        }
    }
}
