// External fault injection only. The exact proposed recipe has no test hooks.
const fs = require("node:fs");
const path = require("node:path");
const { syncBuiltinESMExports } = require("node:module");
const root = process.env.FAULT_SANDBOX_ROOT;
const scenario = process.env.FAULT_SCENARIO;
if (!root || !["install2", "install3", "install3-rollback1", "symlink-parent"].includes(scenario)) throw new Error("invalid fault-control setup");
const rename = fs.renameSync;
let installations = 0;
let failedRestore = false;
const event = entry => fs.appendFileSync(path.join(root, "fault-events.ndjson"), `${JSON.stringify(entry)}\n`);
fs.renameSync = function (from, to) {
  const name = path.basename(String(from));
  if (/^\d+\.new$/.test(name)) {
    installations += 1;
    event({ operation: "install", ordinal: installations, from: name, to: path.basename(String(to)) });
    const failAt = scenario === "install2" ? 2 : scenario.startsWith("install3") ? 3 : -1;
    if (installations === failAt) {
      const error = new Error(`INJECTED_INSTALL_${failAt}`); error.code = "EIO"; throw error;
    }
  } else if (/^\d+\.old$/.test(name)) {
    event({ operation: "restore", from: name, to: path.basename(String(to)) });
    if (scenario === "install3-rollback1" && !failedRestore) {
      failedRestore = true;
      const error = new Error("INJECTED_RESTORE_1"); error.code = "EIO"; throw error;
    }
  }
  return rename.call(fs, from, to);
};
syncBuiltinESMExports();
