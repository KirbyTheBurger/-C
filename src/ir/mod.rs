use crate::Spanned;

pub mod builder;
mod tests;

pub type VReg = usize;

#[derive(Debug, PartialEq)]
pub enum Instr {
    LoadImm(VReg, u16),
    Print(VReg),
}

impl Instr {
    fn with_span(self, span: std::ops::Range<usize>) -> Spanned<Self> {
        Spanned {
            element: self,
            span,
        }
    }
}
