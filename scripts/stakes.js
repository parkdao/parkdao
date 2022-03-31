import * as utils from "./utils.js";
import * as constants from "./constants.js";

async function get_stakes() {
  const { park } = await utils.contracts();
  const stakes = await park.stakes();
  console.log(stakes);
}
get_stakes();
