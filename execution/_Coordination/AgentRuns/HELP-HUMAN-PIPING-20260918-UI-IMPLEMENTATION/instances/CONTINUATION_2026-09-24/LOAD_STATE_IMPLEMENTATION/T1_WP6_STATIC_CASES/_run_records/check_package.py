"""Structural and identification checks of the WP6 load-reference case package.

Written by the T1_WP6_STATIC_CASES TASK. It runs no comparison: it never reads
an observed result value. Checks (each counted and printed):

A  MANIFEST: format, closed keys, WORKING_ROOT-relative paths, sha256 of every
   bound file, case_key present, required_scalar_rows = positive assertions.
B  Requests: each preview request parses as a 0.4.0 exact-profile document;
   each runner input is {request, solve.preview_model} with the first-static
   request keys, and its preview_model equals the product request.
C  Generator: generate_reference_values.py reproduces every reference and
   criteria candidate byte for byte.
D  Selectors: every row selector names one load-reference-1 table signature
   (kind, unit, component, dimension, family); every evidence selector uses a
   declared record and field; every rule id exists; reference values cover
   exactly the positive assertions and wrong_values exactly the negatives.
E  (optional, --raw-dir) identification against producer output kept in
   scratch: every row selector matches exactly one row on id, kind, unit,
   entity_ref, basis_ref and metadata, and every evidence selector resolves
   to exactly one record entry whose field is a finite number and whose text
   fields equal the selector's `definition`, in both modes.
F  (optional, --producer) refusal controls: each gap of kind refusal_control
   is applied to its request and the named blocking code must appear.

Usage, from WORKING_ROOT or with --root: python check_package.py [--root R]
[--raw-dir D] [--producer EXE]
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import json
import math
from pathlib import Path
import subprocess
import sys
import tempfile

PKG_REL = 'validation/qualification/fixtures/load_reference'
TABLE_REL = 'fixtures/results/semantic_contract_v0_3_load_reference_1.json'
TEMPLATE_REL = 'validation/qualification/fixtures/first_static/axial.runner_input.candidate.json'
CONTRACT = 'openpipestress.result_semantics/0.3.0/load-reference-1'
CASE_KEYS = {'case_id', 'runner_input', 'product_request', 'reference', 'selectors', 'criteria',
             'analytical_reference', 'modes', 'required_scalar_rows'}
ROW_SELECTOR_KEYS = {'id', 'kind', 'unit', 'entity_ref', 'basis_ref', 'metadata', 'dimension'}
EVIDENCE_SELECTOR_KEYS = {'id', 'namespace', 'basis_ref', 'record', 'key', 'field', 'definition', 'unit', 'dimension'}
EVIDENCE_RECORDS = {'member': ('members', ('pipe_id',)), 'support_component': ('support_components', ('support_id', 'dof')),
                    'contribution': ('contributions', ('owner_kind', 'source_id'))}
EVIDENCE_FIELDS = {'member': {'selected_E_pa', 'selected_nu', 'derived_G_pa', 'thermal_strain', 'thermal_stretch',
                              'fit_strain', 'total_eigenstrain', 'installation_datum_stretch',
                              'operating_datum_stretch', 'installation_temperature_k', 'operating_temperature_k',
                              'material_selection_temperature_k', 'coefficient_datum_k', 'reference_length_m'},
                   'support_component': {'prescribed_value'}, 'contribution': {'applied_magnitude'}}
# Per-field criterion family and the record text fields a `definition` may constrain
# (the WP5 closed evidence vocabulary relayed by the manager).
FAMILIES = {'selected_E_pa': 'load_reference_material', 'selected_nu': 'load_reference_material',
            'derived_G_pa': 'load_reference_material', 'installation_temperature_k': 'load_reference_temperature',
            'operating_temperature_k': 'load_reference_temperature',
            'material_selection_temperature_k': 'load_reference_temperature',
            'coefficient_datum_k': 'load_reference_temperature', 'thermal_strain': 'load_reference_strain',
            'thermal_stretch': 'load_reference_strain', 'fit_strain': 'load_reference_strain',
            'total_eigenstrain': 'load_reference_strain', 'installation_datum_stretch': 'load_reference_strain',
            'operating_datum_stretch': 'load_reference_strain', 'reference_length_m': 'load_reference_geometry',
            'prescribed_value': 'load_reference_support_motion', 'applied_magnitude': 'load_reference_applied_load'}
DEFINITION_FIELDS = {'member': {'material_id', 'material_selection_kind', 'reference_basis', 'thermal_definition',
                                'expansion_law_id', 'fit_kind', 'eigenstrain_composition', 'G_basis'},
                     'support_component': {'node_id', 'law_kind', 'meaning', 'physical_state_source', 'global_dof'},
                     'contribution': {'classification', 'category', 'dimension'}}


class Counter:
    def __init__(self):
        self.passed, self.failed = {}, []

    def check(self, group: str, ok: bool, message: str):
        self.passed[group] = self.passed.get(group, 0) + (1 if ok else 0)
        if not ok:
            self.failed.append(f'{group}: {message}')


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def load(path: Path):
    def pairs(items):
        result = {}
        for key, value in items:
            if key in result:
                raise ValueError(f'duplicate JSON key {key} in {path}')
            result[key] = value
        return result
    return json.loads(path.read_bytes(), object_pairs_hook=pairs)


def check_manifest(root: Path, c: Counter) -> dict:
    manifest = load(root / PKG_REL / 'MANIFEST.json')
    c.check('A', set(manifest) == {'format', 'cases'}, 'manifest top-level keys')
    c.check('A', manifest['format'] == 'openpipestress.load_reference_qualification_manifest/1', 'manifest format')
    reference_cases = load(root / 'core/product_physics/tests/fixtures/load_reference_states/reference_cases.json')
    ids = [case['case_id'] for case in manifest['cases']]
    c.check('A', len(ids) == len(set(ids)), 'duplicate case id')
    for case in manifest['cases']:
        c.check('A', set(case) == CASE_KEYS, f'{case.get("case_id")}: case keys')
        for role in ('runner_input', 'product_request', 'reference', 'selectors', 'criteria', 'analytical_reference'):
            bound = case[role]
            path = bound['path']
            c.check('A', not path.startswith('/') and '..' not in Path(path).parts and '\\' not in path,
                    f'{case["case_id"]} {role}: path not WORKING_ROOT-relative')
            data = (root / path).read_bytes()
            c.check('A', sha(data) == bound['sha256'], f'{case["case_id"]} {role}: sha256 mismatch')
        c.check('A', set(case['analytical_reference']) == {'path', 'sha256', 'case_key'}, 'analytical_reference keys')
        c.check('A', case['analytical_reference']['case_key'] in reference_cases['cases'], f'{case["case_id"]}: case_key')
        c.check('A', case['modes'] == ['sparse_interactive', 'dense_scrutiny'], f'{case["case_id"]}: modes')
        selectors = load(root / case['selectors']['path'])
        c.check('A', case['required_scalar_rows'] == len(selectors['assertions']) > 0,
                f'{case["case_id"]}: required_scalar_rows')
    return manifest


def check_requests(root: Path, manifest: dict, c: Counter):
    template = load(root / TEMPLATE_REL)
    for case in manifest['cases']:
        request = load(root / case['product_request']['path'])
        runner = load(root / case['runner_input']['path'])
        model = request.get('model', {})
        c.check('B', set(request) == {'model', 'materials'} and request['materials'] == [], f'{case["case_id"]}: request shape')
        c.check('B', model.get('schema_version') == '0.4.0' and model.get('document_kind') == 'openpipestress.product_preview.model',
                f'{case["case_id"]}: 0.4.0 document')
        c.check('B', model.get('pressure_contract') == {'version': '2.0.0', 'mode': 'exact_straight_pressure_v2'},
                f'{case["case_id"]}: exact pressure contract')
        c.check('B', model.get('combinations') == [] and model.get('components') == [], f'{case["case_id"]}: exact route')
        lcs = [lc['id'] for lc in model['load_cases']]
        c.check('B', all('analysis_state' in lc and lc['pressure_regions'] == [] for lc in model['load_cases']),
                f'{case["case_id"]}: analysis_state and pressure_regions')
        c.check('B', set(runner) == {'request', 'solve'} and set(runner['solve']) == {'preview_model'},
                f'{case["case_id"]}: runner input shape')
        c.check('B', set(runner['request']) == set(template['request']), f'{case["case_id"]}: runner request keys')
        c.check('B', runner['request']['operation'] == 'solve', f'{case["case_id"]}: operation')
        c.check('B', [r['ref_id'] for r in runner['request']['load_basis_refs']] == lcs, f'{case["case_id"]}: load basis refs')
        c.check('B', runner['solve']['preview_model'] == request, f'{case["case_id"]}: preview_model equals request')


def check_generator(root: Path, c: Counter):
    result = subprocess.run([sys.executable, str(root / PKG_REL / 'generate_reference_values.py'), '--check'],
                            capture_output=True, text=True)
    lines = [line for line in result.stdout.splitlines() if line.startswith(('ok ', 'DIFFERS '))]
    for line in lines:
        c.check('C', line.startswith('ok '), line)
    c.check('C', result.returncode == 0 and bool(lines), 'generator --check failed: ' + result.stdout[-500:] + result.stderr[-500:])


def check_selectors(root: Path, manifest: dict, c: Counter):
    table = load(root / TABLE_REL)
    for case in manifest['cases']:
        selectors = load(root / case['selectors']['path'])
        reference = load(root / case['reference']['path'])
        criteria = load(root / case['criteria']['path'])
        name = case['case_id']
        c.check('D', set(selectors) <= {'format', 'case_id', 'producer_contract', 'raw_schema_version', 'row_namespace',
                                        'assertions', 'gaps', 'scoring_readiness', 'negative_assertions'},
                f'{name}: selector keys')
        c.check('D', selectors['producer_contract'] == CONTRACT and selectors['raw_schema_version'] == '0.2.0'
                and selectors['row_namespace'] == 'payload.mechanics_envelope.results', f'{name}: selector header')
        c.check('D', selectors['case_id'] == name == reference['case_id'], f'{name}: case ids')
        rules = {rule['rule_id']: rule for rule in criteria['tolerance_profile']['rules']}
        c.check('D', criteria['tolerance_profile']['profile_status'] == 'draft_pending_independent_review', f'{name}: draft')
        c.check('D', reference['readiness'] == 'pending_independent_review' and reference['independent_review_ref'] is None,
                f'{name}: pending reference')
        for group in ('assertions', 'negative_assertions'):
            for assertion in selectors.get(group, []):
                aid = assertion['id']
                c.check('D', set(assertion) <= {'id', 'selector', 'criterion_rule_id', 'selector_origin'}, f'{aid}: keys')
                selector = assertion['selector']
                rule = rules.get(assertion['criterion_rule_id'])
                c.check('D', rule is not None, f'{aid}: rule missing')
                if set(selector) == ROW_SELECTOR_KEYS:
                    component = selector['metadata']['component'] if selector['metadata'] else None
                    signatures = [row for row in table['rows'] if row['kind'] == selector['kind']
                                  and row['unit'] == selector['unit'] and row['component'] == component]
                    c.check('D', len(signatures) == 1, f'{aid}: kind {selector["kind"]} not one table signature')
                    if signatures:
                        c.check('D', signatures[0]['source_physical_semantic_dimension'] == selector['dimension'],
                                f'{aid}: dimension')
                        c.check('D', rule is not None and rule['result_family'] == signatures[0]['family'],
                                f'{aid}: family')
                        c.check('D', assertion['selector_origin'].get('semantic_signature_id') == signatures[0]['signature_id'],
                                f'{aid}: signature id')
                else:
                    c.check('D', set(selector) == EVIDENCE_SELECTOR_KEYS, f'{aid}: selector shape')
                    c.check('D', selector['namespace'] == 'contract_evidence.load_reference_states', f'{aid}: namespace')
                    record = EVIDENCE_RECORDS.get(selector['record'])
                    c.check('D', record is not None and set(selector['key']) == set(record[1]), f'{aid}: record key')
                    c.check('D', selector['field'] in EVIDENCE_FIELDS.get(selector['record'], ()), f'{aid}: field')
                    c.check('D', rule is not None and rule['result_family'] == FAMILIES.get(selector['field']), f'{aid}: family')
                    definition = selector['definition']
                    c.check('D', isinstance(definition, dict) and bool(definition)
                            and set(definition) <= DEFINITION_FIELDS.get(selector['record'], set())
                            and all(isinstance(v, str) and v for v in definition.values()), f'{aid}: definition')
                if rule is not None:
                    c.check('D', rule['unit_ref']['ref'] == selector['unit'] and rule['dimension_id'] == selector['dimension'],
                            f'{aid}: rule unit/dimension')
        positive = [a['id'] for a in selectors['assertions']]
        negative = [a['id'] for a in selectors.get('negative_assertions', [])]
        c.check('D', [v['assertion_id'] for v in reference['values']] == positive, f'{name}: values cover positives')
        c.check('D', [v['assertion_id'] for v in reference['wrong_values']] == negative, f'{name}: wrong_values cover negatives')
        wrong = {v['assertion_id']: v['value'] for v in reference['wrong_values']}
        pairs = [(json.dumps(a['selector'], sort_keys=True), wrong.get(a['id'])) for a in selectors.get('negative_assertions', [])]
        c.check('D', len(pairs) == len(set(pairs)), f'{name}: (selector, wrong value) pairs unique')
        units = {a['id']: a['selector']['unit'] for a in selectors['assertions'] + selectors.get('negative_assertions', [])}
        c.check('D', all(v['unit'] == units[v['assertion_id']] for v in reference['values'] + reference['wrong_values']),
                f'{name}: value units')
        c.check('D', all(isinstance(v['value'], float) and math.isfinite(v['value'])
                         for v in reference['values'] + reference['wrong_values']), f'{name}: finite values')


def evidence_entry(envelope: dict, selector: dict):
    records = [r for r in envelope['contract_evidence']['load_reference_states']
               if r['load_case_id'] == selector['basis_ref']['ref_id']]
    if len(records) != 1:
        return None, 'case record count'
    list_key, key_fields = EVIDENCE_RECORDS[selector['record']]
    entries = [e for e in records[0][list_key] if all(e.get(k) == selector['key'][k] for k in key_fields)]
    if len(entries) != 1:
        return None, f'{len(entries)} entries'
    return entries[0], ''


def check_identification(root: Path, manifest: dict, raw_dir: Path, c: Counter):
    for case in manifest['cases']:
        selectors = load(root / case['selectors']['path'])
        for mode in ('sparse', 'dense'):
            envelope = load(raw_dir / f'{case["case_id"]}.{mode}.json')
            c.check('E', envelope['producer'].get('semantic_contract_id') == CONTRACT, f'{case["case_id"]} {mode}: contract')
            rows = {}
            for row in envelope['results']:
                rows.setdefault(row['id'], []).append(row)
            for assertion in selectors['assertions'] + selectors.get('negative_assertions', []):
                selector = assertion['selector']
                if set(selector) == ROW_SELECTOR_KEYS:
                    found = rows.get(selector['id'], [])
                    ok = len(found) == 1 and all(found[0].get(k) == selector[k]
                                                 for k in ('kind', 'unit', 'entity_ref', 'basis_ref', 'metadata'))
                    c.check('E', ok, f'{assertion["id"]} {mode}: row selector does not resolve exactly once')
                else:
                    entry, why = evidence_entry(envelope, selector)
                    field = None if entry is None else entry.get(selector['field'])
                    ok = entry is not None and type(field) in (int, float) and math.isfinite(field)
                    c.check('E', ok, f'{assertion["id"]} {mode}: evidence selector {why or "field not a finite number"}')
                    if entry is not None:
                        c.check('E', all(entry.get(k) == v for k, v in selector['definition'].items()),
                                f'{assertion["id"]} {mode}: definition differs from the record')
                    if entry is not None and selector['record'] == 'support_component':
                        c.check('E', entry.get('unit') == selector['unit'], f'{assertion["id"]} {mode}: evidence unit')


def apply_mutation(request: dict, mutation: dict, reference: dict) -> dict:
    result = copy.deepcopy(request)
    parts = mutation['path'].strip('/').split('/')
    parent = result
    for part in parts[:-1]:
        parent = parent[int(part)] if isinstance(parent, list) else parent[part]
    value = mutation.get('value')
    if 'value_pointer' in mutation:
        node = reference
        for part in mutation['value_pointer'].strip('/').split('/'):
            node = node[int(part)] if isinstance(node, list) else node[part]
        value = {'value': node['value'], 'unit': node['unit']}
    if mutation['op'] == 'add' and parts[-1] == '-':
        parent.append(value)
    elif mutation['op'] == 'replace':
        key = parts[-1]
        if isinstance(parent, list):
            parent[int(key)] = value
        else:
            assert key in parent
            parent[key] = value
    else:
        raise ValueError(mutation)
    return result


def check_refusals(root: Path, manifest: dict, producer: Path, c: Counter):
    reference = load(root / 'core/product_physics/tests/fixtures/load_reference_states/reference_cases.json')
    for case in manifest['cases']:
        selectors = load(root / case['selectors']['path'])
        request = load(root / case['product_request']['path'])
        for gap in selectors.get('gaps', []):
            if gap['kind'] != 'refusal_control':
                continue
            mutated = apply_mutation(request, gap['mutation'], reference)
            with tempfile.NamedTemporaryFile('w', suffix='.json', delete=False) as handle:
                json.dump(mutated, handle)
            for mode in ('sparse', 'dense'):
                run = subprocess.run([str(producer), mode, handle.name], capture_output=True, text=True)
                codes = []
                try:
                    envelope = json.loads(run.stdout)
                    codes = [d['code'] for d in envelope['diagnostics'] if d.get('severity') == 'blocking']
                    solved = envelope['status']['mechanics'] == 'MECHANICS_SOLVED'
                except (ValueError, KeyError):
                    solved = None
                ok = gap['expected']['code'] in codes and solved is False
                c.check('F', ok, f'{case["case_id"]} {gap["id"]} {mode}: expected {gap["expected"]["code"]}, got {codes}')
            Path(handle.name).unlink()


def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument('--root', default='.')
    parser.add_argument('--raw-dir')
    parser.add_argument('--producer')
    args = parser.parse_args(argv)
    root = Path(args.root).resolve()
    c = Counter()
    manifest = check_manifest(root, c)
    check_requests(root, manifest, c)
    check_generator(root, c)
    check_selectors(root, manifest, c)
    if args.raw_dir:
        check_identification(root, manifest, Path(args.raw_dir), c)
    if args.producer:
        check_refusals(root, manifest, Path(args.producer), c)
    for group in sorted(c.passed):
        print(f'group {group}: {c.passed[group]} checks passed')
    for failure in c.failed:
        print('FAIL', failure)
    print(f'total passed {sum(c.passed.values())}, failed {len(c.failed)}')
    return 1 if c.failed else 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
