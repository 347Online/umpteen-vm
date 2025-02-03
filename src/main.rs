use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};

use clap::{Args, Parser, Subcommand};
use umpteen::{
    asm::assemble,
    lexer::{Lexer, Token},
    parser::AstParser,
    vm::Vm,
};

#[derive(Debug, Args)]
struct IrArgs {
    /// File to read from
    file: String,
    /// Suppress debug output
    #[arg(short, long)]
    quiet: bool,
    /// Write output to file
    #[arg(long)]
    emit: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Assemble Umpteen bytecode from instructions file
    Assemble {
        /// Suppress debug output
        #[arg(short, long)]
        quiet: bool,
        /// Execute the assembled code
        #[clap(short, long)]
        exec: bool,
        /// File to read from
        #[clap()]
        file: String,
    },
    /// Scan src file to generate tokens
    Lex(IrArgs),
    /// Parse tokens to AST
    Parse(IrArgs),
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    use Commands as C;
    match args.command {
        C::Assemble { quiet, exec, file } => {
            let instrs = assemble(&file)?;

            if !quiet {
                dbg!(&instrs);
            }

            if exec {
                let mut vm = Vm::new();
                vm.load(instrs);
                vm.run()?;
            }
        }
        C::Lex(IrArgs { file, quiet, emit }) => {
            let source = read_to_string(&file)?;
            let lexer = Lexer::new(&source);
            let tokens = lexer.scan_tokens()?;

            if !quiet {
                dbg!(&tokens);
            }

            if emit {
                let outfile = PathBuf::from(file).with_extension("tokens.json");
                let json = serde_json::to_string_pretty(&tokens)?;
                fs::write(&outfile, json)?;
                println!("Wrote tokens to {}", outfile.to_str().unwrap());
            }
        }

        C::Parse(IrArgs { file, quiet, emit }) => {
            let json = read_to_string(file)?;
            let tokens = serde_json::from_str::<Vec<Token>>(&json)?;
            let parser = AstParser::new(tokens);
            let ast = parser.parse()?;

            if !quiet {
                dbg!(&ast);
            }

            if emit {
                todo!()
            }
        }
    }

    Ok(())
}
