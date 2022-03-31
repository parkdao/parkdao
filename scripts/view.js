import * as utils from "./utils.js";

async function view() {
  const { nft } = await utils.contracts();

  const nft_tokens = await nft.nft_tokens({
    args: {
      from_index: "0",
      limit: "1000",
    },
  });
  console.log("nft_tokens", nft_tokens);
}
view();
