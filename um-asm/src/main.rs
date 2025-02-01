use anyhow::anyhow;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use um_vm::{Instr, Value};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("foo.umasm")?;
    let reader = BufReader::new(file);

    let mut instrs = vec![];
    let mut buf = String::new();

    for (i, line) in reader.lines().enumerate() {
        buf.clear();
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let mut chars = line.chars().enumerate().peekable();
        while let Some((j, c)) = chars.next() {
            if c.is_ascii_whitespace() || chars.peek().is_none() {
                break;
            }
            if c.is_ascii_alphabetic() {
                buf.push(c);
            } else {
                Err(anyhow!(
                    "Line {}:{}, unexpected character `{}`",
                    i + 1,
                    j + 1,
                    c
                ))?
            }
        }

        match buf.to_ascii_lowercase().as_str() {
            "push" => {
                instrs.push(Instr::Push(Value::Number(69.0)));
                continue;
            }
            "pop" => {
                instrs.push(Instr::Pop);
                continue;
            }
            "add" => {
                instrs.push(Instr::Add);
                continue;
            }
            "println" => {
                instrs.push(Instr::PrintLn);
                continue;
            }

            x => Err(anyhow!("`{x}`"))?,
        }
    }

    dbg!(instrs);

    Ok(())
}
