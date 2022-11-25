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

    use crate::exec::context::Context;

    use super::Value;

    pub trait Expr<'a> {
        fn prog_string(&self) -> String;
        fn eval(&self, ctx: &Context<'a>) -> Option<Value>;
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
