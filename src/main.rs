use std::sync::RwLock;

use algae::{
    config::Config, exec::context::Context, parse::Parser, scan::Scanner,
};

fn main() {
    // TODO take config options from flags. make `new` take the same options as
    // flags
    let conf = Config::default();
    let context = RwLock::new(Context::new(&conf));
    let mut args = std::env::args();
    if let Some(infile) = args.nth(1) {
        let f = std::fs::File::open(infile).expect("failed to open file");
        let scanner = Scanner::new(&context, "file", Box::new(f));
        let mut parser = Parser::new("<stdin>", scanner, &context);
        while parser.run(&conf, &context, false).is_err() {}
    } else {
        // interactive
        let scanner = Scanner::new(&context, "<stdin>", std::io::stdin());
        let mut parser = Parser::new("<stdin>", scanner, &context);
        while parser.run(&conf, &context, true).is_err() {}
    }
}
