/** Load/reference-state session standing (T1 WP2), on the current exact identities
 * (B7b: pressure-2 ordinary and pressure-source-1 joined, the v3 twins of the retired
 * load-reference-1 and load-reference-source-1): native registration and its
 * invalidation by a load/reference-state edit, standing, and the rule-check boundary.
 * Epoch 8 (owner ruling 11B, 2026-10-21; DEL-04-07 Design `desktop-exact-outputs.md`
 * §5.4) ends T1's deferral for these identities: a load/reference-state result follows
 * the reader. An eligible pressure-2 one becomes Current and reaches the rule backend; a
 * pressure-source-1 one is never eligible, so never Current. Unit transport replay of
 * committed producer bytes through mocked IPC; NOT a native UI qualification witness;
 * the rule backend's answer is a unit simulation. */
import { act, renderHook } from "@testing-library/react";
import { createSolveJobTransport, solveBytes } from "../../test/nativeMechanicsReplay";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { runPreviewMechanics, buildAnalysisRunPreview, hasNativeMechanicsInvocation } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { N_RULE_CHECK_NOT_NUMERICALLY_ELIGIBLE, runRuleChecks } from "../../services/ruleCheckService";
import { N_LOAD_REFERENCE_RULE_CHECK } from "../results/loadReferenceDisclosure";
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

describe("ordinary pressure-2 load/reference-state registration and its invalidation by a load/reference-state edit", () => {
  it.each(["sparse_interactive", "dense_scrutiny"] as Mode[])("%s: registered for the actual model only, readable, eligible standing, recorded as Python records it, and Current (epoch 8)", async mode => {
    const { source, model, manifest } = await solve(`${ORDINARY}/connected-${mode}.raw.json`, `${ORDINARY}/connected.request.json`, mode);
    expect(hasNativeMechanicsInvocation(source, model, mode)).toBe(true);
    expect(numericalResultStanding(source, model)).toMatchObject({ contract: "pressure", eligible: true, findings: [] });
    // B7b: the session records it (the record binds pressure-2's table, as Python's carrier
    // does). Epoch 8 (§4.1, §5.4): with Rust's eligible standing, the matching 0.3.0 record
    // and manifest and the live native invocation, it is Current.
    const analysis = await buildAnalysisRunPreview(source, { inputManifest: manifest });
    const python = json(`fixtures/results/pressure_load_state_connected_${mode === "sparse_interactive" ? "sparse" : "dense"}.analysis_run.json`) as AnalysisRunEnvelope;
    expect(analysis.analysis_run.reproducibility.semantic_contract).toStrictEqual(python.analysis_run.reproducibility.semantic_contract);
    expect(analysis.analysis_run.result_refs.map(r => r.semantic_contract)).toStrictEqual(python.analysis_run.result_refs.map(r => r.semantic_contract));
    const { result } = renderHook(() => useResultsSessionState());
    act(() => { result.current.setResult(source); result.current.setInputManifest(manifest); result.current.setAnalysisRun(analysis); });
    expect(result.current.analysisRun).toBe(analysis);
    expect(result.current.result).toBe(source);
    expect(result.current.currentSolvedResult).toBe(source);
    // A received copy of the same bytes is readable, never Current.
    const copy = structuredClone(source);
    act(() => { result.current.setResult(copy); });
    expect(result.current.result).toBe(copy);
    expect(result.current.currentSolvedResult).toBeNull();
    act(() => { result.current.setResult(source); });
    expect(result.current.currentSolvedResult).toBe(source);
    // A record that does not match the result's run is not Current's record.
    act(() => { result.current.setAnalysisRun({ ...analysis, analysis_run: { ...analysis.analysis_run, run_id: "run:invented-mismatch" } }); });
    expect(result.current.currentSolvedResult).toBeNull();
    act(() => { result.current.setAnalysisRun(analysis); });
    expect(result.current.currentSolvedResult).toBe(source);

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

  it("a pressure-2 source that its reader refuses is never registered", async () => {
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

  it("a sensitive ordinary pressure-2 load/reference-state result is readable and never Current, even with its record", async () => {
    const { source, model, manifest } = await solve("core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-sparse_interactive.raw.json", "core/reporting/result_export/tests/fixtures/load_reference_fallback_uz.request.json", "sparse_interactive");
    expect(hasNativeMechanicsInvocation(source, model, "sparse_interactive")).toBe(true);
    expect(numericalResultStanding(source, model).findings).toContain("NUMERICAL_INTEGRITY_NOT_QUALIFIED");
    const analysis = await buildAnalysisRunPreview(source, { inputManifest: manifest });
    const { result } = renderHook(() => useResultsSessionState());
    act(() => { result.current.setResult(source); result.current.setInputManifest(manifest); result.current.setAnalysisRun(analysis); });
    expect(result.current.result).toBe(source);
    expect(result.current.currentSolvedResult).toBeNull();
  });
});

describe("rule binding on the reachable desktop path", () => {
  it("an ordinary checks_passed pressure-2 load/reference-state result reaches the rule backend with its disclosure line (epoch 8)", async () => {
    const { source, model } = await solve(`${ORDINARY}/pressure-sparse_interactive.raw.json`, `${ORDINARY}/pressure.request.json`, "sparse_interactive");
    expect(hasNativeMechanicsInvocation(source, model, "sparse_interactive")).toBe(true);
    expect(numericalResultStanding(source, model)).toMatchObject({ contract: "pressure", eligible: true, findings: [] });
    const answer = { document_kind: "openpipestress.rule_check.run", rule_pack_id: "invented-test-only", grammar_version: "1.0.0", aggregate_status: "RULE_INPUTS_INCOMPLETE", checks: [], professional_boundary_notice: "notice" };
    invokeMock.mockReset();
    invokeMock.mockResolvedValue(answer);
    const route = await runRuleChecks({ rulePackDocument: { metadata: { rule_pack_id: "invented-test-only" } } as never, model, solvedEnvelope: source });
    const records = source.contract_evidence!.load_reference_states as { reference_configuration_id: string; load_case_id: string; history: { kind: string } }[];
    const distinct = (values: string[]) => [...new Set(values)].join(", ");
    expect(route).toStrictEqual({ route: "tauri_backend", result: answer, loadReferenceDisclosure: N_LOAD_REFERENCE_RULE_CHECK
      .replace("{configurations}", distinct(records.map(r => r.reference_configuration_id)))
      .replace("{cases}", distinct(records.map(r => r.load_case_id)))
      .replace("{histories}", distinct(records.map(r => r.history.kind))) });
    expect(invokeMock.mock.calls.map(([command]) => command)).toEqual(["run_rule_checks"]);
    expect(invokeMock.mock.calls[0][1]).not.toHaveProperty("sourceBlockInvocation");
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
    // A run over it stops at the service's standing step, before the rule backend.
    invokeMock.mockClear();
    await expect(runRuleChecks({ rulePackDocument: { metadata: { rule_pack_id: "invented-test-only" } } as never, model, solvedEnvelope: source }))
      .rejects.toThrow(new Error(`${LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE}: ${N_RULE_CHECK_NOT_NUMERICALLY_ELIGIBLE}`));
    expect(invokeMock).not.toHaveBeenCalled();
  });
});
