pub mod lexer;
pub mod error;
pub mod parser;
pub mod ir;

#[derive(Debug, PartialEq)]
pub struct Spanned<T: std::fmt::Debug + PartialEq> {
    pub element: T,
    pub span: std::ops::Range<usize>,
}
