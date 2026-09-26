from pathlib import Path
from copy import deepcopy
import json,sys,hashlib
ROOT=Path('/private/tmp/piping-validation-foundation-20260925/projects/chirality-piping');O=Path(__file__).parent;D=O.parent
sys.path.insert(0,str(ROOT))
from tools.validation import qualification_gate as gate,qualification_physics as p
manifest=json.loads((D/'EXECUTION_SELECTIONS/sparse_interactive/selection.json').read_text())
original_package=D.parent/'FIRST_STATIC_BINDINGS'
for slug in ['axial','bending_torsion']:
 ids=[x['id'] for x in json.loads((original_package/(slug+'.selectors.candidate.json')).read_text())['assertions']]
 assert tuple(ids)==p.REQUIRED_ASSERTION_IDS and len(ids)==73
mutations={
 'missing_case':lambda m:m['cases'].pop(),
 'missing_assertion':lambda m:m['cases'][0]['assertions'].pop(),
 'duplicate_case':lambda m:m['cases'].append(deepcopy(m['cases'][0])),
 'duplicate_assertion':lambda m:m['cases'][0]['assertions'].append(deepcopy(m['cases'][0]['assertions'][0])),
 'unknown_assertion':lambda m:m['cases'][0]['assertions'][0].update(id='unknown'),
 'reversed_cases':lambda m:m['cases'].reverse(),
 'reversed_assertions':lambda m:m['cases'][0]['assertions'].reverse(),
 'null_cases':lambda m:m.update(cases=None),
 'null_assertions':lambda m:m['cases'][0].update(assertions=None),
 'nonobject_case':lambda m:m['cases'].__setitem__(0,'bad'),
 'nonobject_assertion':lambda m:m['cases'][0]['assertions'].__setitem__(0,'bad'),
 'empty_cases':lambda m:m.update(cases=[]),
}
records=[]
for mode in ['sparse_interactive','dense_scrutiny']:
 for label,mutate in mutations.items():
  x=deepcopy(manifest);x['runner']['solver_mode']=mode;mutate(x);ledger=gate.predeclare(x)
  refused=False
  try:p.validate_submitted_inventory(x['cases'])
  except ValueError as exc:refused=True;reason=str(exc)
  assert refused
  for case in ledger['cases']:gate.fail_case(case,'blocked',reason)
  gate.summarize(ledger);s=ledger['summary']
  assert (s['required_cases'],s['required_assertions'],s['required_structural_checks'],s['required_section_subchecks'])==(2,146,20,18)
  assert s['assertions']['blocked']==146 and s['structural_checks']['blocked']==20 and s['section_subchecks']['blocked']==18
  assert ledger['outcome']=='not_satisfied' and ledger['submitted_inventory'] is not None
  assert [c['id'] for c in ledger['cases']]==list(p.REQUIRED_CASE_IDS)
  for case in ledger['cases']:
   assert [a['id'] for a in case['assertions']]==list(p.REQUIRED_ASSERTION_IDS)
   subs=next(c for c in case['structural_checks'] if c['id']=='complete_case_material_section')['details']['section_quantities'];assert [x['id'] for x in subs]==list(p.REQUIRED_SECTION_IDS)
  records.append({'mode':mode,'mutation':label,'refused':True,'counts':[2,146,20,18],'submitted_inventory':ledger['submitted_inventory']})
# Pure identity-association consequence: reversed prepared order cannot swap labels.
ledger={'assertions':[{'id':'a','state':'not_run'},{'id':'b','state':'not_run'}]}
selector=lambda id:{'id':id,'kind':'synthetic-force','unit':'N','entity_ref':'member','basis_ref':{'ref_type':'load_case','ref_id':'case'},'metadata':None}
rule={'unit_ref':{'ref':'N'},'relative_tolerance_value':0.,'absolute_tolerance_value':0.}
prepared={'profile':{'synthetic':'focus only'},'selected':[{'assertion':{'id':'b','selector':selector('rb')},'expected':2.,'rule':rule},{'assertion':{'id':'a','selector':selector('ra')},'expected':1.,'rule':rule}]}
rows=[{**selector('ra'),'value':1.},{**selector('rb'),'value':2.}]
for row in rows:row.pop('metadata')
gate.evaluate_rows(ledger,prepared,rows);assert ledger['state']=='matched';assert [(r['id'],r['observed']) for r in ledger['assertions']]==[('a',1.),('b',2.)]
partial={'assertions':[{'id':'a','state':'not_run'},{'id':'b','state':'not_run'}]};gate.evaluate_rows(partial,{**prepared,'selected':prepared['selected'][:1]},rows);assert partial['state']!='matched' and partial['assertions'][0]['state']=='not_run'
# Replay the original clean and contradictory namespace controls; include all six fields and falsy values.
request={'request':{'request_id':'synthetic-boundary-probe'},'solve':{'preview_model':{'model':{'project':{'id':'synthetic-model'}}}}}
raw={'schema_version':'0.1.0','document_kind':'openpipestress.product_preview.mechanics_result','run_id':'run:preview-linear-static-001','model_ref':'synthetic-model','accepted_model_state_mutated':False,'status':{'mechanics':'MECHANICS_SOLVED'},'diagnostics':[],'results':[{'id':'mode','kind':'linear_solver_mode_basis','value':1,'unit':'mode_code','entity_ref':'solver'}]}
wrapper={'payload':{'artifact':'openpipestress.headless_runner_cli_output','schema_version':'1.0.0','command':'solve','operation':'solve','request_validation':{'diagnostics':[]},'result_validation':{'diagnostics':[]},'diagnostics':[],'runner_result':{'run_id':'run:headless-preview:synthetic-boundary-probe','job':{'state':'COMPLETED'},'analysis_status':['MECHANICS_SOLVED'],'diagnostics':[]},'mechanics_envelope':raw},'decisions':[],'findings':[],'blocked':False,'summary':{'blocking_count':0,'decision_count':0,'finding_count':0,'warning_count':0}}
gate.unwrap(json.dumps(wrapper).encode(),request,'sparse_interactive')
namespace=[]
for key in ['producer','numerical_quality','formulation_basis','contract_evidence','source_block_recovery','carrier_evidence']:
 for value in [None,False,{},[],'']:
  bad=deepcopy(wrapper);bad['payload']['mechanics_envelope'][key]=value
  refused=False
  try:gate.unwrap(json.dumps(bad).encode(),request,'sparse_interactive')
  except ValueError as exc:refused='not legacy raw0.1' in str(exc)
  assert refused;namespace.append({'field':key,'value':value,'refused':True})
# Legacy caller-selected denominators remain caller-selected.
legacy={'profile_id':'synthetic-legacy','transport':gate.TRANSPORT,'cases':[{'id':'legacy','assertions':[{'id':'a'}]}]}
l=gate.predeclare(legacy);gate.summarize(l);assert l['summary']['required_assertions']==1 and l['summary']['required_structural_checks']==l['summary']['required_section_subchecks']==0
result={'status':'PASS OPA-01/OPA-02 pure independent repair/consequence backcheck','scope':'Actual Python functions; synthetic boundary observations only; no subprocess, helper, pytest, build or real first-static run','inventories':records,'namespace':namespace,'clean_legacy_accepted':True,'legacy_selected_denominator_unchanged':True,'identity_order_control':'reversed prepared list retains a=1,b=2; omitted prepared item remains not_run/nonmatched','counts':{'inventory_negatives':24,'namespace_negatives':30},'current_source':[{ 'path':str(ROOT/'tools/validation'/name),'sha256':hashlib.sha256((ROOT/'tools/validation'/name).read_bytes()).hexdigest()} for name in ['qualification_gate.py','qualification_physics.py']]}
(O/'REPAIR_BOUNDARIES_02.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({k:v for k,v in result.items() if k not in ['inventories','namespace','current_source']},indent=2))
