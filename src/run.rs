use std::{fmt::Debug, io::Read, sync::RwLock};

use crate::{
    config::Config, exec::context::Context, parse::Parser, value::Value,
};

#[derive(Debug)]
pub struct RunError;

/// runs the parser/evaluator until EOF or error. The return value says whether
/// we completed without error. If the return value is true, it means we ran out
/// of data (EOF) and the run was successful. Typical execution is therefore to
/// loop calling Run until it succeeds. Error details are reported to the
/// configured error output stream.
impl<'a, R: Read + Debug> Parser<'a, R> {
    pub fn run(
        &mut self,
        conf: &Config,
        context: &'a RwLock<Context<'a>>,
        interactive: bool,
    ) -> Result<(), RunError> {
        if interactive {
            print!("{}", conf.prompt());
        }
        let Ok(exprs) = self.line() else {
        return Ok(());
    };
        let values = if !exprs.is_empty() {
            // TODO match interactive and time it if true
            context.read().unwrap().eval(exprs)
        } else {
            Vec::new()
        };
        if print_values(conf, &values) {
            // safe to unwrap because print_values checks that we have at least
            // one
            context
                .write()
                .unwrap()
                .assign_global("_", values.last().unwrap().clone());
        }
        if interactive {
            println!();
        }
        Err(RunError)
    }
}

/// neatly prints the values returned from execution, followed by a newline
fn print_values(_conf: &Config, values: &Vec<Value>) -> bool {
    if values.is_empty() {
        return false;
    }
    let mut printed = false;
    for v in values {
        // TODO filter out printing assignments
        // TODO handle formatting based on config
        let s = format!("{}", v);
        if printed && !s.is_empty() && !s.ends_with('\n') {
            print!(" ");
        }
        print!("{}", s);
        printed = true;
    }
    if printed {
        println!();
    }
    printed
}
