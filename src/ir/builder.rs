use std::rc::Rc;

use crate::{Spanned, ir::{Instr, Ty, VReg}, lexer::Token, parser::{Expression, Statement}};

pub struct IRBuilder {
    input: Vec<Rc<Spanned<Statement>>>,
    pos: usize,

    next_reg: VReg,
    label_id: usize,
}

impl IRBuilder {
    pub fn new(input: Vec<Spanned<Statement>>) -> Self {
        Self {
            input: input.into_iter().map(|s| Rc::new(s)).collect(),
            pos: 0,

            next_reg: 0,
            label_id: 1,
        }
    }

    pub fn build(&mut self) -> Vec<Instr> {
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
    ) -> Vec<Instr> {
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
    ) -> Vec<Instr> {
        let instructions = match &expression.element {
            Expression::Number(n) => vec![Instr::LoadImm(dest, *n)],
            Expression::Char(n) => vec![Instr::LoadImm(dest, *n as u16)],
            Expression::Paren(e) => return self.eval_expression(e, dest),
            Expression::Binary { left, op, right } => {
                return self.eval_binary(left, right, op, dest);
            },
        };

        instructions
    }

    fn eval_print(&mut self, expression: &Spanned<Expression>) -> Vec<Instr> {
        let ty = infer_type(&expression.element);
        let dest = self.get_reg();
        let mut instructions = self.eval_expression(expression, dest);

        match ty {
            Ty::Int => instructions.push(Instr::Print(dest)),
            Ty::Char => instructions.extend(self.emit_decimal_print(dest)),
        }

        instructions
    }

    fn emit_decimal_print(&mut self, value: VReg) -> Vec<Instr> {
        let mut out = vec![];

        let zero = self.get_reg(); out.push(Instr::LoadImm(zero, 0));
        let one  = self.get_reg(); out.push(Instr::LoadImm(one, 1));
        let ten  = self.get_reg(); out.push(Instr::LoadImm(ten, 10));

        let cur = self.get_reg(); out.push(Instr::Mov(cur, value));
        let count = self.get_reg(); out.push(Instr::LoadImm(count, 0));

        let push_loop = self.new_label("dec_push");
        let push_done = self.new_label("dec_push_done");

        out.push(Instr::Label(push_loop.clone()));
        out.push(Instr::Cmp(cur, zero));
        out.push(Instr::Jeq(push_done.clone()));

        let digit = self.get_reg();
        out.push(Instr::Mod { left: cur, right: ten, dest: digit });
        out.push(Instr::Push(digit));

        let nc = self.get_reg();
        out.push(Instr::Add { left: count, right: one, dest: nc });
        out.push(Instr::Mov(count, nc));

        let nv = self.get_reg();
        out.push(Instr::Div { left: cur, right: ten, dest: nv });
        out.push(Instr::Mov(cur, nv));

        out.push(Instr::Jmp(push_loop));
        out.push(Instr::Label(push_done));

        let skip = self.new_label("dec_zero_skip");
        out.push(Instr::Cmp(count, zero));
        out.push(Instr::Jne(skip.clone()));
        out.push(Instr::Push(zero));
        out.push(Instr::LoadImm(count, 1));
        out.push(Instr::Label(skip));

        let pop_loop = self.new_label("dec_pop");
        let pop_done = self.new_label("dec_pop_done");

        out.push(Instr::Label(pop_loop.clone()));
        out.push(Instr::Cmp(count, zero));
        out.push(Instr::Jeq(pop_done.clone()));

        let d = self.get_reg();
        out.push(Instr::Pop(d));
        out.push(Instr::Print(d));

        let dc = self.get_reg();
        out.push(Instr::Sub { left: count, right: one, dest: dc });
        out.push(Instr::Mov(count, dc));

        out.push(Instr::Jmp(pop_loop));
        out.push(Instr::Label(pop_done));

        out
    }

    fn eval_binary(
        &mut self,
        left: &Spanned<Expression>,
        right: &Spanned<Expression>,
        op: &Token,
        dest: VReg,
    ) -> Vec<Instr> {
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
        }];

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

    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_id);
        self.label_id += 1;
        label
    }
}

fn infer_type(expr: &Expression) -> Ty {
    match expr {
        Expression::Number(_) => Ty::Int,
        Expression::Char(_) => Ty::Char,
        Expression::Paren(e) => infer_type(&e.element),
        Expression::Binary { left, .. } => infer_type(&left.element),
    }
}
