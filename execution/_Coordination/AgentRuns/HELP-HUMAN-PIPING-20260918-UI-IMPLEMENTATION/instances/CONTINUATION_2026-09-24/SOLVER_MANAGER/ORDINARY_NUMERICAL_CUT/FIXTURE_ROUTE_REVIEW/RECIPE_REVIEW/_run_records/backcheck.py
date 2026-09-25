"""Static recipe patch checks only. Does not execute proposed Node/Cargo code."""
from pathlib import Path
import datetime, difflib, hashlib, json, subprocess, sys

R = Path('/private/tmp/piping-numerical-corrections-20260924')
P = R / 'projects/chirality-piping'
B = P / 'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/ORDINARY_NUMERICAL_CUT'
F = B / 'FIXTURE_GENERATION/RECIPE_REPAIR'
E = B / 'FIXTURE_ROUTE_REVIEW/RECIPE_REVIEW/_run_records'
h = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
j = lambda p: json.loads(p.read_text())
freeze = j(F / 'PATCH_FREEZE.json')
package_before = F / '_run_records/package.before.json'
package_after = F / 'proposed/package.json'
script = F / 'proposed/tools/serialization/generate_product_preview_mechanics.mjs'
package_path = 'projects/chirality-piping/package.json'
script_path = 'projects/chirality-piping/tools/serialization/generate_product_preview_mechanics.mjs'
checks = {
    'exact frozen patch': h(F / 'PROPOSED.patch') == freeze['patch_sha256'],
    'exact original package': h(package_before) == h(R / package_path) == freeze['files'][0]['before_sha256'],
    'exact proposed package': h(package_after) == freeze['files'][0]['proposed_sha256'],
    'exact proposed generator': h(script) == freeze['files'][1]['proposed_sha256'],
    'maintained script still absent': not (R / script_path).exists(),
    'all maintained freeze files unchanged': all(h(R / x['path']) == x['sha256'] for x in freeze['maintained_files_unchanged_at_seal']),
}
delta = ''.join(difflib.unified_diff(package_before.read_text().splitlines(True), package_after.read_text().splitlines(True),fromfile='before/'+package_path,tofile='after/'+package_path))
delta += ''.join(difflib.unified_diff([],script.read_text().splitlines(True),fromfile='/dev/null',tofile='after/'+script_path))
checks['complete patch reconstructed from exact sources'] = delta == (F / 'PROPOSED.patch').read_text()
original, proposed = j(package_before), j(package_after)
original['scripts']['generate:product-preview-mechanics'] = proposed['scripts']['generate:product-preview-mechanics']
checks['package has only intended script-command edit'] = original == proposed
command = [sys.executable,str(R/'tools/software_workflow/validate_change_scope.py'),str(R)]
for path in [package_path,script_path]: command += ['--allowed',path,'--path',path]
scope = subprocess.run(command,capture_output=True,text=True)
(E/'scope.stdout.json').write_text(scope.stdout)
(E/'scope.stderr.log').write_text(scope.stderr)
checks['explicit proposed-path scope validation without Git'] = scope.returncode == 0
prior = j(B/'FIXTURE_ROUTE_REVIEW/_run_records/ORIGINS.json')
checks['prior read instruction origins unchanged'] = all(h(R/x['path']) == x['sha256'] for x in prior['instructions_fully_read'])
report = {'time_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'checks':checks,'all_static_identity_checks_passed':all(checks.values()),'scope_command':command,'qualification':'Identity/applicability checks only. Semantic review has two unresolved write-safety findings. No Node/Cargo/generator/test execution.'}
(E/'STATIC_PATCH_CHECKS.json').write_text(json.dumps(report,indent=2)+'\n')
sources = ['core/product_physics/Cargo.toml','core/units/Cargo.toml','core/product_physics/src/lib.rs','core/units/src/lib.rs','core/product_physics/examples/preview_result.rs']
origins = {'agent':'/root/solver_manager/fixture_route_review','parent':'/root/solver_manager','role':'TASK Type 2; independent patch reviewer, no implementation contribution','mechanism':'delegated-harness-native followup; no descendants','instruction_origins':prior['instructions_fully_read'],'brief':'Review only unapplied two-file recipe patch for preservation of genuine both-mode outputs, bounded paths/writes, source evidence, staging and rollback. App diagnosis excluded.','patch_origins':[{'path':str(p.relative_to(R)),'sha256':h(p)} for p in sorted(F.rglob('*')) if p.is_file()],'additional_source_slices':[{'path':str((P/x).relative_to(R)),'sha256':h(P/x)} for x in sources],'boundary':'No Git, Node, Cargo, recipe execution, builds, tests, native, source edits or dependency/configuration changes. Own evidence writes only.'}
(E/'ORIGINS.json').write_text(json.dumps(origins,indent=2)+'\n')
print(json.dumps(report,indent=2))
