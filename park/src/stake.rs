use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_contract_standards::non_fungible_token::core::NonFungibleTokenReceiver;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Stake {
  pub staker_id: AccountId,
  pub proposal_id: Option<ContractAndTokenId>, // vote on another stake (proposal)
  pub proposal: Option<proposals::Proposal>, // create a new proposal
  pub time: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SupportedNft {
  pub contract_id: AccountId,
  pub role: NftRole,
}

#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub enum NftAction {
  Stake,
  Create,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum NftRole {
  #[serde(rename = "voter", alias = "voter")]
  Voter,
  #[serde(rename = "creator", alias = "creator")]
  Creator,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NftCall {
  pub proposal_id: Option<ContractAndTokenId>, // the proposal to vote on
  pub proposal: Option<proposals::Proposal>, // a new proposal
}

#[near_bindgen]
impl NonFungibleTokenReceiver for Contract {
  // stake
  fn nft_on_transfer(
    &mut self,
    sender_id: AccountId,
    previous_owner_id: AccountId,
    token_id: TokenId,
    msg: String,
  ) -> PromiseOrValue<bool> {
    let nft_contract = env::predecessor_account_id();
    let caller = previous_owner_id;
    let call: NftCall = near_sdk::serde_json::from_str(&msg).expect("invalid StakeCall");
    let proposal = call.proposal;
    let proposal_id = call.proposal_id;
    let contract_and_token_id = format!("{}{}{}", nft_contract, DELIMETER, token_id);
    let action = if proposal.is_some() {
      let stake = self.stakes.get(&contract_and_token_id);
      require!(stake.is_none(), "already a proposal by that id");
      let existing = self.proposal_by_name(proposal.clone().unwrap().name);
      require!(existing.is_none(), "already a proposal with that name");
      NftAction::Create
    } else {
      require!(proposal_id.is_some(), "must include proposal id");
      let stake = self.stakes.get(&proposal_id.clone().unwrap());
      require!(stake.is_some(), "no matching proposal");
      NftAction::Stake
    };
    self.assert_nft_can_act(nft_contract.clone(), action.clone());
    self.stakes.insert(
      &contract_and_token_id.clone(),
      &Stake {
        staker_id: caller,
        proposal_id: proposal_id,
        proposal: proposal,
        time: self.now(),
      },
    );
    PromiseOrValue::Value(false) // return false = success
  }
}

impl SupportedNft {
  pub fn can(&self, action: NftAction) -> bool {
    self.role.acl().contains(&action)
  }
}

impl NftRole {
  pub fn acl(&self) -> Vec<NftAction> {
    match self {
      NftRole::Voter => vec![NftAction::Stake],
      NftRole::Creator => vec![NftAction::Stake, NftAction::Create],
    }
  }
}

#[near_bindgen]
impl Contract {
  #[private]
  pub(crate) fn assert_nft_can_act(
    &self,
    nft_contract: AccountId,
    action: NftAction,
  ) -> SupportedNft {
    let nfts = self.supported_nfts().to_vec();
    let contract = nfts.iter().find(|n| n.contract_id == nft_contract);
    require!(contract.is_some(), "invalid nft contract");
    let nft_contract = contract.unwrap();
    require!(nft_contract.can(action.clone()));
    nft_contract.to_owned()
  }
}
