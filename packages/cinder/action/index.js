const axios = require("axios");
const core = require("@actions/core");

(async () => {
  try {
    const CINDER_ENDPOINT = core.getInput("cinder-endpoint");
    const data = (await axios.get(CINDER_ENDPOINT)).data;
    core.setOutput("affected", data);
  } catch (error) {
    core.setFailed(error.message);
  }
})();
