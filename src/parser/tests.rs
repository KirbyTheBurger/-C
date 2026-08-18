#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;
    use crate::parser::{Parser, Statement, Expression};

    fn run(src: &str) -> Vec<Statement> {
        let tokens = tokenize(src).unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse().unwrap().into_iter().map(|s| s.element).collect()
    }

    #[test]
    fn parses_number_literal_as_expression_statement() {
        let result = run("5");
        match &result[0] {
            Statement::Expression(e) => assert_eq!(e.element, Expression::Number(5)),
            _ => panic!("expected Statement::Expression, got {:?}", result[0]),
        }
    }

    #[test]
    fn parses_print_with_number() {
        let result = run("print 5");
        match &result[0] {
            Statement::Print(e) => assert_eq!(e.element, Expression::Number(5)),
            _ => panic!("expected Statement::Print, got {:?}", result[0]),
        }
    }

    #[test]
    fn parses_multiple_statements() {
        let result = run("5 print 10");
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
        let result = run("1 2 3");
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
        let result = run("print 1 print 2");
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
        let result = run("print 0xFF print 0b1010");
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
        run("print");
    }
}