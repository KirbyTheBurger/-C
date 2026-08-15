pub mod lexer;
pub mod error;
pub mod parser;

pub struct Spanned<T> {
    element: T,
    span: std::ops::Range<usize>,
}
