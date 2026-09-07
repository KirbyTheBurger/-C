use std::rc::Rc;

use crate::{Spanned, error::Error, ir::{Instr, VReg}, parser::{Expression, Statement}};

pub struct IRBuilder {
    input: Vec<Rc<Spanned<Statement>>>,
    pos: usize,
    next_reg: usize,
}

impl IRBuilder {
    pub fn new(input: Vec<Spanned<Statement>>) -> Self {
        Self {
            input: input.into_iter().map(|s| Rc::new(s)).collect(),
            pos: 0,
            next_reg: 0,
        }
    }

    pub fn build(&mut self) -> Result<Vec<Instr>, Vec<Error>> {
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

    fn eval_statement(&mut self, statement: Rc<Spanned<Statement>>) -> Result<Vec<Instr>, Vec<Error>> {
        todo!()
    }

    fn eval_expression(&mut self, expression: Spanned<Expression>, dest: VReg) -> Result<Vec<Instr>, Vec<Error>> {
        todo!()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn current(&self) -> Option<Rc<Spanned<Statement>>> {
        self.input.get(self.pos).cloned()
    }
}
