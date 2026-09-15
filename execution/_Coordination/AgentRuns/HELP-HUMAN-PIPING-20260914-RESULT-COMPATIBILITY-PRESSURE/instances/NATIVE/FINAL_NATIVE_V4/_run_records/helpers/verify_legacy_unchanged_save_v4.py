#!/usr/bin/env python3
"""Verify all persisted payload fields of one legacy GUI unchanged-save."""
from __future__ import annotations
import argparse, hashlib, json, sqlite3
from pathlib import Path

p=argparse.ArgumentParser(); p.add_argument('--prepared-row',required=True,type=Path); p.add_argument('--store',required=True,type=Path); p.add_argument('--report',required=True,type=Path); p.add_argument('--isolated-binding-token',required=True); a=p.parse_args(); prepared=json.loads(a.prepared_row.read_text()); store=a.store.resolve()
if a.isolated_binding_token not in str(store): raise SystemExit('STORE_NOT_ISOLATED_BINDING')
if any(Path(str(store)+suffix).exists() for suffix in ('-wal','-shm')): raise SystemExit('STORE_SIDECAR_PRESENT_APP_MUST_BE_CLOSED')
db=sqlite3.connect(f'file:{store}?mode=ro&immutable=1',uri=True); db.row_factory=sqlite3.Row; row=db.execute('select * from local_projects where project_id=?',(prepared['row']['project_id'],)).fetchone(); integrity=db.execute('pragma integrity_check').fetchone()[0]; db.close()
if row is None: raise AssertionError('PROJECT_MISSING_AFTER_SAVE')
actual=dict(row); fields=['project_name','model_json','editor_intents_json','proposal_json','selected_review_target_json','mechanics_result_json','analysis_run_json','model_hash_json','project_envelope_hash_json','model_migration_ledger_json']; h=lambda value:hashlib.sha256(str(value).encode()).hexdigest(); checks={field:{'exact':actual[field]==prepared['row'][field],'expected_sha256':h(prepared['row'][field]),'actual_sha256':h(actual[field])} for field in fields}
if integrity!='ok' or not all(item['exact'] for item in checks.values()): raise AssertionError('LEGACY_UNCHANGED_SAVE_PAYLOAD_CHANGED')
report={'schema':'legacy-historical-unchanged-save-verification-v4','status':'PASS','case_id':prepared['case_id'],'profile':prepared['profile'],'analysis_schema_version':json.loads(actual['analysis_run_json'])['schema_version'],'source_fixture':prepared['expectations']['source_fixture'],'source_fixture_sha256':prepared['expectations']['source_fixture_sha256'],'payload_fields':checks,'mechanics_result_json_sha256':h(actual['mechanics_result_json']),'analysis_run_json_sha256':h(actual['analysis_run_json']),'sqlite_integrity_check':integrity,'timestamps':{'created_at_unix':actual['created_at_unix'],'updated_at_unix':actual['updated_at_unix']}}
a.report.parent.mkdir(parents=True,exist_ok=True); a.report.write_text(json.dumps(report,indent=2,sort_keys=True)+'\n'); print(json.dumps({'status':'PASS','case_id':prepared['case_id'],'payload_fields':len(fields)},sort_keys=True))
