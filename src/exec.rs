pub mod context;

pub mod opdef {
    #[allow(unused)]
    pub struct OpDef {
        name: String,
        is_binary: bool,
    }
}

pub mod operator {
    use crate::value::eval::{
        binary::{is_binary_op, BinaryBuiltin},
        unary::{is_unary_op, UnaryBuiltin},
    };
    use std::str::FromStr;

    use super::context::Context;

    /// reports whether the operator is predefined, a built-in
    pub fn predefined(op: &str) -> bool {
        BinaryBuiltin::from_str(op).is_ok()
            || UnaryBuiltin::from_str(op).is_ok()
    }

    impl<'a> Context<'a> {
        /// reports whether or not `op` is known
        pub fn defined_op(&self, op: &str) -> bool {
            if self.is_variable(op) {
                return false;
            }
            predefined(op)
                || self.binary_fn.contains_key(op)
                || self.unary_fn.contains_key(op)
        }

        pub fn defined_binary(&self, op: &str) -> bool {
            if self.is_variable(op) {
                return false;
            }
            self.binary_fn.contains_key(op) || is_binary_op(op)
        }

        /// reports whether the operator is a known unary
        pub fn defined_unary(&self, op: &str) -> bool {
            if self.is_variable(op) {
                return false;
            }
            self.unary_fn.contains_key(op) || is_unary_op(op)
        }
    }
}

pub mod function {

    use crate::value::{
        context::{expr::Expr, BinaryOp, UnaryOp},
        Value,
    };

    #[allow(unused)]
    #[derive(Clone, Debug, Default)]
    pub struct Function {
        pub is_binary: bool,
        pub name: String,
        pub left: String,
        pub right: String,
        body: Vec<Expr>,
        pub(crate) locals: Vec<String>,
        globals: Vec<String>,
    }

    impl Function {
        pub fn name(&self) -> &str {
            self.name.as_ref()
        }
    }

    impl<'a> UnaryOp<'a> for &'a Function {
        fn eval_unary(
            &self,
            _ctx: &super::context::Context,
            _right: crate::value::Value,
        ) -> Value {
            todo!()
        }
    }
    impl<'a> BinaryOp<'a> for &'a Function {
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
