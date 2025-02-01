use um_asm::assemble;

fn main() -> anyhow::Result<()> {
    let instrs = assemble("foo.umasm")?;

    dbg!(instrs);

    Ok(())
}
