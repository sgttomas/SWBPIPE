#!/usr/bin/env python3
"""Preparation only: restore a disposable baseline SQLite store, seed one prepared row, and capture exact bytes."""
from __future__ import annotations
import argparse,hashlib,json,shutil,sqlite3
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('--baseline-store',required=True,type=Path);p.add_argument('--active-store',required=True,type=Path);p.add_argument('--prepared-row',required=True,type=Path);p.add_argument('--evidence-dir',required=True,type=Path);p.add_argument('--isolated-binding-token',required=True);a=p.parse_args()
baseline,active,prepared,evidence=a.baseline_store.resolve(),a.active_store.resolve(),json.loads(a.prepared_row.read_text()),a.evidence_dir.resolve()
if a.isolated_binding_token not in str(active): raise SystemExit('ACTIVE_STORE_NOT_ISOLATED_BINDING')
if not baseline.is_file(): raise SystemExit('BASELINE_STORE_MISSING')
if baseline==active: raise SystemExit('BASELINE_AND_ACTIVE_STORE_MUST_DIFFER')
if any(Path(str(active)+suffix).exists() for suffix in ('-wal','-shm')): raise SystemExit('ACTIVE_STORE_SIDECAR_PRESENT_APP_MUST_BE_CLOSED')
evidence.mkdir(parents=True,exist_ok=True);case=prepared['case_id'];target=evidence/f'{case}.before_open.sqlite3'
if target.exists(): raise SystemExit('EVIDENCE_TARGET_EXISTS')
shutil.copy2(baseline,active);db=sqlite3.connect(active);row=prepared['row'];project=row['project_id']
fields=['project_name','model_json','editor_intents_json','proposal_json','selected_review_target_json','mechanics_result_json','analysis_run_json','model_hash_json','project_envelope_hash_json','model_migration_ledger_json']
with db:
 found=db.execute('select count(*) from local_projects where project_id=?',(project,)).fetchone()[0]
 if found!=1: raise AssertionError(f'PROJECT_CARDINALITY:{found}')
 db.execute('update local_projects set '+','.join(f'{field}=?' for field in fields)+' where project_id=?',tuple(row[field] for field in fields)+(project,))
 actual=db.execute('select '+','.join(fields)+' from local_projects where project_id=?',(project,)).fetchone()
 if tuple(actual)!=tuple(row[field] for field in fields): raise AssertionError('SEEDED_ROW_MISMATCH')
 if db.execute('pragma integrity_check').fetchone()[0]!='ok': raise AssertionError('SQLITE_INTEGRITY')
db.close();src=sqlite3.connect(active);dst=sqlite3.connect(target);src.backup(dst);dst.close();src.close()
record={'schema':'malformed-store-seed-capture-v5','case_id':case,'active_store':str(active),'baseline_store_sha256':hashlib.sha256(baseline.read_bytes()).hexdigest(),'prepared_row':str(a.prepared_row.resolve()),'prepared_row_sha256':hashlib.sha256(a.prepared_row.read_bytes()).hexdigest(),'before_open_snapshot':str(target),'before_open_snapshot_sha256':hashlib.sha256(target.read_bytes()).hexdigest(),'mechanics_result_json':row['mechanics_result_json'],'mechanics_result_json_sha256':hashlib.sha256(row['mechanics_result_json'].encode()).hexdigest(),'app_process_required_absent_during_call':True}
(evidence/f'{case}.seed.json').write_text(json.dumps(record,indent=2,sort_keys=True)+'\n');print(json.dumps({'status':'SEEDED','case_id':case,'snapshot':str(target)},sort_keys=True))
