pub mod context;

pub mod function {

    use crate::value::{
        context::{BinaryOp, Expr, UnaryOp},
        Value,
    };

    pub struct Function<'a> {
        pub is_binary: bool,
        name: String,
        left: String,
        right: String,
        body: Vec<&'a dyn Expr>,
        pub(crate) locals: Vec<String>,
        globals: Vec<String>,
    }

    impl<'a> Function<'a> {
        pub fn name(&self) -> &str {
            self.name.as_ref()
        }
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
    impl<'a> BinaryOp<'a> for &'a Function<'a> {
        fn eval_binary(
            &self,
            _ctx: &super::context::Context,
            _right: Value,
            _left: Value,
        ) -> Value {
            todo!()
        }
    }
}
