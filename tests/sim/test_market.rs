use crate::utils::{init, TOKEN_ID};
// use near_contract_standards::non_fungible_token::Token;
use near_sdk::serde::Serialize;
use near_sdk::{serde_json, AccountId};
use near_sdk_sim::{call, view};

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
struct SaleArgs {
    price: u128,
    exp: u64,
    ft_contract: Option<AccountId>,
}

#[test]
fn simulate_market_listing() {
    let (root, nft, _, _, market) = init();
    // let near_id = AccountId::new_unchecked("near".to_string());
    let a = SaleArgs {
        price: 12,
        exp: 9999999999,
        ft_contract: None,
    };
    let msg = serde_json::to_string(&a).unwrap();
    call!(
        root,
        nft.nft_approve(TOKEN_ID.into(), market.account_id(), Some(msg.to_string())),
        // note that token_receiver's account name is longer, and so takes more bytes to store and
        // therefore requires a larger deposit! each character is one more
        deposit = 180_000_000_000_000_000_000
    )
    .assert_success();

    let alice_approved: bool =
        view!(nft.nft_is_approved(TOKEN_ID.into(), market.account_id(), None)).unwrap_json();
    assert!(alice_approved);
}
