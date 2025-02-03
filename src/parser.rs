use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::lexer::{Position, Token, TokenType};

#[derive(Serialize, Deserialize, Debug)]
enum UnOp {
    Negate,
    Not,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Serialize, Deserialize, Debug)]
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
pub struct AstParser<'s> {
    tokens: Vec<Token<'s>>,
}

impl<'s> AstParser<'s> {
    pub fn new(tokens: Vec<Token<'s>>) -> Self {
        AstParser { tokens }
    }

    pub fn parse(self) -> anyhow::Result<Expr> {
        let mut tokens = self.tokens.into_iter().peekable();

        while let Some(token) = tokens.next() {
            use TokenType as TT;
            match token.kind() {
                TT::Number => {
                    return Ok(Expr::Number(token.lexeme().parse().unwrap()));
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
