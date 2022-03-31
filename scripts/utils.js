import * as NEAR from "near-api-js";
import * as OS from "os";
import * as constants from "./constants.js";

export async function contracts() {
  const { near, account } = await connect();
  const accid = account.accountId;
  const nft = nftContract(account, accid);
  const market = marketContract(account, accid);
  const park = parkContract(account, accid);
  return { near, nft, market, park };
}

export function nftContract(account, accountId) {
  const contract = new NEAR.Contract(account, constants.NFT_ID, {
    viewMethods: [
      "nft_token",
      "nft_tokens",
      "nft_supply_for_owner",
      "nft_tokens_for_owner",
      "nft_metadata",
      "nft_is_approved",
      "nft_total_supply",
    ],
    changeMethods: [
      "new",
      "new_default_meta",
      "nft_mint",
      "nft_transfer",
      "nft_transfer_call",
      "nft_approve",
      "nft_revoke",
      "nft_revoke_all",
    ],
    sender: accountId,
  });
  return contract;
}

export function marketContract(account, accountId) {
  return new NEAR.Contract(account, constants.MARKET_ID, {
    viewMethods: [
      "supported_ft_contracts",
      "supported_nft_contracts",
      "get_now",
      "get_supply_sales",
      "get_all_sales",
      "get_sales_by_contract",
      "get_sale",
    ],
    changeMethods: [
      "new",
      "add_nft_contract",
      "add_ft_contract",
      "bid",
      "settle",
    ],
    sender: accountId,
  });
}

export function parkContract(account, accountId) {
  return new NEAR.Contract(account, constants.PARK_ID, {
    viewMethods: [
      "genesis",
      "reward_rate",
      "epoch_of",
      "ft_balance_of",
      "ft_total_supply",
      "supported_nfts",
      "stakes",
      "proposals",
    ],
    changeMethods: [
      "new",
      "new_default_meta",
      "mint",
      "set_reward_rate",
      "reward",
      "ft_transfer",
      "ft_transfer_call",
      "multi_ownable_call",
      "unstake",
    ],
    sender: accountId,
  });
}

export async function connect() {
  const homedir = OS.homedir();
  const credentialsPath = homedir + "/.near-credentials";
  const keyStore = new NEAR.keyStores.UnencryptedFileSystemKeyStore(
    credentialsPath
  );
  const near = await NEAR.connect(constants.config(keyStore));
  const account = await near.account(constants.ACCOUNT_ID);
  return { near, account };
}
