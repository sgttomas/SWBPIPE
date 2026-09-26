import { afterEach, describe, expect, it, vi } from "vitest";
import { act, renderHook, waitFor } from "@testing-library/react";
import { mkdirSync, writeFileSync } from "node:fs";
import path from "node:path";
import { isDeepStrictEqual } from "node:util";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { buildAnalysisRunPreview, runPreviewMechanics, hasNativeMechanicsInvocation, loadDesignKnowledge } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { getLocalStorageCapability } from "../../services/projectService";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { physicsSourceModeMatches } from "./physicsSourceRecovery";
import { buildCurrentResultExport, validateResultDocument } from "../result-export/resultExportAdapter";
import { buildStressNeutralExportPacket, validateStressNeutralExportPacket, validateStressNeutralCsv } from "../stress-neutral/StressNeutralExportPanel";
import { canonicalJsonCheckedV1, canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { buildReportPackageRequest } from "../report/reportPackageRequest";
import { numericalResultStanding } from "./numericalResultQuality";
import { analysisRowSemantics, analysisRecordProjection, validateAnalysisRunV03 } from '../../services/analysisRunCompatibility';
import { resultSemantics } from './resultSemantics';

afterEach(() => { invokeMock.mockReset(); window.localStorage.clear(); delete (window as any).__TAURI_INTERNALS__; });
const fixturePrefix = "../../../../../fixtures/product_preview/physics_source/";
const fixtureSources = import.meta.glob("../../../../../fixtures/product_preview/physics_source/*.json", { query: "?raw", import: "default", eager: true }) as Record<string,string>;
const pairs = ["n05", "n06", "mixed", "fields", "n05_units", "mixed_units", "n05_unicode"].flatMap(name => ["sparse_interactive", "dense_scrutiny"].map(mode => [name, mode] as const));
function valueBits(value: number) { const bytes = new DataView(new ArrayBuffer(8)); bytes.setFloat64(0,value,false);return bytes.getBigUint64(0,false).toString(16).padStart(16,"0"); }
async function received(name: string, mode: string) {
  const sourceText = fixtureSources[`${fixturePrefix}${name}-${mode}.raw.json`];
  const request = JSON.parse(fixtureSources[`${fixturePrefix}${name}.request.json`]);
  const original = JSON.parse(sourceText) as MechanicsResult, model = request.model as PreviewModel;
  (window as any).__TAURI_INTERNALS__ = {};
  // Unit-only transport replay of the exact separately captured producer pair.
  // Only the production private IPC path can register these delivered bytes.
  invokeMock.mockImplementation(async (command: string, args: any) => {
    expect(command).toBe("run_preview_mechanics_with_solver_mode");
    expect(args.solverMode).toBe(mode);
    expect(isDeepStrictEqual({ model: args.model, materials: [] }, request)).toBe(true);
    return structuredClone(original);
  });
  const source = await runPreviewMechanics(model, mode as "sparse_interactive" | "dense_scrutiny");
  const inputManifest = await buildCurrentSessionInputManifest({model,solver:{solver_name:source.producer!.component_name,solver_version:source.producer!.component_version,solver_build_ref:"unit-transport-replay-not-native-witness",solver_mode:mode,settings:{}},active_rule_packs:[],external_assets:[]});
  const analysisRun = await buildAnalysisRunPreview(source,{inputManifest});
  return {model,source,original,sourceText,inputManifest,analysisRun};
}
// The same captured pair through the actual workspace session, the permitted
// session boundary: load_preview_model returns the captured request model and
// the session's own backend-job solve receives the captured producer bytes for
// every requested mode. Current standing comes only from the session's solve
// path and qualification gate. Unit transport replay, NOT a native UI witness.
async function sessionReceived(name: string, mode: string) {
  const sourceText = fixtureSources[`${fixturePrefix}${name}-${mode}.raw.json`];
  const request = JSON.parse(fixtureSources[`${fixturePrefix}${name}.request.json`]);
  const original = JSON.parse(sourceText) as MechanicsResult;
  // Capture the real browser bootstrap records before installing the replay.
  delete (window as any).__TAURI_INTERNALS__;
  const storage = await getLocalStorageCapability(), knowledge = await loadDesignKnowledge();
  const scope = "unit_transport_replay_not_native_ui_qualification", started: string[] = [];
  invokeMock.mockImplementation(async (command: string, args: any) => {
    if (command === "get_local_storage_capability") return storage;
    if (command === "load_design_knowledge") return knowledge;
    if (command === "sync_native_shell_state") return null;
    if (command === "load_preview_model" && args === undefined) return structuredClone(request.model);
    if (command === "start_preview_mechanics_job_with_solver_mode") {
      if (!isDeepStrictEqual({ model: args.model, materials: [] }, request)) throw new Error("REPLAY_REQUEST_MISMATCH");
      started.push(args.solverMode);
      const job_id = `unit-transport-replay:${name}:${started.length}`;
      return { job_id, backend_cancellation_token: `${job_id}:token`, state: "queued", cancellation_scope: scope };
    }
    if (command === "poll_preview_mechanics_job" && args?.jobId === `unit-transport-replay:${name}:${started.length}`) {
      return { job_id: args.jobId, state: "completed", cancellation_requested: false, cancellation_status: "not_requested", cancellation_scope: scope, result: structuredClone(original), error_message: null };
    }
    throw new Error(`REPLAY_COMMAND_UNSUPPORTED: ${command}`);
  });
  (window as any).__TAURI_INTERNALS__ = {};
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  await waitFor(() => expect(hook.result.current.model.modelHash).not.toBeNull());
  act(() => hook.result.current.results.setSolverMode(mode as "sparse_interactive" | "dense_scrutiny"));
  await act(async () => { await hook.result.current.results.handleRun(); });
  await waitFor(() => expect(hook.result.current.results.currentSolvedResult).not.toBeNull());
  expect(started).toEqual([mode]);
  const { result: source, inputManifest, analysisRun } = hook.result.current.results;
  return { hook, started, model: hook.result.current.model.model!, source: source!, original, sourceText, inputManifest: inputManifest!, analysisRun: analysisRun! };
}

describe("joined composite native transport unit simulation", () => {
  it.each(pairs)("%s %s binds actual input, Current gates and portable evidence", async (name, mode) => {
    // Direct IPC route premise, as before: the production registrar binds the
    // exact captured pair. Current itself is then reached through the session.
    const direct = await received(name,mode);
    expect(direct.source).toEqual(direct.original);
    expect(hasNativeMechanicsInvocation(direct.source,direct.model,mode)).toBe(true);
    const args = await sessionReceived(name,mode), {hook,source,model,inputManifest,analysisRun}=args;
    expect(source).toEqual(args.original);
    expect(inputManifest.manifest.solver_basis.solver_mode).toBe(mode);
    expect(source.numerical_quality!.status).not.toBe("checks_passed");
    expect(hasNativeMechanicsInvocation(source,model,mode)).toBe(true);
    expect(numericalResultStanding(source,model).eligible).toBe(true);
    expect(numericalResultStanding(structuredClone(source),model).eligible).toBe(false);
    for (const row of source.results.filter(r=>r.kind==='pipe_elastic_normal_stress_maximum_v2')) {
      const signature=row.metadata?.basis==='retained_source_endpoint_normal_max_v1'?'physics-source-endpoint-normal-maximum':'supported-source-068';
      expect(resultSemantics(row,source)?.signature_id).toBe(signature);
      expect(analysisRowSemantics(row,source).semantic?.signature_id).toBe(signature);
      expect(analysisRun.analysis_run.result_refs.find(r=>r.result_ref.ref===row.id)?.semantic_contract?.signature_id).toBe(signature);
    }
    expect(hook.result.current.results.currentSolvedResult).toBe(source);
    const document=await buildCurrentResultExport({model,result:source,analysisRun,inputManifest});
    expect(document.result_envelope.contract_evidence).toEqual(source.contract_evidence);
    expect(document.result_envelope.source_block_recovery).toEqual(source.source_block_recovery);
    await validateResultDocument(document,source);
    const packet=await buildStressNeutralExportPacket({model,result:source,analysisRun});
    expect(packet.export_profile.csv_encoding).toBe('utf-8');
    expect(packet.export_profile.csv_row_order).toBe('unicode_scalar_value_result_id');
    expect(packet.manifest.checksums.find((c:any)=>c.payload_ref.ref==='stress_neutral_results.csv').canonicalization).toBe('utf8_csv_record_lf_v1');
    for(const row of source.results.filter(r=>['support_reaction_component_v2','support_reaction_force_magnitude_v2','support_reaction_moment_magnitude_v2'].includes(r.kind))) {
      expect(packet.result_rows.find((r:any)=>r.result_id===row.id).component_ref).toEqual({object_type:'Support',ref:row.entity_ref});
    }
    await validateStressNeutralExportPacket(packet,source,analysisRun,analysisRun.analysis_run.load_basis_refs);
    const wire=JSON.parse(await canonicalJsonCheckedV1(packet));
    await validateStressNeutralExportPacket(wire,source,analysisRun,analysisRun.analysis_run.load_basis_refs);
    expect(wire.source_annotations).toHaveLength(source.results.length);
    wire.source_annotations.forEach((a:any,index:number)=>{
      expect(a.source_result_id).toBe(source.results[index].id);
      expect(a.source_value_bits).toBe(valueBits(source.results[index].value));
      if(a.source_row.value===0)expect(Object.is(a.source_row.value,-0)).toBe(false);
    });
    await expect(buildReportPackageRequest({model,result:source,analysisRun,inputManifest} as any)).rejects.toThrow("REPORT-PACKAGE-SOURCE-BLOCKS-UNAVAILABLE");
    // Mode binding. The session only publishes a manifest recording the mode it
    // actually requested, so the other mode is probed through the gate's own
    // predicates and through a session solve that receives these bytes, which
    // were recorded under `mode`, for the other mode: neither becomes Current.
    const otherMode=mode==="sparse_interactive"?"dense_scrutiny":"sparse_interactive";
    expect(hasNativeMechanicsInvocation(source,model,otherMode)).toBe(false);
    expect(physicsSourceModeMatches(source,model,otherMode)).toBe(false);
    act(()=>hook.result.current.results.setSolverMode(otherMode));
    await act(async()=>{await hook.result.current.results.handleRun();});
    expect(args.started).toEqual([mode,otherMode]);
    expect(hook.result.current.results.result).toBeNull();
    expect(hook.result.current.results.currentSolvedResult).toBeNull();
    expect(hook.result.current.results.solveJob.state).toBe("failed");
    expect(hook.result.current.results.solveJob.events.at(-1)?.message).toContain("SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED");
    hook.unmount();
    expect(source).toEqual(args.original);
    if(process.env.PHYSICS_SOURCE_TS_OUTPUT_DIR){
      const folder=process.env.PHYSICS_SOURCE_TS_OUTPUT_DIR;mkdirSync(folder,{recursive:true});const stem=`${name}-${mode}`;
      writeFileSync(path.join(folder,`${stem}.raw.json`),args.sourceText);
      for(const[suffix,value]of Object.entries({model,analysis:analysisRun,document,packet}))writeFileSync(path.join(folder,`${stem}.${suffix}.json`),await canonicalJsonCheckedV1(value));
    }
  },30000);
});
it('rejects rehashed analysis signatures that substitute ordinary and retained maxima',async()=>{
  const {source,analysisRun}=await received('mixed','sparse_interactive');
  for(const basis of ['retained_source_endpoint_normal_max_v1','recovered_from_open_mechanics_stress_components']) {
    const row=source.results.find(r=>r.kind==='pipe_elastic_normal_stress_maximum_v2'&&r.metadata?.basis===basis)!;
    expect(row).toBeDefined();
    const changed=structuredClone(analysisRun),reference=changed.analysis_run.result_refs.find(r=>r.result_ref.ref===row.id)!;
    reference.semantic_contract!.signature_id=basis==='retained_source_endpoint_normal_max_v1'?'supported-source-068':'physics-source-endpoint-normal-maximum';
    changed.analysis_run.hashes.find(h=>h.payload_scope==='analysis_run_record')!.value=await canonicalSha256HexCheckedV1(analysisRecordProjection(changed));
    await expect(validateAnalysisRunV03(changed,source)).rejects.toThrow('ANALYSIS_ROW_SOURCE_BINDING_MISMATCH');
    for(const invalid of [undefined,'unrecognized_basis']) {
      const copied=structuredClone(row);copied.metadata!.basis=invalid as any;
      expect(()=>analysisRowSemantics(copied,source)).toThrow('SOURCE_BASIS_CONTRADICTION');
      expect(()=>resultSemantics(copied,source)).toThrow('SOURCE_BASIS_CONTRADICTION');
    }
  }
});
it('preserves UTF8 quoted identifier code points and refuses invalid encoding or record separators', async () => {
  const {source,model,analysisRun}=await received('n05_unicode','sparse_interactive');
  const packet=await buildStressNeutralExportPacket({model,result:source,analysisRun});
  const columns=packet.export_profile.csv_columns as string[];
  const special='case:"温,\r\n度":😀';
  const ids=[special,'\ue000','😀']; // Scalar order differs from UTF16 order at the last two IDs.
  const rows=ids.map(id=>({...packet.result_rows[0],result_id:id,canonical_ref:{object_type:'Result',ref:id}}));
  const csv=[columns.join(','),...rows.map(row=>columns.map(key=>{
    const value=(row as any)[key],text=typeof value==='object'?value.ref:String(value);
    return /[",\r\n]/.test(text)?`"${text.replaceAll('"','""')}"`:text;
  }).join(','))].join('\n')+'\n';
  expect(csv).toContain('"case:""温,\r\n度"":😀"');
  expect(()=>validateStressNeutralCsv(csv,rows,true)).not.toThrow();
  expect(()=>validateStressNeutralCsv(csv,rows)).toThrow('SN-CSV-ROW-BINDING-MISMATCH');
  for(const bad of ['\ufeff'+csv,csv.replace('温','\ud800'),csv.replace(columns.join(',')+'\n',columns.join(',')+'\r\n')]) {
    expect(()=>validateStressNeutralCsv(bad,rows,true)).toThrow('SN-CSV-ROW-BINDING-MISMATCH');
  }
  expect(()=>validateStressNeutralCsv(csv,[rows[0],rows[2],rows[1]],true)).toThrow('SN-CSV-ROW-BINDING-MISMATCH');
  for(const mutate of [
    (p:any)=>{p.export_profile.csv_encoding='ascii';},
    (p:any)=>{p.export_profile.csv_row_order='locale';},
    (p:any)=>{p.manifest.checksums.find((c:any)=>c.payload_ref.ref==='stress_neutral_results.csv').canonicalization='normalized_ascii_lf_text';},
  ]) {
    const bad=structuredClone(packet);mutate(bad);
    await expect(validateStressNeutralExportPacket(bad)).rejects.toThrow(/SN-CSV-ENCODING-PROFILE-MISMATCH|SN-CHECKSUM-METADATA-MISMATCH/);
  }
});
it("preserves signed-zero sidecars and rejects a coherently rehashed false sign against original source",async()=>{
  const {source,model,analysisRun}=await received("mixed","sparse_interactive");
  const packet=await buildStressNeutralExportPacket({model,result:source,analysisRun});
  const changed=JSON.parse(await canonicalJsonCheckedV1(packet));
  const annotation=changed.source_annotations.find((a:any)=>a.source_value_bits==="8000000000000000");expect(annotation).toBeDefined();
  annotation.source_value_bits="0000000000000000";
  // Exact original-source comparison must reject before any package self-hash
  // can present the changed sign as authentic. The raw producer is untouched.
  await expect(validateStressNeutralExportPacket(changed,source,analysisRun)).rejects.toThrow("SN-SOURCE-ANNOTATION-BINDING-MISMATCH");
  const before=await canonicalSha256HexCheckedV1(source);expect(before).toBe(await canonicalSha256HexCheckedV1(JSON.parse(fixtureSources[`${fixturePrefix}mixed-sparse_interactive.raw.json`])));
});
