use thiserror::Error;

#[derive(Debug)]
pub enum TokenType {
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,

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

    Ident,
}

#[derive(Debug)]
pub struct Position(usize, usize);

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

#[derive(Debug)]
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

    pub fn scan_tokens(&mut self) -> anyhow::Result<Vec<Token>> {
        let mut tokens = vec![];

        for (line_index, line) in self.source.lines().enumerate() {
            let mut bytes = line.bytes().peekable();
            let mut pos = 0;
            while let Some(c) = bytes.next() {
                let start = pos;
                pos += 1;

                macro_rules! catch_or {
                    ($c:literal, $yes:tt, $no:tt) => {
                        if bytes.next_if_eq(&$c).is_some() {
                            TokenType::$yes
                        } else {
                            TokenType::$no
                        }
                    };
                }

                use TokenType as TT;
                let kind = match c {
                    c if !c.is_ascii() => Err(SyntaxError::NonAsciiCharacter(Position(
                        line_index + 1,
                        pos,
                    )))?,

                    b'+' => TT::Plus,
                    b'-' => TT::Minus,
                    b'*' => TT::Star,
                    b'/' => TT::Slash,
                    b'(' => TT::ParOpen,
                    b')' => TT::ParClose,

                    b'=' => catch_or!(b'=', EqualEqual, Equal),
                    b'!' => catch_or!(b'=', BangEqual, Bang),
                    b'>' => catch_or!(b'=', GreaterEqual, Greater),
                    b'<' => catch_or!(b'=', LessEqual, Less),

                    c if c.is_ascii_whitespace() => continue,

                    c if c == b'_' || c.is_ascii_alphabetic() => {
                        while bytes
                            .next_if(|c| c.is_ascii_alphanumeric() || *c == b'_')
                            .is_some()
                        {
                            pos += 1;
                        }
                        let name = &line[start..pos];
                        match name {
                            "if" => TT::If,
                            "then" => TT::Then,
                            "else" => TT::Else,
                            "end" => TT::End,

                            _ => TT::Ident,
                        }
                    }

                    c => Err(SyntaxError::UnexpectedCharacter(
                        c.into(),
                        Position(line_index + 1, pos),
                    ))?,
                };

                tokens.push(Token::new(kind, &line[start..pos], Position(start, pos)));
            }
        }

        Ok(tokens)
    }
}
