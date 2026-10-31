/** B7b (DEL-14-02): an exact session solve on the desktop. The exact profile's producer
 * publishes pressure-2 (3.0.0/exact_pressure_v3), with or without the
 * load/reference-state group. The session solve completes: the desktop records it as
 * Python's `build_analysis_run_v0_3` does, holds the result and its record, and shows it
 * (results panel, standing, notices, the resolved-state block) through Rust's verdict.
 * Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §4.1, §4.4, §4.5; owner rulings
 * 10A and 11B, 2026-10-21): with Rust's eligible standing, its matching 0.3.0 record and
 * manifest and the live native invocation it is Current; the result JSON and
 * stress-neutral exports and the rule check admit it, and every other output surface
 * refuses it by name (the report package as a fresh result). A rule status recorded on
 * it saves, and reopens as historical evidence, never Current.
 * Unit transport replay of committed producer bytes through mocked IPC; NOT a native UI
 * qualification witness. The rule backend's answer is a unit simulation, NOT rule-evaluation
 * evidence. All data are invented. */
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
import { numericalResultStanding, SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED } from "../results/resultStanding";
import { PRESSURE_SOURCE_CONTRACT_ID } from "../results/numericalResultQuality";
import { PRESSURE_SOURCE_CONTRACT_SHA256 } from "../../test-support/contractBindings";
import { OUTPUT_SURFACES, PRESSURE_2_OUTPUT_REFUSAL, surfaceOutputRefusal, surfaceRouteRefusal } from "../results/outputPolicy";
import { N_LOAD_REFERENCE_RULE_CHECK } from "../results/loadReferenceDisclosure";
import { loadReferenceOutputRefusal } from "../results/loadReferenceOutputAvailability";
import { ResultsPanel } from "../results/ResultsPanel";
import { KnownSemanticNotices } from "../results/KnownSemanticNotices";
import { SolvePanel } from "../solve/SolvePanel";
import { buildCurrentResultExport, resultSchemaVersion } from "../result-export/resultExportAdapter";
import { buildStressNeutralExportPacket } from "../stress-neutral/StressNeutralExportPanel";
import { reportPackageUnavailableReason } from "../report/reportPackageRequest";
import { NativePackagePanel } from "../native-package/NativePackagePanel";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Mode = "sparse_interactive" | "dense_scrutiny";
const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
const PRESSURE_1 = { id: "openpipestress.result_semantics/0.3.0/pressure-2", sha256: "5451fbf3e10e085c90b523baa7daf5fc723659e0d874d988d23807be7d2fefe6" };
/** The three surfaces the owner opened (ruling 10A). */
/** Epoch 8's three opened surfaces and W-11's two (owner, 2026-10-26). */
const OPENED: readonly string[] = ["result-export", "stress-neutral", "rule-check", "rendered-report", "report-package"];
/** The mocked rule backend's answer (unit simulation; invented). */
const RULE_ANSWER = {
  document_kind: "openpipestress.rule_check.run",
  rule_pack_id: "invented-test-only",
  grammar_version: "1.0.0",
  aggregate_status: "USER_RULE_CHECKED",
  checks: [],
  professional_boundary_notice: "notice"
};
const RULE_PACK = { metadata: { rule_pack_id: "invented-test-only" } };

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
    if (command === "run_rule_checks") return structuredClone(RULE_ANSWER);
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
/** The exact profile's replay pair (pressure-2 without the load/reference-state group). */
function exactPair(mode: Mode) {
  const pair = nativeMechanicsReplayPair(mode, { profile: "pressure" });
  return { model: pair.model, bytes: pair.bytes };
}
/** A committed load/reference-state pressure-2 request and its producer bytes, with the
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
/** What the session holds after the solve: the pressure-2 result, its record and manifest,
 * Current (epoch 8: eligible standing, matching record and manifest, live invocation). */
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
  expect(numericalResultStanding(result!, model)).toMatchObject({ contract: "pressure", eligible: true, findings: [] });
  expect(currentSolvedResult).toBe(result);
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

describe("an exact (pressure-2) session solve", () => {
  it.each(["sparse_interactive", "dense_scrutiny"] as Mode[])("%s: completes, and the session holds and shows the result and its Python-shaped record, Current (epoch 8)", async mode => {
    const { model, bytes } = exactPair(mode);
    const { result: hook } = await session(model, bytes, mode);
    const state = held(hook.current);
    expect(hook.current.results.solveJob.error_message ?? null).toBeNull();
    await expectPythonShapedRecord(state.analysisRun, state.result, state.model);
    expect(Object.hasOwn(state.result.contract_evidence as object, "load_reference_states")).toBe(false);
    expectViewable(state.result, state.analysisRun, state.model);
  });

  it.each(["sparse_interactive", "dense_scrutiny"] as Mode[])("%s with the load/reference-state group: completes, its record matches Python's carrier, the resolved-state block shows it, and it is Current (epoch 8)", async mode => {
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
    expect(screen.getByTestId("load-reference-states-route")).toHaveTextContent("Route: ordinary pressure-2");
    expect(screen.queryByTestId("load-reference-states-refused")).toBeNull();
  });
});

/** The disclosure line from the published records, in published order, without duplicates. */
function expectedRuleCheckDisclosure(result: MechanicsResult): string {
  const records = (result.contract_evidence as Json).load_reference_states as { reference_configuration_id: string; load_case_id: string; history: { kind: string } }[];
  const distinct = (values: string[]) => [...new Set(values)].join(", ");
  return N_LOAD_REFERENCE_RULE_CHECK
    .replace("{configurations}", distinct(records.map(r => r.reference_configuration_id)))
    .replace("{cases}", distinct(records.map(r => r.load_case_id)))
    .replace("{histories}", distinct(records.map(r => r.history.kind)));
}
const SAVED_RUNS = [
  ["pressure-2", () => exactPair("sparse_interactive"), false],
  ["pressure-2 with the load/reference-state group", () => loadStatePair("sparse_interactive"), true],
] as const;

describe("a saved exact session", () => {
  it("keeps the pressure-2 result and its record byte for byte and reopens them as history, never Current", async () => {
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

  it.each(SAVED_RUNS)("%s: a run with a rule status saves, reopens as historical evidence, validates against its saved source, and is never Current after reopening (epoch 8, §4.4)", async (_name, pair, loadReference) => {
    const { model, bytes } = pair();
    const { result: hook } = await session(model, bytes, "sparse_interactive");
    const { result, analysisRun: solved } = held(hook.current);
    // The rule check on the Current result reaches the (mocked) backend.
    invokeMock.mockClear();
    const route = await runRuleChecks({ rulePackDocument: RULE_PACK, model: hook.current.model.model!, solvedEnvelope: hook.current.results.currentSolvedResult });
    expect(route).toStrictEqual({ route: "tauri_backend", result: RULE_ANSWER, loadReferenceDisclosure: loadReference ? expectedRuleCheckDisclosure(result) : null });
    expect(invokeMock.mock.calls.map(([command]) => command)).toEqual(["run_rule_checks"]);
    // The session records the aggregate in the run's record, as the run panel lifts it.
    const { projectSessionGeneration, uiModelRevision } = hook.current.model;
    await act(async () => {
      await hook.current.results.handleRuleCheckAggregate(route.route === "tauri_backend" ? route.result.aggregate_status : null, {
        projectSessionGeneration, modelRevision: uiModelRevision,
        result: hook.current.results.currentSolvedResult, inputManifest: hook.current.results.inputManifest
      });
    });
    await waitFor(() => expect(hook.current.results.analysisRun).not.toBe(solved));
    const ruled = hook.current.results.analysisRun!;
    expect(hook.current.results.ruleCheckAggregate).toBe("USER_RULE_CHECKED");
    expect(ruled.analysis_run.analysis_status).toContain("USER_RULE_CHECKED");
    expect(ruled.analysis_run.analysis_status).not.toContain("RULE_INPUTS_INCOMPLETE");
    expect(hook.current.results.currentSolvedResult).toBe(result);
    await expectPythonShapedRecord(ruled, result, model);
    const original = JSON.stringify(result), originalAnalysis = JSON.stringify(ruled);

    // Save (the browser persistence adapter, no native runtime).
    delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
    await act(async () => { await hook.current.project.handleCreateProject(); });
    const saved = await openLocalProject();
    expect(JSON.stringify(saved!.mechanics_result)).toBe(original);
    expect(JSON.stringify(saved!.analysis_run)).toBe(originalAnalysis);
    // The saved record validates against its saved source.
    const savedRecord = saved!.analysis_run as AnalysisRunEnvelope, savedSource = saved!.mechanics_result as MechanicsResult;
    expect(savedRecord.analysis_run.analysis_status).toContain("USER_RULE_CHECKED");
    expect(await verifyAnalysisRunRecord(savedRecord)).toBe("match");
    await expect(validateAnalysisRunV03(savedRecord, savedSource, modelLoadBasisRefs(saved!.model as PreviewModel))).resolves.toBeUndefined();

    // Reopen: historical evidence, never Current.
    await act(async () => { await hook.current.project.handleOpenProject(); });
    const historical = hook.current.results.historicalRun!;
    expect(historical.designation).toBe("historical_saved_run");
    expect(JSON.stringify(historical.rawMechanicsResult)).toBe(original);
    expect(JSON.stringify(historical.rawAnalysisRun)).toBe(originalAnalysis);
    expect(historical.analysisRun!.analysis_run.analysis_status).toContain("USER_RULE_CHECKED");
    // Its only finding is the historical one: the saved evidence validates.
    expect(historical.findings).toEqual(["HISTORICAL_INPUT_MANIFEST_MISSING"]);
    expect(hook.current.results.currentSolvedResult).toBeNull();
    // A reopened result has no live invocation, so a rule check over it stops before the backend.
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockClear();
    const reopened = (hook.current.results.result ?? historical.mechanicsResult)!;
    await expect(runRuleChecks({ rulePackDocument: RULE_PACK, model: hook.current.model.model!, solvedEnvelope: reopened })).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    expect(invokeMock.mock.calls.map(([command]) => command)).not.toContain("run_rule_checks");
  });
});

describe("a saved exact session: pressure-source-1 (epoch 8, §4.4)", () => {
  it("n05: a registered, eligible run with a rule status saves byte for byte, validates against its saved source, reopens as historical evidence, and is never Current after reopening", async () => {
    // The committed request is a complete desktop document and is sent unchanged: the
    // joined receipt binds the model it was solved for.
    const folder = "fixtures/product_preview/pressure_source/physics_source";
    const model = structuredClone(json(`${folder}/n05.request.json`).model) as PreviewModel;
    const bytes = readFileSync(resolve(root, `${folder}/n05-sparse_interactive.raw.json`), "utf8");
    const { result: hook } = await session(model, bytes, "sparse_interactive");
    const { result, analysisRun: solved, inputManifest, currentSolvedResult } = hook.current.results;
    const held = hook.current.model.model!;
    expect(sourceContract(result!)).toBe("pressure_source");
    expect(result!.producer!.semantic_contract_id).toBe(PRESSURE_SOURCE_CONTRACT_ID);
    expect(hasNativeMechanicsInvocation(result!, held, "sparse_interactive")).toBe(true);
    expect(numericalResultStanding(result!, held)).toMatchObject({ contract: "pressure_source", eligible: true, findings: [] });
    expect(inputManifest).not.toBeNull();
    expect(currentSolvedResult).toBe(result);

    // The rule check on the Current result reaches the (mocked) backend with its registered invocation.
    invokeMock.mockClear();
    const route = await runRuleChecks({ rulePackDocument: RULE_PACK, model: held, solvedEnvelope: currentSolvedResult });
    expect(route).toStrictEqual({ route: "tauri_backend", result: RULE_ANSWER, loadReferenceDisclosure: null });
    expect(invokeMock.mock.calls.map(([command]) => command)).toEqual(["run_rule_checks"]);
    expect((invokeMock.mock.calls[0][1] as Json).sourceBlockInvocation).toEqual({ request: { model: held, materials: [] }, solver_mode: "sparse_interactive" });
    const { projectSessionGeneration, uiModelRevision } = hook.current.model;
    await act(async () => {
      await hook.current.results.handleRuleCheckAggregate(route.route === "tauri_backend" ? route.result.aggregate_status : null, {
        projectSessionGeneration, modelRevision: uiModelRevision,
        result: hook.current.results.currentSolvedResult, inputManifest: hook.current.results.inputManifest
      });
    });
    await waitFor(() => expect(hook.current.results.analysisRun).not.toBe(solved));
    const ruled = hook.current.results.analysisRun!;
    expect(ruled.analysis_run.analysis_status).toContain("USER_RULE_CHECKED");
    expect(hook.current.results.currentSolvedResult).toBe(result);
    // The record binds pressure-source-1's table and copies both source namespaces.
    expect(ruled.analysis_run.reproducibility.semantic_contract).toStrictEqual({ id: PRESSURE_SOURCE_CONTRACT_ID, sha256: PRESSURE_SOURCE_CONTRACT_SHA256 });
    expect(ruled.analysis_run.contract_evidence).toEqual(result!.contract_evidence);
    expect(ruled.analysis_run.source_block_recovery).toEqual(result!.source_block_recovery);
    expect(await verifyAnalysisRunRecord(ruled)).toBe("match");
    await expect(validateAnalysisRunV03(ruled, result!, modelLoadBasisRefs(held))).resolves.toBeUndefined();
    const original = JSON.stringify(result), originalAnalysis = JSON.stringify(ruled);

    // Save (the browser persistence adapter, no native runtime): byte for byte.
    delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
    await act(async () => { await hook.current.project.handleCreateProject(); });
    const saved = await openLocalProject();
    expect(JSON.stringify(saved!.mechanics_result)).toBe(original);
    expect(JSON.stringify(saved!.analysis_run)).toBe(originalAnalysis);
    // The saved record verifies and validates against its saved source.
    const savedRecord = saved!.analysis_run as AnalysisRunEnvelope, savedSource = saved!.mechanics_result as MechanicsResult;
    expect(savedRecord.analysis_run.analysis_status).toContain("USER_RULE_CHECKED");
    expect(await verifyAnalysisRunRecord(savedRecord)).toBe("match");
    await expect(validateAnalysisRunV03(savedRecord, savedSource, modelLoadBasisRefs(saved!.model as PreviewModel))).resolves.toBeUndefined();

    // Reopen: historical evidence, never Current. The saved source has no registration, so
    // its standing carries Rust's finding beside the historical one.
    await act(async () => { await hook.current.project.handleOpenProject(); });
    const historical = hook.current.results.historicalRun!;
    expect(historical.designation).toBe("historical_saved_run");
    expect(JSON.stringify(historical.rawMechanicsResult)).toBe(original);
    expect(JSON.stringify(historical.rawAnalysisRun)).toBe(originalAnalysis);
    expect(historical.analysisRun!.analysis_run.analysis_status).toContain("USER_RULE_CHECKED");
    expect(historical.findings).toEqual(["HISTORICAL_INPUT_MANIFEST_MISSING", SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED]);
    expect(hook.current.results.currentSolvedResult).toBeNull();
    // A rule run over the reopened result stops before the backend.
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockClear();
    const reopened = (hook.current.results.result ?? historical.mechanicsResult)!;
    await expect(runRuleChecks({ rulePackDocument: RULE_PACK, model: hook.current.model.model!, solvedEnvelope: reopened })).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    expect(invokeMock.mock.calls.map(([command]) => command)).not.toContain("run_rule_checks");
  });
});

describe("the held pressure-2 result: admitted on the opened surfaces, refused by name on every other (epoch 8)", () => {
  it("adds no policy refusal on the result JSON and stress-neutral exports and the rule check, and names PRESSURE_2_OUTPUT_REFUSAL elsewhere", async () => {
    const { model, bytes } = exactPair("sparse_interactive");
    const { result: hook } = await session(model, bytes, "sparse_interactive");
    const { result, analysisRun, inputManifest } = held(hook.current);
    for (const surface of OUTPUT_SURFACES) {
      const expected = OPENED.includes(surface) ? null : PRESSURE_2_OUTPUT_REFUSAL;
      expect(surfaceRouteRefusal(result, surface), surface).toBe(expected);
      expect(surfaceOutputRefusal(result, model, surface), surface).toBe(expected);
    }
    // The route's shared reason is unchanged (§5.2); no product surface reads it since W-11.
    expect(loadReferenceOutputRefusal(result)).toBe(PRESSURE_2_OUTPUT_REFUSAL);
    // Result export and stress-neutral build from the session's own record and manifest.
    const document = await buildCurrentResultExport({ model, result, analysisRun, inputManifest });
    expect(() => resultSchemaVersion(document)).not.toThrow();
    await expect(buildStressNeutralExportPacket({ model, result, analysisRun })).resolves.toBeTruthy();
    // W-11 (DEL-08-01 Design `exact-route-report-sections.md` §5.1; owner, 2026-10-26): the
    // report package's cell admits it; through the session the save no longer stops at the
    // route refusal: the request is built and screened, and, without the package's own
    // local-private selection (not chosen here), the screen blocks it before any IPC.
    expect(reportPackageUnavailableReason(result)).toBeNull();
    await act(async () => { await hook.current.results.handleSaveReportPackage(); });
    const packageRoute = hook.current.results.reportPackageRoute as Json;
    expect(packageRoute.route).toBe("redaction_blocked");
    expect(packageRoute.diagnostic).toMatch(/^REPORT-PACKAGE-REDACTION-BLOCKED: \d+ finding\(s\) prevent package assembly or persistence\.$/);
    // Native package: the export link is replaced by the refusal.
    render(<NativePackagePanel model={model} modelHash={hook.current.model.modelHash} result={result} analysisRun={analysisRun} editorIntents={[]}
      projectSummary={null} proposal={null} selectedReviewTarget={null} storageCapability={null} />);
    expect(screen.getByTestId("native-package-load-reference-output-unavailable")).toHaveTextContent(PRESSURE_2_OUTPUT_REFUSAL);
    expect(screen.queryByTestId("native-package-link")).toBeNull();
    cleanup();
    // Rule check: the Current result reaches the (mocked) rule backend.
    invokeMock.mockClear();
    await expect(runRuleChecks({ rulePackDocument: RULE_PACK, model, solvedEnvelope: result })).resolves.toStrictEqual({ route: "tauri_backend", result: RULE_ANSWER, loadReferenceDisclosure: null });
    expect(invokeMock.mock.calls.map(([command]) => command)).toEqual(["run_rule_checks"]);
    expect(JSON.stringify(result)).toBe(JSON.stringify(hook.current.results.result));
    expect((analysisRun as Json).analysis_run.reproducibility.semantic_contract).toStrictEqual(PRESSURE_1);
  });

  it("a received copy of the held result is readable, never Current, and never reaches the rule backend", async () => {
    const { model, bytes } = exactPair("sparse_interactive");
    const { result: hook } = await session(model, bytes, "sparse_interactive");
    const { result, analysisRun, inputManifest } = held(hook.current);
    const copy = structuredClone(result);
    expect(hasNativeMechanicsInvocation(copy, model)).toBe(false);
    await expect(buildCurrentResultExport({ model, result: copy, analysisRun, inputManifest })).rejects.toThrow("CURRENT_NATIVE_INVOCATION_REQUIRED");
    invokeMock.mockClear();
    await expect(runRuleChecks({ rulePackDocument: RULE_PACK, model, solvedEnvelope: copy })).rejects.toThrow("RULE_NATIVE_INVOCATION_REQUIRED");
    expect(invokeMock.mock.calls.map(([command]) => command)).not.toContain("run_rule_checks");
  });
});
