use std::{collections::HashMap, io::Write};

use crate::{
    config::Config,
    value::{
        context::{Expr, UnaryOp},
        eval::{reduce, scan},
        unary::unary_ops,
        Value,
    },
};

use super::function::Function;

/// Symtab is a symbol table, a map of names to values.
type Symtab = HashMap<String, Value>;

struct OpDef {
    name: String,
    is_binary: bool,
}

/// Context holds execution context, specifically the binding of names to values
/// and operators.
pub struct Context<'a, O: Write, E: Write> {
    /// config is the configuration state used for evaluation, printing, etc.
    /// Accessed through the [config] method.
    config: &'a Config<O, E>,

    /// size of each stack frame on the call stack
    frame_sizes: Vec<usize>,
    stack: Vec<Value>,
    globals: Symtab,

    ///  `unary_fn` maps the names of unary functions (ops) to their
    ///  implemenations.
    unary_fn: HashMap<String, Function<O, E>>,

    ///  `binary_fn` maps the names of binary functions (ops) to their
    ///  implemenations.
    binary_fn: HashMap<String, Function<O, E>>,

    /// defs is a list of defined ops, in time order. it is used when saving the
    /// `Context` to a file.
    defs: Vec<OpDef>,

    /// names of variables declared in the currently-being-parsed function
    variables: Vec<String>,
}

impl<'a, O: Write, E: Write> Context<'a, O, E> {
    /// returns a new execution context: the stack and variables, plus the
    /// execution configuration.
    pub fn new(config: &'a Config<O, E>) -> Self {
        Self {
            config,
            frame_sizes: Vec::new(),
            stack: Vec::new(),
            globals: HashMap::new(),
            unary_fn: HashMap::new(),
            binary_fn: HashMap::new(),
            defs: Vec::new(),
            variables: Vec::new(),
        }
    }

    pub fn config(&self) -> &Config<O, E> {
        self.config
    }

    /// re-assigns the fundamental constant values
    pub fn set_constants(&mut self) {
        self.assign_global("e", Value::Float(std::f64::consts::E));
        self.assign_global("pi", Value::Float(std::f64::consts::PI));
    }

    /// returns the value of a global symbol, or None if the symbol is not
    /// defined globally
    pub fn global(&self, name: String) -> Option<&Value> {
        self.globals.get(&name)
    }

    /// returns the value of the local variable with index i
    pub fn local(&self, i: usize) -> &Value {
        let l = self.stack.len();
        &self.stack[l - i]
    }

    /// assigns the local variable with the given index the value.
    pub fn assign_local(&mut self, i: usize, value: Value) {
        let l = self.stack.len();
        self.stack[l - i] = value;
    }

    /// assigns the global variable the value. The variable must be defined
    /// either in the current function or globally. Inside a function, new
    /// variables become locals.
    fn assign_global(&mut self, name: &str, value: Value) {
        self.globals.insert(name.to_owned(), value);
    }

    /// push pushes a new local frame onto the context stack
    fn push(&mut self, fun: Function<O, E>) {
        let n = self.stack.len();
        let lfun = fun.locals.len();
        self.frame_sizes.push(lfun);
        self.stack.resize_with(n + lfun, Value::default);
    }

    /// pop pops the top frame from the stack
    fn pop(&mut self) {
        self.frame_sizes.pop();
        self.stack.pop();
    }

    /// eval evaluates a list of expressions
    pub fn eval(&self, exprs: Vec<&dyn Expr<O, E>>) -> Vec<Value> {
        exprs.iter().flat_map(|e| e.eval(self)).collect()
    }

    pub fn eval_unary(&self, op: String, right: Value) -> Value {
        let l = op.len();
        if l > 1 {
            let opi = op.chars().last().unwrap();
            match opi {
                '/' => return reduce(self, &op[..l - 1], right),
                '\\' => return scan(self, &op[..l - 1], right),
                _ => {}
            }
        }
        let fun = self.unary(op);
	fun.unwrap().eval_unary(self, right)
    }

    pub fn unary(&self, op: String) -> Option<&dyn UnaryOp<O, E>> {
        if let Some(user_fun) = self.unary_fn.get(&op) {
            return Some(user_fun);
        }
        if let Some(builtin) = unary_ops(&op) {
            return Some(builtin);
        }
        None
    }
}
