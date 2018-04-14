use std::iter::Peekable;
use std::result::Result as StdResult;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid character in identifier: {}", _0)]
    InvalidIdent(char),
    #[fail(display = "invalid number: {:?}", _0)]
    InvalidNumber(String),
    #[fail(display = "unclosed comment")]
    UnclosedComment,
    #[fail(display = "unclosed string")]
    UnclosedString,
    #[fail(display = "unexpected end of file")]
    UnexpectedEOF,
    #[fail(display = "unknown escape character: {}", _0)]
    UnknownEscape(char),
    #[fail(display = "unknown token")]
    UnknownToken,
}

type Result<T> = StdResult<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Noob,
    Yarn(String),
    Numbr(i64),
    Numbar(f64),
    Troof(bool)
}
impl Default for Value {
    fn default() -> Self {
        Value::Noob
    }
}
impl Value {
    pub fn cast_yarn(self) -> Option<String> {
        match self {
            Value::Noob => None,
            Value::Yarn(inner) => Some(inner),
            Value::Numbr(n) => Some(n.to_string()),
            Value::Numbar(n) => Some(n.to_string()),
            Value::Troof(b) => Some(b.to_string())
        }
    }
    pub fn cast_numbr(&self) -> Option<i64> {
        match *self {
            Value::Noob => None,
            Value::Yarn(ref inner) => Some(inner.parse().unwrap_or(0)),
            Value::Numbr(n) => Some(n),
            Value::Numbar(n) => Some(n as i64),
            Value::Troof(b) => Some(b as i64)
        }
    }
    pub fn cast_numbar(&self) -> Option<f64> {
        match *self {
            Value::Noob => None,
            Value::Yarn(ref inner) => Some(inner.parse().unwrap_or(0.0)),
            Value::Numbr(n) => Some(n as f64),
            Value::Numbar(n) => Some(n),
            Value::Troof(b) => Some(b as i64 as f64)
        }
    }
    pub fn is_numbar(&self) -> bool {
        match *self {
            Value::Yarn(ref inner) => inner.parse::<f64>().is_ok(),
            Value::Numbar(_) => true,
            _ => false
        }
    }
    pub fn cast_troof(&self) -> bool {
        match *self {
            Value::Noob => false,
            Value::Yarn(ref inner) => inner.is_empty(),
            Value::Numbr(n) => n == 0,
            Value::Numbar(n) => n == 0.0,
            Value::Troof(b) => b
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Value(Value),

    Separator,

    IHasA,
    Itz,
    R,

    SumOf,
    DiffOf,
    ProduktOf,
    QuoshuntOf,
    ModOf,
    BiggrOf,
    SmallrOf,

    BothOf,
    EitherOf,
    WonOf,
    Not,
    AllOf,
    AnyOf,
    BothSaem,
    Diffrint,

    An,
    Mkay,

    ORly,
    YaRly,
    Mebbe,
    NoWai,
    Oic,

    Visible,
    Exclamation,
    Gimmeh
}

#[derive(Clone)]
pub struct Tokenizer<I: Iterator<Item = char> + Clone> {
    iter: Peekable<I>
}

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t'
}

impl<I: Iterator<Item = char> + Clone> Tokenizer<I> {
    fn trim(&mut self) {
        loop {
            match self.iter.peek().cloned() {
                Some(c) if is_space(c) => { self.iter.next(); },
                _ => break
            }
        }
    }
    fn peek(&mut self) -> Option<char> {
        self.trim();
        self.iter.peek().cloned()
    }
    fn word(&mut self) -> String {
        let mut word = String::new();
        loop {
            match self.iter.peek().cloned() {
                Some(c) if is_space(c) => {
                    self.trim();
                    return word;
                },
                None | Some('\n') | Some(',') => return word,
                Some(c) => {
                    self.iter.next();
                    word.push(c);
                }
            }
        }
    }
    pub fn next(&mut self) -> Result<Option<Token>> {
        let c = match self.peek() {
            Some(c) => c,
            None => return Ok(None)
        };
        if c == '"' {
            self.iter.next(); // leading "
            let mut string = String::new();
            while let Some(c) = self.iter.next() {
                if c == ':' {
                    string.push(match self.iter.next() {
                        Some(')') => '\n',
                        Some('>') => '\t',
                        Some('o') => '\x07',
                        Some('"') => '"',
                        Some(':') => ':',
                        Some(c) => return Err(Error::UnknownEscape(c)),
                        None => return Err(Error::UnclosedString)
                    });
                    continue;
                } else if c == '"' {
                    break;
                }
                string.push(c);
            }
            return Ok(Some(Token::Value(Value::Yarn(string))));
        } else if c == '\n' || c == ',' {
            self.iter.next();
            return Ok(Some(Token::Separator));
        }

        let word = self.word();
        match &*word {
            "BTW" => {
                loop {
                    match self.iter.next() {
                        Some('\n') | None => break,
                        _ => ()
                    }
                }
                return self.next();
            },
            "OBTW" => {
                loop {
                    match self.peek() {
                        None => return Err(Error::UnclosedComment),
                        Some('T') => {
                            if self.word() == "TLDR" {
                                return self.next();
                            } else {
                                self.iter.next();
                            }
                        },
                        _ => { self.iter.next(); },
                    }
                }
            },
            "I" => {
                let mut clone = self.clone();
                if clone.word() == "HAS" {
                    if clone.word() == "A" {
                        *self = clone;
                        return Ok(Some(Token::IHasA));
                    }
                }
            },
            "ITZ" => return Ok(Some(Token::Itz)),
            "R" => return Ok(Some(Token::R)),
            "SUM" | "DIFF" | "PRODUKT" | "QUOSHUNT" | "MOD" | "BIGGR" | "SMALLR" |
            "BOTH" | "EITHER" | "WON" | "ALL" | "ANY" => {
                let mut clone = self.clone();
                match &*clone.word() {
                    "OF" => {
                        *self = clone;
                        return Ok(Some(match &*word {
                            "SUM" => Token::SumOf,
                            "DIFF" => Token::DiffOf,
                            "PRODUKT" => Token::ProduktOf,
                            "QUOSHUNT" => Token::QuoshuntOf,
                            "MOD" => Token::ModOf,
                            "BIGGR" => Token::BiggrOf,
                            "SMALLR" => Token::SmallrOf,

                            "BOTH" => Token::BothOf,
                            "EITHER" => Token::EitherOf,
                            "WON" => Token::WonOf,
                            "ALL" => Token::AllOf,
                            "ANY" => Token::AnyOf,

                            _ => unreachable!()
                        }));
                    },
                    "SAEM" if word == "BOTH" => {
                        *self = clone;
                        return Ok(Some(Token::BothSaem));
                    },
                    _ => ()
                }
            },
            "NOT" => return Ok(Some(Token::Not)),
            "DIFFRINT" => return Ok(Some(Token::Diffrint)),
            "AN" => return Ok(Some(Token::An)),
            "MKAY" => return Ok(Some(Token::Mkay)),
            "O" => {
                let mut clone = self.clone();
                if clone.word() == "RLY?" {
                    *self = clone;
                    return Ok(Some(Token::ORly));
                }
            },
            "YA" => {
                let mut clone = self.clone();
                if clone.word() == "RLY" {
                    *self = clone;
                    return Ok(Some(Token::YaRly));
                }
            },
            "MEBBE" => return Ok(Some(Token::Mebbe)),
            "NO" => {
                let mut clone = self.clone();
                if clone.word() == "WAI" {
                    *self = clone;
                    return Ok(Some(Token::NoWai));
                }
            },
            "OIC" => return Ok(Some(Token::Oic)),

            "VISIBLE" => return Ok(Some(Token::Visible)),
            "!" => return Ok(Some(Token::Exclamation)),
            "GIMMEH" => return Ok(Some(Token::Gimmeh)),
            _ => ()
        }

        match c {
            'a'...'z' |
            'A'...'Z' |
            '_' => {
                for c in word.chars() {
                    match c {
                        'a'...'z' |
                        'A'...'Z' |
                        '0'...'9' |
                        '_' => (),
                        c => return Err(Error::InvalidIdent(c))
                    }
                }
                return Ok(Some(Token::Ident(word)));
            },
            '0'...'9' => {
                if let Ok(num) = word.parse::<i64>() {
                    return Ok(Some(Token::Value(Value::Numbr(num))));
                } else if let Ok(num) = word.parse::<f64>() {
                    return Ok(Some(Token::Value(Value::Numbar(num))));
                }
                return Err(Error::InvalidNumber(word));
            },
            _ => ()
        }

        Err(Error::UnknownToken)
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Token>> {
    let mut tokenizer = Tokenizer { iter: input.chars().peekable() };
    let mut tokens = Vec::new();
    while let Some(token) = tokenizer.next()? {
        tokens.push(token);
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strings() {
        assert_eq!(
            tokenize(r#" "Hello World :) How are you :>? I'm:: :"fine:"" "#).unwrap(),
            &[Token::Value(Value::Yarn("Hello World \n How are you \t? I'm: \"fine\"".to_string()))]
        );
    }
    #[test]
    fn assign() {
        assert_eq!(
            tokenize("I HAS A VAR ITZ 12           BTW this is a comment").unwrap(),
            &[Token::IHasA, Token::Ident("VAR".to_string()), Token::Itz, Token::Value(Value::Numbr(12))]
        );
    }
    #[test]
    fn sum_of() {
        assert_eq!(
            tokenize("SUM OF OBTW hi TLDR 2 AN 4").unwrap(),
            &[Token::SumOf, Token::Value(Value::Numbr(2)), Token::An, Token::Value(Value::Numbr(4))]
        );
    }
    #[test]
    fn ifs() {
        assert_eq!(
            tokenize("\
                BOTH SAEM 1 AN 1, O RLY?
                    YA RLY, RESULT R \"YES\"
                    MEBBE BOTH SAEM 1 AN 2, RESULT R \"CLOSE\"
                    NO WAI, RESULT R \"NO\"
                OIC\
            ").unwrap(),
            &[
                Token::BothSaem, Token::Value(Value::Numbr(1)), Token::An, Token::Value(Value::Numbr(1)), Token::Separator,
                Token::ORly, Token::Separator,
                    Token::YaRly, Token::Separator,
                        Token::Ident("RESULT".to_string()), Token::R, Token::Value(Value::Yarn("YES".to_string())),
                        Token::Separator,
                    Token::Mebbe,
                        Token::BothSaem, Token::Value(Value::Numbr(1)), Token::An, Token::Value(Value::Numbr(2)),
                        Token::Separator,
                        Token::Ident("RESULT".to_string()), Token::R, Token::Value(Value::Yarn("CLOSE".to_string())),
                        Token::Separator,
                    Token::NoWai, Token::Separator,
                        Token::Ident("RESULT".to_string()), Token::R, Token::Value(Value::Yarn("NO".to_string())),
                        Token::Separator,
                Token::Oic
            ]
        )
    }
}