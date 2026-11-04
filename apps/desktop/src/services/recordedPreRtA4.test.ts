// RT-A4 §10.4 (DEL-05-02 Design `absolute-sum-srss-combinations.md`, "Recorded results from before
// RT-A4 (historical read)"): runs recorded before RT-A4 keep main's read on the recorded-run read
// and compare paths only. Fixtures: fixtures/persistence/recorded_pre_rt_a4_pf (main 6c3cb46d3's
// recorded preview-physics-2 runs verbatim; main-built records of main's verbatim Timoshenko and
// successor statements; #101's physics-retained-1). At a4ff570dc every one of these records was
// refused by the record validator (SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS, or
// ANALYSIS_SEMANTIC_CONTRACT_MISMATCH for physics-retained-1); the current-path assertions below
// are that refusal, kept.
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import type { AnalysisRunRead } from "./analysisRunService";
import type { ModelStateSummary } from "./modelStateService";
import type { AnalysisRunEnvelope, LocalProjectEnvelope, MechanicsResult, PreviewModel } from "../types";
import { analysisRecordProjection, buildAnalysisRunV03, recordedAnalysisRunRevision, validateAnalysisRunV03, verifyAnalysisRunRecord } from "./analysisRunCompatibility";
import { canonicalSha256HexCheckedV1, computeModelHash, computeProjectEnvelopeHash } from "./hashService";
import { buildHistoricalRunContext, buildRecordedRunContext, recordedRunBindingFindings } from "../features/results/HistoricalRunContext";
import { numericalResultStanding, recordedResultStanding } from "../features/results/resultStanding";
import { readerDispatch, readerValidate, recordedReaderDispatch, recordedReaderValidate } from "../features/results/resultReader";
import { validateRetainedPrecision, validateRetainedPrecisionRecorded } from "../features/results/retainedPrecisionReader";
import { recordedRunSide } from "../features/comparison/runComparisonSides";
import { COMPARISON_RUN_RECORD_MISMATCH } from "../features/comparison/runComparison";
import { RECORDED_PRE_RT_A4_TABLES, recordedPreRtA4Binding, recordedPreRtA4Table } from "./recordedPreRtA4";

const root = resolve(__dirname, "../../../../");
const fixture = (path: string) => readFileSync(resolve(root, "fixtures/persistence/recorded_pre_rt_a4_pf", path), "utf8");
const json = <T,>(path: string) => JSON.parse(fixture(path)) as T;
const statementOf = (document: Record<string, unknown>) => ("producer" in document ? document : document.source) as MechanicsResult;

type Variant = { name: string; record: () => AnalysisRunEnvelope; source: () => MechanicsResult; sha256: string; refusal: string };
const VARIANTS: Variant[] = [
  { name: "preview-physics-2", record: () => json<{ run_record: AnalysisRunEnvelope }>("preview_physics_2/transient_run_input.json").run_record,
    source: () => json<MechanicsResult>("preview_physics_2/preview_physics_connected_sparse.json"),
    sha256: "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2", refusal: "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS" },
  { name: "preview-physics-timoshenko-1", record: () => json("preview_physics_timoshenko_1/preview_physics_timoshenko_straight_sparse.analysis_run.json"),
    source: () => json<MechanicsResult>("preview_physics_timoshenko_1/preview_physics_timoshenko_straight_sparse.json"),
    sha256: "a7b83c37a4af9bf70f56513f80883003a2a3860387319311ea2e967dd0cf764b", refusal: "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS" },
  { name: "preview-physics-retained-1", record: () => json("preview_physics_retained_1/retained_precision_milestone_successor_sparse_interactive.analysis_run.json"),
    source: () => statementOf(json("preview_physics_retained_1/retained_precision_milestone_successor_sparse_interactive.json")),
    // The desktop's current record validator refuses the successor at its header (no current route).
    sha256: "b001070e5fedeeb101522b22ce0d61bb7cf85f82e86b8d851713f8d9fc8c4c46", refusal: "ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH" },
  { name: "physics-retained-1 (main)", record: () => json("physics_retained_1/main/retained_precision_exact_successor_sparse_interactive.analysis_run.json"),
    source: () => statementOf(json("physics_retained_1/main/retained_precision_exact_successor_sparse_interactive.json")),
    sha256: "cf29fd5fc5d61de0f1133e9668b05f3d62ae8f696944fe04de7470029f5357dd", refusal: "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH" },
  { name: "physics-retained-1 (#101)", record: () => json("physics_retained_1/pr101/retained_precision_exact_successor_sparse_interactive.analysis_run.json"),
    source: () => statementOf(json("physics_retained_1/pr101/retained_precision_exact_successor_sparse_interactive.json")),
    sha256: "045f6f0d8ce350246b21fb712d2dd14bf3a7f7f3690e66e062cbbf77d3696a3b", refusal: "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH" },  { name: "physics-retained-1 (main, RT-A4)", record: () => json("physics_retained_1/main_bbf384dec/retained_precision_exact_successor_sparse_interactive.analysis_run.json"),
    source: () => statementOf(json("physics_retained_1/main_bbf384dec/retained_precision_exact_successor_sparse_interactive.json")),
    sha256: "040ef9a613607cb6cb4aab6f34646481efd81e060681c364b72df4e129c8c034", refusal: "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH" },
  { name: "physics-retained-1 (main, before the combination certificate)", record: () => json("physics_retained_1/main_252bef969/retained_precision_exact_successor_sparse_interactive.analysis_run.json"),
    source: () => statementOf(json("physics_retained_1/main_252bef969/retained_precision_exact_successor_sparse_interactive.json")),
    sha256: "85c2678a78e794b44761c11efb73c215645c7e2560d802ac8ad6075e7804c691", refusal: "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH" },
];
const RECORDED = { recordedRead: true } as const;
const basis = (record: AnalysisRunEnvelope) => record.analysis_run.load_basis_refs;

async function reseal(record: AnalysisRunEnvelope): Promise<AnalysisRunEnvelope> {
  const claim = record.analysis_run.hashes.find(hash => hash.payload_scope === "analysis_run_record")!;
  claim.value = await canonicalSha256HexCheckedV1(analysisRecordProjection(record));
  return record;
}
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
async function rebind(record: AnalysisRunEnvelope, binding: { id: string; sha256: string }, rows = true): Promise<AnalysisRunEnvelope> {
  const copy = structuredClone(record) as Json;
  copy.analysis_run.reproducibility.semantic_contract = { ...binding };
  if (rows) for (const ref of copy.analysis_run.result_refs) ref.semantic_contract = { ...ref.semantic_contract, ...binding };
  return reseal(copy);
}
const CURRENT_TABLES: Record<string, string> = {
  "openpipestress.result_semantics/0.3.0/preview-physics-2": "preview_physics_2",
  "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1": "preview_physics_timoshenko_1",
  "openpipestress.result_semantics/0.3.0/preview-physics-retained-1": "preview_physics_retained_1",
  "openpipestress.result_semantics/0.3.0/physics-retained-1": "physics_retained_1",
};
/** The current table's binding of the statement's identity (its file's SHA-256). */
const current = (source: MechanicsResult) => {
  const id = source.producer!.semantic_contract_id;
  const bytes = readFileSync(resolve(root, `fixtures/results/semantic_contract_v0_3_${CURRENT_TABLES[id]}.json`));
  return { id, sha256: createHash("sha256").update(bytes).digest("hex") };
};

describe("a run recorded before RT-A4 (historical read)", () => {
  it.each(VARIANTS)("$name reads as main read it on the recorded read, and is refused on the current path", async variant => {
    const record = variant.record(), source = variant.source();
    const before = JSON.stringify([record, source]);
    const binding = { id: source.producer!.semantic_contract_id, sha256: variant.sha256 };
    expect(record.analysis_run.reproducibility.semantic_contract).toEqual(binding);
    expect(recordedPreRtA4Binding(binding)).toEqual(binding);
    expect(current(source).sha256).not.toBe(variant.sha256);
    expect(await verifyAnalysisRunRecord(record)).toBe("match");
    await validateAnalysisRunV03(record, source, basis(record), undefined, RECORDED);
    await expect(validateAnalysisRunV03(record, source, basis(record))).rejects.toThrow(variant.refusal);
    expect(JSON.stringify([record, source])).toBe(before);
  });

  it.each(VARIANTS.slice(0, 3))("$name stays refused on every current admission path", async variant => {
    const source = variant.source(), binding = { id: source.producer!.semantic_contract_id, sha256: variant.sha256 };
    const manifest = { manifest_ref: { object_type: "InputManifest", ref: "input-manifest:current" }, manifest_sha256: "1".repeat(64),
      manifest: { model_basis: { model_ref: source.model_ref }, solver_basis: { solver_name: "s", solver_version: "v", solver_build_ref: "b" }, active_rule_packs: [] } };
    // The successor's header itself is refused at the current dispatch (its transport reading
    // checks the base limitations), so it has no current route at all.
    const refusal = variant.name === "preview-physics-retained-1" ? "SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED" : "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS";
    await expect(buildAnalysisRunV03(source, manifest as never)).rejects.toThrow(refusal);
    expect(numericalResultStanding(source).eligible).toBe(false);
    if (variant.name === "preview-physics-retained-1") {
      await expect(validateRetainedPrecision(source)).rejects.toThrow("SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS");
      expect((await validateRetainedPrecisionRecorded(source, binding)).publication_sha256).toBeTruthy();
    } else {
      expect(() => readerValidate("preview-physics", source)).toThrow("SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS");
      expect(recordedReaderValidate(source, binding)).toEqual({});
      expect(recordedReaderDispatch(source, binding).binding).toEqual(binding);
      expect(recordedResultStanding(source, binding).findings).not.toContain("SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS");
    }
    // Nothing leaks past the recorded read.
    if (variant.name === "preview-physics-retained-1") expect(readerDispatch(source).route).toBe("unsupported");
    else {
      expect(readerDispatch(source).binding).toEqual(current(source));
      expect(numericalResultStanding(source).findings).toContain("SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS");
    }
  });

  it.each(VARIANTS)("$name: a tampered or mismatched variant is refused on the recorded read", async variant => {
    const record = variant.record(), source = variant.source();
    const id = source.producer!.semantic_contract_id, now = current(source);
    for (const sha256 of ["0".repeat(64), `${variant.sha256.slice(0, -1)}${variant.sha256.endsWith("0") ? "1" : "0"}`]) {
      await expect(validateAnalysisRunV03(await rebind(record, { id, sha256 }), source, basis(record), undefined, RECORDED)).rejects.toThrow();
    }
    const mixed = structuredClone(record) as Json;
    mixed.analysis_run.result_refs[0].semantic_contract = { ...mixed.analysis_run.result_refs[0].semantic_contract, sha256: now.sha256 };
    await expect(validateAnalysisRunV03(await reseal(mixed), source, basis(record), undefined, RECORDED)).rejects.toThrow("ANALYSIS_ROW_SOURCE_BINDING_MISMATCH");
    if (variant.refusal !== "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH") {
      await expect(validateAnalysisRunV03(await rebind(record, now), source, basis(record), undefined, RECORDED)).rejects.toThrow(variant.refusal);
    }
    for (const other of RECORDED_PRE_RT_A4_TABLES.filter(entry => entry.id !== id)) {
      await expect(validateAnalysisRunV03(await rebind(record, { id: other.id, sha256: other.sha256 }), source, basis(record), undefined, RECORDED)).rejects.toThrow(/SOURCE_RECORDED_BINDING_IDENTITY/);
    }
    const tampered = structuredClone(record);
    tampered.analysis_run.run_name += " ";
    await expect(validateAnalysisRunV03(tampered, source, basis(record), undefined, RECORDED)).rejects.toThrow("ANALYSIS_RECORD_CHECKSUM_MISMATCH");
    const changed = structuredClone(source);
    changed.diagnostics = [...changed.diagnostics];
    changed.status = { ...changed.status, rule_check: changed.status.rule_check === "USER_RULE_CHECKED" ? "USER_RULE_FAILED" : "USER_RULE_CHECKED" };
    await expect(validateAnalysisRunV03(record, changed, basis(record), undefined, RECORDED)).rejects.toThrow();
  });

  it("a current statement under a variant binding is refused (old digest, new limitations)", async () => {
    const source = JSON.parse(readFileSync(resolve(root, "fixtures/results/preview_physics_connected_sparse.json"), "utf8")) as MechanicsResult;
    const binding = { id: source.producer!.semantic_contract_id, sha256: VARIANTS[0].sha256 };
    expect(readerValidate("preview-physics", source)).toEqual({});
    expect(() => recordedReaderValidate(source, binding)).toThrow("SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS");
  });

  it("each registry table is its pinned bytes, and only an exact binding names one", async () => {
    for (const variant of RECORDED_PRE_RT_A4_TABLES) {
      const table = await recordedPreRtA4Table(variant) as { semantic_contract_id: string };
      expect(table.semantic_contract_id).toBe(variant.id);
    }
    expect(recordedPreRtA4Binding({ id: "openpipestress.result_semantics/0.3.0/preview-physics-2", sha256: VARIANTS[0].sha256, extra: 1 })).toBeNull();
    expect(recordedPreRtA4Binding({ id: "openpipestress.result_semantics/0.3.0/preview-physics-2", sha256: "e7b93f416f700f8efdb31bf1ba26dedde52963d768d6ead5a73dbf1c645a360c" })).toBeNull();
    await expect(recordedPreRtA4Table({ id: "openpipestress.result_semantics/0.3.0/preview-physics-2", sha256: "0".repeat(64) })).rejects.toThrow("SOURCE_RECORDED_BINDING_UNKNOWN");
  });
});

// main's recorded revision of its preview-physics-2 run (the desktop store's committed witness at 6c3cb46d3).
const transient = json<{ run_record: AnalysisRunEnvelope; input_manifest_canonical: string }>("preview_physics_2/transient_run_input.json");
const witness = json<{ state_id: string; state_model_hash: string; created_at: string; recording_id: string; run_record_hash: string; recorded_record: AnalysisRunEnvelope }>("preview_physics_2/recorded_run_witness.json");
const resultText = fixture("preview_physics_2/preview_physics_connected_sparse.json");
const projectId = JSON.parse(transient.input_manifest_canonical).model_basis.model_ref as string;
function read(): AnalysisRunRead {
  const state = {
    project_id: projectId, state_id: witness.state_id, state_kind: "post_solve_snapshot", created_at: witness.created_at, parent_state_id: null,
    model_hash: witness.state_model_hash, record_hash: "r", labels_updated_at: witness.created_at, child_state_ids: [], recorded_run_count: 1,
    analysis_status: ["HUMAN_REVIEW_REQUIRED"], state_name: "After solve", tags: [], notes: [], assumptions: []
  } as ModelStateSummary;
  return {
    run: {
      project_id: projectId, recording_id: witness.recording_id, state_id: witness.state_id, producer_run_id: witness.recorded_record.analysis_run.run_id,
      created_at: witness.created_at, run_record_hash: witness.run_record_hash, result_hash: "r", labels_updated_at: witness.created_at,
      analysis_status: witness.recorded_record.analysis_run.analysis_status, run_name: "Witness run", tags: [], notes: [], assumptions: []
    },
    state, record: structuredClone(witness.recorded_record), result_json: resultText, input_manifest_canonical: transient.input_manifest_canonical
  };
}

describe("main's recorded preview-physics-2 run (historical read, compare and re-save)", () => {
  it("reads as a recorded run with no binding finding, and stays historical", async () => {
    const context = await buildRecordedRunContext(read());
    expect(context.designation).toBe("recorded_run");
    expect(recordedRunBindingFindings(context)).toEqual([]);
    expect(context.findings).not.toContain("SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS");
    // Display rule: the historical reader's findings (the statement's standing as it read when
    // recorded) are kept apart, scoped to the table they were read under; the ordinary findings
    // are the reopen's checks, and there are none.
    const model = JSON.parse(transient.input_manifest_canonical).model_basis.model_payload as PreviewModel;
    const binding = witness.recorded_record.analysis_run.reproducibility.semantic_contract!;
    expect(context.recordedTableRead).toEqual({ id: binding.id, sha256: VARIANTS[0].sha256 });
    expect(context.historicalReaderFindings).toEqual(recordedResultStanding(context.mechanicsResult!, binding, model).findings);
    expect(context.findings).toEqual([]);
    expect(context.checkFindings).toEqual([]);
  });

  it("is offered for comparison, with the standing it read with when recorded, labelled with its table", async () => {
    const side = await recordedRunSide(read());
    expect(side.kind).toBe("recorded");
    expect(side.standing.findings).not.toContain("SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS");
    const model = JSON.parse(transient.input_manifest_canonical).model_basis.model_payload as PreviewModel;
    const binding = { id: witness.recorded_record.analysis_run.reproducibility.semantic_contract!.id, sha256: VARIANTS[0].sha256 };
    expect(side.recordedTableRead).toEqual(binding);
    expect(side.standing).toEqual(recordedResultStanding(JSON.parse(resultText), binding, model));
    // A tampered record is still refused for comparison.
    const tampered = read();
    tampered.record.analysis_run.run_name += " ";
    await expect(recordedRunSide(tampered)).rejects.toThrow();
  });

  // A failed reopen validation is never masked by the favourable read of the variant: the stored
  // record below reads under its variant as main's does (same statement, same table), but its
  // record is not main's (resealed, so the store and checksum agree) and the record validator
  // refuses it, with a code outside the HISTORICAL_/RECORDED_RUN_/ANALYSIS prefixes.
  async function resealedRead(edit: (record: Json) => void | Promise<void>): Promise<AnalysisRunRead> {
    const tampered = read();
    await edit(tampered.record);
    await reseal(tampered.record);
    tampered.run.run_record_hash = tampered.record.analysis_run.hashes.find(hash => hash.payload_scope === "analysis_run_record")!.value as string;
    return tampered;
  }
  it.each([
    { name: "a record validator refusal", refusal: "SOURCE_RECOVERY_METADATA_CONTRADICTION", favourable: true,
      edit: (record: Json) => { record.analysis_run.source_block_recovery = { forged: true }; } },
    { name: "a refused recorded binding (a variant of another identity)", refusal: "SOURCE_RECORDED_BINDING_IDENTITY", favourable: false,
      edit: async (record: Json) => {
        const other = RECORDED_PRE_RT_A4_TABLES.find(entry => entry.id !== record.analysis_run.reproducibility.semantic_contract.id)!;
        Object.assign(record, await rebind(record, { id: other.id, sha256: other.sha256 }));
      } },
  ])("a stored record whose reopen validation fails ($name) shows the failure and is not compared", async ({ refusal, favourable, edit }) => {
    const tampered = await resealedRead(edit);
    expect(await verifyAnalysisRunRecord(tampered.record)).toBe("match");
    const context = await buildRecordedRunContext(tampered);
    expect(context.findings).toContain(refusal);
    expect(recordedRunBindingFindings(context)).toContain(refusal);
    expect(context.historicalReaderFindings).not.toContain(refusal);
    if (favourable) {
      // The variant read itself is the untampered run's: favourable, as recorded.
      const untampered = await buildRecordedRunContext(read());
      expect(context.recordedTableRead).toEqual(untampered.recordedTableRead);
      expect(context.historicalReaderFindings).toEqual(untampered.historicalReaderFindings);
      expect((await recordedRunSide(read())).standing.eligible).toBe(true);
    }
    await expect(recordedRunSide(tampered)).rejects.toThrow(COMPARISON_RUN_RECORD_MISMATCH);
  });

  it("is re-saved as main saved it: the recorded revision of main's transient record is main's witness", async () => {
    const manifest = JSON.parse(transient.input_manifest_canonical);
    const resaved = await recordedAnalysisRunRevision(transient.run_record, { stateId: witness.state_id, createdAt: witness.created_at, inputManifest: manifest });
    expect(await canonicalSha256HexCheckedV1(resaved)).toBe(await canonicalSha256HexCheckedV1(witness.recorded_record));
    expect(resaved.analysis_run.reproducibility.semantic_contract?.sha256).toBe(VARIANTS[0].sha256);
    await validateAnalysisRunV03(resaved, JSON.parse(resultText), undefined, { stateId: witness.state_id, modelHash: witness.state_model_hash, inputManifest: manifest }, RECORDED);
  });

  it("main's S2 run keeps main's own refusal, by name", async () => {
    const s2 = json<{ input_manifest_canonical: string; state_id: string; state_model_hash: string; recorded_record: AnalysisRunEnvelope }>("preview_physics_2/s2_unresolved_rule_pack_run.json");
    const state = { stateId: s2.state_id, modelHash: s2.state_model_hash, inputManifest: JSON.parse(s2.input_manifest_canonical) };
    await expect(validateAnalysisRunV03(s2.recorded_record, JSON.parse(resultText), undefined, state, RECORDED)).rejects.toThrow("ANALYSIS-RUN-RULE-PACKS-NOT-RESOLVED");
    await expect(validateAnalysisRunV03(s2.recorded_record, JSON.parse(resultText), undefined, state)).rejects.toThrow("SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS");
  });

  it("a project saved before RT-A4 reopens as a historical run with no record finding", async () => {
    const model = JSON.parse(transient.input_manifest_canonical).model_basis.model_payload as PreviewModel;
    const mechanics_result = JSON.parse(resultText) as MechanicsResult, analysis_run = structuredClone(transient.run_record);
    const model_hash = await computeModelHash(model);
    const payload = { model, mechanics_result, analysis_run, model_hash, editor_intents: [], proposal: null, selected_review_target: null };
    const opened = { ...payload, project_envelope_hash: await computeProjectEnvelopeHash(payload) } as unknown as LocalProjectEnvelope;
    const before = JSON.stringify(opened);
    const context = (await buildHistoricalRunContext(opened))!;
    expect(context.designation).toBe("historical_saved_run");
    expect(context.findings.filter(finding => /^ANALYSIS[_-]|^SOURCE_/.test(finding))).toEqual([]);
    expect(context.findings).toContain("HISTORICAL_INPUT_MANIFEST_MISSING");
    expect(context.recordedTableRead).toEqual({ id: mechanics_result.producer!.semantic_contract_id, sha256: VARIANTS[0].sha256 });
    expect(context.historicalReaderFindings).toEqual(recordedResultStanding(mechanics_result, context.recordedTableRead!, model).findings);
    expect(context.findings.filter(finding => context.historicalReaderFindings.includes(finding))).toEqual([]);
    expect(JSON.stringify(opened)).toBe(before);
  });
});
