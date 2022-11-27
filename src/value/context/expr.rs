use super::super::Value;

use crate::exec::context::Context;

#[derive(Clone, Debug)]
pub struct Binary {
    pub op: String,
    pub left: Expr,
    pub right: Expr,
}

#[derive(Clone, Debug)]
pub struct Unary {
    pub op: String,
    pub right: Expr,
}

#[derive(Clone, Debug)]
pub struct Index {
    pub op: String,
    pub left: Expr,
    pub right: Vec<Expr>,
}

#[derive(Clone, Debug)]
pub enum Expr {
    Conditional {
        binary: Box<Binary>,
    },

    Binary {
        binary: Box<Binary>,
    },

    /// `local` is the local index, 0 for global
    VariableExpr {
        name: String,
        local: usize,
    },

    Index {
        index: Box<Index>,
    },

    Unary {
        unary: Box<Unary>,
    },

    SliceExpr {
        exprs: Vec<Expr>,
    },

    Value(Value),

    Nil,
}

impl From<Value> for Expr {
    fn from(value: Value) -> Self {
        Self::Value(value)
    }
}

macro_rules! binary_holders {
    ($($fn_name: ident => $var_name: ident$(,)*)*) => {
	$(
	    pub fn $fn_name(op: String, left: Expr, right: Expr) -> Self {
		Self::$var_name {
		    binary: Box::new(Binary { op, left, right }),
		}
	    }
	)*
    }
}

impl Expr {
    binary_holders! {
    conditional => Conditional,
    binary => Binary,
    }

    pub fn unary(op: String, right: Expr) -> Self {
        Self::Unary {
            unary: Box::new(Unary { op, right }),
        }
    }

    pub fn index(op: String, left: Expr, right: Vec<Expr>) -> Self {
        Self::Index {
            index: Box::new(Index { op, left, right }),
        }
    }

    pub fn prog_string(&self) -> String {
        todo!();
    }

    #[allow(unused)]
    pub fn eval(&self, context: &Context) -> Value {
        match self {
            Expr::Conditional { binary } => todo!(),
            Expr::Binary { binary: b } => {
                if b.op == "=" {
                    todo!()
                }
                let rhs = b.right.eval(context);
                let lhs = b.left.eval(context);
                return context.eval_binary(lhs, &b.op, rhs);
            }
            Expr::VariableExpr { name, local } => todo!(),
            Expr::Index { index } => todo!(),
            Expr::Unary { unary: u } => {
                return context.eval_unary(&u.op, u.right.eval(context));
            }
            Expr::SliceExpr { exprs } => todo!(),
            Expr::Value(v) => return v.clone(),
            Expr::Nil => todo!(),
        }
        todo!();
    }

    /// Returns `true` if the expr is [`Nil`].
    ///
    /// [`Nil`]: Expr::Nil
    #[must_use]
    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }
}
