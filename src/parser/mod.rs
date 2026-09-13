use std::rc::Rc;

use crate::{Spanned, error::Error, lexer::Token};

mod tests;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Print(Box<Spanned<Expression>>),
    Expression(Box<Spanned<Expression>>),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(u16),

    Binary {
        left: Box<Spanned<Expression>>,
        op: Token,
        right: Box<Spanned<Expression>>,
    },
    Paren(Box<Spanned<Expression>>),
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
            let pos_before = self.pos;
            match self.parse_statement(t) {
                Ok(s) => statements.push(s),
                Err(e) => {
                    errors.push(e);
                    if self.pos == pos_before {
                        self.advance();
                    }
                },
            }
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
            Token::Print => self.read_print()?,
            _ => Statement::Expression(Box::new(self.parse_expression(current)?)),
        };

        let span_end = self.previous().span.end;

        Ok(Spanned {
            element: statement,
            span: span_start..span_end,
        })
    }

    fn parse_expression(&mut self, current: Rc<Spanned<Token>>) -> Result<Spanned<Expression>, Error> {
        self.parse_expression_bp(current, 0)
    }

    fn parse_expression_bp(&mut self, current: Rc<Spanned<Token>>, min_bp: u8) -> Result<Spanned<Expression>, Error> {
        let span_start = current.span.start;
        let mut left = self.parse_primary(current)?;

        loop {
            let Some(op_tok) = self.current() else { break; };

            let Some((left_bp, right_bp)) = binding_power(&op_tok.element) else { break; };
            if left_bp < min_bp {
                break;
            }

            let rhs_start = self.expect_some("after operator")?;
            self.advance();
            let right = self.parse_expression_bp(rhs_start, right_bp)?;

            let span_end = right.span.end;
            left = Spanned {
                element: Expression::Binary {
                    left: Box::new(left),
                    op: op_tok.element.clone(),
                    right: Box::new(right),
                },
                span: span_start..span_end,
            }
        }

        Ok(left)
    }

    fn parse_primary(&mut self, current: Rc<Spanned<Token>>) -> Result<Spanned<Expression>, Error> {
        let span_start = current.span.start;

        let expression = match current.element {
            Token::Number(n) => Expression::Number(n),
            Token::LParen => {
                let inner_start = self.expect_some("after `(`")?;
                self.advance();
                let inner = self.parse_expression(inner_start)?;
                self.expect_current(Token::RParen, "after expression")?;
                Expression::Paren(Box::new(inner))
            },
            _ => panic!("unexpected token"),
        };

        let span_end = self.current().unwrap().span.end;
        self.advance();

        Ok(Spanned {
            element: expression,
            span: span_start..span_end,
        })
    }

    fn read_print(&mut self) -> Result<Statement, Error> {
        let current = self.expect_some("after `print`")?;
        self.advance();
        Ok(Statement::Print(Box::new(self.parse_expression(current)?)))
    }

    /// Assumes previous token isn't `None`
    fn previous(&mut self) -> Rc<Spanned<Token>> {
        self.tokens[self.pos - 1].clone()
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
    fn expect_some(&self, context: &str) -> Result<Rc<Spanned<Token>>, Error> {
        match self.peek() {
            Some(t) => Ok(t),
            None => {
                let current = self.current().unwrap();
                Err(Error::new(format!(
                    "Expected something `{}`, got EOF",
                    context
                ), current.span.end..current.span.end))
            }
        }
    }

    #[allow(unused)]
    /// This function doesn't advance and assumes the current token isn't `None`
    fn expect(&self, expected: Token, context: &str) -> Result<Rc<Spanned<Token>>, Error> {
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

    fn expect_current(&mut self, expected: Token, context: &str) -> Result<Rc<Spanned<Token>>, Error> {
        let current = self.current();
        match current {
            Some(t) if t.element == expected => Ok(t),
            Some(t) => Err(Error::new(
                format!("Expected `{:?}` {}, got `{:?}`", expected, context, t.element),
                t.span.clone(),
            )),
            None => Err(Error::new(
                format!("Expected `{:?}` {}, got EOF", expected, context),
                self.previous().span.end..self.previous().span.end,
            )),
        }
    }
}

fn binding_power(op: &Token) -> Option<(u8, u8)> {
    match op {
        Token::Add | Token::Sub => Some((1, 2)),
        Token::Mul | Token::Div => Some((3, 4)),
        _ => None,
    }
}
