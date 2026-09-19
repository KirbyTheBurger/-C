use crate::{ir::builder::IRBuilder, lexer::tokenize, parser::Parser, writer::Writer};

pub mod lexer;
pub mod error;
pub mod parser;
pub mod ir;
pub mod writer;

#[derive(Debug, PartialEq)]
pub struct Spanned<T: std::fmt::Debug + PartialEq> {
    pub element: T,
    pub span: std::ops::Range<usize>,
}

pub fn compile(src: String, file: String, debug: bool) -> Option<String> {
    let tokens = match tokenize(&src) {
        Ok(t) => {
            if debug {
                t.iter().for_each(|t| print!("{:?}, ", t.element));
                print!("\n");
            }
            t
        },
        Err(e) => {
            e.iter().for_each(|e| e.report(&file));
            return None;
        },
    };

    let statements = match Parser::new(tokens).parse() {
        Ok(s) => {
            if debug {
                s.iter().for_each(|s| println!("{:?}", s.element));
            }
            s
        },
        Err(e) => {
            e.iter().for_each(|e| e.report(&file));
            return None;
        }
    };

    let ir = IRBuilder::new(statements).build();
    if debug {
        ir.iter().for_each(|i| println!("{:?}", i));
    }

    let asm = Writer::new(ir).process();
    Some(asm)
}
