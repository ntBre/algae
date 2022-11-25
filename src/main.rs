use std::{cell::RefCell, rc::Rc};

use algae::{
    config::Config, exec::context::Context, parse::Parser, scan::Scanner,
};

fn main() {
    // TODO take config options from flags. make `new` take the same options as
    // flags
    let conf = Config::default();
    let context = Rc::new(RefCell::new(Context::new(&conf)));
    let scanner = Scanner::new(context.clone(), "<stdin>", std::io::stdin());
    let mut parser = Parser::new("<stdin>", scanner, context.clone());
    while parser.run(context.clone(), true).is_err() {}
}
