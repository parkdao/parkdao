use crate::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance, Promise, PromiseResult};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Bid {
    pub owner_id: AccountId,
    pub price: Balance,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Sale {
    pub owner_id: AccountId,
    pub approval_id: u64,
    pub nft_contract: AccountId,
    pub token_id: String,
    pub ft_contract: AccountId,
    pub min_price: Balance,
    pub bid: Option<Bid>,
    pub created_at: U64,
    pub exp: U64, // expiry timestamp. 0 = not an auction
}

#[near_bindgen]
impl Market {
    #[payable]
    pub fn bid(&mut self, nft_contract: AccountId, token_id: String) {
        let contract_and_token_id = format!("{}{}{}", nft_contract, DELIMETER, token_id);
        let mut sale = self.sales.get(&contract_and_token_id).expect("No sale");
        let buyer_id = env::predecessor_account_id();
        assert_ne!(sale.owner_id, buyer_id, "Cannot bid on your own sale.");

        let ft_contract = AccountId::new_unchecked("near".to_string());
        assert_eq!(sale.ft_contract, ft_contract, "Not for sale in NEAR");

        let deposit = env::attached_deposit();
        assert!(deposit > 0, "Attached deposit must be greater than 0");

        let is_auction = sale.exp.0 > 0;

        if !is_auction && deposit >= sale.min_price {
            self.process_purchase(nft_contract, token_id, ft_contract, U128(deposit), buyer_id);
        } else {
            if is_auction && sale.min_price > 0 {
                assert!(
                    deposit >= sale.min_price,
                    "Attached deposit must be greater than min price"
                );
            }
            self.add_bid(contract_and_token_id, deposit, ft_contract, buyer_id, &mut sale);
        }
    }

    // the highest bid wins (if exp is passed). Anyone can call "settle"
    pub fn settle(&mut self, nft_contract: AccountId, token_id: String) {
        let contract_and_token_id = format!("{}{}{}", nft_contract, DELIMETER, token_id);
        let sale = self.sales.get(&contract_and_token_id).expect("No sale");
        assert!(sale.bid.is_some(), "No bid to settle");
        let bid = sale.bid.unwrap();
        assert!(sale.exp.0 > 0, "Can only settle auctions");
        assert!(sale.exp.0 < self.now(), "Can only settle after expiry time");
        self.process_purchase(nft_contract, token_id, sale.ft_contract, U128(bid.price), bid.owner_id);
    }

    #[private]
    pub fn process_purchase(
        &mut self,
        nft_contract: AccountId,
        token_id: String,
        ft_contract: AccountId,
        price: U128,
        buyer_id: AccountId,
    ) -> Promise {
        let sale = self.internal_remove_sale(nft_contract.clone(), token_id.clone());

        ext_contract::nft_transfer(
            buyer_id.clone(),
            sale.token_id,
            Some(sale.approval_id),
            None,
            sale.nft_contract,
            1,
            Gas(NFT_TRANSFER_GAS),
        )
        .then(ext_self::resolve_purchase(
            ft_contract,
            sale.owner_id,
            price,
            env::current_account_id(),
            NO_DEPOSIT,
            Gas(BASE_GAS),
        ))
    }

    #[private]
    pub fn add_bid(
        &mut self,
        contract_and_token_id: ContractAndTokenId,
        amount: Balance,
        ft_contract: AccountId,
        buyer_id: AccountId,
        sale: &mut Sale,
    ) {
        // store a bid and refund any current bid lower
        let new_bid = Bid {
            owner_id: buyer_id,
            price: amount,
        };
        if sale.bid.is_some() {
            let current_bid = sale.bid.clone().unwrap();
            assert!(
                amount > current_bid.price,
                "Can't pay less than or equal to current bid price: {}",
                current_bid.price
            );
            // refund
            self.payout(ft_contract, current_bid.owner_id, current_bid.price);
        }
        sale.bid = Some(new_bid);
        // update the storage
        self.sales.insert(&contract_and_token_id, &sale);
    }

    #[private]
    pub fn resolve_purchase(&mut self, ft_contract: AccountId, seller_id: AccountId, price: U128) {
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic_str("nft_transfer failed"),
            PromiseResult::Successful(_result) => {
                self.payout(ft_contract, seller_id, price.0);
            },
        }
    }

    #[private]
    pub fn payout(&self, ft_contract: AccountId, receiver: AccountId, amount: Balance) {
        if ft_contract.to_string() == "near" {
            Promise::new(receiver.clone()).transfer(u128::from(amount));
        } else {
            ext_contract::ft_transfer(receiver.clone(), amount, None, ft_contract, 1, Gas(BASE_GAS));
        }
    }

    pub(crate) fn internal_remove_sale(&mut self, nft_contract: AccountId, token_id: TokenId) -> Sale {
        let contract_and_token_id = format!("{}{}{}", &nft_contract, DELIMETER, token_id);
        let sale = self.sales.remove(&contract_and_token_id).expect("No sale");
        sale
    }
}

#[ext_contract(ext_self)]
trait ExtSelf {
    fn resolve_purchase(&mut self, ft_contract: AccountId, seller_id: AccountId, price: U128) -> Promise;
}
