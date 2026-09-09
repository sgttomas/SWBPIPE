import json, sys
from decimal import Decimal, getcontext
from fractions import Fraction
from itertools import combinations
getcontext().prec = 60
source, output = sys.argv[1:]
rows = [json.loads(line, parse_float=Decimal) for line in open(source, encoding='utf-8')]
by = {(row['fixture'], row['variant'], row['mode']): row for row in rows}
integer_probes = [0, 1, -1, 2, -2, 3, -3, 5, -5, 7, -7, 10, -10, 17, -17, 23, -23, 37, -37, 50, -50]
fractional_l200 = ['0.123456', '-0.123456', '0.314159', '-0.314159', '0.707107', '-0.707107', '1.414213', '-1.414213', '2.718282', '-2.718282', '3.141593', '-3.141593', '6.283185', '-6.283185', '11.111111', '-11.111111', '19.876543', '-19.876543', '31.415927', '-31.415927', '47.123456', '-47.123456', '49.885', '-49.89', '49.8703', '-49.8805']
eps = Fraction(1, 2_000_000)
mu = Fraction(1, 100)

def frac(value):
    return Fraction(str(value))

def variant_for(case, x):
    if x == 0:
        return 'base_mu0'
    text = str(Decimal(x.numerator) / Decimal(x.denominator))
    if case == 'l200' and text in fractional_l200:
        return f'probe_l200_fractional_{fractional_l200.index(text):02d}_mu0'
    sign = 'plus' if x > 0 else 'minus'
    return f'probe_{case}_{sign}{abs(x.numerator)}_mu0'

def publish6(value):
    scaled = value * 1_000_000
    whole, remainder = divmod(scaled.numerator, scaled.denominator)
    if 2 * remainder >= scaled.denominator:
        whole += 1
    return Fraction(whole, 1_000_000)

def decimal(value):
    return str(Decimal(value.numerator) / Decimal(value.denominator))

results = []
for fixture in ('retained_spring', 'historical_no_spring'):
    for case in ('l100', 'l200'):
        probes = [Fraction(x) for x in integer_probes]
        if case == 'l200':
            probes += [Fraction(x) for x in fractional_l200]
        observations = [(x, frac(by[(fixture, variant_for(case, x), 'dense_scrutiny')][f'{case}_normal_n'])) for x in probes]
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
        fixed = [base / (1 - mu * influence) for base, influence in vertices]
        friction = [mu * value for value in fixed]
        normal_cells = sorted({publish6(min(fixed)), publish6(max(fixed))})
        friction_cells = sorted({publish6(min(friction)), publish6(max(friction))})
        assert len(friction_cells) == 1
        target = by[(fixture, 'target_mu001', 'dense_scrutiny')]
        observed_normal = frac(target[f'{case}_normal_n'])
        observed_friction = frac(target[f'{case}_friction_n'])
        assert observed_normal in normal_cells
        assert observed_friction == friction_cells[0]
        results.append({
            'fixture': fixture,
            'load_case': case.upper().replace('L', 'L-'),
            'arithmetic': 'exact_fraction',
            'probe_count': len(observations),
            'affine_equation': 'N=N0+Hq; q=mu*N; N*=N0/(1-mu*H)',
            'bounded_N0_fraction': [str(min(base for base, _ in vertices)), str(max(base for base, _ in vertices))],
            'bounded_H_fraction': [str(min(h for _, h in vertices)), str(max(h for _, h in vertices))],
            'bounded_fixed_normal_decimal': [decimal(min(fixed)), decimal(max(fixed))],
            'bounded_fixed_friction_decimal': [decimal(min(friction)), decimal(max(friction))],
            'independent_published_normal_cells_n': [decimal(x) for x in normal_cells],
            'independent_published_friction_n': decimal(friction_cells[0]),
            'observed_target_normal_n': decimal(observed_normal),
            'observed_target_friction_n': decimal(observed_friction),
        })
payload = {
    'method': 'Exact Fraction intersections of six-decimal mu=0 public normal observations under independent global-Z probes; target mu=0.01 is observational only.',
    'calibration': 'Both L-200 normal bounds span two publication cells; their exact target normal is therefore observation-backed. Both L-200 friction publications and both L-100 pairs are uniquely fixed by the independent affine bounds.',
    'results': results,
}
with open(output, 'w', encoding='utf-8', newline='\n') as f:
    json.dump(payload, f, indent=2, sort_keys=True); f.write('\n')
