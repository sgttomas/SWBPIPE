"""Seeded faults against the WP6 package checks, on scratch copies only.

Written by the T1_WP6_STATIC_CASES TASK. For each mutant a fresh scratch tree
is built from ``git archive HEAD`` of the three tracked inputs the checks read
(reference_cases.json, the load-reference-1 table, the first-static runner
template) plus a copy of the untracked package. The mutant is applied there,
bound hashes in MANIFEST.json are recomputed when the mutant targets a bound
file (so detection must come from the substantive check, not the hash), and
check_package.py runs. A mutant is killed when the checks fail. The baseline
(no mutation) must pass. Nothing in the worktree is modified.

Usage: python mutations.py --worktree W --scratch S --raw-dir D --producer EXE
"""
from __future__ import annotations

import argparse
import hashlib
import io
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile

WR = 'projects/chirality-piping'
PKG = 'validation/qualification/fixtures/load_reference'
TRACKED = ['core/product_physics/tests/fixtures/load_reference_states/reference_cases.json',
           'fixtures/results/semantic_contract_v0_3_load_reference_1.json',
           'validation/qualification/fixtures/first_static/axial.runner_input.candidate.json']
CHECK = Path(__file__).with_name('check_package.py')


def build(worktree: Path, target: Path):
    if target.exists():
        shutil.rmtree(target)
    target.mkdir(parents=True)
    archive = subprocess.run(['git', '-C', str(worktree), 'archive', 'HEAD', '--'] + [f'{WR}/{p}' for p in TRACKED],
                             check=True, capture_output=True).stdout
    with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
        tar.extractall(target, filter='data')
    root = target / WR
    shutil.copytree(worktree / WR / PKG, root / PKG, ignore=shutil.ignore_patterns('__pycache__'))
    return root


def rehash(root: Path):
    path = root / PKG / 'MANIFEST.json'
    manifest = json.loads(path.read_bytes())
    for case in manifest['cases']:
        for role in ('runner_input', 'product_request', 'reference', 'selectors', 'criteria', 'analytical_reference'):
            case[role]['sha256'] = hashlib.sha256((root / case[role]['path']).read_bytes()).hexdigest()
    path.write_text(json.dumps(manifest, indent=2) + '\n')


def edit_json(path: Path, fn):
    data = json.loads(path.read_bytes())
    fn(data)
    path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + '\n')


def pkg(root, name):
    return root / PKG / name


def assertion(data, aid):
    return next(a for a in data['assertions'] + data.get('negative_assertions', []) if a['id'] == aid)


TB = 'prescribed_translation_two_bar'
FIT = 'signed_fit_states.fixed'
REFC = 'core/product_physics/tests/fixtures/load_reference_states/reference_cases.json'


def m_ref_value(root):
    def fn(d):
        d['values'][0]['value'] = d['values'][0]['value'] * (1 + 2e-16) + 1e-300
    edit_json(pkg(root, f'{TB}.reference.candidate.json'), fn)


def m_pointer(root):
    def fn(d):
        a = assertion(d, 'case:prescribed-root-translation.reaction.support:root.Fx')
        a['selector_origin']['reference_origin']['pointer'] = a['selector_origin']['reference_origin']['pointer'].replace('root_Fx', 'far_Fx')
    edit_json(pkg(root, f'{TB}.selectors.candidate.json'), fn)


def m_transform(root):
    def fn(d):
        assertion(d, 'case:prescribed-root-translation.axial.pipe:1.end_i')['selector_origin']['reference_origin']['transform'] = 'identity'
    edit_json(pkg(root, f'{TB}.selectors.candidate.json'), fn)


def m_unit_factor(root):
    def fn(d):
        assertion(d, 'case:prescribed-root-translation.disp.node:middle.ux')['selector']['unit'] = 'm'
    edit_json(pkg(root, f'{TB}.selectors.candidate.json'), fn)


def m_reference_decimal(root):
    def fn(d):
        q = d['cases'][TB]['variants']['annular_companion']['expected']['middle_UX']
        q['decimal'] = q['decimal'].replace('0.00006666', '0.00006667', 1)
    edit_json(root / REFC, fn)


def m_reference_exact(root):
    def fn(d):
        q = d['cases'][TB]['variants']['annular_companion']['expected']['root_Fx']
        q['exact']['rational'] = '38001/3'
    edit_json(root / REFC, fn)


def m_kind(root):
    def fn(d):
        assertion(d, 'case:prescribed-root-translation.reaction.support:root.Fx')['selector']['kind'] = 'support_reaction_component_v3'
    edit_json(pkg(root, f'{TB}.selectors.candidate.json'), fn)


def m_manifest_sha(root):
    def fn(d):
        d['cases'][0]['reference']['sha256'] = '0' * 64
    edit_json(pkg(root, 'MANIFEST.json'), fn)


def m_required_rows(root):
    def fn(d):
        d['cases'][0]['required_scalar_rows'] -= 1
    edit_json(pkg(root, 'MANIFEST.json'), fn)


def m_absolute_path(root):
    def fn(d):
        d['cases'][0]['reference']['path'] = str(root / d['cases'][0]['reference']['path'])
    edit_json(pkg(root, 'MANIFEST.json'), fn)


def m_runner_drift(root):
    def fn(d):
        d['solve']['preview_model']['model']['nodes'][1]['position']['x'] = 1.5
    edit_json(pkg(root, f'{TB}.runner_input.json'), fn)


def m_drop_value(root):
    def fn(d):
        d['values'].pop()
    edit_json(pkg(root, f'{TB}.reference.candidate.json'), fn)


def m_relax_rule(root):
    def fn(d):
        rule = next(r for r in d['tolerance_profile']['rules'] if r['absolute_tolerance_value'] > 0)
        rule['absolute_tolerance_value'] *= 10
    edit_json(pkg(root, f'{FIT}.criteria.candidate.json'), fn)


def m_relative_rule(root):
    def fn(d):
        rule = next(r for r in d['tolerance_profile']['rules'] if r['relative_tolerance_value'] > 0)
        rule['relative_tolerance_value'] = 1e-8
    edit_json(pkg(root, f'{FIT}.criteria.candidate.json'), fn)


def m_row_id(root):
    def fn(d):
        a = assertion(d, 'case:prescribed-root-translation.disp.node:middle.ux')
        a['selector']['id'] = a['selector']['id'].replace('node-middle', 'node-far')
    edit_json(pkg(root, f'{TB}.selectors.candidate.json'), fn)


def m_metadata(root):
    def fn(d):
        a = assertion(d, 'case:prescribed-root-translation.axial.pipe:1.midspan')
        a['selector']['metadata']['location'] = 'quarter_1'
    edit_json(pkg(root, f'{TB}.selectors.candidate.json'), fn)


def m_evidence_field(root):
    def fn(d):
        a = assertion(d, 'case:cold.evidence.member.pipe:fit.fit_strain')
        a['selector']['field'] = 'fit_strain_x'
    edit_json(pkg(root, f'{FIT}.selectors.candidate.json'), fn)


def m_evidence_key(root):
    def fn(d):
        a = assertion(d, 'case:cold.evidence.member.pipe:fit.fit_strain')
        a['selector']['key'] = {'pipe_id': 'pipe:other'}
    edit_json(pkg(root, f'{FIT}.selectors.candidate.json'), fn)


def m_definition(root):
    def fn(d):
        a = assertion(d, 'case:cold.evidence.member.pipe:fit.fit_strain')
        a['selector']['definition']['material_selection_kind'] = 'explicit_base_properties'
    edit_json(pkg(root, f'{FIT}.selectors.candidate.json'), fn)


def m_indistinct_negative(root):
    def fn(d):
        n = assertion(d, 'case:cold.axial.pipe:fit.midspan.not.sign_flipped')
        n['selector_origin']['reference_origin'] = {
            'kind': 'analytical', 'pointer': '/cases/signed_fit_states/variants/annular_companion/expected/cold/fixed_wall_N',
            'reference_unit': 'N', 'transform': 'identity'}
    edit_json(pkg(root, f'{FIT}.selectors.candidate.json'), fn)


def m_zero_scale_removed(root):
    def fn(d):
        a = assertion(d, 'case:cold.reaction.support:root.Fy')
        del a['selector_origin']['reference_origin']['zero_scale']
    edit_json(pkg(root, f'{FIT}.selectors.candidate.json'), fn)


def m_equilibrium_zero_as_nonzero(root):
    def fn(d):
        a = assertion(d, 'case:cold.reaction.support:root.Fy')
        a['selector_origin']['reference_origin'] = {
            'kind': 'analytical', 'pointer': '/cases/signed_fit_states/variants/annular_companion/expected/cold/fixed_root_Fx',
            'reference_unit': 'N', 'transform': 'identity'}
    edit_json(pkg(root, f'{FIT}.selectors.candidate.json'), fn)


def m_refusal_code(root):
    def fn(d):
        g = next(g for g in d['gaps'] if g['kind'] == 'refusal_control')
        g['expected']['code'] = 'LOAD_STATE_SOURCE_UNRESOLVED'
    edit_json(pkg(root, 'persistent_source_once.selectors.candidate.json'), fn)


def m_request_version(root):
    for name in (f'{TB}.preview_request.json', f'{TB}.runner_input.json'):
        def fn(d, name=name):
            model = d['model'] if 'model' in d else d['solve']['preview_model']['model']
            model['schema_version'] = '0.3.0'
        edit_json(pkg(root, name), fn)


MUTANTS = [
    ('M01 reference value +2 ulp in a committed reference file', m_ref_value, True),
    ('M02 selector pointer root_Fx -> far_Fx', m_pointer, True),
    ('M03 end_i negate transform dropped', m_transform, True),
    ('M04 selector unit mm -> m (no factor)', m_unit_factor, True),
    ('M05 reference_cases.json decimal rendering corrupted', m_reference_decimal, True),
    ('M06 reference_cases.json exact rational changed', m_reference_exact, True),
    ('M07 row kind not in the load-reference-1 table', m_kind, True),
    ('M08 manifest sha256 corrupted', m_manifest_sha, False),
    ('M09 required_scalar_rows off by one', m_required_rows, False),
    ('M10 absolute path in manifest', m_absolute_path, False),
    ('M11 runner preview_model drifts from product request', m_runner_drift, True),
    ('M12 positive reference value dropped', m_drop_value, True),
    ('M13 zero-scale absolute budget relaxed x10', m_relax_rule, True),
    ('M14 relative criterion relaxed to 1e-8', m_relative_rule, True),
    ('M15 row selector id points at another node', m_row_id, True),
    ('M16 row selector metadata location changed', m_metadata, True),
    ('M17 evidence field not in the declared field table', m_evidence_field, True),
    ('M18 evidence key names a missing member', m_evidence_key, True),
    ('M19 negative discriminator equal to the reference', m_indistinct_negative, True),
    ('M24 evidence definition contradicts the record', m_definition, True),
    ('M20 zero scale removed from a zero expectation', m_zero_scale_removed, True),
    ('M21 equilibrium zero replaced by a nonzero reference under a zero rule', m_equilibrium_zero_as_nonzero, True),
    ('M22 refusal control expects the wrong code', m_refusal_code, True),
    ('M23 request schema_version 0.4.0 -> 0.3.0', m_request_version, True),
]


def run_check(root: Path, raw_dir: str, producer: str) -> tuple[int, str]:
    result = subprocess.run([sys.executable, str(CHECK), '--root', str(root), '--raw-dir', raw_dir, '--producer', producer],
                            capture_output=True, text=True, env={'PYTHONDONTWRITEBYTECODE': '1', 'PATH': '/usr/bin:/bin'})
    lines = (result.stdout + result.stderr).strip().splitlines()
    first_fail = next((line for line in lines if line.startswith('FAIL') or 'Error' in line), lines[-1] if lines else '')
    return result.returncode, first_fail[:300]


def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument('--worktree', required=True)
    parser.add_argument('--scratch', required=True)
    parser.add_argument('--raw-dir', required=True)
    parser.add_argument('--producer', required=True)
    args = parser.parse_args(argv)
    worktree, scratch = Path(args.worktree), Path(args.scratch)
    root = build(worktree, scratch / 'baseline')
    code, line = run_check(root, args.raw_dir, args.producer)
    print(f'BASELINE exit={code} {line}')
    if code != 0:
        return 1
    killed = 0
    for name, fn, needs_rehash in MUTANTS:
        root = build(worktree, scratch / 'mutant')
        fn(root)
        if needs_rehash:
            rehash(root)
        code, line = run_check(root, args.raw_dir, args.producer)
        status = 'KILLED' if code != 0 else 'SURVIVED'
        killed += code != 0
        print(f'{status} {name} :: {line}')
    shutil.rmtree(scratch)
    print(f'{killed}/{len(MUTANTS)} mutants killed')
    return 0 if killed == len(MUTANTS) else 1


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
