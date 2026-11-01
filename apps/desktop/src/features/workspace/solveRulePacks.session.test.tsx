/** FR-CMP-002 rule-pack references (DEL-14-01 Design `desktop-states-runs-comparison.md` §3):
 * the solve resolves the model's `rule_pack_refs` against the local rule-pack store and records
 * the resolved packs in the input manifest's `active_rule_packs`, in the model's order, and each
 * excluded reference as a diagnostic of the run record; Record Run carries the packs into the
 * recorded revision's `rule_pack_refs` and keeps the diagnostics. The solve is the native job
 * transport replay of committed producer bytes (unit transport replay; NOT a native UI
 * qualification witness); the store's own derivation is `analysis_runs_tests.rs`. All
 * rule-pack data are invented. C1 (DEL-04-05 `c1-native-custody-plan.md` slice 4): the session
 * holds the result natively and the record's header; the full record, whose bytes these tests
 * compare, is the C2 full record the session builds on the result's full copy (`loadC2Full`),
 * checked against the native record's checksums. */
import { act, cleanup, render, screen, waitFor } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { hex, inventedEnvelope, inventedReference } from "../../test/inventedRulePacks";
import { getLocalStorageCapability } from "../../services/projectService";
import { loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { canonicalJsonString, canonicalSha256HexCheckedV1, computeModelHash } from "../../services/hashService";
import { analysisRecordProjection, ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH, recordedAnalysisRunRevision } from "../../services/analysisRunCompatibility";
import type { AnalysisRunRead, AnalysisRunSummary, RunLabels } from "../../services/analysisRunService";
import type { CurrentSessionInputManifest } from "../../services/inputManifestService";
import type { ModelStateSummary } from "../../services/modelStateService";
import { RULE_PACK_REF_CHECKSUM_MISMATCH, RULE_PACK_REF_UNRESOLVED } from "../../services/modelRulePackResolution";
import type { LocalRulePackEnvelope } from "../../services/rulePackService";
import { buildRecordedRunContext, recordedRunBindingFindings } from "../results/HistoricalRunContext";
import { SolvePanel } from "../solve/SolvePanel";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";
import { isNativeAnalysisRun, isNativeResult, nativeRunPreview } from "../../services/nativeResult";
import type { C2FullResult } from "../results/C2ResultGate";
import type { AnalysisRunEnvelope } from "../../types";
import type { LocalProjectEnvelope, PreviewModel, RulePackReference } from "../../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as Json).__TAURI_INTERNALS__;
});

const LABELS: RunLabels = { run_name: "Run with invented packs", tags: [], notes: [], assumptions: [] };
const TRANSIENT_INPUT = resolve(__dirname, "../../../../../fixtures/persistence/transient_run_input.json");

let sequence = 0;
/** The session over the replayed sparse solve of the committed preview pair, its model given
 * `references`, a fake rule-pack store holding `packs` and a fake run store. */
async function session(references: RulePackReference[] | null, packs: LocalRulePackEnvelope[] = [], options: { tauri?: boolean } = {}) {
  const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const model = structuredClone(pair.model) as PreviewModel;
  if (references) model.rule_pack_refs = references;
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode: "sparse_interactive" })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return pair.bytes;
  }, { jobIdPrefix: `unit-solve-rule-packs:${++sequence}` });
  const store = { base: null as string | null, states: [] as ModelStateSummary[], runs: [] as AnalysisRunSummary[], records: [] as Json[], reads: new Map<string, AnalysisRunRead>() };
  invokeMock.mockImplementation(async (command: string, args: Json, invokeOptions?: unknown) => {
    switch (command) {
      case "get_local_storage_capability": return storage;
      case "load_design_knowledge": return knowledge;
      case "sync_native_shell_state": return null;
      case "check_model_library_refs": return [];
      case "load_preview_model": if (args === undefined) return structuredClone(model); break;
      case "open_local_rule_pack": return structuredClone(packs.find((pack) => pack.project_id === args.projectId && pack.rule_pack_id === args.rulePackId) ?? null);
      case "find_local_project": return store.base ? { project_id: args.projectId, project_name: "Stored", storage_mode: "local_sqlite", created_at_unix: 1, updated_at_unix: 2, revision: `rev:${store.base}`, saved_state_count: store.states.length, recorded_run_count: store.runs.length } : null;
      case "save_local_project": {
        const request = args.request as Json;
        store.base = `sha256:${request.model.project.id}#saved`;
        return {
          summary: { project_id: request.model.project.id, project_name: request.model.project.name, message: "Saved." },
          model: request.model, editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: request.mechanics_result ?? null,
          analysis_run: request.analysis_run ?? null, model_hash: request.model_hash, project_envelope_hash: { value: store.base }, model_document_version: null
        } as unknown as LocalProjectEnvelope;
      }
      case "list_model_states": return store.states.filter((state) => state.project_id === args.projectId);
      case "list_analysis_runs": return store.runs.filter((run) => run.project_id === args.projectId);
      case "record_analysis_run": {
        // The native store's derivation of the recorded revision, mirrored by the TypeScript
        // builder (its parity with Rust is `recordedRunParity.test.ts` against the witness).
        const request = args.request as Json;
        store.records.push(request);
        const created = "2026-10-24T10:00:01.000Z";
        const manifest = JSON.parse(request.input_manifest_canonical) as CurrentSessionInputManifest;
        const state = {
          ...request.new_state.labels, project_id: request.project_id, state_id: `state:${"1".padStart(32, "0")}`, state_kind: "post_solve_snapshot",
          created_at: created, parent_state_id: null, model_hash: (await computeModelHash(manifest.model_basis.model_payload))!.value, record_hash: "r",
          labels_updated_at: created, child_state_ids: [], recorded_run_count: 0, analysis_status: ["HUMAN_REVIEW_REQUIRED"]
        } as ModelStateSummary;
        store.states.push(state);
        const recorded = await recordedAnalysisRunRevision(request.run_record, { stateId: state.state_id, createdAt: created, inputManifest: manifest });
        const run: AnalysisRunSummary = {
          ...request.run_labels, project_id: request.project_id, recording_id: `runrec:${"1".padStart(32, "0")}`, state_id: state.state_id,
          producer_run_id: request.run_record.analysis_run.run_id, created_at: created,
          run_record_hash: recorded.analysis_run.hashes.find((hash) => hash.payload_scope === "analysis_run_record")!.value,
          result_hash: "r", labels_updated_at: created, analysis_status: request.run_record.analysis_run.analysis_status
        };
        store.runs.push(run);
        store.reads.set(run.recording_id, { run, state, record: recorded, result_json: request.result_json, input_manifest_canonical: request.input_manifest_canonical });
        return { run, state, state_outcome: "created", message: `Recorded run "${run.run_name}".` };
      }
      case "read_analysis_run": return structuredClone(store.reads.get(args.recordingId));
    }
    return transport.invoke(command, args, invokeOptions);
  });
  if (options.tauri !== false) (window as Json).__TAURI_INTERNALS__ = {};
  let current!: WorkspaceSession;
  function Harness() {
    current = useWorkspaceSession();
    const results = current.results;
    return current.model.model ? (
      <SolvePanel analysisRun={results.analysisRun} inputManifest={results.inputManifest?.manifest ?? null} model={current.model.model} result={results.result}
        running={results.running} solveJob={results.solveJob} solverMode={results.solverMode} onCancel={() => undefined} onRun={() => undefined} onSolverModeChange={() => undefined} />
    ) : null;
  }
  render(<Harness />);
  await waitFor(() => expect(current.model.model).not.toBeNull());
  await waitFor(() => expect(current.model.modelHash).not.toBeNull());
  return { state: () => current, store, model };
}

async function solve(state: () => WorkspaceSession) {
  await act(async () => { await state().results.handleRun(); });
  await waitFor(() => expect(state().results.solveJob.state).toBe("completed"));
  expect(state().results.inputManifest).not.toBeNull();
  return state().results.inputManifest!;
}

/** The session result's C2 full copy and the full record of its current record. */
async function c2Full(state: () => WorkspaceSession): Promise<C2FullResult & { analysisRun: AnalysisRunEnvelope }> {
  let full: C2FullResult | null = null;
  await act(async () => { full = await state().results.loadC2Full(state().results.result!); });
  expect(full!.analysisRun).not.toBeNull();
  return full! as C2FullResult & { analysisRun: AnalysisRunEnvelope };
}

const calls = (command: string) => invokeMock.mock.calls.filter(([name]) => name === command).map(([, args]) => args as Json);

describe("the solve records the model's rule packs in the input manifest (FR-CMP-002)", () => {
  it("a model without references keeps the committed manifest bytes and makes no rule-pack store call", async () => {
    const { state } = await session(null);
    const evidence = await solve(state);
    const committed = JSON.parse(readFileSync(TRANSIENT_INPUT, "utf8")) as { input_manifest_canonical: string; run_record: Json };
    // Byte-identical to the pinned manifest (written before the change by the session's builder).
    expect(evidence.canonical_bytes).toBe(committed.input_manifest_canonical);
    expect(evidence.manifest_sha256).toBe(committed.run_record.analysis_run.reproducibility.input_manifest_hashes[0].value);
    expect(evidence.manifest_sha256).toBe("cca3862b0ec08ffeceada7188a5be6f505e81feb06ea28cc9a062dee96a89c83");
    expect(evidence.manifest.active_rule_packs).toEqual([]);
    expect(Object.keys(evidence.manifest).sort()).toEqual(["active_rule_packs", "document_kind", "external_assets", "identity_policy", "load_basis", "model_basis", "replay_boundary", "schema_version", "solver_basis", "unit_basis"]);
    // The transient record is the committed one, byte for byte (its own hash included): the
    // session's native record header carries its checksum, and the full record is the same bytes.
    const header = state().results.analysisRun!;
    expect(isNativeAnalysisRun(header)).toBe(true);
    expect(header.analysis_run.hashes).toStrictEqual(committed.run_record.analysis_run.hashes);
    expect(await canonicalJsonString((await c2Full(state)).analysisRun)).toBe(await canonicalJsonString(committed.run_record));
    expect(calls("open_local_rule_pack")).toHaveLength(0);
    expect(screen.queryByTestId("solve-rule-pack-refs")).toBeNull();
  });

  it("resolved packs enter in the model's order; unresolved and changed references are excluded and named with the solve; Record Run carries the packs into rule_pack_refs", async () => {
    const references = [
      inventedReference("invented-pack-zeta", hex("e")),
      inventedReference("invented-pack-missing", hex("2")),
      inventedReference("invented-pack-alpha", hex("a"), "3.1.0"),
      inventedReference("invented-pack-changed", hex("3"))
    ];
    const project = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }).model.project.id;
    const packs = [
      inventedEnvelope(project, "invented-pack-alpha", hex("a"), { version: "3.1.0" }),
      inventedEnvelope(project, "invented-pack-zeta", hex("e")),
      inventedEnvelope(project, "invented-pack-changed", hex("7")),
      // Another project's pack of the same id is not this project's.
      inventedEnvelope("project:invented-other", "invented-pack-missing", hex("2"))
    ];
    const { state, store, model } = await session(references, packs);
    const evidence = await solve(state);
    expect(calls("open_local_rule_pack")).toEqual(references.map((reference) => ({ projectId: model.project.id, rulePackId: reference.rule_pack_id })));
    expect(evidence.manifest.active_rule_packs).toEqual([
      { rule_pack_id: "invented-pack-zeta", version: "1.0.0", checksum_sha256: hex("e"), source_notice: "Invented notice for invented-pack-zeta; not an engineering basis." },
      { rule_pack_id: "invented-pack-alpha", version: "3.1.0", checksum_sha256: hex("a"), source_notice: "Invented notice for invented-pack-alpha; not an engineering basis." }
    ]);
    // The excluded references are diagnostics of the transient record, after the producer's (the
    // native record header carries them whole; the full record holds them after the producer's).
    const transient = state().results.analysisRun!;
    expect(isNativeAnalysisRun(transient) && nativeRunPreview(transient).rule_pack_ref_diagnostics.map((item) => (item.source_annotation as { code: string }).code))
      .toEqual([RULE_PACK_REF_UNRESOLVED, RULE_PACK_REF_CHECKSUM_MISMATCH]);
    const fullRecord = (await c2Full(state)).analysisRun;
    const producerCount = state().results.result!.diagnostics.length;
    expect(fullRecord.analysis_run.diagnostics!.slice(producerCount).map((item) => (item.source_annotation as { code: string; affected_refs: string[] })))
      .toEqual([
        expect.objectContaining({ code: RULE_PACK_REF_UNRESOLVED, affected_refs: ["invented-pack-missing@1.0.0"] }),
        expect.objectContaining({ code: RULE_PACK_REF_CHECKSUM_MISMATCH, affected_refs: ["invented-pack-changed@1.0.0"] })
      ]);
    // Visible with the solve.
    expect(screen.getByTestId("solve-rule-pack-refs-recorded").textContent).toContain("invented-pack-zeta v1.0.0");
    const diagnostics = screen.getAllByTestId("solve-rule-pack-refs-diagnostic").map((node) => node.textContent);
    expect(diagnostics).toHaveLength(2);
    expect(diagnostics[0]).toMatch(/^MODEL-RULE-PACK-REF-UNRESOLVED \(warning\): Rule pack invented-pack-missing@1\.0\.0 /);
    expect(diagnostics[1]).toMatch(/^MODEL-RULE-PACK-REF-CHECKSUM-MISMATCH \(warning\): Rule pack invented-pack-changed@1\.0\.0 /);
    // The transient record is unchanged: its rule_pack_refs stay empty until Record Run.
    expect(state().results.analysisRun!.analysis_run.rule_pack_refs).toEqual([]);
    // Shown in the Record Run form, before recording.
    expect(state().project.history.recordRunManifest).toBe(evidence.manifest);
    expect(state().project.history.recordRunRecord).toBe(transient);

    let recorded = false;
    await act(async () => { recorded = await state().project.history.recordRun({ labels: LABELS, snapshotName: "After solve" }); });
    expect(recorded).toBe(true);
    const request = store.records[0];
    expect(request.input_manifest_canonical).toBe(evidence.canonical_bytes);
    expect(request.run_record).toEqual(fullRecord);
    const read = store.reads.get(store.runs[0].recording_id)!;
    // The recorded revision keeps the diagnostics unchanged, under its new hash.
    expect(read.record.analysis_run.diagnostics).toEqual(fullRecord.analysis_run.diagnostics);
    expect(read.record.analysis_run.rule_pack_refs).toEqual([
      { object_type: "RulePack", ref: "invented-pack-zeta@1.0.0" },
      { object_type: "RulePack", ref: "invented-pack-alpha@3.1.0" }
    ]);
    // The recorded run's check accepts the non-empty packs and shows the excluded references.
    const context = await buildRecordedRunContext(read);
    expect(recordedRunBindingFindings(context)).toEqual([]);
    await act(async () => { await state().project.history.checkRun(store.runs[0].recording_id); });
    const check = state().project.history.runCheck!;
    expect(check.failure).toBeNull();
    expect(check.context && recordedRunBindingFindings(check.context)).toEqual([]);
    expect(check.manifest?.active_rule_packs.map((pack) => pack.rule_pack_id)).toEqual(["invented-pack-zeta", "invented-pack-alpha"]);
    expect(check.context?.analysisRun?.analysis_run.diagnostics).toEqual(fullRecord.analysis_run.diagnostics);
  });

  it("a recorded run whose rule-pack diagnostics do not name exactly the references its manifest leaves out is flagged", async () => {
    const project = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }).model.project.id;
    const { state, store } = await session([inventedReference("invented-pack-alpha", hex("a")), inventedReference("invented-pack-missing", hex("2"))], [inventedEnvelope(project, "invented-pack-alpha", hex("a"))]);
    await solve(state);
    await act(async () => { await state().project.history.recordRun({ labels: LABELS, snapshotName: "After solve" }); });
    const read = structuredClone(store.reads.get(store.runs[0].recording_id)!);
    // A coherent forgery: the diagnostic removed and the record re-hashed.
    read.record.analysis_run.diagnostics!.pop();
    read.record.analysis_run.hashes.find((hash) => hash.payload_scope === "analysis_run_record")!.value = await canonicalSha256HexCheckedV1(analysisRecordProjection(read.record));
    read.run.run_record_hash = read.record.analysis_run.hashes.find((hash) => hash.payload_scope === "analysis_run_record")!.value;
    const context = await buildRecordedRunContext(read);
    expect(recordedRunBindingFindings(context)).toEqual([ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH]);
  });

  it("a recorded run whose record's rule_pack_refs differ from its stored manifest's packs is flagged", async () => {
    const project = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }).model.project.id;
    const { state, store } = await session([inventedReference("invented-pack-alpha", hex("a"))], [inventedEnvelope(project, "invented-pack-alpha", hex("a"))]);
    await solve(state);
    await act(async () => { await state().project.history.recordRun({ labels: LABELS, snapshotName: "After solve" }); });
    const read = structuredClone(store.reads.get(store.runs[0].recording_id)!);
    read.record.analysis_run.rule_pack_refs = [];
    const context = await buildRecordedRunContext(read);
    expect(recordedRunBindingFindings(context).length).toBeGreaterThan(0);
  });

  it("two unresolved references whose flattened identities would collide publish the solve and record", async () => {
    const references = [inventedReference("invented-pack@branch", hex("2")), inventedReference("invented-pack", hex("3"), "branch@1.0.0")];
    const { state, store } = await session(references, []);
    await solve(state);
    const transient = (await c2Full(state)).analysisRun;
    const producerCount = state().results.result!.diagnostics.length;
    expect(transient.analysis_run.diagnostics!.slice(producerCount).map((item) => (item.source_annotation as { affected_refs: string[] }).affected_refs[0]))
      .toEqual(["invented-pack%40branch@1.0.0", "invented-pack@branch%401.0.0"]);
    let recorded = false;
    await act(async () => { recorded = await state().project.history.recordRun({ labels: LABELS, snapshotName: "After solve" }); });
    expect(recorded).toBe(true);
    const read = store.reads.get(store.runs[0].recording_id)!;
    expect(read.record.analysis_run.diagnostics).toEqual(transient.analysis_run.diagnostics);
    expect(recordedRunBindingFindings(await buildRecordedRunContext(read))).toEqual([]);
  });

  it("the exclusion diagnostics survive checked, failed and cleared rule-check aggregates; the result is unchanged", async () => {
    const project = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" }).model.project.id;
    const { state } = await session([inventedReference("invented-pack-alpha", hex("a")), inventedReference("invented-pack-missing", hex("2"))], [inventedEnvelope(project, "invented-pack-alpha", hex("a"))]);
    await solve(state);
    const result = state().results.result!;
    expect(isNativeResult(result)).toBe(true);
    // The result's bytes, read through its one C2 copy; the full records are built on that copy.
    const solveFull = await c2Full(state);
    const resultBytes = await canonicalJsonString(solveFull.result);
    const solveTime = solveFull.analysisRun;
    const exclusions = solveTime.analysis_run.diagnostics!.slice(result.diagnostics.length);
    expect(exclusions).toHaveLength(1);
    const received = (run: { analysis_run: { hashes: AnalysisRunEnvelope["analysis_run"]["hashes"] } }) => run.analysis_run.hashes.filter((hash) => hash.payload_scope === "received_result");
    for (const aggregate of ["USER_RULE_CHECKED", "USER_RULE_FAILED", null] as const) {
      const before = state().results.analysisRun;
      const current = state();
      await act(async () => {
        await current.results.handleRuleCheckAggregate(aggregate, {
          projectSessionGeneration: current.model.projectSessionGeneration,
          modelRevision: current.model.uiModelRevision,
          result: current.results.currentSolvedResult,
          inputManifest: current.results.inputManifest
        });
      });
      // Identity only: a deep comparison would read the native record header's guarded members.
      await waitFor(() => expect(state().results.analysisRun !== before).toBe(true));
      expect(received(state().results.analysisRun!)).toEqual(received(solveTime));
      const revisedFull = await c2Full(state);
      const revised = revisedFull.analysisRun;
      expect(revised.analysis_run.analysis_status).toContain(aggregate ?? solveTime.analysis_run.analysis_status.find((status) => status.startsWith("RULE_"))!);
      expect(revised.analysis_run.diagnostics!.slice(result.diagnostics.length)).toEqual(exclusions);
      expect(received(revised)).toEqual(received(solveTime));
      expect(state().results.result).toBe(result);
      expect(revisedFull.result).toBe(solveFull.result);
      expect(await canonicalJsonString(revisedFull.result)).toBe(resultBytes);
    }
    // Cleared: the record is the solve-time record again, byte for byte.
    expect(await canonicalJsonString((await c2Full(state)).analysisRun)).toBe(await canonicalJsonString(solveTime));
  });

  it("the browser preview refuses the solve before any manifest, so no rule-pack resolution runs", async () => {
    const { state } = await session([inventedReference("invented-pack-alpha", hex("a"))], [], { tauri: false });
    await act(async () => { await state().results.handleRun(); });
    await waitFor(() => expect(state().results.solveJob.state).toBe("failed"));
    expect(state().results.inputManifest).toBeNull();
    expect(calls("open_local_rule_pack")).toHaveLength(0);
  });
});
