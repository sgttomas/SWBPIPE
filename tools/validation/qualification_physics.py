"""Closed ordinary pressure-2 adapter support for the first-static cases; physical consistency is not origin.

B7b retired physics-1 (2.0.0/exact_straight_pressure_v2) and the 0.1.0-0.4.0 model documents. The two
first-static cases are qualified as pressure-2 (3.0.0/exact_pressure_v3 on model document 0.9.0): the frozen,
SHA-bound runner inputs are never edited; the gate verifies their bytes and submits a re-authored copy
(``reauthor_runner_input``, the product tests' one re-authoring rule) to the selected runner.

The final reviewed candidate binding is supplied explicitly. Its physical-consistency
verdict is the Rust product reader's (b7-plan R2: Rust is the only product reader),
taken from the explicitly built `openpipestress-runner read pressure` named by
OPENPIPESTRESS_RUNNER_BIN. No build, network, unit conversion or older-profile fallback occurs.
"""
from __future__ import annotations
from copy import deepcopy
import json
from pathlib import Path
import sys

PROJECT = Path(__file__).resolve().parents[2]
if str(PROJECT) not in sys.path:
    sys.path.insert(0, str(PROJECT))
try:
    from .qualification_process import capture, file_sha256, sha256_bytes
except ImportError:
    from qualification_process import capture, file_sha256, sha256_bytes
from core.analysis_runs import rust_reader

TRANSPORT = 'ordinary_pressure_1_cli_1.0_raw0.2'
CONTRACT = 'openpipestress.result_semantics/0.3.0/pressure-2'
PROFILE = 'exact_pressure_v3'
TABLE = 'fixtures/results/semantic_contract_v0_3_pressure_2.json'
# b7-plan R2: the owned reader is Rust's current pressure-2 reader (`validate_pressure_evidence`, held in
# physics_evidence.rs); the binding selects its source with the table and unit authority. Re-pin deliberately
# whenever the reader changes. B7b re-pinned all three: physics-1's reader and table (a6681f69...) are retired.
MODULE = 'core/reporting/result_export/src/physics_evidence.rs'
UNITS = 'core/units/src/lib.rs'
# T4-U6 (D-5, §2.4): the table's per-basis stress signatures (106 signatures, 59 kinds); T4-U5b's tie-rod force
# and T4-U4b's (§2.2) per-basis kinds (129 signatures, 66 kinds).
TABLE_SHA256 = '5451fbf3e10e085c90b523baa7daf5fc723659e0d874d988d23807be7d2fefe6'
DEPENDENCIES = {
    # T4-U6/U8 re-pins: the D-5 section bases and the Timoshenko beam theory (8bf263d6..., was 38302413...);
    # then §2.5's realized-arc marker (98bdfeee3e: arcs known by `member_kind`, refused on the composite);
    # then model 0.6.0: the reader's comments name pressure-2 (comment text only; was f91e678e...);
    # then T4-U4b: the section stress, toroidal, arc maximum and directional rows (was 8a79bfcf...);
    # then T4-U4b S4 (§4.3): the composite admits the straight σ±/τ signatures of a selected source case (was 79ec35b3...);
    # then S4 round 2: the composite recomputes every straight member's σ±/τ and their coverage (was 4bc56dea...);
    # then S4 round 3: the composite refuses TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN by name (was 52a15bec...);
    # then the T4-U4b review repairs: basis-twin order and no transverse-shear Poisson exemption (merged with S4;
    # was c7699864..., 56f18624..., 58491ca8... and 20337f1c...);
    # then W-7 (DEL-04-01 global-end-actions.md): the reader checks the global end-action rows (was 520e7189...),
    # after the published rows' own coverage (was 8b61f340...); then W-15 slice d (DEL-03-05 Design §5): the
    # component-body records, bodies out of the maxima and headlines, merged with W-7 (was ad8edd63... on W-7 and
    # 420e60e4... on W-15); then W-16 slice c (DEL-05-01 exact-equivalent-static.md §2): the generated-primitive
    # evidence binding (was 0b296b0e...); then W-16 slice b (DEL-05-02 exact-combinations.md §6): the direct pass
    # reads the document without its combination rows and records, then the combination pass re-derives them
    # (was ef200d25...); then RT-A3 increment 1 (DEL-04-07 pressure-contract.md, "Tees and pressure junctions"): the
    # direct pass binds the branch-intensified rows by their records (`u4b_evidence::branch_records`; was c82a0f11...).
    MODULE: 'f27242f055f91b09b649e5a2ddc4e6b76cf9ddd7325bd9df3d7839bc5d609b83',
    TABLE: TABLE_SHA256,
    UNITS: '521717eb4cac8114b5c1bbeef3f8caebd57eb3021c7743860f9be8f1078b710c',
}
# Version 3: the binding names the pressure-2 reader (version 2 named the retired physics-1 Rust reader,
# version 1 the former Python reader); both older formats are refused.
BINDING_FORMAT = 'openpipestress.ordinary_physics_consistency_binding/3'
ENTRYPOINT = 'validate_pressure_evidence'
READER = 'pressure'
# Model document 0.9.0 is the only current version (DEL-04-07 Design u4b-u5b-implementation.md §1.1); its exact
# contract names the shear model. 0.5.0, 0.6.0, 0.7.0 and 0.8.0 are retired and re-authored by their version alone
# (§1.4; DEL-02-01 Design model-document-0.7.0.md §2, model-document-0.9.0.md §3.6).
MODEL_VERSION = '0.9.0'
PREVIOUS_MODEL_VERSIONS = ('0.5.0', '0.6.0', '0.7.0', '0.8.0')
RETIRED_MODEL_VERSIONS = ('0.1.0', '0.2.0', '0.3.0', '0.4.0', *PREVIOUS_MODEL_VERSIONS)
PRESSURE_CONTRACT = {'version': '3.0.0', 'mode': PROFILE, 'shear_model': 'euler_bernoulli'}
REAUTHORED_PRESSURE_CONTRACTS = ({'version': '2.0.0', 'mode': 'exact_straight_pressure_v2'}, {'version': '3.0.0', 'mode': PROFILE})
ZERO_CORROSION_ALLOWANCE = {'value': 0.0, 'unit': 'm'}
REAUTHORING = 'openpipestress.current_model_reauthoring/1'
LIMIT = 8 * 1024 * 1024
REFERENCE_BASIS_SHA256 = 'be6e332513634f747a1f44c6f38bb94bd2ad0d7ce8e3caa75ccfcdfc9124bdf9'
SECTION_CRITERIA_SHA256 = '1ed8aac0910318902cd5f690aeee8cb74bfee036617e33eaa4e2c23ec98ff3c2'
STRUCTURAL_OBLIGATIONS_SHA256 = '6d7b5ed5a900b4ba417d32bc4624f1511812f8dc889438cc9bbf5aaca72660da'
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
# T4-U6 (D-5; DEL-05-03 section-bases-and-timoshenko.md §2.1, §2.5): every exact stress row is published on the
# nominal and on the reduced section, each naming its basis. The frozen selectors (bare recovery tokens, basis-free
# ids) are never edited; the gate derives a declared re-authored copy (``reauthor_selectors``): each stress selector
# becomes its nominal twin and its reduced twin, with the same expected value and criterion. The frozen 'stress.*'
# assertions are exactly the stress-kind selectors (checked when deriving).
SECTION_BASES = ('nominal_section', 'reduced_section')
SECTION_BASIS_SEPARATOR = '; section_basis='
STRESS_KINDS = (
    'element_local_axial_normal_stress', 'element_local_bending_normal_stress_y', 'element_local_bending_normal_stress_z',
    'element_local_torsional_shear_stress', 'pipe_elastic_normal_stress_maximum_v2', 'pipe_axial_membrane_stress_v2',
    'pipe_lame_radial_stress_v2', 'pipe_lame_hoop_stress_v2')
# §2.5: these id families end in the underscore basis; the element-stress and source-derived families in the hyphenated one.
UNDERSCORE_ID_FAMILIES = ('result:pressure-exact:', 'result:elastic-maximum:')
SELECTOR_REAUTHORING = 'openpipestress.section_basis_selector_reauthoring/1'
DERIVED_ASSERTION_IDS = tuple(name for aid in REQUIRED_ASSERTION_IDS
                              for name in ((f'{aid}.{basis}' for basis in SECTION_BASES) if aid.startswith('stress.') else (aid,)))


def section_basis_id(result_id: str, basis: str) -> str:
    """§2.5's id segment for one basis of a stress row."""
    return f'{result_id}:{basis}' if result_id.startswith(UNDERSCORE_ID_FAMILIES) else f"{result_id}:{basis.replace('_', '-')}"


def _canonical(value) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=False, allow_nan=False).encode()


def reauthor_selectors(assertions: list, values: list) -> tuple[list, list, dict]:
    """The declared re-authored copy of a frozen first-static selector list and its reference values.

    Each stress-kind selector becomes two, nominal then reduced: assertion id ``<id>.<basis>``, selector id with
    §2.5's basis segment, ``metadata.basis`` with ``; section_basis=<basis>``, and the same kind, unit, entity,
    case basis, dimension, criterion and expected value. Every other selector is unchanged. The frozen lists are
    not modified; the record carries both hashes."""
    require(type(assertions) is list and type(values) is list, 'selectors and reference values must be lists')
    by_id = {row.get('assertion_id'): row for row in values if type(row) is dict}
    require(len(by_id) == len(values) and set(by_id) == {row.get('id') for row in assertions if type(row) is dict},
            'reference assertion denominator mismatch')
    derived, derived_values, twinned = [], [], []
    for assertion in assertions:
        selector = assertion['selector']
        stress = selector.get('kind') in STRESS_KINDS
        require(stress == assertion['id'].startswith('stress.'), 'stress selector inventory differs from the frozen stress assertions')
        if not stress:
            derived.append(deepcopy(assertion))
            derived_values.append(deepcopy(by_id[assertion['id']]))
            continue
        metadata = selector.get('metadata')
        require(type(metadata) is dict and type(metadata.get('basis')) is str and SECTION_BASIS_SEPARATOR not in metadata['basis'],
                'frozen stress selector must name a basis-free recovery token')
        twinned.append(assertion['id'])
        for basis in SECTION_BASES:
            twin = deepcopy(assertion)
            twin['id'] = f"{assertion['id']}.{basis}"
            twin['selector']['id'] = section_basis_id(selector['id'], basis)
            twin['selector']['metadata']['basis'] = metadata['basis'] + SECTION_BASIS_SEPARATOR + basis
            derived.append(twin)
            value = deepcopy(by_id[assertion['id']])
            value['assertion_id'] = twin['id']
            derived_values.append(value)
    record = {'rule': SELECTOR_REAUTHORING,
              'frozen_selectors_sha256': sha256_bytes(_canonical(assertions)),
              'derived_selectors_sha256': sha256_bytes(_canonical(derived)),
              'frozen_reference_values_sha256': sha256_bytes(_canonical(values)),
              'derived_reference_values_sha256': sha256_bytes(_canonical(derived_values)),
              'assertion_counts': [len(assertions), len(derived)], 'twinned_assertion_ids': twinned,
              'scope': 'declared re-authoring of a copy: each stress selector as its nominal and reduced twin with the same '
                       'expected value and criterion; the frozen selectors, references and criteria are unchanged'}
    return derived, derived_values, record

def reauthor_model(model: dict) -> dict:
    """The one re-authoring of a copied model document (Rust `tests/common/current_model.rs` `reauthor_model`): a
    0.5.0, 0.6.0, 0.7.0 or 0.8.0 document changes its ``schema_version`` to 0.9.0 and nothing else. An older document (B7b's rule): a retired
    ``schema_version`` -> 0.9.0; a declared v2 or v3 exact contract -> the current contract; and, on a document
    carrying the current contract, an explicit zero ``corrosion_allowance`` on every pipe section that lacks one (the
    retired contracts had no allowance: the authored wall was the effective wall). Nothing else changes: no physical
    or numerical input and no other contract. The caller's object is not modified."""
    model = deepcopy(model)
    if model.get('schema_version') in PREVIOUS_MODEL_VERSIONS:
        model['schema_version'] = MODEL_VERSION
        return model
    if model.get('schema_version') in RETIRED_MODEL_VERSIONS:
        model['schema_version'] = MODEL_VERSION
    contract = model.get('pressure_contract')
    if type(contract) is dict and {key: contract.get(key) for key in ('version', 'mode')} in REAUTHORED_PRESSURE_CONTRACTS:
        model['pressure_contract'] = deepcopy(PRESSURE_CONTRACT)
    if model.get('pressure_contract') == PRESSURE_CONTRACT:
        for pipe in model.get('pipe_segments') or []:
            pipe['section'].setdefault('corrosion_allowance', deepcopy(ZERO_CORROSION_ALLOWANCE))
    return model


def model_carries_load_state(model) -> bool:
    """Authored presence selects the optional load/reference-state group (Rust `physics_source::model_carries_load_state`):
    ``reference_configurations``, any material's ``expansion_laws`` or any case's ``analysis_state``; never a version."""
    if type(model) is not dict:
        return False
    materials, cases = model.get('materials'), model.get('load_cases')
    return ('reference_configurations' in model
            or (type(materials) is list and any(type(m) is dict and 'expansion_laws' in m for m in materials))
            or (type(cases) is list and any(type(c) is dict and 'analysis_state' in c for c in cases)))


def reauthor_runner_input(data: bytes) -> tuple[bytes, dict]:
    """The runner input submitted for a frozen, SHA-bound CLI input: the same request with its preview model
    re-authored (``reauthor_model``). Returns the submitted bytes and the closed re-authoring record."""
    request = json.loads(data)
    require(type(request) is dict and type(request.get('solve')) is dict and type(request['solve'].get('preview_model')) is dict
            and type(request['solve']['preview_model'].get('model')) is dict, 'runner input has no preview model to re-author')
    original = request['solve']['preview_model']['model']
    request['solve']['preview_model']['model'] = reauthor_model(original)
    submitted = json.dumps(request, ensure_ascii=False, separators=(',', ':'), allow_nan=False).encode()
    # A 0.5.0, 0.6.0, 0.7.0 or 0.8.0 copy moves only its version: its contract and sections are recorded as they stand.
    version_only = original.get('schema_version') in PREVIOUS_MODEL_VERSIONS
    return submitted, {'rule': REAUTHORING, 'frozen_input_sha256': sha256_bytes(data), 'submitted_input_sha256': sha256_bytes(submitted),
                       'model_schema_version': [original.get('schema_version'), MODEL_VERSION],
                       'pressure_contract': [original.get('pressure_contract'),
                                             original.get('pressure_contract') if version_only else PRESSURE_CONTRACT],
                       'explicit_zero_corrosion_allowance': not version_only,
                       'scope': 'declared re-authoring of a copy; the frozen input, references and criteria are unchanged'}


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
    require(binding.get('format') == BINDING_FORMAT
            and binding.get('contract_id') == CONTRACT, 'unsupported binding identity')
    require(binding.get('entrypoint') == ENTRYPOINT, 'unsupported owned entry')
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


def reader_verdict(process: dict, response_bytes: bytes, reader: str, require=None) -> tuple[str, str | None, dict]:
    """The Rust reader's verdict from one captured `read <reader>` process: ('consistent', None, outcome) on exit 0,
    ('refused', its text, outcome) on exit 1; any other process or response is an error (the caller's `require`)."""
    require = require or globals()['require']
    try:
        verdict = json.loads(response_bytes)
    except ValueError:
        verdict = None
    outcome = verdict.get('outcome') if type(verdict) is dict else None
    require(type(verdict) is dict and set(verdict) == {'artifact', 'schema_version', 'reader', 'outcome'}
            and verdict['artifact'] == rust_reader.ARTIFACT and verdict['schema_version'] == rust_reader.SCHEMA_VERSION
            and verdict['reader'] == reader and type(outcome) is dict, 'unknown reader response')
    if process['outcome'] == 'completed' and process['stdin_delivery_complete'] and outcome == {'ok': True}:
        return 'consistent', None, outcome
    error = outcome.get('error')
    require(process['outcome'] == 'nonzero_exit' and process['return_code'] == 1 and outcome.get('ok') is False
            and set(outcome) == {'ok', 'error'} and type(error) is dict and set(error) == {'code', 'detail'}
            and type(error['code']) is str and error['code'] and (error['detail'] is None or type(error['detail']) is str),
            'reader process failed: ' + process['outcome'])
    return 'refused', error['detail'] or error['code'], outcome


def consistency_observation(raw: dict, binding: dict, directory: Path) -> dict:
    """Take the Rust product reader's verdict on the parsed snapshot (`openpipestress-runner read pressure`).

    Raw is a transport snapshot extracted from separately captured/checked solver
    stdout. This derived replay hash is not the original stdout's raw-byte hash.
    The reader executable is identified by its digest only; it is not proof of a
    build from the bound sources.
    """
    require(raw.get('schema_version') == '0.2.0' and raw.get('producer') == {
        'component_name': 'open_pipe_stress_product_physics', 'component_version': '0.2.0',
        'semantic_contract_id': CONTRACT}, 'ordinary raw/producer dispatch mismatch')
    require('source_block_recovery' not in raw and 'carrier_evidence' not in raw, 'source/composite namespace refused')
    request_bytes = json.dumps({'source': raw}, ensure_ascii=True, separators=(',', ':'), allow_nan=False).encode()
    require(len(request_bytes) <= LIMIT, 'physical consistency input limit')
    require(bounded(binding['binding_path'], binding['binding_sha256']) == binding['binding_bytes'], 'binding changed before reader')
    for row in binding['binding']['files']:
        bounded(binding['source_root'] / row['path'], row['sha256'])
    executable = rust_reader.runner_executable().resolve()
    executable_sha256 = file_sha256(executable)
    process = capture(executable, ['read', READER], request_bytes, directory,
                      timeout_seconds=30, output_limit_bytes=LIMIT, expected_executable_sha256=executable_sha256)
    require(file_sha256(executable) == executable_sha256, 'reader executable changed during execution')
    response_bytes = bounded(directory / 'stdout.bin', process['stdout_sha256'])
    require(len(response_bytes) == process['stdout_bytes'], 'reader response byte count differs')
    verdict, reason, outcome = reader_verdict(process, response_bytes, READER)
    require(verdict == 'consistent', 'owned reader refused: ' + str(reason))
    for row in binding['binding']['files']:
        bounded(binding['source_root'] / row['path'], row['sha256'])
    response = {'artifact': rust_reader.ARTIFACT, 'reader': READER, 'verdict': verdict, 'outcome': outcome,
                'binding_sha256': binding['binding_sha256'], 'source_bytes_sha256': sha256_bytes(request_bytes),
                'dependencies': binding['binding']['files'], 'contract_id': CONTRACT,
                'claim': 'internal physical consistency only; no input freshness, producer origin, numerical accuracy or Current qualification'}
    return {'process': process, 'response': response, 'reader_executable_sha256': executable_sha256,
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
    require(model.get('schema_version') == MODEL_VERSION and model.get('pressure_contract') == PRESSURE_CONTRACT,
            'actual exact-profile model required')
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
