use um_asm::assemble;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    #[arg(required = true)]
    files: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    for file in args.files {
        let instrs = assemble(&file)?;

        if args.debug {
            dbg!(instrs);
        }
    }

    Ok(())
}
