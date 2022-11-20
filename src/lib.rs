pub mod config {
    use std::{
        io::{Stderr, Stdout, Write},
        time::{self, Duration, SystemTime, UNIX_EPOCH},
    };

    pub struct Config<O: Write, E: Write> {
        prompt: String,
        output: O,
        err_output: E,
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

    impl Default for Config<Stdout, Stderr> {
        fn default() -> Self {
            Self {
                prompt: String::new(),
                output: std::io::stdout(),
                err_output: std::io::stderr(),
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
        use std::io::Write;

        use crate::exec::context::Context;

        use super::Value;

        type UnaryFn<O: Write, E: Write> =
            dyn Fn(Context<O, E>, Value) -> Value;

        pub struct UnaryOp<O: Write + 'static, E: Write + 'static> {
            name: String,
            element_wise: bool,
            fun: Vec<&'static UnaryFn<O, E>>,
        }

        pub fn reduce<O: Write, E: Write>(
            c: &Context<O, E>,
            op: &str,
            v: Value,
        ) -> Value {
            todo!()
        }

        pub fn scan<O: Write, E: Write>(
            c: &Context<O, E>,
            op: &str,
            v: Value,
        ) -> Value {
            todo!()
        }
    }

    pub mod context {
        use std::io::Write;

        use crate::exec::context::Context;

        use super::Value;

        pub trait Expr<O, E>
        where
            O: Write,
            E: Write,
        {
            fn prog_string(&self) -> String;
            fn eval(&self, ctx: &Context<O, E>) -> Option<Value>;
        }

        pub trait UnaryOp<O: Write, E: Write>: Sync {
            fn eval_unary(&self, ctx: &Context<O, E>, right: Value) -> Value;
        }
    }

    pub mod unary {
        use std::{collections::HashMap, io::Write};

        use super::eval::UnaryOp;

        pub static UNARY_OPS: HashMap<String, UnaryOp> = HashMap::new();

        pub fn unary_ops(
            op: &str,
        ) -> Option<UnaryOp> {
            UNARY_OPS.get(op).copied()
        }
    }
}

pub mod exec;
