use crate::*;
// use near_contract_standards::non_fungible_token::Token;

#[ext_contract(ext_contract)]
pub trait ExtContract {
  fn nft_transfer(
    &self,
    receiver_id: AccountId,
    token_id: String,
    approval_id: Option<u64>,
    memo: Option<String>,
  ) -> Vec<Token>;
  fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token>;
  fn nft_tokens_for_owner(
    &self,
    account_id: AccountId,
    from_index: Option<U128>,
    limit: Option<u64>,
  ) -> Vec<Token>;
}
