from __future__ import annotations
import hashlib, json, shutil, sys
from pathlib import Path
PROJECT=Path('/Users/ryan/.codex/worktrees/8728/chirality-compatibility-20260914/projects/chirality-piping')
sys.path[:0]=[str(PROJECT),str(PROJECT/'tests')]
from core.handoff.stress_neutral import validate_stress_neutral_export_package_v0_2,write_materialized_members_v0_2
from schema_validation import validate_instance
schema=json.loads((PROJECT/'schemas/stress_neutral_export.schema.json').read_text())
base=PROJECT/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE/instances/COMPATIBILITY/EXPLICIT_DIMENSION_REPAIR_V16/TASK_WRITER/_run_records/reuse_v15'
source=base/'output';materialized=base/'materialized';shutil.rmtree(materialized,ignore_errors=True)
reports=[]
for case in sorted(p.name for p in source.iterdir() if p.is_dir()):
 packet=json.loads((source/case/'stress_neutral_v0_2.json').read_text())
 validate_stress_neutral_export_package_v0_2(packet)
 validate_instance(schema,packet,schema_label='schemas/stress_neutral_export.schema.json',instance_label=case)
 paths=write_materialized_members_v0_2(packet,materialized/case)
 members=[]
 for path in paths:
  data=path.read_bytes();members.append({'filename':path.name,'sha256':hashlib.sha256(data).hexdigest(),'bytes':len(data)})
 assert len(members)==9
 reports.append({'case':case,'rows':len(packet['result_rows']),'witnesses':len(packet['unit_preservation_witnesses']),'members':members})
report={'schema':'compatibility-v16-v15-reuse-validation-v1','status':'PASS','cases':reports,'case_count':len(reports),'schema_runtime_write9_pass':len(reports)}
(base/'PYTHON_SCHEMA_WRITE9_RESULT.json').write_text(json.dumps(report,indent=2,sort_keys=True)+'\n')
print(json.dumps({'status':'PASS','cases':len(reports),'schema_runtime_write9':len(reports),'member_files':sum(len(x['members']) for x in reports)}))
