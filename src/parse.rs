use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    io::Read,
    sync::RwLock,
};

use crate::{
    exec::{context::Context, function::Function},
    scan::{Scanner, Token, Type},
    value::{context::expr::Expr, parse, parse_string, Value},
};

#[allow(unused)]
pub struct Parser<'a, R: Read> {
    scanner: Scanner<'a, R>,
    tokens: Vec<Token>,
    token_buf: [Token; 100],
    filename: String,
    line_num: usize,
    context: &'a RwLock<Context<'a>>,
}

#[derive(Debug, Clone)]
pub struct ParseError;

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

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
        context: &'a RwLock<Context<'a>>,
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
                self.context.write().unwrap().set_constants();
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
                    errorf!(self, "{:#?}", tok);
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

    /// expressionList:
    /// statementList <eol>
    fn expression_list(&mut self) -> Result<Vec<Expr>, ParseError> {
        let exprs = self.statement_list()?;
        let tok = self.next();
        if !tok.typ.is_eof() {
            errorf!(self, "unexpected {tok}");
        }
        // TODO debugging tree print
        Ok(exprs)
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
            self.context.write().unwrap().declare(&fun.left);
            self.context.write().unwrap().declare(&fun.right);
            install_map = self.context.read().unwrap().binary_fn.clone();
        } else {
            fun.name = idents[0].clone();
            fun.right = idents[1].clone();
            self.context.write().unwrap().declare(&fun.right);
            install_map = self.context.read().unwrap().unary_fn.clone();
        }
        if fun.name == fun.left || fun.name == fun.right {
            errorf!(self, "argument name `{}` is function name", fun.name);
        }
        // define it but prepare to undefine if there's trouble
        let name = fun.name.clone();
        self.context.write().unwrap().define(fun);
        let mut succeeded = false;
        let prev_defn = install_map.get(&name);
    }

    /// statementList:
    ///    expr [':' expr] [';' statementList]
    fn statement_list(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut expr = self.expr();
        if !expr.is_nil() && self.peek().typ == Type::Colon {
            let tok = self.next();
            expr = Expr::conditional(tok.text, expr, self.expr());
        }
        let mut exprs = Vec::new();
        if !expr.is_nil() {
            exprs.push(expr);
        }
        if self.peek().typ == Type::Semicolon {
            self.next();
            if let Ok(more) = self.statement_list() {
                exprs.extend(more);
            }
        }
        Ok(exprs)
    }

    /// expr
    ///    operand
    ///    operand binop expr
    fn expr(&mut self) -> Expr {
        let mut tok = self.next();
        let expr = self.operand(tok, true);
        tok = self.peek();
        use Type::*;
        match tok.typ {
            Eof | RightParen | RightBrack | Semicolon | Colon => return expr,
            Identifier => {
                if self.context.read().unwrap().defined_binary(&tok.text) {
                    self.next();
                    return Expr::binary(tok.text, expr, self.expr());
                }
            }
            Assign => {
                self.next();
                match expr {
                    Expr::VariableExpr { .. } | Expr::Index { .. } => {
                        return Expr::binary(tok.text, expr, self.expr());
                    }
                    _ => {
                        errorf!(self, "cannot assign to {:#?}", expr);
                    }
                }
            }
            Operator => {
                self.next();
                return Expr::binary(tok.text, expr, self.expr());
            }
            _ => {}
        }
        errorf!(self, "after expression: unexpected {}", self.peek());
    }

    /// operand
    ///    number
    ///    char constant
    ///    string constant
    ///    vector
    ///    operand [ Expr ]...
    ///    unop Expr
    fn operand(&mut self, tok: Token, index_ok: bool) -> Expr {
        use Type::*;
        let mut expr = match tok.typ {
            Operator => Expr::unary(tok.text, self.expr()),
            Identifier => {
                if self.context.read().unwrap().defined_unary(&tok.text) {
                    Expr::unary(tok.text, self.expr())
                } else {
                    self.number_or_vector(tok)
                }
            }
            Number | Rational | Complex | String | LeftParen => {
                self.number_or_vector(tok)
            }
            _ => {
                errorf!(self, "unexpected {tok}");
            }
        };
        if index_ok {
            expr = self.index(expr);
        }
        expr
    }

    // numberOrVector turns the token and what follows into a numeric Value,
    // possibly a vector.
    //
    // numberOrVector
    //	number
    //	string
    //	numberOrVector...
    pub(crate) fn number_or_vector(&mut self, tok: Token) -> Expr {
        let (mut expr, mut s) = self.number(tok);
        use Type::*;
        let done = !matches!(
            self.peek().typ,
            Number | Rational | Complex | String | Identifier | LeftParen
        );
        let mut slice = Vec::new();
        if expr.is_nil() {
            slice.extend(eval_string(s));
        } else {
            slice = vec![expr];
        }
        if !done {
            loop {
                let tok = self.peek();
                match tok.typ {
                    LeftParen | Identifier => {
                        if self.context.read().unwrap().defined_op(&tok.text) {
                            break;
                        }
                        let n = self.next();
                        (expr, s) = self.number(n);
                        if expr.is_nil() {
                            // must be a string
                            slice.extend(eval_string(s));
                            continue;
                        }
                    }
                    _ => break,
                }
                slice.push(expr);
            }
        }
        if slice.len() == 1 {
            return slice[0].clone();
        }
        Expr::SliceExpr { exprs: slice }
    }

    // index
    //
    //	expr
    //	expr [ expr ]
    //	expr [ expr ] [ expr ] ....
    pub(crate) fn index(&mut self, mut expr: Expr) -> Expr {
        while self.peek().typ == Type::LeftBrack {
            self.next();
            let list = self.index_list();
            let tok = self.next();
            if tok.typ != Type::RightBrack {
                errorf!(self, "expected right bracket, found {tok}");
            }
            expr = Expr::index(String::new(), expr, list);
        }
        expr
    }

    // indexList
    //	[[expr] [';' [expr]] ...]
    fn index_list(&mut self) -> Vec<Expr> {
        let mut list = Vec::new();
        // previous element contained an expression
        let mut seen = false;
        loop {
            let tok = self.peek();
            use Type::*;
            match tok.typ {
                RightBrack => {
                    if !seen {
                        list.push(Expr::Nil);
                    }
                    return list;
                }
                Semicolon => {
                    self.next();
                    if !seen {
                        list.push(Expr::Nil);
                    }
                    seen = false;
                }
                _ => {
                    list.push(self.expr());
                    seen = true;
                }
            }
        }
    }

    // number
    //	integer
    //	rational
    //	string
    //	variable
    //	'(' Expr ')'
    // If the value is a string, value.Expr is nil.
    pub(crate) fn number(&mut self, tok: Token) -> (Expr, String) {
        let text = tok.text;
        let (expr, s) = match tok.typ {
            Type::Identifier => (self.variable(text), String::new()),
            Type::String => (Expr::Nil, parse_string(text)),
            Type::Number | Type::Rational | Type::Complex => {
                match parse(self.context.read().unwrap().config(), &text) {
                    Ok(v) => (v.into(), String::new()),
                    Err(e) => {
                        errorf!(self, "{text}: {:#?}", e);
                    }
                }
            }
            Type::LeftParen => {
                let expr = self.expr();
                let tok = self.next();
                if tok.typ != Type::RightParen {
                    errorf!(self, "expected right paren, found {tok}");
                }
                (expr, String::new())
            }
            _ => (Expr::Nil, String::new()),
        };
        (expr, s)
    }

    fn variable(&self, _text: String) -> Expr {
        todo!()
    }
}

fn eval_string(s: String) -> Vec<Expr> {
    s.chars().map(|c| Expr::Value(Value::Char(c))).collect()
}
