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

    pub trait Expr {
        fn prog_string(&self) -> String;
        fn eval(&self, ctx: &Context) -> Option<Value>;
    }

    pub trait UnaryOp<'a> {
        fn eval_unary(&self, ctx: &Context, right: Value) -> Value;
    }

    pub trait BinaryOp<'a> {
        fn eval_binary(
            &self,
            ctx: &Context,
            right: Value,
            left: Value,
        ) -> Value;
    }
}
