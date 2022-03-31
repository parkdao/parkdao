import * as utils from "./utils.js";
import * as constants from "./constants.js";
import Big from "big.js";

// 10^12 is one TGas
const BASE_GAS = Big(5).times(10 ** 12);
const GAS_30 = BASE_GAS.times(6);

async function init_market() {
  const { market } = await utils.contracts();
  console.log(market);
  // const r1 = await nft.nft_tokens();
  // console.log(r1);
  const r1 = await market.new({
    args: {
      owner_id: constants.ACCOUNT_ID,
      nft_contract: constants.NFT_ID,
    },
    gas: GAS_30.toFixed(),
  });
  console.log("market new_default_meta", r1);
}
init_market();
