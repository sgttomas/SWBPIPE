/** 0.4.0 session standing: Current, its invalidation by a load/reference-state
 * edit, and rule binding (T1 WP2). Unit transport replay of committed producer
 * bytes through mocked IPC; NOT a native UI qualification witness. */
import { act, renderHook } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { runPreviewMechanics, buildAnalysisRunPreview, hasNativeMechanicsInvocation } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { runRuleChecks } from "../../services/ruleCheckService";
import { numericalResultStanding } from "../results/numericalResultQuality";
import { LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE } from "../results/loadReferenceSourceEvidence";
import { useResultsSessionState } from "./resultsSessionState";
import type { MechanicsResult, PreviewModel } from "../../types";

afterEach(() => { invokeMock.mockReset(); delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__; });
const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
type Mode = "sparse_interactive" | "dense_scrutiny";

async function solve(rawPath: string, requestPath: string, mode: Mode) {
  const request = json(requestPath), original = json(rawPath) as MechanicsResult;
  // The committed request model plus the case metadata the desktop session
  // requires (invented labels only; the product reads no physics from them).
  const model = structuredClone(request.model) as PreviewModel;
  for (const loadCase of model.load_cases) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(async (command: string, args: { model: unknown; solverMode: string }) => {
    if (command !== "run_preview_mechanics_with_solver_mode") throw new Error(`unexpected ${command}`);
    expect(args.solverMode).toBe(mode);
    expect(args.model).toEqual(model);
    return structuredClone(original);
  });
  const source = await runPreviewMechanics(model, mode);
  const manifestFor = (m: PreviewModel) => buildCurrentSessionInputManifest({ model: m, solver: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "test:load-reference-unit-transport-replay", solver_mode: mode, settings: {} }, active_rule_packs: [], external_assets: [] });
  const manifest = await manifestFor(model);
  const analysis = await buildAnalysisRunPreview(source, { inputManifest: manifest });
  return { source, model, manifest, analysis, manifestFor };
}

describe("load-reference-1 Current and its invalidation by a 0.4.0 edit", () => {
  it.each(["sparse_interactive", "dense_scrutiny"] as Mode[])("%s: Current only for the actual model; a reference_configurations or analysis_state edit is not Current", async mode => {
    const { source, model, manifest, analysis, manifestFor } = await solve(`fixtures/product_preview/load_reference/connected-${mode}.raw.json`, "fixtures/product_preview/load_reference/connected.request.json", mode);
    expect(hasNativeMechanicsInvocation(source, model, mode)).toBe(true);
    const { result } = renderHook(() => useResultsSessionState());
    act(() => { result.current.setResult(source); result.current.setInputManifest(manifest); result.current.setAnalysisRun(analysis); });
    expect(result.current.currentSolvedResult).toBe(source);

    const reference = structuredClone(model);
    const fit = reference.reference_configurations![0].member_references[1].fit as { kind: string; length_change: { value: number; unit: string } };
    fit.length_change.value = -2;
    expect(hasNativeMechanicsInvocation(source, reference, mode)).toBe(false);
    const referenceManifest = await manifestFor(reference);
    act(() => result.current.setInputManifest(referenceManifest));
    expect(result.current.currentSolvedResult).toBeNull();

    const state = structuredClone(model);
    state.load_cases[0].analysis_state!.support_states[0].boundary_motion![0].value.value = 0.75;
    expect(hasNativeMechanicsInvocation(source, state, mode)).toBe(false);
    const stateManifest = await manifestFor(state);
    act(() => result.current.setInputManifest(stateManifest));
    expect(result.current.currentSolvedResult).toBeNull();

    act(() => result.current.setInputManifest(manifest));
    expect(result.current.currentSolvedResult).toBe(source);
    // A copy of the same bytes has no live registration.
    act(() => result.current.setResult(structuredClone(source)));
    expect(result.current.currentSolvedResult).toBeNull();
  });

  it("a load-reference-1 source that its reader refuses is never registered for Current", async () => {
    const request = json("fixtures/product_preview/load_reference/connected.request.json");
    const model = structuredClone(request.model) as PreviewModel;
    for (const loadCase of model.load_cases) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
    const tampered = json("fixtures/product_preview/load_reference/connected-sparse_interactive.raw.json");
    tampered.contract_evidence.load_reference_states[0].members[0].fit_strain = 0.5; // invented tamper
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(async () => structuredClone(tampered));
    const source = await runPreviewMechanics(model, "sparse_interactive");
    expect(source).toEqual(tampered); // preserved for inspection
    expect(hasNativeMechanicsInvocation(source, model, "sparse_interactive")).toBe(false);
  });

  it("a sensitive ordinary load-reference-1 result is readable and never Current", async () => {
    const { source, manifest, analysis } = await solve("core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-sparse_interactive.raw.json", "core/reporting/result_export/tests/fixtures/load_reference_fallback_uz.request.json", "sparse_interactive");
    const { result } = renderHook(() => useResultsSessionState());
    act(() => { result.current.setResult(source); result.current.setInputManifest(manifest); result.current.setAnalysisRun(analysis); });
    expect(result.current.result).toBe(source);
    expect(result.current.currentSolvedResult).toBeNull();
  });
});

describe("rule binding on the reachable desktop path", () => {
  it("an ordinary checks_passed load-reference-1 result binds rules exactly as physics-1 does", async () => {
    const { source, model, manifest, analysis } = await solve("fixtures/product_preview/load_reference/pressure-sparse_interactive.raw.json", "fixtures/product_preview/load_reference/pressure.request.json", "sparse_interactive");
    const { result } = renderHook(() => useResultsSessionState());
    act(() => { result.current.setResult(source); result.current.setInputManifest(manifest); result.current.setAnalysisRun(analysis); });
    expect(result.current.currentSolvedResult).toBe(source); // RuleCheckRunPanel receives it
    invokeMock.mockReset();
    invokeMock.mockResolvedValue({ status: "RULE_INPUTS_INCOMPLETE" });
    await runRuleChecks({ rulePackDocument: { metadata: { rule_pack_id: "invented-test-only" } } as never, model, solvedEnvelope: source });
    expect(invokeMock).toHaveBeenCalledTimes(1);
    const [command, args] = invokeMock.mock.calls[0];
    expect(command).toBe("run_rule_checks");
    // The physics-1 argument shape: envelope and model, no source-block invocation.
    expect(Object.keys(args).sort()).toEqual(["model", "rulePackDocument", "solvedEnvelope"]);
    expect(args.solvedEnvelope).toBe(source);
  });

  it.each(["n05", "mixed"])("a registered joined %s result is refused through standing and never reaches the rule panel", async name => {
    const { source, model, manifest, analysis } = await solve(`fixtures/product_preview/load_reference_source/${name}-sparse_interactive.raw.json`, `fixtures/product_preview/load_reference_source/${name}.request.json`, "sparse_interactive");
    expect(hasNativeMechanicsInvocation(source, model, "sparse_interactive")).toBe(true);
    expect(numericalResultStanding(source, model)).toEqual({ contract: "load_reference_source", status: "needs_recompute", eligible: false, findings: [LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE] });
    // The joined AnalysisRun retains both namespaces, like physics-source-1.
    expect(analysis.analysis_run.contract_evidence).toEqual(source.contract_evidence);
    expect(analysis.analysis_run.source_block_recovery).toEqual(source.source_block_recovery);
    const { result } = renderHook(() => useResultsSessionState());
    act(() => { result.current.setResult(source); result.current.setInputManifest(manifest); result.current.setAnalysisRun(analysis); });
    expect(result.current.result).toBe(source); // readable
    expect(result.current.currentSolvedResult).toBeNull(); // RuleCheckRunPanel receives null
  });
});
