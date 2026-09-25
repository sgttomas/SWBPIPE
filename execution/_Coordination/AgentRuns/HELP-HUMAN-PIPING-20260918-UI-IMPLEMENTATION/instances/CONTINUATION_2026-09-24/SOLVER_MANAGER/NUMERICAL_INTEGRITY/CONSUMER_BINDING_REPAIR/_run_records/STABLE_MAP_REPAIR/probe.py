"""Source-only binding probe. Injected deterministic JSON hash is NOT production JCS.
It holds checksum generation and validation consistent while isolating relational
validation. No subprocess, build, test suite, or native code executes."""
import ast
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import sys

ROOT=Path(__file__).resolve()
while not (ROOT/'projects/chirality-piping').is_dir():
    ROOT=ROOT.parent
P=ROOT/'projects/chirality-piping'
sys.path.insert(0,str(P))
from core.serialization.canonical_json import adapter
encode=lambda x: json.dumps(x,sort_keys=True,separators=(',',':'),allow_nan=False)
digest=lambda x: hashlib.sha256(encode(x).encode()).hexdigest()
adapter.canonical_json_checked_v1=encode
adapter.canonical_sha256_checked_v1=digest
from core.analysis_runs import compatibility as a
from core.handoff.stress_neutral import package_v0_3 as s
from core.handoff.stress_neutral.package import render_stress_neutral_csv
scope={'deepcopy':deepcopy,'PRECISION_CONTRACT_ID':a.PRECISION_CONTRACT_ID,
       'BASIS':{'ref_type':'load_case','ref_id':'load:actual'}}
for path,names in [
    (P/'tests/test_precision_consumer_contract.py',['source']),
    (P/'tests/test_stress_neutral_export_package.py',['ref','source_payload'])
]:
    tree=ast.parse(path.read_text())
    exec(compile(ast.Module(body=[node for node in tree.body if isinstance(node,ast.FunctionDef) and node.name in names],type_ignores=[]),str(path),'exec'),scope)
raw=scope['source']()
raw['results']=[{'id':f'result:{i}','kind':'displacement_magnitude','value':x,'unit':'mm','entity_ref':'node:actual','basis_ref':scope['BASIS']} for i,x in enumerate([1.2345678901234567e-7,-1.0000000000000002])]
record=a.build_analysis_run_v0_3(raw,input_manifest_ref={'object_type':'InputManifest','ref':'manifest:test'},input_manifest_hash='1'*64)
args=scope['source_payload']()
args.update(source_result_ref={'object_type':'ResultEnvelope','ref':f"result-envelope:{raw['run_id']}"},source_run_ref={'object_type':'AnalysisRun','ref':raw['run_id']},source_model_ref={'object_type':'Model','ref':raw['model_ref']},source_hashes=deepcopy(record['analysis_run']['hashes']))
args['reproducibility_refs']=[args['source_run_ref']]
for row,original,mapping in zip(args['result_rows'],raw['results'],args['stable_id_map']):
    row.update(result_id=original['id'],canonical_ref={'object_type':'Result','ref':original['id']},source_result_ref={'object_type':'Result','ref':original['id']},value=original['value'],unit='mm',dimension='length',result_family='displacement',load_case_ref={'object_type':'LoadCase','ref':'load:actual'},component_ref={'object_type':'Node','ref':'node:actual'})
    mapping.update(canonical_ref=row['canonical_ref'],export_ref={'object_type':'StressNeutralRow','ref':original['id']})
for row,original in zip(args['result_rows'],raw['results']): row.update(s.source_row_projection_v0_3(raw,original))

base=s.build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=record,**args)
observations=[]
def attempt(name,fn):
    try:
        fn();observations.append({'case':name,'result':'ACCEPTED'})
    except Exception as exc:
        observations.append({'case':name,'result':'REJECTED','exception':type(exc).__name__,'message':str(exc)})
for field,value in [('canonical_ref',{'object_type':'Material','ref':'result:0'}),('export_ref',{'object_type':'Row','ref':''}),('export_ref',{'object_type':'Row','ref':None}),('export_ref',None)]:
    wrong=deepcopy(args);wrong['stable_id_map'][0][field]=value
    attempt('builder_'+field+'_'+str(value),lambda wrong=wrong:s.build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=record,**wrong))
custom=deepcopy(args)
custom['stable_id_map'][0]['export_ref']={'object_type':'CustomA','ref':'same'}
custom['stable_id_map'][1]['export_ref']={'object_type':'CustomB','ref':'same'}
custom['result_rows'].reverse();custom['stable_id_map'].reverse()
attempt('custom_namespaces_reordered',lambda:s.build_stress_neutral_export_package_v0_3(source_envelope=raw,analysis_record=record,**custom))
for value in ['',0,False,[],{},'UNKNOWN',None]:
    attempt('rule_override_'+repr(value),lambda value=value:a.build_analysis_run_v0_3(raw,input_manifest_ref={'object_type':'InputManifest','ref':'m'},input_manifest_hash='1'*64,rule_check_status=value))

def rehash(packet):
    members=s.materialized_members_v0_3(packet)
    checks=[c for c in packet['manifest']['checksums'] if c['payload_ref']['ref']!='manifest.json']
    for c in checks:c['value']=hashlib.sha256(members[c['payload_ref']['ref']]).hexdigest()
    checks.append(s._checked(s._manifest_seed(packet,checks),'manifest.json','manifest_seed'))
    packet['manifest']['checksums']=checks
    packet['manifest']['package_members']=[{'filename':name,'checksum':deepcopy(next(c for c in checks if c['payload_ref']['ref']==name))} for name in s.MEMBERS]
    packet['package_checksum']['value']=digest(s.package_projection(packet))
for field,value in [('canonical_ref',{'object_type':'Material','ref':'result:0'}),('export_ref',{'object_type':'Row','ref':''}),('export_ref',{'object_type':'Row','ref':None}),('export_ref',None)]:
    packet=deepcopy(base);packet['stable_id_map'][0][field]=value;rehash(packet)
    attempt('rehashed_'+field+'_'+str(value),lambda packet=packet:s.validate_stress_neutral_export_package_v0_3(packet,source_envelope=raw,analysis_record=record))
print(json.dumps({'actor':'/root/solver_manager/consumer_binding_repair','method':'source-only injected deterministic JSON hash; NOT production JCS/native or schema proof','observations':observations},indent=2))
