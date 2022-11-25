// might embed this as ValueType in Value struct that also contains is_assigment
// field. see parse/assign.go
#[derive(Default)]
pub enum Value {
    Float(f64),
    Int(i32),
    #[default]
    None,
}

pub mod eval;

pub mod context {

    use std::io::Write;

    use crate::exec::context::Context;

    use super::Value;

    pub trait Expr<'a, O: Write, E: Write> {
        fn prog_string(&self) -> String;
        fn eval(&self, ctx: &Context<'a, O, E>) -> Option<Value>;
    }

    pub trait UnaryOp<'a, O: Write, E: Write> {
        fn eval_unary(&self, ctx: &Context<'a, O, E>, right: Value) -> Value;
    }

    pub trait BinaryOp<'a, O: Write, E: Write> {
        fn eval_binary(
            &self,
            ctx: &Context<'a, O, E>,
            right: Value,
            left: Value,
        ) -> Value;
    }
}
