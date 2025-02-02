use crate::value::Value;

#[derive(Debug)]
pub enum Instr {
    True,
    False,
    Push(Value),
    Pop,
    Add,
    Sub,
    Mul,
    Div, // ???
    PrintLn,
}
