use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum MultiOwnableCall {
  #[serde(rename = "update_weights", alias = "update_weights")]
  UpdateWeights,
  #[serde(rename = "set_reward_rate", alias = "set_reward_rate")]
  SetRewardRate,
  #[serde(rename = "add_nft_contract", alias = "add_nft_contract")]
  AddNftContract,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SetRewardRateArgs {
  pub rate: u16,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AddNftContractArgs {
  pub nft_contract: AccountId,
  pub role: stake::NftRole,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct UpdateWeightsArgs {
  pub proposal_id: String,
}

#[near_bindgen]
impl Contract {
  // FIXME: remove mint
  pub fn mint(&mut self, amount: U128) {
    let caller = env::predecessor_account_id();
    self.assert_is_owner();
    self.token.internal_deposit(&caller, amount.into());
  }

  pub(crate) fn _set_reward_rate(&mut self, args: &str) {
    let SetRewardRateArgs { rate } =
      near_sdk::serde_json::from_str(&args).expect("Invalid SetRewardRateArgs");
    assert!(rate <= 1000, "Rate too high");
    self.reward_rate = rate;
  }

  pub(crate) fn _add_nft_contract(&mut self, args: &str) {
    let AddNftContractArgs { nft_contract, role } =
      near_sdk::serde_json::from_str(&args).expect("Invalid AddNftContractArgs");
    self.nfts.insert(&stake::SupportedNft {
      role: role,
      contract_id: nft_contract,
    });
  }

  pub(crate) fn _update_weights(&mut self, args: &str) {
    let UpdateWeightsArgs { proposal_id: _ } =
      near_sdk::serde_json::from_str(&args).expect("Invalid UpdateWeightsArgs");
  }
}
