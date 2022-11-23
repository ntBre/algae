use algae::{config::Config, exec::context::Context, scan::Scanner};

fn main() {
    // TODO take config options from flags. make `new` take the same options as
    // flags
    let conf = Config::default();
    let context = Context::new(&conf);
    let _scanner = Scanner::new(context, "<stdin>", std::io::stdin());
}
