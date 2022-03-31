use fungible_token::ContractContract as FungibleContract;
use market::MarketContract;
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_sdk::json_types::U128;
use near_sdk::AccountId;
use non_fungible_token::ContractContract as NftContract;

use near_contract_standards::non_fungible_token::TokenId;
use near_sdk_sim::{call, deploy, init_simulator, to_yocto, ContractAccount, UserAccount};

// Load in contract bytes at runtime
near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    NFT_WASM_BYTES => "out/non_fungible_token.wasm",
    FT_WASM_BYTES => "out/fungible_token.wasm",
    MARKET_WASM_BYTES => "out/market.wasm",
}

const NFT_ID: &str = "nft";
const FT_ID: &str = "ft";
const MARKET_ID: &str = "market";

// TODO: how to export String instead of &str? Way too much `into`/`to_string` with &str.
pub const TOKEN_ID: &str = "0";

/// Initialize simulator and return:
/// * root: the root user, set as owner_id for NFT contract, owns a token with ID=1
/// * nft: the NFT contract, callable with `call!` and `view!`
/// * alice: a user account, does not yet own any tokens
pub fn init() -> (
    UserAccount,
    ContractAccount<NftContract>,
    UserAccount,
    ContractAccount<FungibleContract>,
    ContractAccount<MarketContract>,
) {
    let root = init_simulator(None);
    // uses default values for deposit and gas
    let nft = deploy!(
        // Contract Proxy
        contract: NftContract,
        // Contract account id
        contract_id: NFT_ID,
        // Bytes of contract
        bytes: &NFT_WASM_BYTES,
        // User deploying the contract,
        signer_account: root,
        // init method
        init_method: new_default_meta(
            root.account_id()
        )
    );

    call!(
        root,
        nft.nft_mint(
            TOKEN_ID.into(),
            root.account_id(),
            TokenMetadata {
                title: Some("Olympus Mons".into()),
                description: Some("The tallest mountain in the charted solar system".into()),
                media: None,
                media_hash: None,
                copies: Some(1u64),
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
            }
        ),
        deposit = 7000000000000000000000
    );

    let alice = root.create_user(
        AccountId::new_unchecked("alice".to_string()),
        to_yocto("100"),
    );

    let token_receiver = deploy!(
        contract: FungibleContract,
        contract_id: FT_ID,
        bytes: &FT_WASM_BYTES,
        signer_account: root,
        init_method: new_default_meta(
            root.account_id(),
            U128(1_000_000_000_000_000)
        )
    );

    let approval_receiver = deploy!(
        contract: MarketContract,
        contract_id: MARKET_ID,
        bytes: &MARKET_WASM_BYTES,
        signer_account: root,
        init_method: new(
            root.account_id(),
            nft.account_id()
        )
    );

    (root, nft, alice, token_receiver, approval_receiver)
}

pub fn helper_mint(
    token_id: TokenId,
    root: &UserAccount,
    nft: &ContractAccount<NftContract>,
    title: String,
    desc: String,
) {
    call!(
        root,
        nft.nft_mint(
            token_id,
            root.account_id(),
            TokenMetadata {
                title: Some(title),
                description: Some(desc),
                media: None,
                media_hash: None,
                copies: Some(1u64),
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
            }
        ),
        deposit = 7000000000000000000000
    );
}
