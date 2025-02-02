use crate::value::Value;

#[derive(Debug)]
pub enum Instr {
    Push(Value),
    Pop,
    Add,
    Sub,
    Mul,
    Div, // ???
    PrintLn,
}
