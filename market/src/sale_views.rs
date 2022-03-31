use crate::*;
use near_sdk::json_types::U64;
use near_sdk::near_bindgen;

#[near_bindgen]
impl Market {
    
    //returns the number of sales the marketplace has up (as a string)
    pub fn get_supply_sales(&self) -> U64 {
        U64(self.sales.len())
    }

    pub fn get_all_sales(&self) -> Vec<Sale> {
        self.sales.to_vec().iter().map(|t| t.1.clone()).collect()
    }

    pub fn get_sales_by_contract(&self, nft_contract: AccountId) -> Vec<Sale> {
        self.sales.to_vec().iter().filter(|t| {
            let split: Vec<&str> = t.0.split(DELIMETER).collect();
            let nft_id = split.get(0).unwrap();
            nft_id.to_string() == nft_contract.to_string()
        }).map(|t| t.1.clone()).collect()
    }

    pub fn get_sale(&self, contract_and_token: ContractAndTokenId) -> Option<Sale> {
        self.sales.get(&contract_and_token)
    }

}