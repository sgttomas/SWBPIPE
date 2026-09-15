#!/usr/bin/env python3
"""Preparation only: verify the synthetic malformed carrier and all payload fields survived GUI unchanged-save."""
from __future__ import annotations
import argparse,hashlib,json,sqlite3
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--prepared-row',required=True,type=Path);p.add_argument('--store',required=True,type=Path);p.add_argument('--report',required=True,type=Path);p.add_argument('--isolated-binding-token',required=True);a=p.parse_args()
store=a.store.resolve(); prepared=json.loads(a.prepared_row.read_text())
if a.isolated_binding_token not in str(store): raise SystemExit('STORE_NOT_ISOLATED_BINDING')
if any(Path(str(store)+suffix).exists() for suffix in ('-wal','-shm')): raise SystemExit('STORE_SIDECAR_PRESENT_APP_MUST_BE_CLOSED')
db=sqlite3.connect(f'file:{store}?mode=ro&immutable=1',uri=True);db.row_factory=sqlite3.Row
if db.execute('pragma integrity_check').fetchone()[0]!='ok': raise AssertionError('SQLITE_INTEGRITY')
row=db.execute('select * from local_projects where project_id=?',(prepared['row']['project_id'],)).fetchone();db.close()
if row is None: raise AssertionError('PROJECT_MISSING_AFTER_SAVE')
actual=dict(row); fields=['project_name','model_json','editor_intents_json','proposal_json','selected_review_target_json','mechanics_result_json','analysis_run_json','model_hash_json','project_envelope_hash_json','model_migration_ledger_json']
checks={field:{'exact':actual[field]==prepared['row'][field],'expected_sha256':hashlib.sha256(str(prepared['row'][field]).encode()).hexdigest(),'actual_sha256':hashlib.sha256(str(actual[field]).encode()).hexdigest()} for field in fields}
if not all(item['exact'] for item in checks.values()): raise AssertionError('MALFORMED_UNCHANGED_SAVE_PAYLOAD_CHANGED')
if actual['mechanics_result_json']!=prepared['expectations']['mechanics_result_json']: raise AssertionError('MALFORMED_CARRIER_CHANGED')
report={'schema':'malformed-historical-unchanged-save-verification-v5','status':'PASS','case_id':prepared['case_id'],'payload_fields':checks,'mechanics_result_json':actual['mechanics_result_json'],'mechanics_result_json_sha256':hashlib.sha256(actual['mechanics_result_json'].encode()).hexdigest(),'timestamps':{'created_at_unix':actual['created_at_unix'],'updated_at_unix':actual['updated_at_unix']},'gui_assertions_required_separately':['historical_saved_run visible','final repaired malformed-evidence finding visible','no Current solve/result/export state','safe fallback rendering','normal quit before this read']}
a.report.parent.mkdir(parents=True,exist_ok=True);a.report.write_text(json.dumps(report,indent=2,sort_keys=True)+'\n');print(json.dumps({'status':'PASS','case_id':prepared['case_id'],'payload_fields':len(fields)},sort_keys=True))
