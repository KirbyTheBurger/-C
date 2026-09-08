#[cfg(test)]
mod tests {
    use crate::{ir::{Instr, builder::IRBuilder}, lexer::tokenize, parser::Parser};

    fn run(src: &str) -> Vec<Instr> {
        let tokens = tokenize(src).unwrap();
        let statements = Parser::new(tokens).parse().unwrap();
        IRBuilder::new(statements).build().unwrap()
            .into_iter().map(|i| i.element).collect()
    }

    #[test]
    fn single_print() {
        let ir = run("print 5");
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(0, 5),
                Instr::Print(0),
            ]
        );
    }

    #[test]
    fn multiple_prints() {
        let ir = run("print 1\nprint 2\nprint 3");
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(0, 1),
                Instr::Print(0),
                Instr::LoadImm(1, 2),
                Instr::Print(1),
                Instr::LoadImm(2, 3),
                Instr::Print(2),
            ]
        );
    }

    #[test]
    fn standalone_expression_single_number() {
        let ir = run("5");
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(0, 5),
            ]
        );
    }

    #[test]
    fn multiple_standalone_expressions() {
        let ir = run("5\n7\n9");
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(0, 5),
                Instr::LoadImm(1, 7),
                Instr::LoadImm(2, 9),
            ]
        );
    }

    #[test]
    fn prints_and_standalone_expressions_mixed() {
        let ir = run("print 1\n5\nprint 2\n7");
        assert_eq!(
            ir,
            vec![
                Instr::LoadImm(0, 1),
                Instr::Print(0),
                Instr::LoadImm(1, 5),
                Instr::LoadImm(2, 2),
                Instr::Print(2),
                Instr::LoadImm(3, 7),
            ]
        );
    }

    #[test]
    fn empty_program() {
        let ir = run("");
        assert_eq!(ir, vec![]);
    }
}
