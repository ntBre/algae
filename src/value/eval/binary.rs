use super::super::Value;

use crate::exec::context::Context;

use super::super::context::BinaryOp;

use super::ParseBuiltinError;

use std::str::FromStr;

pub enum BinaryBuiltin {
    Plus,
}

/// return whether or not `s` is a BinaryBuiltin
pub fn is_binary_op(s: &str) -> bool {
    BinaryBuiltin::from_str(s).is_ok()
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

impl<'a> BinaryOp<'a> for BinaryBuiltin {
    fn eval_binary(
        &self,
        _ctx: &Context<'a>,
        right: Value,
        left: Value,
    ) -> Value {
        match self {
            BinaryBuiltin::Plus => {
                if let Value::Int(i) = left {
                    if let Value::Int(j) = right {
                        return Value::Int(i + j);
                    }
                }
            }
        }
        todo!()
    }
}
