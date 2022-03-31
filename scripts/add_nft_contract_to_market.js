import * as utils from "./utils.js";
import * as constants from "./constants.js";

async function add_nft_contract() {
  const { market } = await utils.contracts();
  const added = await market.add_nft_contract({
    args: {
      nft_contract: constants.NFT_ID,
    },
  });
  console.log(added);
}
add_nft_contract();
