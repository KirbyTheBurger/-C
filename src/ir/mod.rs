pub mod builder;

pub type VReg = usize;

pub enum Instr {
    LoadImm(VReg, u16),
    Print(VReg),
}
