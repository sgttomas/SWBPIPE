"""Synthetic structural API tests; these are not solver or engineering results."""
from __future__ import annotations

from copy import deepcopy
import json
from pathlib import Path
import sys
import unittest

PROJECT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(PROJECT))
from tools.validation import qualification_physics_structure as structure

BINDINGS = PROJECT / 'validation/qualification/fixtures/first_static'


def load(name):
    return json.loads((BINDINGS / name).read_text())


def synthetic_case(slug='axial', mode='sparse_interactive'):
    """Construct a test observation from authored reference/selector semantics.

    No product outputs supply any numeric target, and no process is invoked.
    Upstream flags here are stubs solely for testing this pure reader's contract.
    """
    request = load(slug + '.runner_input.candidate.json')
    selectors = load(slug + '.selectors.candidate.json')['assertions']
    values = {row['assertion_id']: row['value'] for row in load(slug + '.reference.candidate.json')['values']}
    criteria = load('STRUCTURAL_NUMERIC_ADDENDUM/STRUCTURAL_NUMERIC_BINDING.json')
    rows = []
    for assertion in selectors:
        row = deepcopy(assertion['selector'])
        row.pop('dimension')
        if row['metadata'] is None:
            row.pop('metadata')
        row['value'] = values[assertion['id']]
        rows.append(row)
    rows.append({'id': 'synthetic:mode', 'kind': 'linear_solver_mode_basis', 'unit': 'mode_code',
                 'value': 1 if mode == 'sparse_interactive' else 2,
                 'basis_ref': deepcopy(structure.CASE_BASIS)})
    maximum = values['stress.maximum_absolute_normal']
    section = {row['observed_field']: row['reference_value_binary64'] for row in criteria['quantities']}
    section.update(pipe_id='pipe', geometry_basis='authored_normalized_od_wall_v1')
    raw = {
        'schema_version': '0.2.0', 'producer': {'component_name': 'open_pipe_stress_product_physics',
            'component_version': '0.2.0', 'semantic_contract_id': structure.CONTRACT},
        'document_kind': 'openpipestress.product_preview.mechanics_result',
        'run_id': 'run:preview-linear-static-001', 'status': {'mechanics': 'MECHANICS_SOLVED'},
        'model_ref': request['solve']['preview_model']['model']['project']['id'],
        'accepted_model_state_mutated': False,
        'formulation_basis': {'profile_id': structure.PROFILE, 'limitations': ['synthetic test observation']},
        'contract_evidence': {'connector': [], 'pressure': [], 'exact_cases': [{
            'load_case_id': 'case', 'material_basis': 'base_material_common_E_nu',
            'profile_mode': structure.PROFILE,
            'pressure_rhs_assembly': {'load_case_id': 'case', 'groups': [], 'node_order': ['root', 'tip'],
                'assembled_pressure_rhs_global': [0.0] * 12, 'rounded_cap_rhs_global': [0.0] * 12,
                'rounded_poisson_rhs_global': [0.0] * 12},
            'pipe_materials': [{'E_pa': 200e9, 'G_pa': 80e9, 'nu': .25,
                'pipe_id': 'pipe', 'material_id': 'material', 'alpha_per_kelvin': None,
                'constitutive_basis': 'homogeneous_isotropic_E_nu_v1', 'thermal_consumed': False}],
            'pipe_sections': [section],
            'pipe_stress_extrema': [{'pipe_id': 'pipe', 'result_id': structure.MAXIMUM_ID,
                'approximation': 'piecewise_quadratic_straight_section_statics',
                'coefficient_basis': 'j_side_section_equilibrium_binary64', 'enclosure_scope': structure.ENCLOSURE,
                'value_lower_pa': maximum, 'value_upper_pa': maximum, 'global_upper_bound_pa': maximum,
                'certified_gap_pa': 0.0, 'station_fraction': 0.0, 'local_fraction': 0.0,
                'span_index': 0, 'subdivisions': 0}],
            'stress_maximum_coverage': {'complete': True, 'unavailable_pipe_ids': []}}]},
        'results': rows,
        'numerical_quality': {'value_representation': 'finite_binary64', 'publication_quantization': 'none',
            'integrity_policy': 'M03-INTEGRITY-v1', 'status': 'checks_passed', 'cases': [{
                'basis_ref': deepcopy(structure.CASE_BASIS), 'structural_status': 'passive_model_basis',
                'solve_quality': 'checks_passed', 'model_matrix_fidelity': 'represented_equations_retained',
                'accuracy_evidence': 'not_claimed', 'evidence_refs': ['synthetic:integrity']}]},
        'diagnostics': [{'id': 'synthetic:integrity', 'code': 'NUMERICAL_INTEGRITY_CHECKS_PASSED',
                         'severity': 'info', 'affected_refs': ['case']}],
        'summary': {'node_count': 2, 'segment_count': 1, 'support_count': 1, 'load_case_count': 1,
            'component_stress_modifier_count': 0, 'component_user_stiffness_macro_element_count': 0,
            'component_pressure_thrust_load_count': 0, 'spring_hanger_user_input_count': 0,
            'max_displacement': {'value': values['node.tip.magnitude'], 'unit': 'mm',
                'location_ref': 'tip', 'result_ref': 'result:disp:tip'},
            'max_open_formula_stress': {'value': maximum, 'unit': 'Pa', 'location_ref': 'pipe',
                'result_ref': structure.MAXIMUM_ID}},
    }
    binding = {'case_id': 'original_static_' + slug + '_v1', 'mode': mode,
               'transport_identity_ok': True, 'actual_input_binding_ok': True, 'ordinary_contract_ok': True}
    return dict(raw=raw, request=request, case_binding=binding, section_reference=load('section.reference.json'),
                section_criteria=criteria, scalar_coverage_ok=True, physical_consistency_ok=True)


class FirstStaticStructureTests(unittest.TestCase):
    def setUp(self):
        self.inputs = synthetic_case()

    def checks(self):
        rows = structure.check_first_static_structure(**self.inputs)
        self.assertEqual([row['id'] for row in rows], list(structure.CHECK_IDS))
        self.assertTrue(all(set(row) == {'id', 'state', 'reason', 'details'} for row in rows))
        return {row['id']: row for row in rows}

    def failed(self, check):
        self.assertEqual(self.checks()[check]['state'], 'failed')

    def exact_case(self):
        return self.inputs['raw']['contract_evidence']['exact_cases'][0]

    def extremum(self):
        return self.exact_case()['pipe_stress_extrema'][0]

    def test_all_four_synthetic_case_modes_and_no_mutation(self):
        for slug in ('axial', 'bending_torsion'):
            for mode in ('sparse_interactive', 'dense_scrutiny'):
                with self.subTest(slug=slug, mode=mode):
                    self.inputs = synthetic_case(slug, mode)
                    before = deepcopy(self.inputs)
                    checks = self.checks()
                    self.assertEqual({row['state'] for row in checks.values()}, {'matched'})
                    self.assertEqual(self.inputs, before)
                    subchecks = checks['complete_case_material_section']['details']['section_quantities']
                    self.assertEqual(len(subchecks), 9)
                    self.assertEqual({row['state'] for row in subchecks}, {'matched'})

    def test_binding_failure_is_not_observation_success(self):
        mutations = [
            lambda x: x['case_binding'].update(case_id='unknown'),
            lambda x: x['case_binding'].update(mode='dense_fallback'),
            lambda x: x['case_binding'].update(transport_identity_ok=1),
            lambda x: x.update(scalar_coverage_ok='true'),
            lambda x: x.update(physical_consistency_ok='true'),
            lambda x: x['section_criteria']['quantities'].pop(),
            lambda x: x['section_criteria']['quantities'].__setitem__(0, deepcopy(x['section_criteria']['quantities'][1])),
            lambda x: x['section_criteria']['quantities'][0].update(bound_input_pointer_for_exact_identity='/different'),
            lambda x: x['section_criteria']['quantities'][2].update(unit='mm'),
            lambda x: x['section_criteria']['criteria']['section.area_relative'].update(absolute_tolerance=1e-5),
            lambda x: x['section_criteria']['criteria']['section.area_relative'].update(relative_tolerance=float('nan')),
            lambda x: x['section_reference']['values'].update(ro_m='0.2'),
        ]
        for mutate in mutations:
            with self.subTest(mutation=mutate):
                inputs = synthetic_case()
                mutate(inputs)
                with self.assertRaises(ValueError):
                    structure.check_first_static_structure(**inputs)

    def test_each_upstream_flag_is_required_not_truthy(self):
        for flag, check in [('transport_identity_ok', 'transport_identity_and_mode'),
                            ('actual_input_binding_ok', 'actual_input_binding'),
                            ('ordinary_contract_ok', 'ordinary_physics_contract')]:
            with self.subTest(flag=flag):
                self.inputs = synthetic_case()
                self.inputs['case_binding'][flag] = False
                self.failed(check)

    def test_all_obligations_retained_for_missing_raw(self):
        self.inputs['raw'] = {}
        checks = self.checks()
        self.assertEqual({row['state'] for row in checks.values()}, {'failed'})
        self.assertEqual(len(checks['complete_case_material_section']['details']['section_quantities']), 9)

    def test_wrong_mode_and_fallback_fail(self):
        self.inputs['raw']['results'][-1]['value'] = 3
        self.failed('transport_identity_and_mode')

    def test_blocking_diagnostic_and_sensitive_standing(self):
        self.inputs['raw']['diagnostics'][0]['severity'] = 'error'
        self.failed('transport_identity_and_mode')
        self.inputs = synthetic_case()
        self.inputs['raw']['numerical_quality']['cases'][0]['solve_quality'] = 'sensitive'
        self.failed('finite_numerical_standing')

    def test_ordinary_contract_not_same_kind_fallback(self):
        for mutate in [lambda r: r['producer'].update(semantic_contract_id='other'),
                       lambda r: r.update(source_block_recovery={}),
                       lambda r: r['formulation_basis'].update(profile_id='legacy')]:
            self.inputs = synthetic_case()
            mutate(self.inputs['raw'])
            self.failed('ordinary_physics_contract')

    def test_explicit_input_units_loads_material_and_geometry(self):
        mutations = [
            lambda m: m['materials'][0]['elastic_modulus'].update(unit='MPa'),
            lambda m: m['materials'][0]['poisson_ratio'].update(value=.3),
            lambda m: m['materials'][0].update(density={'value': 1, 'unit': 'kg/m^3'}),
            lambda m: m['pipe_segments'][0]['section']['outside_diameter'].update(unit='mm'),
            lambda m: m['pipe_segments'][0]['section'].update(mill_tolerance=0),
            lambda m: m['nodes'][1]['position'].update(x=3),
            lambda m: m['load_cases'][0]['primitive_loads'][0]['magnitude'].update(value=1001),
            lambda m: m['load_cases'][0]['pressure_regions'].append({'id': 'extra'}),
            lambda m: m['components'].append({'id': 'extra'}),
            lambda m: m['load_cases'][0].update(temperature=1),
        ]
        for mutate in mutations:
            with self.subTest(mutation=mutate):
                self.inputs = synthetic_case()
                mutate(self.inputs['request']['solve']['preview_model']['model'])
                self.failed('complete_case_material_section')
                self.failed('actual_input_binding')

    def test_material_and_case_coverage(self):
        for key, value in [('E_pa', 199e9), ('nu', .3), ('G_pa', 76e9),
                           ('material_id', 'wrong'), ('pipe_id', 'wrong'), ('thermal_consumed', True)]:
            self.inputs = synthetic_case()
            self.exact_case()['pipe_materials'][0][key] = value
            self.failed('complete_case_material_section')
        for field in ('pipe_materials', 'pipe_sections'):
            self.inputs = synthetic_case()
            self.exact_case()[field].append(deepcopy(self.exact_case()[field][0]))
            self.failed('complete_case_material_section')
        self.inputs = synthetic_case()
        self.inputs['raw']['contract_evidence']['exact_cases'].append(deepcopy(self.exact_case()))
        self.failed('complete_case_material_section')

    def test_pressure_sources_or_rhs_not_hidden_by_empty_pressure_regions(self):
        for field in ('assembled_pressure_rhs_global', 'rounded_cap_rhs_global', 'rounded_poisson_rhs_global'):
            for bad in (1, True):
                self.inputs = synthetic_case()
                self.exact_case()['pressure_rhs_assembly'][field][0] = bad
                self.failed('complete_case_material_section')
        self.inputs = synthetic_case()
        self.exact_case()['pressure_rhs_assembly']['groups'].append({'synthetic': 'unexpected pressure source'})
        self.failed('complete_case_material_section')

    def test_each_required_section_value_missing_bad_type_or_outside_budget(self):
        for field, *_ in structure.SECTION_FIELDS:
            for value in (None, True, '0.1', float('nan'), float('inf'), -1):
                with self.subTest(field=field, value=value):
                    self.inputs = synthetic_case()
                    self.exact_case()['pipe_sections'][0][field] = value
                    check = self.checks()['complete_case_material_section']
                    self.assertEqual(check['state'], 'failed')
                    self.assertEqual(len(check['details']['section_quantities']), 9)
            self.inputs = synthetic_case()
            self.exact_case()['pipe_sections'][0][field] *= 1 + 2e-9
            self.failed('complete_case_material_section')

    def test_derived_rounding_allowed_but_source_identity_exact(self):
        self.exact_case()['pipe_sections'][0]['ri_m'] = .1 - .01
        self.exact_case()['pipe_sections'][0]['As_m2'] *= 1 + 5e-10
        self.assertEqual(self.checks()['complete_case_material_section']['state'], 'matched')
        self.exact_case()['pipe_sections'][0]['outside_diameter_m'] *= 1 + 5e-10
        self.failed('complete_case_material_section')

    def test_relative_criterion_boundary_and_missing_quantity(self):
        # Both sides of the declared 1e-9 rule, with enough binary64 separation
        # that the expected distinction does not depend on a last-bit tie.
        for factor, state in [(1 + .999999e-9, 'matched'), (1 + 1.000001e-9, 'failed')]:
            self.inputs = synthetic_case()
            self.exact_case()['pipe_sections'][0]['Ai_m2'] *= factor
            self.assertEqual(self.checks()['complete_case_material_section']['state'], state)
        self.inputs = synthetic_case()
        del self.exact_case()['pipe_sections'][0]['Ai_m2']
        check = self.checks()['complete_case_material_section']
        self.assertEqual(check['state'], 'failed')
        self.assertEqual(len(check['details']['section_quantities']), 9)

    def test_scalar_coverage_and_metadata_absence(self):
        self.inputs['scalar_coverage_ok'] = False
        self.failed('complete_unique_scalar_rows')
        for metadata in (None, {}):
            self.inputs = synthetic_case()
            row = next(row for row in self.inputs['raw']['results'] if row['id'] == 'result:disp:tip')
            row['metadata'] = metadata
            self.failed('complete_unique_scalar_rows')
        self.inputs = synthetic_case()
        self.inputs['raw']['results'].append(deepcopy(self.inputs['raw']['results'][0]))
        self.failed('complete_unique_scalar_rows')

    def test_additional_disclosure_does_not_change_73_denominator(self):
        self.inputs['raw']['results'].append({'id': 'synthetic:additional-disclosure',
            'kind': 'synthetic_disclosure', 'value': 0, 'unit': 'unitless'})
        check = self.checks()['complete_unique_scalar_rows']
        self.assertEqual(check['state'], 'matched')
        self.assertEqual(check['details']['required_scalar_rows'], 73)
        self.assertEqual(check['details']['observed_total_rows'], 75)

    def test_finite_and_actual_numerical_diagnostics(self):
        for value in (True, '0', float('inf')):
            self.inputs = synthetic_case()
            self.inputs['raw']['results'][0]['value'] = value
            self.failed('finite_numerical_standing')
        self.inputs = synthetic_case()
        self.inputs['raw']['diagnostics'] = []
        self.failed('finite_numerical_standing')
        self.inputs = synthetic_case()
        self.inputs['raw']['numerical_quality']['cases'][0]['accuracy_evidence'] = 'certified'
        self.failed('finite_numerical_standing')

    def test_maximum_bound_coverage_and_method_negatives(self):
        mutations = [
            lambda e: e.update(pipe_id='wrong'), lambda e: e.update(result_id='wrong'),
            lambda e: e.update(coefficient_basis='ideal_decimal'),
            lambda e: e.update(enclosure_scope='all_uncertainty'),
            lambda e: e.update(value_lower_pa=e['value_upper_pa'] + 1),
            lambda e: e.update(value_upper_pa=e['value_lower_pa'] - 1),
            lambda e: e.update(global_upper_bound_pa=e['value_upper_pa'] - 1),
            lambda e: e.update(certified_gap_pa=-1),
        ]
        for mutate in mutations:
            self.inputs = synthetic_case()
            mutate(self.extremum())
            self.failed('normal_maximum_evidence')
        self.inputs = synthetic_case()
        self.exact_case()['stress_maximum_coverage']['unavailable_pipe_ids'] = ['pipe']
        self.failed('normal_maximum_evidence')
        self.inputs = synthetic_case()
        self.exact_case()['stress_maximum_coverage']['complete'] = 1
        self.failed('normal_maximum_evidence')

    def test_physical_consistency_is_required_and_not_reimplemented(self):
        self.inputs['physical_consistency_ok'] = False
        self.failed('normal_maximum_evidence')
        self.inputs['physical_consistency_ok'] = None
        self.assertEqual(self.checks()['normal_maximum_evidence']['state'], 'blocked')
        # No invented epsilon rescues a rejected owned-checker gap verdict.
        self.inputs['physical_consistency_ok'] = False
        self.extremum()['certified_gap_pa'] = 1e-300
        self.failed('normal_maximum_evidence')

    def test_coefficient_enclosure_does_not_need_to_contain_decimal_reference(self):
        # Synthetic binary64-coefficient observation outside the ideal oracle.
        # Scalar accuracy is separately assessed by the caller's existing rule.
        row = next(row for row in self.inputs['raw']['results'] if row['id'] == structure.MAXIMUM_ID)
        row['value'] += 1e-7
        for field in ('value_lower_pa', 'value_upper_pa', 'global_upper_bound_pa'):
            self.extremum()[field] = row['value']
        self.inputs['raw']['summary']['max_open_formula_stress']['value'] = row['value']
        self.assertEqual(self.checks()['normal_maximum_evidence']['state'], 'matched')

    def test_axial_tie_and_strict_combined_location(self):
        self.extremum().update(station_fraction=.75, local_fraction=.75)
        check = self.checks()['governing_location']
        self.assertEqual(check['state'], 'matched')
        self.assertIn('no unique', check['details']['location_interpretation'])
        self.inputs = synthetic_case('bending_torsion')
        self.extremum().update(station_fraction=.75, local_fraction=.75)
        self.failed('governing_location')
        for field, value in [('station_fraction', -0.1), ('local_fraction', 1.1), ('span_index', True)]:
            self.inputs = synthetic_case()
            self.extremum()[field] = value
            self.failed('governing_location')

    def test_summaries_bind_exact_value_units_and_references(self):
        for key, check in [('max_open_formula_stress', 'stress_summary_binding'),
                           ('max_displacement', 'displacement_summary_binding')]:
            for field, value in [('value', 0), ('unit', 'wrong'), ('location_ref', 'wrong'), ('result_ref', 'wrong')]:
                self.inputs = synthetic_case()
                self.inputs['raw']['summary'][key][field] = value
                self.failed(check)
        self.inputs = synthetic_case()
        self.inputs['raw']['summary']['load_case_count'] = True
        self.failed('displacement_summary_binding')


if __name__ == '__main__':
    unittest.main()
