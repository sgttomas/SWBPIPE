"""Read-only bounded review audit; writes no files and runs no product/build code.
Run from repository root: python3 <repository-relative path to this file>.
"""
from pathlib import Path
from decimal import Decimal as D, localcontext
import hashlib
import json
import re
import subprocess
import sys

sys.dont_write_bytecode = True
root = Path(subprocess.check_output(['git', 'rev-parse', '--show-toplevel'], text=True).strip())
here = Path(__file__).resolve().parent
solver = here.parent
continuation = solver.parent
sys.path.insert(0, str(root / 'tools/practitioner_harness'))
from surface_roles import classify_surface, iter_machine_path_lines, SurfaceRole, MACHINE_ABS_PATH_RE

def sha(data):
    return hashlib.sha256(data).hexdigest()

def read(path):
    return json.loads(path.read_text())

def gitbytes(commit, path):
    return subprocess.check_output(['git', '-C', str(root), 'show', f'{commit}:{path}'])

report = {}
head = subprocess.check_output(['git', '-C', str(root), 'rev-parse', 'HEAD'], text=True).strip()
assert head == 'beaf4869bee2ae8c866e7e62eaa6a8f369610d7e'
after = read(solver / 'M24_INTEGRATION/RUN_AFTER.json')
before = read(solver / 'M24_INTEGRATION/RUN_BEFORE.json')
for p, h in after['source_sha256'].items():
    assert sha((root / p).read_bytes()) == h, p
changed = [p for p in before['source_sha256'] if before['source_sha256'][p] != after['source_sha256'][p]]
assert changed == after['changed_during_execution']
assert changed == ['projects/chirality-piping/validation/benchmarks/physics_audit_regression/Cargo.lock']
log = solver / 'M24_INTEGRATION' / after['log_ref']
assert sha(log.read_bytes()) == after['log_sha256']
assert len(re.findall(r'^test .* \.\.\. ok$', log.read_text(), re.M)) == 5
assert '5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out' in log.read_text()
assert after['exit_code'] == 0
authoring = read(continuation / 'AUTHORING_MANAGER/EXPANDED_CANDIDATE.json')
for entry in authoring['files']:
    p, h = entry['path'], entry['sha256']
    assert sha((root / p).read_bytes()) == h
    assert sha(gitbytes('ff2cb76ddfda43f6bcb4202b80b7fd199eb3c44e', p)) == h
for p in ['projects/chirality-piping/core/product_physics/src/lib.rs',
          'projects/chirality-piping/core/product_physics/src/validation.rs']:
    assert (root / p).read_bytes() == gitbytes('c90089c3279ee1213501835eaef08938ed800897', p)
report['candidate'] = {'head': head, 'authoring_source_hashes_matched': 9,
    'solver_source_hashes_matched': 2, 'execution_sources_matched': 7,
    'manager_test_rows': 5, 'manager_log_hash_match': True,
    'reviewer_product_execution': False}

lock_path = root / changed[0]
oldlock = (solver / 'M24_INTEGRATION/_run_records/Cargo.lock.before').read_bytes()
assert sha(oldlock) == before['source_sha256'][changed[0]]
assert oldlock == gitbytes(head, changed[0])
def lock_packages(text):
    packages = {}
    for block in text.split('[[package]]')[1:]:
        name = re.search(r'^name = "([^"]+)"$', block, re.M).group(1)
        dependencies = re.search(r'^dependencies = \[\n(.*?)\n\]', block, re.M | re.S)
        packages[name] = {'bytes': block.strip(), 'dependencies': re.findall(r'"([^"]+)"', dependencies.group(1)) if dependencies else []}
    return packages
old = lock_packages(oldlock.decode())
new = lock_packages(lock_path.read_text())
for name, p in old.items():
    expected = dict(p)
    if name == 'open_pipe_stress_physics_audit_regression':
        expected['dependencies'] = sorted(p['dependencies'] + ['open_pipe_stress_operation_applier'])
        expected['bytes'] = p['bytes'].replace('dependencies = [\n', 'dependencies = [\n "open_pipe_stress_operation_applier",\n')
    assert new[name] == expected, name
added = set(new) - set(old)
reachable = set()
def walk(name):
    if name in reachable:
        return
    reachable.add(name)
    for dep in new[name].get('dependencies', []):
        walk(dep.split()[0])
walk('open_pipe_stress_operation_applier')
assert added <= reachable
report['lock'] = {'old_packages_unchanged_except_declared_edge': True,
    'added_packages': sorted(added), 'all_added_reachable_from_dev_dependency': True,
    'new_package_count': len(added)}

manifest_path = solver / 'PORTABILITY/CUSTODY.json'
manifest = read(manifest_path)
assert sha(manifest_path.read_bytes()) == 'caad74e0b1f9eb0958210a61ef1965042b7aa073be6f8fb837116381c8e038e0'
tokens = re.compile(r'\{[A-Z][A-Z0-9_]*\}')
records = []
for entry in manifest['records']:
    current = root / entry['source_path']
    original = root / entry['original_raw']['path']
    raw = original.read_bytes()
    assert sha(raw) == entry['original_raw']['sha256']
    assert sha(current.read_bytes()) == entry['view_sha256']
    assert classify_surface(original.relative_to(root).as_posix()).role == SurfaceRole.EVIDENCE
    if entry.get('source_commit'):
        assert raw == gitbytes(entry['source_commit'], entry['source_path'])
    substitutions = {}
    used_locations = {}
    def compare_string(a, b, location, bases):
        if a == b:
            return
        names = tokens.findall(b)
        assert names and all(n in bases for n in names), (location, 'undefined token')
        pieces = tokens.split(b)
        expression = ''.join(re.escape(piece) + (r'(.+?)' if i < len(names) else '') for i, piece in enumerate(pieces))
        match = re.fullmatch(expression, a, re.S)
        assert match, (location, 'non-path semantic edit')
        for name, value in zip(names, match.groups()):
            assert MACHINE_ABS_PATH_RE.fullmatch(value), (location, 'replacement is not a machine path')
            assert substitutions.setdefault(name, value) == value, (location, 'inconsistent token')
            used_locations.setdefault(name, set()).add(location)
    def compare(a, b, location, bases):
        assert type(a) is type(b), (location, 'type changed')
        if isinstance(a, dict):
            assert len(a) == len(b), (location, 'field count changed')
            for (ak, av), (bk, bv) in zip(a.items(), b.items()):
                escaped = ak.replace('~', '~0').replace('/', '~1')
                pointer = location + '/' + escaped
                compare_string(ak, bk, 'key:' + pointer, bases)
                compare(av, bv, pointer, bases)
        elif isinstance(a, list):
            assert len(a) == len(b), (location, 'list count changed')
            for i, (av, bv) in enumerate(zip(a, b)):
                compare(av, bv, location + '/' + str(i), bases)
        elif isinstance(a, str):
            compare_string(a, b, location, bases)
        else:
            assert a == b, (location, 'value changed')
    if current.suffix == '.json':
        a, b = json.loads(raw), read(current)
        metadata = b.pop('_portability')
        assert metadata['raw_record'] == entry['original_raw']
        assert metadata.get('source_commit') == entry.get('source_commit')
        compare(a, b, '', metadata['path_bases'])
        assert set(used_locations) == set(metadata['path_bases'])
        for name, locations in used_locations.items():
            assert locations == set(metadata['path_bases'][name]['raw_locations']), (current.name, name)
    else:
        b = current.read_text().split('\n## Portable view custody\n')[0]
        a = raw.decode().rstrip()
        b = b.rstrip()
        compare_string(a, b, 'whole-document', entry['path_bases'])
    records.append({'path': entry['source_path'], 'archive_hash_match': True,
        'git_original_match': bool(entry.get('source_commit')), 'semantic_preservation': True,
        'tokens': sorted(substitutions)})
assert len(records) == 24

archival = []
for folder in ['SOLVER_MANAGER', 'AUTHORING_MANAGER']:
    tracked = subprocess.check_output(['git', '-C', str(root), 'ls-tree', '-r', '--name-only', head,
        '--', (continuation / folder).relative_to(root).as_posix()], text=True).splitlines()
    for p in tracked:
        path = root / p
        if path.suffix in ['.log', '.patch'] or path.name in ['RETURN.md', 'REVIEW.md']:
            assert path.read_bytes() == gitbytes(head, p), p
            archival.append(p)
actionable, archived_hits = [], []
scanned = 0
for folder in ['SOLVER_MANAGER', 'AUTHORING_MANAGER']:
    for path in (continuation / folder).rglob('*'):
        if not path.is_file() or path.suffix not in ['.md', '.json', '.yaml', '.yml']:
            continue
        scanned += 1
        rel = path.relative_to(root).as_posix()
        cls = classify_surface(rel)
        lines = list(iter_machine_path_lines(path.read_text()))
        if lines:
            item = {'path': rel, 'role': cls.role.value, 'lines': lines}
            if cls.active and cls.role != SurfaceRole.EVIDENCE:
                actionable.append(item)
            else:
                archived_hits.append(item)
assert not actionable, actionable
report['custody'] = {'manifest_sha256': sha(manifest_path.read_bytes()), 'records': records,
    'all_24_archives_match': True, 'all_23_committed_originals_match': True,
    'all_23_json_views_preserve_semantics': True, 'markdown_body_preserves_semantics': True,
    'unchanged_historical_logs_patches_returns_reviews': len(archival),
    'scanned_records': scanned, 'active_machine_path_hits': actionable,
    'recognized_evidence_records_with_machine_paths': len(archived_hits)}

prep = solver / 'NUMERICAL_REFERENCE_PREP'
preparation = read(prep / 'PREPARATION.json')
for p, h in preparation['artifacts'].items():
    assert sha((prep / p).read_bytes()) == h
original_preparation = read(root / preparation['_portability']['raw_record']['path'])
reference = Path(original_preparation['source_reference'])
assert sha(reference.read_bytes()) == preparation['source_sha256']
generated = subprocess.check_output([sys.executable, str(prep / 'independent_oracle.py')])
assert generated == (prep / 'prepared_expectations.json').read_bytes()
expected = read(prep / 'prepared_expectations.json')
with localcontext() as ctx:
    ctx.prec = 85
    pi = D('3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825')
    area = D('.0019') * pi
    inertia = D('.0000085975') * pi
    ei = D(1719500) * pi
    gj = D(1375600) * pi
    a = gj / 2
    s = (a / (a + D('.0001'))).sqrt()
    values = {'area_m2': area, 'inertia_m4': inertia, 'polar_m4': 2 * inertia,
        'N01_tip_y_m': D(8000) / (D(5158500) * pi), 'N01_tip_rz_rad': D(2000) / ei,
        'N01_root_fy_N': D(-1000), 'N01_root_mz_Nm': D(-2000),
        'N05_root_rx_rad': D('.0001'), 'N05_tip_rx_rad': D('.0001') + D('1e-8') / a,
        'N05_spring_reaction_Nm': D('-1e-8'), 'N05_scaled_rcond_2_exact': (1-s)/(1+s),
        'N08_tip_rx_rad': D(2)/gj, 'N08_root_mx_Nm': D(-1),
        'N09_tip_y_m': D(100000)/(D(5158500)*pi), 'N09_tip_rz_rad': D(5000)/ei,
        'N09_torque_tip_rx_rad': 1/gj, 'R01_original_free_residual_N': D(10),
        'R01_eta_c': D(1)/201, 'R03_original_free_residual_N': D(-1000), 'R03_eta_c': D(1),
        'R04_middle_ux_m': D('.00005'), 'R04_root_fx_N': D(19000)*pi}
    assert set(values) == set(expected['expected_decimal'])
    errors = {key: abs(D(expected['expected_decimal'][key])-value)/max(abs(value),D('1e-80'))
        for key,value in values.items()}
    assert all(error < D('1e-57') for error in errors.values())
report['preparation'] = {'artifacts_match': True, 'regenerated_bytes_match': True,
    'source_reference_hash_matches': True, 'independent_scalar_count': len(values),
    'maximum_relative_decimal_difference': str(max(errors.values())),
    'classification_review': 'N02 null torsion, N03 intersection, N04 disconnected rigid modes, N07 +/- eigenvalues checked analytically',
    'policy_activation': False}
report['thermal_reference'] = {'delta_celsius_or_kelvin': 100, 'delta_fahrenheit': 180,
    'alpha_per_kelvin': '0.000012', 'length_m': 6, 'strain': '0.0012',
    'free_tip_displacement_mm': '7.2', 'anchor_force_N': 0,
    'modulus_scale_sensitivity': 'Free axial extension is independent of E; catalog arithmetic covered separately.'}
print(json.dumps(report, indent=2))
