#[cfg(test)]
mod tests {
    use crate::{ir::Instr, writer::Writer};

    #[test]
    fn test_single_print() {
        let instrs = vec![
            Instr::LoadImm(0, 42),
            Instr::Print(0),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(output, "LD r7, 16384\nLD r0, 42\nOUT r0\n");
    }

    #[test]
    fn test_double_print() {
        let instrs = vec![
            Instr::LoadImm(0, 1),
            Instr::Print(0),
            Instr::LoadImm(1, 2),
            Instr::Print(1),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(output, "LD r7, 16384\nLD r0, 1\nOUT r0\nLD r0, 2\nOUT r0\n");
    }

    #[test]
    fn test_single_add() {
        let instrs = vec![
            Instr::LoadImm(1, 1),
            Instr::LoadImm(2, 2),
            Instr::Add { left: 1, right: 2, dest: 0 },
            Instr::Print(0),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(
            output,
            "LD r7, 16384\nLD r0, 1\nLD r1, 2\nLD r2, r0\nADD r2, r1\nOUT r2\n"
        );
    }

    #[test]
    fn test_single_sub() {
        let instrs = vec![
            Instr::LoadImm(4, 5),
            Instr::LoadImm(5, 3),
            Instr::Sub { left: 4, right: 5, dest: 3 },
            Instr::Print(3),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(
            output,
            "LD r7, 16384\nLD r0, 5\nLD r1, 3\nLD r2, r0\nSUB r2, r1\nOUT r2\n"
        );
    }

    #[test]
    fn test_single_mul() {
        let instrs = vec![
            Instr::LoadImm(7, 4),
            Instr::LoadImm(8, 6),
            Instr::Mul { left: 7, right: 8, dest: 6 },
            Instr::Print(6),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(
            output,
            "LD r7, 16384\nLD r0, 4\nLD r1, 6\nLD r2, r0\nMUL r2, r1\nOUT r2\n"
        );
    }

    #[test]
    fn test_single_div() {
        let instrs = vec![
            Instr::LoadImm(10, 8),
            Instr::LoadImm(11, 2),
            Instr::Div { left: 10, right: 11, dest: 9 },
            Instr::Print(9),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(
            output,
            "LD r7, 16384\nLD r0, 8\nLD r1, 2\nLD r2, r0\nDIV r2, r1\nOUT r2\n"
        );
    }

    #[test]
    fn test_add_mul_precedence() {
        let instrs = vec![
            Instr::LoadImm(13, 1),
            Instr::LoadImm(15, 2),
            Instr::LoadImm(16, 3),
            Instr::Mul { left: 15, right: 16, dest: 14 },
            Instr::Add { left: 13, right: 14, dest: 12 },
            Instr::Print(12),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(
            output,
            "LD r7, 16384\nLD r0, 1\nLD r1, 2\nLD r2, 3\nLD r3, r1\nMUL r3, r2\nLD r4, r0\nADD r4, r3\nOUT r4\n"
        );
    }

    #[test]
    fn test_paren_overrides_precedence() {
        let instrs = vec![
            Instr::LoadImm(21, 1),
            Instr::LoadImm(22, 2),
            Instr::Add { left: 21, right: 22, dest: 19 },
            Instr::LoadImm(20, 3),
            Instr::Mul { left: 19, right: 20, dest: 18 },
            Instr::Print(18),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(
            output,
            "LD r7, 16384\nLD r0, 1\nLD r1, 2\nLD r2, r0\nADD r2, r1\nLD r3, 3\nLD r4, r2\nMUL r4, r3\nOUT r4\n"
        );
    }

    #[test]
    fn test_spill_and_reload() {
        let instrs = vec![
            Instr::LoadImm(0, 10),
            Instr::LoadImm(1, 11),
            Instr::LoadImm(2, 12),
            Instr::LoadImm(3, 13),
            Instr::LoadImm(4, 14),
            Instr::LoadImm(5, 15),
            Instr::LoadImm(6, 16),
            // all 7 usable registers (r0-r6) are now occupied by vregs 0-6;
            // this forces eviction of the least-recently-used one (vreg 0, in r0)
            Instr::LoadImm(7, 17),
            // vreg 0 was spilled — reading it now forces a reload,
            // which itself evicts vreg 1 (now the least-recently-used)
            Instr::Print(0),
        ];

        let mut writer = Writer::new(instrs);
        let output = writer.process();

        assert_eq!(
            output,
            "LD r7, 16384\n\
            LD r0, 10\n\
            LD r1, 11\n\
            LD r2, 12\n\
            LD r3, 13\n\
            LD r4, 14\n\
            LD r5, 15\n\
            LD r6, 16\n\
            ADD r7, 0\n\
            ST [r7], r0\n\
            SUB r7, 0\n\
            LD r0, 17\n\
            ADD r7, 1\n\
            ST [r7], r1\n\
            SUB r7, 1\n\
            ADD r7, 0\n\
            LD r1, [r7]\n\
            SUB r7, 0\n\
            OUT r1\n"
        );
    }
}
