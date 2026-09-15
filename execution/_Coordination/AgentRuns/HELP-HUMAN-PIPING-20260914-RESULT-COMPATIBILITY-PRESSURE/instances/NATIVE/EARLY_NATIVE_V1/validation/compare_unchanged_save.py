from pathlib import Path
import hashlib,json,sys
before=json.loads(Path(sys.argv[1]).read_text())
after=json.loads(Path(sys.argv[2]).read_text())
fields=["project_id","project_name","model_json","editor_intents_json","proposal_json","selected_review_target_json","mechanics_result_json","analysis_run_json","model_hash_json","project_envelope_hash_json","model_migration_ledger_json"]
comparison={}
for k in fields:
    comparison[k]={"exact_equal":before[k]==after[k],"before_sha256":hashlib.sha256(str(before[k]).encode()).hexdigest(),"after_sha256":hashlib.sha256(str(after[k]).encode()).hexdigest()}
result={"status":"PASS" if all(v["exact_equal"] for v in comparison.values()) else "FAIL","fields":comparison,"timestamp_fields":{"created_equal":before["created_at_unix"]==after["created_at_unix"],"updated_before":before["updated_at_unix"],"updated_after":after["updated_at_unix"]},"note":"timestamp fields are store metadata; exact preservation assertion applies to payload, result, analysis, hash and attachment fields"}
assert result["status"]=="PASS",json.dumps(result)
Path(sys.argv[3]).write_text(json.dumps(result,indent=2,sort_keys=True)+"\n")
print(json.dumps({"status":result["status"],"field_count":len(fields),"updated_changed":result["timestamp_fields"]["updated_before"]!=result["timestamp_fields"]["updated_after"]},sort_keys=True))

