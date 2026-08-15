use std::rc::Rc;

use crate::{Spanned, error::Error, lexer::Token};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Print(Box<Spanned<Expression>>),
    Expression(Box<Spanned<Expression>>),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(u16),
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
            self.advance();
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    fn parse_statement(&mut self, current: Rc<Spanned<Token>>) -> Result<Spanned<Statement>, Error> {
        let span_start = current.span.start;

        let statement = match current.element {
            Token::Print => Statement::Print(Box::new(self.parse_expression(current)?)),
            _ => Statement::Expression(Box::new(self.parse_expression(current)?)),
        };

        let span_end = self.current().unwrap().span.end;

        Ok(Spanned {
            element: statement,
            span: span_start..span_end,
        })
    }

    fn parse_expression(&mut self, current: Rc<Spanned<Token>>) -> Result<Spanned<Expression>, Error> {
        let span_start = current.span.start;

        let expression = match current.element {
            Token::Number(n) => Expression::Number(n),
            _ => todo!(),
        };

        let span_end = self.current().unwrap().span.end;

        Ok(Spanned {
            element: expression,
            span: span_start..span_end,
        })
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