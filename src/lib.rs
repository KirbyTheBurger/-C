pub mod lexer;
pub mod error;
pub mod parser;

pub struct Shared<T> {
    element: T,
    span: std::ops::Range<usize>,
}
