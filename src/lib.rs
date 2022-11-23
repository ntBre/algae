#![feature(iter_collect_into, let_chains)]

pub mod config {
    use std::{
        io::{stderr, stdout, Stderr, Stdout, Write},
        time::{self, Duration, SystemTime, UNIX_EPOCH},
    };

    #[allow(unused)]
    pub struct Config<O, E>
    where
        O: Write,
        E: Write,
    {
        prompt: String,
        output: O,
        error: E,
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

    impl<O: Write, E: Write> Config<O, E> {
        pub fn input_base(&self) -> usize {
            self.input_base
        }

        pub(crate) fn output(&self) -> &O {
            &self.output
        }
    }

    impl Default for Config<Stdout, Stderr> {
        fn default() -> Self {
            Self {
                prompt: String::new(),
                output: stdout(),
                error: stderr(),
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
pub mod exec;
pub mod parse;
pub mod run;
pub mod scan;
pub mod value;
