"""Static identity checks for the unapplied R1/R2 repair; no recipe execution."""
from pathlib import Path
import datetime, difflib, hashlib, json, subprocess, sys

R = Path('/private/tmp/piping-numerical-corrections-20260924')
P = R/'projects/chirality-piping'
B = P/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/ORDINARY_NUMERICAL_CUT'
F = B/'FIXTURE_GENERATION/RECIPE_REPAIR'
N = F/'REPAIR_01'
E = B/'FIXTURE_ROUTE_REVIEW/RECIPE_REVIEW/REPAIR_01_BACKCHECK/_run_records'
h = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
j = lambda p: json.loads(p.read_text())
freeze, old_freeze = j(N/'PATCH_FREEZE.json'), j(F/'PATCH_FREEZE.json')
package = 'projects/chirality-piping/package.json'
script = 'projects/chirality-piping/tools/serialization/generate_product_preview_mechanics.mjs'
before_package = F/'_run_records/package.before.json'
after_package = N/'proposed/package.json'
old_script = F/'proposed/tools/serialization/generate_product_preview_mechanics.mjs'
new_script = N/'proposed/tools/serialization/generate_product_preview_mechanics.mjs'
diff = lambda a,b,old,new: ''.join(difflib.unified_diff(a,b,fromfile=old,tofile=new))
complete = diff(before_package.read_text().splitlines(True),after_package.read_text().splitlines(True),'before/'+package,'after/'+package) + diff([],new_script.read_text().splitlines(True),'/dev/null','after/'+script)
repair = diff(old_script.read_text().splitlines(True),new_script.read_text().splitlines(True),'before/'+script,'after/'+script)
checks = {
    'revised patch exact freeze': h(N/'PROPOSED.patch') == freeze['patch_sha256'],
    'complete revised patch reconstructed': complete == (N/'PROPOSED.patch').read_text(),
    'repair-only delta reconstructed': repair == (N/'REPAIR.diff').read_text(),
    'original proposed script preserved': h(old_script) == old_freeze['files'][1]['proposed_sha256'] == freeze['files'][1]['original_proposal_sha256'],
    'original proposed patch preserved': h(F/'PROPOSED.patch') == old_freeze['patch_sha256'],
    'revised proposed script exact freeze': h(new_script) == freeze['files'][1]['proposed_sha256'],
    'package proposal unchanged and frozen': h(after_package) == h(F/'proposed/package.json') == freeze['files'][0]['proposed_sha256'],
    'maintained package equals original preimage': h(R/package) == h(before_package) == freeze['files'][0]['before_sha256'],
    'maintained recipe still absent': not (R/script).exists(),
    'original maintained freeze byte identities unchanged': all(h(R/x['path']) == x['sha256'] for x in old_freeze['maintained_files_unchanged_at_seal']),
}
def validator(p):
    return p.read_text().split('function validate(result, mode, value, model) {',1)[1].split('\n\n//',1)[0]
checks['raw output validator unchanged by safety repair'] = validator(old_script) == validator(new_script)
prior = j(B/'FIXTURE_ROUTE_REVIEW/_run_records/ORIGINS.json')
checks['prior instruction origins unchanged'] = all(h(R/x['path']) == x['sha256'] for x in prior['instructions_fully_read'])
command = [sys.executable,str(R/'tools/software_workflow/validate_change_scope.py'),str(R)]
for p in [package,script]: command += ['--allowed',p,'--path',p]
scope = subprocess.run(command,capture_output=True,text=True)
(E/'scope.stdout.json').write_text(scope.stdout)
(E/'scope.stderr.log').write_text(scope.stderr)
checks['explicit-path proposed scope validation without Git'] = scope.returncode == 0
report = {'time_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'checks':checks,'all_passed':all(checks.values()),'scope_command':command,'limits':'Static identity/applicability checks only. No Node/Cargo/generator/build/test/Git execution; maintained files not applied by reviewer. No fixture freshness, numerical qualification or runtime outcome asserted.'}
(E/'STATIC_BACKCHECK.json').write_text(json.dumps(report,indent=2)+'\n')
origins = {'agent':'/root/solver_manager/fixture_route_review','parent':'/root/solver_manager','role':'TASK Type 2; same independent reviewer backcheck','mechanism':'delegated-harness-native followup, no descendants','instruction_origins':prior['instructions_fully_read'],'supplied_basis':'Original R1/R2 review plus full RECIPE_REPAIR/REPAIR_01 proposal; App diagnosis excluded','review_inputs':[{'path':str(p.relative_to(R)),'sha256':h(p)} for p in [B/'FIXTURE_ROUTE_REVIEW/RECIPE_REVIEW/RETURN.md',F/'PATCH_FREEZE.json',F/'PROPOSED.patch',old_script,before_package] + [p for p in sorted(N.rglob('*')) if p.is_file()]],'limits':report['limits']}
(E/'ORIGINS.json').write_text(json.dumps(origins,indent=2)+'\n')
print(json.dumps({'all_passed':report['all_passed'],'count':len(checks),'failures':[name for name,ok in checks.items() if not ok],'patch_sha256':h(N/'PROPOSED.patch'),'proposed_script_sha256':h(new_script)},indent=2))
