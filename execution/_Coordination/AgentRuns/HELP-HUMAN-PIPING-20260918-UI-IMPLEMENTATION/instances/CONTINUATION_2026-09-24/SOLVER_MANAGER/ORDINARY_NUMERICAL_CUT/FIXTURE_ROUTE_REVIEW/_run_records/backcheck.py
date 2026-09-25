"""Read-only source/data review checks; never runs Git, TS, Rust, or builds."""
from pathlib import Path
import datetime
import difflib
import hashlib
import json
import subprocess
import sys

R = Path('/private/tmp/piping-numerical-corrections-20260924')
P = R / 'projects/chirality-piping'
B = P / 'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/ORDINARY_NUMERICAL_CUT'
E = B / 'FIXTURE_ROUTE_REVIEW/_run_records'
C = B / 'FIXTURE_CONSUMER'
G = B / 'FIXTURE_GENERATION/_run_records'
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
read = lambda p: json.loads(p.read_text())
checks = []
def check(name, result, details=None):
    checks.append({'check': name, 'passed': bool(result), 'details': details})

freeze = read(C / 'SOURCE_FREEZE.json')
expected_diff = ''
for item in freeze['files']:
    path = item['path']
    suffix = path.removeprefix('projects/chirality-piping/')
    before = C / '_run_records/before' / suffix
    after = C / '_run_records/after' / suffix
    check('consumer frozen before/after/current: ' + path,
          digest(before) == item['before_sha256'] and digest(after) == item['sha256']
          and digest(R / path) == item['sha256'] and (R / path).stat().st_size == item['bytes'])
    expected_diff += ''.join(difflib.unified_diff(before.read_text().splitlines(True), after.read_text().splitlines(True), fromfile='before/' + path, tofile='after/' + path))
check('complete consumer diff equals before/after reconstruction', expected_diff == (C / 'SOURCE.diff').read_text())

generator_origin = read(G / 'GENERATOR_BEFORE_ORIGIN.json')
generator = P / 'core/product_physics/examples/preview_result.rs'
check('generator exact supplied original/after/diff hashes', digest(G / 'preview_result.before.rs') == generator_origin['before_sha256'] and digest(generator) == generator_origin['after_sha256'] and digest(G / 'preview_result.diff') == generator_origin['diff_sha256'])
generator_delta = ''.join(difflib.unified_diff((G / 'preview_result.before.rs').read_text().splitlines(True), generator.read_text().splitlines(True), fromfile='a/projects/chirality-piping/core/product_physics/examples/preview_result.rs', tofile='b/projects/chirality-piping/core/product_physics/examples/preview_result.rs'))
check('generator supplied diff covers complete before/after', generator_delta == ''.join((G / 'preview_result.diff').read_text().splitlines(True)[2:]))

basis = read(P / 'fixtures/product_preview/precision_fixture_generation.json')
build = read(G / 'BUILD_INPUTS.json')
source_mismatches = [p for p,h in basis['source_input_files'].items() if digest(R / p) != h]
check('all generation source/input/lock hashes still match', not source_mismatches, {'file_count': len(basis['source_input_files']), 'mismatches': source_mismatches})
check('maintained source list equals executed input snapshot', basis['source_input_files'] == build['snapshot']['hashes'])
check('input snapshot identity', hashlib.sha256(json.dumps(build['snapshot'],sort_keys=True,separators=(',', ':')).encode()).hexdigest() == build['source_snapshot_json_sha256'])
for key,hash_key in [('input_model','input_model_sha256'),('historical_fixture_preserved','legacy_fixture_sha256')]:
    entry = basis[key]
    check(key + ' immutable actual bytes', digest(P / entry['path']) == entry['sha256'] == build[hash_key])

model = read(P / basis['input_model']['path'])
requested = [c['id'] for c in model['load_cases']]
semantic_rows = read(P / 'fixtures/results/semantic_contract_v0_3_precision_1.json')['rows']
generation = read(G / 'GENERATION.json')
check('exact two successful recorded mode executions', [x['requested_mode'] for x in generation] == ['sparse_interactive','dense_scrutiny'] and all(x['exit_code'] == 0 for x in generation))
inventory = []
for output, record in zip(basis['outputs'], generation):
    mode = output['mode']
    suffix = 'sparse' if mode == 'sparse_interactive' else 'dense'
    raw = P / output['path']
    stdout = G / (suffix + '.stdout.json')
    data = read(raw)
    check(mode + ' fixture is exact recorded producer stdout', raw.read_bytes() == stdout.read_bytes() and digest(raw) == output['sha256'] == record['stdout_sha256'])
    check(mode + ' exact requested command and successful executable stderr', record['command'][-2:] == ['--',mode] and '--locked' in record['command'] and '--offline' in record['command'] and 'preview_result ' + mode in (G / (suffix + '.stderr.log')).read_text())
    rows = [row for row in data['results'] if row['kind'] == 'linear_solver_mode_basis']
    wanted_value = 1 if suffix == 'sparse' else 2
    wanted_basis = suffix + '_structural_integrity_primary'
    mode_ok = len(rows) == len(requested) and {r['basis_ref']['ref_id'] for r in rows} == set(requested)
    for row in rows:
        md = row['metadata']
        fields = [s.strip() for s in md['basis'].split(';')]
        mode_ok &= row['value'] == wanted_value and row['unit'] == 'mode_code' and row['basis_ref']['ref_type'] == 'load_case' and md['location'] == row['basis_ref']['ref_id'] and md['component'] == 'linear_solver_mode' and md['coordinate_system'] == 'reduced_system'
        mode_ok &= [s for s in fields if s.startswith('solver_mode=')] == ['solver_mode=' + mode]
        mode_ok &= [s for s in fields if s.startswith('solution_basis=')] == ['solution_basis=' + wanted_basis]
    check(mode + ' actual per-case mode evidence', mode_ok and rows == record['mode_rows'])
    semantic_errors = []
    for row in data['results']:
        kinds = [s for s in semantic_rows if s['kind'] == row['kind']]
        units = [s for s in kinds if s['unit'] == row['unit']]
        component = row.get('metadata', {}).get('component')
        exact = next((s for s in units if s['component'] == component), None)
        generic = next((s for s in units if s['component'] is None), None)
        selected = exact or generic
        if kinds and (not units or (component is not None and not selected)):
            semantic_errors.append(row['id'])
        if selected and row.get('dimension') and selected.get('legacy_declared_dimension') and row['dimension'] != selected['legacy_declared_dimension']:
            semantic_errors.append(row['id'])
    check(mode + ' no source row unit/component/dimension contradictions', not semantic_errors, semantic_errors)
    emitted_ids = [r['id'] for r in data['results']] + [d['id'] for d in data['diagnostics']]
    cases = data['numerical_quality']['cases']
    quality_ok = len(cases) == len(requested) and all(sum(c['basis_ref'] == {'ref_type':'load_case','ref_id':cid} for c in cases) == 1 for cid in requested)
    quality_ok &= all(c['structural_status'] == 'passive_model_basis' and c['solve_quality'] == 'checks_passed' and c['model_matrix_fidelity'] == 'represented_equations_retained' and c['accuracy_evidence'] in ['not_claimed','reference_verified'] and c['evidence_refs'] and all(x in emitted_ids for x in c['evidence_refs']) for c in cases)
    check(mode + ' claimed checks_passed has complete local case/ref evidence', quality_ok and len(emitted_ids) == len(set(emitted_ids)) and data['numerical_quality']['status'] == 'checks_passed')
    inventory.append({'mode':mode,'sha256':digest(raw),'rows':len(data['results']),'mechanics':data['status']['mechanics'],'numerical_status':data['numerical_quality']['status'],'cases':requested,'mode_rows':len(rows)})

scope_paths = [i['path'] for i in freeze['files']] + ['projects/chirality-piping/core/product_physics/examples/preview_result.rs'] + ['projects/chirality-piping/' + o['path'] for o in basis['outputs']] + ['projects/chirality-piping/fixtures/product_preview/precision_fixture_generation.json','projects/chirality-piping/fixtures/product_preview/PRECISION_FIXTURES.md']
command = [sys.executable, str(R / 'tools/software_workflow/validate_change_scope.py'), str(R)]
for path in scope_paths:
    command += ['--allowed',path,'--path',path]
completed = subprocess.run(command, capture_output=True, text=True)
(E / 'scope.stdout.json').write_text(completed.stdout)
(E / 'scope.stderr.log').write_text(completed.stderr)
check('scope helper explicit-path mode; no Git discovery', completed.returncode == 0)
report = {'performed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'scope':'read-only frozen fixture route; Python data backcheck only; no application or solver execution','checks':checks,'inventory':inventory,'scope_command':command,'all_passed':all(x['passed'] for x in checks)}
(E / 'STATIC_BACKCHECK.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'all_passed':report['all_passed'],'checks':len(checks),'failed':[x for x in checks if not x['passed']],'inventory':inventory},indent=2))
