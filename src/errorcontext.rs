use crate::span::Span;

#[derive(Debug)]
pub struct ErrorContext {
    errors: Vec<Error>
}

impl ErrorContext {
    pub fn new() -> Self {
        Self {errors: Vec::new()}
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UndefinedVariable
}
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    span: Span
}