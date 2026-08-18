#[cfg(test)]
mod tests {
    use crate::lexer::{Token::{self, *}, tokenize};

    fn run(src: &str) -> Vec<Token> {
        let tokens = tokenize(src).unwrap();
        tokens.into_iter().map(|t| t.element).collect()
    }

    #[test]
    fn print_keyword() {
        assert_eq!(run("print"), vec![Print]);
    }

    #[test]
    fn print_with_number() {
        assert_eq!(run("print 5"), vec![Print, Number(5)]);
    }

    #[test]
    fn decimal_basic() {
        assert_eq!(run("5"), vec![Number(5)]);
        assert_eq!(run("12345"), vec![Number(12345)]);
    }

    #[test]
    fn decimal_edge_values() {
        assert_eq!(run("0"), vec![Number(0)]);
        assert_eq!(run("65535"), vec![Number(65535)]);
    }

    #[test]
    fn hex_prefix_case() {
        assert_eq!(run("0xff"), vec![Number(255)]);
        assert_eq!(run("0XFF"), vec![Number(255)]);
    }

    #[test]
    fn hex_mixed_case_digits() {
        assert_eq!(run("0xAb12"), vec![Number(0xAB12)]);
    }

    #[test]
    fn hex_edge_values() {
        assert_eq!(run("0x0"), vec![Number(0)]);
        assert_eq!(run("0xFFFF"), vec![Number(65535)]);
    }

    #[test]
    fn binary_prefix_case() {
        assert_eq!(run("0b1010"), vec![Number(10)]);
        assert_eq!(run("0B1010"), vec![Number(10)]);
    }

    #[test]
    fn binary_edge_values() {
        assert_eq!(run("0b0"), vec![Number(0)]);
        assert_eq!(run("0b1111111111111111"), vec![Number(65535)]);
    }

    #[test]
    fn chained_numbers() {
        assert_eq!(run("1 2 3"), vec![Number(1), Number(2), Number(3)]);
        assert_eq!(run("10 0xA 0b1010"), vec![Number(10), Number(10), Number(10)]);
    }

    #[test]
    #[should_panic]
    fn decimal_overflow_errors() {
        run("65536");
    }

    #[test]
    #[should_panic]
    fn hex_overflow_errors() {
        run("0x10000");
    }
}