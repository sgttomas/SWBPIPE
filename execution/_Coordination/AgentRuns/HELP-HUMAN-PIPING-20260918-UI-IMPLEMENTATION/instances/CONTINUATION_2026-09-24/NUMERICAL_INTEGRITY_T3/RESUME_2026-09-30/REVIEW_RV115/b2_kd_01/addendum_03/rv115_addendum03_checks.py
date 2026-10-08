#!/usr/bin/env python3
"""RV115 ADDENDUM_03: S-4 (a)'s numerical content (standard library only).

Reads only argv[1] (DEF-C r1) and argv[2] (DEF-C v0) for a wording diff; imports
nothing from the repository; writes only to stdout.

1. The final gate on a combination displacement-magnitude row, with an exact
   (point) dual enclosure, i.e. the most favourable certificate: h is only the
   published value's distance from the exact norm. DEF-O's relative predicate
   (final_case.rs `gate`, SharperExact and SharperBinary64) is applied for scale
   S* = n, 4n, 64n and 2^13 n. Two recipes:
     old (DEF-C v0, DEF-O's displacement recipe): y = RN64(norm in mm), n = RN64(y/1000);
     new (DEF-C r1): p = hypot(hypot(x,y),z) of the published mm components
         x = RN64(q_x) ..., with hypot correctly rounded (CR) or an adversarial
         faithful 1-ulp library (FA, worst of RD/RU at each call), n = RN64(p/1000).
   Also: whether every component row passes, so a magnitude failure is a pure
   regression of the recipe.
2. The coverage bound: h_mag <= sqrt(3) max_k h_k + |p - ||x||| + SI rounding, checked.
3. The 64-epsilon guard between two independent faithful libraries (CR vs FA
   extremes), including subnormal triples, as a ratio of the allowance.
4. DEF-C r1 against v0: changed leaf paths and the new texts.
"""
import json, math, random, struct, sys
from fractions import Fraction as F

TWO = F(2)

def next_up(x):
    return math.nextafter(x, math.inf)

def next_down(x):
    return math.nextafter(x, -math.inf)

def cr_sqrt(s):
    """Nearest binary64 to sqrt(s), s a nonnegative Fraction, ties to even (normal range)."""
    if s == 0:
        return 0.0
    t = math.sqrt(float(s))
    for _ in range(8):
        lo_mid = (F(t) + F(next_down(t))) / 2
        hi_mid = (F(t) + F(next_up(t))) / 2
        if s < lo_mid * lo_mid:
            t = next_down(t)
        elif s > hi_mid * hi_mid:
            t = next_up(t)
        else:
            if s == lo_mid * lo_mid and (struct.unpack('<Q', struct.pack('<d', t))[0] & 1):
                t = next_down(t)
            if s == hi_mid * hi_mid and (struct.unpack('<Q', struct.pack('<d', t))[0] & 1):
                t = next_up(t)
            return t
    raise RuntimeError('cr_sqrt did not converge')

def dir_sqrt(s):
    """(RD, RU) binary64 neighbours of sqrt(s) (equal when exact)."""
    t = cr_sqrt(s)
    if F(t) * F(t) == s:
        return t, t
    return (t, next_up(t)) if F(t) * F(t) < s else (next_down(t), t)

def sqrt_hi(s, bits=400):
    """sqrt(s) to about 2^-bits relative, as a Fraction."""
    if s == 0:
        return F(0)
    k = bits - (s.numerator.bit_length() - s.denominator.bit_length()) // 2
    k = max(k, 0)
    num = s.numerator * (1 << (2 * k)) // s.denominator
    return F(math.isqrt(num), 1 << k)

def cr_hypot(a, b):
    return cr_sqrt(F(a) ** 2 + F(b) ** 2)

def fa_hypot_all(a, b):
    lo, hi = dir_sqrt(F(a) ** 2 + F(b) ** 2)
    return {lo, hi}

def allowance_exact(n, S):
    m = max(abs(F(n)), F(S))
    return m * TWO ** -64 + m * TWO ** -85 + abs(F(n)) * TWO ** -53 + F(5e-324)

def allowance_b64(n, S):
    a0 = 2.0 ** -64 * max(abs(n), S)
    a1 = a0 * struct.unpack('<d', struct.pack('<Q', 0x3ff0_0000_8000_0000))[0]
    u0 = 2.0 ** -53 * abs(n)
    u1 = u0 + 5e-324
    return a1 + u1

def passes(n, truth_si, S):
    h = abs(F(n) - truth_si)
    return h <= allowance_exact(n, S) and h <= F(allowance_b64(n, S))

rng = random.Random(115003)

def sample(kind):
    e = rng.randint(-10, 6)
    if kind == 'comparable':
        c = [rng.uniform(-1, 1) for _ in range(3)]
    elif kind == 'one_small':
        c = [rng.uniform(-1, 1), rng.uniform(-1, 1), rng.uniform(-1, 1) * 2.0 ** -rng.randint(4, 30)]
    else:  # 'planar'
        c = [rng.uniform(-1, 1), rng.uniform(-1, 1), 0.0]
    q = []
    for v in c:
        if v == 0.0:
            q.append(F(0))
        else:
            # an exact truth between binary64 neighbours (sub-ulp offset), in mm
            q.append(F(v * 2.0 ** e) * (1 + F(rng.randint(-2 ** 40, 2 ** 40), 2 ** 95)))
    return q

def study(kind, trials):
    stats = {S: {'old_fail': 0, 'cr_fail': 0, 'fa_fail': 0, 'cr3_fail': 0, 'cr_fail_components_pass': 0, 'any_component_fail': 0} for S in ('n', '4n', '64n', '8192n')}
    bound_ok = 0
    max_extra_ulps = {'cr': 0.0, 'fa': 0.0}
    for _ in range(trials):
        q = sample(kind)
        xs = [float(v) for v in q]                      # published component values, mm
        s_exact = sum(v * v for v in q)
        T_mm = sqrt_hi(s_exact)
        T_si = T_mm / 1000
        y_old = cr_sqrt(s_exact)
        n_old = float(F(y_old) / 1000)
        p_cr = cr_hypot(cr_hypot(xs[0], xs[1]), xs[2])
        n_cr = float(F(p_cr) / 1000)
        p_cr3 = cr_sqrt(sum(F(x) ** 2 for x in xs))          # one rounding of the exact 3-norm
        n_cr3 = float(F(p_cr3) / 1000)
        p_fa = set()
        for inner in fa_hypot_all(xs[0], xs[1]):
            p_fa |= fa_hypot_all(inner, xs[2])
        n_fa = [float(F(p) / 1000) for p in p_fa]
        n_fa_worst = max(n_fa, key=lambda v: abs(F(v) - T_si))
        # coverage bound (2): h_mag <= sqrt(3) max_k h_k + |p - ||x|| | + SI rounding
        comp_si = [float(F(x) / 1000) for x in xs]
        hk = [abs(F(c) - qq / 1000) for c, qq in zip(comp_si, q)]
        norm_x_mm = sqrt_hi(sum(F(x) ** 2 for x in xs))
        for tag, p, n in (('cr', p_cr, n_cr),):
            lhs = abs(F(n) - T_si)
            rhs = F(1.7320508075688773) * max(hk) * F(1 + 2 ** -50) + abs(F(p) - norm_x_mm) / 1000 + abs(F(n) - F(p) / 1000) + max(abs(F(x) / 1000 - c) for x, c in zip(xs, comp_si)) * 0
            # components' h_k are measured in SI after their own SI rounding, so add the
            # difference between mm/1000 and the SI component for a true bound:
            rhs += F(1.7320508075688773) * max(abs(F(x) / 1000 - F(c)) for x, c in zip(xs, comp_si))
            if lhs <= rhs:
                bound_ok += 1
        ulp = F(next_up(p_cr) - p_cr) if p_cr else F(0)
        if ulp:
            max_extra_ulps['cr'] = max(max_extra_ulps['cr'], float(abs(F(p_cr) - norm_x_mm) / ulp))
            max_extra_ulps['fa'] = max(max_extra_ulps['fa'], max(float(abs(F(p) - norm_x_mm) / ulp) for p in p_fa))
        for label, mult in (('n', 1), ('4n', 4), ('64n', 64), ('8192n', 8192)):
            S_old, S_new = n_old * mult, n_cr * mult
            comps_pass = all(passes(c, qq / 1000, S_new) for c, qq in zip(comp_si, q) if abs(c) >= 2.0 ** -34 * S_new)
            if not passes(n_old, T_si, S_old):
                stats[label]['old_fail'] += 1
            if not passes(n_cr, T_si, S_new):
                stats[label]['cr_fail'] += 1
                if comps_pass:
                    stats[label]['cr_fail_components_pass'] += 1
            if not passes(n_fa_worst, T_si, n_fa_worst * mult):
                stats[label]['fa_fail'] += 1
            if not passes(n_cr3, T_si, n_cr3 * mult):
                stats[label]['cr3_fail'] += 1
            if not comps_pass:
                stats[label]['any_component_fail'] += 1
    return {'trials': trials, 'per_scale': stats, 'coverage_bound_held': bound_ok,
            'max_|p-||x|||_in_ulps(p)': max_extra_ulps}

out = {'1_gate': {k: study(k, 3000) for k in ('comparable', 'one_small', 'planar')}}

# 3. the guard between two faithful libraries, extremes (CR vs adversarial FA), incl. subnormal
def guard_ratio(p, r):
    return abs(F(p) - F(r)) / (F(64) * F(2.0 ** -52) * max(abs(F(p)), F(2.0 ** -1022)))
worst = F(0)
worst_at = None
cases = 0
for _ in range(3000):
    band = rng.choice([(-1074, -1023), (-1060, -1000), (-60, 60), (900, 1000)])
    xs = [rng.choice([1, -1]) * rng.random() * 2.0 ** rng.randint(*band) for _ in range(3)]
    try:
        cand = set()
        for inner in fa_hypot_all(xs[0], xs[1]):
            cand |= fa_hypot_all(inner, xs[2])
    except (OverflowError, RuntimeError):
        continue
    if not all(math.isfinite(c) for c in cand):
        continue
    cases += 1
    for p in cand:
        for r in cand:
            g = guard_ratio(p, r)
            if g > worst:
                worst, worst_at = g, [x.hex() for x in xs]
out['3_guard_two_faithful_libraries'] = {'triples': cases, 'max_ratio_of_allowance': float(worst), 'at': worst_at}

# 4. DEF-C r1 against v0
r1, v0 = json.load(open(sys.argv[1])), json.load(open(sys.argv[2]))
def leaves(v, path=''):
    if isinstance(v, dict):
        o = {}
        for k in v:
            o.update(leaves(v[k], f'{path}/{k}'))
        return o
    return {path: json.dumps(v, sort_keys=True)}
a, b = leaves(v0), leaves(r1)
changed = sorted(p for p in set(a) | set(b) if a.get(p) != b.get(p))
out['4_defc_r1_vs_v0'] = {'changed_leaf_paths': changed, 'r1_texts': {p: json.loads(b[p]) if p in b else None for p in changed}}
import hashlib
def jcs(v):
    return json.dumps(v, ensure_ascii=False, separators=(',', ':'), sort_keys=True, allow_nan=False)
def H(d, v):
    return hashlib.sha256(jcs({'domain': d, 'payload': v}).encode('utf-8')).hexdigest()
raw_r1 = open(sys.argv[1], 'rb').read()
defo = json.load(open(sys.argv[3]))
out['5_hash'] = {'DEF_O_H_control': H('retained_precision_formation_v1', defo),
                 'DEF_C_r1_raw_sha256': hashlib.sha256(raw_r1).hexdigest(),
                 'DEF_C_r1_raw_is_canonical': jcs(r1).encode('utf-8') == raw_r1,
                 'DEF_C_r1_H': H('retained_precision_formation_v1', r1),
                 'DEF_C_r1_H_alternative_domain': H('retained_precision_combination_formation_v1', r1),
                 'operand_definition_is_H_DEF_O': r1['inherits']['operand_definition']['sha256'] == H('retained_precision_formation_v1', defo),
                 'support_magnitude_equals_DEF_O': r1['rows']['support_magnitude'] == defo['rows']['support_magnitude']}
# 6. an explicit mm->SI double-rounding example (point enclosure, S* = n): DEF-O's projection alone
ex = None
for k in range(1, 200000):
    t = F(k, 7919) * F(1, 1000) * (1 + F(1, 3 * 2 ** 70))
    y = float(t * 1000); n = float(F(y) / 1000)
    if abs(F(n) - t) > allowance_exact(n, n):
        ex = {'t_m': str(t), 'y_mm': y.hex(), 'n_si': n.hex(), 'h_over_allowance': float(abs(F(n) - t) / allowance_exact(n, n))}
        break
out['6_mm_si_double_rounding_example'] = ex
print(json.dumps(out, indent=1, sort_keys=True, ensure_ascii=True))
