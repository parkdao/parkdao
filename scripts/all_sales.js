import * as utils from "./utils.js";
import * as constants from "./constants.js";
import Big from "big.js";

const BID_GAS = Big(45).times(10 ** 19);

async function settle() {
  const { market } = await utils.contracts();
  const sales = await market.get_all_sales();
  console.log(sales);
}
settle();
