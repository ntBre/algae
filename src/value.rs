use num::{Complex, FromPrimitive, Rational64};
use std::{error::Error, fmt::Display};

use crate::{config::Config, parse::ParseError};

// might embed this as ValueType in Value struct that also contains is_assigment
// field. see parse/assign.go
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Value {
    Float(f64),
    Int(i64),
    Complex(Complex<f64>),
    Rational(Rational64),
    Char(char),
    #[default]
    None,
}

impl Value {
    fn complex(v1: Value, v2: Value) -> Self {
        if let Self::Int(v1) = v1 {
            if let Self::Int(v2) = v2 {
                return Self::Complex(Complex {
                    re: v1 as f64,
                    im: v2 as f64,
                });
            }
        }
        todo!()
    }
    fn rational(v1: Value, v2: Value) -> Self {
        if let Self::Int(v1) = v1 {
            if let Self::Int(v2) = v2 {
                return Self::Rational(Rational64::new(v1, v2));
            }
        }

        if let Self::Rational(v1) = v1 {
            if let Self::Int(v2) = v2 {
                return Self::Rational(v1 / Rational64::from(v2));
            }
        }

        if let Self::Int(v1) = v1 {
            if let Self::Rational(v2) = v2 {
                return Self::Rational(Rational64::from(v1) / v2);
            }
        }
        panic!("tried to make a rational from {v1:#?} and {v2:#?}");
    }

    /// Returns `true` if the value is [`Float`].
    ///
    /// [`Float`]: Value::Float
    #[must_use]
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(..))
    }

    /// Returns `true` if the value is [`Int`].
    ///
    /// [`Int`]: Value::Int
    #[must_use]
    pub fn is_int(&self) -> bool {
        matches!(self, Self::Int(..))
    }
}

impl Display for Value {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Float(f) => write!(w, "{:.8}", *f),
            Value::Int(d) => write!(w, "{d}"),
            Value::Complex(c) => write!(w, "{c}"),
            Value::Rational(r) => write!(w, "{r}"),
            Value::None => todo!(),
            Value::Char(c) => write!(w, "{c}"),
        }
    }
}

pub fn parse_string(text: String) -> String {
    unquote(text).unwrap_or_else(|_| panic!("invalid string syntax"))
}

/// unquote is a simplified strconv.Unquote that treats ' and " equally. Raw
/// quotes are Go-like and bounded by ``. The return value is the string and a
/// boolean rather than error, which was almost always the same anyway.
fn unquote(s: String) -> Result<String, ParseError> {
    let n = s.len();
    if n < 2 {
        return Err(ParseError);
    }
    let chars: Vec<_> = s.chars().collect();
    let quote = chars[0];
    if quote != chars[n - 1] {
        return Err(ParseError);
    }
    // NOTE Go version looks at bytes not chars, might need to use u8
    let chars = &chars[1..n - 1];
    if quote == '`' {
        if chars.contains(&'`') {
            return Err(ParseError);
        }
        return Ok(chars.iter().collect());
    }

    if quote != '"' && quote != '\'' {
        return Err(ParseError);
    }

    if s.contains('\n') {
        return Err(ParseError);
    }

    if !s.contains('\\') && !s.contains(quote) {
        return Ok(chars.iter().collect());
    }

    // TODO utf8 stuff
    Ok(chars.iter().collect())
}

pub fn parse(conf: &Config, s: &str) -> Result<Value, ParseError> {
    let (v1, v2, sep) = parse_two(conf, s)?;
    match sep {
        // a complex
        "j" => return Ok(Value::complex(v1, v2)),
        // a rational. NOTE: skipping "tricky" case of big nums
        "/" => return Ok(Value::rational(v1, v2)),
        _ => {}
    }
    // not a rational, but might be something like 1.3e-2, which could become a
    // rational.
    if let Ok(i) = set_int_string(conf, s) {
        return Ok(Value::Int(i));
    }
    if let Ok(r) = set_big_rat_from_float_string(s) {
        return Ok(r);
    }
    if let Ok(r) = s.parse::<f64>() {
        return Ok(Value::Float(r));
    }
    Err(ParseError)
}

fn set_big_rat_from_float_string(s: &str) -> Result<Value, Box<dyn Error>> {
    if !s.contains(['.', 'e', 'E']) {
        panic!("bad number syntax: {s}");
    }
    let f = s.parse::<f64>()?;
    let r = Rational64::from_f64(f).ok_or(ParseError)?;
    Ok(Value::rational(
        Value::Int(*r.numer()),
        Value::Int(*r.denom()),
    ))
}

#[test]
fn big_rat() {
    assert_eq!(
        Value::rational(Value::Int(3), Value::Int(2500)),
        set_big_rat_from_float_string("1.2e-3").unwrap()
    );
}

fn set_int_string(
    conf: &Config,
    s: &str,
) -> Result<i64, std::num::ParseIntError> {
    let mut base = conf.input_base();
    if base == 0 {
        base = 10;
    }
    if s.len() >= 2 {
        match &s[..2] {
            "0x" => {
                return i64::from_str_radix(s.strip_prefix("0x").unwrap(), 16)
            }
            "0o" => {
                return i64::from_str_radix(s.strip_prefix("0o").unwrap(), 8);
            }
            _ => {}
        }
    }
    i64::from_str_radix(s, base as u32)
}

fn parse_two(
    conf: &Config,
    s: &str,
) -> Result<(Value, Value, &'static str), ParseError> {
    let (sep, typ) = if s.contains('j') {
        ("j", "complex")
    } else if s.contains('/') {
        ("/", "rational")
    } else {
        return Ok((Value::Int(0), Value::Int(0), ""));
    };
    let elems: Vec<_> = s.split(sep).collect();
    if elems.len() != 2 || elems[0].is_empty() || elems[1].is_empty() {
        panic!("bad {typ} number syntax: `{s}`");
    }
    let v1 = parse(conf, elems[0])?;
    let v2 = parse(conf, elems[1])?;
    Ok((v1, v2, sep))
}

pub mod eval;

pub mod context {

    use crate::exec::context::Context;

    use super::Value;

    pub mod expr;

    pub trait UnaryOp<'a> {
        fn eval_unary(&self, ctx: &Context<'a>, right: Value) -> Value;
    }

    pub trait BinaryOp<'a> {
        fn eval_binary(
            &self,
            ctx: &Context<'a>,
            right: Value,
            left: Value,
        ) -> Value;
    }
}
