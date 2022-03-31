mod external;
mod nft_on_approve;
mod sale;
mod sale_views;

use crate::external::*;
use crate::sale::*;

use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, UnorderedSet};
use near_sdk::json_types::U64;
use near_sdk::{
    env, ext_contract, near_bindgen, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault,
};

const BASE_GAS: u64 = 5_000_000_000_000;
const NFT_TRANSFER_GAS: u64 = 10_000_000_000_000;
const NO_DEPOSIT: Balance = 0;
static DELIMETER: &str = "||";

pub type ContractAndTokenId = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Market {
    pub owner_id: AccountId,
    pub nft_contracts: UnorderedSet<AccountId>,
    pub ft_contracts: UnorderedSet<AccountId>,
    pub sales: UnorderedMap<ContractAndTokenId, Sale>,
}

/// Helper structure to for keys of the persistent collections.
#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    Sales,
    NFTContractIds,
    FTTokenIds,
}

#[near_bindgen]
impl Market {
    #[init]
    pub fn new(owner_id: AccountId, nft_contract: AccountId) -> Self {
        let mut this = Self {
            owner_id: owner_id,
            nft_contracts: UnorderedSet::new(StorageKey::NFTContractIds),
            ft_contracts: UnorderedSet::new(StorageKey::FTTokenIds),
            sales: UnorderedMap::new(StorageKey::Sales),
        };
        // support near by default
        let near_id = AccountId::new_unchecked("near".to_string());
        this.ft_contracts.insert(&near_id);
        // initial nft contract
        this.nft_contracts.insert(&nft_contract);
        this
    }

    pub fn owner_id(&self) -> AccountId {
        self.owner_id.clone()
    }

    /// only owner
    pub fn add_nft_contract(&mut self, nft_contract: AccountId) -> bool {
        self.assert_owner();
        self.nft_contracts.insert(&nft_contract)
    }

    /// only owner
    pub fn add_ft_contract(&mut self, ft_contract: AccountId) -> bool {
        self.assert_owner();
        self.ft_contracts.insert(&ft_contract)
    }

    pub fn supported_ft_contracts(&self) -> Vec<AccountId> {
        self.ft_contracts.to_vec()
    }

    pub fn supported_nft_contracts(&self) -> Vec<AccountId> {
        self.nft_contracts.to_vec()
    }
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            &env::predecessor_account_id(),
            &self.owner_id,
            "Owner's method"
        );
    }

    pub(crate) fn now(&self) -> u64 {
        env::block_timestamp() / 1_000_000 // millis
    }

    pub fn get_now(&self) -> U64 {
        U64(self.now())
    }
}
