use um_asm::assemble;

use clap::Parser;
use um_vm::Vm;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Execute after assembly
    #[arg(short, long)]
    exec: bool,

    /// Print debug IR
    #[arg(short, long)]
    debug: bool,

    /// File to assemble
    #[arg(required = true)]
    file: String,
}

fn main() -> anyhow::Result<()> {
    let Args { exec, debug, file } = Args::parse();

    let instrs = assemble(&file)?;

    if debug {
        dbg!(&instrs);
    }

    if exec {
        let mut vm = Vm::new();
        vm.load(instrs);
        vm.run();
    }

    Ok(())
}
