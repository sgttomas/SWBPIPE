#!/usr/bin/env python3
"""Thin development required-output gate around the existing headless solve.

Records observed assertion outcomes, never app Current, independent reference
approval, physical validation, professional acceptance or release qualification.
The current closed transport is main's sparse CLI with raw mechanics0.1. Future
producer/mode contracts need explicit dispatch, not labels or profile fallback.
"""
from __future__ import annotations
import argparse
from datetime import datetime, timezone
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import re
import subprocess
import sys

PROJECT = Path(__file__).resolve().parents[2]
if str(PROJECT) not in sys.path:
    sys.path.insert(0, str(PROJECT))
from core.comparison.analysis_run.engine import _classify_delta
try:
    from .qualification_process import capture, sha256_bytes, file_sha256
except ImportError:
    from qualification_process import capture, sha256_bytes, file_sha256

TRANSPORT = 'main_sparse_cli_1.0_raw0.1'
MANIFEST_FORMAT = 'openpipestress.qualification_selection/1'
REFERENCE_FORMAT = 'openpipestress.qualification_reference_values/1'
MAX_INPUT_BYTES = 8 * 1024 * 1024
MAX_CASES = 256
MAX_ASSERTIONS = 65536
SEMANTICS = PROJECT / 'fixtures/results/semantic_contract_v0_2.json'
SEMANTICS_SHA256 = '4d6886d19e304db897e5e9f8f0054cbee91ba7795868f9698e2bbe070bde94da'
CLASSIFIER = PROJECT / 'core/comparison/analysis_run/engine.py'
CLASSIFIER_SHA256 = '14aada4049fde08c6f68a99a844f84e73b16fd8b6e4bb2a91cb7e4c8f63b7f2c'
META = ('component', 'coordinate_system', 'location', 'basis', 'sign_convention')
SELECTOR_KEYS = {'id', 'kind', 'unit', 'entity_ref', 'basis_ref', 'metadata', 'dimension'}
STATES = ('not_run', 'matched', 'failed', 'blocked', 'error')


class AdmissionError(ValueError):
    pass


def require(value: bool, reason: str) -> None:
    if not value:
        raise AdmissionError(reason)


def _pairs(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, 'duplicate JSON member')
        result[key] = value
    return result


def strict_json(data: bytes):
    require(len(data) <= MAX_INPUT_BYTES, 'JSON byte limit exceeded')
    def forbidden(value):
        raise AdmissionError('nonfinite JSON constant: ' + value)
    def real_token(token):
        value = float(token)
        require(math.isfinite(value), 'nonfinite numeric token')
        significand = token.lower().split('e')[0]
        require(value != 0.0 or not any(c in '123456789' for c in significand), 'nonzero numeric token underflow')
        return value
    try:
        return json.loads(data.decode('utf-8'), object_pairs_hook=_pairs, parse_constant=forbidden, parse_float=real_token)
    except (UnicodeDecodeError, json.JSONDecodeError, RecursionError) as exc:
        raise AdmissionError('invalid UTF-8/JSON: ' + type(exc).__name__) from exc


def finite(value) -> float:
    require(type(value) in (int, float), 'numeric value required (bool is not a number)')
    try:
        result = float(value)
    except OverflowError as exc:
        raise AdmissionError('numeric overflow') from exc
    require(math.isfinite(result), 'nonfinite numeric value')
    return result


def read_bound(binding: dict, base: Path) -> tuple[bytes, Path]:
    require(type(binding) is dict and set(binding) == {'path', 'sha256'}, 'exact path/hash binding required')
    require(type(binding['path']) is str and binding['path'], 'bound path missing')
    require(type(binding['sha256']) is str and bool(re.fullmatch('[0-9a-f]{64}', binding['sha256'])), 'invalid SHA256 binding')
    path = Path(binding['path'])
    path = path if path.is_absolute() else base / path
    require(path.is_file() and path.stat().st_size <= MAX_INPUT_BYTES, 'bound file missing or oversized')
    with path.open('rb') as stream:
        data = stream.read(MAX_INPUT_BYTES + 1)
    require(len(data) <= MAX_INPUT_BYTES, 'bound file grew beyond byte limit')
    require(sha256_bytes(data) == binding['sha256'], 'bound file digest mismatch: ' + str(path))
    return data, path.resolve()



def read_captured_bytes(path: Path, expected_sha256: str, expected_size: int, limit: int) -> bytes:
    """One bounded read supplies both the integrity check and the consumer bytes.

    A filesystem check is an observation, not a lock or promise against later
    writes. Parsing must use the returned immutable bytes, never reopen the path.
    """
    require(type(expected_size) is int and 0 <= expected_size <= limit, 'captured size outside admission limit')
    with path.open('rb') as stream:
        data = stream.read(expected_size + 1)
    require(len(data) == expected_size, 'retained artifact size changed: ' + str(path))
    require(sha256_bytes(data) == expected_sha256, 'retained artifact digest changed: ' + str(path))
    return data


def artifact_binding(role: str, path: str, data: bytes, limit: int = MAX_INPUT_BYTES) -> dict:
    return {'role': role, 'path': path, 'sha256': sha256_bytes(data), 'byte_length': len(data), 'byte_limit': limit}


def verify_retained_artifacts(output_dir: Path, artifacts: list[dict]) -> None:
    for artifact in artifacts:
        read_captured_bytes(output_dir / artifact['path'], artifact['sha256'],
                            artifact['byte_length'], artifact['byte_limit'])


def verify_publication_custody(ledger: dict, output_dir: Path, selection: dict,
                               case_artifacts: dict[str, list[dict]], original_bindings: dict[str, list[dict]]) -> None:
    """Reobserve all retained cases, including ones a later process could alter.

    The ledger owns these expected identities in memory. This is the final
    observation before publication; it is not post-publication file enforcement.
    """
    global_error = None
    try:
        verify_retained_artifacts(output_dir, [selection])
    except (OSError, ValueError) as exc:
        global_error = str(exc)
        ledger['diagnostics'].append('publication selection custody: ' + global_error)
    for case in ledger['cases']:
        try:
            require(global_error is None, 'publication selection custody failed')
            verify_retained_artifacts(output_dir, case_artifacts.get(case['id'], []))
            for binding in original_bindings.get(case['id'], []):
                require(file_sha256(Path(binding['path']), max_bytes=MAX_INPUT_BYTES) == binding['sha256'],
                        'original input/reference/criterion changed before publication')
            case['custody_at_publication'] = 'checked' if case['id'] in case_artifacts else 'not_captured'
        except (OSError, ValueError) as exc:
            fail_case(case, 'error', 'publication custody: ' + str(exc))
            case['custody_at_publication'] = 'failed'
    ledger['custody_observation'] = {
        'checked_at_utc': datetime.now(timezone.utc).isoformat(),
        'scope': 'bounded retained-selection/input/reference/criterion/stream snapshots and original bindings immediately before final publication',
        'limits': 'observation-time byte integrity; no filesystem lock, post-publication tamper prevention, internal solver-use or build-source attestation'}
    ledger['publication_phase'] = 'complete'


def _id(value, label):
    require(type(value) is str and bool(value.strip()), label + ' missing')
    return value


def predeclare(manifest: dict) -> dict:
    """Every intelligible required row exists before file/process admission."""
    require(type(manifest) is dict, 'selection must be an object')
    cases = manifest.get('cases')
    require(type(cases) is list and 0 < len(cases) <= MAX_CASES, 'nonzero bounded case inventory required')
    ledger, seen, assertion_total = [], set(), 0
    for case in cases:
        require(type(case) is dict, 'case must be an object')
        cid = _id(case.get('id'), 'case ID')
        require(cid not in seen, 'duplicate case ID')
        seen.add(cid)
        assertions = case.get('assertions')
        require(type(assertions) is list and bool(assertions), 'case needs required assertions')
        ids = set()
        rows = []
        for assertion in assertions:
            require(type(assertion) is dict, 'assertion must be an object')
            aid = _id(assertion.get('id'), 'assertion ID')
            require(aid not in ids, 'duplicate assertion ID within case')
            ids.add(aid)
            assertion_total += 1
            require(assertion_total <= MAX_ASSERTIONS, 'assertion inventory limit')
            rows.append({'id': aid, 'state': 'not_run', 'reason': 'not executed', 'observed': None})
        ledger.append({'id': cid, 'state': 'not_run', 'reason': 'not executed', 'assertions': rows, 'process': None})
    return {'artifact': 'openpipestress.qualification_development_ledger', 'version': '1.0.0',
            'profile_id': manifest.get('profile_id'), 'selection_sha256': None,
            'qualification': 'not_established_by_this_harness', 'outcome': 'not_run', 'publication_phase': 'in_progress',
            'cases': ledger, 'summary': {}, 'diagnostics': [],
            'created_at_utc': datetime.now(timezone.utc).isoformat()}


def summarize(ledger: dict) -> None:
    counts = {state: 0 for state in STATES}
    for case in ledger['cases']:
        for row in case['assertions']:
            counts[row['state']] += 1
    total = sum(counts.values())
    ledger['summary'] = {'required_cases': len(ledger['cases']), 'required_assertions': total,
                         'assertions': counts, 'cases': {state: sum(c['state'] == state for c in ledger['cases']) for state in STATES}}
    if total and counts['matched'] == total and all(c['state'] == 'matched' for c in ledger['cases']):
        ledger['outcome'] = 'all_required_assertions_matched' if ledger['publication_phase'] == 'complete' else 'in_progress'
    else:
        ledger['outcome'] = 'not_satisfied'


def fail_case(case: dict, state: str, reason: str) -> None:
    case.update(state=state, reason=reason)
    for row in case['assertions']:
        row.update(state=state, reason=reason, observed=None)


def atomic_record(path: Path, value: dict) -> None:
    temporary = path.with_suffix(path.suffix + '.partial')
    data = json.dumps(value, indent=2, ensure_ascii=True, allow_nan=False).encode() + b'\n'
    with temporary.open('wb') as file:
        file.write(data)
        file.flush()
        os.fsync(file.fileno())
    os.replace(temporary, path)


def _diagnostics(value, label):
    require(type(value) is list and all(type(d) is dict for d in value), label + ' diagnostics missing')
    severities = [d.get('severity') for d in value]
    require(all(type(s) is str and s.lower() in ('info', 'warning', 'blocking', 'error', 'fatal') for s in severities), label + ' has unknown diagnostic severity')
    require(not any(s.lower() in ('blocking', 'error', 'fatal') for s in severities), label + ' has blocking diagnostics')


def unwrap(raw: bytes, request: dict, mode: str) -> tuple[dict, list[dict]]:
    doc = strict_json(raw)
    require(type(doc) is dict and set(doc) == {'payload', 'decisions', 'findings', 'blocked', 'summary'}, 'unsupported controlled wrapper')
    require(doc['blocked'] is False and type(doc['payload']) is dict, 'controlled output blocked')
    _diagnostics(doc['findings'], 'wrapper')
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
        _diagnostics(payload[key].get('diagnostics'), key)
    _diagnostics(payload.get('diagnostics'), 'CLI')
    runner = payload.get('runner_result')
    require(type(runner) is dict and runner.get('run_id') == 'run:headless-preview:' + request['request']['request_id'], 'runner request identity mismatch')
    require(runner.get('job', {}).get('state') == 'COMPLETED', 'runner not completed')
    require('MECHANICS_SOLVED' in runner.get('analysis_status', []), 'runner mechanics not solved')
    _diagnostics(runner.get('diagnostics'), 'runner')
    mechanics = payload.get('mechanics_envelope')
    require(type(mechanics) is dict and mechanics.get('schema_version') == '0.1.0' and mechanics.get('document_kind') == 'openpipestress.product_preview.mechanics_result', 'unsupported raw mechanics version')
    require(mechanics.get('run_id') == 'run:preview-linear-static-001', 'raw run identity mismatch for current profile')
    require(mechanics.get('model_ref') == request['solve']['preview_model']['model']['project']['id'], 'actual model identity mismatch')
    require(mechanics.get('accepted_model_state_mutated') is False, 'solver reports model mutation')
    require(mechanics.get('status', {}).get('mechanics') == 'MECHANICS_SOLVED', 'mechanics not solved')
    _diagnostics(mechanics.get('diagnostics'), 'mechanics')
    require(mode == 'sparse_interactive', 'unsupported actual mode')
    rows = mechanics.get('results')
    require(type(rows) is list and rows and all(type(row) is dict for row in rows), 'result rows missing')
    ids = [row.get('id') for row in rows]
    require(all(type(id) is str and id for id in ids) and len(ids) == len(set(ids)), 'missing/duplicate result ID')
    for row in rows:
        require(set(row) <= {'id', 'kind', 'value', 'unit', 'entity_ref', 'basis_ref', 'metadata', 'source_result_refs'}, 'unknown raw row field for closed profile')
        for name in ('id', 'kind', 'unit', 'entity_ref'):
            _id(row.get(name), 'raw row ' + name)
        if 'basis_ref' in row:
            require(type(row['basis_ref']) is dict and set(row['basis_ref']) == {'ref_type', 'ref_id'}, 'invalid raw basis shape')
            require(row['basis_ref']['ref_type'] in ('load_case', 'combination'), 'unknown raw basis type')
            _id(row['basis_ref']['ref_id'], 'raw basis ID')
        if 'metadata' in row:
            require(type(row['metadata']) is dict and set(row['metadata']) == set(META), 'invalid raw metadata shape')
            require(all(type(v) is str for v in row['metadata'].values()), 'invalid raw metadata value')
        if 'source_result_refs' in row:
            require(type(row['source_result_refs']) is list and all(type(v) is str for v in row['source_result_refs']), 'invalid raw source refs')
        finite(row.get('value'))
    # Actual current producer code is pinned by executable/source identity; its
    # numeric mode row supplements that dispatch, never replaces the identity.
    mode_rows = [row for row in rows if row.get('kind') == 'linear_solver_mode_basis']
    require(bool(mode_rows) and all(finite(row.get('value')) == 1.0 and row.get('unit') == 'mode_code' for row in mode_rows), 'actual mode evidence mismatch or fallback')
    return mechanics, rows


def prepare_case(case: dict, base: Path, purpose: str, semantic_rows: list[dict]) -> dict:
    require(set(case) == {'id', 'input', 'reference', 'criterion', 'assertions'}, 'unknown/missing case fields')
    input_bytes, input_path = read_bound(case['input'], base)
    ref_bytes, ref_path = read_bound(case['reference'], base)
    criterion_bytes, criterion_path = read_bound(case['criterion'], base)
    request, reference, criterion = map(strict_json, (input_bytes, ref_bytes, criterion_bytes))
    require(type(request) is dict and type(request.get('request')) is dict and type(request.get('solve')) is dict, 'input is not a production solve request')
    require(request['request'].get('operation') == 'solve', 'input operation not solve')
    _id(request['request'].get('request_id'), 'actual request ID')
    _id(request['solve']['preview_model']['model']['project']['id'], 'actual model ID')
    require(reference.get('format') == REFERENCE_FORMAT and reference.get('case_id') == case['id'], 'reference identity/version mismatch')
    require(reference.get('readiness') == 'ready' and bool(reference.get('basis')), 'reference not ready or basis absent')
    kind = reference.get('reference_kind')
    require(kind in ('harness_synthetic', 'analytical', 'regression', 'published_numerical'), 'unknown reference kind')
    require(purpose == 'harness_development' or kind != 'harness_synthetic', 'synthetic target cannot score a mechanics development comparison')
    if purpose != 'harness_development':
        require(bool(reference.get('independent_review_ref')), 'independent reference admission missing')
    require(type(criterion) is dict and criterion.get('schema_version') == '0.1.0', 'unsupported tolerance document version')
    profile = criterion.get('tolerance_profile')
    require(type(profile) is dict and profile.get('profile_status') == 'reviewed', 'criterion profile not reviewed')
    rules = profile.get('rules')
    require(type(rules) is list and rules and all(type(x) is dict for x in rules), 'criterion rules missing')
    rule_ids = [rule.get('rule_id') for rule in rules]
    require(all(type(id) is str and id for id in rule_ids) and len(rule_ids) == len(set(rule_ids)), 'duplicate/missing criterion IDs')
    values = reference.get('values')
    require(type(values) is list and all(type(x) is dict for x in values), 'reference values missing')
    expected = {value.get('assertion_id'): value for value in values}
    require(len(expected) == len(values) and set(expected) == {a['id'] for a in case['assertions']}, 'reference assertion denominator mismatch')
    selected, selector_ids = [], set()
    for assertion in case['assertions']:
        require(set(assertion) == {'id', 'selector', 'criterion_rule_id'}, 'unknown/missing assertion fields')
        selector = assertion['selector']
        require(type(selector) is dict and set(selector) == SELECTOR_KEYS, 'complete explicit selector required')
        for key in ('id', 'kind', 'unit', 'entity_ref', 'dimension'):
            _id(selector[key], 'selector ' + key)
        require(selector['id'] not in selector_ids, 'duplicate required result selector')
        selector_ids.add(selector['id'])
        require(type(selector['metadata']) is dict and set(selector['metadata']) == set(META), 'complete source metadata required')
        require(all(type(v) is str and v and v != 'TBD' for v in selector['metadata'].values()), 'unresolved source metadata')
        require(type(selector['basis_ref']) is dict and set(selector['basis_ref']) == {'ref_type', 'ref_id'}, 'explicit case/state basis required')
        require(selector['basis_ref']['ref_type'] in ('load_case', 'combination'), 'invalid case basis')
        _id(selector['basis_ref']['ref_id'], 'case basis ID')
        signatures = [row for row in semantic_rows if row['kind'] == selector['kind'] and row['unit'] == selector['unit'] and row['component'] == selector['metadata']['component']]
        require(len(signatures) == 1 and signatures[0]['source_physical_semantic_dimension'] == selector['dimension'], 'selector contradicts pinned source semantics')
        rules_found = [rule for rule in rules if rule['rule_id'] == assertion['criterion_rule_id']]
        require(len(rules_found) == 1, 'criterion rule missing')
        rule = rules_found[0]
        require(rule.get('dimension_id') == selector['dimension'], 'criterion dimension mismatch')
        require(rule.get('result_family') == signatures[0]['family'] and signatures[0]['family'] is not None, 'criterion result family mismatch')
        unit = rule.get('unit_ref', {})
        require(unit.get('ref') == selector['unit'], 'criterion same-unit binding required')
        require(rule.get('normalization_basis') == 'same_unit_required', 'unit conversion not supported by first gate')
        require(rule.get('tolerance_value_status') in ('externally_governed', 'project_specific_review_required'), 'criterion unresolved')
        require(bool(rule.get('review')) and bool(rule.get('provenance')), 'criterion review/provenance missing')
        pair = ('relative_tolerance_value' in rule, 'absolute_tolerance_value' in rule)
        require(pair in ((True, True), (False, False)), 'incomplete tolerance pair')
        for key in ('relative_tolerance_value', 'absolute_tolerance_value') if pair[0] else ('tolerance_value',):
            require(finite(rule.get(key)) >= 0, 'negative criterion')
        value = expected[assertion['id']]
        require(value.get('unit') == selector['unit'], 'reference unit mismatch')
        selected.append({'assertion': assertion, 'expected': finite(value.get('value')), 'rule': rule})
    return {'input': input_bytes, 'reference_bytes': ref_bytes, 'criterion_bytes': criterion_bytes,
            'request': request, 'selected': selected, 'profile': profile,
            'bindings': [{'role': role, 'path': str(path), 'sha256': sha256_bytes(data)} for role, path, data in
                         [('input', input_path, input_bytes), ('reference', ref_path, ref_bytes), ('criterion', criterion_path, criterion_bytes)]],
            'reference_kind': kind}


def evaluate_rows(case_ledger: dict, prepared: dict, rows: list[dict]) -> None:
    indexed = {row['id']: row for row in rows}
    for item, target in zip(prepared['selected'], case_ledger['assertions']):
        selector = item['assertion']['selector']
        try:
            require(selector['id'] in indexed, 'required result missing')
            row = indexed[selector['id']]
            require(all(row.get(key) == selector[key] for key in ('kind', 'unit', 'entity_ref', 'basis_ref', 'metadata')), 'required semantic selector mismatch')
            actual, expected = finite(row.get('value')), item['expected']
            delta = finite(actual - expected)
            absolute = finite(abs(delta))
            rule = item['rule']
            if 'relative_tolerance_value' in rule:
                finite(finite(rule['relative_tolerance_value']) * max(abs(actual), abs(expected)))
            classification, basis = _classify_delta(absolute, expected, actual, prepared['profile'], rule, selector['unit'])
            require(classification in ('within_tolerance_profile', 'exceeds_tolerance_profile'), 'required comparison unclassified')
            target.update(state='matched' if classification == 'within_tolerance_profile' else 'failed', reason=classification,
                          observed=actual, expected=expected, absolute_delta=absolute, unit=selector['unit'], predicate_basis=basis)
        except (ValueError, KeyError, TypeError, AttributeError, OverflowError) as exc:
            target.update(state='error', reason=str(exc), observed=None)
    states = {row['state'] for row in case_ledger['assertions']}
    case_ledger.update(state='matched' if states == {'matched'} else 'error' if 'error' in states else 'failed', reason='required assertion assessment')


def run_selection(manifest_path: Path, executable: Path, source_root: Path, output_dir: Path,
                  timeout_seconds: float = 30, output_limit_bytes: int = 8 * 1024 * 1024) -> dict:
    with manifest_path.open('rb') as stream:
        raw_manifest = stream.read(MAX_INPUT_BYTES + 1)
    manifest = strict_json(raw_manifest)
    ledger = predeclare(manifest)
    output_dir.mkdir(parents=True, exist_ok=False)
    (output_dir / 'selection.json').write_bytes(raw_manifest)
    ledger['selection_sha256'] = sha256_bytes(raw_manifest)
    selection_artifact = artifact_binding('selection', 'selection.json', raw_manifest)
    case_artifacts = {}
    original_bindings = {}
    summarize(ledger)
    atomic_record(output_dir / 'ledger.json', ledger)
    try:
        require(math.isfinite(timeout_seconds) and 0 < timeout_seconds <= 3600, 'invalid process time limit')
        require(type(output_limit_bytes) is int and 0 < output_limit_bytes <= 64 * 1024 * 1024, 'invalid process output limit')
        require(set(manifest) == {'format', 'profile_id', 'purpose', 'transport', 'runner', 'cases'}, 'unknown/missing selection fields')
        require(manifest['format'] == MANIFEST_FORMAT and manifest['transport'] == TRANSPORT, 'unsupported selection/transport version')
        require(manifest['purpose'] in ('harness_development', 'development_comparison'), 'unsupported run purpose; release qualification not implemented')
        _id(manifest['profile_id'], 'profile ID')
        runner = manifest['runner']
        require(set(runner) == {'candidate_commit', 'executable_sha256', 'solver_mode'}, 'unknown/missing runner binding')
        require(runner['solver_mode'] == 'sparse_interactive', 'dense mode not available through current CLI')
        require(executable.is_absolute() and executable.is_file(), 'explicit executable required')
        require(file_sha256(executable) == runner['executable_sha256'], 'executable digest mismatch')
        actual_commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=source_root, text=True).strip()
        require(actual_commit == runner['candidate_commit'], 'candidate revision mismatch')
        status = subprocess.check_output(['git', 'status', '--porcelain=v1'], cwd=source_root, text=True)
        require(sha256_bytes(SEMANTICS.read_bytes()) == SEMANTICS_SHA256, 'pinned semantic table changed')
        require(sha256_bytes(CLASSIFIER.read_bytes()) == CLASSIFIER_SHA256, 'existing comparison predicate changed')
        semantics = strict_json(SEMANTICS.read_bytes())['rows']
        ledger['execution_basis'] = {'source_commit': actual_commit, 'dirty_tree': bool(status), 'source_status': status,
            'executable_sha256': runner['executable_sha256'], 'solver_mode_dispatch': 'sparse_interactive implicit current CLI',
            'semantic_contract_sha256': SEMANTICS_SHA256, 'existing_predicate_sha256': CLASSIFIER_SHA256,
            'harness_sha256': sha256_bytes(Path(__file__).read_bytes()),
            'process_supervisor_sha256': sha256_bytes(Path(__file__).with_name('qualification_process.py').read_bytes()),
            'python': sys.version, 'platform': platform.platform(),
            'source_artifact_association': 'selected checkout and executable identities; not independent build-source attestation',
            'producer_precision': 'legacy raw0.1 six-decimal publication; not unmerged precise producer'}
    except (ValueError, KeyError, TypeError, AttributeError, OSError, subprocess.SubprocessError) as exc:
        ledger['diagnostics'].append(str(exc))
        for case in ledger['cases']:
            fail_case(case, 'blocked', str(exc))
        verify_publication_custody(ledger, output_dir, selection_artifact, case_artifacts, original_bindings)
        summarize(ledger)
        atomic_record(output_dir / 'ledger.json', ledger)
        write_summary(output_dir / 'summary.md', ledger)
        return ledger
    for index, (case, case_ledger) in enumerate(zip(manifest['cases'], ledger['cases'])):
        try:
            prepared = prepare_case(case, manifest_path.parent, manifest['purpose'], semantics)
            case_ledger['bindings'] = prepared['bindings']
            original_bindings[case['id']] = prepared['bindings']
            case_ledger['reference_kind'] = prepared['reference_kind']
            # Received bindings are copied before execution. Their expected
            # identities stay in memory and are rechecked before publication.
            (output_dir / f'case-{index:04}.reference.json').write_bytes(prepared['reference_bytes'])
            (output_dir / f'case-{index:04}.criterion.json').write_bytes(prepared['criterion_bytes'])
            case_artifacts[case['id']] = [
                artifact_binding('reference', f'case-{index:04}.reference.json', prepared['reference_bytes']),
                artifact_binding('criterion', f'case-{index:04}.criterion.json', prepared['criterion_bytes'])]
            case_ledger['retained_artifacts'] = case_artifacts[case['id']]
            case_ledger['reason'] = 'admitted, process pending'
            atomic_record(output_dir / 'ledger.json', ledger)
            process = capture(executable, ['solve'], prepared['input'], output_dir / f'case-{index:04}', timeout_seconds, output_limit_bytes, expected_executable_sha256=runner['executable_sha256'])
            case_ledger['process'] = process
            case_artifacts[case['id']].append(artifact_binding('input', f'case-{index:04}/stdin.bin', prepared['input']))
            for stream_name in ('stdout', 'stderr'):
                case_artifacts[case['id']].append({'role': stream_name, 'path': f'case-{index:04}/{stream_name}.bin',
                    'sha256': process[stream_name + '_sha256'], 'byte_length': process[stream_name + '_bytes'],
                    'byte_limit': process['output_limit_bytes_per_stream']})
            require(process['executable_sha256_before'] == runner['executable_sha256'], 'executable changed before case')
            if process['outcome'] != 'completed' or not process['stdin_delivery_complete']:
                fail_case(case_ledger, 'error', 'process ' + process['outcome'])
            else:
                stdout_bytes = read_captured_bytes(output_dir / f'case-{index:04}' / 'stdout.bin',
                    process['stdout_sha256'], process['stdout_bytes'], min(output_limit_bytes, MAX_INPUT_BYTES))
                # Decode exactly the bytes verified above, not a reopened archive.
                mechanics, rows = unwrap(stdout_bytes, prepared['request'], runner['solver_mode'])
                case_ledger['observed_result_identity'] = {k: mechanics[k] for k in ('document_kind', 'schema_version', 'run_id', 'model_ref')}
                evaluate_rows(case_ledger, prepared, rows)
            verify_retained_artifacts(output_dir, case_artifacts[case['id']])
            for binding in prepared['bindings']:
                require(file_sha256(Path(binding['path']), max_bytes=MAX_INPUT_BYTES) == binding['sha256'], 'bound input/reference/criterion changed during run')
            if process['outcome'] == 'interrupted':
                ledger['diagnostics'].append('supervisor interrupted; subsequent cases remain not_run')
                break
        except (ValueError, KeyError, TypeError, AttributeError, OSError, OverflowError) as exc:
            fail_case(case_ledger, 'blocked' if case_ledger['process'] is None else 'error', str(exc))
        except KeyboardInterrupt:
            fail_case(case_ledger, 'error', 'interrupted before or after process capture')
            ledger['diagnostics'].append('interrupted; subsequent cases remain not_run')
            break
        finally:
            summarize(ledger)
            atomic_record(output_dir / 'ledger.json', ledger)
    verify_publication_custody(ledger, output_dir, selection_artifact, case_artifacts, original_bindings)
    summarize(ledger)
    atomic_record(output_dir / 'ledger.json', ledger)
    write_summary(output_dir / 'summary.md', ledger)
    return ledger


def write_summary(path: Path, ledger: dict) -> None:
    counts = ledger['summary']['assertions']
    lines = ['# Qualification development observation', '', f"Outcome: {ledger['outcome']}",
             f"Required: {ledger['summary']['required_cases']} cases / {ledger['summary']['required_assertions']} assertions.",
             ' | '.join(f'{state}: {counts[state]}' for state in STATES), '',
             'This report does not establish app Current, independent oracle admission, physical validation, professional acceptance or release.', '',
             '| Case | Outcome | Required assertions |', '|---|---|---|']
    for case in ledger['cases']:
        label = str(case['id']).replace('|', '\\|').replace('\n', ' ')
        lines.append(f"| {label} | {case['state']} | {len(case['assertions'])} |")
    path.write_text('\n'.join(lines) + '\n')


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
        result = run_selection(args.selection.resolve(), args.executable, args.source_root.resolve(), args.output_dir.resolve(), args.timeout_seconds, args.output_limit_bytes)
    except (AdmissionError, OSError, ValueError, KeyError, TypeError) as exc:
        print(json.dumps({'outcome': 'selection_error', 'reason': str(exc), 'qualification': 'not_established_by_this_harness'}))
        raise SystemExit(2)
    print(json.dumps({'outcome': result['outcome'], 'summary': result['summary'], 'ledger': str(args.output_dir / 'ledger.json')}))
    raise SystemExit(0 if result['outcome'] == 'all_required_assertions_matched' else 1)


if __name__ == '__main__':
    main()
