pub mod config {
    use std::time::{self, Duration, SystemTime, UNIX_EPOCH};

    pub struct Config {
        prompt: String,
        format: String,
        rat_format: String,
        format_verb: String,
        format_prec: usize,
        format_float: bool,
        origin: usize,
        seed: u64,
        max_bits: usize,
        max_digits: usize,
        max_stack: usize,
        float_prec: usize,
        real_time: time::Duration,
        user_time: time::Duration,
        sys_time: time::Duration,
        input_base: usize,
        output_base: usize,
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                prompt: String::new(),
                format: String::new(),
                rat_format: String::new(),
                format_verb: String::new(),
                format_prec: 0,
                format_float: false,
                origin: 1,
                seed: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                max_bits: 1_000_000,
                max_digits: 10_000,
                max_stack: 100_000,
                float_prec: 256,
                real_time: Duration::default(),
                user_time: Duration::default(),
                sys_time: Duration::default(),
                input_base: 0,
                output_base: 0,
            }
        }
    }
}

pub mod value {
    #[derive(Default)]
    pub enum Value {
        Float(f64),
        #[default]
        None,
    }

    pub mod eval {
        use std::str::FromStr;

        use crate::exec::context::Context;

        use super::{context::UnaryOp, Value};

        pub enum Builtin {
            Roll,
        }

        pub struct ParseBuiltinError;
        impl FromStr for Builtin {
            type Err = ParseBuiltinError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    "?" => Ok(Self::Roll),
                    _ => Err(ParseBuiltinError),
                }
            }
        }

        impl UnaryOp<'_> for Builtin {
            fn eval_unary(&self, _ctx: &Context, _right: Value) -> Value {
                todo!()
            }
        }

        pub fn reduce(_c: &Context, _op: &str, _v: Value) -> Value {
            todo!()
        }

        pub fn scan(_c: &Context, _op: &str, _v: Value) -> Value {
            todo!()
        }
    }

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
    }
}

pub mod exec;
