#[cfg(test)]
mod tests {
    use crate::{Spanned, ir::Instr, writer::Writer};

    fn spanned(instr: Instr) -> Spanned<Instr> {
        Spanned {
            element: instr,
            span: 0..0,
        }
    }

    #[test]
    fn test_single_print() {
        let instrs = vec![
            spanned(Instr::LoadImm(0, 42)),
            spanned(Instr::Print(0)),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(output, "LD r0, 42\nOUT r0\n");
    }

        #[test]
    fn test_double_print() {
        let instrs = vec![
            spanned(Instr::LoadImm(0, 1)),
            spanned(Instr::Print(0)),
            spanned(Instr::LoadImm(1, 2)),
            spanned(Instr::Print(1)),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(output, "LD r0, 1\nOUT r0\nLD r0, 2\nOUT r0\n");
    }   
}
