import pathlib,json,shlex,subprocess,datetime,hashlib,os
q=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/scratch/i28-ordinary-artifacts-02")
r=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30")
rr=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I28/ordinary_artifacts_02")/'_run_records'
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def meta(p): return {'path':str(p),'sha256':sha(p),'bytes':pathlib.Path(p).stat().st_size}
def dump(n,d): (rr/n).write_text(json.dumps(d,indent=2)+'\n')
rows=[]
for line in (q/'logs/build-vr.log').read_text().splitlines():
 if 'Running `' in line: rows.append(shlex.split(line.split('Running `',1)[1].rsplit('`',1)[0]))
comp=[a for a in rows if '--crate-name' in a]
prod=[a for a in comp if a[a.index('--crate-name')+1] in ['piping_numerical_robustness','open_pipe_stress_frame_kernel','open_pipe_stress_solver_performance_harness','vk_scale']]
assert len(prod)==4 and all('--cfg' not in a and '--test' not in a for a in prod)
assert all('--test' not in a for a in comp)
fp=[{'path':str(p),'sha256':sha(p),'value':json.loads(p.read_text())} for p in (q/'target-vr').glob('**/.fingerprint/*/*.json')]
for x in fp:
 if any(s in x['path'] for s in ['piping_numerical_robustness','open_pipe_stress_frame_kernel','open_pipe_stress_solver_performance_harness']):
  if 'features' in x['value']: assert x['value']['features']=='[]' and x['value']['rustflags']==[]
a=next(a for a in comp if a[a.index('--crate-name')+1]=='vk_scale')
externs={a[i+1].split('=',1)[0]:a[i+1].split('=',1)[1] for i,x in enumerate(a) if x=='--extern'}
vr=externs['piping_numerical_robustness']; fk=externs['open_pipe_stress_frame_kernel']
dump('BUILD_VR_CHECK.json',{'exit':0,'start_observed_utc':'2026-10-02T03:59:05Z','completion_observed_utc':'2026-10-02T03:59:24Z','all_verbose_running_argv':rows,'fingerprints':fp,'final_externs':externs})
commands=[]
def run(name,argv,env=None):
 t0=datetime.datetime.now(datetime.timezone.utc).isoformat()
 p=subprocess.run(argv,cwd=q/'source',env=env,capture_output=True)
 (rr/(name+'.stdout')).write_bytes(p.stdout); (rr/(name+'.stderr')).write_bytes(p.stderr)
 commands.append({'label':name,'argv':argv,'cwd':str(q/'source'),'start_utc':t0,'end_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit':p.returncode,'stdout':meta(rr/(name+'.stdout')),'stderr':meta(rr/(name+'.stderr'))})
 dump('PUBLIC_LAYOUT_EXECUTION.json',commands)
 return p
guard=run('reporter_guard',['ps','-p','5387','-o','pid=,command='])
assert guard.returncode==0 and b'memguard.sh' in guard.stdout
src=r/'verification/public_layout_20/_run_records/public_layout.rs'
assert sha(src)=='7f4e0aa8a7a05f50bd325f015a81c1f70c291355ec09193010877f386a601dc6'
rust=pathlib.Path('/Users/ryan/.rustup/toolchains/1.97.1-aarch64-apple-darwin/bin/rustc')
env=dict(os.environ)
for k in ['FK_SEEDED_FAULT','RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','RUSTC_BOOTSTRAP']: env.pop(k,None)
env.update(RUSTUP_TOOLCHAIN='1.97.1',RUSTUP_AUTO_INSTALL='0')
argv=[str(rust),'--edition=2021','--crate-name','t3_public_layout_20','--target','aarch64-apple-darwin','-C','opt-level=3','--extern','piping_numerical_robustness='+vr,'--extern','open_pipe_stress_frame_kernel='+fk,'-L','dependency='+str(q/'target-vr/aarch64-apple-darwin/release/deps'),str(src),'-o',str(q/'public_layout_20')]
bound={'source':meta(src),'compiler':meta(rust),'vr_rlib':meta(vr),'fk_rlib':meta(fk),'environment_set':{'RUSTUP_TOOLCHAIN':'1.97.1','RUSTUP_AUTO_INSTALL':'0'},'environment_unset':['FK_SEEDED_FAULT','RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','RUSTC_BOOTSTRAP']}
dump('PUBLIC_LAYOUT_BINDING.json',bound)
p=run('public_layout_compile',argv,env); assert p.returncode==0,'No retry authorized'
bound['binary']=meta(q/'public_layout_20')
p=run('public_layout_run',[str(q/'public_layout_20')],env); assert p.returncode==0,'No retry authorized'
actual=[line.split('\t') for line in p.stdout.decode().splitlines()]
expected=[('H.coordinates_item',32,8),('VR.constraints_item',16,8),('VR.adjacency_header',24,8),('VR.formed_item',1168,8),('VR.borrowed_control_pair',24,8),('VR.floor_input_pair',40,8),('VR.prescribed_coupling',24,8),('VR.decimal_chunk_view',16,8),('VR.failure_head_pointer',8,8),('VR.skyline_row_header',24,8)]
assert [(x[0],int(x[2]),int(x[3])) for x in actual]==expected
assert len(actual)==10 and sha(src)==bound['source']['sha256'] and sha(vr)==bound['vr_rlib']['sha256'] and sha(fk)==bound['fk_rlib']['sha256']
bound['values']=actual;bound['expected_rows_match']=True
dump('PUBLIC_LAYOUT_BINDING.json',bound)
h=q/'target-h/aarch64-apple-darwin/release/k6_observe'; v=q/'target-vr/aarch64-apple-darwin/release/examples/vk_scale'
dump('ARTIFACTS.json',{'H':meta(h),'VR':meta(v),'reporter':bound['binary']})
print(json.dumps({'VR_compile_invocations':len(comp),'VR_total_logged_commands':len(rows),'reporter_rows':len(actual),'H':meta(h),'VR':meta(v),'compiler_work_finished_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}))

