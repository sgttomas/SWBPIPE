import pathlib,json,hashlib,os,subprocess,datetime,tarfile,stat
base=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c")
r=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30")
out=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I28/ordinary_artifacts_02")
q=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/scratch/i28-ordinary-artifacts-02")
num=base.parent/'numerics'; nr=num/r.relative_to(base)
assert not q.exists(), 'QUAL_ROOT exists; stop before changing'
assert not out.exists(), 'Packet exists; stop before changing'
q.mkdir(); (q/'source').mkdir(); (q/'logs').mkdir(); (out/'_run_records').mkdir(parents=True)
rr=out/'_run_records'
def sha(p): return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def meta(p): return {'path':str(p),'bytes':pathlib.Path(p).stat().st_size,'sha256':sha(p)}
def dump(n,d): (rr/n).write_text(json.dumps(d,indent=2)+'\n')
commands=[]
def run(argv,cwd=base):
 start=datetime.datetime.now(datetime.timezone.utc).isoformat()
 p=subprocess.run(argv,cwd=cwd,env=dict(os.environ,GIT_OPTIONAL_LOCKS='0'),capture_output=True,text=True)
 row={'argv':argv,'cwd':str(cwd),'start_utc':start,'end_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr}
 commands.append(row); dump('PREPARATION_COMMANDS.json',commands)
 return p
g=run(['ps','-p','5387','-o','pid=,command='])
assert g.returncode==0 and 'memguard.sh' in g.stdout
for name in ['cargo','rustc']:
 x=run(['pgrep','-x',name]); assert x.returncode==1 and not x.stdout
freeze='81c03849033f3ce745668f581f446530789397b8'
roots=[str(pathlib.Path(x['manifest']).parent.relative_to(base)) for x in json.loads((r/'I28/ordinary_qualification_preflight_01/_run_records/SOURCE_BINDING.json').read_text())['path_dependency_closure']]
assert len(roots)==10
a=run(['git','archive','--format=tar','--output='+str(q/'source.tar'),freeze,*roots]); assert a.returncode==0
a=run(['tar','-xf',str(q/'source.tar'),'-C',str(q/'source')]); assert a.returncode==0
a=run(['chmod','-R','a-w',str(q/'source')]); assert a.returncode==0
a=run(['git','ls-tree','-r','--full-tree',freeze,'--',*roots]); assert a.returncode==0
(rr/'GIT_SOURCE_TREE.txt').write_text(a.stdout)
def inventory(root):
 return [dict(meta(p),relative=str(p.relative_to(root)),mode=oct(stat.S_IMODE(p.stat().st_mode))) for p in sorted(root.rglob('*')) if p.is_file()]
dump('SOURCE_BEFORE.json',inventory(q/'source'))
cfg=[]
for ancestor in [q/'source',*list((q/'source').parents)]:
 for tail in ['.cargo/config','.cargo/config.toml']:
  p=ancestor/tail
  if p.is_file(): cfg.append(dict(meta(p),content=p.read_text()))
cargo_home=pathlib.Path(os.environ.get('CARGO_HOME','/Users/ryan/.cargo'))
for tail in ['config','config.toml']:
 p=cargo_home/tail
 if p.is_file(): cfg.append(dict(meta(p),content=p.read_text()))
rust=pathlib.Path('/Users/ryan/.rustup/toolchains/1.97.1-aarch64-apple-darwin')
libs=rust/'lib/rustlib/aarch64-apple-darwin/lib'
tool=[rust/'bin/cargo',rust/'bin/rustc',pathlib.Path('/usr/bin/nm'),pathlib.Path('/usr/bin/otool'),pathlib.Path('/usr/bin/objdump')]
dump('COMPILER_SYSROOT_BEFORE.json',{'tool_files':[meta(p) for p in tool],'target_library_inventory':inventory(libs),'status':'Contemporaneous installed-input inventory; actual compile/dependency/link observations remain separate.'})
kept={k:v for k,v in os.environ.items() if k.startswith(('CARGO_','RUST','FK_')) or k in ['PATH','SDKROOT','MACOSX_DEPLOYMENT_TARGET','CC','CXX','AR','LD','LIBRARY_PATH','DYLD_LIBRARY_PATH']}
dump('BUILD_INPUTS.json',{'freeze':freeze,'archive':meta(q/'source.tar'),'package_roots':roots,'cargo_inputs':[meta(p) for p in sorted((q/'source').rglob('Cargo.*')) if p.name in ['Cargo.toml','Cargo.lock']],'configuration':cfg,'relevant_inherited_environment':kept,'targets_initially_absent':all(not(q/t).exists() for t in ['target-h','target-vr']),'source_root_fixed':str(q/'source'),'guard_pid':5387})
origins=[base/'AGENTS.md',base/'agents/AGENT_TASK.md',base/'projects/chirality-piping/AGENTS.md',nr/'BRIEFS/COMMON.md',nr/'BRIEFS/I28_ORDINARY_ARTIFACTS_02.md',nr.parent/'ROOT_RULINGS_V1.md']
origins += [r/'I28/ordinary_qualification_preflight_01'/x for x in ['RETURN.md','PLAN.md','COMMANDS.md','SOURCE_BINDING.md','SHA256SUMS']]
dump('ORIGINS.json',[meta(p) for p in origins])
brief=run(['git','show','b37cfdcf3014dd8e829773035b28fbf2e24c002d:'+str((nr/'BRIEFS/I28_ORDINARY_ARTIFACTS_02.md').relative_to(num))],cwd=num)
assert brief.returncode==0 and brief.stdout==(nr/'BRIEFS/I28_ORDINARY_ARTIFACTS_02.md').read_text()
assert not cfg,'Unexplained Cargo configuration: stop for inspection'
dump('RUN.json',{'role':'TASK Type2','identity':'/root/i28_artifact_preflight','parent':'/root HELP_HUMAN Agent0','mechanism':'native followup_task','receipt_utc':'2026-10-02T03:57:16Z','new_work_cutoff_utc':'2026-10-02T04:37:16Z','hard_deadline_utc':'2026-10-02T04:42:16Z','write_roots':[str(out),str(q)],'prompt_fence_not_OS_sandbox':True,'no_delegation':True})
print(json.dumps({'archive':meta(q/'source.tar'),'source_files':len(inventory(q/'source')),'configs':len(cfg),'ready':True}))

