import pathlib,json,hashlib
r=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30");q=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/scratch/i28-ordinary-artifacts-02");rr=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I28/ordinary_artifacts_02")/'_run_records'
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
old=json.loads((r/'I21/layout08_run/PREFLIGHT.json').read_text())['source_check']
now=json.loads((rr/'COMPILER_SYSROOT_BEFORE.json').read_text())['tool_files']
matched=[]
for x in old['installed_binaries']:
 y=next(z for z in now if z['path'].endswith('/bin/'+x['name']))
 assert y['sha256']==x['sha256']; matched.append({'tool':x['name'],'current':y,'accepted_prior_sha256':x['sha256'],'identity_basis':'same executable bytes; prior full compiler/version record retained'})
node=json.loads((r/'I21/layout08_prep/LABEL_TYPE_MAP.json').read_text())
types=[]
for x in node['rows']:
 if x['kind'].startswith('actual size_of'):
  p=q/'source'/x['production_path']; assert sha(p)==x['production_sha256']
  types.append({'label':x['label'],'type':x['actual_type'],'current_path':str(p),'sha256':sha(p),'basis':'whole original type/consumer source byte-identical; fixed primitive tuple expressions retained; no layout08 build/run'})
arc=json.loads((r/'I21/layout_04/ARC_SOURCE_BINDING.json').read_text())['pages'][0]
ap=pathlib.Path(arc['origin'].replace('<TOOLCHAIN>','/Users/ryan/.rustup/toolchains/1.97.1-aarch64-apple-darwin'))
assert sha(ap)==arc['html_sha256']
d={'tool_identity_matches':matched,'compiler_version_identity':old['channel_versions'],'full_rust_commit':'8bab26f4f68e0e26f0bb7960be334d5b520ea452','llvm_version_identity':'22.1.6 from accepted same-compiler record','basis_record':'I21/layout08_run/PREFLIGHT.json and source_review_RV30/layout08_03/backcheck_run/RETURN.md','current_version_query_executed':False,'cache_missing':'Neither fresh target emitted .rustc_info.json; optional cache absence retained, no rebuild/query performed.','observed_compile_fields':['absolute compiler path/hash','target aarch64-apple-darwin','-C opt-level=3 for target libraries/binaries','-C embed-bitcode=no','-C strip=debuginfo','effective --cfg features and rustflags in argv/fingerprints','normal crate type / no --test','release profile identity'], 'not_present_as_explicit_target_compile_options':['panic strategy','overflow-check setting','debug-assertions setting','LTO setting','codegen-units setting'],'omitted_settings_note':'No invented observed flag values. Ordinary Cargo release default interpretation is derived from the bound compiler/build profile; host dependency-build-script debug-assertions=off is separately explicit and is not mislabeled a target-crate flag.','layout08_five_types':types,'arc_source_identity':{'path':str(ap),'sha256':sha(ap),'basis_record':'I21/layout_04/ARC_SOURCE_BINDING.json','new_arc_derivation':False}}
(rr/'COMPILER_AND_RETAINED_TYPE_BINDING.json').write_text(json.dumps(d,indent=2)+'\n')
events={'preparation_syntax_error':{'effect':'First VENV heredoc failed to parse before execution; no scratch or output path was created by that call. Corrected punctuation then original new-directory assertions passed. No build retry.'},'missing_optional_rustc_cache':{'effect':'Metadata preservation script FileNotFoundError at copying target-h/.rustc_info.json. Source/compiler/sysroot before-after equality had already passed. Both targets have no such file. Recorded absence and continued metadata-only preservation; no compiler/Cargo command repeated. ROOT confirmed this handling in native message.'},'truncated_tool_displays':'Some tool displays truncated raw logs/disassembly. Complete original stdout/stderr retained in _run_records; selected contexts/identities were reread from those files. No native repeat caused by display truncation.','no_build_or_reporter_failures':True,'no_source_or_flag_repairs':True}
(rr/'EXECUTION_EVENTS.json').write_text(json.dumps(events,indent=2)+'\n')
print('Compiler byte identity, five retained type sources, Arc source and execution events recorded.')

