// The evidence journeys' runner, without a build or a launch (`node --test`): it refuses a command line it does not
// accept before it takes the build lock, and its source starts the app only through the shared launch pieces.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { readFileSync } from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const runner = path.join(here, "evidence-journeys.mjs");

test("a command line the runner does not accept is refused before the build lock, any build or any launch", () => {
  for (const args of [[], ["--skip-build", "--all"], ["--all", "--library-listing", "/x"], ["--build-only", "--all"], ["--journey", "reference-system,report-exact"]]) {
    const result = spawnSync(process.execPath, [runner, ...args], { encoding: "utf8", timeout: 60_000, env: { PATH: process.env.PATH ?? "/usr/bin:/bin" } });
    const said = `${result.stdout}\n${result.stderr}`;
    assert.equal(result.status, 2, `${args.join(" ")}: ${said}`);
    assert.match(said, /evidence journeys: (macOS only|unknown arguments|choose --all|--build-only builds|.*runs alone)/, said);
    assert.doesNotMatch(said, /^\$ \(/m, "no build command was started");
    assert.doesNotMatch(said, /launching|== .*: phase|build lock/, "neither the lock nor a launch was reached");
  }
});

test("the runner starts the app only through judgedPhase or runUnderLock with caffeinate, and signals no process itself", () => {
  const source = readFileSync(runner, "utf8");
  assert.doesNotMatch(source, /\bimport\s*\{[^}]*\bspawn\b[^}]*\}\s*from\s*"node:child_process"/, "no direct spawn");
  assert.doesNotMatch(source, /process\.kill\(|\.kill\("SIG/, "no signal sent by the runner itself");
  const starts = [...source.matchAll(/runUnderLock\(([^,]+),/g)].map((match) => match[1].trim());
  // The build's commands (`run` and runCargo) and the autosave journey's one aborted launch.
  assert.deepEqual(starts.sort(), ["CAFFEINATE", "command", "command"].sort());
  assert.equal((source.match(/judgedPhase\(\{/g) ?? []).length, 1, "one judged launch path");
  assert.match(source, /runUnderLock\(CAFFEINATE, \["-di", binary\], \{[^}]*abort: controller\.signal \}\)/);
});
