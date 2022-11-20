use super::super::Value;

use crate::exec::context::Context;

use super::super::context::UnaryOp;

use super::ParseBuiltinError;

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

impl UnaryOp<'_> for UnaryBuiltin {
    fn eval_unary(&self, _ctx: &Context, _right: Value) -> Value {
        todo!()
    }
}
