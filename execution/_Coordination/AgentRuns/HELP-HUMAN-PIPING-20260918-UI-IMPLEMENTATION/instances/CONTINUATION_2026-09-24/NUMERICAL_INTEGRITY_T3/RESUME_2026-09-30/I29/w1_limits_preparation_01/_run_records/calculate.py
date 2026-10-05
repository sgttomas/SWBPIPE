"""I29 run-specific immutable-record extraction/arithmetic; no product code runs.

Rerun: python3 calculate.py K6C_ROOT OUTPUT_DIRECTORY
Only CALCULATIONS.json and CALCULATION_ORIGINS.json are written below OUTPUT_DIRECTORY.
"""
import hashlib
import json
from pathlib import Path
import re
import statistics
import sys
import tarfile

root, out = map(Path, sys.argv[1:])
t3 = Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3')
r = t3 / 'RESUME_2026-09-30'
origins = []

def read(path):
    data = (root / path).read_bytes()
    origins.append({'path': str(path), 'sha256': hashlib.sha256(data).hexdigest(), 'bytes': len(data)})
    return data

def span(rows, field):
    return [min(x[field] for x in rows), max(x[field] for x in rows)]

processes = []
raw_rss = {}
closure_checks = 0
for tier, archive in [('T1', 'records.tar.gz'), ('T2', 'delta_records.tar.gz'), ('T3', 'delta_records.tar.gz'), ('T4', 'delta_records.tar.gz')]:
    path = r / 'MEASUREMENTS' / ('W1_' + tier) / '_run_records' / archive
    read(path)
    with tarfile.open(root / path) as tar:
        for member in tar.getmembers():
            if not re.fullmatch(r'\d+_.*_(w1a|sparse).jsonl', Path(member.name).name):
                continue
            rows = [json.loads(line) for line in tar.extractfile(member) if line.strip()]
            start = next(x for x in rows if x['kind'] == 'start')
            record_member = str(Path(member.name).with_suffix('.record.json'))
            record = json.load(tar.extractfile(record_member))
            raw_rss[record['run_id']] = record['peak_rss_bytes']
            if start['mode'] != 'w1a':
                continue
            outcomes = [x for x in rows if x['kind'] == 'outcome']
            attempts = [x for x in rows if x['kind'] == 'attempt']
            for outcome in outcomes:
                a = [x for x in attempts if x['repeat'] == outcome['repeat']]
                assert sum(x['charged_by'] for x in a) == outcome['meter_charged']
                for x in a:
                    assert x['charged_by'] == x['own_total'] + (x['shared_work'] if x['shared_built_here'] else 0) + (x['verification_shared_work'] if x['verification_shared_built_here'] else 0)
                    closure_checks += 1
            assert len(outcomes) == 5 == record['summary']['repeats_completed']
            assert len({x['meter_charged'] for x in outcomes}) == 1
            counts = next(x for x in rows if x['kind'] == 'counts')
            stages = [x for x in rows if x['kind'] == 'stage']
            solves = [x for x in stages if x['stage'] == 'w1_solve']
            first = [x for x in attempts if x['repeat'] == 0]
            outcome = outcomes[0]
            item = {
                'tier': tier, 'model': start['model'], 'run_id': record['run_id'],
                'archive': str(path), 'jsonl_member': member.name, 'record_member': record_member,
                'class': outcome['class'], 'reason': outcome['reason'],
                'selected_precision': outcome['selected_precision'], 'verification_precision': outcome['verification_precision'],
                'repeats': len(outcomes), 'lme_per_repeat': outcome['meter_charged'],
                'own_lme': sum(x['own_total'] for x in first),
                'shared_lme': sum(x['shared_work'] + x['verification_shared_work'] for x in first),
                'solve_median_s': statistics.median(x['elapsed_ns'] / 1e9 for x in solves),
                'solve_heap_requested_max_B': max(x['heap_peak'] for x in solves),
                'solve_heap_moving_max_B': max(x['heap_peak_move'] for x in solves),
                'matched_stage_names': sorted({x['stage'] for x in stages}),
                'stage_requested_max_B': max(x['heap_peak'] for x in stages),
                'stage_moving_max_B': max(x['heap_peak_move'] for x in stages),
                'whole_process_RSS_B': record['peak_rss_bytes'],
                'whole_process_footprint_B': record['rss']['time_peak_footprint_bytes'],
                'whole_process_wall_s': record['wall_s'], 'load_before': record['load_before'], 'load_after': record['load_after'],
                'conditional_full_H_estimate_B': counts['estimate_adm_bytes_w1a'],
                'heap_cap_B': start['heap_cap_bytes'], 'rss_cap_B': record['rss_cap_bytes'],
            }
            if tier == 'T4':
                item['first_repeat_precision_work'] = [{k: a[k] for k in ['precision', 'role', 'outcome', 'own_total', 'shared_work', 'verification_shared_work', 'charged_by', 'storage_pattern_entries', 'storage_profile_entries', 'storage_limbs_per_entry']} for a in first]
                item['prefixes'] = [x for x in rows if x['kind'] == 'prefix']
            processes.append(item)

assert len(processes) == 66
groups = []
for tier in ['T1', 'T2', 'T3', 'T4']:
    for family in ['RF-LARGE-CHAIN', 'RF-LARGE-TREE', 'RF-LARGE-CONT', 'DEC053']:
        q = [x for x in processes if x['tier'] == tier and x['model'].startswith(family)]
        if not q:
            continue
        fields = ['lme_per_repeat', 'solve_median_s', 'solve_heap_requested_max_B', 'solve_heap_moving_max_B', 'stage_requested_max_B', 'stage_moving_max_B', 'whole_process_RSS_B', 'whole_process_footprint_B', 'whole_process_wall_s', 'conditional_full_H_estimate_B']
        groups.append({'tier': tier, 'family': family, 'models': len({x['model'] for x in q}), 'processes': len(q), 'repeats': sum(x['repeats'] for x in q), 'classes': sorted({x['class'] + '/' + str(x['reason']) for x in q}), **{key: span(q, key) for key in fields}})

vr = []
vr_dir = Path('projects/chirality-piping/validation/benchmarks/numerical_robustness/observations/kernel_lane')
for absolute in sorted((root / vr_dir).glob('rf_*.json')):
    path = absolute.relative_to(root)
    cases = json.loads(read(path))
    charges = [x['invocation_charged'] for x in cases]
    maximum = max(cases, key=lambda x: x['invocation_charged'])
    vr.append({'path': str(path), 'family': cases[0]['family'], 'cases': len(cases), 'selected': sum(x.get('selected_precision') is not None for x in cases), 'classes': sorted({x['outcome'] for x in cases}), 'lme_min': min(charges), 'lme_median': statistics.median(charges), 'lme_max': max(charges), 'max_case': maximum['id'], 'precisions_reached': sorted({a['precision'] for x in cases for a in x['attempts']}), 'unresolved': [{'id': x['id'], 'outcome': x['outcome'], 'lme': x['invocation_charged']} for x in cases if x['outcome'].startswith('Unresolved')]})
assert sum(x['cases'] for x in vr) == 201

rss_path = r / 'REVIEW_RV34/t4_measurements_07/_run_records/ADMISSIONS_AND_RSS_MISSES.json'
admissions = json.loads(read(rss_path))
misses = []
for x in admissions:
    assert raw_rss[x['run_id']] == x['observed_RSS']
    if x['projection_missed']:
        actual, projected = x['observed_RSS'], x['projected_RSS']
        misses.append({'run_id': x['run_id'], 'observed_RSS_B': actual, 'projected_RSS_B': projected, 'excess_B': actual - projected, 'observed_over_projected': actual / projected})
assert len(misses) == 8

def distinct(tier, selected_only=False):
    result = {}
    for x in processes:
        if x['tier'] == tier and (not selected_only or x['class'] == 'Selected'):
            result.setdefault(x['model'], x)
    return list(result.values())

options = []
for name, rows, case_quantum, invocation_quantum, memory in [('A', distinct('T3'), 10**9, 10**10, 512*2**20), ('B', distinct('T4', True), 10**10, 10**10, 15*2**28), ('C', distinct('T4'), 10**10, 10**10, 15*2**28)]:
    maximum = max(x['lme_per_repeat'] for x in rows)
    total = sum(x['lme_per_repeat'] for x in rows)
    ceil = lambda x,q: ((x+q-1)//q)*q
    options.append({'id': name, 'basis_models': [x['model'] for x in rows], 'measured_case_max_LME': maximum, 'arithmetic_portfolio_sum_LME': total, 'case_rounding_quantum_LME': case_quantum, 'invocation_rounding_quantum_LME': invocation_quantum, 'proposed_case_LME': ceil(maximum,case_quantum), 'proposed_invocation_LME': ceil(total,invocation_quantum), 'proposed_complete_composed_memory_ceiling_B': memory, 'max_conditional_single_H_estimate_B': max(x['conditional_full_H_estimate_B'] for x in rows), 'memory_is_not_a_facade_bound': True, 'portfolio_is_not_a_measured_multicase_invocation': True})

result = {'schema': 'i29-w1-limits-preparation-v1', 'candidate': '9994462204231fb92073e17eb09775756bec22f3', 'maintained_source': '81c03849033f3ce745668f581f446530789397b8', 'units': {'LME': 'limb-multiply equivalents; current metered arithmetic only', 'heap': 'requested and moving stage windows, no precision-isolated peak', 'RSS_and_footprint': 'whole external process, five repeats plus actual prefixes/output', 'time': 'observations at recorded load; never a guarantee'}, 'normal_W1_processes': len(processes), 'normal_W1_repeats': sum(x['repeats'] for x in processes), 'attempt_charge_closure_checks': closure_checks, 'T4_raw_RSS_to_review_checks': len(admissions), 'coverage_groups': groups, 'current_VK_records': vr, 'rss_misses': misses, 'options': options, 'W1_process_rows': processes}
out.mkdir(parents=True, exist_ok=True)
(out/'CALCULATIONS.json').write_text(json.dumps(result, indent=2)+'\n')
(out/'CALCULATION_ORIGINS.json').write_text(json.dumps(origins, indent=2)+'\n')
print(json.dumps({'normal_W1_processes': len(processes), 'normal_W1_repeats': sum(x['repeats'] for x in processes), 'VK_cases': sum(x['cases'] for x in vr), 'options': options, 'rss_misses': len(misses), 'attempt_charge_closure_checks': closure_checks},indent=2))
