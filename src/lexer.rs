use derive_getters::{Dissolve, Getters};
use paste::paste;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug)]
pub enum TokenType {
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,
    Modulo,
    ModuloEqual,
    Ampersand,
    AmpersandEqual,
    Pipe,
    PipeEqual,

    Semicolon,
    ParOpen,
    ParClose,

    If,
    Then,
    Else,
    End,
    Fnc,
    Return,
    And,
    Or,

    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Number,
    String,
    Ident,

    Eof,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Position(usize, usize);

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

#[derive(Serialize, Deserialize, Getters, Dissolve, Debug)]
pub struct Token<'s> {
    kind: TokenType,
    lexeme: &'s str,
    at: Position,
}

impl<'s> Token<'s> {
    pub fn new(kind: TokenType, lexeme: &'s str, at: Position) -> Self {
        Token { kind, lexeme, at }
    }
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Encountered non-ascii character at {0}")]
    NonAsciiCharacter(Position),
    #[error("Unexpected error `{0}`")]
    UnexpectedCharacter(char, Position),
}

#[derive(Debug)]
pub struct Lexer {
    source: String,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.to_owned(),
        }
    }

    pub fn scan_tokens(&self) -> anyhow::Result<Vec<Token>> {
        let mut tokens = vec![];
        let mut line_number = 1;

        for (i, line) in self.source.lines().enumerate() {
            line_number = i + 1;
            let mut bytes = line.bytes().peekable();
            let mut pos = 0;
            while let Some(c) = bytes.next() {
                let start = pos;
                pos += 1;

                macro_rules! push_token {
                    ($kind:tt, $lexeme:expr) => {
                        tokens.push(Token::new(
                            TokenType::$kind,
                            $lexeme,
                            Position(line_number, start + 1),
                        ))
                    };
                    ($kind:tt) => {
                        push_token!($kind, &line[start..pos])
                    };
                }

                macro_rules! catch_or {
                    ($c:literal, $double:tt, $base:tt) => {
                        if bytes.next_if_eq(&$c).is_some() {
                            pos += 1;
                            push_token!($double);
                        } else {
                            push_token!($base);
                        }
                    };
                    ($base:tt) => {
                        if bytes.next_if_eq(&b'=').is_some() {
                            pos += 1;
                            paste! {
                                push_token!([<$base E q u a l>]);
                            }
                        } else {
                            push_token!($base);
                        }
                    };
                }

                match c {
                    c if !c.is_ascii() => Err(SyntaxError::NonAsciiCharacter(Position(
                        line_number,
                        start + 1,
                    )))?,

                    c if c.is_ascii_whitespace() => {
                        continue;
                    }

                    b';' => push_token!(Semicolon),
                    b'(' => push_token!(ParOpen),
                    b')' => push_token!(ParClose),

                    b'+' => catch_or!(Plus),
                    b'-' => catch_or!(Minus),
                    b'*' => catch_or!(Star),
                    b'/' => catch_or!(Slash),

                    b'=' => catch_or!(Equal),
                    b'!' => catch_or!(Bang),
                    b'>' => catch_or!(Greater),
                    b'<' => catch_or!(Less),
                    b'&' => catch_or!(Ampersand),
                    b'|' => catch_or!(Pipe),

                    b'"' => {
                        for c in bytes.by_ref() {
                            pos += 1;
                            if c == b'"' {
                                break;
                            }
                        }

                        push_token!(String, &line[start..pos]);
                    }

                    c if c.is_ascii_digit() => {
                        while bytes.next_if(|c| c.is_ascii_digit()).is_some() {
                            pos += 1;
                        }
                        if bytes.next_if_eq(&b'.').is_some() {
                            pos += 1;
                            while bytes.next_if(|c| c.is_ascii_digit()).is_some() {
                                pos += 1;
                            }
                        }

                        push_token!(Number);
                    }

                    c if c == b'_' || c.is_ascii_alphabetic() => {
                        while bytes
                            .next_if(|c| c.is_ascii_alphanumeric() || *c == b'_')
                            .is_some()
                        {
                            pos += 1;
                        }
                        let lexeme = &line[start..pos];
                        match lexeme {
                            "if" => push_token!(If),
                            "then" => push_token!(Then),
                            "else" => push_token!(Else),
                            "end" => push_token!(End),
                            "fnc" => push_token!(Fnc),
                            "return" => push_token!(Return),
                            "and" => push_token!(And),
                            "or" => push_token!(Or),

                            _ => push_token!(Ident),
                        }
                    }

                    c => Err(SyntaxError::UnexpectedCharacter(
                        c.into(),
                        Position(line_number, start + 1),
                    ))?,
                }
            }
        }

        tokens.push(Token::new(
            TokenType::Eof,
            "<EOF>",
            Position(line_number + 1, 1),
        ));

        Ok(tokens)
    }
}
