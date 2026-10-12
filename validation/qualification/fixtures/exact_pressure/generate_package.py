"""Generate the VP-STATIC exact_pressure_1 package from the frozen T4-I7 and T4-I8 references.

Every expected value, wrong value and zero-scale floor is read at run time from
the frozen independent references:

- ``validation/references/t4_i7/u2_reference_cases.json`` (T4-I7, bends) with
  its documents ``u2_document_sketches.json``;
- ``validation/references/t4_i8/rebuilt_reference_cases.json`` (T4-I8, the
  rebuilt straight cases).

No constant value is copied and no product module or producer output is read.
Row identities (row ID and published metadata) follow the producer's naming
and metadata vocabulary, which is stated below as data (``METADATA`` and
``row_id``) and was identified once by running the producer on the documents;
a selector that does not match the producer's row fails resolution in the gate,
it never changes a value.

Values. T4-I7 writes 20-significant-digit decimals; each is converted by an
exact definitional unit factor (m to mm, Pa to MPa) in decimal and rounded once
to binary64. T4-I8 writes maintained quantities ``{unit, exact, decimal,
value}``; ``exact`` is authoritative and evaluated in decimal at 130 digits
(``rational``, ``rational_times_pi``, ``rational_over_pi`` and the two symbolic
forms ``a + (b)/pi`` and ``sqrt((a*pi)^2 + (b)^2)``), cross-checked against the
file's ``decimal`` (1e-80 relative) and ``value`` (bitwise), then converted and
rounded once.

Criteria. T4-I7: ``|observed - expected| <= 1e-9 * max(|expected|,
zero_scale(group))`` encoded as relative 1e-9 with the group's absolute floor
1e-9*zero_scale in the row unit. T4-I8: each row's own criterion (relative 1e-9;
a zero row's absolute floor 1e-9*zero_scale), recomputed here from its zero
scale and required equal to the frozen precomputed floor.

Usage (WORKING_ROOT is ``projects/chirality-piping``)::

    python validation/qualification/fixtures/exact_pressure/generate_package.py --write
    python validation/qualification/fixtures/exact_pressure/generate_package.py --check

``--write`` writes every case file under ``cases/`` (not committed) and
``MANIFEST.json`` (committed; it binds each case file by sha256). ``--check``
regenerates in memory and requires ``MANIFEST.json`` and any present case file
to be byte identical.
"""
from __future__ import annotations

import argparse
import copy
import decimal
from decimal import Decimal
from fractions import Fraction
import hashlib
import json
from pathlib import Path
import re
import sys

PACKAGE = Path(__file__).resolve().parent
WORKING_ROOT = PACKAGE.parents[3]
PACKAGE_PATH = 'validation/qualification/fixtures/exact_pressure'
CASES = 'cases'
I7 = 'validation/references/t4_i7/u2_reference_cases.json'
I7_DOCUMENTS = 'validation/references/t4_i7/u2_document_sketches.json'
I8 = 'validation/references/t4_i8/rebuilt_reference_cases.json'
TABLE = 'fixtures/results/semantic_contract_v0_3_pressure_1.json'
CONTRACT = 'openpipestress.result_semantics/0.3.0/pressure-1'
PRESSURE_CONTRACT = {'version': '3.0.0', 'mode': 'exact_pressure_v3'}
MANIFEST_FORMAT = 'openpipestress.exact_pressure_qualification_manifest/1'
SELECTOR_FORMAT = 'openpipestress.first_static_selector_candidate/1'
REFERENCE_FORMAT = 'openpipestress.qualification_reference_values/1'
ADMISSION_FORMAT = 'openpipestress.exact_pressure_admission/1'
ROW_NAMESPACE = 'payload.mechanics_envelope.results'
EVIDENCE_NAMESPACE = 'contract_evidence.pressure'
MODES = ['sparse_interactive', 'dense_scrutiny']
VERSIONS = ('0.3.0', '0.4.0')
RELATIVE = Decimal('1e-9')
CONTEXT = decimal.Context(prec=130, rounding=decimal.ROUND_HALF_EVEN)
UNIT_FACTORS = {('m', 'mm'): Decimal(1000), ('Pa', 'MPa'): Decimal('0.000001')}
LOCATIONS = ('end_i', 'quarter_1', 'midspan', 'quarter_3', 'end_j')
TERMINAL_FIELDS = (('closure_pressure_load_global', 'closure_pressure_load_global_n'),
                   ('pipe_cap_transfer_global', 'pipe_cap_transfer_global_n'),
                   ('remote_closure_support_reaction_global', 'remote_closure_support_reaction_global_n'))
TERMINAL_FAMILY = 'pressure_terminal_force'
AXES = ('x', 'y', 'z')

# --------------------------------------------------------------------------
# Row identity: the producer's row IDs and published metadata (identification)
# --------------------------------------------------------------------------

STIFFNESS = 'recovered_from_local_element_stiffness'
STRESS = 'recovered_from_open_mechanics_stress_components'
END_SIGN = {'end_i': 'positive value follows the element-local DOF at the i-end force vector',
            'end_j': 'positive value follows the element-local DOF at the j-end force vector'}
STATION_SIGN = ('positive value follows the j-side section action in the element-local frame (x toward end j); '
                'section equilibrium from stiffness-recovered end actions with consistent distributed-load fixed-end correction')
END_STRESS_SIGN = ('positive value follows the j-side section action in the element-local frame (local x toward end j); '
                   'resultants come from section equilibrium over assembled end actions')
STATION_STRESS_SIGN = {
    'bending_normal_stress_y': 'positive bending normal stress follows the section-equilibrium element-local y bending resultant at this station; ',
    'bending_normal_stress_z': 'positive bending normal stress follows the section-equilibrium element-local z bending resultant at this station; ',
    'torsional_shear_stress': 'positive torsional shear stress follows the section-equilibrium element-local torsional resultant at this station; ',
}
STATION_STRESS_TAIL = ('j-side section action in the element-local frame (x toward end j); '
                       'section equilibrium from stiffness-recovered end actions with consistent distributed-load fixed-end correction')
ARC_SECTION_SIGN = ('local x is endpoint arc tangent toward j; local z is bend-plane normal; local y is z cross x toward arc center; '
                    'resultants come from section equilibrium over assembled end actions')
PIPE_SIGN = {
    'straight': {
        'pipe_wall_endpoint_action_v2': 'node-on-element wall action, positive along authored local x toward end j; cap transfer is not subtracted from wall recovery',
        'pipe_wall_axial_force_v2': 'tension-positive material wall section resultant Nw',
        'pipe_effective_axial_force_v2': 'effective wall-fluid resultant S=Nw-pAi; not material stress or a support reaction',
        'pipe_axial_membrane_stress_v2': 'tension-positive axial wall membrane stress Nw/As; no added longitudinal pressure scalar',
        'pipe_lame_radial_stress_v2': 'tension-positive radial stress at named surface, inner traction -p and zero external pressure increment',
        'pipe_lame_hoop_stress_v2': 'tension-positive circumferential stress at named surface for long straight annulus, zero external pressure increment',
        'pipe_elastic_normal_stress_maximum_v2': ('nonnegative circumferential maximum |Nw/As|+hypot(My,Mz)/Z; bounded over all straight statics '
                                                  'intervals; torsional shear remains separate; no code stress or equivalent stress claim'),
    },
    'arc': {
        'pipe_wall_endpoint_action_v2': ('node-on-element wall action along the arc end tangent (local x toward end j at that end); '
                                         'N_w = N_el + pAi; cap transfer is not subtracted from wall recovery'),
        'pipe_wall_axial_force_v2': ('tension-positive material wall section resultant Nw = N_el + pAi along the arc tangent; '
                                     'shear and moments are the elastic section actions'),
        'pipe_effective_axial_force_v2': "effective wall-fluid resultant S=Nw-pAi (the arc's elastic axial force); not material stress or a support reaction",
        'pipe_axial_membrane_stress_v2': 'tension-positive axial wall membrane stress Nw/As on the arc section; no added longitudinal pressure scalar',
    },
}
PIPE_FORCE_KINDS = {'pipe_wall_endpoint_action_v2', 'pipe_wall_axial_force_v2', 'pipe_effective_axial_force_v2'}
SUPPORT_SIGN = 'support-on-pipe; positive global force and right-hand couple about attached node; force and moment norms remain separate'
NODE_SIGN = {
    'nodal_displacement_x': 'positive value follows the global cartesian X axis displacement of the node',
    'nodal_displacement_y': 'positive value follows the global cartesian Y axis displacement of the node',
    'nodal_displacement_z': 'positive value follows the global cartesian Z axis displacement of the node',
    'nodal_rotation_x': 'positive value follows the right-hand-rule rotation about the global cartesian X axis',
    'nodal_rotation_y': 'positive value follows the right-hand-rule rotation about the global cartesian Y axis',
    'nodal_rotation_z': 'positive value follows the right-hand-rule rotation about the global cartesian Z axis',
}
SECTION_ACTIONS = {  # kind -> (component, ID group, ID suffix)
    'element_local_shear_force_y': ('shear_force_y', 'force', 'shear-y'),
    'element_local_shear_force_z': ('shear_force_z', 'force', 'shear-z'),
    'element_local_torsional_moment': ('torsional_moment', 'moment', 'torsion'),
    'element_local_bending_moment_y': ('bending_moment_y', 'moment', 'bending-y'),
    'element_local_bending_moment_z': ('bending_moment_z', 'moment', 'bending-z'),
}
SECTION_STRESSES = {
    'element_local_bending_normal_stress_y': ('bending_normal_stress_y', 'bending-normal-y'),
    'element_local_bending_normal_stress_z': ('bending_normal_stress_z', 'bending-normal-z'),
    'element_local_torsional_shear_stress': ('torsional_shear_stress', 'torsional-shear'),
}
NODE_KINDS = {'global_nodal_displacement_x': 'ux', 'global_nodal_displacement_y': 'uy', 'global_nodal_displacement_z': 'uz',
              'global_nodal_rotation_x': 'rx', 'global_nodal_rotation_y': 'ry', 'global_nodal_rotation_z': 'rz'}
NODE_COMPONENT = {'ux': 'nodal_displacement_x', 'uy': 'nodal_displacement_y', 'uz': 'nodal_displacement_z',
                  'rx': 'nodal_rotation_x', 'ry': 'nodal_rotation_y', 'rz': 'nodal_rotation_z'}


class GenerationError(ValueError):
    pass


def require(ok: bool, message: str) -> None:
    if not ok:
        raise GenerationError(message)


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def slug(identifier: str) -> str:
    return identifier.replace(':', '-')


def counted(identifier: str) -> str:
    return f'{len(identifier)}:{identifier}'


def metadata(kind: str, component: str, member: str, location: str) -> dict:
    """The producer's published metadata for one row (identification, not value)."""
    def row(coordinate_system, basis, sign):
        return {'component': component, 'coordinate_system': coordinate_system, 'location': location,
                'basis': basis, 'sign_convention': sign}
    if kind in NODE_KINDS:
        return row('global', 'solved_from_global_linear_system', NODE_SIGN[component])
    if kind.startswith('support_reaction_'):
        return row('global', 'recovered_from_assembled_support_law', SUPPORT_SIGN)
    if kind in SECTION_ACTIONS:
        if location in END_SIGN:
            return row('element_local', STIFFNESS, END_SIGN[location])
        return row('element_local', STIFFNESS, ARC_SECTION_SIGN if member == 'arc' else STATION_SIGN)
    if kind in SECTION_STRESSES:
        if member == 'arc':
            return row('element_local', STIFFNESS, ARC_SECTION_SIGN)
        if location in END_SIGN:
            return row('element_local', STIFFNESS, END_STRESS_SIGN)
        return row('element_local', STRESS, STATION_STRESS_SIGN[component] + STATION_STRESS_TAIL)
    signs = PIPE_SIGN[member]
    require(kind in signs, f'no published {kind} row on a {member} member')
    if kind in PIPE_FORCE_KINDS:
        return row('element_local', STIFFNESS, signs[kind])
    return row('pipe_section', STRESS, signs[kind])


def row_id(kind: str, component: str, entity: str, location: str, case_id: str, first_case: bool) -> str:
    """The producer's row ID; later load cases carry the ``loadcase:`` prefix."""
    prefix = 'result:' if first_case else f'result:loadcase:{slug(case_id)}:'
    if kind in NODE_KINDS:
        return f'{prefix}disp:{slug(entity)}:{NODE_KINDS[kind]}'
    if kind.startswith('support_reaction_'):
        return f'result:support-action:{counted(case_id)}:{counted(entity)}:{component}'
    if kind == 'pipe_elastic_normal_stress_maximum_v2':
        return f'result:elastic-maximum:{counted(case_id)}:{counted(entity)}'
    if kind.startswith('pipe_'):
        return f'result:pressure-exact:{counted(case_id)}:{counted(entity)}:{location}:{component}'
    if kind in SECTION_ACTIONS:
        _, group, suffix = SECTION_ACTIONS[kind]
        if location == 'end_i':
            return f'{prefix}{group}:{slug(entity)}:{suffix}'
        if location == 'end_j':
            return f'{prefix}{group}:{slug(entity)}:{suffix}:end-j'
        return f'{prefix}{group}:{slug(entity)}:{location.replace("_", "-")}:{suffix}'
    _, suffix = SECTION_STRESSES[kind]
    return f'{prefix}stress:{slug(entity)}:{location.replace("_", "-")}:{suffix}'


class Semantics:
    """Dimensions and result families from the pinned pressure-1 table (the contract, not output)."""

    def __init__(self):
        table = json.loads((WORKING_ROOT / TABLE).read_bytes())
        require(table['semantic_contract_id'] == CONTRACT, 'pressure-1 table identity')
        self.rows = {(row['kind'], row['unit'], row['component']): row for row in table['rows']}

    def selector(self, kind, unit, component, entity, location, member, case_id, first_case):
        row = self.rows.get((kind, unit, component))
        require(row is not None and row['family'] is not None, f'no pressure-1 quantity {kind} {unit} {component}')
        selector = {'id': row_id(kind, component, entity, location, case_id, first_case), 'kind': kind, 'unit': unit,
                    'entity_ref': entity, 'basis_ref': {'ref_type': 'load_case', 'ref_id': case_id},
                    'metadata': metadata(kind, component, member, location),
                    'dimension': row['source_physical_semantic_dimension']}
        return selector, row['family']


# --------------------------------------------------------------------------
# Exact values
# --------------------------------------------------------------------------

RATIONAL = r'-?\d+(?:/\d+)?'
OVER_PI = re.compile(rf'^(?P<a>{RATIONAL}) \+ \((?P<b>{RATIONAL})\)/pi$')
HYPOT_PI = re.compile(rf'^sqrt\(\((?P<a>{RATIONAL})\*pi\)\^2 \+ \((?P<b>{RATIONAL})\)\^2\)$')


def rational(text: str) -> Decimal:
    fraction = Fraction(text)
    return Decimal(fraction.numerator) / Decimal(fraction.denominator)


def maintained(quantity: dict, pi: Decimal, label: str) -> Decimal:
    """A T4-I8 maintained quantity, exact first, cross-checked with the file."""
    require(isinstance(quantity, dict) and isinstance(quantity.get('exact'), dict), f'not a maintained quantity: {label}')
    exact = quantity['exact']
    kind = exact.get('kind')
    allowed = {'unit', 'exact', 'decimal', 'value'} | ({'evaluation'} if kind == 'symbolic' else set())
    require(set(quantity) == allowed, f'maintained quantity keys: {label}')
    with decimal.localcontext(CONTEXT):
        if kind == 'rational':
            result = rational(exact['rational'])
        elif kind == 'rational_times_pi':
            result = rational(exact['rational']) * pi
        elif kind == 'rational_over_pi':
            result = rational(exact['rational']) / pi
        elif kind == 'symbolic':
            expression = exact.get('expression', '')
            over, hypot = OVER_PI.match(expression), HYPOT_PI.match(expression)
            if over:
                result = rational(over['a']) + rational(over['b']) / pi
            elif hypot:
                result = ((rational(hypot['a']) * pi) ** 2 + rational(hypot['b']) ** 2).sqrt()
            else:
                raise GenerationError(f'unsupported symbolic form at {label}: {expression}')
        else:
            raise GenerationError(f'unknown exact kind {kind!r} at {label}')
        rendered = Decimal(quantity['decimal'])
        if result != 0:
            require(abs((result - rendered) / result) <= Decimal('1e-80'), f'exact/decimal disagreement at {label}')
        else:
            require(rendered == 0, f'exact zero rendered nonzero at {label}')
    require(binary64(result) == float(quantity['value']), f'binary64 projection disagreement at {label}')
    return result


def binary64(value: Decimal) -> float:
    result = float(value)  # correctly rounded
    return 0.0 if result == 0.0 else result


def convert(value: Decimal, from_unit: str, to_unit: str, label: str) -> Decimal:
    if from_unit == to_unit:
        return value
    factor = UNIT_FACTORS.get((from_unit, to_unit))
    require(factor is not None, f'{label}: no exact unit factor {from_unit} -> {to_unit}')
    with decimal.localcontext(CONTEXT):
        return value * factor


def resolve(document: dict, pointer: str):
    require(pointer.startswith('/'), f'JSON pointer required: {pointer}')
    node = document
    for raw in pointer[1:].split('/'):
        key = raw.replace('~1', '/').replace('~0', '~')
        require(isinstance(node, dict) and key in node, f'pointer key {key} missing in {pointer}')
        node = node[key]
    return node


# --------------------------------------------------------------------------
# One case: selectors, values, criteria
# --------------------------------------------------------------------------

class Case:
    def __init__(self, case_id: str, reference_path: str, reference_key: str, document: dict, semantics: Semantics):
        self.case_id, self.reference_path, self.reference_key = case_id, reference_path, reference_key
        self.document, self.semantics = document, semantics
        self.load_cases = [case['id'] for case in document['model']['load_cases']]
        self.positive, self.negative, self.values, self.wrong, self.gaps, self.absences = [], [], [], [], [], []
        self.rules, self.identities, self.by_origin = {}, set(), {}

    def rule(self, family, dimension, unit, tag, relative, absolute, provenance):
        rule_id = f'criterion:{family}:{dimension}:{unit}:{tag}'
        rule = {'rule_id': rule_id, 'dimension_id': dimension, 'result_family': family,
                'unit_ref': {'ref_type': 'unit', 'ref': unit}, 'normalization_basis': 'same_unit_required',
                'relative_tolerance_value': relative, 'absolute_tolerance_value': absolute,
                'tolerance_value_status': 'project_specific_review_required', 'provenance': provenance}
        if rule_id in self.rules:
            require(self.rules[rule_id] == rule, f'{self.case_id}: rule {rule_id} defined inconsistently')
        self.rules[rule_id] = rule
        return rule

    def assert_positive(self, selector, family, expected: Decimal, rule, origin, handle=None):
        """``handle`` names the asserted quantity for negatives (default: its reference pointer)."""
        identity = json.dumps(selector, sort_keys=True)
        require(identity not in self.identities, f'{self.case_id}: duplicate selector {selector["id"]}')
        self.identities.add(identity)
        aid = f'assert:{len(self.positive) + 1:04}'
        self.positive.append({'id': aid, 'selector': selector, 'criterion_rule_id': rule['rule_id'],
                              'selector_origin': {**origin, 'family': family}})
        value = binary64(expected)
        self.values.append({'assertion_id': aid, 'value': value, 'unit': selector['unit']})
        self.by_origin[handle or origin['pointer']] = (aid, selector, rule, value, Decimal(1))
        return aid

    def assert_negative(self, control: str, pointer: str, wrong: Decimal, discriminator: str):
        """A named wrong value at an asserted quantity, excluded under the same rule."""
        target, selector, rule, correct, sign = self.by_origin[pointer]
        value = binary64(wrong * sign)
        allowed = max(rule['absolute_tolerance_value'], rule['relative_tolerance_value'] * max(abs(correct), abs(value)))
        require(abs(correct - value) > allowed, f'{self.case_id}: {control} at {pointer} is not distinct from the reference')
        pair = (selector['id'], value)
        if any((item['selector']['id'], wrong_value['value']) == pair for item, wrong_value in zip(self.negative, self.wrong)):
            return  # one negative scores identical (selector, wrong value) discriminators
        aid = f'negative:{control}:{len(self.negative) + 1:03}'
        self.negative.append({'id': aid, 'selector': copy.deepcopy(selector), 'criterion_rule_id': rule['rule_id'],
                              'selector_origin': {'negates_assertion': target, 'pointer': pointer.split('@result:')[0], 'control': control}})
        self.wrong.append({'assertion_id': aid, 'value': value, 'unit': selector['unit'], 'discriminator': discriminator})

    def gap(self, kind, reason, pointers=None):
        entry = {'kind': kind, 'reason': reason}
        if pointers:
            entry['pointers'] = sorted(pointers)
        self.gaps.append(entry)

    def files(self, admitted: dict | None, reference_sha256: str) -> dict:
        selectors = {'format': SELECTOR_FORMAT, 'case_id': self.case_id, 'producer_contract': CONTRACT,
                     'raw_schema_version': '0.2.0', 'row_namespace': ROW_NAMESPACE, 'assertions': self.positive,
                     'negative_assertions': self.negative, 'absences': self.absences, 'gaps': self.gaps,
                     'scoring_readiness': {'modes': MODES, 'reference': f'{self.reference_path}#{self.reference_key}'}}
        review = (f"{admitted['independent_review']['path']}#sha256={admitted['independent_review']['sha256']}" if admitted else None)
        reference = {'format': REFERENCE_FORMAT, 'case_id': self.case_id,
                     'readiness': 'ready' if admitted else 'pending_independent_review',
                     'basis': (f'generate_package.py evaluating {self.reference_path} (sha256 {reference_sha256}) case '
                               f'{self.reference_key}; exact decimal, exact unit factors, rounded once to binary64; no observed producer value'),
                     'reference_kind': 'analytical', 'independent_review_ref': review,
                     'values': self.values, 'wrong_values': self.wrong}
        for rule in self.rules.values():
            rule['review'] = ('the frozen reference criterion (T4-I7 refuted by T4-RV3, T4-I8 by T4-RV4); package mapping admitted '
                              'in the independent review named in ADMISSION.json; not a release criterion' if admitted else
                              'the frozen reference criterion; package mapping pending independent review, not admitted')
        criteria = {'schema_version': '0.1.0', 'tolerance_profile': {
            'profile_id': f'exact-pressure-vp-static-{self.case_id}-v1',
            'profile_status': 'reviewed' if admitted else 'draft_pending_independent_review',
            'scope': 'This case only; the frozen reference criterion, no new or relaxed threshold; not a release criterion.',
            'rules': sorted(self.rules.values(), key=lambda rule: rule['rule_id'])}}
        return {'selectors': selectors, 'reference': reference, 'criteria': criteria}


# --------------------------------------------------------------------------
# T4-I7 (bends)
# --------------------------------------------------------------------------

# Station quantity -> (kind, component, reference unit, published unit, zero-scale group).
I7_STATION = (
    ('N_w', 'pipe_wall_axial_force_v2', 'wall_axial_force', 'N', 'N', 'wall_axial_force'),
    ('S', 'pipe_effective_axial_force_v2', 'effective_axial_force', 'N', 'N', 'effective_axial_force'),
    ('sigma_m', 'pipe_axial_membrane_stress_v2', 'axial_membrane_stress', 'Pa', 'Pa', 'membrane_stress'),
    ('V_y', 'element_local_shear_force_y', 'shear_force_y', 'N', 'N', 'shear_force'),
    ('V_z', 'element_local_shear_force_z', 'shear_force_z', 'N', 'N', 'shear_force'),
    ('T', 'element_local_torsional_moment', 'torsional_moment', 'N*m', 'N*m', 'section_moment'),
    ('M_y', 'element_local_bending_moment_y', 'bending_moment_y', 'N*m', 'N*m', 'section_moment'),
    ('M_z', 'element_local_bending_moment_z', 'bending_moment_z', 'N*m', 'N*m', 'section_moment'),
    ('sigma_b_y', 'element_local_bending_normal_stress_y', 'bending_normal_stress_y', 'Pa', 'MPa', 'bending_torsion_stress'),
    ('sigma_b_z', 'element_local_bending_normal_stress_z', 'bending_normal_stress_z', 'Pa', 'MPa', 'bending_torsion_stress'),
    ('tau_t', 'element_local_torsional_shear_stress', 'torsional_shear_stress', 'Pa', 'MPa', 'bending_torsion_stress'),
)
I7_SECTION_ACTIONS = {'V_y', 'V_z', 'T', 'M_y', 'M_z'}
# End row quantity -> station quantity it equals (end_i negated, end_j as is), kind and component.
I7_END_ROWS = (('F_y', 'V_y'), ('F_z', 'V_z'), ('M_x', 'T'), ('M_y', 'M_y'), ('M_z', 'M_z'))
I7_LAME = (('inner_radial', 'pipe_lame_radial_stress_v2', 'lame_inner_radial_stress'),
           ('outer_radial', 'pipe_lame_radial_stress_v2', 'lame_outer_radial_stress'),
           ('inner_hoop', 'pipe_lame_hoop_stress_v2', 'lame_inner_hoop_stress'),
           ('outer_hoop', 'pipe_lame_hoop_stress_v2', 'lame_outer_hoop_stress'))
I7_NODE_UNITS = {'ux': ('m', 'mm', 'displacement'), 'uy': ('m', 'mm', 'displacement'), 'uz': ('m', 'mm', 'displacement'),
                 'rx': ('rad', 'rad', 'rotation'), 'ry': ('rad', 'rad', 'rotation'), 'rz': ('rad', 'rad', 'rotation')}
# Withheld on realized arcs (plan section 4.3 item 1; the reference's lame_surface convention).
I7_ARC_ABSENT = ('pipe_lame_radial_stress_v2', 'pipe_lame_hoop_stress_v2', 'pipe_elastic_normal_stress_maximum_v2',
                 'element_local_axial_force', 'element_local_axial_normal_stress')
I7_NOT_PUBLISHED = ('not published: the arc end-station section actions (tangent frame) and tangent-frame end rows, whose '
                    'end actions the chord-frame elastic end rows carry, and the axial end force F_x (element_local_axial_force '
                    'is replaced by the pressure family)')


def i7_case(case_key: str, version: str, case: dict, document: dict, semantics: Semantics) -> Case:
    out = Case(f'{case_key}@{version}', I7, case_key, document, semantics)
    require(out.load_cases == ['case:u2'], f'{case_key}: one load case expected')
    load_case = out.load_cases[0]
    floors = {group: (Decimal(entry['zero_scale']), entry['unit']) for group, entry in case['zero_scale'].items()}

    def floor_rule(family, dimension, unit, group, reference_unit):
        scale, scale_unit = floors[group]
        require(scale_unit == reference_unit, f'{case_key}: zero scale {group} unit {scale_unit}, row {reference_unit}')
        with decimal.localcontext(CONTEXT):
            absolute = RELATIVE * convert(scale, scale_unit, unit, group)
        return out.rule(family, dimension, unit, f'floor:{group}', 1e-09, binary64(absolute),
                        f'T4-I7 criterion |observed - expected| <= 1e-9*max(|expected|, zero_scale({group})); '
                        f'zero_scale = {scale} {scale_unit} (case {case_key} zero_scale block)')

    def add(pointer, text, reference_unit, kind, component, unit, entity, location, member, group):
        selector, family = semantics.selector(kind, unit, component, entity, location, member, load_case, True)
        rule = floor_rule(family, selector['dimension'], unit, group, reference_unit)
        expected = convert(Decimal(text), reference_unit, unit, pointer)
        out.assert_positive(selector, family, expected, rule,
                            {'reference': I7, 'pointer': pointer, 'reference_unit': reference_unit, 'transform': 'identity', 'group': group})

    expected = case['expected']
    for node, values in expected['nodes'].items():
        for key, text in values.items():
            reference_unit, unit, group = I7_NODE_UNITS[key]
            kind = next(k for k, v in NODE_KINDS.items() if v == key)
            add(f'/cases/{case_key}/expected/nodes/{node}/{key}', text, reference_unit, kind, NODE_COMPONENT[key], unit,
                node, 'node', 'straight', group)
    for support, values in expected['supports'].items():
        for key, text in values.items():
            force = key.startswith('F')
            add(f'/cases/{case_key}/expected/supports/{support}/{key}', text, 'N' if force else 'N*m',
                'support_reaction_component_v2', key, 'N' if force else 'N*m', support, 'node', 'straight',
                f'support_{"force" if force else "moment"}@{support}')
    implied, not_published = [], []
    for pipe, member in expected['members'].items():
        arc = member['kind'] == 'arc'
        kind_of = 'arc' if arc else 'straight'
        base = f'/cases/{case_key}/expected/members/{pipe}'
        for location in LOCATIONS:
            station = member['stations'][location]
            for key, kind, component, reference_unit, unit, group in I7_STATION:
                pointer = f'{base}/stations/{location}/{key}'
                if key in I7_SECTION_ACTIONS and location in ('end_i', 'end_j'):
                    # The end station equals the node-on-element end row (end_i negated);
                    # straights assert it through the end row, arcs do not publish it.
                    (not_published if arc else implied).append(pointer)
                    continue
                add(pointer, station[key], reference_unit, kind, component, unit, pipe, location, kind_of, group)
        for location in ('end_i', 'end_j'):
            block = 'chord_frame_elastic' if arc else 'element_local'
            end = member['end_rows'][location]
            for key, station_key in I7_END_ROWS:
                kind, component = next((k, c) for q, k, c, *_ in I7_STATION if q == station_key)
                group = 'section_moment' if key.startswith('M') else ('elastic_end_force_chord_frame' if arc else 'shear_force')
                unit = 'N*m' if key.startswith('M') else 'N'
                pointer = f'{base}/end_rows/{location}/{block}/{key}'
                add(pointer, end[block][key], unit, kind, component, unit, pipe, location, kind_of, group)
                if not arc:
                    station = Decimal(member['stations'][location][station_key])
                    require(station == (-1 if location == 'end_i' else 1) * Decimal(end[block][key]),
                            f'{case_key}: {pipe} {location} {station_key} differs from its end row')
                    # A control listed at the station pointer scores this end row (end_i negated).
                    aid, selector, rule, value, _ = out.by_origin[pointer]
                    out.by_origin[f'{base}/stations/{location}/{station_key}'] = (aid, selector, rule, value, Decimal(-1 if location == 'end_i' else 1))
            wall = 'tangent_frame' if arc else 'element_local'
            add(f'{base}/end_rows/{location}/{wall}/wall_axial_end_action', end[wall]['wall_axial_end_action'], 'N',
                'pipe_wall_endpoint_action_v2', 'wall_axial_end_action', 'N', pipe, location, kind_of, 'wall_axial_force')
            not_published.append(f'{base}/end_rows/{location}/{block}/F_x')
            if arc:
                not_published.extend(f'{base}/end_rows/{location}/tangent_frame/{key}' for key in ('F_x', 'F_y', 'F_z', 'M_x', 'M_y', 'M_z'))
        if arc:
            out.absences.extend({'row': {'kind': kind, 'entity_ref': pipe}} for kind in I7_ARC_ABSENT)
        else:
            for location in LOCATIONS:
                # One surface value per member, published at every station (pointer@station).
                for key, kind, component in I7_LAME:
                    add(f'{base}/lame_surface/{key}@{location}', member['lame_surface'][key], 'Pa', kind, component, 'Pa',
                        pipe, location, 'straight', 'lame_stress')
    for node, terminal in expected['terminals'].items():
        for key, field in TERMINAL_FIELDS:
            if terminal[key] is None:
                out.absences.append({'evidence': {'basis_ref': {'ref_type': 'load_case', 'ref_id': load_case}, 'record': 'terminal',
                                                  'key': {'node_ref': node}, 'field': field}})
                continue
            for axis, component in zip(AXES, ('Fx', 'Fy', 'Fz')):
                pointer = f'/cases/{case_key}/expected/terminals/{node}/{key}/{component}'
                selector = {'id': f'evidence:{load_case}:terminal:{node}:{field}:{axis}', 'namespace': EVIDENCE_NAMESPACE,
                            'basis_ref': {'ref_type': 'load_case', 'ref_id': load_case}, 'record': 'terminal',
                            'key': {'node_ref': node}, 'field': field, 'component': axis,
                            'definition': {'closure_transfer': terminal['closure_transfer']}, 'unit': 'N', 'dimension': 'force'}
                rule = floor_rule(TERMINAL_FAMILY, 'force', 'N', 'remote_closure_force', 'N')
                out.assert_positive(selector, TERMINAL_FAMILY, Decimal(terminal[key][component]), rule,
                                    {'reference': I7, 'pointer': pointer, 'reference_unit': 'N', 'transform': 'identity',
                                     'group': 'remote_closure_force'})
    for control in case.get('wrong_result_discriminators') or []:
        for row in control['discriminating_values']:
            pointer = f'/cases/{case_key}' + row['pointer']
            if pointer not in out.by_origin:
                require(pointer in not_published, f'{case_key}: control {control["id"]} names an unmapped pointer {pointer}')
                out.gap('not_published', f'control {control["id"]}: this row is not published (arc end station)', [pointer])
                continue
            unit = out.by_origin[pointer][1]['unit']
            reference_unit = 'm' if unit == 'mm' else ('Pa' if unit == 'MPa' else unit)
            out.assert_negative(control['id'], pointer, convert(Decimal(row['wrong_value']), reference_unit, unit, pointer),
                                f'{pointer} wrong_value of control {control["id"]}')
    if implied:
        out.gap('implied_by_positive', 'straight end-station section actions equal the asserted end rows (end_i negated); checked exactly by the generator', implied)
    if not_published:
        out.gap('not_published', I7_NOT_PUBLISHED, not_published)
    out.gap('not_scored', 'geometry, derived section and arc blocks are inputs of the reference, not published rows',
            [f'/cases/{case_key}/derived'] + [f'/cases/{case_key}/expected/members/{p}/arc' for p, m in expected['members'].items() if m['kind'] == 'arc'])
    return out


# --------------------------------------------------------------------------
# T4-I8 (rebuilt straight cases)
# --------------------------------------------------------------------------

# Each discriminator -> [(owner variant or None, expected key)]; the load case is the control's own `case`.
I8_DISCRIMINATORS = {
    'milltol_lame_membrane': {
        'retired_thin_wall_hoop': [('free_transferring', 'lame_inner_hoop'), ('axially_restrained_transferring', 'lame_inner_hoop')],
        'retired_thin_wall_longitudinal_free': [('free_transferring', 'sigma_z')],
        'retired_thin_wall_longitudinal_restrained': [('axially_restrained_transferring', 'sigma_z')],
        'allowance_not_folded': [('free_transferring', 'lame_inner_hoop'), ('axially_restrained_transferring', 'lame_inner_hoop')],
        'allowance_not_folded_sigma_z_free': [('free_transferring', 'sigma_z')],
        'mill_tolerance_ignored': [('free_transferring', 'lame_inner_hoop'), ('axially_restrained_transferring', 'lame_inner_hoop')],
        'mill_tolerance_ignored_sigma_z_free': [('free_transferring', 'sigma_z')],
        'longitudinal_pressure_added_free': [('free_transferring', 'sigma_z')],
        'poisson_sign_reversed_restrained': [('axially_restrained_transferring', 'Nw')],
        'cap_area_on_authored_bore': [('free_transferring', 'sigma_z')],
    },
    'tp_phys_pressure_halves': {
        'legacy_same_sign_thrust': [(None, 'Nw')], 'pressure_as_compression': [(None, 'Nw')],
        'poisson_term_omitted': [(None, 'Nw')], 'caps_subtracted_in_recovery': [(None, 'Nw')],
        'S_sign_reversed': [(None, 'S')], 'legacy_same_sign_reaction': [(None, 'A_Fx')],
        'pressure_half_compression': [(None, 'Nw')], 'pressure_half_legacy_thrust': [(None, 'Nw')],
        'pressure_half_caps_subtracted': [(None, 'Nw')], 'station_sign_i_side': [(None, 'A-B_midspan_bending_moment_z')],
        'full_span_load': [(None, 'D_uy')],
    },
    'pressure_membrane_thin_wall_limit': {
        'thin_hoop': [(None, 'lame_inner_hoop'), (None, 'lame_outer_hoop')], 'thin_longitudinal': [(None, 'sigma_z')],
        'mean_hoop_as_surface': [(None, 'lame_inner_hoop')],
    },
}
I8_ABSENT = ('element_local_axial_force', 'element_local_axial_normal_stress',
             'pipe_section_pressure_hoop_stress', 'pipe_section_pressure_longitudinal_stress')
# No rows of its own: its values are the v2 fixture SOURCE_ODWALL_EXPECTATIONS.json, and its SP-1 twin
# clause is checked bit for bit by core/product_physics/tests/t4_u2_pressure_references.rs.
I8_NO_ROWS = {'v3_straight_twin'}


def i8_owners(references: dict):
    for case_key, case in references['cases'].items():
        if 'variants' in case:
            for variant, body in case['variants'].items():
                yield case_key, variant, body
        else:
            yield case_key, None, case


def i8_case(references: dict, case_key: str, variant: str | None, owner: dict, version: str, pi: Decimal, semantics: Semantics) -> Case:
    patch = owner['documents']['v3_patch_for_both']
    require(patch['op'] == 'replace' and patch['path'] == '/model/pressure_contract' and patch['value'] == PRESSURE_CONTRACT,
            f'{case_key}: v3 patch')
    document = copy.deepcopy(owner['documents'][f'v2_model_{version}'])
    document['model']['pressure_contract'] = copy.deepcopy(patch['value'])
    owner_id = case_key if variant is None else f'{case_key}.{variant}'
    out = Case(f'{owner_id}@{version}', I8, case_key, document, semantics)
    require(set(out.load_cases) == set(owner['load_cases']), f'{owner_id}: load cases differ from the document')
    pressurized = set()
    for region in (r for case in document['model']['load_cases'] for r in case.get('pressure_regions', [])):
        pressurized.update(region['member_pipe_ids'])
    by_key = {}
    for index, case_id in enumerate(out.load_cases):
        for row in owner['load_cases'][case_id]['rows']:
            origin = row['reference_origin']
            require(origin['kind'] == 'analytical' and origin['transform'] == 'identity', f'{owner_id}: origin kind')
            require(row['basis_ref'] == {'ref_type': 'load_case', 'ref_id': case_id}, f'{owner_id}: row basis')
            selector, family = semantics.selector(row['kind'], row['unit'], row['component'], row['entity_ref'], row['location'],
                                                  'straight', case_id, index == 0)
            require(selector['metadata']['coordinate_system'] == row['coordinate_system'], f'{owner_id}: coordinate system of {selector["id"]}')
            quantity = resolve(references, origin['pointer'])
            require(quantity['unit'] == origin['reference_unit'], f'{owner_id}: reference unit at {origin["pointer"]}')
            value = convert(maintained(quantity, pi, origin['pointer']), origin['reference_unit'], row['unit'], origin['pointer'])
            criterion = row['criterion']
            if criterion['kind'] == 'relative':
                require(criterion == {'kind': 'relative', 'relative_tolerance': 1e-09, 'absolute_tolerance': 0.0}, f'{owner_id}: criterion')
                require(value != 0, f'{owner_id}: a relative criterion needs a nonzero reference')
                rule = out.rule(family, selector['dimension'], row['unit'], 'relative_1e-9', 1e-09, 0.0,
                                'T4-I8 criterion |observed - expected| <= 1e-9*|expected| (the existing protected relative criterion)')
            else:
                require(criterion['kind'] == 'zero_scale' and criterion['relative_tolerance'] == 0.0 and value == 0, f'{owner_id}: criterion')
                scale = origin['zero_scale']
                base = resolve(references, scale['base']['pointer'])
                require(base['unit'] == scale['base']['reference_unit'] == scale['scale_unit'], f'{owner_id}: zero scale unit')
                with decimal.localcontext(CONTEXT):
                    absolute = RELATIVE * convert(abs(maintained(base, pi, scale['base']['pointer'])), scale['scale_unit'], row['unit'], scale['tag'])
                require(binary64(absolute) == criterion['absolute_tolerance'], f'{owner_id}: zero floor differs from the frozen row criterion')
                rule = out.rule(family, selector['dimension'], row['unit'], f'zero_scale:{case_id}:{scale["tag"]}', 0.0, binary64(absolute),
                                f'T4-I8 zero rule |observed| <= 1e-9*zero_scale; zero_scale {scale["base"]["pointer"]} in {row["unit"]}')
            handle = f'{origin["pointer"]}@{selector["id"]}'
            out.assert_positive(selector, family, value, rule,
                                {'reference': I8, 'pointer': origin['pointer'], 'reference_unit': origin['reference_unit'], 'transform': 'identity'},
                                handle)
            by_key.setdefault((case_id, row['expected']), []).append(handle)
    out.absences.extend({'row': {'kind': kind, 'entity_ref': pipe}} for pipe in sorted(pressurized) for kind in I8_ABSENT)
    mapping = I8_DISCRIMINATORS.get(case_key, {})
    for control in references['cases'][case_key].get('wrong_result_discriminators') or []:
        if control.get('producible_today') is False:
            out.gap('not_active', f'control {control["id"]} is active from {control.get("active_from")}; not producible today')
            continue
        targets = mapping.get(control['id'])
        require(targets is not None, f'{case_key}: discriminator {control["id"]} has no mapping')
        for target_variant, key in targets:
            if target_variant != variant:
                continue
            case_id = control.get('case') or out.load_cases[0]
            handles = by_key.get((case_id, key))
            require(bool(handles), f'{owner_id}: control {control["id"]} names {key} without a row')
            # The first row carrying the quantity scores the control (every such row is asserted positively).
            unit = out.by_origin[handles[0]][1]['unit']
            wrong = control['wrong']
            out.assert_negative(control['id'], handles[0], convert(maintained(wrong, pi, control['id']), wrong['unit'], unit, control['id']),
                                f'/cases/{case_key}/wrong_result_discriminators/{control["id"]}')
    return out


# --------------------------------------------------------------------------
# Runner inputs, manifest and files
# --------------------------------------------------------------------------

def runner_input(case: Case) -> dict:
    model = case.document['model']
    project = model['project']['id']
    request = {
        'input_manifest_ref': {'ref_type': 'audit_manifest', 'ref_id': f'manifest:ep1-{slug(case.case_id)}'},
        'load_basis_refs': [{'ref_type': 'load_case', 'ref_id': cid} for cid in case.load_cases],
        'model_ref': {'ref_type': 'model', 'ref_id': project},
        'operation': 'solve',
        'operation_ref': {'ref_id': 'solve', 'ref_type': 'api_operation'},
        'privacy': {'classification': 'public_metadata', 'local_only': True, 'private_payload_redacted': True, 'telemetry_allowed': False},
        'professional_boundary': {'human_review_required': True, 'software_makes_approval_claim': False,
                                  'software_makes_authentication_claim': False, 'software_makes_certification_claim': False,
                                  'software_makes_compliance_claim': False, 'software_makes_sealing_claim': False},
        'project_ref': {'ref_type': 'project', 'ref_id': project},
        'provenance': {'source_name': f'VP-STATIC exact_pressure_1 case {case.case_id}',
                       'source_location': f'{PACKAGE_PATH}/generate_package.py',
                       'source_license': 'project-original-public-content',
                       'contributor': 'T4 exact_pressure_1 package generator',
                       'contributor_certification': f'Invented test document from {case.reference_path}; no external project, material library or code data',
                       'redistribution_status': 'invented_non_engineering_example',
                       'review_status': 'prepared_for_independent_review'},
        'request_id': f'ep1-{slug(case.case_id)}'.replace('@', '-').replace('.', '-').lower(),
        'requested_outputs': ['result_envelope', 'audit_manifest', 'diagnostics'],
        'tbd_decisions': {'ci_provider': 'TBD', 'external_adapter_formats': 'TBD', 'filesystem_mutation_policy': 'SETTLED_DEC_065',
                          'final_cli_command_syntax': 'SETTLED_DEC_065', 'network_access': 'SETTLED_DEC_065',
                          'package_scripts': 'SETTLED_DEC_065', 'physical_project_container': 'TBD',
                          'process_invocation': 'SETTLED_DEC_065', 'public_transport_protocol': 'TBD', 'release_matrix': 'TBD'},
        'unit_system_ref': {'ref_id': 'invented-si', 'ref_type': 'unit_system'},
    }
    return {'request': request, 'solve': {'preview_model': case.document}}


def dump(value) -> bytes:
    return (json.dumps(value, indent=1, ensure_ascii=False, allow_nan=False) + '\n').encode('utf-8')


def admission() -> dict | None:
    """The admission record, written only after an independent review of this package.

    Absent: every reference is ``pending_independent_review`` and every profile a
    draft, so the gate refuses the package. Values and rules never change."""
    path = PACKAGE / 'ADMISSION.json'
    if not path.exists():
        return None
    record = json.loads(path.read_bytes())
    require(set(record) == {'format', 'status', 'independent_review', 'admitted_by'}, 'ADMISSION.json keys')
    require(record['format'] == ADMISSION_FORMAT and record['status'] == 'admitted', 'ADMISSION.json identity')
    review = record['independent_review']
    require(set(review) == {'path', 'sha256'}, 'ADMISSION.json review keys')
    relative = Path(review['path'])
    require(not relative.is_absolute() and '..' not in relative.parts, 'ADMISSION.json review path must be WORKING_ROOT-relative')
    require(sha256_bytes((WORKING_ROOT / relative).read_bytes()) == review['sha256'], 'ADMISSION.json review hash')
    return record


def cases() -> list[Case]:
    """Every case: the T4-I7 valued cases, then the T4-I8 owners with rows, each on 0.3.0 and 0.4.0."""
    semantics = Semantics()
    i7 = json.loads((WORKING_ROOT / I7).read_bytes())
    sketches = json.loads((WORKING_ROOT / I7_DOCUMENTS).read_bytes())['sketches']
    i8 = json.loads((WORKING_ROOT / I8).read_bytes())
    pi = Decimal(i8['numeric_representation']['pi_decimal'])
    built = []
    for case_key, case in i7['cases'].items():
        if case['expected'] is None:
            continue  # the mitre refusal control publishes nothing (core/product_physics/tests/t4_u2_pressure_references.rs)
        for version in VERSIONS:
            document = sketches[case_key][version]['document']
            require(document['model']['schema_version'] == version and document['model']['pressure_contract'] == PRESSURE_CONTRACT,
                    f'{case_key} {version}: document identity')
            built.append(i7_case(case_key, version, case, copy.deepcopy(document), semantics))
    for case_key, variant, owner in i8_owners(i8):
        if case_key in I8_NO_ROWS:
            continue
        for version in VERSIONS:
            built.append(i8_case(i8, case_key, variant, owner, version, pi, semantics))
    return built


def generate() -> dict[str, bytes]:
    """WORKING_ROOT-relative path -> bytes for every case file and the manifest."""
    admitted = admission()
    hashes = {path: sha256_bytes((WORKING_ROOT / path).read_bytes()) for path in (I7, I8)}
    outputs, entries = {}, []
    for case in cases():
        stem = f'{PACKAGE_PATH}/{CASES}/{case.case_id.replace("@", ".")}'
        request = runner_input(case)
        files = case.files(admitted, hashes[case.reference_path])
        paths = {'runner_input': (f'{stem}.runner_input.json', request),
                 'product_request': (f'{stem}.preview_request.json', request['solve']['preview_model']),
                 'selectors': (f'{stem}.selectors.json', files['selectors']),
                 'reference': (f'{stem}.reference.json', files['reference']),
                 'criteria': (f'{stem}.criteria.json', files['criteria'])}
        entry = {'case_id': case.case_id}
        for role, (path, value) in paths.items():
            outputs[path] = dump(value)
            entry[role] = {'path': path, 'sha256': sha256_bytes(outputs[path])}
        entry.update(analytical_reference={'path': case.reference_path, 'sha256': hashes[case.reference_path], 'case_key': case.reference_key},
                     modes=MODES, required_scalar_rows=len(case.positive))
        entries.append({key: entry[key] for key in ('case_id', 'runner_input', 'product_request', 'reference', 'selectors',
                                                     'criteria', 'analytical_reference', 'modes', 'required_scalar_rows')})
    outputs[f'{PACKAGE_PATH}/MANIFEST.json'] = dump({'format': MANIFEST_FORMAT, 'cases': entries})
    return outputs


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument('--write', action='store_true')
    group.add_argument('--check', action='store_true')
    args = parser.parse_args(argv)
    outputs = generate()
    if args.write:
        for path, data in outputs.items():
            (WORKING_ROOT / path).parent.mkdir(parents=True, exist_ok=True)
            (WORKING_ROOT / path).write_bytes(data)
        print(f'wrote {len(outputs)} files')
        return 0
    failures = absent = 0
    for path, data in outputs.items():
        target = WORKING_ROOT / path
        if not target.exists() and f'/{CASES}/' in path:
            absent += 1
        elif not target.exists() or target.read_bytes() != data:
            failures += 1
            print(f'DIFFERS {path}')
    print(f'{len(outputs)} files, {failures} differences' + (f', {absent} case files not generated' if absent else ''))
    return 1 if failures else 0

if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))
