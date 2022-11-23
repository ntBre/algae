use std::io::Read;

use crate::{
    exec::context::Context,
    scan::{Scanner, Token},
};

#[allow(unused)]
pub struct Parser<'a, R: Read> {
    scanner: &'a Scanner<'a, R>,
    tokens: Vec<Token>,
    token_buf: [Token; 100],
    filename: String,
    line_num: usize,
    // why do we take this? surely it can't be a different context than the one
    // in our scanner?
    context: &'a Context<'a>,
}

impl<'a, R: Read> Parser<'a, R> {
    pub fn new(
        filename: String,
        scanner: &'a Scanner<'a, R>,
        context: &'a Context<'a>,
    ) -> Self {
        Self {
            scanner,
            tokens: Vec::new(),
            token_buf: std::array::from_fn(|_| Token::default()),
            filename,
            line_num: 0,
            context,
        }
    }
}
