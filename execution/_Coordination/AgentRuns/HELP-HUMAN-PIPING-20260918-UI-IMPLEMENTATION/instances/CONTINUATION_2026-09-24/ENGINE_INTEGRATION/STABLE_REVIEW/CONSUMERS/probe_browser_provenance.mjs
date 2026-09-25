// Independent Node probe: actual module code and current WASM, simulated IPC.
// The simulation is not native execution, Current qualification, or a solve.
import { registerHooks } from 'node:module';
import { readFileSync, writeFileSync, existsSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { pathToFileURL, fileURLToPath } from 'node:url';
import { createHash } from 'node:crypto';
import assert from 'node:assert/strict';
const here = dirname(fileURLToPath(import.meta.url));
let project = here;
while (!existsSync(join(project, 'apps/desktop/src/services/previewService.ts'))) project = dirname(project);
registerHooks({
  resolve(specifier, context, next) {
    if (specifier === '@tauri-apps/api/core') return { url:'probe:ipc', shortCircuit:true };
    try { return next(specifier, context); }
    catch (error) {
      if (specifier.startsWith('.') && !specifier.endsWith('.ts')) return next(specifier+'.ts', context);
      throw error;
    }
  },
  load(url, context, next) {
    if (url === 'probe:ipc') return { format:'module', source:'export const invoke = (...args) => globalThis.__reviewInvoke(...args);', shortCircuit:true };
    if (url.startsWith('file:') && url.endsWith('.json')) return { format:'module', source:`export default ${readFileSync(fileURLToPath(url),'utf8')};`, shortCircuit:true };
    return next(url, context);
  },
});
const load = relative => import(pathToFileURL(join(project,relative)).href);
const service = await load('apps/desktop/src/services/previewService.ts');
const { createNativeMechanicsReplay, nativeMechanicsReplayPair } = await load('apps/desktop/src/test/nativeMechanicsReplay.ts');
const { buildCurrentSessionInputManifest } = await load('apps/desktop/src/services/inputManifestService.ts');
const { buildCurrentResultExport, validateResultDocument } = await load('apps/desktop/src/features/result-export/resultExportAdapter.ts');
const { buildReportPackageRequest } = await load('apps/desktop/src/features/report/reportPackageRequest.ts');
const { runRuleChecks } = await load('apps/desktop/src/services/ruleCheckService.ts');
const outcomes = [];
globalThis.window = {};
const reference = await service.loadBundledMechanicsReference();
const referenceBefore = JSON.stringify(reference);
assert.equal(service.hasNativeMechanicsInvocation(reference.source, reference.model), false);
await assert.rejects(service.runPreviewMechanics(reference.model), /BROWSER_SOLVE_BACKEND_REQUIRED_REFERENCE_ONLY/);
await assert.rejects(service.startPreviewMechanicsJob(reference.model), /BROWSER_SOLVE_BACKEND_REQUIRED_REFERENCE_ONLY/);
outcomes.push({case:'bundled reference and fresh browser routes', result:'reference unregistered; solve and job refused'});
for (const profile of ['physics','precision']) for (const mode of ['sparse_interactive','dense_scrutiny']) {
  const pair = nativeMechanicsReplayPair(mode, {profile});
  const replay = createNativeMechanicsReplay({profile});
  globalThis.window.__TAURI_INTERNALS__ = {};
  globalThis.__reviewInvoke = replay.invoke;
  const result = await service.runPreviewMechanics(pair.model, mode);
  const original = JSON.stringify(result);
  assert.equal(service.hasNativeMechanicsInvocation(result,pair.model,mode),true);
  assert.equal(service.hasNativeMechanicsInvocation(structuredClone(result),pair.model,mode),false);
  assert.equal(service.hasNativeMechanicsInvocation(result,pair.model,mode==='sparse_interactive'?'dense_scrutiny':'sparse_interactive'),false);
  const changed = structuredClone(pair.model); changed.nodes[0].position.x += 0.1;
  assert.equal(service.hasNativeMechanicsInvocation(result,changed,mode),false);
  const retained = service.retainedNativeMechanicsInvocation(result,pair.model); retained.request.model.nodes[0].position.x += 1;
  assert.deepEqual(service.retainedNativeMechanicsInvocation(result,pair.model).request.model,pair.model);
  const inputManifest = await buildCurrentSessionInputManifest({model:pair.model,solver:{solver_name:result.producer.component_name,solver_version:result.producer.component_version,solver_build_ref:'probe:actual-captured-bytes-through-simulated-IPC',solver_mode:mode,settings:{}},active_rule_packs:[],external_assets:[]});
  const analysisRun = await service.buildAnalysisRunPreview(result,{inputManifest});
  const args = {model:pair.model,result,analysisRun,inputManifest};
  const document = await buildCurrentResultExport(args); await validateResultDocument(document,result);
  assert.equal(document.result_envelope.row_accounting.length,result.results.length);
  await assert.rejects(buildCurrentResultExport({...args,result:structuredClone(result)}), /CURRENT_NATIVE_INVOCATION_REQUIRED/);
  const reportArgs = {...args,projectSummary:null,comparison:null,ruleCheckAggregate:null};
  if (profile === 'physics') await assert.rejects(buildReportPackageRequest(reportArgs), /REPORT-PACKAGE-PHYSICS-PROJECTION-UNAVAILABLE/);
  else {
    assert.ok((await buildReportPackageRequest(reportArgs)).result_envelopes.length);
    await assert.rejects(buildReportPackageRequest({...reportArgs,result:structuredClone(result)}), /REPORT-PACKAGE-NATIVE-INVOCATION-REQUIRED/);
  }
  await assert.rejects(runRuleChecks({model:pair.model,solvedEnvelope:structuredClone(result),rulePackDocument:{}}), /RULE_NATIVE_INVOCATION_REQUIRED/);
  assert.equal(JSON.stringify(result),original);
  const start = await service.startPreviewMechanicsJob(pair.model,mode);
  const first = await service.pollPreviewMechanicsJob(start.job_id);
  assert.equal(service.hasNativeMechanicsInvocation(first.result,pair.model,mode),true);
  const second = await service.pollPreviewMechanicsJob(start.job_id);
  assert.equal(service.hasNativeMechanicsInvocation(second.result,pair.model,mode),false);
  const cancelled = await service.startPreviewMechanicsJob(pair.model,mode);
  await service.cancelPreviewMechanicsJob(cancelled.job_id,cancelled.backend_cancellation_token);
  const stopped = await service.pollPreviewMechanicsJob(cancelled.job_id);
  assert.equal(stopped.state,'cancelled'); assert.equal(service.hasNativeMechanicsInvocation(stopped.result,pair.model,mode),false);
  result.results[0].unit = 'mutated'; assert.equal(service.hasNativeMechanicsInvocation(result,pair.model,mode),false);
  outcomes.push({profile,mode,result:'private direct/job registration passes; clone/model/mode/mutation/replay/cancel refusals pass; canonical derivative source accounting passes',report_transport:profile==='physics'?'explicitly unavailable':'ordinary precision package passes'});
}
assert.equal(JSON.stringify(reference),referenceBefore);
const sha = relative => createHash('sha256').update(readFileSync(join(project,relative))).digest('hex');
const report = {scope:'Unit IPC simulation only using preserved actual producer pairs; no native UI or producer rerun.',runtime:process.version,wasm_sha256:sha('apps/desktop/public/wasm-engine/open_pipe_stress_operation_applier_bg.wasm'),outcomes};
writeFileSync(join(here,'BROWSER_INDEPENDENT_PROBES.json'),JSON.stringify(report,null,2)+'\n');
console.log(JSON.stringify(report,null,2));
