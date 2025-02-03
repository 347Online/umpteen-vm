use derive_getters::{Dissolve, Getters};
use thiserror::Error;

#[derive(Debug)]
pub enum TokenType {
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,

    Semicolon,
    ParOpen,
    ParClose,

    If,
    Then,
    Else,
    End,

    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Number,
    Ident,

    Eof,
}

#[derive(Debug)]
pub struct Position(usize, usize);

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

#[derive(Getters, Dissolve, Debug)]
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
                    ($kind:tt) => {{
                        let token = Token::new(
                            TokenType::$kind,
                            &line[start..pos],
                            Position(line_number, pos),
                        );
                        tokens.push(token);
                    }};
                }

                macro_rules! catch_or {
                    ($c:literal, $yes:tt, $no:tt) => {
                        if bytes.next_if_eq(&$c).is_some() {
                            pos += 1;
                            push_token!($yes);
                        } else {
                            push_token!($no)
                        }
                    };
                }

                match c {
                    c if !c.is_ascii() => {
                        Err(SyntaxError::NonAsciiCharacter(Position(i + 1, pos)))?
                    }

                    c if c.is_ascii_whitespace() => {
                        continue;
                    }

                    b'+' => push_token!(Plus),
                    b'-' => push_token!(Minus),
                    b'*' => push_token!(Star),
                    b'/' => push_token!(Slash),
                    b';' => push_token!(Semicolon),
                    b'(' => push_token!(ParOpen),
                    b')' => push_token!(ParClose),

                    b'=' => catch_or!(b'=', EqualEqual, Equal),
                    b'!' => catch_or!(b'=', BangEqual, Bang),
                    b'>' => catch_or!(b'=', GreaterEqual, Greater),
                    b'<' => catch_or!(b'=', LessEqual, Less),

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

                            _ => push_token!(Ident),
                        }
                    }

                    c => Err(SyntaxError::UnexpectedCharacter(
                        c.into(),
                        Position(line_number, pos),
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
