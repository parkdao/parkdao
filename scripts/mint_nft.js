import * as utils from "./utils.js";
import * as constants from "./constants.js";
import Big from "big.js";
import { NFTS } from "./nfts.js";

// 10000000000000000000000
const MINT_GAS = Big(1).times(10 ** 22);

async function mint(token_id, media) {
  if (!token_id || !media) return console.log("MISSING A PARAM");
  const { nft } = await utils.contracts();

  // let token_id = "1";
  const minted = await nft.nft_mint({
    amount: MINT_GAS.toFixed(),
    args: {
      token_id,
      receiver_id: constants.ACCOUNT_ID,
      token_metadata: {
        title: "Tile #" + token_id,
        description: "Park DAO Tile",
        copies: 1,
        media: media,
      },
    },
  });
  console.log(minted);
}

for (let i = 0; i < NFTS.length; i++) {
  const nft = NFTS[i];
  await mint(nft.id, nft.url);
}
