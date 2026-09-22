#!/usr/bin/env python3
"""Validate repaired metadata without changing original application evidence."""
import ast,copy,csv,hashlib,json,re,subprocess,sys
from collections import Counter,defaultdict
from pathlib import Path
ROOT=Path(__file__).resolve().parents[7];SCA=ROOT/'projects/chirality-piping/execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP';OUT=SCA/'application/dependencies';DAG=ROOT/'projects/chirality-piping/execution/_DAG/DAG-011';PROJECT=ROOT/'projects/chirality-piping'
sys.path.insert(0,str(ROOT/'tools/coordination'));import audit_dag
sys.path.insert(0,str(ROOT/'tools/validation'));import validate_decomposition_registers as registers

def read(p):
 with p.open(newline='') as f:return list(csv.DictReader(f))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def rel(p):return str(p.relative_to(ROOT))
old=read(PROJECT/'execution/_DAG/DAG-010/DependencyEdges.csv');rows=read(DAG/'DependencyEdges.csv');nodes=read(DAG/'DeliverableNodes.csv');original=read(SCA/'dependencies/DependencyEdges.additions.csv');added=rows[len(old):]
cross=read(OUT/'ID_CROSSWALK.csv');mapping={r['OriginalDependencyID']:r['CurrentDependencyID'] for r in cross};reverse={v:k for k,v in mapping.items()}
assert len(mapping)==len(reverse)==len(added)==84 and len(rows)==1571 and len(nodes)==106
assert rows[:len(old)]==old
for n in ['DependencyEdges.csv','DeliverableNodes.csv']:assert (DAG/n).read_bytes().startswith((PROJECT/'execution/_DAG/DAG-010'/n).read_bytes())
allowed={'DependencyID','EvidenceFile','SourceRef','EvidenceQuote','Notes'};original_byid={r['DependencyID']:r for r in original};quotes=[]
for before,after in zip(original,added):
 assert mapping[before['DependencyID']]==after['DependencyID']
 assert all(before[k]==after[k] for k in before if k not in allowed)
 assert after['Notes'].startswith(before['Notes']) and 'SCA011OriginalDependencyID='+before['DependencyID'] in after['Notes']
 assert re.fullmatch('DEP-'+after['FromDeliverableID'].removeprefix('DEL-')+r'-[0-9]{3}',after['DependencyID'])
 p=PROJECT/after['EvidenceFile'];quote=after['EvidenceQuote'];assert quote and len(quote.split())<=30 and quote in p.read_text()
 assert after['SourceRef'].startswith(after['EvidenceFile']+' # ')
 quotes.append({'DependencyID':after['DependencyID'],'OriginalDependencyID':before['DependencyID'],'EvidenceFile':after['EvidenceFile'],'SourceRef':after['SourceRef'],'source_sha256':sha(p),'quote_words':len(quote.split()),'verbatim':'PASS'})
# The selected stock validator's evidence and owner-binding predicates now see
# the corrected rows. Baseline findings remain outside this repair.
reg={r['DeliverableID']:r for r in read(PROJECT/'docs/_Registers/Deliverables.csv')};findings=[];seen={};counters={'by_class':{}};local=[];commands=[]
manifest=read(SCA/'APPLY_MANIFEST.csv')
for r in manifest:
 if r['ApplicationLane']!='GROUP2_DEPENDENCY_TASK' or not r['CanonicalTarget'].endswith('/Dependencies.csv'):continue
 p=ROOT/r['CanonicalTarget'];lr=read(p);owner=lr[0]['FromDeliverableID'];new=[r for r in lr if r['DependencyID'] in reverse];expected=[r for r in added if r['FromDeliverableID']==owner]
 assert len(new)==len(expected) and all(all(a[k]==b[k] for k in a) for a,b in zip(new,expected))
 beforecopy=OUT/'repair_preimages'/r['CanonicalTarget'];before=read(beforecopy)
 assert len(lr)==len(before)
 for a,b in zip(before,lr):
  if a['DependencyID'] not in mapping:assert a==b
 prefix=beforecopy.read_bytes().split(b'v3.1,SCA011-',1)[0];assert p.read_bytes().startswith(prefix)
 md=(p.parent/'_DEPENDENCIES.md').read_text();assert 'APPLIED; GROUP 3 PENDING' in md
 assert all(row['DependencyID'] in md for row in new)
 registers.check_evidence_quality(new,rel(p),PROJECT,findings,counters)
 registers.check_dependency_binding(new,rel(p),owner,reg,seen,findings)
 cmd=['python3','tools/validation/validate_dependencies_schema.py',rel(p)];proc=subprocess.run(cmd,cwd=ROOT,text=True,capture_output=True);commands.append({'command':cmd,'exit_code':proc.returncode,'stdout':proc.stdout,'stderr':proc.stderr});assert proc.returncode==0
 local.append({'owner':owner,'rows':len(lr),'repaired_rows':len(new),'unchanged_baseline_rows':len(lr)-len(new),'schema':'PASS','prefix_preservation':'PASS'})
assert not findings,findings
# Reuse the exact accepted stage definitions and graph-check functions by AST
# selection. Bind their DOCUMENT basis IDs through the recorded bijection;
# no historical stage source or acceptance predicate is edited.
source=SCA/'dependencies/validate_candidate.py';tree=ast.parse(source.read_text())
selected=[n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name in {'stage_edges','check'} or isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='stage_alias' for t in n.targets)]
assert len(selected)==3
staged=json.loads((SCA/'dependencies/StagedInterfaces.json').read_text())['bindings'];staged=copy.deepcopy(staged)
for b in staged:b['edge_id']=mapping[b['edge_id']]
ns={'old':old,'staged':staged,'audit_dag':audit_dag,'defaultdict':defaultdict};exec(compile(ast.Module(body=selected,type_ignores=[]),str(source),'exec'),ns)
check=ns['check'];errors=check(rows,nodes);assert not errors,errors
probes={}
mut=copy.deepcopy(rows);mut[0]['Notes']+=' damaged';probes['baseline_damage']=check(mut,nodes)
mut=copy.deepcopy(rows);mut.append(copy.deepcopy(added[-4]));probes['duplicate_row']=check(mut,nodes)
mut=copy.deepcopy(rows);next(r for r in reversed(mut) if r['TargetType']=='DELIVERABLE')['TargetDeliverableID']='DEL-99-99';probes['dangling_target']=check(mut,nodes)
mut=copy.deepcopy(rows);next(r for r in reversed(mut) if r['DependencyClass']=='EXECUTION')['SatisfactionStatus']='SATISFIED';probes['unsupported_satisfaction']=check(mut,nodes)
for b in staged:probes['reverse_stage_'+b['id']]=check(rows,nodes,[{'consumer':b['producer_stage'],'upstream':b['consumer_stage'],'basis':'negative probe'}])
expected={'baseline_damage':'BASELINE_ROWS_CHANGED','duplicate_row':'DUPLICATE_ID','dangling_target':'DANGLING_TARGET','unsupported_satisfaction':'UNSUPPORTED_MATURITY_OR_SATISFACTION'}
expected.update({'reverse_stage_'+s['id']:'STAGE_CYCLE' for s in staged})
assert all(expected[k] in probes[k] for k in expected)
# Exercise the predicates which caught the metadata defects, not a validator
# configured to ignore the relevant finding families.
for name,field,value,expected_code in [('blank_quote','EvidenceQuote','','EVQ-003'),('bad_owner_id','DependencyID','SCA011-E001','DRB-006')]:
 mut=copy.deepcopy(added[:1]);mut[0][field]=value;negative=[]
 registers.check_evidence_quality(mut,'negative.csv',PROJECT,negative,{'by_class':{}})
 registers.check_dependency_binding(mut,'negative.csv',mut[0]['FromDeliverableID'],reg,{},negative)
 assert any(x.code==expected_code for x in negative);probes[name]=[x.code for x in negative]
# Verify the future transformations are well-founded but leave them unapplied.
future=json.loads((OUT/'REPAIRED_GROUP3_PROMOTION.json').read_text())
for f in future['group3_files']:
 p=ROOT/f['target'];data=p.read_bytes();assert sha(p)==f['applied_sha256']
 for op in f['operations']:
  assert data.count(op['old'].encode())==op['expected_occurrences'];data=data.replace(op['old'].encode(),op['new'].encode())
 assert hashlib.sha256(data).hexdigest()==f['accepted_sha256']
# Source/output identity and delta detail remain replayable.
report={'status':'PASS','phase':'REPAIRED_GROUP2_POSTSTATE_GROUP3_PENDING','actual_edges_sha256':sha(DAG/'DependencyEdges.csv'),'actual_nodes_sha256':sha(DAG/'DeliverableNodes.csv'),'rows':len(rows),'nodes':len(nodes),'preserved_rows':len(old),'preserved_nodes':102,'repaired_rows':84,'changed_fields':sorted(allowed),'unchanged_semantic_fields':'All other CSV fields are equal to accepted candidate; baseline prefix unchanged.','quote_checks':84,'stock_evq_drb_findings_on_repaired_rows':0,'local_registers':local,'stage_edges':len(ns['stage_edges'](rows)),'negative_probes':probes,'source_validator':{'path':rel(source),'sha256':sha(source),'reused_ast_nodes':['stage_alias','stage_edges','check']},'limits':['Only new metadata repaired; all existing satisfaction, owner scopes, endpoints, types and protected constraints preserved.','Baseline stock-validator findings are not repaired or waived here. Generic validate_id_format.sh uses a conflicting three-digit template; canonical Root SPEC DEP-owner form and stock owner-binding predicate are checked instead.','No graph adoption or lifecycle, product, native, engineering or release acceptance.']}
for name,data in [('REPAIRED_VALIDATION.json',report),('REPAIRED_QUOTE_CHECKS.json',quotes),('REPAIRED_LOCAL_COMMANDS.json',commands),('REPAIRED_StageGraph.json',{'status':'STAGED_NOT_APPROVED_GROUP3_PENDING','aliases':ns['stage_alias'],'edges':ns['stage_edges'](rows),'legacy_id_crosswalk':'ID_CROSSWALK.csv'}),('REPAIRED_StagedInterfaces.json',{'status':'STAGED_NOT_APPROVED_GROUP3_PENDING','bindings':staged,'legacy_id_crosswalk':'ID_CROSSWALK.csv'})]:
 (OUT/name).write_text(json.dumps(data,indent=2)+'\n')
print(json.dumps({k:report[k] for k in ['status','rows','nodes','repaired_rows','quote_checks','stock_evq_drb_findings_on_repaired_rows','stage_edges']}))
