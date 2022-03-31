import * as utils from "./utils.js";

async function list() {
  const { nft } = await utils.contracts();
  const tokens = await nft.nft_tokens();
  console.log(tokens);
}
list();
