use std::fs::read_to_string;

use clap::{Parser, Subcommand};
use umpteen::{asm::assemble, lexer::Lexer, vm::Vm};

#[derive(Debug, Subcommand)]
enum Commands {
    /// Assemble Umpteen bytecode from instructions file
    Assemble {
        #[clap(short, long)]
        debug: bool,
        #[clap(short, long)]
        exec: bool,
        #[clap()]
        file: String,
    },
    /// Scan tokens from a .um src file
    Lex {
        #[clap()]
        file: String,
    },
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    use Commands as C;
    match args.command {
        C::Assemble { debug, exec, file } => {
            let instrs = assemble(&file)?;

            if debug {
                dbg!(&instrs);
            }

            if exec {
                let mut vm = Vm::new();
                vm.load(instrs);
                vm.run()?;
            }
        }
        C::Lex { file } => {
            let source = read_to_string(file)?;
            let lexer = Lexer::new(&source);
            let tokens = lexer.scan_tokens()?;

            dbg!(tokens);
        }
    }

    Ok(())
}
