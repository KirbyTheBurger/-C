use std::rc::Rc;

use crate::{Spanned, error::Error, ir::{Instr, VReg}, parser::{Expression, Statement}};

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

    fn eval_statement(&mut self, statement: Rc<Spanned<Statement>>) -> Result<Vec<Spanned<Instr>>, Vec<Error>> {
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

    fn eval_expression(&mut self, expression: &Spanned<Expression>, dest: VReg) -> Result<Vec<Spanned<Instr>>, Vec<Error>> {
        let span = expression.span.clone();

        match expression.element {
            Expression::Number(n) => Ok(vec![Instr::LoadImm(dest, n).with_span(span)]),
        }
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
