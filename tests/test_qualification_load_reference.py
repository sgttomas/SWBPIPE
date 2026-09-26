"""load-reference-1 VP-HARNESS adapter checks: synthetic controls, not solver evidence.

The fake runner replays the committed producer raws
(``fixtures/product_preview/load_reference/*.raw.json``) inside a synthetic
ControlledExport wrapper. Reference values are synthetic harness controls
copied from those raws (``reference_kind: harness_synthetic``); they are not
analytical targets, criteria or production policy, and no product solver or
runner binary is executed. The owned reader genuinely runs in its isolated
helper against the committed raws. Run with unittest to avoid the repository
pytest setup's unrelated Rust build.
"""
from __future__ import annotations
from copy import deepcopy
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

PROJECT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(PROJECT))
from tools.validation import qualification_load_reference as lr
from tools.validation.qualification_process import capture, sha256_bytes

RAWS = PROJECT / 'fixtures/product_preview/load_reference'
TABLE = json.loads((PROJECT / lr.TABLE).read_text())
FAMILIES = {(row['kind'], row['unit'], row['component']): row for row in TABLE['rows']}
PACKAGE = 'validation/qualification/fixtures/synthetic_load_reference_harness'
FAMILIES_CASES = ('connected', 'pressure')
SYNTHETIC = 'synthetic harness control copied from a committed producer raw; not an analytical reference'


def pinned_bytes(path, digest):
    """The pinned dependency bytes: the working tree, else the newest matching revision.

    Concurrent WP1 work may change the reader in the working tree before the
    integration re-pin; the fixture still runs the exact pinned reader bytes.
    """
    data = (PROJECT / path).read_bytes()
    if sha256_bytes(data) == digest:
        return data
    revisions = subprocess.check_output(['git', 'log', '--format=%H', '-n', '200', '--', path], cwd=PROJECT, text=True).split()
    for revision in revisions:
        data = subprocess.check_output(['git', 'show', f'{revision}:./{path}'], cwd=PROJECT)
        if sha256_bytes(data) == digest:
            return data
    raise AssertionError('pinned bytes not found for ' + path)


def raw_for(family, mode):
    return json.loads((RAWS / f'{family}-{mode}.raw.json').read_text())


def runner_input(family):
    product = json.loads((RAWS / f'{family}.request.json').read_text())
    return {'request': {'request_id': f'synthetic-lr-harness-{family}', 'operation': 'solve',
                        'provenance': {'source_name': 'synthetic WP5 harness control', 'review_status': 'synthetic_test_only'}},
            'solve': {'preview_model': product}}


def wrapper(mechanics, request):
    return {'blocked': False, 'decisions': [], 'findings': [],
            'summary': {'blocking_count': 0, 'decision_count': 0, 'finding_count': 0, 'warning_count': 0},
            'payload': {'artifact': 'openpipestress.headless_runner_cli_output', 'schema_version': '1.0.0',
                        'command': 'solve', 'operation': 'solve', 'request_validation': {'diagnostics': []},
                        'result_validation': {'diagnostics': []}, 'diagnostics': [],
                        'runner_result': {'run_id': 'run:headless-preview:' + request['request']['request_id'],
                                          'job': {'state': 'COMPLETED'}, 'analysis_status': ['MECHANICS_SOLVED'], 'diagnostics': []},
                        'mechanics_envelope': mechanics}}


def rule_id(family, unit):
    return f'rule:synthetic:{family}:{unit}'


def evidence_selectors(raw):
    """Member, support-component and contribution selectors for every record."""
    rows = []
    for record in raw['contract_evidence']['load_reference_states']:
        basis = {'ref_type': 'load_case', 'ref_id': record['load_case_id']}
        for member in record['members']:
            for field in ('total_eigenstrain', 'selected_E_pa', 'thermal_strain', 'operating_temperature_k'):
                unit, dimension, family = lr.MEMBER_FIELDS[field]
                rows.append(({'id': f"evidence:{record['load_case_id']}:member:{member['pipe_id']}:{field}", 'namespace': lr.EVIDENCE_NAMESPACE,
                              'basis_ref': basis, 'record': 'member', 'key': {'pipe_id': member['pipe_id']}, 'field': field,
                              'definition': {'material_selection_kind': member['material_selection_kind'], 'thermal_definition': member['thermal_definition']},
                              'unit': unit, 'dimension': dimension}, family, member[field]))
        for support in record['support_components']:
            unit, dimension = lr.motion_unit(support['dof'])
            rows.append(({'id': f"evidence:{record['load_case_id']}:support:{support['support_id']}:{support['dof']}", 'namespace': lr.EVIDENCE_NAMESPACE,
                          'basis_ref': basis, 'record': 'support_component', 'key': {'support_id': support['support_id'], 'dof': support['dof']},
                          'field': 'prescribed_value', 'definition': {'global_dof': support['global_dof']}, 'unit': unit, 'dimension': dimension},
                         lr._MOTION, support['prescribed_value']))
        for item in record['contributions']:
            fields = lr.CONTRIBUTION_FIELDS.get(item['owner_kind'], {})
            for field, (unit, dimension, family) in fields.items():
                if item['owner_kind'] == 'support_state':
                    unit, dimension = lr.motion_unit(item['source_id'].rsplit(':', 1)[-1])
                elif unit is None:
                    unit, dimension = lr.LOAD_UNITS[item['dimension']], item['dimension']
                rows.append(({'id': f"evidence:{record['load_case_id']}:contribution:{item['source_id']}:{field}", 'namespace': lr.EVIDENCE_NAMESPACE,
                              'basis_ref': basis, 'record': 'contribution', 'key': {'owner_kind': item['owner_kind'], 'source_id': item['source_id']},
                              'field': field, 'definition': {'classification': item['classification']}, 'unit': unit, 'dimension': dimension},
                             family, item[field]))
    return rows


def synthetic_package(case_id, raw):
    """Selectors, synthetic reference values and harness-only criteria from one raw."""
    assertions, values, rules = [], [], {}
    for row in raw['results']:
        metadata = row.get('metadata')
        semantic = FAMILIES[(row['kind'], row['unit'], metadata['component'] if metadata else None)]
        if semantic['family'] is None:
            continue
        selector = {'id': row['id'], 'kind': row['kind'], 'unit': row['unit'], 'entity_ref': row['entity_ref'],
                    'basis_ref': row['basis_ref'], 'metadata': metadata, 'dimension': semantic['source_physical_semantic_dimension']}
        assertions.append((selector, semantic['family'], row['value']))
    assertions += evidence_selectors(raw)
    positive = []
    for index, (selector, family, value) in enumerate(assertions):
        aid = f'assert:{index:04}'
        rules[(family, selector['dimension'], selector['unit'])] = rule_id(family, selector['unit'])
        positive.append({'id': aid, 'selector': selector, 'criterion_rule_id': rule_id(family, selector['unit'])})
        values.append({'assertion_id': aid, 'value': value, 'unit': selector['unit']})
    target = next(item for item in positive if 'namespace' not in item['selector'] and item['selector']['kind'] == 'support_reaction_component_v2'
                  and values[positive.index(item)]['value'] != 0)
    negative = [{'id': 'negative:omitted-coupling', 'selector': deepcopy(target['selector']), 'criterion_rule_id': target['criterion_rule_id']}]
    wrong = [{'assertion_id': 'negative:omitted-coupling', 'value': 0.0, 'unit': target['selector']['unit'], 'discriminator': 'synthetic wrong-result control'}]
    selectors = {'format': lr.SELECTOR_FORMAT, 'case_id': case_id, 'producer_contract': lr.CONTRACT, 'raw_schema_version': '0.2.0',
                 'row_namespace': lr.ROW_NAMESPACE, 'assertions': positive, 'negative_assertions': negative,
                 'gaps': ['synthetic harness package only']}
    reference = {'format': lr.REFERENCE_FORMAT, 'case_id': case_id, 'readiness': 'ready', 'reference_kind': 'harness_synthetic',
                 'basis': SYNTHETIC, 'independent_review_ref': None, 'values': values, 'wrong_values': wrong}
    criteria = {'schema_version': '0.1.0', 'tolerance_profile': {'profile_id': 'synthetic-harness-only', 'profile_status': 'reviewed', 'rules': [
        {'rule_id': rid, 'dimension_id': dimension, 'result_family': family, 'unit_ref': {'ref_type': 'unit', 'ref': unit},
         'normalization_basis': 'same_unit_required', 'relative_tolerance_value': 1e-9, 'absolute_tolerance_value': 0.0,
         'tolerance_value_status': 'project_specific_review_required', 'review': 'synthetic harness fixture only; not a criterion',
         'provenance': 'invented harness test value'} for (family, dimension, unit), rid in sorted(rules.items())]}}
    return selectors, reference, criteria


def dump(value):
    return (json.dumps(value, indent=1, allow_nan=False) + '\n').encode()


class Fixture:
    """A temporary git candidate holding reader dependencies and a synthetic package."""

    def __init__(self, test, mode='sparse_interactive', families=FAMILIES_CASES):
        self.test, self.mode = test, mode
        temp = tempfile.TemporaryDirectory()
        test.addCleanup(temp.cleanup)
        self.root = Path(temp.name)
        self.candidate = self.root / 'candidate'
        for name in list(lr.DEPENDENCIES) + [lr.ANALYTICAL_REFERENCE]:
            target = self.candidate / name
            target.parent.mkdir(parents=True, exist_ok=True)
            if name in lr.DEPENDENCIES:
                target.write_bytes(pinned_bytes(name, lr.DEPENDENCIES[name]))
            else:
                shutil.copyfile(PROJECT / name, target)
        review = self.write('reviews/SYNTHETIC_READER_REVIEW.md', b'Synthetic test-only review stub. No independent review is claimed.\n')
        self.raws, self.cases = {}, []
        for family in families:
            self.raws[family] = raw_for(family, mode)
            self.add_case(family, self.raws[family])
        analytical = json.loads((PROJECT / lr.ANALYTICAL_REFERENCE).read_bytes())
        self.analytical_sha256 = sha256_bytes((PROJECT / lr.ANALYTICAL_REFERENCE).read_bytes())
        assert 'prescribed_translation_two_bar' in analytical['cases']
        binding = {'format': lr.BINDING_FORMAT, 'contract_id': lr.CONTRACT, 'entrypoint': lr.ENTRYPOINT, 'status': 'reviewed_candidate',
                   'review_basis': [review], 'files': [{'path': path, 'sha256': digest} for path, digest in lr.DEPENDENCIES.items()]}
        self.binding = binding
        self.executable = self.root / 'fake-runner'
        self.outputs = {}
        for family in families:
            self.set_output(family, wrapper(self.raws[family], runner_input(family)))
        self.write_process()
        self.counter = 0

    def write(self, relative, data):
        path = self.candidate / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
        return {'path': relative, 'sha256': sha256_bytes(data)}

    def add_case(self, family, raw, *, case_id=None, package=None):
        case_id = case_id or f'synthetic-{family}'
        selectors, reference, criteria = package or synthetic_package(case_id, raw)
        base = f'{PACKAGE}/{case_id}'
        request = runner_input(family)
        self.cases.append({'case_id': case_id,
                           'runner_input': self.write(f'{base}.runner_input.json', dump(request)),
                           'product_request': self.write(f'{base}.preview_request.json', dump(request['solve']['preview_model'])),
                           'reference': self.write(f'{base}.reference.json', dump(reference)),
                           'selectors': self.write(f'{base}.selectors.json', dump(selectors)),
                           'criteria': self.write(f'{base}.criteria.json', dump(criteria)),
                           'analytical_reference': {'path': lr.ANALYTICAL_REFERENCE,
                                                    'sha256': sha256_bytes((PROJECT / lr.ANALYTICAL_REFERENCE).read_bytes()),
                                                    'case_key': 'prescribed_translation_two_bar'},
                           'modes': list(lr.MODES), 'required_scalar_rows': len(selectors['assertions'])})

    def set_output(self, family, value, *, text=None):
        path = self.root / f'output-{family}.json'
        path.write_text(text if text is not None else json.dumps(value, allow_nan=False))
        self.outputs[runner_input(family)['request']['request_id']] = str(path)

    def write_process(self, code=None):
        body = code if code is not None else (
            'import json, sys\n'
            f'outputs = {self.outputs!r}\n'
            'request = json.loads(sys.stdin.buffer.read())\n'
            'sys.stdout.buffer.write(open(outputs[request["request"]["request_id"]], "rb").read())\n')
        self.executable.write_text('#!' + sys.executable + '\n' + body)
        self.executable.chmod(0o700)

    def commit(self):
        git = ['git', '-c', 'user.name=synthetic-test', '-c', 'user.email=synthetic@example.invalid', '-c', 'commit.gpgsign=false']
        if not (self.candidate / '.git').exists():
            subprocess.run(['git', 'init', '-q'], cwd=self.candidate, check=True)
        subprocess.run(git + ['add', '-A'], cwd=self.candidate, check=True)
        subprocess.run(git + ['commit', '-q', '--allow-empty', '-m', 'synthetic candidate'], cwd=self.candidate, check=True)
        return subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=self.candidate, text=True).strip()

    def selection(self, manifest=None, **changes):
        self.counter += 1
        manifest_value = {'format': lr.MANIFEST_FORMAT, 'cases': self.cases} if manifest is None else manifest
        manifest_ref = self.write(f'{PACKAGE}/MANIFEST.json', dump(manifest_value))
        binding_path = self.root / f'binding-{self.counter}.json'
        binding_path.write_bytes(dump(self.binding))
        commit = self.commit()
        run = {'format': lr.RUN_FORMAT, 'profile_id': 'synthetic-lr-harness', 'purpose': 'harness_development', 'transport': lr.TRANSPORT,
               'runner': {'candidate_commit': commit, 'executable_sha256': sha256_bytes(self.executable.read_bytes()),
                          'solver_mode': self.mode, 'explicit_local_private_intent': False},
               'case_manifest': {'path': 'candidate/' + manifest_ref['path'], 'sha256': manifest_ref['sha256']},
               'reader_binding': {'path': binding_path.name, 'sha256': sha256_bytes(binding_path.read_bytes())}}
        for key, value in changes.items():
            run[key] = value
        path = self.root / f'selection-{self.counter}.json'
        path.write_bytes(dump(run))
        return path

    def run(self, selection=None, **kwargs):
        selection = selection or self.selection()
        self.runs = getattr(self, 'runs', 0) + 1
        output = self.root / f'run-{self.runs}'
        return lr.run_selection(selection, self.executable, self.candidate, output, **kwargs), output


def assertion(case, aid):
    return next(row for row in case['assertions'] if row['id'] == aid)


def check(case, cid):
    return next(row for row in case['structural_checks'] if row['id'] == cid)


class PinTests(unittest.TestCase):
    def test_pinned_identities_match_recorded_bytes(self):
        self.assertEqual(sha256_bytes((PROJECT / lr.TABLE).read_bytes()), lr.TABLE_SHA256)
        self.assertEqual(TABLE['semantic_contract_id'], lr.CONTRACT)
        self.assertEqual(TABLE['formulation_profile_id'], lr.PROFILE)
        for path, digest in lr.DEPENDENCIES.items():
            if path != lr.MODULE:
                self.assertEqual(sha256_bytes((PROJECT / path).read_bytes()), digest, path)
        start = subprocess.check_output(['git', 'show', f'1ccca8b87:./{lr.MODULE}'], cwd=PROJECT)
        self.assertEqual(sha256_bytes(start), lr.MODULE_SHA256, 'reader pin is not the integrated WP1 reader bytes (1ccca8b87, after the REVIEW_A follow-up; earlier pins bfef71b19 and the WP5 start commit d8f0dc4f7)')
        self.assertEqual(lr.TRANSPORT, 'load_reference_1_cli_1.0_raw0.2')
        self.assertNotIn('source', lr.CONTRACT.rsplit('/', 1)[-1])

    def test_working_tree_reader_matches_pin(self):
        # Fails deliberately while the working-tree reader differs from the pin
        # (e.g. WP1 changed it): re-pin MODULE_SHA256 at integration, never skip.
        self.assertEqual(sha256_bytes((PROJECT / lr.MODULE).read_bytes()), lr.MODULE_SHA256,
                         'working-tree load_reference_evidence.py differs from MODULE_SHA256: re-pin at integration')


class CompleteRunTests(unittest.TestCase):
    def test_complete_record_both_modes_and_same_ledger_json_and_markdown(self):
        for mode in lr.MODES:
            fixture = Fixture(self, mode)
            result, output = fixture.run()
            self.assertEqual(result['outcome'], 'all_required_assertions_matched', json.dumps(result['summary']))
            self.assertEqual(result['summary']['required_cases'], 2)
            positive = sum(case['required_scalar_rows'] for case in fixture.cases)
            self.assertEqual(sum(result['summary']['assertions_by_polarity']['positive'].values()), positive)
            self.assertEqual(result['summary']['assertions_by_polarity']['negative']['matched'], 2)
            self.assertEqual(result['summary']['structural_checks']['matched'], 2 * len(lr.REQUIRED_STRUCTURAL_CHECKS))
            self.assertEqual(result['summary']['numerical_standing']['checks_passed'], 2)
            self.assertEqual(result['qualification'], 'not_established_by_this_harness')
            for case in result['cases']:
                self.assertEqual(case['numerical_standing']['standing'], 'checks_passed')
                self.assertEqual(case['reader_consistency']['response']['verdict'], 'consistent')
                self.assertEqual(case['process']['command'][1:], ['solve', '--input', '-', '--solver-mode', mode])
                evidence = [row for row in case['assertions'] if row['id'].startswith('assert:') and row['state'] == 'matched']
                self.assertTrue(evidence)
            self.assertEqual(json.loads((output / 'ledger.json').read_text()), result)
            summary = (output / 'summary.md').read_text()
            self.assertIn('Outcome: all_required_assertions_matched', summary)
            self.assertIn('Numerical standing (reported separately from outcomes): checks_passed: 2', summary)
            self.assertEqual(result['execution_basis']['reader_module_sha256'], lr.MODULE_SHA256)

    def test_evidence_selectors_cover_member_support_and_contribution_records(self):
        fixture = Fixture(self, families=('connected',))
        result, _ = fixture.run()
        selectors = json.loads((fixture.candidate / fixture.cases[0]['selectors']['path']).read_text())['assertions']
        records = {row['selector'].get('record') for row in selectors}
        self.assertTrue({'member', 'support_component', 'contribution'} <= records)
        self.assertEqual(result['outcome'], 'all_required_assertions_matched')

    def test_ledger_exists_before_process_starts(self):
        fixture = Fixture(self, families=('pressure',))
        original = capture
        def observing(*args, **kwargs):
            if args[1][:1] == ['solve']:
                ledger = json.loads((args[3].parent / 'ledger.json').read_text())
                self.assertEqual({row['state'] for row in ledger['cases'][0]['assertions']}, {'not_run'})
                self.assertEqual(len(ledger['cases'][0]['assertions']), fixture.cases[0]['required_scalar_rows'] + 1)
            return original(*args, **kwargs)
        with patch.object(lr, 'capture', observing):
            self.assertEqual(fixture.run()[0]['outcome'], 'all_required_assertions_matched')

    def test_reference_files_unchanged_after_run(self):
        fixture = Fixture(self, families=('pressure',))
        selection = fixture.selection()
        before = {p: p.read_bytes() for p in (fixture.candidate / PACKAGE).glob('*.json')}
        self.assertEqual(len(before), 6)
        self.assertEqual(fixture.run(selection)[0]['outcome'], 'all_required_assertions_matched')
        self.assertEqual({p: p.read_bytes() for p in (fixture.candidate / PACKAGE).glob('*.json')}, before)


class ValueFaultTests(unittest.TestCase):
    def run_mutated(self, mutate, family='connected'):
        fixture = Fixture(self, families=(family,))
        raw = deepcopy(fixture.raws[family])
        mutate(raw)
        fixture.set_output(family, wrapper(raw, runner_input(family)))
        return fixture.run()[0], fixture

    def test_wrong_value_and_sign_fail_without_losing_denominator(self):
        for factor in (1.001, -1.0):
            fixture = Fixture(self, families=('connected',))
            raw = deepcopy(fixture.raws['connected'])
            row = next(r for r in raw['results'] if r['kind'] == 'element_local_bending_moment_z' and r['value'] != 0)
            row['value'] *= factor
            fixture.set_output('connected', wrapper(raw, runner_input('connected')))
            result, _ = fixture.run()
            case = result['cases'][0]
            self.assertEqual(result['outcome'], 'not_satisfied')
            self.assertEqual(result['summary']['assertions']['failed'], 1)
            self.assertEqual(len(case['assertions']), fixture.cases[0]['required_scalar_rows'] + 1)
            failed = next(r for r in case['assertions'] if r['state'] == 'failed')
            self.assertEqual(failed['reason'], 'exceeds_tolerance_profile')

    def test_negative_assertion_reproducing_wrong_result_fails(self):
        fixture = Fixture(self, families=('connected',))
        selectors = json.loads((fixture.candidate / fixture.cases[0]['selectors']['path']).read_text())
        target_id = selectors['negative_assertions'][0]['selector']['id']
        raw = deepcopy(fixture.raws['connected'])
        next(r for r in raw['results'] if r['id'] == target_id)['value'] = 0.0
        fixture.set_output('connected', wrapper(raw, runner_input('connected')))
        result, _ = fixture.run()
        negative = assertion(result['cases'][0], 'negative:omitted-coupling')
        self.assertEqual((negative['state'], negative['reason']), ('failed', 'wrong_result_reproduced'))
        self.assertEqual(result['summary']['assertions_by_polarity']['positive']['failed'], 1)

    def test_missing_row_stays_unavailable_never_zero(self):
        fixture = Fixture(self, families=('connected',))
        raw = deepcopy(fixture.raws['connected'])
        removed = next(r for r in raw['results'] if r['kind'] == 'element_local_axial_force')
        raw['results'].remove(removed)
        fixture.set_output('connected', wrapper(raw, runner_input('connected')))
        result, _ = fixture.run()
        case = result['cases'][0]
        errored = [r for r in case['assertions'] if r['state'] == 'error']
        self.assertEqual(len(errored), 1)
        self.assertIsNone(errored[0]['observed'])
        self.assertIn('unavailable', errored[0]['reason'])
        self.assertEqual(check(case, 'complete_unique_selector_coverage')['state'], 'failed')
        self.assertEqual(len(case['assertions']), fixture.cases[0]['required_scalar_rows'] + 1)

    def test_null_evidence_value_is_unavailable_not_zero(self):
        result, fixture = self.run_mutated(lambda raw: raw['contract_evidence']['load_reference_states'][0]['members'][0].update(total_eigenstrain=None))
        case = result['cases'][0]
        errored = [r for r in case['assertions'] if r['state'] == 'error']
        self.assertEqual(len(errored), 1)
        self.assertIsNone(errored[0]['observed'])
        self.assertIn('null is not zero', errored[0]['reason'])
        self.assertEqual(check(case, 'owned_reader_consistency')['state'], 'failed')

    def test_missing_evidence_record_or_member_is_unavailable(self):
        for mutate in (lambda raw: raw['contract_evidence']['load_reference_states'].pop(),
                       lambda raw: raw['contract_evidence']['load_reference_states'][0]['members'].pop()):
            result, _ = self.run_mutated(mutate)
            case = result['cases'][0]
            self.assertNotEqual(result['outcome'], 'all_required_assertions_matched')
            self.assertTrue(any(r['state'] == 'error' and 'does not resolve exactly once' in r['reason'] for r in case['assertions']))
            self.assertEqual(check(case, 'complete_unique_selector_coverage')['state'], 'failed')

    def test_duplicate_evidence_member_refused(self):
        result, _ = self.run_mutated(lambda raw: raw['contract_evidence']['load_reference_states'][0]['members'].append(
            deepcopy(raw['contract_evidence']['load_reference_states'][0]['members'][0])))
        self.assertTrue(any('does not resolve exactly once' in r['reason'] for r in result['cases'][0]['assertions']))

    def test_support_component_unit_must_match_selector(self):
        # REVIEW_B X03: the record's own unit binds the selected quantity.
        result, _ = self.run_mutated(lambda raw: raw['contract_evidence']['load_reference_states'][0]['support_components'][0].update(unit='mm'))
        self.assertTrue(any(r['state'] == 'error' and 'support component unit differs' in r['reason'] for r in result['cases'][0]['assertions']))

    def test_applied_load_dimension_must_match_selector(self):
        # REVIEW_B X04: an applied magnitude's dimension/unit comes from the contribution.
        def mutate(raw):
            item = next(c for c in raw['contract_evidence']['load_reference_states'][0]['contributions'] if c['owner_kind'] == 'stored_primitive')
            item['dimension'] = 'moment'
        result, _ = self.run_mutated(mutate)
        errors = [r for r in result['cases'][0]['assertions'] if r['state'] == 'error']
        self.assertEqual(len(errors), 2)  # applied and authored magnitudes of that contribution
        self.assertTrue(all('applied load dimension/unit differs' in r['reason'] for r in errors))

    def test_definition_value_type_is_exact(self):
        # REVIEW_B X10: 0.0 is not the index 0 even though they compare equal.
        result, _ = self.run_mutated(lambda raw: raw['contract_evidence']['load_reference_states'][0]['support_components'][0].update(
            global_dof=float(raw['contract_evidence']['load_reference_states'][0]['support_components'][0]['global_dof'])))
        self.assertTrue(any(r['state'] == 'error' and 'definition differs: global_dof' in r['reason'] for r in result['cases'][0]['assertions']))

    def test_evidence_definition_mismatch_refused(self):
        result, _ = self.run_mutated(lambda raw: raw['contract_evidence']['load_reference_states'][0]['members'][0].update(thermal_definition='constant_alpha_interval'))
        self.assertTrue(any('definition differs' in r['reason'] for r in result['cases'][0]['assertions']))

    def test_duplicate_result_signature_under_another_id_does_not_resolve_once(self):
        def mutate(raw):
            row = next(r for r in raw['results'] if r['kind'] == 'global_nodal_displacement_y')
            raw['results'].append(dict(deepcopy(row), id=row['id'] + ':shadow'))
        result, _ = self.run_mutated(mutate)
        case = result['cases'][0]
        self.assertTrue(any('does not resolve exactly once' in r['reason'] for r in case['assertions']))
        self.assertEqual(check(case, 'complete_unique_selector_coverage')['state'], 'failed')

    def test_wrong_case_frame_location_sign_definition_or_unit_metadata(self):
        mutations = [('basis_ref', {'ref_type': 'load_case', 'ref_id': 'case:elsewhere'}), ('unit', 'm'), ('entity_ref', 'node:elsewhere')]
        for field in ('coordinate_system', 'location', 'sign_convention', 'basis', 'component'):
            mutations.append(('metadata.' + field, 'changed'))
        for field, value in mutations:
            def mutate(raw, field=field, value=value):
                row = next(r for r in raw['results'] if r['kind'] == 'element_local_shear_force_y')
                if field.startswith('metadata.'):
                    row['metadata'][field.split('.', 1)[1]] = value
                else:
                    row[field] = value
            result, _ = self.run_mutated(mutate)
            self.assertNotEqual(result['outcome'], 'all_required_assertions_matched', field)
            case = result['cases'][0]
            self.assertTrue(any(r['state'] == 'error' for r in case['assertions']) or case['state'] == 'error', field)


class BindingTests(unittest.TestCase):
    def test_connector_evidence_fails_the_contract_obligation(self):
        # REVIEW_B X07: connector evidence is outside the load-reference-1 profile.
        case = self.observed(lambda raw: raw['contract_evidence'].update(connector=[{'synthetic': 'connector'}]))
        contract = check(case, 'load_reference_contract')
        self.assertEqual(contract['state'], 'failed')
        self.assertIn('connector evidence unsupported', contract['reason'])
        self.assertNotEqual(case['state'], 'matched')

    def observed(self, mutate, family='connected'):
        fixture = Fixture(self, families=(family,))
        raw = deepcopy(fixture.raws[family])
        mutate(raw)
        fixture.set_output(family, wrapper(raw, runner_input(family)))
        return fixture.run()[0]['cases'][0]

    def test_raw_load_cases_must_equal_bound_input(self):
        def drop_exact(raw):
            raw['contract_evidence']['exact_cases'].pop()
        def foreign_row(raw):
            row = deepcopy(next(r for r in raw['results'] if r['kind'] == 'global_nodal_displacement_z'))
            raw['results'].append(dict(row, id=row['id'] + ':foreign', basis_ref={'ref_type': 'load_case', 'ref_id': 'case:foreign'}))
        for mutate, message in ((drop_exact, 'substitution'), (foreign_row, 'outside the bound input')):
            case = self.observed(mutate)
            binding = check(case, 'actual_input_binding')
            self.assertEqual(binding['state'], 'failed')
            self.assertIn(message, binding['reason'])
            self.assertNotEqual(case['state'], 'matched')

    def test_record_mode_join_and_reference_configuration_bind(self):
        def dense(raw):
            for record in raw['contract_evidence']['load_reference_states']:
                record['solve'].update(requested_mode='dense_scrutiny', recovery_method='ordinary_dense_structural_v1')
        def joined(raw):
            raw['contract_evidence']['load_reference_states'][0]['source_recovery'] = {'status': 'joined', 'code': 'LOAD_STATE_SOURCE_RECOVERY_JOINED'}
        def configuration(raw):
            for record in raw['contract_evidence']['load_reference_states']:
                record['reference_configuration_id'] = 'reference:substituted'
        def contract(raw):
            raw['contract_evidence']['load_reference_states'][0]['contract'] = 'openpipestress.load_reference_state/2.0.0'
        def profile(raw):
            raw['contract_evidence']['load_reference_states'][-1]['profile'] = 'resolved_straight_load_state_source_v1'
        # REVIEW_B X12: record contract and profile.
        for mutate, message in ((dense, 'mode/recovery method differs'), (joined, 'joined source recovery'),
                                (configuration, 'reference configuration differs'), (contract, 'record contract/profile differs'),
                                (profile, 'record contract/profile differs')):
            case = self.observed(mutate)
            records = check(case, 'load_reference_record_binding')
            self.assertEqual(records['state'], 'failed')
            self.assertIn(message, records['reason'])
            self.assertEqual(case['state'], 'failed')


class TransportRefusalTests(unittest.TestCase):
    def refused(self, raw=None, *, text=None, family='connected', mode='sparse_interactive', message=None):
        fixture = Fixture(self, mode=mode, families=(family,))
        fixture.set_output(family, wrapper(raw, runner_input(family)) if raw is not None else None, text=text)
        result, _ = fixture.run()
        case = result['cases'][0]
        self.assertEqual(case['state'], 'error', case['reason'])
        self.assertEqual({r['state'] for r in case['assertions']}, {'error'})
        self.assertTrue(all(r['observed'] is None for r in case['assertions']))
        self.assertEqual(len(case['assertions']), fixture.cases[0]['required_scalar_rows'] + 1)
        self.assertEqual(len(case['structural_checks']), len(lr.REQUIRED_STRUCTURAL_CHECKS))
        if message:
            self.assertIn(message, case['reason'])
        return case

    def test_wrong_contract_profile_schema_and_foreign_namespaces_refuse(self):
        base = raw_for('connected', 'sparse_interactive')
        changes = [
            (lambda raw: raw['producer'].update(semantic_contract_id='openpipestress.result_semantics/0.3.0/physics-1'), 'raw/producer identity'),
            (lambda raw: raw['producer'].update(semantic_contract_id='openpipestress.result_semantics/0.3.0/load-reference-source-1'), 'raw/producer identity'),
            (lambda raw: raw['formulation_basis'].update(profile_id='resolved_straight_load_state_source_v1'), 'formulation basis invalid'),
            (lambda raw: raw['formulation_basis'].update(profile_id='exact_straight_pressure_v2'), 'formulation basis invalid'),
            (lambda raw: raw.update(schema_version='0.1.0'), 'raw/producer identity'),
            (lambda raw: raw.update(source_block_recovery=None), 'source/composite namespace refused'),
            (lambda raw: raw.update(carrier_evidence={}), 'source/composite namespace refused'),
            (lambda raw: raw['contract_evidence'].pop('load_reference_states'), 'load-reference evidence namespace required'),
        ]
        for change, message in changes:
            raw = deepcopy(base)
            change(raw)
            self.refused(raw, message=message)

    def test_nonfinite_and_nonnumeric_values_refuse(self):
        text = json.dumps(wrapper(raw_for('connected', 'sparse_interactive'), runner_input('connected')))
        first = raw_for('connected', 'sparse_interactive')['results'][1]
        needle = json.dumps(first['id']) + ', "kind": ' + json.dumps(first['kind']) + ', "value": ' + json.dumps(first['value'])
        self.assertIn(needle, text)
        for token, message in (('NaN', 'nonfinite JSON constant'), ('Infinity', 'nonfinite JSON constant'), ('1e999', 'nonfinite numeric token'),
                               ('null', 'numeric value required'), ('"1.0"', 'numeric value required'), ('true', 'numeric value required'),
                               ('1e-400', 'underflow')):
            self.refused(text=text.replace(needle, needle.rsplit(' ', 1)[0] + ' ' + token), message=message)

    def test_duplicate_result_id_and_wrong_mode_refuse(self):
        raw = raw_for('pressure', 'sparse_interactive')
        raw['results'].append(deepcopy(raw['results'][2]))
        self.refused(raw, family='pressure', message='missing/duplicate result ID')
        self.refused(raw_for('pressure', 'dense_scrutiny'), family='pressure', message='actual mode evidence mismatch')

    def test_substituted_model_or_blocked_output_refuses(self):
        self.refused(raw_for('pressure', 'sparse_interactive'), family='connected', message='actual model identity mismatch')
        blocked = wrapper(raw_for('connected', 'sparse_interactive'), runner_input('connected'))
        blocked['blocked'] = True
        self.refused(text=json.dumps(blocked), message='controlled output blocked')
        unsolved = raw_for('connected', 'sparse_interactive')
        unsolved['status']['mechanics'] = 'MECHANICS_BLOCKED'
        self.refused(unsolved, message='mechanics not solved')

    def test_wrapper_warning_count_must_match_findings(self):
        # REVIEW_B X08.
        value = wrapper(raw_for('connected', 'sparse_interactive'), runner_input('connected'))
        value['summary']['warning_count'] = 1
        self.refused(text=json.dumps(value), message='wrapper warning count inconsistent')
        value = wrapper(raw_for('connected', 'sparse_interactive'), runner_input('connected'))
        value['findings'] = [{'severity': 'warning', 'code': 'SYNTHETIC'}]
        value['summary'].update(finding_count=1, warning_count=0)
        self.refused(text=json.dumps(value), message='wrapper warning count inconsistent')

    def test_missing_standing_refuses(self):
        raw = raw_for('connected', 'sparse_interactive')
        raw.pop('numerical_quality')
        self.refused(raw, message='numerical quality shape invalid')

    def test_truncated_and_malformed_output_refuses(self):
        text = json.dumps(wrapper(raw_for('connected', 'sparse_interactive'), runner_input('connected')))
        self.refused(text=text[: len(text) // 2])
        self.refused(text='')
        self.refused(text='{"payload": {}}')


class StandingTests(unittest.TestCase):
    def run_standing(self, envelope, case_quality):
        fixture = Fixture(self, families=('connected',))
        raw = deepcopy(fixture.raws['connected'])
        raw['numerical_quality']['status'] = envelope
        raw['numerical_quality']['cases'][0]['solve_quality'] = case_quality
        fixture.set_output('connected', wrapper(raw, runner_input('connected')))
        return fixture.run()[0]

    def test_sensitive_standing_reported_separately_from_matched_outcomes(self):
        result = self.run_standing('sensitive', 'sensitive')
        case = result['cases'][0]
        self.assertEqual(case['numerical_standing']['standing'], 'sensitive')
        self.assertEqual(case['numerical_standing']['envelope_status'], 'sensitive')
        # Comparison outcomes stand on their own; standing is not folded into them.
        self.assertEqual({r['state'] for r in case['assertions']}, {'matched'})
        self.assertEqual(check(case, 'numerical_standing')['state'], 'failed')
        self.assertEqual(case['state'], 'failed')
        self.assertEqual(result['outcome'], 'not_satisfied')
        self.assertEqual(result['summary']['numerical_standing']['sensitive'], 1)
        self.assertEqual(result['summary']['numerical_standing']['checks_passed'], 0)

    def test_other_insufficient_standings(self):
        for envelope, case_quality in (('failed', 'failed'), ('not_assessed', 'not_assessed'), ('checks_passed', 'unresolved')):
            case = self.run_standing(envelope, case_quality)['cases'][0]
            self.assertEqual(case['numerical_standing']['standing'], 'insufficient')
            self.assertEqual(check(case, 'numerical_standing')['state'], 'failed')

    def test_unknown_or_uncovered_standing_is_an_error(self):
        case = self.run_standing('mostly_fine', 'checks_passed')['cases'][0]
        self.assertEqual(case['numerical_standing']['standing'], 'unavailable')
        self.assertEqual(check(case, 'numerical_standing')['state'], 'error')
        fixture = Fixture(self, families=('connected',))
        raw = deepcopy(fixture.raws['connected'])
        raw['numerical_quality']['cases'].pop()
        fixture.set_output('connected', wrapper(raw, runner_input('connected')))
        case = fixture.run()[0]['cases'][0]
        self.assertEqual(check(case, 'numerical_standing')['state'], 'error')

    def test_standing_evidence_codes_must_all_be_checks_passed(self):
        # REVIEW_B X02: checks_passed labels without passing integrity evidence are insufficient.
        raw = raw_for('pressure', 'sparse_interactive')
        cases = [c['basis_ref']['ref_id'] for c in raw['numerical_quality']['cases']]
        ref = raw['numerical_quality']['cases'][0]['evidence_refs'][0]
        next(d for d in raw['diagnostics'] if d['id'] == ref)['code'] = 'NUMERICAL_INTEGRITY_SENSITIVE'
        standing = lr.numerical_standing(raw, cases)
        self.assertEqual(standing['standing'], 'insufficient')
        self.assertEqual(standing['load_cases'][0]['evidence_codes'], ['NUMERICAL_INTEGRITY_SENSITIVE'])

    def test_standing_classification_pure(self):
        raw = raw_for('pressure', 'sparse_interactive')
        cases = [c['basis_ref']['ref_id'] for c in raw['numerical_quality']['cases']]
        self.assertEqual(lr.numerical_standing(raw, cases)['standing'], 'checks_passed')
        with self.assertRaises(ValueError):
            lr.numerical_standing(raw, cases + ['case:absent'])
        raw['numerical_quality']['cases'][0]['accuracy_evidence'] = 'reference_verified'
        self.assertEqual(lr.numerical_standing(raw, cases)['standing'], 'insufficient')


class ReaderTests(unittest.TestCase):
    def test_owned_reader_refusal_fails_its_obligation_only(self):
        fixture = Fixture(self, families=('connected',))
        raw = deepcopy(fixture.raws['connected'])
        member = raw['contract_evidence']['load_reference_states'][0]['members'][0]
        member['thermal_strain'] = member['thermal_strain'] + 1e-3
        fixture.cases.clear()
        fixture.add_case('connected', raw)
        fixture.set_output('connected', wrapper(raw, runner_input('connected')))
        result, _ = fixture.run()
        case = result['cases'][0]
        self.assertEqual({r['state'] for r in case['assertions']}, {'matched'})
        reader = check(case, 'owned_reader_consistency')
        self.assertEqual(reader['state'], 'failed')
        self.assertTrue(reader['details']['reason'].startswith('SOURCE_LOAD_REFERENCE_'))
        self.assertEqual(case['state'], 'failed')

    def test_helper_isolated_and_bound(self):
        fixture = Fixture(self, families=('pressure',))
        binding_path = fixture.root / 'binding.json'
        binding_path.write_bytes(dump(fixture.binding))
        digest = sha256_bytes(binding_path.read_bytes())
        raw = (RAWS / 'pressure-dense_scrutiny.raw.json').read_bytes()
        (fixture.root / 'json.py').write_text('raise RuntimeError("shadow json executed")')
        (fixture.root / 'load_reference_evidence.py').write_text('raise RuntimeError("shadow reader executed")')
        def helper(isolated=True, source=None, binding_digest=digest, data=raw):
            command = [sys.executable, *(['-I', '-S'] if isolated else []), str(lr.HELPER), '--source-root', str(source or fixture.candidate),
                       '--binding', str(binding_path), '--binding-sha256', binding_digest]
            # Shadow modules sit in PYTHONPATH and the cwd; only the isolated helper must ignore them.
            env = dict(os.environ, PYTHONPATH=str(fixture.root)) if isolated else {k: v for k, v in os.environ.items() if k != 'PYTHONPATH'}
            return subprocess.run(command, input=data, cwd=fixture.root, env=env, capture_output=True, timeout=60)
        result = helper()
        self.assertEqual(result.returncode, 0, result.stderr)
        reply = json.loads(result.stdout)
        self.assertEqual((reply['verdict'], reply['entrypoint'], reply['isolation']['unit_authority_execution']),
                         ('consistent', lr.ENTRYPOINT, False))
        self.assertEqual(json.loads(helper(isolated=False).stdout)['verdict'], 'refused')
        self.assertIn('binding digest mismatch', json.loads(helper(binding_digest='0' * 64).stdout)['reason'])
        for name in lr.DEPENDENCIES:
            path = fixture.candidate / name
            before = path.read_bytes()
            path.write_bytes(before + b'\n')
            self.assertIn('dependency digest mismatch', json.loads(helper().stdout)['reason'])
            path.write_bytes(before)
        foreign = json.loads(raw)
        foreign['producer']['semantic_contract_id'] = 'openpipestress.result_semantics/0.3.0/load-reference-source-1'
        self.assertEqual(json.loads(helper(data=json.dumps(foreign).encode()).stdout)['verdict'], 'refused')


class AdmissionTests(unittest.TestCase):
    def test_manifest_refusals_before_any_ledger(self):
        fixture = Fixture(self, families=('pressure',))
        case = fixture.cases[0]
        for manifest, message in (
                ({'format': lr.MANIFEST_FORMAT, 'cases': []}, 'nonzero bounded case inventory'),
                ({'format': lr.MANIFEST_FORMAT, 'cases': [case, deepcopy(case)]}, 'duplicate case ID'),
                ({'format': 'openpipestress.qualification_selection/1', 'cases': [case]}, 'unsupported case manifest version'),
                ({'format': lr.MANIFEST_FORMAT, 'cases': [case], 'extra': True}, 'closed case manifest required'),
                ({'format': lr.MANIFEST_FORMAT, 'cases': [dict(case, modes=[])]}, 'modes must be'),
                ({'format': lr.MANIFEST_FORMAT, 'cases': [dict(case, modes=['dense_fallback'])]}, 'modes must be'),
                ({'format': lr.MANIFEST_FORMAT, 'cases': [dict(case, required_scalar_rows=0)]}, 'positive required_scalar_rows'),
                ({'format': lr.MANIFEST_FORMAT, 'cases': [dict(case, unknown=1)]}, 'unknown/missing manifest case fields'),
                ({'format': lr.MANIFEST_FORMAT, 'cases': [dict(case, modes=['dense_scrutiny'])]}, 'no manifest case is required')):
            with self.assertRaisesRegex(ValueError, message):
                fixture.run(fixture.selection(manifest))

    def test_locked_manifest_hash_and_run_fields(self):
        fixture = Fixture(self, families=('pressure',))
        selection = fixture.selection()
        run = json.loads(selection.read_text())
        variants = [dict(run, case_manifest=dict(run['case_manifest'], sha256='0' * 64)), dict(run, transport='ordinary_physics_1_cli_1.0_raw0.2'),
                    dict(run, purpose='release_qualification'), dict(run, runner=dict(run['runner'], solver_mode='dense_fallback_after_sparse_failure')),
                    dict(run, extra=None), dict(run, runner=dict(run['runner'], explicit_local_private_intent='yes'))]
        for index, variant in enumerate(variants):
            path = fixture.root / f'variant-{index}.json'
            path.write_bytes(dump(variant))
            with self.assertRaises(ValueError):
                lr.run_selection(path, fixture.executable, fixture.candidate, fixture.root / f'variant-run-{index}')

    def blocked(self, fixture, selection=None, *, contains=None):
        result, _ = fixture.run(selection)
        for case in result['cases']:
            self.assertEqual(case['state'], 'blocked', case['reason'])
            self.assertIsNone(case['process'])
            self.assertEqual({r['state'] for r in case['assertions']}, {'blocked'})
            if contains:
                self.assertIn(contains, case['reason'])
        self.assertEqual(result['outcome'], 'not_satisfied')
        return result

    def test_selector_inventory_refusal_keeps_manifest_denominator(self):
        fixture = Fixture(self, families=('pressure',))
        selectors, reference, criteria = synthetic_package('synthetic-pressure', fixture.raws['pressure'])
        mutations = [(lambda s: s['negative_assertions'][0].update(id=s['assertions'][0]['id']), 'duplicate assertion ID'),
                     (lambda s: s['assertions'].append(dict(deepcopy(s['assertions'][0]), id='assert:duplicate-selector')), 'duplicate required selector'),
                     (lambda s: s.update(case_id='synthetic-connected'), 'selector identity/version mismatch'),
                     (lambda s: s.update(producer_contract='openpipestress.result_semantics/0.3.0/physics-1'), 'another producer contract'),
                     (lambda s: s.update(unknown_field=1), 'unknown/missing selector file fields'),
                     (lambda s: s['assertions'][0].update(extra=1), 'unknown/missing assertion fields'),
                     (lambda s: s['assertions'].pop(), 'differs from required_scalar_rows')]
        for mutate, message in mutations:
            fixture.cases.clear()
            changed = deepcopy(selectors)
            mutate(changed)
            fixture.add_case('pressure', fixture.raws['pressure'], package=(changed, reference, criteria))
            fixture.cases[0]['required_scalar_rows'] = len(selectors['assertions'])
            result = self.blocked(fixture, contains='selector inventory not admitted')
            self.assertIn(message, result['cases'][0]['reason'])
            self.assertEqual(len(result['cases'][0]['assertions']), len(selectors['assertions']))
            self.assertEqual(result['cases'][0]['inventory_basis'], 'required_scalar_rows_placeholder')

    def negatives_package(self, fixture, extra):
        """Synthetic package plus extra negatives: (id, selector source, wrong value)."""
        selectors, reference, criteria = synthetic_package('synthetic-connected', fixture.raws['connected'])
        by_id = {row['id']: row for row in selectors['assertions'] + selectors['negative_assertions']}
        for aid, source, value in extra:
            selectors['negative_assertions'].append(dict(deepcopy(by_id[source]), id=aid))
            reference['wrong_values'].append({'assertion_id': aid, 'value': value, 'unit': by_id[source]['selector']['unit'],
                                              'discriminator': 'synthetic wrong-result control'})
        fixture.cases.clear()
        fixture.add_case('connected', fixture.raws['connected'], package=(selectors, reference, criteria))
        return selectors

    def test_several_wrong_values_per_selector_are_admitted(self):
        fixture = Fixture(self, families=('connected',))
        selectors, _, _ = synthetic_package('synthetic-connected', fixture.raws['connected'])
        evidence = next(row for row in selectors['assertions'] if row['selector'].get('field') == 'total_eigenstrain')
        extra = [('negative:second-wrong', 'negative:omitted-coupling', 12345.0),
                 ('negative:strain-a', evidence['id'], 0.0016), ('negative:strain-b', evidence['id'], 0.00172)]
        self.negatives_package(fixture, extra)
        result, _ = fixture.run()
        self.assertEqual(result['outcome'], 'all_required_assertions_matched', json.dumps(result['summary']))
        self.assertEqual(result['summary']['assertions_by_polarity']['negative']['matched'], 4)
        self.assertEqual(assertion(result['cases'][0], 'negative:strain-b')['reason'], 'wrong_result_excluded')

    def test_duplicate_negative_selector_value_pair_is_refused(self):
        selectors, _, _ = synthetic_package('synthetic-connected', raw_for('connected', 'sparse_interactive'))
        evidence = next(row for row in selectors['assertions'] if row['selector'].get('field') == 'total_eigenstrain')
        for extra in ([('negative:same-pair', 'negative:omitted-coupling', 0.0)],
                      [('negative:signed-zero', 'negative:omitted-coupling', -0.0)],
                      [('negative:strain-a', evidence['id'], 0.0016), ('negative:strain-a-again', evidence['id'], 0.0016)]):
            fixture = Fixture(self, families=('connected',))
            package = self.negatives_package(fixture, extra)
            if extra[-1][0] == 'negative:strain-a-again':
                # Same quantity under another selector ID is still the same pair.
                package['negative_assertions'][-1]['selector']['id'] = 'evidence:renamed'
                fixture.cases.clear()
                _, reference, criteria = synthetic_package('synthetic-connected', fixture.raws['connected'])
                reference['wrong_values'] += [{'assertion_id': aid, 'value': value, 'unit': '1', 'discriminator': 'synthetic'} for aid, _, value in extra]
                fixture.add_case('connected', fixture.raws['connected'], package=(package, reference, criteria))
            result = self.blocked(fixture, contains='duplicate negative (selector, wrong value) pair')
            case = result['cases'][0]
            self.assertEqual(case['inventory_basis'], 'selectors')
            self.assertEqual(len(case['assertions']), fixture.cases[0]['required_scalar_rows'] + 1 + len(extra))

    def test_case_binding_refusals_block(self):
        cases = [
            ('runner input preview model differs', lambda f, c: f.write(c['product_request']['path'], b'{"model": {}, "materials": []}\n')),
            ('bound file digest mismatch', lambda f, c: (f.candidate / c['reference']['path']).write_bytes(b'{}')),
            ('analytical case key not in reference file', lambda f, c: c['analytical_reference'].update(case_key='not_a_reference_case')),
            ('WORKING_ROOT-relative', lambda f, c: c['analytical_reference'].update(path='../reference_cases.json')),
            ('unselected analytical reference path', lambda f, c: c['analytical_reference'].update(path=c['criteria']['path'])),
            ('WORKING_ROOT-relative', lambda f, c: c['criteria'].update(path=str(f.candidate / c['criteria']['path']))),
        ]
        for label, mutate in cases:
            fixture = Fixture(self, families=('pressure',))
            case = fixture.cases[0]
            ref = mutate(fixture, case)
            if isinstance(ref, dict):
                case['product_request'] = ref
            self.blocked(fixture, contains=label)

    def test_unready_reference_criteria_and_synthetic_comparison_block(self):
        fixture = Fixture(self, families=('pressure',))
        selectors, reference, criteria = synthetic_package('synthetic-pressure', fixture.raws['pressure'])
        draft = {'schema_version': '0.1.0', 'tolerance_profile': dict(criteria['tolerance_profile'], profile_status='draft_pending_independent_review')}
        reviewed_reference = dict(reference, reference_kind='analytical', independent_review_ref=None)
        for changed_reference, changed_criteria, purpose, message in [
                (dict(reference, readiness='pending_independent_review'), criteria, 'harness_development', 'reference not ready'),
                (reference, draft, 'harness_development', 'criterion profile not reviewed'),
                (reference, criteria, 'development_comparison', 'synthetic target cannot score'),
                (reviewed_reference, criteria, 'development_comparison', 'independent reference admission missing'),
                (dict(reference, values=reference['values'][1:]), criteria, 'harness_development', 'reference values denominator mismatch'),
                (dict(reference, wrong_values=[]), criteria, 'harness_development', 'reference wrong_values denominator mismatch'),
                (dict(reference, values=[dict(reference['values'][0], value=None)] + reference['values'][1:]), criteria, 'harness_development', 'numeric value required'),
                (dict(reference, values=[dict(reference['values'][0], unit='furlong')] + reference['values'][1:]), criteria, 'harness_development', 'reference unit mismatch'),
                # REVIEW_B X05: the rule must be for the selected quantity's family.
                (reference, {'schema_version': '0.1.0', 'tolerance_profile': dict(criteria['tolerance_profile'], rules=[
                    dict(rule, result_family='displacement' if rule['result_family'] != 'displacement' else 'rotation')
                    for rule in criteria['tolerance_profile']['rules']])}, 'harness_development', 'criterion result family mismatch')]:
            fixture.cases.clear()
            fixture.add_case('pressure', fixture.raws['pressure'], package=(selectors, changed_reference, changed_criteria))
            self.blocked(fixture, fixture.selection(purpose=purpose), contains=message)

    def test_semantic_selector_refusals_block(self):
        fixture = Fixture(self, families=('pressure',))
        selectors, reference, criteria = synthetic_package('synthetic-pressure', fixture.raws['pressure'])
        row_index = next(i for i, a in enumerate(selectors['assertions']) if 'namespace' not in a['selector'] and a['selector']['metadata'])
        evidence_index = next(i for i, a in enumerate(selectors['assertions']) if a['selector'].get('record') == 'member')
        mutations = [
            (lambda s: s['assertions'][row_index]['selector'].update(dimension='angle'), 'contradicts pinned load-reference-1 semantics'),
            (lambda s: s['assertions'][row_index]['selector'].update(kind='invented_kind'), 'contradicts pinned load-reference-1 semantics'),
            (lambda s: s['assertions'][row_index]['selector']['metadata'].update(location='TBD'), 'unresolved source metadata'),
            (lambda s: s['assertions'][row_index]['selector'].pop('dimension'), 'complete explicit row selector required'),
            (lambda s: s['assertions'][row_index]['selector'].update(basis_ref={'ref_type': 'combination', 'ref_id': 'combo'}), 'bind a load case'),
            (lambda s: s['assertions'][row_index].update(criterion_rule_id='rule:absent'), 'criterion rule missing'),
            (lambda s: s['assertions'][evidence_index]['selector'].update(field='material_id'), 'field outside the closed evidence vocabulary'),
            (lambda s: s['assertions'][evidence_index]['selector'].update(unit='Pa'), 'evidence unit/dimension differs'),
            (lambda s: s['assertions'][evidence_index]['selector'].update(record='excluded_source'), 'unsupported load_reference_states record'),
            (lambda s: s['assertions'][evidence_index]['selector']['definition'].update(provenance='x'), 'unknown definition field'),
            (lambda s: s['assertions'][evidence_index]['selector'].update(namespace='contract_evidence.exact_cases'), 'unsupported evidence namespace'),
            (lambda s: s['assertions'][evidence_index]['selector'].update(key={'pipe_id': 'pipe:first', 'extra': 'x'}), 'exact record key required'),
            (lambda s: s['assertions'][evidence_index]['selector']['basis_ref'].update(ref_id='case:not-in-input'), 'not a load case of the bound input'),
        ]
        for mutate, message in mutations:
            changed = deepcopy(selectors)
            mutate(changed)
            fixture.cases.clear()
            fixture.add_case('pressure', fixture.raws['pressure'], package=(changed, reference, criteria))
            self.blocked(fixture, contains=message)

    def test_run_level_identity_refusals_block_every_case(self):
        fixture = Fixture(self)
        selection = fixture.selection()
        run = json.loads(selection.read_text())
        wrong_exe = dict(run, runner=dict(run['runner'], executable_sha256='1' * 64))
        wrong_commit = dict(run, runner=dict(run['runner'], candidate_commit='2' * 40))
        for index, (variant, message) in enumerate(((wrong_exe, 'executable digest mismatch'), (wrong_commit, 'candidate revision mismatch'))):
            path = fixture.root / f'identity-{index}.json'
            path.write_bytes(dump(variant))
            self.blocked(fixture, path, contains=message)
        fixture.binding = dict(fixture.binding, status='prepared_not_admitted')
        self.blocked(fixture, contains='not admitted')
        fixture.binding = dict(fixture.binding, status='reviewed_candidate', files=[dict(row, sha256='3' * 64) if row['path'] == lr.MODULE else row
                                                                                  for row in fixture.binding['files']])
        self.blocked(fixture, contains='unselected owned reader dependency')


class ProcessTests(unittest.TestCase):
    def test_nonzero_timeout_output_limit_and_no_output(self):
        for code, kwargs, outcome in [
                ('import sys\nsys.stdin.buffer.read()\nsys.exit(3)\n', {}, 'process nonzero_exit'),
                ('import sys, time\nsys.stdin.buffer.read()\ntime.sleep(10)\n', {'timeout_seconds': 0.5}, 'process timeout'),
                ('import sys\nsys.stdin.buffer.read()\nsys.stdout.write("x" * 5000)\n', {'output_limit_bytes': 1024}, 'process output_limit')]:
            fixture = Fixture(self, families=('pressure',))
            fixture.write_process(code)
            result, _ = fixture.run(**kwargs)
            case = result['cases'][0]
            self.assertEqual((case['state'], case['reason']), ('error', outcome))
            self.assertEqual({r['state'] for r in case['assertions']}, {'error'})
            self.assertEqual(len(case['structural_checks']), len(lr.REQUIRED_STRUCTURAL_CHECKS))
            self.assertEqual(result['outcome'], 'not_satisfied')

    def test_interruption_leaves_later_cases_not_run(self):
        fixture = Fixture(self)
        original = capture
        def interrupted(*args, **kwargs):
            if args[1][:1] == ['solve']:
                result = original(*args, **kwargs)
                result['outcome'] = 'interrupted'
                return result
            return original(*args, **kwargs)
        with patch.object(lr, 'capture', interrupted):
            result, _ = fixture.run()
        first, second = result['cases']
        self.assertEqual(first['state'], 'error')
        self.assertEqual(second['state'], 'not_run')
        self.assertEqual({r['state'] for r in second['assertions']}, {'not_run'})
        self.assertIn('supervisor interrupted; subsequent cases remain not_run', result['diagnostics'])
        self.assertEqual(result['outcome'], 'not_satisfied')

    def test_keyboard_interrupt_after_capture_keeps_ledger(self):
        fixture = Fixture(self)
        with patch.object(lr, 'unwrap', side_effect=KeyboardInterrupt):
            result, output = fixture.run()
        self.assertEqual([case['state'] for case in result['cases']], ['error', 'not_run'])
        self.assertEqual(json.loads((output / 'ledger.json').read_text())['outcome'], 'not_satisfied')

    def test_bound_file_changed_during_run_is_error(self):
        fixture = Fixture(self, families=('pressure',))
        path = fixture.candidate / fixture.cases[0]['reference']['path']
        before = path.read_bytes()
        original, original_publish = capture, lr.publish
        def tamper(*args, **kwargs):
            result = original(*args, **kwargs)
            if args[1][:1] == ['solve']:
                path.write_bytes(before + b' ')
            return result
        def restore_then_publish(*args):
            # Restored before the publication custody observation, so only the
            # during-run recheck can have seen the change.
            path.write_bytes(before)
            return original_publish(*args)
        with patch.object(lr, 'capture', tamper), patch.object(lr, 'publish', restore_then_publish):
            result, _ = fixture.run()
        self.assertEqual(result['cases'][0]['state'], 'error')
        self.assertIn('changed during run', result['cases'][0]['reason'])


MIB = 1024 * 1024


def inflated(raw, extra_bytes):
    """A valid raw whose numerical-integrity diagnostic text is padded, as real large outputs are."""
    raw = deepcopy(raw)
    diagnostic = next(d for d in raw['diagnostics'] if d['code'] == 'NUMERICAL_INTEGRITY_CHECKS_PASSED')
    diagnostic['message'] += ' synthetic padding ' + 'x' * extra_bytes
    return raw


class OutputLimitTests(unittest.TestCase):
    """Runner stdout is admitted up to the selected output limit, not the 8 MiB input limit."""

    def large_fixture(self, extra_bytes):
        fixture = Fixture(self, families=('connected',))
        fixture.set_output('connected', wrapper(inflated(fixture.raws['connected'], extra_bytes), runner_input('connected')))
        size = Path(fixture.outputs[runner_input('connected')['request']['request_id']]).stat().st_size
        return fixture, size

    def test_stdout_above_gate_limit_admitted_within_selected_limit(self):
        fixture, size = self.large_fixture(9 * MIB)
        self.assertGreater(size, lr.LIMIT)
        result, _ = fixture.run(output_limit_bytes=16 * MIB)
        case = result['cases'][0]
        self.assertEqual(result['outcome'], 'all_required_assertions_matched', case['reason'])
        self.assertEqual(case['process']['stdout_bytes'], size)
        self.assertEqual(case['reader_consistency']['response']['verdict'], 'consistent')
        reader_input = next(a for a in case['retained_artifacts'] if a['role'] == 'reader_input')
        self.assertGreater(reader_input['byte_length'], lr.LIMIT)
        self.assertEqual(reader_input['byte_limit'], 16 * MIB)
        self.assertEqual(case['custody_at_publication'], 'checked')

    def test_stdout_above_selected_limit_refused_with_denominator_kept(self):
        fixture, size = self.large_fixture(9 * MIB)
        result, _ = fixture.run(output_limit_bytes=size - 1)
        case = result['cases'][0]
        self.assertEqual((case['state'], case['reason']), ('error', 'process output_limit'))
        self.assertEqual({r['state'] for r in case['assertions']}, {'error'})
        self.assertEqual(len(case['assertions']), fixture.cases[0]['required_scalar_rows'] + 1)
        self.assertEqual(result['outcome'], 'not_satisfied')

    def test_unwrap_and_parser_enforce_the_selected_limit(self):
        raw = inflated(raw_for('connected', 'sparse_interactive'), 9 * MIB)
        data = json.dumps(wrapper(raw, runner_input('connected'))).encode()
        request = runner_input('connected')
        for limit in (lr.LIMIT, len(data) - 1):
            with self.assertRaisesRegex(ValueError, 'JSON byte limit exceeded'):
                lr.unwrap(data, request, 'sparse_interactive', limit=limit)
        self.assertEqual(lr.unwrap(data, request, 'sparse_interactive', limit=len(data))[0]['model_ref'], raw['model_ref'])
        for limit in (0, lr.MAX_OUTPUT_LIMIT + 1, True):
            with self.assertRaisesRegex(ValueError, 'invalid JSON byte limit'):
                lr.strict_json_limited(b'{}', limit)

    def test_large_parse_keeps_the_gate_strict_rules(self):
        pad = b' ' * (lr.LIMIT + 1)
        refused = [b'{"a": NaN}', b'{"a": Infinity}', b'{"a": -Infinity}', b'{"a": 1e999}', b'{"a": 1e-400}',
                   b'{"a": 1, "a": 2}', b'{"a": "\xff"}', b'{"a": ', b'\xff{}']
        accepted = [(b'{"a": 0e5}', {'a': 0.0}), (b'{"a": 1e-300}', {'a': 1e-300}), (b'{"a": [1, "x", null, true]}', {'a': [1, 'x', None, True]})]
        for doc in refused:
            with self.assertRaises(ValueError):
                lr.gate.strict_json(doc)
            with self.assertRaises(ValueError, msg=doc):
                lr.strict_json_limited(doc + pad, lr.MAX_OUTPUT_LIMIT)
        for doc, value in accepted:
            self.assertEqual(lr.gate.strict_json(doc), value)
            self.assertEqual(lr.strict_json_limited(doc + pad, lr.MAX_OUTPUT_LIMIT), value)

    def test_reader_snapshot_bounded_by_selected_limit_before_helper(self):
        raw = raw_for('pressure', 'sparse_interactive')
        with tempfile.TemporaryDirectory() as temp:
            for limit, message in ((1024, 'reader input limit'), (lr.MAX_OUTPUT_LIMIT + 1, 'reader input limit')):
                with self.assertRaisesRegex(ValueError, message):
                    lr.consistency_observation(raw, {}, Path(temp) / 'never-created', input_limit=limit)
            self.assertFalse((Path(temp) / 'never-created').exists())

    def test_helper_source_limit_is_selected_and_bounded(self):
        fixture = Fixture(self, families=('connected',))
        binding_path = fixture.root / 'binding.json'
        binding_path.write_bytes(dump(fixture.binding))
        data = json.dumps(inflated(fixture.raws['connected'], 9 * MIB), separators=(',', ':')).encode()
        def helper(*extra):
            command = [sys.executable, '-I', '-S', str(lr.HELPER), '--source-root', str(fixture.candidate), '--binding', str(binding_path),
                       '--binding-sha256', sha256_bytes(binding_path.read_bytes()), *extra]
            return json.loads(subprocess.run(command, input=data, capture_output=True, timeout=120).stdout)
        self.assertEqual(helper('--source-limit-bytes', str(16 * MIB))['verdict'], 'consistent')
        self.assertIn('helper JSON byte limit', helper()['reason'])
        self.assertIn('helper JSON byte limit', helper('--source-limit-bytes', str(len(data) - 1))['reason'])
        self.assertIn('invalid source byte limit', helper('--source-limit-bytes', str(lr.MAX_OUTPUT_LIMIT + 1))['reason'])


class PureSemanticsTests(unittest.TestCase):
    def test_evidence_vocabulary_is_closed(self):
        base = {'id': 'e', 'namespace': lr.EVIDENCE_NAMESPACE, 'basis_ref': {'ref_type': 'load_case', 'ref_id': 'c'},
                'record': 'contribution', 'key': {'owner_kind': 'pressure_region', 'source_id': 'pressure_region:r'},
                'field': 'factor', 'definition': {}, 'unit': '1', 'dimension': 'dimensionless'}
        with self.assertRaises(ValueError):
            lr.evidence_semantics(base)
        support = dict(base, record='support_component', key={'support_id': 's', 'dof': 'RZ'}, field='prescribed_value', unit='m', dimension='length')
        with self.assertRaises(ValueError):
            lr.evidence_semantics(support)
        self.assertEqual(lr.evidence_semantics(dict(support, unit='rad', dimension='angle')), lr._MOTION)
        load = dict(base, key={'owner_kind': 'stored_primitive', 'source_id': 'load:x'}, field='applied_magnitude', unit='N*m', dimension='force')
        with self.assertRaises(ValueError):
            lr.evidence_semantics(load)
        self.assertEqual(lr.evidence_semantics(dict(load, dimension='moment')), lr._LOAD)
        motion = dict(base, key={'owner_kind': 'support_state', 'source_id': 'support_state:support:root:UX'}, field='value', unit='m', dimension='length')
        self.assertEqual(lr.evidence_semantics(motion), lr._MOTION)
        with self.assertRaises(ValueError):
            lr.evidence_semantics(dict(motion, key={'owner_kind': 'support_state', 'source_id': 'support_state:support:root:QQ'}))
        with self.assertRaises(ValueError):
            lr.evidence_semantics(dict(base, record='support_component', key={'support_id': 's', 'dof': 'RZ'}, field='prescribed_value',
                                       unit='rad', dimension='angle', definition={'global_dof': True}))


if __name__ == '__main__':
    unittest.main()
