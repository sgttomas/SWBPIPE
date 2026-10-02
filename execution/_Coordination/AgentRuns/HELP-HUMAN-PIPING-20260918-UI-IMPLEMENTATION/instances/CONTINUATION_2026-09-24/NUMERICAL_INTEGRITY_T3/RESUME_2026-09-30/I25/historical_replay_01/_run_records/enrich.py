from inspect import *
import datetime,copy
files={'K6B':T3/'IMPLEMENTATION/K6B/_run_records/b3/records/records.jsonl','VK':T3/'IMPLEMENTATION/VK/_run_records/b/runs/records.jsonl','KF3':T3/'IMPLEMENTATION/KF3/_run_records/b/runs/records.jsonl'}
rows={(d,x['run_id']):x for d,p in files.items() for x in [json.loads(t)for t in read(p).splitlines()] if x.get('mode')=='w1a'}
for prefix in ['','ACTUAL_H_']:
 table=js(REC/(prefix+'REPLAY.json'))
 for x in table:
  r=rows[x['dataset'],x['run_id']]
  x['original_record_path']=str(files[x['dataset']]);x['original_stdout_sha256']=r['stdout_sha256']
  x['original_result']={k:copy.deepcopy(r.get(k))for k in ['classification','outcome','selected_precision','invocation_charged','summary','repeats_heap_peak','repeats_heap_peak_move','peak_rss_bytes','peak_footprint_bytes','source_commit','source_tree']}
  x['changed_admission_fields']={k:{'old':x['recorded_decision'].get(k),'new':x['new_as_recorded_history'].get(k)}for k in sorted(set(x['recorded_decision'])|set(x['new_as_recorded_history'])) if x['recorded_decision'].get(k)!=x['new_as_recorded_history'].get(k)}
 save(prefix+'REPLAY.json',table)
WT=pathlib.Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3')
raw_meta=js(WT/'scratch/i17/b_records/metadata.json');repo_meta=js(T3/'IMPLEMENTATION/VK/_run_records/b/runs/metadata.json');assert raw_meta==repo_meta
raw=read(WT/'scratch/i17/b_records/records.jsonl');old=read(T3/'IMPLEMENTATION/VK/_run_records/b/runs/records.jsonl');assert raw==old
log=read(WT/'scratch/i17/b_build_archive.log');(REC/'VK_ORIGINAL_build_archive.log').write_text(log)
read(WT/'scratch/i17/b_build_release.log')
read(T3/'IMPLEMENTATION/VK/_run_records/b/setup/binary.txt')
read(T3/'IMPLEMENTATION/VK/RETURN.md');read(T3/'IMPLEMENTATION/KF3/RETURN.md');read(T3/'IMPLEMENTATION/KF3/_run_records/b/runner_v3.log')
read(T3/'IMPLEMENTATION/K6B/_run_records/b3/run_w1t4.py.txt')
inputs=js(R/'metric_design_14_reference_callers/_run_records/CLI_EXTERNAL_INPUTS.json')
print('EXTERNAL_INPUT_KEYS',list(inputs)if isinstance(inputs,dict)else len(inputs))
print(str(inputs)[:2000])
save('VR_HISTORICAL_LAUNCH_GAPS.json',{'VK':{'raw_portable_records_equal':True,'metadata':raw_meta,'archive_build_log':str(WT/'scratch/i17/b_build_archive.log'),'compiled_source_path_evidence':'archive build log retains numerical_robustness compile directory; exact runtime argv0 still lost by basename normalization','external_model_directory':'WT/scratch/i17/b_models is documented by historical RETURN; caller retained actual --model-file token still needs a raw invocation record rather than inferred expansion','missing':['original full argv[0] token for normal launches','original full --model-file tokens for each of the 12 external normal launches','artifact-bound compiled manifest path and profile transfer confirmation']},'KF3':{'metadata':js(T3/'IMPLEMENTATION/KF3/_run_records/b/runs/metadata.json'),'pointer':'runner_v3.log has literal $W/kf3-b/vk_scale and truncates subsequent command','missing':['original full argv[0] token','original full --model-file tokens for each of the 12 external normal launches','artifact-bound compiled manifest path','complete ordinary-production profile transfer confirmation']},'disposition':'Exact reference-profile arithmetic and path-independent 10000 binary deferral are returned. No unqualified actual-historical VR total, peak-bound or admission attestation is asserted.'})
