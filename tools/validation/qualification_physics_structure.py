"""Pure checks for the two selected first-static development cases only.

The caller supplies parsed, byte-bound inputs and private upstream check results.
Those booleans must be computed from captured transport/process/table evidence,
never copied from a selection manifest. This reader does no I/O, admission,
solver execution, unit conversion, or physical-evidence reimplementation.

T4-U6 (D-5; validation/qualification/GATE_USAGE.md): the frozen addendum and
structural expectations are read through the declared re-authored copy. The
nine section quantities are taken at their D-5 locations on coincident bases,
and the maximum, coverage and stress headline obligations hold on each basis.
"""
from __future__ import annotations

from decimal import Decimal, InvalidOperation
import math

from core.comparison.analysis_run.engine import _classify_delta


CHECK_IDS = (
    'transport_identity_and_mode', 'actual_input_binding', 'ordinary_physics_contract',
    'complete_case_material_section', 'finite_numerical_standing',
    'complete_unique_scalar_rows', 'normal_maximum_evidence', 'governing_location',
    'stress_summary_binding', 'displacement_summary_binding',
)
# The first-static cases are qualified as pressure-2 on the re-authored current (0.8.0) copy of their frozen input.
CONTRACT = 'openpipestress.result_semantics/0.3.0/pressure-2'
PROFILE = 'exact_pressure_v3'
MODEL_VERSION = '0.8.0'
PRESSURE_CONTRACT = {'version': '3.0.0', 'mode': PROFILE, 'shear_model': 'euler_bernoulli'}
ZERO_CORROSION_ALLOWANCE = {'value': 0, 'unit': 'm'}
CASE_BASIS = {'ref_type': 'load_case', 'ref_id': 'case'}
# T4-U6 (D-5; DEL-05-03 section-bases-and-timoshenko.md §2.2, §2.5): each section basis has its own maximum row,
# extremum, coverage record and headline; the basis-free headline is null on exact documents.
SECTION_BASES = ('nominal_section', 'reduced_section')
MAXIMUM_ID = 'result:elastic-maximum:4:case:4:pipe'
MAXIMUM_IDS = {basis: MAXIMUM_ID + ':' + basis for basis in SECTION_BASES}
HEADLINE_KEYS = {basis: 'max_open_formula_stress_' + basis for basis in SECTION_BASES}
ENCLOSURE = 'supplied_binary64_polynomial_coefficient_enclosures; solution error is separate'
FROZEN_SCALAR_ROWS, DERIVED_SCALAR_ROWS = 73, 94
D5_GEOMETRY_BASIS = 'authored_od_wall_d5_sections_v1'
D5_SECTION_KEYS = {'pipe_id', 'geometry_basis', 'outside_diameter_m', 'ro_m', 'wall_thickness_m', 'mill_tolerance_m',
                   'corrosion_allowance_m', 'nominal_section', 'reduced_section', 'pressure_bore', 'beam_theory'}
# The frozen addendum's nine flat quantities, read at their D-5 locations. The first location is compared with the
# addendum; the others are the same quantity on the other, coincident basis and must equal it (both first-static
# documents author zero mill tolerance and zero allowance, so the nominal and reduced sections coincide).
D5_LOCATIONS = {
    'outside_diameter_m': (('outside_diameter_m',),),
    'effective_wall_thickness_m': (('wall_thickness_m',), ('nominal_section', 'wall_m'), ('reduced_section', 'wall_m')),
    'ro_m': (('ro_m',),),
    'ri_m': (('nominal_section', 'ri_m'), ('reduced_section', 'ri_m'), ('pressure_bore', 'ri_m')),
    'As_m2': (('nominal_section', 'As_m2'), ('reduced_section', 'As_m2')),
    'Ai_m2': (('pressure_bore', 'Ai_m2'),),
    'I_m4': (('nominal_section', 'I_m4'), ('reduced_section', 'I_m4')),
    'J_m4': (('nominal_section', 'J_m4'), ('reduced_section', 'J_m4')),
    'Z_m3': (('nominal_section', 'Z_m3'), ('reduced_section', 'Z_m3')),
}
CASES = {
    'original_static_axial_v1': ('axial', [('global_x', 1000, 'force')]),
    'original_static_bending_torsion_v1': (
        'bending_torsion', [('global_y', 30, 'force'), ('global_z', -40, 'force'),
                            ('rotation_x', 20, 'moment')]),
}
# Field, dimension, unit, original independent section-reference key (if any).
SECTION_FIELDS = (
    ('outside_diameter_m', 'length', 'm', None),
    ('effective_wall_thickness_m', 'length', 'm', None),
    ('ro_m', 'length', 'm', 'ro_m'), ('ri_m', 'length', 'm', 'ri_m'),
    ('As_m2', 'area', 'm^2', 'A_m2'), ('Ai_m2', 'area', 'm^2', None),
    ('I_m4', 'second_moment_area', 'm^4', 'I_m4'),
    ('J_m4', 'second_moment_area', 'm^4', 'J_m4'),
    ('Z_m3', 'section_modulus', 'm^3', 'Z_m3'),
)
IDENTITY_POINTERS = {
    'outside_diameter_m': '/solve/preview_model/model/pipe_segments/0/section/outside_diameter/value',
    'effective_wall_thickness_m': '/solve/preview_model/model/pipe_segments/0/section/wall_thickness/value',
}


def _require(ok, reason):
    if not ok:
        raise ValueError(reason)


def _number(value):
    _require(type(value) in (int, float), 'finite numeric value required; no coercion')
    _require(math.isfinite(value), 'nonfinite numeric value')
    return value


def _same_number(value, expected, label):
    _require(_number(value) == expected, label + ' differs')


def _one(rows, label):
    _require(type(rows) is list and len(rows) == 1 and type(rows[0]) is dict,
             label + ' requires exactly one object')
    return rows[0]


def _exact_case(raw):
    case = _one(raw['contract_evidence']['exact_cases'], 'actual exact case')
    _require(case.get('load_case_id') == 'case', 'actual case ID differs')
    return case


def _extrema(raw):
    """The member's two per-basis extrema, nominal then reduced (§2.5), each bound to its own maximum row."""
    rows = _exact_case(raw)['pipe_stress_extrema']
    _require(type(rows) is list and len(rows) == len(SECTION_BASES) and all(type(row) is dict for row in rows),
             'pipe maximum evidence requires one object per section basis')
    for row, basis in zip(rows, SECTION_BASES):
        _require(row.get('pipe_id') == 'pipe' and row.get('section_basis') == basis and row.get('result_id') == MAXIMUM_IDS[basis],
                 'maximum member/basis/result binding differs')
    return dict(zip(SECTION_BASES, rows))


def _leaf(section, path):
    value = section
    for key in path:
        _require(type(value) is dict and key in value, 'D-5 section location missing: ' + '.'.join(path))
        value = value[key]
    return value


def _d5_section(section):
    """§2.5's pipe_sections entry with coincident bases: zero mill tolerance and allowance, Euler-Bernoulli."""
    _require(type(section) is dict and set(section) == D5_SECTION_KEYS, 'actual D-5 section shape differs')
    _require(section['pipe_id'] == 'pipe' and section['geometry_basis'] == D5_GEOMETRY_BASIS, 'actual geometry basis differs')
    _require(_number(section['mill_tolerance_m']) == 0 and _number(section['corrosion_allowance_m']) == 0
             and section['beam_theory'] == {'model': 'euler_bernoulli'},
             'coincident section bases required: zero mill tolerance and allowance, Euler-Bernoulli')
    return section


def _row(raw, result_id, kind, unit, entity):
    rows = raw['results']
    _require(type(rows) is list and all(type(row) is dict for row in rows), 'rows must be objects')
    row = _one([row for row in rows if row.get('id') == result_id], result_id)
    _require(row.get('kind') == kind and row.get('unit') == unit
             and row.get('entity_ref') == entity and row.get('basis_ref') == CASE_BASIS,
             'required raw row identity differs: ' + result_id)
    _number(row.get('value'))
    return row


def _all_finite(value):
    if type(value) in (int, float):
        _number(value)
    elif type(value) is dict:
        for item in value.values():
            _all_finite(item)
    elif type(value) is list:
        for item in value:
            _all_finite(item)


def _decimal(value):
    _require(type(value) is str, 'reference decimal must be explicit text')
    try:
        result = Decimal(value)
    except InvalidOperation as exc:
        raise ValueError('invalid reference decimal') from exc
    _require(result.is_finite() and result > 0, 'reference decimal must be finite positive')
    return result


def _binding(case_binding, section_reference, section_criteria, scalar_ok, physics_ok):
    """Reject incomplete method bindings before observations are assessed."""
    _require(type(case_binding) is dict and set(case_binding) == {
        'case_id', 'mode', 'transport_identity_ok', 'actual_input_binding_ok', 'ordinary_contract_ok'},
        'closed private case binding required')
    _require(case_binding['case_id'] in CASES, 'unsupported first-static case')
    _require(case_binding['mode'] in ('sparse_interactive', 'dense_scrutiny'), 'unsupported mode')
    for name in ('transport_identity_ok', 'actual_input_binding_ok', 'ordinary_contract_ok'):
        _require(type(case_binding[name]) is bool, name + ' must be a private boolean check result')
    _require(type(scalar_ok) is bool, 'scalar coverage must be a private boolean check result')
    _require(physics_ok is None or type(physics_ok) is bool, 'physical consistency must be bool or unavailable None')
    _require(type(section_reference) is dict and type(section_reference.get('values')) is dict,
             'original section reference missing')
    _require(type(section_criteria) is dict and section_criteria.get('format') ==
             'openpipestress.first_static_structural_numeric_addendum/1'
             and section_criteria.get('parent_obligation') == 'complete_case_material_section',
             'explicit structural numeric addendum required')
    _require(section_criteria.get('selection') == {
        'case_key': 'load_case_id', 'case_value': 'case', 'case_match_count': 1,
        'nested_array': 'pipe_sections', 'pipe_key': 'pipe_id', 'pipe_value': 'pipe',
        'pipe_match_count': 1, 'geometry_basis': 'authored_normalized_od_wall_v1'},
        'section selection binding differs')
    quantities = section_criteria.get('quantities')
    _require(type(quantities) is list and len(quantities) == 9
             and all(type(row) is dict for row in quantities), 'all nine quantity bindings required')
    indexed = {row.get('observed_field'): row for row in quantities}
    _require(set(indexed) == {row[0] for row in SECTION_FIELDS}, 'duplicate/missing/unknown section quantity')
    rules = section_criteria.get('criteria')
    _require(type(rules) is dict and rules, 'explicit per-field criteria missing')
    for field, dimension, unit, source in SECTION_FIELDS:
        q = indexed[field]
        _require(q.get('quantity_id') == 'section.' + field and q.get('dimension') == dimension
                 and q.get('unit') == unit, 'section quantity identity/unit/dimension differs')
        expected = _decimal(q.get('expected_decimal'))
        _require(_number(q.get('reference_value_binary64')) > 0
                 and q['reference_value_binary64'] == float(expected), 'binary64 reference differs from decimal')
        if source:
            _require(expected == _decimal(section_reference['values'].get(source)),
                     'addendum differs from original section reference: ' + field)
        rule = rules.get(q.get('criterion_ref'))
        _require(type(rule) is dict and rule.get('unit') == unit and rule.get('dimension') == dimension,
                 'missing/mismatched same-unit field rule')
        relative = _number(rule.get('relative_tolerance'))
        absolute = _number(rule.get('absolute_tolerance'))
        _require(relative >= 0 and absolute == 0, 'invalid relative criterion or unauthorized absolute floor')
        if field in IDENTITY_POINTERS:
            _require(rule.get('kind') == 'exact_same_unit_input_identity' and relative == 0
                     and q.get('bound_input_pointer_for_exact_identity') == IDENTITY_POINTERS[field],
                     'authored SI identity rule missing')
        else:
            _require(rule.get('kind') == 'same_unit_relative'
                     and q.get('bound_input_pointer_for_exact_identity') is None,
                     'derived quantity requires explicit relative rule')
    applies = section_criteria.get('applies_to')
    _require(type(applies) is list and all(type(row) is dict for row in applies), 'case applicability missing')
    selected = _one([row for row in applies if row.get('case_id') == case_binding['case_id']], 'addendum applicability')
    slug = CASES[case_binding['case_id']][0]
    _require(selected.get('request_id') == 'first-static-' + slug + '-v1'
             and selected.get('model_ref') == 'project:original-static-' + slug
             and selected.get('load_case_ref') == 'case' and selected.get('pipe_ref') == 'pipe'
             and selected.get('modes') == ['sparse_interactive', 'dense_scrutiny']
             and selected.get('quantity_ids') == ['section.' + row[0] for row in SECTION_FIELDS],
             'addendum case/model/mode/quantity applicability differs')
    return indexed, rules


def _input_model(request, case_id):
    """Closed finite SI fixture; no library defaults or hidden additional loads."""
    slug, expected_loads = CASES[case_id]
    model = request['solve']['preview_model']['model']
    _require(request['request']['request_id'] == 'first-static-' + slug + '-v1'
             and request['request']['operation'] == 'solve', 'request identity differs')
    _require(model['project']['id'] == 'project:original-static-' + slug, 'project identity differs')
    _require(model['project']['units'] == {'length': 'm', 'force': 'N', 'pressure': 'Pa',
             'stress': 'Pa', 'angle': 'rad', 'temperature': 'degC'}, 'authored SI units required')
    _require(set(model) == {'schema_version', 'document_kind', 'data_boundary', 'analysis_status',
             'project', 'nodes', 'pipe_segments', 'materials', 'supports', 'components',
             'combinations', 'load_cases', 'pressure_contract'}, 'unknown/missing input model fields')
    _require(request['solve']['preview_model'].get('materials') == [] and model['components'] == []
             and model['combinations'] == [], 'extra material override/component/combination')
    nodes = model['nodes']
    _require(type(nodes) is list and len(nodes) == 2 and {row['id'] for row in nodes} == {'root', 'tip'},
             'exact root/tip nodes required')
    for node in nodes:
        _require(set(node) <= {'id', 'label', 'position', 'provenance'}, 'extra node input')
        _require(set(node['position']) == {'x', 'y', 'z'}, 'position shape differs')
        for axis in ('x', 'y', 'z'):
            _same_number(node['position'][axis], 2 if node['id'] == 'tip' and axis == 'x' else 0,
                         'node position')
    pipe = _one(model['pipe_segments'], 'input pipe')
    _require(set(pipe) <= {'id', 'label', 'from', 'to', 'section', 'material', 'y_reference', 'provenance'}
             and pipe['id'] == 'pipe' and pipe['from'] == 'root' and pipe['to'] == 'tip'
             and pipe['material'] == 'material', 'input member identity/fields differ')
    _require(set(pipe['y_reference']) == {'x', 'y', 'z'}, 'member axis shape differs')
    for axis in ('x', 'y', 'z'):
        _same_number(pipe['y_reference'][axis], 1 if axis == 'y' else 0, 'member axis')
    # The re-authored copy declares the retired contract's implicit zero allowance explicitly; no other deduction.
    _require(set(pipe['section']) == {'outside_diameter', 'wall_thickness', 'corrosion_allowance'}
             and set(pipe['section']['corrosion_allowance']) == {'value', 'unit'}
             and pipe['section']['corrosion_allowance']['unit'] == 'm', 'section overrides/deductions unsupported')
    _same_number(pipe['section']['corrosion_allowance']['value'], ZERO_CORROSION_ALLOWANCE['value'], 'authored corrosion allowance')
    for key, expected in [('outside_diameter', .2), ('wall_thickness', .01)]:
        quantity = pipe['section'][key]
        _require(set(quantity) == {'value', 'unit'} and quantity['unit'] == 'm', 'section must be authored in m')
        _same_number(quantity['value'], expected, 'authored section ' + key)
    _require(model['schema_version'] == MODEL_VERSION and model['pressure_contract'] == PRESSURE_CONTRACT,
             'current model document and exact contract required')
    material = _one(model['materials'], 'input material')
    _require(set(material) <= {'id', 'label', 'constitutive_basis', 'elastic_modulus', 'poisson_ratio', 'provenance'}
             and material['id'] == 'material'
             and material['constitutive_basis'] == 'homogeneous_isotropic_E_nu_v1', 'material input differs')
    for name, unit, expected in [('elastic_modulus', 'Pa', 200e9), ('poisson_ratio', '1', .25)]:
        quantity = material[name]
        _require(set(quantity) == {'value', 'unit'} and quantity['unit'] == unit, 'explicit SI material quantity required')
        _same_number(quantity['value'], expected, 'input material ' + name)
    support = _one(model['supports'], 'input support')
    _require(set(support) <= {'id', 'label', 'node', 'family', 'restraints', 'provenance'}
             and support['id'] == 'anchor' and support['node'] == 'root' and support['family'] == 'anchor'
             and support['restraints'] == ['UX', 'UY', 'UZ', 'RX', 'RY', 'RZ'], 'root anchor differs')
    case = _one(model['load_cases'], 'input load case')
    _require(set(case) <= {'id', 'label', 'primitive_loads', 'pressure_regions', 'provenance'}
             and case['id'] == 'case' and case['pressure_regions'] == [], 'extra case/pressure/thermal input')
    loads = case['primitive_loads']
    _require(type(loads) is list and len(loads) == len(expected_loads), 'load count differs')
    indexed = {load['id']: load for load in loads}
    _require(set(indexed) == {'load:' + row[0] for row in expected_loads}, 'load IDs differ or duplicate')
    for direction, value, dimension in expected_loads:
        load = indexed['load:' + direction]
        _require(set(load) <= {'id', 'category', 'target', 'direction', 'magnitude', 'dimension', 'provenance'}
                 and load['target'] == {'type': 'node', 'node': 'tip'} and load['direction'] == direction
                 and load['category'] == 'concentrated_' + dimension and load['dimension'] == dimension,
                 'authored load kind/target differs')
        magnitude = load['magnitude']
        _require(set(magnitude) == {'value', 'unit'} and magnitude['unit'] == ('N' if dimension == 'force' else 'N*m'),
                 'authored load unit differs')
        _same_number(magnitude['value'], value, 'authored load value')
    return model


def check_first_static_structure(raw, request, case_binding, section_reference,
                                 section_criteria, scalar_coverage_ok, physical_consistency_ok):
    """Return all ten {id,state,reason,details} outcomes, without mutating inputs.

    ``raw`` is the mechanics envelope, ``request`` the full runner input, and
    ``section_criteria`` the complete explicit numeric addendum. Malformed method
    bindings raise ValueError; bad observations fail checks without dropping IDs.
    ``physical_consistency_ok=None`` reports the maximum check blocked. The two
    absent signed-fiber/shear output families remain outside direct row coverage.
    """
    try:
        quantities, rules = _binding(case_binding, section_reference, section_criteria,
                                    scalar_coverage_ok, physical_consistency_ok)
    except (KeyError, TypeError, AttributeError, OverflowError) as exc:
        raise ValueError('malformed first-static method binding: ' + str(exc)) from exc
    output = []

    def assess(check_id, operation):
        details = {}
        try:
            operation(details)
            state, reason = 'matched', 'required structural predicate satisfied'
        except (ValueError, KeyError, TypeError, AttributeError, OverflowError, IndexError) as exc:
            state, reason = 'failed', str(exc)
        output.append({'id': check_id, 'state': state, 'reason': reason, 'details': details})

    def transport(details):
        _require(case_binding['transport_identity_ok'], 'upstream captured wrapper/runner check failed')
        _require(raw['document_kind'] == 'openpipestress.product_preview.mechanics_result'
                 and raw['run_id'] == 'run:preview-linear-static-001'
                 and raw['status']['mechanics'] == 'MECHANICS_SOLVED', 'raw solve identity/status differs')
        diagnostics = raw['diagnostics']
        _require(type(diagnostics) is list and all(type(row) is dict and
                 type(row.get('severity')) is str and row['severity'].lower() in ('info', 'warning')
                 for row in diagnostics), 'unknown/blocking/fatal/error raw diagnostics')
        mode = _one([row for row in raw['results'] if row.get('kind') == 'linear_solver_mode_basis'], 'mode evidence')
        _require(mode['basis_ref'] == CASE_BASIS and mode['unit'] == 'mode_code', 'mode basis/unit differs')
        _same_number(mode['value'], 1 if case_binding['mode'] == 'sparse_interactive' else 2, 'actual mode')
        details['mode'] = case_binding['mode']

    def input_binding(details):
        _require(case_binding['actual_input_binding_ok'], 'upstream captured input/source/executable binding failed')
        model = _input_model(request, case_binding['case_id'])
        _require(raw['model_ref'] == model['project']['id'] and raw['accepted_model_state_mutated'] is False,
                 'raw model binding differs or accepted model mutated')
        details['model_ref'] = raw['model_ref']

    def contract(details):
        _require(case_binding['ordinary_contract_ok'], 'upstream pinned ordinary contract/table check failed')
        _require(raw['schema_version'] == '0.2.0' and raw['producer'] == {
            'component_name': 'open_pipe_stress_product_physics', 'component_version': '0.2.0',
            'semantic_contract_id': CONTRACT}, 'ordinary pressure-2 producer required')
        _require('source_block_recovery' not in raw and 'carrier_evidence' not in raw, 'source/composite namespace refused')
        formulation = raw['formulation_basis']
        _require(set(formulation) == {'profile_id', 'limitations'} and formulation['profile_id'] == PROFILE
                 and type(formulation['limitations']) is list and formulation['limitations']
                 and all(type(row) is str and row for row in formulation['limitations']), 'formulation basis differs')
        _require(request['solve']['preview_model']['model']['pressure_contract'] == PRESSURE_CONTRACT,
                 'authored formulation differs')
        details['semantic_contract_id'] = CONTRACT

    def material_section(details):
        # Always retain nine subchecks, even when the surrounding case is absent.
        details['section_quantities'] = [dict(id='section.' + field, state='failed',
            reason='section evidence unavailable') for field, *_ in SECTION_FIELDS]
        model = _input_model(request, case_binding['case_id'])
        evidence = raw['contract_evidence']
        _require(evidence['pressure'] == [] and evidence['connector'] == [], 'extra pressure/connector evidence')
        case = _exact_case(raw)
        _require(case['profile_mode'] == PROFILE and case['material_basis'] == 'base_material_common_E_nu',
                 'actual profile/material basis differs')
        pressure_rhs = case['pressure_rhs_assembly']
        _require(pressure_rhs['load_case_id'] == 'case' and pressure_rhs['groups'] == []
                 and pressure_rhs['node_order'] == ['root', 'tip'], 'unexpected pressure RHS source/member evidence')
        for field in ('assembled_pressure_rhs_global', 'rounded_cap_rhs_global', 'rounded_poisson_rhs_global'):
            values = pressure_rhs[field]
            _require(type(values) is list and len(values) == 12, 'pressure RHS DOF count differs')
            _require(all(_number(value) == 0 for value in values), 'unexpected pressure load in unpressurized fixture')
        material = _one(case['pipe_materials'], 'actual pipe material')
        _require(material['pipe_id'] == 'pipe' and material['material_id'] == 'material'
                 and material['constitutive_basis'] == 'homogeneous_isotropic_E_nu_v1'
                 and material['thermal_consumed'] is False and material['alpha_per_kelvin'] is None,
                 'actual material/thermal binding differs')
        for key, expected in [('E_pa', 200e9), ('nu', .25), ('G_pa', 80e9)]:
            _same_number(material[key], expected, 'actual material ' + key)
        # T4-U6 (D-5): the addendum's frozen selection names the retired single-section basis; the derived reading
        # takes each of its nine quantities at its D-5 location, on coincident bases.
        section = _d5_section(_one(case['pipe_sections'], 'actual pipe section'))
        for target, (field, _, unit, _) in zip(details['section_quantities'], SECTION_FIELDS):
            q, rule = quantities[field], rules[quantities[field]['criterion_ref']]
            target.update(unit=unit, criterion_ref=q['criterion_ref'],
                          locations=['.'.join(path) for path in D5_LOCATIONS[field]])
            try:
                first, *aliases = D5_LOCATIONS[field]
                actual = _number(_leaf(section, first))
                _require(actual > 0, 'positive section value required')
                _require(all(_number(_leaf(section, path)) == actual for path in aliases),
                         'section bases do not coincide: ' + field)
                expected = q['reference_value_binary64']
                if field in IDENTITY_POINTERS:
                    key = 'outside_diameter' if field == 'outside_diameter_m' else 'wall_thickness'
                    expected = model['pipe_segments'][0]['section'][key]['value']
                    _require(expected == q['reference_value_binary64'], 'bound SI input differs from addendum')
                delta = _number(abs(actual - expected))
                classification, basis = _classify_delta(delta, expected, actual, section_criteria,
                    {'unit_ref': {'ref': unit}, 'relative_tolerance_value': rule['relative_tolerance'],
                     'absolute_tolerance_value': rule['absolute_tolerance']}, unit)
                target.update(observed=actual, expected=expected, absolute_delta=delta, predicate_basis=basis)
                _require(classification == 'within_tolerance_profile', 'section field exceeds explicit criterion')
                target.update(state='matched', reason=classification)
            except (ValueError, TypeError, OverflowError) as exc:
                target.update(state='failed', reason=str(exc))
        _require(all(row['state'] == 'matched' for row in details['section_quantities']), 'required section quantities failed')

    def numerical(details):
        _all_finite(raw)
        for row in raw['results']:
            _number(row['value'])
        quality = raw['numerical_quality']
        _require(quality['value_representation'] == 'finite_binary64' and quality['publication_quantization'] == 'none'
                 and quality['integrity_policy'] == 'M03-INTEGRITY-v1' and quality['status'] == 'checks_passed',
                 'numerical publication/standing differs')
        case = _one(quality['cases'], 'numerical case')
        _require(case['basis_ref'] == CASE_BASIS and case['structural_status'] == 'passive_model_basis'
                 and case['solve_quality'] == 'checks_passed'
                 and case['model_matrix_fidelity'] == 'represented_equations_retained'
                 and case['accuracy_evidence'] == 'not_claimed', 'numerical case standing/accuracy claim differs')
        diagnostics = raw['diagnostics']
        indexed = {row['id']: row for row in diagnostics}
        _require(len(indexed) == len(diagnostics), 'duplicate numerical diagnostic identity')
        refs = case['evidence_refs']
        _require(type(refs) is list and refs and len(set(refs)) == len(refs), 'numerical evidence references missing')
        _require(all(ref in indexed and indexed[ref]['code'] == 'NUMERICAL_INTEGRITY_CHECKS_PASSED'
                 and indexed[ref]['severity'] == 'info' and 'case' in indexed[ref]['affected_refs'] for ref in refs),
                 'numerical diagnostics unavailable or incorrectly bound')
        details['accuracy_evidence'] = 'not_claimed'

    def coverage(details):
        _require(scalar_coverage_ok, f'upstream {DERIVED_SCALAR_ROWS} derived exact selector/metadata/dimension coverage failed')
        rows = raw['results']
        _require(type(rows) is list and all(type(row) is dict for row in rows), 'scalar rows missing')
        ids = [row.get('id') for row in rows]
        _require(all(type(item) is str and item for item in ids) and len(ids) == len(set(ids)), 'duplicate/missing row IDs')
        # The upstream result concerns the 94 derived engineering rows (the 73
        # frozen selectors, each stress selector on both section bases). The
        # producer's additional disclosed solver/physical rows stay in the raw
        # audit record; they are not a new required-row denominator.
        for entity in ('root', 'tip'):
            row = _row(raw, 'result:disp:' + entity, 'displacement_magnitude', 'mm', entity)
            _require('metadata' not in row, 'displacement magnitude metadata must be absent, not null/object')
        details.update(required_scalar_rows=DERIVED_SCALAR_ROWS, frozen_scalar_rows=FROZEN_SCALAR_ROWS,
                       observed_total_rows=len(rows))

    def maximum(details):
        extrema = _extrema(raw)
        coverage = _exact_case(raw)['stress_maximum_coverage']
        _require(type(coverage) is list and coverage == [
            {'section_basis': basis, 'complete': True, 'unavailable_pipe_ids': []} for basis in SECTION_BASES]
            and all(type(record['complete']) is bool for record in coverage),
            'normal maximum coverage incomplete or not per section basis')
        for basis, ex in extrema.items():
            _require(ex['approximation'] == 'piecewise_quadratic_straight_section_statics'
                     and ex['coefficient_basis'] == 'j_side_section_equilibrium_binary64_enclosures'
                     and ex['enclosure_scope'] == ENCLOSURE, 'normal maximum method/enclosure scope differs')
            row = _row(raw, MAXIMUM_IDS[basis], 'pipe_elastic_normal_stress_maximum_v2', 'Pa', 'pipe')
            lower, upper, global_upper = [_number(ex[key]) for key in
                                         ('value_lower_pa', 'value_upper_pa', 'global_upper_bound_pa')]
            _require(0 <= lower <= row['value'] <= upper <= global_upper, 'maximum interval ordering/containment differs')
            _require(_number(ex['certified_gap_pa']) >= 0, 'negative certified gap')
        # Coincident bases: the reduced extremum is the nominal one apart from its basis and row.
        strip = lambda ex: {key: value for key, value in ex.items() if key not in ('section_basis', 'result_id')}
        _require(strip(extrema['nominal_section']) == strip(extrema['reduced_section']), 'coincident-basis extrema differ')
        ex = extrema['nominal_section']
        details.update(coefficient_basis=ex['coefficient_basis'], enclosure_scope=ex['enclosure_scope'],
                       section_bases=list(SECTION_BASES), physical_consistency=physical_consistency_ok)
        _require(physical_consistency_ok is True, 'owned coefficient/gap consistency unavailable or failed')

    def location(details):
        for basis, ex in _extrema(raw).items():
            fraction = _number(ex['station_fraction'])
            _require(0 <= fraction <= 1 and 0 <= _number(ex['local_fraction']) <= 1, 'station witness outside member')
            _require(type(ex['span_index']) is int and ex['span_index'] == 0, 'single interval span index differs')
            if case_binding['case_id'] == 'original_static_bending_torsion_v1':
                _require(fraction == 0 and ex['local_fraction'] == 0, 'combined-case maximum must govern at root')
                details['location_interpretation'] = 'strict root station; opposite absolute-normal fibers tied'
            else:
                details['location_interpretation'] = 'witness of constant axial all-stations tie; no unique station/fiber claim'
            details.setdefault('station_fraction', {})[basis] = fraction

    def summary(details, key, result_id, kind, unit, entity):
        row = _row(raw, result_id, kind, unit, entity)
        summary_value = raw['summary'][key]
        _require(type(summary_value) is dict and set(summary_value) == {'value', 'unit', 'location_ref', 'result_ref'}
                 and summary_value['unit'] == unit and summary_value['location_ref'] == entity
                 and summary_value['result_ref'] == result_id, 'summary quantity/reference differs')
        _same_number(summary_value['value'], row['value'], 'summary/raw exact value')
        details.update(result_ref=result_id, unit=unit, observed=row['value'])

    def stress_headlines(details):
        # §2.2: no governing basis; each basis's headline binds its own maximum row.
        _require('max_open_formula_stress' in raw['summary'] and raw['summary']['max_open_formula_stress'] is None,
                 'the basis-free stress headline must be null on an exact document')
        for basis in SECTION_BASES:
            binding = {}
            summary(binding, HEADLINE_KEYS[basis], MAXIMUM_IDS[basis], 'pipe_elastic_normal_stress_maximum_v2', 'Pa', 'pipe')
            details.setdefault('bindings', {})[HEADLINE_KEYS[basis]] = binding

    def displacement(details):
        summary(details, 'max_displacement', 'result:disp:tip', 'displacement_magnitude', 'mm', 'tip')
        _input_model(request, case_binding['case_id'])
        for name, count in {'node_count': 2, 'segment_count': 1, 'support_count': 1, 'load_case_count': 1,
                            'component_stress_modifier_count': 0, 'curved_bend_macro_element_count': 0,
                            'spring_hanger_user_input_count': 0}.items():
            _require(type(raw['summary'].get(name)) is int and raw['summary'][name] == count,
                     'summary count differs: ' + name)

    operations = [transport, input_binding, contract, material_section, numerical, coverage,
                  maximum, location, stress_headlines, displacement]
    for check_id, operation in zip(CHECK_IDS, operations):
        assess(check_id, operation)
    maximum_check = output[6]
    if physical_consistency_ok is None and maximum_check['details'].get('physical_consistency', False) is None:
        maximum_check['state'] = 'blocked'
    return output
