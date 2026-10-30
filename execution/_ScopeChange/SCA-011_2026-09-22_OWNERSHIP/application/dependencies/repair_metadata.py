#!/usr/bin/env python3
"""One-time authorized metadata correction of the 84 SCA-011 added rows only.

The exact accepted candidate, initial application evidence and historical stage
sources are read-only. No ownership, endpoint, type or maturity field changes.
"""
import csv,hashlib,io,json,re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[7]
SCA=ROOT/'projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP'
OUT=SCA/'application/dependencies';DAG=ROOT/'projects/chirality-piping/execution/_DAG/DAG-011'
def read(p):
 with p.open(newline='') as f:return list(csv.DictReader(f))
def sha(data):return hashlib.sha256(data).hexdigest()
def rel(p):return str(p.relative_to(ROOT))
def writecsv(p,rows):
 with p.open('w',newline='') as f:w=csv.DictWriter(f,fieldnames=list(rows[0]));w.writeheader();w.writerows(rows)
manifest=read(SCA/'APPLY_MANIFEST.csv');targets=[r for r in manifest if r['ApplicationLane']=='GROUP2_DEPENDENCY_TASK']
assert len(targets)==20
back={r['DependencyID']:r for r in read(OUT/'LOCAL_SOURCE_BACKCHECK.csv')}
added=read(SCA/'dependencies/DependencyEdges.additions.csv');assert len(added)==84==len(back)
# Check allocation collisions in actual local registers and aggregate; no source
# content or historical snapshot is amended by this identity census.
used=set();allocation_inputs=[]
for p in sorted((ROOT/'projects/chirality-piping/execution').glob('PKG-*/1_Working/DEL-*/Dependencies.csv')):
 rs=read(p);used.update(r['DependencyID'] for r in rs);allocation_inputs.append({'path':rel(p),'sha256':sha(p.read_bytes()),'scope':'DependencyID cells only'})
used.update(r['DependencyID'] for r in read(DAG/'DependencyEdges.csv'))
next_seq={};mapping={};crosswalk=[];modified={};field_delta=[]
for old in added:
 d=old['FromDeliverableID'];prefix='DEP-'+d.removeprefix('DEL-')+'-';n=next_seq.get(d,1)
 while prefix+f'{n:03d}' in used:n+=1
 new_id=prefix+f'{n:03d}';used.add(new_id);next_seq[d]=n+1;mapping[old['DependencyID']]=new_id
 source=back[old['DependencyID']];sourcepath=ROOT/source['LocalEvidenceFile'];quote=source['LocalEvidenceQuote'];assert sha(sourcepath.read_bytes())==source['LocalEvidenceSHA256'];assert quote in sourcepath.read_text();assert len(quote.split())<=30
 new=dict(old);new['DependencyID']=new_id;new['EvidenceFile']=str(sourcepath.relative_to(ROOT/'projects/chirality-piping'))
 new['SourceRef']=new['EvidenceFile']+' # '+source['LocalSourceRef'];new['EvidenceQuote']=quote
 new['Notes']+='; SCA011OriginalDependencyID='+old['DependencyID']+'; original_evidence_file='+old['EvidenceFile']+'; original_source_ref='+old['SourceRef']+'; metadata_repair=SCA-011/application/dependencies/CORRECTNESS_REPAIR.md'
 allowed={'DependencyID','EvidenceFile','SourceRef','EvidenceQuote','Notes'}
 assert all(old[k]==new[k] for k in old if k not in allowed)
 for k in old:
  if old[k]!=new[k]:field_delta.append({'OriginalDependencyID':old['DependencyID'],'CurrentDependencyID':new_id,'Owner':d,'Field':k,'Before':old[k],'After':new[k]})
 modified[old['DependencyID']]=new
 crosswalk.append({'OriginalDependencyID':old['DependencyID'],'CurrentDependencyID':new_id,'Owner':d,'OriginalCandidate':'dependencies/DependencyEdges.additions.csv','Reason':'Root SPEC §6.8 canonical DEP-owner identity; unchanged information-flow relation','SourceSHA256':source['LocalEvidenceSHA256']})
# Freeze the actual pre-repair files before changing any. Initial application
# reports remain unchanged, so this is a separate, inspectable follow-on act.
paths=[ROOT/r['CanonicalTarget'] for r in targets]+[DAG/n for n in ['DependencyEdges.csv','PROVENANCE.json','APPROVAL_RECORD.md','HANDOFF.md','MANIFEST.sha256']]
before=[]
for p in paths:
 data=p.read_bytes();backup=OUT/'repair_preimages'/p.relative_to(ROOT);backup.parent.mkdir(parents=True,exist_ok=True);assert not backup.exists(),str(backup);backup.write_bytes(data)
 approved=next((r['AppliedSHA256'] for r in targets if r['CanonicalTarget']==rel(p)),None)
 if approved:assert sha(data)==approved
 if p==DAG/'DependencyEdges.csv':approved=json.loads((SCA/'DEPENDENCY_SNAPSHOT_MANIFEST.json').read_text())['files'][0]['sha256'];assert sha(data)==approved
 before.append({'path':rel(p),'original_group2_applied_sha256':approved or 'NOT_AN_ACCEPTED_POSTIMAGE; initial staged metadata','before_repair_sha256':sha(data),'before_repair_copy':rel(backup)})
for p in [ROOT/r['CanonicalTarget'] for r in targets if r['CanonicalTarget'].endswith('/Dependencies.csv')]+[DAG/'DependencyEdges.csv']:
 data=p.read_bytes();rs=read(p);ids=[r['DependencyID'] for r in rs if r['DependencyID'] in modified];assert ids
 first=data.find(b'v3.1,SCA011-');assert first>0;prefix=data[:first]
 with p.open(newline='') as f:fields=next(csv.reader(f))
 buf=io.StringIO(newline='');w=csv.DictWriter(buf,fieldnames=fields,lineterminator='\n')
 for r in rs:
  if r['DependencyID'] in modified:w.writerow({k:modified[r['DependencyID']][k] for k in fields})
 p.write_bytes(prefix+buf.getvalue().encode())
 newrs=read(p);assert len(newrs)==len(rs)
 for old,new in zip(rs,newrs):
  if old['DependencyID'] not in modified:assert old==new
  else:assert new=={k:modified[old['DependencyID']][k] for k in fields}
 assert p.read_bytes().startswith(prefix)
for r in targets:
 if not r['CanonicalTarget'].endswith('/_DEPENDENCIES.md'):continue
 p=ROOT/r['CanonicalTarget'];text=p.read_text()
 for old,new in mapping.items():text=text.replace(old,new)
 text+='\n## Group-2 metadata correction\n\nRoot SPEC §6.8 canonical DEP-owner identities and §6.5 verbatim local evidence have been applied to the added rows. Original SCA011 identities and proposal citations remain in CSV Notes and the ID crosswalk. The accepted ownership relations, prior rows and pending maturity are unchanged. Correction authority and exact field deltas: `projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/application/dependencies/CORRECTNESS_REPAIR.md`. At correction time, graph adoption remained subject to Group 3.\n'
 p.write_text(text)
writecsv(OUT/'ID_CROSSWALK.csv',crosswalk);writecsv(OUT/'FIELD_LEVEL_DELTA.csv',field_delta)
(OUT/'REPAIR_PREIMAGES.json').write_text(json.dumps({'files':before,'allocation_inputs':allocation_inputs},indent=2)+'\n')
# Future Group-3 transformations are rebound to repaired current bytes; their
# effect is previewed and hashed but never applied here.
original=json.loads((SCA/'dependencies/LOCAL_MIRRORS_PROMOTION.json').read_text());future=[]
for r in original['group3_files']:
 p=ROOT/r['target'];data=p.read_bytes();after=data
 for op in r['operations']:
  old=op['old'].encode();assert after.count(old)==op['expected_occurrences'],(r['target'],op['old']);after=after.replace(old,op['new'].encode())
 future.append({'target':r['target'],'applied_sha256':sha(data),'operations':r['operations'],'accepted_sha256':sha(after)})
(OUT/'REPAIRED_GROUP3_PROMOTION.json').write_text(json.dumps({'status':'PREPARED_FUTURE_GROUP3_TRANSFORMS_NOT_EXECUTED','condition':'Execute only after an actual Group-3 audited-poststate decision and recorded DAG-011 adoption. No transformation creates acceptance.','original_transforms':'dependencies/LOCAL_MIRRORS_PROMOTION.json#/group3_files','group3_files':future},indent=2)+'\n')
print(json.dumps({'repaired_rows':len(modified),'field_changes':len(field_delta),'crosswalk_rows':len(crosswalk),'local_files':20,'group3_applied':False}))
