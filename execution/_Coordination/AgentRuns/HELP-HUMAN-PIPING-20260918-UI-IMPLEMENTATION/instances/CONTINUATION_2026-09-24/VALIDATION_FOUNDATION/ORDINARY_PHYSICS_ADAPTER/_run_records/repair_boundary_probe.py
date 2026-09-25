# Adapted original independent probe with corrected assertions/output path. Original negative evidence is unchanged.
from pathlib import Path
from copy import deepcopy
import sys,json,hashlib
ROOT=Path('/private/tmp/piping-validation-foundation-20260925/projects/chirality-piping')
OUT=Path(__file__).parent
sys.path.insert(0,str(ROOT))
from tools.validation import qualification_gate as gate, qualification_physics as physics
D=OUT.parent
selected=json.loads((D/'EXECUTION_SELECTIONS/sparse_interactive/selection.json').read_text())
records=[]
for label,selection in [('complete',deepcopy(selected)),('missing_case',deepcopy(selected)),('missing_assertion',deepcopy(selected))]:
 if label=='missing_case':selection['cases'].pop()
 if label=='missing_assertion':selection['cases'][0]['assertions'].pop()
 ledger=gate.predeclare(selection)
 for case in ledger['cases']:gate.fail_case(case,'blocked','focused pure admission refusal placeholder; no process executed')
 gate.summarize(ledger)
 records.append({'scenario':label,'summary':ledger['summary'],'outcome':ledger['outcome']})
assert records[0]['summary']['required_assertions']==146 and records[1]['summary']['required_assertions']==146 and records[2]['summary']['required_assertions']==146
request={'request':{'request_id':'synthetic-boundary-probe'},'solve':{'preview_model':{'model':{'project':{'id':'synthetic-model'}}}}}
raw={'schema_version':'0.1.0','document_kind':'openpipestress.product_preview.mechanics_result','run_id':'run:preview-linear-static-001','model_ref':'synthetic-model','accepted_model_state_mutated':False,'status':{'mechanics':'MECHANICS_SOLVED'},'diagnostics':[],'results':[{'id':'mode','kind':'linear_solver_mode_basis','value':1,'unit':'mode_code','entity_ref':'solver'}]}
wrapper={'payload':{'artifact':'openpipestress.headless_runner_cli_output','schema_version':'1.0.0','command':'solve','operation':'solve','request_validation':{'diagnostics':[]},'result_validation':{'diagnostics':[]},'diagnostics':[],'runner_result':{'run_id':'run:headless-preview:synthetic-boundary-probe','job':{'state':'COMPLETED'},'analysis_status':['MECHANICS_SOLVED'],'diagnostics':[]},'mechanics_envelope':raw},'decisions':[],'findings':[],'blocked':False,'summary':{'blocking_count':0,'decision_count':0,'finding_count':0,'warning_count':0}}
namespace=[]
for label,fields in [('clean_legacy',{}),('source_receipt_presence',{'source_block_recovery':None}),('composite_producer_claim',{'producer':{'component_name':'open_pipe_stress_product_physics','component_version':'0.2.0','semantic_contract_id':'openpipestress.result_semantics/0.3.0/physics-source-1'}}),('foreign_namespace',{'carrier_evidence':None})]:
 candidate=deepcopy(wrapper);candidate['payload']['mechanics_envelope'].update(fields)
 try:gate.unwrap(json.dumps(candidate).encode(),request,'sparse_interactive');accepted=True;reason=None
 except ValueError as exc:accepted=False;reason=str(exc)
 namespace.append({'scenario':label,'legacy_transport_accepted':accepted,'reason':reason,'added_fields':fields})
assert namespace[0]['legacy_transport_accepted'] and not any(r['legacy_transport_accepted'] for r in namespace[1:])
result={'scope':'Bounded pure actual predeclare/summarize/unwrap functions with deliberately synthetic boundary data; no test suite, subprocess, helper, solver or actual first-static execution','denominator':records,'legacy_namespace':namespace,'source_sha256':hashlib.sha256((ROOT/'tools/validation/qualification_gate.py').read_bytes()).hexdigest()}
(OUT/'REPAIR_BOUNDARIES_01.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
