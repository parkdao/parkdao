use crate::*;

#[ext_contract(ext_contract)]
trait ExtContract {
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: u128, memo: Option<String>);
}
