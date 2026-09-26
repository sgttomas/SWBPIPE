"""Author the WP6 VP-STATIC load/reference-state case package (authoring only).

Written by the T1_WP6_STATIC_CASES TASK. Three phases, run from WORKING_ROOT
(``projects/chirality-piping``):

1. ``requests``: write ``<case>.preview_request.json`` and
   ``<case>.runner_input.json``. Every reference input is read from
   ``reference_cases.json`` (its ``value`` field, the authored decimal); every
   other number is an invented input and is labelled so in the object's
   provenance text.
2. ``selectors --inventory DIR``: resolve each row assertion to the product's
   row ID and metadata, using a row *inventory* (row id, kind, unit,
   entity_ref, basis_ref and metadata only) extracted from producer output
   of these requests. Identification only: no observed value is read or
   written. Evidence assertions need no inventory.
3. ``manifest``: write MANIFEST.json and PROVENANCE.json from the package
   files and their sha256.

Reference values and criteria are written by the package's own
``generate_reference_values.py``; this script never computes a target.
"""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import sys

HERE = Path(__file__).resolve()
WORKING_ROOT = HERE.parents[9]
PACKAGE_REL = 'validation/qualification/fixtures/load_reference'
PACKAGE = WORKING_ROOT / PACKAGE_REL
REFERENCE_REL = 'core/product_physics/tests/fixtures/load_reference_states/reference_cases.json'
TABLE_REL = 'fixtures/results/semantic_contract_v0_3_load_reference_1.json'
CONTRACT = 'openpipestress.result_semantics/0.3.0/load-reference-1'
STATE_CONTRACT = 'openpipestress.load_reference_state/1.0.0'
REFERENCE_ID = 'reference:installed'
EVIDENCE_NAMESPACE = 'contract_evidence.load_reference_states'
ALL = ['UX', 'UY', 'UZ', 'RX', 'RY', 'RZ']
REACTIONS = ['Fx', 'Fy', 'Fz', 'Mx', 'My', 'Mz']
STATIONS = ['end_i', 'quarter_1', 'midspan', 'quarter_3', 'end_j']
INVENTED = 'invented VP-STATIC load-reference qualification input; not library, component or code-rule data'
INVENTED_NU = 0.3  # invented; stated per material where it cannot enter a checked result

spec = importlib.util.spec_from_file_location('lr_generator', PACKAGE / 'generate_reference_values.py')
GEN = importlib.util.module_from_spec(spec)
spec.loader.exec_module(GEN)
REF, REF_BYTES = GEN.load_reference()
TABLE = json.loads((WORKING_ROOT / TABLE_REL).read_bytes())


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def node_at(pointer: str):
    return GEN.resolve(REF, pointer)


def q(pointer: str, unit: str) -> float:
    node = node_at(pointer)
    assert node['unit'] == unit, (pointer, node['unit'], unit)
    return node['value']


def text(pointer: str) -> str:
    node = node_at(pointer)
    assert isinstance(node, str), pointer
    return node


def ann(case: str, rest: str) -> str:
    return f'/cases/{case}/variants/annular_companion/{rest}'


def gen(case: str, rest: str) -> str:
    return f'/cases/{case}/variants/generic_reviewed/{rest}'


def var(case: str, variant: str, rest: str) -> str:
    return f'/cases/{case}/variants/{variant}/{rest}'


def qv(value, unit: str) -> dict:
    return {'value': value, 'unit': unit}


def refq(pointer: str, unit: str) -> dict:
    """Authored quantity taken from the reference file's authored decimal."""
    return qv(q(pointer, unit), unit)


# ---------------------------------------------------------------------------
# Request builder (CP2_WIRE.md and addenda)
# ---------------------------------------------------------------------------

OD = '/geometry/authored/outside_diameter'
WALL = '/geometry/authored/wall_thickness'


class Model:
    def __init__(self, project: str, name: str):
        self.project, self.name = project, name
        self.nodes, self.pipes, self.members, self.materials = [], [], [], []
        self.supports, self.cases = [], []

    def node(self, node_id: str, x: float, y: float = 0.0, note: str = ''):
        self.nodes.append({'id': node_id, 'position': {'x': x, 'y': y, 'z': 0.0},
                           'provenance': note or f'{INVENTED}; coordinates from the reference lengths'})
        return self

    def pipe(self, pipe_id, start, end, material, basis, fit, note=''):
        self.pipes.append({
            'id': pipe_id, 'from': start, 'to': end,
            'section': {'outside_diameter': refq(OD, 'm'), 'wall_thickness': refq(WALL, 'm')},
            'material': material, 'y_reference': {'x': 0.0, 'y': 1.0, 'z': 0.0},
            'provenance': 'section from reference_cases.json geometry annulus_od_0p20_wall_0p01; '
                          'y_reference is an authoring convention' + (f'; {note}' if note else '')})
        self.members.append({'pipe_ref': pipe_id, 'basis': basis, 'fit': fit,
                             'provenance': note or 'member reference from the reference case inputs'})
        return self

    def material(self, material: dict):
        self.materials.append(material)
        return self

    def support(self, support_id, node, restraints=ALL, note=''):
        self.supports.append({'id': support_id, 'node': node, 'family': 'anchor', 'restraints': list(restraints),
                              'provenance': note or f'{INVENTED}; rigid anchor representing the reference topology'})
        return self

    def case(self, case: dict):
        self.cases.append(case)
        return self

    def request(self) -> dict:
        return {'model': {
            'schema_version': '0.4.0',
            'document_kind': 'openpipestress.product_preview.model',
            'pressure_contract': {'version': '2.0.0', 'mode': 'exact_straight_pressure_v2'},
            'project': {'id': self.project, 'name': self.name,
                        'description': 'Invented VP-STATIC load/reference-state qualification request built from '
                                       'reference_cases.json; no libraries or code rules.',
                        'units': {'length': 'm', 'force': 'N', 'angle': 'rad', 'pressure': 'Pa',
                                  'stress': 'Pa', 'temperature': 'degC'}},
            'analysis_status': {'mechanics': 'ready_for_preview_diagnostics',
                                'rule_check': 'not_performed_user_rule_inputs_missing',
                                'professional_acceptance': 'not_provided'},
            'nodes': self.nodes,
            'pipe_segments': self.pipes,
            'supports': self.supports,
            'components': [],
            'materials': self.materials,
            'reference_configurations': [{
                'id': REFERENCE_ID, 'label': 'Installed reference',
                'geometry_ref': {'kind': 'authored_model_geometry'},
                'member_references': self.members, 'provenance': INVENTED}],
            'load_cases': self.cases,
            'combinations': []},
            'materials': []}


def material(mat_id, e, nu, note, points=None, laws=None):
    record = {'id': mat_id, 'constitutive_basis': 'homogeneous_isotropic_E_nu_v1',
              'elastic_modulus': e, 'poisson_ratio': nu, 'provenance': note}
    if points is not None:
        record['temperature_points'] = points
    if laws is not None:
        record['expansion_laws'] = laws
    return record


def point(point_id, temperature, e, nu, note):
    return {'id': point_id, 'temperature': temperature, 'elastic_modulus': e, 'poisson_ratio': nu,
            'provenance': note}


def basis_direct():
    return {'kind': 'direct_strain_reference'}


def basis_temperature(quantity):
    return {'kind': 'temperature_reference', 'installation_temperature': quantity}


def fit_none():
    return {'kind': 'none'}


def sel_base(mat_id):
    return {'kind': 'explicit_base_properties', 'material_ref': mat_id,
            'applicability_reference': 'invented analytical basis declared applicable to this member '
                                       '(reference_cases.json inputs)'}


def sel_point(mat_id, point_id):
    return {'kind': 'exact_point', 'material_ref': mat_id, 'point_ref': point_id}


def th_unchanged():
    return {'kind': 'unchanged_reference', 'provenance': 'reference state unchanged (reference strain zero)'}


def th_free(law):
    return {'kind': 'free_length_state', 'expansion_law_ref': law}


def element(pipe, selection, thermal, operating=None):
    state = {'pipe_ref': pipe, 'material_selection': selection, 'thermal_state': thermal}
    if operating is not None:
        state['operating_temperature'] = operating
    return state


def support_state(support, motions=()):
    state = {'support_ref': support, 'participation': {'kind': 'active_model_device'}}
    if motions:
        state['boundary_motion'] = [{'dof': dof, 'value': value, 'meaning': 'absolute_reference_displacement'}
                                    for dof, value in motions]
    return state


def load_case(case_id, elements, supports, primitives=(), sources=(), note=''):
    return {'id': case_id, 'pressure_regions': [], 'primitive_loads': list(primitives),
            'analysis_state': {'contract': STATE_CONTRACT, 'reference_configuration_ref': REFERENCE_ID,
                               'element_states': elements, 'support_states': supports,
                               'load_sources': [{'source_ref': s, 'factor': f} for s, f in sources],
                               'history': {'kind': 'independent_equilibrium'},
                               'provenance': note or INVENTED},
            'provenance': note or INVENTED}


def nodal_force(pid, node, magnitude, note):
    return {'id': pid, 'category': 'concentrated_force', 'target': {'type': 'node', 'node': node},
            'direction': 'global_x', 'magnitude': magnitude, 'dimension': 'force', 'provenance': note}


# ---------------------------------------------------------------------------
# Assertion specifications
# ---------------------------------------------------------------------------

def signature(kind, unit, component):
    rows = [r for r in TABLE['rows'] if r['kind'] == kind and r['unit'] == unit and r['component'] == component]
    assert len(rows) == 1, (kind, unit, component)
    return rows[0]


def an(pointer, unit, transform='identity', zero=None, **extra):
    origin = {'kind': 'analytical', 'pointer': pointer, 'reference_unit': unit, 'transform': transform}
    origin.update(extra)
    if zero is not None:
        origin['zero_scale'] = zero
    return origin


def eqz(reason, zero):
    return {'kind': 'equilibrium_zero', 'reason': reason, 'zero_scale': zero}


def scale(tag, pointer, unit, scale_unit=None, times=()):
    record = {'tag': tag, 'base': {'pointer': pointer, 'reference_unit': unit},
              'scale_unit': scale_unit or unit}
    if times:
        record['multiply_by_sum_of'] = [{'pointer': p, 'reference_unit': u} for p, u in times]
    return record


class Case:
    def __init__(self, case_id, key, family_label, request, load_state_decomposition):
        self.case_id, self.key, self.family_label = case_id, key, family_label
        self.request = request
        self.decomposition = load_state_decomposition
        self.assertions, self.negatives, self.gaps = [], [], []
        self.limits = []

    # rows -------------------------------------------------------------
    def row(self, aid, lc, kind, entity, component, location, unit, origin, why):
        sig = signature(kind, unit, component)
        self.assertions.append({'id': aid, 'lookup': {'lc': lc, 'kind': kind, 'entity': entity,
                                                      'component': component, 'location': location},
                                'unit': unit, 'dimension': sig['source_physical_semantic_dimension'],
                                'family': sig['family'], 'signature': sig['signature_id'],
                                'origin': origin, 'why': why})

    def disp(self, lc, node, axis, origin, why):
        self.row(f'{lc}.disp.{node}.u{axis}', lc, f'global_nodal_displacement_{axis}', node,
                 f'nodal_displacement_{axis}', 'node', 'mm', origin, why)

    def rot(self, lc, node, axis, origin, why):
        self.row(f'{lc}.rot.{node}.r{axis}', lc, f'global_nodal_rotation_{axis}', node,
                 f'nodal_rotation_{axis}', 'node', 'rad', origin, why)

    def reactions(self, lc, support, defined, force_zero, moment_zero, reason):
        for component in REACTIONS:
            unit = 'N' if component.startswith('F') else 'N*m'
            zero = force_zero if component.startswith('F') else moment_zero
            if component in defined:
                origin = copy.deepcopy(defined[component])
                if 'zero_scale' not in origin:
                    origin['zero_scale'] = zero
                why = 'reference-defined support-on-member action (all six reactions required, T1_PLAN WP6)'
            else:
                origin = eqz(reason, zero)
                why = 'component the reference topology leaves at zero by equilibrium (all six reactions required)'
            self.row(f'{lc}.reaction.{support}.{component}', lc, 'support_reaction_component_v2', support,
                     component, 'node', unit, origin, why)

    def axial(self, lc, pipe, pointer, zero, why):
        """Tension-positive N: stations and end_j carry N, end_i carries -N."""
        for location in STATIONS:
            transform = 'negate' if location == 'end_i' else 'identity'
            self.row(f'{lc}.axial.{pipe}.{location}', lc, 'element_local_axial_force', pipe, 'axial_force',
                     location, 'N', an(pointer, 'N', transform, zero), why)

    # evidence ---------------------------------------------------------
    def evidence(self, lc, record, key, field, unit, dimension, origin, why, meaning):
        keystr = ':'.join(str(v) for v in key.values())
        aid = f'{lc}.evidence.{record}.{keystr}.{field}'
        self.assertions.append({'id': aid, 'evidence': {
            'id': f'evidence:{lc}:{record}:{keystr}:{field}', 'namespace': EVIDENCE_NAMESPACE,
            'basis_ref': {'ref_type': 'load_case', 'ref_id': lc}, 'record': record, 'key': key,
            'field': field, 'definition': None, 'unit': unit, 'dimension': dimension},
            'family': EVIDENCE_FAMILIES[field], 'origin': origin, 'why': f'{why} ({meaning})'})

    def member(self, lc, pipe, field, origin, why):
        unit, dimension, meaning = MEMBER_FIELDS[field]
        self.evidence(lc, 'member', {'pipe_id': pipe}, field, unit, dimension, origin, why, meaning)

    def prescribed(self, lc, support, dof, origin, why):
        unit, dimension = ('m', 'length') if dof.startswith('U') else ('rad', 'angle')
        self.evidence(lc, 'support_component', {'support_id': support, 'dof': dof}, 'prescribed_value', unit,
                      dimension, origin, why, 'resolved rigid prescribed boundary value of this support DOF')

    def contribution(self, lc, source, origin, why):
        self.evidence(lc, 'contribution', {'owner_kind': 'stored_primitive', 'source_id': source},
                      'applied_magnitude', 'N', 'force', origin, why,
                      'factor times authored normalized magnitude of an included stored primitive')

    # negatives and gaps ----------------------------------------------
    def negative(self, target, name, origin):
        self.negatives.append({'target': target, 'name': name, 'origin': origin})

    def gap(self, gap_id, kind, detail, **extra):
        record = {'id': gap_id, 'kind': kind, 'detail': detail}
        record.update(extra)
        self.gaps.append(record)


EVIDENCE_FAMILIES = {
    'selected_E_pa': 'load_reference_material', 'selected_nu': 'load_reference_material',
    'derived_G_pa': 'load_reference_material',
    'installation_temperature_k': 'load_reference_temperature', 'operating_temperature_k': 'load_reference_temperature',
    'material_selection_temperature_k': 'load_reference_temperature', 'coefficient_datum_k': 'load_reference_temperature',
    'thermal_strain': 'load_reference_strain', 'thermal_stretch': 'load_reference_strain',
    'fit_strain': 'load_reference_strain', 'total_eigenstrain': 'load_reference_strain',
    'installation_datum_stretch': 'load_reference_strain', 'operating_datum_stretch': 'load_reference_strain',
    'reference_length_m': 'load_reference_geometry',
    'prescribed_value': 'load_reference_support_motion', 'applied_magnitude': 'load_reference_applied_load',
}

MEMBER_FIELDS = {
    'selected_E_pa': ('Pa', 'stress', "member's selected elastic modulus"),
    'selected_nu': ('1', 'dimensionless', "member's selected Poisson ratio"),
    'derived_G_pa': ('Pa', 'stress', 'shear modulus derived E/[2(1+nu)] from the selected pair'),
    'thermal_strain': ('1', 'dimensionless', 'resolved thermal strain lambda_th-1 from installation to state'),
    'thermal_stretch': ('1', 'dimensionless', 'resolved thermal stretch lambda_th'),
    'fit_strain': ('1', 'dimensionless', 'resolved fit strain lambda_fit-1'),
    'total_eigenstrain': ('1', 'dimensionless', 'composed eigenstrain lambda_fit*lambda_th-1'),
    'installation_datum_stretch': ('1', 'dimensionless', 'datum stretch lambda(T_install) of the expansion law'),
    'operating_datum_stretch': ('1', 'dimensionless', 'datum stretch lambda(T) of the expansion law'),
    'installation_temperature_k': ('K', 'temperature', 'installation temperature in kelvin'),
    'operating_temperature_k': ('K', 'temperature', 'operating temperature in kelvin'),
    'material_selection_temperature_k': ('K', 'temperature', 'temperature of the selected material data'),
    'coefficient_datum_k': ('K', 'temperature', 'expansion-law datum temperature in kelvin'),
    'reference_length_m': ('m', 'length', 'reference (authored chord) length of the member'),
}


# ---------------------------------------------------------------------------
# Cases
# ---------------------------------------------------------------------------

CASES: list[Case] = []


def add(case: Case) -> Case:
    CASES.append(case)
    return case


def reaction_scales(force_ptr, force_unit, lengths, suffix=''):
    tail = f'.{suffix}' if suffix else ''
    return (scale('case_force' + tail, force_ptr, force_unit),
            scale('case_moment' + tail, force_ptr, force_unit, 'N*m', lengths))


def author_two_bar():
    c = 'prescribed_translation_two_bar'
    L1, L2 = ann(c, 'inputs/L1'), ann(c, 'inputs/L2')
    lc = 'case:prescribed-root-translation'
    m = (Model('project:lr-prescribed-translation-two-bar', 'Prescribed translation, two bars')
         .node('node:root', 0.0).node('node:middle', q(L1, 'm')).node('node:far', q(L1, 'm') + q(L2, 'm'))
         .material(material('material:bar-1', refq(ann(c, 'inputs/E1'), 'Pa'), qv(INVENTED_NU, '1'),
                            f'E from {ann(c, "inputs/E1")}; nu invented (enters no checked axial result)'))
         .material(material('material:bar-2', refq(ann(c, 'inputs/E2'), 'Pa'), qv(INVENTED_NU, '1'),
                            f'E from {ann(c, "inputs/E2")}; nu invented (enters no checked axial result)'))
         .pipe('pipe:1', 'node:root', 'node:middle', 'material:bar-1', basis_direct(), fit_none())
         .pipe('pipe:2', 'node:middle', 'node:far', 'material:bar-2', basis_direct(), fit_none())
         .support('support:root', 'node:root').support('support:far', 'node:far'))
    m.case(load_case(lc, [element('pipe:1', sel_base('material:bar-1'), th_unchanged()),
                          element('pipe:2', sel_base('material:bar-2'), th_unchanged())],
                     [support_state('support:root', [('UX', refq(ann(c, 'inputs/root_UX'), 'm'))]),
                      support_state('support:far', [('UX', refq(ann(c, 'inputs/far_UX'), 'm'))])]))
    k = add(Case(c, c, 'support_motion', m.request(), 'one load case; one state'))
    length_zero = scale('prescribed_translation', ann(c, 'inputs/root_UX'), 'm')
    k.disp(lc, 'node:root', 'x', an(ann(c, 'inputs/root_UX'), 'm'), 'complete u includes the prescribed value')
    k.disp(lc, 'node:middle', 'x', an(ann(c, 'expected/middle_UX'), 'm'), 'free DOF coupled to the prescribed motion')
    k.disp(lc, 'node:far', 'x', an(ann(c, 'inputs/far_UX'), 'm', zero=length_zero), 'explicit zero prescribed value')
    k.prescribed(lc, 'support:root', 'UX', an(ann(c, 'inputs/root_UX'), 'm'), 'resolved support motion owner')
    k.prescribed(lc, 'support:far', 'UX', an(ann(c, 'inputs/far_UX'), 'm', zero=length_zero), 'explicit zero motion')
    force_zero, moment_zero = reaction_scales(ann(c, 'expected/root_Fx'), 'N', [(L1, 'm'), (L2, 'm')])
    k.axial(lc, 'pipe:1', ann(c, 'expected/member1_N'), force_zero, 'member force of bar 1 at every station')
    k.axial(lc, 'pipe:2', ann(c, 'expected/member2_N'), force_zero, 'member force of bar 2 at every station')
    reason = 'straight bar on global X with axial prescribed motion only: no transverse force or couple'
    k.reactions(lc, 'support:root', {'Fx': an(ann(c, 'expected/root_Fx'), 'N')}, force_zero, moment_zero, reason)
    k.reactions(lc, 'support:far', {'Fx': an(ann(c, 'expected/far_Fx'), 'N')}, force_zero, moment_zero, reason)
    k.negative(f'{lc}.disp.node:middle.ux', 'omitted_Kfc_gc',
               an(f'/cases/{c}/wrong_result_discriminators/omitted_Kfc_gc/middle_UX', 'm', zero=length_zero))
    k.gap('free_rhs_after_prescribed_coupling', 'not_published',
          'The reduced free right-hand side after prescribed coupling is an internal solve quantity; no result '
          'row or evidence field publishes it. Its effect is pinned by middle UX and the reactions.',
          pointer=ann(c, 'expected/free_rhs_after_prescribed_coupling'))


def author_all_fixed():
    c = 'prescribed_translation_all_fixed'
    L = ann(c, 'inputs/L')
    lc = 'case:prescribed-root-translation'
    m = (Model('project:lr-prescribed-translation-all-fixed', 'Prescribed translation, all DOFs fixed')
         .node('node:root', 0.0).node('node:far', q(L, 'm'))
         .material(material('material:bar', refq(ann(c, 'inputs/E'), 'Pa'), qv(INVENTED_NU, '1'),
                            f'E from {ann(c, "inputs/E")}; nu invented (enters no checked axial result)'))
         .pipe('pipe:bar', 'node:root', 'node:far', 'material:bar', basis_direct(), fit_none())
         .support('support:root', 'node:root').support('support:far', 'node:far'))
    m.case(load_case(lc, [element('pipe:bar', sel_base('material:bar'), th_unchanged())],
                     [support_state('support:root', [('UX', refq(ann(c, 'inputs/root_UX'), 'm'))]),
                      support_state('support:far', [('UX', refq(ann(c, 'inputs/far_UX'), 'm'))])]))
    k = add(Case(c, c, 'support_motion', m.request(), 'one load case; one state; zero free DOFs'))
    length_zero = scale('prescribed_translation', ann(c, 'inputs/root_UX'), 'm')
    k.disp(lc, 'node:root', 'x', an(ann(c, 'inputs/root_UX'), 'm'), 'complete u includes the prescribed value')
    k.disp(lc, 'node:far', 'x', an(ann(c, 'inputs/far_UX'), 'm', zero=length_zero), 'explicit zero prescribed value')
    k.prescribed(lc, 'support:root', 'UX', an(ann(c, 'inputs/root_UX'), 'm'), 'resolved support motion owner')
    k.prescribed(lc, 'support:far', 'UX', an(ann(c, 'inputs/far_UX'), 'm', zero=length_zero), 'explicit zero motion')
    force_zero, moment_zero = reaction_scales(ann(c, 'expected/root_Fx'), 'N', [(L, 'm')])
    k.axial(lc, 'pipe:bar', ann(c, 'expected/N'), force_zero, 'member force at every station')
    reason = 'straight bar on global X with axial prescribed motion only: no transverse force or couple'
    k.reactions(lc, 'support:root', {'Fx': an(ann(c, 'expected/root_Fx'), 'N')}, force_zero, moment_zero, reason)
    k.reactions(lc, 'support:far', {'Fx': an(ann(c, 'expected/far_Fx'), 'N')}, force_zero, moment_zero, reason)
    k.gap('zero_size_free_problem_discarded', 'implied_by_positive',
          'Text discriminator (a zero-size free system does not imply a zero result). It is excluded whenever the '
          'nonzero reaction and member-force assertions pass.',
          pointer=f'/cases/{c}/wrong_result_discriminators/zero_size_free_problem_discarded',
          assertions=[f'{lc}.reaction.support:root.Fx', f'{lc}.reaction.support:far.Fx',
                      f'{lc}.axial.pipe:bar.midspan'])


def rotation_model(c, project, far_anchor):
    L = ann(c, 'inputs/L')
    m = (Model(project, f'Prescribed rotation ({c})')
         .node('node:root', 0.0).node('node:far', q(L, 'm'))
         .material(material('material:beam', refq(ann(c, 'inputs/E'), 'Pa'), qv(INVENTED_NU, '1'),
                            f'E from {ann(c, "inputs/E")}; nu invented (enters no checked in-plane result)'))
         .pipe('pipe:beam', 'node:root', 'node:far', 'material:beam', basis_direct(), fit_none())
         .support('support:root', 'node:root'))
    supports = [support_state('support:root', [('UY', refq(ann(c, 'inputs/root_UY'), 'm')),
                                               ('RZ', refq(ann(c, 'inputs/root_RZ'), 'rad'))])]
    if far_anchor:
        m.support('support:far', 'node:far')
        supports.append(support_state('support:far', [('UY', refq(ann(c, 'inputs/far_UY'), 'm')),
                                                      ('RZ', refq(ann(c, 'inputs/far_RZ'), 'rad'))]))
    return m, supports


def author_rotation_all_fixed():
    c = 'prescribed_rotation_all_fixed'
    lc = 'case:prescribed-root-rotation'
    m, supports = rotation_model(c, 'project:lr-prescribed-rotation-all-fixed', True)
    m.case(load_case(lc, [element('pipe:beam', sel_base('material:beam'), th_unchanged())], supports))
    k = add(Case(c, c, 'support_motion', m.request(), 'one load case; one state'))
    L = ann(c, 'inputs/L')
    angle_zero = scale('prescribed_rotation', ann(c, 'inputs/root_RZ'), 'rad')
    length_zero = scale('rotation_times_length', ann(c, 'inputs/root_RZ'), 'rad', 'm', [(L, 'm')])
    k.rot(lc, 'node:root', 'z', an(ann(c, 'inputs/root_RZ'), 'rad'), 'complete u includes the prescribed rotation')
    k.rot(lc, 'node:far', 'z', an(ann(c, 'inputs/far_RZ'), 'rad', zero=angle_zero), 'explicit zero prescribed value')
    k.disp(lc, 'node:root', 'y', an(ann(c, 'inputs/root_UY'), 'm', zero=length_zero), 'explicit zero prescribed value')
    k.disp(lc, 'node:far', 'y', an(ann(c, 'inputs/far_UY'), 'm', zero=length_zero), 'explicit zero prescribed value')
    for support, dof, name, zero in [('support:root', 'RZ', 'root_RZ', None), ('support:root', 'UY', 'root_UY', length_zero),
                                     ('support:far', 'RZ', 'far_RZ', angle_zero), ('support:far', 'UY', 'far_UY', length_zero)]:
        unit = 'rad' if dof == 'RZ' else 'm'
        zero_here = zero
        if zero is not None and unit == 'm':
            zero_here = dict(zero)  # evidence is in metres
        k.prescribed(lc, support, dof, an(ann(c, f'inputs/{name}'), unit, zero=zero_here), 'resolved support motion')
    force_zero = scale('case_force', ann(c, 'expected/root_Fy'), 'N')
    moment_zero = scale('case_moment', ann(c, 'expected/root_Mz'), 'N*m')
    reason = 'beam bending in the global X-Y plane only: no axial force, out-of-plane force or other couples'
    k.reactions(lc, 'support:root', {'Fy': an(ann(c, 'expected/root_Fy'), 'N'),
                                     'Mz': an(ann(c, 'expected/root_Mz'), 'N*m')}, force_zero, moment_zero, reason)
    k.reactions(lc, 'support:far', {'Fy': an(ann(c, 'expected/far_Fy'), 'N'),
                                    'Mz': an(ann(c, 'expected/far_Mz'), 'N*m')}, force_zero, moment_zero, reason)
    why = ('element end action equals the reference support-on-member vector (single element between the two '
           'supports; element-local frame equals global for a +X member with y_reference +Y)')
    k.row(f'{lc}.end_force.pipe:beam.end_i.shear_y', lc, 'element_local_shear_force_y', 'pipe:beam', 'shear_force_y',
          'end_i', 'N', an(ann(c, 'expected/root_Fy'), 'N'), why)
    k.row(f'{lc}.end_force.pipe:beam.end_i.bending_z', lc, 'element_local_bending_moment_z', 'pipe:beam',
          'bending_moment_z', 'end_i', 'N*m', an(ann(c, 'expected/root_Mz'), 'N*m'), why)
    k.row(f'{lc}.end_force.pipe:beam.end_j.shear_y', lc, 'element_local_shear_force_y', 'pipe:beam', 'shear_force_y',
          'end_j', 'N', an(ann(c, 'expected/far_Fy'), 'N'), why)
    k.row(f'{lc}.end_force.pipe:beam.end_j.bending_z', lc, 'element_local_bending_moment_z', 'pipe:beam',
          'bending_moment_z', 'end_j', 'N*m', an(ann(c, 'expected/far_Mz'), 'N*m'), why)
    k.gap('translation_substitution_or_length_normalized_angle', 'implied_by_positive',
          'Text discriminator: all four signed reactions and their balance must be reproduced. Excluded whenever '
          'the four reaction and end-action assertions pass.',
          pointer=f'/cases/{c}/wrong_result_discriminators/translation_substitution_or_length_normalized_angle',
          assertions=[f'{lc}.reaction.support:root.Fy', f'{lc}.reaction.support:root.Mz',
                      f'{lc}.reaction.support:far.Fy', f'{lc}.reaction.support:far.Mz'])


def author_rotation_free_tip():
    c = 'prescribed_rotation_free_tip'
    fixed = 'prescribed_rotation_all_fixed'
    lc = 'case:prescribed-root-rotation'
    m, supports = rotation_model(c, 'project:lr-prescribed-rotation-free-tip', False)
    m.case(load_case(lc, [element('pipe:beam', sel_base('material:beam'), th_unchanged())], supports))
    k = add(Case(c, c, 'support_motion', m.request(), 'one load case; one state'))
    tip_zero = scale('tip_translation', ann(c, 'expected/tip_UY'), 'm')
    tip_angle_zero = scale('tip_rotation', ann(c, 'expected/tip_RZ'), 'rad')
    # Exact-zero actions: the same beam's all-fixed end actions are its natural magnitudes.
    force_zero = scale('all_fixed_force', ann(fixed, 'expected/root_Fy'), 'N')
    moment_zero = scale('all_fixed_moment', ann(fixed, 'expected/root_Mz'), 'N*m')
    k.disp(lc, 'node:far', 'y', an(ann(c, 'expected/tip_UY'), 'm'), 'rigid first-order tip translation L*theta')
    k.rot(lc, 'node:far', 'z', an(ann(c, 'expected/tip_RZ'), 'rad'), 'rigid tip rotation in radians')
    k.rot(lc, 'node:root', 'z', an(ann(c, 'inputs/root_RZ'), 'rad'), 'complete u includes the prescribed rotation')
    k.disp(lc, 'node:root', 'y', an(ann(c, 'inputs/root_UY'), 'm', zero=tip_zero), 'explicit zero prescribed value')
    k.prescribed(lc, 'support:root', 'RZ', an(ann(c, 'inputs/root_RZ'), 'rad'), 'resolved support motion')
    k.prescribed(lc, 'support:root', 'UY', an(ann(c, 'inputs/root_UY'), 'm', zero=tip_zero), 'explicit zero motion')
    reason = 'stress-free rigid rotation: the free cantilever carries no action'
    k.reactions(lc, 'support:root', {'Fy': an(ann(c, 'expected/root_Fy'), 'N'),
                                     'Mz': an(ann(c, 'expected/root_Mz'), 'N*m')}, force_zero, moment_zero, reason)
    k.axial(lc, 'pipe:beam', ann(c, 'expected/wall_N'), force_zero, 'stress-free rotation: zero wall force')
    d = f'/cases/{c}/wrong_result_discriminators'
    k.negative(f'{lc}.disp.node:far.uy', 'missing_rotation_coupling',
               an(f'{d}/missing_rotation_coupling/wrong_tip_UY', 'm', zero=tip_zero))
    k.negative(f'{lc}.disp.node:far.uy', 'length_normalized_angle',
               an(f'{d}/length_normalized_angle/wrong_tip_UY', 'm'))
    k.negative(f'{lc}.rot.node:far.rz', 'length_normalized_angle',
               an(f'{d}/length_normalized_angle/wrong_tip_RZ', 'rad'))
    del tip_angle_zero


def author_serial():
    c = 'shared_material_serial_companion'
    L1, L2 = ann(c, 'inputs/L1'), ann(c, 'inputs/L2')
    shared = text(ann(c, 'inputs/material_id'))
    cold = text(ann(c, 'inputs/material_point_cold'))
    hot = text(ann(c, 'inputs/material_point_hot'))
    lc = 'case:shared-material'
    points = [
        point(cold, qv(20.0, 'degC'), refq(ann(c, 'inputs/E1'), 'Pa'), refq(ann(c, 'inputs/nu1'), '1'),
              f'E/nu from {ann(c, "inputs/E1")} and nu1; point temperature 20 degC invented (the reference gives none)'),
        point(hot, qv(300.0, 'degC'), refq(ann(c, 'inputs/E2'), 'Pa'), refq(ann(c, 'inputs/nu2'), '1'),
              f'E/nu from {ann(c, "inputs/E2")} and nu2; point temperature 300 degC invented (the reference gives none)'),
    ]
    m = (Model('project:lr-shared-material-serial', 'Shared material, serial companion')
         .node('node:root', 0.0).node('node:middle', q(L1, 'm')).node('node:far', q(L1, 'm') + q(L2, 'm'))
         .material(material(shared, qv(170.0e9, 'Pa'), qv(0.28, '1'),
                            'base E/nu invented and selected by no member; the two points carry the reference data',
                            points=points))
         .pipe('pipe:1', 'node:root', 'node:middle', shared, basis_direct(), fit_none())
         .pipe('pipe:2', 'node:middle', 'node:far', shared, basis_direct(), fit_none())
         .support('support:root', 'node:root').support('support:far', 'node:far'))
    m.case(load_case(lc, [
        element('pipe:1', sel_point(shared, cold), th_unchanged()),
        element('pipe:2', sel_point(shared, hot),
                {'kind': 'explicit_interval_strain', 'strain': refq(ann(c, 'inputs/epsilon2'), '1'),
                 'interval_reference': 'reference_cases.json shared_material_serial_companion epsilon2',
                 'provenance': 'direct strain input; not a temperature inferred from a property point'})],
        [support_state('support:root', [('UX', refq(ann(c, 'inputs/root_UX'), 'm'))]),
         support_state('support:far', [('UX', refq(ann(c, 'inputs/far_UX'), 'm'))])]))
    k = add(Case(c, c, 'reference_temperatures', m.request(), 'one load case; one state'))
    k.limits.append('Practical companion of the analytical-only shared_material_parallel topology (two coincident '
                    'parallel bars). The parallel topology is not authored; see the package README.')
    length_zero = scale('middle_translation', ann(c, 'expected/middle_UX'), 'm')
    k.disp(lc, 'node:middle', 'x', an(ann(c, 'expected/middle_UX'), 'm'), 'per-member selected E and eigenstrain')
    k.disp(lc, 'node:root', 'x', an(ann(c, 'inputs/root_UX'), 'm', zero=length_zero), 'explicit zero prescribed value')
    k.disp(lc, 'node:far', 'x', an(ann(c, 'inputs/far_UX'), 'm', zero=length_zero), 'explicit zero prescribed value')
    force_zero, moment_zero = reaction_scales(ann(c, 'expected/root_Fx'), 'N', [(L1, 'm'), (L2, 'm')])
    k.axial(lc, 'pipe:1', ann(c, 'expected/member1_N'), force_zero, 'member force of bar 1 at every station')
    k.axial(lc, 'pipe:2', ann(c, 'expected/member2_N'), force_zero, 'member force of bar 2 at every station')
    reason = 'straight bars on global X with axial eigenstrain only: no transverse force or couple'
    k.reactions(lc, 'support:root', {'Fx': an(ann(c, 'expected/root_Fx'), 'N')}, force_zero, moment_zero, reason)
    k.reactions(lc, 'support:far', {'Fx': an(ann(c, 'expected/far_Fx'), 'N')}, force_zero, moment_zero, reason)
    strain_zero = scale('member_strain', ann(c, 'inputs/epsilon2'), '1')
    for pipe, n in [('pipe:1', '1'), ('pipe:2', '2')]:
        k.member(lc, pipe, 'selected_E_pa', an(ann(c, f'inputs/E{n}'), 'Pa'), 'per-member point selection')
        k.member(lc, pipe, 'selected_nu', an(ann(c, f'inputs/nu{n}'), '1'), 'per-member point selection')
        k.member(lc, pipe, 'derived_G_pa', an(ann(c, f'expected/G{n}'), 'Pa'), 'G derived from the selected pair')
        k.member(lc, pipe, 'thermal_strain', an(ann(c, f'inputs/epsilon{n}'), '1', zero=strain_zero),
                 'direct interval strain input resolved per member')
        k.member(lc, pipe, 'total_eigenstrain', an(ann(c, f'inputs/epsilon{n}'), '1', zero=strain_zero),
                 'no fit: total eigenstrain equals the member strain input')
    k.negative(f'{lc}.disp.node:middle.ux', 'material_ID_only_E_override',
               an(f'/cases/{c}/wrong_result_discriminators/material_ID_only_E_override/wrong_middle_UX', 'm'))
    for pipe, point_id in [('pipe:1', cold), ('pipe:2', hot)]:
        k.gap(f'selected_point.{pipe}', 'structural_expectation_not_scored',
              'Non-numeric expectation: consumed_material_points == [point_id "' + point_id + '"]. '
              'material_selection_kind == "exact_point" is carried as a definition constraint of every member '
              'evidence selector; the numeric E/nu/G assertions discriminate the points.', pipe_id=pipe, point_id=point_id)


def thermal_request(c, fixed):
    L = ann(c, 'inputs/L')
    three = ann(c, 'inputs/table_points')
    two = var(c, 'verification_two_point', 'inputs/table_points')

    def table(pointer):
        return [{'temperature': refq(f'{pointer}/{i}/temperature', 'degC'),
                 'coefficient': refq(f'{pointer}/{i}/alpha', '1/K')} for i in range(len(node_at(pointer)))]

    laws = [
        {'id': 'law:three-point', 'definition': text(ann(c, 'inputs/coefficient_definition')),
         'datum_temperature': refq(ann(c, 'inputs/datum_temperature'), 'degC'),
         'data': {'kind': 'table', 'interpolation': text(ann(c, 'inputs/interpolation')), 'points': table(three)},
         'provenance': f'annular companion table {three}; its 20 degC point is the reference\'s own invented coverage point'},
        {'id': 'law:two-point',
         'definition': text(var(c, 'verification_two_point', 'inputs/coefficient_definition')),
         'datum_temperature': refq(var(c, 'verification_two_point', 'inputs/datum_temperature'), 'degC'),
         'data': {'kind': 'table',
                  'interpolation': text(var(c, 'verification_two_point', 'inputs/interpolation')),
                  'points': table(two)},
         'provenance': f'VERIFICATION control-4 exact two-point table {two}'},
    ]
    m = (Model(f'project:lr-thermal-datum-ratio-{"fixed" if fixed else "free"}',
               f'Thermal datum ratio, {"fixed" if fixed else "free"} bar')
         .node('node:root', 0.0).node('node:far', q(L, 'm'))
         .material(material('material:thermal', refq(ann(c, 'inputs/E'), 'Pa'), qv(INVENTED_NU, '1'),
                            f'E from {ann(c, "inputs/E")} (both variants carry the same E); nu invented '
                            '(enters no checked axial result)', laws=laws))
         .pipe('pipe:thermal', 'node:root', 'node:far', 'material:thermal',
               basis_temperature(refq(ann(c, 'inputs/installation_temperature'), 'degC')), fit_none())
         .support('support:root', 'node:root'))
    if fixed:
        m.support('support:far', 'node:far')
    supports = [support_state('support:root')] + ([support_state('support:far')] if fixed else [])
    for lc, law, operating in [
        ('case:annular-three-point', 'law:three-point', refq(ann(c, 'inputs/operating_temperature'), 'degC')),
        ('case:verification-two-point', 'law:two-point',
         refq(var(c, 'verification_two_point', 'inputs/operating_temperature'), 'degC')),
        ('case:annular-three-point-kelvin', 'law:three-point', refq(ann(c, 'expected/operating_temperature_K'), 'K')),
    ]:
        m.case(load_case(lc, [element('pipe:thermal', sel_base('material:thermal'), th_free(law), operating)],
                         copy.deepcopy(supports)))
    return m.request()


THERMAL_CASES = [('case:annular-three-point', 'annular_companion'),
                 ('case:verification-two-point', 'verification_two_point'),
                 ('case:annular-three-point-kelvin', 'annular_companion')]


def author_thermal(fixed):
    c = 'thermal_datum_ratio'
    label = 'fixed' if fixed else 'free'
    k = add(Case(f'{c}.{label}', c, 'reference_temperatures', thermal_request(c, fixed),
                 'three load cases in one request: the three-point annular table, the reviewed two-point table, '
                 'and the three-point table with the operating temperature authored in K'))
    L = ann(c, 'inputs/L')
    d = f'/cases/{c}/wrong_result_discriminators'
    for lc, variant in THERMAL_CASES:
        p = lambda rest: var(c, variant, rest)  # noqa: E731
        force_zero, moment_zero = reaction_scales(p('expected/fixed_wall_N'), 'N', [(L, 'm')], variant)
        reason = 'straight bar on global X with axial eigenstrain only: no transverse force or couple'
        if fixed:
            k.axial(lc, 'pipe:thermal', p('expected/fixed_wall_N'), force_zero, 'restrained thermal wall force')
            k.reactions(lc, 'support:root', {'Fx': an(p('expected/fixed_root_Fx'), 'N')}, force_zero, moment_zero, reason)
            k.reactions(lc, 'support:far', {'Fx': an(p('expected/fixed_far_Fx'), 'N')}, force_zero, moment_zero, reason)
        else:
            k.disp(lc, 'node:far', 'x', an(p('expected/free_tip_UX'), 'm'), 'free thermal elongation')
            k.axial(lc, 'pipe:thermal', p('expected/free_wall_N'), force_zero, 'free bar: zero wall force')
            k.reactions(lc, 'support:root', {}, force_zero, moment_zero,
                        'free cantilever with self-equilibrated eigenstrain: no support action')
        k.member(lc, 'pipe:thermal', 'thermal_strain', an(p('expected/thermal_strain'), '1'), 'datum-ratio strain')
        k.member(lc, 'pipe:thermal', 'thermal_stretch', an(p('expected/thermal_stretch'), '1'), 'datum-ratio stretch')
        k.member(lc, 'pipe:thermal', 'installation_datum_stretch', an(p('expected/dilation_install'), '1', 'one_plus'),
                 'lambda(T_install) = 1 + alpha_sec(T_install)*(T_install - T_m)')
        k.member(lc, 'pipe:thermal', 'operating_datum_stretch', an(p('expected/dilation_operating'), '1', 'one_plus'),
                 'lambda(T) = 1 + alpha_sec(T)*(T - T_m)')
        k.member(lc, 'pipe:thermal', 'installation_temperature_k', an(p('expected/installation_temperature_K'), 'K'),
                 'absolute temperature conversion includes +273.15')
        k.member(lc, 'pipe:thermal', 'operating_temperature_k', an(p('expected/operating_temperature_K'), 'K'),
                 'absolute temperature conversion; K and degC authoring are identical')
        k.member(lc, 'pipe:thermal', 'coefficient_datum_k', an(p('expected/datum_temperature_K'), 'K'),
                 'law datum temperature')
        k.member(lc, 'pipe:thermal', 'selected_E_pa', an(p('inputs/E'), 'Pa'), 'explicit base modulus')
        for name in ('alpha_hot_times_operating_minus_install', 'subtract_datum_dilations'):
            k.negative(f'{lc}.evidence.member.pipe:thermal.thermal_strain', name, an(f'{d}/{name}', '1'))


def author_coefficient_definition():
    c = 'coefficient_definition'
    points_ptr = gen(c, 'inputs/linear_coefficient_points')
    points = [{'temperature': refq(f'{points_ptr}/{i}/temperature', 'degC'),
               'coefficient': refq(f'{points_ptr}/{i}/coefficient', '1/K')} for i in range(len(node_at(points_ptr)))]
    datum = refq(gen(c, 'inputs/datum_temperature'), 'degC')
    laws = [{'id': f'law:{name}', 'definition': definition, 'datum_temperature': datum,
             'data': {'kind': 'table', 'interpolation': 'linear_coefficient', 'points': copy.deepcopy(points)},
             'provenance': f'{points_ptr}; the same numeric table denotes distinct data under each definition'}
            for name, definition in [('datum-length', 'differential_per_datum_length'),
                                     ('current-length', 'logarithmic_per_current_length')]]
    t_install = refq(gen(c, 'inputs/installation_temperature'), 'degC')
    t_operating = refq(gen(c, 'inputs/operating_temperature'), 'degC')
    assert t_install['value'] == 20.0 and t_operating['value'] == 120.0
    t_mid = qv((t_install['value'] + t_operating['value']) / 2, 'degC')
    m = Model('project:lr-coefficient-definition', 'Coefficient definition (datum/current length)')
    m.material(material('material:coefficient', qv(200.0e9, 'Pa'), qv(INVENTED_NU, '1'),
                        'E/nu invented: this reference defines strains only, no mechanics', laws=laws))
    pipes = [('pipe:install-low', t_install), ('pipe:install-high', t_operating), ('pipe:install-mid', t_mid)]
    for index, (pipe, installation) in enumerate(pipes):
        root, tip = f'node:{pipe[5:]}-root', f'node:{pipe[5:]}-tip'
        layout = f'{INVENTED}; separate 1 m cantilever per installation temperature (layout invented)'
        m.node(root, 0.0, float(index), layout).node(tip, 1.0, float(index), layout)
        note = ('installation temperature is the midpoint of the reference interval, implied by its '
                'first_half/second_half quantities' if pipe.endswith('mid') else
                'installation temperature from the reference inputs')
        m.pipe(pipe, root, tip, 'material:coefficient', basis_temperature(installation), fit_none(), note)
        m.support(f'support:{pipe[5:]}', root)
    supports = [support_state(f'support:{p[5:]}') for p, _ in pipes]
    k = add(Case(c, c, 'reference_temperatures', None,
                 'four load cases in one request: each definition with (a) forward, reverse and second-half '
                 'intervals and (b) the first-half interval; each interval is its own member'))
    d = f'/cases/{c}/wrong_result_discriminators'
    for law, prefix in [('datum-length', 'datum_length_strain'), ('current-length', 'current_length_strain')]:
        a, b = f'case:{law}-a', f'case:{law}-b'
        m.case(load_case(a, [element('pipe:install-low', sel_base('material:coefficient'), th_free(f'law:{law}'), t_operating),
                             element('pipe:install-high', sel_base('material:coefficient'), th_free(f'law:{law}'), t_install),
                             element('pipe:install-mid', sel_base('material:coefficient'), th_free(f'law:{law}'), t_operating)],
                         copy.deepcopy(supports)))
        m.case(load_case(b, [element('pipe:install-low', sel_base('material:coefficient'), th_free(f'law:{law}'), t_mid),
                             element('pipe:install-high', sel_base('material:coefficient'), th_unchanged()),
                             element('pipe:install-mid', sel_base('material:coefficient'), th_unchanged())],
                         copy.deepcopy(supports)))
        for lc, pipe, suffix in [(a, 'pipe:install-low', 'forward'), (a, 'pipe:install-high', 'reverse'),
                                 (a, 'pipe:install-mid', 'second_half'), (b, 'pipe:install-low', 'first_half')]:
            k.member(lc, pipe, 'thermal_strain', an(gen(c, f'expected/{prefix}_{suffix}'), '1'),
                     f'{law} strain over the {suffix} interval')
        k.negative(f'{a}.evidence.member.pipe:install-low.thermal_strain', 'endpoint_alpha_times_interval',
                   an(f'{d}/endpoint_alpha_times_interval', '1'))
        wrong = 'wrong_datum_reverse' if law == 'datum-length' else 'wrong_log_reverse'
        k.negative(f'{a}.evidence.member.pipe:install-high.thermal_strain', 'reverse_by_negation',
                   an(f'{d}/reverse_by_negation/{wrong}', '1'))
    k.request = m.request()
    k.limits.append('Strain-only reference: no mechanics companion values exist, so no reaction or member-force '
                    'assertion is made (the six-reaction rule applies where the reference defines mechanics).')
    k.limits.append('The 70 degC installation temperature of pipe:install-mid is the midpoint implied by the '
                    'reference first_half/second_half quantities; it is not a separate reference quantity.')


def author_constant_alpha():
    c = 'constant_alpha_interval'
    m = (Model('project:lr-constant-alpha-interval', 'Constant alpha interval')
         .node('node:root', 0.0, 0.0, f'{INVENTED}; 1 m cantilever (length invented)')
         .node('node:tip', 1.0, 0.0, f'{INVENTED}; 1 m cantilever (length invented)')
         .material(material('material:interval', qv(200.0e9, 'Pa'), qv(INVENTED_NU, '1'),
                            'E/nu invented: this reference defines strains only, no mechanics'))
         .pipe('pipe:interval', 'node:root', 'node:tip', 'material:interval', basis_direct(), fit_none())
         .support('support:root', 'node:root'))
    alpha = q(gen(c, 'inputs/alpha'), '1/K')
    change = q(gen(c, 'inputs/deltaT'), 'K')
    meaning = text(gen(c, 'inputs/meaning'))
    for lc, a_unit, dt_unit in [('case:kelvin-interval', '1/K', 'K'), ('case:celsius-interval', '1/degC', 'degC')]:
        m.case(load_case(lc, [element('pipe:interval', sel_base('material:interval'), {
            'kind': 'constant_alpha_interval', 'coefficient': qv(alpha, a_unit),
            'temperature_change': qv(change, dt_unit), 'coefficient_meaning': meaning,
            'provenance': f'{gen(c, "inputs")}; the degC case authors the same interval in degC (1 degC interval = 1 K)'})],
            [support_state('support:root')]))
    k = add(Case(c, c, 'reference_temperatures', m.request(),
                 'two load cases in one request: the interval authored in K and in degC'))
    for lc in ('case:kelvin-interval', 'case:celsius-interval'):
        k.member(lc, 'pipe:interval', 'thermal_strain', an(gen(c, 'expected/thermal_strain'), '1'), 'alpha*deltaT')
        k.member(lc, 'pipe:interval', 'thermal_stretch', an(gen(c, 'expected/thermal_stretch'), '1'), '1+alpha*deltaT')
        k.gap(f'{lc}.invented_ambient_temperature', 'structural_expectation_not_scored',
              'Discriminator invented_ambient_temperature: members[pipe:interval].installation_temperature_k, '
              'operating_temperature_k, material_selection_temperature_k and coefficient_datum_k must be JSON '
              'null (no absolute temperature is required or derivable). Not expressible as a scalar assertion.',
              pointer=f'/cases/{c}/wrong_result_discriminators/invented_ambient_temperature', load_case=lc)
    k.limits.append('Strain-only reference: no mechanics assertion is made. The 1 m cantilever, E and nu are '
                    'invented carriers only.')


def author_multi_segment():
    c = 'multi_segment_free_length'
    coef = var(c, 'linear_coefficient_table', 'inputs')
    dil = var(c, 'linear_dilation_table', 'inputs')
    cpoints = [{'temperature': refq(f'{coef}/linear_coefficient_points/{i}/temperature', 'K'),
                'coefficient': refq(f'{coef}/linear_coefficient_points/{i}/coefficient', '1/K')}
               for i in range(len(node_at(f'{coef}/linear_coefficient_points')))]
    dpoints = [{'temperature': refq(f'{dil}/linear_dilation_points/{i}/temperature', 'K'),
                'dilation': refq(f'{dil}/linear_dilation_points/{i}/dilation', '1')}
               for i in range(len(node_at(f'{dil}/linear_dilation_points')))]
    definitions = node_at(f'{coef}/definitions')
    laws = [{'id': 'law:datum-length', 'definition': definitions[0],
             'datum_temperature': refq(f'{coef}/datum_temperature', 'K'),
             'data': {'kind': 'table', 'interpolation': text(f'{coef}/interpolation'), 'points': copy.deepcopy(cpoints)},
             'provenance': f'{coef} (reference-invented table)'},
            {'id': 'law:current-length', 'definition': definitions[1],
             'datum_temperature': refq(f'{coef}/datum_temperature', 'K'),
             'data': {'kind': 'table', 'interpolation': text(f'{coef}/interpolation'), 'points': copy.deepcopy(cpoints)},
             'provenance': f'{coef} (reference-invented table)'},
            {'id': 'law:dilation', 'definition': text(f'{dil}/definition'),
             'datum_temperature': refq(f'{dil}/datum_temperature', 'K'),
             'data': {'kind': 'table', 'interpolation': text(f'{dil}/interpolation'), 'points': dpoints},
             'provenance': f'{dil} (reference-invented table)'}]
    assert definitions == ['differential_per_datum_length', 'logarithmic_per_current_length']
    temps = {'install': 'installation_temperature', 'operating': 'operating_temperature', 'split': 'split_temperature'}
    for name, key in temps.items():
        assert q(f'{coef}/{key}', 'K') == q(f'{dil}/{key}', 'K')
    t = {name: refq(f'{coef}/{key}', 'K') for name, key in temps.items()}
    m = Model('project:lr-multi-segment-free-length', 'Multi-segment free length')
    m.material(material('material:multi', qv(200.0e9, 'Pa'), qv(INVENTED_NU, '1'),
                        'E/nu invented: this reference defines free-length strains only, no mechanics', laws=laws))
    pipes = ['install', 'operating', 'split']
    for index, name in enumerate(pipes):
        layout = f'{INVENTED}; separate 1 m cantilever per installation temperature (layout invented)'
        m.node(f'node:{name}-root', 0.0, float(index), layout).node(f'node:{name}-tip', 1.0, float(index), layout)
        m.pipe(f'pipe:{name}', f'node:{name}-root', f'node:{name}-tip', 'material:multi', basis_temperature(t[name]),
               fit_none(), f'installation temperature = reference {temps[name]}')
        m.support(f'support:{name}', f'node:{name}-root')
    supports = [support_state(f'support:{p}') for p in pipes]
    k = add(Case(c, c, 'reference_temperatures', None,
                 'six load cases in one request: for each of the three definitions, (a) forward, reverse and '
                 'split-to-operating and (b) install-to-split, operating-to-split and split-to-install; each '
                 'installation temperature is its own member'))
    # (pipe installation, state temperature, expected suffix)
    groups = {'a': [('install', 'operating', 'forward'), ('operating', 'install', 'reverse'),
                    ('split', 'operating', 'split_to_operating')],
              'b': [('install', 'split', 'install_to_split'), ('operating', 'split', 'operating_to_split'),
                    ('split', 'install', 'split_to_install')]}
    stretch_source = {  # published datum stretches whose reference values exist
        'datum-length': {'install': ('linear_coefficient_table', 'integral_datum_to_install'),
                         'operating': ('linear_coefficient_table', 'integral_datum_to_operating'),
                         'split': ('linear_coefficient_table', 'integral_datum_to_split')},
        'dilation': {'install': ('linear_dilation_table', 'dilation_install'),
                     'operating': ('linear_dilation_table', 'dilation_operating'),
                     'split': ('linear_dilation_table', 'dilation_split')}}
    d = f'/cases/{c}/wrong_result_discriminators'
    for law, prefix, variant in [('datum-length', 'datum_length_strain', 'linear_coefficient_table'),
                                 ('current-length', 'current_length_strain', 'linear_coefficient_table'),
                                 ('dilation', 'dilation_strain', 'linear_dilation_table')]:
        for group, rows in groups.items():
            lc = f'case:{law}-{group}'
            m.case(load_case(lc, [element(f'pipe:{p}', sel_base('material:multi'), th_free(f'law:{law}'), t[s])
                                  for p, s, _ in rows], copy.deepcopy(supports)))
            for pipe, state, suffix in rows:
                k.member(lc, f'pipe:{pipe}', 'thermal_strain', an(var(c, variant, f'expected/{prefix}_{suffix}'), '1'),
                         f'{law} strain {suffix}, spanning the consumed table segments')
                if law in stretch_source:
                    v1, key1 = stretch_source[law][pipe]
                    v2, key2 = stretch_source[law][state]
                    k.member(lc, f'pipe:{pipe}', 'installation_datum_stretch',
                             an(var(c, v1, f'expected/{key1}'), '1', 'one_plus'),
                             'lambda(T_install) = 1 + I(T_m,T_install) or 1 + d(T_install)')
                    k.member(lc, f'pipe:{pipe}', 'operating_datum_stretch',
                             an(var(c, v2, f'expected/{key2}'), '1', 'one_plus'),
                             'lambda(T) = 1 + I(T_m,T) or 1 + d(T)')
        target = f'case:{law}-a.evidence.member.pipe:install.thermal_strain'
        k.negative(target, 'first_segment_only', an(f'{d}/first_segment_only/{prefix}_forward', '1'))
        k.negative(target, 'last_segment_only.interval_difference_form',
                   an(f'{d}/last_segment_only/interval_difference_form/{prefix}_forward', '1'))
        if law == 'dilation':
            # Same wrong value as first_segment_only on the same selector (both 1/2001):
            # one negative per (selector, wrong value); the second name is recorded as a gap.
            k.gap('dilation.last_segment_only.datum_integral_form', 'numerically_identical_discriminator',
                  'For engineering_dilation this discriminator equals first_segment_only on the same forward strain '
                  '(the reference notes both give the same value); it is scored by the first_segment_only negative.',
                  pointer=f'{d}/last_segment_only/datum_integral_form/{prefix}_forward',
                  covered_by=f'{target}.not.first_segment_only')
        else:
            k.negative(target, 'last_segment_only.datum_integral_form',
                       an(f'{d}/last_segment_only/datum_integral_form/{prefix}_forward', '1'))
        if law != 'dilation':
            k.negative(target, 'interior_breakpoints_skipped', an(f'{d}/interior_breakpoints_skipped/{prefix}_forward', '1'))
            k.negative(target, 'endpoint_alpha_times_interval', an(f'{d}/endpoint_alpha_times_interval/strain', '1'))
    k.request = m.request()
    k.limits.append('Free-length strains only; the reference allocates no mechanics companion values, so no '
                    'reaction or member-force assertion is made.')
    k.limits.append('Logarithmic datum stretches exp(I) are not asserted: the reference gives no value for them.')
    k.gap('segment_integrals_install_to_operating', 'structural_expectation_not_scored',
          'The reference lists the three consumed [T_install,T] segment integrals. The published '
          'consumed_law_segments entries (CP3_WIRE_ADDENDUM section 2) are structural records, not scored here; the '
          'forward strains and the segment-truncation negatives pin the same content numerically.',
          pointer=var(c, 'linear_coefficient_table', 'expected/segment_integrals_install_to_operating'))


def author_unit_identity():
    c = 'temperature_unit_identity'
    base = var(c, 'exact_affine_identity', '')
    g = f'{base}inputs/identity_groups'
    e = f'{base}expected/identity_groups'
    ne = f'{base}inputs/non_equal_control'

    def authored(pointer):
        node = node_at(pointer)
        return qv(node['value'], node['unit'])

    points = [
        ('point:minus-50-degC-group', authored(f'{g}/0/authored/1')),
        ('point:242-degC-group', authored(f'{g}/1/authored/2')),
        ('point:20-degC-rankine-group', authored(f'{g}/2/authored/1')),
    ]
    m = (Model('project:lr-temperature-unit-identity', 'Temperature unit identity')
         .node('node:root', 0.0, 0.0, f'{INVENTED}; 1 m cantilever (length invented)')
         .node('node:tip', 1.0, 0.0, f'{INVENTED}; 1 m cantilever (length invented)')
         .material(material('material:identity', qv(200.0e9, 'Pa'), qv(INVENTED_NU, '1'),
                            'E/nu invented: this reference defines temperature identities only', points=[
                                point(pid, temperature, qv(200.0e9, 'Pa'), qv(INVENTED_NU, '1'),
                                      'point temperature authored from the reference identity group; E/nu invented')
                                for pid, temperature in points]))
         .pipe('pipe:identity', 'node:root', 'node:tip', 'material:identity', basis_direct(), fit_none())
         .support('support:root', 'node:root'))
    cases = [
        ('case:minus-50-degC', f'{g}/0/authored/0', 'point:minus-50-degC-group', f'{e}/0/kelvin'),
        ('case:242-degC', f'{g}/1/authored/0', 'point:242-degC-group', f'{e}/1/kelvin'),
        ('case:467p6-degF', f'{g}/1/authored/1', 'point:242-degC-group', f'{e}/1/kelvin'),
        ('case:20-degC-rankine', f'{g}/2/authored/0', 'point:20-degC-rankine-group', f'{e}/2/kelvin'),
    ]
    for lc, temperature, point_id, _ in cases:
        m.case(load_case(lc, [element('pipe:identity', sel_point('material:identity', point_id), th_unchanged(),
                                      authored(temperature))], [support_state('support:root')]))
    m.case(load_case('case:non-equal-control', [element('pipe:identity', sel_base('material:identity'),
                                                        th_unchanged(), authored(f'{ne}/authored/0'))],
                     [support_state('support:root')]))
    k = add(Case(c, c, 'reference_temperatures', m.request(),
                 'five load cases in one request: four exact-point selections whose operating temperature is '
                 'authored in a different unit from the point, and the non-equal control on base properties'))
    for lc, _temperature, point_id, kelvin in cases:
        k.member(lc, 'pipe:identity', 'operating_temperature_k', an(kelvin, 'K'), 'exact affine unit definition')
        k.member(lc, 'pipe:identity', 'material_selection_temperature_k', an(kelvin, 'K'),
                 'the exact point selected with no actual-versus-selected override')
        k.gap(f'{lc}.selected_point', 'structural_expectation_not_scored',
              f'Identity witness: the case solves with exact_point "{point_id}" and no override. A failed identity '
              'blocks with LOAD_STATE_MATERIAL_TEMPERATURE_OVERRIDE_REQUIRED and leaves every assertion of the case '
              'unavailable. material_selection_kind == "exact_point" is a definition constraint of the member '
              f'evidence selectors; consumed_material_points[0].point_id == "{point_id}" is not scored.', load_case=lc, point_id=point_id)
    ctrl = 'case:non-equal-control'
    k.member(ctrl, 'pipe:identity', 'operating_temperature_k', an(f'{base}expected/non_equal_control/authored_kelvin/0', 'K'),
             'the non-equal authored temperature is kept, not snapped to 223.15 K')
    k.negative(f'{ctrl}.evidence.member.pipe:identity.operating_temperature_k', 'non_equal_control.snapped_to_223p15_K',
               an(f'{base}expected/non_equal_control/authored_kelvin/1', 'K'))
    k.gap('non_equal_control.refusal', 'refusal_control',
          'Refusal control (not a scalar assertion): replace case:minus-50-degC operating_temperature by the '
          'non-equal authored value; exact_point selection must then block with '
          'LOAD_STATE_MATERIAL_TEMPERATURE_OVERRIDE_REQUIRED.',
          mutation={'op': 'replace', 'path': '/model/load_cases/0/analysis_state/element_states/0/operating_temperature',
                    'value_pointer': f'{ne}/authored/0'},
          expected={'channel': 'blocking_diagnostic', 'code': 'LOAD_STATE_MATERIAL_TEMPERATURE_OVERRIDE_REQUIRED'})
    k.gap('binary64_affine_conversion', 'not_representable_under_criterion',
          'The binary64-conversion discriminator differs from the identity by about 1e-16 relative, far below the '
          'protected 1e-9 criterion, so no scalar negative can separate it. The identity is witnessed by the '
          'exact-point selections succeeding without override (see selected_point gaps).',
          pointer=f'/cases/{c}/wrong_result_discriminators/binary64_affine_conversion')
    k.limits.append('Temperature identities only; no mechanics assertion is made.')


FIT_STATES = [('case:cold', 'cold'), ('case:hot', 'hot'), ('case:return', 'return')]


def fit_request(c, fixed):
    L = ann(c, 'inputs/L')
    install = refq(ann(c, 'inputs/installation_temperature'), 'degC')
    hot = refq(ann(c, 'inputs/hot_temperature'), 'degC')
    cold_e, hot_e = refq(ann(c, 'inputs/cold_E'), 'Pa'), refq(ann(c, 'inputs/hot_E'), 'Pa')
    law = {'id': 'law:constant', 'definition': 'engineering_secant',
           'datum_temperature': refq(ann(c, 'inputs/coefficient_datum_temperature'), 'degC'),
           'data': {'kind': 'constant', 'coefficient': refq(ann(c, 'inputs/alpha'), '1/K')},
           'provenance': f'constant engineering_secant alpha {ann(c, "inputs/alpha")}'}
    m = (Model(f'project:lr-signed-fit-{"fixed" if fixed else "released"}',
               f'Signed fit states, {"fixed" if fixed else "released"} bar')
         .node('node:root', 0.0).node('node:far', q(L, 'm'))
         .material(material('material:fit', cold_e, qv(INVENTED_NU, '1'),
                            'base E = reference cold_E (selected by no case); nu invented (enters no checked axial result)',
                            points=[point('point:cold', install, cold_e, qv(INVENTED_NU, '1'),
                                          'cold E at the reference installation temperature; nu invented'),
                                    point('point:hot', hot, hot_e, qv(INVENTED_NU, '1'),
                                          'hot E at the reference hot temperature; nu invented')],
                            laws=[law]))
         .pipe('pipe:fit', 'node:root', 'node:far', 'material:fit', basis_temperature(install),
               {'kind': 'natural_length_change', 'length_change': refq(ann(c, 'inputs/signed_fit_length_change'), 'm')},
               'signed natural length change (cut short) from the reference inputs')
         .support('support:root', 'node:root'))
    if fixed:
        m.support('support:far', 'node:far')
    supports = [support_state('support:root')] + ([support_state('support:far')] if fixed else [])
    for lc, state in FIT_STATES:
        operating, point_id = (hot, 'point:hot') if state == 'hot' else (install, 'point:cold')
        m.case(load_case(lc, [element('pipe:fit', sel_point('material:fit', point_id), th_free('law:constant'),
                                      operating)], copy.deepcopy(supports)))
    return m.request()


def author_fit(fixed):
    c = 'signed_fit_states'
    label = 'fixed' if fixed else 'released'
    k = add(Case(f'{c}.{label}', c, 'cold_spring', fit_request(c, fixed),
                 'three load cases in one request (cold, hot, return), each an independent equilibrium; the fixed '
                 'and released topologies are separate requests'))
    L = ann(c, 'inputs/L')
    d = f'/cases/{c}/wrong_result_discriminators'
    strain_zero = scale('hot_thermal_strain', ann(c, 'expected/hot/thermal_strain'), '1')
    ratio = {'numerator': {'pointer': '/geometry/derived/As', 'reference_unit': 'm^2'},
             'denominator': {'pointer': gen(c, 'inputs/area'), 'reference_unit': 'm^2'}}
    for lc, state in FIT_STATES:
        p = lambda rest: ann(c, f'expected/{state}/{rest}')  # noqa: E731
        force_zero, moment_zero = reaction_scales(p('fixed_wall_N'), 'N', [(L, 'm')], state)
        if fixed:
            k.axial(lc, 'pipe:fit', p('fixed_wall_N'), force_zero, 'restrained wall force of the composed eigenstrain')
            reason = 'straight bar on global X with axial eigenstrain only: no transverse force or couple'
            k.reactions(lc, 'support:root', {'Fx': an(p('fixed_root_Fx'), 'N')}, force_zero, moment_zero, reason)
            k.reactions(lc, 'support:far', {'Fx': an(p('fixed_far_Fx'), 'N')}, force_zero, moment_zero, reason)
        else:
            k.disp(lc, 'node:far', 'x', an(p('released_tip_UX'), 'm'), 'released tip moves L*eps_star')
            k.axial(lc, 'pipe:fit', p('released_wall_N'), force_zero, 'released bar: zero wall force')
            k.reactions(lc, 'support:root', {}, force_zero, moment_zero,
                        'released cantilever with self-equilibrated eigenstrain: no support action')
        k.member(lc, 'pipe:fit', 'selected_E_pa', an(p('E'), 'Pa'), 'state E by exact point selection')
        k.member(lc, 'pipe:fit', 'thermal_strain', an(p('thermal_strain'), '1', zero=strain_zero), 'state thermal strain')
        k.member(lc, 'pipe:fit', 'fit_strain', an(p('fit_strain'), '1'), 'signed fit strain dL/L, applied once')
        k.member(lc, 'pipe:fit', 'total_eigenstrain', an(p('total_eigenstrain'), '1'), 'lambda_fit*lambda_th-1')
        k.member(lc, 'pipe:fit', 'reference_length_m', an(ann(c, 'inputs/L'), 'm'), 'authored chord length')
    if fixed:
        k.negative('case:hot.axial.pipe:fit.midspan', 'hot_additive_strains',
                   an(f'{d}/hot_additive_strains/wrong_generic_N', 'N', 'generic_area_to_annulus', area_ratio=ratio))
        k.negative('case:hot.axial.pipe:fit.midspan', 'hot_uses_cold_E',
                   an(f'{d}/hot_uses_cold_E/wrong_generic_N', 'N', 'generic_area_to_annulus', area_ratio=ratio))
        k.negative('case:cold.axial.pipe:fit.midspan', 'fit_applied_twice',
                   an(f'{d}/fit_applied_twice/wrong_cold_generic_N', 'N', 'generic_area_to_annulus', area_ratio=ratio))
        k.negative('case:cold.axial.pipe:fit.midspan', 'sign_flipped',
                   an(f'{d}/sign_flipped/wrong_cold_generic_N', 'N', 'generic_area_to_annulus', area_ratio=ratio))
        k.gap('misfit_plus_anchor_motion', 'implied_by_positive',
              'Text discriminator (fit eigenstrain plus an equivalent support motion double-counts one mismatch). '
              'No boundary_motion is authored, and the fixed wall-force and fit-strain assertions exclude a second '
              'count.', pointer=f'{d}/misfit_plus_anchor_motion',
              assertions=['case:cold.axial.pipe:fit.midspan', 'case:cold.evidence.member.pipe:fit.fit_strain'])
    k.gap('baselines', 'not_authored',
          'The reference baselines (no fit cold/hot, cut long cold) are other member-reference inputs and would '
          'need further requests; they are not authored in this package.', pointer=ann(c, 'baselines'))


def author_source():
    c = 'persistent_source_once'
    L = ann(c, 'inputs/L')
    ledger = f'/cases/{c}/source_ledger'
    stored = next(i for i, e in enumerate(node_at(ledger)) if e['source_id'] == 'source:stored-unused')
    note = ('reference input; the +100 N preload is represented as an ordinary stored primitive, as the reference '
            'annular topology declares it an equivalent affine tip action (no device law)')
    combined = 'case:combined'
    pair = 'case:two-equal-actions'
    primitives = [
        nodal_force('source:preload', 'node:tip', refq(ann(c, 'inputs/preload'), 'N'), note),
        nodal_force('source:weight', 'node:tip', refq(ann(c, 'inputs/weight'), 'N'), 'reference input'),
        nodal_force('source:independent', 'node:tip', refq(ann(c, 'inputs/independent_action'), 'N'), 'reference input'),
        nodal_force('source:stored-unused', 'node:tip', qv(node_at(f'{ledger}/{stored}/force_N'), 'N'),
                    'reference source_ledger entry; stored but not referenced'),
    ]
    pair_primitives = [
        nodal_force(pid, 'node:tip', refq(ann(c, 'inputs/independent_action'), 'N'),
                    'reference numeric_deduplication discriminator: two distinct sources of the independent action value')
        for pid in ('source:action-a', 'source:action-b')]
    m = (Model('project:lr-persistent-source-once', 'Persistent source counted once')
         .node('node:root', 0.0).node('node:tip', q(L, 'm'))
         .material(material('material:source', refq(ann(c, 'inputs/E'), 'Pa'), qv(INVENTED_NU, '1'),
                            f'E from {ann(c, "inputs/E")}; nu invented (enters no checked axial result)'))
         .pipe('pipe:cantilever', 'node:root', 'node:tip', 'material:source', basis_direct(), fit_none())
         .support('support:root', 'node:root'))
    m.case(load_case(combined, [element('pipe:cantilever', sel_base('material:source'), th_unchanged())],
                     [support_state('support:root')], primitives,
                     [('source:preload', 1.0), ('source:weight', 1.0), ('source:independent', 1.0)]))
    m.case(load_case(pair, [element('pipe:cantilever', sel_base('material:source'), th_unchanged())],
                     [support_state('support:root')], pair_primitives,
                     [('source:action-a', 1.0), ('source:action-b', 1.0)]))
    k = add(Case(c, c, 'cold_spring', m.request(),
                 'two load cases in one request: the combined preload/weight/independent case and the two distinct '
                 'equal actions of the numeric_deduplication discriminator'))
    force_zero, moment_zero = reaction_scales(ann(c, 'expected/root_Fx'), 'N', [(L, 'm')])
    reason = 'axial cantilever with global-X tip actions only: no transverse force or couple'
    k.disp(combined, 'node:tip', 'x', an(ann(c, 'expected/combined_displacement'), 'm'), 'u = F_combined/k, preload once')
    k.reactions(combined, 'support:root', {'Fx': an(ann(c, 'expected/root_Fx'), 'N')}, force_zero, moment_zero, reason)
    k.axial(combined, 'pipe:cantilever', ann(c, 'expected/combined_rhs'), force_zero,
            'axial cantilever: tension-positive N equals the combined tip action F_combined')
    for source, name in [('source:preload', 'preload'), ('source:weight', 'weight'),
                         ('source:independent', 'independent_action')]:
        k.contribution(combined, source, an(ann(c, f'inputs/{name}'), 'N'), 'included once with factor 1')
    pair_force, pair_moment = reaction_scales(ann(c, 'expected/two_distinct_equal_actions_rhs'), 'N', [(L, 'm')], 'pair')
    k.reactions(pair, 'support:root', {'Fx': an(ann(c, 'expected/two_distinct_equal_actions_rhs'), 'N', 'negate')},
                pair_force, pair_moment, reason)
    k.axial(pair, 'pipe:cantilever', ann(c, 'expected/two_distinct_equal_actions_rhs'), pair_force,
            'two distinct equal sources both apply: N equals their sum')
    for source in ('source:action-a', 'source:action-b'):
        k.contribution(pair, source, an(ann(c, 'inputs/independent_action'), 'N'), 'distinct source, not deduplicated')
    k.negative(f'{combined}.disp.node:tip.ux', 'naive_total_sum_displacement',
               an(ann(c, 'expected/naive_total_sum_displacement'), 'm'))
    k.negative(f'{combined}.reaction.support:root.Fx', 'naive_total_sum_rhs',
               an(ann(c, 'expected/naive_total_sum_rhs'), 'N', 'negate'))
    d = f'/cases/{c}/wrong_result_discriminators'
    k.gap('implicit_case_array', 'structural_expectation_not_scored',
          'Discriminator implicit_case_array: no contribution entry with source_id "source:stored-unused" may exist, '
          'and excluded_sources must list it with classification "excluded". Not expressible as a scalar '
          'assertion; the tip displacement and root reaction assertions exclude the +999 N numerically.',
          pointer=f'{d}/implicit_case_array', load_case=combined)
    k.gap('duplicate_source_id', 'refusal_control',
          'Refusal control (not a scalar assertion): append a second load_sources entry for source:weight '
          '(factor 2); the request must block with LOAD_STATE_SOURCE_DUPLICATE.',
          pointer=f'{d}/duplicate_source_id',
          mutation={'op': 'add', 'path': '/model/load_cases/0/analysis_state/load_sources/-',
                    'value': {'source_ref': 'source:weight', 'factor': 2.0}},
          expected={'channel': 'blocking_diagnostic', 'code': 'LOAD_STATE_SOURCE_DUPLICATE'})
    k.gap('preload_classification', 'representability_limit',
          'The reference ledger classifies the preload as a selected device affine_reference. device_reference '
          'parses and blocks in this capability, so the preload is an ordinary stored primitive here and its '
          'contribution classification is ordinary_applied. No assertion is made on the classification.')
    k.gap('intermediate_rhs', 'not_published',
          'weight_total_rhs, additional_total_rhs and excluded_unreferenced_action_rhs are intermediate sums with '
          'no published row; combined_rhs is asserted through the member force.',
          pointers=[ann(c, 'expected/weight_total_rhs'), ann(c, 'expected/additional_total_rhs'),
                    ann(c, 'expected/excluded_unreferenced_action_rhs')])


def author_all():
    author_two_bar()
    author_all_fixed()
    author_rotation_all_fixed()
    author_rotation_free_tip()
    author_serial()
    author_thermal(True)
    author_thermal(False)
    author_coefficient_definition()
    author_constant_alpha()
    author_multi_segment()
    author_unit_identity()
    author_fit(True)
    author_fit(False)
    author_source()


# ---------------------------------------------------------------------------
# Output
# ---------------------------------------------------------------------------

def dump(value) -> bytes:
    return (json.dumps(value, indent=2, ensure_ascii=False, allow_nan=False) + '\n').encode('utf-8')


def runner_input(case: Case) -> dict:
    lcs = [lc['id'] for lc in case.request['model']['load_cases']]
    slug = case.case_id.replace('_', '-').replace('.', '-')
    return {
        'request': {
            'input_manifest_ref': {'ref_type': 'audit_manifest', 'ref_id': f'manifest:lr-{slug}-v1'},
            'load_basis_refs': [{'ref_type': 'load_case', 'ref_id': lc} for lc in lcs],
            'model_ref': {'ref_type': 'model', 'ref_id': case.request['model']['project']['id']},
            'operation': 'solve',
            'operation_ref': {'ref_id': 'solve', 'ref_type': 'api_operation'},
            'privacy': {'classification': 'public_metadata', 'local_only': True, 'private_payload_redacted': True,
                        'telemetry_allowed': False},
            'professional_boundary': {'human_review_required': True, 'software_makes_approval_claim': False,
                                      'software_makes_authentication_claim': False,
                                      'software_makes_certification_claim': False,
                                      'software_makes_compliance_claim': False,
                                      'software_makes_sealing_claim': False},
            'project_ref': {'ref_type': 'project', 'ref_id': case.request['model']['project']['id']},
            'provenance': {
                'source_name': f'VP-STATIC load/reference-state case {case.case_id}',
                'source_location': 'LOAD_STATE_IMPLEMENTATION/T1_WP6_STATIC_CASES',
                'source_license': 'project-original-public-content',
                'contributor': 'T1_WP6_STATIC_CASES TASK (Type 2)',
                'contributor_certification': ('Explicitly invented mathematical test inputs built from '
                                              'reference_cases.json; no external project, material library or code data'),
                'redistribution_status': 'invented_non_engineering_example',
                'review_status': 'prepared_for_independent_freeze_check'},
            'request_id': f'lr-{slug}-v1',
            'requested_outputs': ['result_envelope', 'audit_manifest', 'diagnostics'],
            'tbd_decisions': {'ci_provider': 'TBD', 'external_adapter_formats': 'TBD',
                              'filesystem_mutation_policy': 'SETTLED_DEC_065',
                              'final_cli_command_syntax': 'SETTLED_DEC_065', 'network_access': 'SETTLED_DEC_065',
                              'package_scripts': 'SETTLED_DEC_065', 'physical_project_container': 'TBD',
                              'process_invocation': 'SETTLED_DEC_065', 'public_transport_protocol': 'TBD',
                              'release_matrix': 'TBD'},
            'unit_system_ref': {'ref_id': 'invented-si', 'ref_type': 'unit_system'}},
        'solve': {'preview_model': copy.deepcopy(case.request)}}


def phase_requests():
    for case in CASES:
        (PACKAGE / f'{case.case_id}.preview_request.json').write_bytes(dump(case.request))
        (PACKAGE / f'{case.case_id}.runner_input.json').write_bytes(dump(runner_input(case)))
        print('request', case.case_id)


def resolve_row(inventory: list[dict], lookup: dict, context: str) -> dict:
    matches = [r for r in inventory
               if r['kind'] == lookup['kind'] and r['entity_ref'] == lookup['entity']
               and r['basis_ref'] == {'ref_type': 'load_case', 'ref_id': lookup['lc']}
               and isinstance(r.get('metadata'), dict) and r['metadata'].get('component') == lookup['component']
               and r['metadata'].get('location') == lookup['location']]
    if len(matches) != 1:
        raise SystemExit(f'{context}: {len(matches)} rows match {lookup}')
    return matches[0]


def evidence_definition(request: dict, selector: dict) -> dict:
    """Record text constraints derived from the authored request (not from producer output)."""
    model = request['model']
    lc = next(c for c in model['load_cases'] if c['id'] == selector['basis_ref']['ref_id'])
    state = lc['analysis_state']
    key = selector['key']
    if selector['record'] == 'member':
        element = next(e for e in state['element_states'] if e['pipe_ref'] == key['pipe_id'])
        member = next(m for m in model['reference_configurations'][0]['member_references'] if m['pipe_ref'] == key['pipe_id'])
        pipe = next(p for p in model['pipe_segments'] if p['id'] == key['pipe_id'])
        thermal = element['thermal_state']
        definition = {'material_id': pipe['material'], 'material_selection_kind': element['material_selection']['kind'],
                      'reference_basis': member['basis']['kind'], 'fit_kind': member['fit']['kind']}
        if thermal['kind'] == 'free_length_state':
            material = next(m for m in model['materials'] if m['id'] == pipe['material'])
            law = next(l for l in material['expansion_laws'] if l['id'] == thermal['expansion_law_ref'])
            definition['thermal_definition'] = law['definition']
            definition['expansion_law_id'] = law['id']
        else:
            definition['thermal_definition'] = thermal['kind']
        return definition
    if selector['record'] == 'support_component':
        support = next(x for x in model['supports'] if x['id'] == key['support_id'])
        motion = next(x for x in state['support_states'] if x['support_ref'] == key['support_id'])
        dof = next(b for b in motion['boundary_motion'] if b['dof'] == key['dof'])
        return {'node_id': support['node'], 'law_kind': 'rigid_prescribed', 'meaning': dof['meaning']}
    primitive = next(p for p in lc['primitive_loads'] if p['id'] == key['source_id'])
    return {'classification': 'ordinary_applied', 'category': primitive['category'], 'dimension': primitive['dimension']}


def phase_selectors(inventory_dir: Path):
    for case in CASES:
        inventory = json.loads((inventory_dir / f'{case.case_id}.inventory.json').read_bytes())
        assertions = []
        by_id = {}
        for a in case.assertions:
            if 'lookup' in a:
                row = resolve_row(inventory, a['lookup'], a['id'])
                assert row['unit'] == a['unit'], (a['id'], row['unit'])
                selector = {'id': row['id'], 'kind': row['kind'], 'unit': row['unit'], 'entity_ref': row['entity_ref'],
                            'basis_ref': row['basis_ref'], 'metadata': row['metadata'], 'dimension': a['dimension']}
                origin = {'semantic_signature_id': a['signature'], 'family': a['family']}
            else:
                selector = copy.deepcopy(a['evidence'])
                selector['definition'] = evidence_definition(case.request, selector)
                origin = {'family': a['family']}
            origin['reference_origin'] = a['origin']
            origin['why_required'] = a['why']
            unit, dimension = selector['unit'], selector['dimension']
            expected = GEN.to_binary64(GEN.origin_value(REF, a['origin'], unit, a['id']))
            rule = (f'criterion:{a["family"]}:{dimension}:{unit}:' +
                    ('relative_1e-9' if expected != 0.0 else 'zero_scale:' + a['origin']['zero_scale']['tag']))
            record = {'id': a['id'], 'selector': selector, 'criterion_rule_id': rule, 'selector_origin': origin}
            assertions.append(record)
            by_id[a['id']] = record
        negatives = []
        for n in case.negatives:
            target = by_id[n['target']]
            unit, dimension = target['selector']['unit'], target['selector']['dimension']
            family = target['selector_origin']['family']
            wrong = GEN.to_binary64(GEN.origin_value(REF, n['origin'], unit, n['target']))
            rule = (f'criterion:{family}:{dimension}:{unit}:' +
                    ('relative_1e-9' if wrong != 0.0 else 'zero_scale:' + n['origin']['zero_scale']['tag']))
            origin = {'family': family, 'negates_assertion': n['target'], 'discriminator': n['name'],
                      'reference_origin': n['origin'],
                      'predicate': ('observed must fall OUTSIDE this rule around the wrong value: '
                                    '|observed - wrong| > max(absolute, relative*max(|observed|,|wrong|))')}
            if 'semantic_signature_id' in target['selector_origin']:
                origin['semantic_signature_id'] = target['selector_origin']['semantic_signature_id']
            negatives.append({'id': f'{n["target"]}.not.{n["name"]}', 'selector': copy.deepcopy(target['selector']),
                              'criterion_rule_id': rule, 'selector_origin': origin})
        document = {
            'format': GEN.SELECTOR_FORMAT,
            'case_id': case.case_id,
            'producer_contract': CONTRACT,
            'raw_schema_version': '0.2.0',
            'row_namespace': 'payload.mechanics_envelope.results',
            'assertions': assertions,
            'negative_assertions': negatives,
            'gaps': case.gaps,
            'scoring_readiness': {
                'status': 'authoring_only_pending_independent_freeze',
                'reference_case_key': case.key,
                'family': case.family_label,
                'load_state_decomposition': case.decomposition,
                'limits': case.limits,
                'identification': ('row ids and metadata were identified by running core/product_physics '
                                   'examples/physics_source_connected on this request in both modes; no observed '
                                   'value is used as a reference or criterion and none is committed'),
            },
        }
        (PACKAGE / f'{case.case_id}.selectors.candidate.json').write_bytes(dump(document))
        print('selectors', case.case_id, len(assertions), len(negatives))


def rel(path: Path) -> str:
    return path.resolve().relative_to(WORKING_ROOT).as_posix()


def binding(path: Path) -> dict:
    return {'path': rel(path), 'sha256': sha(path.read_bytes())}


def phase_manifest():
    cases = []
    for case in CASES:
        selectors_path = PACKAGE / f'{case.case_id}.selectors.candidate.json'
        selectors = json.loads(selectors_path.read_bytes())
        runner = json.loads((PACKAGE / f'{case.case_id}.runner_input.json').read_bytes())
        request = json.loads((PACKAGE / f'{case.case_id}.preview_request.json').read_bytes())
        assert runner['solve']['preview_model'] == request
        assert case.key in REF['cases']
        cases.append({
            'case_id': case.case_id,
            'runner_input': binding(PACKAGE / f'{case.case_id}.runner_input.json'),
            'product_request': binding(PACKAGE / f'{case.case_id}.preview_request.json'),
            'reference': binding(PACKAGE / f'{case.case_id}.reference.candidate.json'),
            'selectors': binding(selectors_path),
            'criteria': binding(PACKAGE / f'{case.case_id}.criteria.candidate.json'),
            'analytical_reference': {'path': REFERENCE_REL, 'sha256': sha(REF_BYTES), 'case_key': case.key},
            'modes': ['sparse_interactive', 'dense_scrutiny'],
            'required_scalar_rows': len(selectors['assertions']),
        })
    manifest = {'format': 'openpipestress.load_reference_qualification_manifest/1', 'cases': cases}
    (PACKAGE / 'MANIFEST.json').write_bytes(dump(manifest))
    print('manifest', len(cases))


if __name__ == '__main__':
    author_all()
    phase = sys.argv[1]
    if phase == 'requests':
        phase_requests()
    elif phase == 'selectors':
        phase_selectors(Path(sys.argv[3]) if sys.argv[2] == '--inventory' else None)
    elif phase == 'manifest':
        phase_manifest()
    elif phase == 'list':
        for case in CASES:
            print(case.case_id)
    else:
        raise SystemExit('phase: requests | selectors --inventory DIR | manifest | list')
