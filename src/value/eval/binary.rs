use super::super::Value;

use crate::exec::context::Context;

use super::super::context::BinaryOp;

use super::ParseBuiltinError;

use std::str::FromStr;

#[derive(Debug)]
pub enum BinaryBuiltin {
    NewComplex,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Exp,
}

/// return whether or not `s` is a BinaryBuiltin
pub fn is_binary_op(s: &str) -> bool {
    BinaryBuiltin::from_str(s).is_ok()
}

impl FromStr for BinaryBuiltin {
    type Err = ParseBuiltinError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "j" => Ok(Self::NewComplex),
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            "%" => Ok(Self::Mod),
            "**" => Ok(Self::Exp),
            _ => Err(ParseBuiltinError),
        }
    }
}

impl<'a> BinaryOp<'a> for BinaryBuiltin {
    fn eval_binary(
        &self,
        _ctx: &Context<'a>,
        left: Value,
        right: Value,
    ) -> Value {
        match self {
            BinaryBuiltin::Plus => {
                if let Value::Int(i) = left {
                    if let Value::Int(j) = right {
                        return Value::Int(i + j);
                    }
                }
            }
            BinaryBuiltin::Minus => {
                if let Value::Int(i) = left {
                    if let Value::Int(j) = right {
                        return Value::Int(i - j);
                    }
                }
            }
            BinaryBuiltin::NewComplex => return Value::complex(left, right),
            BinaryBuiltin::Mul => {
                if let Value::Int(i) = left {
                    if let Value::Int(j) = right {
                        return Value::Int(i * j);
                    }
                }
            }
            BinaryBuiltin::Div => {
                if let Value::Int(i) = left {
                    if let Value::Int(j) = right {
                        return Value::Int(i / j);
                    }
                }
            }
            BinaryBuiltin::Mod => {
                if let Value::Int(i) = left {
                    if let Value::Int(j) = right {
                        return Value::Int(i % j);
                    }
                }
            }
            BinaryBuiltin::Exp => {
                if let Value::Int(i) = left {
                    if let Value::Int(j) = right {
                        return Value::Int(i.pow(j.try_into().unwrap()));
                    }
                }
            }
        }
        todo!()
    }
}
