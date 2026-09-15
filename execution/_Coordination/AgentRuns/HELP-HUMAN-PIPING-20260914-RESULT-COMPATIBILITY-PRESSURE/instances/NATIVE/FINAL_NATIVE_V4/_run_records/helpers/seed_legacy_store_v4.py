#!/usr/bin/env python3
"""Seed one prepared legacy row into the V4 isolated store while the app is closed."""
from __future__ import annotations
import argparse, hashlib, json, shutil, sqlite3
from pathlib import Path

p=argparse.ArgumentParser(); p.add_argument('--baseline-store',required=True,type=Path); p.add_argument('--active-store',required=True,type=Path); p.add_argument('--prepared-row',required=True,type=Path); p.add_argument('--evidence-dir',required=True,type=Path); p.add_argument('--isolated-binding-token',required=True); a=p.parse_args()
baseline,active,evidence=a.baseline_store.resolve(),a.active_store.resolve(),a.evidence_dir.resolve(); prepared=json.loads(a.prepared_row.read_text()); row=prepared['row']
if a.isolated_binding_token not in str(active): raise SystemExit('ACTIVE_STORE_NOT_ISOLATED_BINDING')
if any(Path(str(active)+suffix).exists() for suffix in ('-wal','-shm')): raise SystemExit('ACTIVE_STORE_SIDECAR_PRESENT_APP_MUST_BE_CLOSED')
evidence.mkdir(parents=True,exist_ok=True); target=evidence/f"{prepared['case_id']}.before_open.sqlite3"
if target.exists(): raise SystemExit('EVIDENCE_TARGET_EXISTS')
shutil.copy2(baseline,active); db=sqlite3.connect(active); fields=['project_name','model_json','editor_intents_json','proposal_json','selected_review_target_json','mechanics_result_json','analysis_run_json','model_hash_json','project_envelope_hash_json','model_migration_ledger_json']
with db:
    old=prepared['baseline_project_id']; found=db.execute('select count(*) from local_projects where project_id=?',(old,)).fetchone()[0]
    if found!=1: raise AssertionError(f'BASELINE_PROJECT_CARDINALITY:{found}')
    db.execute('update local_projects set project_id=?,'+','.join(f'{field}=?' for field in fields)+' where project_id=?',(row['project_id'],)+tuple(row[field] for field in fields)+(old,))
    actual=db.execute('select '+','.join(fields)+' from local_projects where project_id=?',(row['project_id'],)).fetchone()
    if tuple(actual)!=tuple(row[field] for field in fields): raise AssertionError('SEEDED_ROW_MISMATCH')
    if db.execute('pragma integrity_check').fetchone()[0]!='ok': raise AssertionError('SQLITE_INTEGRITY')
db.close(); src=sqlite3.connect(active); dst=sqlite3.connect(target); src.backup(dst); dst.close(); src.close()
sha=lambda path:hashlib.sha256(path.read_bytes()).hexdigest(); record={'schema':'legacy-store-seed-capture-v4','case_id':prepared['case_id'],'prepared_row':str(a.prepared_row.resolve()),'prepared_row_sha256':sha(a.prepared_row.resolve()),'baseline_store_sha256':sha(baseline),'before_open_snapshot':str(target),'before_open_snapshot_sha256':sha(target),'app_process_required_absent_during_call':True}; (evidence/f"{prepared['case_id']}.seed.json").write_text(json.dumps(record,indent=2,sort_keys=True)+'\n'); print(json.dumps({'status':'SEEDED','case_id':prepared['case_id'],'snapshot':str(target)},sort_keys=True))
