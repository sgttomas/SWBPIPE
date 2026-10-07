#!/usr/bin/env python3
"""RV115 ADDENDUM_01 checks for B3-K's K3-2 (standard library only).

1. The represented-Z hull, hull(I_K/c, Z_hat), for the frozen oracle's two
   ExactENu vectors (exact_normal, ratio_amplification), from their committed
   K primitives, and the 1024-bit outward tokens the regenerated fixture
   would carry. The token rule is re-implemented here from its definition
   (1024-bit significand, lower rounded down, upper rounded up); nothing is
   imported from the repository.
2. The same hull for the six non-ExactENu vectors, to show the formula the
   generator already applies is material-independent.
3. That the hull contains both I_K/c and Z_hat, so the K-lane signed-corner
   division encloses both M/Z_hat (the published recipe) and M*c/I_K.
4. The exact-route bending stress recipe used by PP: sigma = M / Z (read in
   code; recorded here as the property the hull must cover).
"""
from fractions import Fraction as F
import json, struct

def floorlog2(x):
    x = abs(x)
    e = x.numerator.bit_length() - x.denominator.bit_length()
    if x < F(2) ** e:
        e -= 1
    return e

def token(x, up=False):
    if x == 0:
        return 'Z+'
    neg = x < 0
    x = abs(x)
    e = floorlog2(x)
    q = F(2) ** (e - 1023)
    k = x / q
    m = k.numerator // k.denominator
    if k != m and (up != neg):
        m += 1
    if m.bit_length() > 1024:
        m //= 2
        e += 1
    return ('-' if neg else '+') + format(m, '0256x').rstrip('0') + 'p' + str(e)

# (name, mode, D, K primitives [E, G, A, J, Iz, Iy, Zhat]) from the frozen generator's case table.
h = float.fromhex('0x0.0000000000001p-1022')
x = float.fromhex('0x1.0000000000001p0')
cases = [
    ('exact_normal', 0, 4., [210e9, 210e9 / 2.6, 9., 30., 15., 15., 7.]),
    ('ordinary_exact_product', 1, 4., [x, x, x, x, x, x, 2.]),
    ('interpolated_negative_e', 2, 4., [1., 4., 9., 30., 15., 15., 7.]),
    ('thin_cancellation', 1, 1., [2., 3., 2. ** -98, 2. ** -102, 2. ** -103, 2. ** -103, 2. ** -102]),
    ('subnormal_geometry_material', 1, 4 * h, [h, h, h, h, h, h, h]),
    ('ratio_amplification', 0, 4., [1., 2. ** 52, 1., 1., 1., 1., 1.]),
    ('interpolated_wide_span', 2, 4., [.5, .5, 1., 1., 1., 1., 1.]),
    ('interpolated_cancellation_span', 2, 4., [2. ** -53, 2 * h, 1., 1., 1., 1., 1.]),
]
out = {}
for name, mode, D, k in cases:
    c = F(D) / 2
    iz, iy, zhat = F(k[4]), F(k[5]), F(k[6])
    ikc = iz / c
    lo, hi = min(ikc, zhat), max(ikc, zhat)
    entry = {
        'mode': mode,
        'axis_bits_equal': k[4] == k[5],
        'I_K_over_c': str(ikc), 'Z_hat': str(zhat),
        'hull': [str(lo), str(hi)],
        'hull_contains_both': lo <= ikc <= hi and lo <= zhat <= hi,
        'hull_positive': lo > 0,
        'tokens': [token(lo), token(hi, True)],
        'currently_pinned_as_zero': mode == 0,
    }
    out[name] = entry
    assert entry['hull_contains_both'] and entry['hull_positive'] and entry['axis_bits_equal']
assert out['exact_normal']['hull'] == ['7', '15/2']
assert out['ratio_amplification']['hull'] == ['1/2', '1']

# 3. signed-corner division by a positive interval encloses both quotients.
M = F(-123456789, 1000)
for name in ('exact_normal', 'ratio_amplification'):
    lo, hi = (F(v) for v in out[name]['hull'])
    q = sorted([M / lo, M / hi])
    ikc, zhat = F(out[name]['I_K_over_c']), F(out[name]['Z_hat'])
    assert q[0] <= M / zhat <= q[1] and q[0] <= M / ikc <= q[1]
out['corner_division_encloses_both_quotients'] = True
print(json.dumps(out, indent=1, sort_keys=True))
