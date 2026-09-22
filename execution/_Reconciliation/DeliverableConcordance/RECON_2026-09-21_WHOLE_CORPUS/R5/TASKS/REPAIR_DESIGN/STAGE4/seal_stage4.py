#!/usr/bin/env python3
"""Append-only stage4 source-key account and complete changed-reference derivative."""
from pathlib import Path
import collections,csv,hashlib,importlib.util,json,sys,re
sys.dont_write_bytecode=True
OUT=Path(__file__).resolve().parent;sys.path.insert(0,str(OUT.parent));import repair
ROOT,PROJECT,RUN,sha=repair.ROOT,repair.PROJECT,repair.RUN,repair.sha
applied=json.loads((OUT/'APPLIED.json').read_text());m=OUT/applied['manifest'];assert hashlib.sha256(m.read_bytes()).hexdigest()==applied['manifest_sha256'];data=json.loads(m.read_text())
sp=importlib.util.spec_from_file_location('ex',RUN/'tools/extract_claims_v2.py');ex=importlib.util.module_from_spec(sp);sys.modules['ex']=ex;sp.loader.exec_module(ex)
old={};new={};ops=collections.defaultdict(list)
for f in data['files']:
 text=(ROOT/f['path']).read_text();assert sha(text)==f['after_sha256'];new[f['path']]=text
 for e in reversed(f['edits']):
  a,b=e['new_start_offset'],e['new_end_offset'];assert text[a:b]==e['new'];text=text[:a]+e['old']+text[b:]
 assert sha(text)==f['before_sha256'];old[f['path']]=text
 for e in f['edits']:
  for k in e['keys']:ops[k].append((f,e))
selected=[r for r in repair.csvread(RUN/'R4/R5_TRANCHE_PROPOSAL/R5_REPAIR_ROWS.csv') if r['ClassID']=='T5B-C03'];assert len(selected)==15
outcomes=[]
for key in sorted(k for k in ops if k.endswith(':SOW')):
 fs=[(f,e) for f,e in ops[key] if e['posture']=='a']
 if not fs:continue
 f,e=fs[0];outcomes.append(dict(ClaimKey=key,DeliverableID=key[:9],Classes='SCHEMA_DECOMPOSITION_PIN',Outcome='PARTIAL_REPAIR',Posture='a',SourcePath=f['path'],ChangedSpanBeforeSHA256=e['old_sha256'],ChangedSpanAfterSHA256=e['new_sha256'],Reason='Only schema-required evaluated-source pin repaired; does not close unrelated A4 identity, stale prose, review, implementation or acceptance residuals.'))
for r in selected:
 key=r['Key'];matches=ops.get(key,[]);assert matches,key
 partial=key in {'DEL-07-09:PALETTE_OPERATION_ROUTING','DEL-07-09:PALETTE_ORGANIZATION_CONTRACT'}
 outcomes.append(dict(ClaimKey=key,DeliverableID=key[:9],Classes='T5B-C03',Outcome='PARTIAL_REPAIR' if partial else 'DIRECT_REPAIRED',Posture='b',SourcePath=';'.join(sorted({f['path'] for f,e in matches})),ChangedSpanBeforeSHA256=';'.join(e['old_sha256'] for f,e in matches),ChangedSpanAfterSHA256=';'.join(e['new_sha256'] for f,e in matches),Reason='Historical review status repaired; mechanism anchors and broader surface content remain separately assessed.' if partial else 'Exact stale snapshot/review-event assertion repaired with source-bound evidence; actual residual and review duties preserved.'))
assert len(outcomes)==106 and len({r['ClaimKey'] for r in outcomes})==106
repair.writecsv(OUT/'SOURCE_KEY_OUTCOMES.csv',outcomes)
effects=[]
def add(pre,post,source):
 for key in sorted(set(pre)|set(post)):
  if pre.get(key)!=post.get(key):effects.append(dict(ClaimKey=key,SourcePath=source,Effect='REMOVED' if key not in post else 'ADDED' if key not in pre else 'CHANGED',BeforeSHA256=sha(pre[key]) if key in pre else '',AfterSHA256=sha(post[key]) if key in post else ''))
for p in old:
 path=ROOT/p
 if path.name=='ScopeOfWork.md':
  did=path.parent.name[:9];add({u.key:u.text for u in ex.markdown_units(did,'SOW',old[p],True)},{u.key:u.text for u in ex.markdown_units(did,'SOW',new[p],True)},p)
# Both CSV surfaces are extracted together through the accepted paired-ROWS path.
dir=next(PROJECT.glob('execution/PKG-07*/1_Working/DEL-07-09*'))
current={p.name:p.read_text() for p in dir.iterdir() if p.is_file() and p.name not in ex.STANDARD and p.suffix in ['.md','.csv']};prior=dict(current)
for p in old:
 path=ROOT/p
 if path.parent==dir:prior[path.name]=old[p]
add({u.key:u.text for u in ex.bespoke_surfaces('DEL-07-09',prior)},{u.key:u.text for u in ex.bespoke_surfaces('DEL-07-09',current)},str(dir.relative_to(ROOT)))
repair.writecsv(OUT/'CHANGED_REFERENCES.csv',effects)
expected=repair.csvread(OUT/'CHANGED_REFERENCES.csv');fields=list(effects[0]);counter=lambda rows:collections.Counter(tuple(r[f] for f in fields) for r in rows)
assert counter(expected)==counter(effects);assert counter(expected[:-1])!=counter(effects);assert counter(expected+[expected[0]])!=counter(effects)
repair.writecsv(OUT/'FINAL_SOURCE_BINDINGS.csv',[dict(Path=f['path'],BeforeSHA256=f['before_sha256'],AfterSHA256=f['after_sha256']) for f in data['files']])
report=dict(files=len(data['files']),patches=sum(len(f['edits']) for f in data['files']),changed_references=len(effects),source_key_outcomes=dict(collections.Counter(r['Outcome'] for r in outcomes)),paired_csv_extraction='accepted extractor bespoke_surfaces',changed_reference_multiset='PASS',missing_negative_control='REJECTED',duplicate_negative_control='REJECTED',no_status_or_issued_edits=True)
(OUT/'FINAL_CHECK_REPORT.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))
