use serde_json::json;
// use workspaces::prelude::*;
use workspaces::{Account, AccountId, Contract, Network, Worker};

pub const BASE_GAS: u64 = 5_000_000_000_000;

pub async fn init_nft(
    worker: &Worker<impl Network>,
    nft: &Contract,
    root: &Account,
) -> anyhow::Result<()> {
    let outcome = nft
        .call(&worker, "new_default_meta")
        .args_json(json!({
            "owner_id": root.id(),
        }))?
        .transact()
        .await?;
    println!("nft new_default_meta: {:#?}", outcome);
    Ok(())
}

pub async fn mint_nft(
    token_id: &str,
    worker: &Worker<impl Network>,
    account: &Account,
    nft: &Contract,
    receiver_id: AccountId,
) -> anyhow::Result<()> {
    let deposit = 10000000000000000000000;
    let res = account
        .call(&worker, nft.id().clone(), "nft_mint")
        .args_json(json!({
            "token_id": token_id,
            "receiver_id": receiver_id,
            "token_metadata": {
                "title": "Olympus Mons",
                "dscription": "Tallest mountain in charted solar system",
                "copies": 1,
            },
        }))?
        .deposit(deposit)
        .transact()
        .await?;
    println!("nft_mint: {:#?}", res);
    Ok(())
}

pub async fn init_market(
    worker: &Worker<impl Network>,
    market: &Contract,
    nft: &Contract,
    root: &Account,
) -> anyhow::Result<()> {
    let outcome2 = market
        .call(&worker, "new")
        .args_json(json!({
            "owner_id": root.id(),
            "nft_contract": nft.id(),
        }))?
        .transact()
        .await?;

    println!("market new: {:#?}", outcome2);

    Ok(())
}

pub async fn init_park(
    worker: &Worker<impl Network>,
    park: &Contract,
    root: &Account,
    epoch_millis: u64,
) -> anyhow::Result<()> {
    let outcome = root
        .call(&worker, park.id().clone(), "new_default_meta")
        .args_json(json!({
            "owner_id": root.id(),
            "total_supply": "1000000",
            "epoch_millis": epoch_millis
        }))?
        .transact()
        .await?;
    println!("park new_default_meta: {:#?}", outcome);

    Ok(())
}
