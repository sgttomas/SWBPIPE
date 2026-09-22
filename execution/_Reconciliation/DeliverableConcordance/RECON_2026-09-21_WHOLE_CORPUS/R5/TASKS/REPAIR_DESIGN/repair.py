#!/usr/bin/env python3
"""Bounded R5 repairs. Build exact patches, apply once, invert/replay check.
Only --apply writes production. Frozen R0-R4 are never mutated.
"""
from pathlib import Path
import argparse,collections,csv,hashlib,importlib.util,json,re,sys
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parents[9]
PROJECT=ROOT/'projects/chirality-piping'
RUN=PROJECT/'execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS'
OUT=Path(__file__).resolve().parent
BASIS='00115c71931bcae79909602d653740d3bb72dfa1'
sha=lambda s: hashlib.sha256(s.encode()).hexdigest()
def csvread(p): return [r for r in csv.DictReader(p.open()) if next(iter(r.values()), '') != '#END']
def writecsv(p,rows):
 if rows:
  with p.open('w',newline='') as f:
   w=csv.DictWriter(f,fieldnames=list(rows[0]));w.writeheader();w.writerows(rows)

def build():
 keys={r['ClaimKey']:r for r in csvread(RUN/'CLAIM_KEYS_V2.csv')}
 rows=csvread(RUN/'R4/R5_TRANCHE_PROPOSAL/R5_REPAIR_ROWS.csv')
 spec=importlib.util.spec_from_file_location('extractor',RUN/'tools/extract_claims_v2.py');ex=importlib.util.module_from_spec(spec);sys.modules['extractor']=ex;spec.loader.exec_module(ex)
 originals={};changes=collections.defaultdict(dict);decisions=[]
 def get(p):
  if p not in originals:originals[p]=p.read_text()
  return originals[p]
 def unit(row):return keys.get(row['Key']) or keys.get(re.sub(r'\.s\d+$','',row['Key']))
 def editline(p,n,new,key,cl,posture,why):
  lines=get(p).splitlines(keepends=True);old=lines[n-1];new=new.rstrip('\n')+'\n'
  if old==new:return False
  entry=changes[p].setdefault((n,n),{'start':n,'end':n,'old':old,'new':new,'keys':[],'classes':[],'posture':posture,'reason':why})
  if entry['new']!=new:raise ValueError(('conflicting patch',p,n,entry,new))
  if key not in entry['keys']:entry['keys'].append(key)
  if cl not in entry['classes']:entry['classes'].append(cl)
  return True
 def remove(p,u,row):
  a,b=int(u['LineStart']),int(u['LineEnd']);lines=get(p).splitlines(keepends=True);old=''.join(lines[a-1:b])
  if sha('\n'.join(get(p).split('\n')[a-1:b]))!=u['TextSHA256']:raise ValueError(('stale A9 unit',u['ClaimKey']))
  replacement=''
  if 'PDU-054 current declaration' in u['Title']:
   tail=old[old.index('> Under `DEC-077`'):]
   replacement='### CLM-007 — Load-case material-selection requirements\n\n'+tail
  elif '> **Generated:**' in old:
   tail=old[old.index('> **Generated:**'):]
   replacement=old.splitlines()[0].split(' — ')[0]+' — Historical source provenance\n\n> Historical source metadata; no current-state assertion.\n>\n'+tail
  changes[p][(a,b)]={'start':a,'end':b,'old':old,'new':replacement,'keys':[row['Key']],'classes':['T4A-C02'],'posture':'b','reason':'A9: retire repeated dated current-declaration snapshot; frozen discovery preserves original evidence and common limits retained once per file.'}
 # C02: only actual declaration blocks, plus one schema-required frontmatter exception.
 a9files=set()
 for r in rows:
  if r['ClassID']!='T4A-C02':continue
  u=unit(r);p=ROOT/u['SourcePath']
  if u['Surface']=='STATUS':decisions.append(dict(Key=r['Key'],ClassID=r['ClassID'],Outcome='PARENT_OWNS_STATUS',Detail='A9 annotation delegated to status child'));continue
  if u['Surface']=='MEMORY':
   original=get(p);h='## D-41 R5 T7 PDU-055 current declaration'; n=next(i+1 for i,l in enumerate(original.splitlines()) if l==h)
   editline(p,n,h+'\n\nHistorical evidence only — superseded as a current declaration by the R5 A9 disposition (2026-09-22). The dated text below is preserved as historical evidence; it does not establish current authority, completion, lifecycle, review, validation, release, professional reliance, or code-compliance closure.',r['Key'],r['ClassID'],'b','Annotate retained history without rewriting historical words.')
  elif u['UnitKind']=='SURFACE':
   assert r['Key']=='DEL-04-05:SOW'
   n=next(i+1 for i,l in enumerate(get(p).splitlines()) if l.startswith('decomposition_basis:'))
   editline(p,n,'decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@'+BASIS,r['Key'],r['ClassID'],'a','Schema requires path@revision (tools/scope_of_work/common.py:229); preserve representation contract and bind accepted discovery decomp Rev0.12.')
  else:
   assert 'current declaration' in u['Title'];remove(p,u,r);a9files.add(p)
 # One derivative evidence note per file replaces 207 repeated claim blocks.
 for p in a9files:
  n=next(i+1 for i,l in enumerate(get(p).splitlines()) if l.startswith('# Scope of Work'))
  note='Reconciliation evidence: the retired D-41 current-declaration snapshots are preserved at source `'+BASIS+'` and in `execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/`. Current scope and decisions are resolved through `execution/_Decomposition/SOFTWARE_DECOMP.md` and `execution/_Coordination/_DECISIONS/_REGISTER.md`; dependency context is resolved through `execution/_DAG/_LATEST.md`. Open work and lifecycle remain governed by `_STATUS.md`. This text repair establishes no lifecycle, review, validation, release, professional-reliance, or code-compliance closure.'
  a9keys=[k for v in changes[p].values() if v['classes']==['T4A-C02'] for k in v['keys']]
  editline(p,n,get(p).splitlines()[n-1]+'\n\n'+note,a9keys[0],'T4A-C02','b','One evidence-reference note preserves common boundaries without keeping duplicated mechanism-level declarations.')
 # Pins: unambiguous named references in indexed C01/C03 units only. No general revision replacement.
 for r in rows:
  if r['ClassID'] not in {'T4A-C01','T4A-C03'}:continue
  u=unit(r)
  if not u:decisions.append(dict(Key=r['Key'],ClassID=r['ClassID'],Outcome='HELD',Detail='No reproducible extractor parent'));continue
  p=ROOT/u['SourcePath'];lines=get(p).splitlines();changed=False
  for n in range(int(u['LineStart']),int(u['LineEnd'])+1):
   if n>len(lines):continue
   l=lines[n-1]
   if any(a<=n<=b and a!=b for a,b in changes[p]):continue
   new=l
   if r['ClassID']=='T4A-C01':
    if l.startswith('- **Accepted Revision:**'):new='- **Authority basis:** Resolve `execution/_Decomposition/SOFTWARE_DECOMP.md` and its accepted amendments through `execution/_Coordination/_DECISIONS/_REGISTER.md`; source-state-bound evidence records the evaluated revision.'
    elif l.startswith('- **Decomposition Revision:**'):new='- **Decomposition Reference:** `execution/_Decomposition/SOFTWARE_DECOMP.md`; accepted decisions and amendments are recorded in `execution/_Coordination/_DECISIONS/_REGISTER.md`.'
    elif l.startswith('- **Approved DAG Context:**'):new='- **Approved DAG Context:** Resolve `execution/_DAG/_LATEST.md` for the accepted graph.'
   else:
    # Table/single-reference lines only; narrative/compound .sNN parents remain held.
    if ('SOFTWARE_DECOMP' in l and ('| Decomposition basis |' in l or '| Decomposition Basis |' in l or re.match(r'^> - `?execution/_Decomposition/SOFTWARE_DECOMP.md`? revision 0\.[789]$',l))):
     new=re.sub(r' revision 0\.[789]\b',' (accepted authority; see the project decision register)',l)
    elif l.startswith('Authority basis:'):
     new=re.sub(r' revision 0\.[789]\b',' (accepted authority; see `execution/_Coordination/_DECISIONS/_REGISTER.md`)',l)
   if new!=l:changed|=editline(p,n,new,r['Key'],r['ClassID'],'b','Replace duplicated dated authority metadata with the named authoritative home; preserve scope, decisions, held siblings, and evaluated evidence history.')
  if not changed:decisions.append(dict(Key=r['Key'],ClassID=r['ClassID'],Outcome='HELD',Detail='No unique snapshot-only line outside other edits; broader narrative/amendment/scope or minted sibling requires separate claim review'))
 # A5 identical obsolete premise only (including unminted copies): decisions now live in consolidated AB files.
 for p in sorted(PROJECT.glob('execution/PKG-*/1_Working/DEL-*/_CONTEXT.md')):
  if p.parent.name.startswith('DEL-01-01_'):continue
  did=p.parent.name[:9]
  for n,l in enumerate(get(p).splitlines(),1):
   if l.startswith('- **Architecture Basis:**') and 'at `SEMANTIC_READY` supplies dispatchable architecture-basis constraints' in l:
    editline(p,n,'- **Architecture Basis:** Applicable `PKG-00 - Software Architecture Runway` constraints are carried by the consolidated `ArchitectureBasis.md` references under D-43 / SCA-006 and their cited decisions. Use applicable basis IDs as context constraints; this reference makes no lifecycle or issuance assertion.',did+':CONTEXT#architecture-basis-injection','A5','b','D43/SCA006 consolidated PKG00 reference semantics replace obsolete SEMANTIC_READY premise without changing deliverable lifecycle or decision content.')
 for p in PROJECT.glob('execution/PKG-11*/1_Working/DEL-11-05*/ScopeOfWork.md'):
  for n,l in enumerate(get(p).splitlines(),1):
   if l.startswith('> SCA-001 allows `PKG-00` `SEMANTIC_READY` content'):
    editline(p,n,'> Applicable PKG-00 architecture constraints are supplied by the consolidated `ArchitectureBasis.md` references under D-43 / SCA-006 and their cited decisions. Contributors use applicable basis IDs as context constraints, including the no-bypass API/adapter baseline, without treating PKG-00 as `ISSUED` or copying full PKG-00 prose into new artifacts.','DEL-11-05:SOW#CLM-031','A5','b','Replace obsolete lifecycle-conditioned architecture reference; preserve no-bypass, no-issuance and no-copy requirements.')
 # Four-document residue: only exact combination and noun-phrase substitutions.
 grouped=[r'`Datasheet\.md`, `Specification\.md`, `Guidance\.md`, and `Procedure\.md`',r'`Datasheet\.md`, `Specification\.md`, `Guidance\.md`, `Procedure\.md`',r'`Datasheet\.md`, `Specification\.md`, `Procedure\.md`, `Guidance\.md`',r'Datasheet, Specification, Guidance, and Procedure',r'Datasheet, Specification, Guidance and Procedure']
 for r in rows:
  if r['ClassID']!='T4B-C05':continue
  if r['BlockedOnPacket']:
   decisions.append(dict(Key=r['Key'],ClassID=r['ClassID'],Outcome='HELD',Detail='Existing packet blocker '+r['BlockedOnPacket']));continue
  u=unit(r);p=ROOT/u['SourcePath'];lines=get(p).splitlines();changed=False
  for n in range(int(u['LineStart']),min(int(u['LineEnd']),len(lines))+1):
   l=lines[n-1]
   if any(a<=n<=b and a!=b for a,b in changes[p]):continue
   # Retained historical records and provenance are not rewritten to pretend they were SOWs.
   if any(w in l.lower() for w in ['sourcepath:','consolidat','histor','source bytes','source hashes','produced by','write scope','write targets','check_four_documents','four-documents','four-document check','four-document structure check','PASS for this setup','PASS |']):continue
   new=l
   for pat in grouped:new=re.sub(pat,'`ScopeOfWork.md`',new)
   new=re.sub(r'\b[Ff]our-document kit\b','`ScopeOfWork.md`',new)
   new=re.sub(r'\brequired four documents\b','required `ScopeOfWork.md`',new)
   if new!=l:
    if any(token in new for token in ['Four production documents','All four documents','Four-document setup kit','Four-document sections','Four documents exist','Four-document setup exists']):continue
    new=new.replace('`ScopeOfWork.md` exist and retain their default sections','`ScopeOfWork.md` exists and retains its required sections').replace('`ScopeOfWork.md` exist and retain the default required sections','`ScopeOfWork.md` exists and retains the required sections').replace('Default sections are present in `ScopeOfWork.md`','Required sections are present in `ScopeOfWork.md`').replace('`ScopeOfWork.md` use consistent','`ScopeOfWork.md` uses consistent').replace('`ScopeOfWork.md` record active','`ScopeOfWork.md` records active').replace('`ScopeOfWork.md` are present','`ScopeOfWork.md` is present').replace('`ScopeOfWork.md` exist','`ScopeOfWork.md` exists').replace('`ScopeOfWork.md` use the same','`ScopeOfWork.md` uses the same').replace('`ScopeOfWork.md`: `ScopeOfWork.md`','`ScopeOfWork.md`').replace('preserves `ScopeOfWork.md` sections','preserves its required sections').replace('`ScopeOfWork.md` existss','`ScopeOfWork.md` exists').replace('exists and remain consistent','exists and remains consistent').replace('exists and include default sections','exists and includes required sections').replace('exists and preserve default sections','exists and preserves required sections').replace('Four-document presence','Scope of Work presence').replace('Four-document consistency','Scope of Work consistency').replace('Four-doc consistency','Scope of Work consistency')
    if (n,n) in changes[p] and changes[p][(n,n)]['new'].rstrip('\n')!=new:continue
    changed|=editline(p,n,new,r['Key'],r['ClassID'],'b','D43 representation migration: name surviving ScopeOfWork surface while retaining substantive checks and boundaries; no historical provenance rewrite.')
  if not changed:decisions.append(dict(Key=r['Key'],ClassID=r['ClassID'],Outcome='HELD',Detail='No exact current-surface combination; component references, historical provenance, or task-specific scope require separate review'))
 # Serialize exact physical edits and reproduce all extractor references affected, including ancestors.
 files=[];effects=[]
 for p in sorted(changes):
  es=sorted(changes[p].values(),key=lambda e:(e['start'],e['end']))
  if not es:continue
  for i,e in enumerate(es):
   if i and es[i-1]['end']>=e['start']:raise ValueError(('overlap',p,es[i-1],e))
  old=originals[p];lines=old.splitlines(keepends=True);new=old
  for e in reversed(es):
   a=sum(map(len,lines[:e['start']-1]));b=sum(map(len,lines[:e['end']]));assert old[a:b]==e['old'];e.update(old_start_offset=a,old_end_offset=b,old_sha256=sha(e['old']),new_sha256=sha(e['new']))
   new=new[:a]+e['new']+new[b:]
  for e in es:
   e['new_start_offset']=e['old_start_offset']+sum(len(q['new'])-len(q['old']) for q in es if q['end']<e['start']);e['new_end_offset']=e['new_start_offset']+len(e['new'])
  did=p.parent.name[:9];surf={'ScopeOfWork.md':'SOW','ArchitectureBasis.md':'AB','_CONTEXT.md':'CONTEXT','MEMORY.md':'MEMORY'}[p.name]
  def units(text):
   if surf=='MEMORY':return {did+':MEMORY':text}
   return {u.key:u.text for u in ex.markdown_units(did,surf,text,surf=='SOW')}
  pre,post=units(old),units(new)
  for k in sorted(set(pre)|set(post)):
   if pre.get(k)!=post.get(k):effects.append(dict(ClaimKey=k,SourcePath=str(p.relative_to(ROOT)),Effect='REMOVED' if k not in post else 'ADDED' if k not in pre else 'CHANGED',BeforeSHA256=sha(pre[k]) if k in pre else '',AfterSHA256=sha(post[k]) if k in post else '',Accounting='Direct repair or changed enclosing extractor unit; see exact physical patches'))
  files.append(dict(path=str(p.relative_to(ROOT)),before_sha256=sha(old),after_sha256=sha(new),edits=es))
 (OUT/'PHYSICAL_EDITS.json').write_text(json.dumps(dict(source_basis=BASIS,files=files),indent=2)+'\n')
 writecsv(OUT/'EXTRACTOR_EFFECTS.csv',effects);writecsv(OUT/'HELD_AND_DELEGATED.csv',decisions)
 instr=['AGENTS.md','agents/AGENT_TASK.md','projects/chirality-piping/AGENTS.md','workflows/reconciliation/WORKFLOW.md','workflows/reconciliation/resources/method.md','docs/DELIVERABLE_CONCORDANCE_METHOD.md','projects/chirality-piping/docs/RECONCILIATION_PROFILE.md']
 writecsv(OUT/'INSTRUCTION_BASIS.csv',[dict(Path=p,SHA256=hashlib.sha256((ROOT/p).read_bytes()).hexdigest(),Origin='supplied TASK basis; native collaboration /root/piping_manager/repair_design') for p in instr])
 print(json.dumps(dict(files=len(files),edits=sum(len(f['edits']) for f in files),effects=len(effects),outcomes=len(decisions),class_keys={c:len({k for f in files for e in f['edits'] if c in e['classes'] for k in e['keys']}) for c in sorted({c for f in files for e in f['edits'] for c in e['classes']})}),indent=2))

def replay(apply):
 data=json.loads((OUT/'PHYSICAL_EDITS.json').read_text());n=0
 for f in data['files']:
  p=ROOT/f['path'];cur=p.read_text()
  if sha(cur)==f['before_sha256']:
   before=cur;after=cur
   for e in reversed(f['edits']):
    assert sha(e['old'])==e['old_sha256'] and sha(e['new'])==e['new_sha256'];assert after[e['old_start_offset']:e['old_end_offset']]==e['old'];after=after[:e['old_start_offset']]+e['new']+after[e['old_end_offset']:]
  elif sha(cur)==f['after_sha256']:
   after=cur;before=cur
   for e in reversed(f['edits']):
    assert before[e['new_start_offset']:e['new_end_offset']]==e['new'];before=before[:e['new_start_offset']]+e['old']+before[e['new_end_offset']:]
  else:raise ValueError(('unrecognized file basis',f['path']))
  assert sha(before)==f['before_sha256'] and sha(after)==f['after_sha256']
  assert '/DEL-01-01_' not in f['path'] and p.name!='_STATUS.md'
  if apply and cur!=after:p.write_text(after)
  n+=1
 print(json.dumps(dict(checked_files=n,inverse_and_forward='PASS',production_written=apply)))
if __name__=='__main__':
 ap=argparse.ArgumentParser();ap.add_argument('--build',action='store_true');ap.add_argument('--apply',action='store_true');ap.add_argument('--check',action='store_true');a=ap.parse_args()
 if a.build:build()
 if a.apply or a.check:replay(a.apply)
