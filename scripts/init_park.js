import * as utils from "./utils.js";
import * as constants from "./constants.js";
import Big from "big.js";

// 10^12 is one TGas
const BASE_GAS = Big(5).times(10 ** 12);
const GAS_30 = BASE_GAS.times(6);

async function init_park() {
  const { park } = await utils.contracts();
  console.log(park);
  // const r1 = await nft.nft_tokens();
  // console.log(r1);
  const r1 = await park.new_default_meta({
    args: {
      owner_id: constants.ACCOUNT_ID,
      total_supply: "1000000000000",
      epoch_millis: 60000,
    },
    gas: GAS_30.toFixed(),
  });
  console.log("park new_default_meta", r1);
}
init_park();
