use std::{cell::RefCell, collections::HashMap, fmt::Debug, io::Read, rc::Rc};

use crate::{
    exec::{context::Context, function::Function},
    scan::{Scanner, Token, Type},
    value::context::Expr,
};

#[allow(unused)]
pub struct Parser<'a, R: Read> {
    scanner: Scanner<'a, R>,
    tokens: Vec<Token>,
    token_buf: [Token; 100],
    filename: String,
    line_num: usize,
    // why do we take this? surely it can't be a different context than the one
    // in our scanner?
    context: Rc<RefCell<Context<'a>>>,
}

#[derive(Debug)]
pub struct ParseError;

macro_rules! errorf {
    ($parser: ident, $($args:tt)*) => {
	$parser.tokens.clear();
	panic!($($args)*);
    }
}

impl<'a, R: Read + Debug> Parser<'a, R> {
    pub fn new(
        filename: &str,
        scanner: Scanner<'a, R>,
        context: Rc<RefCell<Context<'a>>>,
    ) -> Self {
        Self {
            scanner,
            tokens: Vec::new(),
            token_buf: std::array::from_fn(|_| Token::default()),
            filename: filename.to_owned(),
            line_num: 0,
            context,
        }
    }

    /// Line reads a line of input and returns the values it evaluates. An empty
    /// returned Vec means there were no values. An Error means the line was
    /// invalid.
    ///
    /// Line
    ///
    /// ) special command '\n'
    /// def function defintion
    /// expressionList '\n'
    pub fn line(&mut self) -> Result<Vec<Expr>, ParseError> {
        if !self.read_tokens_to_newline() {
            return Err(ParseError);
        }
        let exprs = Vec::new();
        let tok = self.peek();
        match tok.typ {
            Type::Eof => Ok(exprs),
            Type::RightParen => {
                self.special();
                self.context.borrow_mut().set_constants();
                Ok(exprs)
            }
            Type::Op => {
                self.function_defn();
                Ok(exprs)
            }
            _ => self.expression_list(),
        }
    }

    fn peek(&self) -> Token {
        if self.tokens.is_empty() {
            return Token::new(Type::Eof, 0, String::new());
        }
        self.tokens[0].clone()
    }

    fn read_tokens_to_newline(&mut self) -> bool {
        self.tokens.clear();
        loop {
            let tok = self.scanner.next_token();
            match tok.typ {
                Type::Eof => return !self.tokens.is_empty(),
                Type::Error => {
                    errorf!(self, "{}", tok);
                }
                Type::Newline => return true,
                _ => {}
            }
            self.tokens.push(tok.clone());
        }
    }

    fn special(&mut self) {
        self.need(Type::RightParen);
        // TODO should be changing base here
    }

    fn expression_list(&self) -> Result<Vec<Expr>, ParseError> {
        todo!()
    }

    fn need(&mut self, typ: Type) -> Token {
        let tok = self.next();
        // TODO take multiple typ and loop over it
        if tok.typ == typ {
            return tok;
        }
        errorf!(self, "{}", tok);
    }

    fn next(&mut self) -> Token {
        let tok = self.peek();
        if tok.typ != Type::Eof {
            // go code says self.tokens[1..], not sure if it's better to call
            // to_vec after or remove the front element
            self.tokens.remove(0);
            self.line_num = tok.line;
        }
        if tok.typ == Type::Error {
            errorf!(self, "{}", tok);
        }
        tok
    }

    #[allow(unused)]
    fn function_defn(&mut self) {
        self.need(Type::Op);
        let mut fun = Function::default();
        // two identifiers means op arg
        // three identifiers means arg op arg
        let mut idents = vec![
            self.need(Type::Identifier).text,
            self.need(Type::Identifier).text,
        ];
        if self.peek().typ.is_identifier() {
            idents.push(self.next().text);
        }
        let tok = self.next();
        let mut install_map = HashMap::new();
        if idents.len() == 3 {
            if idents[1] == "o" {
                errorf!(self, "o is not a valid name for a binary operator");
            }
            fun.is_binary = true;
            fun.left = idents[0].clone();
            fun.name = idents[1].clone();
            fun.right = idents[2].clone();
            self.context.borrow_mut().declare(&fun.left);
            self.context.borrow_mut().declare(&fun.right);
            install_map = self.context.borrow().binary_fn.clone();
        } else {
            fun.name = idents[0].clone();
            fun.right = idents[1].clone();
            self.context.borrow_mut().declare(&fun.right);
            install_map = self.context.borrow().unary_fn.clone();
        }
        if fun.name == fun.left || fun.name == fun.right {
            errorf!(self, "argument name `{}` is function name", fun.name);
        }
        // define it but prepare to undefine if there's trouble
        let name = fun.name.clone();
        self.context.borrow_mut().define(fun);
        let mut succeeded = false;
        let prev_defn = install_map.get(&name);
    }
}
