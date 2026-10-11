#!/usr/bin/env python3
"""Closed VP-HARNESS adapter for the load-reference-1 raw0.2 transport.

Mirrors the ordinary physics-1 adapter: an explicitly selected foreground
``openpipestress-runner solve --input - --solver-mode <mode>`` process, the
ControlledExport/CLI1.0 wrapper and a raw0.2 mechanics envelope whose producer
is exactly ``openpipestress.result_semantics/0.3.0/load-reference-1`` with
profile ``resolved_straight_load_state_v1``. Cases are not hard-coded: they
come from a hash-locked case manifest
(``openpipestress.load_reference_qualification_manifest/1``) named by a run
selection record. Every required selector, including fields of
``contract_evidence.load_reference_states``, resolves exactly once. Numerical
standing is reported beside, never inside, the comparison outcomes.

Records observed outcomes only: never app Current, independent reference
approval, physical validation, professional acceptance or release
qualification. The joined ``load-reference-source-1`` transport, legacy raw0.1,
physics-1, physics-source/composite and unknown future profiles are refused.
No build, network, unit conversion, reference regeneration or fallback occurs.
"""
from __future__ import annotations
import argparse
from datetime import datetime, timezone
import json
import math
import os
from pathlib import Path, PurePosixPath
import platform
import re
import subprocess
import sys

PROJECT = Path(__file__).resolve().parents[2]
if str(PROJECT) not in sys.path:
    sys.path.insert(0, str(PROJECT))
try:
    from .qualification_process import capture, file_sha256, sha256_bytes
    from . import qualification_gate as gate
    from . import qualification_physics as physics
except ImportError:
    from qualification_process import capture, file_sha256, sha256_bytes
    import qualification_gate as gate
    import qualification_physics as physics

TRANSPORT = 'load_reference_1_cli_1.0_raw0.2'
RUN_FORMAT = 'openpipestress.load_reference_qualification_run/1'
MANIFEST_FORMAT = 'openpipestress.load_reference_qualification_manifest/1'
SELECTOR_FORMAT = 'openpipestress.first_static_selector_candidate/1'
REFERENCE_FORMAT = gate.REFERENCE_FORMAT
BINDING_FORMAT = 'openpipestress.load_reference_consistency_binding/1'
ENTRYPOINT = 'validate_load_reference_evidence'
CONTRACT = 'openpipestress.result_semantics/0.3.0/load-reference-1'
PROFILE = 'resolved_straight_load_state_v1'
TABLE = 'fixtures/results/semantic_contract_v0_3_load_reference_1.json'
TABLE_SHA256 = '44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d'
MODULE = 'core/analysis_runs/load_reference_evidence.py'
# Reader bytes as integrated with WP1 at 1ccca8b87 (REVIEW_A follow-up; earlier pins bfef71b19 and the WP5 start commit d8f0dc4f7);
# re-pin deliberately whenever the reader changes.
MODULE_SHA256 = '14e1750ebb96c8284d71b49e01e55455c9e42fa6197ac979f0118271503b57c7'
UNITS = physics.UNITS
DEPENDENCIES = {
    MODULE: MODULE_SHA256,
    physics.MODULE: physics.DEPENDENCIES[physics.MODULE],
    TABLE: TABLE_SHA256,
    physics.TABLE: physics.TABLE_SHA256,
    UNITS: physics.DEPENDENCIES[UNITS],
}
ANALYTICAL_REFERENCE = 'core/product_physics/tests/fixtures/load_reference_states/reference_cases.json'
HELPER = Path(__file__).with_name('qualification_load_reference_helper.py')
LIMIT = gate.MAX_INPUT_BYTES  # bound input files: manifest, selectors, references, criteria, binding
MAX_OUTPUT_LIMIT = 64 * 1024 * 1024  # the run's selectable process output limit ceiling
MODES = ('sparse_interactive', 'dense_scrutiny')
MODE_CODES = {'sparse_interactive': 1.0, 'dense_scrutiny': 2.0}
RECOVERY_METHODS = {'sparse_interactive': 'ordinary_sparse_structural_v1', 'dense_scrutiny': 'ordinary_dense_structural_v1'}
PRODUCER = {'component_name': 'open_pipe_stress_product_physics', 'component_version': '0.2.0', 'semantic_contract_id': CONTRACT}
PRESSURE_CONTRACT = {'version': '2.0.0', 'mode': 'exact_straight_pressure_v2'}
NOT_JOINED = {'status': 'not_joined', 'code': 'LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED'}
STATES = gate.STATES
STANDING_VALUES = ('not_assessed', 'checks_passed', 'sensitive', 'unresolved', 'failed')
REQUIRED_STRUCTURAL_CHECKS = (
    'transport_identity_and_mode', 'actual_input_binding', 'load_reference_contract', 'numerical_standing',
    'load_reference_record_binding', 'owned_reader_consistency', 'complete_unique_selector_coverage',
)
MANIFEST_CASE_KEYS = {'case_id', 'runner_input', 'product_request', 'reference', 'selectors', 'criteria',
                      'analytical_reference', 'modes', 'required_scalar_rows'}
RUN_KEYS = {'format', 'profile_id', 'purpose', 'transport', 'runner', 'case_manifest', 'reader_binding'}
RUNNER_KEYS = {'candidate_commit', 'executable_sha256', 'solver_mode', 'explicit_local_private_intent'}
SELECTOR_FILE_KEYS = {'format', 'case_id', 'producer_contract', 'raw_schema_version', 'row_namespace', 'assertions'}
SELECTOR_FILE_OPTIONAL = {'gaps', 'scoring_readiness', 'negative_assertions'}
ROW_NAMESPACE = 'payload.mechanics_envelope.results'
EVIDENCE_NAMESPACE = 'contract_evidence.load_reference_states'
EVIDENCE_SELECTOR_KEYS = {'id', 'namespace', 'basis_ref', 'record', 'key', 'field', 'definition', 'unit', 'dimension'}
DOFS = ('UX', 'UY', 'UZ', 'RX', 'RY', 'RZ')
_MATERIAL = 'load_reference_material'
_TEMPERATURE = 'load_reference_temperature'
_STRAIN = 'load_reference_strain'
_MOTION = 'load_reference_support_motion'
_LOAD = 'load_reference_applied_load'
# Closed evidence-field vocabulary. Each field is a fixed (unit, dimension,
# family); None marks a unit/dimension taken from the addressed record itself.
MEMBER_FIELDS = {
    'selected_E_pa': ('Pa', 'stress', _MATERIAL), 'derived_G_pa': ('Pa', 'stress', _MATERIAL),
    'selected_nu': ('1', 'dimensionless', _MATERIAL), 'interpolation_fraction': ('1', 'dimensionless', _MATERIAL),
    'operating_temperature_k': ('K', 'temperature', _TEMPERATURE),
    'material_selection_temperature_k': ('K', 'temperature', _TEMPERATURE),
    'installation_temperature_k': ('K', 'temperature', _TEMPERATURE),
    'coefficient_datum_k': ('K', 'temperature', _TEMPERATURE),
    'installation_datum_stretch': ('1', 'dimensionless', _STRAIN),
    'operating_datum_stretch': ('1', 'dimensionless', _STRAIN),
    'thermal_strain': ('1', 'dimensionless', _STRAIN), 'thermal_stretch': ('1', 'dimensionless', _STRAIN),
    'fit_strain': ('1', 'dimensionless', _STRAIN), 'fit_stretch': ('1', 'dimensionless', _STRAIN),
    'total_eigenstrain': ('1', 'dimensionless', _STRAIN),
    'reference_length_m': ('m', 'length', 'load_reference_geometry'),
}
SUPPORT_FIELDS = {'prescribed_value': (None, None, _MOTION)}
CONTRIBUTION_FIELDS = {
    'resolved_member_state': {'value': ('1', 'dimensionless', _STRAIN)},
    'support_state': {'value': (None, None, _MOTION)},
    'stored_primitive': {'factor': ('1', 'dimensionless', 'load_reference_load_factor'),
                         'authored_normalized_magnitude': (None, None, _LOAD),
                         'applied_magnitude': (None, None, _LOAD)},
}
RECORDS = {
    'member': ('members', ('pipe_id',), {'material_id', 'material_selection_kind', 'reference_basis', 'thermal_definition',
               'expansion_law_id', 'fit_kind', 'eigenstrain_composition', 'G_basis'}),
    'support_component': ('support_components', ('support_id', 'dof'),
                          {'node_id', 'law_kind', 'meaning', 'physical_state_source', 'global_dof'}),
    'contribution': ('contributions', ('owner_kind', 'source_id'), {'classification', 'category', 'dimension'}),
}
LOAD_UNITS = {'force': 'N', 'moment': 'N*m'}


class AdmissionError(gate.AdmissionError):
    pass


def require(ok, reason):
    if not ok:
        raise AdmissionError('LOAD-REFERENCE: ' + reason)


def strict_json_limited(data: bytes, limit: int):
    """``gate.strict_json`` with a caller-selected byte bound.

    Used only for runner stdout (and the derived reader snapshot), which the
    run admits up to its selected ``output_limit_bytes`` (at most 64 MiB). The
    parsing rules are the gate's, unchanged: UTF-8, duplicate members refused,
    NaN/Infinity refused, nonfinite and nonzero-underflowing numeric tokens
    refused. At or below the gate limit the gate function itself is used.
    """
    require(type(limit) is int and 0 < limit <= MAX_OUTPUT_LIMIT, 'invalid JSON byte limit')
    require(len(data) <= limit, 'JSON byte limit exceeded')
    if len(data) <= gate.MAX_INPUT_BYTES:
        return gate.strict_json(data)
    def forbidden(value):
        raise gate.AdmissionError('nonfinite JSON constant: ' + value)
    def real_token(token):
        value = float(token)
        gate.require(math.isfinite(value), 'nonfinite numeric token')
        significand = token.lower().split('e')[0]
        gate.require(value != 0.0 or not any(c in '123456789' for c in significand), 'nonzero numeric token underflow')
        return value
    try:
        return json.loads(data.decode('utf-8'), object_pairs_hook=gate._pairs, parse_constant=forbidden, parse_float=real_token)
    except (UnicodeDecodeError, json.JSONDecodeError, RecursionError) as exc:
        raise gate.AdmissionError('invalid UTF-8/JSON: ' + type(exc).__name__) from exc


def _text(value, label):
    require(type(value) is str and bool(value.strip()), label + ' missing')
    return value


def motion_unit(dof):
    require(dof in DOFS, 'unknown support degree of freedom')
    return ('m', 'length') if dof.startswith('U') else ('rad', 'angle')


# --------------------------------------------------------------------------
# Locked manifest and selectors (admission; no process yet)
# --------------------------------------------------------------------------

def project_root(source_root: Path) -> Path:
    if (source_root / MODULE).is_file():
        return source_root.resolve()
    candidate = source_root / 'projects/chirality-piping'
    require((candidate / MODULE).is_file(), 'owned load-reference reader not available in selected candidate')
    return candidate.resolve()


def read_project_bound(binding, project: Path, *, exact_path: str | None = None):
    """WORKING_ROOT-relative path/hash binding; no absolute path or escape."""
    require(type(binding) is dict and type(binding.get('path')) is str, 'exact path/hash binding required')
    relative = PurePosixPath(binding['path'])
    require(not relative.is_absolute() and '..' not in relative.parts and '\\' not in binding['path'] and binding['path'],
            'manifest paths must be WORKING_ROOT-relative without escape')
    if exact_path is not None:
        require(binding['path'] == exact_path, 'unselected analytical reference path')
    data, path = gate.read_bound(binding, project)
    require(path.is_relative_to(project), 'bound file leaves the selected candidate root')
    return data, path


def admit_manifest(data: bytes) -> dict:
    manifest = gate.strict_json(data)
    require(type(manifest) is dict and set(manifest) == {'format', 'cases'}, 'closed case manifest required')
    require(manifest['format'] == MANIFEST_FORMAT, 'unsupported case manifest version')
    cases = manifest['cases']
    require(type(cases) is list and 0 < len(cases) <= gate.MAX_CASES, 'nonzero bounded case inventory required')
    seen = set()
    for case in cases:
        require(type(case) is dict and set(case) == MANIFEST_CASE_KEYS, 'unknown/missing manifest case fields')
        cid = _text(case['case_id'], 'case ID')
        require(cid not in seen, 'duplicate case ID')
        seen.add(cid)
        for role in ('runner_input', 'product_request', 'reference', 'selectors', 'criteria'):
            require(type(case[role]) is dict and set(case[role]) == {'path', 'sha256'}, role + ' binding must be {path, sha256}')
        analytical = case['analytical_reference']
        require(type(analytical) is dict and set(analytical) == {'path', 'sha256', 'case_key'}, 'analytical reference binding shape')
        _text(analytical['case_key'], 'analytical case key')
        modes = case['modes']
        require(type(modes) is list and modes and len(set(modes)) == len(modes) and set(modes) <= set(MODES),
                'modes must be a nonempty unique subset of the two closed modes')
        rows = case['required_scalar_rows']
        require(type(rows) is int and 0 < rows <= gate.MAX_ASSERTIONS, 'positive required_scalar_rows required')
    return manifest


def _selector_identity(selector):
    if 'namespace' in selector:
        return ('evidence', json.dumps([selector.get(k) for k in ('basis_ref', 'record', 'key', 'field')], sort_keys=True))
    return ('row', selector.get('id'))


def admit_selectors(selectors: dict, case: dict) -> tuple[list[dict], list[dict]]:
    """Selector file shape and identities; semantics are checked in prepare_case."""
    require(type(selectors) is dict and SELECTOR_FILE_KEYS <= set(selectors) <= SELECTOR_FILE_KEYS | SELECTOR_FILE_OPTIONAL,
            'unknown/missing selector file fields')
    require(selectors['format'] == SELECTOR_FORMAT and selectors['case_id'] == case['case_id'], 'selector identity/version mismatch')
    require(selectors['producer_contract'] == CONTRACT, 'selectors bind another producer contract')
    require(selectors['raw_schema_version'] == '0.2.0' and selectors['row_namespace'] == ROW_NAMESPACE, 'selector raw namespace differs')
    positive = selectors['assertions']
    negative = selectors.get('negative_assertions', [])
    require(type(positive) is list and positive and type(negative) is list, 'nonempty assertion list required')
    ids = set()
    # Positive selectors are unique. A reference may name several wrong values
    # for one quantity, so negatives are keyed on their assertion ID; their
    # (selector, wrong value) pairs are checked unique once values are read.
    for rows, unique_selectors in ((positive, True), (negative, False)):
        identities = set()
        for row in rows:
            require(type(row) is dict and {'id', 'selector', 'criterion_rule_id'} <= set(row) <= {'id', 'selector', 'criterion_rule_id', 'selector_origin'},
                    'unknown/missing assertion fields')
            aid = _text(row['id'], 'assertion ID')
            require(aid not in ids, 'duplicate assertion ID within case')
            ids.add(aid)
            require(type(row['selector']) is dict, 'selector must be an object')
            selector_id = _text(row['selector'].get('id'), 'selector ID')
            identity = _selector_identity(row['selector'])
            if unique_selectors:
                require(identity not in identities and ('id', selector_id) not in identities, 'duplicate required selector')
                identities.update({identity, ('id', selector_id)})
    require(len(positive) == case['required_scalar_rows'], 'positive assertion count differs from required_scalar_rows')
    require(len(ids) <= gate.MAX_ASSERTIONS, 'assertion inventory limit')
    return positive, negative


def case_inventory(case: dict, project: Path) -> dict:
    """Assertion IDs for predeclaration; an unreadable file keeps the manifest count."""
    try:
        data, _ = read_project_bound(case['selectors'], project)
        positive, negative = admit_selectors(gate.strict_json(data), case)
        return {'positive': [row['id'] for row in positive], 'negative': [row['id'] for row in negative], 'refusal': None}
    except (ValueError, OSError, KeyError, TypeError) as exc:
        width = len(str(case['required_scalar_rows']))
        return {'positive': ['required-row-%0*d-unresolved' % (width, index + 1) for index in range(case['required_scalar_rows'])],
                'negative': [], 'refusal': 'selector inventory not admitted: ' + str(exc)}


def structural_obligations(state='not_run', reason='not executed'):
    return [{'id': key, 'state': state, 'reason': reason, 'details': {}} for key in REQUIRED_STRUCTURAL_CHECKS]


def predeclare(manifest: dict, mode: str, inventories: dict) -> dict:
    """Every intelligible required obligation exists before execution."""
    ledger, excluded = [], []
    for case in manifest['cases']:
        if mode not in case['modes']:
            excluded.append(case['case_id'])
            continue
        inventory = inventories[case['case_id']]
        assertions = ([{'id': aid, 'polarity': 'positive', 'state': 'not_run', 'reason': 'not executed', 'observed': None}
                       for aid in inventory['positive']] +
                      [{'id': aid, 'polarity': 'negative', 'state': 'not_run', 'reason': 'not executed', 'observed': None}
                       for aid in inventory['negative']])
        ledger.append({'id': case['case_id'], 'state': 'not_run', 'reason': 'not executed', 'assertions': assertions,
                       'structural_checks': structural_obligations(), 'process': None,
                       'inventory_basis': 'required_scalar_rows_placeholder' if inventory['refusal'] else 'selectors',
                       'numerical_standing': {'standing': 'unavailable', 'reason': 'not executed'}})
    require(bool(ledger), 'no manifest case is required in the selected mode')
    return {'artifact': 'openpipestress.qualification_development_ledger', 'version': '1.0.0', 'transport': TRANSPORT,
            'profile_id': None, 'selection_sha256': None, 'case_manifest_sha256': None, 'solver_mode': mode,
            'qualification': 'not_established_by_this_harness', 'outcome': 'not_run', 'publication_phase': 'in_progress',
            'cases_not_required_in_mode': excluded, 'cases': ledger, 'summary': {}, 'diagnostics': [],
            'created_at_utc': datetime.now(timezone.utc).isoformat()}


def fail_case(case: dict, state: str, reason: str) -> None:
    gate.fail_case(case, state, reason)
    if case['numerical_standing'].get('standing') in (None, 'unavailable'):
        case['numerical_standing'] = {'standing': 'unavailable', 'reason': reason}


def summarize(ledger: dict) -> None:
    gate.summarize(ledger)
    rows = [row for case in ledger['cases'] for row in case['assertions']]
    ledger['summary']['assertions_by_polarity'] = {
        polarity: {state: sum(row['polarity'] == polarity and row['state'] == state for row in rows) for state in STATES}
        for polarity in ('positive', 'negative')}
    standings = [case['numerical_standing'].get('standing', 'unavailable') for case in ledger['cases']]
    # Standing is reported beside the comparison outcomes, not folded into them.
    ledger['summary']['numerical_standing'] = {value: standings.count(value) for value in sorted(set(standings) | {'checks_passed', 'sensitive', 'insufficient', 'unavailable'})}


# --------------------------------------------------------------------------
# Per-case preparation: bindings, references, criteria and selector semantics
# --------------------------------------------------------------------------

def _rule(rules, rule_id, dimension, family, unit):
    found = [rule for rule in rules if rule.get('rule_id') == rule_id]
    require(len(found) == 1, 'criterion rule missing')
    rule = found[0]
    require(rule.get('dimension_id') == dimension, 'criterion dimension mismatch')
    require(family is not None and rule.get('result_family') == family, 'criterion result family mismatch')
    require(type(rule.get('unit_ref')) is dict and rule['unit_ref'].get('ref') == unit, 'criterion same-unit binding required')
    require(rule.get('normalization_basis') == 'same_unit_required', 'unit conversion not supported')
    require(rule.get('tolerance_value_status') in ('externally_governed', 'project_specific_review_required'), 'criterion unresolved')
    require(bool(rule.get('review')) and bool(rule.get('provenance')), 'criterion review/provenance missing')
    pair = ('relative_tolerance_value' in rule, 'absolute_tolerance_value' in rule)
    require(pair in ((True, True), (False, False)), 'incomplete tolerance pair')
    for key in ('relative_tolerance_value', 'absolute_tolerance_value') if pair[0] else ('tolerance_value',):
        require(gate.finite(rule.get(key)) >= 0, 'negative criterion')
    return rule


def row_semantics(selector, semantic_rows):
    require(set(selector) == gate.SELECTOR_KEYS, 'complete explicit row selector required')
    for key in ('id', 'kind', 'unit', 'entity_ref', 'dimension'):
        _text(selector[key], 'selector ' + key)
    metadata = selector['metadata']
    if metadata is not None:
        require(type(metadata) is dict and set(metadata) == set(gate.META), 'complete source metadata required')
        require(all(type(v) is str and v and v != 'TBD' for v in metadata.values()), 'unresolved source metadata')
    signatures = [row for row in semantic_rows if row['kind'] == selector['kind'] and row['unit'] == selector['unit']
                  and row['component'] == (metadata['component'] if metadata is not None else None)]
    require(len(signatures) == 1 and signatures[0]['source_physical_semantic_dimension'] == selector['dimension'],
            'selector contradicts pinned load-reference-1 semantics')
    return signatures[0]['family']


def evidence_semantics(selector):
    """Closed load_reference_states selector: (family) or refusal."""
    require(set(selector) == EVIDENCE_SELECTOR_KEYS, 'complete explicit evidence selector required')
    require(selector['namespace'] == EVIDENCE_NAMESPACE, 'unsupported evidence namespace')
    require(selector['record'] in RECORDS, 'unsupported load_reference_states record')
    _list, key_fields, definition_fields = RECORDS[selector['record']]
    key = selector['key']
    require(type(key) is dict and set(key) == set(key_fields), 'exact record key required')
    for name in key_fields:
        _text(key[name], 'record key ' + name)
    definition = selector['definition']
    require(type(definition) is dict and set(definition) <= definition_fields, 'unknown definition field')
    for name, value in definition.items():
        require(type(value) is int if name == 'global_dof' else type(value) is str and value, 'definition values must be exact text/index')
    field = selector['field']
    if selector['record'] == 'member':
        fields = MEMBER_FIELDS
    elif selector['record'] == 'support_component':
        require(key['dof'] in DOFS, 'unknown support degree of freedom')
        fields = SUPPORT_FIELDS
    else:
        require(key['owner_kind'] in CONTRIBUTION_FIELDS, 'contribution owner kind has no numeric field')
        fields = CONTRIBUTION_FIELDS[key['owner_kind']]
    require(field in fields, 'field outside the closed evidence vocabulary')
    unit, dimension, family = fields[field]
    if unit is None:
        if selector['record'] == 'support_component':
            unit, dimension = motion_unit(key['dof'])
        elif key['owner_kind'] == 'support_state':
            unit, dimension = motion_unit(key['source_id'].rsplit(':', 1)[-1])
        else:
            options = {(u, d) for d, u in LOAD_UNITS.items()}
            require((selector['unit'], selector['dimension']) in options, 'applied load unit/dimension unsupported')
            unit, dimension = selector['unit'], selector['dimension']
    require(selector['unit'] == unit and selector['dimension'] == dimension, 'evidence unit/dimension differs from closed vocabulary')
    return family


def basis_of(selector):
    basis = selector.get('basis_ref')
    require(type(basis) is dict and set(basis) == {'ref_type', 'ref_id'}, 'explicit case/state basis required')
    require(basis['ref_type'] == 'load_case', 'load-reference-1 selectors bind a load case')
    return _text(basis['ref_id'], 'case basis ID')


def load_case_ids(request):
    cases = request['solve']['preview_model']['model']['load_cases']
    ids = [case.get('id') if type(case) is dict else None for case in cases]
    require(type(cases) is list and ids and all(type(i) is str and i for i in ids) and len(set(ids)) == len(ids),
            'actual load case inventory invalid')
    return ids


def prepare_case(case: dict, project: Path, purpose: str, semantic_rows: list[dict]) -> dict:
    files, bindings = {}, []
    for role in ('runner_input', 'product_request', 'reference', 'selectors', 'criteria'):
        data, path = read_project_bound(case[role], project)
        files[role] = data
        bindings.append({'role': role, 'path': str(path), 'sha256': sha256_bytes(data)})
    analytical_bytes, analytical_path = read_project_bound(
        {'path': case['analytical_reference']['path'], 'sha256': case['analytical_reference']['sha256']}, project,
        exact_path=ANALYTICAL_REFERENCE)
    bindings.append({'role': 'analytical_reference', 'path': str(analytical_path), 'sha256': sha256_bytes(analytical_bytes)})
    analytical = gate.strict_json(analytical_bytes)
    require(type(analytical) is dict and type(analytical.get('cases')) is dict
            and case['analytical_reference']['case_key'] in analytical['cases'], 'analytical case key not in reference file')
    request, product, reference, selectors, criterion = (gate.strict_json(files[r]) for r in
        ('runner_input', 'product_request', 'reference', 'selectors', 'criteria'))
    require(type(request) is dict and type(request.get('request')) is dict and type(request.get('solve')) is dict
            and set(request['solve']) == {'preview_model'}, 'input is not a production solve request')
    require(request['request'].get('operation') == 'solve', 'input operation not solve')
    _text(request['request'].get('request_id'), 'actual request ID')
    require(request['solve']['preview_model'] == product, 'runner input preview model differs from bound product request')
    model = product['model']
    require(model.get('schema_version') == '0.4.0' and model.get('pressure_contract') == PRESSURE_CONTRACT,
            'actual 0.4.0 resolved load-state model required')
    _text(model['project']['id'], 'actual model ID')
    request_cases = load_case_ids(request)
    require(type(reference) is dict and reference.get('format') == REFERENCE_FORMAT and reference.get('case_id') == case['case_id'],
            'reference identity/version mismatch')
    require(reference.get('readiness') == 'ready' and bool(reference.get('basis')), 'reference not ready or basis absent')
    kind = reference.get('reference_kind')
    require(kind in ('harness_synthetic', 'analytical', 'regression', 'published_numerical'), 'unknown reference kind')
    require(purpose == 'harness_development' or kind != 'harness_synthetic', 'synthetic target cannot score a development comparison')
    if purpose != 'harness_development':
        require(bool(reference.get('independent_review_ref')), 'independent reference admission missing')
    require(type(criterion) is dict and criterion.get('schema_version') == '0.1.0', 'unsupported tolerance document version')
    profile = criterion.get('tolerance_profile')
    require(type(profile) is dict and profile.get('profile_status') == 'reviewed', 'criterion profile not reviewed')
    rules = profile.get('rules')
    require(type(rules) is list and rules and all(type(x) is dict for x in rules), 'criterion rules missing')
    rule_ids = [rule.get('rule_id') for rule in rules]
    require(all(type(i) is str and i for i in rule_ids) and len(rule_ids) == len(set(rule_ids)), 'duplicate/missing criterion IDs')
    positive, negative = admit_selectors(selectors, case)
    expected = {}
    for key, rows in (('values', positive), ('wrong_values', negative)):
        values = reference.get(key, [] if key == 'wrong_values' else None)
        require(type(values) is list and all(type(x) is dict for x in values), 'reference ' + key + ' missing')
        indexed = {value.get('assertion_id'): value for value in values}
        require(len(indexed) == len(values) and set(indexed) == {row['id'] for row in rows}, 'reference ' + key + ' denominator mismatch')
        expected.update(indexed)
    selected = []
    for polarity, rows in (('positive', positive), ('negative', negative)):
        for assertion in rows:
            selector = assertion['selector']
            family = evidence_semantics(selector) if 'namespace' in selector else row_semantics(selector, semantic_rows)
            require(basis_of(selector) in request_cases, 'selector basis is not a load case of the bound input')
            rule = _rule(rules, assertion['criterion_rule_id'], selector['dimension'], family, selector['unit'])
            value = expected[assertion['id']]
            require(value.get('unit') == selector['unit'], 'reference unit mismatch')
            selected.append({'assertion': assertion, 'polarity': polarity, 'expected': gate.finite(value.get('value')), 'rule': rule})
    # The quantity identity (row ID, or evidence case/record/key/field) with the
    # wrong value; -0.0 and 0.0 are the same wrong value.
    pairs = [(_selector_identity(item['assertion']['selector']), item['expected'])
             for item in selected if item['polarity'] == 'negative']
    require(len(set(pairs)) == len(pairs), 'duplicate negative (selector, wrong value) pair')
    return {'input': files['runner_input'], 'reference_bytes': files['reference'], 'criterion_bytes': files['criteria'],
            'selector_bytes': files['selectors'], 'request': request, 'request_cases': request_cases, 'selected': selected,
            'profile': profile, 'bindings': bindings, 'reference_kind': kind}


# --------------------------------------------------------------------------
# Transport: wrapper, raw header, standing and identity binding
# --------------------------------------------------------------------------

def unwrap(raw: bytes, request: dict, mode: str, *, limit: int = LIMIT) -> tuple[dict, list[dict]]:
    """Controlled wrapper and exact load-reference-1 raw0.2 envelope.

    Refuses anything but the closed identity. Numerical standing is only shape
    checked here; its value is assessed separately by ``numerical_standing``.
    """
    require(mode in MODES, 'unsupported actual mode')
    doc = strict_json_limited(raw, limit)
    require(type(doc) is dict and set(doc) == {'payload', 'decisions', 'findings', 'blocked', 'summary'}, 'unsupported controlled wrapper')
    require(doc['blocked'] is False and type(doc['payload']) is dict, 'controlled output blocked')
    gate._diagnostics(doc['findings'], 'wrapper')
    summary = doc['summary']
    require(type(summary) is dict and type(summary.get('blocking_count')) is int and summary['blocking_count'] == 0, 'wrapper blocking summary missing or inconsistent')
    require(type(doc['decisions']) is list and type(summary.get('decision_count')) is int and summary['decision_count'] == len(doc['decisions']), 'wrapper decision count inconsistent')
    require(type(summary.get('finding_count')) is int and summary['finding_count'] == len(doc['findings']), 'wrapper finding count inconsistent')
    require(type(summary.get('warning_count')) is int and summary['warning_count'] == sum(d['severity'].lower() == 'warning' for d in doc['findings']), 'wrapper warning count inconsistent')
    payload = doc['payload']
    require(payload.get('artifact') == 'openpipestress.headless_runner_cli_output' and payload.get('schema_version') == '1.0.0', 'unsupported CLI contract')
    require(payload.get('command') == 'solve' and payload.get('operation') == 'solve', 'wrong CLI operation')
    for key in ('request_validation', 'result_validation'):
        require(type(payload.get(key)) is dict, key + ' missing')
        gate._diagnostics(payload[key].get('diagnostics'), key)
    gate._diagnostics(payload.get('diagnostics'), 'CLI')
    runner = payload.get('runner_result')
    require(type(runner) is dict and runner.get('run_id') == 'run:headless-preview:' + request['request']['request_id'], 'runner request identity mismatch')
    require(type(runner.get('job')) is dict and runner['job'].get('state') == 'COMPLETED', 'runner not completed')
    require('MECHANICS_SOLVED' in runner.get('analysis_status', []), 'runner mechanics not solved')
    gate._diagnostics(runner.get('diagnostics'), 'runner')
    mechanics = payload.get('mechanics_envelope')
    require(type(mechanics) is dict and mechanics.get('document_kind') == 'openpipestress.product_preview.mechanics_result', 'unsupported raw mechanics document')
    require(mechanics.get('schema_version') == '0.2.0' and mechanics.get('producer') == PRODUCER, 'load-reference-1 raw/producer identity required')
    require('source_block_recovery' not in mechanics and 'carrier_evidence' not in mechanics, 'source/composite namespace refused')
    formulation = mechanics.get('formulation_basis')
    require(type(formulation) is dict and set(formulation) == {'profile_id', 'limitations'} and formulation['profile_id'] == PROFILE
            and type(formulation['limitations']) is list and formulation['limitations']
            and all(type(x) is str and x for x in formulation['limitations']), 'formulation basis invalid')
    model = request['solve']['preview_model']['model']
    require(model.get('schema_version') == '0.4.0' and model.get('pressure_contract') == PRESSURE_CONTRACT, 'actual 0.4.0 resolved load-state model required')
    require(mechanics.get('run_id') == 'run:preview-linear-static-001', 'raw run identity mismatch for current profile')
    require(mechanics.get('model_ref') == model['project']['id'], 'actual model identity mismatch')
    require(mechanics.get('accepted_model_state_mutated') is False, 'solver reports model mutation')
    require(type(mechanics.get('status')) is dict and mechanics['status'].get('mechanics') == 'MECHANICS_SOLVED', 'mechanics not solved')
    gate._diagnostics(mechanics.get('diagnostics'), 'mechanics')
    quality = mechanics.get('numerical_quality')
    require(type(quality) is dict and set(quality) == {'value_representation', 'publication_quantization', 'integrity_policy', 'status', 'cases'},
            'numerical quality shape invalid')
    require(quality['value_representation'] == 'finite_binary64' and quality['publication_quantization'] == 'none'
            and quality['integrity_policy'] == 'M03-INTEGRITY-v1', 'numerical publication basis differs')
    require(type(quality['cases']) is list and all(type(c) is dict and set(c) == {'basis_ref', 'structural_status', 'solve_quality',
            'model_matrix_fidelity', 'accuracy_evidence', 'evidence_refs'} for c in quality['cases']), 'numerical standing missing or malformed')
    evidence = mechanics.get('contract_evidence')
    require(type(evidence) is dict and set(evidence) == {'pressure', 'connector', 'exact_cases', 'load_reference_states'},
            'load-reference evidence namespace required')
    rows = mechanics.get('results')
    require(type(rows) is list and rows and all(type(row) is dict for row in rows), 'result rows missing')
    ids = [row.get('id') for row in rows]
    require(all(type(i) is str and i for i in ids) and len(ids) == len(set(ids)), 'missing/duplicate result ID')
    for row in rows:
        require(set(row) <= {'id', 'kind', 'value', 'unit', 'entity_ref', 'basis_ref', 'metadata', 'source_result_refs'}, 'unknown raw row field for closed profile')
        for name in ('id', 'kind', 'unit', 'entity_ref'):
            _text(row.get(name), 'raw row ' + name)
        if 'basis_ref' in row:
            require(type(row['basis_ref']) is dict and set(row['basis_ref']) == {'ref_type', 'ref_id'}, 'invalid raw basis shape')
            require(row['basis_ref']['ref_type'] in ('load_case', 'combination'), 'unknown raw basis type')
            _text(row['basis_ref']['ref_id'], 'raw basis ID')
        if 'metadata' in row:
            require(type(row['metadata']) is dict and set(row['metadata']) == set(gate.META), 'invalid raw metadata shape')
            require(all(type(v) is str for v in row['metadata'].values()), 'invalid raw metadata value')
        gate.finite(row.get('value'))
    mode_rows = [row for row in rows if row.get('kind') == 'linear_solver_mode_basis']
    require(bool(mode_rows) and all(gate.finite(row.get('value')) == MODE_CODES[mode] and row.get('unit') == 'mode_code' for row in mode_rows),
            'actual mode evidence mismatch or fallback')
    mode_cases = sorted(str(row.get('basis_ref', {}).get('ref_id')) for row in mode_rows)
    require(mode_cases == sorted(load_case_ids(request)), 'one mode row per actual load case required')
    return mechanics, rows


def numerical_standing(mechanics: dict, request_cases: list[str]) -> dict:
    """Observed standing, reported separately. Missing/unknown values raise."""
    quality = mechanics['numerical_quality']
    require(quality['status'] in STANDING_VALUES, 'unknown numerical standing')
    diagnostics = {row.get('id'): row for row in mechanics['diagnostics']}
    require(len(diagnostics) == len(mechanics['diagnostics']), 'duplicate diagnostic identity')
    cases = []
    for case in quality['cases']:
        basis = case['basis_ref']
        require(type(basis) is dict and set(basis) == {'ref_type', 'ref_id'} and basis['ref_type'] == 'load_case', 'numerical case basis invalid')
        require(case['solve_quality'] in STANDING_VALUES, 'unknown case numerical standing')
        refs = case['evidence_refs']
        require(type(refs) is list and refs and len(set(refs)) == len(refs)
                and all(ref in diagnostics and basis['ref_id'] in diagnostics[ref].get('affected_refs', []) for ref in refs),
                'numerical standing evidence unavailable or unbound')
        cases.append({'load_case_id': basis['ref_id'], **{k: case[k] for k in ('solve_quality', 'structural_status', 'model_matrix_fidelity', 'accuracy_evidence')},
                      'evidence_codes': [diagnostics[ref].get('code') for ref in refs]})
    ids = [row['load_case_id'] for row in cases]
    require(len(ids) == len(set(ids)) and set(ids) == set(request_cases), 'numerical standing does not cover every actual load case')
    passed = quality['status'] == 'checks_passed' and all(
        row['solve_quality'] == 'checks_passed' and row['structural_status'] == 'passive_model_basis'
        and row['model_matrix_fidelity'] == 'represented_equations_retained' and row['accuracy_evidence'] == 'not_claimed'
        and row['evidence_codes'] and all(code == 'NUMERICAL_INTEGRITY_CHECKS_PASSED' for code in row['evidence_codes'])
        for row in cases)
    observed = {quality['status']} | {row['solve_quality'] for row in cases}
    if passed:
        value = 'checks_passed'
    elif 'sensitive' in observed and observed <= {'checks_passed', 'sensitive'}:
        value = 'sensitive'
    else:
        value = 'insufficient'
    return {'standing': value,
            'envelope_status': quality['status'], 'load_cases': cases,
            'scope': 'producer numerical standing; not accuracy evidence and not a comparison outcome'}


# --------------------------------------------------------------------------
# Selector resolution and comparison
# --------------------------------------------------------------------------

def resolve_row(rows: list[dict], selector: dict) -> float:
    by_id = [row for row in rows if row.get('id') == selector['id']]
    require(len(by_id) == 1, 'required result unavailable')
    matches = [row for row in rows if gate.selector_matches(row, selector)]
    require(len(matches) == 1 and matches[0] is by_id[0], 'required selector does not resolve exactly once to its row')
    return gate.finite(by_id[0].get('value'))


def _one(items, predicate, label):
    found = [item for item in items if type(item) is dict and predicate(item)]
    require(len(found) == 1, label + ' does not resolve exactly once')
    return found[0]


def resolve_evidence(mechanics: dict, selector: dict) -> float:
    records = mechanics['contract_evidence']['load_reference_states']
    require(type(records) is list, 'load_reference_states unavailable')
    case_id = selector['basis_ref']['ref_id']
    record = _one(records, lambda r: r.get('load_case_id') == case_id, 'load reference state record')
    list_name, key_fields, _ = RECORDS[selector['record']]
    items = record.get(list_name)
    require(type(items) is list, 'record list unavailable')
    item = _one(items, lambda i: all(i.get(k) == selector['key'][k] for k in key_fields), selector['record'])
    for name, value in selector['definition'].items():
        require(type(item.get(name)) is type(value) and item.get(name) == value, 'definition differs: ' + name)
    field = selector['field']
    require(field in item, 'evidence field unavailable')
    value = item[field]
    require(value is not None, 'evidence value unavailable (null is not zero)')
    if selector['record'] == 'support_component':
        require(item.get('unit') == selector['unit'], 'support component unit differs')
    if selector['record'] == 'contribution' and field in ('authored_normalized_magnitude', 'applied_magnitude'):
        require(item.get('dimension') == selector['dimension'] and LOAD_UNITS.get(item.get('dimension')) == selector['unit'],
                'applied load dimension/unit differs')
    return gate.finite(value)


def evaluate(case_ledger: dict, prepared: dict, mechanics: dict, rows: list[dict]) -> bool:
    """Compare every required selector; return complete exact-once coverage."""
    required = {row['id']: row for row in case_ledger['assertions']}
    covered = True
    for item in prepared['selected']:
        target = required[item['assertion']['id']]
        selector = item['assertion']['selector']
        try:
            try:
                actual = resolve_evidence(mechanics, selector) if 'namespace' in selector else resolve_row(rows, selector)
            except ValueError:
                covered = False
                raise
            expected, rule = item['expected'], item['rule']
            delta = gate.finite(actual - expected)
            absolute = gate.finite(abs(delta))
            if 'relative_tolerance_value' in rule:
                gate.finite(gate.finite(rule['relative_tolerance_value']) * max(abs(actual), abs(expected)))
            classification, basis = gate._classify_delta(absolute, expected, actual, prepared['profile'], rule, selector['unit'])
            require(classification in ('within_tolerance_profile', 'exceeds_tolerance_profile'), 'required comparison unclassified')
            within = classification == 'within_tolerance_profile'
            if item['polarity'] == 'positive':
                state, reason = ('matched' if within else 'failed'), classification
            else:
                state = 'failed' if within else 'matched'
                reason = 'wrong_result_reproduced' if within else 'wrong_result_excluded'
            target.update(state=state, reason=reason, observed=actual, expected=expected, absolute_delta=absolute,
                          unit=selector['unit'], predicate_basis=basis, classification=classification)
        except (ValueError, KeyError, TypeError, AttributeError, OverflowError) as exc:
            target.update(state='error', reason=str(exc), observed=None)
    return covered


def structural_checks(mechanics: dict, prepared: dict, mode: str, standing: dict | None, standing_error: str | None,
                      reader: dict | None, reader_error: str | None, coverage: bool) -> list[dict]:
    request = prepared['request']
    model = request['solve']['preview_model']['model']
    output = []

    def assess(check_id, operation, failure_state='failed'):
        details = {}
        try:
            operation(details)
            state, reason = 'matched', 'required load-reference predicate satisfied'
        except (ValueError, KeyError, TypeError, AttributeError, OverflowError, IndexError) as exc:
            state, reason = failure_state, str(exc)
        output.append({'id': check_id, 'state': state, 'reason': reason, 'details': details})

    def transport(details):
        details.update(mode=mode, mode_code=MODE_CODES[mode])

    def input_binding(details):
        evidence = mechanics['contract_evidence']
        exact = [case.get('load_case_id') for case in evidence['exact_cases']]
        records = [record.get('load_case_id') for record in evidence['load_reference_states']]
        wanted = sorted(prepared['request_cases'])
        require(sorted(exact) == wanted and sorted(records) == wanted, 'raw load cases differ from the bound input (substitution)')
        foreign = sorted({row['basis_ref']['ref_id'] for row in mechanics['results'] if 'basis_ref' in row} - set(wanted))
        require(not foreign, 'raw rows carry load cases outside the bound input')
        details.update(model_ref=mechanics['model_ref'], request_id=request['request']['request_id'], load_cases=wanted)

    def contract(details):
        require(sha256_bytes((PROJECT / TABLE).read_bytes()) == TABLE_SHA256, 'pinned load-reference-1 table changed')
        require(mechanics['producer'] == PRODUCER and mechanics['formulation_basis']['profile_id'] == PROFILE, 'contract/profile differs')
        require(mechanics['contract_evidence']['connector'] == [], 'connector evidence unsupported')
        details.update(semantic_contract_id=CONTRACT, profile_id=PROFILE, table_sha256=TABLE_SHA256)

    def standing_check(details):
        require(standing is not None, standing_error or 'numerical standing unavailable')
        details.update(standing)
        require(standing['standing'] == 'checks_passed', 'insufficient numerical standing: ' + standing['standing'])

    def records(details):
        by_case = {}
        for record in mechanics['contract_evidence']['load_reference_states']:
            require(type(record) is dict and record.get('load_case_id') not in by_case, 'duplicate load reference state record')
            by_case[record.get('load_case_id')] = record
        for case in model['load_cases']:
            record = by_case.get(case['id'])
            require(record is not None, 'load reference state unavailable: ' + case['id'])
            state = case.get('analysis_state')
            require(type(state) is dict, 'resolved load state absent from bound input: ' + case['id'])
            require(record.get('contract') == 'openpipestress.load_reference_state/1.0.0' and record.get('profile') == PROFILE,
                    'record contract/profile differs')
            require(record.get('reference_configuration_id') == state.get('reference_configuration_ref'), 'reference configuration differs from input')
            solve = record.get('solve')
            require(type(solve) is dict and solve.get('requested_mode') == mode and solve.get('recovery_method') == RECOVERY_METHODS[mode],
                    'record mode/recovery method differs')
            require(record.get('source_recovery') == NOT_JOINED, 'joined source recovery is not load-reference-1')
        details['records'] = sorted(by_case)

    def owned_reader(details):
        require(reader is not None, 'owned reader observation unavailable: ' + str(reader_error))
        details.update(verdict=reader['response'].get('verdict'), reason=reader['response'].get('reason'), helper_sha256=reader['helper_sha256'],
                       binding_sha256=reader['binding_sha256'], scope=reader['scope'])
        require(reader['response'].get('verdict') == 'consistent', 'owned reader refused the envelope')

    def selector_coverage(details):
        details['required_selectors'] = len(prepared['selected'])
        require(coverage, 'required selectors do not all resolve exactly once')

    for check_id, operation, failure in [
            ('transport_identity_and_mode', transport, 'failed'), ('actual_input_binding', input_binding, 'failed'),
            ('load_reference_contract', contract, 'failed'),
            ('numerical_standing', standing_check, 'failed' if standing is not None else 'error'),
            ('load_reference_record_binding', records, 'failed'),
            ('owned_reader_consistency', owned_reader, 'failed' if reader is not None else 'error'),
            ('complete_unique_selector_coverage', selector_coverage, 'failed')]:
        assess(check_id, operation, failure)
    return output


# --------------------------------------------------------------------------
# Owned reader binding and isolated observation
# --------------------------------------------------------------------------

def bounded(path: Path, expected: str, limit: int = LIMIT) -> bytes:
    with path.open('rb') as stream:
        data = stream.read(limit + 1)
    require(len(data) <= limit and sha256_bytes(data) == expected, 'bound bytes differ or exceed limit: ' + str(path))
    return data


def load_binding(project: Path, binding_path: Path, expected_sha256: str, *, require_reviewed: bool = True) -> dict:
    data = bounded(binding_path, expected_sha256)
    binding = gate.strict_json(data)
    require(type(binding) is dict and binding.get('format') == BINDING_FORMAT and binding.get('contract_id') == CONTRACT,
            'unsupported reader binding identity')
    require(binding.get('entrypoint') == ENTRYPOINT, 'unsupported owned entry')
    if require_reviewed:
        require(binding.get('status') == 'reviewed_candidate' and bool(binding.get('review_basis')), 'reviewed candidate/module binding not admitted')
    files = binding.get('files')
    require(type(files) is list and len(files) == len(DEPENDENCIES), 'closed dependency inventory required')
    paths = [row.get('path') if type(row) is dict else None for row in files]
    require(set(paths) == set(DEPENDENCIES) and len(set(paths)) == len(paths), 'dependency inventory mismatch')
    content = {}
    for row in files:
        require(set(row) == {'path', 'sha256'}, 'unknown dependency fields')
        require(row['sha256'] == DEPENDENCIES[row['path']], 'unselected owned reader dependency: ' + row['path'])
        path = (project / row['path']).resolve()
        require(path.is_relative_to(project), 'dependency leaves selected candidate root')
        content[row['path']] = bounded(path, row['sha256'])
    review_bytes = []
    if require_reviewed:
        for review in binding['review_basis']:
            require(type(review) is dict and set(review) == {'path', 'sha256'}, 'review binding shape invalid')
            path = (project / review['path']).resolve()
            require(path.is_relative_to(project), 'review origin leaves selected candidate root')
            review_bytes.append({'path': str(path), 'sha256': review['sha256'], 'bytes': bounded(path, review['sha256'])})
    table = gate.strict_json(content[TABLE])
    require(table.get('semantic_contract_id') == CONTRACT and table.get('formulation_profile_id') == PROFILE, 'semantic identity mismatch')
    return {'source_root': project, 'binding_path': binding_path.resolve(), 'binding_sha256': expected_sha256, 'binding': binding,
            'semantic_table': table, 'binding_bytes': data, 'dependency_bytes': content, 'review_bytes': review_bytes,
            'admitted': require_reviewed and binding.get('status') == 'reviewed_candidate'}


def consistency_observation(raw: dict, binding: dict, directory: Path, *, input_limit: int = LIMIT,
                            timeout_seconds: float = 60) -> dict:
    """Run only the owned reader entry in the fixed isolated helper.

    The helper input is a derived parsed snapshot, not the runner's stdout bytes.
    """
    require(raw.get('schema_version') == '0.2.0' and raw.get('producer') == PRODUCER, 'load-reference raw/producer dispatch mismatch')
    request_bytes = json.dumps(raw, ensure_ascii=True, separators=(',', ':'), allow_nan=False).encode()
    require(type(input_limit) is int and 0 < input_limit <= MAX_OUTPUT_LIMIT and len(request_bytes) <= input_limit, 'reader input limit')
    require(bounded(binding['binding_path'], binding['binding_sha256']) == binding['binding_bytes'], 'binding changed before helper')
    helper_sha256 = file_sha256(HELPER)
    arguments = ['-I', '-S', str(HELPER), '--source-root', str(binding.get('snapshot_root', binding['source_root'])),
                 '--binding', str(binding.get('snapshot_binding_path', binding['binding_path'])), '--binding-sha256', binding['binding_sha256'],
                 '--source-limit-bytes', str(input_limit)]
    executable = Path(sys.executable).resolve()
    process = capture(executable, arguments, request_bytes, directory, timeout_seconds=max(60, timeout_seconds), output_limit_bytes=LIMIT,
                      expected_executable_sha256=file_sha256(executable))
    require(file_sha256(HELPER) == helper_sha256, 'fixed helper changed during execution')
    response_bytes = gate.read_captured_bytes(directory / 'stdout.bin', process['stdout_sha256'], process['stdout_bytes'], LIMIT)
    response = gate.strict_json(response_bytes) if response_bytes else {}
    if process['outcome'] == 'nonzero_exit' and process['return_code'] == 2 and response.get('verdict') == 'refused':
        pass  # a genuine owned-reader refusal: reported as a failed obligation, not an error
    else:
        require(process['outcome'] == 'completed' and process['stdin_delivery_complete'], 'reader helper failed: ' + process['outcome'])
        require(response.get('verdict') == 'consistent', 'unknown helper response')
    require(response.get('artifact') == 'openpipestress.load_reference_consistency_observation' and response.get('version') == '1.0.0',
            'unknown helper response')
    if response['verdict'] == 'consistent':
        require(response.get('binding_sha256') == binding['binding_sha256'] and response.get('source_bytes_sha256') == sha256_bytes(request_bytes)
                and response.get('dependencies') == binding['binding']['files'] and response.get('contract_id') == CONTRACT,
                'helper response identity mismatch')
    for row in binding['binding']['files']:
        bounded(binding['source_root'] / row['path'], row['sha256'])
    return {'process': process, 'response': response, 'helper_sha256': helper_sha256, 'binding_sha256': binding['binding_sha256'],
            'binding_admitted': binding['admitted'],
            'scope': 'internal source consistency of parsed snapshot only; runner provenance and numerical comparison are separate'}


# --------------------------------------------------------------------------
# Run
# --------------------------------------------------------------------------

def admit_run(run: dict) -> None:
    require(type(run) is dict and set(run) == RUN_KEYS, 'unknown/missing run selection fields')
    require(run['format'] == RUN_FORMAT, 'unsupported run selection version')
    require(run['transport'] == TRANSPORT, 'unsupported transport version')
    require(run['purpose'] in ('harness_development', 'development_comparison'), 'unsupported run purpose; release qualification not implemented')
    _text(run['profile_id'], 'profile ID')
    runner = run['runner']
    require(type(runner) is dict and set(runner) == RUNNER_KEYS, 'unknown/missing runner binding')
    require(runner['solver_mode'] in MODES, 'mode not available through selected CLI')
    require(type(runner['explicit_local_private_intent']) is bool, 'explicit local private intent must be boolean')
    require(type(runner['candidate_commit']) is str and bool(re.fullmatch('[0-9a-f]{40}', runner['candidate_commit'])), 'exact candidate commit required')
    require(type(runner['executable_sha256']) is str and bool(re.fullmatch('[0-9a-f]{64}', runner['executable_sha256'])), 'exact executable digest required')
    for key in ('case_manifest', 'reader_binding'):
        require(type(run[key]) is dict and set(run[key]) == {'path', 'sha256'}, key + ' binding must be {path, sha256}')


def write_summary(path: Path, ledger: dict) -> None:
    summary = ledger['summary']
    counts = summary['assertions']
    lines = ['# Load-reference-1 qualification development observation', '',
             f"Outcome: {ledger['outcome']} (transport {TRANSPORT}, mode {ledger['solver_mode']})",
             f"Required: {summary['required_cases']} cases / {summary['required_assertions']} assertions "
             f"({sum(summary['assertions_by_polarity']['positive'].values())} positive, "
             f"{sum(summary['assertions_by_polarity']['negative'].values())} negative) / "
             f"{summary['required_structural_checks']} structural checks.",
             'Assertions: ' + ' | '.join(f'{state}: {counts[state]}' for state in STATES),
             'Structural: ' + ' | '.join(f"{state}: {summary['structural_checks'][state]}" for state in STATES),
             'Numerical standing (reported separately from outcomes): ' +
             ', '.join(f'{key}: {value}' for key, value in summary['numerical_standing'].items()), '',
             'This report does not establish app Current, independent oracle admission, physical validation, professional acceptance or release.', '',
             '| Case | Outcome | Standing | Assertions matched/required | Structural matched/required |', '|---|---|---|---|---|']
    for case in ledger['cases']:
        label = str(case['id']).replace('|', '\\|').replace('\n', ' ')
        lines.append(f"| {label} | {case['state']} | {case['numerical_standing'].get('standing')} | "
                     f"{sum(r['state'] == 'matched' for r in case['assertions'])}/{len(case['assertions'])} | "
                     f"{sum(r['state'] == 'matched' for r in case['structural_checks'])}/{len(case['structural_checks'])} |")
    path.write_text('\n'.join(lines) + '\n')


def publish(ledger, output_dir, global_artifacts, case_artifacts, original_bindings, global_originals):
    gate.verify_publication_custody(ledger, output_dir, global_artifacts, case_artifacts, original_bindings, global_originals)
    summarize(ledger)
    gate.atomic_record(output_dir / 'ledger.json', ledger)
    write_summary(output_dir / 'summary.md', ledger)
    return ledger


def run_selection(run_path: Path, executable: Path, source_root: Path, output_dir: Path,
                  timeout_seconds: float = 30, output_limit_bytes: int = 8 * 1024 * 1024) -> dict:
    with run_path.open('rb') as stream:
        raw_run = stream.read(LIMIT + 1)
    run = gate.strict_json(raw_run)
    admit_run(run)
    mode = run['runner']['solver_mode']
    project = project_root(source_root)
    manifest_bytes, manifest_path = gate.read_bound(run['case_manifest'], run_path.parent)
    manifest = admit_manifest(manifest_bytes)
    inventories = {case['case_id']: case_inventory(case, project) for case in manifest['cases']}
    ledger = predeclare(manifest, mode, inventories)
    ledger.update(profile_id=run['profile_id'], selection_sha256=sha256_bytes(raw_run), case_manifest_sha256=sha256_bytes(manifest_bytes))
    output_dir.mkdir(parents=True, exist_ok=False)
    (output_dir / 'selection.json').write_bytes(raw_run)
    (output_dir / 'case-manifest.json').write_bytes(manifest_bytes)
    global_artifacts = [gate.artifact_binding('selection', 'selection.json', raw_run),
                        gate.artifact_binding('case_manifest', 'case-manifest.json', manifest_bytes)]
    global_originals = [{'path': str(manifest_path), 'sha256': sha256_bytes(manifest_bytes)}]
    case_artifacts, original_bindings = {}, {}
    for case in ledger['cases']:
        refusal = inventories[case['id']]['refusal']
        if refusal:
            fail_case(case, 'blocked', refusal)
    summarize(ledger)
    gate.atomic_record(output_dir / 'ledger.json', ledger)
    try:
        require(math.isfinite(timeout_seconds) and 0 < timeout_seconds <= 3600, 'invalid process time limit')
        require(type(output_limit_bytes) is int and 0 < output_limit_bytes <= MAX_OUTPUT_LIMIT, 'invalid process output limit')
        runner = run['runner']
        require(executable.is_absolute() and executable.is_file(), 'explicit executable required')
        require(file_sha256(executable) == runner['executable_sha256'], 'executable digest mismatch')
        actual_commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=source_root, text=True).strip()
        require(actual_commit == runner['candidate_commit'], 'candidate revision mismatch')
        status = subprocess.check_output(['git', 'status', '--porcelain=v1'], cwd=source_root, text=True)
        require(sha256_bytes((PROJECT / TABLE).read_bytes()) == TABLE_SHA256, 'pinned load-reference-1 table changed')
        require(sha256_bytes(gate.CLASSIFIER.read_bytes()) == gate.CLASSIFIER_SHA256, 'existing comparison predicate changed')
        binding_bytes, binding_path = gate.read_bound(run['reader_binding'], run_path.parent)
        reader_binding = load_binding(project, binding_path, sha256_bytes(binding_bytes))
        snapshot_root = output_dir / 'reader-basis'
        snapshot_root.mkdir()
        for name, data in reader_binding['dependency_bytes'].items():
            copied = snapshot_root / name
            copied.parent.mkdir(parents=True, exist_ok=True)
            copied.write_bytes(data)
            global_artifacts.append(gate.artifact_binding('reader_dependency', str(copied.relative_to(output_dir)), data))
            global_originals.append({'path': str(project / name), 'sha256': sha256_bytes(data)})
        snapshot_binding = snapshot_root / 'binding.json'
        snapshot_binding.write_bytes(binding_bytes)
        global_artifacts.append(gate.artifact_binding('reader_binding', str(snapshot_binding.relative_to(output_dir)), binding_bytes))
        global_originals.append({'path': str(binding_path), 'sha256': sha256_bytes(binding_bytes)})
        reader_binding.update(snapshot_root=snapshot_root, snapshot_binding_path=snapshot_binding)
        for index, review in enumerate(reader_binding['review_bytes']):
            name = f'reader-basis/review-{index:02}.bin'
            (output_dir / name).write_bytes(review['bytes'])
            global_artifacts.append(gate.artifact_binding('reader_review', name, review['bytes']))
            global_originals.append({'path': review['path'], 'sha256': review['sha256']})
        ledger['global_artifacts'] = global_artifacts
        semantics = reader_binding['semantic_table']['rows']
        ledger['execution_basis'] = {
            'source_commit': actual_commit, 'dirty_tree': bool(status), 'source_status': status,
            'executable_sha256': runner['executable_sha256'], 'solver_mode_dispatch': mode + ' explicit CLI flag',
            'semantic_contract_id': CONTRACT, 'semantic_contract_sha256': TABLE_SHA256, 'profile_id': PROFILE,
            'reader_module_sha256': MODULE_SHA256, 'reader_binding_sha256': reader_binding['binding_sha256'],
            'reader_binding_admitted': reader_binding['admitted'], 'units_sha256': DEPENDENCIES[UNITS],
            'existing_predicate_sha256': gate.CLASSIFIER_SHA256, 'harness_sha256': file_sha256(Path(__file__)),
            'gate_sha256': file_sha256(Path(gate.__file__)), 'helper_sha256': file_sha256(HELPER),
            'process_supervisor_sha256': file_sha256(Path(__file__).with_name('qualification_process.py')),
            'python': sys.version, 'platform': platform.platform(),
            'source_artifact_association': 'selected checkout and executable identities; not independent build-source attestation',
            'producer_precision': 'load-reference-1 raw0.2 finite binary64 without publication quantization'}
    except (ValueError, KeyError, TypeError, AttributeError, OSError, subprocess.SubprocessError) as exc:
        ledger['diagnostics'].append(str(exc))
        for case in ledger['cases']:
            fail_case(case, 'blocked', str(exc))
        return publish(ledger, output_dir, global_artifacts, case_artifacts, original_bindings, global_originals)
    manifest_cases = {case['case_id']: case for case in manifest['cases']}
    for index, case_ledger in enumerate(ledger['cases']):
        if inventories[case_ledger['id']]['refusal']:
            continue
        case = manifest_cases[case_ledger['id']]
        prefix = f'case-{index:04}'
        try:
            prepared = prepare_case(case, project, run['purpose'], semantics)
            case_ledger['bindings'] = prepared['bindings']
            original_bindings[case_ledger['id']] = prepared['bindings']
            case_ledger['reference_kind'] = prepared['reference_kind']
            case_artifacts[case_ledger['id']] = []
            for role, key in (('reference', 'reference_bytes'), ('criterion', 'criterion_bytes'), ('selectors', 'selector_bytes')):
                name = f'{prefix}.{role}.json'
                (output_dir / name).write_bytes(prepared[key])
                case_artifacts[case_ledger['id']].append(gate.artifact_binding(role, name, prepared[key]))
            case_ledger['retained_artifacts'] = case_artifacts[case_ledger['id']]
            case_ledger['reason'] = 'admitted, process pending'
            gate.atomic_record(output_dir / 'ledger.json', ledger)
            arguments = ['solve', '--input', '-', '--solver-mode', mode]
            if runner['explicit_local_private_intent']:
                arguments.append('--explicit-local-private-intent')
            process = capture(executable, arguments, prepared['input'], output_dir / prefix, timeout_seconds, output_limit_bytes,
                              expected_executable_sha256=runner['executable_sha256'])
            case_ledger['process'] = process
            case_artifacts[case_ledger['id']].append(gate.artifact_binding('input', f'{prefix}/stdin.bin', prepared['input']))
            for stream_name in ('stdout', 'stderr'):
                case_artifacts[case_ledger['id']].append({'role': stream_name, 'path': f'{prefix}/{stream_name}.bin',
                    'sha256': process[stream_name + '_sha256'], 'byte_length': process[stream_name + '_bytes'],
                    'byte_limit': process['output_limit_bytes_per_stream']})
            require(process['executable_sha256_before'] == runner['executable_sha256'], 'executable changed before case')
            if process['outcome'] != 'completed' or not process['stdin_delivery_complete']:
                fail_case(case_ledger, 'error', 'process ' + process['outcome'])
            else:
                # Runner stdout is admitted up to the run's selected output limit,
                # not the 8 MiB bound-input limit (which stays on bound files).
                stdout_bytes = gate.read_captured_bytes(output_dir / prefix / 'stdout.bin', process['stdout_sha256'],
                                                        process['stdout_bytes'], output_limit_bytes)
                mechanics, rows = unwrap(stdout_bytes, prepared['request'], mode, limit=output_limit_bytes)
                case_ledger['observed_result_identity'] = {k: mechanics[k] for k in ('document_kind', 'schema_version', 'run_id', 'model_ref')}
                try:
                    standing, standing_error = numerical_standing(mechanics, prepared['request_cases']), None
                except (ValueError, KeyError, TypeError) as exc:
                    standing, standing_error = None, str(exc)
                case_ledger['numerical_standing'] = standing or {'standing': 'unavailable', 'reason': standing_error}
                coverage = evaluate(case_ledger, prepared, mechanics, rows)
                reader, reader_error = None, None
                try:
                    reader = consistency_observation(mechanics, reader_binding, output_dir / f'{prefix}-reader',
                                                     input_limit=output_limit_bytes, timeout_seconds=timeout_seconds)
                except (ValueError, KeyError, TypeError, OSError) as exc:
                    reader_error = str(exc)
                finally:
                    helper_dir = output_dir / f'{prefix}-reader'
                    if reader is not None:
                        helper_process = reader['process']
                        for stream_name in ('stdout', 'stderr'):
                            case_artifacts[case_ledger['id']].append({'role': 'reader_' + stream_name, 'path': f'{prefix}-reader/{stream_name}.bin',
                                'sha256': helper_process[stream_name + '_sha256'], 'byte_length': helper_process[stream_name + '_bytes'],
                                'byte_limit': helper_process['output_limit_bytes_per_stream']})
                        helper_bytes = gate.read_captured_bytes(helper_dir / 'stdin.bin', helper_process['stdin_sha256'],
                                                                helper_process['stdin_bytes_delivered'], output_limit_bytes)
                        case_artifacts[case_ledger['id']].append(gate.artifact_binding('reader_input', f'{prefix}-reader/stdin.bin', helper_bytes,
                                                                                       output_limit_bytes))
                        case_ledger['reader_consistency'] = {k: reader[k] for k in ('response', 'helper_sha256', 'binding_sha256', 'binding_admitted', 'scope')}
                checks = structural_checks(mechanics, prepared, mode, standing, standing_error, reader, reader_error, coverage)
                require([row['id'] for row in checks] == list(REQUIRED_STRUCTURAL_CHECKS), 'structural checker lost required denominator')
                case_ledger['structural_checks'] = checks
                states = {row['state'] for row in case_ledger['assertions'] + checks}
                case_ledger.update(state='matched' if states == {'matched'} else 'error' if 'error' in states else 'failed',
                                   reason='required assertion and structural assessment')
            gate.verify_retained_artifacts(output_dir, case_artifacts[case_ledger['id']])
            for binding in prepared['bindings']:
                require(file_sha256(Path(binding['path']), max_bytes=LIMIT) == binding['sha256'], 'bound input/reference/criterion changed during run')
            if process['outcome'] == 'interrupted':
                ledger['diagnostics'].append('supervisor interrupted; subsequent cases remain not_run')
                break
        except (ValueError, KeyError, TypeError, AttributeError, OSError, OverflowError, IndexError) as exc:
            fail_case(case_ledger, 'blocked' if case_ledger['process'] is None else 'error', str(exc))
        except KeyboardInterrupt:
            fail_case(case_ledger, 'error', 'interrupted before or after process capture')
            ledger['diagnostics'].append('interrupted; subsequent cases remain not_run')
            break
        finally:
            summarize(ledger)
            gate.atomic_record(output_dir / 'ledger.json', ledger)
    return publish(ledger, output_dir, global_artifacts, case_artifacts, original_bindings, global_originals)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--selection', type=Path, required=True)
    parser.add_argument('--executable', type=Path, required=True)
    parser.add_argument('--source-root', type=Path, required=True)
    parser.add_argument('--output-dir', type=Path, required=True)
    parser.add_argument('--timeout-seconds', type=float, default=30)
    parser.add_argument('--output-limit-bytes', type=int, default=8 * 1024 * 1024)
    args = parser.parse_args()
    try:
        result = run_selection(args.selection.resolve(), args.executable, args.source_root.resolve(), args.output_dir.resolve(),
                               args.timeout_seconds, args.output_limit_bytes)
    except (gate.AdmissionError, OSError, ValueError, KeyError, TypeError) as exc:
        print(json.dumps({'outcome': 'selection_error', 'reason': str(exc), 'qualification': 'not_established_by_this_harness'}))
        raise SystemExit(2)
    print(json.dumps({'outcome': result['outcome'], 'summary': result['summary'], 'ledger': str(args.output_dir / 'ledger.json')}))
    raise SystemExit(0 if result['outcome'] == 'all_required_assertions_matched' else 1)


if __name__ == '__main__':
    main()
