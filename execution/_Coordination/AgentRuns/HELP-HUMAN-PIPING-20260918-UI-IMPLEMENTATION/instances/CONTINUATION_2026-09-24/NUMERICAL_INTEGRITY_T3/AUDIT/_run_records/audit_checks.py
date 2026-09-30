#!/usr/bin/env python3
"""Read-only audit checks. Run from a Git checkout; standard-library Python.

Modes: arithmetic, hashes, merges (GitHub reads), basis.
Outputs contain repository-relative paths only. No source or evidence is edited.
"""
from fractions import Fraction as F
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime
import hashlib
import io
import importlib.util
import json
import re
import subprocess
import sys
import zipfile

ROOT = Path(subprocess.check_output(['git', 'rev-parse', '--show-toplevel'], text=True).strip())
T3 = Path('projects/chirality-piping/execution/_Coordination/AgentRuns/'
          'HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/'
          'CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3')
FK = Path('projects/chirality-piping/core/solver/frame_kernel')
H = Path('projects/chirality-piping/core/solver/performance_harness')
VR = Path('projects/chirality-piping/validation/benchmarks/numerical_robustness')
BASE = '74b3c7313491f27f71c4361d5e1657ee4a39e2f1'


def command(args):
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()


def arithmetic():
    unit = F(1, 2**1074)
    rotation = F(5, 4) * unit
    extent = F(2**100)
    s_ver = extent * rotation
    s_pub = F.from_float(float(extent) * float(rotation))
    epsilon = F(1, 2**64)
    gap_claim = epsilon + F(1, 2**52)
    # Abstract accepted-state example: exact verification; candidate translation
    # zero; identical nonzero rotation at p and 2p; zero verification error.
    # This is NOT a source model, Rust execution, or a product-level witness.
    b = epsilon * s_pub
    translation_truth = F(9, 8) * b
    premise_fails = s_ver - s_pub > gap_claim * s_ver + unit / 2
    rule_accepts = translation_truth <= epsilon * s_ver
    claimed_interval_fails = translation_truth > b * (1 + F(1, 2**22))
    assert premise_fails and rule_accepts and claimed_interval_fails
    # An additional boundary: the coupled published scale becomes exactly zero.
    # A1 does not add its one-subnormal term when s_star == 0.
    tiny_extent = F(1, 2**100)
    tiny_s_ver = tiny_extent * rotation
    tiny_s_pub = float(tiny_extent) * float(rotation)
    assert tiny_s_ver > 0 and tiny_s_pub == 0.0
    return {
        'basis': BASE,
        'method': 'independent exact fractions and IEEE binary64 conversion; no generator imports',
        'rotation_in_min_subnormal_units': str(rotation / unit),
        'published_rotation_hex': float(rotation).hex(),
        'extent_hex': float(extent).hex(),
        'published_translation_scale_hex': float(s_pub).hex(),
        'verification_to_published_scale': str(s_ver / s_pub),
        'relative_scale_loss': str((s_ver - s_pub) / s_ver),
        'published_scale_outside_A1_small_scale_branch': s_pub >= F(1, 2**988),
        'review_scale_closeness_premise_refuted': premise_fails,
        'abstract_state_stop_rule_accepts': rule_accepts,
        'abstract_state_published_interval_misses_truth': claimed_interval_fails,
        'abstract_state_error_over_published_bound': str(translation_truth / b),
        'zero_scale_extent_hex': float(tiny_extent).hex(),
        'zero_scale_verification_positive': tiny_s_ver > 0,
        'zero_scale_published_zero': tiny_s_pub == 0.0,
        'limitation': 'No realized PrimitiveSource or Rust solve is claimed. Reachability remains open.',
    }


def hashes():
    results = []
    exceptions = {
        f'{name}/_run_records/SHA256SUMS'
        for name in ['REVIEW', 'DESIGN_NUMERICS', 'DESIGN_STANDING', 'REFERENCES', 'REFERENCES_ELOAD']
    }
    for manifest in sorted((ROOT / T3).rglob('SHA256SUMS')):
        rel = manifest.relative_to(ROOT / T3).as_posix()
        if rel.startswith('AUDIT/'):
            continue
        wd = manifest.parent.parent if rel in exceptions else manifest.parent
        arg = manifest.relative_to(wd).as_posix()
        run = subprocess.run(['shasum', '-a', '256', '-c', arg], cwd=wd,
                             text=True, capture_output=True)
        results.append({
            'manifest': rel, 'working_directory': wd.relative_to(ROOT / T3).as_posix(),
            'exit': run.returncode,
            'verified_entries': sum(line.endswith(': OK') for line in run.stdout.splitlines()),
            'failures': [line for line in run.stdout.splitlines() if not line.endswith(': OK')],
            'stderr': run.stderr,
        })
    return {'basis': BASE, 'manifests': len(results),
            'passed': sum(r['exit'] == 0 for r in results),
            'verified_entries': sum(r['verified_entries'] for r in results), 'results': results}


def merges():
    results = []
    for name, number in [('K4', 1054), ('KF1', 1056), ('VK', 1057),
                         ('K6B', 1058), ('KF3', 1059), ('KF2', 1060)]:
        record = (ROOT / T3 / 'IMPLEMENTATION' / (name + '_MERGE') / 'RECORD.md').read_text()
        head = re.search(r'Candidate head:\*\* `([a-f0-9]{40})`', record)[1]
        merge = re.search(r'as `([a-f0-9]{40})`', record)[1]
        dispatch, base = re.search(r'full-SHA dispatch \*\*(\d+)\*\* \(target_base `([a-f0-9]{40})`', record).groups()
        pr = json.loads(command(['gh', 'pr', 'view', str(number), '--repo', 'sgttomas/chirality',
                                 '--json', 'state,headRefOid,mergeCommit,mergedAt']))
        run = json.loads(command(['gh', 'run', 'view', dispatch, '--repo', 'sgttomas/chirality',
                                  '--json', 'headSha,event,conclusion,status,workflowName']))
        parents = command(['git', 'show', '-s', '--format=%P', merge]).split()
        ancestor = subprocess.run(['git', 'merge-base', '--is-ancestor', base, head], cwd=ROOT).returncode == 0
        changed = command(['git', 'diff', '--name-only', head, merge, '--',
                           'projects/chirality-piping', 'tools', '.github']).splitlines()
        results.append({'slice': name, 'pr': number, 'recorded_head': head,
                        'recorded_merge': merge, 'recorded_target_base': base,
                        'github_pr': pr, 'github_dispatch': run, 'dispatch_id': dispatch,
                        'parents': parents, 'head_matches': pr['headRefOid'] == head,
                        'merge_matches': pr['mergeCommit']['oid'] == merge,
                        'head_is_second_parent': len(parents) == 2 and parents[1] == head,
                        'recorded_base_is_first_parent': parents[0] == base,
                        'recorded_base_ancestor_of_head': ancestor,
                        'relevant_merge_diff': changed, 'dispatch_head_matches': run['headSha'] == head})
    return {'basis': BASE, 'results': results,
            'limitation': 'Run metadata verifies event, head and conclusion, not the dispatch input value; target_base input needs its run artifact/log.'}


def dispatches():
    prior = json.loads((Path(__file__).parent / 'merges.json').read_text())
    results = []
    for row in prior['results']:
        run_id = row['dispatch_id']
        listing = json.loads(command(['gh', 'api',
            f'repos/sgttomas/chirality/actions/runs/{run_id}/artifacts']))
        artifact = next(a for a in listing['artifacts'] if a['name'] == 'piping-e2e-selection')
        raw = subprocess.check_output(['gh', 'api',
            f"repos/sgttomas/chirality/actions/artifacts/{artifact['id']}/zip"], cwd=ROOT)
        archive = zipfile.ZipFile(io.BytesIO(raw))
        plan_bytes = archive.read('piping-e2e-plan.json')
        plan = json.loads(plan_bytes)
        results.append({'slice': row['slice'], 'run_id': run_id, 'artifact_id': artifact['id'],
                        'archive_sha256': hashlib.sha256(raw).hexdigest(),
                        'plan_sha256': hashlib.sha256(plan_bytes).hexdigest(), 'plan': plan,
                        'head_matches': plan['head'] == row['recorded_head'],
                        'base_matches': plan['base'] == row['recorded_target_base'],
                        'target_base_matches': plan['target_base'] == row['recorded_target_base']})
    return {'basis': BASE, 'results': results}


def ci():
    prior = json.loads((Path(__file__).parent / 'merges.json').read_text())
    tasks = []
    for row in prior['results']:
        text = (ROOT / T3 / 'IMPLEMENTATION' / (row['slice'] + '_MERGE') / 'RECORD.md').read_text()
        line = next(line for line in text.splitlines() if 'pull_request runs:' in line)
        ids = re.findall(r'\*\*(\d+)\*\*', line) + [row['dispatch_id']]
        tasks.extend((row['slice'], run, row['recorded_head']) for run in ids)
    def read(task):
        name, run, head = task
        data = json.loads(command(['gh', 'run', 'view', run, '--repo', 'sgttomas/chirality',
                                  '--json', 'headSha,event,conclusion,status,jobs']))
        jobs = []
        for job in data.pop('jobs'):
            start, end = job.get('startedAt'), job.get('completedAt')
            duration = None
            if start and end and not start.startswith('0001') and not end.startswith('0001'):
                duration = (datetime.fromisoformat(end.replace('Z', '+00:00')) -
                            datetime.fromisoformat(start.replace('Z', '+00:00'))).total_seconds()
            jobs.append({k: job.get(k) for k in ['name', 'conclusion', 'status', 'startedAt', 'completedAt']}
                        | {'duration_seconds': duration})
        return {'slice': name, 'run_id': run, 'metadata': data, 'head_matches': data['headSha'] == head, 'jobs': jobs}
    with ThreadPoolExecutor(max_workers=3) as pool:
        results = list(pool.map(read, tasks))
    return {'basis': BASE, 'results': results}


def dec025():
    prior = json.loads((Path(__file__).parent / 'merges.json').read_text())
    results = []
    for row in prior['results']:
        folder = ROOT / T3 / 'IMPLEMENTATION' / (row['slice'] + '_MERGE') / 'dec025'
        sweeps = []
        for file in sorted(folder.glob('SWEEP_*.json')):
            data = json.loads(file.read_text())
            sweeps.append({'file': file.relative_to(ROOT).as_posix(), 'git': data['git'],
                           'matches_final_head': data['git']['commit_hash'] == row['recorded_head']})
        suites = (folder / 'suites.log').read_text()
        results.append({'slice': row['slice'], 'sweeps': sweeps,
                        'suite_results': [dict(zip(['exit', 'passed', 'failed', 'ignored', 'manifest'], match))
                            for match in re.findall(r'^(\d+) passed=(\d+) failed=(\d+) ignored=(\d+) (\S+)', suites, re.M)],
                        'failed_test_names': sorted(set(re.findall(r'^test (\S+) \.\.\. FAILED', suites, re.M))),
                        'surfaces': (folder / 'surfaces.txt').read_text(),
                        'stored_comparison': (folder / 'suites_vs_baseline.txt').read_text()})
    return {'basis': BASE, 'results': results,
            'limitation': 'Inspection of hash-verified historical logs, not fresh execution. suites.log may be a driver summary; per-manifest logs are inspected separately if available.'}


def admissions():
    spec = importlib.util.spec_from_file_location('audit_vk_runner', ROOT / VR / 'runner/vk_scale_runner.py')
    runner = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(runner)
    def lines(path):
        return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]
    hc = {row['model']: row for row in lines(ROOT / H / 'observations/k6b/counts.jsonl')}
    results = []
    for name in ['VK', 'KF3']:
        folder = ROOT / T3 / 'IMPLEMENTATION' / name / '_run_records/b'
        records = lines(folder / 'runs/records.jsonl')
        counts = lines(folder / ('setup/counts.jsonl' if name == 'VK' else 'counts.json'))
        old = {row['model']: row for row in counts}
        new = {}
        compatibility = []
        for mid, row in old.items():
            h = hc[mid]
            # The known shift-pass correction, not a complete replacement E_max.
            delta = 2 * h['w1_free_dofs'] * 144 - 8 * (3 * h['w1_rows'] + h['w1_free_dofs'])
            estimate = h['estimate_adm_bytes_w1a'] - h['estimate_w1_fixed_bytes'] + row['estimate_fixed_bytes'] + max(0, delta)
            for key in ['free_dofs', 'rows', 'profile_entries', 'pattern_entries']:
                assert row[key] == h['w1_' + key], (mid, key)
            compatibility.append({'model': mid, 'old_estimate': row['estimate_max_bytes'],
                                  'sensitivity_estimate': estimate, 'known_shift_delta': delta})
            new[mid] = dict(row, estimate_max_bytes=estimate, estimate_adm_bytes_w1a=estimate)
        previous_old, previous_new, rows = [], [], []
        for rec in records:
            original = runner.admission(rec, old, previous_old, rec['baseline_rss_bytes'],
                                        rec['baseline_footprint_bytes'], True)
            changed = runner.admission(rec, new, previous_new, rec['baseline_rss_bytes'],
                                       rec['baseline_footprint_bytes'], True)
            rows.append({'run_id': rec['run_id'], 'recorded_decision': rec['admission']['decision'],
                         'replayed_decision': original['decision'], 'sensitivity_decision': changed['decision'],
                         'recorded_estimate': rec['estimate_adm_bytes'], 'sensitivity': changed,
                         'original_decision_fields_match': original == rec['admission']})
            previous_old.append(rec)
            previous_new.append(dict(rec, estimate_adm_bytes=new[rec['model']]['estimate_max_bytes']))
        results.append({'slice': name, 'count_compatibility': compatibility, 'runs': rows,
                        'recorded_source_commits': sorted({r['source_commit'] for r in records})})
    return {'basis': BASE, 'results': results,
            'limitation': 'Sensitivity replay uses final H kernel terms plus VR fixed term and the known shift delta at every size, recalibrating rho. It is not K6c phase-by-phase re-derivation, and does not establish a complete memory upper bound.'}


def figures():
    def lines(path):
        return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]
    h = {r['model']: r for r in lines(ROOT / H / 'observations/k6b/counts.jsonl')}
    kf3 = ROOT / T3 / 'IMPLEMENTATION/KF3/_run_records'
    vr = {r['model']: r for r in lines(kf3 / 'b/counts.json')}
    memory = []
    for suffix in ['AX', 'ROT']:
        mid = 'RF-LARGE-TREE-n10000-' + suffix
        path = next((kf3 / 'b/runs').glob('[0-9][0-9]_' + mid + '_w1a.jsonl'))
        peak = next(r for r in lines(path) if r['kind'] == 'summary')['repeats_heap_peak']
        hc, vc = h[mid], vr[mid]
        estimate = hc['estimate_adm_bytes_w1a']
        like = estimate - hc['estimate_w1_fixed_bytes'] + vc['estimate_fixed_bytes']
        omitted = 2 * hc['w1_free_dofs'] * 144 - 8 * (3 * hc['w1_rows'] + hc['w1_free_dofs'])
        memory.append({'model': mid, 'unit': 'bytes', 'measured_heap': peak,
                       'vr_stale_estimate': vc['estimate_max_bytes'], 'h_estimate': estimate,
                       'h_kernel_plus_vr_fixed': like, 'like_for_like_margin': like - peak,
                       'known_shift_undercount': omitted, 'h_plus_known_shift': estimate + omitted})
    probe = (kf3 / 'd/b1_probe.txt').read_text()
    size, coefficients = None, []
    for line in probe.splitlines():
        match = re.match(r'MODEL KF3-TREE-n(\d+)-AX', line)
        if match:
            size = int(match[1])
        match = re.match(r'PAIR (\d+) vs (\d+):.*worst \(b\) ratio Some\(([^)]+)\)', line)
        if match:
            coefficients.append({'members': size, 'candidate_p': int(match[1]),
                                 'verification_P': int(match[2]), 'ratio': float(match[3]),
                                 'c': 64 * float(match[3])})
    scale_results = []
    for name in ['VK', 'KF3']:
        folder = ROOT / T3 / 'IMPLEMENTATION' / name / '_run_records/b/runs'
        records = lines(folder / 'records.jsonl')
        for row in records:
            if row['members'] == 10000:
                scale_results.append({k: row.get(k) for k in [
                    'model', 'selected_precision', 'outcome', 'invocation_charged',
                    'repeats_heap_peak', 'wall_s']} | {'slice': name})
    previous = {'K4': 'K6', 'KF1': 'K4', 'VK': 'KF1', 'K6B': 'VK', 'KF3': 'K6B', 'KF2': 'KF3'}
    def suites(name):
        txt = (ROOT / T3 / 'IMPLEMENTATION' / (name + '_MERGE') / 'dec025/suites.log').read_text()
        return {m[4]: [int(x) for x in m[:4]] for m in re.findall(
            r'^(\d+) passed=(\d+) failed=(\d+) ignored=(\d+) (\S+)', txt, re.M)}
    suite_deltas = []
    for candidate, base in previous.items():
        a, b = suites(base), suites(candidate)
        delta = {k: {'before_exit_pass_fail_ignore': a.get(k), 'after_exit_pass_fail_ignore': b.get(k)}
                 for k in sorted(set(a) | set(b)) if a.get(k) != b.get(k)}
        suite_deltas.append({'slice': candidate, 'baseline': base, 'delta': delta})
    return {'basis': BASE, 'memory': memory, 'b1_coefficients': coefficients,
            'scale_10000': scale_results, 'suite_deltas': suite_deltas,
            'qualification': 'B1 is the scaled-load K4-adapter probe, not the VR committed model. 10000-member B1 classification remains extrapolation; no design option is selected.'}


def basis():
    paths = [Path('AGENTS.md'), Path('agents/AGENT_TASK.md'),
             Path('projects/chirality-piping/AGENTS.md'),
             Path('.agents/skills/chirality-change/SKILL.md')]
    paths += [T3 / p for p in [
        'HANDOFF_2026-09-30_AUDIT_PAUSE.md', 'OPERATING_NOTES_2026-09-30.md',
        'OPERATING_NOTES_FOR_LOCAL_ROOT.md', 'ROOT_RULINGS_V1.md',
        'TASK_BRIEFS/_COMMON.md', 'TASK_BRIEFS/I8R_K1_RESUME.md',
        'TASK_BRIEFS/I21_K6C_IMPLEMENTATION.md', 'DESIGN_NUMERICS/DESIGN.md',
        'DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md',
        'IMPLEMENTATION/KF3/PLAN_CHECKPOINT0.md', 'IMPLEMENTATION/KF2/PLAN_CHECKPOINT0.md',
        'IMPLEMENTATION/KF3/RETURN.md', 'IMPLEMENTATION/VK/RETURN.md',
        'IMPLEMENTATION/S11K/RETURN.md', 'IMPLEMENTATION/K2B/RETURN.md',
        'IMPLEMENTATION/K6/RETURN.md', 'IMPLEMENTATION/K6B/RETURN.md',
        'TASK_BRIEFS/I1_S11K_RV1_FIXES.md',
        'IMPLEMENTATION/KF3/_run_records/d/b2_emax.py',
        'IMPLEMENTATION/KF3/_run_records/d/b1_probe.txt',
        'IMPLEMENTATION/KF1/_run_records/a/t_tradeoff_probe.txt',
        'IMPLEMENTATION/M03_SKEW_PIN_MERGE/dec025/dec025_mac.sh.txt',
        'HANDOFF_2026-09-30_TOOLS/cmp_suites.py.txt',
        'REVIEW/K4_REVIEW.md', 'REVIEW/KF3_REVIEW.md', 'REVIEW/RECORDS_PR1062_REVIEW.md']]
    paths += [FK / 'src/structural/retained' / (n + '.rs') for n in
              ['adaptive', 'source', 'bound', 'verify', 'wide_sum', 'assemble', 'factor']]
    paths += [FK / 'tests/retained_k4/gen_k4_vectors.py', FK / 'tests/retained_k4/models.rs',
              H / 'src/k6/w1/counts.rs', H / 'runner/k6_runner.py',
              H / 'src/bin/k6_observe/main.rs', VR / 'src/scale.rs',
              VR / 'runner/vk_scale_runner.py', VR / 'runner/check_fault_sites.py',
              VR / 'tests/feature_guard.rs', Path('tools/practitioner_harness/test_live_baseline.py'),
              Path('tools/practitioner_harness/cmd_self_check.py'),
              Path('.github/workflows/piping-desktop-e2e.yml'),
              Path('projects/chirality-piping/tools/release/check_release_readiness.py')]
    paths += [T3 / 'IMPLEMENTATION' / (n + '_MERGE') / 'RECORD.md'
              for n in ['K4', 'KF1', 'VK', 'K6B', 'KF3', 'KF2']]
    paths += [T3 / 'REVIEW' / (n + '_REVIEW.md') for n in ['K4', 'KF1', 'VK', 'K6B', 'KF3', 'KF2']]
    paths += [Path('projects/chirality-piping/execution/_Coordination/WorkGraphs/'
                   'HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md')]
    return {'basis': BASE, 'branch': command(['git', 'branch', '--show-current']),
            'role': 'independent auditor reporting directly to owner; TASK instructions read as requested',
            'execution': 'Codex native host tools, no delegated agents',
            'inputs': [{'path': p.as_posix(), 'sha256': hashlib.sha256((ROOT / p).read_bytes()).hexdigest()}
                       for p in sorted(set(paths))]}


if __name__ == '__main__':
    modes = {'arithmetic': arithmetic, 'hashes': hashes, 'merges': merges, 'basis': basis,
             'dispatches': dispatches, 'dec025': dec025, 'admissions': admissions, 'figures': figures,
             'ci': ci}
    print(json.dumps(modes[sys.argv[1]](), indent=2, sort_keys=True))
