use std::rc::Rc;

use crate::{Spanned, ir::{Instr, VReg}, lexer::Token, parser::{Expression, Statement}};

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

    pub fn build(&mut self) -> Vec<Spanned<Instr>> {
        let mut instructions = vec![];

        while let Some(stat) = self.current() {
            let mut v = self.eval_statement(stat.clone());
            instructions.append(&mut v);
            self.advance();
        }

        instructions
    }

    fn eval_statement(
        &mut self,
        statement: Rc<Spanned<Statement>>
    ) -> Vec<Spanned<Instr>> {
        match &statement.element {
            Statement::Print(e) => self.eval_print(e),
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
    ) -> Vec<Spanned<Instr>> {
        let instructions = match &expression.element {
            Expression::Number(n) => vec![Instr::LoadImm(dest, *n)],
            Expression::Char(n) => vec![Instr::LoadImm(dest, *n as u16)],
            Expression::Paren(e) => return self.eval_expression(e, dest),
            Expression::Binary { left, op, right } => {
                return self.eval_binary(left, right, op, dest, expression.span.clone());
            },
        };

        instructions.into_iter().map(|i| i.with_span(expression.span.clone())).collect()
    }

    fn eval_print(&mut self, expression: &Spanned<Expression>) -> Vec<Spanned<Instr>> {
        let dest = self.get_reg();
        let mut instructions = vec![];

        if let Expression::Number(n) = expression.element {
            for c in n.to_string().chars() {
                instructions.append(&mut vec![
                    Instr::LoadImm(dest, c as u16).with_span(expression.span.clone()),
                    Instr::Print(dest).with_span(expression.span.clone()),
                ]);
            }
            return instructions;
        }
        
        instructions.append(&mut self.eval_expression(expression, dest));
        instructions.push(Instr::Print(dest).with_span(expression.span.clone()));
        instructions
    }

    fn eval_binary(
        &mut self,
        left: &Spanned<Expression>,
        right: &Spanned<Expression>,
        op: &Token,
        dest: VReg,
        span: std::ops::Range<usize>,
    ) -> Vec<Spanned<Instr>> {
        let left_reg = self.get_reg();
        let right_reg = self.get_reg();

        let left_ir = self.eval_expression(left, left_reg);
        let right_ir = self.eval_expression(right, right_reg);

        let op_ir = vec![match op {
            Token::Add => Instr::Add { left: left_reg, right: right_reg, dest },
            Token::Sub => Instr::Sub { left: left_reg, right: right_reg, dest },
            Token::Mul => Instr::Mul { left: left_reg, right: right_reg, dest },
            Token::Div => Instr::Div { left: left_reg, right: right_reg, dest },
            _ => panic!("unexpected token"),
        }.with_span(span)];

        left_ir.into_iter().chain(right_ir).chain(op_ir).collect()
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
