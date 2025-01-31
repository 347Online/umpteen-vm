use um_vm::{Instr, Value, Vm};

fn main() {
    let program = vec![
        Instr::Push(Value::Number(32.0)),
        Instr::Push(Value::Number(48.0)),
        Instr::Add,
        Instr::PrintLn,
    ];

    let mut vm = Vm::new();
    vm.load(program);
    vm.run();
}
