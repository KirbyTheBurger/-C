use std::rc::Rc;

use crate::{Spanned, ir::Instr};

pub struct Writer {
    instructions: Vec<Rc<Spanned<Instr>>>,
    pos: usize,

    output: String,
}

impl Writer {
    pub fn new(instructions: Vec<Spanned<Instr>>) -> Self {
        Self {
            instructions: instructions.into_iter().map(|i| Rc::new(i)).collect(),
            pos: 0,

            output: String::new(),
        }
    }

    pub fn write(&mut self) -> String {
        while let Some(instruction) = self.current() {
            self.process_instruction(instruction);
            self.advance();
        }

        self.output.clone()
    }

    fn process_instruction(&mut self, instruction: Rc<Spanned<Instr>>) {
        match instruction {
            _ => todo!()
        }
    }

    fn write_str(&mut self, src: impl ToString) {
        self.output.extend(src.to_string().chars());
    }

    fn newline(&mut self) {
        self.output.extend("\n".chars());
    }

    fn current(&self) -> Option<Rc<Spanned<Instr>>> {
        self.instructions.get(self.pos).cloned()
    }

    fn advance(&mut self) {
        self.pos += 1
    }
}
