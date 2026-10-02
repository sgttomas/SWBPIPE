import pathlib,json,re,subprocess,datetime,hashlib
q=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/scratch/i28-ordinary-artifacts-02")
rr=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I28/ordinary_artifacts_02")/'_run_records'
sel={
'h':{'mutex':'1001d5de0','registry':'1001b6c70','geometry_root':'1001543cc','geometry_split':'100158090','tracker_root':'100153440','tracker_split':'100156fa0','holding_split':'100157a70'},
'vr':{'mutex':'1001d44a0','registry':'1001b3f7c','geometry_root':'1001526a8','geometry_split':'10015636c','tracker_root':'100151f2c','tracker_split':'10015527c','holding_split':'100155d4c','json_root':'10006b018','publication_split':'100014b10','allowance_split':'10001549c','pair_set_split':'1000162bc','usize_set_split':'1000168e0','case_bulk':'10002e2f0','floor_bulk':'10002e440','controls_bulk':'10002e580','io_new_str':'1001966c4','json_error_io':'1001ce400','writer_formatter':'100067bdc','compact_escaped':'100067e00','compact_value_serialize':'100067f4c'}}
records=json.loads((rr/'NATIVE_COMMANDS.json').read_text())
picked=[]
for binary_id,targets in sel.items():
 binary=q/('target-h/aarch64-apple-darwin/release/k6_observe' if binary_id=='h' else 'target-vr/aarch64-apple-darwin/release/examples/vk_scale')
 symbols=[]
 for line in (rr/(binary_id+'-symbols.stdout')).read_text().splitlines():
  m=re.match(r'^([0-9a-f]+)\s+[tT]\s+(.*)$',line)
  if m: symbols.append((int(m.group(1),16),m.group(2)))
 addresses=sorted(set(a for a,n in symbols))
 for label,startstr in targets.items():
  start=int(startstr,16); assert start in addresses
  stop=addresses[addresses.index(start)+1]; assert 0<stop-start<=4096,(label,stop-start)
  names=[n for a,n in symbols if a==start]
  argv=['/usr/bin/objdump','--disassemble','--demangle','--start-address='+hex(start),'--stop-address='+hex(stop),str(binary)]
  t=datetime.datetime.now(datetime.timezone.utc).isoformat(); p=subprocess.run(argv,capture_output=True)
  stem=binary_id+'_'+label
  (rr/(stem+'.stdout')).write_bytes(p.stdout);(rr/(stem+'.stderr')).write_bytes(p.stderr)
  row={'label':stem,'binary_id':binary_id,'argv':argv,'symbol_names':names,'start':hex(start),'stop':hex(stop),'range_bytes':stop-start,'start_utc':t,'end_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit':p.returncode,'stdout_file':stem+'.stdout','stderr_file':stem+'.stderr','stdout_sha256':hashlib.sha256(p.stdout).hexdigest(),'stderr_sha256':hashlib.sha256(p.stderr).hexdigest(),'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest()}
  records.append(row);picked.append(row)
  (rr/'NATIVE_COMMANDS.json').write_text(json.dumps(records,indent=2)+'\n')
  assert p.returncode==0,(stem,p.stderr)
  text=p.stdout.decode(); lines=text.splitlines()
  print(stem,hex(start),hex(stop),stop-start,'bytes')
  print('\n'.join(lines[:5]))
  for i,line in enumerate(lines):
   if 'bl' in line and ('__rust_alloc' in line or '__rustc' in line): print('\n'.join(lines[max(0,i-4):i+4]))
(rr/'SELECTED_RANGES.json').write_text(json.dumps(picked,indent=2)+'\n')

