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
