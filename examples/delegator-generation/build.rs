use near_abi_client::Generator;

fn main() -> anyhow::Result<()> {
    Generator::new("gen".into())
        .file("src/adder.json")
        .generate()?;
    Ok(())
}
