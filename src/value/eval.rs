use std::io::Write;

use super::Value;
use crate::exec::context::Context;

pub struct ParseBuiltinError;

pub mod binary;
pub mod unary;

pub fn reduce<'a, O: Write, E: Write>(
    _c: &Context<'a, O, E>,
    _op: &str,
    _v: Value,
) -> Value {
    todo!()
}

pub fn scan<'a, O: Write, E: Write>(
    _c: &Context<'a, O, E>,
    _op: &str,
    _v: Value,
) -> Value {
    todo!()
}

pub fn product<'a, O: Write, E: Write>(
    _c: &Context<'a, O, E>,
    _u: Value,
    _op: &str,
    _v: Value,
) -> Value {
    todo!()
}
