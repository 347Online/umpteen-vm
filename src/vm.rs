use crate::{instr::Instr, value::Value};
use anyhow::anyhow;

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
        self.stack.pop().ok_or(anyhow!("Tried to pop empty stack"))
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        use Instr as I;

        let program = std::mem::take(&mut self.program);

        macro_rules! binary {
            ($op:tt) => {{
                let b = self.pop()?;
                let a = self.pop()?;

                let (Value::Number(a), Value::Number(b)) = (a, b) else {
                    return Err(anyhow!("Operands must both be numbers"));
                };

                self.push(a $op b);
            }};
        }

        for instr in program {
            match instr {
                I::Push(x) => self.stack.push(x),
                I::Pop => std::mem::drop(self.pop()?),

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
