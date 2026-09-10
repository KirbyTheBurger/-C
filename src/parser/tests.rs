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
        // lexing 0xFF / 0b1010 is the lexer's job now, so just supply the
        // already-decoded values the lexer would have produced.
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
}
