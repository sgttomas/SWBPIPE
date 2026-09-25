"""Read-only source/evidence and independent arithmetic checks; no product execution."""
from pathlib import Path
from fractions import Fraction
import hashlib
import json
import math
import re
import struct
import subprocess
import sys

ROOT = Path('/private/tmp/piping-engine-integration-20260925')
HERE = Path(__file__).resolve().parent
JOIN = HERE.parent.parent
HIST = Path('/private/tmp/piping-pressure-stress-20260924')
PREFIX = 'projects/chirality-piping/'
HP = HIST / str(JOIN.relative_to(ROOT)).replace('ENGINE_INTEGRATION', 'PHYSICS_MANAGER')
checks = []
def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()
def check(name, value, **details):
    checks.append(dict(name=name, passed=bool(value), **details))
    assert value, (name, details)
def git_bytes(revision, path):
    return subprocess.run(['git', '-C', str(ROOT), 'show', revision + ':' + path], capture_output=True, check=True).stdout

manifest = json.loads((HERE / 'MANIFEST.json').read_text())
patch = (HERE / 'CANDIDATE.patch').read_text()
sections = re.split(r'(?=^diff --git )', patch, flags=re.M)[1:]
paths = []
for section in sections:
    lines = section.splitlines(keepends=True)
    path = re.match(r'diff --git a/(.*?) b/(.*?)\n', lines[0]).group(2)
    paths.append(path)
    before = [] if 'new file mode 100644\n' in lines else git_bytes(manifest['base'], path).decode().splitlines(keepends=True)
    out, cursor, i = [], 0, 0
    while i < len(lines):
        match = re.match(r'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@', lines[i])
        if not match:
            i += 1
            continue
        start = max(0, int(match[1]) - 1)
        out.extend(before[cursor:start]); cursor = start; i += 1
        consumed = emitted = 0
        while i < len(lines) and not lines[i].startswith('@@ '):
            line = lines[i]
            if line.startswith((' ', '-')):
                check('patch old/context: ' + path, cursor < len(before) and before[cursor] == line[1:])
                cursor += 1; consumed += 1
            if line.startswith((' ', '+')):
                out.append(line[1:]); emitted += 1
            i += 1
        check('hunk counts: ' + path, consumed == int(match[2] or 1) and emitted == int(match[4] or 1))
    out.extend(before[cursor:])
    check('patch reconstructs frozen file: ' + path, ''.join(out).encode() == (ROOT / path).read_bytes())
check('exact patch/manifest scope', paths == [f['path'] for f in manifest['files']])
check('patch hash', sha(HERE / 'CANDIDATE.patch') == manifest['patch_sha256'])
command = [sys.executable, str(ROOT / 'tools/software_workflow/validate_change_scope.py'), str(ROOT)]
for path in paths:
    command += ['--allowed', path, '--path', path]
scope = subprocess.run(command, capture_output=True, text=True, check=True)
(HERE / 'scope_check.json').write_text(scope.stdout)

relocation = json.loads((JOIN / 'PORTABLE_REFERENCE_JOIN.json').read_text())
portable = []
for fixture in relocation['fixtures']:
    path = ROOT / PREFIX / 'core/product_physics/tests/fixtures/pressure_reference' / fixture['fixture']
    original = git_bytes(relocation['source_checkpoint'], fixture['preserved_origin'])
    check('portable unchanged: ' + fixture['fixture'], path.read_bytes() == original and sha(path) == fixture['sha256'])
    portable.append(dict(path=str(path.relative_to(ROOT)), sha256=sha(path), checkpoint=relocation['source_checkpoint']))

projections = 0
for path in sorted((ROOT / PREFIX / 'core/product_physics/tests/fixtures/pressure_reference').glob('*.json')):
    def visit(value):
        global projections
        if isinstance(value, dict):
            if 'exact_rational_with_bounded_pi' in value:
                try:
                    projected = float(Fraction(value['exact_rational_with_bounded_pi']))
                except OverflowError:
                    projected = None
                check('independent rational projection: ' + path.name, projected == value['f64'])
                projections += 1
            for child in value.values():
                visit(child)
        elif isinstance(value, list):
            for child in value:
                visit(child)
    visit(json.loads(path.read_text()))

# Analytical annulus and cantilever quantities, independently evaluated from
# the full UI source fixture. No production helper is imported or executed.
capture = json.loads((JOIN / 'UI_PRODUCER_CAPTURE.json').read_text())
ui = []
for run in capture['runs']:
    check('UI model binding', sha(ROOT / run['model_path']) == run['model_sha256'])
    check('UI output binding', sha(ROOT / run['result_path']) == run['result_sha256'])
    for path, digest in run['source'].items():
        check('UI captured source binding: ' + path, sha(ROOT / path) == digest)
    model = json.loads((ROOT / run['model_path']).read_text())
    output = json.loads((ROOT / run['result_path']).read_text())
    check('raw0.2/producer0.2/physics-1', output['schema_version'] == '0.2.0' and output['producer']['component_version'] == '0.2.0' and output['producer']['semantic_contract_id'].endswith('/physics-1'))
    check('actual NUM case quality', output['numerical_quality']['publication_quantization'] == 'none' and all(c['solve_quality'] == 'checks_passed' and c['accuracy_evidence'] == 'not_claimed' for c in output['numerical_quality']['cases']))
    rows = output['results']
    check('finite unique UI rows', len({r['id'] for r in rows}) == len(rows) and all(math.isfinite(r['value']) for r in rows))
    def row(case, kind, entity, component=None, location=None):
        found = [r for r in rows if r['basis_ref']['ref_id'] == case and r['kind'] == kind and r['entity_ref'] == entity and (component is None or r['metadata']['component'] == component) and (location is None or r['metadata']['location'] == location)]
        assert len(found) == 1, found
        return found[0]['value']
    def close(label, actual, expected, zero_scale=0):
        check(label, abs(actual-expected) <= 1e-9*(abs(expected) if expected else zero_scale), actual=actual, expected=expected)
    section = model['pipe_segments'][0]['section']
    od = section['outside_diameter']['value']; wall = section['wall_thickness']['value']
    # Fixture authored in metres/Pa with no wall deduction or material override.
    assert section['outside_diameter']['unit'] == section['wall_thickness']['unit'] == 'm'
    ro, ri = od/2, od/2-wall
    area = math.pi*(ro*ro-ri*ri); bore = math.pi*ri*ri
    inertia = math.pi*(ro**4-ri**4)/4; z = inertia/ro
    material = model['materials'][0]
    e = material['elastic_modulus']['value']; nu = material['poisson_ratio']['value']
    case = model['load_cases'][0]; pressure = case['pressure_regions'][0]['pressure']
    p = pressure['value'] * {'Pa':1, 'kPa':1000}[pressure['unit']]
    cap = p*bore
    close('UI pressure extension mm', row(case['id'], 'global_nodal_displacement_x', 'node:fixture-tip'), (1-2*nu)*cap/(e*area)*1000)
    for location in ['end_i','end_j','quarter_1','midspan','quarter_3']:
        close('UI membrane ' + location, row(case['id'], 'pipe_axial_membrane_stress_v2','pipe:fixture-span','axial_membrane_stress', location), cap/area)
        close('UI wall force ' + location, row(case['id'], 'pipe_wall_axial_force_v2','pipe:fixture-span','wall_axial_force', location), cap)
        close('UI effective force ' + location, row(case['id'], 'pipe_effective_axial_force_v2','pipe:fixture-span','effective_axial_force', location), 0, cap)
    mechanical = model['load_cases'][1]['id']
    for component, expected in zip(['Fx','Fy','Fz','Mx','My','Mz'],[-1000,-2000,3000,-400,-2500,-2600]):
        close('UI signed reaction ' + component, row(mechanical,'support_reaction_component_v2','support:fixture-root',component), expected)
    expected_normal = 1000/area + math.hypot(2500,2600)/z
    close('UI normal maximum', row(mechanical,'pipe_elastic_normal_stress_maximum_v2','pipe:fixture-span'), expected_normal)
    ux=1000/(e*area); uy=(2000/3+600/2)/(e*inertia); uz=(-3000/3+500/2)/(e*inertia)
    close('UI displacement maximum', output['summary']['max_displacement']['value'], math.hypot(ux,uy,uz)*1000)
    ui.append(dict(output=run['result_path'], output_sha256=run['result_sha256'], row_count=len(rows), independent_normal_maximum_pa=expected_normal))

od, wall, nu, pressure = map(Fraction.from_float,[4e-77,1e-77,0.1,4.7e-170])
r4 = float(2*nu*pressure*(od/2-wall)**2/(wall*(od-wall)))
check('R4 unchanged Fraction reference', r4 == 3.1333333333333337e-171)
range_probe=json.loads((JOIN/'MEMBRANE_RANGE_JOIN/range_probe.json').read_text())
g=range_probe['gamma_24']; k=range_probe['bending_12_stiffness']
allowance=g*(abs(k)/(1-g)+abs(k))*(1+64*sys.float_info.epsilon)
check('R4 subnormal transformation allowance', struct.unpack('>Q',struct.pack('>d',allowance))[0] == 1525 and 0<allowance<sys.float_info.min)
for path,digest in range_probe['historical_input_sha256'].items():
    check('preserved R4 evidence ' + path, sha(HIST/path)==digest)

log=(JOIN/'product-regressions-04.log').read_text()
counts=[int(n) for n in re.findall(r'test result: ok\. (\d+) passed; 0 failed; 0 ignored;',log)]
check('observed library/public test counts', counts[0]==243 and sum(counts[1:])==53, counts=counts)
check('R4 direct controls observed', all(('test membrane_publication_range::'+name+' ... ok') in log for name in ['original_subnormal_wall_force_direct_publisher_preserves_membrane','original_subnormal_wall_force_direct_extrema_preserves_membrane']))
final=[]
for file in manifest['files']:
    digest=sha(ROOT/file['path']);check('final source freeze '+file['path'], digest==file['sha256'])
    final.append(dict(path=file['path'],sha256=digest))
evidence_paths=[JOIN/'RETURN.md',JOIN/'DEPENDENCIES.json',JOIN/'TEST_JOIN_DISPOSITION.md',JOIN/'NATIVE_HANDOFF.md',JOIN/'UI_PRODUCER_CAPTURE.json',JOIN/'PORTABLE_REFERENCE_JOIN.json',JOIN/'product-regressions-04.json',JOIN/'product-regressions-04.log',JOIN/'MEMBRANE_RANGE_JOIN/RETURN.md',JOIN/'MEMBRANE_RANGE_JOIN/range_probe.json',JOIN/'EXACT_AUTHORING/RETURN.md',JOIN/'CREATE_SECTION_INPUTS/RETURN.md',JOIN/'PHYSICS_READER_JOIN/RUST_TASK/RETURN.md',HP/'NONLINEAR_CURRENT_REFERENCE/FROZEN_EXPECTATIONS.json',HP/'MEMBRANE_BACKCHECK/RETURN.md']
report=dict(actor='/root/physics_resume/joined_producer_review',parent='/root/physics_resume',role='TASK Type2',mechanism='delegated-harness-native',python=sys.version,execution='Static/source/hash and independent arithmetic only; no Cargo/npm/native/public solver run',checks_passed=len(checks),portable_fixtures=portable,rational_projections_checked=projections,full_ui_analytical_checks=ui,r4_reference_pa=r4,r4_allowance=allowance,observed_test_counts=counts,evidence_hashes={str(p):sha(p) for p in evidence_paths},final_source_hashes=final,patch_sha256=sha(HERE/'CANDIDATE.patch'))
(HERE/'verification.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:report[k] for k in ['checks_passed','rational_projections_checked','observed_test_counts','r4_reference_pa','r4_allowance']}))
