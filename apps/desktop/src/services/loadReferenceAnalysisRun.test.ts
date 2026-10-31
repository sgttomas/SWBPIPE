/** The desktop AnalysisRun record of a load/reference-state result (T1 WP2), on the
 * current exact identities (B7b: the v3 twins of the retired load-reference-1 and
 * load-reference-source-1). For both, the desktop record mirrors Python
 * `build_analysis_run_v0_3` (DEL-14-02). Proof: the desktop record of each committed raw
 * equals the committed Python carrier by canonical hash after normalizing only the
 * determinism notes, in which the two records differ because their inputs differ
 * (`DESKTOP_NOTES`, `CARRIER_NOTES` below), and therefore the record self-hash; provenance
 * labels are compared, not normalized. The desktop validator reads Python's carrier.
 * - Joined, pressure-source-1 (fixtures/results/pressure_source_load_state_*.analysis_run.json):
 *   a source record carrying contract_evidence and source_block_recovery.
 * - Ordinary, pressure-2 (fixtures/results/pressure_load_state_*.analysis_run.json): a record
 *   carrying neither, its result refs binding pressure-2's current table. */
import { describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../types";
import { buildAnalysisRunV03, modelLoadBasisRefs, validateAnalysisRunV03, verifyAnalysisRunRecord } from "./analysisRunCompatibility";
import { canonicalSha256HexCheckedV1 } from "./hashService";
import { receivedFixture } from "../test-support/receivedText";

const root = resolve(__dirname, "../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
const LR = ["connected-sparse", "connected-dense", "pressure-sparse", "pressure-dense"];
const LRS = ["n05", "n06", "fields", "mixed", "eigen_motion"].flatMap(name => ["sparse", "dense"].map(mode => `${name}-${mode}`));
const MODE: Record<string, string> = { sparse: "sparse_interactive", dense: "dense_scrutiny" };
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
const FAMILY = {
  ordinary: { folder: "pressure/load_reference", carrier: "pressure_load_state" },
  joined: { folder: "pressure_source/load_reference_source", carrier: "pressure_source_load_state" },
} as const;

/** The one field in which the two records differ, and only because their inputs differ:
 * the desktop always binds an input-manifest payload (`model_state_ref_and_solver_settings_
 * unit_basis_bound_by_input_manifest`), while the committed Python carriers were built
 * without one (`input_manifest_payload_unavailable_…`); the record self-hash follows it.
 * Provenance labels are the same (B7b aligned the desktop's to Python's 0.3 labels). */
const DESKTOP_NOTES = ["created_at_unavailable", "model_state_ref_and_solver_settings_unit_basis_bound_by_input_manifest"];
const CARRIER_NOTES = ["created_at_unavailable", "input_manifest_payload_unavailable_solver_settings_unit_basis_not_independently_verified"];
function normalize(record: Json): Json {
  const copy = structuredClone(record);
  const run = copy.analysis_run;
  expect([DESKTOP_NOTES, CARRIER_NOTES]).toContainEqual(run.reproducibility.determinism_notes);
  delete run.reproducibility.determinism_notes;
  run.hashes = run.hashes.filter((h: Json) => h.payload_scope !== "analysis_run_record");
  return copy;
}

function inputs(kind: keyof typeof FAMILY, name: string) {
  const [base, mode] = [name.slice(0, name.lastIndexOf("-")), name.slice(name.lastIndexOf("-") + 1)];
  const { folder, carrier } = FAMILY[kind];
  const raw = receivedFixture<MechanicsResult>(`fixtures/product_preview/${folder}/${base}-${MODE[mode]}.raw.json`);
  const model = json(`fixtures/product_preview/${folder}/${base}.request.json`).model as PreviewModel;
  const python = json(`fixtures/results/${carrier}_${name.replace("-", "_")}.analysis_run.json`) as AnalysisRunEnvelope;
  const ref = python.analysis_run.reproducibility.input_manifest_refs[0];
  const manifest = { manifest_ref: ref, manifest_sha256: "1".repeat(64), manifest: { model_basis: { model_ref: raw.model_ref }, solver_basis: { solver_name: raw.producer!.component_name, solver_version: raw.producer!.component_version, solver_build_ref: "unavailable:solver-build-not-supplied" } } };
  return { raw, model, python, manifest };
}
async function mirror(name: string, kind: keyof typeof FAMILY = "joined") {
  const { raw, model, python, manifest } = inputs(kind, name);
  const desktop = await buildAnalysisRunV03(raw, manifest, undefined, modelLoadBasisRefs(model));
  return { raw, model, python, desktop };
}
const PRESSURE_1 = { id: "openpipestress.result_semantics/0.3.0/pressure-2", sha256: "5451fbf3e10e085c90b523baa7daf5fc723659e0d874d988d23807be7d2fefe6" };

describe("joined (pressure-source-1) AnalysisRun mirror", () => {
  it.each(LRS)("%s equals the Python carrier by canonical hash (route-independent fields normalized)", async name => {
    const { raw, model, python, desktop } = await mirror(name);
    expect(await canonicalSha256HexCheckedV1(normalize(desktop))).toBe(await canonicalSha256HexCheckedV1(normalize(python)));
    const run = desktop.analysis_run;
    expect(run.contract_evidence).toEqual(raw.contract_evidence);
    expect(run.source_block_recovery).toEqual(raw.source_block_recovery);
    expect(await verifyAnalysisRunRecord(desktop)).toBe("match");
    await expect(validateAnalysisRunV03(desktop, raw, modelLoadBasisRefs(model))).resolves.toBeUndefined();
    await expect(validateAnalysisRunV03(python, raw, modelLoadBasisRefs(model))).resolves.toBeUndefined();
  });
});

describe("ordinary (pressure-2) AnalysisRun mirror", () => {
  it.each(LR)("%s equals the Python carrier by canonical hash (route-independent fields normalized); each validator reads the other's record", async name => {
    const { raw, model, python, desktop } = await mirror(name, "ordinary");
    expect(python.analysis_run.run_id).toBe(raw.run_id);
    expect(await canonicalSha256HexCheckedV1(normalize(desktop))).toBe(await canonicalSha256HexCheckedV1(normalize(python)));
    const run = desktop.analysis_run;
    // Neither namespace is embedded: pressure-2 is not a source record (Python's SOURCE_RECORD_CONTRACT_IDS).
    expect(Object.hasOwn(run, "contract_evidence")).toBe(false);
    expect(Object.hasOwn(run, "source_block_recovery")).toBe(false);
    expect(Object.hasOwn(run, "retained_precision")).toBe(false);
    // The record and every result ref bind pressure-2's current table.
    expect(run.reproducibility.semantic_contract).toStrictEqual(PRESSURE_1);
    expect(run.result_refs).toHaveLength(raw.results.length);
    for (const ref of run.result_refs) expect({ id: ref.semantic_contract?.id, sha256: ref.semantic_contract?.sha256 }).toStrictEqual(PRESSURE_1);
    expect(await verifyAnalysisRunRecord(desktop)).toBe("match");
    await expect(validateAnalysisRunV03(desktop, raw, modelLoadBasisRefs(model))).resolves.toBeUndefined();
    await expect(validateAnalysisRunV03(python, raw, modelLoadBasisRefs(model))).resolves.toBeUndefined();
  });
});

/** Refusal codes in which the desktop validator and Python's `validate_analysis_run_v0_3`
 * (`core/analysis_runs/compatibility.py`) differ for the same forged record or source: each
 * row's desktop code is exercised here, and its Python code is one Python's validator raises
 * (each measured against Python's validator on this forgery). They predate B7b and hold for
 * every identity; they are declared here, not normalized away. The table is not exhaustive.
 * Measured differences it does not exercise: a record without `run_contract_status` (Python
 * names ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH; the desktop validator reads the member
 * unguarded and fails with a runtime error, which a reopen names
 * HISTORICAL_ANALYSIS_EVIDENCE_INVALID), and the order of the run and model-state checks
 * (the desktop checks the run_id first, Python the model state, so a record wrong in both
 * is ANALYSIS_SOURCE_RUN_MISMATCH on the desktop and ANALYSIS_SOURCE_MODEL_MISMATCH in Python). */
const DECLARED_CODE_DIFFERENCES: ReadonlyArray<{ forgery: string; desktop: string; python: string; edit: (record: Json, source: Json) => void }> = [
  { forgery: "a non-source record carrying a source receipt", desktop: "SOURCE_RECOVERY_METADATA_CONTRADICTION", python: "SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN",
    edit: (r: Json) => { r.analysis_run.source_block_recovery = {}; } },
  { forgery: "a record naming another model state", desktop: "ANALYSIS_SOURCE_MODEL_STATE_MISMATCH", python: "ANALYSIS_SOURCE_MODEL_MISMATCH",
    edit: (r: Json) => { r.analysis_run.model_state_ref = { object_type: "ModelState", ref: "state:invented-other-model:preview" }; } },
  { forgery: "a source whose rule_check is outside the vocabulary", desktop: "ANALYSIS_SOURCE_RULE_STATUS_INVALID", python: "ANALYSIS_RULE_STATUS_INVALID",
    edit: (_r: Json, source: Json) => { source.status.rule_check = "INVENTED_RULE_STATUS"; } },
];
/** The text of Python's `validate_analysis_run_v0_3`, which the declared Python codes are read against. */
function pythonValidatorText(): string {
  const text = readFileSync(resolve(root, "core/analysis_runs/compatibility.py"), "utf8");
  const start = text.indexOf("\ndef validate_analysis_run_v0_3(");
  expect(start).toBeGreaterThan(0);
  return text.slice(start, text.indexOf("\ndef ", start + 1));
}

describe("records bound to other evidence are refused", () => {
  it.each([
    ["ordinary carrying contract_evidence", (r: Json, raw: Json) => { r.analysis_run.contract_evidence = structuredClone(raw.contract_evidence); }, "ANALYSIS_PHYSICS_SOURCE_DOWNGRADE_FORBIDDEN"],
    ["ordinary bound to pressure-source-1's table", (r: Json) => { r.analysis_run.reproducibility.semantic_contract = { id: "openpipestress.result_semantics/0.3.0/pressure-source-1", sha256: "609a4e92ac19de0f5a1dd067dab64b4abeb7b3b70511e2095455f1d000d28e2c" }; }, "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH"],
    ["ordinary with another row binding", (r: Json) => { r.analysis_run.result_refs[0].semantic_contract.sha256 = "0".repeat(64); }, "ANALYSIS_ROW_SOURCE_BINDING_MISMATCH"],
    ["ordinary carrying a retained-precision receipt", (r: Json) => { r.analysis_run.retained_precision = { receipt_sha256: "0".repeat(64) }; }, "ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"],
  ])("%s (the same code as Python's validator)", async (_name, edit, code) => {
    const { raw, model, python } = await mirror("pressure-sparse", "ordinary");
    const forged = structuredClone(python); edit(forged, raw);
    await expect(validateAnalysisRunV03(forged, raw, modelLoadBasisRefs(model))).rejects.toThrow(code);
  });
  it.each(DECLARED_CODE_DIFFERENCES)("$forgery: the desktop names $desktop where Python names $python (declared difference)", async ({ desktop, python: pythonCode, edit }) => {
    const { raw, model, python } = await mirror("pressure-sparse", "ordinary");
    const forged = structuredClone(python), source = structuredClone(raw); edit(forged, source);
    const refusal = await validateAnalysisRunV03(forged, source, modelLoadBasisRefs(model)).then(() => null, (error: Error) => error.message);
    expect(refusal).toBe(desktop);
    // The declared Python code is one Python's validator raises; the desktop's is not one of its codes.
    const pythonValidator = pythonValidatorText();
    expect(pythonValidator).toContain(`raise ValueError("${pythonCode}")`);
    expect(pythonValidator).not.toContain(`"${desktop}"`);
  });
  it("a tampered raw ordinary result is refused by the reader before a record is built", async () => {
    const { raw, model, manifest } = inputs("ordinary", "connected-sparse");
    const tampered = structuredClone(raw) as Json; tampered.contract_evidence.load_reference_states[0].members[0].fit_strain = 0.5;
    await expect(buildAnalysisRunV03(tampered, manifest, undefined, modelLoadBasisRefs(model))).rejects.toThrow("SOURCE_LOAD_REFERENCE_STRETCH_BINDING");
  });
  it.each([
    ["joined without its retained evidence", (r: Json) => { delete r.analysis_run.contract_evidence; }, "ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH"],
    ["joined without load_reference_states", (r: Json) => { delete r.analysis_run.contract_evidence.load_reference_states; }, "ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH"],
    ["joined without its receipt", (r: Json) => { delete r.analysis_run.source_block_recovery; }, "ANALYSIS_SOURCE_BLOCK_RECEIPT_MISMATCH"],
    ["joined with another table hash", (r: Json) => { r.analysis_run.reproducibility.semantic_contract.sha256 = "0".repeat(64); }, "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH"],
  ])("%s", async (_name, edit, code) => {
    const { raw, model, desktop } = await mirror("mixed-sparse");
    const forged = structuredClone(desktop); edit(forged);
    await expect(validateAnalysisRunV03(forged, raw, modelLoadBasisRefs(model))).rejects.toThrow(code);
  });
  it("a tampered raw joined result is refused by the reader before a record is built", async () => {
    const { raw, model, manifest } = inputs("joined", "mixed-sparse");
    const tampered = structuredClone(raw) as Json; tampered.contract_evidence.load_reference_states[0].members[0].fit_strain = 0.5;
    await expect(buildAnalysisRunV03(tampered, manifest, undefined, modelLoadBasisRefs(model))).rejects.toThrow("SOURCE_LOAD_REFERENCE_STRETCH_BINDING");
  });
});
