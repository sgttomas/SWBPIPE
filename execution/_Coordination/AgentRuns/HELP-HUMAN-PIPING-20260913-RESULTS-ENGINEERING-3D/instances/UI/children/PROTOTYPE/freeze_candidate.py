from pathlib import Path
import json,hashlib,datetime,sys
out=Path(__file__).resolve().parent;root=out
while root.name!='chirality-3d-design-20260913':root=root.parent
ui=out.parent.parent
p=out/'write_wireframes.py';s=p.read_text().replace('y+=control+26','y+=control+15');p.write_text(s)
# Preserve supplemental actual supplied context bytes and source-qualified origins.
f=out/'_run_records/INPUT_MANIFEST.json';m=json.loads(f.read_text())
extra=[ui/'INTERACTION_CONTRACT_V1.md',ui/'WITNESS_PLAN_V1.md',root/'projects/chirality-piping/apps/desktop/src/features/geometry-tools/GeometryToolsPanel.tsx']
for n,p in enumerate(extra,30):
 b=p.read_bytes();raw=out/'_run_records/context'/f'{n:02d}-{p.name}';raw.write_bytes(b)
 m['context'].append({'origin':str(p.relative_to(root)),'sha256':hashlib.sha256(b).hexdigest(),'bytes':len(b),'preservedRawCopy':str(raw.relative_to(out))})
m['context']=list({r['origin']:r for r in m['context']}.values())
f.write_text(json.dumps(m,indent=2)+'\n')
files=[p for p in out.rglob('*') if p.is_file() and p.name not in ['RETURN.md','MANIFEST.json'] and '_run_records' not in p.relative_to(out).parts]
items=[{'path':str(p.relative_to(out)),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(files)]
version=sys.argv[1] if len(sys.argv)>1 else 'V2'
freeze=out/f'_run_records/CANDIDATE_FREEZE_{version}.json'
if freeze.exists():raise RuntimeError('Freeze already exists; use a new immutable version')
freeze.write_text(json.dumps({'status':'FROZEN_SOURCE_CANDIDATE_UNTESTED','sourceHEAD':'8f27fa3d8ec5e128e61fd3ac4076e74d7955f355','claims':'design simulation only','files':items},indent=2)+'\n')
