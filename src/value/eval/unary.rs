use super::super::Value;

use crate::exec::context::Context;

use super::super::context::UnaryOp;

use super::ParseBuiltinError;

use std::io::Write;
use std::str::FromStr;

pub enum UnaryBuiltin {
    Roll,
}

impl FromStr for UnaryBuiltin {
    type Err = ParseBuiltinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "?" => Ok(Self::Roll),
            _ => Err(ParseBuiltinError),
        }
    }
}

impl<'a, O: Write, E: Write> UnaryOp<'a, O, E> for UnaryBuiltin {
    fn eval_unary(&self, _ctx: &Context<'a, O, E>, _right: Value) -> Value {
        todo!()
    }
}
