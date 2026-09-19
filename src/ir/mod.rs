pub mod builder;
mod tests;

pub type VReg = usize;

#[derive(Debug, PartialEq, Clone)]
pub enum Instr {
    Drop(VReg),

    LoadImm(VReg, u16),
    Mov(VReg, VReg),

    Add {
        left: VReg,
        right: VReg,
        dest: VReg,
    },
    Sub {
        left: VReg,
        right: VReg,
        dest: VReg,
    },
    Mul {
        left: VReg,
        right: VReg,
        dest: VReg,
    },
    Div {
        left: VReg,
        right: VReg,
        dest: VReg,
    },
    Mod {
        left: VReg,
        right: VReg,
        dest: VReg,
    },

    Cmp(VReg, VReg),

    Label(String),
    Jmp(String),
    Jeq(String),
    Jne(String),

    Push(VReg),
    Pop(VReg),

    Print(VReg),
}

enum Ty {
    Char,
    Int,
}

