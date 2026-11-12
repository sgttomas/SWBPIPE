// The evidence journeys' runner refuses before anything is built or launched when it is not inside a held
// host slot, however its ancestors are named, and while the committed launch guard is absent. The runner is
// started for real; it reaches neither a build nor a launch (`node --test`).
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const runner = path.join(path.dirname(fileURLToPath(import.meta.url)), "evidence-journeys.mjs");

test("findings 3 and 7: started from a stand-in script named t3_slot_run.sh, the runner refuses before any build or launch", () => {
  const dir = mkdtempSync(path.join(os.tmpdir(), "swbpipe-evidence-runner-test-"));
  try {
    // A stand-in for the slot wrapper's job script: it has the wrapper's name in its arguments and holds no slot.
    const standIn = path.join(dir, "t3_slot_run.sh");
    writeFileSync(standIn, `#!/bin/bash\n"${process.execPath}" "${runner}" --all\n`);
    chmodSync(standIn, 0o755);
    const result = spawnSync("/bin/bash", [standIn, "mark", "cargo_job.slot2.lock"], { encoding: "utf8", timeout: 60_000, env: { PATH: process.env.PATH ?? "/usr/bin:/bin" } });
    const said = `${result.stdout}\n${result.stderr}`;
    assert.equal(result.status, 2, said);
    // Outside macOS it refuses as macOS only; on a host where the test itself runs inside a held slot, the
    // slot check passes and the launch guard seam refuses; otherwise the slot check refuses.
    assert.match(said, /evidence journeys: (macOS only|not inside a held host slot|refused: the committed launch guard is not present)/);
    if (process.platform === "darwin" && !/inside host slot/.test(said)) assert.match(said, /not inside a held host slot/);
    assert.doesNotMatch(said, /^\$ \(/m, "no build command was started");
    assert.doesNotMatch(said, /launching|== .*: phase/, "nothing was launched");
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});
