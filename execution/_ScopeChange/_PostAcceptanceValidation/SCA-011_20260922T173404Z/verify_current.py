#!/usr/bin/env python3
"""Validate accepted current bytes without modifying the immutable SCA snapshot."""
import csv,hashlib,json,subprocess
from pathlib import Path
POST=Path(__file__).resolve().parent
ROOT=POST
while not (ROOT/'tools/scope_of_work/validate_scope_of_work.py').exists(): ROOT=ROOT.parent
P=ROOT/'projects/chirality-piping';RUN=P/'execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP'
BASE='d6cc1482eee78ce860ff18658f11157f7efbd401'
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def old(name):return subprocess.check_output(['git','show',BASE+':'+name],cwd=ROOT)
errors=[]
manifest=list(csv.DictReader((RUN/'ACCEPTED_MANIFEST.csv').open()))
for row in manifest:
 q=ROOT/row['Path']
 if not q.is_file()or sha(q)!=row['SHA256']:errors.append('ACCEPTED_BINDING '+row['Path'])
final=json.loads((POST/'FINAL_TARGET_MANIFEST.json').read_text())
for row in final['files']:
 if sha(ROOT/row['target'])!=row['expected_final_sha256']:errors.append('FINAL_TARGET '+row['target'])
for relative,required in [('execution/_ScopeChange/_LATEST.md','SCA-011_2026-09-22_OWNERSHIP'),('execution/_DAG/_LATEST.md','DAG-011')]:
 if required not in (P/relative).read_text():errors.append('ACTIVE_POINTER '+relative)
for name in ['DependencyEdges.csv','DeliverableNodes.csv']:
 q=P/'execution/_DAG/DAG-011'/name
 if q.read_bytes()!=old(str(q.relative_to(ROOT))):errors.append('GRAPH_BYTES_CHANGED '+name)
protected=['execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/application','execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/evidence','execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/candidate','execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/interfaces','execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/dependencies','execution/_ScopeChange/SCA-009_2026-08-20_0000','execution/_ScopeChange/SCA-010_2026-09-18_1400','execution/_DAG/DAG-010','execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS','core','apps','schemas','tests']
for name in protected:
 path=str((P/name).relative_to(ROOT))
 changed=subprocess.check_output(['git','diff','--name-only',BASE,'--',path],cwd=ROOT,text=True)
 if changed.strip():errors.append('HISTORY_OR_PRODUCT_CHANGED '+path)
result={'status':'FAIL'if errors else'PASS','reviewed_source_commit':BASE,'accepted_snapshot_bindings':len(manifest),'accepted_snapshot_manifest_sha256':sha(RUN/'ACCEPTED_MANIFEST.csv'),'final_canonical_targets':len(final['files']),'conditional_targets':71,'graph_semantic_bytes_unchanged':not any(x.startswith('GRAPH_BYTES')for x in errors),'current_sca':'SCA-011','current_dag':'DAG-011','historical_evidence_and_product_unchanged':not any(x.startswith('HISTORY_OR_PRODUCT')for x in errors),'errors':errors,'limit':'Identity/authority/preservation check; source currency and semantic audit use separate owning returns.'}
(POST/'CURRENT_VALIDATION.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2));raise SystemExit(bool(errors))
