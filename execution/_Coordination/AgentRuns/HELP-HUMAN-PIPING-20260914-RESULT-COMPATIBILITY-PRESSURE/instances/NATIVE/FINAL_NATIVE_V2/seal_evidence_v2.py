from pathlib import Path
import datetime,hashlib,json,sys
root=Path(sys.argv[1]).resolve()
assert root.name=='FINAL_NATIVE_V2' and 'instances/NATIVE' in str(root)
run=root.parents[2]
now=datetime.datetime.now(datetime.timezone.utc).isoformat()
build=json.loads((root/'_run_records/build/build_binding.json').read_text())
model_envelope=json.loads((root/'validation/after_first_save_model_envelope_hashes.json').read_text())
analysis=json.loads((root/'validation/after_first_save_analysis.json').read_text())
idempotence=json.loads((root/'validation/unchanged_second_save.json').read_text())
stress=json.loads((root/'findings/native_stress_delivery_failure.json').read_text())
process=json.loads((root/'_run_records/process/final_owned_process_scan.json').read_text())
assert model_envelope['status']=='PASS' and analysis['status']=='PASS' and idempotence['status']=='PASS'
assert stress['status']=='PRODUCT_DEFECT_CONFIRMED' and process['owned_processes_absent']
result={
 'schema':'final-native-result-v2','sealed_at':now,'status':'BLOCKED_BY_NATIVE_STRESS_DELIVERY_DIMENSION_CARRIER',
 'candidate_commit_declared':'e244a2479207fa32db23e150d6cb41544ddc829f','base_commit_declared':'eff9a58dd712ff9673fa26b9fa809a2f725f6f97',
 'brief':{'path':'projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/briefs/FINAL_NATIVE_EXECUTION_V2.md','sha256':'cb56bccdb09f15e6f645caeca84237aa791bddd3eee1090f28eb523c301beae9'},
 'release':{'path':'instances/ROOT/_run_records/FINAL_SOURCE_RELEASE_V1.json','sha256':'e09d75d4f7f4c2621497103ac5e6a383e6df335aea25e0a0b4d9f52e2423ce0b'},
 'lease':{'path':'instances/ROOT/_run_records/FINAL_NATIVE_LEASE_V2.json','sha256':'bbcc571c8c0be8840c2373a68677788466fc594f4f5f2edfb4293beb3e765596'},
 'source_binding':{'file_count':76,'before_build':'PASS','after_build':'PASS','after_gui':'PASS','after_gui_evidence':'_run_records/bindings/after_gui.json'},
 'build':{'status':'PASS','native_executable':build['native_executable'],'bundle':build['bundle'],'dist':{'tree_sha256':build['dist']['tree_sha256']},'wasm':build['wasm'],'checked_json':build['checked_json'],'runner':build['runner'],'runner_dependency_resolution':build['runner_dependency_resolution'],'maintained_locked_runner_build_claimed':False},
 'passing_observations':[
  'Fresh maintained beforeBuild native workflow completed against the exact frozen candidate; native executable, bound bundle, dist, WASM and checked JSON authority are hashed.',
  'The headless runner was built offline from an exact-commit temporary archive after preserving the maintained --locked failure; archive, generated temporary lockfile and runner are hashed, with no maintained --locked claim.',
  'First actual native create/solve/save established a Tauri backend Current run with MECHANICS_SOLVED, model identity match, and 830 rows.',
  'Normal quit and exact-bundle reopen restored Historical with 830/830 rows. HISTORICAL_INPUT_MANIFEST_MISSING was the only visible finding; no model, envelope, received-result, analysis-record or row checksum mismatch appeared.',
  'Independent rfc8785_jcs WASM recomputation matched the stored post-normalization model hash and the exact seven-field project-envelope hash.',
  'Independent checked JSON validation passed strict analysis_run 0.2, the full-record self-exclusion checksum, received-result checksum and all 830 row checksums.',
  'The migration ledger post-migration model hash matches the persisted normalized model hash; its create-time envelope evidence remains correctly scoped to the migration return.',
  'The unchanged second native save preserved all 11 project/model/result/analysis/hash/attachment payload fields byte-for-byte; only updated_at_unix advanced.',
  'A minimal recovery solve re-established Current solely to exercise actual native stress delivery, then the app quit normally and all owned app/build/browser/server processes were absent.'
 ],
 'first_save_integrity':{
  'status':'PASS','model_hash':model_envelope['model_hash']['stored_claim']['value'],'project_envelope_hash':model_envelope['project_envelope_hash']['stored_claim']['value'],
  'analysis_record_hash':analysis['full_record_self_exclusion_checksum']['independently_computed'],'received_result_hash':analysis['received_result_checksum']['independently_computed'],'row_count':analysis['result_row_checksums']['count'],'row_mismatches':analysis['result_row_checksums']['mismatches'],'historical_visible_findings':['HISTORICAL_INPUT_MANIFEST_MISSING'],'unchanged_second_save':idempotence['status']
 },
 'blocker':{
  'finding_id':'NATIVE-STRESS-DELIVERY-001','evidence':'findings/native_stress_delivery_failure.json','summary':'The actual native 830-row ResultItem carrier contains no dimension field on any row. Stress-neutral construction emits zero preservation witnesses, then validation conflates all 830 unwitnessed rows with the two diagnostic-work rows and throws SN-DIAGNOSTIC-WORK-FINDING-MISMATCH. The component catch clears the packet and displays the misleading no-result state despite a completed solve.',
  'error_code':'SN-DIAGNOSTIC-WORK-FINDING-MISMATCH','native_rows':830,'native_rows_with_dimension':0,'expected_eligible_witnesses':828,'actual_witnesses_before_throw':0,'diagnostic_work_rows':stress['actual_native_result']['diagnostic_work_row_ids'],'repair_attempted':False,
  'acceptance_effect':'Required actual native stress package delivery, 828 eligible witnesses, two explicit diagnostic-work withholds, delivered bytes/receipt and downstream package verification cannot be established on this candidate.'
 },
 'not_executed_after_bounded_cut':[
  'seven producer captures and all-60 constructor composition','sparse/dense native and browser distribution delivery','nine-member Python materialization','3.2 m GUI-authored 350 N to 500 N lifecycle','legacy saved-fixture backcheck'
 ],
 'additional_review_finding':{'status':'NOT_RUNTIME_EXERCISED','summary':'Fresh review separately identified malformed scalar/array HistoricalRunContext crash risk; no malformed record was injected in this bounded execution.'},
 'process':{'final_normal_quit':True,'final_candidate_process_gone':True,'owned_processes_absent':True,'owned_browser_or_server_launched':False,'evidence':'_run_records/process/final_owned_process_scan.json'},
 'store':{'isolated_path':'/Users/ryan/Library/Application Support/org.openpipestress.foundation-final-20260915/openpipestress-projects.sqlite3','synthetic_only':True,'snapshots':['_run_records/store/after_first_save.sqlite3','_run_records/store/after_unchanged_save_v2.sqlite3']},
 'effects':{'product_source_modified':False,'governance_modified':False,'git_modified':False,'network_action':False,'private_or_user_model_used':False,'browser_launched':False}
}
(root/'FINAL_NATIVE_RESULT_V2.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
release={
 'schema':'native-build-gui-lease-release-v2','released_at':now,'lease':'exclusive native/WASM/desktop build and native/browser GUI surfaces','state':'RELEASED','result_status':result['status'],'final_candidate_process_gone':True,'owned_processes_absent':True,'final_process_evidence':'_run_records/process/final_owned_process_scan.json','bound_application_preserved':str(Path(build['bundle']['root'])),'statement':'No further build, browser or GUI action will be performed for this candidate. Root may acquire the native/build/browser/GUI lease for repair and successor backcheck.'
}
(root/'LEASE_RELEASE.json').write_text(json.dumps(release,indent=2,sort_keys=True)+'\n')
excluded={'EVIDENCE_MANIFEST.json','SEALED_RESULT.json'}
files=[]
for path in sorted(root.rglob('*')):
 if not path.is_file() or path.name in excluded: continue
 data=path.read_bytes(); files.append({'path':str(path.relative_to(root)),'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()})
lines=''.join(f'{x["path"]}\0{x["bytes"]}\0{x["sha256"]}\n' for x in files).encode()
manifest={'schema':'final-native-evidence-manifest-v2','created_at':now,'file_count':len(files),'tree_sha256':hashlib.sha256(lines).hexdigest(),'excludes':sorted(excluded),'files':files}
(root/'EVIDENCE_MANIFEST.json').write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n')
seal={'schema':'final-native-seal-v2','sealed_at':now,'status':result['status'],'candidate_commit_declared':result['candidate_commit_declared'],'result_sha256':hashlib.sha256((root/'FINAL_NATIVE_RESULT_V2.json').read_bytes()).hexdigest(),'lease_release_sha256':hashlib.sha256((root/'LEASE_RELEASE.json').read_bytes()).hexdigest(),'evidence_manifest_sha256':hashlib.sha256((root/'EVIDENCE_MANIFEST.json').read_bytes()).hexdigest(),'evidence_tree_sha256':manifest['tree_sha256'],'blocker_sha256':hashlib.sha256((root/'findings/native_stress_delivery_failure.json').read_bytes()).hexdigest(),'lease_released':True}
(root/'SEALED_RESULT.json').write_text(json.dumps(seal,indent=2,sort_keys=True)+'\n')
print(json.dumps(seal,sort_keys=True))
