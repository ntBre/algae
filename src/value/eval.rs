use super::Value;
use crate::exec::context::Context;

pub struct ParseBuiltinError;

pub mod binary;
pub mod unary;

pub fn reduce<'a>(_c: &Context<'a>, _op: &str, _v: Value) -> Value {
    todo!()
}

pub fn scan<'a>(_c: &Context<'a>, _op: &str, _v: Value) -> Value {
    todo!()
}

pub fn product<'a>(_c: &Context<'a>, _u: Value, _op: &str, _v: Value) -> Value {
    todo!()
}
