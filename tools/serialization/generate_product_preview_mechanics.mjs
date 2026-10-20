// Generate example transport data from the actual product. No numerical outcome
// is upgraded, and no input model is ever a write destination.
// Default mode: the browser's bundled demo, the valid invented_demo_model's
// solved preview-physics-2 pair, and their record. `--preview-physics-1`: the refused invented_preview_model's
// preview-physics-2 pair and its own record. Neither mode writes the other's files.
// Cargo runs through `$CARGO` when set (a program taking cargo's own arguments); records still name the logical `cargo`.
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFileSync, writeFileSync, readdirSync, lstatSync, realpathSync,
  existsSync, mkdtempSync, renameSync, unlinkSync, rmSync, openSync, fsyncSync, closeSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const manifest = "core/product_physics/Cargo.toml";
const generator = "core/product_physics/examples/preview_result.rs";
const recipe = "tools/serialization/generate_product_preview_mechanics.mjs";
const previewContract = "openpipestress.result_semantics/0.3.0/preview-physics-2";
// Both models are compiled into the generator, so both are inputs of every run.
const models = {
  demo: "fixtures/product_preview/invented_demo_model.json",
  refused: "fixtures/product_preview/invented_preview_model.json",
};
const configs = {
  demo: { contract: previewContract, model: "invented_demo_model", input: models.demo, requireSolved: true,
    output: suffix => `fixtures/product_preview/invented_demo_result_preview_physics_1_${suffix}.json`,
    record: "fixtures/product_preview/demo_fixture_generation.json", stagePrefix: ".demo-generation-", label: "Demo" },
  preview: { contract: previewContract, model: "invented_preview_model", input: models.refused, requireSolved: false,
    output: suffix => `fixtures/product_preview/invented_mechanics_result_preview_physics_1_${suffix}.json`,
    record: "fixtures/product_preview/preview_physics_fixture_generation.json", stagePrefix: ".preview-physics-generation-", label: "Preview-physics" },
};
const destinations = config => [config.record, ...modes.map(([, suffix]) => config.output(suffix))];
const modes = [
  ["sparse_interactive", "sparse", 1],
  ["dense_scrutiny", "dense", 2],
];
const sha = bytes => createHash("sha256").update(bytes).digest("hex");
const bytes = relative => readFileSync(checkedPath(relative, "file"));
const fail = message => { throw new Error(message); };
const exact = (actual, expected, message) => { if (actual !== expected) fail(message); };
const portable = absolute => {
  const relative = path.relative(root, absolute);
  if (relative.startsWith(`..${path.sep}`) || relative === ".." || path.isAbsolute(relative)) fail("source outside project root");
  return relative.split(path.sep).join("/");
};
// Reject redirects in every root/relative component before reads or writes.
// Rechecked around replacement; this is not claimed to defeat hostile concurrent
// directory replacement (generation still requires the ordinary single-writer lease).
function validateRoot() {
  let cursor = path.parse(root).root;
  for (const part of root.slice(cursor.length).split(path.sep).filter(Boolean)) {
    cursor = path.join(cursor, part);
    const stat = lstatSync(cursor);
    if (stat.isSymbolicLink() || !stat.isDirectory() || realpathSync(cursor) !== cursor) fail(`redirected project root: ${cursor}`);
  }
}
function checkedPath(relative, kind, allowMissingLeaf = false) {
  validateRoot();
  if (path.isAbsolute(relative) || relative.split(/[\\/]/).some(part => part === ".." || part === "." || !part)) fail(`invalid project path: ${relative}`);
  const parts = relative.split("/");
  let cursor = root;
  for (let index = 0; index < parts.length; index++) {
    cursor = path.join(cursor, parts[index]);
    let stat;
    try { stat = lstatSync(cursor); }
    catch (error) {
      if (allowMissingLeaf && index === parts.length - 1 && error.code === "ENOENT") return cursor;
      throw error;
    }
    if (stat.isSymbolicLink() || realpathSync(cursor) !== cursor) fail(`redirected project path: ${relative}`);
    if (index < parts.length - 1 || kind === "directory") {
      if (!stat.isDirectory()) fail(`not a project directory: ${relative}`);
    } else if (!stat.isFile()) fail(`not a regular project file: ${relative}`);
  }
  return cursor;
}
function command(program, args, name = program) {
  validateRoot();
  const result = spawnSync(program, args, { cwd: root, maxBuffer: 128 * 1024 * 1024 });
  if (result.stderr?.length) process.stderr.write(result.stderr);
  if (result.error || result.status !== 0) fail(`${name} failed: ${result.error?.message ?? result.signal ?? result.status}`);
  return { stdout: result.stdout, stderr: result.stderr };
}
// The executing program is never recorded: commands and tools name the logical `cargo`.
const cargo = args => command(process.env.CARGO || "cargo", args, "cargo");
const decode = value => new TextDecoder("utf-8", { fatal: true }).decode(value);
const parse = value => JSON.parse(decode(value));
// Asked of the same program that ran the generation, so it names the toolchain that
// produced the outputs; anything but a cargo version line is refused, not recorded.
function cargoVersion() {
  const version = decode(cargo(["--version"]).stdout).trim();
  if (!/^cargo \d+\.\d+\.\d+[^\s]*( \([^\n]*\))?$/.test(version)) fail(`unexpected cargo --version report: ${JSON.stringify(version)}`);
  return version;
}
function metadata() {
  return parse(cargo(["metadata", "--offline", "--locked", "--format-version", "1", "--manifest-path", manifest]).stdout);
}
function dependencyInventory(meta) {
  return meta.packages.map(pkg => ({ name: pkg.name, version: pkg.version,
    source: pkg.source ?? "project_local", ...(pkg.source === null ? { manifest: portable(pkg.manifest_path) } : {}) }))
    .sort((a, b) => JSON.stringify(a) < JSON.stringify(b) ? -1 : JSON.stringify(a) > JSON.stringify(b) ? 1 : 0);
}
function sourceInventory(meta) {
  const names = new Set([manifest, generator, models.demo, models.refused, recipe, "package.json"]);
  function walk(relative) {
    for (const entry of readdirSync(checkedPath(relative, "directory"), { withFileTypes: true })) {
      if (["target", ".git", "node_modules", "__pycache__"].includes(entry.name)) continue;
      const name = `${relative}/${entry.name}`;
      if (entry.isSymbolicLink()) fail(`source inventory refuses symlink: ${name}`);
      if (entry.isDirectory()) walk(name);
      else if (entry.isFile()) names.add(name);
    }
  }
  // Cargo identifies the actual local dependency closure; hash all its package
  // files (excluding build/cache directories), not an assumed list of crate names.
  for (const pkg of meta.packages.filter(pkg => pkg.source === null)) {
    const directory = portable(path.dirname(pkg.manifest_path));
    checkedPath(directory, "directory");
    if (!directory.startsWith("core/")) fail(`local dependency outside bounded core inventory: ${directory}`);
    walk(directory);
  }
  // The current product/unit include_str! inputs outside package directories.
  walk("schemas");
  for (const config of [".cargo/config", ".cargo/config.toml", "rust-toolchain", "rust-toolchain.toml"]) {
    if (existsSync(path.join(root, config))) { checkedPath(config, "file"); names.add(config); }
  }
  return Object.fromEntries([...names].sort().map(name => {
    checkedPath(name, "file");
    return [name, sha(bytes(name))];
  }));
}
function validate(result, mode, value, model, config) {
  exact(result.schema_version, "0.2.0", "unexpected raw schema");
  exact(result.document_kind, "openpipestress.product_preview.mechanics_result", "unexpected document kind");
  exact(result.producer?.component_name, "open_pipe_stress_product_physics", "unexpected producer");
  exact(result.producer?.component_version, "0.2.0", "unexpected producer version");
  // A fresh non-exact solve is preview-physics-2 (T0R); any other identity is refused.
  exact(result.producer?.semantic_contract_id, config.contract, `unexpected semantics for ${config.label} mode`);
  exact(result.model_ref, model.project.id, "output/input model mismatch");
  if (!["MECHANICS_SOLVED", "MODEL_INCOMPLETE"].includes(result.status?.mechanics)) fail("unknown mechanics status");
  // The bundled demo must be a solved result; a refusal is never installed as the demo.
  if (config.requireSolved && result.status.mechanics !== "MECHANICS_SOLVED") fail(`${config.label} model did not solve: ${result.status.mechanics}`);
  const quality = result.numerical_quality;
  const statuses = ["checks_passed", "sensitive", "not_assessed", "unresolved", "failed"];
  if (!quality || !statuses.includes(quality.status) || !Array.isArray(quality.cases)) fail("missing numerical evidence");
  exact(quality.value_representation, "finite_binary64", "unexpected numeric representation");
  exact(quality.publication_quantization, "none", "unexpected publication quantization");
  exact(quality.integrity_policy, "M03-INTEGRITY-v1", "unexpected integrity policy");
  if (!Array.isArray(result.results) || !Array.isArray(result.diagnostics)) fail("invalid result collections");
  const allIds = [...result.results, ...result.diagnostics].map(row => row.id);
  if (allIds.some(id => typeof id !== "string" || !id) || new Set(allIds).size !== allIds.length) fail("ambiguous output evidence IDs");
  if (result.results.some(row => typeof row.value !== "number" || !Number.isFinite(row.value))) fail("nonfinite/missing result scalar");
  const expected = model.load_cases.map(item => item.id);
  if (!expected.length || new Set(expected).size !== expected.length) fail("ambiguous requested cases");
  if (quality.cases.some(c => c.basis_ref?.ref_type !== "load_case" || !expected.includes(c.basis_ref.ref_id) || !statuses.includes(c.solve_quality))) fail("unexpected numerical case");
  // A genuine early blocked response may have no execution/quality-case rows.
  // Preserve it as nonpassing; never manufacture mode or completed coverage.
  if (result.status.mechanics === "MODEL_INCOMPLETE" && result.results.length === 0) {
    if (["checks_passed", "sensitive"].includes(quality.status)) fail("empty blocked output claims passing quality");
    return;
  }
  if (quality.cases.length !== expected.length || expected.some(id => quality.cases.filter(c => c.basis_ref.ref_id === id).length !== 1)) fail("numerical case coverage mismatch");
  const aggregate = statuses[Math.max(...quality.cases.map(c => statuses.indexOf(c.solve_quality)))];
  exact(quality.status, aggregate, "numerical aggregate contradiction");
  const modeRows = result.results.filter(row => row.kind === "linear_solver_mode_basis");
  if (modeRows.length !== expected.length) fail("mode evidence coverage mismatch");
  for (const id of expected) {
    const rows = modeRows.filter(row => row.basis_ref?.ref_type === "load_case" && row.basis_ref.ref_id === id);
    if (rows.length !== 1) fail(`missing/duplicate mode evidence for ${id}`);
    const row = rows[0], meta = row.metadata;
    exact(row.value, value, "wrong mode value");
    exact(row.unit, "mode_code", "wrong mode unit");
    exact(meta?.component, "linear_solver_mode", "wrong mode component");
    exact(meta?.coordinate_system, "reduced_system", "wrong mode frame");
    exact(meta?.location, id, "wrong mode case location");
    if (typeof meta.basis !== "string") fail("missing mode basis");
    const fields = meta.basis.split(";").map(field => field.trim());
    exact(fields.filter(field => field.startsWith("solver_mode=")).join(), `solver_mode=${mode}`, "crossed mode evidence");
    exact(fields.filter(field => field.startsWith("solution_basis=")).join(), `solution_basis=${value === 1 ? "sparse" : "dense"}_structural_integrity_primary`, "wrong primary mode basis");
  }
}

// Capture/validation finish before staging. Durable rollback preimages are
// written and synced before any replacement; the hash record commits last.
// Process/power-loss multi-file atomicity is not claimed.
function replaceSet(items, config) {
  const directory = checkedPath("fixtures/product_preview", "directory");
  const stage = mkdtempSync(path.join(directory, config.stagePrefix));
  const stageRelative = portable(stage);
  checkedPath(stageRelative, "directory");
  const allowed = new Set(destinations(config));
  // No mode writes an input model or another mode's files.
  const forbidden = new Set([models.demo, models.refused,
    ...Object.values(configs).filter(other => other !== config).flatMap(destinations)]);
  const staged = [];
  const installed = [];
  let retainRecovery = false;
  function stageFile(name, existing = false) {
    return checkedPath(`${stageRelative}/${name}`, "file", !existing);
  }
  function stageWrite(name, payload) {
    const descriptor = openSync(stageFile(name), "wx");
    try { writeFileSync(descriptor, payload); fsyncSync(descriptor); }
    finally { closeSync(descriptor); }
  }
  try {
    if (items.length !== allowed.size || new Set(items.map(([name]) => name)).size !== allowed.size) fail(`generation requires exactly ${allowed.size} distinct destinations`);
    items.forEach(([name, payload], index) => {
      if (!allowed.has(name) || forbidden.has(name)) fail("forbidden generation destination");
      const target = checkedPath(name, "file", true);
      const existed = existsSync(target);
      const previous = existed ? bytes(name) : null;
      if (previous !== null) stageWrite(`${index}.old`, previous);
      stageWrite(`${index}.new`, payload);
      staged.push({ path: name, index, existed, previous_sha256: previous === null ? null : sha(previous), next_sha256: sha(payload) });
    });
    stageWrite("recovery.json", Buffer.from(`${JSON.stringify({ destinations: staged, note: "Restore each existing destination from its index.old backup; absent preimages require removing the new destination. Inspect actual files/hashes after interruption." }, null, 2)}\n`));
    // Every backup and new file now exists before the first destination changes.
    for (const entry of staged) {
      checkedPath("fixtures/product_preview", "directory");
      const target = checkedPath(entry.path, "file", true);
      // Do not overwrite a destination changed since preimage capture.
      exact(existsSync(target) ? sha(bytes(entry.path)) : null, entry.previous_sha256, "destination changed during staging");
      renameSync(stageFile(`${entry.index}.new`, true), target);
      installed.push(entry);
    }
  } catch (error) {
    const failures = [];
    for (const entry of installed.reverse()) {
      try {
        checkedPath("fixtures/product_preview", "directory");
        const target = checkedPath(entry.path, "file", true);
        // Refuse to destroy a subsequent writer's contents during recovery.
        exact(existsSync(target) ? sha(bytes(entry.path)) : null, entry.next_sha256, "destination changed before rollback");
        if (entry.existed) renameSync(stageFile(`${entry.index}.old`, true), target);
        else unlinkSync(target);
      } catch (rollbackError) {
        failures.push(`${entry.path}: ${rollbackError.message}`);
      }
    }
    if (failures.length) {
      retainRecovery = true;
      throw new Error(`Installation failed: ${error.message}; rollback incomplete: ${failures.join("; ")}; recovery material retained at ${stage}`);
    }
    throw error;
  } finally {
    if (!retainRecovery) {
      try {
        checkedPath(stageRelative, "directory");
        rmSync(stage, { recursive: true, force: true });
      } catch (cleanupError) {
        process.stderr.write(`Staging cleanup incomplete; inspect retained material at ${stage}: ${cleanupError.message}\n`);
      }
    }
  }
}

try {
  const usage = "usage: node tools/serialization/generate_product_preview_mechanics.mjs [--preview-physics-1]";
  if (process.argv.length > 3 || (process.argv.length === 3 && process.argv[2] !== "--preview-physics-1")) fail(usage);
  const config = process.argv.length === 3 ? configs.preview : configs.demo;
  validateRoot();
  checkedPath("fixtures/product_preview", "directory");
  for (const fixed of [manifest, generator, models.demo, models.refused, recipe, "package.json"]) checkedPath(fixed, "file");
  const meta = metadata();
  const before = sourceInventory(meta);
  const beforeText = JSON.stringify(before);
  const model = parse(bytes(config.input));
  const outputs = [];
  const fixtureWrites = [];
  for (const [mode, suffix, value] of modes) {
    const args = ["run", "--offline", "--locked", "--quiet", "--manifest-path", manifest, "--example", "preview_result", "--", mode, config.model];
    const captured = cargo(args);
    const result = parse(captured.stdout);
    validate(result, mode, value, model, config);
    const outputPath = config.output(suffix);
    fixtureWrites.push([outputPath, captured.stdout]); // exact raw stdout, no reserialization
    // stderr is passed through, not recorded: cargo's warnings carry the checkout's
    // absolute paths, so a hash of it would differ in every checkout.
    outputs.push({ mode, path: outputPath, sha256: sha(captured.stdout), command: ["cargo", ...args],
      exit_code: 0, model_ref: result.model_ref,
      mechanics_status: result.status.mechanics, numerical_status: result.numerical_quality.status,
      row_count: result.results.length, load_case_ids: result.numerical_quality.cases.map(c => c.basis_ref.ref_id) });
  }
  const afterMeta = metadata();
  exact(JSON.stringify(dependencyInventory(afterMeta)), JSON.stringify(dependencyInventory(meta)), "dependency resolution changed during generation");
  const after = sourceInventory(afterMeta);
  exact(JSON.stringify(after), beforeText, "source/input/lock changed during generation; no fixture replacement");
  const record = {
    record_kind: "actual_product_generated_example_fixture_basis", path_base: "projects/chirality-piping",
    recipe: { path: recipe, sha256: before[recipe] }, generator: { path: generator, sha256: before[generator] },
    input_model: { path: config.input, sha256: before[config.input], generator_argument: config.model },
    semantic_contract_id: config.contract,
    source_input_files: before, source_input_files_after: after,
    inventory_json_sha256: sha(Buffer.from(beforeText)),
    dependencies: dependencyInventory(meta),
    tools: { node: process.version, cargo: cargoVersion(), rustc: decode(command("rustc", ["--version"]).stdout).trim() },
    outputs,
    provenance: "Actual unchanged-input Rust executions; raw stdout retained as fixture bytes. No headers, quality or numeric values synthesized. Example transport data, not independent physics or source/build authentication.",
  };
  exact(JSON.stringify(sourceInventory(meta)), beforeText, "source/input changed before commit");
  replaceSet([...fixtureWrites, [config.record, Buffer.from(`${JSON.stringify(record, null, 2)}\n`)]], config);
  process.stdout.write(`${JSON.stringify({ generated: outputs.map(output => output.path), record: config.record })}\n`);
} catch (error) {
  process.stderr.write(`Product preview fixture generation failed: ${error.message}\n`);
  process.exitCode = 1;
}
