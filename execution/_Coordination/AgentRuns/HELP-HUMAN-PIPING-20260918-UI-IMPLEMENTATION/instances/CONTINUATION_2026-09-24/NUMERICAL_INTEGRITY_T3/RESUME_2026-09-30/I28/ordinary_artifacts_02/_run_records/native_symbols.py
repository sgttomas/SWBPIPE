import pathlib,subprocess,json,hashlib,datetime
q=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/scratch/i28-ordinary-artifacts-02")
rr=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I28/ordinary_artifacts_02")/'_run_records'
rows=[]
for label,sub in [('h','target-h/aarch64-apple-darwin/release/k6_observe'),('vr','target-vr/aarch64-apple-darwin/release/examples/vk_scale')]:
 binary=q/sub
 for tool,flags,suffix in [('/usr/bin/nm',['-n'],'symbols'),('/usr/bin/otool',['-L'],'dylibs')]:
  argv=[tool,*flags,str(binary)]; name=label+'-'+suffix
  t=datetime.datetime.now(datetime.timezone.utc).isoformat(); p=subprocess.run(argv,capture_output=True)
  (rr/(name+'.stdout')).write_bytes(p.stdout);(rr/(name+'.stderr')).write_bytes(p.stderr)
  rows.append({'label':name,'argv':argv,'start_utc':t,'end_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit':p.returncode,'stdout_file':name+'.stdout','stderr_file':name+'.stderr','stdout_sha256':hashlib.sha256(p.stdout).hexdigest(),'stderr_sha256':hashlib.sha256(p.stderr).hexdigest(),'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest()})
  assert p.returncode==0
(rr/'NATIVE_COMMANDS.json').write_text(json.dumps(rows,indent=2)+'\n')
for label in ['h','vr']:
 print(label, (rr/(label+'-dylibs.stdout')).read_text())
 print('Selected symbol names:')
 for line in (rr/(label+'-symbols.stdout')).read_text().splitlines():
  if any(s in line for s in ['OnceBox','set_current_info','StringError','ErrorImpl','VacantEntry','BoundedExtremeTracker','LeafNode','WriterFormatter']) or ('from_iter' in line and 'btree' in line) or ('Error' in line and ('3new' in line or '2io' in line)):
   print(line)

