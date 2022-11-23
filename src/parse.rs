use std::io::{Read, Write};

use crate::{
    exec::context::Context,
    scan::{Scanner, Token},
};

#[allow(unused)]
pub struct Parser<'a, R: Read, O: Write, E: Write> {
    scanner: &'a Scanner<'a, R, O, E>,
    tokens: Vec<Token>,
    token_buf: [Token; 100],
    filename: String,
    line_num: usize,
    // why do we take this? surely it can't be a different context than the one
    // in our scanner?
    context: &'a Context<'a, O, E>,
}

impl<'a, R: Read, O: Write, E: Write> Parser<'a, R, O, E> {
    pub fn new(
        filename: &str,
        scanner: &'a Scanner<'a, R, O, E>,
        context: &'a Context<'a, O, E>,
    ) -> Self {
        Self {
            scanner,
            tokens: Vec::new(),
            token_buf: std::array::from_fn(|_| Token::default()),
            filename: filename.to_owned(),
            line_num: 0,
            context,
        }
    }
}
