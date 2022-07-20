use near_abi_client::Config;

fn main() -> anyhow::Result<()> {
    let config = Config {
        out_dir: Some("gen".into()),
    };
    config.generate_abi(&[("src/adder.json", None)])?;
    Ok(())
}
