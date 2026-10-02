import pathlib,json,hashlib,datetime
out=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I28/ordinary_artifacts_02");rr=out/'_run_records';q=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/scratch/i28-ordinary-artifacts-02")
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
for p in rr.glob('*.json'):json.loads(p.read_text())
for f in ['RETURN.md','TRANSFER.md']:assert '/Users/' not in (out/f).read_text()
art=json.loads((rr/'ARTIFACTS.json').read_text())
assert all(sha(v['path'])==v['sha256'] for v in art.values())
pre=json.loads((rr/'SOURCE_BEFORE.json').read_text());post=json.loads((rr/'SOURCE_AFTER.json').read_text())
assert pre==post and len(pre)==299
assert all(sha(x['path'])==x['sha256'] for x in post)
public=json.loads((rr/'PUBLIC_LAYOUT_EXECUTION.json').read_text())
assert len([x for x in public if x['label']=='public_layout_compile'])==1
assert len([x for x in public if x['label']=='public_layout_run'])==1
assert all(x['exit']==0 for x in public)
native=json.loads((rr/'NATIVE_COMMANDS.json').read_text())
assert len(native)==37 and all(x['exit']==0 for x in native)
assert all('libstd-' not in str(x['argv']) for x in native)
for x in native:
 assert sha(rr/x['stdout_file'])==x['stdout_sha256'] and sha(rr/x['stderr_file'])==x['stderr_sha256']
bind=json.loads((rr/'BUILD_INPUTS.json').read_text())
assert sha(q/'source.tar')==bind['archive']['sha256']
now=datetime.datetime.now(datetime.timezone.utc)
assert now<datetime.datetime.fromisoformat('2026-10-02T04:37:16+00:00')
v={'completed_utc':now.isoformat(),'hard_deadline_utc':'2026-10-02T04:42:16Z','new_work_cutoff_utc':'2026-10-02T04:37:16Z','all_work_completed_before_cutoff':True,'source_blob_matches':299,'source_preserved':True,'artifact_hashes_preserved':True,'ordinary_builds':2,'public_reporter_compiles':1,'public_reporter_runs':1,'native_commands':37,'bounded_disassembly_ranges':33,'native_code_bytes':31648,'production_binary_runs':0,'model_count_solver_measurement_runs':0,'no_follow_on':True,'status':'Artifact correspondence candidate ready for independent review; no complete runtime/input/launch/admission/measurement acceptance'}
(rr/'VERIFICATION.json').write_text(json.dumps(v,indent=2)+'\n')
files=[p for p in sorted(out.rglob('*')) if p.is_file() and p.name not in ['WRITE_INVENTORY.json','SHA256SUMS']]
inventory=[{'path':str(p.relative_to(out)),'bytes':p.stat().st_size,'sha256':sha(p)} for p in files]
(out/'WRITE_INVENTORY.json').write_text(json.dumps({'packet':'R/I28/ordinary_artifacts_02','files':inventory,'scratch_inventory_records':['_run_records/SOURCE_BEFORE.json','_run_records/SOURCE_AFTER.json','_run_records/TARGET_INVENTORIES.json','_run_records/ARTIFACTS.json','_run_records/BUILD_INPUTS.json'],'scratch_preserved':True},indent=2)+'\n')
files=[p for p in sorted(out.rglob('*')) if p.is_file() and p.name!='SHA256SUMS']
(out/'SHA256SUMS').write_text(''.join(sha(p)+'  '+str(p.relative_to(out))+'\n' for p in files))
for line in (out/'SHA256SUMS').read_text().splitlines():
 want,rel=line.split('  ',1);assert sha(out/rel)==want
print(json.dumps({'complete_utc':v['completed_utc'],'payloads':len(files),'packet_bytes':sum(p.stat().st_size for p in files),'seal':sha(out/'SHA256SUMS'),'return':str(out/'RETURN.md')}))

