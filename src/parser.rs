use thiserror::Error;

use crate::lexer::{Position, Token, TokenType};

#[derive(Debug)]
enum UnOp {
    Negate,
    Not,
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    UnOp {
        op: UnOp,
        expr: Box<Expr>,
    },
    BinOp {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token `` at {1}")]
    UnexpectedToken(TokenType, Position),
}

#[derive(Debug)]
pub struct Parser<'s> {
    tokens: Vec<Token<'s>>,
}

impl<'s> Parser<'s> {
    pub fn new(tokens: Vec<Token<'s>>) -> Self {
        Parser { tokens }
    }

    pub fn parse(self) -> anyhow::Result<Expr> {
        let mut tokens = self.tokens.into_iter().peekable();

        while let Some(token) = tokens.next() {
            use TokenType as TT;
            match token.kind() {
                TT::Number => {
                    Expr::Number(token.lexeme().parse().unwrap());
                }

                _ => {
                    let (kind, _, at) = token.dissolve();
                    Err(ParseError::UnexpectedToken(kind, at))?;
                }
            }
        }

        todo!()
    }
}
