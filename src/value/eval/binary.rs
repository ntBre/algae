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
            "%" | "mod" => Ok(Self::Mod),
            "**" => Ok(Self::Exp),
            _ => Err(ParseBuiltinError),
        }
    }
}

macro_rules! make_ops {
    ($typ: path, $p1: pat_param, $p2: pat_param, $left: ident, $right: ident,
     $expr: expr) => {
        if let $typ($p1) = $left {
            if let $typ($p2) = $right {
                return $typ($expr);
            }
        }
    };
    ($typ1: path, $p1: pat_param, $typ2: path, $p2: pat_param,
     $left: ident, $right: ident, $ret_typ: path, $expr: expr) => {
        if let $typ1($p1) = $left {
            if let $typ2($p2) = $right {
                return $ret_typ($expr);
            }
        }
    };
}

impl<'a> BinaryOp<'a> for BinaryBuiltin {
    fn eval_binary(
        &self,
        _ctx: &Context<'a>,
        left: Value,
        right: Value,
    ) -> Value {
        use Value::*;
        match self {
            BinaryBuiltin::Plus => {
                make_ops!(Int, i, j, left, right, i + j);
                make_ops!(Rational, i, j, left, right, i + j);
            }
            BinaryBuiltin::Minus => {
                make_ops!(Int, i, j, left, right, i - j);
            }
            BinaryBuiltin::NewComplex => return Value::complex(left, right),
            BinaryBuiltin::Mul => {
                make_ops!(Int, i, j, left, right, i * j);
            }
            BinaryBuiltin::Div => {
                make_ops!(Int, i, j, left, right, i / j);
            }
            BinaryBuiltin::Mod => {
                make_ops!(Int, i, j, left, right, i % j);
            }
            BinaryBuiltin::Exp => {
                make_ops!(Int, i, j, left, right, i.pow(j.try_into().unwrap()));
                make_ops!(Rational, i, Int, j, left, right, Rational, i.pow(j));
            }
        }
        todo!()
    }
}
