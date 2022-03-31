import * as utils from "./utils.js";
import Big from "big.js";

const TO = "evanfeenstra.testnet";

async function mint() {
  const { nft } = await utils.contracts();

  const trans = await nft.nft_transfer({
    amount: Big(1).toFixed(),
    gas: Big(10)
      .times(10 ** 12)
      .toFixed(),
    args: {
      receiver_id: TO,
      token_id: "0",
    },
  });
  console.log(trans);
}
mint();
