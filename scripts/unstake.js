import * as utils from "./utils.js";
import * as constants from "./constants.js";
import Big from "big.js";

async function unstake() {
  const { park } = await utils.contracts();
  let token_id = "4";
  const unstaked = await park.unstake({
    args: {
      nft_contract: constants.NFT_ID,
      token_id,
    },
  });
  console.log(unstaked);
}
unstake();
