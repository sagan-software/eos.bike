import ScatterJS from "scatterjs-core";
import ScatterEOS from "scatterjs-plugin-eosjs";

ScatterJS.plugins(new ScatterEOS());

export function connect() {
  console.log("CONNECT 1");
  ScatterJS.scatter
    .connect(
      "eos.bike",
      { initTimeout: 10000 }
    )
    .then(connected => {
      console.log("CONNECT 2");
      if (!connected) {
        console.log("CONNECT 3");
        // User does not have Scatter installed/unlocked.
        return false;
      }

      console.log("CONNECT 4");
      // Use `scatter` normally now.
      // ScatterJS.scatter.getIdentity(...);
    });
}
