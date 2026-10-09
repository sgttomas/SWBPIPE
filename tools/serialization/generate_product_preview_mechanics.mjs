// Generate example transport data from the actual product. No numerical outcome
// is upgraded, and no input model is ever a write destination.
// Default mode: the browser's bundled demo, the valid invented_demo_model's
// solved preview-physics-1 pair, plus its historical-format carrier (below), and
// their record. `--preview-physics-1`: the refused invented_preview_model's
// preview-physics-1 pair and its own record. Neither mode writes the other's files.
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
const previewContract = "openpipestress.result_semantics/0.3.0/preview-physics-1";
// Both models are compiled into the generator, so both are inputs of every run.
const models = {
  demo: "fixtures/product_preview/invented_demo_model.json",
  refused: "fixtures/product_preview/invented_preview_model.json",
};
const configs = {
  demo: { contract: previewContract, model: "invented_demo_model", input: models.demo, requireSolved: true,
    output: suffix => `fixtures/product_preview/invented_demo_result_preview_physics_1_${suffix}.json`,
    legacyCarrier: "fixtures/product_preview/invented_demo_result_legacy_0_1.json",
    record: "fixtures/product_preview/demo_fixture_generation.json", stagePrefix: ".demo-generation-", label: "Demo" },
  preview: { contract: previewContract, model: "invented_preview_model", input: models.refused, requireSolved: false,
    output: suffix => `fixtures/product_preview/invented_mechanics_result_preview_physics_1_${suffix}.json`,
    legacyCarrier: null,
    record: "fixtures/product_preview/preview_physics_fixture_generation.json", stagePrefix: ".preview-physics-generation-", label: "Preview-physics" },
};
const destinations = config => [config.record, ...modes.map(([, suffix]) => config.output(suffix)), ...(config.legacyCarrier ? [config.legacyCarrier] : [])];
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
function command(program, args) {
  validateRoot();
  const result = spawnSync(program, args, { cwd: root, maxBuffer: 128 * 1024 * 1024 });
  if (result.stderr?.length) process.stderr.write(result.stderr);
  if (result.error || result.status !== 0) fail(`${program} failed: ${result.error?.message ?? result.signal ?? result.status}`);
  return { stdout: result.stdout, stderr: result.stderr };
}
const decode = value => new TextDecoder("utf-8", { fatal: true }).decode(value);
const parse = value => JSON.parse(decode(value));
function metadata() {
  return parse(command("cargo", ["metadata", "--offline", "--locked", "--format-version", "1", "--manifest-path", manifest]).stdout);
}
function dependencyInventory(meta) {
  return meta.packages.map(pkg => ({ name: pkg.name, version: pkg.version,
    source: pkg.source ?? "project_local", ...(pkg.source === null ? { manifest: portable(pkg.manifest_path) } : {}) }))
    .sort((a, b) => JSON.stringify(a) < JSON.stringify(b) ? -1 : JSON.stringify(a) > JSON.stringify(b) ? 1 : 0);
}
function sourceInventory(meta) {
  const names = new Set([manifest, generator, models.demo, models.refused, recipe, legacySemantics, "package.json"]);
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
  // A fresh non-exact solve is preview-physics-1 (T0R); any other identity is refused.
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

// The historical-format carrier of the demo's sparse output, for consumers that
// read the legacy 0.1.0 result format. It is the producer's exact text with:
// - the four 0.2.0 header members (producer, numerical_quality,
//   formulation_basis, contract_evidence) removed;
// - schema_version set to 0.1.0;
// - every result row whose kind the historical 0.1.0 result semantics
//   (`legacySemantics`) do not define removed, since a 0.1.0 reader has no
//   meaning for it;
// - a summary headline whose result row was removed set to null (withheld), so
//   no reference dangles, and a nonzero summary count of a removed kind's rows
//   (`carrierCountedKinds`) set to null, so the summary describes only the rows
//   the carrier carries.
// Every remaining byte is the producer's; no kept row, value, other summary
// member or diagnostic is changed. It is a format carrier for readers' tests,
// not a producer output and not a legacy computation.
const carrierRemoved = ["producer", "numerical_quality", "formulation_basis", "contract_evidence"];
const legacySemantics = "fixtures/results/semantic_contract_v0_2.json";
// Summary counts and the row kind each counts (the producer sets
// component_stress_modifier_count to the number of intensified bending rows).
const carrierCountedKinds = { component_stress_modifier_count: "component_equal_factor_intensified_bending_stress_v1" };
function legacyKinds() {
  const kinds = new Set((parse(bytes(legacySemantics)).rows ?? []).map(row => row.kind));
  if (!kinds.size || [...kinds].some(kind => typeof kind !== "string" || !kind)) fail("legacy carrier: no historical result kinds");
  return kinds;
}
function legacyCarrier(raw) {
  const kinds = legacyKinds();
  const lines = decode(raw).split("\n");
  const kept = [];
  let skipping = false;
  for (const line of lines) {
    // A pretty-printed top-level member starts with exactly two spaces and a quote.
    const member = /^  "([^"]+)": /.exec(line);
    if (member) skipping = carrierRemoved.includes(member[1]);
    else if (line === "}") skipping = false;
    if (!skipping) kept.push(line);
  }
  const close = kept.lastIndexOf("}");
  if (close < 1) fail("legacy carrier: no closing brace");
  kept[close - 1] = kept[close - 1].replace(/,$/, "");
  const schema = kept.findIndex(line => line === '  "schema_version": "0.2.0",');
  if (schema < 0 || kept.filter(line => line.startsWith('  "schema_version": ')).length !== 1) fail("legacy carrier: schema_version line");
  kept[schema] = '  "schema_version": "0.1.0",';
  // Result rows are the pretty-printed elements of "results" at four spaces.
  const open = kept.indexOf('  "results": [');
  const end = kept.findIndex((line, index) => index > open && /^  \],?$/.test(line));
  if (open < 0 || end < 0 || kept.filter(line => line === '  "results": [').length !== 1) fail("legacy carrier: results array");
  const elements = [];
  for (let index = open + 1; index < end;) {
    if (kept[index] !== "    {") fail("legacy carrier: result row start");
    const stop = kept.findIndex((line, at) => at > index && /^    \},?$/.test(line));
    if (stop < 0) fail("legacy carrier: result row end");
    const block = kept.slice(index, stop + 1);
    block[block.length - 1] = "    }";
    elements.push(block);
    index = stop + 1;
  }
  const rows = elements.filter(block => kinds.has(JSON.parse(block.join("\n")).kind));
  const keptIds = new Set(rows.map(block => JSON.parse(block.join("\n")).id));
  rows.forEach((block, index) => { block[block.length - 1] = index === rows.length - 1 ? "    }" : "    },"; });
  let carried = [...kept.slice(0, open + 1), ...rows.flat(), ...kept.slice(end)];
  // Summary headlines are pretty-printed members of "summary" at four spaces.
  const summaryOpen = carried.indexOf('  "summary": {');
  const summaryEnd = carried.findIndex((line, index) => index > summaryOpen && /^  \},?$/.test(line));
  if (summaryOpen < 0 || summaryEnd < 0) fail("legacy carrier: summary object");
  const summary = [];
  const nulled = [];
  for (let index = summaryOpen + 1; index < summaryEnd;) {
    const member = /^    "([^"]+)": (.*)$/.exec(carried[index]);
    if (!member) fail("legacy carrier: summary member");
    if (member[2] !== "{") {
      const counted = carrierCountedKinds[member[1]];
      if (counted && !kinds.has(counted) && JSON.parse(member[2].replace(/,$/, "")) !== 0) {
        nulled.push(member[1]);
        summary.push(`    "${member[1]}": null${member[2].endsWith(",") ? "," : ""}`);
      } else summary.push(carried[index]);
      index += 1;
      continue;
    }
    const stop = carried.findIndex((line, at) => at > index && /^    \},?$/.test(line));
    if (stop < 0) fail("legacy carrier: summary member end");
    const block = carried.slice(index, stop + 1);
    const value = JSON.parse(`{${block.join("\n").replace(/,$/, "")}}`)[member[1]];
    if (typeof value?.result_ref === "string" && !keptIds.has(value.result_ref)) {
      nulled.push(member[1]);
      summary.push(`    "${member[1]}": null${block[block.length - 1].endsWith(",") ? "," : ""}`);
    } else summary.push(...block);
    index = stop + 1;
  }
  carried = [...carried.slice(0, summaryOpen + 1), ...summary, ...carried.slice(summaryEnd)];
  const payload = Buffer.from(carried.join("\n"));
  // Structural check: exactly the source minus the four members and the
  // undefined kinds' rows, schema 0.1.0, key order kept.
  const expected = parse(raw);
  for (const key of carrierRemoved) {
    if (!Object.hasOwn(expected, key)) fail(`legacy carrier: source lacks ${key}`);
    delete expected[key];
  }
  expected.schema_version = "0.1.0";
  expected.results = expected.results.filter(row => kinds.has(row.kind));
  for (const key of nulled) expected.summary[key] = null;
  exact(JSON.stringify(parse(payload)), JSON.stringify(expected), "legacy carrier differs from its source");
  const ids = new Set(expected.results.map(row => row.id));
  if (Object.values(expected.summary).some(value => typeof value?.result_ref === "string" && !ids.has(value.result_ref))) fail("legacy carrier: dangling summary reference");
  return { payload, removedKinds: [...new Set(parse(raw).results.map(row => row.kind).filter(kind => !kinds.has(kind)))].sort(),
    removedRows: parse(raw).results.length - expected.results.length, nulledSummary: nulled };
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
    const captured = command("cargo", args);
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
  const derivedOutputs = [];
  if (config.legacyCarrier) {
    const [source, raw] = fixtureWrites.find(([name]) => name === config.output("sparse"));
    const { payload, removedKinds, removedRows, nulledSummary } = legacyCarrier(raw);
    fixtureWrites.push([config.legacyCarrier, payload]);
    derivedOutputs.push({ path: config.legacyCarrier, sha256: sha(payload), derived_from: { path: source, sha256: sha(raw) },
      schema_version: "0.1.0", removed_members: carrierRemoved,
      historical_kinds: { path: legacySemantics, sha256: before[legacySemantics] }, removed_row_kinds: removedKinds, removed_row_count: removedRows,
      row_count: parse(payload).results.length, nulled_summary_members: nulledSummary,
      derivation: "The derived_from file's exact bytes with the removed top-level members deleted, schema_version set to 0.1.0, the rows of the removed_row_kinds (kinds the historical_kinds file does not define) deleted, and each nulled_summary_members member set to null: a headline whose row was deleted, or a nonzero count of a removed kind's rows. No kept row, value, other summary member or diagnostic changed. A historical-format carrier for legacy-format readers' tests, not producer output." });
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
    tools: { node: process.version, cargo: decode(command("cargo", ["--version"]).stdout).trim(), rustc: decode(command("rustc", ["--version"]).stdout).trim() },
    outputs, ...(derivedOutputs.length ? { derived_outputs: derivedOutputs } : {}),
    provenance: "Actual unchanged-input Rust executions; raw stdout retained as fixture bytes. No headers, quality or numeric values synthesized. Example transport data, not independent physics or source/build authentication.",
  };
  exact(JSON.stringify(sourceInventory(meta)), beforeText, "source/input changed before commit");
  replaceSet([...fixtureWrites, [config.record, Buffer.from(`${JSON.stringify(record, null, 2)}\n`)]], config);
  process.stdout.write(`${JSON.stringify({ generated: [...outputs, ...derivedOutputs].map(output => output.path), record: config.record })}\n`);
} catch (error) {
  process.stderr.write(`Product preview fixture generation failed: ${error.message}\n`);
  process.exitCode = 1;
}
