#!/usr/bin/env python3
"""Verify all three repair stages, re-extract effects, prove multiset equality.
--record freezes this task's evidence; ordinary invocation verifies it.
"""
from pathlib import Path
import argparse,collections,csv,importlib.util,json,sys,re
sys.dont_write_bytecode=True
import repair
ROOT,PROJECT,RUN,OUT,sha=repair.ROOT,repair.PROJECT,repair.RUN,repair.OUT,repair.sha
ap=argparse.ArgumentParser();ap.add_argument('--record',action='store_true');args=ap.parse_args()
stages=[json.loads((OUT/name).read_text()) for name in ['PHYSICAL_EDITS.json','PHYSICAL_EDITS_STAGE2.json','PHYSICAL_EDITS_STAGE3.json']]
paths=sorted({f['path'] for s in stages for f in s['files']});final={p:(ROOT/p).read_text() for p in paths};basis=dict(final)
for stage in reversed(stages):
 for f in stage['files']:
  p=f['path'];text=basis[p];assert sha(text)==f['after_sha256'],('after-state drift',p)
  for e in reversed(f['edits']):
   assert sha(e['old'])==e['old_sha256'] and sha(e['new'])==e['new_sha256'];a,b=e['new_start_offset'],e['new_end_offset'];assert text[a:b]==e['new'];text=text[:a]+e['old']+text[b:]
  assert sha(text)==f['before_sha256'];basis[p]=text
replayed=dict(basis)
for stage in stages:
 for f in stage['files']:
  p=f['path'];text=replayed[p];assert sha(text)==f['before_sha256']
  for e in reversed(f['edits']):
   a,b=e['old_start_offset'],e['old_end_offset'];assert text[a:b]==e['old'];text=text[:a]+e['new']+text[b:]
  assert sha(text)==f['after_sha256'];replayed[p]=text
assert replayed==final
sp=importlib.util.spec_from_file_location('ex',RUN/'tools/extract_claims_v2.py');ex=importlib.util.module_from_spec(sp);sys.modules['ex']=ex;sp.loader.exec_module(ex)
classes={r['ClaimKey']:r for r in repair.csvread(RUN/'R3/CLASS_ASSIGNMENTS.csv')}
effects=[];sow=[]
sys.path.insert(0,str(ROOT/'tools/scope_of_work'));from common import resolve_production_format
for p in paths:
 path=ROOT/p;did=path.parent.name[:9];surface={'ScopeOfWork.md':'SOW','ArchitectureBasis.md':'AB','_CONTEXT.md':'CONTEXT','MEMORY.md':'MEMORY'}[path.name]
 def units(text):
  if surface=='MEMORY':return {did+':MEMORY':text}
  return {u.key:u.text for u in ex.markdown_units(did,surface,text,surface=='SOW')}
 pre,post=units(basis[p]),units(final[p])
 for k in sorted(set(pre)|set(post)):
  if pre.get(k)!=post.get(k):effects.append({'ClaimKey':k,'SourcePath':p,'Effect':'REMOVED' if k not in post else 'ADDED' if k not in pre else 'CHANGED','BeforeSHA256':sha(pre[k]) if k in pre else '', 'AfterSHA256':sha(post[k]) if k in post else '', 'DiscoveryClass':classes.get(k,{}).get('ClassID','NON_DIVERGENT_OR_STRUCTURE')})
 if surface=='SOW':
  r=resolve_production_format(path.parent);sow.append(dict(Path=p,Valid=r.valid,State=r.state,Issues=';'.join(r.issues)));assert r.valid,(p,r.issues)
 assert '/DEL-01-01_' not in p and path.name!='_STATUS.md'
expected_path=OUT/'FINAL_EXTRACTOR_EFFECTS.csv'
if args.record:repair.writecsv(expected_path,effects)
expected=repair.csvread(expected_path)
fields=list(effects[0])
identity=lambda rows:collections.Counter(tuple(r[f] for f in fields) for r in rows)
assert identity(expected)==identity(effects),'changed-reference multiset mismatch'
assert identity(expected[:-1])!=identity(effects),'missing-reference negative control failed'
assert identity(expected+[expected[0]])!=identity(effects),'duplicate-reference negative control failed'
# Each direct source key binds the precise changed span; mixed rows keep explicit partial outcomes.
rowkeys={r['Key'] for r in repair.csvread(RUN/'R4/R5_TRANCHE_PROPOSAL/R5_REPAIR_ROWS.csv')}
keypatches=collections.defaultdict(list)
for si,stage in enumerate(stages,1):
 for f in stage['files']:
  for e in f['edits']:
   for key in e['keys']:
    primary=key
    if 'A5' in e['classes'] and key.endswith(':CONTEXT#architecture-basis-injection') and key+'.s01' in rowkeys:primary=key+'.s01'
    keypatches[primary].append((si,f,e))
outcomes=[]
for key,ops in sorted(keypatches.items()):
 cl=sorted({c for _,_,e in ops for c in e['classes']});partial=bool(set(cl)&{'T4A-C03','T4B-C05'}) or ('T4A-C01' in cl and '#architecture-basis-injection' in key) or 'A3' in cl or 'A4' in cl
 why='Exact named text repair applied; no substantive acceptance or implementation closure.'
 if 'T4A-C01' in cl and '#architecture-basis-injection' in key:why='Pin lifted; sibling applicability/scope/TBD/lifecycle content remains governed independently.'
 if set(cl)&{'T4A-C03','T4B-C05'}:why='Only exact current-reference spans repaired; heterogeneous or historical portions retained for individual review.'
 if 'A3' in cl:why='Corrected sealed-brief/local mechanism authority; unrelated CP10 topics and formal review remain open.'
 if 'A4' in cl:why='Current standalone prose renamed; held code/contract identifiers and historical names preserved.'
 outcomes.append(dict(ClaimKey=key,DeliverableID=key[:9],Classes=';'.join(cl),Outcome='PARTIAL_REPAIR' if partial else 'DIRECT_REPAIRED',Posture=';'.join(sorted({e['posture'] for _,_,e in ops})),SourcePath=ops[0][1]['path'],ChangedSpanBeforeSHA256=';'.join(e['old_sha256'] for _,_,e in ops),ChangedSpanAfterSHA256=';'.join(e['new_sha256'] for _,_,e in ops),Reason=why))
if args.record:
 repair.writecsv(OUT/'SOURCE_KEY_OUTCOMES.csv',outcomes)
 repair.writecsv(OUT/'FINAL_SOURCE_BINDINGS.csv',[dict(Path=p,BeforeSHA256=sha(basis[p]),AfterSHA256=sha(final[p])) for p in paths])
 repair.writecsv(OUT/'SOW_VALIDATION_FINAL.csv',sow)
 report={'files':len(paths),'patches':sum(len(f['edits']) for s in stages for f in s['files']),'changed_extractor_refs':len(effects),'source_key_outcomes':dict(collections.Counter(r['Outcome'] for r in outcomes)),'sow_pass':len(sow),'inverse_replay':'PASS','changed_reference_multiset':'PASS','missing_negative_control':'REJECTED','duplicate_negative_control':'REJECTED','issued_and_status_writes':0}
 (OUT/'CHECK_REPORT.json').write_text(json.dumps(report,indent=2)+'\n')
 print(json.dumps(report,indent=2))
else:print('PASS: three-stage inverse/replay, exact changed-reference multiset, missing/duplicate negative controls, all changed SOW validators; protected targets untouched')
