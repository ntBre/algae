use std::fmt::Display;

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

pub mod eval;

pub mod context {

    use crate::exec::context::Context;

    use super::Value;

    #[derive(Clone, Debug)]
    pub enum Expr {
        Empty,
    }

    impl Expr {
        pub fn prog_string(&self) -> String {
            todo!();
        }
        pub fn eval(&self, _ctx: &Context) -> Option<Value> {
            todo!();
        }
    }

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
