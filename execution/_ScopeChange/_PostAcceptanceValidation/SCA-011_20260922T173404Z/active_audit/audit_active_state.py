"""Read-only poststate audit; all writes stay beside this script. Git is read-only."""
import pathlib,csv,io,json,re,subprocess,hashlib,collections,datetime,sys,platform
ROOT=pathlib.Path.cwd();P='projects/chirality-piping';E=P+'/execution';R=E+'/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP';H=E+'/_ScopeChange/_PostAcceptanceValidation/SCA-011_20260922T173404Z';O=ROOT/H/'active_audit';REVIEWED='d6cc1482eee78ce860ff18658f11157f7efbd401'; BASE='3e18334eca72509475684cc86786b3eeade83572';sources={};checks=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def read(p):
 p=str(p);f=ROOT/p
 if not f.is_file():return None
 b=f.read_bytes();sources[p]={'sha256':sha(b),'bytes':len(b),'basis':'actual_working_state'};return b.decode()
def old(p):
 x=subprocess.run(['git','show',BASE+':'+p],capture_output=True)
 if x.returncode:return None
 sources[BASE+':'+p]={'sha256':sha(x.stdout),'bytes':len(x.stdout),'basis':BASE};return x.stdout.decode()
def reviewed_read(p):
 b=subprocess.check_output(['git','show',REVIEWED+':'+p]);sources[REVIEWED+':'+p]={'sha256':sha(b),'bytes':len(b),'basis':REVIEWED};return b.decode()
def rows(p):return list(csv.DictReader(io.StringIO(read(p))))
def current_backchecks():
 binding=json.loads(read(H+'/dependencies/CURRENT_SOURCE_BINDINGS.json'));reverse={x['CurrentDependencyID']:x['OriginalDependencyID']for x in rows(R+'/application/dependencies/ID_CROSSWALK.csv')}
 return [{'DependencyID':reverse[x['DependencyID']],'LocalEvidenceFile':P+'/'+x['EvidenceFile'],'LocalSourceRef':x['SourceRef'].split(' # ',1)[1],'LocalEvidenceSHA256':x['accepted_source_sha256'],'LocalEvidenceQuote':x['EvidenceQuote']}for x in binding['row_bindings']]
def ids(v,k):return sorted(set(re.findall(k+r'-\d+(?:-\d+)?',v)))
def emit(n,v):(O/n).write_text(json.dumps(v,indent=2)+'\n')
def check(name,ok,detail):checks.append({'check':name,'pass':bool(ok),'detail':detail})
def tables(ls):
 out=[];header=None
 for l in ls:
  if not l.startswith('|'):continue
  cells=[x.strip() for x in l.strip('|').split('|')]
  if all(re.fullmatch('[: -]+',x)for x in cells):continue
  if cells[0] in ['PackageID','DeliverableID','ScopeItemID','ObjectiveID','Metric']:header=cells;continue
  if header:out.append(dict(zip(header,cells)))
 return out
dp=E+'/_Decomposition/SOFTWARE_DECOMP.md';dt=read(dp);lines=dt.splitlines();heads=[{'heading':l[3:],'line':n,'normalized':re.sub(r'^\d+[a-zA-Z]?\.\s+','',l[3:]).strip().casefold()}for n,l in enumerate(lines,1)if l.startswith('## ')]
def bind(term):
 for mode,fn in [('exact',lambda x:x==term),('prefix',lambda x:x.startswith(term)),('substring',lambda x:term in x)]:
  hit=[x for x in heads if fn(x['normalized'])]
  if hit:return dict(hit[0],match_mode=mode,ambiguity=len(hit)>1)
 raise ValueError('Unresolved '+term)
bindings={k:bind(v) for k,v in {'Ledger':'scope ledger','Objectives_reference':'objectives','Partitions':'packages','Production Units':'deliverables'}.items()}
def section(b):return lines[b['line']:next((h['line']-1 for h in heads if h['line']>b['line']),len(lines))]
packages=tables(section(bindings['Partitions']));md_del=tables(section(bindings['Production Units']));md_ledger=tables(section(bindings['Ledger']));md_obj=tables(section(bindings['Objectives_reference']))
regs={n:rows(P+'/docs/_Registers/'+n+'.csv')for n in ['Deliverables','ScopeLedger','ContextBudgetQA']};dr={x['DeliverableID']:x for x in regs['Deliverables']};qr={x['DeliverableID']:x for x in regs['ContextBudgetQA']};md={x['DeliverableID']:x for x in md_del};lr={x['ScopeItemID']:x for x in regs['ScopeLedger']};new=['DEL-04-07','DEL-07-11','DEL-07-12','DEL-16-06']
folders=collections.defaultdict(list)
for pkg in (ROOT/E).glob('PKG-*'):
 for d in pkg.glob('[123]_*/DEL-*'):
  if d.is_dir():folders[re.match(r'DEL-\d+-\d+',d.name)[0]].append(str(d.relative_to(ROOT)))
check('unique_source_ids',len(dr)==len(regs['Deliverables'])==len(md_del)and len(lr)==len(regs['ScopeLedger'])==len(md_ledger)and len(qr)==len(dr),'Deliverables, scope and context registers; markdown tables')
check('forward_reverse_unit_coverage',set(dr)==set(folders)==set(md)and all(len(v)==1 for v in folders.values()),{'declared':len(dr),'folders':len(folders)})
check('package_coverage',{x['PackageID']for x in packages}=={re.match(r'PKG-\d+',p.name)[0]for p in (ROOT/E).glob('PKG-*')if p.is_dir()},len(packages))
check('expected_topology',(len(packages),len(dr),len(lr))==(18,106,79),[len(packages),len(dr),len(lr)])
manifest=rows(R+'/APPLY_MANIFEST.csv');application=[]
read(H+'/OWNER_DECISION.md');read(H+'/PLAN.md');read(H+'/SOURCE_BASIS.json')
final_sources=[(R+'/evidence/SOURCES_DECOMPOSITION.json','group3_promotion_metadata'),(R+'/interfaces/INTERFACE_PROMOTION.json','group3_files'),(R+'/application/dependencies/REPAIRED_GROUP3_PROMOTION.json','group3_files')];finaltargets={};transformchecks=[]
for path,key in final_sources:
 body=read(path);check('reviewed_transform_manifest:'+path,body==reviewed_read(path),path);j=json.loads(body)[key];items=j['files']if isinstance(j,dict)else j
 for item in items:
  assert item['target']not in finaltargets;finaltargets[item['target']]=item;prior=reviewed_read(item['target']);ok=sha(prior.encode())==item['applied_sha256'];transformed=prior
  for op in item['operations']:
   ok=ok and transformed.count(op['old'])==op['expected_occurrences'];transformed=transformed.replace(op['old'],op['new'])
  actual=read(item['target']);transformchecks.append({'path':item['target'],'reviewed_applied_sha256':item['applied_sha256'],'expected_accepted_sha256':item['accepted_sha256'],'observed_sha256':sha(actual.encode())if actual else'ABSENT','pass':ok and sha(transformed.encode())==item['accepted_sha256']and actual==transformed})
check('71_exact_reviewed_final_transform_targets',len(finaltargets)==71 and all(x['pass']for x in transformchecks),transformchecks)
authority_rows=rows(E+'/_ScopeChange/checkpoint_snapshots/SCA-011_GROUP-2_2026-09-22/ACCEPTED_MANIFEST.csv');binding=next(x for x in authority_rows if x['Path']==R+'/APPLY_MANIFEST.csv');check('manifest_binds_accepted_commit',binding['SourceCommit']==BASE and sha(read(binding['Path']).encode())==binding['SHA256'] and read(binding['Path'])==old(binding['Path']),binding)
check('draft_wire_schemas_excluded',not any('/schemas/'in x['CanonicalTarget']or x['CanonicalTarget'].endswith('.draft.schema.json')for x in manifest),'No wire-schema canonical promotion')
for r in manifest:
 body=read(r['CanonicalTarget']);observed=sha(body.encode())if body is not None else'ABSENT';expected=finaltargets.get(r['CanonicalTarget'],{}).get('accepted_sha256',r['AppliedSHA256']);application.append({'target':r['CanonicalTarget'],'expected_sha256':expected,'observed_sha256':observed,'pass':observed==expected})
repair_meta=json.loads(read(R+'/application/dependencies/REPAIR_PREIMAGES.json'));repair_paths={x['path']for x in repair_meta['files']};mismatches=[x['target']for x in application if not x['pass']]
check('83_active_postimages_71_final_12_unchanged',len(application)==83 and len(finaltargets)==71 and set(finaltargets)<=set(x['target']for x in application)and all(x['pass']for x in application),{'targets':len(application),'final_targets':71,'unchanged_targets':12,'mismatches':mismatches})
deviations=json.loads(read(R+'/application/dependencies/POSTIMAGE_DEVIATIONS.json'));deviation_checks=[]
for x in deviations['changed_paths']:
 b=reviewed_read(x['path']);deviation_checks.append({'path':x['path'],'pass':b is not None and sha(b.encode())==x['after_repair_sha256']})
check('historical_reviewed_repair_postimage_custody',len(deviation_checks)>=21 and all(x['pass']for x in deviation_checks),deviation_checks)
repair_preimage_checks=[]
for x in repair_meta['files']:
 b=read(x['before_repair_copy']);repair_preimage_checks.append({'path':x['path'],'pass':b is not None and sha(b.encode())==x['before_repair_sha256']})
check('repair_preimage_custody',all(x['pass']for x in repair_preimage_checks),repair_preimage_checks)
issues=[]
def issue(n,severity,entity,msg,fs,baseline=False):
 issues.append(dict(IssueID=f'COV-{len(issues)+1:03}',CheckNumber=str(n),Severity=severity,EntityType='PRODUCTION_UNIT'if entity.startswith('DEL')else 'DERIVATIVE_SURFACE',ConcreteLabel='Deliverable'if entity.startswith('DEL')else'Package',EntityID=entity,Description=('BASELINE: 'if baseline else'')+msg,DecompositionRef=dp+' section '+('Deliverables'if entity.startswith('DEL')else'Objectives / package'),FilesystemRef=fs))
inventory=[];sowresults=[];claimchecks=[];contexts=[];lifecycle=[];artifacts=[]
for did,r in dr.items():
 folder=folders[did][0];ct=read(folder+'/_CONTEXT.md')or'';st=read(folder+'/_STATUS.md')or'';memory=read(folder+'/MEMORY.md');sow=read(folder+'/ScopeOfWork.md');fields={'Name':r['Name'],'Package ID':r['PackageID'],'Type':r['Type'],'Deliverable ID':did,'Envelope':r['ContextEnvelope']};mismatch=[]
 for key,val in fields.items():
  m=re.search(r'\*\*'+re.escape(key)+r':\*\*\s*(.+)',ct)
  if not m or m[1].strip()!=val:mismatch.append(key)
 for key,mk in [('Name','Name'),('Type','Type'),('ContextEnvelope','Context')]:
  if r[key]!=md[did].get(mk):mismatch.append('markdown:'+key)
 for key,mk,k in [('CoversScopeItems','Scope Items','SOW'),('SupportsObjectives','Objectives','OBJ')]:
  if ids(r[key],k)!=ids(md[did].get(mk,''),k):mismatch.append('markdown:'+key)
 if qr.get(did,{}).get('ContextEnvelope')!=r['ContextEnvelope']:mismatch.append('ContextBudgetQA')
 state=re.search(r'\*\*Current State:\*\*\s*(\w+)',st);state=state[1]if state else'UNKNOWN';before=old(folder+'/_STATUS.md');bs=re.search(r'\*\*Current State:\*\*\s*(\w+)',before or'');lifecycle.append({'id':did,'before':bs[1]if bs else'ABSENT','after':state,'pass':state==(bs[1]if bs else'OPEN')})
 if mismatch:issue(5,'WARNING',did,'Context/register field mismatch: '+','.join(mismatch),folder+'/_CONTEXT.md',old(folder+'/_CONTEXT.md')==ct)
 contexts.append({'id':did,'fields_checked':fields,'mismatches':mismatch,'description_parity':r['Description']in ct if did in new else'not_comprehensively_adjudicated'})
 if sow:
  contract='SOW_V1';expected=['ScopeOfWork.md'];cmd=['python3','tools/scope_of_work/validate_scope_of_work.py',folder,'--json'];run=subprocess.run(cmd,capture_output=True,text=True);sowresults.append({'id':did,'command':cmd,'exit_code':run.returncode,'stdout':json.loads(run.stdout)if run.stdout.startswith('{')else run.stdout,'stderr':run.stderr})
  if run.returncode:issue(6,'WARNING',did,'Core SoW validator failed',folder+'/ScopeOfWork.md')
 elif did.startswith('DEL-00-'):
  contract='ARCHITECTURE_BASIS_REFERENCE';expected=['ArchitectureBasis.md']
 elif did=='DEL-07-09':contract='ACCEPTED_OPEN_CUSTOM_CONTRACT';expected=['Vocabulary_Coverage.csv','Palette_Organization_Contract.md','Palette_Operation_Routing.md']
 else:contract='UNKNOWN';expected=['ScopeOfWork.md'];issue(6,'WARNING',did,'Core contract unresolved',folder)
 found=sum((ROOT/folder/f).is_file()for f in expected);artifacts.append({'id':did,'contract':contract,'core_expected':expected,'core_found':found,'anticipated':r['AnticipatedArtifacts'],'production_realization':'INCOMPLETE_NOT_RECOUNTED'})
 inventory.append({'DeliverableID':did,'PackageID':r['PackageID'],'Folder':folder,'ContextMatch':'MISMATCH'if mismatch else'MATCH','LifecycleState':state,'MemoryPresent':memory is not None,'CoreContract':contract,'CoreArtifactFound':found,'CoreArtifactExpected':len(expected),'Objectives':ids(r['SupportsObjectives'],'OBJ'),'ScopeItems':ids(r['CoversScopeItems'],'SOW')})
check('all_context_identity_and_envelopes',not any(x['mismatches']for x in contexts),contexts)
check('lifecycle_preserved_new_open',all(x['pass']for x in lifecycle),lifecycle)
check('all_core_contracts',all(x['CoreArtifactFound']==x['CoreArtifactExpected']for x in inventory)and all(x['exit_code']==0 for x in sowresults),{'sows':len(sowresults),'other_contracts':9})
for row in manifest:
 if not row['CanonicalTarget'].endswith('/ScopeOfWork.md'):continue
 p=row['CanonicalTarget'];cur=read(p);prior=old(p);tokens=lambda x:set(re.findall(r'\b(?:CLM|AC|VER|OUT)-\d+\b',x or''));before=tokens(prior);after=tokens(cur);claimchecks.append({'path':p,'existing':prior is not None,'stable_ids_retained':sorted(before),'removed_ids':sorted(before-after),'new_ids':sorted(after-before),'pass':before<=after,'validator_pass':next(x['exit_code']==0 for x in sowresults if x['id']==re.search(r'DEL-\d+-\d+',p)[0])})
check('24_sow_claim_ids_hooks',len(claimchecks)==24 and all(x['pass']and x['validator_pass']for x in claimchecks),claimchecks)
ledger_mismatches=[]
for r in md_ledger:
 c=lr.get(r['ScopeItemID']);
 for mk,ck,k in [('PackageID','PackageID','PKG'),('DeliverableID(s)','DeliverableIDs','DEL'),('ObjectiveID(s)','ObjectiveIDs','OBJ')]:
  if not c or ids(r[mk],k)!=ids(c[ck],k):ledger_mismatches.append([r['ScopeItemID'],mk])
 for d in ids(c['DeliverableIDs'],'DEL'):
  if d not in dr:ledger_mismatches.append([r['ScopeItemID'],'unknown:'+d])
check('explicit_docs_register_ledger_parity',not ledger_mismatches,ledger_mismatches)
objparity=[]
for row in md_obj:
 oid=row['ObjectiveID'];actual={d for d,r in dr.items()if oid in ids(r['SupportsObjectives'],'OBJ')};declared=set(ids(row['Mapped Deliverables'],'DEL'));objparity.append({'objective':oid,'support_ids':sorted(actual),'missing':sorted(actual-declared),'extra':sorted(declared-actual),'pass':actual==declared and bool(actual)})
check('objective_support_summary_parity',all(x['pass']for x in objparity),objparity)
prior_dt=old(dp);prior_lines=prior_dt.splitlines();start=next(i for i,l in enumerate(prior_lines)if l=='## 5. Objectives');end=next(i for i in range(start+1,len(prior_lines))if prior_lines[i].startswith('## '));prior_obj=tables(prior_lines[start+1:end]);objective_nonmapping=lambda rr:[{k:v for k,v in r.items()if k in ['ObjectiveID','Statement']}for r in rr];check('no_objective_statement_change',objective_nonmapping(prior_obj)==objective_nonmapping(md_obj),{'count':len(md_obj),'mapping_only_updates':True})
new_control=[]
for did in new:
 f=folders[did][0];controls={n:read(f+'/'+n)for n in ['_CONTEXT.md','_STATUS.md','_REFERENCES.md','_SEMANTIC.md','_DEPENDENCIES.md']};new_control.append({'id':did,'all_controls_present':all(controls.values()),'envelope':dr[did]['ContextEnvelope'],'description_in_context':dr[did]['Description']in controls['_CONTEXT.md'],'finite_envelope_notes':dr[did]['ContextEnvelopeNotes'],'semantic_placeholder':'PLACEHOLDER'in controls['_SEMANTIC.md']})
check('four_finite_control_sets',all(x['all_controls_present']and x['description_in_context']and x['semantic_placeholder']for x in new_control)and collections.Counter(x['envelope']for x in new_control)=={'L':3,'M':1},new_control)
sca_pointer=read(E+'/_ScopeChange/_LATEST.md');dag_pointer=read(E+'/_DAG/_LATEST.md');decision=read(H+'/OWNER_DECISION.md')
check('actual_group3_owner_authority','Accept the audited result and adopt DAG-011'in decision and REVIEWED in decision,H+'/OWNER_DECISION.md')
check('active_sca011_pointer','SCA-011'in sca_pointer and 'accepted'in sca_pointer.lower(),sca_pointer)
check('active_dag011_pointer','DAG-011'in dag_pointer and 'Approved graph authority: `execution/_DAG/DAG-011/'in dag_pointer,dag_pointer)
check('accepted_current_basis_status','status: current_basis'in dt and 'status: applied_group3_pending'not in dt,dp)
for p in [R+'/Handoff_State.md',R+'/RUN_SUMMARY.md',R+'/GROUP3_PACKAGE.md',R+'/application/PLAN.md',R+'/DEPENDENCY_SNAPSHOT_MANIFEST.json']:read(p)
handoff=read(R+'/Handoff_State.md');check('truthful_accepted_handoff','Status: **ACCEPTED; CLOSED_FOR_SCOPE_CHANGE_ONLY**'in handoff and REVIEWED in handoff and 'ACCEPTED_PREDECESSOR'not in handoff,handoff)
prior_dr={x['DeliverableID']:x for x in csv.DictReader(io.StringIO(old(P+'/docs/_Registers/Deliverables.csv')))}
check('identities_retained_four_new',set(prior_dr)<=set(dr) and set(dr)-set(prior_dr)==set(new),{'before':len(prior_dr),'after':len(dr),'added':sorted(set(dr)-set(prior_dr))})
read(R+'/application/dependencies/LOCAL_EXTRACTION_VALIDATION.json');read(R+'/application/dependencies/STAGED_VALIDATION.json');read('workflows/dependency-extract/WORKFLOW.md')
prior_lr={x['ScopeItemID']:x for x in csv.DictReader(io.StringIO(old(P+'/docs/_Registers/ScopeLedger.csv')))};check('scope_ids_retained_two_new',set(prior_lr)<=set(lr) and set(lr)-set(prior_lr)=={'SOW-078','SOW-079'},{'before':len(prior_lr),'after':len(lr),'added':sorted(set(lr)-set(prior_lr))})
staged=json.loads(read(R+'/DEPENDENCY_SNAPSHOT_MANIFEST.json'))['files'];check('staged_dag_nodes_preserved',all((read(x['proposed_target'])is not None and sha((ROOT/x['proposed_target']).read_bytes())==x['sha256'])for x in staged if x['proposed_target'].endswith('DeliverableNodes.csv')),staged)
cross=rows(R+'/application/dependencies/ID_CROSSWALK.csv');idmap={x['OriginalDependencyID']:x['CurrentDependencyID']for x in cross};newedges=rows(E+'/_DAG/DAG-011/DependencyEdges.csv');oldedges=rows(R+'/dependencies/DependencyEdges.csv');actualedges={x['DependencyID']:x for x in newedges};allowed={'DependencyID','EvidenceFile','SourceRef','EvidenceQuote','Notes'};rowchecks=[];sourceback={x['DependencyID']:x for x in current_backchecks()}
for before in oldedges:
 after=actualedges.get(idmap.get(before['DependencyID'],before['DependencyID']));changed=[k for k in before if not after or before[k]!=after.get(k)];added=before['DependencyID']in idmap;ok=after is not None and (set(changed)<=allowed if added else not changed)
 detail={'before_id':before['DependencyID'],'after_id':idmap.get(before['DependencyID'],before['DependencyID']),'changed_fields':changed,'pass':ok}
 if added and after:
  ev=read(P+'/'+after['EvidenceFile']);back=sourceback[before['DependencyID']];ref=back['LocalSourceRef'];ls=(ev or'').splitlines();located=False
  if ref.startswith('frontmatter/'):
   located=after['EvidenceQuote']in '\n'.join(ls[:next((i for i in range(1,len(ls))if ls[i]=='---'),0)+1])
  else:
   starts=[i for i,l in enumerate(ls)if ref in l and (l.startswith('#')or '**'+ref+'**'in l)]
   for i in starts:
    level=len(ls[i])-len(ls[i].lstrip('#')) if ls[i].startswith('#')else 0
    end=next((j for j in range(i+1,len(ls))if ls[j].startswith('#')and len(ls[j])-len(ls[j].lstrip('#'))<=level),len(ls))if level else i+1
    if after['EvidenceQuote']in '\n'.join(ls[i:end]):located=True
  evidence_ok=bool(ev) and bool(after['EvidenceQuote'])and after['EvidenceQuote']in ev and len(after['EvidenceQuote'].split())<=30 and after['SourceRef']==after['EvidenceFile']+' # '+ref and located and sha(ev.encode())==back['LocalEvidenceSHA256']
  detail.update({'quote_words':len(after['EvidenceQuote'].split()),'source_locus_verified':located,'actual_evidence_pass':evidence_ok});detail['pass']=detail['pass']and evidence_ok and after['DependencyID'].startswith('DEP-'+after['FromDeliverableID'][4:]+'-')
 rowchecks.append(detail)
check('84_repairs_metadata_only_1487_baseline_rows_unchanged',len(idmap)==84 and len(set(idmap.values()))==84 and len(oldedges)==len(newedges)==1571 and len(actualedges)==1571 and all(x['pass']for x in rowchecks),rowchecks)
localchecks=[]
for x in repair_meta['files']:
 if not x['path'].endswith('/Dependencies.csv'):continue
 before=list(csv.DictReader(io.StringIO(read(x['before_repair_copy']))));after=rows(x['path']);aftermap={r['DependencyID']:r for r in after};ok=len(before)==len(after)==len(aftermap)
 for br in before:
  ar=aftermap.get(idmap.get(br['DependencyID'],br['DependencyID']));expected=actualedges.get(ar['DependencyID'])if ar and br['DependencyID']in idmap else br
  same=ar is not None and all((ar.get(k)or'')==(expected.get(k)or'')for k in set(ar)|set(expected))if br['DependencyID']in idmap else ar==br
  ok=ok and same
 localchecks.append({'path':x['path'],'pass':ok,'rows':len(after)})
check('ten_repaired_mirrors_match_staged_rows',len(localchecks)==10 and all(x['pass']for x in localchecks),localchecks)
read(R+'/application/dependencies/FIELD_LEVEL_DELTA.csv');read(R+'/application/dependencies/CORRECTNESS_REPAIR.md');read('docs/SPEC.md')
dagcmd=['python3','tools/coordination/audit_dag.py','--dag-dir',E+'/_DAG/DAG-011','--canonical','--strict','--json-out',str(O/'DAG_Audit.json'),'--markdown-out',str(O/'DAG_Audit.md')];dagrun=subprocess.run(dagcmd,capture_output=True,text=True);(O/'dag_audit_stdout.txt').write_text(dagrun.stdout+dagrun.stderr);check('fresh_active_dag_canonical_strict',dagrun.returncode==0,{'command':dagcmd,'exit_code':dagrun.returncode})
dep_validation=json.loads(read(H+'/dependencies/VALIDATION.json'));check('active_dependency_lane_bound_to_actual_graph',dep_validation['status']=='PASS'and dep_validation['actual_edges_sha256']==sha((ROOT/E/'_DAG/DAG-011/DependencyEdges.csv').read_bytes())and dep_validation['actual_nodes_sha256']==sha((ROOT/E/'_DAG/DAG-011/DeliverableNodes.csv').read_bytes())and dep_validation['stage_edges']==1041,dep_validation)
read(H+'/dependencies/RETURN.md');read(H+'/reconciliation/RETURN.md');read(H+'/reconciliation/OUTPUT_MANIFEST.json');read(H+'/reconciliation/PROVENANCE.json')
# Independent current derivative/source-currency checks; do not rerun the original census.
final_manifest=json.loads(read(H+'/FINAL_TARGET_MANIFEST.json'));expected_by_target={x['target']:x['expected_sha256']for x in application}
check('manager_final_target_manifest_matches_actual',len(final_manifest['files'])==83 and all(x['expected_final_sha256']==expected_by_target.get(x['target'])and sha((ROOT/x['target']).read_bytes())==x['expected_final_sha256']for x in final_manifest['files']),H+'/FINAL_TARGET_MANIFEST.json')
accepted_manifest=rows(R+'/ACCEPTED_MANIFEST.csv');accepted_hashchecks=[]
for x in accepted_manifest:
 b=(ROOT/x['Path']).read_bytes();sources[x['Path']]={'sha256':sha(b),'bytes':len(b),'basis':'actual_accepted_manifest_input'};accepted_hashchecks.append({'path':x['Path'],'pass':sha(b)==x['SHA256']})
check('accepted_sca_manifest_418_entries',len(accepted_hashchecks)==418 and all(x['pass']for x in accepted_hashchecks),accepted_hashchecks)
read(R+'/DECISION.md');read(E+'/_DAG/DAG-011/APPROVAL_RECORD.md');read(E+'/_DAG/DAG-011/PROVENANCE.json')
reviewed_edges=list(csv.DictReader(io.StringIO(reviewed_read(E+'/_DAG/DAG-011/DependencyEdges.csv'))));reviewed_by_id={x['DependencyID']:x for x in reviewed_edges};final_relation_checks=[]
for x in newedges:
 prior=reviewed_by_id[x['DependencyID']];delta=[k for k in x if x[k]!=prior[k]];final_relation_checks.append({'id':x['DependencyID'],'changed_fields':delta,'pass':set(delta)<={'Notes'}})
check('final_adoption_changes_no_relation_or_quoted_support',len(final_relation_checks)==1571 and all(x['pass']for x in final_relation_checks),final_relation_checks)
rbase=R+'/application/reconciliation/';rnow=H+'/reconciliation/';comparisons=json.loads(read(rnow+'BEFORE_AFTER_COMPARISON.json'));derivative_checks=[]
for row in comparisons:
 name=row['path'];a=read(rbase+name);b=read(rnow+name);ok=sha(a.encode())==row['original_sha256']and sha(b.encode())==row['accepted_sha256']
 if row['change']=='byte-identical':ok=ok and a==b
 elif row['change']=='source hashes only':
  ar=list(csv.DictReader(io.StringIO(a)));br=list(csv.DictReader(io.StringIO(b)));ok=ok and len(ar)==len(br)and all({k:v for k,v in x.items()if k!='SourceSHA256'}=={k:v for k,v in y.items()if k!='SourceSHA256'}for x,y in zip(ar,br))
 else:ok=False
 derivative_checks.append({'path':name,'pass':ok,'change':row['change']})
check('derivative_same_keys_facets_mappings_residuals',bool(derivative_checks)and all(x['pass']for x in derivative_checks),derivative_checks)
bindings_current=json.loads(read(rnow+'SOURCES_RECONCILIATION_ACCEPTED.json'));derivative_sources=[]
for x in bindings_current['inputs']:
 f=ROOT/x['path'];b=f.read_bytes();sources[x['path']]={'sha256':sha(b),'bytes':len(b),'basis':'actual_reconciliation_input_hash_checked'};derivative_sources.append({'path':x['path'],'pass':sha(b)==x['sha256']})
check('accepted_derivative_actual_source_currency',all(x['pass']for x in derivative_sources),derivative_sources)
hook_checks=[]
for x in rows(rnow+'CURRENT_HOOK_BINDINGS.csv'):
 b=read(x['SourcePath']);lines_now=b.splitlines();first=int(x['FirstLine'])-1;excerpt=x['ExactCurrentExcerpt'];located='\n'.join(lines_now[first:]).startswith(excerpt.rstrip('\n'));hook_checks.append({'hook':x['Hook'],'pass':sha(b.encode())==x['SourceSHA256']and sha(excerpt.encode())==x['ExcerptSHA256']and located})
check('current_derivative_hooks_and_loci',bool(hook_checks)and all(x['pass']for x in hook_checks),hook_checks)
for x in rows(rnow+'BOUNDARY_RELATIONS.csv'):check('boundary_source_currency:'+x['Subject'],sha((ROOT/x['SourcePath']).read_bytes())==x['SourceSHA256'],x['SourcePath'])
read(rnow+'VALIDATION.json');read(rnow+'BUILDER_IDENTITIES.json');read(rnow+'BUILDER_ADAPTATION.patch');read(rnow+'SOURCE_BINDING_COMPARISON.json')
historical_paths=[R+'/application',R+'/evidence',R+'/candidate',R+'/interfaces',R+'/dependencies'];historical_diff=subprocess.check_output(['git','diff','--name-only',REVIEWED,'--']+historical_paths,text=True).splitlines();check('reviewed_application_and_audit_outputs_preserved',not historical_diff,historical_diff)
protected=[P+'/core',P+'/apps',P+'/schemas',P+'/tests',E+'/_DAG/DAG-010',E+'/_ScopeChange/SCA-009_2026-08-20_0000',E+'/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS'];diff=subprocess.check_output(['git','diff','--name-only',BASE,'--']+protected,text=True).splitlines();check('protected_piping_bytes_unchanged',not diff,{'changed':diff,'paths':protected})
cmd=['python3','tools/validation/validate_decomposition_registers.py',E,'--evidence-root',P,'--json',str(O/'register_validation.json'),'--max-per-code','2'];run=subprocess.run(cmd,capture_output=True,text=True);(O/'register_validation.txt').write_text(run.stdout+run.stderr);rv=json.loads((O/'register_validation.json').read_text());bv=json.loads(read(R+'/evidence/prechange_audit/register_validation.json'));key=lambda x:tuple(x.get(k)for k in ['code','severity','path','row_id','detail']);oldkeys={key(x)for x in bv['findings']};newkeys={key(x)for x in rv['findings']};regdelta={'command':cmd,'exit_code':run.returncode,'before_errors':bv['error_count'],'after_errors':rv['error_count'],'unchanged_findings':len(oldkeys&newkeys),'resolved':len(oldkeys-newkeys),'new_findings':[x for x in rv['findings']if key(x)not in oldkeys],'skipped':rv['skipped'],'limit':'Stock validator excludes docs/_Registers; explicit parity checks above cover canonical companions.'};# Preserve and classify observed findings; accepted exact-row fidelity is separate from stock quality conformance.
backchecks=current_backchecks();backcheck_results=[]
for row in backchecks:
 b=read(row['LocalEvidenceFile']);backcheck_results.append({'id':row['DependencyID'],'pass':b is not None and sha(b.encode())==row['LocalEvidenceSHA256'] and bool(row['LocalEvidenceQuote']) and row['LocalEvidenceQuote']in b})
check('84_actual_local_quotes_and_hashes',len(backcheck_results)==84 and all(x['pass']for x in backcheck_results),backcheck_results)
new_finding_counts=collections.Counter(x['code']for x in regdelta['new_findings']);check('stock_delta_enumerated',not new_finding_counts and regdelta['unchanged_findings']==2097 and regdelta['after_errors']==2097,dict(new_finding_counts))
issue(8,'INFO','LOCAL_MIRRORS_REPAIRED','Initial applied state added168 stock errors; parent-authorized84-row metadata repair removes all168 without modifying1487 baseline dependency rows. New IDs, actual quotes and source loci are independently checked. SPEC6.8 supersedes the initial audit interpretation of preserved candidate IDs as an allowable convention.',R+'/application/dependencies/CORRECTNESS_REPAIR.md')
issue('9b','WARNING','DECOMPOSITION','Working-surface label retained; no explicit companion inventory. Heavy inline/register duplication still requires coordinated parity maintenance.',P+'/docs/_Registers',True)
issue(8,'INFO','LOCAL_MIRRORS',f"Stock validator exits {run.returncode}: baseline {bv['error_count']} errors, actual {rv['error_count']}; new findings {len(regdelta['new_findings'])}. Cross-register family skipped; explicit parity audited independently.",E+'/_DAG/_LATEST.md',True)
issue(10,'INFO','SCA-010','Accepted compact predecessor remains historical under its own contract; current active SCA-011 is separately verified against the actual Group3 decision.',E+'/_ScopeChange/SCA-010_2026-09-18_1400')
for c in checks:
 if not c['pass']:issue(10 if 'pointer'in c['check']or'dag'in c['check']else 5,'BLOCKER','APPLICATION','Actual check failed: '+c['check'],H+'/active_audit/raw_checks.json')
for p in ['AGENTS.md','agents/AGENT_TASK.md',P+'/AGENTS.md','workflows/index.json','workflows/audit-decomp/execution.json','workflows/audit-decomp/WORKFLOW.md','workflows/audit-decomp/resources/contract.md','workflows/audit-decomp/resources/method.md','docs/DECOMPOSITION_STANDARD.md','tools/validation/validate_decomposition_registers.py','tools/scope_of_work/validate_scope_of_work.py','tools/scope_of_work/common.py',E+'/_ScopeChange/SCA-010_2026-09-18_1400/Brief.md',E+'/_ScopeChange/SCA-010_2026-09-18_1400/ACCEPTANCE_RECORD.md'] :read(p)
for p in (ROOT/E/'_ScopeChange/checkpoint_snapshots/SCA-011_GROUP-2_2026-09-22').glob('*'):
 if p.is_file():read(str(p.relative_to(ROOT)))
baseline=json.loads(read(R+'/evidence/prechange_audit/coverage_summary.json'));counts=collections.Counter(x['Severity']for x in issues);summary={'run_label':'SCA-011_ACCEPTED_ACTIVE_STATE_20260922','timestamp':datetime.datetime.now(datetime.timezone.utc).isoformat(),'decomp_variant':'SOFTWARE','expected_source_snapshot':H+'/OWNER_DECISION.md','accepted_package_commit':REVIEWED,'original_amendment_commit':BASE,'expected_handoff_phase':'ACCEPTED_ACTIVE_STATE','decomposition_path':dp,'decomposition_revision':'0.13','scope':'ALL','repository_topology':{'packages':len(packages),'deliverables':len(dr),'objectives':len(md_obj),'scope_items':len(lr),'ledger_rows':len(lr)},'partitions_declared':len(packages),'partitions_found':len(packages),'production_units_declared':len(dr),'production_units_found':len(folders),'forward_coverage_partitions_pct':100.0,'forward_coverage_production_units_pct':100.0,'reverse_coverage_pct':100.0,'context_fidelity_pct':100*sum(not x['mismatches']for x in contexts)/len(dr),'artifact_presence_pct':None,'artifact_presence_limit':'Core contracts inventoried and validated; anticipated production artifact realization not recounted.','objective_coverage_pct':100.0,'deliverables_without_objective_mapping':sum(not x['Objectives']for x in inventory),'in_ledger_rows_without_objective_mapping':sum(not ids(x['ObjectiveIDs'],'OBJ')for x in regs['ScopeLedger']if x['InOutStatus']=='IN'),'package_shape_conformance':'WARN','derivative_package_status':'SKIPPED','active_snapshot_status':'PASS','handoff_state_status':'PASS','objective_evidence_integrity':'PASS'if all(x['pass']for x in objparity)else'FAIL','issues_blocker':counts['BLOCKER'],'issues_warning':counts['WARNING'],'issues_info':counts['INFO'],'check_count':12,'lifecycle_distribution':dict(collections.Counter(x['LifecycleState']for x in inventory)),'overall_status':'BLOCKERS'if counts['BLOCKER']else'WARNINGS','closure_readiness':'FAIL'if counts['BLOCKER']else'WARN','concrete_labels':{'partition':'Package','production_unit':'Deliverable'},'semantic_bindings':bindings,'comparison':{'baseline':baseline['repository_topology'],'actual':{'packages':len(packages),'deliverables':len(dr),'objectives':len(md_obj),'scope_items':len(lr)},'baseline_matrix_scope':baseline['scope'],'baseline_matrix_units':80,'actual_matrix_units':len(inventory),'methodology_change':'Expanded 14-package/80-unit baseline audit to all 18 packages/106 units, recognizing the existing eight D-43 architecture reference contracts. Compare whole-topology and shared-unit results, not raw lifecycle totals.','resolved_baseline_warning':'OBJ-018 summary support parity restored','retained_baseline_warning':'Companion inventory/package shape','stock_validator':regdelta},'group3_acceptance':True,'core_contracts':dict(collections.Counter(x['CoreContract']for x in inventory)),'new_control_sets':new_control,'limitations':['No product tests, engineering acceptance, lifecycle advancement or original evidence recertification.','Memory read as continuity only, not authority.','Description intent and all anticipated implementation artifact realization not comprehensively adjudicated.']}
emit('coverage_summary.json',summary);emit('raw_checks.json',{'checks':checks,'application':application,'sow_claims':claimchecks,'objective_support':objparity});emit('inventory.json',inventory);emit('sow_validation.json',sowresults);emit('artifact_inventory.json',artifacts);emit('register_comparison.json',regdelta);emit('SOURCES.json',{'source_qualified_workflow':'bundled:chirality-root/audit-decomp','actual_mechanism':'Codex delegated-harness-native collaboration','parent':'/root/piping_scope_manager','task':'/root/piping_scope_manager/piping_postchange_audit','role':'TASK','sources':sources,'scope_limit':'Read-only canonical state; only '+H+'/active_audit writes. No delegation or Git mutations.','model_and_effort':'Inherited host configuration; exact model and effort not exposed to executor, no override requested.','host_enforcement':'Shared checkout; scoped instructions and scripts, not a claimed isolated per-file sandbox.','additional_consultation':'dependency-extract Function 1/3 inspected read-only to adjudicate validator discrepancies; not selected as this TASK method.'})
with (O/'Decomp_Coverage_IssueLog.csv').open('w')as f:
 w=csv.DictWriter(f,fieldnames=list(issues[0]));w.writeheader();w.writerows(issues)
with (O/'Decomp_Coverage_Matrix.csv').open('w')as f:
 keys=['ProductionUnitID','PartitionID','ConcreteProductionUnitLabel','ConcretePartitionLabel','FolderExists','ContextPresent','ContextMatch','ArtifactCoverage','ObjectivesMapped','LifecycleState','IssueCount'];w=csv.DictWriter(f,fieldnames=keys);w.writeheader()
 for i in inventory:w.writerow(dict(zip(keys,[i['DeliverableID'],i['PackageID'],'Deliverable','Package',True,True,i['ContextMatch'],str(i['CoreArtifactFound'])+'/'+str(i['CoreArtifactExpected']),str(len(i['Objectives']))+'/'+str(len(i['Objectives'])),i['LifecycleState'],sum(x['EntityID']==i['DeliverableID']for x in issues)])))
print(json.dumps({'overall_status':summary['overall_status'],'checks_failed':[x['check']for x in checks if not x['pass']],'issue_counts':dict(counts),'sows_validated':len(sowresults),'stock_validator':{k:v for k,v in regdelta.items()if k not in ['command','new_findings']},'new_stock_findings':len(regdelta['new_findings'])},indent=2))
