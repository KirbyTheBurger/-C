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

    #[test]
    fn test_single_add() {
        let instrs = vec![
            spanned(Instr::LoadImm(1, 1)),
            spanned(Instr::LoadImm(2, 2)),
            spanned(Instr::Add { left: 1, right: 2, dest: 0 }),
            spanned(Instr::Print(0)),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(
            output,
            "LD r0, 1\nLD r1, 2\nADD r0, r1\nLD r2, r0\nOUT r2\n"
        );
    }

    #[test]
    fn test_single_sub() {
        let instrs = vec![
            spanned(Instr::LoadImm(4, 5)),
            spanned(Instr::LoadImm(5, 3)),
            spanned(Instr::Sub { left: 4, right: 5, dest: 3 }),
            spanned(Instr::Print(3)),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(
            output,
            "LD r0, 5\nLD r1, 3\nSUB r0, r1\nLD r2, r0\nOUT r2\n"
        );
    }

    #[test]
    fn test_single_mul() {
        let instrs = vec![
            spanned(Instr::LoadImm(7, 4)),
            spanned(Instr::LoadImm(8, 6)),
            spanned(Instr::Mul { left: 7, right: 8, dest: 6 }),
            spanned(Instr::Print(6)),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(
            output,
            "LD r0, 4\nLD r1, 6\nMUL r0, r1\nLD r2, r0\nOUT r2\n"
        );
    }

    #[test]
    fn test_single_div() {
        let instrs = vec![
            spanned(Instr::LoadImm(10, 8)),
            spanned(Instr::LoadImm(11, 2)),
            spanned(Instr::Div { left: 10, right: 11, dest: 9 }),
            spanned(Instr::Print(9)),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(
            output,
            "LD r0, 8\nLD r1, 2\nDIV r0, r1\nLD r2, r0\nOUT r2\n"
        );
    }

    #[test]
    fn test_add_mul_precedence() {
        let instrs = vec![
            spanned(Instr::LoadImm(13, 1)),
            spanned(Instr::LoadImm(15, 2)),
            spanned(Instr::LoadImm(16, 3)),
            spanned(Instr::Mul { left: 15, right: 16, dest: 14 }),
            spanned(Instr::Add { left: 13, right: 14, dest: 12 }),
            spanned(Instr::Print(12)),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(
            output,
            "LD r0, 1\nLD r1, 2\nLD r2, 3\nMUL r1, r2\nLD r3, r1\nADD r0, r3\nLD r1, r0\nOUT r1\n"
        );
    }

    #[test]
    fn test_paren_overrides_precedence() {
        let instrs = vec![
            spanned(Instr::LoadImm(21, 1)),
            spanned(Instr::LoadImm(22, 2)),
            spanned(Instr::Add { left: 21, right: 22, dest: 19 }),
            spanned(Instr::LoadImm(20, 3)),
            spanned(Instr::Mul { left: 19, right: 20, dest: 18 }),
            spanned(Instr::Print(18)),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(
            output,
            "LD r0, 1\nLD r1, 2\nADD r0, r1\nLD r2, r0\nLD r0, 3\nMUL r2, r0\nLD r1, r2\nOUT r1\n"
        );
    }
}
