#!/usr/bin/env python3
"""Reproduce this continuation's records and changed-source overlays only.

Run without --check to materialize derived evidence in this home; --check compares
it without rewriting earlier evidence. Git supplies preimages, not copied trees.
"""
import argparse, collections, csv, difflib, hashlib, importlib.util, io, json, re, subprocess, sys
from pathlib import Path
sys.dont_write_bytecode=True
OUT=Path(__file__).resolve().parent
ROOT=Path(subprocess.check_output(['git','rev-parse','--show-toplevel'],text=True).strip())
PROJECT=ROOT/'projects/chirality-piping'; RUN=OUT.parent
BASE='1b5adbf50142a4c01c454c62a31dfcdc60da1894'
POST=PROJECT/'execution/_ScopeChange/_PostAcceptanceValidation/SCA-011_20260922T173404Z'
SURFACES={'ScopeOfWork.md':'SOW','ArchitectureBasis.md':'AB','_CONTEXT.md':'CONTEXT','_STATUS.md':'STATUS','MEMORY.md':'MEMORY','_REFERENCES.md':'REFERENCES'}
spec=importlib.util.spec_from_file_location('piping_record_extractor',RUN/'tools/extract_claims_v2.py')
ex=importlib.util.module_from_spec(spec);sys.modules[spec.name]=ex;spec.loader.exec_module(ex)
def sha(b):return hashlib.sha256(b.encode()if isinstance(b,str)else b).hexdigest()
def git(*args):return subprocess.check_output(['git','-C',str(ROOT),*args])
def rel(p):return str(p.relative_to(ROOT))
def rows(p):return list(csv.DictReader(p.open()))
def units(path,text):
 match=re.search(r'/(DEL-\d\d-\d\d)[^/]*/',path)
 did=match.group(1)if match else'CARRIER:'+path; surface=SURFACES.get(Path(path).name,re.sub(r'[^A-Za-z0-9]+','_',Path(path).stem).upper()if match else'DOCUMENT')
 if surface=='STATUS': us=ex.status_units(did,text)
 elif surface=='MEMORY':us=[ex.Unit(did+':MEMORY','SURFACE','',1,len(text.splitlines()),text,'MEMORY')]
 else:us=ex.markdown_units(did,surface,text,surface=='SOW')
 return {u.key:u for u in us}
def csvtext(data,fields=None):
 s=io.StringIO();w=csv.DictWriter(s,fieldnames=fields or list(data[0]),lineterminator='\n');w.writeheader();w.writerows(data);return s.getvalue()
def lifecycle(text):
 m=re.search(r'^\*\*Current State:\*\*\s*(\S+)',text,re.M)
 if not m:m=re.search(r'^Current State:\s*(\S+)',text,re.M)
 if not m:m=re.search(r'(?im)^.*current state[^\n]*',text)
 assert m,'Missing current lifecycle'
 return m.group(1)if m.lastindex else m.group(0)
def quote_section(text,locus):
 if locus.startswith('frontmatter/'):
  field=locus.split('/',1)[1];return next(x for x in text.splitlines()if x.startswith(field+':'))
 if locus.startswith('CLM-')or locus=='Governing Values and Decisions':
  m=re.search(r'^#{2,3} '+re.escape(locus)+r'[^\n]*$',text,re.M);assert m,locus
  tail=text[m.end():];n=re.search(r'^#{1,3} ',tail,re.M);return tail[:n.start()]if n else tail
 return '\n'.join(x for x in text.splitlines()if locus in x)
def build(probe=None):
 original=rows(RUN/'BACKCHECK/R6_2026-09-22/CLAIM_DISPOSITIONS.csv')
 selected={r['ClaimKey']:r for r in original if r['OriginalRoute']=='R5_RECORD_REPAIR'and r['R5Outcome']in('HELD_UNCHANGED','PARTIAL_REPAIR','CHANGED_BINDING_RESIDUAL_RETAINED')}
 assert len(selected)==1685
 accounting=[];physical=[];supplemental=[]
 for batch in ('PKG00_06','PKG07_12','PKG13_17'):
  accounting+=rows(OUT/batch/'ACCOUNTING.csv');physical+=json.loads((OUT/batch/'PHYSICAL_EDITS.json').read_text())['files']
  if(OUT/batch/'SUPPLEMENTAL_ACCOUNTING.csv').exists():supplemental+=rows(OUT/batch/'SUPPLEMENTAL_ACCOUNTING.csv')
 if (OUT/'INTEGRATION_EDITS.json').exists():physical+=json.loads((OUT/'INTEGRATION_EDITS.json').read_text())['files']
 if probe=='missing-key':accounting=accounting[1:]
 count=collections.Counter(r['ClaimKey']for r in accounting)
 assert count==collections.Counter(selected.keys()),'Selected accounting key mismatch'
 corpus={r['ClaimKey']:r for r in original};assert len(corpus)==9889
 suppkeys=[r['ClaimKey']for r in supplemental]
 assert len(suppkeys)==len(set(suppkeys))and not(set(suppkeys)&selected.keys())and set(suppkeys)<=corpus.keys(),'Invalid supplemental population'
 accounting+=supplemental
 permitted={'REPAIRED_CURRENT_RECORD','NO_CHANGE_HISTORICAL_EVIDENCE','NO_CHANGE_CURRENT_REQUIREMENT','RESERVED_AUTHORITY','DELIVERY_TASK_RETAINED'}
 for row in accounting:
  assert row['Disposition']in permitted,row
  assert row['Reason']and row['Authority']and row['Evidence'],row['ClaimKey']
  assert row['DeliverableID']==corpus[row['ClaimKey']]['DeliverableID']
  if row['Disposition']in('DELIVERY_TASK_RETAINED','RESERVED_AUTHORITY'):assert row['RemainingAction'],row['ClaimKey']
 for row in accounting:
  source_paths=row['SourcePath'].split(';')
  assert all((ROOT/p).is_file()for p in source_paths),('Missing accounting source',row['ClaimKey'],source_paths)
  row['CurrentSourceSHA256']=';'.join(sha((ROOT/p).read_bytes())for p in source_paths)
  row['OriginalR5Outcome']=corpus[row['ClaimKey']]['R5Outcome']
  row['OriginalRoute']=corpus[row['ClaimKey']]['OriginalRoute']
  row['OverlayPopulation']='PRIMARY_RECORD_REPAIR'if row['ClaimKey']in selected else'SUPPLEMENTAL_CARRIER_REPAIR'
 accounting.sort(key=lambda r:r['ClaimKey'])
 paths=[r['path']for r in physical];assert len(paths)==len(set(paths)),'Overlapping author file writes'
 changed_paths=git('diff','--name-only',BASE,'--',rel(PROJECT/'execution')).decode().splitlines()
 deliverable_paths=[p for p in changed_paths if '/1_Working/DEL-'in p]
 assert set(deliverable_paths)<=set(paths),('Unmanifested deliverable change',set(deliverable_paths)-set(paths))
 sources=[];changed=[];span_keys=set();full_keys=set()
 for file in sorted(physical,key=lambda r:r['path']):
  p=file['path'];before=git('show',BASE+':'+p);after=(ROOT/p).read_bytes()
  assert sha(before)==file['before_sha256']and sha(after)==file['after_sha256'],('File binding',p)
  assert '/DEL-01-01_'not in p, 'ISSUED modified'
  reconstructed=before.decode()
  for edit in file['edits']:
   old,new=edit['before'],edit['after'];assert old!=new,p
   if 'occurrences'in edit:assert reconstructed.count(old)==edit['occurrences'],(p,old[:80])
   else:assert reconstructed.count(old)==1,(p,'ambiguous replacement',old[:80],reconstructed.count(old))
   reconstructed=reconstructed.replace(old,new)
   assert edit['reason']and edit['posture']in ('a','b')
  assert reconstructed.encode()==after,('Physical edit replay',p)
  sources.append({'SourcePath':p,'BeforeSHA256':sha(before),'AfterSHA256':sha(after),'Baseline':BASE})
  if Path(p).suffix!='.md':continue
  old,new=units(p,before.decode()),units(p,after.decode())
  actual={k for k in old.keys()|new.keys()if k not in old or k not in new or old[k].text!=new[k].text}
  edits=[o for o in difflib.SequenceMatcher(None,before.decode().split('\n'),after.decode().split('\n'),autojunk=False).get_opcodes()if o[0]!='equal']
  spans={k for k,u in old.items()if any((a<u.end and b>u.start-1)if a!=b else u.start-1<=a<u.end for _,a,b,_,_ in edits)and(k not in new or u.text!=new[k].text)}| (new.keys()-old.keys())
  # Insertions can rebind ordinal row IDs even when that row's bytes moved intact.
  new_spans={k for k,u in new.items()if any((c<u.end and d>u.start-1)if c!=d else u.start-1<=c<u.end for _,_,_,c,d in edits)and(k not in old or u.text!=old[k].text)}
  spans.update(new_spans)
  old_text={u.text for u in old.values()};new_text={u.text for u in new.values()}
  spans.update(k for k in old.keys()&new.keys()if old[k].text!=new[k].text and(old[k].text in new_text or new[k].text in old_text))
  # Historical .sNN rows were minted under a recoverable parent envelope.
  for key in corpus:
   parent=re.sub(r'\.s\d\d$','',key)
   if parent!=key and key not in old and key not in new and parent in actual:actual.add(key)
   if parent!=key and key not in old and key not in new and parent in spans:spans.add(key)
  if probe=='missing-changed-unit'and actual:actual.remove(sorted(actual)[0])
  assert actual==spans,('Changed key extraction/span disagreement',p,actual^spans)
  full_keys.update(actual);span_keys.update(spans)
  for key in sorted(actual):
   bound=key if key in old or key in new else re.sub(r'\.s\d\d$','',key)
   a,b=old.get(bound),new.get(bound)
   changed.append({'ClaimKey':key,'SourcePath':p,'BoundReference':bound,'Binding':'EXACT_UNIT'if bound==key else'PARENT_ENVELOPE','BeforeSHA256':sha(a.text)if a else'','AfterSHA256':sha(b.text)if b else'','CurrentPresence':'PRESENT'if b else'REMOVED','AfterLine':b.start if b else'','SelectedOriginalRepair':'PRIMARY'if key in selected else'SUPPLEMENTAL'if key in suppkeys else'NO_CONTAINER_OR_COLLATERAL'})
 # Keyed CSV companion rows have one combined original ROWS identity.
 for directory in sorted({str(Path(p).parent)for p in paths if Path(p).suffix=='.csv'}):
  names=sorted(q.name for q in(ROOT/directory).glob('*.csv')if q.name not in ex.STANDARD)
  did=Path(directory).name[:9]
  old={u.key:u for u in ex.bespoke_surfaces(did,{n:git('show',BASE+':'+directory+'/'+n).decode()for n in names})}
  new={u.key:u for u in ex.bespoke_surfaces(did,{n:(ROOT/directory/n).read_text()for n in names})}
  actual={k for k in old.keys()|new.keys()if k not in old or k not in new or old[k].text!=new[k].text}
  full_keys.update(actual);span_keys.update(actual)
  for key in sorted(actual):
   a,b=old.get(key),new.get(key);u=b or a
   changed.append({'ClaimKey':key,'SourcePath':';'.join(directory+'/'+n for n in u.source.split(';')),'BoundReference':key,'Binding':'EXACT_KEYED_COMPANION_ROW','BeforeSHA256':sha(a.text)if a else'','AfterSHA256':sha(b.text)if b else'','CurrentPresence':'PRESENT'if b else'REMOVED','AfterLine':b.start if b else'','SelectedOriginalRepair':'PRIMARY'if key in selected else'SUPPLEMENTAL'if key in suppkeys else'NO_CONTAINER_OR_COLLATERAL'})
 assert len(changed)==len(full_keys),'Duplicate changed references'
 # Three controls: missing, duplicate and unrecognized selected key fail equality.
 expected=collections.Counter(selected.keys()); keys=list(selected)
 negative={
  'missing_key_rejected':collections.Counter(keys[:-1])!=expected,
  'duplicate_key_rejected':collections.Counter(keys+[keys[0]])!=expected,
  'unknown_key_rejected':collections.Counter(keys[:-1]+['INVALID:KEY'])!=expected,
  'missing_changed_reference_rejected':collections.Counter(sorted(full_keys)[:-1])!=collections.Counter(span_keys),
 }
 # Run corrupted in-memory inputs through this script's real validation path.
 # Each child must fail at its intended gate, not merely compare two constructed counters.
 probe_results=[]
 for name,gate in [('missing-key','Selected accounting key mismatch'),('missing-changed-unit','Changed key extraction/span disagreement')]:
  result=subprocess.run([sys.executable,str(Path(__file__).resolve()),'--negative-probe',name],capture_output=True,text=True)
  rejected=result.returncode!=0 and gate in result.stderr
  assert rejected,('Negative probe did not fail at intended gate',name,result.returncode,result.stderr[-500:])
  negative[name+'_real_validator_rejection']=rejected
  probe_results.append({'probe':name,'exit_code':result.returncode,'expected_gate':gate,'observed_error':next(l for l in reversed(result.stderr.splitlines())if gate in l)[:900],'mutation':'In-memory only; no source/evidence file modified'})
 assert all(negative.values())
 # Every actual deliverable is represented, including the four SCA-011 additions.
 census=[];deliverables=list(PROJECT.glob('execution/PKG-*/1_Working/DEL-*'));assert len(deliverables)==106
 for folder in sorted(deliverables):
  p=folder/'_STATUS.md';text=p.read_text();before=git('show',BASE+':'+rel(p)).decode()
  state=lifecycle(text);assert lifecycle(before)==state,('Lifecycle changed',rel(p))
  remaining=text.split('## Remaining',1)[1]if '## Remaining'in text else''
  remaining=re.split(r'\n## ',remaining)[0].strip();items=[x.strip()for x in remaining.splitlines()if x.strip()]
  if not items or remaining=='NONE' or all(x in('NONE','(none)')for x in items):items=['NONE — no item recorded; not evidence of completion']
  for i,item in enumerate(items,1):census.append({'DeliverableID':folder.name[:9],'PackageID':folder.parent.parent.name[:6],'Lifecycle':state,'Item':'NONE'if item.startswith('NONE')else str(i),'RecordedText':item,'EvidenceMeaning':'Residual record; owner-steered graph controls selection; delivery/acceptance not inferred','SourcePath':rel(p),'SourceSHA256':sha(text)})
 # Protect all prior Piping reconciliation and scope-change evidence plus product.
 protected=[]
 for p in changed_paths:
  if p.startswith(rel(OUT)+'/')or p in paths:continue
  if '/_Coordination/'in p:continue # parent-owned navigation is not claimed here
  protected.append(p)
 assert not protected,('Unexpected Piping execution writes',protected)
 untracked=git('ls-files','--others','--exclude-standard','--',rel(PROJECT)).decode().splitlines()
 assert all(x.startswith(rel(OUT)+'/')for x in untracked),('Unscoped Piping untracked files',[x for x in untracked if not x.startswith(rel(OUT)+'/')])
 ignored=git('ls-files','--others','--ignored','--exclude-standard','--',rel(OUT)).decode().splitlines()
 assert not ignored,('Ignored evidence pollution',ignored)
 for area in ('core','apps','schemas','tests','validation','docs','AGENTS.md','execution/_DAG','execution/_ScopeChange'):
  unexpected=set(git('diff','--name-only',BASE,'--',rel(PROJECT/area)).decode().splitlines())-set(paths)
  assert not unexpected,('Protected surface changed',area,unexpected)
 accepted=rows(PROJECT/'execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/ACCEPTED_MANIFEST.csv')
 accepted_delta=[]
 for r in accepted:
  assert sha(git('show',BASE+':'+r['Path']))==r['SHA256'],('Accepted historical binding',r['Path'])
  current=sha((ROOT/r['Path']).read_bytes())
  if current!=r['SHA256']:
   assert r['Path']in paths,('Unexplained accepted target change',r['Path'])
   accepted_delta.append({'SourcePath':r['Path'],'AcceptedSHA256':r['SHA256'],'CurrentSHA256':current,'Meaning':'Current record carrier changed; original accepted manifest and its historical target bytes remain bound to baseline Git.'})
 # Validate affected production SoW representations without executing product suites.
 sys.path.insert(0,str(ROOT/'tools/scope_of_work'))
 from common import resolve_production_format
 sow_checks=[]
 for path in paths:
  if Path(path).name=='ScopeOfWork.md':
   resolution=resolve_production_format((ROOT/path).parent)
   assert resolution.valid and not resolution.issues,(path,resolution.state,resolution.issues)
   sow_checks.append(path)
 # Source currency: delta overlays only; original maps, graph and reports immutable.
 input_records=json.loads((POST/'reconciliation/SOURCES_RECONCILIATION_ACCEPTED.json').read_text())['inputs']
 input_delta=[]
 for r in input_records:
  current=sha((ROOT/r['path']).read_bytes())
  if current!=r['sha256']:
   assert r['path']in paths,('Unexplained ownership input change',r['path'])
   input_delta.append({'SourcePath':r['path'],'PreviousSHA256':r['sha256'],'CurrentSHA256':current})
 hook_delta=[]
 for r in rows(POST/'reconciliation/CURRENT_HOOK_BINDINGS.csv'):
  p=ROOT/r['SourcePath'];text=p.read_text();excerpt=r['ExactCurrentExcerpt'];assert excerpt in text,('Ownership hook altered',r['Hook'])
  fragment=r['Hook'].split('#',1)[1];lines=text.splitlines()
  starts=[i for i,line in enumerate(lines)if line.startswith('### '+fragment+' ')or line.startswith('- **'+fragment+'**')or(':CONTEXT#'in r['Hook']and line=='## SCA-011 responsibility')]
  assert len(starts)==1 and '\n'.join(lines[starts[0]:starts[0]+7])==excerpt,('Ownership hook locus altered',r['Hook'])
  if sha(text)!=r['SourceSHA256']:
   hook_delta.append({'Hook':r['Hook'],'SourcePath':r['SourcePath'],'PreviousSHA256':r['SourceSHA256'],'CurrentSHA256':sha(text),'FirstLine':starts[0]+1,'ExcerptSHA256':sha(excerpt),'ExactExcerptUnchanged':'YES'})
 dependency_delta=[]
 dep=json.loads((POST/'dependencies/CURRENT_SOURCE_BINDINGS.json').read_text())['row_bindings'];assert len(dep)==84
 for r in dep:
  p=PROJECT/r['EvidenceFile'];text=p.read_text();locus=r['SourceRef'].split(' # ',1)[1]
  assert r['EvidenceQuote']in quote_section(text,locus),('Dependency quote changed',r['DependencyID'])
  if sha(text)!=r['accepted_source_sha256']:
   dependency_delta.append({'DependencyID':r['DependencyID'],'SourcePath':rel(p),'SourceRef':r['SourceRef'],'PreviousSHA256':r['accepted_source_sha256'],'CurrentSHA256':sha(text),'QuoteSHA256':sha(r['EvidenceQuote']),'QuoteUnchangedInNamedLocus':'YES'})
 overlays={'accepted_manifest_bindings_checked_against_baseline':len(accepted),'accepted_target_current_delta':accepted_delta,'baseline_finalization':rel(POST),'ownership_source_count':len(input_records),'ownership_source_delta':input_delta,'ownership_hooks_checked':31,'ownership_hook_delta':hook_delta,'dependency_quotes_checked':84,'dependency_quote_delta':dependency_delta,'meaning':'Apply changed-source bindings over immutable prior records; accepted ownership/residual mappings and graph bytes unchanged. No new product conformance or acceptance.'}
 summary={'status':'PASS','baseline':BASE,'original_corpus_keys':len(corpus),'primary_selected_keys':len(selected),'supplemental_keys':len(supplemental),'original_keys_carried_forward_without_new_disposition':len(corpus)-len(accounting),'dispositions':dict(collections.Counter(r['Disposition']for r in accounting)),'original_families':dict(collections.Counter(r['ClassID'].split('-')[0]for r in selected.values())),'source_files_changed':len(sources),'changed_references':len(changed),'selected_references_changed':sum(r['SelectedOriginalRepair']in('PRIMARY','SUPPLEMENTAL')for r in changed),'census_deliverables':len(deliverables),'census_rows':len(census),'affected_sow_format_checks':len(sow_checks),'lifecycle_unchanged':True,'accepted_manifest_bindings':len(accepted),'negative_controls':negative,'negative_probe_results':probe_results,'independent_review':'See INDEPENDENT_REVIEW.md; this mechanical report does not stand in for it','limits':'Record repair only; prior product/engineering evidence unchanged; no new lifecycle, release or reliance result.'}
 return {'ACCOUNTING.csv':csvtext(accounting),'SOURCE_BINDINGS.csv':csvtext(sources),'CHANGED_CLAIM_REEXTRACTION.csv':csvtext(changed),'REMAINING_WORK_CENSUS.csv':csvtext(census),'CURRENT_DERIVATIVE_BINDING_DELTA.json':json.dumps(overlays,indent=2)+'\n','CHECK_RESULT.json':json.dumps(summary,indent=2)+'\n'}
def main():
 args=argparse.ArgumentParser();args.add_argument('--check',action='store_true');args.add_argument('--negative-probe',choices=['missing-key','missing-changed-unit']);opts=args.parse_args()
 outputs=build(opts.negative_probe)
 for name,text in outputs.items():
  path=OUT/name
  if opts.check:assert path.read_text()==text,('Derived evidence stale',name)
  else:path.write_text(text)
 print(outputs['CHECK_RESULT.json'])
if __name__=='__main__':main()
