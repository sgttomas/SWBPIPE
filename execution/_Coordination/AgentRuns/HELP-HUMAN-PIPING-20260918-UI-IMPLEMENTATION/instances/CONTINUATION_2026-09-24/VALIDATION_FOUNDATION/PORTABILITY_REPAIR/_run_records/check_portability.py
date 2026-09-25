"""Scoped records/path/hash/reconstruction checks; no solver or broad tests."""
from pathlib import Path
import hashlib,json,re,sys,subprocess
R=Path.cwd();P=R/'projects/chirality-piping';E=P/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/VALIDATION_FOUNDATION';O=E/'PORTABILITY_REPAIR';A=E/'_run_records/ORDINARY_PHYSICS_ADAPTER';F=E/'_run_records/FIRST_STATIC_BINDINGS'
sys.path[:0]=[str(R/'tools/practitioner_harness'),str(P)]
from surface_roles import load_project_policy,effective_role,iter_machine_path_lines,SurfaceRole
from tools.validation.build_first_static_selection import build
BASE='c278f64ba122eb8b848b9f14e58a0e533e94439a'
def sha(data):return hashlib.sha256(data).hexdigest()
def write(name,value):(O/'_run_records'/name).write_text(json.dumps(value,indent=2)+'\n')
map_data=json.loads((O/'RELOCATION_MAP.json').read_text());mapping={row['old_path']:row for row in map_data['files']}
for row in mapping.values():assert sha((R/row['new_path']).read_bytes())==row['sha256']
original_findings=json.loads(Path('/private/tmp/piping-joined-qualification-20260925/portability-findings.json').read_text())
owned=[row for row in original_findings if '/VALIDATION_FOUNDATION/' in row['path']]
assert len(owned)==32 and all(row['path'] in mapping for row in owned)
policy=load_project_policy(R,P);assert not policy.issues
violations=[];classes={};evidence_paths=0
for path in E.rglob('*'):
 if path.is_file() and path.suffix in ('.md','.json','.yaml','.yml'):
  hits=list(iter_machine_path_lines(path.read_text()))
  if not hits:continue
  role=effective_role(str(path.relative_to(R)),policy)
  if role.active and role.role is not SurfaceRole.EVIDENCE:violations.append({'path':str(path.relative_to(R)),'lines':hits,'role':role.role.value})
  else:evidence_paths+=1
for old in owned:
 row=mapping[old['path']];role=effective_role(row['new_path'],policy)
 assert role.role is SurfaceRole.EVIDENCE,(row['new_path'],role)
 classes[old['path']]={'new_path':row['new_path'],'role':role.role.value,'sha256':row['sha256']}
assert not violations,violations
packet=json.loads((F/'PACKET_MANIFEST.json').read_text())
assert len(packet['files'])==25
for row in packet['files']:assert sha((F/row['path']).read_bytes())==row['sha256']
# Confirm unchanged runtime/tests/fixtures from the combined source basis.
source=json.loads((A/'SOURCE_FREEZE_03.json').read_text())
for row in source['files']:
 assert (R/row['path']).read_bytes()==subprocess.check_output(['git','show',BASE+':'+row['path']])
# Derive metadata against the relocated exact package, never execute the runner.
b=json.loads((A/'HEADLESS_BUILD/BUILD_RESULT.json').read_text());derived=[]
for mode in ('sparse_interactive','dense_scrutiny'):
 out=O/'_run_records/DERIVATION_CHECK'/mode
 path=build(F,out,b['candidate_commit'],b['executable']['sha256'],A/'REVIEWED_READER_BINDING.json',mode)
 selection=json.loads(path.read_text());derivation=json.loads((out/'DERIVATION.json').read_text())
 assert len(selection['cases'])==2 and sum(len(case['assertions']) for case in selection['cases'])==146
 assert derivation['actual_runs_performed']==0 and derivation['structural_obligations']==20 and derivation['section_subchecks']==18
 for case in selection['cases']:
  for field in ('input','reference','criterion'):
   bound=case[field];assert sha((out/bound['path']).read_bytes())==bound['sha256']
 derived.append({'mode':mode,'selection':str(path.relative_to(R)),'sha256':sha(path.read_bytes()),'scalar_obligations':146,'structural_obligations':20,'nested_section_subchecks':18,'actual_runs':0})
# Original archived execution selections still resolve after sibling-bundle move.
for mode in ('sparse_interactive','dense_scrutiny'):
 base=A/'EXECUTION_SELECTIONS'/mode;selected=json.loads((base/'selection.json').read_text())
 for field in ('physics_binding','reference_basis'):
  row=selected[field];assert sha((base/row['path']).read_bytes())==row['sha256']
 for case in selected['cases']:
  for field in ('input','reference','criterion'):
   row=case[field];assert sha((base/row['path']).read_bytes())==row['sha256']
  for row in case['structural'].values():assert sha((base/row['path']).read_bytes())==row['sha256']
programme=A/'COMPARISON_RUN_01/PROGRAMME.json'
assert sha(programme.read_bytes())=='834ecc0ce0405882b6c723ce720dd3d044d8ea1c20e45bea662acb88888b6e2e'
counts=json.loads(programme.read_text());assert counts['all_selected_obligations_matched'] and counts['actual_case_mode_executions']==4
record={'original_findings':len(owned),'resolved_by_structural_evidence_relocation':classes,'unresolved_owned_path_findings':violations,'machine_path_evidence_artifacts':evidence_paths,'byte_identical_relocated_files':len(mapping),'original_25_packet_hashes_match':True,'unchanged_maintained_source_paths':len(source['files']),'old_execution_selection_relative_bindings_resolve':True,'metadata_derivations':derived,'original_programme_sha256':sha(programme.read_bytes()),'scope':'Records/path/hash and metadata-only package reconstruction checks. No original capture rewritten, protected target or runtime changed, solver, build, native, broad test or qualification rerun.'}
write('CHECKS.json',record)
print(json.dumps({'relocations':len(mapping),'original_findings_resolved':len(owned),'remaining_owned_path_findings':len(violations),'original_packet_files':25,'metadata_derivations':len(derived),'actual_solver_runs':0},indent=2))
