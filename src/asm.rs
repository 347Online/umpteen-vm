use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use thiserror::Error;

use crate::value::Value;

#[derive(Serialize, Deserialize, Debug)]
pub enum Instr {
    True,
    False,
    Push(Value),
    Pop,
    Add,
    Sub,
    Mul,
    Div, // ???
    Mod,
    PrintLn,
}

#[derive(Error, Debug)]
pub enum AsmError {
    #[error("Missing argument for push")]
    MissingArgument,
    #[error("Unknown instruction `{0}`")]
    UnknownInstruction(String),
}

pub fn assemble(path: &str) -> anyhow::Result<Vec<Instr>> {
    let mut instrs = vec![];

    let push_re = Regex::new(r"PUSH\s+(\d+(?:\.(?:\d*)?)?)").unwrap();

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    macro_rules! instr {
        ($($i:tt)+) => {
            instrs.push(Instr::$($i)+)
        };
    }

    for line in reader.lines() {
        match line?.as_str() {
            "" => (),
            "TRUE" => instr!(True),
            "FALSE" => instr!(False),
            "POP" => instr!(Pop),
            "ADD" => instr!(Add),
            "SUB" => instr!(Sub),
            "MUL" => instr!(Mul),
            "DIV" => instr!(Div),
            "MOD" => instr!(Mod),
            "PRINTLN" => instr!(PrintLn),

            // PUSH
            line if let Some(caps) = push_re.captures(line) => {
                let num: f64 = caps
                    .get(1)
                    .ok_or(AsmError::MissingArgument)?
                    .as_str()
                    .parse()?;
                instr!(Push(num.into()));
            }

            line => Err(AsmError::UnknownInstruction(line.to_owned()))?,
        }
    }

    Ok(instrs)
}
