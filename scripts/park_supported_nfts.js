import * as utils from "./utils.js";

async function supported_nfts() {
  const { park } = await utils.contracts();
  const nfts = await park.supported_nfts();
  console.log(nfts);
}
supported_nfts();
