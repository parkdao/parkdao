//
export const ACCOUNT_ID = "evanfeenstra.testnet";
// export const ACCOUNT_ID = "parkdao.testnet";

export const NFT_ID = "nft6.evanfeenstra.testnet";
export const MARKET_ID = "market3.evanfeenstra.testnet";
export const PARK_ID = "park3.evanfeenstra.testnet";

export function config(keyStore) {
  return {
    networkId: "testnet",
    keyStore, // optional if not signing transactions
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
    explorerUrl: "https://explorer.testnet.near.org",
  };
}
