#!/usr/bin/env python3
"""T1_WP6_FREEZE independent recomputation (non-author freeze checker).

Run from WORKING_ROOT (projects/chirality-piping):

    python <LSI>/T1_WP6_FREEZE/_run_records/independent_recompute.py

It does NOT import or call generate_reference_values.py. It evaluates every
`exact` with its own arithmetic (fractions.Fraction; pi by Machin's formula on
integers; exp by an exact rational Taylor series with an explicit remainder
bound), applies the declared transform by this checker's own reading of the
reference sign conventions, applies the exact m->mm factor, rounds once to
binary64 (Fraction.__float__, correctly rounded; for pi-bearing values both
pi bounds must round to the same double) and compares bitwise with the
package reference files. It also recomputes every zero-scale absolute, checks
rule shape (relative/absolute) per expectation, checks that each negative is
outside the tolerance of its positive, and cross-checks key reference values
from first principles (inputs + physics), independent of the `exact` fields.

It runs no product, no runner, and reads no observed value.
"""
from __future__ import annotations

import json
import re
import sys
from fractions import Fraction
from pathlib import Path

ROOT = Path('.')
PKG = ROOT / 'validation/qualification/fixtures/load_reference'
REF = ROOT / 'core/product_physics/tests/fixtures/load_reference_states/reference_cases.json'

# ---------------------------------------------------------------- arithmetic

def _atan_inv(n: int, scale: int) -> int:
    """floor-ish scale*atan(1/n) by the alternating series on integers."""
    total = 0
    term = scale // n
    k = 0
    n2 = n * n
    while term:
        total += term // (2 * k + 1) if k % 2 == 0 else -(term // (2 * k + 1))
        term //= n2
        k += 1
    return total


def pi_bounds(digits: int = 90) -> tuple[Fraction, Fraction]:
    scale = 10 ** (digits + 10)
    p = 16 * _atan_inv(5, scale) - 4 * _atan_inv(239, scale)
    err = 10 ** 6  # generous integer-truncation error bound in units of 1/scale
    return Fraction(p - err, scale), Fraction(p + err, scale)


PI_LO, PI_HI = pi_bounds()


def exp_bounds(x: Fraction, terms: int = 40) -> tuple[Fraction, Fraction]:
    assert abs(x) < 1
    s = Fraction(0)
    t = Fraction(1)
    for k in range(terms):
        s += t
        t = t * x / (k + 1)
    # |remainder| <= |t| * e (|x|<1) ; t is the first omitted term
    r = abs(t) * 3
    return s - r, s + r


class Interval:
    def __init__(self, lo: Fraction, hi: Fraction | None = None):
        self.lo = lo
        self.hi = lo if hi is None else hi
        if self.lo > self.hi:
            self.lo, self.hi = self.hi, self.lo

    def __neg__(self):
        return Interval(-self.hi, -self.lo)

    def __add__(self, o):
        o = o if isinstance(o, Interval) else Interval(Fraction(o))
        return Interval(self.lo + o.lo, self.hi + o.hi)

    def __sub__(self, o):
        o = o if isinstance(o, Interval) else Interval(Fraction(o))
        return self + (-o)

    def __radd__(self, o):
        return self + o

    def __rsub__(self, o):
        return (-self) + o

    def __rmul__(self, o):
        return self * o

    def __rtruediv__(self, o):
        return Interval(Fraction(o)) / self

    def __mul__(self, o):
        o = o if isinstance(o, Interval) else Interval(Fraction(o))
        c = [self.lo * o.lo, self.lo * o.hi, self.hi * o.lo, self.hi * o.hi]
        return Interval(min(c), max(c))

    def __truediv__(self, o):
        o = o if isinstance(o, Interval) else Interval(Fraction(o))
        assert o.lo > 0 or o.hi < 0
        return self * Interval(1 / o.hi, 1 / o.lo)

    def to_float(self) -> float:
        a, b = float(self.lo), float(self.hi)
        if a != b:
            raise ValueError(f'binary64 rounding not decided: {a!r} vs {b!r}')
        return a

    def is_zero(self) -> bool:
        return self.lo == 0 and self.hi == 0


PI = Interval(PI_LO, PI_HI)
SYM = re.compile(r'^(?:(exp)\((-?\d+)/(\d+)\)-1|1-(exp)\((-?\d+)/(\d+)\))$')


def eval_exact(q: dict) -> Interval:
    e = q['exact']
    if isinstance(e, (int, float, str)) and not isinstance(e, dict):
        raise ValueError(f'unexpected exact form {e!r}')
    kind = e['kind']
    if kind == 'rational':
        return Interval(Fraction(e['rational']))
    if kind == 'rational_times_pi':
        return PI * Fraction(e['rational'])
    if kind == 'rational_over_pi':
        return Interval(Fraction(e['rational'])) / PI
    if kind == 'symbolic':
        m = SYM.match(e['expression'].replace(' ', ''))
        if not m:
            raise ValueError(f'unknown symbolic {e}')
        if m.group(1):
            lo, hi = exp_bounds(Fraction(int(m.group(2)), int(m.group(3))))
            return Interval(lo - 1, hi - 1)
        lo, hi = exp_bounds(Fraction(int(m.group(5)), int(m.group(6))))
        return Interval(1 - hi, 1 - lo)
    raise ValueError(f'unknown exact kind {kind}')


def resolve(doc, pointer: str):
    cur = doc
    for part in pointer.lstrip('/').split('/'):
        part = part.replace('~1', '/').replace('~0', '~')
        cur = cur[int(part)] if isinstance(cur, list) else cur[part]
    return cur


# ---------------------------------------------------------------- checks

class Counter:
    def __init__(self):
        self.ok = 0
        self.fail: list[str] = []

    def check(self, cond: bool, msg: str):
        if cond:
            self.ok += 1
        else:
            self.fail.append(msg)


UNIT_FACTOR = {('m', 'mm'): Fraction(1000)}


def unit_factor(ref_unit: str, sel_unit: str) -> Fraction:
    if ref_unit == sel_unit:
        return Fraction(1)
    return UNIT_FACTOR[(ref_unit, sel_unit)]


def origin_value(ref, ro: dict, sel_unit: str) -> Interval:
    q = resolve(ref, ro['pointer'])
    assert q['unit'] == ro['reference_unit'], (ro['pointer'], q['unit'], ro['reference_unit'])
    v = eval_exact(q)
    # the file's own binary64 projection must agree with this evaluation
    t = ro['transform']
    if t == 'identity':
        pass
    elif t == 'negate':
        v = -v
    elif t == 'one_plus':
        v = v + 1
    elif t == 'generic_area_to_annulus':
        num = resolve(ref, ro['area_ratio']['numerator']['pointer'])
        den = resolve(ref, ro['area_ratio']['denominator']['pointer'])
        v = v * eval_exact(num) / eval_exact(den)
    else:
        raise ValueError(t)
    return v * unit_factor(ro['reference_unit'], sel_unit)


def zero_scale_value(ref, zs: dict, rule_unit: str) -> Interval:
    b = zs['base']
    q = resolve(ref, b['pointer'])
    assert q['unit'] == b['reference_unit']
    v = eval_exact(q)
    v = Interval(min(abs(v.lo), abs(v.hi)), max(abs(v.lo), abs(v.hi)))
    unit = b['reference_unit']
    if 'multiply_by_sum_of' in zs:
        s = Interval(Fraction(0))
        for term in zs['multiply_by_sum_of']:
            tq = resolve(ref, term['pointer'])
            assert tq['unit'] == term['reference_unit'] == 'm'
            s = s + eval_exact(tq)
        v = v * s
        unit = {'N': 'N*m', 'rad': 'm'}[unit]  # force x length; small-angle rotation x length
    assert unit == zs['scale_unit'], (unit, zs['scale_unit'])
    return v * unit_factor(unit, rule_unit)


MARGINS: list = []


def classify_inside(obs: float, exp: float, rel: float, ab: float) -> bool:
    return abs(obs - exp) <= max(ab, rel * max(abs(obs), abs(exp)))


def main() -> int:
    ref = json.loads(REF.read_text())
    manifest = json.loads((PKG / 'MANIFEST.json').read_text())
    c = Counter()
    per_case = {}
    for case in manifest['cases']:
        cid = case['case_id']
        sel = json.loads((PKG / f'{cid}.selectors.candidate.json').read_text())
        rv = json.loads((PKG / f'{cid}.reference.candidate.json').read_text())
        cr = json.loads((PKG / f'{cid}.criteria.candidate.json').read_text())
        vals = {v['assertion_id']: v for v in rv['values']}
        wvals = {v['assertion_id']: v for v in rv.get('wrong_values', [])}
        rules = {r['rule_id']: r for r in cr['tolerance_profile']['rules']}
        n_pos = n_neg = n_zero = 0
        positive_by_id = {}
        for a in sel['assertions']:
            aid = a['id']
            s = a['selector']
            ro = a['selector_origin']['reference_origin']
            pv = vals[aid]
            c.check(pv['unit'] == s['unit'], f'{cid} {aid}: value unit {pv["unit"]} != selector {s["unit"]}')
            rule = rules[a['criterion_rule_id']]
            c.check(rule['unit_ref']['ref'] == s['unit'], f'{cid} {aid}: rule unit')
            c.check(rule['dimension_id'] == s['dimension'], f'{cid} {aid}: rule dimension')
            if ro['kind'] == 'equilibrium_zero':
                expected = 0.0
            else:
                expected = origin_value(ref, ro, s['unit']).to_float()
            c.check(pv['value'] == expected and repr(pv['value']) == repr(expected),
                    f'{cid} {aid}: package {pv["value"]!r} != independent {expected!r}')
            positive_by_id[aid] = (expected, rule)
            if expected == 0.0:
                n_zero += 1
                zs = ro.get('zero_scale')
                c.check(zs is not None, f'{cid} {aid}: zero expectation without zero_scale')
                c.check(rule['relative_tolerance_value'] == 0.0, f'{cid} {aid}: zero rule relative != 0')
                if zs is not None:
                    want = (zero_scale_value(ref, zs, s['unit']) * Fraction(1, 10 ** 9)).to_float()
                    got = rule['absolute_tolerance_value']
                    c.check(got == want, f'{cid} {aid}: zero absolute {got!r} != 1e-9*scale {want!r}')
                    c.check(a['criterion_rule_id'].endswith(':zero_scale:' + zs['tag']),
                            f'{cid} {aid}: rule id/tag mismatch')
            else:
                c.check(rule['relative_tolerance_value'] == 1e-09 and rule['absolute_tolerance_value'] == 0.0,
                        f'{cid} {aid}: nonzero rule not relative_1e-9/abs 0')
            n_pos += 1
        for a in sel.get('negative_assertions', []):
            aid = a['id']
            s = a['selector']
            ro = a['selector_origin']['reference_origin']
            wv = wvals[aid]
            wrong = origin_value(ref, ro, s['unit']).to_float()
            c.check(wv['value'] == wrong, f'{cid} {aid}: wrong {wv["value"]!r} != independent {wrong!r}')
            pos_id = a['selector_origin']['negates_assertion']
            pos = next(p for p in sel['assertions'] if p['id'] == pos_id)
            c.check(pos['selector'] == s, f'{cid} {aid}: selector differs from negated positive')
            correct, _ = positive_by_id[pos_id]
            rule = rules[a['criterion_rule_id']]
            rel, ab = rule['relative_tolerance_value'], rule['absolute_tolerance_value']
            # the correct value must lie OUTSIDE the tolerance band of the wrong value
            c.check(not classify_inside(correct, wrong, rel, ab),
                    f'{cid} {aid}: correct {correct!r} inside tolerance of wrong {wrong!r}')
            # and the wrong value must lie outside the positive rule band of the correct value
            prule = positive_by_id[pos_id][1]
            c.check(not classify_inside(wrong, correct, prule['relative_tolerance_value'],
                                        prule['absolute_tolerance_value']),
                    f'{cid} {aid}: wrong inside the positive rule of the correct value')
            margin = abs(correct - wrong) / max(ab, rel * max(abs(correct), abs(wrong)))
            MARGINS.append((margin, cid, aid))
            n_neg += 1
        per_case[cid] = (n_pos, n_neg, n_zero)
    first_principles(ref, c)
    for cid, (p, n, z) in per_case.items():
        print(f'{cid}: positives {p} (zero expectations {z}), negatives {n}')
    MARGINS.sort()
    print('smallest negative separations (|correct-wrong| / tolerance around wrong):')
    for m, cid, aid in MARGINS[:5]:
        print(f'  {m:.4g}  {cid} {aid}')
    print(f'checks passed {c.ok}, failed {len(c.fail)}')
    for f in c.fail:
        print('FAIL', f)
    return 1 if c.fail else 0


# ------------------------------------------------ first-principles cross-check

def F(ref, p):
    return eval_exact(resolve(ref, p))


def close_iv(c: Counter, got: Interval, want: Interval, msg: str):
    # both are exact enclosures; require overlap within 1e-60 relative
    tol = max(abs(want.hi), Fraction(1, 10 ** 30)) * Fraction(1, 10 ** 60)
    c.check(got.lo - tol <= want.hi and want.lo - tol <= got.hi, f'first-principles {msg}')


def first_principles(ref, c: Counter):
    g = '/geometry/authored'
    od, t = F(ref, g + '/outside_diameter'), F(ref, g + '/wall_thickness')
    As = PI * (od - t) * t  # pi*t*(OD-t)
    ro = od * Fraction(1, 2)
    ri = ro + (-t)
    I = PI * (ro * ro * ro * ro + -(ri * ri * ri * ri)) * Fraction(1, 4)
    close_iv(c, As, F(ref, '/geometry/derived/As'), 'As')
    close_iv(c, I, F(ref, '/geometry/derived/I'), 'I')
    # two-bar
    b = '/cases/prescribed_translation_two_bar/variants/annular_companion'
    k1 = F(ref, b + '/inputs/E1') * As / F(ref, b + '/inputs/L1')
    k2 = F(ref, b + '/inputs/E2') * As / F(ref, b + '/inputs/L2')
    g0, g2 = F(ref, b + '/inputs/root_UX'), F(ref, b + '/inputs/far_UX')
    um = (k1 * g0 + k2 * g2) / (k1 + k2)
    close_iv(c, um, F(ref, b + '/expected/middle_UX'), 'two-bar middle UX')
    n1 = k1 * (um + -g0)
    close_iv(c, n1, F(ref, b + '/expected/member1_N'), 'two-bar N1')
    close_iv(c, -n1, F(ref, b + '/expected/root_Fx'), 'two-bar root Fx = -N1')
    # all-fixed translation
    b = '/cases/prescribed_translation_all_fixed/variants/annular_companion'
    n = F(ref, b + '/inputs/E') * As * (F(ref, b + '/inputs/far_UX') + -F(ref, b + '/inputs/root_UX')) / F(ref, b + '/inputs/L')
    close_iv(c, n, F(ref, b + '/expected/N'), 'all-fixed N')
    # rotation all-fixed: stiffness end actions
    b = '/cases/prescribed_rotation_all_fixed/variants/annular_companion'
    EI = F(ref, b + '/inputs/E') * I
    L, th = F(ref, b + '/inputs/L'), F(ref, b + '/inputs/root_RZ')
    close_iv(c, EI * th * 6 / (L * L), F(ref, b + '/expected/root_Fy'), 'rotation root Fy = 6EI th/L^2')
    close_iv(c, EI * th * 4 / L, F(ref, b + '/expected/root_Mz'), 'rotation root Mz = 4EI th/L')
    close_iv(c, -(EI * th * 6 / (L * L)), F(ref, b + '/expected/far_Fy'), 'rotation far Fy')
    close_iv(c, EI * th * 2 / L, F(ref, b + '/expected/far_Mz'), 'rotation far Mz = 2EI th/L')
    # free tip
    b = '/cases/prescribed_rotation_free_tip/variants/annular_companion'
    close_iv(c, F(ref, b + '/inputs/L') * F(ref, b + '/inputs/root_RZ'), F(ref, b + '/expected/tip_UY'), 'free tip UY = L th')
    # serial companion
    b = '/cases/shared_material_serial_companion/variants/annular_companion'
    E1, E2 = F(ref, b + '/inputs/E1'), F(ref, b + '/inputs/E2')
    L1, L2 = F(ref, b + '/inputs/L1'), F(ref, b + '/inputs/L2')
    e1, e2 = F(ref, b + '/inputs/epsilon1'), F(ref, b + '/inputs/epsilon2')
    N = -(e1 * L1 + e2 * L2) / (L1 / (E1 * As) + L2 / (E2 * As))
    close_iv(c, N, F(ref, b + '/expected/member1_N'), 'serial N')
    close_iv(c, N * L1 / (E1 * As) + e1 * L1, F(ref, b + '/expected/middle_UX'), 'serial middle UX')
    close_iv(c, E1 / (2 * (1 + F(ref, b + '/inputs/nu1'))), F(ref, b + '/expected/G1'), 'serial G1')
    # material-ID-only override: equal E for both members
    Nw = -(e1 * L1 + e2 * L2) / ((L1 + L2) / (E1 * As))
    close_iv(c, Nw * L1 / (E1 * As) + e1 * L1,
             F(ref, '/cases/shared_material_serial_companion/wrong_result_discriminators/material_ID_only_E_override/wrong_middle_UX'),
             'serial wrong middle UX (single E)')
    # thermal datum ratio (secant table, linear coefficient)
    b = '/cases/thermal_datum_ratio/variants/annular_companion'
    Tm, Ti, To = (F(ref, b + '/inputs/' + k) for k in ('datum_temperature', 'installation_temperature', 'operating_temperature'))
    ai, ao = F(ref, b + '/inputs/alpha_install'), F(ref, b + '/inputs/alpha_operating')
    li = ai * (Ti + -Tm) + 1
    lo_ = ao * (To + -Tm) + 1
    eps = lo_ / li + -1
    close_iv(c, eps, F(ref, b + '/expected/thermal_strain'), 'thermal strain secant ratio')
    close_iv(c, -(F(ref, b + '/inputs/E') * As * eps), F(ref, b + '/expected/fixed_wall_N'), 'thermal fixed N')
    close_iv(c, F(ref, b + '/inputs/L') * eps, F(ref, b + '/expected/free_tip_UX'), 'thermal free tip')
    close_iv(c, Ti + Fraction(27315, 100), F(ref, b + '/expected/installation_temperature_K'), 'T_install K')
    # coefficient definition (trapezoid integrals)
    b = '/cases/coefficient_definition/variants/generic_reviewed'
    a0, a1 = Fraction(1, 100000), Fraction(1, 50000)
    I_full = (a0 + a1) * 100 / 2
    I_half = (a0 + (a0 + a1) / 2) * 50 / 2
    close_iv(c, Interval(1 / (1 + I_full) - 1), F(ref, b + '/expected/datum_length_strain_reverse'), 'coef datum reverse')
    close_iv(c, Interval((1 + I_full) / (1 + I_half) - 1), F(ref, b + '/expected/datum_length_strain_second_half'), 'coef datum second half')
    lo, hi = exp_bounds(I_full)
    close_iv(c, Interval(lo - 1, hi - 1), F(ref, b + '/expected/current_length_strain_forward'), 'coef log forward')
    close_iv(c, Interval(a1 * 100), F(ref, '/cases/coefficient_definition/wrong_result_discriminators/endpoint_alpha_times_interval'), 'coef endpoint wrong')
    # constant alpha
    b = '/cases/constant_alpha_interval/variants/generic_reviewed'
    close_iv(c, F(ref, b + '/inputs/alpha') * F(ref, b + '/inputs/deltaT'), F(ref, b + '/expected/thermal_strain'), 'constant alpha strain')
    # multi-segment: piecewise trapezoid
    pts = [(300, Fraction(1, 100000)), (400, Fraction(1, 50000)), (500, Fraction(1, 100000)), (600, Fraction(3, 100000))]

    def alpha(T):
        for (t0, a0_), (t1, a1_) in zip(pts, pts[1:]):
            if t0 <= T <= t1:
                return a0_ + (a1_ - a0_) * Fraction(T - t0, t1 - t0)
        raise ValueError(T)

    def integ(a, b_):
        sgn = 1
        if b_ < a:
            a, b_, sgn = b_, a, -1
        br = [a] + [t for t, _ in pts if a < t < b_] + [b_]
        return sgn * sum((alpha(x) + alpha(y)) * (y - x) / 2 for x, y in zip(br, br[1:]))

    b = '/cases/multi_segment_free_length/variants/linear_coefficient_table/expected'
    close_iv(c, Interval(integ(300, 350)), F(ref, b + '/integral_datum_to_install'), 'ms I(Tm,Ti)')
    close_iv(c, Interval(integ(300, 550)), F(ref, b + '/integral_datum_to_operating'), 'ms I(Tm,T)')
    close_iv(c, Interval(integ(300, 450)), F(ref, b + '/integral_datum_to_split'), 'ms I(Tm,Ts)')
    close_iv(c, Interval((1 + integ(300, 550)) / (1 + integ(300, 350)) - 1), F(ref, b + '/datum_length_strain_forward'), 'ms datum forward')
    close_iv(c, Interval((1 + integ(300, 350)) / (1 + integ(300, 450)) - 1), F(ref, b + '/datum_length_strain_split_to_install'), 'ms datum split->install')
    lo, hi = exp_bounds(integ(450, 550))
    close_iv(c, Interval(lo - 1, hi - 1), F(ref, b + '/current_length_strain_split_to_operating'), 'ms log split->op')
    dpts = [(300, Fraction(0)), (400, Fraction(1, 1000)), (500, Fraction(3, 2000)), (600, Fraction(7, 2000))]

    def dil(T):
        for (t0, d0), (t1, d1) in zip(dpts, dpts[1:]):
            if t0 <= T <= t1:
                return d0 + (d1 - d0) * Fraction(T - t0, t1 - t0)
        raise ValueError(T)

    b = '/cases/multi_segment_free_length/variants/linear_dilation_table/expected'
    close_iv(c, Interval((1 + dil(550)) / (1 + dil(350)) - 1), F(ref, b + '/dilation_strain_forward'), 'ms dilation forward')
    close_iv(c, Interval((1 + dil(350)) / (1 + dil(550)) - 1), F(ref, b + '/dilation_strain_reverse'), 'ms dilation reverse')
    close_iv(c, Interval(alpha(550) * (550 - 350)),
             F(ref, '/cases/multi_segment_free_length/wrong_result_discriminators/endpoint_alpha_times_interval/strain'), 'ms endpoint wrong')
    # temperature identities
    b = '/cases/temperature_unit_identity/variants/exact_affine_identity/expected'
    close_iv(c, Interval((Fraction(2338, 5) + Fraction(45967, 100)) * 5 / 9), F(ref, b + '/identity_groups/1/kelvin'), 'degF identity')
    close_iv(c, Interval(Fraction(52767, 100) * 5 / 9), F(ref, b + '/identity_groups/2/kelvin'), 'degR identity')
    close_iv(c, Interval(Fraction(-49999999, 1000000) + Fraction(27315, 100)), F(ref, b + '/non_equal_control/authored_kelvin/0'), 'non-equal control K')
    # signed fit
    b = '/cases/signed_fit_states/variants/annular_companion'
    L = F(ref, b + '/inputs/L')
    lam = F(ref, b + '/inputs/signed_fit_length_change') / L + 1
    eth = F(ref, b + '/inputs/alpha') * (F(ref, b + '/inputs/hot_temperature') + -F(ref, b + '/inputs/installation_temperature'))
    est_hot = lam * (eth + 1) + -1
    close_iv(c, est_hot, F(ref, b + '/expected/hot/total_eigenstrain'), 'fit hot eigenstrain')
    close_iv(c, -(F(ref, b + '/inputs/hot_E') * As * est_hot), F(ref, b + '/expected/hot/fixed_wall_N'), 'fit hot N')
    est_cold = lam + -1
    close_iv(c, -(F(ref, b + '/inputs/cold_E') * As * est_cold), F(ref, b + '/expected/cold/fixed_wall_N'), 'fit cold N')
    close_iv(c, L * est_hot, F(ref, b + '/expected/hot/released_tip_UX'), 'fit hot released tip')
    # persistent source
    b = '/cases/persistent_source_once/variants/annular_companion'
    Fc = F(ref, b + '/inputs/preload') + F(ref, b + '/inputs/weight') + F(ref, b + '/inputs/independent_action')
    close_iv(c, Fc, F(ref, b + '/expected/combined_rhs'), 'source combined rhs')
    k = F(ref, b + '/inputs/E') * As / F(ref, b + '/inputs/L')
    close_iv(c, Fc / k, F(ref, b + '/expected/combined_displacement'), 'source tip u = F/k')
    close_iv(c, -Fc, F(ref, b + '/expected/root_Fx'), 'source root Fx = -F')
    close_iv(c, F(ref, b + '/inputs/independent_action') * 2, F(ref, b + '/expected/two_distinct_equal_actions_rhs'), 'source pair = 2x200')


if __name__ == '__main__':
    sys.exit(main())
