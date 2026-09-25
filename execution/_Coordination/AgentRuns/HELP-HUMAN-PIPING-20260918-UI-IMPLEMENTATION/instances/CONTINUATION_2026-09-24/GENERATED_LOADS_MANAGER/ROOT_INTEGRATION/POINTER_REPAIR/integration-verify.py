"""Read-only integration/evidence inspection. Writes only beside this script."""
from pathlib import Path
import datetime
import hashlib
import json
import subprocess

OUT = Path(__file__).resolve().parent
R = Path('/private/tmp/piping-generated-loads-20260924')
P = R / 'projects/chirality-piping'
C = P / 'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/GENERATED_LOADS_MANAGER'
BASE = 'f702b439536c6e76af8e1c81ee536d5685e87907'
SOURCE = '7654fb36eb0912ba540f40d5a4dfdf736f7cbfce'
MAIN = '3c3adae355739b2ad24acdb33d3b5422bb36db95'
CAND = '64487068b740977083275baa4ead5ed4171c6084'
def git(*args):
    return subprocess.check_output(['git', '-C', str(R), *args])
def digest(data):
    return hashlib.sha256(data).hexdigest()
def sha(path):
    return digest(path.read_bytes())
def load(path):
    return json.loads(path.read_text())
def check_rows(rows):
    return [{'path': x['path'], 'expected_sha256': x['sha256'],
             'live_sha256': sha(R / x['path']),
             'matches': sha(R / x['path']) == x['sha256']} for x in rows]

assert git('rev-parse', '--show-toplevel').decode().strip() == str(R)
assert git('rev-parse', 'HEAD').decode().strip() == CAND
status_before = git('status', '--porcelain').decode()
w = load(C / 'HANDOFF_WHITELIST.json')
f = load(C / 'CANDIDATE_FILES.json')
wasm = load(C / 'WASM_ARTIFACTS.json')
root = load(C / 'ROOT_INTEGRATION/FINAL_RUNTIME_BACKCHECK.json')
source_checks = check_rows(w['source_files'])
evidence_checks = check_rows(w['evidence_files'])
old_delta = git('diff', '--binary', BASE, SOURCE)
new_delta = git('diff', '--binary', MAIN, CAND)
paths = git('diff', '--name-only', MAIN, CAND).decode().splitlines()
root_paths = [str(x.relative_to(R)) for x in sorted((C / 'ROOT_INTEGRATION').iterdir())]
expected = [x['path'] for x in w['source_files'] + w['evidence_files']] + [w['manifest_self_path']] + root_paths
source_patch = git('diff', '--binary', BASE, CAND, '--', *[x['path'] for x in f['files']])
incoming = git('diff', '--name-only', BASE, MAIN).decode().splitlines()
command = ['python3', str(R / 'tools/software_workflow/validate_change_scope.py'), str(R), '--base', MAIN, '--head', CAND]
for path in expected:
    command.extend(['--allowed', path])
scope = subprocess.run(command, capture_output=True, text=True)
(OUT / 'SCOPE_CHECK.json').write_text(scope.stdout)
(OUT / 'SCOPE_CHECK.stderr.txt').write_text(scope.stderr)

runtime = []
for name in ['narrowed_tsc', 'narrowed_vitest']:
    record_path = C / '_run_records/checks' / (name + '.result.json')
    log_path = C / '_run_records/checks' / (name + '.log')
    record = load(record_path)
    root_record = next(x for x in root['checks'] if x['record'] == str(record_path.relative_to(R)))
    runtime.append({'name': name, 'record_sha256': sha(record_path), 'log_sha256': sha(log_path),
        'command': record['command'], 'exit_code': record['exit_code'], 'started_at': record['started_at'],
        'pre_source_count': len(record['source_files']), 'post_source_count': len(record['source_files_after']),
        'pre_equals_final': record['source_files'] == f['files'],
        'post_equals_final': record['source_files_after'] == f['files'],
        'recorded_unchanged': record['candidate_unchanged'],
        'patch_matches': record['candidate_patch_sha256'] == f['patch_sha256'],
        'log_hash_matches': record['log_sha256'] == sha(log_path),
        'root_record_hash_matches': root_record['record_sha256'] == sha(record_path),
        'root_log_hash_matches': root_record['log_sha256'] == sha(log_path),
        'root_exit_matches': root_record['exit_code'] == record['exit_code'],
        'log': log_path.read_text()})

prior = load(C / '_run_records/checks/candidate_wasm_final.result.json')
prior_files = {x['path']: x['sha256'] for x in prior['source_files']}
current_files = {x['path']: x['sha256'] for x in f['files']}
prior_after = {x['path']: x['sha256'] for x in prior['source_files_after']}
runtime_carry = check_rows(wasm['carried_runtime_source_files'])
for row in runtime_carry:
    row['matches_build_before'] = row['expected_sha256'] == prior_files.get(row['path'])
    row['matches_build_after'] = row['expected_sha256'] == prior_after.get(row['path'])
    row['matches_current_manifest'] = row['expected_sha256'] == current_files.get(row['path'])
assets = check_rows(wasm['artifacts'])
for row, expected_asset in zip(assets, wasm['artifacts']):
    row['bytes_match'] = (R / row['path']).stat().st_size == expected_asset['bytes']

check_map = load(C / 'CHECK_MAP.json')
check_map_results = []
for row in check_map['checks'] + check_map['application_checks']:
    path = C / row['result_path']
    record = load(path)
    log_path = path.with_name(path.name.replace('.result.json', '.log'))
    check_map_results.append({'check': row['check'], 'result_sha256': sha(path),
        'result_hash_matches': sha(path) == row['result_sha256'],
        'log_hash_matches': sha(log_path) == record['log_sha256'],
        'exit_code': record['exit_code'], 'reported_passed': row['passed']})

models, unparseable = [], []
def scan(value, path, key='$'):
    if isinstance(value, dict):
        if 'nodes' in value and ('pipe_segments' in value or 'load_cases' in value):
            models.append({'path': path, 'key': key, 'project_id': value.get('project', {}).get('id'),
                'nodes': len(value.get('nodes', [])), 'pipes': len(value.get('pipe_segments', [])),
                'pipe_labels': [x.get('label') for x in value.get('pipe_segments', [])]})
        for k, v in value.items(): scan(v, path, key + '.' + k)
    elif isinstance(value, list):
        for i, v in enumerate(value): scan(v, path, f'{key}[{i}]')
    elif isinstance(value, str) and value.startswith('{'):
        try: decoded = json.loads(value)
        except ValueError: return
        scan(decoded, path, key + '.parsed')
for row in w['source_files'] + w['evidence_files']:
    path = R / row['path']
    if path.suffix == '.json':
        try: value = load(path)
        except ValueError:
            unparseable.append({'path': row['path'], 'bytes': path.stat().st_size})
            continue
        scan(value, row['path'])
origin = load(C / '_run_records/LEGACY_FIXTURE_ORIGIN.json')
baseline = C / '_run_records/checks/baseline_artifacts/density_baseline.json'
fixture = R / origin['fixture_path']

consulted = ['AGENTS.md', 'agents/AGENT_TASK.md', '.agents/skills/software-code-review/SKILL.md',
    'projects/chirality-piping/AGENTS.md', 'projects/chirality-piping/loop/LOOP_INIT.md',
    'projects/chirality-piping/loop/WORKPLAN_2026-07-18b_piping_loop.md', 'tools/software_workflow/validate_change_scope.py']
for rel in ['HANDOFF_WHITELIST.json', 'CANDIDATE_FILES.json', 'CHECK_MAP.json', 'RETURN.md', 'ROOT_BRIEF.md',
    'NATIVE_WITNESS_HANDOFF.md', 'WASM_ARTIFACTS.json', 'review/RETURN.md', 'review/REFERENCE_CHECK.md',
    'review/_run_records/FINAL_BACKCHECK.md', 'review/_run_records/LOCK_NORMALIZATION_BACKCHECK.md',
    'review/_run_records/DOWNSTREAM_LOCK_BACKCHECK.md', 'review/_run_records/UI_RUNTIME_TSC_BACKCHECK.md',
    'review/_run_records/tsc_narrowing_backcheck/MANIFEST.json', 'ROOT_INTEGRATION/FINAL_RUNTIME_BACKCHECK.json',
    'ROOT_INTEGRATION/FINAL_RUNTIME_BACKCHECK.md', '_run_records/LEGACY_FIXTURE_ORIGIN.json',
    '_run_records/FINAL_REVIEW_FOLLOWUP_REFUSAL.json', '_run_records/run_check.py', 'baseline/RETURN.md']:
    consulted.append(str((C / rel).relative_to(R)))
audit = {
    'schema': 'bounded-integration-evidence-review/v1', 'observed_at_utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
    'reviewer': '/root/m35_integration_review', 'parent': '/root', 'role': 'TASK',
    'mechanism': 'delegated-harness-native fresh independent reviewer; no subdelegation',
    'scope': 'Integration identity and final evidence custody; prior complete source review is carried, not re-performed',
    'review_limits': 'No tests, builds, native/browser activity or Git mutations; writes only in this separate output directory. No review of future CI.',
    'repo_root': str(R), 'working_root': str(P), 'candidate': CAND, 'reviewed_source_commit': SOURCE, 'main': MAIN, 'base': BASE,
    'candidate_parents': git('show', '-s', '--format=%P', CAND).decode().strip().split(),
    'origin_main': git('rev-parse', 'origin/main').decode().strip(),
    'origin_candidate': git('rev-parse', 'origin/codex/piping-generated-loads-20260924').decode().strip(),
    'status_before': status_before,
    'deltas': {'byte_equal': old_delta == new_delta, 'old_sha256': digest(old_delta), 'new_sha256': digest(new_delta),
        'file_count': len(paths), 'missing': sorted(set(expected)-set(paths)), 'unexpected': sorted(set(paths)-set(expected))},
    'incoming_main_paths': incoming,
    'incoming_qualification': 'No Piping path or applicable Root/Piping/TASK/review-skill instruction changed. Main does add a Root reverse-engineer-software workflow/catalog and alter its catalog test, plus PEC records.',
    'scope_check_exit': scope.returncode,
    'source_patch_sha256': digest(source_patch), 'stored_patch_sha256': sha(R / w['patch_path']),
    'source_patch_matches_record': digest(source_patch) == f['patch_sha256'] == w['patch_sha256'],
    'whitelist_sha256': sha(C / 'HANDOFF_WHITELIST.json'),
    'whitelist_matches_root_record': sha(C / 'HANDOFF_WHITELIST.json') == root['handoff_manifest_sha256'],
    'source_rows': source_checks, 'evidence_rows': evidence_checks, 'root_integration_files': check_rows([{'path': x, 'sha256': sha(R/x)} for x in root_paths]),
    'runtime_final': runtime, 'check_map_results': check_map_results,
    'compiled_carry': runtime_carry, 'wasm_assets': assets,
    'wasm_pre_post_identical': prior['source_files'] == prior['source_files_after'],
    'source_changes_since_final_wasm_build': [p for p in current_files if current_files[p] != prior_files.get(p)],
    'latest_review_source_manifest_matches': load(C / 'review/_run_records/tsc_narrowing_backcheck/CANDIDATE_FILES.reviewed.json')['files'] == f['files'],
    'latest_review_patch_matches': sha(C / 'review/_run_records/tsc_narrowing_backcheck/CANDIDATE.reviewed.patch') == f['patch_sha256'],
    'privacy_json_models': models, 'retained_unparseable_json': unparseable,
    'privacy_scan_note': 'Initial all-JSON parser hit preserved zero-byte failure output; corrected bounded scan records those entries explicitly. No test execution.',
    'fixture': {'origin_hash_matches': sha(baseline) == origin['origin_sha256'],
        'fixture_hash_matches': sha(fixture) == origin['fixture_sha256'],
        'parsed_exact_origin_value': load(fixture) == load(baseline)[origin['selector']],
        'origin_claim': origin['claim']},
    'actionable_findings': [{'priority': 3, 'path': str((C/'NATIVE_WITNESS_HANDOFF.md').relative_to(R)), 'line': 5,
        'issue': 'Original baseline pointer omits checks/; the stated path does not exist.',
        'stated_exists': (C/'_run_records/baseline_artifacts/density_baseline.json').exists(),
        'correct_path': '_run_records/checks/baseline_artifacts/density_baseline.json', 'correct_sha256': sha(baseline)}],
    'consulted_origins': [{'path': x, 'sha256': sha(R/x)} for x in consulted],
    'wider_consultation': 'ROOT_BRIEF.md embeds full WORKING_ITEMS role; read as historical assignment/provenance only, no role transition.',
    'status_after': git('status', '--porcelain').decode()
}
(OUT/'HASHES.json').write_text(json.dumps(audit, indent=2)+'\n')
print(json.dumps({'source': len(source_checks), 'evidence': len(evidence_checks), 'delta_identical': old_delta == new_delta,
    'scope_exit': scope.returncode, 'bad_hashes': [x['path'] for x in source_checks+evidence_checks+runtime_carry+assets if not x['matches']],
    'models': len(models), 'status_after': audit['status_after'], 'hashes_sha256': sha(OUT/'HASHES.json')}, indent=2))
