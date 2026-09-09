use std::{collections::{HashMap, HashSet}, rc::Rc, sync::Arc};

use crate::{Spanned, ir::{Instr, VReg}};

pub type Reg = usize;

pub struct Writer {
    instructions: Vec<Rc<Spanned<Instr>>>,
    pos: usize,
    free_regs: HashSet<Reg>,
    vreg_map: HashMap<VReg, Reg>,

    output: String,
}

impl Writer {
    pub fn new(instructions: Vec<Spanned<Instr>>) -> Self {
        Self {
            instructions: instructions.into_iter().map(|i| Rc::new(i)).collect(),
            pos: 0,
            free_regs: HashSet::new(),
            vreg_map: HashMap::new(),

            output: String::new(),
        }
    }

    pub fn process(&mut self) -> String {
        while let Some(instruction) = self.current() {
            self.process_instruction(instruction);
            self.advance();
        }

        self.output.clone()
    }

    fn process_instruction(&mut self, instruction: Rc<Spanned<Instr>>) {
        match instruction.element {
            Instr::LoadImm(vreg, n) => {
                let reg = self.get_reg(vreg).expect("no free regs");
                self.write(format!("LD {reg}, {n}"));
            },
            Instr::Print(vreg) => {
                let reg = self.get_reg(vreg).expect("no free regs");
                self.write(format!("OUT {reg}"));
            }
            _ => todo!()
        }
    }

    fn get_reg(&mut self, vreg: VReg) -> Option<usize> {
        if let Some(reg) = self.vreg_map.get(&vreg) {
            return Some(*reg);
        }

        let reg = self.free_regs.iter().next().copied();
        if let Some(r) = reg {
            self.free_regs.remove(&r);
        }
        reg
    }

    fn free_reg(&mut self, reg: Reg) {
        self.free_regs.insert(reg);
    }

    fn write(&mut self, src: impl Into<Arc<str>>) {
        self.output.extend(src.into().chars());
        self.output.extend("\n".chars());
    }

    fn current(&self) -> Option<Rc<Spanned<Instr>>> {
        self.instructions.get(self.pos).cloned()
    }

    fn advance(&mut self) {
        self.pos += 1
    }
}
