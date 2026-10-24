// W-12 S2 (DEL-14-01 Design `desktop-states-runs-comparison.md` §3, §6): the
// recorded revision of a transient analysis_run.v0.3 record. The native store
// computes it (`apps/desktop/src-tauri/src/analysis_runs.rs`); this test checks
// the TypeScript builder against the store's committed witness and checks that
// the record validator accepts both the transient and the recorded form.
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { nativeMechanicsReplayPair } from "../test/nativeMechanicsReplay";
import {
  CREATED_AT_UNAVAILABLE,
  modelLoadBasisRefs,
  recordedAnalysisRunRevision,
  recordedLibraryRefs,
  recordedRulePackRefs,
  validateAnalysisRunV03,
  verifyAnalysisRunRecord
} from "./analysisRunCompatibility";
import { canonicalJsonString, canonicalSha256Hex, canonicalSha256HexCheckedV1 } from "./hashService";
import { buildCurrentSessionInputManifest, type CurrentSessionInputManifest } from "./inputManifestService";
import { buildAnalysisRunPreview } from "./previewService";
import type { AnalysisRunEnvelope } from "../types";

const PERSISTENCE_FIXTURES = resolve(__dirname, "../../../../fixtures/persistence");
const INPUT_PATH = resolve(PERSISTENCE_FIXTURES, "transient_run_input.json");
const WITNESS_PATH = resolve(PERSISTENCE_FIXTURES, "recorded_run_witness.json");
const WRITE_INPUT_ENV = "OPENPIPESTRESS_WRITE_TRANSIENT_RUN_INPUT";

type TransientInput = { description: string; source_result_ref: string; run_record: AnalysisRunEnvelope; input_manifest_canonical: string };
type Witness = { state_id: string; state_model_hash: string; created_at: string; recording_id: string; run_record_hash: string; recorded_record: AnalysisRunEnvelope };

/** The transient record a sparse solve of the preview pair gets in the session (`workspaceSession.ts`). */
async function sessionTransient() {
  const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const manifest = await buildCurrentSessionInputManifest({
    model: pair.model,
    solver: {
      solver_name: pair.source.producer!.component_name,
      solver_version: pair.source.producer!.component_version,
      solver_build_ref: `${pair.source.producer!.component_name}@${pair.source.producer!.component_version}`,
      solver_mode: pair.mode,
      settings: { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: true }
    },
    active_rule_packs: [],
    external_assets: []
  });
  const record = await buildAnalysisRunPreview(pair.source, { inputManifest: manifest });
  return { pair, manifest, record };
}

describe("the recorded revision of a transient record", () => {
  it("the committed transient input is the session's record of the preview pair (the writer)", async () => {
    const { manifest, record } = await sessionTransient();
    const input: TransientInput = {
      description: "A transient analysis_run.v0.3 record built by the desktop session's builder (buildAnalysisRunPreview) for fixtures/results/preview_physics_connected_sparse.json and fixtures/model_operations/precision_connected_ui_model.json, with its input manifest's canonical text. Written by apps/desktop/src/services/recordedRunParity.test.ts with OPENPIPESTRESS_WRITE_TRANSIENT_RUN_INPUT=<path>; apps/desktop/src-tauri/src/analysis_runs_tests.rs records it to fixtures/persistence/recorded_run_witness.json.",
      source_result_ref: "fixtures/results/preview_physics_connected_sparse.json",
      run_record: record,
      input_manifest_canonical: manifest.canonical_bytes
    };
    const text = `${JSON.stringify(input, null, 2)}\n`;
    const target = process.env[WRITE_INPUT_ENV];
    if (target) { writeFileSync(target, text); return; }
    expect(readFileSync(INPUT_PATH, "utf8"), `the committed input is stale; regenerate it with ${WRITE_INPUT_ENV}`).toBe(text);
  });

  it("the TypeScript builder gives the native store's witness revision and hash", async () => {
    if (!existsSync(WITNESS_PATH)) throw new Error("recorded_run_witness.json is missing");
    const input = JSON.parse(readFileSync(INPUT_PATH, "utf8")) as TransientInput;
    const witness = JSON.parse(readFileSync(WITNESS_PATH, "utf8")) as Witness;
    const manifest = JSON.parse(input.input_manifest_canonical) as CurrentSessionInputManifest;
    const rebuilt = await recordedAnalysisRunRevision(input.run_record, { stateId: witness.state_id, createdAt: witness.created_at, inputManifest: manifest });
    expect(await canonicalJsonString(rebuilt)).toBe(await canonicalJsonString(witness.recorded_record));
    const claim = rebuilt.analysis_run.hashes.find((hash) => hash.payload_scope === "analysis_run_record")!;
    expect(claim.value).toBe(witness.run_record_hash);
    expect(witness.recording_id).toBe(`runrec:${witness.run_record_hash.slice(0, 32)}`);
    expect(await verifyAnalysisRunRecord(rebuilt)).toBe("match");
    const run = witness.recorded_record.analysis_run;
    expect(run.created_at).toBe(witness.created_at);
    expect(run.model_state_ref).toEqual({ object_type: "ModelState", ref: witness.state_id });
    expect(run.reproducibility.determinism_notes).not.toContain(CREATED_AT_UNAVAILABLE);
    expect(input.run_record.analysis_run.reproducibility.determinism_notes).toContain(CREATED_AT_UNAVAILABLE);
    // The received-result checksum is the transient record's, unchanged.
    expect(run.hashes.filter((hash) => hash.payload_scope === "received_result")).toEqual(input.run_record.analysis_run.hashes.filter((hash) => hash.payload_scope === "received_result"));
    expect(witness.state_model_hash).toBe(`sha256:${await canonicalSha256Hex(manifest.model_basis.model_payload)}`);
  });

  it("the validator accepts the transient form and the recorded form bound to its stored state", async () => {
    const { pair, manifest, record } = await sessionTransient();
    const basis = modelLoadBasisRefs(pair.model);
    await validateAnalysisRunV03(record, pair.source, basis);
    const stateId = "state:0123456789abcdef0123456789abcdef";
    const modelHash = `sha256:${await canonicalSha256Hex(pair.model)}`;
    const recorded = await recordedAnalysisRunRevision(record, { stateId, createdAt: "2026-10-27T11:30:45.123Z", inputManifest: manifest.manifest });
    const binding = { stateId, modelHash, inputManifest: manifest.manifest };
    await validateAnalysisRunV03(recorded, pair.source, basis, binding);
    // Each form is refused in the other's place.
    await expect(validateAnalysisRunV03(recorded, pair.source, basis)).rejects.toThrow("ANALYSIS_SOURCE_MODEL_STATE_MISMATCH");
    await expect(validateAnalysisRunV03(record, pair.source, basis, binding)).rejects.toThrow("ANALYSIS_SOURCE_MODEL_STATE_MISMATCH");
    // Another state, another model hash, another manifest.
    await expect(validateAnalysisRunV03(recorded, pair.source, basis, { ...binding, stateId: "state:other" })).rejects.toThrow("ANALYSIS_SOURCE_MODEL_STATE_MISMATCH");
    await expect(validateAnalysisRunV03(recorded, pair.source, basis, { ...binding, modelHash: `sha256:${"0".repeat(64)}` })).rejects.toThrow("ANALYSIS_RECORDED_MODEL_STATE_HASH_MISMATCH");
    const otherManifest = structuredClone(manifest.manifest);
    otherManifest.solver_basis.solver_mode = "dense_scrutiny";
    await expect(validateAnalysisRunV03(recorded, pair.source, basis, { ...binding, inputManifest: otherManifest })).rejects.toThrow("ANALYSIS_RECORDED_INPUT_MANIFEST_MISMATCH");
    // A recorded revision without its time, or with edited references, is refused.
    const untimed = structuredClone(recorded);
    untimed.analysis_run.created_at = null;
    await expect(validateAnalysisRunV03(untimed, pair.source, basis, binding)).rejects.toThrow("ANALYSIS_RECORDED_REVISION_TIME_MISSING");
    const edited = structuredClone(recorded);
    edited.analysis_run.rule_pack_refs = [{ object_type: "RulePack", ref: "rule-pack:x@1" }];
    edited.analysis_run.hashes[0].value = await canonicalSha256HexCheckedV1({ ...edited, analysis_run: { ...edited.analysis_run, hashes: edited.analysis_run.hashes.slice(1) } });
    await expect(validateAnalysisRunV03(edited, pair.source, basis, binding)).rejects.toThrow("ANALYSIS_RECORDED_REFERENCES_MISMATCH");
  });

  it("rule-pack and library references follow the manifest", () => {
    expect(recordedRulePackRefs({ active_rule_packs: [
      { rule_pack_id: "rule-pack:b", version: "2", checksum_sha256: "b", source_notice: "n" },
      { rule_pack_id: "rule-pack:a", version: "1", checksum_sha256: "a", source_notice: "n" }
    ] })).toEqual([{ object_type: "RulePack", ref: "rule-pack:b@2" }, { object_type: "RulePack", ref: "rule-pack:a@1" }]);
    const payload = {
      materials: [
        { id: "material:a", provenance_record: { library_ref: { library_kind: "material", library_id: "invented-lib", library_version: "sha256:aa", record_id: "steel" } } },
        { id: "material:b", provenance_record: { library_ref: { library_kind: "material", library_id: "invented-lib", library_version: "sha256:aa", record_id: "steel" } } }
      ],
      components: [{ id: "component:c", nested: { provenance_record: { library_ref: { library_kind: "component", library_id: "valves" } } } }],
      library_ref: { library_kind: "material", library_id: "not-under-a-provenance-record" }
    };
    // The same vector as the Rust test `rule_pack_and_library_refs_follow_the_manifest`.
    expect(recordedLibraryRefs(payload)).toEqual([
      { object_type: "Library", ref: "library:component/valves" },
      { object_type: "Library", ref: "library:material/invented-lib@sha256:aa#steel" }
    ]);
    // Code-point order, not UTF-16 order: U+FF5A sorts before U+1F600.
    const astral = { a: { provenance_record: { library_ref: { library_kind: "k", library_id: "\u{1F600}" } } }, b: { provenance_record: { library_ref: { library_kind: "k", library_id: "ｚ" } } } };
    expect(recordedLibraryRefs(astral).map((ref) => ref.ref)).toEqual(["library:k/ｚ", "library:k/\u{1F600}"]);
  });
});
