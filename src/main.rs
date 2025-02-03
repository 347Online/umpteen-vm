use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};

use clap::{Parser, Subcommand};
use umpteen::{
    asm::assemble,
    lexer::{Lexer, Token},
    parser::AstParser,
    vm::Vm,
};

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
    /// Scan src file to generate tokens
    Lex {
        #[clap(short, long)]
        debug: bool,
        #[clap()]
        file: String,
    },
    /// Parse tokens to AST
    Parse {
        #[clap(short, long)]
        debug: bool,
        #[clap()]
        file: String,
    },
}

#[derive(Debug, clap::Parser)]
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
        C::Lex { file, debug } => {
            let source = read_to_string(&file)?;
            let lexer = Lexer::new(&source);
            let tokens = lexer.scan_tokens()?;

            if debug {
                dbg!(&tokens);
            }

            let outfile = PathBuf::from(file).with_extension("tokens.json");
            let json = serde_json::to_string_pretty(&tokens)?;
            fs::write(&outfile, json)?;
            println!("Wrote tokens to {}", outfile.to_str().unwrap());
        }

        C::Parse { file, debug } => {
            let json = read_to_string(file)?;
            let tokens = serde_json::from_str::<Vec<Token>>(&json)?;
            let parser = AstParser::new(tokens);
            let ast = parser.parse()?;

            if debug {
                dbg!(&ast);
            }
        }
    }

    Ok(())
}
