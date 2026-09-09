import json, sys
from decimal import Decimal, getcontext, ROUND_HALF_UP
from itertools import combinations
getcontext().prec = 50
source, output = sys.argv[1:]
rows = [json.loads(line) for line in open(source, encoding='utf-8')]
assert all(row['mechanics'] == 'MECHANICS_SOLVED' for row in rows)
by = {(row['fixture'], row['variant'], row['mode']): row for row in rows}
variants = {row['variant'] for row in rows}
for fixture in ('retained_spring', 'historical_no_spring'):
    for variant in variants:
        dense = {k: v for k, v in by[(fixture, variant, 'dense_scrutiny')].items() if k not in ('mode', 'diagnostic_codes')}
        sparse = {k: v for k, v in by[(fixture, variant, 'sparse_interactive')].items() if k not in ('mode', 'diagnostic_codes')}
        assert dense == sparse
integer_probes = [Decimal(0)] + [Decimal(x) for x in (1, -1, 2, -2, 3, -3, 5, -5, 7, -7, 10, -10, 17, -17, 23, -23, 37, -37, 50, -50)]
fractional_l200 = [Decimal(x) for x in ('0.123456', '-0.123456', '0.314159', '-0.314159', '0.707107', '-0.707107', '1.414213', '-1.414213', '2.718282', '-2.718282', '3.141593', '-3.141593', '6.283185', '-6.283185', '11.111111', '-11.111111', '19.876543', '-19.876543', '31.415927', '-31.415927', '47.123456', '-47.123456', '49.885', '-49.89', '49.8703', '-49.8805')]
eps = Decimal('0.0000005')
mu = Decimal('0.01')

def variant_for(case, x):
    if x == 0:
        return 'base_mu0'
    if case == 'l200' and x in fractional_l200:
        return f'probe_l200_fractional_{fractional_l200.index(x):02d}_mu0'
    sign = 'plus' if x > 0 else 'minus'
    return f'probe_{case}_{sign}{abs(int(x))}_mu0'

def rounded_cells(low, high):
    cells = {low.quantize(Decimal('0.000001'), rounding=ROUND_HALF_UP), high.quantize(Decimal('0.000001'), rounding=ROUND_HALF_UP)}
    return sorted(cells)

results = []
for fixture in ('retained_spring', 'historical_no_spring'):
    for case in ('l100', 'l200'):
        probes = integer_probes + (fractional_l200 if case == 'l200' else [])
        observations = []
        for x in probes:
            value = Decimal(str(by[(fixture, variant_for(case, x), 'dense_scrutiny')][f'{case}_normal_n']))
            observations.append((x, value))
        boundaries = [(x, value + offset) for x, value in observations for offset in (-eps, eps)]
        vertices = []
        for (x1, y1), (x2, y2) in combinations(boundaries, 2):
            if x1 == x2:
                continue
            influence = (y1 - y2) / (x1 - x2)
            base = y1 - influence * x1
            if all(value - eps <= base + influence * x <= value + eps for x, value in observations):
                vertices.append((base, influence))
        assert vertices
        fixed = [base / (Decimal(1) - mu * influence) for base, influence in vertices]
        friction = [mu * value for value in fixed]
        normal_cells = rounded_cells(min(fixed), max(fixed))
        friction_cells = rounded_cells(min(friction), max(friction))
        assert len(friction_cells) == 1
        target = by[(fixture, 'target_mu001', 'dense_scrutiny')]
        observed_normal = Decimal(str(target[f'{case}_normal_n']))
        observed_friction = Decimal(str(target[f'{case}_friction_n']))
        assert observed_normal in normal_cells
        assert observed_friction == friction_cells[0]
        results.append({
            'fixture': fixture,
            'load_case': case.upper().replace('L', 'L-'),
            'mu': str(mu),
            'affine_equation': 'N = N0 + H*q; q = mu*N; N* = N0/(1-mu*H)',
            'probe_count': len(observations),
            'bounded_N0_n': [str(min(base for base, _ in vertices)), str(max(base for base, _ in vertices))],
            'bounded_H_normal_n_per_tangential_n': [str(min(influence for _, influence in vertices)), str(max(influence for _, influence in vertices))],
            'bounded_fixed_point_normal_n': [str(min(fixed)), str(max(fixed))],
            'bounded_fixed_point_friction_n': [str(min(friction)), str(max(friction))],
            'independent_published_normal_candidates_n': [str(value) for value in normal_cells],
            'independent_published_friction_n': str(friction_cells[0]),
            'observed_target_published_normal_n': str(observed_normal),
            'observed_target_published_friction_n': str(observed_friction),
            'dense_sparse_equal': True,
            'stable_friction_state_code': target[f'{case}_friction_state'],
            'stable_one_way_state_code': target[f'{case}_one_way_state'],
            'iterations': target[f'{case}_iterations'],
            'converged': target[f'{case}_converged'],
        })
payload = {
    'method': 'Existing public API at mu=0 with explicit independent global-Z nodal probes. Six-decimal probe intervals are intersected before the affine fixed-point calculation. The mu=0.01 run is used only to observe the published target and backcheck the independently bounded relation.',
    'publication_half_interval_n': str(eps),
    'dense_sparse_all_rows_equal': True,
    'calibration': 'The retained-spring L-200 normal interval straddles one final-digit publication boundary; its exact public value is therefore observational, while its friction publication is independently unique. All other listed publications are independently unique.',
    'results': results,
}
with open(output, 'w', encoding='utf-8', newline='\n') as f:
    json.dump(payload, f, indent=2, sort_keys=True)
    f.write('\n')
