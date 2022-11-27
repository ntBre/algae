use num::integer::Roots;

use super::super::Value;

use crate::exec::context::Context;

use super::super::context::UnaryOp;

use super::ParseBuiltinError;

use std::str::FromStr;

pub enum UnaryBuiltin {
    Roll,
    Sqrt,
    Acos,
}

/// return whether or not `s` is a UnaryBuiltin
pub fn is_unary_op(s: &str) -> bool {
    UnaryBuiltin::from_str(s).is_ok()
}

impl FromStr for UnaryBuiltin {
    type Err = ParseBuiltinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "?" => Ok(Self::Roll),
            "sqrt" => Ok(Self::Sqrt),
            "acos" => Ok(Self::Acos),
            _ => Err(ParseBuiltinError),
        }
    }
}

impl<'a> UnaryOp<'a> for UnaryBuiltin {
    fn eval_unary(&self, _ctx: &Context<'a>, right: Value) -> Value {
        use Value::*;
        match self {
            UnaryBuiltin::Sqrt => match right {
                Float(f) => Value::Float(f.sqrt()),
                Int(f) => {
                    if f >= 0 {
                        Value::Int(f.sqrt())
                    } else {
                        Value::complex(Int(0), Int(f.abs().sqrt()))
                    }
                }
                Complex(_) => todo!(),
                Rational(_) => todo!(),
                None => todo!(),
            },
            UnaryBuiltin::Roll => todo!(),
            UnaryBuiltin::Acos => match right {
                Float(_) => todo!(),
                Int(_) => todo!(),
                Complex(c) => Value::Complex(c.acos()),
                Rational(_) => todo!(),
                None => todo!(),
            },
        }
    }
}
