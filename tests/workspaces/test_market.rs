use crate::utils::{init_market, init_nft, mint_nft, BASE_GAS};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    serde_json,
    serde_json::{json, Value},
};
use near_units::parse_gas;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};
use workspaces::prelude::*;
use workspaces::{Account, Contract, Network, Worker};

const NFT_WASM_FILEPATH: &str = "./out/non_fungible_token.wasm";
const MARKET_WASM_FILEPATH: &str = "./out/market.wasm";

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
struct SaleArgs {
    price: u128,
    exp: u64,
    ft_contract: Option<String>,
}

#[tokio::test]
async fn test_market() -> anyhow::Result<()> {
    let worker = workspaces::sandbox();

    let root = worker.root_account();

    let alice = worker.dev_create_account().await?;

    let wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft = worker.dev_deploy(wasm).await?;
    init_nft(&worker, &nft, &root).await?;

    let wasm = std::fs::read(MARKET_WASM_FILEPATH)?;
    let market = worker.dev_deploy(wasm).await?;
    init_market(&worker, &market, &nft, &root).await?;

    let id = "0";
    let from_now = 10_000;
    let exp = millis() + from_now;
    list_sale(id, exp, &worker, &root, &nft, &market).await?;

    bid_and_settle(id, from_now, &worker, &nft, &market, &alice).await?;

    let id1 = "1";
    let exp1 = 0; // not an auction
    list_sale(id1, exp1, &worker, &root, &nft, &market).await?;

    buy(id1, &worker, &nft, &market, &alice).await?;

    Ok(())
}

async fn list_sale(
    token_id: &str,
    exp: u64,
    worker: &Worker<impl Network>,
    root: &Account,
    nft: &Contract,
    market: &Contract,
) -> anyhow::Result<()> {
    mint_nft(token_id, worker, root, nft, root.id().clone()).await?;

    let a = SaleArgs {
        price: 12,
        exp: exp,
        ft_contract: None,
    };
    let msg = serde_json::to_string(&a).unwrap();
    let deposit = 450_000_000_000_000_000_000;
    let res = root
        .call(&worker, nft.id().clone(), "nft_approve")
        .args_json(json!({
            "token_id": token_id,
            "account_id": market.id(),
            "msg": msg,
        }))?
        .deposit(deposit)
        .gas(parse_gas!("100 Tgas") as u64)
        .transact()
        .await?;
    println!("nft_approve: {:#?}", res.status);

    let contract_and_token: String = res.json()?;
    println!("contract_and_token: {:#?}", contract_and_token);

    let result: Value = worker
        .view(
            nft.id().clone(),
            "nft_is_approved",
            json!({
                "token_id": token_id,
                "approved_account_id": market.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    println!("nft_is_approved: {:#?}", result);

    let supply_sales: String = worker
        .view(market.id().clone(), "get_supply_sales", Vec::new())
        .await?
        .json()?;
    println!("supply sales: {:?}", supply_sales);
    assert_eq!(supply_sales, "1", "should be 1 nft");

    let all_sales: Value = worker
        .view(market.id().clone(), "get_all_sales", Vec::new())
        .await?
        .json()?;
    println!("all_sales: {:?}", all_sales);

    let sale: Value = worker
        .view(
            market.id().clone(),
            "get_sale",
            json!({
                "contract_and_token": contract_and_token,
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    println!("sale: {:?}", sale);

    Ok(())
}

async fn bid_and_settle(
    token_id: &str,
    exp_from_now: u64,
    worker: &Worker<impl Network>,
    nft: &Contract,
    market: &Contract,
    alice: &Account,
) -> anyhow::Result<()> {
    let res = alice
        .call(&worker, market.id().clone(), "bid")
        .args_json(json!({
            "nft_contract": nft.id(),
            "token_id": token_id,
        }))?
        .deposit(BASE_GAS.into())
        .transact()
        .await?;
    println!("bid: {:#?}", res);

    let res = alice
        .call(&worker, market.id().clone(), "settle")
        .args_json(json!({
            "nft_contract": nft.id(),
            "token_id": token_id,
        }))?
        .transact()
        .await?;
    println!("settle 1: {:#?}", res);

    let alices_tokens: Value = worker
        .view(
            nft.id().clone(),
            "nft_tokens_for_owner",
            json!({
                "account_id": alice.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    println!("alices_tokens: {:?}", alices_tokens);
    let len1 = alices_tokens.as_array().unwrap().len();

    sleep(Duration::from_millis(exp_from_now)).await;

    let res = alice
        .call(&worker, market.id().clone(), "settle")
        .args_json(json!({
            "nft_contract": nft.id(),
            "token_id": token_id,
        }))?
        .gas(parse_gas!("100 Tgas") as u64)
        .transact()
        .await?;
    println!("settle 2: {:#?}", res);

    let alices_tokens2: Value = worker
        .view(
            nft.id().clone(),
            "nft_tokens_for_owner",
            json!({
                "account_id": alice.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    let len2 = alices_tokens2.as_array().unwrap().len();
    println!("alices_tokens len 2: {:?}", len2);

    assert!(len1 == len2 - 1, "didnt get the token");
    Ok(())
}

async fn buy(
    token_id: &str,
    worker: &Worker<impl Network>,
    nft: &Contract,
    market: &Contract,
    alice: &Account,
) -> anyhow::Result<()> {
    let alices_tokens: Value = worker
        .view(
            nft.id().clone(),
            "nft_tokens_for_owner",
            json!({
                "account_id": alice.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    let len1 = alices_tokens.as_array().unwrap().len();
    println!("alices_tokens len: {:?}", len1);
    // buy it
    // since it "settles", gas is needed
    let res = alice
        .call(&worker, market.id().clone(), "bid")
        .args_json(json!({
            "nft_contract": nft.id(),
            "token_id": token_id,
        }))?
        .deposit(BASE_GAS.into())
        .gas(parse_gas!("100 Tgas") as u64)
        .transact()
        .await?;
    println!("bid: {:#?}", res);

    let alices_tokens2: Value = worker
        .view(
            nft.id().clone(),
            "nft_tokens_for_owner",
            json!({
                "account_id": alice.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    println!("alices_tokens 2: {:?}", alices_tokens2);
    let len2 = alices_tokens2.as_array().unwrap().len();

    assert!(len1 == len2 - 1, "didnt get the token");

    // alice.view_account()

    Ok(())
}

fn millis() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis() as u64
}
