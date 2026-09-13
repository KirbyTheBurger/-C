use std::rc::Rc;

use crate::{Spanned, error::Error, ir::{Instr, VReg}, lexer::Token, parser::{Expression, Statement}};

pub struct IRBuilder {
    input: Vec<Rc<Spanned<Statement>>>,
    pos: usize,
    next_reg: VReg,
}

impl IRBuilder {
    pub fn new(input: Vec<Spanned<Statement>>) -> Self {
        Self {
            input: input.into_iter().map(|s| Rc::new(s)).collect(),
            pos: 0,
            next_reg: 0,
        }
    }

    pub fn build(&mut self) -> Result<Vec<Spanned<Instr>>, Vec<Error>> {
        let mut instructions = vec![];
        let mut errors = vec![];

        while let Some(stat) = self.current() {
            match self.eval_statement(stat.clone()) {
                Ok(mut v) => instructions.append(&mut v),
                Err(mut e) => errors.append(&mut e),
            }
            self.advance();
        }

        if errors.is_empty() {
            Ok(instructions)
        } else {
            Err(errors)
        }
    }

    fn eval_statement(
        &mut self,
        statement: Rc<Spanned<Statement>>
    ) -> Result<Vec<Spanned<Instr>>, Vec<Error>> {
        let span = statement.span.clone();
        
        match &statement.element {
            Statement::Print(e) => {
                let dest = self.get_reg();
                let mut v = self.eval_expression(e, dest)?;
                v.push(Instr::Print(dest).with_span(span));
                Ok(v)
            },
            Statement::Expression(e) => {
                let dest = self.get_reg();
                self.eval_expression(e, dest)
            },
        }
    }

    fn eval_expression(
        &mut self,
        expression: &Spanned<Expression>,
        dest: VReg
    ) -> Result<Vec<Spanned<Instr>>, Vec<Error>> {
        let instructions = match &expression.element {
            Expression::Number(n) => vec![Instr::LoadImm(dest, *n)],
            Expression::Paren(e) => return self.eval_expression(e, dest),
            Expression::Binary { left, op, right } => {
                return self.eval_binary(left, right, op, dest, expression.span.clone());
            },
        };

        Ok(instructions.into_iter().map(|i| i.with_span(expression.span.clone())).collect())
    }

    fn eval_binary(
        &mut self,
        left: &Spanned<Expression>,
        right: &Spanned<Expression>,
        op: &Token,
        dest: VReg,
        span: std::ops::Range<usize>,
    ) -> Result<Vec<Spanned<Instr>>, Vec<Error>> {
        let left_reg = self.get_reg();
        let right_reg = self.get_reg();

        let left_ir = self.eval_expression(left, left_reg)?;
        let right_ir = self.eval_expression(right, right_reg)?;

        let op_ir = vec![match op {
            Token::Add => Instr::Add { left: left_reg, right: right_reg, dest },
            Token::Sub => Instr::Sub { left: left_reg, right: right_reg, dest },
            Token::Mul => Instr::Mul { left: left_reg, right: right_reg, dest },
            Token::Div => Instr::Div { left: left_reg, right: right_reg, dest },
            _ => panic!("unexpected token"),
        }.with_span(span)];

        Ok(left_ir.into_iter().chain(right_ir).chain(op_ir).collect())
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<Rc<Spanned<Statement>>> {
        self.input.get(self.pos).cloned()
    }

    fn get_reg(&mut self) -> VReg {
        self.next_reg += 1;
        self.next_reg - 1
    }
}
