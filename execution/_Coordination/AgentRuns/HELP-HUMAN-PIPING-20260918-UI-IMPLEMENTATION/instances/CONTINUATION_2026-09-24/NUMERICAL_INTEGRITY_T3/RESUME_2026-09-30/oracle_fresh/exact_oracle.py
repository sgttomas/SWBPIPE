#!/usr/bin/env python3
"""Independent exact axial/torsional oracle. Standard library; no solver imports.

Usage: python3 exact_oracle.py freeze DIRECTORY
       python3 exact_oracle.py compare DIRECTORY OUTPUT_TSV REPORT_JSON
Freeze reads only inputs/cases.rs. Compare requires a previously frozen TRUTH.json.
"""
import hashlib
import json
import re
import sys
from fractions import Fraction as F
from pathlib import Path


def p2(e):
    return F(2**e) if e >= 0 else F(1, 2**(-e))


H = p2(-1074)
SMALL = p2(-988)
MAX_BITS = 0x7fefffffffffffff
SIGN = 1 << 63
INF = 0x7ff0000000000000
KINDS = ('Translation', 'Rotation', 'Force', 'Moment')


def value(bits):
    """Decode finite IEEE 754 bits to a rational, with no host float arithmetic."""
    if isinstance(bits, str):
        bits = int(bits, 16)
    sign = -1 if bits & SIGN else 1
    e = (bits >> 52) & 2047
    f = bits & ((1 << 52) - 1)
    if e == 2047:
        raise ValueError('nonfinite binary64')
    return sign * (f * H if not e else (2**52 + f) * p2(e - 1075))


def round_bits(x):
    """Exact nearest-even using ordered binary64 bit-pattern bisection.

    The virtual successor to MAX is 2^1024; the overflow midpoint ties to INF.
    A nonzero magnitude at/below H/2 can round to zero (range=Underflow).
    """
    x = F(x)
    sign = SIGN if x < 0 else 0
    a = abs(x)
    if a == 0:
        return 0
    if a >= (value(MAX_BITS) + p2(1024)) / 2:
        return sign | INF
    lo, hi = 0, MAX_BITS
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if value(mid) <= a:
            lo = mid
        else:
            hi = mid - 1
    if lo == MAX_BITS or value(lo) == a:
        return sign | lo
    left, right = a - value(lo), value(lo + 1) - a
    chosen = lo if left < right or (left == right and lo % 2 == 0) else lo + 1
    return sign | chosen


def nearest(x):
    return value(round_bits(x))


def upward_bits(x):
    assert x >= 0
    b = round_bits(x)
    if b == INF:
        return b
    return b + (value(b) < x)


def encoded(x):
    """Lossless compact rational encoding; these inputs yield dyadic results."""
    x = F(x)
    if x == 0:
        return {'significand': '0', 'exponent2': 0}
    n, d = x.numerator, x.denominator
    if d & (d - 1):
        return {'numerator': str(n), 'denominator': str(d)}
    exponent = -(d.bit_length() - 1)
    while n % 2 == 0:
        n //= 2
        exponent += 1
    return {'significand': str(n), 'exponent2': exponent}


def decoded(x):
    if 'significand' in x:
        return F(int(x['significand'])) * p2(x['exponent2'])
    return F(int(x['numerator']), int(x['denominator']))


def outcome(x):
    b = round_bits(x)
    a = b & (SIGN - 1)
    if a == INF:
        return 'Overflow', None
    if not a and x:
        return 'Underflow', None
    return 'Value', b


def cell(x):
    """Direct-rounding cell. Endpoints denote numerical RN-even rounding.

    Zero from an exact zero is publishable; a nonzero in the zero cell is an
    Underflow outcome instead. Source API therefore distinguishes their classes.
    """
    b = round_bits(x)
    a = b & (SIGN - 1)
    if a == INF:
        edge = (value(MAX_BITS) + p2(1024)) / 2
        return {'overflow_midpoint': encoded(-edge if x < 0 else edge)}
    y = value(a)
    lower = (value(a - 1) + y) / 2 if a else -H / 2
    upper = (y + (value(a + 1) if a < MAX_BITS else p2(1024))) / 2
    if x < 0:
        lower, upper = -upper, -lower
    return {'lower': encoded(lower), 'upper': encoded(upper),
            'endpoints_included': a % 2 == 0}


def parse_cases(text):
    pat = (r'Spec \{ id: "([BC]\d\d)", length: (0x[0-9a-f]+), '
           r'area: (0x[0-9a-f]+), torsion: (0x[0-9a-f]+), '
           r'spring: \((\d+), (0x[0-9a-f]+)\), free: &\[([^]]*)\], '
           r'loads: &\[([^]]*)\] \}')
    cases = []
    for m in re.finditer(pat, text):
        cid, length, area, torsion, sdof, spring, free, loads = m.groups()
        terms = [(int(g), bits, sid) for g, bits, sid in
                 re.findall(r'\((\d+), (0x[0-9a-f]+), "([^"]+)"\)', loads)]
        cases.append({'id': cid, 'length_bits': length[2:], 'area_bits': area[2:],
                      'torsion_bits': torsion[2:], 'spring_dof': int(sdof),
                      'spring_bits': spring[2:],
                      'free_dofs': [int(s.strip()) for s in free.split(',') if s.strip()],
                      'loads': [{'dof': g, 'bits': b[2:], 'source_id': sid}
                                for g, b, sid in terms]})
    assert len(cases) == 24 and len({c['id'] for c in cases}) == 24
    return cases


def solve_exact(c):
    L, A, J = (value(c[k]) for k in ('length_bits', 'area_bits', 'torsion_bits'))
    k = value(c['spring_bits'])
    a, t = A / L, J / L  # E=G=1; axes are exactly the identity.
    f = [F(0)] * 12
    for term in c['loads']:
        f[term['dof']] += value(term['bits'])
    u = [F(0)] * 12
    free = set(c['free_dofs'])
    if c['id'].startswith('B'):
        assert len(free) == 1 and k == 0
        g = next(iter(free))
        assert g in (6, 9)
        u[g] = f[g] / (a if g == 6 else t)
    else:
        assert free == {0, 6, 9} and k > 0
        s = c['spring_dof']
        assert s in (0, 6)
        # Sum the two axial equilibrium equations: k*u_s=f_0+f_6.
        u[s] = (f[0] + f[6]) / k
        if s == 0:
            u[6] = u[0] + f[6] / a
        else:
            u[0] = u[6] + f[0] / a
        u[9] = f[9] / t
    N, T = a * (u[6] - u[0]), t * (u[9] - u[3])
    internal = [F(0)] * 12
    internal[0], internal[6] = -N, N
    internal[3], internal[9] = -T, T
    if k:
        internal[c['spring_dof']] += k * u[c['spring_dof']]
    assert all(internal[g] == f[g] for g in free)
    rows = []
    def add(key, kind, truth, derived=False):
        rows.append({'key': key, 'kind': kind, 'input_derived': derived, 'truth': truth})
    for g in range(12):
        add('D:'+str(g), KINDS[0 if g % 6 < 3 else 1], u[g], g not in free)
    # Only UX can be nonzero: Euclidean translation magnitude is exactly |UX|.
    add('M:0', 'Translation', abs(u[0]))
    add('M:1', 'Translation', abs(u[6]))
    for end, sign in [('I', -1), ('J', 1)]:
        for component in range(6):
            add(f'E:1:{end}:{component}', KINDS[2 if component < 3 else 3],
                sign * N if component == 0 else sign * T if component == 3 else F(0))
    if k:
        add('S:1:0', 'Force', -k * u[c['spring_dof']])
    for g in range(12):
        if g not in free:
            add('R:'+str(g), KINDS[2 if g % 6 < 3 else 3], internal[g] - f[g])
    assert len(rows) == (37 if not k else 36)
    return rows, {'length': L, 'axial_stiffness': a, 'torsional_stiffness': t,
                  'spring_stiffness': k, 'axial_action': N, 'torsional_action': T,
                  'ledger_load_0': f[0], 'ledger_load_6': f[6], 'ledger_load_9': f[9]}


def scales(rows, L, floor=None):
    raw = {kind: F(0) for kind in KINDS}
    for r in rows:
        if not r['input_derived'] and r['value'] is not None:
            raw[r['kind']] = max(raw[r['kind']], abs(r['value']))
    tr, ro, fo, mo = (raw[k] for k in KINDS)
    coupled = [max(tr, nearest(L*ro)), max(ro, nearest(tr/L)),
               max(fo, nearest(mo/L)), max(mo, nearest(L*fo))]
    if floor:
        coupled[2], coupled[3] = max(coupled[2], floor[0]), max(coupled[3], floor[1])
    return raw, dict(zip(KINDS, coupled))


def classification(q, scale, input_derived=False):
    if q is None:
        return 'Unpublishable', None
    if input_derived:
        return 'InputDerived', None
    if scale < SMALL or abs(q) < nearest(p2(-34) * scale):
        bound = value(upward_bits(p2(-64) * scale))
        if 0 < scale < SMALL:
            bound = value(upward_bits(bound + value(upward_bits(p2(-53)*abs(q))) + H))
        return 'AbsoluteVerified', round_bits(bound)
    return 'RelativeVerified', None


def allowances(q, s, cls, bound, precision):
    factor = 1 + p2(-21 if precision == 512 else -22)
    if cls == 'AbsoluteVerified':
        exact = bound * factor
        source = nearest(bound * factor)
    elif cls == 'RelativeVerified':
        exact = p2(-64) * max(abs(q), s) * (1+p2(-21)) + p2(-53)*abs(q) + H
        # Rust source evaluates each binary64 operation in written order.
        source = nearest(nearest(nearest(p2(-64)*max(abs(q), s))*(1+p2(-21))) +
                         nearest(nearest(p2(-53)*abs(q)) + H))
    elif cls == 'InputDerived':
        exact = p2(-53)*abs(q) + H
        source = nearest(nearest(p2(-53)*abs(q)) + H)
    else:
        return None
    return exact, source


def freeze(directory):
    cases = parse_cases((directory/'inputs/cases.rs').read_text())
    result = {'schema': 'a1-independent-truth-v1', 'basis': 'd01ad98a754698631f927709d08284c272de85e8',
              'case_source_sha256': hashlib.sha256((directory/'inputs/cases.rs').read_bytes()).hexdigest(),
              'baseline': 'exact truth directly rounded, before any p=512 Force/Moment floor; not a prediction of solver selection',
              'cases': []}
    for c in cases:
        rs, terms = solve_exact(c)
        for r in rs:
            oc, b = outcome(r['truth'])
            r.update(outcome=oc, bits=b, value=None if b is None else value(b))
        raw, sc = scales(rs, terms['length'])
        outrows = []
        for r in rs:
            cls, bound = classification(r['value'], sc[r['kind']], r['input_derived'])
            outrows.append({'key': r['key'], 'kind': r['kind'], 'input_derived': r['input_derived'],
                            'truth': encoded(r['truth']), 'direct_outcome': r['outcome'],
                            'direct_bits': None if r['bits'] is None else f"{r['bits']:016x}",
                            'rounding_cell': cell(r['truth']), 'direct_class': cls,
                            'direct_bound_bits': None if bound is None else f'{bound:016x}',
                            'o9_included_in_direct_scale': not r['input_derived'] and r['bits'] is not None})
        result['cases'].append({'id': c['id'], 'primitive': c,
                                'derived_equation_terms': {k: encoded(v) for k,v in terms.items()},
                                'direct_raw_scales': {k: encoded(v) for k,v in raw.items()},
                                'direct_coupled_scale_bits': {k: f'{round_bits(v):016x}' for k,v in sc.items()},
                                'rows': outrows})
    target = directory/'TRUTH.json'
    if target.exists():
        raise FileExistsError('frozen truth is never overwritten')
    target.write_text(json.dumps(result, indent=2)+'\n')
    print(json.dumps({'truth_sha256': hashlib.sha256(target.read_bytes()).hexdigest(),
                      'case_count': len(cases), 'row_count': sum(len(c['rows']) for c in result['cases'])}))


def compare(directory, tsv, report):
    """Compare only after ROOT releases output. Never changes frozen truth."""
    truth = json.loads((directory/'TRUTH.json').read_text())
    records = [line.split('\t') for line in tsv.read_text().splitlines()]
    select = lambda tag: [p[1:] for p in records if p[0] == tag]
    assert select('FORMAT') == [['a1-public-tsv-v1']]
    cases = select('CASE'); assert len(cases) == 1
    case = next(c for c in truth['cases'] if c['id'] == cases[0][0])
    status = select('STATUS'); assert len(status) == 1
    summary = {'schema': 'a1-independent-comparison-v1', 'case': case['id'],
               'output_sha256': hashlib.sha256(tsv.read_bytes()).hexdigest(),
               'truth_sha256': hashlib.sha256((directory/'TRUTH.json').read_bytes()).hexdigest(),
               'status': status[0][0], 'failures': [], 'rows': []}
    if status[0][0] != 'Selected':
        summary['claim_boundary'] = 'No publication claimed; no accuracy pass inferred.'
    else:
        selected = select('SELECTED'); assert len(selected) == 1
        p = int(selected[0][0]); assert p in (128,256,512) and int(selected[0][1]) == 2*p
        observations = select('ROW')
        assert len({r[0] for r in observations}) == len(observations)
        exp = {r['key']: r for r in case['rows']}
        assert set(r[0] for r in observations) == set(exp)
        observed = []
        for key, kind, oc, b, cls, bound in observations:
            e = exp[key]
            assert kind == e['kind']
            q = None if b == 'none' else value(b)
            observed.append({'key': key, 'kind': kind, 'value': q, 'input_derived': e['input_derived'],
                             'bits': None if b == 'none' else b,
                             'outcome': oc, 'class': cls, 'bound': None if bound == 'none' else value(bound)})
        floor_records = select('FLOOR')
        floor_map = dict(floor_records)
        if p == 512:
            assert set(floor_map) == {'Force','Moment'}
            floor = [value(floor_map[k]) for k in ('Force','Moment')]
        else:
            assert not floor_records
            floor = None
        _, sc = scales(observed, decoded(case['derived_equation_terms']['length']), floor)
        scale_records = select('SCALE')
        assert len(scale_records) == 4 and len(dict(scale_records)) == 4
        for kind,b in scale_records:
            if round_bits(sc[kind]) != int(b,16): summary['failures'].append('scale:'+kind)
        layouts = select('LAYOUT')
        assert len(layouts) == len(exp) and len({r[0] for r in layouts}) == len(exp)
        for key,kind,body,derived in layouts:
            if (kind,body,derived) != (exp[key]['kind'],'0',str(exp[key]['input_derived']).lower()):
                summary['failures'].append('layout:'+key)
        for r in observed:
            key = r['key']; e = exp[key]; x = decoded(e['truth']); q = r['value']
            cls, b = classification(q, sc[r['kind']], r['input_derived'])
            rr = {'key':key, 'truth':e['truth'], 'class_expected_from_publication':cls,
                  'direct_rounding_matches': r['bits'] == e['direct_bits'] and r['outcome']==e['direct_outcome']}
            if cls != r['class'] or (None if b is None else value(b)) != r['bound']:
                summary['failures'].append('classification:'+key)
            if q is None:
                ok = r['outcome'] == e['direct_outcome'] and r['outcome'] in ('Underflow','Overflow')
                rr['range_consistent'] = ok
                if not ok: summary['failures'].append('range:'+key)
            else:
                assert r['outcome'] == 'Value'
                if q == 0 and r['bits'] != '0000000000000000':
                    summary['failures'].append('noncanonical-zero:'+key)
                err = abs(q-x)
                aa = allowances(q,sc[r['kind']],cls,None if b is None else value(b),p)
                if aa is None:
                    summary['failures'].append('value-without-claim:'+key)
                else:
                    exact, source = aa
                    rr.update(error=encoded(err), exact_allowance=encoded(exact), source_binary64_allowance=encoded(source),
                              exact_predicate_pass=err<=exact, source_binary64_predicate_pass=err<=source)
                    if cls == 'RelativeVerified':
                        public_allowance = abs(q) / 10**9
                        truth_denominator_allowance = abs(x) / 10**9
                        rr.update(public_relative_allowance=encoded(public_allowance),
                                  public_relative_predicate_pass=err<=public_allowance,
                                  truth_denominator_relative_allowance=encoded(truth_denominator_allowance),
                                  truth_denominator_relative_predicate_pass=err<=truth_denominator_allowance)
                        if err > public_allowance:
                            summary['failures'].append('public-relative-claim:'+key)
                    if cls == 'InputDerived':
                        rr['prescribed_value_exact'] = q == x
                        if q != x: summary['failures'].append('prescription:'+key)
                    if err > exact: summary['failures'].append('exact-claim:'+key)
                    if err > source: summary['failures'].append('source-claim:'+key)
            summary['rows'].append(rr)
        summary['claim_boundary'] = 'Exact per-row truth, publication classes/scales and accepted claim predicates only; no design closure or host admission.'
    report.write_text(json.dumps(summary,indent=2)+'\n')
    print(json.dumps({'case':summary['case'], 'status':summary['status'], 'failures':summary['failures'], 'compared_rows':len(summary['rows'])}))


def self_check():
    assert round_bits(H/2) == 0 and round_bits(3*H/2) == 2 and round_bits(5*H/4) == 1
    assert outcome(0) == ('Value',0) and outcome(H/2) == ('Underflow',None)
    assert round_bits(p2(-1022)) == 0x0010000000000000
    assert round_bits(1+p2(-53)) == 0x3ff0000000000000
    assert round_bits(1+3*p2(-53)) == 0x3ff0000000000002
    assert round_bits((value(MAX_BITS)+p2(1024))/2) == INF
    for b in (1,2,3,0x000fffffffffffff,0x0010000000000000,0x3ff0000000000000,MAX_BITS):
        assert round_bits(value(b)) == b and round_bits(-value(b)) == (SIGN|b)
    assert classification(F(0),F(0)) == ('AbsoluteVerified',0)
    assert classification(H,H)[1] == 3


if __name__ == '__main__':
    self_check()
    mode, directory = sys.argv[1], Path(sys.argv[2])
    if mode == 'freeze':
        freeze(directory)
    elif mode == 'compare':
        compare(directory,Path(sys.argv[3]),Path(sys.argv[4]))
    else:
        raise SystemExit('mode must be freeze or compare')
