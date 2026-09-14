from pathlib import Path
import json,hashlib,difflib,collections,os
root=Path.cwd();records=root/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260913-RESULTS-ENGINEERING-3D/instances/RESULTS/_run_records';lane=records/'TASK_WRITER_IMPLEMENTATION_V1';integration=Path('/Users/ryan/.codex/worktrees/8728/chirality-results-engineering-3d-20260913/projects/chirality-piping')
allowed=set('''schemas/results.schema.yaml
schemas/results.v0.1.schema.yaml
schemas/results.v0.2.schema.yaml
fixtures/results/semantic_contract_v0_2.json
fixtures/results/invented/result_export_v0_2.json
fixtures/results/invented/result_export_v0_2_rejections.json
core/reporting/result_export/Cargo.toml
core/reporting/result_export/src/lib.rs
core/reporting/result_export/src/semantic_contract.rs
core/reporting/result_export/src/derivative.rs
core/reporting/result_export/tests/derivative_contract.rs
core/runner/headless/src/lib.rs
core/runner/headless/src/result_envelope_binding.rs
core/runner/headless/src/bin/openpipestress-runner.rs
apps/desktop/src/App.tsx
apps/desktop/src/features/results/ResultsPanel.tsx
apps/desktop/src/features/results/resultInterpretation.ts
apps/desktop/src/features/results/resultSemantics.ts
apps/desktop/src/features/results/resultSemantics.test.ts
apps/desktop/src/features/results/ResultsPanel.test.tsx
apps/desktop/src/features/result-export/ResultExportPanel.tsx
apps/desktop/src/features/result-export/resultExportAdapter.ts
apps/desktop/src/features/result-export/resultExportAdapter.test.ts
apps/desktop/src/features/result-export/ResultExportPanel.test.tsx
apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.tsx
apps/desktop/src/features/headless-runner/HeadlessRunnerPanel.test.tsx
apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx
apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.test.tsx
apps/desktop/src/features/handoff/HandoffPanel.tsx
apps/desktop/src/features/handoff/HandoffPanel.test.tsx
apps/desktop/src/features/report/reportPackageRequest.ts
apps/desktop/src/features/report/reportPackageRequest.test.ts
apps/desktop/src/features/redaction-controls/ControlledExportLink.tsx
apps/desktop/src/features/redaction-controls/ControlledExportLink.test.tsx
 tests/test_results_schema.py
 tests/test_result_export_v0_2.py'''.replace('\n ','\n').splitlines())
def pin(p):return {'path':str(p.relative_to(root)) if p.is_relative_to(root) else str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'bytes':p.stat().st_size}
def save(name,value):(lane/name).write_text(json.dumps(value,indent=2)+'\n')
changed=[];locks=[];patch=[]
for prefix in ['core','apps/desktop/src','fixtures','schemas','tests','tools','validation']:
 for p in sorted((root/prefix).rglob('*')):
  if not p.is_file() or any(part in ['target','node_modules','__pycache__'] for part in p.relative_to(root).parts):continue
  rel=str(p.relative_to(root));old=integration/rel
  if p.name=='Cargo.lock':locks.append(pin(p));continue
  if not old.exists() or p.read_bytes()!=old.read_bytes():
   assert rel in allowed,('OUTSIDE_FENCE',rel);item=pin(p);item.update({'original_sha256':hashlib.sha256(old.read_bytes()).hexdigest() if old.exists() else None,'original_bytes':old.stat().st_size if old.exists() else 0,'original_comparison_origin':str(old)});changed.append(item);patch.extend(difflib.unified_diff(old.read_text().splitlines(keepends=True) if old.exists() else [],p.read_text().splitlines(keepends=True),fromfile='a/'+rel if old.exists() else '/dev/null',tofile='b/'+rel))
assert len(changed)==len(allowed),(len(changed),len(allowed))
oldapp=(integration/'apps/desktop/src/App.tsx').read_text();assert (root/'apps/desktop/src/App.tsx').read_text()==oldapp.replace('<ResultExportPanel model={model} result={currentSolvedResult} analysisRun={analysisRun} />','<ResultExportPanel model={model} result={currentSolvedResult} analysisRun={analysisRun} inputManifest={inputManifest} />')
(lane/'SOURCE_TRANSFER_V1.patch').write_text(''.join(patch));save('SOURCE_FENCE_AND_HASH_AUDIT_V1.json',{'source_basis':'8f27fa3d8ec5e128e61fd3ac4076e74d7955f355','comparison_origin':str(integration),'integration_refresh_basis':'12f5b1a5adeb151b1762a7972507c370ac8a1ff7; parent/root verified no Piping source delta; own8f27 not rebased','changed_paths':changed,'changed_count':len(changed),'all_changed_paths_in_fence':True,'all_changed_product_source_sha256_coverage':True,'app_only_accepted_prop':True,'protected_solver_source':pin(root/'core/product_physics/src/lib.rs'),'runtime_generated_cargo_locks_not_source_patch':locks,'patch':pin(lane/'SOURCE_TRANSFER_V1.patch'),'note':'No Git operation; difflib transfer against qualified unchanged Piping comparison origin.'})
# Exact final consumer statuses, with the original74 inventory retained upstream.
original=json.loads((records/'TASK_WRITER/DETAILED_CONSUMER_VERSION_DISPOSITIONS_V1.json').read_text());consumer=[]
for entry in original['records']:
 p=root/entry['path'];row=dict(entry);row['original_sha256']=row.pop('sha256');row['current_sha256']=hashlib.sha256(p.read_bytes()).hexdigest();row['source_changed']=row['current_sha256']!=row['original_sha256'];row['implementation_disposition']='localized accepted repair' if row['source_changed'] else 'exact source preserved; existing consumer/read-only or legacy test disposition retained';row['canonical_version_adoption']='separate qualified0.2 derivative only; original raw/legacy0.1 readers remain unchanged'
 if 'HistoricalRunContext' in row['path'] or 'previewService' in row['path'] or row['path'].endswith('hashService.ts'):row['implementation_disposition']='immutable existing legacy enrichment/localeCompare or Historical wasm behavior preserved independently; no current-record migration'
 if 'StressNeutralExportPanel' in row['path']:row['implementation_disposition']='shared independent semantics; all numerical rows/units/CSV preserved; incompatible/unavailable declaration witnesses withheld; strict new blocking diagnostic channels; existing noncanonical preview/schema911 gaps root-owned'
 if 'HandoffPanel' in row['path'] or 'HeadlessRunnerPanel' in row['path']:row['implementation_disposition']='reference-only received declared dimensions preserved without authentic origin/physical interpretation claim; absent declarations withhold witnesses and existing diagnostics/loss disclosure'
 if 'reportPackageRequest' in row['path']:row['implementation_disposition']='physical-only exact shared semantics; supported legacy metadata only; review/diagnostic/missing required metadata rows retained as source refs in existing diagnostic channels; old run hashes unchanged'
 if 'ResultExportPanel' in row['path']:row['implementation_disposition']='Current exact manifest/run/result proof plus0.2 derivative; initially blocked local link; existing policy/own intent/lossless/exact equality exposes original href only; no canonical persistence migration'
 consumer.append(row)
save('FINAL_CONSUMER_DISPOSITIONS_V1.json',{'original_inventory':pin(records/'TASK_WRITER/DETAILED_CONSUMER_VERSION_DISPOSITIONS_V1.json'),'complete_original_records':len(consumer),'records':consumer,'new_source_files':[item for item in changed if item['original_sha256'] is None],'source_only_supported_variants':10,'legacy_current_creation_gate':'47 equal,4 accepted declared-dimension differences retained independently,9 existing legacy creation throws; complete private model-aware headless proof separate; no blanket all60 Current/native acceptance','future_legacy_current_record_decision':'DEL14 current-record migration remains root-owned; immutable binder/functions/schema0.1 independently preserved','LocalFea':'read-only; existing referenced source shape retained; no new0.2 transport or readiness obligation'})
# Immutable upstream revalidation.
selected=json.loads((records/'CONSOLIDATED_CANDIDATE_INDEX_V4.json').read_text());selected_checks=[]
for name,item in selected['selected_artifacts'].items():p=Path(item['path']);assert hashlib.sha256(p.read_bytes()).hexdigest()==item['sha256'];selected_checks.append({'name':name,**pin(p)})
oldmanifest=json.loads((records/'TASK_WRITER/MANIFEST_FINAL_PHASE1.json').read_text());oldchecks=[]
for item in oldmanifest['files']:
 p=records/'TASK_WRITER'/item['path'];assert hashlib.sha256(p.read_bytes()).hexdigest()==item['sha256'];oldchecks.append(pin(p))
save('IMMUTABLE_UPSTREAM_FAN_IN_V1.json',{'selected28':selected_checks,'original97':oldchecks,'all_selected_and_historical_bytes_preserved':True})
# Exact ledger preserves each failed execution and subsequent repair.
ledger=[]
for p in sorted((lane/'attempts').glob('*/COMMAND.json')):
 command=json.loads(p.read_text());result=json.loads((p.parent/'RESULT.json').read_text());ledger.append({'attempt':p.parent.name,'command':command,'result':result})
save('COMMAND_LEDGER_V1.json',{'allocation':'ROOT_LEASE_RESULTS_IMPLEMENTATION_01','execution':'exclusive sequential focused writer runtime; parent runtime idle; complete after042','attempts':ledger,'actual_failure_attempts':[x['attempt'] for x in ledger if x['result']['exit_code']!=0],'early_input_pin_scope':'001-033 command input hashes initially focused core/fixtures/schema subset;034 onward includes all desktop source/tests plus direct argv files. Final source audit pins100 percent changed product sources; each actual log/exit/source subset preserved.'})
# Complete physical inventory, with dependencies/runtime scratch classified as ignored artifacts.
ignored=[]
for label,base in [('node_modules',root/'node_modules'),('desktop_node_modules',root/'apps/desktop/node_modules'),('wasm_engine_generated',root/'apps/desktop/public/wasm-engine'),('self_weight_generated',root/'apps/desktop/public/self-weight-engine'),('external_runtime_target',Path('/tmp/chirality-results-engineering-3d-20260913/results-implementation-target'))]:
 files=[]
 if base.exists():
  for current_dir,dirs,names in os.walk(base,followlinks=False):
   for name in names:
    p=Path(current_dir)/name
    if p.is_symlink():files.append({'path':str(p),'symlink_target':os.readlink(p),'bytes':p.lstat().st_size})
    else:files.append({'path':str(p),'bytes':p.stat().st_size})
 ignored.append({'class':label,'root':str(base),'file_count':len(files),'bytes':sum(x['bytes'] for x in files),'files':files,'authority':'cached dependency/generated runtime artifacts; excluded from product source patch'})
save('IGNORED_PHYSICAL_INVENTORY_V1.json',{'groups':ignored,'runtime_cargo_locks':locks,'no_build_target_under_AgentRuns':True,'all_dependency_execution_offline':'cargo flags/env and wasm script logs captured','source_patch_excludes_dependency/generated/scratch_artifacts':True})
save('EVIDENCE_PREPARATION_ERRORS_V1.json',{'sealing_probe':'First inline audit failed at App literal matcher using currentResult/currentAnalysisRun names; actual accepted caller is currentSolvedResult/analysisRun. Corrected evidence assertion only; no product source mutation. Earlier changed-set/fence count assertion passed.','earlier':'PREPARATION_ERROR_01 missing authorized new tests directory and FAILED_LOCATION_PROBES_V1 preserved.'})
print('SEALED_AUDIT',len(changed),'FILES',len(ledger),'COMMANDS','SELECTED',len(selected_checks),'ORIGINAL',len(oldchecks))
