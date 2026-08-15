use std::rc::Rc;

use crate::{Spanned, error::Error, lexer::Token};

#[derive(Debug, PartialEq)]
pub enum Statement {

}

#[derive(Debug, PartialEq)]
pub enum Expression {

}

pub struct Parser {
    pos: usize,
    tokens: Vec<Rc<Spanned<Token>>>,
}

impl Parser {
    pub fn new(tokens: Vec<Spanned<Token>>) -> Parser {
        Parser {
            pos: 0,
            tokens: tokens.into_iter().map(|t| Rc::new(t)).collect(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Spanned<Statement>>, Vec<Error>> {
        let mut statements = vec![];
        let mut errors = vec![];

        while let Some(t) = self.current() {
            match self.parse_statement(t) {
                Ok(s) => statements.push(s),
                Err(e) => errors.push(e),
            }
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    fn parse_statement(&mut self, current: Rc<Spanned<Token>>) -> Result<Spanned<Statement>, Error> {
        match current {
            _ => todo!(),
        }
    }

    fn parse_expression(&mut self, current: Rc<Spanned<Token>>) -> Result<Spanned<Expression>, Error> {
        match current {
            _ => todo!(),
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<Rc<Spanned<Token>>> {
        self.tokens.get(self.pos).cloned()
    }

    fn peek(&self) -> Option<Rc<Spanned<Token>>> {
        self.tokens.get(self.pos + 1).cloned()
    }

    /// This function doesn't advance and assumes the current token isn't `None`
    fn expect(&mut self, expected: Token, context: &str) -> Result<Rc<Spanned<Token>>, Error> {
        match self.peek() {
            Some(t) => {
                if t.element == expected {
                    Ok(t)
                } else {
                    Err(Error::new(format!(
                        "Expected `{:?}` {}, got `{:?}`",
                        expected, context, t.element
                    ), t.span.clone()))
                }
            },
            None => {
                let current = self.current().unwrap();
                Err(Error::new(format!(
                    "Expected {:?} {}, got EOF",
                    expected, context
                ), current.span.clone()))
            }
        }
    }
}