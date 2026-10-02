import pathlib,json,hashlib,shlex,subprocess,os,datetime,stat,shutil
base=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c");r=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30");q=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/scratch/i28-ordinary-artifacts-02");rr=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I28/ordinary_artifacts_02")/'_run_records'
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def meta(p):return {'path':str(p),'bytes':pathlib.Path(p).stat().st_size,'sha256':sha(p)}
def dump(n,d):(rr/n).write_text(json.dumps(d,indent=2)+'\n')
def inventory(root):return [dict(meta(p),relative=str(p.relative_to(root)),mode=oct(stat.S_IMODE(p.stat().st_mode))) for p in sorted(root.rglob('*')) if p.is_file()]
before=json.loads((rr/'SOURCE_BEFORE.json').read_text());after=inventory(q/'source');assert before==after
dump('SOURCE_AFTER.json',after)
pre=json.loads((rr/'COMPILER_SYSROOT_BEFORE.json').read_text())
tool=[meta(x['path']) for x in pre['tool_files']]
libroot=pathlib.Path('/Users/ryan/.rustup/toolchains/1.97.1-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib')
libs=inventory(libroot);assert tool==pre['tool_files'] and libs==pre['target_library_inventory']
dump('COMPILER_SYSROOT_AFTER.json',{'tool_files':tool,'target_library_inventory':libs,'before_after_equal':True,'compiler_driver_postbuild_inventory':[meta(p) for p in sorted(libroot.parents[2].glob('*.dylib'))],'compiler_driver_limit':'Top-level driver dylibs hashed after build only; not presented as prebuild identity.'})
targetinventories={};compile_rows=[];depinfo=[];fingerprints=[];cache_status=[]
for ident in ['h','vr']:
 target=q/('target-'+ident)
 targetinventories[ident]=inventory(target)
 shutil.copyfile(q/'logs'/('build-'+ident+'.log'),rr/('build-'+ident+'.log'))
 info=target/'.rustc_info.json'
 cache_status.append({'binary_group':ident,'path':str(info),'exists':info.is_file(),'interpretation':'Optional cache availability only; no compiler query or rebuild to create it.'})
 if info.is_file(): shutil.copyfile(info,rr/('rustc-info-'+ident+'.json'))
 for line in (q/'logs'/('build-'+ident+'.log')).read_text().splitlines():
  if 'Running `' not in line:continue
  a=shlex.split(line.split('Running `',1)[1].rsplit('`',1)[0])
  if '--crate-name' not in a:continue
  cn=a[a.index('--crate-name')+1]
  compile_rows.append({'binary_group':ident,'crate':cn,'argv':a,'cfg':[a[i+1] for i,x in enumerate(a) if x=='--cfg'],'codegen':[a[i+1] for i,x in enumerate(a) if x=='-C'],'target':a[a.index('--target')+1] if '--target' in a else 'host build-script compilation; no explicit --target','test_flag':'--test' in a})
 for p in sorted(target.rglob('*.d')):
  depinfo.append({'binary_group':ident,**meta(p),'content':p.read_text()})
 for p in sorted(target.glob('**/.fingerprint/*/*.json')):
  fingerprints.append({'binary_group':ident,**meta(p),'content':json.loads(p.read_text())})
dump('TARGET_INVENTORIES.json',targetinventories);dump('COMPILE_ARGV.json',compile_rows);dump('DEPENDENCY_INFO.json',depinfo);dump('CARGO_FINGERPRINTS.json',fingerprints);dump('RUSTC_CACHE_AVAILABILITY.json',cache_status)
for row in compile_rows:
 assert not row['test_flag']
 if row['crate'] in ['k6_observe','vk_scale','open_pipe_stress_frame_kernel','open_pipe_stress_solver_performance_harness','piping_numerical_robustness']:assert not row['cfg']
for crate,want in [('serde_json',['feature="default"','feature="float_roundtrip"','feature="std"','fast_arithmetic="64"']),('serde_core',['feature="std"'])]:
 rows=[x for x in compile_rows if x['crate']==crate];assert len(rows)==1 and rows[0]['cfg']==want
argv=['git','diff','cb13fcf3ea560c0d07ad9ac02dac78e9d2e06e00','81c03849033f3ce745668f581f446530789397b8','--','projects/chirality-piping/core/solver/performance_harness/src/k6/models.rs','projects/chirality-piping/validation/benchmarks/numerical_robustness/src/cases.rs']
z=subprocess.run(argv,cwd=base,env=dict(os.environ,GIT_OPTIONAL_LOCKS='0'),capture_output=True)
assert z.returncode==0
(rr/'TYPE_SOURCE_SCOPE.diff').write_bytes(z.stdout)
dump('TYPE_SOURCE_DIFF_COMMAND.json',{'argv':argv,'cwd':str(base),'environment_set':{'GIT_OPTIONAL_LOCKS':'0'},'exit':z.returncode,'stdout_file':'TYPE_SOURCE_SCOPE.diff','stdout_sha256':hashlib.sha256(z.stdout).hexdigest(),'stderr':z.stderr.decode(),'inspection':'Full two-file diff contains capture/wrapper function changes and new ModelOrigin/ModelRecipe only; no original type-declaration attribute or field modification.'})
dump('PRESERVATION_CHECK.json',{'time_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_files':len(after),'source_before_after_equal':True,'compiler_and_target_sysroot_before_after_equal':True,'ordinary_archive':meta(q/'source.tar'),'H_manifest_dir':str(q/'source/projects/chirality-piping/core/solver/performance_harness'),'VR_manifest_dir':str(q/'source/projects/chirality-piping/validation/benchmarks/numerical_robustness'),'no_production_binary_launch':True,'layout_reporter_only':True,'linkage_limit':'Verbose rustc --extern/dep-info/fingerprint plus contemporaneous standard-library inventory and final symbols are retained. No separate linker response/map or per-standard-object input list was emitted by the normal commands; otool reports only libSystem.B dynamically. No unavailable link-map fact is asserted.'})
print(json.dumps({'source_files':len(after),'ordinary_compile_argv':len(compile_rows),'depinfo_files':len(depinfo),'fingerprints':len(fingerprints),'target_files':{k:len(v) for k,v in targetinventories.items()},'std_files':len(libs),'all_preserved':True}))
