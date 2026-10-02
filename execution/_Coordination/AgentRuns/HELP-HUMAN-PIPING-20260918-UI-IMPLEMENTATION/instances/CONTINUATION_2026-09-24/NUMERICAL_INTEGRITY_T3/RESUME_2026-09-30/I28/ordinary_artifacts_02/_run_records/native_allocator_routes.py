import pathlib,json,re,subprocess,hashlib,datetime
q=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/scratch/i28-ordinary-artifacts-02")
rr=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I28/ordinary_artifacts_02")/'_run_records'
rows=json.loads((rr/'NATIVE_COMMANDS.json').read_text())
for ident,starts in {'h':{'allocator_alloc':'100003e78','allocator_dealloc':'100003f90','allocator_realloc':'100003fc0'},'vr':{'allocator_alloc':'10000c814','allocator_dealloc':'10000c91c','allocator_realloc':'10000c94c'}}.items():
 binary=q/('target-h/aarch64-apple-darwin/release/k6_observe' if ident=='h' else 'target-vr/aarch64-apple-darwin/release/examples/vk_scale')
 sym=[]
 for line in (rr/(ident+'-symbols.stdout')).read_text().splitlines():
  m=re.match(r'^([0-9a-f]+)\s+[tT]\s+(.*)$',line)
  if m: sym.append((int(m.group(1),16),m.group(2)))
 adds=sorted(set(a for a,n in sym))
 for name,addr in starts.items():
  start=int(addr,16);stop=adds[adds.index(start)+1];assert stop-start<=512
  argv=['/usr/bin/objdump','--disassemble','--demangle','--start-address='+hex(start),'--stop-address='+hex(stop),str(binary)]
  t=datetime.datetime.now(datetime.timezone.utc).isoformat();p=subprocess.run(argv,capture_output=True)
  stem=ident+'_'+name
  (rr/(stem+'.stdout')).write_bytes(p.stdout);(rr/(stem+'.stderr')).write_bytes(p.stderr)
  rows.append({'label':stem,'argv':argv,'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'start':hex(start),'stop':hex(stop),'range_bytes':stop-start,'symbol_names':[n for a,n in sym if a==start],'start_utc':t,'end_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit':p.returncode,'stdout_file':stem+'.stdout','stderr_file':stem+'.stderr','stdout_sha256':hashlib.sha256(p.stdout).hexdigest(),'stderr_sha256':hashlib.sha256(p.stderr).hexdigest()})
  (rr/'NATIVE_COMMANDS.json').write_text(json.dumps(rows,indent=2)+'\n')
  assert p.returncode==0
  print(p.stdout.decode())

