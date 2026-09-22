#!/usr/bin/env python3
"""Stage4 dry-run proposal. --apply requires parent's explicit go signal."""
from pathlib import Path
import collections,csv,hashlib,importlib.util,io,json,re,sys
sys.dont_write_bytecode=True
HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE.parent));import repair
ROOT,PROJECT,RUN,BASIS,sha=repair.ROOT,repair.PROJECT,repair.RUN,repair.BASIS,repair.sha
sp=importlib.util.spec_from_file_location('ex',RUN/'tools/extract_claims_v2.py');ex=importlib.util.module_from_spec(sp);sys.modules['ex']=ex;sp.loader.exec_module(ex)
REG='execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY/'
REVIEW=REG+'instances/N7_FINAL_REVIEW/V2_BACKCHECK/REVIEW_RETURN_V2.md'
FANIN=REG+'PRECOMMIT_PARENT_FAN_IN_V1.md'
REFERENCE='N7 V2 PASS and bounded parent fan-in are recorded in `'+REVIEW+'` and `'+FANIN+'` for their exact frozen source. This is historical bounded review evidence; later source changes require affected review, and lifecycle, release, professional-reliance and existing residual gates remain unchanged.'
source_keys={r['ClaimKey']:r for r in repair.csvread(RUN/'CLAIM_KEYS_V2.csv')}
selected=[r for r in repair.csvread(RUN/'R4/R5_TRANCHE_PROPOSAL/R5_REPAIR_ROWS.csv') if r['ClassID']=='T5B-C03']
assert len(selected)==15
originals={};edits=collections.defaultdict(dict)
def text(p):
 if p not in originals:originals[p]=p.read_text()
 return originals[p]
def replace(p,n,new,keys,posture,why):
 old=text(p).splitlines(keepends=True)[n-1];new=new.rstrip('\n')+'\n'
 if old==new:return
 assert (p,n) not in edits[p]
 if n in edits[p]:
  assert edits[p][n]['new']==new
  edits[p][n]['keys']=list(dict.fromkeys(edits[p][n]['keys']+keys));return
 edits[p][n]=dict(start=n,end=n,old=old,new=new,keys=keys,classes=['T5B-C03'] if posture=='b' else ['SCHEMA_DECOMPOSITION_PIN'],posture=posture,reason=why)
def matchreplace(p,predicate,new,keys,why):
 hits=[(i+1,l) for i,l in enumerate(text(p).splitlines()) if predicate(l)]
 assert len(hits)==1,(p,hits)
 n,l=hits[0];replace(p,n,new(l) if callable(new) else new,keys,'b',why)
def target(did,name='ScopeOfWork.md'):return next(PROJECT.glob('execution/PKG-*/1_Working/'+did+'*/'+name))
# Required source-bound schema metadata; one SoW already carries the accepted basis.
pin_noops=[]
for p in sorted(PROJECT.glob('execution/PKG-*/1_Working/DEL-*/ScopeOfWork.md')):
 if p.parent.name.startswith('DEL-01-01_'):continue
 ls=text(p).splitlines();nn=[i+1 for i,l in enumerate(ls) if l.startswith('decomposition_basis:')];assert len(nn)==1
 new='decomposition_basis: projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md@'+BASIS
 if ls[nn[0]-1]==new:pin_noops.append(p.parent.name[:9]+':SOW')
 else:replace(p,nn[0],new,[p.parent.name[:9]+':SOW'],'a','Explicit schema-contract exception: preserve required path@revision metadata and use one accepted discovery basis (Rev0.12); evaluated-source binding, not scope reacceptance.')
# Six SoW snapshot claims. Historical numbers are kept as history, never updated to a fresh magic count.
p=target('DEL-04-06')
for old,new in [('### CLM-008 — Evidence Snapshot','### CLM-008 — Historical verification snapshot'),('> ##### Evidence Snapshot','> ##### Historical verification snapshot')]:matchreplace(p,lambda l,o=old:l==o,new,['DEL-04-06:SOW#CLM-008'],'Label historical evidence; preserve the recorded 19-test outcome and all deferrals.')
matchreplace(p,lambda l:l.startswith('> - `_run_records/TASK_RUN_2026-06-05_0736_') and '19 tests' in l,lambda l:l+' This is the recorded result for that run\'s source; it is not a current suite count or engineering-validation claim.',['DEL-04-06:SOW#CLM-008'],'Bind old result to its actual event without inventing a new result.')
matchreplace(p,lambda l:l.startswith('> | REQ-04-06-001 |') and '19 tests' in l,'> | REQ-04-06-001 | Verification hook: `cargo test --manifest-path core/solver/diagnostics/Cargo.toml --locked`, with result and source recorded in the governing run evidence. The June 5 Worker B record is historical evidence of 19 passing tests on its bound source, not a result for a later source basis. |',['DEL-04-06:SOW#CLM-015/REQ-04-06-001'],'Retain named verification hook; move dated count to historical evidence.')
matchreplace(p,lambda l:l.startswith('> - Recent validation evidence in `_run_records/TASK_RUN_2026-06-05'),'> - Source-bound verification evidence for the diagnostics test suite; `_run_records/TASK_RUN_2026-06-05_0736_TP-DEL-04-03-04-06_SUPPORT-BOUNDARY-HARDENING_B.md` is the historical June 5 result. A later candidate uses evidence bound to its own source; unit-test results do not establish engineering validation.',['DEL-04-06:SOW#CLM-019'],'Preserve evidence prerequisite while distinguishing verification from validation and historical from current source.')
p=target('DEL-05-01')
matchreplace(p,lambda l:l.startswith('> | Current crate tests |'),'> | Verification hook | Run the source-bound suite in `core/loads/primitive_loads/src/lib.rs` for primitive categories, boundary metadata, retired dimension aliases, equivalent-static handling, load-case records, deterministic sorting, lumping, axial effects, diagnostic records, missing/invalid inputs, and solver-vector assembly findings; record the evaluated inventory and result in the run evidence. |',['DEL-05-01:SOW#CLM-008'],'Preserve coverage obligations and named suite while removing contingent test-count snapshot.')
matchreplace(p,lambda l:l.startswith('> | Current validation evidence |'),lambda l:l.replace('Current validation evidence','Historical verification evidence').replace('; doc-alignment',' on that run\'s source; this is not a current suite count or engineering-validation result. Doc-alignment'),['DEL-05-01:SOW#CLM-008'],'Retain historical 40-test result, correct evidence category, preserve diff-hygiene duty.')
matchreplace(p,lambda l:l.startswith('> | REQ-05-01-009 |') and '40-test' in l,'> | REQ-05-01-009 | Verification uses the source-bound primitive-load suite (`cargo test --manifest-path core/loads/primitive_loads/Cargo.toml --locked`); counts and outcomes belong to the corresponding run record. Broader release gates remain under project validation/review authority. |',['DEL-05-01:SOW#CLM-016/REQ-05-01-009'],'Lift transient suite count while keeping the check and broader release authority.')
p=target('DEL-17-02')
matchreplace(p,lambda l:l.startswith('### CLM-033 — Current validator-path'),lambda l:l.replace('Current validator-path','Historical validator-path'),['DEL-17-02:SOW#CLM-033'],'Retain July12 outcomes as historical evidence and preserve RF001 human disposition.')
matchreplace(p,lambda l:l.startswith('> ###### Current validator-path'),lambda l:l.replace('Current validator-path','Historical validator-path'),['DEL-17-02:SOW#CLM-033'],'Keep stable CLM identity; only historical-evidence label changes.')
matchreplace(p,lambda l:'review record with `HumanDisposition=TBD`; this current evidence' in l,lambda l:l.replace('this current evidence','this historical evidence'),['DEL-17-02:SOW#CLM-033'],'No new review outcome asserted; still-required human disposition preserved.')
# Nine DEL07-09 claims: exact formerly-pending event, bounded to independently read review/fan-in records.
for name,surf in [('Palette_Operation_Routing.md','PALETTE_OPERATION_ROUTING'),('Palette_Organization_Contract.md','PALETTE_ORGANIZATION_CONTRACT')]:
 p=target('DEL-07-09',name);units=ex.markdown_units('DEL-07-09',surf,text(p),False)
 for n,l in enumerate(text(p).splitlines(),1):
  new=l
  replacements={
   'Current App integration is frozen at':'The historical App integration snapshot is recorded at',
   'Current aggregate source freeze:':'Historical reviewed source freeze:',
   'This current derivative amendment records':'This derivative amendment records',
   'A2_COVERAGE_FINAL freezes this docs slice with':'A2_COVERAGE_FINAL recorded the historical docs freeze with',
   'final root N7 full source/docs review and integrated acceptance are **pending**.':'the source-bound N7 V2 review and bounded parent fan-in are recorded below.',
   'The accepted module snapshots do not assert that later App integration or subsequent module amendments passed N7.':'The accepted module snapshots alone do not assert that later App integration or subsequent module amendments passed N7; the later N7 V2 record supports only its exact frozen source.',
   'final integration review pending':'historical N7 V2 PASS and bounded fan-in recorded; later source changes require affected review',
   'current UI implementation review remains pending':'historical N7 V2 PASS and bounded fan-in recorded; later source changes require affected review',
   'Root integrated N7 rereview remains pending.':REFERENCE,
   'Final N7 integrated source/docs rereview remains pending;':'The historical N7 V2 source/docs review returned PASS and bounded parent fan-in was recorded; later source changes require affected review;',
   'root N7 final rereview remains pending.':REFERENCE,
   'Root final integrated rereview and parent fan-in remain pending, with lifecycle, D58, SOW-070, R1–R3 and bounded toolkit residuals unchanged.':REFERENCE+' D58, SOW-070, R1–R3 and bounded toolkit residuals remain unchanged.',
  }
  if l=='## N7 F1/F2/F3 repair amendment — final rereview pending':new='## N7 F1/F2/F3 repair amendment — source-bound review recorded'
  else:
   for a,b in replacements.items():new=new.replace(a,b)
  if new!=l:
   containers=[u for u in units if u.start<=n<=u.end];nearest=min(containers,key=lambda u:(u.end-u.start,0 if u.kind=='ITEM' else 1));keys=[nearest.key,'DEL-07-09:'+surf]
   if '#n7-f1-f2-f3-' in nearest.key:keys.append('DEL-07-09:'+surf+'#n7-f1-f2-f3-repair-amendment-final-rereview-pend.s04')
   replace(p,n,new,list(dict.fromkeys(keys)),'b','Record actual N7 V2 PASS and accepted bounded fan-in, tied to exact historical source; preserve operations, findings history, residuals and subsequent review duties.')
# CSV event status only; all historical fields, route semantics, residuals, row identity and deferred status unchanged.
for name in ['Vocabulary_Coverage.csv','Capability_Comparison.csv']:
 p=target('DEL-07-09',name);raw=text(p);reader=list(csv.reader(io.StringIO(raw)));hdr=reader[0];lines=raw.splitlines();assert len(reader)==len(lines)
 for n,row in enumerate(reader[1:],2):
  newrow=list(row)
  for field,old,new in [('CurrentAcceptance','IMPLEMENTATION_EVIDENCE_ONLY_N7_PENDING','HISTORICAL_N7_V2_PASS_BOUNDED_FAN_IN_NOT_LIFECYCLE'),('current_coverage_state','BOUNDED_IMPLEMENTATION_N7_PENDING','BOUNDED_IMPLEMENTATION_HISTORICAL_N7_V2_PASS'),('current_derivative_status','implementation/source evidence; final N7 integration review pending; not lifecycle acceptance','N7 V2 PASS and bounded fan-in recorded for exact historical source; not lifecycle acceptance; later source changes require affected review')]:
   if field in hdr and newrow[hdr.index(field)]==old:newrow[hdr.index(field)]=new
  if newrow==row:continue
  basisfield='CurrentEvidenceBasis' if name=='Vocabulary_Coverage.csv' else 'current_basis';newrow[hdr.index(basisfield)]+=';'+REVIEW+';'+FANIN
  b=io.StringIO();csv.writer(b,lineterminator='\n').writerow(newrow)
  replace(p,n,b.getvalue(),['DEL-07-09:ROWS','DEL-07-09:ROWS/ROW-'+row[0]],'b','Only current derivative event-status and evidence-reference cells change; historical columns, accepted scope, actual routes, residuals, and deferred classes are byte-value preserved.')
files=[]
for p in sorted(edits):
 before=text(p);lines=before.splitlines(keepends=True);es=sorted(edits[p].values(),key=lambda e:e['start']);after=before
 for e in reversed(es):
  a=sum(map(len,lines[:e['start']-1]));b=a+len(e['old']);assert before[a:b]==e['old'];e.update(old_start_offset=a,old_end_offset=b,old_sha256=sha(e['old']),new_sha256=sha(e['new']));after=after[:a]+e['new']+after[b:]
 for e in es:e['new_start_offset']=e['old_start_offset']+sum(len(q['new'])-len(q['old']) for q in es if q['end']<e['start']);e['new_end_offset']=e['new_start_offset']+len(e['new'])
 files.append(dict(path=str(p.relative_to(ROOT)),before_sha256=sha(before),after_sha256=sha(after),edits=es))
# Prepare only. No production writes occur in this entrypoint.
manifest=HERE/'PROPOSED_PHYSICAL_EDITS_STAGE4_V2.json';assert not manifest.exists(),'Do not overwrite prepared evidence; create a successor if basis changes.'
manifest.write_text(json.dumps(dict(status='PREPARED_NOT_APPLIED',source_basis='final stages1-3 plus current parent status; current file hashes enforced',files=files),indent=2)+'\n')
report=dict(status='PREPARED_NOT_APPLIED',files=len(files),physical_edits=sum(len(f['edits']) for f in files),sow_pin_changes=sum(len([e for e in f['edits'] if e['posture']=='a']) for f in files),already_bound_pin_noops=pin_noops,t5b_c03_source_rows=15,primary_sources={p:hashlib.sha256((PROJECT/p).read_bytes()).hexdigest() for p in [REVIEW,FANIN]})
(HERE/'PREPARATION_REPORT_V2.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
