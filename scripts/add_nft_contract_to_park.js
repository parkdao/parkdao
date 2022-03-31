import * as utils from "./utils.js";
import * as constants from "./constants.js";

async function add_nft_contract() {
  const { park } = await utils.contracts();
  const added = await park.multi_ownable_call({
    args: {
      call_name: "add_nft_contract",
      arguments: JSON.stringify({
        role: "creator",
        nft_contract: constants.NFT_ID,
      }),
    },
  });
  console.log(added);
}
add_nft_contract();
