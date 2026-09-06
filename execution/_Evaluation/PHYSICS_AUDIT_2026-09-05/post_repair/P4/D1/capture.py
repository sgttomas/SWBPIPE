from pathlib import Path
import subprocess,json,hashlib,datetime,os,tomllib,base64
r=Path(subprocess.check_output(['git','rev-parse','--show-toplevel'],text=True).strip()); w=r/'projects/chirality-piping'; d=Path(__file__).resolve().parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
seen=set(); files=set()
def visit(crate):
 crate=crate.resolve()
 if crate in seen:return
 seen.add(crate)
 doc=tomllib.loads((crate/'Cargo.toml').read_text())
 for p in crate.rglob('*'):
  if p.is_file() and 'target' not in p.parts and (p.suffix=='.rs' or p.name in ['Cargo.toml','Cargo.lock','build.rs']):files.add(p)
 for kind in ['dependencies','build-dependencies']:
  for v in doc.get(kind,{}).values():
   if isinstance(v,dict) and 'path' in v:visit(crate/v['path'])
visit(w/'core/solver/performance_harness')
assert (w/'core/solver/performance_harness/Cargo.lock').is_file()
historical=[w/'validation/benchmarks'/n for n in ['sparse_default_promotion_observation.dec053.json','sparse_default_promotion_policy.dec053.json']]
files.update(historical)
checkpoint=w/'execution/_Evaluation/PHYSICS_AUDIT_2026-09-05/post_repair/P4/KERNEL_CHECKPOINT_V1'
assert sha(checkpoint/'MANIFEST.json')=='ff28049d1ff3ea27feb8f8ec9759cef10335792797a01a4dede03b897de5eae1'
accepted=json.loads((checkpoint/'SOURCE_HASHES.json').read_text())
for p in files:
 key=str(p.relative_to(r))
 if key in accepted:assert sha(p)==accepted[key],key
snap=lambda:{str(p.relative_to(r)):sha(p) for p in sorted(files)}
status=lambda:subprocess.check_output(['git','status','--porcelain=v1','--untracked-files=all'],cwd=r,text=True)
pre=snap(); before=status(); head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=r,text=True).strip()
(d/'source_before.json').write_text(json.dumps(pre,indent=2)+'\n');(d/'git_before.txt').write_text(before)
cmd=['cargo','run','--locked','--offline','--manifest-path','core/solver/performance_harness/Cargo.toml','--example','sparse_default_promotion_observation']
env=os.environ.copy();env['CARGO_TARGET_DIR']='/tmp/piping-physics-audit-p4-d1-target'
start=datetime.datetime.now(datetime.timezone.utc).isoformat();res=subprocess.run(cmd,cwd=w,env=env,capture_output=True);end=datetime.datetime.now(datetime.timezone.utc).isoformat()
(d/'emitter.raw.json').write_bytes(res.stdout)
(d/'command.json').write_text(json.dumps({'argv':cmd,'cwd':str(w),'env_override':{'CARGO_TARGET_DIR':env['CARGO_TARGET_DIR']},'start_utc':start,'end_utc':end,'exit_code':res.returncode,'stdout_sha256':hashlib.sha256(res.stdout).hexdigest(),'stderr_sha256':hashlib.sha256(res.stderr).hexdigest(),'stderr_base64':base64.b64encode(res.stderr).decode(),'cargo_version':subprocess.check_output(['cargo','--version'],text=True).strip(),'rustc_version':subprocess.check_output(['rustc','--version'],text=True).strip()},indent=2)+'\n')
post=snap();(d/'source_after.json').write_text(json.dumps(post,indent=2)+'\n');(d/'git_after.txt').write_text(status())
(d/'capture_provenance.json').write_text(json.dumps({'record_kind':'new_derivative_current_repaired_kernel_capture','capture_id':'PHYSICS-AUDIT-20260905-P4-D1','source_head':head,'working_tree_dirty':bool(before),'source_before':'source_before.json','source_after':'source_after.json','source_hashes_unchanged':pre==post,'accepted_kernel_manifest_sha256':sha(checkpoint/'MANIFEST.json'),'raw_emitter':'emitter.raw.json','legacy_label_notice':'All record_id, tranche_id, solver_version, status and CI labels inside raw emitter JSON are retained historical emitter metadata. This is a NEW dirty working-tree derivative observation, not replay or renewal of historical acceptance and not a completed DEC025 sweep.','policy_boundary':'Existing DEC053 bounded observation criteria only; timing, RSS and true condition number have no new release thresholds. Unit-separated design not implemented or backfilled.','dependency_crates':[str(p.relative_to(r)) for p in sorted(seen)]},indent=2)+'\n')
assert res.returncode==0,res.stderr.decode();assert pre==post,'Dependency/source drift'
print('capture complete:',len(files),'source/control files;',len(json.loads(res.stdout)['observations']),'observations')
