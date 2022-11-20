pub mod context;

pub mod function {

    use crate::value::{
        context::{Expr, UnaryOp},
        Value,
    };

    pub struct Function<'a> {
        is_binary: bool,
        name: String,
        left: String,
        right: String,
        body: Vec<&'a dyn Expr>,
        pub(crate) locals: Vec<String>,
        globals: Vec<String>,
    }

    impl<'a> UnaryOp<'a> for &'a Function<'a> {
        fn eval_unary(
            &self,
            _ctx: &super::context::Context,
            _right: crate::value::Value,
        ) -> Value {
            todo!()
        }
    }
}
