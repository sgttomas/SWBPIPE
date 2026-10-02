import pathlib,json,hashlib,subprocess,os,datetime
out=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I28/ordinary_qualification_preflight_01")
rr=out/'_run_records'
base=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c")
r=out.parents[1]
digest=lambda b:hashlib.sha256(b).hexdigest()
data=json.loads((rr/'AVAILABLE_PREREQUISITES.json').read_text())
expected={
'i21_kernel_layout':'63ea631507802f2a0ec939299b5dba2b7bdfb08728cb432be45d20c9e6938607',
'vk_records':'7c477a6bd129a1e2d5dd70c4f7557984615337a155ba8f4b0f0a91f08ddc861e',
'rcm-09e04a758b08a557':'cd4955ba3863095bd7af92c02d3471b42a0da22f7237a150b74f4ac4ab41d90f',
'scale-7b5e14694df45805':'7db81a7d5086fe79c8f1b776b80213749915ac313378e4c9d6ae3a27470ae0ae',
'public_layout.rs':'7f4e0aa8a7a05f50bd325f015a81c1f70c291355ec09193010877f386a601dc6',
'rustc':'210df6794001b73ec3d453878707fa1e0bdcb63c427024a6e6574bbe5615a4da'}
matches=[{'path':q['path'],'expected':expected[pathlib.Path(q['path']).name],'actual':q['sha256'],'match':q['sha256']==expected[pathlib.Path(q['path']).name]} for q in data['checks'] if pathlib.Path(q['path']).name in expected]
assert all(x['match'] for x in matches)
orig=json.loads((rr/'EVIDENCE_ORIGINS.json').read_text())
assert all(q['exists'] for q in orig)
source=json.loads((rr/'SOURCE_BINDING.json').read_text())
assert not source['tracked_product_diff_from_frozen']
assert source['registry_lock_blocks_unchanged'] and source['VR_allocator_module_byte_equal_old_to_current']
assert len(source['path_dependency_closure'])==10
assert all('path' in q for q in orig)
for f in ['PLAN.md','SOURCE_BINDING.md','COMMANDS.md']:
 assert '/Users/' not in (out/f).read_text()
for p in rr.glob('*.json'): json.loads(p.read_text())
cmd=['git','diff','--name-only','81c03849033f3ce745668f581f446530789397b8','--','projects/chirality-piping/core','projects/chirality-piping/validation/benchmarks/numerical_robustness']
g=subprocess.run(cmd,cwd=base,env=dict(os.environ,GIT_OPTIONAL_LOCKS='0'),capture_output=True,text=True)
assert g.returncode==0 and not g.stdout
v={'completed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'basis':'source-only preflight, no artifact qualification','checks':{'json_metadata_readable':True,'required_origins_present':True,'top_level_paths_portable':True,'ten_path_dependency_packages':True,'registry_unchanged':True,'VR_allocator_unchanged':True,'frozen_product_diff_empty':True,'historical_artifact_and_compiler_hashes_match':matches},'final_readonly_command':{'cwd':str(base),'argv':cmd,'environment_set':{'GIT_OPTIONAL_LOCKS':'0'},'exit':g.returncode,'stdout':g.stdout,'stderr':g.stderr},'commands_not_executed':'All COMMANDS.md prospective archive/build/disassembly/diagnostic commands','missing_prerequisites':['Final ordinary binaries/build/link and request-site records','Final launch/input path manifest and prepass evidence','ROOT execution/guard slot','Historic exact linker-input lists unavailable; Apple archive reader incompatible'], 'no_new_runtime_or_numeric_evidence':True}
(rr/'VERIFICATION.json').write_text(json.dumps(v,indent=2)+'\n')
ret="""# I28 return

**Preflight complete; no current ordinary H/VR artifact qualification claimed.**

ROOT can grant the bounded final ordinary archive/build/request-check step in
[PLAN.md](PLAN.md), with the exact prospective commands in [COMMANDS.md](COMMANDS.md).
[SOURCE_BINDING.md](SOURCE_BINDING.md) identifies the existing facts, their scoped
diagnostic/seeded/debug artifacts, and each remaining ordinary correspondence.

Read-only checks found the maintained source unchanged from81c038, ten local
dependency packages, unchanged registry lock blocks, unchanged FK/H allocator
basis and byte-identical VR allocator module. The four named existing artifacts,
reporter source and compiler match their recorded hashes;46 installed source pages
and the two serde package archives also match their retained identities.
These checks establish source/evidence identity only.

The finite outstanding cells are final ordinary binaries and contemporaneous
build/library/request-site bindings, final exact launch/input manifest and runner
prepass, plus ROOT's guard/host-slot release. Existing Apple tools cannot parse the
LLVM22 libstd archive; no replacement was sought. Old artifacts have no exact
contemporaneous linker-input hash list, which current hashes cannot backfill.
Matching compiler/target/profile labels never promote seeded/debug private requests.

No Rust/Cargo/build/native artifact tool, diagnostic/model/solver/count run,
measurement, new library proof/tool/configuration, delegation or Git/index write
occurred. All prospective commands remain unrun; no follow-on is active.
Current mathematics remains conditionally accepted at its existing scope.

Actual native parentage, instruction/brief/source origins, direct metadata checks,
execution limits, raw Git commands and hashes are under _run_records.
SHA256SUMS is the complete additive write inventory. Work completed before the
03:55:20 UTC hard deadline; ROOT owns any next grant.
"""
(out/'RETURN.md').write_text(ret)
payloads=[]
for p in sorted(out.rglob('*')):
 if p.is_file() and p.name not in ['SHA256SUMS','WRITE_INVENTORY.json']:
  payloads.append({'path':str(p.relative_to(out)),'bytes':p.stat().st_size,'sha256':digest(p.read_bytes())})
(out/'WRITE_INVENTORY.json').write_text(json.dumps({'write_root':'R/I28/ordinary_qualification_preflight_01','only_additive_files':True,'files':payloads},indent=2)+'\n')
files=[p for p in sorted(out.rglob('*')) if p.is_file() and p.name!='SHA256SUMS']
(out/'SHA256SUMS').write_text(''.join(digest(p.read_bytes())+'  '+str(p.relative_to(out))+'\n' for p in files))
for line in (out/'SHA256SUMS').read_text().splitlines():
 sha,relative=line.split('  ',1); assert digest((out/relative).read_bytes())==sha
print(json.dumps({'complete_utc':v['completed_utc'],'payloads':len(files),'seal':digest((out/'SHA256SUMS').read_bytes()),'total_bytes':sum(p.stat().st_size for p in files),'return':str(out/'RETURN.md')}))

