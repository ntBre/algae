use std::str::FromStr;

use crate::value::eval::{binary::BinaryBuiltin, unary::UnaryBuiltin};

pub mod context;

pub fn predefined(op: &str) -> bool {
    BinaryBuiltin::from_str(op).is_ok() || UnaryBuiltin::from_str(op).is_ok()
}

pub mod function {

    use std::io::Write;

    use crate::value::{
        context::{BinaryOp, Expr, UnaryOp},
        Value,
    };

    #[allow(unused)]
    pub struct Function<'a, O: Write, E: Write> {
        pub is_binary: bool,
        name: String,
        left: String,
        right: String,
        body: Vec<&'a dyn Expr<'a, O, E>>,
        pub(crate) locals: Vec<String>,
        globals: Vec<String>,
    }

    impl<'a, O: Write, E: Write> Function<'a, O, E> {
        pub fn name(&self) -> &str {
            self.name.as_ref()
        }
    }

    impl<'a, O: Write, E: Write> UnaryOp<'a, O, E> for &'a Function<'a, O, E> {
        fn eval_unary(
            &self,
            _ctx: &super::context::Context<O, E>,
            _right: crate::value::Value,
        ) -> Value {
            todo!()
        }
    }
    impl<'a, O: Write, E: Write> BinaryOp<'a, O, E> for &'a Function<'a, O, E> {
        fn eval_binary(
            &self,
            _ctx: &super::context::Context<O, E>,
            _right: Value,
            _left: Value,
        ) -> Value {
            todo!()
        }
    }
}
