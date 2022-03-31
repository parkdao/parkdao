import * as utils from "./utils.js";
import * as constants from "./constants.js";
import Big from "big.js";

const SETTLE_GAS = Big(45).times(10 ** 19);

async function settle() {
  const { market } = await utils.contracts();
  let token_id = "0";
  const settled = await market.settle({
    args: {
      nft_contract: constants.NFT_ID,
      token_id,
    },
    deposit: SETTLE_GAS.toFixed(),
  });
  console.log(settled);
}
settle();
