"""One bounded byte-identical records relocation from the recorded Git basis."""
from pathlib import Path
import hashlib,json,subprocess
R=Path.cwd();P=R/'projects/chirality-piping';E=P/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/VALIDATION_FOUNDATION';O=E/'PORTABILITY_REPAIR'
BASE='c278f64ba122eb8b848b9f14e58a0e533e94439a'
assert subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip()==BASE
moves=[('FIRST_STATIC_BINDINGS','_run_records/FIRST_STATIC_BINDINGS','Frozen selected reference/criterion/review bundle; complete internal manifest and paths preserved'),('ORDINARY_PHYSICS_ADAPTER','_run_records/ORDINARY_PHYSICS_ADAPTER','Completed source/test/build/review/actual-comparison evidence bundle; original relative sibling-package links preserved'),('HARNESS_INVENTORY','_run_records/HARNESS_INVENTORY','Completed TASK source-inventory return and origins'),('INDEPENDENT_REVIEW','_run_records/INDEPENDENT_REVIEW','Completed initial mapping review, probes and actual execution origin'),('STATIC_REFERENCE_BASIS/INDEPENDENT_REVIEW','STATIC_REFERENCE_BASIS/_run_records/INDEPENDENT_REVIEW','Completed external-source applicability review and private acquisition origin hashes'),('SOURCE_FREEZE_01.json','_run_records/SOURCE_FREEZE_01.json','Historical mapping source/check execution capture'),('SOURCE_FREEZE_02.json','_run_records/SOURCE_FREEZE_02.json','Historical mapping repair source/check execution capture'),('CHECKPOINT_SELECTION.json','_run_records/CHECKPOINT_SELECTION.json','Exact historical checkpoint selection; original paths remain Git-bound'),('FIRST_STATIC_CHECKPOINT_SELECTION.json','_run_records/FIRST_STATIC_CHECKPOINT_SELECTION.json','Exact historical original-reference checkpoint selection'),('ADAPTER_CHECKPOINT_SELECTION.json','_run_records/ADAPTER_CHECKPOINT_SELECTION.json','Exact historical 26b adapter checkpoint selection'),('RETURN.md','_run_records/FOUNDATION_RETURN_BEFORE_PORTABILITY.md','Preserved foundation handoff before new current portability index'),('CURRENT_COORDINATION.md','_run_records/COORDINATION_BEFORE_PORTABILITY.md','Preserved dated coordination facts before new current index')]
records=[];protected=[]
for old,new,reason in moves:
 src=E/old;dst=E/new
 assert src.exists() and not dst.exists(),(old,new)
 paths=sorted(p for p in src.rglob('*') if p.is_file()) if src.is_dir() else [src]
 for path in paths:
  relative=str(path.relative_to(R));data=path.read_bytes();original=subprocess.check_output(['git','show',BASE+':'+relative])
  assert data==original,relative
  target=dst/path.relative_to(src) if src.is_dir() else dst
  records.append({'old_path':relative,'new_path':str(target.relative_to(R)),'sha256':hashlib.sha256(data).hexdigest(),'byte_length':len(data),'original_git_commit':BASE,'original_git_blob':subprocess.check_output(['git','rev-parse',BASE+':'+relative],text=True).strip(),'classification':'preserved historical execution/reference/custody evidence','reason':reason})
 dst.parent.mkdir(parents=True,exist_ok=True);src.rename(dst)
for row in records:
 assert hashlib.sha256((R/row['new_path']).read_bytes()).hexdigest()==row['sha256']
map_record={'format':'openpipestress.validation_evidence_relocation/1','original_git_commit':BASE,'scope':str(E.relative_to(R)),'kind':'byte-identical historical evidence relocation; no numerical, runtime, instruction or qualification change','bundles':[{'old':old,'new':new,'reason':reason} for old,new,reason in moves],'files':records,'path_interpretation':'Old addresses and any internal original-address fields remain factual at original_git_commit. Whole reference and adapter bundles preserve their internal relative relationships. Resolve explicit old repo-relative origin addresses through this map or git show c278:path; do not rewrite git_blob sources or treat archived commands as new instructions.'}
(O/'RELOCATION_MAP.json').write_text(json.dumps(map_record,indent=2)+'\n')
print(json.dumps({'moved_files':len(records),'bundles':len(moves),'bytes':sum(row['byte_length'] for row in records),'map_sha256':hashlib.sha256((O/'RELOCATION_MAP.json').read_bytes()).hexdigest()},indent=2))
