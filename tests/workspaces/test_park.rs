use crate::utils::{init_nft, init_park, mint_nft};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    serde_json,
    serde_json::{json, Value},
};
use near_units::parse_gas;
use std::collections::HashMap;
use workspaces::prelude::*;
use workspaces::{Account, AccountId, Contract, Network, Worker};
// use std::time::{SystemTime, UNIX_EPOCH};
// use tokio::time::{sleep, Duration};

const NFT_WASM_FILEPATH: &str = "./out/non_fungible_token.wasm";
const PARK_WASM_FILEPATH: &str = "./out/park_token.wasm";

#[tokio::test]
async fn test_park() -> anyhow::Result<()> {
    let worker = workspaces::sandbox();

    let root = worker.root_account();

    let alice = worker.dev_create_account().await?;

    let wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let nft = worker.dev_deploy(wasm).await?;
    init_nft(&worker, &nft, &alice).await?;

    let token_id = "0";

    // alice mints an NFT
    mint_nft(token_id, &worker, &alice, &nft, alice.id().clone()).await?;

    let token_2 = "1";
    // root mints an NFT
    mint_nft(token_2, &worker, &alice, &nft, root.id().clone()).await?;

    let wasm2 = std::fs::read(PARK_WASM_FILEPATH)?;
    let park = worker.dev_deploy(wasm2).await?;
    let epoch_length = 5_000;
    init_park(&worker, &park, &root, epoch_length).await?;

    // alice needs to "register" to park
    check_and_register_storage(&worker, &park, &alice).await?;

    let add_nft_contract_args = json!({ "nft_contract": nft.id(), "role": "creator" }).to_string();
    multi_ownable_call(
        &worker,
        &park,
        &root,
        "add_nft_contract",
        add_nft_contract_args.as_str(),
    )
    .await?;

    // alice creates proposal
    create_proposal(&worker, &park, &nft, &alice, token_id).await?;
    let nft_count = nft_tokens_for_owner(&worker, &nft, park.id().clone()).await?;
    assert_eq!(nft_count, 1, "wrong number of NFTs");

    now(&worker, &park).await?;

    stakes(&worker, &park).await?;

    proposals(&worker, &park).await?;

    // root votes on proposal
    let prop_id = format!("{}||{}", nft.id(), token_id);
    println!("PROPS ID {:?}", prop_id);
    vote_on_proposal(&worker, &park, &nft, &root, token_2, prop_id.as_str()).await?;
    let nft_count = nft_tokens_for_owner(&worker, &nft, park.id().clone()).await?;
    assert_eq!(nft_count, 2, "wrong number of NFTs");

    stakes(&worker, &park).await?;

    stakes_for_proposal_id(&worker, &park, prop_id.clone()).await?;

    // sleep(Duration::from_millis(epoch_length)).await;

    complete_proposal(&worker, &park, &nft, &alice, token_id).await?;

    stakes_for_proposal_id(&worker, &park, prop_id).await?;

    stakes(&worker, &park).await?;

    let p1 = nft_tokens_for_owner(&worker, &nft, park.id().clone()).await?;
    println!("PARK TOKENS {}", p1);

    let p2 = nft_tokens_for_owner(&worker, &nft, alice.id().clone()).await?;
    println!("ALICE TOKENS {}", p2);

    let p3 = nft_tokens_for_owner(&worker, &nft, root.id().clone()).await?;
    println!("VOTER TOKENS {}", p3);

    let nft_count = nft_tokens_for_owner(&worker, &nft, park.id().clone()).await?;
    assert_eq!(nft_count, 0, "wrong number of NFTs");

    println!("SUCCESS!");
    Ok(())
}

async fn _balance_of(
    worker: &Worker<impl Network>,
    park: &Contract,
    account: &Account,
) -> anyhow::Result<String> {
    let balance: String = worker
        .view(
            park.id().clone(),
            "ft_balance_of",
            json!({
                "account_id": account.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    Ok(balance)
}

async fn check_and_register_storage(
    worker: &Worker<impl Network>,
    park: &Contract,
    account: &Account,
) -> anyhow::Result<()> {
    let storage_balance: Value = worker
        .view(
            park.id().clone(),
            "storage_balance_of",
            json!({
                "account_id": account.id(),
            })
            .to_string()
            .into_bytes(),
        )
        .await?
        .json()?;
    println!("storage balance: {:?}", storage_balance);

    if storage_balance.is_null() {
        register_storage(&worker, &park, &account).await?;
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
struct StorageBalanceBounds {
    min: U128,
    max: U128,
}

async fn register_storage(
    worker: &Worker<impl Network>,
    park: &Contract,
    account: &Account,
) -> anyhow::Result<()> {
    let storage_prices: StorageBalanceBounds = worker
        .view(
            park.id().clone(),
            "storage_balance_bounds",
            json!({}).to_string().into_bytes(),
        )
        .await?
        .json()?;
    // println!("storage_balance_bounds: {:?}", storage_prices);

    let outcome = account
        .call(&worker, park.id().clone(), "storage_deposit")
        .args_json(json!({
            "account_id": account.id(),
            "registration_only": true,
        }))?
        .deposit(storage_prices.min.into())
        .transact()
        .await?;
    println!("storage_deposit: {:#?}", outcome);
    Ok(())
}

pub type ProposalId = String;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub owner_id: AccountId,
    pub name: String,
    pub lifetime: u16, // in days
    pub metadata: Option<Metadata>,
    pub tasks: Vec<Metadata>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
    pub budget: U128,
    pub ft_contract: Option<AccountId>,
    pub name: Option<String>,
    pub note: Option<String>,
    pub url: Option<String>,
    pub contact: Option<String>,
    pub media: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NftCall {
    pub proposal_id: Option<ProposalId>,
    pub proposal: Option<Proposal>,
}

async fn create_proposal(
    worker: &Worker<impl Network>,
    park: &Contract,
    nft: &Contract,
    account: &Account,
    token_id: &str,
) -> anyhow::Result<()> {
    let args: NftCall = NftCall {
        proposal_id: None,
        proposal: Some(Proposal {
            owner_id: account.id().clone(),
            name: "to do a thing".to_string(),
            lifetime: 1,
            metadata: None,
            tasks: Vec::new(),
        }),
    };
    let outcome = account
        .call(&worker, nft.id().clone(), "nft_transfer_call")
        .args_json(json!({
            "receiver_id": park.id(),
            "token_id": token_id,
            "msg": serde_json::to_string(&args).unwrap(),
        }))?
        .deposit(1)
        .gas(parse_gas!("200 Tgas") as u64)
        .transact()
        .await?;
    println!("nft_transfer_call: {:#?}", outcome);
    Ok(())
}

async fn vote_on_proposal(
    worker: &Worker<impl Network>,
    park: &Contract,
    nft: &Contract,
    account: &Account,
    token_id: &str,
    proposal_id: &str,
) -> anyhow::Result<()> {
    let args: NftCall = NftCall {
        proposal_id: Some(proposal_id.to_string()),
        proposal: None,
    };
    let outcome = account
        .call(&worker, nft.id().clone(), "nft_transfer_call")
        .args_json(json!({
            "receiver_id": park.id(),
            "token_id": token_id,
            "msg": serde_json::to_string(&args).unwrap(),
        }))?
        .deposit(1)
        .gas(parse_gas!("100 Tgas") as u64)
        .transact()
        .await?;
    println!("nft_transfer_call: {:#?}", outcome);
    Ok(())
}

pub async fn multi_ownable_call(
    worker: &Worker<impl Network>,
    park: &Contract,
    account: &Account,
    call: &str,
    args: &str,
) -> anyhow::Result<()> {
    let outcome = account
        .call(&worker, park.id().clone(), "multi_ownable_call")
        .args_json(json!({ "call_name": call, "arguments": args }))?
        .transact()
        .await?;
    println!("park multi_ownable_call: {:#?}", outcome);
    Ok(())
}

async fn now(worker: &Worker<impl Network>, park: &Contract) -> anyhow::Result<()> {
    let n: Value = worker
        .view(park.id().clone(), "now", Vec::new())
        .await?
        .json()?;
    println!("now: {:?}", n);

    Ok(())
}

async fn nft_tokens_for_owner(
    worker: &Worker<impl Network>,
    nft: &Contract,
    owner: AccountId,
) -> anyhow::Result<usize> {
    let n: Vec<Value> = worker
        .view(
            nft.id().clone(),
            "nft_tokens_for_owner",
            json!({ "account_id": owner }).to_string().into_bytes(),
        )
        .await?
        .json()?;
    // println!("nft_tokens_for_owner: {:?}", n);
    Ok(n.len())
}

async fn stakes(worker: &Worker<impl Network>, park: &Contract) -> anyhow::Result<()> {
    let ss: HashMap<String, Value> = worker
        .view(park.id().clone(), "stakes", Vec::new())
        .await?
        .json()?;
    println!("stakes: {:?}", ss);

    Ok(())
}

async fn proposals(worker: &Worker<impl Network>, park: &Contract) -> anyhow::Result<()> {
    let props: Vec<Value> = worker
        .view(park.id().clone(), "proposals", Vec::new())
        .await?
        .json()?;
    println!("proposals: {:?}", props);

    Ok(())
}

async fn stakes_for_proposal_id(
    worker: &Worker<impl Network>,
    park: &Contract,
    prop_id: ProposalId,
) -> anyhow::Result<()> {
    let ss: HashMap<String, Value> = worker
        .view(
            park.id().clone(),
            "stakes_for_proposal_id",
            json!({ "proposal_id": prop_id }).to_string().into_bytes(),
        )
        .await?
        .json()?;
    println!("stakes_for_proposal_id: {:?}", ss);

    Ok(())
}

pub async fn complete_proposal(
    worker: &Worker<impl Network>,
    park: &Contract,
    nft: &Contract,
    account: &Account,
    token_id: &str,
) -> anyhow::Result<()> {
    let outcome = account
        .call(&worker, park.id().clone(), "complete_proposal")
        .args_json(json!({ "nft_contract": nft.id(), "token_id": token_id }))?
        .gas(parse_gas!("300 Tgas") as u64)
        .transact()
        .await?;
    println!("park complete_proposal: {:#?}", outcome);
    Ok(())
}
