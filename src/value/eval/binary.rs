use super::super::Value;

use crate::exec::context::Context;

use super::super::context::BinaryOp;

use super::ParseBuiltinError;

use std::str::FromStr;

pub enum BinaryBuiltin {
    Plus,
}

impl FromStr for BinaryBuiltin {
    type Err = ParseBuiltinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Plus),
            _ => Err(ParseBuiltinError),
        }
    }
}

impl BinaryOp<'_> for BinaryBuiltin {
    fn eval_binary(
        &self,
        _ctx: &Context,
        _right: Value,
        _left: Value,
    ) -> Value {
        todo!()
    }
}
