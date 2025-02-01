use anyhow::anyhow;
use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::vm::{Instr, Value};

pub fn assemble(path: &str) -> anyhow::Result<Vec<Instr>> {
    let mut instrs = vec![];

    let push_re = Regex::new(r"PUSH\s+(\d+(?:\.(?:\d*)?)?)").unwrap();

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line?.as_str() {
            "" => (),
            "POP" => instrs.push(Instr::Pop),
            "ADD" => instrs.push(Instr::Add),
            "PRINTLN" => instrs.push(Instr::PrintLn),

            // PUSH
            line if let Some(caps) = push_re.captures(line) => {
                let num: f64 = caps
                    .get(1)
                    .ok_or(anyhow!("Missing argument for push"))?
                    .as_str()
                    .parse()
                    .unwrap();
                instrs.push(Instr::Push(Value::Number(num)));
            }

            line => Err(anyhow!("Unknown instruction `{line}`"))?,
        }
    }

    Ok(instrs)
}
