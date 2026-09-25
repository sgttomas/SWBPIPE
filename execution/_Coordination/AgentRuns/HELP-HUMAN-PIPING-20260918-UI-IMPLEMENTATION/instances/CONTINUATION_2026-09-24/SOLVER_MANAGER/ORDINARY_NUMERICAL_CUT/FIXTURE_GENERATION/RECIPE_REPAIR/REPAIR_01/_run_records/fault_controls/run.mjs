// Source-only preparation until the manager authorizes execution. Tests filesystem
// transactions with stub Cargo; it proves no producer or numerical behavior.
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync, mkdirSync, mkdtempSync, copyFileSync,
  readdirSync, existsSync, chmodSync, renameSync, symlinkSync, realpathSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const expectedRecipe = "4ebeca47a8040f8334ea38ea4c36bb5d50078b7ad2912a0981a9e86e3d10d399";
const sha = value => createHash("sha256").update(value).digest("hex");
const recipeBytes = readFileSync(path.join(here, "recipe.mjs"));
assert.equal(sha(recipeBytes), expectedRecipe, "must run exact reviewed recipe bytes");
if (process.argv.length > 3) throw new Error("usage: node run.mjs [output-directory]");
const output = path.resolve(process.argv[2] ?? path.join(here, "results"));
mkdirSync(output, { recursive: true });
// /tmp is a macOS symlink. Use the canonical parent deliberately.
const sandboxParent = realpathSync("/private/tmp");
const targets = ["invented_mechanics_result_precision_1_sparse.json", "invented_mechanics_result_precision_1_dense.json", "precision_fixture_generation.json"];
const labels = ["sparse", "dense", "record"];
const seed = name => readFileSync(path.join(here, "inputs", name));
const cases = [];
for (const scenario of ["install2", "install3", "install3-rollback1", "symlink-parent"]) {
  const container = mkdtempSync(path.join(sandboxParent, "precision-transaction-fault-"));
  const root = path.join(container, "project");
  const fixtureDir = path.join(root, "fixtures/product_preview");
  for (const directory of [fixtureDir, path.join(root, "tools/serialization"), path.join(root, "core/product_physics/src"), path.join(root, "core/product_physics/examples"), path.join(root, "schemas"), path.join(root, "bin"), path.join(root, "stub-payloads")]) mkdirSync(directory, { recursive: true });
  writeFileSync(path.join(root, "tools/serialization/generate_product_preview_mechanics.mjs"), recipeBytes);
  writeFileSync(path.join(root, "package.json"), '{"private":true,"description":"transaction-only stub project"}\n');
  writeFileSync(path.join(root, "core/product_physics/Cargo.toml"), '[package]\nname="transaction_only_local_stub"\nversion="0.0.0"\n');
  writeFileSync(path.join(root, "core/product_physics/Cargo.lock"), '# transaction-only stub; no dependencies compiled\n');
  writeFileSync(path.join(root, "core/product_physics/src/lib.rs"), '// Transaction stub, never compiled.\n');
  writeFileSync(path.join(root, "core/product_physics/examples/preview_result.rs"), '// Transaction stub, never compiled.\n');
  writeFileSync(path.join(fixtureDir, "invented_preview_model.json"), seed("model.json"));
  writeFileSync(path.join(fixtureDir, "invented_mechanics_result.json"), seed("legacy.preimage.json"));
  targets.forEach((name, index) => writeFileSync(path.join(fixtureDir, name), seed(`${labels[index]}.preimage.json`)));
  for (const mode of ["sparse", "dense"]) writeFileSync(path.join(root, "stub-payloads", `${mode}.stdout.json`), seed(`${mode}.stdout.json`));
  for (const name of ["cargo", "rustc"]) {
    copyFileSync(path.join(here, `${name}-stub.mjs`), path.join(root, "bin", name));
    chmodSync(path.join(root, "bin", name), 0o755);
  }
  let observedDirectory = fixtureDir;
  if (scenario === "symlink-parent") {
    observedDirectory = path.join(container, "outside-project-fixtures");
    renameSync(fixtureDir, observedDirectory);
    symlinkSync(observedDirectory, fixtureDir, "dir");
  }
  const before = Object.fromEntries([...targets, "invented_mechanics_result.json", "invented_preview_model.json"]
    .map(name => [name, sha(readFileSync(path.join(observedDirectory, name)))]));
  const env = { ...process.env, PATH: `${path.join(root, "bin")}${path.delimiter}${path.dirname(process.execPath)}${path.delimiter}${process.env.PATH ?? ""}`,
    FAULT_SANDBOX_ROOT: root, FAULT_SCENARIO: scenario };
  delete env.NODE_OPTIONS;
  const command = [process.execPath, "--require", path.join(here, "preload.cjs"), path.join(root, "tools/serialization/generate_product_preview_mechanics.mjs")];
  const result = spawnSync(command[0], command.slice(1), { cwd: root, env, maxBuffer: 16 * 1024 * 1024 });
  const record = { scenario, sandbox: container, command, status: result.status,
    recipe_sha256: sha(recipeBytes), before, mocked_boundary: "Cargo metadata/output and rustc version are stubs; no product/compiler invoked" };
  writeFileSync(path.join(output, `${scenario}.stdout.log`), result.stdout ?? Buffer.alloc(0));
  writeFileSync(path.join(output, `${scenario}.stderr.log`), result.stderr ?? Buffer.alloc(0));
  try {
    assert.ifError(result.error);
    assert.equal(result.status, 1, "injected/refused transaction must fail");
    const stderr = result.stderr.toString("utf8");
    const after = Object.fromEntries(Object.keys(before).map(name => [name, sha(readFileSync(path.join(observedDirectory, name)))]));
    record.after = after;
    const stages = readdirSync(observedDirectory).filter(name => name.startsWith(".precision-generation-"));
    const eventsFile = path.join(root, "fault-events.ndjson");
    const events = existsSync(eventsFile) ? readFileSync(eventsFile, "utf8").trim().split("\n").map(line => JSON.parse(line)) : [];
    record.events = events;
    record.recovery_directories = stages;
    assert.equal(after["invented_mechanics_result.json"], before["invented_mechanics_result.json"], "legacy never changes");
    assert.equal(after["invented_preview_model.json"], before["invented_preview_model.json"], "input never changes");
    if (scenario === "install2" || scenario === "install3") {
      assert.match(stderr, new RegExp(`INJECTED_INSTALL_${scenario.at(-1)}`));
      assert.deepEqual(after, before, "all three output preimages restored exactly");
      assert.equal(stages.length, 0, "complete rollback cleanup");
      assert.equal(events.filter(event => event.operation === "install").length, Number(scenario.at(-1)));
    } else if (scenario === "install3-rollback1") {
      assert.match(stderr, /INJECTED_INSTALL_3/);
      assert.match(stderr, /INJECTED_RESTORE_1/);
      assert.match(stderr, /rollback incomplete/);
      assert.equal(stages.length, 1, "durable recovery directory retained");
      const recovery = path.join(observedDirectory, stages[0]);
      assert.ok(stderr.includes(recovery), "recovery location disclosed");
      const recoveryMap = JSON.parse(readFileSync(path.join(recovery, "recovery.json"), "utf8"));
      assert.equal(recoveryMap.destinations.length, 3);
      assert.equal(sha(readFileSync(path.join(recovery, "1.old"))), before[targets[1]], "failed dense rollback keeps exact backup");
      assert.equal(sha(readFileSync(path.join(recovery, "2.old"))), before[targets[2]], "record preimage also remains recoverable");
      assert.equal(after[targets[0]], before[targets[0]], "other rollback still attempted and restored sparse");
      assert.equal(after[targets[2]], before[targets[2]], "record never replaced");
      assert.equal(after[targets[1]], sha(seed("dense.stdout.json")), "unrestored destination matches captured stub output");
      assert.equal(events.filter(event => event.operation === "restore").length, 2, "rollback continues after failure");
    } else {
      assert.match(stderr, /redirected project path/);
      assert.deepEqual(after, before, "linked outside inputs/outputs untouched");
      assert.equal(stages.length, 0, "refuses before outside staging");
      assert.equal(events.length, 0, "no rename attempted");
      assert.equal(existsSync(path.join(root, "stub-commands.ndjson")), false, "refuses before Cargo stub");
    }
    record.assertions = "passed";
  } catch (error) {
    record.assertions = "failed";
    record.error = error.stack ?? String(error);
    process.exitCode = 1;
  }
  // Retain disposable sandboxes and recovery bytes for independent inspection.
  writeFileSync(path.join(output, `${scenario}.json`), `${JSON.stringify(record, null, 2)}\n`);
  cases.push(record);
}
const preparationInputs = ["recipe.mjs", "preload.cjs", "cargo-stub.mjs", "rustc-stub.mjs", "run.mjs", ...readdirSync(path.join(here, "inputs")).map(name => `inputs/${name}`)];
writeFileSync(path.join(output, "SUMMARY.json"), `${JSON.stringify({ scope: "mocked transaction fault controls only", node: process.version,
  inputs: Object.fromEntries(preparationInputs.sort().map(name => [name, sha(readFileSync(path.join(here, name)))])),
  cases: cases.map(record => ({ scenario: record.scenario, assertions: record.assertions, sandbox: record.sandbox })) }, null, 2)}\n`);
