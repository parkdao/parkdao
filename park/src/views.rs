use crate::*;
use std::collections::HashMap;

#[near_bindgen]
impl Contract {
  pub fn proposal_by_name(&self, name: String) -> Option<proposals::Proposal> {
    self
      .stakes
      .iter()
      .filter(|s| s.1.proposal.is_some())
      .map(|s| s.1.proposal.unwrap())
      .find(|p| p.name == name)
  }
  pub fn proposals(&self) -> Vec<proposals::Proposal> {
    self
      .stakes
      .iter()
      .filter(|s| s.1.proposal.is_some())
      .map(|s| s.1.proposal.unwrap())
      .collect()
  }
  pub fn stakes(&self) -> HashMap<ContractAndTokenId, stake::Stake> {
    let mut ret = HashMap::new();
    self.stakes.iter().for_each(|s| {
      ret.insert(s.0, s.1);
    });
    ret
  }
  pub fn stakes_for_proposal_id(
    &self,
    proposal_id: ContractAndTokenId,
  ) -> HashMap<ContractAndTokenId, stake::Stake> {
    let mut ret = HashMap::new();
    self
      .stakes
      .iter()
      .filter(|s| {
        if let Some(pid) = &s.1.proposal_id {
          return &proposal_id == pid;
        }
        false
      })
      .for_each(|s| {
        ret.insert(s.0, s.1);
      });
    ret
  }
  pub fn my_stakes(&self) -> Vec<stake::Stake> {
    let caller = env::predecessor_account_id();
    self
      .stakes
      .iter()
      .map(|s| s.1)
      .filter(|s| s.staker_id == caller)
      .collect()
  }
}
