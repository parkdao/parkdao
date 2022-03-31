import * as utils from "./utils.js";
import * as constants from "./constants.js";
import Big from "big.js";

// 10000000000000000000000000
const MINT_GAS = Big(1).times(10 ** 22);

async function mint() {
  const { park } = await utils.contracts();

  const minted = await park.mint({
    // amount: MINT_GAS.toFixed(),
    args: {
      amount: "100000000000000000000000000",
    },
  });
  console.log(minted);
}
mint();
