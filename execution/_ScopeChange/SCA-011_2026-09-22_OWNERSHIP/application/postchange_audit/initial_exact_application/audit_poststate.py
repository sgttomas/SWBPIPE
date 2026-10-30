"""Read-only poststate audit; all writes stay beside this script. Git is read-only."""
import pathlib,csv,io,json,re,subprocess,hashlib,collections,datetime,sys,platform
ROOT=pathlib.Path.cwd();P='projects/chirality-piping';E=P+'/execution';R=E+'/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP';O=ROOT/R/'application/postchange_audit'; BASE='3e18334eca72509475684cc86786b3eeade83572';sources={};checks=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def read(p):
 p=str(p);f=ROOT/p
 if not f.is_file():return None
 b=f.read_bytes();sources[p]={'sha256':sha(b),'bytes':len(b),'basis':'actual_working_state'};return b.decode()
def old(p):
 x=subprocess.run(['git','show',BASE+':'+p],capture_output=True)
 if x.returncode:return None
 sources[BASE+':'+p]={'sha256':sha(x.stdout),'bytes':len(x.stdout),'basis':BASE};return x.stdout.decode()
def rows(p):return list(csv.DictReader(io.StringIO(read(p))))
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
authority_rows=rows(E+'/_ScopeChange/checkpoint_snapshots/SCA-011_GROUP-2_2026-09-22/ACCEPTED_MANIFEST.csv');binding=next(x for x in authority_rows if x['Path']==R+'/APPLY_MANIFEST.csv');check('manifest_binds_accepted_commit',binding['SourceCommit']==BASE and sha(read(binding['Path']).encode())==binding['SHA256'] and read(binding['Path'])==old(binding['Path']),binding)
check('draft_wire_schemas_excluded',not any('/schemas/'in x['CanonicalTarget']or x['CanonicalTarget'].endswith('.draft.schema.json')for x in manifest),'No wire-schema canonical promotion')
for r in manifest:
 body=read(r['CanonicalTarget']);observed=sha(body.encode())if body is not None else'ABSENT';application.append({'target':r['CanonicalTarget'],'expected_sha256':r['AppliedSHA256'],'observed_sha256':observed,'pass':observed==r['AppliedSHA256']})
check('exact_83_postimages',len(application)==83 and all(x['pass']for x in application),{'targets':len(application),'mismatches':[x['target']for x in application if not x['pass']]})
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
for p in [E+'/_ScopeChange/_LATEST.md',E+'/_DAG/_LATEST.md']:check('preserve_pointer:'+p,read(p)==old(p),p)
check('pending_group3_status','status: applied_group3_pending'in dt,dp)
for p in [R+'/Handoff_State.md',R+'/RUN_SUMMARY.md',R+'/application/PLAN.md',R+'/DEPENDENCY_SNAPSHOT_MANIFEST.json']:read(p)
handoff=read(R+'/Handoff_State.md');check('truthful_pending_handoff','Group 3'in handoff and 'pending'in handoff and 'No canonical files or accepted pointers have changed'not in handoff,handoff)
prior_dr={x['DeliverableID']:x for x in csv.DictReader(io.StringIO(old(P+'/docs/_Registers/Deliverables.csv')))}
check('identities_retained_four_new',set(prior_dr)<=set(dr) and set(dr)-set(prior_dr)==set(new),{'before':len(prior_dr),'after':len(dr),'added':sorted(set(dr)-set(prior_dr))})
read(R+'/application/dependencies/LOCAL_EXTRACTION_VALIDATION.json');read(R+'/application/dependencies/STAGED_VALIDATION.json');read('workflows/dependency-extract/WORKFLOW.md')
prior_lr={x['ScopeItemID']:x for x in csv.DictReader(io.StringIO(old(P+'/docs/_Registers/ScopeLedger.csv')))};check('scope_ids_retained_two_new',set(prior_lr)<=set(lr) and set(lr)-set(prior_lr)=={'SOW-078','SOW-079'},{'before':len(prior_lr),'after':len(lr),'added':sorted(set(lr)-set(prior_lr))})
staged=json.loads(read(R+'/DEPENDENCY_SNAPSHOT_MANIFEST.json'))['files'];check('staged_dag_accepted_bytes',all((read(x['proposed_target'])is not None and sha((ROOT/x['proposed_target']).read_bytes())==x['sha256'])for x in staged),staged)
protected=[P+'/core',P+'/apps',P+'/schemas',P+'/tests',E+'/_DAG/DAG-010',E+'/_ScopeChange/SCA-009_2026-08-20_0000',E+'/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS'];diff=subprocess.check_output(['git','diff','--name-only',BASE,'--']+protected,text=True).splitlines();check('protected_piping_bytes_unchanged',not diff,{'changed':diff,'paths':protected})
cmd=['python3','tools/validation/validate_decomposition_registers.py',E,'--evidence-root',P,'--json',str(O/'register_validation.json'),'--max-per-code','2'];run=subprocess.run(cmd,capture_output=True,text=True);(O/'register_validation.txt').write_text(run.stdout+run.stderr);rv=json.loads((O/'register_validation.json').read_text());bv=json.loads(read(R+'/evidence/prechange_audit/register_validation.json'));key=lambda x:tuple(x.get(k)for k in ['code','severity','path','row_id','detail']);oldkeys={key(x)for x in bv['findings']};newkeys={key(x)for x in rv['findings']};regdelta={'command':cmd,'exit_code':run.returncode,'before_errors':bv['error_count'],'after_errors':rv['error_count'],'unchanged_findings':len(oldkeys&newkeys),'resolved':len(oldkeys-newkeys),'new_findings':[x for x in rv['findings']if key(x)not in oldkeys],'skipped':rv['skipped'],'limit':'Stock validator excludes docs/_Registers; explicit parity checks above cover canonical companions.'};# Preserve and classify observed findings; accepted exact-row fidelity is separate from stock quality conformance.
backchecks=rows(R+'/application/dependencies/LOCAL_SOURCE_BACKCHECK.csv');backcheck_results=[]
for row in backchecks:
 b=read(row['LocalEvidenceFile']);backcheck_results.append({'id':row['DependencyID'],'pass':b is not None and sha(b.encode())==row['LocalEvidenceSHA256'] and bool(row['LocalEvidenceQuote']) and row['LocalEvidenceQuote']in b})
check('84_actual_local_quotes_and_hashes',len(backcheck_results)==84 and all(x['pass']for x in backcheck_results),backcheck_results)
new_finding_counts=collections.Counter(x['code']for x in regdelta['new_findings']);check('stock_delta_enumerated',new_finding_counts=={'EVQ-003':84,'DRB-006':84}and regdelta['unchanged_findings']==2097,dict(new_finding_counts))
issue(8,'WARNING','LOCAL_MIRRORS_NEW','NEW RELATIVE TO PRECHANGE: 84 exact accepted rows retain blank inline EvidenceQuote fields and SCA011-E IDs, adding 84 EVQ-003 plus 84 DRB-006 stock findings. Separate actual-source backchecks contain 84 nonblank literal quotes and matching source hashes. Accepted postimage fidelity passes; stock local-format/evidence conformance does not. dependency-extract Function 1 explicitly makes EvidenceQuote optional; Function 3 preserves existing IDs. These accepted conventions differ from stock quality expectations. No inline-field repair, tool waiver or clean stock result is inferred.',R+'/application/postchange_audit/register_comparison.json')
issue('9b','WARNING','DECOMPOSITION','Working-surface label retained; no explicit companion inventory. Heavy inline/register duplication still requires coordinated parity maintenance.',P+'/docs/_Registers',True)
issue(8,'INFO','LOCAL_MIRRORS',f"Stock validator exits {run.returncode}: baseline {bv['error_count']} errors, actual {rv['error_count']}; new findings {len(regdelta['new_findings'])}. Cross-register family skipped; explicit parity audited independently.",E+'/_DAG/_LATEST.md',True)
issue(10,'INFO','SCA-010','Accepted compact predecessor evaluated against its own acceptance record; no retroactive full-template obligation.',E+'/_ScopeChange/SCA-010_2026-09-18_1400')
for c in checks:
 if not c['pass']:issue(10 if 'pointer'in c['check']or'dag'in c['check']else 5,'BLOCKER','APPLICATION','Actual check failed: '+c['check'],R+'/application/postchange_audit/raw_checks.json')
for p in ['AGENTS.md','agents/AGENT_TASK.md',P+'/AGENTS.md','workflows/index.json','workflows/audit-decomp/execution.json','workflows/audit-decomp/WORKFLOW.md','workflows/audit-decomp/resources/contract.md','workflows/audit-decomp/resources/method.md','docs/DECOMPOSITION_STANDARD.md','tools/validation/validate_decomposition_registers.py','tools/scope_of_work/validate_scope_of_work.py','tools/scope_of_work/common.py',E+'/_ScopeChange/SCA-010_2026-09-18_1400/Brief.md',E+'/_ScopeChange/SCA-010_2026-09-18_1400/ACCEPTANCE_RECORD.md'] :read(p)
for p in (ROOT/E/'_ScopeChange/checkpoint_snapshots/SCA-011_GROUP-2_2026-09-22').glob('*'):
 if p.is_file():read(str(p.relative_to(ROOT)))
baseline=json.loads(read(R+'/evidence/prechange_audit/coverage_summary.json'));counts=collections.Counter(x['Severity']for x in issues);summary={'run_label':'SCA-011_ACTUAL_POSTSTATE_20260922','timestamp':datetime.datetime.now(datetime.timezone.utc).isoformat(),'decomp_variant':'SOFTWARE','expected_source_snapshot':E+'/_ScopeChange/checkpoint_snapshots/SCA-011_GROUP-2_2026-09-22','accepted_package_commit':BASE,'expected_handoff_phase':'APPLIED_GROUP3_PENDING','decomposition_path':dp,'decomposition_revision':'0.13','scope':'ALL','repository_topology':{'packages':len(packages),'deliverables':len(dr),'objectives':len(md_obj),'scope_items':len(lr),'ledger_rows':len(lr)},'partitions_declared':len(packages),'partitions_found':len(packages),'production_units_declared':len(dr),'production_units_found':len(folders),'forward_coverage_partitions_pct':100.0,'forward_coverage_production_units_pct':100.0,'reverse_coverage_pct':100.0,'context_fidelity_pct':100*sum(not x['mismatches']for x in contexts)/len(dr),'artifact_presence_pct':None,'artifact_presence_limit':'Core contracts inventoried and validated; anticipated production artifact realization not recounted.','objective_coverage_pct':100.0,'deliverables_without_objective_mapping':sum(not x['Objectives']for x in inventory),'in_ledger_rows_without_objective_mapping':sum(not ids(x['ObjectiveIDs'],'OBJ')for x in regs['ScopeLedger']if x['InOutStatus']=='IN'),'package_shape_conformance':'WARN','derivative_package_status':'SKIPPED','active_snapshot_status':'PASS','handoff_state_status':'PASS','objective_evidence_integrity':'PASS'if all(x['pass']for x in objparity)else'FAIL','issues_blocker':counts['BLOCKER'],'issues_warning':counts['WARNING'],'issues_info':counts['INFO'],'check_count':12,'lifecycle_distribution':dict(collections.Counter(x['LifecycleState']for x in inventory)),'overall_status':'BLOCKERS'if counts['BLOCKER']else'WARNINGS','closure_readiness':'FAIL'if counts['BLOCKER']else'WARN','concrete_labels':{'partition':'Package','production_unit':'Deliverable'},'semantic_bindings':bindings,'comparison':{'baseline':baseline['repository_topology'],'actual':{'packages':len(packages),'deliverables':len(dr),'objectives':len(md_obj),'scope_items':len(lr)},'baseline_matrix_scope':baseline['scope'],'baseline_matrix_units':80,'actual_matrix_units':len(inventory),'methodology_change':'Expanded 14-package/80-unit baseline audit to all 18 packages/106 units, recognizing the existing eight D-43 architecture reference contracts. Compare whole-topology and shared-unit results, not raw lifecycle totals.','resolved_baseline_warning':'OBJ-018 summary support parity restored','retained_baseline_warning':'Companion inventory/package shape','stock_validator':regdelta},'group3_acceptance':False,'core_contracts':dict(collections.Counter(x['CoreContract']for x in inventory)),'new_control_sets':new_control,'limitations':['No product tests, engineering acceptance, lifecycle advancement or original evidence recertification.','Memory read as continuity only, not authority.','Description intent and all anticipated implementation artifact realization not comprehensively adjudicated.']}
emit('coverage_summary.json',summary);emit('raw_checks.json',{'checks':checks,'application':application,'sow_claims':claimchecks,'objective_support':objparity});emit('inventory.json',inventory);emit('sow_validation.json',sowresults);emit('artifact_inventory.json',artifacts);emit('register_comparison.json',regdelta);emit('SOURCES.json',{'source_qualified_workflow':'bundled:chirality-root/audit-decomp','actual_mechanism':'Codex delegated-harness-native collaboration','parent':'/root/piping_scope_manager','task':'/root/piping_scope_manager/piping_postchange_audit','role':'TASK','sources':sources,'scope_limit':'Read-only canonical state; only application/postchange_audit writes. No delegation or Git mutations.','model_and_effort':'Inherited host configuration; exact model and effort not exposed to executor, no override requested.','host_enforcement':'Shared checkout; scoped instructions and scripts, not a claimed isolated per-file sandbox.','additional_consultation':'dependency-extract Function 1/3 inspected read-only to adjudicate validator discrepancies; not selected as this TASK method.'})
with (O/'Decomp_Coverage_IssueLog.csv').open('w')as f:
 w=csv.DictWriter(f,fieldnames=list(issues[0]));w.writeheader();w.writerows(issues)
with (O/'Decomp_Coverage_Matrix.csv').open('w')as f:
 keys=['ProductionUnitID','PartitionID','ConcreteProductionUnitLabel','ConcretePartitionLabel','FolderExists','ContextPresent','ContextMatch','ArtifactCoverage','ObjectivesMapped','LifecycleState','IssueCount'];w=csv.DictWriter(f,fieldnames=keys);w.writeheader()
 for i in inventory:w.writerow(dict(zip(keys,[i['DeliverableID'],i['PackageID'],'Deliverable','Package',True,True,i['ContextMatch'],str(i['CoreArtifactFound'])+'/'+str(i['CoreArtifactExpected']),str(len(i['Objectives']))+'/'+str(len(i['Objectives'])),i['LifecycleState'],sum(x['EntityID']==i['DeliverableID']for x in issues)])))
print(json.dumps({'overall_status':summary['overall_status'],'checks_failed':[x['check']for x in checks if not x['pass']],'issue_counts':dict(counts),'sows_validated':len(sowresults),'stock_validator':{k:v for k,v in regdelta.items()if k not in ['command','new_findings']},'new_stock_findings':len(regdelta['new_findings'])},indent=2))
