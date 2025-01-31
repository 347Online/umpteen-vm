#[derive(Debug)]
pub enum Value {
    Number(f64),
    Bool(bool),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(x) => write!(f, "{x}"),
            Value::Bool(x) => write!(f, "{x}"),
        }
    }
}

#[derive(Debug)]
pub enum Instr {
    Push(Value),
    Pop,
    Add,
    PrintLn,
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

    pub fn run(&mut self) -> Option<Value> {
        use Instr as I;

        let program = std::mem::take(&mut self.program);

        for instr in program {
            match instr {
                I::Push(x) => self.stack.push(x),
                I::Pop => {
                    self.stack.pop();
                }
                I::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();

                    let (Value::Number(a), Value::Number(b)) = (a, b) else {
                        eprintln!("Add operands must both be numbers");
                        continue;
                    };

                    self.stack.push(Value::Number(a + b));
                }
                I::PrintLn => {
                    let x = self.stack.pop().unwrap();
                    println!("{}", x);
                }
            }
        }

        None
    }
}
