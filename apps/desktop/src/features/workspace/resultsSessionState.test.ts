import { PRESSURE_CONTRACT_ID } from "../results/numericalResultQuality";
import { retiredPrecisionStatement } from "../../test-support/retiredPrecision";
import { PRESSURE_CONTRACT_SHA256 } from "../../test-support/contractBindings";
import { currentSemanticContract, hasCurrentSourceContract } from "../results/resultReader";
import { numericalResultStanding, sourceRecoveryModeMatches } from "../results/resultStanding";
import { act, renderHook } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createNativeMechanicsReplay, createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { runPreviewMechanics, buildAnalysisRunPreview } from "../../services/previewService";
afterEach(() => { invokeMock.mockReset(); delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__; });
import { currentRequirementsExceptNumericalEligibility, useResultsSessionState } from "./resultsSessionState";
import { loadedWasmEngine, type WasmOperationEngine } from "../../services/wasmEngine/loadWasmEngine";
import type { MechanicsResult, PreviewModel } from "../../types";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
it("keeps solved historical and unassessed carriers out of Current without changing stored evidence", () => {
 const {result} = renderHook(() => useResultsSessionState());
 const legacy = {schema_version:"0.1.0", document_kind:"MechanicsResult", run_id:"r",model_ref:"m",status:{mechanics:"MECHANICS_SOLVED",rule_check:"RULE_INPUTS_INCOMPLETE",professional_acceptance:"NOT_PROVIDED"}, summary:{},results:[],diagnostics:[]} as MechanicsResult;
 act(()=>result.current.setResult(legacy));
 expect(result.current.result).toBe(legacy);
 expect(result.current.currentSolvedResult).toBeNull();
 expect(result.current.currentSolvedResultRef.current).toBeNull();
 expect(result.current.comparison).toBeNull();
 const unknown = {...legacy,schema_version:"0.2.0"};
 act(()=>result.current.setResult(unknown));
 expect(result.current.result).toBe(unknown);
 expect(result.current.currentSolvedResult).toBeNull();
});

it("drops the retained C2 copy itself when its source is replaced or cleared", () => {
 const {result} = renderHook(() => useResultsSessionState());
 const source = JSON.parse(readFileSync(resolve(__dirname, '../../../../../fixtures/results/preview_physics_connected_sparse.json'), 'utf8')) as MechanicsResult;
 const copy = structuredClone(source);
 act(() => { result.current.setResult(source); result.current.setFullResultCopy({of:source,copy}); });
 expect(result.current.fullResultCopy?.copy).toBe(copy);
 const replacement = structuredClone(source);
 act(() => result.current.setResult(replacement));
 expect(result.current.fullResultCopy).toBeNull();
 act(() => result.current.setFullResultCopy({of:replacement,copy}));
 expect(result.current.fullResultCopy?.copy).toBe(copy);
 act(() => result.current.setResult(null));
 expect(result.current.fullResultCopy).toBeNull();
});

it("requires actual native invocation plus joined preview-physics analysis and manifest identities before Current", async () => {
 // Explicit unit transport simulation over the unchanged, genuinely emitted
 // current ordinary pair (preview-physics-2 since T0R; a stored precision-1
 // statement is refused, the next test). This is not a native UI/host qualification witness.
 const pair = nativeMechanicsReplayPair("sparse_interactive", {profile:"preview"});
 const replay = createNativeMechanicsReplay({profile:"preview"});
 (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
 invokeMock.mockImplementation(replay.invoke);
 const source = await runPreviewMechanics(pair.model, pair.mode);
 const manifest = await buildCurrentSessionInputManifest({model:pair.model,solver:{solver_name:source.producer!.component_name,solver_version:source.producer!.component_version,solver_build_ref:"test:actual-core-pair-native-transport-simulation",solver_mode:pair.mode,settings:{}},active_rule_packs:[],external_assets:[]});
 const analysis = await buildAnalysisRunPreview(source,{inputManifest:manifest});
 const {result} = renderHook(() => useResultsSessionState());
 act(()=>{result.current.setResult(source);result.current.setInputManifest(manifest);});
 expect(result.current.currentSolvedResult).toBeNull();
 act(()=>result.current.setAnalysisRun(analysis));
 expect(result.current.currentSolvedResult).toBe(source);
 const receivedCopy = structuredClone(source);
 act(()=>result.current.setResult(receivedCopy));
 expect(result.current.currentSolvedResult).toBeNull(); // hashes/headers cannot recreate live registration
 const sensitive = structuredClone(source);
 sensitive.numerical_quality!.status = "sensitive";
 sensitive.numerical_quality!.cases[0].solve_quality = "sensitive";
 sensitive.numerical_quality!.cases[0].accuracy_evidence = "reference_verified";
 const before = JSON.stringify(sensitive);
 act(()=>result.current.setResult(sensitive));
 expect(result.current.result).toBe(sensitive);
 expect(result.current.currentSolvedResult).toBeNull();
 expect(result.current.currentSolvedResultRef.current).toBeNull();
 expect(JSON.stringify(sensitive)).toBe(before);
 act(()=>result.current.setResult(source));
 expect(result.current.currentSolvedResult).toBe(source);
 act(()=>result.current.setAnalysisRun({...analysis,schema_version:"0.2.0"}));
 expect(result.current.currentSolvedResult).toBeNull();
});

it("never admits a stored precision-1 statement as Current (B1: refused by name, unsupported)", async () => {
 const pair = nativeMechanicsReplayPair("sparse_interactive", {profile:"preview"});
 const source = retiredPrecisionStatement();
 const {result} = renderHook(() => useResultsSessionState());
 act(()=>{result.current.setResult(source);});
 expect(result.current.result).toBe(source); // readable as data
 expect(numericalResultStanding(source, pair.model).findings).toEqual(["SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED"]);
 expect(result.current.currentSolvedResult).toBeNull();
});


// Maintained outputs from actual sparse/dense producer invocations. These are
// received fixture bytes, not a live invocation or a Current-session manifest.
const fixtureRoot = resolve(__dirname, "../../../../../");
const readFixture = (path: string) => JSON.parse(readFileSync(resolve(fixtureRoot, path), "utf8"));
// B7b: the exact route's received statement is pressure-2, the v3 twin of the retired physics-1.
const pressureSource = (mode = "sparse") => readFixture(`fixtures/results/pressure_connected_mechanics_${mode}.json`) as MechanicsResult;
const pressureModel = () => readFixture("core/product_physics/tests/fixtures/exact_pressure_connected_request.json").model as PreviewModel;

it.each(["sparse", "dense"])("recognizes actual %s pressure-2 semantics without turning received history into Current", async mode => {
 const source = pressureSource(mode), model = pressureModel(), before = JSON.stringify(source);
 const binding = {id:PRESSURE_CONTRACT_ID,sha256:PRESSURE_CONTRACT_SHA256};
 expect(hasCurrentSourceContract(source)).toBe(true);
 expect(currentSemanticContract(source)).toEqual(binding);
 expect(numericalResultStanding(source,model).eligible).toBe(true); // numerical/source scope only
 const solver = {solver_name:source.producer!.component_name,solver_version:source.producer!.component_version,solver_build_ref:"test:received-source-no-live-invocation",solver_mode:mode === "sparse" ? "sparse_interactive" : "dense_scrutiny",settings:{}};
 // An independently hashed reference-only basis: the desktop records pressure-2 as Python
 // does (B7b), and a held record still does not make a received statement Current.
 const manifest = {model_basis:{model_ref:model.project.id,model_payload:model},solver_basis:solver};
 const referenceBasis = {manifest,manifest_ref:{object_type:"InputManifest",ref:"test:reference-only-pressure-state"},manifest_sha256:await canonicalSha256HexCheckedV1(manifest)};
 const record = await buildAnalysisRunV03(source,referenceBasis,undefined,modelLoadBasisRefs(model));
 expect(record.analysis_run.reproducibility.semantic_contract).toEqual(binding);
 const {result} = renderHook(() => useResultsSessionState());
 act(()=>{result.current.setResult(source);});
 expect(result.current.result).toBe(source);
 expect(result.current.inputManifest).toBeNull();
 expect(result.current.currentSolvedResult).toBeNull();
 expect(result.current.currentSolvedResultRef.current).toBeNull();
 await expect(buildCurrentSessionInputManifest({model,solver,active_rule_packs:[],external_assets:[]})).rejects.toThrow("INPUT-MANIFEST-LOAD-BASIS-INCOMPLETE");
 expect(JSON.stringify(source)).toBe(before);
});

it.each(["unknown method", "Sensitive", "broken physical case"])("keeps actual-source %s controls outside Current", scenario => {
 const source = pressureSource(), model = pressureModel();
 if(scenario === "unknown method") source.producer!.semantic_contract_id = "openpipestress.result_semantics/0.3.0/source-blocks-1";
 else if(scenario === "Sensitive") {
   source.numerical_quality!.status = "sensitive";
   for(const item of source.numerical_quality!.cases) {item.solve_quality="sensitive";item.accuracy_evidence="reference_verified";}
 } else (source.contract_evidence!.exact_cases as unknown[]).pop();
 const before = JSON.stringify(source);
 expect(currentSemanticContract(source)).toEqual(scenario === "unknown method" ? null : {id:PRESSURE_CONTRACT_ID,sha256:PRESSURE_CONTRACT_SHA256});
 expect(numericalResultStanding(source,model).eligible).toBe(false);
 const {result} = renderHook(() => useResultsSessionState());
 act(()=>result.current.setResult(source));
 expect(result.current.result).toBe(source);
 expect(result.current.currentSolvedResult).toBeNull();
 expect(result.current.currentSolvedResultRef.current).toBeNull();
 expect(JSON.stringify(source)).toBe(before);
});

// B7b: the gate's physics-source mode clause went with physics-source-1. Its v3 twin is
// pressure-source-1 (the composite; B4/D1 (b): source blocks remain only there). Epoch 8
// (DEL-04-07 Design `desktop-exact-outputs.md` §4.1): it is Current only with Rust's
// eligible standing (the registered invocation), the matching 0.3.0 record and manifest
// and the live native invocation in the manifest's mode. Its registration records the one
// mode it was captured in, so a manifest naming the other mode is not Current.
const retainedSourceFixtures = import.meta.glob("../../../../../fixtures/product_preview/pressure_source/physics_source/n05*.json", { query: "?raw", import: "default", eager: true }) as Record<string, string>;
it.each([["pressure_source", "pressure_source/physics_source", "n05.request.json"]] as const)("admits a registered %s result as Current in its captured mode only (epoch 8)", async (_family, folder, requestName) => {
 const prefix = `../../../../../fixtures/product_preview/${folder}/`;
 const mode = "sparse_interactive";
 const request = JSON.parse(retainedSourceFixtures[`${prefix}${requestName}`]);
 const originalText = retainedSourceFixtures[`${prefix}n05-${mode}.raw.json`];
 const model = request.model as PreviewModel;
 (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
 // Unit transport replay of the captured producer bytes; not a native UI witness.
 invokeMock.mockImplementation(createSolveJobTransport((sent) => {
   expect(sent.mode).toBe(mode);
   expect(sent.model).toEqual(request.model);
   return originalText; // the producer's bytes, as captured
 }).invoke);
 const source = await runPreviewMechanics(model, mode);
 const manifest = await buildCurrentSessionInputManifest({model,solver:{solver_name:source.producer!.component_name,solver_version:source.producer!.component_version,solver_build_ref:"test:retained-source-mode-control",solver_mode:mode,settings:{}},active_rule_packs:[],external_assets:[]});
 const analysis = await buildAnalysisRunPreview(source,{inputManifest:manifest});
 const {result} = renderHook(() => useResultsSessionState());
 act(()=>{result.current.setResult(source);result.current.setInputManifest(manifest);result.current.setAnalysisRun(analysis);});
 expect(numericalResultStanding(source,model).eligible).toBe(true);
 expect(sourceRecoveryModeMatches(source,model,mode)).toBe(true);
 expect(sourceRecoveryModeMatches(source,model,"dense_scrutiny")).toBe(false);
 expect(result.current.currentSolvedResult).toBe(source);
 const otherMode = structuredClone(manifest);
 otherMode.manifest.solver_basis.solver_mode = "dense_scrutiny";
 act(()=>result.current.setInputManifest(otherMode));
 expect(result.current.currentSolvedResult).toBeNull();
 act(()=>result.current.setInputManifest(manifest));
 expect(result.current.currentSolvedResult).toBe(source);
 // A received copy has no registration: readable, never Current.
 const copy = structuredClone(source);
 act(()=>result.current.setResult(copy));
 expect(numericalResultStanding(copy,model).eligible).toBe(false);
 expect(result.current.currentSolvedResult).toBeNull();
});

// DEL-07-05 numerical health §3: the factored Current predicate and the session's
// indicator. Admission is unchanged: the indicator never makes or unmakes Current, and
// the hold applies only to a non-Current result whose other requirements hold.
const engineForHealth = loadedWasmEngine() as WasmOperationEngine;
const originalHealthBinding = engineForHealth.numericalHealthJson;
afterEach(() => { engineForHealth.numericalHealthJson = originalHealthBinding; });
it("keeps admission as the full conjunction and reads the indicator beside it", async () => {
  const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const replay = createNativeMechanicsReplay({ profile: "preview" });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(replay.invoke);
  const source = await runPreviewMechanics(pair.model, pair.mode);
  const manifest = await buildCurrentSessionInputManifest({ model: pair.model, solver: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "test:numerical-health-session", solver_mode: pair.mode, settings: {} }, active_rule_packs: [], external_assets: [] });
  const analysis = await buildAnalysisRunPreview(source, { inputManifest: manifest });
  const { result } = renderHook(() => useResultsSessionState());
  act(() => { result.current.setResult(source); result.current.setInputManifest(manifest); result.current.setAnalysisRun(analysis); });
  expect(currentRequirementsExceptNumericalEligibility({ result: source, analysisRun: analysis, inputManifest: manifest })).toBe(true);
  expect(result.current.currentSolvedResult).toBe(source);
  expect(result.current.numericalHealth?.headline).toBe("Numerical health: checks passed");
  expect(result.current.numericalHealthHold).toBeNull();

  // Even an indicator claiming a verdict-only red case changes neither Current nor the hold.
  engineForHealth.numericalHealthJson = () => JSON.stringify({ route: "generic", verdict_only: true, combinations: [],
    cases: [{ case_id: "x", state: "sensitive", code: "NUMERICAL_INTEGRITY_SENSITIVE", integrity_diagnostic_id: null, rcond: null, digits: null, conditioning_sensitive: false, causes: [] }] });
  const copy = structuredClone(source);
  act(() => result.current.setResult(copy));
  expect(result.current.currentSolvedResult).toBeNull();
  // A copy has no native registration: another Current requirement fails, so no hold.
  expect(currentRequirementsExceptNumericalEligibility({ result: copy, analysisRun: analysis, inputManifest: manifest })).toBe(false);
  expect(result.current.numericalHealth?.headline).toBe("Numerical health: 1 sensitive");
  expect(result.current.numericalHealth?.outputSentence).toBeNull();
  expect(result.current.numericalHealthHold).toBeNull();
  act(() => result.current.setResult(source));
  expect(result.current.currentSolvedResult).toBe(source);
  expect(result.current.numericalHealthHold).toBeNull();
});

it("holds a real verdict-only generic result: every other Current requirement met, the verdict alone refused", async () => {
  // The committed producer output for an invented skewed-tee model whose ordinary solve is
  // sensitive (dense scrutiny), handed over through the mocked native job transport.
  const product = resolve(__dirname, "../../../../../fixtures/product_preview");
  const model = JSON.parse(readFileSync(resolve(product, "rf_skew_t_cant_off_122_r1e-04.desktop_model.json"), "utf8")) as PreviewModel;
  const bytes = readFileSync(resolve(product, "rf_skew_t_cant_off_122_r1e-04.desktop_result_dense_scrutiny.json"), "utf8");
  const transport = createSolveJobTransport(() => bytes);
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(transport.invoke);
  const source = await runPreviewMechanics(model, "dense_scrutiny");
  const manifest = await buildCurrentSessionInputManifest({ model, solver: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "test:numerical-health-hold", solver_mode: "dense_scrutiny", settings: {} }, active_rule_packs: [], external_assets: [] });
  const analysis = await buildAnalysisRunPreview(source, { inputManifest: manifest });
  const { result } = renderHook(() => useResultsSessionState());
  act(() => { result.current.setResult(source); result.current.setInputManifest(manifest); result.current.setAnalysisRun(analysis); });
  expect(currentRequirementsExceptNumericalEligibility({ result: source, analysisRun: analysis, inputManifest: manifest })).toBe(true);
  expect(result.current.currentSolvedResult).toBeNull();
  expect(result.current.numericalHealthHold).toEqual({
    headline: "Numerical health: 1 sensitive",
    shortHeadline: "Numerical health: sensitive",
    outputSentence: "No results of this run can be exported or used by rule checks while 1 load case(s) are not checks passed."
  });
  expect(result.current.numericalHealth?.outputSentence).toBe("No results of this run can be exported or used by rule checks while 1 load case(s) are not checks passed.");
  // A later edit of the record removes the hold: another requirement now fails.
  act(() => result.current.setAnalysisRun({ ...analysis, schema_version: "0.2.0" } as typeof analysis));
  expect(result.current.numericalHealthHold).toBeNull();
});
