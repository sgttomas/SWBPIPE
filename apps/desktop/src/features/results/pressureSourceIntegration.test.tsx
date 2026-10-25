import { afterEach, describe, expect, it, vi } from "vitest";
import { act, renderHook, waitFor } from "@testing-library/react";
import { isDeepStrictEqual } from "node:util";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { buildAnalysisRunPreview, runPreviewMechanics, hasNativeMechanicsInvocation, loadDesignKnowledge } from "../../services/previewService";
import { createSolveJobTransport, REPLAY_RESOURCE_GRANT, resultArrayBuffer, startRequestModel } from "../../test/nativeMechanicsReplay";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { getLocalStorageCapability } from "../../services/projectService";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { retainedSourceRecoveryInvocation, sourceRecoveryModeMatches } from "./resultStanding";
import { buildCurrentResultExport } from "../result-export/resultExportAdapter";
import { buildStressNeutralExportPacket, validateStressNeutralExportPacket } from "../stress-neutral/StressNeutralExportPanel";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { buildReportPackageRequest } from "../report/reportPackageRequest";
import { numericalResultStanding } from "./resultStanding";
import { OUTPUT_SURFACES, PRESSURE_SOURCE_OUTPUT_REFUSAL, surfaceOutputRefusal } from "./outputPolicy";
import { isFreshSemanticResult } from "./knownSemanticLimitations";
import { analysisRowSemantics, analysisRecordProjection, modelLoadBasisRefs, validateAnalysisRunV03 } from '../../services/analysisRunCompatibility';
import { readerRowSemantics, sourceContract } from './resultReader';
const resultSemantics = (row: Parameters<ReturnType<typeof readerRowSemantics>>[0], source?: Parameters<typeof readerRowSemantics>[0]) => readerRowSemantics(source)(row).signature;
/** T4-U6 (D-5 §2.4): pressure-source-1's maximum signature for a recovery token on one section basis. */
const SECTION_BASES = ["nominal_section", "reduced_section"] as const;
function maximumSignature(basis: string): string {
  const [token, section] = basis.split("; section_basis=");
  expect(SECTION_BASES).toContain(section);
  const family = token === "retained_source_endpoint_normal_max_v1"
    ? "physics-source-endpoint-normal-maximum-retained-source-endpoint-normal-max-v1"
    : "supported-source-068-open-mechanics-stress-components";
  expect(["retained_source_endpoint_normal_max_v1", "recovered_from_open_mechanics_stress_components"]).toContain(token);
  return `${family}-${section.replace("_", "-")}`;
}

/** B7b: the v3 twin of the retired physics-source-1 integration suite. The joined
 * composite is pressure-source-1 (3.0.0/exact_pressure_v3): its committed producer pairs
 * (fixtures/product_preview/pressure_source/physics_source/) through the actual capture
 * and the workspace session. It registers with its captured invocation and reads
 * numerically eligible standing there, its analysis-run record binds its own table
 * (pressure-source-1). Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md` §4.1-4.5,
 * §5.1; owner, 2026-10-21): it is fresh, so with its registered invocation it is Current;
 * the result JSON export and the stress-neutral export emit it, the report package
 * refuses it with its fresh-result refusal, and every other surface refuses it by name
 * (b7-plan D6). Unit transport replay, NOT a native UI witness. */

// Test isolation (T0R S5): a case that times out keeps running in the background.
// Teardown advances the case generation, unmounts every session hook it created,
// and resets the replay; a stale case body stops at its next checkpoint instead of
// invoking the next case's replay, and job ids carry the generation so a stale
// in-flight poll can never match another case's job.
let caseGeneration = 0;
const liveHooks = new Set<{ unmount(): void }>();
function assertLiveCase(generation: number) {
  if (generation !== caseGeneration) throw new Error("STALE_CASE_AFTER_TEARDOWN");
}
afterEach(() => {
  caseGeneration += 1;
  for (const hook of liveHooks) { try { hook.unmount(); } catch { /* already unmounted */ } }
  liveHooks.clear();
  invokeMock.mockReset(); window.localStorage.clear(); delete (window as any).__TAURI_INTERNALS__; // eslint-disable-line @typescript-eslint/no-explicit-any
});
const fixturePrefix = "../../../../../fixtures/product_preview/pressure_source/physics_source/";
const fixtureSources = import.meta.glob("../../../../../fixtures/product_preview/pressure_source/physics_source/*.json", { query: "?raw", import: "default", eager: true }) as Record<string,string>;
const pairs = ["n05", "n06", "mixed", "fields", "n05_units", "mixed_units", "n05_unicode"].flatMap(name => ["sparse_interactive", "dense_scrutiny"].map(mode => [name, mode] as const));
async function received(name: string, mode: string, generation = caseGeneration) {
  assertLiveCase(generation);
  const sourceText = fixtureSources[`${fixturePrefix}${name}-${mode}.raw.json`];
  const request = JSON.parse(fixtureSources[`${fixturePrefix}${name}.request.json`]);
  const original = JSON.parse(sourceText) as MechanicsResult, model = request.model as PreviewModel;
  (window as any).__TAURI_INTERNALS__ = {}; // eslint-disable-line @typescript-eslint/no-explicit-any
  // Unit-only transport replay of the exact separately captured producer pair.
  // Only the production private IPC path can register these delivered bytes.
  const transport = createSolveJobTransport((sent) => {
    assertLiveCase(generation);
    expect(sent.mode).toBe(mode);
    expect(isDeepStrictEqual({ model: sent.model, materials: [] }, request)).toBe(true);
    return sourceText; // the producer's bytes, as captured
  });
  invokeMock.mockImplementation(async (command: string, args: unknown, options?: unknown) => {
    assertLiveCase(generation);
    return transport.invoke(command, args, options);
  });
  const source = await runPreviewMechanics(model, mode as "sparse_interactive" | "dense_scrutiny");
  const inputManifest = await buildCurrentSessionInputManifest({model,solver:{solver_name:source.producer!.component_name,solver_version:source.producer!.component_version,solver_build_ref:"unit-transport-replay-not-native-witness",solver_mode:mode,settings:{}},active_rule_packs:[],external_assets:[]});
  const analysisRun = await buildAnalysisRunPreview(source,{inputManifest});
  return {model,source,original,sourceText,inputManifest,analysisRun,request};
}
// The same captured pair through the actual workspace session: load_preview_model
// returns the captured request model and the session's own backend-job solve
// receives the captured producer bytes. Unit transport replay, NOT a native UI witness.
async function sessionReceived(name: string, mode: string, generation = caseGeneration) {
  assertLiveCase(generation);
  const sourceText = fixtureSources[`${fixturePrefix}${name}-${mode}.raw.json`];
  const request = JSON.parse(fixtureSources[`${fixturePrefix}${name}.request.json`]);
  const original = JSON.parse(sourceText) as MechanicsResult;
  // Capture the real browser bootstrap records before installing the replay.
  delete (window as any).__TAURI_INTERNALS__; // eslint-disable-line @typescript-eslint/no-explicit-any
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const scope = "unit_transport_replay_not_native_ui_qualification", started: string[] = [];
  assertLiveCase(generation);
  invokeMock.mockImplementation(async (command: string, args: any, options?: unknown) => { // eslint-disable-line @typescript-eslint/no-explicit-any
    assertLiveCase(generation);
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(request.model);
    if (command === "start_preview_mechanics_job") {
      const sent = startRequestModel(args, options);
      if (!isDeepStrictEqual({ model: sent.model, materials: [] }, request)) throw new Error("REPLAY_REQUEST_MISMATCH");
      started.push(String(sent.mode));
      const job_id = `unit-transport-replay:${generation}:${name}:${started.length}`;
      return { job_id, backend_cancellation_token: `${job_id}:token`, state: "queued", cancellation_scope: scope, resource: { ...REPLAY_RESOURCE_GRANT, ingress_reserved_bytes: 4096 } };
    }
    const current = `unit-transport-replay:${generation}:${name}:${started.length}`;
    if (command === "poll_preview_mechanics_job" && args?.jobId === current) {
      return { job_id: args.jobId, state: "completed", queue_reason: null, cancellation_requested: false, cancellation_status: "not_requested", cancellation_scope: scope, result_available: true, result_bytes: sourceText.length, error_message: null, resource_refusal: null, resource: { ...REPLAY_RESOURCE_GRANT, ingress_reserved_bytes: 4096 } };
    }
    if (command === "take_preview_mechanics_job_result" && args?.jobId === current) return resultArrayBuffer(sourceText);
    if (command === "dispose_preview_mechanics_result") return null;
    throw new Error(`REPLAY_COMMAND_UNSUPPORTED: ${command}`);
  });
  (window as any).__TAURI_INTERNALS__ = {}; // eslint-disable-line @typescript-eslint/no-explicit-any
  const hook = renderHook(() => useWorkspaceSession());
  liveHooks.add(hook);
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  await waitFor(() => expect(hook.result.current.model.modelHash).not.toBeNull());
  assertLiveCase(generation);
  act(() => hook.result.current.results.setSolverMode(mode as "sparse_interactive" | "dense_scrutiny"));
  await act(async () => { await hook.result.current.results.handleRun(); });
  await waitFor(() => expect(hook.result.current.results.result).not.toBeNull());
  expect(started).toEqual([mode]);
  const { result: source, inputManifest, analysisRun } = hook.result.current.results;
  return { hook, started, model: hook.result.current.model.model!, source: source!, original, sourceText, inputManifest: inputManifest!, analysisRun: analysisRun! };
}

describe("joined composite (pressure-source-1) native transport unit simulation", () => {
  it.each(pairs)("%s %s binds actual input and portable evidence, reads eligible with its capture, and is offered on the opened outputs only", async (name, mode) => {
    const generation = caseGeneration;
    const direct = await received(name,mode,generation);
    const {source,model,analysisRun,inputManifest,request} = direct;
    expect(source).toEqual(direct.original);
    expect(sourceContract(source)).toBe("pressure_source");
    expect(hasNativeMechanicsInvocation(source,model,mode)).toBe(true);
    expect(source.numerical_quality!.status).not.toBe("checks_passed");
    expect(numericalResultStanding(source,model).eligible).toBe(true);
    expect(numericalResultStanding(structuredClone(source),model).eligible).toBe(false);
    expect(retainedSourceRecoveryInvocation(source,model)).toEqual({ request, solver_mode: mode });
    // T4-U6 (D-5 §2.4): one signature per (recovery token x section basis).
    for (const row of source.results.filter(r=>r.kind==='pipe_elastic_normal_stress_maximum_v2')) {
      const signature=maximumSignature(row.metadata!.basis as string);
      expect(resultSemantics(row,source)?.signature_id).toBe(signature);
      expect(analysisRowSemantics(row,source).semantic?.signature_id).toBe(signature);
      expect(analysisRun.analysis_run.result_refs.find(r=>r.result_ref.ref===row.id)?.semantic_contract?.signature_id).toBe(signature);
    }
    expect(analysisRun.analysis_run.source_block_recovery).toEqual(source.source_block_recovery);
    await validateAnalysisRunV03(analysisRun, source);
    // Epoch 8 (design §5.1): the result JSON export, the stress-neutral export and the rule
    // check add no policy refusal; every other surface refuses pressure-source-1 by name
    // (b7-plan D6), under pressure-2's code.
    expect(isFreshSemanticResult(source)).toBe(true);
    for (const surface of OUTPUT_SURFACES) {
      expect(surfaceOutputRefusal(source,model,surface),surface).toBe(["result-export","stress-neutral","rule-check"].includes(surface)?null:PRESSURE_SOURCE_OUTPUT_REFUSAL);
    }
    // Design §4.2: the eligible result exports, carrying its contract evidence and received
    // receipt whole, as Rust's derivative does.
    const document = await buildCurrentResultExport({model,result:source,analysisRun,inputManifest}) as any; // eslint-disable-line @typescript-eslint/no-explicit-any
    expect(document.result_envelope.contract_evidence).toEqual(source.contract_evidence);
    expect(document.result_envelope.source_block_recovery).toEqual(source.source_block_recovery);
    // Design §4.3: the stress-neutral package likewise, and it validates against its source.
    const packet = await buildStressNeutralExportPacket({model,result:source,analysisRun});
    expect(packet.contract_evidence).toEqual(source.contract_evidence);
    expect(packet.source_block_recovery).toEqual(source.source_block_recovery);
    await validateStressNeutralExportPacket(JSON.parse(JSON.stringify(packet)),source,analysisRun,modelLoadBasisRefs(model));
    // Without its registered invocation (a copy) standing declines it at both builders.
    const copy = structuredClone(source);
    await expect(buildCurrentResultExport({model,result:copy,analysisRun,inputManifest})).rejects.toThrow("CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE");
    await expect(buildStressNeutralExportPacket({model,result:copy,analysisRun})).rejects.toThrow("SN-NUMERICAL-INTEGRITY-NEEDS-RECOMPUTE");
    // Design §4.5; owner ruling 10A: the report package refuses it by its report-package cell.
    await expect(buildReportPackageRequest({model,result:source,analysisRun,inputManifest} as never)).rejects.toThrow(PRESSURE_SOURCE_OUTPUT_REFUSAL);
    // Mode binding: these bytes were captured under `mode` only.
    const otherMode=mode==="sparse_interactive"?"dense_scrutiny":"sparse_interactive";
    expect(hasNativeMechanicsInvocation(source,model,otherMode)).toBe(false);
    expect(sourceRecoveryModeMatches(source,model,mode)).toBe(true);
    expect(sourceRecoveryModeMatches(source,model,otherMode)).toBe(false);
    // Through the workspace session: the solve completes and records the result, which is
    // Current with its registered invocation (design §4.1).
    const session = await sessionReceived(name,mode,generation);
    expect(session.source).toEqual(session.original);
    expect(session.inputManifest.manifest.solver_basis.solver_mode).toBe(mode);
    expect(hasNativeMechanicsInvocation(session.source,session.model,mode)).toBe(true);
    expect(numericalResultStanding(session.source,session.model).eligible).toBe(true);
    expect(session.analysisRun.analysis_run.run_id).toBe(session.source.run_id);
    expect(session.hook.result.current.results.currentSolvedResult).toBe(session.source);
    // Mode binding through the session: a solve that receives these bytes, recorded under
    // `mode`, for the other mode is refused at the session boundary and holds nothing.
    assertLiveCase(generation);
    act(()=>session.hook.result.current.results.setSolverMode(otherMode));
    await act(async()=>{await session.hook.result.current.results.handleRun();});
    expect(session.started).toEqual([mode,otherMode]);
    expect(session.hook.result.current.results.result).toBeNull();
    expect(session.hook.result.current.results.analysisRun).toBeNull();
    expect(session.hook.result.current.results.currentSolvedResult).toBeNull();
    expect(session.hook.result.current.results.solveJob.state).toBe("failed");
    expect(session.hook.result.current.results.solveJob.events.at(-1)?.message).toContain("SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED");
    session.hook.unmount(); liveHooks.delete(session.hook);
  },30000);
});
it('rejects rehashed analysis signatures that substitute ordinary and retained maxima',async()=>{
  const {source,analysisRun}=await received('mixed','sparse_interactive');
  // T4-U6 (D-5 §2.4): each maximum names its section basis; the substituted signature is the
  // other recovery token's on the same basis.
  for(const basis of SECTION_BASES.flatMap(section=>['retained_source_endpoint_normal_max_v1','recovered_from_open_mechanics_stress_components'].map(token=>`${token}; section_basis=${section}`))) {
    const row=source.results.find(r=>r.kind==='pipe_elastic_normal_stress_maximum_v2'&&r.metadata?.basis===basis)!;
    expect(row).toBeDefined();
    const changed=structuredClone(analysisRun),reference=changed.analysis_run.result_refs.find(r=>r.result_ref.ref===row.id)!;
    const [token,section]=basis.split('; section_basis=');
    const other=token==='retained_source_endpoint_normal_max_v1'?'recovered_from_open_mechanics_stress_components':'retained_source_endpoint_normal_max_v1';
    expect(maximumSignature(basis)).not.toBe(maximumSignature(`${other}; section_basis=${section}`));
    reference.semantic_contract!.signature_id=maximumSignature(`${other}; section_basis=${section}`);
    changed.analysis_run.hashes.find(h=>h.payload_scope==='analysis_run_record')!.value=await canonicalSha256HexCheckedV1(analysisRecordProjection(changed));
    await expect(validateAnalysisRunV03(changed,source)).rejects.toThrow('ANALYSIS_ROW_SOURCE_BINDING_MISMATCH');
    for(const invalid of [undefined,'unrecognized_basis']) {
      const copied=structuredClone(row);copied.metadata!.basis=invalid as never;
      expect(()=>analysisRowSemantics(copied,source)).toThrow('SOURCE_BASIS_CONTRADICTION');
      expect(()=>resultSemantics(copied,source)).toThrow('SOURCE_BASIS_CONTRADICTION');
    }
  }
});
