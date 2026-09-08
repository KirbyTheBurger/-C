pub mod builder;

pub type VReg = usize;

#[derive(Debug)]
pub enum Instr {
    LoadImm(VReg, u16),
    Print(VReg),
}
