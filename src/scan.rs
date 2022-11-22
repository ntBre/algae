use std::{fmt::Display, io::Read};

use crate::exec::{context::Context, predefined};

/// identifies the type of lex items
#[derive(Clone, Copy, Debug, Default)]
pub enum Type {
    #[default]
    Eof,
    Error,
    Newline,
    Assign,
    Char,
    Identifier,
    LeftBrack,
    LeftParen,
    Number,
    Operator,
    Op,
    Rational,
    Complex,
    RightBrack,
    RightParen,
    Semicolon,
    String,
    Colon,
}

impl Type {
    /// Returns `true` if the type is [`Newline`].
    ///
    /// [`Newline`]: Type::Newline
    #[must_use]
    #[allow(unused)]
    fn is_newline(&self) -> bool {
        matches!(self, Self::Newline)
    }
}

/// a token or text string returned from the scanner
#[derive(Clone, Debug, Default)]
pub struct Token {
    pub typ: Type,
    pub line: usize,
    pub text: String,
}

impl Token {
    fn new(typ: Type, line: usize, text: String) -> Self {
        Self { typ, line, text }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.typ {
            Type::Eof => write!(f, "EOF"),
            Type::Error => write!(f, "error: {}", self.text),
            t => {
                if self.text.len() > 10 {
                    write!(f, "{:?}: {:.10}...", t, self.text)
                } else {
                    write!(f, "{:?}: {}", t, self.text)
                }
            }
        }
    }
}

#[allow(unused)]
pub struct Scanner<'a, R: Read> {
    context: Context<'a>,
    r: R,
    done: bool,
    name: String,
    buf: Vec<u8>,
    input: String,
    last_char: Option<u8>,
    last_width: usize,
    read_ok: bool,
    line: usize,

    /// current position in the input
    pos: usize,

    /// start position of this item
    start: usize,
    token: Token,
}

impl<'a, R: Read> Scanner<'a, R> {
    #[allow(unused)]
    fn new(context: Context<'a>, name: String, r: R) -> Self {
        Self {
            context,
            r,
            name,
            done: false,
            buf: Vec::new(),
            input: String::new(),
            last_char: None,
            last_width: 0,
            read_ok: false,
            line: 0,
            pos: 0,
            start: 0,
            token: Token::default(),
        }
    }

    // TODO looks like reading to newline, which we could probably do with
    // lines()
    fn load_line(&mut self) {
        self.buf.clear();
        for c in (&mut self.r).bytes() {
            let Ok(c) = c else {
		self.done = true;
		break;
	    };
            if c != b'\r' {
                self.buf.push(c);
            }
            if c == b'\n' {
                break;
            }
        }
        if self.start == self.pos {
            self.input = String::from_utf8(self.buf.clone()).unwrap();
            self.start = 0;
            self.pos = 0;
        } else {
            self.input
                .push_str(&String::from_utf8(self.buf.clone()).unwrap())
        }
    }

    /// None indicates EOF
    fn read_rune(&mut self) -> (Option<u8>, usize) {
        if !self.done && self.pos == self.input.len() {
            if !self.read_ok {
                self.errorf("incomplete token");
                return (Some(b'\n'), 1);
            }
            self.load_line();
        }
        if self.input.len() == self.pos {
            return (None, 0);
        }
        (Some(self.input.as_bytes()[self.pos]), 1)
    }

    fn next_inner(&mut self) -> Option<u8> {
        let (c, w) = self.read_rune();
        self.pos += w;
        c
    }

    fn peek(&mut self) -> Option<u8> {
        let (c, _) = self.read_rune();
        c
    }

    /// return the next two runes without consuming anything
    fn peek2(&mut self) -> (Option<u8>, Option<u8>) {
        let pos = self.pos;
        let c1 = self.next_inner();
        let c2 = self.next_inner();
        self.pos = pos;
        (c1, c2)
    }

    #[allow(unused)]
    fn backup(&mut self) {
        if self.last_char.is_none() {
            return;
        }
        if self.pos == self.start {
            self.errorf("internal error: backup at start of input");
        }
        if self.pos > self.start {
            todo!("can't happen?");
            // if it can happen, this is the code
            // self.pos -= self.last_width;
        }
    }

    #[allow(unused)]
    fn emit(&mut self, t: Type) -> Lex {
        if t.is_newline() {
            self.line += 1;
        }
        let text = &self.input[self.start..self.pos];
        self.token = Token::new(t, self.line, text.to_owned());
        self.start = self.pos;
        Lex::None
    }

    #[allow(unused)]
    fn accept(&mut self, valid: String) -> bool {
        if let Some(c) = self.next_inner() {
            if valid.contains(char::from(c)) {
                return true;
            }
        }
        self.backup();
        false
    }

    /// consumes a run of runes from the valid set
    #[allow(unused)]
    fn accept_run(&mut self, valid: String) {
        while let Some(c) = self.next_inner() {
            if !valid.contains(char::from(c)) {
                break;
            }
        }
        self.backup()
    }

    fn errorf(&mut self, arg: &str) -> Lex {
        self.token = Token::new(Type::Error, self.start, arg.to_owned());
        self.start = 0;
        self.pos = 0;
        self.input.clear();
        Lex::None
    }

    #[allow(unused)]
    fn next(&mut self) -> &Token {
        self.read_ok = true;
        self.last_char = None;
        self.last_width = 0;
        self.token = Token::new(Type::Eof, self.pos, String::from("EOF"));
        let mut state = Lex::Any;
        loop {
            state = state.run(self);
            if state.is_none() {
                return &self.token;
            }
        }
    }

    #[allow(unused)]
    fn is_numeral(&self, r: u8) -> bool {
        #[allow(unused)]
        if (b'0'..=b'9').contains(&r) {
            return true;
        }
        let base = self.context.config().input_base();
        if base < 10 {
            return false;
        }
        let top = base - 10;
        if b'a' <= r && r <= b'a' + top as u8 {
            return true;
        }
        if b'A' <= r && r <= b'A' + top as u8 {
            return true;
        }
        false
    }

    #[allow(unused)]
    fn is_operator(&mut self, r: u8) -> bool {
        match r {
            b'?' | b'+' | b'-' | b'/' | b'%' | b'&' | b'|' | b'^' | b',' => {}
            b'!' => {
                if let Some(p) = self.peek() {
                    if p == b'=' {
                        self.next();
                    }
                }
            }
            b'>' => {
                if let Some(p) = self.peek() {
                    if p == b'>' || p == b'=' {
                        self.next();
                    }
                }
            }
            b'<' => {
                if let Some(p) = self.peek() {
                    if [b'<', b'='].contains(&p) {
                        self.next();
                    }
                }
            }
            b'*' => {
                if let Some(p) = self.peek() {
                    if p == b'*' {
                        self.next();
                    }
                }
            }
            b'=' => {
                if let Some(p) = self.peek() {
                    if p != b'=' {
                        return false;
                    }
                }
                self.next();
            }
            _ => return false,
        }
        true
    }

    #[allow(unused)]
    fn at_terminator(&mut self) -> bool {
        let Some(r) = self.peek() else {
	    return true;
	};
        // TODO supposed to check unicode.is_symbol too
        if is_space(r) || is_end_of_line(r) || r.is_ascii_punctuation() {
            return true;
        }
        if self.pos < self.input.len() {
            let (r1, r2) = self.peek2();
            if let Some(r1) = r1 {
                if let Some(r2) = r2 {
                    if r1 == b'o' && r2 == b'.' {
                        return true;
                    }
                }
            }
        }
        false
    }

    #[allow(unused)]
    fn defined(&self, word: &str) -> bool {
        predefined(word) || self.context.user_defined(word, true)
    }
}
#[allow(unused)]
fn is_alpha_numeric(r: u8) -> bool {
    r == b'_' || r.is_ascii_alphabetic() || r.is_ascii_digit()
}

#[allow(unused)]
enum Lex {
    Any,
    Comment,
    Space,
    Quote,
    RawQuote,
    Operator,
    Complex,
    Identifier,
    None,
}

impl Lex {
    #[allow(unused)]
    fn run<R: Read>(self, l: &mut Scanner<R>) -> Self {
        match self {
            Lex::Comment => todo!(),
            Lex::Any => {
                let Some(r) = l.next_inner() else {
		    return Self::None
		};
                match r {
                    b'\n' => return l.emit(Type::Newline),
                    b';' => return l.emit(Type::Semicolon),
                    b'#' => return Self::Comment,
                    b' ' | b'\t' => return Self::Space,
                    b'\'' | b'"' => {
                        // backup so lex can read the quote
                        l.backup();
                        return Self::Quote;
                    }
                    b'`' => return Self::RawQuote,
                    b'-' | b'+' => {
                        if l.start > 0 {
                            let rr = l.input.bytes().last().unwrap();
                            if rr.is_ascii_alphanumeric()
                                || rr == b')'
                                || rr == b']'
                            {
                                return Self::Operator;
                            }
                            let (r1, r2) = l.peek2();
                            if let Some(r1) = r1 {
                                if let Some(r2) = r2 {
                                    if r1 == b'.' && !l.is_numeral(r2) {
                                        return Self::Operator;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                };
                self.fallthrough(l, r)
            }
            Lex::Space => {
                while let Some(c) = l.peek() {
                    if !is_space(c) {
                        break;
                    }
                    l.next();
                }
                l.start = l.pos;
                Self::Any
            }
            Lex::Identifier => {
                while let Some(c) = l.peek() {
                    if !is_alpha_numeric(c) {
                        break;
                    }
                    l.next();
                }
                if l.at_terminator() {
                    let e = format!("bad character {:?}", l.next());
                    return l.errorf(&e);
                }
                let word = &l.input[l.start..l.pos];
                if word == "op" {
                    return l.emit(Type::Op);
                } else if word == "o" {
                    if let Some(c) = l.peek() {
                        if c == b'.' {
                            return Self::Operator;
                        }
                    }
                } else if l.defined(word) {
                    return Self::Operator;
                } else if is_all_digits(word, l.context.config().input_base()) {
                    l.pos = l.start;
                    return Self::Complex;
                }
                l.emit(Type::Identifier)
            }
            Lex::Operator => todo!(),
            Lex::Complex => todo!(),
            Lex::Quote => todo!(),
            Lex::RawQuote => todo!(),
            Lex::None => Lex::None,
        }
    }

    #[allow(unused)]
    fn fallthrough<'a, R: Read>(&self, l: &mut Scanner<'a, R>, r: u8) -> Lex {
        if r == b'.' || (b'0'..=b'9').contains(&r) {
            l.backup();
            return Lex::Complex;
        }
        if r == b'=' {
            if let Some(c) = l.peek() {
                if c == b'=' {
                    return l.emit(Type::Assign);
                }
            }
            l.next();
        }
        self.fallthrough2(l, r)
    }

    #[allow(unused)]
    fn fallthrough2(&self, l: &mut Scanner<impl Read>, r: u8) -> Lex {
        if l.is_operator(r) {
            return Self::Operator;
        }
        if is_alpha_numeric(r) {
            l.backup();
            return Self::Identifier;
        }
        match r {
            b'[' => return l.emit(Type::LeftBrack),
            b':' => return l.emit(Type::Colon),
            b']' => return l.emit(Type::RightBrack),
            b'(' => return l.emit(Type::LeftParen),
            b')' => return l.emit(Type::RightParen),
            _ => {}
        }
        if r.is_ascii() {
            return l.emit(Type::Char);
        }
        l.errorf(&format!("unrecognized character {:?}", r))
    }

    /// Returns `true` if the lex is [`None`].
    ///
    /// [`None`]: Lex::None
    #[must_use]
    #[allow(unused)]
    fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

fn is_end_of_line(r: u8) -> bool {
    r == b'\n' || r == b';'
}

fn is_space(r: u8) -> bool {
    r == b' ' || r == b'\t'
}

#[allow(unused)]
fn is_all_digits(s: &str, base: usize) -> bool {
    let base = base as u8;
    let top = b'a' + base - 10 - 1;
    let ctop = b'A' + base - 10 - 1;
    let mut sawj = false;
    for c in s.bytes() {
        if c == b'j' && !sawj {
            sawj = true;
            continue;
        }
        if (b'0'..=b'9').contains(&c) {
            continue;
        }
        if (b'a'..=top).contains(&c) {
            continue;
        }
        if (b'A'..=ctop).contains(&c) {
            continue;
        }
        return false;
    }
    true
}
