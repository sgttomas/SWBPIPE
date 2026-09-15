from __future__ import annotations
from copy import deepcopy
from pathlib import Path
import hashlib,json,os,sys
project=Path(sys.argv[1]).resolve()
row_path=Path(sys.argv[2]).resolve()
output=Path(sys.argv[3]).resolve()
checked=Path(sys.argv[4]).resolve()
assert checked.is_file()
os.environ["OPENPIPESTRESS_CHECKED_JSON_BIN"]=str(checked)
sys.path.insert(0,str(project))
sys.path.insert(0,str(project/"tests"))
from schema_validation import validate_instance
from core.serialization.canonical_json.adapter import canonicalize_batch
row=json.loads(row_path.read_text())
analysis=json.loads(row["analysis_run_json"])
mechanics=json.loads(row["mechanics_result_json"])
dispatcher=json.loads((project/"schemas/analysis_run.schema.json").read_text())
selected=json.loads((project/"schemas/analysis_run.v0.2.schema.json").read_text())
validate_instance(dispatcher,analysis,schema_label="analysis_run dispatcher",instance_label="stored native analysis")
validate_instance(selected,analysis,schema_label="analysis_run 0.2",instance_label="stored native analysis")
hashes=analysis["analysis_run"]["hashes"]
record_claims=[h for h in hashes if h.get("payload_scope")=="analysis_run_record"]
received_claims=[h for h in hashes if h.get("payload_scope")=="received_result"]
assert len(record_claims)==1 and len(received_claims)==1
projection=deepcopy(analysis)
projection["analysis_run"]["hashes"]=[h for h in projection["analysis_run"]["hashes"] if h.get("payload_scope")!="analysis_run_record"]
refs=analysis["analysis_run"]["result_refs"]
rows=mechanics["results"]
assert len(refs)==len(rows)
items=[("analysis_record_projection",projection),("received_result",mechanics)]
for index,(ref,result) in enumerate(zip(refs,rows)):
    assert ref["source_row_index"]==index
    items.append((f"row-{index}",result))
canonical=canonicalize_batch(items)
digests={key:hashlib.sha256(value.encode("utf-8")).hexdigest() for key,value in canonical.items()}
row_mismatches=[]
for index,ref in enumerate(refs):
    claims=ref["hash_refs"]
    if len(claims)!=1 or claims[0].get("value")!=digests[f"row-{index}"]:
        row_mismatches.append(index)
record_match=record_claims[0]["value"]==digests["analysis_record_projection"]
received_match=received_claims[0]["value"]==digests["received_result"]
assert record_match and received_match and not row_mismatches
known=sum(1 for ref in refs if ref.get("semantic_contract",{}).get("signature_id"))
unavailable=len(refs)-known
ledger=json.loads(row["model_migration_ledger_json"])
model_hash=json.loads(row["model_hash_json"])
assert len(ledger)==1
ledger_record=ledger[0]
ledger_checks={
 "record_kind":ledger_record.get("record_kind")=="model_document_migration_ledger_record",
 "post_model_matches_stored":ledger_record.get("post_migration_model_hash")==model_hash.get("value"),
 "computed_post_model_matches_stored":ledger_record.get("hash_evidence",{}).get("computed",{}).get("post_migration_model_hash")==model_hash.get("value"),
}
assert all(ledger_checks.values())
result={
 "status":"PASS",
 "source_snapshot":str(row_path),
 "schema_validation":{"dispatcher":"PASS","selected_schema":"analysis_run.v0.2.schema.json","selected_version":analysis["schema_version"],"strict_v0_2":"PASS"},
 "checked_authority":{"executable":str(checked),"executable_sha256":hashlib.sha256(checked.read_bytes()).hexdigest(),"protocol_profile":"openpipestress_jcs_ijson_v1","items_canonicalized":len(items)},
 "analysis_run":{"run_id":analysis["analysis_run"]["run_id"],"result_ref_count":len(refs),"known_semantic_signature_rows":known,"semantics_unavailable_rows":unavailable,"analysis_status":analysis["analysis_run"]["analysis_status"],"input_manifest_refs":analysis["analysis_run"]["reproducibility"]["input_manifest_refs"],"input_manifest_hashes":analysis["analysis_run"]["reproducibility"]["input_manifest_hashes"],"input_manifest_payload_persisted":False},
 "full_record_self_exclusion_checksum":{"advertised":record_claims[0],"independently_computed":digests["analysis_record_projection"],"match":record_match,"projection":"whole envelope with all analysis_run.hashes entries of payload_scope analysis_run_record removed"},
 "received_result_checksum":{"advertised":received_claims[0],"independently_computed":digests["received_result"],"match":received_match,"payload":"exact parsed mechanics_result_json from same stored row"},
 "result_row_checksums":{"count":len(refs),"mismatches":row_mismatches,"status":"PASS"},
 "migration_ledger":{"record_count":len(ledger),"checks":ledger_checks,"record":ledger_record,"envelope_note":"computed post-migration envelope binds the create-time normalized return and is not asserted equal to the later solve/save envelope"},
 "stored_field_sha256":{k:hashlib.sha256(str(row[k]).encode()).hexdigest() for k in ["model_json","editor_intents_json","proposal_json","selected_review_target_json","mechanics_result_json","analysis_run_json","model_hash_json","project_envelope_hash_json","model_migration_ledger_json"]}
}
output.write_text(json.dumps(result,indent=2,sort_keys=True)+"\n")
print(json.dumps(result["schema_validation"],sort_keys=True))
print(json.dumps({"record_match":record_match,"received_match":received_match,"row_checks":len(refs),"row_mismatches":len(row_mismatches)},sort_keys=True))

