import { currentSemanticContract, hasCurrentSourceContract, numericalResultStanding, PHYSICS_CONTRACT_ID, PHYSICS_CONTRACT_SHA256, PRECISION_CONTRACT_ID, PRECISION_CONTRACT_SHA256 } from "../results/numericalResultQuality";
import { act, renderHook } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import { createNativeMechanicsReplay, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { runPreviewMechanics, buildAnalysisRunPreview } from "../../services/previewService";
afterEach(() => { invokeMock.mockReset(); delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__; });
import { useResultsSessionState } from "./resultsSessionState";
import type { MechanicsResult, PreviewModel } from "../../types";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { buildAnalysisRunV03, modelLoadBasisRefs, validateAnalysisRunV03 } from "../../services/analysisRunCompatibility";
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

it("requires actual native invocation plus joined precision analysis and manifest identities before Current", async () => {
 // Explicit unit transport simulation over the unchanged, genuinely emitted
 // current ordinary pair. This is not a native UI/host qualification witness.
 const pair = nativeMechanicsReplayPair("sparse_interactive", {profile:"precision"});
 const replay = createNativeMechanicsReplay({profile:"precision"});
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

// Maintained outputs from actual sparse/dense producer invocations. These are
// received fixture bytes, not a live invocation or a Current-session manifest.
const fixtureRoot = resolve(__dirname, "../../../../../");
const readFixture = (path: string) => JSON.parse(readFileSync(resolve(fixtureRoot, path), "utf8"));
const physicsSource = (mode = "sparse") => readFixture(`fixtures/results/physics_connected_mechanics_${mode}.json`) as MechanicsResult;
const physicsModel = () => readFixture("core/product_physics/tests/fixtures/exact_pressure_connected_request.json").model as PreviewModel;

it.each(["sparse", "dense"])("recognizes actual %s physics semantics without turning received history into Current", async mode => {
 const source = physicsSource(mode), model = physicsModel(), before = JSON.stringify(source);
 const binding = {id:PHYSICS_CONTRACT_ID,sha256:PHYSICS_CONTRACT_SHA256};
 expect(hasCurrentSourceContract(source)).toBe(true);
 expect(currentSemanticContract(source)).toEqual(binding);
 expect(numericalResultStanding(source,model).eligible).toBe(true); // numerical/source scope only
 const solver = {solver_name:source.producer!.component_name,solver_version:source.producer!.component_version,solver_build_ref:"test:received-source-no-live-invocation",solver_mode:mode === "sparse" ? "sparse_interactive" : "dense_scrutiny",settings:{}};
 // Build an independently hashed reference-only analysis record. Do not install
 // it as CurrentSessionInputManifestEvidence or invent the missing UI fields.
 const manifest = {model_basis:{model_ref:model.project.id,model_payload:model},solver_basis:solver};
 const referenceBasis = {manifest,manifest_ref:{object_type:"InputManifest",ref:"test:reference-only-physics-state"},manifest_sha256:await canonicalSha256HexCheckedV1(manifest)};
 const analysis = await buildAnalysisRunV03(source,referenceBasis,undefined,modelLoadBasisRefs(model));
 expect(analysis.analysis_run.reproducibility.semantic_contract).toEqual(binding);
 const {result} = renderHook(() => useResultsSessionState());
 act(()=>{result.current.setResult(source);result.current.setAnalysisRun(analysis);});
 expect(result.current.result).toBe(source);
 expect(result.current.inputManifest).toBeNull();
 expect(result.current.currentSolvedResult).toBeNull();
 expect(result.current.currentSolvedResultRef.current).toBeNull();
 await expect(buildCurrentSessionInputManifest({model,solver,active_rule_packs:[],external_assets:[]})).rejects.toThrow("INPUT-MANIFEST-LOAD-BASIS-INCOMPLETE");
 const mismatched = structuredClone(analysis);
 mismatched.analysis_run.reproducibility.semantic_contract = {id:PRECISION_CONTRACT_ID,sha256:PRECISION_CONTRACT_SHA256};
 await expect(validateAnalysisRunV03(mismatched,source,modelLoadBasisRefs(model))).rejects.toThrow("ANALYSIS_SEMANTIC_CONTRACT_MISMATCH");
 act(()=>result.current.setAnalysisRun(mismatched));
 expect(result.current.currentSolvedResult).toBeNull();
 expect(JSON.stringify(source)).toBe(before);
});

it.each(["unknown method", "Sensitive", "broken physical case"])("keeps actual-source %s controls outside Current", scenario => {
 const source = physicsSource(), model = physicsModel();
 if(scenario === "unknown method") source.producer!.semantic_contract_id = "openpipestress.result_semantics/0.3.0/source-blocks-1";
 else if(scenario === "Sensitive") {
   source.numerical_quality!.status = "sensitive";
   for(const item of source.numerical_quality!.cases) {item.solve_quality="sensitive";item.accuracy_evidence="reference_verified";}
 } else (source.contract_evidence!.exact_cases as unknown[]).pop();
 const before = JSON.stringify(source);
 expect(currentSemanticContract(source)).toEqual(scenario === "unknown method" ? null : {id:PHYSICS_CONTRACT_ID,sha256:PHYSICS_CONTRACT_SHA256});
 expect(numericalResultStanding(source,model).eligible).toBe(false);
 const {result} = renderHook(() => useResultsSessionState());
 act(()=>result.current.setResult(source));
 expect(result.current.result).toBe(source);
 expect(result.current.currentSolvedResult).toBeNull();
 expect(result.current.currentSolvedResultRef.current).toBeNull();
 expect(JSON.stringify(source)).toBe(before);
});
