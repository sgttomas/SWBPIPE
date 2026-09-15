#!/usr/bin/env node
// V2 evidence-local preparation: seed native-normalized canonical JSON strings.
// V1 and its failed byte-level unchanged-save witness remain preserved.
import fs from "node:fs";
import path from "node:path";
import crypto from "node:crypto";
import { pathToFileURL } from "node:url";

const argv = process.argv.slice(2);
const arg = (name) => {
  const index = argv.indexOf(name);
  if (index < 0 || !argv[index + 1]) throw new Error(`MISSING:${name}`);
  return argv[index + 1];
};
const baselinePath = arg("--baseline-row");
const workingRoot = arg("--working-root");
const gluePath = arg("--wasm-glue");
const wasmPath = arg("--wasm");
const outputDir = arg("--output-dir");
const candidate = arg("--candidate");
if (candidate !== "8ad37207cf088025623aa1e777a97a6fcb802f48") throw new Error("CANDIDATE_BINDING_MISMATCH");

const sha = (value) => crypto.createHash("sha256").update(value).digest("hex");
const glue = await import(pathToFileURL(gluePath).href);
await glue.default({ module_or_path: new Uint8Array(fs.readFileSync(wasmPath)) });
const baseline = JSON.parse(fs.readFileSync(baselinePath, "utf8"));
fs.mkdirSync(outputDir, { recursive: true });

for (const profile of ["desktop", "python"]) {
  const fixturePath = path.join(workingRoot, "fixtures", "analysis_runs", "invented", `legacy_${profile}_v0_1.json`);
  const fixtureBytes = fs.readFileSync(fixturePath);
  const analysis = JSON.parse(fixtureBytes.toString("utf8"));
  const suffix = `legacy-${profile}`;
  const modelId = `model:${suffix}`;
  const runId = `run:${suffix}`;
  const resultId = `result:${suffix}`;
  const row = structuredClone(baseline);
  const model = JSON.parse(row.model_json);
  model.project.id = modelId;
  model.project.name = `Legacy ${profile} 0.1 compatibility witness`;
  const mechanics = {
    schema_version: "0.2.0",
    document_kind: "MechanicsResult",
    run_id: runId,
    model_ref: modelId,
    status: { mechanics: "MECHANICS_SOLVED", rule_check: "RULE_INPUTS_INCOMPLETE" },
    summary: {},
    results: [{ id: resultId, entity_ref: "node:legacy", kind: "displacement_magnitude", value: 1, unit: "mm", dimension: "length" }],
    diagnostics: [],
  };
  if (analysis.analysis_run.run_id !== runId) throw new Error(`FIXTURE_RUN_MISMATCH:${profile}`);
  if (analysis.analysis_run.model_state_ref.ref !== `state:${modelId}:preview`) throw new Error(`FIXTURE_MODEL_MISMATCH:${profile}`);
  row.project_id = modelId;
  row.project_name = model.project.name;
  row.model_json = glue.canonical_json_string(JSON.stringify(model));
  row.editor_intents_json = "[]";
  row.proposal_json = "null";
  row.selected_review_target_json = "null";
  row.mechanics_result_json = glue.canonical_json_string(JSON.stringify(mechanics));
  row.analysis_run_json = glue.canonical_json_string(JSON.stringify(analysis));
  row.model_migration_ledger_json = "[]";
  const modelDigest = glue.canonical_sha256_hex(row.model_json);
  const modelHash = { algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_scope: "model_payload", payload_ref: modelId, value: `sha256:${modelDigest}`, hash_status: "computed_local_preview" };
  row.model_hash_json = glue.canonical_json_string(JSON.stringify(modelHash));
  const envelopePayload = { model, editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: mechanics, analysis_run: analysis, model_hash: modelHash };
  const envelopeDigest = glue.canonical_sha256_hex(JSON.stringify(envelopePayload));
  row.project_envelope_hash_json = glue.canonical_json_string(JSON.stringify({ algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_scope: "project_envelope_payload", payload_excludes: "storage_summary_and_envelope_hash_carrier_fields", payload_ref: modelId, value: `sha256:${envelopeDigest}`, hash_status: "computed_local_preview" }));
  const output = {
    schema: "prepared-legacy-historical-row-v4-v2",
    candidate_commit: candidate,
    case_id: `legacy_${profile}_v2`,
    profile,
    baseline_project_id: baseline.project_id,
    row,
    expectations: {
      mechanics_result_json: row.mechanics_result_json,
      analysis_run_json: row.analysis_run_json,
      source_fixture: path.relative(workingRoot, fixturePath),
      source_fixture_sha256: sha(fixtureBytes),
      analysis_schema_version: analysis.schema_version,
      analysis_canonicalizations: [...analysis.analysis_run.hashes, ...analysis.analysis_run.result_refs.flatMap((item) => item.hash_refs ?? [])].map((item) => item.canonicalization),
      raw_payloads_must_survive_open_and_unchanged_save: true,
    },
    authority: { model_and_envelope_profile: "rfc8785_jcs", wasm_glue: path.relative(workingRoot, gluePath), wasm: path.relative(workingRoot, wasmPath), wasm_sha256: sha(fs.readFileSync(wasmPath)) },
  };
  fs.writeFileSync(path.join(outputDir, `legacy_${profile}_v2.json`), JSON.stringify(output, null, 2) + "\n");
}
console.log(JSON.stringify({ status: "PREPARED", profiles: ["desktop", "python"], candidate_commit: candidate }));
