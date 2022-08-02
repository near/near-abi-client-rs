use workspaces::network::DevAccountDeployer;

#[path = "../gen/adder.rs"]
mod adder;

pub async fn run(a: u32, b: u32, c: u32, d: u32) -> anyhow::Result<(u32, u32)> {
    let worker = workspaces::sandbox().await?;
    let contract = worker
        .dev_deploy(include_bytes!("../res/adder.wasm"))
        .await?;

    let contract = adder::AbiClient { contract };
    let res = contract
        .add(&worker, vec![a.into(), b.into()], vec![c.into(), d.into()])
        .await?;

    Ok((res[0].try_into().unwrap(), res[1].try_into().unwrap()))
}
