use super::Value;
use crate::exec::context::Context;

pub struct ParseBuiltinError;

pub mod binary;
pub mod unary;

pub fn reduce(_c: &Context, _op: &str, _v: Value) -> Value {
    todo!()
}

pub fn scan(_c: &Context, _op: &str, _v: Value) -> Value {
    todo!()
}

pub fn product(_c: &Context, _u: Value, _op: &str, _v: Value) -> Value {
    todo!()
}
