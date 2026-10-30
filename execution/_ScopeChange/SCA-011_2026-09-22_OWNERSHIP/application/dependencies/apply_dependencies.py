#!/usr/bin/env python3
"""Apply only the accepted Group-2 dependency files; no graph adoption.

This is an exact application helper, not a replacement dependency workflow.
Local source support is independently recorded in LOCAL_SOURCE_BACKCHECK.csv.
Default is read-only poststate verification; --apply performs initial creation.
"""
import argparse, csv, hashlib, json, re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[7]
SCA=ROOT/'projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP'
OUT=SCA/'application/dependencies'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest() if p.is_file() else 'ABSENT'
def read(p):
    with p.open(newline='') as f:return list(csv.DictReader(f))
def dump(p,obj):p.write_text(json.dumps(obj,indent=2)+'\n')
def rel(p):return str(p.relative_to(ROOT))
def main():
    apply=argparse.ArgumentParser();apply.add_argument('--apply',action='store_true');args=apply.parse_args()
    manifest=read(SCA/'APPLY_MANIFEST.csv'); bytarget={r['CanonicalTarget']:r for r in manifest}
    own=[r for r in manifest if r['ApplicationLane']=='GROUP2_DEPENDENCY_TASK'];assert len(own)==20
    promotion=json.loads((SCA/'dependencies/LOCAL_MIRRORS_PROMOTION.json').read_text())
    rules={r['target']:r for r in promotion['files']};assert set(rules)=={r['CanonicalTarget'] for r in own}
    checkpoint=ROOT/'projects/chirality-piping/execution/_ScopeChange/checkpoint_snapshots/SCA-011_GROUP-2_2026-09-22'
    accepted={r['Path']:r['SHA256'] for r in read(checkpoint/'ACCEPTED_MANIFEST.csv')}
    for name in ['APPLY_MANIFEST.csv','DEPENDENCY_SNAPSHOT_MANIFEST.json','dependencies/LOCAL_MIRRORS_PROMOTION.json']:
        path=SCA/name;assert digest(path)==accepted[rel(path)],name
    sources=[];prepared=[]; local=[]
    for r in own:
        target=ROOT/r['CanonicalTarget'];cand=ROOT/r['CandidatePath'];rule=rules[r['CanonicalTarget']]
        assert digest(cand)==r['CandidateSHA256']==rule['candidate_sha256']
        data=cand.read_bytes()
        for op in rule['operations']:
            old=op['old'].encode();new=op['new'].encode();assert data.count(old)==op['expected_occurrences'];data=data.replace(old,new)
        assert hashlib.sha256(data).hexdigest()==r['AppliedSHA256']==rule['applied_sha256']
        observed=digest(target);assert observed==(r['BeforeSHA256'] if args.apply else r['AppliedSHA256']),(str(target),observed)
        prepared.append((target,data));local.append({'target':r['CanonicalTarget'],'before_sha256':r['BeforeSHA256'],'candidate_sha256':r['CandidateSHA256'],'applied_sha256':r['AppliedSHA256']})
        if target.name=='Dependencies.csv':
            for name in ['ScopeOfWork.md','_CONTEXT.md','_REFERENCES.md']:
                source=target.parent/name;raw=source.read_bytes(); source_rel=rel(source)
                if source_rel in bytarget: assert digest(source)==bytarget[source_rel]['AppliedSHA256'],source_rel
                sources.append({'path':source_rel,'sha256':hashlib.sha256(raw).hexdigest(),'bytes':len(raw),'read_scope':'full bytes; additive claim/identity/reference analysis; no recertification of historical claims'})
    snap=json.loads((SCA/'DEPENDENCY_SNAPSHOT_MANIFEST.json').read_text())
    for r in snap['files']:
        cand=ROOT/r['candidate'];target=ROOT/r['proposed_target'];assert digest(cand)==r['sha256']
        assert digest(target)==('ABSENT' if args.apply else r['sha256'])
        prepared.append((target,cand.read_bytes()))
    if args.apply:
        for target,data in prepared:
            target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(data)
    for target,data in prepared:assert target.read_bytes()==data
    for r in own:
        if r['CanonicalTarget'].endswith('/Dependencies.csv') and r['BeforeSHA256']!='ABSENT':
            data=(ROOT/r['CanonicalTarget']).read_bytes();offset=data.find(b'v3.1,SCA011-');assert offset>0
            assert hashlib.sha256(data[:offset]).hexdigest()==r['BeforeSHA256'],r['CanonicalTarget']
    decomp=ROOT/'projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md'
    assert digest(decomp)==bytarget[rel(decomp)]['AppliedSHA256']
    sources.append({'path':rel(decomp),'sha256':digest(decomp),'bytes':decomp.stat().st_size,'read_scope':'identity, scope/objective trace and architecture-basis resolution'})
    dump(OUT/'APPLICATION_FILE_CHECKS.json',{'status':'PASS','phase':'GROUP2_APPLIED_GROUP3_PENDING','local_files':local,'dag_staged_files':snap['files'],'source_files':sources,'pointer_changes':False,'source_edits_by_this_task':False})
    print(json.dumps({'status':'PASS','local_files':len(local),'staged_files':len(snap['files']),'mode':'APPLY' if args.apply else 'VERIFY'}))
if __name__=='__main__':main()
