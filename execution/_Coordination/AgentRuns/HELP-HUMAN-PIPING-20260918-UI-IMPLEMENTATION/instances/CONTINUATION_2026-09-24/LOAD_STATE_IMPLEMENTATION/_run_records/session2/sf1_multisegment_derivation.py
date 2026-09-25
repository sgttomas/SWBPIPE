"""Manager's own exact derivation of SF1 multi-segment targets (no kernel import)."""
from fractions import Fraction as F
from decimal import Decimal, getcontext
getcontext().prec = 60
coef = [(300, F(1, 100000)), (400, F(2, 100000)), (500, F(1, 100000)), (600, F(3, 100000))]
dil = [(300, F(0)), (400, F(1, 1000)), (500, F(15, 10000)), (600, F(35, 10000))]
def lin(points, t):
    t = F(t)
    for (a, va), (b, vb) in zip(points, points[1:]):
        if a <= t <= b:
            return va + (vb - va) * (t - a) / (b - a)
    raise ValueError(t)
def integral(points, lo, hi):
    sign = 1
    if lo > hi: lo, hi, sign = hi, lo, -1
    knots = sorted({F(lo), F(hi)} | {F(t) for t, _ in points if lo < t < hi})
    return sign * sum((lin(points, a) + lin(points, b)) / 2 * (b - a) for a, b in zip(knots, knots[1:]))
datum = 300
lam_datum = lambda t: 1 + integral(coef, datum, t)
lam_log = lambda t: Decimal(integral(coef, datum, t).numerator) / Decimal(integral(coef, datum, t).denominator)
out = {}
out["datum_fwd_350_550"] = lam_datum(550) / lam_datum(350) - 1
out["datum_rev_550_350"] = lam_datum(350) / lam_datum(550) - 1
d = lambda a, b: integral(coef, a, b)
out["log_fwd_integral"] = d(350, 550)
out["log_fwd"] = (Decimal(d(350, 550).numerator) / Decimal(d(350, 550).denominator)).exp() - 1
out["log_rev"] = (Decimal(d(550, 350).numerator) / Decimal(d(550, 350).denominator)).exp() - 1
lam_dil = lambda t: 1 + lin(dil, t)
out["dilation_fwd_350_550"] = lam_dil(550) / lam_dil(350) - 1
out["dilation_rev_550_350"] = lam_dil(350) / lam_dil(550) - 1
# split composition through interior table knots 350->450->550
out["datum_split_check"] = (lam_datum(450) / lam_datum(350)) * (lam_datum(550) / lam_datum(450)) - 1
# SF2 two-point secant table, datum 20 C outside the table (exact inputs of VERIFICATION 4)
K = lambda c: F(c) + F(27315, 100)
sec = [(K(50), F(12, 1000000)), (K(150), F(16, 1000000))]
lam_sec = lambda t: 1 + lin(sec, t) * (t - K(20))
out["verification4_two_point"] = lam_sec(K(150)) / lam_sec(K(50)) - 1
# secant interior interpolation from the reviewer's 3-point table
sec3 = [(300, F(1, 100000)), (400, F(2, 100000)), (500, F(1, 100000))]
lam3 = lambda t: 1 + lin(sec3, t) * (F(t) - 300)
out["secant_interior_350_475"] = lam3(475) / lam3(350) - 1
for k, v in out.items():
    print(k, v, float(v) if not isinstance(v, Decimal) else repr(float(v)))
