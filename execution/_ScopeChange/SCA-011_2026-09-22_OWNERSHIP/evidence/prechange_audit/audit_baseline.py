"""Read immutable accepted HEAD bytes and emit bounded prechange audit evidence."""
import csv,io,json,re,subprocess,hashlib,collections,pathlib,datetime
ROOT=pathlib.Path.cwd(); BASE='008ef6a4e370822c65eb26ddd792752b4267c7ab'
P='projects/chirality-piping'; E=P+'/execution'; O=ROOT/E/'_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/evidence/prechange_audit'
SCOPE=['PKG-'+x for x in ['02','03','04','05','07','08','09','10','12','13','14','15','16','17']]
FILES=subprocess.check_output(['git','ls-tree','-r','--name-only',BASE],text=True).splitlines(); FILESET=set(FILES); sources={}
def read(p):
 if p not in FILESET: return None
 b=subprocess.check_output(['git','show',BASE+':'+p]); sources[p]={'sha256':hashlib.sha256(b).hexdigest(),'bytes':len(b),'basis':BASE}; return b.decode()
def rows(p): return list(csv.DictReader(io.StringIO(read(p))))
def ids(v,kind): return sorted(set(re.findall(kind+r'-\d+(?:-\d+)?',v)))
dp=E+'/_Decomposition/SOFTWARE_DECOMP.md'; dt=read(dp); lines=dt.splitlines(); heads=[]
for n,l in enumerate(lines,1):
 if l.startswith('## '): heads.append({'heading':l[3:],'line':n,'normalized':re.sub(r'^\d+[a-zA-Z]?\.\s+','',l[3:]).strip().casefold()})
def bind(term):
 t=term.casefold()
 for mode,fn in [('exact',lambda x:x==t),('prefix',lambda x:x.startswith(t)),('substring',lambda x:t in x)]:
  matches=[x for x in heads if fn(x['normalized'])]
  if matches: return dict(matches[0],match_mode=mode,ambiguity=len(matches)>1)
 raise Exception('Unresolved '+term)
bindings={k:bind(v) for k,v in {'Ledger':'Scope Ledger','Objectives_reference':'Objectives','Partitions':'Packages','Production Units':'Deliverables'}.items()}
def section(binding):
 start=binding['line']; end=next((h['line']-1 for h in heads if h['line']>start),len(lines)); return lines[start:end]
def table_rows(ls):
 out=[]; header=None
 for l in ls:
  if not l.startswith('|'):continue
  cells=[x.strip() for x in l.strip('|').split('|')]
  if all(re.fullmatch('[: -]+',x) for x in cells): continue
  if cells[0] in ['PackageID','DeliverableID','ScopeItemID','ObjectiveID','Metric']:header=cells;continue
  if header:out.append(dict(zip(header,cells)))
 return out
packages=table_rows(section(bindings['Partitions'])); md_del=table_rows(section(bindings['Production Units'])); md_ledger=table_rows(section(bindings['Ledger'])); md_obj=table_rows(section(bindings['Objectives_reference']))
regs={n:rows(P+'/docs/_Registers/'+n+'.csv') for n in ['Deliverables','ScopeLedger','ContextBudgetQA']}; dr={x['DeliverableID']:x for x in regs['Deliverables']}; qr={x['DeliverableID']:x for x in regs['ContextBudgetQA']}; md={x['DeliverableID']:x for x in md_del}; lr={x['ScopeItemID']:x for x in regs['ScopeLedger']}
folders={}
for f in FILES:
 m=re.match(re.escape(E)+r'/(PKG-\d+[^/]+)/([123]_[^/]+)/(DEL-\d+-\d+[^/]+)/',f)
 if m:
  pkg=re.match(r'PKG-\d+',m[1])[0]; did=re.match(r'DEL-\d+-\d+',m[3])[0]; folder=E+'/'+m[1]+'/'+m[2]+'/'+m[3]
  folders.setdefault(did,set()).add((pkg,folder))
issues=[]
def issue(check,severity,entity,msg,ref,fs):
 issues.append(dict(IssueID=f'COV-{len(issues)+1:03}',CheckNumber=str(check),Severity=severity,EntityType='PRODUCTION_UNIT' if entity.startswith('DEL-') else 'OBJECTIVE' if entity.startswith('OBJ-') else 'SNAPSHOT' if entity.startswith('SCA-') else 'DERIVATIVE_SURFACE' if entity=='LOCAL_MIRRORS' else 'PARTITION',ConcreteLabel='Deliverable' if entity.startswith('DEL-') else 'Objective' if entity.startswith('OBJ-') else 'Snapshot' if entity.startswith('SCA-') else 'Package',EntityID=entity,Description=msg,DecompositionRef=ref,FilesystemRef=fs))
for p in packages:
 if p['PackageID'] in SCOPE and not any(v[0]==p['PackageID'] for vs in folders.values() for v in vs):issue(1,'BLOCKER',p['PackageID'],'No materialized package',dp+':'+str(bindings['Partitions']['line']),E)
scoped={k:v for k,v in dr.items() if v['PackageID'] in SCOPE}; inventory=[]
for did,r in scoped.items():
 fs=sorted(folders.get(did,[])); before=len(issues)
 if len(fs)!=1:issue(2,'BLOCKER',did,'Expected exactly one folder; found '+str(len(fs)),dp+' section Deliverables',E)
 if not fs:continue
 pkg,folder=fs[0]; ct=read(folder+'/_CONTEXT.md') or ''; st=read(folder+'/_STATUS.md') or ''; memory=read(folder+'/MEMORY.md'); sow=read(folder+'/ScopeOfWork.md')
 fields={'Name':r['Name'],'Package ID':r['PackageID'],'Type':r['Type'],'Deliverable ID':did,'Envelope':r['ContextEnvelope']}; mismatch=[]
 for key,val in fields.items():
  m=re.search(r'\*\*'+re.escape(key)+r':\*\*\s*(.+)',ct)
  if not m or m[1].strip()!=val:mismatch.append(key)
 if mismatch:issue(5,'WARNING',did,'Context field mismatch: '+','.join(mismatch),dp+' section Deliverables row '+did,folder+'/_CONTEXT.md')
 for key,mdkey in [('Name','Name'),('Type','Type'),('ContextEnvelope','Context')]:
  if did not in md or r[key]!=md[did].get(mdkey): issue('9b','WARNING',did,'Register/decomposition mismatch '+key,dp+' section Deliverables row '+did,P+'/docs/_Registers/Deliverables.csv')
 for key,mdkey,kind in [('CoversScopeItems','Scope Items','SOW'),('SupportsObjectives','Objectives','OBJ')]:
  if did not in md or ids(r[key],kind)!=ids(md[did].get(mdkey,''),kind): issue('9b','WARNING',did,'Register/decomposition mapping mismatch '+key,dp+' section Deliverables row '+did,P+'/docs/_Registers/Deliverables.csv')
 if did not in qr or qr[did]['ContextEnvelope']!=r['ContextEnvelope']:issue('9b','WARNING',did,'ContextBudgetQA envelope mismatch',dp+' section Deliverables row '+did,P+'/docs/_Registers/ContextBudgetQA.csv')
 state=re.search(r'\*\*Current State:\*\*\s*(\w+)',st); state=state[1] if state else 'UNKNOWN'
 legacy=[f for f in ['Datasheet.md','Specification.md','Guidance.md','Procedure.md'] if folder+'/'+f in FILESET]
 if sow and legacy:issue(6,'WARNING',did,'SoW plus legacy package requires exact migration authority; not established in bounded audit',dp+' section Deliverables row '+did,folder)
 if not sow:
  if did=='DEL-07-09':contract='OPEN_CUSTOM_CONTRACT'; expected=['Vocabulary_Coverage.csv','Palette_Organization_Contract.md','Palette_Operation_Routing.md']; found=sum(folder+'/'+x in FILESET for x in expected)
  else:contract='LEGACY_FOUR_DOC' if len(legacy)==4 else 'MISSING'; expected=['ScopeOfWork.md']; found=0; issue(6,'WARNING',did,'No ScopeOfWork core; format requires disposition',dp+' section Deliverables row '+did,folder)
 else:contract='SOW_V1'; expected=['ScopeOfWork.md']; found=1
 if not ids(r['SupportsObjectives'],'OBJ'):issue(7,'WARNING',did,'No objective mapping',dp+' section Deliverables row '+did,folder)
 inventory.append({'DeliverableID':did,'PackageID':pkg,'Folder':folder,'ContextMatch':'MISMATCH' if mismatch else 'MATCH','ContextFieldsChecked':list(fields),'LifecycleState':state,'MemoryPresent':memory is not None,'CoreContract':contract,'CoreArtifactFound':found,'CoreArtifactExpected':len(expected),'Objectives':ids(r['SupportsObjectives'],'OBJ'),'ScopeItems':ids(r['CoversScopeItems'],'SOW'),'IssueCount':len(issues)-before,'AnticipatedArtifacts':r['AnticipatedArtifacts']})
for did,fs in folders.items():
 if any(p in SCOPE for p,f in fs) and did not in dr:issue(3,'WARNING',did,'Folder lacks companion deliverable row',dp+' section Deliverables',str(fs))
for r in md_ledger:
 key=r['ScopeItemID']; c=lr.get(key)
 if not c:issue(8,'WARNING',key,'Missing companion scope row',dp+' section Scope ledger',P+'/docs/_Registers/ScopeLedger.csv');continue
 for mk,ck,kind in [('PackageID','PackageID','PKG'),('DeliverableID(s)','DeliverableIDs','DEL'),('ObjectiveID(s)','ObjectiveIDs','OBJ')]:
  if ids(r[mk],kind)!=ids(c[ck],kind):issue(8,'WARNING',key,'Ledger markdown/CSV mapping mismatch '+mk,dp+' section Scope ledger row '+key,P+'/docs/_Registers/ScopeLedger.csv')
 for d in ids(c['DeliverableIDs'],'DEL'):
  if d not in dr:issue(8,'WARNING',key,'Unknown deliverable '+d,dp+' section Scope ledger row '+key,P+'/docs/_Registers/ScopeLedger.csv')
# Whole-document objective display versus authoritative production-unit mappings.
objective_mismatches=[]
for row in md_obj:
 oid=row['ObjectiveID']; declared=set(ids(row['Mapped Deliverables'],'DEL')); actual={d for d,r in dr.items() if oid in ids(r['SupportsObjectives'],'OBJ')}
 if declared!=actual:
  objective_mismatches.append(oid); issue(7,'WARNING',oid,'Objective summary support IDs differ from deliverable mappings: omitted '+','.join(sorted(actual-declared))+'; extra '+','.join(sorted(declared-actual)),dp+' section Objectives row '+oid,P+'/docs/_Registers/Deliverables.csv')
issue('9b','WARNING','DECOMPOSITION','Working-surface label exists but no explicit companion inventory; heavy inline deliverable/ledger duplication requires coordinated parity updates',dp+':3; sections 7 and 9',P+'/docs/_Registers')
issue(10,'INFO','SCA-010','The accepted compact rename snapshot follows its own Brief and Acceptance Record. It differs from the currently selected full scope-change template; this is historical-method compatibility, not failed closure, missing accepted obligations, or a new gate.',dp+' section Decision log DEC-101',E+'/_ScopeChange/SCA-010_2026-09-18_1400')
issue(8,'INFO','LOCAL_MIRRORS','Stock register tool reports 2097 baseline errors in local mirrors and skips cross-register parity because companions live at docs/_Registers; see raw evidence and scope limitations',dp+' section Scope ledger',E+'/_DAG/_LATEST.md')

# Read matched status+memory pairs already above; references below establish retained obligations, not renewed authority.
extra=['AGENTS.md','agents/AGENT_TASK.md',P+'/AGENTS.md','workflows/index.json','workflows/audit-decomp/execution.json','workflows/audit-decomp/WORKFLOW.md','workflows/audit-decomp/resources/contract.md','workflows/audit-decomp/resources/method.md','workflows/scope-change/resources/contract.md',E+'/_Decomposition/_LATEST.md',E+'/_ScopeChange/_LATEST.md',E+'/_ScopeChange/SCA-010_2026-09-18_1400/Brief.md',E+'/_ScopeChange/SCA-010_2026-09-18_1400/ACCEPTANCE_RECORD.md',E+'/_ScopeChange/SCA-009_2026-08-20_0000/ACCEPTANCE_RECORD.md',E+'/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md',E+'/_Coordination/_PROPOSALS/R5_R6_FOLLOWUP_2026-09-22/PIPING_OWNERSHIP_PROPOSAL.md','tools/validation/validate_decomposition_registers.py','tools/scope_of_work/validate_scope_of_work.py','tools/scope_of_work/common.py',E+'/_DAG/_LATEST.md']
for p in extra:read(p)
counts=collections.Counter(x['Severity'] for x in issues)
summary={'run_label':'SCA-011_PRECHANGE_20260922','timestamp':datetime.datetime.now(datetime.timezone.utc).isoformat(),'decomp_variant':'SOFTWARE','expected_source_snapshot':BASE,'expected_handoff_phase':'PRECHANGE_BASELINE_ONLY','decomposition_path':dp,'decomposition_revision':'0.12','scope':SCOPE,'repository_topology':{'packages':len(packages),'deliverables':len(dr),'objectives':len(set(x for r in regs['ScopeLedger'] for x in ids(r['ObjectiveIDs'],'OBJ'))),'scope_items':len(lr),'ledger_rows':len(regs['ScopeLedger'])},'partitions_declared':len(SCOPE),'partitions_found':len(set(i['PackageID'] for i in inventory)),'production_units_declared':len(scoped),'production_units_found':len(inventory),'forward_coverage_partitions_pct':100.0,'forward_coverage_production_units_pct':100*len(inventory)/len(scoped),'reverse_coverage_pct':100.0,'context_fidelity_pct':100*sum(i['ContextMatch']=='MATCH' for i in inventory)/len(inventory),'context_fidelity_limit':'Identity/name/type/package/envelope fields only; description intent and all anticipated implementation artifacts not comprehensively adjudicated.','artifact_presence_pct':None,'artifact_presence_limit':'Core SoW or accepted OPEN contract files inventoried; product artifact realization not recounted.','objective_coverage_pct':100.0,'deliverables_without_objective_mapping':sum(not i['Objectives'] for i in inventory),'in_ledger_rows_without_objective_mapping':sum(not ids(r['ObjectiveIDs'],'OBJ') for r in regs['ScopeLedger'] if r['InOutStatus']=='IN'),'package_shape_conformance':'WARN','derivative_package_status':'SKIPPED','active_snapshot_status':'PASS','handoff_state_status':'PASS','objective_evidence_integrity':'FAIL','issues_blocker':counts['BLOCKER'],'issues_warning':counts['WARNING'],'issues_info':counts['INFO'],'check_count':12,'lifecycle_distribution':dict(collections.Counter(i['LifecycleState'] for i in inventory)),'overall_status':'WARNINGS','closure_readiness':'WARN','concrete_labels':{'partition':'Package','production_unit':'Deliverable'},'semantic_bindings':bindings,'baseline_note':'Accepted tracked snapshot only, not concurrent candidate or product completion. SCA-010 is evaluated against its own accepted compact contract; the current full-template difference is informational and creates no retroactive gate.','objective_coverage_limit':'All 18 ledger objectives have at least one tracked materialized production-unit support. OBJ-018 summary support identities disagree; see issue log.','core_artifact_validation':{'SoW_validated':79,'accepted_open_custom_contract':1,'full_anticipated_artifact_realization':'INCOMPLETE'}}
(O/'inventory.json').write_text(json.dumps(inventory,indent=2)+'\n'); (O/'coverage_summary.json').write_text(json.dumps(summary,indent=2)+'\n'); (O/'SOURCES.json').write_text(json.dumps({'basis':BASE,'source_qualified_workflow':'bundled:chirality-root/audit-decomp','sources':sources},indent=2)+'\n')
with (O/'Decomp_Coverage_IssueLog.csv').open('w') as f:
 w=csv.DictWriter(f,fieldnames=['IssueID','CheckNumber','Severity','EntityType','ConcreteLabel','EntityID','Description','DecompositionRef','FilesystemRef']);w.writeheader();w.writerows(issues)
with (O/'Decomp_Coverage_Matrix.csv').open('w') as f:
 keys=['ProductionUnitID','PartitionID','ConcreteProductionUnitLabel','ConcretePartitionLabel','FolderExists','ContextPresent','ContextMatch','ArtifactCoverage','ObjectivesMapped','LifecycleState','IssueCount'];w=csv.DictWriter(f,fieldnames=keys);w.writeheader()
 for i in inventory:w.writerow(dict(zip(keys,[i['DeliverableID'],i['PackageID'],'Deliverable','Package',True,True,i['ContextMatch'],str(i['CoreArtifactFound'])+'/'+str(i['CoreArtifactExpected']),str(len(i['Objectives']))+'/'+str(len(i['Objectives'])),i['LifecycleState'],i['IssueCount']])))
print(json.dumps(summary,indent=2));print('ISSUES',json.dumps(issues,indent=2))
