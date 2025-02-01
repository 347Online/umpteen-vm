use um_asm::assemble;

fn main() -> anyhow::Result<()> {
    let instrs = assemble()?;

    dbg!(instrs);

    Ok(())
}
