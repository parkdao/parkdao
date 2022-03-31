use crate::*;
use near_contract_standards::non_fungible_token::approval::NonFungibleTokenApprovalReceiver;
use near_sdk::json_types::U64;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{log, PromiseOrValue};

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
struct SaleArgs {
    price: u128,
    exp: u64,
    ft_contract: Option<AccountId>,
}

#[near_bindgen]
impl NonFungibleTokenApprovalReceiver for Market {
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) -> PromiseOrValue<String> {
        assert!(
            &self.nft_contracts.contains(&env::predecessor_account_id()),
            "Only supports some NFT contracts"
        );
        log!(
            "in nft_on_approve; sender_id={}, previous_owner_id={}, token_id={}, msg={}",
            &token_id,
            &owner_id,
            &approval_id,
            msg
        );
        // enforce cross contract call and owner_id is signer
        let nft_contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        assert_ne!(
            nft_contract_id, signer_id,
            "nft_on_approve should only be called via cross-contract call"
        );
        assert_eq!(owner_id, signer_id, "owner_id should be signer_id");
        let SaleArgs { price, exp, ft_contract } =
            near_sdk::serde_json::from_str(&msg).expect("Not valid SaleArgs");

        assert!(exp==0 || exp > self.now(), "exp should be in the future");

        let the_ft_contract = if ft_contract.is_some() {
            ft_contract.unwrap()
        } else {
            AccountId::new_unchecked("near".to_string())
        };
        if !self.ft_contracts.contains(&the_ft_contract) {
            env::panic_str("Token {} not supported by this market");
        }
        let contract_and_token_id = format!("{}{}{}", nft_contract_id, DELIMETER, token_id);
        self.sales.insert(
            &contract_and_token_id,
            &Sale {
                owner_id: owner_id.clone().into(),
                approval_id,
                nft_contract: nft_contract_id.clone(),
                token_id: token_id.clone(),
                ft_contract: the_ft_contract,
                bid: None,
                min_price: price,
                exp: U64(exp),
                created_at: U64(self.now()),
            },
        );

        // let prepaid_gas = env::prepaid_gas();
        // let account_id = env::current_account_id();
        PromiseOrValue::Value(contract_and_token_id)
    }
}
