use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use near_sdk::{Promise, PromiseResult};
use std::str::FromStr;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
  pub name: String,  // must be unique
  pub lifetime: u16, // in days
  pub metadata: Option<Metadata>,
  pub tasks: Vec<Metadata>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Metadata {
  pub name: String,
  pub budget: Option<U128>,
  pub ft_contract: Option<AccountId>,
  pub note: Option<String>,
  pub url: Option<String>,
  pub contact: Option<String>,
  pub media: Option<String>,
}

#[ext_contract(ext_self)]
trait ExtSelf {
  fn resolve_send_nft(
    &self,
    to: AccountId,
    new_tokens: u128,
    nft_contract: AccountId,
    token_id: String,
  ) -> Promise;
}

#[near_bindgen]
impl Contract {
  pub fn complete_proposal(
    &mut self,
    nft_contract: AccountId,
    token_id: String,
  ) -> PromiseOrValue<U128> {
    let cat_id = format!("{}{}{}", nft_contract, DELIMETER, token_id);
    let stake = self.stakes.get(&cat_id).expect("stake not found");
    let prop = stake.proposal.expect("proposal must exist");
    let end_time = stake.time + (self.epoch_millis * prop.lifetime as u64);
    require!(self.now() > end_time, "proposal still active");

    let active_stakes = self.stakes_for_proposal_id(cat_id.clone());
    for s in active_stakes.iter() {
      let cat: Vec<&str> = s.0.split(DELIMETER).collect();
      let staked_nft_contract =
        AccountId::from_str(&cat[0].to_string()).expect("couldnt parse stake id");
      let staked_token_id = cat[1].to_string();
      self._send_nft(s.1.staker_id.clone(), staked_nft_contract, staked_token_id);
    }
    self
      ._send_nft(stake.staker_id.clone(), nft_contract, token_id)
      .into()
  }
  pub fn unstake(&self, nft_contract: AccountId, token_id: String) {
    let cat_id = format!("{}{}{}", nft_contract, DELIMETER, token_id);
    let stake = self.stakes.get(&cat_id).expect("no matching stake");
    let caller = env::predecessor_account_id();
    require!(stake.staker_id == caller, "unauthrized to unstake");

    let is_proposal = stake.proposal.is_some();
    if is_proposal {
      let prop = stake.proposal.unwrap();
      let end_time = stake.time + (self.epoch_millis * prop.lifetime as u64);
      require!(self.now() < end_time, "too early to unstake");
    } else {
      let min_time = stake.time + self.epoch_millis;
      require!(self.now() < min_time, "must stake for at least one day");
    }
    self._send_nft(caller, nft_contract, token_id);
  }
  #[private]
  pub(crate) fn _send_nft(
    &self,
    to: AccountId,
    nft_contract: AccountId,
    token_id: String,
  ) -> Promise {
    ext_contract::nft_transfer(
      to.clone(),
      token_id.clone(),
      None,
      None,
      nft_contract.clone(),
      1, // one yocto near required
      Gas(GAS_FOR_NFT_TRANSFER),
    )
    .then(ext_self::resolve_send_nft(
      to,
      1 as u128,
      nft_contract,
      token_id,
      env::current_account_id(),
      0,
      Gas(BASE_GAS),
    ))
  }
  #[private]
  pub fn resolve_send_nft(
    &mut self,
    to: AccountId,
    new_tokens: u128,
    nft_contract: AccountId,
    token_id: String,
  ) -> U128 {
    assert_eq!(env::promise_results_count(), 1, "This is a callback method");

    match env::promise_result(0) {
      PromiseResult::NotReady => unreachable!(),
      PromiseResult::Failed => env::panic_str("Unable to return token"),
      PromiseResult::Successful(_) => (),
    };
    let cat_id = format!("{}{}{}", nft_contract, DELIMETER, token_id);
    self.remove_and_reward_stake(cat_id, to, new_tokens);
    U128(new_tokens)
  }
  #[private]
  pub(crate) fn remove_and_reward_stake(
    &mut self,
    contract_and_token_id: ContractAndTokenId,
    to: AccountId,
    amount: u128,
  ) {
    self
      .stakes
      .remove(&contract_and_token_id)
      .expect("couldnt remove stake");
    self.token.internal_deposit(&to, amount.into());
  }
}
