import * as utils from "./utils.js";
import * as constants from "./constants.js";
import Big from "big.js";
import { NFTS } from "./nfts.js";

const LIST_GAS = Big(45).times(10 ** 19);

async function list(token_id) {
  const { nft } = await utils.contracts();
  const extra = parseInt(token_id);
  const args = {
    price: 10 + extra * 2,
    exp: 0, // Date.now() + 10000, // in 10 seconds
  };
  const listed = await nft.nft_approve({
    amount: LIST_GAS.toFixed(),
    args: {
      token_id,
      account_id: constants.MARKET_ID, // approve for the market
      msg: JSON.stringify(args),
    },
  });
  console.log(listed);
}

for (let i = 0; i < NFTS.length; i++) {
  const nft = NFTS[i];
  await list(nft.id);
}
