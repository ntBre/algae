use algae::{
    config::Config, exec::context::Context, parse::Parser, run::run,
    scan::Scanner,
};

fn main() {
    // TODO take config options from flags. make `new` take the same options as
    // flags
    let conf = Config::default();
    let context = Context::new(&conf);
    let scanner = Scanner::new(&context, "<stdin>", std::io::stdin());
    let parser = Parser::new("<stdin>", &scanner, &context);
    while run(&parser, &context, true).is_err() {}
}
