use std::fmt::Display;

use crate::{config::Config, parse::ParseError};

use self::context::expr::Expr;

// might embed this as ValueType in Value struct that also contains is_assigment
// field. see parse/assign.go
#[derive(Clone, Debug, Default)]
pub enum Value {
    Float(f64),
    Int(i32),
    #[default]
    None,
}

impl Display for Value {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Float(f) => write!(w, "{:.8}", *f),
            Value::Int(d) => write!(w, "{d}"),
            Value::None => todo!(),
        }
    }
}

pub fn parse_string(_text: String) -> String {
    todo!()
}

pub fn parse(_conf: &Config, _s: &str) -> Result<Expr, ParseError> {
    todo!()
}

pub mod eval;

pub mod context {

    use crate::exec::context::Context;

    use super::Value;

    pub mod expr;

    pub trait UnaryOp<'a> {
        fn eval_unary(&self, ctx: &Context<'a>, right: Value) -> Value;
    }

    pub trait BinaryOp<'a> {
        fn eval_binary(
            &self,
            ctx: &Context<'a>,
            right: Value,
            left: Value,
        ) -> Value;
    }
}
