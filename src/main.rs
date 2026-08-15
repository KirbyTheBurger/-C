const PATH: &str = "test.nc";

use neg_c::{lexer::tokenize, parser::Parser};

fn main() {
    let input = std::fs::read_to_string(PATH).unwrap();

    let tokens = match tokenize(&input) {
        Ok(t) => {
            t.iter().for_each(|t| print!("{:?}, ", t.element));
            print!("\n");
            t
        },
        Err(e) => {
            e.iter().for_each(|e| e.report(PATH));
            return;
        },
    };

    let _statements = match Parser::new(tokens).parse() {
        Ok(s) => {
            s.iter().for_each(|s| println!("{:?}", s.element));
            s
        },
        Err(e) => {
            e.iter().for_each(|e| e.report(PATH));
            return;
        }
    };
}
