"""Write the package PROVENANCE.json (T1_WP6_STATIC_CASES TASK). Run from anywhere."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve()
WR = HERE.parents[9]
PKG = WR / 'validation/qualification/fixtures/load_reference'
LSI = HERE.parents[2]


def rel(path: Path) -> str:
    return path.resolve().relative_to(WR).as_posix()


def bound(path: Path) -> dict:
    return {'path': rel(path), 'sha256': hashlib.sha256(path.read_bytes()).hexdigest()}


INVENTED_NU = 'Poisson ratio 0.3 invented where the reference gives none; it enters no checked result of the case'
INVENTED = {
    'prescribed_translation_two_bar': [INVENTED_NU, 'rigid six-DOF anchors at root and far (reference: all other DOFs prescribed zero)'],
    'prescribed_translation_all_fixed': [INVENTED_NU, 'rigid six-DOF anchors at root and far'],
    'prescribed_rotation_all_fixed': [INVENTED_NU, 'rigid six-DOF anchors at root and far'],
    'prescribed_rotation_free_tip': [INVENTED_NU, 'rigid six-DOF anchor at root; free far end'],
    'shared_material_serial_companion': [
        'shared material base E 170 GPa and nu 0.28 invented; selected by no member',
        'property-point temperatures 20 degC (point:user-cold) and 300 degC (point:user-hot) invented; the reference gives E/nu and point ids only',
        'rigid six-DOF anchors at root and far'],
    'thermal_datum_ratio.fixed': [INVENTED_NU, 'rigid six-DOF anchors at root and far',
                                  'operating temperature of case:annular-three-point-kelvin authored as the reference operating_temperature_K'],
    'thermal_datum_ratio.free': [INVENTED_NU, 'rigid six-DOF anchor at root; free far end',
                                 'operating temperature of case:annular-three-point-kelvin authored as the reference operating_temperature_K'],
    'coefficient_definition': ['E 200 GPa and nu 0.3 invented (strain-only reference)',
                               'three 1 m cantilevers at y = 0, 1, 2 m invented as carriers',
                               'installation temperature 70 degC of pipe:install-mid is the midpoint implied by the reference first_half/second_half quantities'],
    'constant_alpha_interval': ['E 200 GPa and nu 0.3 invented (strain-only reference)', '1 m cantilever invented as carrier',
                                'case:celsius-interval authors the same interval in 1/degC and degC'],
    'multi_segment_free_length': ['E 200 GPa and nu 0.3 invented (strain-only reference)',
                                  'three 1 m cantilevers at y = 0, 1, 2 m invented as carriers'],
    'temperature_unit_identity': ['E 200 GPa and nu 0.3 invented for the material and its three points (identity-only reference)',
                                  '1 m cantilever invented as carrier',
                                  'the three point temperatures are authored identity-group members; which member is the point is an authoring choice'],
    'signed_fit_states.fixed': [INVENTED_NU, 'material base E = reference cold_E (selected by no case)',
                                'point temperatures = reference installation (cold E) and hot (hot E) temperatures',
                                'rigid six-DOF anchors at root and far'],
    'signed_fit_states.released': [INVENTED_NU, 'material base E = reference cold_E (selected by no case)',
                                   'point temperatures = reference installation (cold E) and hot (hot E) temperatures',
                                   'rigid six-DOF anchor at root; free far end (released)'],
    'persistent_source_once': [INVENTED_NU, 'rigid six-DOF anchor at root',
                               'preload authored as an ordinary stored primitive (reference: equivalent affine tip action)',
                               'source ids source:action-a/-b and their +200 N magnitude (the reference independent_action) from the numeric_deduplication discriminator'],
}


def main():
    manifest = json.loads((PKG / 'MANIFEST.json').read_bytes())
    wire = [LSI / name for name in ('CP2_WIRE.md', 'CP2_WIRE_ADDENDUM_1.md', 'CP2_WIRE_ADDENDUM_2.md',
                                    'CP3_WIRE_ADDENDUM.md', 'CP4_WIRE_ADDENDUM.md', 'CP4_REVIEW_DISPOSITION.md',
                                    'T1_PLAN.md', 'TASK_BRIEFS/_T1_COMMON.md', 'TASK_BRIEFS/T1_WP6_STATIC_CASES.md')]
    cases = []
    for case in manifest['cases']:
        selectors = json.loads((WR / case['selectors']['path']).read_bytes())
        request = json.loads((WR / case['product_request']['path']).read_bytes())
        cases.append({
            'case_id': case['case_id'],
            'case_key': case['analytical_reference']['case_key'],
            'load_cases': [lc['id'] for lc in request['model']['load_cases']],
            'load_state_decomposition': selectors['scoring_readiness']['load_state_decomposition'],
            'positive_assertions': len(selectors['assertions']),
            'negative_assertions': len(selectors.get('negative_assertions', [])),
            'gaps': [g['id'] for g in selectors.get('gaps', [])],
            'invented_inputs': INVENTED[case['case_id']],
        })
    provenance = {
        'format': 'openpipestress.load_reference_qualification_provenance/1',
        'status': 'authoring_only_pending_independent_freeze',
        'author': 'T1_WP6_STATIC_CASES TASK (Type 2), requested by the T1 WORKING_ITEMS manager',
        'date': '2026-09-26',
        'base_commit': 'd8f0dc4f7d95d2893078d335e11d7d31911e6e3b',
        'data_boundary': 'All inputs are invented test quantities; no material, component or code-rule library data.',
        'analytical_reference': bound(WR / 'core/product_physics/tests/fixtures/load_reference_states/reference_cases.json'),
        'analytical_reference_readme': bound(WR / 'core/product_physics/tests/fixtures/load_reference_states/README.md'),
        'producer_table': bound(WR / 'fixtures/results/semantic_contract_v0_3_load_reference_1.json'),
        'runner_input_template': bound(WR / 'validation/qualification/fixtures/first_static/axial.runner_input.candidate.json'),
        'wire_and_briefs': [bound(p) for p in wire],
        'generator': bound(PKG / 'generate_reference_values.py'),
        'generator_outputs': [bound(p) for p in sorted(PKG.glob('*.reference.candidate.json')) + sorted(PKG.glob('*.criteria.candidate.json'))],
        'authoring_records': [bound(HERE.with_name(n)) for n in ('author_package.py', 'check_package.py', 'mutations.py', 'write_provenance.py')],
        'identification': {
            'statement': ('Row ids, kinds and metadata were identified by running core/product_physics '
                          'examples/physics_source_connected (sparse and dense) on each request, and confirmed through the '
                          'openpipestress-runner solve transport. No observed value is used as a reference or criterion, '
                          'no comparison was run, and no producer output is committed.'),
            'producer_standing_observed': ('every request solved in both modes with load-reference-1, profile '
                                           'resolved_straight_load_state_v1, numerical standing checks_passed for every case, '
                                           'no blocking diagnostic and the ordinary structural route (not_joined)'),
        },
        'excluded_reference_cases': [{
            'case_key': 'shared_material_parallel',
            'reason': ('Two coincident parallel bars: the reference marks this an analytical topology only whose '
                       'public/native admissibility must be established separately.'),
            'practical_companion': 'shared_material_serial_companion'}],
        'cases': cases,
    }
    (PKG / 'PROVENANCE.json').write_text(json.dumps(provenance, indent=2, ensure_ascii=False) + '\n')
    print('wrote PROVENANCE.json', hashlib.sha256((PKG / 'PROVENANCE.json').read_bytes()).hexdigest())


if __name__ == '__main__':
    main()
