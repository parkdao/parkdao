[![parkdao](https://parkdao.sfo3.digitaloceanspaces.com/media/3bjKUasf7SVHjvFA3W25wohua7NbkrvYpmvFhsix9ek3)](https://parkdao.xyz/world)

This repository contains 3 NEAR smart contracts that compose the parkdao ecosystem. Frontend code is [here](https://github.com/middlew4y/parkdao-platform)

### park

 - `NEP 141` Fungible Token
 - staking pool for parkdao NFTs, using `nft_on_transfer`
 - DAO contract with proposals and voting
 - core Council based on [multi-ownable](https://crates.io/crates/multi-ownable)

### market
 
 - marketplace for NFTs using `approvals`
 - auctions can be settled without owner being online

### nft

 - `NEP 171` NFT contract

### build

`./build.sh`

### test

`cargo test -- --nocapture`

Integration testing is built with the [workspaces.rs](https://github.com/near/workspaces-rs) framework.


