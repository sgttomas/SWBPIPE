from pathlib import Path
import hashlib,json,os,subprocess,time,tomllib
R=Path('/private/tmp/piping-numerical-corrections-20260924');P=R/'projects/chirality-piping';E=P/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/ORDINARY_NUMERICAL_CUT/_run_records/LOCKS_01';E.mkdir(parents=True,exist_ok=True)
paths=subprocess.check_output(['rg','--files',str(P/'core'),str(P/'validation/benchmarks'),str(P/'apps/desktop/src-tauri'),'-g','Cargo.toml'],text=True).splitlines();manifests={Path(p).resolve() for p in paths}
graph={}
for m in manifests:
 data=tomllib.loads(m.read_text());groups=[data.get(k,{}) for k in ['dependencies','dev-dependencies','build-dependencies']]
 for t in data.get('target',{}).values():groups.extend(t.get(k,{}) for k in ['dependencies','dev-dependencies','build-dependencies'])
 graph[m]={ (m.parent/v['path']/'Cargo.toml').resolve() for g in groups for v in g.values() if isinstance(v,dict) and 'path' in v }
changed={R/p for p in subprocess.check_output(['git','diff','--name-only','--','projects/chirality-piping'],cwd=R,text=True).splitlines() if p.endswith('Cargo.toml')};changed.add(P/'validation/benchmarks/numerical_integrity/Cargo.toml')
def affected(m,seen):
 if m in seen:return False
 if m in changed:return True
 return any(affected(c,seen|{m}) for c in graph.get(m,set()))
roots=sorted(m for m in manifests if affected(m,set()));h=lambda b:hashlib.sha256(b).hexdigest();records=[]
(E/'PLAN.json').write_text(json.dumps({'scope':'metadata/actual required lock reconciliation only; no compilation','changed_manifests':[str(m.relative_to(R)) for m in sorted(changed)],'affected_manifest_roots':[str(m.relative_to(R)) for m in roots]},indent=2)+'\n')
for m in roots:
 rel=str(m.relative_to(R));key=h(rel.encode())[:16];d=E/key;d.mkdir(exist_ok=True);lock=m.parent/'Cargo.lock';before=lock.read_bytes() if lock.exists() else None
 if before is not None:(d/'Cargo.lock.before').write_bytes(before)
 env=os.environ.copy();env['CARGO_BUILD_JOBS']='2';env['CARGO_TARGET_DIR']='/private/tmp/piping-numerical-cut-locks/'+key
 base=['cargo','metadata','--offline','--format-version=1','--filter-platform','aarch64-apple-darwin','--manifest-path',rel]
 cmds=[]
 for phase,cmd in [('locked',base+['--locked']),('reconcile',base)]:
  if phase=='reconcile' and cmds[0]['exit_code']==0:break
  with (d/(phase+'.json')).open('wb') as out,(d/(phase+'.stderr')).open('wb') as err:p=subprocess.run(cmd,cwd=R,env=env,stdout=out,stderr=err)
  cmds.append({'phase':phase,'command':cmd,'exit_code':p.returncode})
  if phase=='locked' and p.returncode and 'lock' not in (d/'locked.stderr').read_text().lower():break
 after=lock.read_bytes() if lock.exists() else None
 if after is not None:(d/'Cargo.lock.after').write_bytes(after)
 upgrades=[]
 if before and after:
  old={(x['name'],x.get('source')):x['version'] for x in tomllib.loads(before.decode())['package'] if x.get('source')}
  for x in tomllib.loads(after.decode())['package']:
   k=(x['name'],x.get('source'))
   if x.get('source') and k in old and old[k]!=x['version']:upgrades.append({'name':x['name'],'before':old[k],'after':x['version']})
 record={'manifest':rel,'evidence':key,'commands':cmds,'before_sha256':None if before is None else h(before),'after_sha256':None if after is None else h(after),'changed':before!=after,'registry_version_changes':upgrades};records.append(record)
 (E/'RESULTS.json').write_text(json.dumps(records,indent=2)+'\n');print(json.dumps({'manifest':rel,'exit_code':cmds[-1]['exit_code'],'changed':before!=after,'registry_changes':upgrades}),flush=True)
 if cmds[-1]['exit_code'] or upgrades:raise SystemExit(cmds[-1]['exit_code'] or 2)
print(json.dumps({'manifest_roots':len(records),'changed_locks':sum(r['changed'] for r in records),'builds':0}),flush=True)
