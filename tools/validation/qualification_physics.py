"""Closed ordinary physics-1 adapter support; physical consistency is not origin.

The final reviewed candidate binding is supplied explicitly. No same-named
module import, build, network, unit conversion or older-profile fallback occurs.
"""
from __future__ import annotations
from copy import deepcopy
import json
from pathlib import Path
import sys

try:
    from .qualification_process import capture, file_sha256, sha256_bytes
except ImportError:
    from qualification_process import capture, file_sha256, sha256_bytes

TRANSPORT = 'ordinary_physics_1_cli_1.0_raw0.2'
CONTRACT = 'openpipestress.result_semantics/0.3.0/physics-1'
PROFILE = 'exact_straight_pressure_v2'
TABLE = 'fixtures/results/semantic_contract_v0_3_physics_1.json'
MODULE = 'core/analysis_runs/physics_evidence.py'
UNITS = 'core/units/src/lib.rs'
TABLE_SHA256 = '9a2cf6268b57bd5265a1a115497c07450819dd4d03cd5ab618097bd9d19da8cc'
DEPENDENCIES = {
    MODULE: '40013aa421adf9f24f1b9ec7b23bbfccafe054904dd5c9c8125a6bd24e653fad',
    TABLE: TABLE_SHA256,
    UNITS: '521717eb4cac8114b5c1bbeef3f8caebd57eb3021c7743860f9be8f1078b710c',
}
HELPER = Path(__file__).with_name('qualification_physics_helper.py')
LIMIT = 8 * 1024 * 1024
REFERENCE_BASIS_SHA256 = 'be6e332513634f747a1f44c6f38bb94bd2ad0d7ce8e3caa75ccfcdfc9124bdf9'
SECTION_CRITERIA_SHA256 = '1ed8aac0910318902cd5f690aeee8cb74bfee036617e33eaa4e2c23ec98ff3c2'
STRUCTURAL_OBLIGATIONS_SHA256 = 'b5f6a9446d5485d707ca86a2df554e4ce9d266246ef6f373f4de7f8d564095fa'
# These are the actual finite package's review/technical-selection records,
# not a general authorisation system or labels supplied by a result producer.
SELECTED_ADMISSION_FILES = {
    'ROOT_SELECTION.md': 'c7899527ad9c63c46e1c016f178a37331eb0ae1bf7d93f1ba20fbc1482caac11',
    'ROOT_SELECTION_BASIS.json': '2a83319cd16c95eb736f4cc846ca5a5a71dea3d39550848d9f7f55dbaa05abc5',
    'INDEPENDENT_REVIEW/RETURN.md': '565dc99ae70ba34bb4d913c0bb19b3934c92e704d75e10ab0e22931cbf73de96',
    'INDEPENDENT_REVIEW/SELECTION_BACKCHECK.md': 'ae487d4b9568be0b50c3dcfa95a24e54aa30a209907d0904bfdca3f92ec564a2',
    'INDEPENDENT_REVIEW/SELECTION_BACKCHECK.json': 'e4c69717ab52683d3cbfad6555cf05a057463a4defe2ce72933c1de10adb4ff3',
    'STRUCTURAL_NUMERIC_ADDENDUM/ROOT_REVIEW/RETURN.md': 'c8eb9b52e5235fb34e0c83c7c6b22a93ff2cf1da0ff2e1702df4f9884e97c9d2',
    'STRUCTURAL_NUMERIC_ADDENDUM/ROOT_REVIEW/CHECKS.json': 'eb09fb43421ecabfeb8ba3ddc4227bde3cedd88f75e320ca61e4685b4736acbb',
}
REQUIRED_STRUCTURAL_CHECKS = (
    'transport_identity_and_mode', 'actual_input_binding', 'ordinary_physics_contract',
    'complete_case_material_section', 'finite_numerical_standing', 'complete_unique_scalar_rows',
    'normal_maximum_evidence', 'governing_location', 'stress_summary_binding', 'displacement_summary_binding',
)


# Authoritative IDs from the exact selected original selector files in
# REFERENCE_BASIS_SHA256; both cases use these same 73 ordered IDs. No target values.
REQUIRED_CASE_IDS = ('original_static_axial_v1', 'original_static_bending_torsion_v1')
REQUIRED_ASSERTION_IDS = (
    'node.root.ux',
    'node.root.uy',
    'node.root.uz',
    'node.root.rx',
    'node.root.ry',
    'node.root.rz',
    'node.root.magnitude',
    'node.tip.ux',
    'node.tip.uy',
    'node.tip.uz',
    'node.tip.rx',
    'node.tip.ry',
    'node.tip.rz',
    'node.tip.magnitude',
    'support.Fx',
    'support.Fy',
    'support.Fz',
    'support.Mx',
    'support.My',
    'support.Mz',
    'support.force_magnitude',
    'support.moment_magnitude',
    'section.end_i.N',
    'section.end_i.Vy',
    'section.end_i.Vz',
    'section.end_i.T',
    'section.end_i.My',
    'section.end_i.Mz',
    'stress.end_i.axial',
    'stress.end_i.bending_y',
    'stress.end_i.bending_z',
    'stress.end_i.torsional_shear',
    'section.quarter_1.N',
    'section.quarter_1.Vy',
    'section.quarter_1.Vz',
    'section.quarter_1.T',
    'section.quarter_1.My',
    'section.quarter_1.Mz',
    'stress.quarter_1.axial',
    'stress.quarter_1.bending_y',
    'stress.quarter_1.bending_z',
    'stress.quarter_1.torsional_shear',
    'section.midspan.N',
    'section.midspan.Vy',
    'section.midspan.Vz',
    'section.midspan.T',
    'section.midspan.My',
    'section.midspan.Mz',
    'stress.midspan.axial',
    'stress.midspan.bending_y',
    'stress.midspan.bending_z',
    'stress.midspan.torsional_shear',
    'section.quarter_3.N',
    'section.quarter_3.Vy',
    'section.quarter_3.Vz',
    'section.quarter_3.T',
    'section.quarter_3.My',
    'section.quarter_3.Mz',
    'stress.quarter_3.axial',
    'stress.quarter_3.bending_y',
    'stress.quarter_3.bending_z',
    'stress.quarter_3.torsional_shear',
    'section.end_j.N',
    'section.end_j.Vy',
    'section.end_j.Vz',
    'section.end_j.T',
    'section.end_j.My',
    'section.end_j.Mz',
    'stress.end_j.axial',
    'stress.end_j.bending_y',
    'stress.end_j.bending_z',
    'stress.end_j.torsional_shear',
    'stress.maximum_absolute_normal',
)
REQUIRED_SECTION_IDS = tuple('section.' + name for name in (
    'outside_diameter_m', 'effective_wall_thickness_m', 'ro_m', 'ri_m',
    'As_m2', 'Ai_m2', 'I_m4', 'J_m4', 'Z_m3'))

def section_obligations(state='not_run', reason='not executed'):
    return [{'id': key, 'state': state, 'reason': reason} for key in REQUIRED_SECTION_IDS]

def validate_submitted_inventory(cases):
    require(type(cases) is list and all(type(case) is dict for case in cases), 'finite submitted cases must be objects')
    require([case.get('id') for case in cases] == list(REQUIRED_CASE_IDS), 'complete ordered first-static case inventory required')
    for case in cases:
        assertions = case.get('assertions')
        require(type(assertions) is list and all(type(row) is dict for row in assertions), 'finite submitted assertions must be objects')
        require([row.get('id') for row in assertions] == list(REQUIRED_ASSERTION_IDS), 'complete ordered first-static assertion inventory required')


def require(ok, reason):
    if not ok:
        raise ValueError('ORDINARY-PHYSICS: ' + reason)


def bounded(path: Path, expected: str, limit: int = LIMIT) -> bytes:
    with path.open('rb') as stream:
        data = stream.read(limit + 1)
    require(len(data) <= limit and sha256_bytes(data) == expected, 'bound bytes differ or exceed limit: ' + str(path))
    return data


def project_root(source_root: Path) -> Path:
    # Resolve the two explicit caller conventions to one recorded origin.
    if (source_root / MODULE).is_file():
        return source_root.resolve()
    candidate = source_root / 'projects/chirality-piping'
    require((candidate / MODULE).is_file(), 'owned validator not available in selected candidate')
    return candidate.resolve()


def load_binding(source_root: Path, binding_path: Path, expected_sha256: str,
                 *, require_reviewed: bool = True) -> dict:
    data = bounded(binding_path, expected_sha256)
    binding = json.loads(data)
    require(binding.get('format') == 'openpipestress.ordinary_physics_consistency_binding/1'
            and binding.get('contract_id') == CONTRACT, 'unsupported binding identity')
    require(binding.get('entrypoint') == 'validate_physics_evidence', 'unsupported owned entry')
    if require_reviewed:
        require(binding.get('status') == 'reviewed_candidate' and bool(binding.get('review_basis')),
                'reviewed candidate/module binding not admitted')
    selected = project_root(source_root)
    files = binding.get('files')
    require(type(files) is list and len(files) == 3, 'closed dependency inventory required')
    paths = [row.get('path') for row in files]
    require(set(paths) == {MODULE, TABLE, UNITS} and len(set(paths)) == len(paths), 'dependency inventory mismatch')
    content = {}
    for row in files:
        require(set(row) == {'path', 'sha256'}, 'unknown dependency fields')
        require(row['sha256'] == DEPENDENCIES[row['path']], 'unselected owned validator dependency')
        path = (selected / row['path']).resolve()
        require(path.is_relative_to(selected), 'dependency leaves selected candidate root')
        content[row['path']] = bounded(path, row['sha256'])
    review_bytes = []
    if require_reviewed:
        for review in binding['review_basis']:
            require(type(review) is dict and set(review) == {'path', 'sha256'}, 'review binding shape invalid')
            path = (selected / review['path']).resolve()
            require(path.is_relative_to(selected), 'review origin leaves selected candidate root')
            review_bytes.append({'path': str(path), 'sha256': review['sha256'], 'bytes': bounded(path, review['sha256'])})
    require(sha256_bytes(content[TABLE]) == TABLE_SHA256, 'ordinary semantic table changed')
    table = json.loads(content[TABLE])
    require(table.get('semantic_contract_id') == CONTRACT, 'semantic identity mismatch')
    return {'source_root': selected, 'binding_path': binding_path.resolve(), 'binding_sha256': expected_sha256,
            'binding': binding, 'semantic_table': table, 'binding_bytes': data, 'dependency_bytes': content, 'review_bytes': review_bytes,
            'admitted': require_reviewed and binding.get('status') == 'reviewed_candidate'}


def consistency_observation(raw: dict, binding: dict, directory: Path) -> dict:
    """Execute only the owned physical-consistency reader in an isolated helper.

    Raw is a transport snapshot extracted from separately captured/checked solver
    stdout. This derived replay hash is not the original stdout's raw-byte hash.
    """
    require(raw.get('schema_version') == '0.2.0' and raw.get('producer') == {
        'component_name': 'open_pipe_stress_product_physics', 'component_version': '0.2.0',
        'semantic_contract_id': CONTRACT}, 'ordinary raw/producer dispatch mismatch')
    require('source_block_recovery' not in raw and 'carrier_evidence' not in raw, 'source/composite namespace refused')
    request_bytes = json.dumps(raw, ensure_ascii=True, separators=(',', ':'), allow_nan=False).encode()
    require(len(request_bytes) <= LIMIT, 'physical consistency input limit')
    require(bounded(binding['binding_path'], binding['binding_sha256']) == binding['binding_bytes'], 'binding changed before helper')
    helper_sha256 = file_sha256(HELPER)
    arguments = ['-I', '-S', str(HELPER), '--source-root', str(binding.get('snapshot_root', binding['source_root'])),
                 '--binding', str(binding.get('snapshot_binding_path', binding['binding_path'])), '--binding-sha256', binding['binding_sha256']]
    process = capture(Path(sys.executable).resolve(), arguments, request_bytes, directory,
                      timeout_seconds=30, output_limit_bytes=LIMIT,
                      expected_executable_sha256=file_sha256(Path(sys.executable).resolve()))
    require(process['outcome'] == 'completed' and process['stdin_delivery_complete'], 'consistency helper refused or failed')
    require(file_sha256(HELPER) == helper_sha256, 'fixed helper changed during execution')
    response_bytes = bounded(directory / 'stdout.bin', process['stdout_sha256'])
    require(len(response_bytes) == process['stdout_bytes'], 'helper response byte count differs')
    response = json.loads(response_bytes)
    require(response.get('artifact') == 'openpipestress.ordinary_physics_consistency_observation'
            and response.get('version') == '1.0.0' and response.get('verdict') == 'consistent', 'unknown or refused helper response')
    require(response.get('binding_sha256') == binding['binding_sha256']
            and response.get('source_bytes_sha256') == sha256_bytes(request_bytes)
            and response.get('dependencies') == binding['binding']['files']
            and response.get('contract_id') == CONTRACT, 'helper response identity mismatch')
    for row in binding['binding']['files']:
        bounded(binding['source_root'] / row['path'], row['sha256'])
    return {'process': process, 'response': response, 'helper_sha256': helper_sha256,
            'binding_sha256': binding['binding_sha256'], 'binding_admitted': binding['admitted'],
            'scope': 'physical consistency of parsed snapshot only; runner provenance and numerical comparison are separate'}


def ordinary_header_and_standing(raw: dict, request: dict, mode: str) -> None:
    require(mode in ('sparse_interactive', 'dense_scrutiny'), 'unsupported actual mode')
    require(raw.get('schema_version') == '0.2.0' and raw.get('producer') == {
        'component_name': 'open_pipe_stress_product_physics', 'component_version': '0.2.0',
        'semantic_contract_id': CONTRACT}, 'ordinary raw/producer identity required')
    require('source_block_recovery' not in raw and 'carrier_evidence' not in raw, 'source/composite namespace refused')
    formulation = raw.get('formulation_basis')
    require(type(formulation) is dict and set(formulation) == {'profile_id', 'limitations'}
            and formulation['profile_id'] == PROFILE and type(formulation['limitations']) is list
            and formulation['limitations'] and all(type(x) is str and x for x in formulation['limitations']), 'formulation basis invalid')
    model = request['solve']['preview_model']['model']
    require(model.get('schema_version') == '0.3.0' and model.get('pressure_contract') == {
        'version': '2.0.0', 'mode': PROFILE}, 'actual exact-profile model required')
    expected_cases = [case['id'] for case in model['load_cases']]
    require(expected_cases and len(set(expected_cases)) == len(expected_cases), 'actual case inventory invalid')
    quality = raw.get('numerical_quality')
    require(type(quality) is dict and set(quality) == {'value_representation', 'publication_quantization', 'integrity_policy', 'status', 'cases'}, 'numerical quality shape invalid')
    require(quality['value_representation'] == 'finite_binary64' and quality['publication_quantization'] == 'none'
            and quality['integrity_policy'] == 'M03-INTEGRITY-v1' and quality['status'] == 'checks_passed', 'ordinary numerical standing not satisfied')
    cases = quality['cases']
    require(type(cases) is list and len(cases) == len(expected_cases), 'quality case denominator mismatch')
    found = []
    diagnostic_rows = raw.get('diagnostics')
    require(type(diagnostic_rows) is list and all(type(row) is dict and type(row.get('id')) is str and row['id'] for row in diagnostic_rows), 'diagnostic identity missing')
    diagnostics = {row['id']: row for row in diagnostic_rows}
    require(len(diagnostics) == len(diagnostic_rows), 'duplicate diagnostic identity')
    for case in cases:
        require(type(case) is dict, 'quality case must be an object')
        require(set(case) == {'basis_ref', 'structural_status', 'solve_quality', 'model_matrix_fidelity', 'accuracy_evidence', 'evidence_refs'}, 'quality case shape invalid')
        basis = case['basis_ref']
        require(type(basis) is dict and set(basis) == {'ref_type', 'ref_id'} and basis['ref_type'] == 'load_case', 'quality case basis invalid')
        found.append(basis['ref_id'])
        require(case['structural_status'] == 'passive_model_basis' and case['solve_quality'] == 'checks_passed'
                and case['model_matrix_fidelity'] == 'represented_equations_retained' and case['accuracy_evidence'] == 'not_claimed', 'ordinary case standing/claim mismatch')
        refs = case['evidence_refs']
        require(type(refs) is list and refs and len(set(refs)) == len(refs), 'numerical diagnostics missing')
        require(all(ref in diagnostics and diagnostics[ref]['code'] == 'NUMERICAL_INTEGRITY_CHECKS_PASSED'
                    and basis['ref_id'] in diagnostics[ref].get('affected_refs', []) for ref in refs), 'numerical diagnostic binding mismatch')
    require(len(set(found)) == len(found) and set(found) == set(expected_cases), 'actual quality case IDs differ')


def verify_selected_case(case: dict, prepared: dict, reference_basis: dict) -> list[dict]:
    """Match the exact Root-selected originals, allowing review metadata only.

    This is a finite test-profile binding, not fixture recognition in the solver.
    A new problem or criterion needs a separately identified profile revision.
    """
    selected = [row for row in reference_basis['value']['cases'] if row['case_id'] == case['id']]
    require(len(selected) == 1, 'case is not in the selected finite package')
    selected = selected[0]
    base = reference_basis['path'].parent
    originals, snapshots = {}, []
    for role in ('input', 'reference', 'criterion', 'selectors'):
        record = selected[role]
        data = bounded(base / record['path'], record['sha256'])
        originals[role] = json.loads(data)
        snapshots.append({'role': role, 'path': str((base / record['path']).resolve()), 'sha256': record['sha256'], 'bytes': data})
    require(sha256_bytes(prepared['input']) == selected['input']['sha256'], 'finite reviewed input bytes changed')
    reference = json.loads(prepared['reference_bytes'])
    restored_reference = deepcopy(reference)
    restored_reference['readiness'] = originals['reference']['readiness']
    restored_reference['independent_review_ref'] = originals['reference']['independent_review_ref']
    require(restored_reference == originals['reference'], 'reference values or non-admission fields changed')
    criterion = deepcopy(prepared['profile'])
    original_profile = originals['criterion']['tolerance_profile']
    criterion['profile_status'] = original_profile['profile_status']
    require(len(criterion['rules']) == len(original_profile['rules']), 'criterion rule denominator changed')
    for derived, original in zip(criterion['rules'], original_profile['rules']):
        derived['review'] = original['review']
    require(criterion == original_profile, 'criterion numeric values or non-review fields changed')
    expected_assertions = [{key: row[key] for key in ('id', 'selector', 'criterion_rule_id')}
                           for row in originals['selectors']['assertions']]
    require(case['assertions'] == expected_assertions, 'reviewed scalar selectors/criteria changed')
    structure = prepared['structural']
    require(structure['obligations']['sha256'] == STRUCTURAL_OBLIGATIONS_SHA256, 'selected structural obligations changed')
    require(structure['section_criteria']['sha256'] == SECTION_CRITERIA_SHA256, 'unselected structural numeric addendum')
    require(structure['section_reference']['sha256'] == structure['section_criteria']['value']['reference_basis']['sha256'], 'original section reference changed')
    return snapshots
