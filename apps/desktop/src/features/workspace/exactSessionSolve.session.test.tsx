/** B7b (DEL-14-02): an exact session solve on the desktop. The exact profile's producer
 * publishes pressure-1 (3.0.0/exact_pressure_v3), with or without the
 * load/reference-state group. The session solve completes: the desktop records it as
 * Python's `build_analysis_run_v0_3` does, holds the result and its record, and shows it
 * (results panel, standing, notices, the resolved-state block) through Rust's verdict. It
 * is never Current, and every output surface still refuses it by name.
 * Unit transport replay of committed producer bytes through mocked IPC; NOT a native UI
 * qualification witness. All data are invented. */
import { act, cleanup, render, renderHook, screen, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { getLocalStorageCapability, openLocalProject } from "../../services/projectService";
import { hasNativeMechanicsInvocation, loadDesignKnowledge, resetSessionResultCustodyForTests } from "../../services/previewService";
import { verifyAnalysisRunRecord, validateAnalysisRunV03, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { runRuleChecks } from "../../services/ruleCheckService";
import { readerDispatch, sourceContract } from "../results/resultReader";
import { numericalResultStanding } from "../results/resultStanding";
import { OUTPUT_SURFACES, PRESSURE_1_OUTPUT_REFUSAL, surfaceOutputRefusal, surfaceRouteRefusal } from "../results/outputPolicy";
import { loadReferenceOutputRefusal } from "../results/loadReferenceOutputAvailability";
import { ResultsPanel } from "../results/ResultsPanel";
import { KnownSemanticNotices } from "../results/KnownSemanticNotices";
import { SolvePanel } from "../solve/SolvePanel";
import { buildCurrentResultExport } from "../result-export/resultExportAdapter";
import { buildStressNeutralExportPacket } from "../stress-neutral/StressNeutralExportPanel";
import { reportPackageUnavailableReason } from "../report/reportPackageRequest";
import { NativePackagePanel } from "../native-package/NativePackagePanel";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Mode = "sparse_interactive" | "dense_scrutiny";
const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
const PRESSURE_1 = { id: "openpipestress.result_semantics/0.3.0/pressure-1", sha256: "4dc927893bf139002f44f57c5b464ab4e8b428d1de184c5c637645b440321b4e" };

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  resetSessionResultCustodyForTests();
  window.localStorage.clear();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

let sequence = 0;
/** A session whose loaded model is `model` and whose native solve hands over `bytes`. */
async function session(model: PreviewModel, bytes: string, mode: Mode) {
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const transport = createSolveJobTransport((request) => {
    if (!isDeepStrictEqual({ model: request.model, mode: request.mode }, { model, mode })) throw new Error("REPLAY_REQUEST_MISMATCH");
    return bytes;
  }, { jobIdPrefix: `unit-exact-session:${++sequence}` });
  invokeMock.mockImplementation(async (command: string, args?: unknown, options?: unknown) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(model);
    return transport.invoke(command, args, options);
  });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  const rendered = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(rendered.result.current.model.model).not.toBeNull());
  await waitFor(() => expect(rendered.result.current.model.modelHash).not.toBeNull());
  if (mode !== rendered.result.current.results.solverMode) act(() => rendered.result.current.results.setSolverMode(mode));
  await act(async () => { await rendered.result.current.results.handleRun(); });
  await waitFor(() => expect(rendered.result.current.results.solveJob.state).toBe("completed"));
  return { ...rendered, transport };
}
/** The exact profile's replay pair (pressure-1 without the load/reference-state group). */
function exactPair(mode: Mode) {
  const pair = nativeMechanicsReplayPair(mode, { profile: "pressure" });
  return { model: pair.model, bytes: pair.bytes };
}
/** A committed load/reference-state pressure-1 request and its producer bytes, with the
 * document metadata the session requires (invented labels, an empty diagnostics list and
 * the exact profile document's data boundary; the product reads no physics from them). */
function loadStatePair(mode: Mode) {
  const model = structuredClone(json("fixtures/product_preview/pressure/load_reference/connected.request.json").model) as PreviewModel;
  for (const loadCase of model.load_cases) if (loadCase.label === undefined) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
  const document = exactPair(mode).model as Json;
  Object.assign(model as Json, { diagnostics: [], data_boundary: structuredClone(document.data_boundary) });
  Object.assign(model.project as Json, { name: "Invented load/reference-state session", description: "Invented test document; no engineering data." });
  return { model, bytes: readFileSync(resolve(root, `fixtures/product_preview/pressure/load_reference/connected-${mode}.raw.json`), "utf8") };
}
/** What the session holds after the solve: the pressure-1 result, its record and manifest, never Current. */
function held(state: WorkspaceSession) {
  const { result, analysisRun, inputManifest, currentSolvedResult, solveProof } = state.results;
  const model = state.model.model!;
  expect(result).not.toBeNull();
  expect(sourceContract(result!)).toBe("pressure");
  expect(result!.producer!.semantic_contract_id).toBe(PRESSURE_1.id);
  expect(hasNativeMechanicsInvocation(result!, model)).toBe(true);
  expect(analysisRun).not.toBeNull();
  expect(inputManifest).not.toBeNull();
  expect(solveProof).toMatchObject({ state: "completed", result_run_id: result!.run_id });
  expect(currentSolvedResult).toBeNull();
  return { result: result!, analysisRun: analysisRun!, inputManifest: inputManifest!, model };
}
async function expectPythonShapedRecord(record: AnalysisRunEnvelope, result: MechanicsResult, model: PreviewModel) {
  const run = record.analysis_run;
  expect(run.run_id).toBe(result.run_id);
  expect(run.reproducibility.semantic_contract).toStrictEqual(PRESSURE_1);
  for (const ref of run.result_refs) expect({ id: ref.semantic_contract?.id, sha256: ref.semantic_contract?.sha256 }).toStrictEqual(PRESSURE_1);
  for (const absent of ["contract_evidence", "source_block_recovery", "retained_precision"]) expect(Object.hasOwn(run, absent), absent).toBe(false);
  expect(run.hashes.find(h => h.payload_scope === "received_result")!.value).toBe(await canonicalSha256HexCheckedV1(result));
  expect(await verifyAnalysisRunRecord(record)).toBe("match");
  await expect(validateAnalysisRunV03(record, result, modelLoadBasisRefs(model))).resolves.toBeUndefined();
}
function expectViewable(result: MechanicsResult, analysisRun: AnalysisRunEnvelope, model: PreviewModel) {
  const dispatch = readerDispatch(result, model);
  expect(dispatch.route).toBe("pressure");
  const standing = numericalResultStanding(result, model);
  expect({ eligible: standing.eligible, findings: standing.findings }).toStrictEqual({ eligible: dispatch.standing.eligible, findings: [...dispatch.standing.findings] });
  render(<ResultsPanel result={result} knowledge={null} analysisRun={analysisRun} selectedResultId={null} onSelectResult={vi.fn()} />);
  expect(screen.getByTestId("numerical-result-standing")).toHaveTextContent(`Numerical integrity: ${result.numerical_quality!.status}.`);
  expect(screen.getByTestId("numerical-result-standing")).not.toHaveTextContent("Unsupported numerical contract");
  expect(screen.getByTestId(`result-row-${result.results[0].id}`)).toBeInTheDocument();
  cleanup();
  render(<KnownSemanticNotices result={result} testIdPrefix="results" />);
  cleanup();
}

describe("an exact (pressure-1) session solve", () => {
  it.each(["sparse_interactive", "dense_scrutiny"] as Mode[])("%s: completes, and the session holds and shows the result and its Python-shaped record, never Current", async mode => {
    const { model, bytes } = exactPair(mode);
    const { result: hook } = await session(model, bytes, mode);
    const state = held(hook.current);
    expect(hook.current.results.solveJob.error_message ?? null).toBeNull();
    await expectPythonShapedRecord(state.analysisRun, state.result, state.model);
    expect(Object.hasOwn(state.result.contract_evidence as object, "load_reference_states")).toBe(false);
    expectViewable(state.result, state.analysisRun, state.model);
  });

  it.each(["sparse_interactive", "dense_scrutiny"] as Mode[])("%s with the load/reference-state group: completes, its record matches Python's carrier, and the resolved-state block shows it", async mode => {
    const { model, bytes } = loadStatePair(mode);
    const { result: hook } = await session(model, bytes, mode);
    const state = held(hook.current);
    await expectPythonShapedRecord(state.analysisRun, state.result, state.model);
    const python = json(`fixtures/results/pressure_load_state_connected_${mode === "sparse_interactive" ? "sparse" : "dense"}.analysis_run.json`) as AnalysisRunEnvelope;
    expect(state.analysisRun.analysis_run.result_refs.map(({ provenance: _, ...ref }) => ref)).toStrictEqual(python.analysis_run.result_refs.map(({ provenance: _, ...ref }) => ref));
    expect(state.analysisRun.analysis_run.hashes.find(h => h.payload_scope === "received_result")).toStrictEqual(python.analysis_run.hashes.find(h => h.payload_scope === "received_result"));
    expectViewable(state.result, state.analysisRun, state.model);
    const { solveJob, solverMode, running } = hook.current.results;
    render(<SolvePanel analysisRun={state.analysisRun} model={state.model} result={state.result} running={running} solveJob={solveJob}
      solverMode={solverMode} onCancel={() => {}} onRun={() => {}} onSolverModeChange={() => {}} />);
    expect(screen.getByTestId("load-reference-states-route")).toHaveTextContent("Route: ordinary pressure-1");
    expect(screen.queryByTestId("load-reference-states-refused")).toBeNull();
  });
});

describe("a saved exact session", () => {
  it("keeps the pressure-1 result and its record byte for byte and reopens them as history, never Current", async () => {
    const { model, bytes } = exactPair("sparse_interactive");
    const { result: hook } = await session(model, bytes, "sparse_interactive");
    const { result, analysisRun } = held(hook.current);
    const original = JSON.stringify(result), originalAnalysis = JSON.stringify(analysisRun);
    // The browser persistence adapter (no native runtime) stores the session's run.
    delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
    await act(async () => { await hook.current.project.handleCreateProject(); });
    const saved = await openLocalProject();
    expect(JSON.stringify(saved!.mechanics_result)).toBe(original);
    expect(JSON.stringify(saved!.analysis_run)).toBe(originalAnalysis);
    await act(async () => { await hook.current.project.handleOpenProject(); });
    expect(hook.current.results.historicalRun!.designation).toBe("historical_saved_run");
    expect(JSON.stringify(hook.current.results.historicalRun!.rawMechanicsResult)).toBe(original);
    expect(hook.current.results.currentSolvedResult).toBeNull();
  });
});

describe("every output surface still refuses the held pressure-1 result by name", () => {
  it("names PRESSURE_1_OUTPUT_REFUSAL on each surface, in each builder, and in the gated panels", async () => {
    const { model, bytes } = exactPair("sparse_interactive");
    const { result: hook } = await session(model, bytes, "sparse_interactive");
    const { result, analysisRun, inputManifest } = held(hook.current);
    for (const surface of OUTPUT_SURFACES) {
      expect(surfaceRouteRefusal(result, surface), surface).toBe(PRESSURE_1_OUTPUT_REFUSAL);
      expect(surfaceOutputRefusal(result, model, surface), surface).toBe(PRESSURE_1_OUTPUT_REFUSAL);
    }
    expect(loadReferenceOutputRefusal(result)).toBe(PRESSURE_1_OUTPUT_REFUSAL);
    // Result export, stress-neutral and report package, given the session's own record and manifest.
    await expect(buildCurrentResultExport({ model, result, analysisRun, inputManifest })).rejects.toThrow(PRESSURE_1_OUTPUT_REFUSAL);
    await expect(buildStressNeutralExportPacket({ model, result, analysisRun })).rejects.toThrow(PRESSURE_1_OUTPUT_REFUSAL);
    expect(reportPackageUnavailableReason(result)).toBe(PRESSURE_1_OUTPUT_REFUSAL);
    // The session itself offers no report package for a result that is not Current.
    await act(async () => { await hook.current.results.handleSaveReportPackage(); });
    expect(hook.current.results.reportPackageRoute).toBeNull();
    // Native package: the export link is replaced by the refusal.
    render(<NativePackagePanel model={model} modelHash={hook.current.model.modelHash} result={result} analysisRun={analysisRun} editorIntents={[]}
      projectSummary={null} proposal={null} selectedReviewTarget={null} storageCapability={null} />);
    expect(screen.getByTestId("native-package-load-reference-output-unavailable")).toHaveTextContent(PRESSURE_1_OUTPUT_REFUSAL);
    expect(screen.queryByTestId("native-package-link")).toBeNull();
    cleanup();
    // Rule check: refused before the rule backend (pressure-1 is outside the fresh set).
    invokeMock.mockClear();
    await expect(runRuleChecks({ rulePackDocument: { metadata: { rule_pack_id: "invented-test-only" } }, model, solvedEnvelope: result })).rejects.toThrow("RULE_SOURCE_IDENTITY_NOT_FRESH");
    expect(invokeMock.mock.calls.map(([command]) => command)).not.toContain("run_rule_checks");
    expect(JSON.stringify(result)).toBe(JSON.stringify(hook.current.results.result));
    expect((analysisRun as Json).analysis_run.reproducibility.semantic_contract).toStrictEqual(PRESSURE_1);
  });
});
