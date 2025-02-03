use crate::{asm::Instr, value::Value};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Cannot apply binary operator `{op}` to types {} and {}", a.kind(), b.kind())]
    // This may need to change
    InvalidBinaryOp { a: Value, b: Value, op: String },
    #[error("Tried to pop empty stack")]
    EmptyPop,
}

#[derive(Debug, Default)]
pub struct Vm {
    stack: Vec<Value>,
    program: Vec<Instr>,
}

impl Vm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&mut self, program: Vec<Instr>) {
        self.program = program;
    }

    pub fn push<V: Into<Value>>(&mut self, value: V) {
        self.stack.push(value.into())
    }

    pub fn pop(&mut self) -> anyhow::Result<Value> {
        Ok(self.stack.pop().ok_or(RuntimeError::EmptyPop)?)
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        use Instr as I;

        let program = std::mem::take(&mut self.program);

        macro_rules! binary {
            ($op:tt) => {{
                let b = self.pop()?;
                let a = self.pop()?;

                let (Value::Number(a), Value::Number(b)) = (&a, &b) else {
                    return Err(RuntimeError::InvalidBinaryOp{ a, b, op: stringify!($op).to_owned() })?;
                };

                self.push(a $op b);
            }};
        }

        for instr in program {
            match instr {
                I::True => self.push(true),
                I::False => self.push(false),
                I::Push(x) => self.push(x),
                I::Pop => {
                    self.pop()?;
                }

                I::Add => binary!(+),
                I::Sub => binary!(-),
                I::Mul => binary!(*),
                I::Div => binary!(/),
                I::PrintLn => println!("{}", self.pop()?),
            }
        }

        Ok(())
    }
}
