pub mod context;

pub mod function {
    use std::io::Write;

    use crate::value::{
        context::{Expr, UnaryOp},
        Value,
    };

    pub struct Function<O: Write, E: Write> {
        is_binary: bool,
        name: String,
        left: String,
        right: String,
        body: Vec<Box<dyn Expr<O, E> + Sync>>,
        pub(crate) locals: Vec<String>,
        globals: Vec<String>,
    }

    impl<O: Write, E: Write> UnaryOp<O, E> for Function<O, E> {
        fn eval_unary(
            &self,
            ctx: &super::context::Context<O, E>,
            right: crate::value::Value,
        ) -> Value {
            todo!()
        }
    }
}
