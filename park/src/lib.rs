mod external;
mod owner_calls;
mod proposals;
mod stake;
mod views;

use crate::external::*;
use crate::owner_calls::MultiOwnableCall;

use multi_ownable::{impl_multi_ownable, MultiOwnableData};

use near_contract_standards::fungible_token::metadata::{
  FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::{
  env, ext_contract, log, near_bindgen, require, AccountId, Balance, BorshStorageKey, Gas,
  PanicOnDefault, PromiseOrValue,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
  genesis: u64,
  epoch_millis: u64,
  nfts: UnorderedSet<stake::SupportedNft>,
  stakes: UnorderedMap<ContractAndTokenId, stake::Stake>,
  reward_rate: u16, // inflation basis points per epoch
  token: FungibleToken,
  metadata: LazyOption<FungibleTokenMetadata>,
  multi_ownable: MultiOwnableData,
}

const DATA_IMAGE_SVG_ICON: &str = "data:image/svg+xml,%3Csvg id='Artwork' xmlns='http://www.w3.org/2000/svg' viewBox='0 0 598.6 369.78'%3E%3Cpath d='M159,209.67c-12.42,0-21.93,9-21.93,21.64v22.44h43.67V231.31C180.75,218.69,171.43,209.67,159,209.67Z'/%3E%3Cpath d='M401.43,228.52c-5.92,0-10.09,2.91-11.45,8.25h22.61C411.33,231.43,407.06,228.52,401.43,228.52Z'/%3E%3Cpath d='M225.86,228.52c-5.92,0-10.09,2.91-11.45,8.25H237C235.76,231.43,231.49,228.52,225.86,228.52Z'/%3E%3Cpath d='M299.3,70.48C172.92,70.48,70.48,172.92,70.48,299.3H528.12C528.12,172.92,425.67,70.48,299.3,70.48Zm-105,195.79H123.49V231.21c0-20.57,15.43-35.42,35.52-35.42s35.32,14.85,35.32,35.42Zm55.4-19.41h-35c1.65,4.85,6.21,8.15,12,8.15a12.44,12.44,0,0,0,9.69-4.43l11.86,4.91c-4.27,7.19-12.13,11.84-21.74,11.84-14.75,0-25.33-11.06-25.33-25.42,0-14.56,10.09-25.62,24.55-25.62s24.46,10.87,24.46,25.42A30,30,0,0,1,249.73,246.86Zm51.09,19.41h-13.2v-28c0-5.05-3.39-9.12-9-9.12a8.82,8.82,0,0,0-9.13,9.12v28H256.28V217.36h13v3.1a19.4,19.4,0,0,1,12.23-4.17c11.16,0,19.31,8.34,19.31,19.41Zm20.44,0h-13.2V217.36h13.2Zm-6.6-56.36a7.06,7.06,0,1,1,7.06-7.06A6.9,6.9,0,0,1,314.66,209.91Zm59.85,19.48-26.3,24.36h26.3v12.52H329.29v-12l26.3-24.35h-26.3V217.36h45.22Zm50.79,17.47h-35c1.65,4.85,6.21,8.15,12,8.15a12.44,12.44,0,0,0,9.69-4.43l11.86,4.91c-4.27,7.19-12.13,11.84-21.74,11.84-14.75,0-25.33-11.06-25.33-25.42,0-14.56,10.09-25.62,24.55-25.62s24.46,10.87,24.46,25.42A30,30,0,0,1,425.3,246.86Zm36.61,19.41v-28c0-5.05-3.4-9.12-9-9.12a8.81,8.81,0,0,0-9.12,9.12v28h-13.2V217.36h13v3.1a19.37,19.37,0,0,1,12.22-4.17c11.16,0,19.31,8.34,19.31,19.41v30.57Z'/%3E%3C/svg%3E";
// there are 2551443 seconds in a lunar month
// const LUNAR_SECONDS: u64 = 2551443;
// there are 637860750 milliseconds in a lunar "week"
// const LUNAR_WEEK_MILLIS: u64 = LUNAR_SECONDS*1000/4;
const BASE_GAS: u64 = 5_000_000_000_000;
const GAS_FOR_NFT_TRANSFER: u64 = 15_000_000_000_000;

// const TWO_WEEKS: u64 = 1_209_600_000; // millis
// const DAY_MILLIS: u64 = 86_400_000;
const DELIMETER: &str = "||";

pub type ContractAndTokenId = String;

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
  NFTContracts,
  Stakes,
  FungibleToken,
  Metadata,
  Owners,
  MultiOwnableCalls,
}

#[near_bindgen]
impl Contract {
  /// Initializes the contract with the given total supply owned by the given `owner_id` with
  /// default metadata (for example purposes only).
  #[init]
  pub fn new_default_meta(owner_id: AccountId, total_supply: U128, epoch_millis: u64) -> Self {
    Self::new(
      owner_id,
      total_supply,
      epoch_millis,
      FungibleTokenMetadata {
        spec: FT_METADATA_SPEC.to_string(),
        name: "Park Token".to_string(),
        symbol: "PARK".to_string(),
        icon: Some(DATA_IMAGE_SVG_ICON.to_string()),
        reference: None,
        reference_hash: None,
        decimals: 24,
      },
    )
  }

  #[init]
  pub fn new(
    owner_id: AccountId,
    total_supply: U128,
    epoch_millis: u64,
    metadata: FungibleTokenMetadata,
  ) -> Self {
    require!(!env::state_exists(), "Already initialized");
    metadata.assert_valid();
    let mut this = Self {
      genesis: 0,
      reward_rate: 0,
      epoch_millis: epoch_millis,
      stakes: UnorderedMap::new(StorageKey::Stakes),
      // proposals: UnorderedMap::new(StorageKey::Proposals),
      nfts: UnorderedSet::new(StorageKey::NFTContracts),
      token: FungibleToken::new(StorageKey::FungibleToken),
      metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata)),
      multi_ownable: MultiOwnableData::new(StorageKey::Owners, StorageKey::MultiOwnableCalls),
    };
    this.init_multi_ownable(vec![owner_id.clone()], 1);
    this.genesis = this.now();
    this.token.internal_register_account(&owner_id);
    this.token.internal_deposit(&owner_id, total_supply.into());
    this
  }

  pub fn now(&self) -> u64 {
    env::block_timestamp() / 1_000_000 // milliseconds
  }

  fn on_account_closed(&mut self, account_id: AccountId, balance: Balance) {
    log!("Closed @{} with {}", account_id, balance);
  }

  fn on_tokens_burned(&mut self, account_id: AccountId, amount: Balance) {
    log!("Account @{} burned {}", account_id, amount);
  }

  pub fn genesis(&self) -> u64 {
    self.genesis
  }

  pub fn reward_rate(&self) -> u16 {
    self.reward_rate
  }

  pub fn supported_nfts(&self) -> Vec<stake::SupportedNft> {
    self.nfts.to_vec()
  }

  fn on_call(&mut self, call_name: MultiOwnableCall, arguments: &str) {
    match call_name {
      MultiOwnableCall::SetRewardRate => self._set_reward_rate(arguments),
      MultiOwnableCall::UpdateWeights => self._update_weights(arguments),
      MultiOwnableCall::AddNftContract => self._add_nft_contract(arguments),
    }
  }
}

near_contract_standards::impl_fungible_token_core!(Contract, token, on_tokens_burned);
near_contract_standards::impl_fungible_token_storage!(Contract, token, on_account_closed);

crate::impl_multi_ownable!(Contract, multi_ownable, MultiOwnableCall, on_call);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
  fn ft_metadata(&self) -> FungibleTokenMetadata {
    self.metadata.get().unwrap()
  }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
  use near_sdk::test_utils::{accounts, VMContextBuilder};
  use near_sdk::{testing_env, Balance};

  use super::*;

  const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

  fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
      .current_account_id(accounts(0))
      .signer_account_id(predecessor_account_id.clone())
      .predecessor_account_id(predecessor_account_id);
    builder
  }

  #[test]
  fn test_new() {
    let mut context = get_context(accounts(1));
    testing_env!(context.build());
    let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into(), 30_000);
    testing_env!(context.is_view(true).build());
    assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
    assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
  }

  #[test]
  #[should_panic(expected = "The contract is not initialized")]
  fn test_default() {
    let context = get_context(accounts(1));
    testing_env!(context.build());
    let _contract = Contract::default();
  }

  #[test]
  fn test_transfer() {
    let mut context = get_context(accounts(2));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into(), 30_000);
    testing_env!(context
      .storage_usage(env::storage_usage())
      .attached_deposit(contract.storage_balance_bounds().min.into())
      .predecessor_account_id(accounts(1))
      .build());
    // Paying for account registration, aka storage deposit
    contract.storage_deposit(None, None);

    testing_env!(context
      .storage_usage(env::storage_usage())
      .attached_deposit(1)
      .predecessor_account_id(accounts(2))
      .build());
    let transfer_amount = TOTAL_SUPPLY / 3;
    contract.ft_transfer(accounts(1), transfer_amount.into(), None);

    testing_env!(context
      .storage_usage(env::storage_usage())
      .account_balance(env::account_balance())
      .is_view(true)
      .attached_deposit(0)
      .build());
    assert_eq!(
      contract.ft_balance_of(accounts(2)).0,
      (TOTAL_SUPPLY - transfer_amount)
    );
    assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
  }
}
