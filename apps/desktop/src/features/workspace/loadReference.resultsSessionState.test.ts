/** Load/reference-state session standing (T1 WP2), on the current exact identities
 * (B7b: pressure-1 ordinary and pressure-source-1 joined, the v3 twins of the retired
 * load-reference-1 and load-reference-source-1): native registration and its
 * invalidation by a load/reference-state edit, standing, and the rule-check boundary.
 * Neither identity is ever Current on the desktop: neither is in its fresh set and
 * every desktop output of either is refused. Unit transport replay of committed
 * producer bytes through mocked IPC; NOT a native UI qualification witness. */
import { act, renderHook } from "@testing-library/react";
import { createSolveJobTransport, solveBytes } from "../../test/nativeMechanicsReplay";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { runPreviewMechanics, buildAnalysisRunPreview, hasNativeMechanicsInvocation } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { runRuleChecks } from "../../services/ruleCheckService";
import { numericalResultStanding } from "../results/resultStanding";
import { LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE } from "../results/resultStanding";
import { useResultsSessionState } from "./resultsSessionState";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel } from "../../types";

afterEach(() => { invokeMock.mockReset(); delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__; });
const root = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8"));
type Mode = "sparse_interactive" | "dense_scrutiny";

async function solve(rawPath: string, requestPath: string, mode: Mode) {
  const request = json(requestPath), originalText = readFileSync(resolve(root, rawPath), "utf8");
  // The committed request model plus the case metadata the desktop session
  // requires where the request lacks it (invented labels only; the product reads no
  // physics from them). A request that carries them is sent unchanged: the joined
  // source's receipt binds the model it was solved for.
  const model = structuredClone(request.model) as PreviewModel;
  for (const loadCase of model.load_cases) if (loadCase.label === undefined) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(createSolveJobTransport((sent) => {
    expect(sent.mode).toBe(mode);
    expect(sent.model).toEqual(model);
    return originalText; // the producer's bytes, as captured
  }).invoke);
  const source = await runPreviewMechanics(model, mode);
  const manifestFor = (m: PreviewModel) => buildCurrentSessionInputManifest({ model: m, solver: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "test:load-reference-unit-transport-replay", solver_mode: mode, settings: {} }, active_rule_packs: [], external_assets: [] });
  const manifest = await manifestFor(model);
  return { source, model, manifest, manifestFor };
}
const ORDINARY = "fixtures/product_preview/pressure/load_reference";
const JOINED = "fixtures/product_preview/pressure_source/load_reference_source";

describe("ordinary pressure-1 load/reference-state registration and its invalidation by a load/reference-state edit", () => {
  it.each(["sparse_interactive", "dense_scrutiny"] as Mode[])("%s: registered for the actual model only, readable, eligible standing, recorded as Python records it, and never Current", async mode => {
    const { source, model, manifest } = await solve(`${ORDINARY}/connected-${mode}.raw.json`, `${ORDINARY}/connected.request.json`, mode);
    expect(hasNativeMechanicsInvocation(source, model, mode)).toBe(true);
    expect(numericalResultStanding(source, model)).toMatchObject({ contract: "pressure", eligible: true, findings: [] });
    // B7b: the session records it (the record binds pressure-1's table, as Python's carrier
    // does); the desktop session never makes it Current.
    const analysis = await buildAnalysisRunPreview(source, { inputManifest: manifest });
    const python = json(`fixtures/results/pressure_load_state_connected_${mode === "sparse_interactive" ? "sparse" : "dense"}.analysis_run.json`) as AnalysisRunEnvelope;
    expect(analysis.analysis_run.reproducibility.semantic_contract).toStrictEqual(python.analysis_run.reproducibility.semantic_contract);
    expect(analysis.analysis_run.result_refs.map(r => r.semantic_contract)).toStrictEqual(python.analysis_run.result_refs.map(r => r.semantic_contract));
    const { result } = renderHook(() => useResultsSessionState());
    act(() => { result.current.setResult(source); result.current.setInputManifest(manifest); result.current.setAnalysisRun(analysis); });
    expect(result.current.analysisRun).toBe(analysis);
    expect(result.current.result).toBe(source);
    expect(result.current.currentSolvedResult).toBeNull();

    const reference = structuredClone(model);
    const fit = reference.reference_configurations![0].member_references[1].fit as { kind: string; length_change: { value: number; unit: string } };
    fit.length_change.value = -2;
    expect(hasNativeMechanicsInvocation(source, reference, mode)).toBe(false);

    const state = structuredClone(model);
    state.load_cases[0].analysis_state!.support_states[0].boundary_motion![0].value.value = 0.75;
    expect(hasNativeMechanicsInvocation(source, state, mode)).toBe(false);
    // A copy of the same bytes has no live registration.
    expect(hasNativeMechanicsInvocation(structuredClone(source), model, mode)).toBe(false);
  });

  it("a pressure-1 source that its reader refuses is never registered", async () => {
    const request = json(`${ORDINARY}/connected.request.json`);
    const model = structuredClone(request.model) as PreviewModel;
    for (const loadCase of model.load_cases) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
    const tampered = json(`${ORDINARY}/connected-sparse_interactive.raw.json`);
    tampered.contract_evidence.load_reference_states[0].members[0].fit_strain = 0.5; // invented tamper
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(createSolveJobTransport(() => solveBytes(tampered)).invoke);
    const source = await runPreviewMechanics(model, "sparse_interactive");
    expect(source).toEqual(tampered); // preserved for inspection
    expect(hasNativeMechanicsInvocation(source, model, "sparse_interactive")).toBe(false);
  });

  it("a sensitive ordinary pressure-1 load/reference-state result is readable and never Current", async () => {
    const { source, model, manifest } = await solve("core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-sparse_interactive.raw.json", "core/reporting/result_export/tests/fixtures/load_reference_fallback_uz.request.json", "sparse_interactive");
    expect(numericalResultStanding(source, model).findings).toContain("NUMERICAL_INTEGRITY_NOT_QUALIFIED");
    const { result } = renderHook(() => useResultsSessionState());
    act(() => { result.current.setResult(source); result.current.setInputManifest(manifest); });
    expect(result.current.result).toBe(source);
    expect(result.current.currentSolvedResult).toBeNull();
  });
});

describe("rule binding on the reachable desktop path", () => {
  it("an ordinary checks_passed pressure-1 load/reference-state result never reaches the rule backend", async () => {
    const { source, model } = await solve(`${ORDINARY}/pressure-sparse_interactive.raw.json`, `${ORDINARY}/pressure.request.json`, "sparse_interactive");
    expect(hasNativeMechanicsInvocation(source, model, "sparse_interactive")).toBe(true);
    invokeMock.mockReset();
    invokeMock.mockResolvedValue({ status: "RULE_INPUTS_INCOMPLETE" });
    await expect(runRuleChecks({ rulePackDocument: { metadata: { rule_pack_id: "invented-test-only" } } as never, model, solvedEnvelope: source })).rejects.toThrow("RULE_SOURCE_IDENTITY_NOT_FRESH");
    expect(invokeMock).not.toHaveBeenCalled();
  });

  it.each(["n05", "mixed"])("a registered joined %s result is refused through standing and never reaches the rule panel", async name => {
    const { source, model, manifest } = await solve(`${JOINED}/${name}-sparse_interactive.raw.json`, `${JOINED}/${name}.request.json`, "sparse_interactive");
    expect(hasNativeMechanicsInvocation(source, model, "sparse_interactive")).toBe(true);
    const standing = numericalResultStanding(source, model);
    expect(standing).toMatchObject({ contract: "pressure_source", status: "needs_recompute", eligible: false });
    expect(standing.findings).toContain(LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE);
    // The joined AnalysisRun retains both namespaces (the source record).
    const analysis = await buildAnalysisRunPreview(source, { inputManifest: manifest });
    expect(analysis.analysis_run.contract_evidence).toEqual(source.contract_evidence);
    expect(analysis.analysis_run.source_block_recovery).toEqual(source.source_block_recovery);
    const { result } = renderHook(() => useResultsSessionState());
    act(() => { result.current.setResult(source); result.current.setInputManifest(manifest); result.current.setAnalysisRun(analysis); });
    expect(result.current.result).toBe(source); // readable
    expect(result.current.currentSolvedResult).toBeNull(); // RuleCheckRunPanel receives null
  });
});
