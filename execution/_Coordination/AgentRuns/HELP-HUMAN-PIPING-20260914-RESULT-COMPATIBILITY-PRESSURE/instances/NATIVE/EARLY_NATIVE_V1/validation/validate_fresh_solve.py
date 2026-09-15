from __future__ import annotations
from copy import deepcopy
from pathlib import Path
import hashlib,json,os,sys
project=Path(sys.argv[1]).resolve()
first=json.loads(Path(sys.argv[2]).read_text())
fresh=json.loads(Path(sys.argv[3]).read_text())
output=Path(sys.argv[4]).resolve()
checked=Path(sys.argv[5]).resolve()
os.environ["OPENPIPESTRESS_CHECKED_JSON_BIN"]=str(checked)
sys.path.insert(0,str(project)); sys.path.insert(0,str(project/"tests"))
from schema_validation import validate_instance
from core.serialization.canonical_json.adapter import canonicalize_batch
a=json.loads(first["analysis_run_json"]); b=json.loads(fresh["analysis_run_json"])
m1=json.loads(first["mechanics_result_json"]); m2=json.loads(fresh["mechanics_result_json"])
schema=json.loads((project/"schemas/analysis_run.schema.json").read_text())
validate_instance(schema,b,schema_label="analysis_run dispatcher",instance_label="fresh stored native analysis")
record_claim=next(h for h in b["analysis_run"]["hashes"] if h["payload_scope"]=="analysis_run_record")
received_claim=next(h for h in b["analysis_run"]["hashes"] if h["payload_scope"]=="received_result")
projection=deepcopy(b); projection["analysis_run"]["hashes"]=[h for h in projection["analysis_run"]["hashes"] if h["payload_scope"]!="analysis_run_record"]
canon=canonicalize_batch([("record",projection),("received",m2)])
record_digest=hashlib.sha256(canon["record"].encode()).hexdigest()
received_digest=hashlib.sha256(canon["received"].encode()).hexdigest()
def diffs(x,y,path="$"):
    out=[]
    if type(x) is not type(y): return [path]
    if isinstance(x,dict):
        for k in sorted(set(x)|set(y)):
            if k not in x or k not in y: out.append(path+"."+k)
            else: out.extend(diffs(x[k],y[k],path+"."+k))
    elif isinstance(x,list):
        if len(x)!=len(y): out.append(path+".length")
        for i,(u,v) in enumerate(zip(x,y)): out.extend(diffs(u,v,f"{path}[{i}]"))
    elif x!=y: out.append(path)
    return out
all_diffs=diffs(a,b)
assert m1==m2
assert record_claim["value"]==record_digest and received_claim["value"]==received_digest
assert len(b["analysis_run"]["result_refs"])==830
result={
 "status":"PASS",
 "fresh_schema_validation":"PASS",
 "fresh_run_id":b["analysis_run"]["run_id"],
 "fresh_result_ref_count":len(b["analysis_run"]["result_refs"]),
 "fresh_full_record_checksum":{"advertised":record_claim["value"],"computed":record_digest,"match":True},
 "fresh_received_result_checksum":{"advertised":received_claim["value"],"computed":received_digest,"match":True},
 "mechanics_content":{"exact_equal_to_first_solve":True,"sha256":hashlib.sha256(fresh["mechanics_result_json"].encode()).hexdigest()},
 "analysis_record_changed":a!=b,
 "analysis_changed_paths":all_diffs,
 "first_input_manifest_hashes":a["analysis_run"]["reproducibility"]["input_manifest_hashes"],
 "fresh_input_manifest_hashes":b["analysis_run"]["reproducibility"]["input_manifest_hashes"],
 "fresh_input_manifest_payload_persisted":False,
 "field_sha256":{k:hashlib.sha256(str(fresh[k]).encode()).hexdigest() for k in ["model_json","editor_intents_json","proposal_json","selected_review_target_json","mechanics_result_json","analysis_run_json","model_hash_json","project_envelope_hash_json","model_migration_ledger_json"]}
}
output.write_text(json.dumps(result,indent=2,sort_keys=True)+"\n")
Path(sys.argv[6]).write_text(json.dumps(b,indent=2,sort_keys=True)+"\n")
Path(sys.argv[7]).write_text(json.dumps(m2,indent=2,sort_keys=True)+"\n")
print(json.dumps({"status":"PASS","mechanics_equal":True,"analysis_changed_paths":len(all_diffs),"fresh_record_checksum_match":True,"fresh_received_checksum_match":True},sort_keys=True))

