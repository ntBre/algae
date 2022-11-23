use std::{collections::HashMap, str::FromStr};

use crate::{
    config::Config,
    value::{
        context::{BinaryOp, Expr, UnaryOp},
        eval::{binary::BinaryBuiltin, product, reduce, scan, unary},
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

impl OpDef {
    fn new(name: String, is_binary: bool) -> Self {
        Self { name, is_binary }
    }
}

/// Context holds execution context, specifically the binding of names to values
/// and operators.
pub struct Context<'a> {
    /// config is the configuration state used for evaluation, printing, etc.
    /// Accessed through the [config] method.
    config: &'a Config,

    /// size of each stack frame on the call stack
    frame_sizes: Vec<usize>,
    stack: Vec<Value>,
    globals: Symtab,

    ///  `unary_fn` maps the names of unary functions (ops) to their
    ///  implemenations.
    unary_fn: HashMap<String, Function<'a>>,

    ///  `binary_fn` maps the names of binary functions (ops) to their
    ///  implemenations.
    binary_fn: HashMap<String, Function<'a>>,

    /// defs is a list of defined ops, in time order. it is used when saving the
    /// `Context` to a file.
    defs: Vec<OpDef>,

    /// names of variables declared in the currently-being-parsed function
    variables: Vec<String>,
}

impl<'a> Context<'a> {
    /// returns a new execution context: the stack and variables, plus the
    /// execution configuration.
    pub fn new(config: &'a Config) -> Self {
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

    pub fn config(&self) -> &Config {
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
    #[allow(unused)]
    fn push(&mut self, fun: Function<'a>) {
        let n = self.stack.len();
        let lfun = fun.locals.len();
        self.frame_sizes.push(lfun);
        self.stack.resize_with(n + lfun, Value::default);
    }

    /// pop pops the top frame from the stack
    #[allow(unused)]
    fn pop(&mut self) {
        self.frame_sizes.pop();
        self.stack.pop();
    }

    /// eval evaluates a list of expressions
    pub fn eval(&self, exprs: Vec<&dyn Expr>) -> Vec<Value> {
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
        let Some(fun) = self.unary(&op) else {
	    panic!("unary `{op}` not implemented")
	};
        fun.eval_unary(self, right)
    }

    /// return the `UnaryOp` represented by `op`
    pub fn unary(&'a self, op: &str) -> Option<Box<dyn UnaryOp<'a> + 'a>> {
        if let Some(user_fun) = self.unary_fn.get(op) {
            return Some(Box::new(user_fun));
        }
        if let Ok(builtin) = unary::UnaryBuiltin::from_str(op) {
            return Some(Box::new(builtin));
        }
        None
    }

    /// check if op has been defined by the user
    pub fn user_defined(&self, op: &str, is_binary: bool) -> bool {
        if is_binary {
            self.binary_fn.get(op).is_some()
        } else {
            self.unary_fn.get(op).is_some()
        }
    }

    pub fn eval_binary(&self, left: Value, op: String, right: Value) -> Value {
        if op.contains('.') {
            return product(self, left, &op, right);
        }
        let Some(fun) = self.binary(&op) else {
	    panic!("binary `{op}` not implemented");
	};
        fun.eval_binary(self, left, right)
    }

    pub fn binary(&'a self, op: &str) -> Option<Box<dyn BinaryOp<'a> + 'a>> {
        if let Some(user_fun) = self.binary_fn.get(op) {
            return Some(Box::new(user_fun));
        }
        if let Ok(builtin) = BinaryBuiltin::from_str(op) {
            return Some(Box::new(builtin));
        }
        None
    }

    /// Define defines the function and installs it. It also performs some error
    /// checking and adds the function to the sequencing information used by the
    /// save method.
    pub fn define(&mut self, fun: Function<'a>) {
        let name = fun.name();
        let nname = name.to_owned();
        self.no_var(name);
        let fib = fun.is_binary;
        if fun.is_binary {
            self.binary_fn.insert(name.to_owned(), fun);
        } else {
            self.unary_fn.insert(name.to_owned(), fun);
        }

        // update the sequence of definitions. first, if it's last (a very
        // common case) there's nothing to do
        if !self.defs.is_empty() {
            let last = self.defs.last().unwrap();
            if last.name == nname && last.is_binary == fib {
                return;
            }
        }

        // is it already defined? drop the existing value if so
        let i = self
            .defs
            .iter()
            .position(|def| def.name == nname && def.is_binary == fib);
        if let Some(i) = i {
            self.defs.remove(i);
        }

        self.defs.push(OpDef::new(nname, fib))
    }

    /// guarantees that there is no global variable with that name, preventing
    /// an op from being defined with the same name as a variable, which could
    /// cause problems. A variable with value zero is considered to be OK, so
    /// one can clear a variable before defining a symbol. A cleared variable is
    /// removed from the global symbol table. `no_var` also prevents defining
    /// builtin variables as ops.
    fn no_var(&mut self, name: &str) {
        // cannot redefine these
        if name == "_" || name == "pi" || name == "e" {
            panic!("can't define op with name `{name}`");
        }
        if let Some(sym) = self.globals.get(name) {
            if let Value::Int(i) = sym {
                if *i == 0 {
                    self.globals.remove(name);
                }
            }
        } else {
            return;
        }
        panic!(
            "cannot define op `{name}`; it is a variable \
		({name} = 0 to clear)"
        );
    }

    /// `no_op` is the dual of noVar. It also checks for assignment to builtins.
    /// It just errors out if there is a conflict.
    #[allow(unused)]
    fn no_op(&mut self, name: &str) {
        if name == "pi" || name == "e" {
            panic!("can't reassign `{name}`");
        }
        if self.unary_fn.contains_key(name) || self.binary_fn.contains_key(name)
        {
            panic!("cannot define variable `{name}`, it is an op");
        }
    }

    /// `declare` makes the name a variable while parsing the next function.
    pub fn declare(&mut self, name: &str) {
        self.variables.push(name.to_owned());
    }

    /// `forget_all` forgets the declared variables
    pub fn forget_all(&mut self) {
        self.variables.clear();
    }

    /// check if `op` is defined as a variable
    #[allow(unused)]
    fn is_variable(&self, op: String) -> bool {
        self.variables.iter().any(|var| *var == op)
    }
}
