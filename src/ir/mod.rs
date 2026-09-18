use crate::Spanned;

pub mod builder;
mod tests;

pub type VReg = usize;

#[derive(Debug, PartialEq)]
pub enum Instr {
    LoadImm(VReg, u16),

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

    Cmp(VReg, VReg),

    Label(String),
    Jmp(String),
    Jnz(String),
    Jz(String),


    Print(VReg),
}

enum Ty {
    Char,
    Int,
}

impl Instr {
    fn with_span(self, span: std::ops::Range<usize>) -> Spanned<Self> {
        Spanned {
            element: self,
            span,
        }
    }
}
