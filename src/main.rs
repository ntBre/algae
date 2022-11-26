use std::sync::RwLock;

use algae::{
    config::Config, exec::context::Context, parse::Parser, scan::Scanner,
};

fn main() {
    // TODO take config options from flags. make `new` take the same options as
    // flags
    let conf = Config::default();
    let context = RwLock::new(Context::new(&conf));
    let scanner = Scanner::new(&context, "<stdin>", std::io::stdin());
    let mut parser = Parser::new("<stdin>", scanner, &context);
    while parser.run(&conf, &context, true).is_err() {}
}
