"""exact_pressure_1 VP-HARNESS adapter and package checks; not the DEC-025 gate run.

The fake runner replays committed producer envelopes inside a synthetic
ControlledExport wrapper: the T4-I7 L line (``U2-L-ANCH-PTW-K2``) on 0.3.0
(sparse) and 0.4.0 (dense) and the admitted kink (``U2-L-KINK-ANCH-P-K2``,
0.3.0, dense) from ``fixtures/results/pressure_v3_arc_reader_corpus.json``,
whose documents are the package's own generated requests. They are scored
against the generated package (analytical values from the frozen T4-I7
references), admitted here with a synthetic test-only stub; no product solver
or runner binary is executed. The owned reader genuinely runs in its isolated
helper.
"""
from __future__ import annotations
from copy import deepcopy
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

PROJECT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(PROJECT))
from tools.validation import qualification_exact_pressure as ep
from tools.validation import qualification_load_reference as lr
from tools.validation import qualification_physics as physics
from tools.validation import qualification_exact_pressure_helper as helper
from tools.validation.qualification_process import sha256_bytes

PACKAGE = 'validation/qualification/fixtures/exact_pressure'
sys.path.insert(0, str(PROJECT / PACKAGE))
import generate_package as generator  # noqa: E402

CORPUS = json.loads((PROJECT / 'fixtures/results/pressure_v3_arc_reader_corpus.json').read_text())
RAWS = {('U2-L-ANCH-PTW-K2@0.3.0', 'sparse_interactive'): CORPUS['cases'][0],
        ('U2-L-ANCH-PTW-K2@0.4.0', 'dense_scrutiny'): CORPUS['cases'][1],
        ('U2-L-KINK-ANCH-P-K2@0.3.0', 'dense_scrutiny'): CORPUS['cases'][2]}
ADMITTED = {'format': generator.ADMISSION_FORMAT, 'status': 'admitted',
            'independent_review': {'path': 'reviews/SYNTHETIC_TEST_ONLY.md', 'sha256': '0' * 64}, 'admitted_by': 'test fixture only'}
_GENERATED = {}


def generated(admitted=True) -> dict[str, bytes]:
    """The package in memory (generated once per admission state)."""
    if admitted not in _GENERATED:
        with patch.object(generator, 'admission', return_value=ADMITTED if admitted else None):
            _GENERATED[admitted] = generator.generate()
    return _GENERATED[admitted]


def manifest_cases(admitted=True) -> dict:
    data = generated(admitted)[f'{PACKAGE}/MANIFEST.json']
    return {case['case_id']: case for case in json.loads(data)['cases']}


def raw_for(case_id, mode):
    return deepcopy(RAWS[(case_id, mode)]['envelope'])


def wrapper(mechanics, request_id):
    return {'blocked': False, 'decisions': [], 'findings': [],
            'summary': {'blocking_count': 0, 'decision_count': 0, 'finding_count': 0, 'warning_count': 0},
            'payload': {'artifact': 'openpipestress.headless_runner_cli_output', 'schema_version': '1.0.0',
                        'command': 'solve', 'operation': 'solve', 'request_validation': {'diagnostics': []},
                        'result_validation': {'diagnostics': []}, 'diagnostics': [],
                        'runner_result': {'run_id': 'run:headless-preview:' + request_id,
                                          'job': {'state': 'COMPLETED'}, 'analysis_status': ['MECHANICS_SOLVED'], 'diagnostics': []},
                        'mechanics_envelope': mechanics}}


def dump(value):
    return (json.dumps(value, indent=1, allow_nan=False) + '\n').encode()


class Fixture:
    """A temporary git candidate holding the reader dependencies and the selected package cases."""

    def __init__(self, test, mode, case_ids, raw_mode=None):
        self.test, self.mode = test, mode
        temp = tempfile.TemporaryDirectory()
        test.addCleanup(temp.cleanup)
        self.root = Path(temp.name)
        self.candidate = self.root / 'candidate'
        for name in list(ep.DEPENDENCIES) + list(ep.ANALYTICAL_REFERENCES):
            target = self.candidate / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(PROJECT / name, target)
        files, cases = generated(), manifest_cases()
        self.cases = [deepcopy(cases[cid]) for cid in case_ids]
        for case in self.cases:
            for role in ('runner_input', 'product_request', 'reference', 'selectors', 'criteria'):
                self.write(case[role]['path'], files[case[role]['path']])
        review = self.write('reviews/SYNTHETIC_READER_REVIEW.md', b'Synthetic test-only review stub. No independent review is claimed.\n')
        self.binding = {'format': ep.BINDING_FORMAT, 'contract_id': ep.CONTRACT, 'entrypoint': ep.ENTRYPOINT, 'status': 'reviewed_candidate',
                        'review_basis': [review], 'files': [{'path': path, 'sha256': digest} for path, digest in ep.DEPENDENCIES.items()]}
        self.executable = self.root / 'fake-runner'
        self.outputs = {}
        for cid in case_ids:
            self.set_output(cid, raw_for(cid, raw_mode or mode))
        self.counter = 0

    def write(self, relative, data):
        path = self.candidate / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
        return {'path': relative, 'sha256': sha256_bytes(data)}

    def request_id(self, case_id):
        case = next(c for c in self.cases if c['case_id'] == case_id)
        return json.loads((self.candidate / case['runner_input']['path']).read_text())['request']['request_id']

    def selectors(self, index=0):
        return json.loads((self.candidate / self.cases[index]['selectors']['path']).read_text())

    def set_output(self, case_id, mechanics, *, text=None):
        request_id = self.request_id(case_id)
        path = self.root / f'output-{request_id}.json'
        path.write_text(text if text is not None else json.dumps(wrapper(mechanics, request_id), allow_nan=False))
        self.outputs[request_id] = str(path)
        self.executable.write_text('#!' + sys.executable + '\nimport json, sys\n'
                                   f'outputs = {self.outputs!r}\n'
                                   'request = json.loads(sys.stdin.buffer.read())\n'
                                   'sys.stdout.buffer.write(open(outputs[request["request"]["request_id"]], "rb").read())\n')
        self.executable.chmod(0o700)

    def commit(self):
        git = ['git', '-c', 'user.name=synthetic-test', '-c', 'user.email=synthetic@example.invalid', '-c', 'commit.gpgsign=false']
        if not (self.candidate / '.git').exists():
            subprocess.run(['git', 'init', '-q'], cwd=self.candidate, check=True)
        subprocess.run(git + ['add', '-A'], cwd=self.candidate, check=True)
        subprocess.run(git + ['commit', '-q', '--allow-empty', '-m', 'synthetic candidate'], cwd=self.candidate, check=True)
        return subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=self.candidate, text=True).strip()

    def selection(self, **changes):
        self.counter += 1
        manifest_ref = self.write(f'{PACKAGE}/MANIFEST.json', dump({'format': ep.MANIFEST_FORMAT, 'cases': self.cases}))
        binding_path = self.root / f'binding-{self.counter}.json'
        binding_path.write_bytes(dump(self.binding))
        run = {'format': ep.RUN_FORMAT, 'profile_id': 'synthetic-exact-pressure-harness', 'purpose': 'development_comparison',
               'transport': ep.TRANSPORT,
               'runner': {'candidate_commit': self.commit(), 'executable_sha256': sha256_bytes(self.executable.read_bytes()),
                          'solver_mode': self.mode, 'explicit_local_private_intent': True},
               'case_manifest': {'path': 'candidate/' + manifest_ref['path'], 'sha256': manifest_ref['sha256']},
               'reader_binding': {'path': binding_path.name, 'sha256': sha256_bytes(binding_path.read_bytes())}}
        run.update(changes)
        path = self.root / f'selection-{self.counter}.json'
        path.write_bytes(dump(run))
        return path

    def run(self):
        self.counter += 1
        output = self.root / f'run-{self.counter}'
        return ep.run_selection(self.selection(), self.executable, self.candidate, output, output_limit_bytes=16 * 1024 * 1024), output


def check(case, cid):
    return next(row for row in case['structural_checks'] if row['id'] == cid)


def mutated(test, mutate, case_id='U2-L-ANCH-PTW-K2@0.3.0', mode='sparse_interactive'):
    fixture = Fixture(test, mode, [case_id])
    raw = raw_for(case_id, mode)
    mutate(raw)
    fixture.set_output(case_id, raw)
    return fixture.run()[0]['cases'][0], fixture


def arc_row(raw, kind, location='midspan'):
    return next(r for r in raw['results'] if r['kind'] == kind and r['entity_ref'] == 'pipe:BEND'
                and r.get('metadata', {}).get('location') == location)


class PinTests(unittest.TestCase):
    def test_reader_pins_are_the_physics_and_load_reference_pins(self):
        self.assertEqual(ep.DEPENDENCIES, {physics.MODULE: physics.DEPENDENCIES[physics.MODULE], lr.MODULE: lr.MODULE_SHA256,
                                           ep.TABLE: ep.TABLE_SHA256, physics.UNITS: physics.DEPENDENCIES[physics.UNITS]})
        for path, digest in ep.DEPENDENCIES.items():
            self.assertEqual(sha256_bytes((PROJECT / path).read_bytes()), digest, path)
        table = json.loads((PROJECT / ep.TABLE).read_text())
        self.assertEqual((table['semantic_contract_id'], table['formulation_profile_id']), (ep.CONTRACT, ep.PROFILE))

    def test_helper_and_generator_share_the_adapter_identities(self):
        self.assertEqual((helper.FORMAT, helper.CONTRACT, helper.ENTRYPOINT), (ep.BINDING_FORMAT, ep.CONTRACT, ep.ENTRYPOINT))
        self.assertEqual(helper.PATHS, set(ep.DEPENDENCIES))
        self.assertEqual((generator.MANIFEST_FORMAT, generator.CONTRACT, generator.TABLE), (ep.MANIFEST_FORMAT, ep.CONTRACT, ep.TABLE))
        self.assertEqual(set(generator.MODES), set(ep.MODES))
        self.assertEqual(generator.TERMINAL_FAMILY, ep.TERMINAL_FAMILY)
        self.assertEqual({generator.I7, generator.I8}, set(ep.ANALYTICAL_REFERENCES))


class PackageTests(unittest.TestCase):
    def test_committed_manifest_is_current_and_binds_every_case_file(self):
        files = generated(admitted=False)
        manifest = f'{PACKAGE}/MANIFEST.json'
        self.assertEqual((PROJECT / manifest).read_bytes(), files[manifest], 'regenerate with generate_package.py --write')
        cases = json.loads(files[manifest])['cases']
        for case in cases:
            for role in ('runner_input', 'product_request', 'reference', 'selectors', 'criteria'):
                self.assertEqual(sha256_bytes(files[case[role]['path']]), case[role]['sha256'])
        self.assertEqual(len(files), 1 + 5 * len(cases))

    def test_inventory_every_valued_i7_case_and_i8_owner_on_both_versions_and_modes(self):
        cases = manifest_cases(admitted=False)
        i7 = json.loads((PROJECT / generator.I7).read_text())['cases']
        valued = {key for key, case in i7.items() if case['expected'] is not None}
        self.assertEqual(len(valued), 79)
        expected = {f'{key}@{v}' for key in valued for v in generator.VERSIONS}
        expected |= {f'{owner}@{v}' for owner in ('milltol_lame_membrane.free_transferring', 'milltol_lame_membrane.axially_restrained_transferring',
                                                  'tp_phys_pressure_halves', 'pressure_membrane_thin_wall_limit',
                                                  'v3_straight_twin_separate_closures') for v in generator.VERSIONS}
        self.assertEqual(set(cases), expected)
        self.assertTrue(all(case['modes'] == list(ep.MODES) for case in cases.values()))

    def test_unadmitted_package_is_refused_by_the_gate(self):
        files = generated(admitted=False)
        case = manifest_cases(admitted=False)['U2-L-ANCH-PTW-K2@0.3.0']
        reference = json.loads(files[case['reference']['path']])
        criteria = json.loads(files[case['criteria']['path']])
        self.assertEqual((reference['readiness'], reference['independent_review_ref']), ('pending_independent_review', None))
        self.assertEqual(criteria['tolerance_profile']['profile_status'], 'draft_pending_independent_review')
        fixture = Fixture(self, 'sparse_interactive', [])
        for role in ('runner_input', 'product_request', 'reference', 'selectors', 'criteria'):
            fixture.write(case[role]['path'], files[case[role]['path']])
        with self.assertRaisesRegex(ValueError, 'reference not ready'):
            ep.prepare_case(case, fixture.candidate.resolve(), 'development_comparison', json.loads((PROJECT / ep.TABLE).read_text())['rows'])

    def i8_value(self, case_id, kind, component, location=None):
        files, case = generated(), manifest_cases()[case_id]
        selectors = json.loads(files[case['selectors']['path']])['assertions']
        values = {v['assertion_id']: v['value'] for v in json.loads(files[case['reference']['path']])['values']}
        found = [a for a in selectors if a['selector'].get('kind') == kind and a['selector']['metadata']['component'] == component
                 and (location is None or a['selector']['metadata']['location'] == location)]
        return [(a, values[a['id']]) for a in found]

    def test_i8_symbolic_forms_and_the_pa_to_mpa_factor(self):
        reference = json.loads((PROJECT / generator.I8).read_text())['cases']['tp_phys_pressure_halves']['load_cases']['case:combined']['expected']
        maximum = [v for a, v in self.i8_value('tp_phys_pressure_halves@0.3.0', 'pipe_elastic_normal_stress_maximum_v2', 'maximum_absolute_normal_stress')
                   if a['selector']['entity_ref'] == 'pipe:A-B' and a['selector']['basis_ref']['ref_id'] == 'case:combined']
        self.assertEqual(maximum, [reference['A-B_maximum']['value']])
        magnitude = [v for a, v in self.i8_value('tp_phys_pressure_halves@0.3.0', 'support_reaction_force_magnitude_v2', 'force_magnitude')
                     if a['selector']['entity_ref'] == 'support:A' and a['selector']['basis_ref']['ref_id'] == 'case:combined']
        self.assertEqual(magnitude, [reference['A_force_magnitude']['value']])
        stress = [(a, v) for a, v in self.i8_value('tp_phys_pressure_halves@0.4.0', 'element_local_bending_normal_stress_z', 'bending_normal_stress_z', 'midspan')
                  if a['selector']['entity_ref'] == 'pipe:A-B' and a['selector']['basis_ref']['ref_id'] == 'case:combined']
        self.assertEqual(stress[0][0]['selector']['unit'], 'MPa')
        exact = generator.maintained(reference['A-B_midspan_bending_normal_stress_z'], generator.Decimal(
            json.loads((PROJECT / generator.I8).read_text())['numeric_representation']['pi_decimal']), 'test')
        self.assertEqual(stress[0][1], float(exact / 1000000))

    def test_i7_criterion_is_relative_with_the_group_floor_in_the_row_unit(self):
        files, case = generated(), manifest_cases()['U2-L-ANCH-PTW-K2@0.3.0']
        rules = {r['rule_id']: r for r in json.loads(files[case['criteria']['path']])['tolerance_profile']['rules']}
        scale = json.loads((PROJECT / generator.I7).read_text())['cases']['U2-L-ANCH-PTW-K2']['zero_scale']
        displacement = rules['criterion:displacement:length:mm:floor:displacement']
        self.assertEqual(displacement['relative_tolerance_value'], 1e-09)
        self.assertEqual(displacement['absolute_tolerance_value'], float(generator.Decimal('1e-9') * generator.Decimal(scale['displacement']['zero_scale']) * 1000))
        stress = rules['criterion:stress:stress:MPa:floor:bending_torsion_stress']
        self.assertEqual(stress['absolute_tolerance_value'], float(generator.Decimal('1e-9') * generator.Decimal(scale['bending_torsion_stress']['zero_scale']) / 1000000))

    def test_exact_forms_and_units_outside_the_closed_set_are_refused(self):
        pi = generator.Decimal('3.14159')
        with self.assertRaisesRegex(generator.GenerationError, 'unsupported symbolic form'):
            generator.maintained({'unit': 'N', 'exact': {'kind': 'symbolic', 'expression': 'exp(1/2)-1'}, 'evaluation': 'x',
                                  'decimal': '0.6', 'value': 0.6}, pi, 'test')
        with self.assertRaisesRegex(generator.GenerationError, 'no exact unit factor'):
            generator.convert(generator.Decimal(1), 'N', 'kN', 'test')

    def test_withheld_arc_rows_and_null_terminal_fields_are_declared_absences(self):
        files, case = generated(), manifest_cases()['U2-L-FREE-SEPD-K1@0.3.0']
        absences = json.loads(files[case['selectors']['path']])['absences']
        self.assertIn({'row': {'kind': 'pipe_lame_hoop_stress_v2', 'entity_ref': 'pipe:BEND'}}, absences)
        self.assertIn({'evidence': {'basis_ref': {'ref_type': 'load_case', 'ref_id': 'case:u2'}, 'record': 'terminal',
                                    'key': {'node_ref': 'node:A'}, 'field': 'remote_closure_support_reaction_global_n'}}, absences)


class CompleteRunTests(unittest.TestCase):
    def assert_complete(self, mode, case_ids):
        fixture = Fixture(self, mode, case_ids)
        result, output = fixture.run()
        self.assertEqual(result['outcome'], 'all_required_assertions_matched', json.dumps(result['summary']))
        self.assertEqual(result['summary']['required_cases'], len(case_ids))
        positive = sum(case['required_scalar_rows'] for case in fixture.cases)
        self.assertEqual(result['summary']['assertions_by_polarity']['positive']['matched'], positive)
        self.assertEqual(result['summary']['structural_checks']['matched'], len(case_ids) * len(ep.REQUIRED_STRUCTURAL_CHECKS))
        self.assertEqual(result['summary']['numerical_standing']['checks_passed'], len(case_ids))
        self.assertEqual(result['qualification'], 'not_established_by_this_harness')
        for case in result['cases']:
            self.assertEqual(case['reader_consistency']['response']['verdict'], 'consistent')
            self.assertEqual(case['process']['command'][1:], ['solve', '--input', '-', '--solver-mode', mode, '--explicit-local-private-intent'])
            self.assertTrue(any(r['id'].startswith('negative:') and r['state'] == 'matched' for r in case['assertions']))
        self.assertEqual(json.loads((output / 'ledger.json').read_text()), result)
        self.assertIn('Outcome: all_required_assertions_matched', (output / 'summary.md').read_text())
        return result

    def test_l_line_on_0_3_0_sparse(self):
        result = self.assert_complete('sparse_interactive', ['U2-L-ANCH-PTW-K2@0.3.0'])
        self.assertGreater(result['summary']['assertions_by_polarity']['negative']['matched'], 0)

    def test_l_line_on_0_4_0_and_the_kink_dense(self):
        result = self.assert_complete('dense_scrutiny', ['U2-L-ANCH-PTW-K2@0.4.0', 'U2-L-KINK-ANCH-P-K2@0.3.0'])
        records = check(result['cases'][0], 'pressure_record_binding')
        self.assertEqual(records['details']['records'], ['case:u2'])


class ValueFaultTests(unittest.TestCase):
    def test_wrong_value_and_sign_fail_without_losing_the_denominator(self):
        for factor in (1.000001, -1.0):
            case, fixture = mutated(self, lambda raw, f=factor: arc_row(raw, 'pipe_wall_axial_force_v2').update(
                value=arc_row(raw, 'pipe_wall_axial_force_v2')['value'] * f))
            failed = [r for r in case['assertions'] if r['state'] == 'failed']
            self.assertEqual([r['reason'] for r in failed], ['exceeds_tolerance_profile'])
            self.assertEqual(len(case['assertions']), fixture.cases[0]['required_scalar_rows'] + len(fixture.selectors()['negative_assertions']))

    def test_a_reproduced_wrong_result_fails_its_negative(self):
        fixture = Fixture(self, 'sparse_interactive', ['U2-L-ANCH-PTW-K2@0.3.0'])
        negative = fixture.selectors()['negative_assertions'][0]
        wrong = next(v for v in json.loads((fixture.candidate / fixture.cases[0]['reference']['path']).read_text())['wrong_values']
                     if v['assertion_id'] == negative['id'])
        raw = raw_for('U2-L-ANCH-PTW-K2@0.3.0', 'sparse_interactive')
        next(r for r in raw['results'] if r['id'] == negative['selector']['id'])['value'] = wrong['value']
        fixture.set_output('U2-L-ANCH-PTW-K2@0.3.0', raw)
        case = fixture.run()[0]['cases'][0]
        row = next(r for r in case['assertions'] if r['id'] == negative['id'])
        self.assertEqual((row['state'], row['reason']), ('failed', 'wrong_result_reproduced'))

    def test_missing_row_stays_unavailable_never_zero(self):
        case, _ = mutated(self, lambda raw: raw['results'].remove(arc_row(raw, 'pipe_effective_axial_force_v2')))
        errored = [r for r in case['assertions'] if r['state'] == 'error']
        self.assertTrue(errored and all(r['observed'] is None and 'unavailable' in r['reason'] for r in errored))
        self.assertEqual(check(case, 'complete_unique_selector_coverage')['state'], 'failed')

    def test_terminal_vector_value_and_closure_definition(self):
        case, _ = mutated(self, lambda raw: raw['contract_evidence']['pressure'][0]['terminals'][0]['closure_pressure_load_global_n'].__setitem__(0, 1.0))
        self.assertTrue(any(r['state'] == 'failed' for r in case['assertions']))
        case, _ = mutated(self, lambda raw: raw['contract_evidence']['pressure'][0]['terminals'][0].update(closure_transfer='separately_supported_or_compensated'))
        self.assertTrue(any(r['state'] == 'error' and 'definition differs' in r['reason'] for r in case['assertions']))

    def test_withheld_arc_row_published_fails_the_declared_absences(self):
        def publish_lame(raw):
            row = deepcopy(next(r for r in raw['results'] if r['kind'] == 'pipe_lame_hoop_stress_v2'))
            raw['results'].append(dict(row, id=row['id'] + ':arc', entity_ref='pipe:BEND'))
        case, _ = mutated(self, publish_lame)
        absences = check(case, 'declared_absences')
        self.assertEqual(absences['state'], 'failed')
        self.assertIn('declared absent row published', absences['reason'])
        self.assertEqual(case['state'], 'failed')

    def test_non_null_remote_reaction_on_a_transferring_terminal_fails(self):
        case, _ = mutated(self, lambda raw: raw['contract_evidence']['pressure'][0]['terminals'][0].update(
            remote_closure_support_reaction_global_n=[0.0, 0.0, 0.0]))
        self.assertIn('is not null', check(case, 'declared_absences')['reason'])

    def test_duplicate_signature_under_another_id_does_not_resolve_once(self):
        def shadow(raw):
            row = arc_row(raw, 'element_local_shear_force_y')
            raw['results'].append(dict(deepcopy(row), id=row['id'] + ':shadow'))
        case, _ = mutated(self, shadow)
        self.assertTrue(any('does not resolve exactly once' in r['reason'] for r in case['assertions']))


class BindingAndReaderTests(unittest.TestCase):
    def test_raw_cases_and_regions_must_equal_the_bound_input(self):
        def foreign_row(raw):
            row = deepcopy(next(r for r in raw['results'] if r['kind'] == 'global_nodal_displacement_z'))
            raw['results'].append(dict(row, id=row['id'] + ':foreign', basis_ref={'ref_type': 'load_case', 'ref_id': 'case:foreign'}))
        case, _ = mutated(self, foreign_row)
        self.assertIn('outside the bound input', check(case, 'actual_input_binding')['reason'])
        case, _ = mutated(self, lambda raw: raw['contract_evidence']['pressure'][0].update(region_id='region:substituted'))
        self.assertIn('published pressure regions differ', check(case, 'pressure_record_binding')['reason'])

    def test_0_4_0_record_mode_binds(self):
        def sparse(raw):
            raw['contract_evidence']['load_reference_states'][0]['solve'].update(requested_mode='sparse_interactive')
        case, _ = mutated(self, sparse, 'U2-L-ANCH-PTW-K2@0.4.0', 'dense_scrutiny')
        self.assertIn('mode/recovery method differs', check(case, 'pressure_record_binding')['reason'])

    def test_owned_reader_refusal_is_a_failed_obligation(self):
        def tangency(raw):
            raw['contract_evidence']['pressure'][0]['tangency_tolerance_rad'] = 2e-3
        case, _ = mutated(self, tangency)
        reader = check(case, 'owned_reader_consistency')
        self.assertEqual(reader['state'], 'failed')
        self.assertEqual(case['reader_consistency']['response']['verdict'], 'refused')


class TransportRefusalTests(unittest.TestCase):
    def refused(self, mutate, message, case_id='U2-L-ANCH-PTW-K2@0.3.0', mode='sparse_interactive', raw_mode=None):
        fixture = Fixture(self, mode, [case_id], raw_mode)
        raw = raw_for(case_id, raw_mode or mode)
        mutate(raw)
        fixture.set_output(case_id, raw)
        case = fixture.run()[0]['cases'][0]
        self.assertEqual(case['state'], 'error', case['reason'])
        self.assertIn(message, case['reason'])
        self.assertTrue(all(r['observed'] is None for r in case['assertions']))

    def test_other_contracts_profiles_namespaces_and_modes_refuse(self):
        changes = [
            (lambda raw: raw['producer'].update(semantic_contract_id='openpipestress.result_semantics/0.3.0/physics-1'), 'raw/producer identity'),
            (lambda raw: raw['producer'].update(semantic_contract_id='openpipestress.result_semantics/0.3.0/load-reference-1'), 'raw/producer identity'),
            (lambda raw: raw['formulation_basis'].update(profile_id='exact_straight_pressure_v2'), 'formulation basis invalid'),
            (lambda raw: raw.update(source_block_recovery=None), 'source/composite namespace refused'),
            (lambda raw: raw['contract_evidence'].update(load_reference_states=[]), 'evidence namespace for the document version'),
            (lambda raw: None, 'actual mode evidence mismatch'),
        ]
        for index, (change, message) in enumerate(changes):
            self.refused(change, message, mode='dense_scrutiny' if index == len(changes) - 1 else 'sparse_interactive',
                         raw_mode='sparse_interactive')
        self.refused(lambda raw: raw['contract_evidence'].pop('load_reference_states'), 'evidence namespace for the document version',
                     'U2-L-ANCH-PTW-K2@0.4.0', 'dense_scrutiny')


class AdmissionTests(unittest.TestCase):
    def test_closed_shapes_refuse(self):
        case = manifest_cases()['U2-L-ANCH-PTW-K2@0.3.0']
        with self.assertRaisesRegex(ValueError, 'unselected analytical reference path'):
            ep.admit_manifest(dump({'format': ep.MANIFEST_FORMAT, 'cases': [dict(case, analytical_reference=dict(
                case['analytical_reference'], path='core/product_physics/tests/fixtures/load_reference_states/reference_cases.json'))]}))
        selectors = json.loads(generated()[case['selectors']['path']])
        for absences, message in (([{'row': {'kind': 'x'}}], 'row absence'),
                                  ([{'evidence': {'basis_ref': {'ref_type': 'load_case', 'ref_id': 'case:u2'}, 'record': 'terminal',
                                                  'key': {'node_ref': 'node:A'}, 'field': 'p_pa'}}], 'terminal vocabulary')):
            with self.assertRaisesRegex(ValueError, message):
                ep.admit_selectors(dict(selectors, absences=absences), case)
        terminal = next(a['selector'] for a in selectors['assertions'] if 'namespace' in a['selector'])
        for change, message in ((dict(terminal, field='p_pa'), 'closed terminal vocabulary'), (dict(terminal, unit='kN'), 'unit/dimension'),
                                (dict(terminal, record='region'), 'unsupported pressure evidence record')):
            with self.assertRaisesRegex(ValueError, message):
                ep.evidence_semantics(change)


if __name__ == '__main__':
    unittest.main()
