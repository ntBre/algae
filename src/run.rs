use std::io::Read;

use crate::{exec::context::Context, parse::Parser};

pub struct RunError;

/// runs the parser/evaluator until EOF or error. The return value says whether
/// we completed without error. If the return value is true, it means we ran out
/// of data (EOF) and the run was successful. Typical execution is therefore to
/// loop calling Run until it succeeds. Error details are reported to the
/// configured error output stream.
pub fn run<'a>(
    _p: &Parser<impl Read>,
    _context: &'a Context,
    _interactive: bool,
) -> Result<(), RunError> {
    Err(RunError)
}
