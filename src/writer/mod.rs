use std::{collections::{BTreeSet, HashMap}, rc::Rc};

use crate::ir::{VReg, Instr};

mod tests;

const SPILL_BASE_REG: Reg = 7;
const SPILL_MEM_BASE: u16 = 0x4000;

pub type Reg = usize;

pub struct Writer {
    instructions: Vec<Rc<Instr>>,
    pos: usize,

    free_regs: BTreeSet<Reg>,
    vreg_map: HashMap<VReg, Reg>,
    spill_slots: HashMap<VReg, u16>,
    next_slot: u16,
    access_order: Vec<Reg>,

    output: String,
}

impl Writer {
    pub fn new(instructions: Vec<Instr>) -> Self {
        let mut w = Self {
            instructions: instructions.into_iter().map(|i| Rc::new(i)).collect(),
            pos: 0,

            free_regs: (0..7_usize).into_iter().collect(),
            vreg_map: HashMap::new(),
            spill_slots: HashMap::new(),
            next_slot: 0,
            access_order: vec![],

            output: String::new(),
        };

        w.write(format!("LD r{SPILL_BASE_REG}, {SPILL_MEM_BASE}"));
        w
    }

    pub fn process(&mut self) -> String {
        while let Some(instruction) = self.current() {
            self.process_instruction(instruction);
            self.advance();
        }

        self.output.clone()
    }

    fn process_instruction(&mut self, instruction: Rc<Instr>) {
        match &*instruction {
            Instr::LoadImm(dest, n) => {
                let dest = self.get_reg(dest);
                self.write(format!("LD r{dest}, {n}"));
            },
            Instr::Print(reg) => {
                let reg = self.get_reg(reg);
                self.write(format!("OUT r{reg}"));
                self.free_reg(reg);
            },
            Instr::Add { left, right, dest } => {
                self.write_binop("ADD", left, right, dest)
            },
            Instr::Sub { left, right, dest } => {
                self.write_binop("SUB", left, right, dest)
            },
            Instr::Mul { left, right, dest } => {
                self.write_binop("MUL", left, right, dest)
            },
            Instr::Div { left, right, dest } => {
                self.write_binop("DIV", left, right, dest)
            },
            Instr::Mod { left, right, dest } => {
                self.write_binop("MOD", left, right, dest);
            },
            Instr::Cmp(left, right) => {
                let left = self.get_reg(left);
                let right = self.get_reg(right);
                self.write(format!("CMP r{left}, r{right}"));
            },
            Instr::Drop(reg) => {
                let reg = self.get_reg(reg);
                self.free_reg(reg);
            },
            Instr::Label(s) => {
                self.write(format!("\n{s}:"));
            },
            Instr::Jmp(s) => self.write(format!("JMP {s}")),
            Instr::Jeq(s) => self.write(format!("JEQ {s}")),
            Instr::Jne(s) => self.write(format!("JNE {s}")),
            Instr::Mov(dest, other) => {
                let dest = self.get_reg(dest);
                let other = self.get_reg(other);
                self.write(format!("LD r{dest}, r{other}"));
            },
            Instr::Push(reg) => {
                let reg = self.get_reg(reg);
                self.write(format!("PUSH r{reg}"));
            },
            Instr::Pop(reg) => {
                let reg = self.get_reg(reg);
                self.write(format!("POP r{reg}"));
            },
        }
    }

    fn write_binop(&mut self, mnemonic: &str, left: &VReg, right: &VReg, dest: &VReg) {
        let dest = self.get_reg(dest);
        let left = self.get_reg(left);
        let right = self.get_reg(right);

        self.write(format!("LD r{dest}, r{left}"));
        self.write(format!("{mnemonic} r{dest}, r{right}"));
    }

    fn get_reg(&mut self, vreg: &VReg) -> Reg {
        if let Some(&reg) = self.vreg_map.get(vreg) {
            self.touch(reg);
            return reg;
        }

        if let Some(&slot) = self.spill_slots.get(vreg) {
            let reg = self.alloc_physical_reg();
            self.spill_load(slot, reg);
            self.vreg_map.insert(*vreg, reg);
            self.touch(reg);
            return reg;
        }

        let reg = self.alloc_physical_reg();
        self.vreg_map.insert(*vreg, reg);
        self.touch(reg);
        reg
    }

    fn alloc_physical_reg(&mut self) -> Reg {
        if let Some(&r) = self.free_regs.iter().next() {
            self.free_regs.remove(&r);
            return r;
        }

        let victim_reg = self.access_order[0];
        let victim_vreg = *self.vreg_map.iter()
            .find(|(_, r)| **r == victim_reg).unwrap().0;

        let slot = *self.spill_slots.entry(victim_vreg).or_insert_with(|| {
            let s = self.next_slot;
            self.next_slot += 1;
            s
        });

        self.spill_store(slot, victim_reg);
        self.vreg_map.remove(&victim_vreg);
        self.access_order.remove(0);

        victim_reg
    }

    fn touch(&mut self, reg: Reg) {
        self.access_order.retain(|&r| r != reg);
        self.access_order.push(reg);
    }

    fn free_reg(&mut self, reg: Reg) {
        self.free_regs.insert(reg);
        if let Some((&vreg, _)) = self.vreg_map.iter().find(|(_, r)| **r == reg) {
            self.spill_slots.remove(&vreg);
        }
        self.vreg_map.retain(|_, r| *r != reg);
        self.access_order.retain(|&r| r != reg);
    }

    fn spill_store(&mut self, slot: u16, reg: Reg) {
        self.write(format!("ADD r{SPILL_BASE_REG}, {slot}"));
        self.write(format!("ST [r{SPILL_BASE_REG}], r{reg}"));
        self.write(format!("SUB r{SPILL_BASE_REG}, {slot}"));
    }

    fn spill_load(&mut self, slot: u16, reg: Reg) {
        self.write(format!("ADD r{SPILL_BASE_REG}, {slot}"));
        self.write(format!("LD r{reg}, [r{SPILL_BASE_REG}]"));
        self.write(format!("SUB r{SPILL_BASE_REG}, {slot}"));
    }

    fn write(&mut self, src: impl Into<String>) {
        self.output.extend(src.into().chars());
        self.output.extend("\n".chars());
    }

    fn current(&self) -> Option<Rc<Instr>> {
        self.instructions.get(self.pos).cloned()
    }

    fn advance(&mut self) {
        self.pos += 1
    }
}
