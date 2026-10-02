import pathlib,json,hashlib,os,subprocess,tomllib,datetime
base=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c")
r=base/"projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30"
num=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/numerics")
nr=num/"projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30"
out=r/'I28/ordinary_qualification_preflight_01'
rr=out/'_run_records'
rr.mkdir(parents=True,exist_ok=True)
def digest(p):
 p=pathlib.Path(p)
 if not p.is_file(): return {'path':str(p),'exists':False}
 data=p.read_bytes()
 return {'path':str(p),'exists':True,'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()}
def write(n,v): (rr/n).write_text(json.dumps(v,indent=2)+'\n')
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
commands=[]
def git(*args,cwd=base):
 p=subprocess.run(['git',*args],cwd=cwd,env=env,capture_output=True,text=True)
 commands.append({'cwd':str(cwd),'argv':['git',*args],'environment_set':{'GIT_OPTIONAL_LOCKS':'0'},'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr})
 if p.returncode: raise RuntimeError(p.stderr)
 return p.stdout
instructions=[base/'AGENTS.md',base/'agents/AGENT_TASK.md',base/'projects/chirality-piping/AGENTS.md',nr/'BRIEFS/COMMON.md',nr/'BRIEFS/I28_ORDINARY_QUALIFICATION_PREFLIGHT.md',nr/'PLAN.md',nr.parent/'ROOT_RULINGS_V1.md']
write('INSTRUCTION_ORIGINS.json',{'actual_read_origins':[digest(p) for p in instructions],'root_rulings_read_lines':['3878-3910','4268-4345'],'note':'Root/TASK/Piping read in K6C; COMMON also first read in K6C, then authoritative NUM copy. No skill/workflow body selected. Parent Native task message is supplied brief; prompt fences are not OS sandbox enforcement.'})
selected=[
'metric_design_12_profile_binding/PROPOSAL.md','metric_design_12_profile_binding/_run_records/ROOT_SCOPE.md','metric_design_12_profile_binding/_run_records/PARENT_CLARIFICATION.txt',
'I21/layout_04/RETURN.md','I21/layout_04/BUILD_IDENTITIES.json','I21/layout_04/LAYOUTS.json','I21/layout_04/LABEL_TYPE_MAP.json','I21/layout_04/H_LABEL_TYPE_MAP.json','I21/layout_04/ARC_SOURCE_BINDING.json','I21/layout_04/OVERLAY.diff','I21/layout_04/H_OVERLAY.diff',
'I21/layout08_prep/LABEL_TYPE_MAP.json','I21/layout08_prep/OVERLAY.diff','I21/layout08_prep/OVERLAY_FILES.json','I21/layout08_prep/RUNTIME_COMMAND.txt','I21/layout08_run/COMMAND_RELEASE_CANDIDATE.json','I21/layout08_run/PREFLIGHT.json',
'source_review_RV30/layout08_03/RETURN.md','source_review_RV30/layout08_03/backcheck_run/RETURN.md',
'I23/private_request_artifacts_08/ARTIFACT_BINDING.json','I23/private_request_artifacts_08/LIBSTD_READER_LIMIT.txt','source_review_RV30/h_request_bindings_10/RETURN.md','source_review_RV30/h_request_bindings_10/SOURCE_AND_BUILD_BINDING.json',
'verification/public_layout_20/RETURN.md','verification/public_layout_20/_run_records/public_layout.rs','verification/public_layout_20/_run_records/EXECUTION.json','verification/public_layout_20/_run_records/VALUES.json','source_review_RV30/public_layout_result_22/RETURN.md',
'I23/vr_node_artifacts_17/_run_records/ARTIFACT_BINDING.json','I23/vr_node_artifacts_17/_run_records/COMMANDS.json','source_review_RV30/vr_nodes_17/RETURN.md','I23/wrapped_error_artifacts_18/_run_records/COMMANDS.json','source_review_RV30/wrapped_errors_19/RETURN.md','I23/serde_binding_07/PACKAGE_BINDING.json','source_review_RV30/serde_binding_08/RETURN.md',
'metric_design_14_reference_callers/METHOD_AND_JOIN.md','metric_design_14_reference_callers/_run_records/SOURCE_ORIGINS.json','metric_design_14_reference_callers/_run_records/PUBLIC_SIZES.json','metric_design_14_reference_callers/_run_records/FINITE_BINDING.json','metric_design_14_reference_callers/_run_records/ROOT_SELECTED_CONTRACT.md','metric_design_14_reference_callers/CLI24_LAUNCHES.json','metric_design_14_reference_callers/REFERENCE_LAUNCHES.json','metric_design_14_reference_callers/_run_records/INPUT_HASHES.json','metric_design_14_reference_callers/_run_records/CLI_EXTERNAL_INPUTS.json',
'metric_design_15_result5_join/METHOD.md','metric_design_15_result5_join/_run_records/COMPONENT_ACCEPTANCE_RULING.txt','metric_design_15_result5_join/_run_records/INPUT_BINDING.json','source_review_RV30/result5_join_28/RETURN.md',
'I21/h_integration_25/SOURCE_MAP.md','I21/_run_records/kernel_implementation_24/SOURCE_MAP.md','I24/vr_integration_01/_run_records/SOURCE_MAP.md','I24/vr_integration_01/RETURN.md',
'I21/source_13/LIBRARY_SOURCE.json','I21/source_11/RETURN.md']
write('EVIDENCE_ORIGINS.json',[dict(digest(r/p),relative=p,read_scope=('metadata/hash only' if p.endswith(('LAYOUTS.json','LABEL_TYPE_MAP.json','H_LABEL_TYPE_MAP.json','OVERLAY.diff','OVERLAY_FILES.json','CLI24_LAUNCHES.json','REFERENCE_LAUNCHES.json','INPUT_HASHES.json','CLI_EXTERNAL_INPUTS.json')) else 'source/record read or selected excerpt; large records read selectively')) for p in selected])
head=git('rev-parse','--show-toplevel','HEAD').splitlines()
frozen='81c03849033f3ce745668f581f446530789397b8'
old='40129a225d73860ac2a53da9a2fa73869df668f3'
p='projects/chirality-piping'
h=p+'/core/solver/performance_harness'; v=p+'/validation/benchmarks/numerical_robustness'
sourcecheck={'head':head,'frozen':frozen,'tracked_product_diff_from_frozen':git('diff','--name-only',frozen,'--',p+'/core',v).splitlines(),'old_to_frozen_changed':git('diff','--name-only',old,frozen,'--',p+'/core',v).splitlines()}
manifests={}; queue=[base/h/'Cargo.toml',base/v/'Cargo.toml']
while queue:
 f=queue.pop().resolve()
 if str(f) in manifests: continue
 d=tomllib.loads(f.read_text()); manifests[str(f)]=d
 for dep in d.get('dependencies',{}).values():
  if isinstance(dep,dict) and 'path' in dep: queue.append(f.parent/dep['path']/'Cargo.toml')
sourcecheck['path_dependency_closure']=[{'manifest':str(f),'package':d['package']['name'],'features':d.get('features',{}),'profile':d.get('profile',{}),'manifest_identity':digest(f),'lock_identity':digest(pathlib.Path(f).with_name('Cargo.lock'))} for f,d in sorted(manifests.items())]
newlock=tomllib.loads((base/v/'Cargo.lock').read_text())
oldlock=tomllib.loads(git('show',old+':'+v+'/Cargo.lock'))
reg=lambda d:[q for q in d['package'] if 'source' in q]
sourcecheck['registry_lock_blocks_unchanged']=reg(newlock)==reg(oldlock)
sourcecheck['registry_packages']=[{'name':q['name'],'version':q['version'],'source':q['source'],'checksum':q.get('checksum')} for q in reg(newlock)]
cur=(base/v/'examples/vk_scale.rs').read_text()
prior=git('show',old+':'+v+'/examples/vk_scale.rs')
part=lambda t:t[t.index('mod alloc {'):t.index('#[global_allocator]')]
sourcecheck['VR_allocator_module_byte_equal_old_to_current']=part(cur)==part(prior)
sourcecheck['VR_allocator_module_sha256']=hashlib.sha256(part(cur).encode()).hexdigest()
selected_source=[h+'/src/k6/w1/envelope.rs',h+'/src/k6/w1/h_envelope.rs',h+'/src/k6/w1/counts.rs',h+'/src/k6/models.rs',h+'/src/k6/canonical.rs',h+'/src/bin/k6_observe/alloc.rs',h+'/src/bin/k6_observe/main.rs',h+'/runner/k6_runner.py',v+'/src/envelope.rs',v+'/src/cases.rs',v+'/src/scale.rs',v+'/examples/vk_scale.rs',v+'/runner/vk_scale_runner.py',p+'/core/solver/frame_kernel/src/structural/retained/adaptive.rs',p+'/core/solver/frame_kernel/src/structural/retained/source.rs',p+'/core/solver/frame_kernel/src/structural/retained/factor.rs']
sourcecheck['current_selected_source_hashes']=[digest(base/x) for x in selected_source]
write('SOURCE_BINDING.json',sourcecheck)
rust=pathlib.Path('/Users/ryan/.rustup/toolchains/1.97.1-aarch64-apple-darwin')
wt=base.parent
availability=[rust/'bin/cargo',rust/'bin/rustc',pathlib.Path('/usr/bin/nm'),pathlib.Path('/usr/bin/objdump'),pathlib.Path('/usr/bin/otool'),wt/'guard/memguard.sh',wt/'k6c-layout08-target/aarch64-apple-darwin/release/examples/i21_kernel_layout',wt/'a1-vk-target/release/examples/vk_records',wt/'a1-consumer-vr-target/debug/deps/rcm-09e04a758b08a557',wt/'a1-consumer-vr-target/debug/deps/scale-7b5e14694df45805',wt/'scratch/public_layout_20/public_layout.rs']
library=json.loads((r/'I23/private_request_artifacts_08/ARTIFACT_BINDING.json').read_text())['current_installed_artifacts']
for q in library: availability.append(pathlib.Path(q['path'].replace('<RUST>',str(rust))))
checks=[]
for f in availability:
 row=digest(f)
 expected=next((q['sha256'] for q in library if q['path'].replace('<RUST>',str(rust))==str(f)),None)
 if expected: row['matches_prior_current_install_hash']=row.get('sha256')==expected
 checks.append(row)
config_paths=[]
for a in [base,*base.parents,pathlib.Path('/Users/ryan')]:
 for name in ['.cargo/config','.cargo/config.toml']:
  f=a/name
  if f.is_file() and str(f) not in config_paths: config_paths.append(str(f))
ch=pathlib.Path(os.environ.get('CARGO_HOME','/Users/ryan/.cargo'))
for name in ['config','config.toml']:
 f=ch/name
 if f.is_file() and str(f) not in config_paths: config_paths.append(str(f))
package_record=json.loads((r/'I23/serde_binding_07/PACKAGE_BINDING.json').read_text())
packages=[]
for q in package_record['packages']:
 archive=pathlib.Path(q['archive']['path'].replace('<CARGO_HOME>',str(ch)))
 x=digest(archive); x['locked_expected_sha256']=q['lock_checksum']; x['matches_lock']=x.get('sha256')==q['lock_checksum']; packages.append(x)
write('AVAILABLE_PREREQUISITES.json',{'checks':checks,'cargo_config_paths_present':config_paths,'package_archives':packages,'not_checked':'Tool execution, Cargo cache resolvability, guard liveness, future final-binary symbols, prospective archive. Filesystem presence/hash only.','observed_relevant_environment':{k:os.environ[k] for k in ['CARGO_HOME','RUSTUP_TOOLCHAIN','RUSTUP_AUTO_INSTALL','RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','RUSTC','RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','CARGO_TARGET_DIR','CARGO_INCREMENTAL','FK_SEEDED_FAULT'] if k in os.environ}})
write('READONLY_GIT_COMMANDS.json',commands)
print(json.dumps({'out':str(out),'head':head[-1],'diff_from_frozen':sourcecheck['tracked_product_diff_from_frozen'],'path_dependency_count':len(manifests),'registry_unchanged':sourcecheck['registry_lock_blocks_unchanged'],'VR_allocator_unchanged':sourcecheck['VR_allocator_module_byte_equal_old_to_current'],'missing_files':[q['path'] for q in checks if not q['exists']],'configs':config_paths,'package_archive_match':[q['matches_lock'] for q in packages]}))

