"""T4-I6 R01 item 7: evidence for the corrected M31b0 derivation (M31B0_DERIVATION.md), binary p = 128.

All chords are formed with this round's own binary p-bit arithmetic (r01_lib.BinP; RV131's rv_wide.py
is not used), in B1's operation order, from K-D5's post-T4-U1 inputs (x_i, x_j, R, y) only:
  actual   c_act = A_p . d_p                              (the correct check's H)
  cancel   (fl(R fl(fl(1 - 2 fl(s^2)) - 1)), fl(R fl(2 s c)), 0)   (M31b0 as historically patched: cos - 1)
  benign   (fl(R (-2 fl(s^2))), fl(R fl(2 s c)), 0)               (the non-cancelling spelling)
A. RV131's 353 adversarial inputs, regenerated with RV131's generator and seed (rv_m31b0_attack.py's
   `case`, transcribed), recomputed here: maxima per class, for comparison with RV131's printed values.
B. An extension: 3000 further inputs (seed 20261010) over phi in [1e-9, pi - 1e-9], L in [1e-3, 1e2] m,
   X in {0, 5e5, 2e6, 5e6, 7.3e6} m, random y.
C. The bound's constants, measured: c1 (the 1/(2 sin(phi/2)) term, cancelling spelling, phi < 1e-4),
   c0 (phi in [0.01, pi - 0.01], |y|/|y_perp| < 10), c2 (an ill-conditioned-y sweep with the exact
   |y|/|y_perp| of the binary64 y).
D. K-D5-level: triggers of the correct check and of M31b0 (both spellings) on K1 (IP, SK) at
   phi in {1e-9, 1e-8, 1e-6, 5 deg} and on the L = 30 m O4 model at 1e-9.

usage: python -I -B i7_m31b0.py > i7_m31b0.stdout.txt  (writes parts/i7_m31b0.json)
"""
import json
import math
import os
import random
import sys
from decimal import Decimal as D, getcontext, localcontext
from fractions import Fraction as Fr

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import r01_lib as Lb  # noqa: E402

C = Lb.C
AR = Lb.BinP(128)
U = 2.0 ** -128


def rel(f, act, L):
    return math.sqrt(sum(float(f[k] - act[k]) ** 2 for k in range(3))) / float(L)


def comp(f, act, L, k):
    return abs(float(f[k] - act[k])) / float(L)


def yratio(d, y):
    """exact |y| / |y_perp| of binary64 y against binary64 d."""
    dF = [Fr(v) for v in d]
    yF = [Fr(v) for v in y]
    dd = sum(v * v for v in dF)
    yd = sum(yF[k] * dF[k] for k in range(3))
    yy = sum(v * v for v in yF)
    perp2 = yy - yd * yd / dd
    return math.sqrt(float(yy / perp2)) if perp2 > 0 else float("inf")


def radius(L, phi):
    with localcontext() as c:
        c.prec = 60
        s = C.sin(D(phi) / 2)
        return float(D(L) / (2 * s))


def evaluate(xi, xj, R, y):
    r = Lb.chords_at_p(AR, xi, xj, R, y)
    L = r["L"]
    return dict(phi=float(r["phi"]), cancel=rel(r["f_cancel"], r["act"], L), benign=rel(r["f_benign"], r["act"], L),
                cancel_x=comp(r["f_cancel"], r["act"], L, 0), ym=float(r["ym"]), s=float(r["s"]))


# ---------------------------------------------------------------- A. RV131's generator (transcribed)
def rv131_inputs():
    rng = random.Random(20261008)
    out = []

    def case(tag, X, L, phi, ydir=None, near_parallel=None):
        u = [rng.gauss(0, 1) for _ in range(3)]
        nu = math.sqrt(sum(v * v for v in u))
        u = [v / nu for v in u]
        xi = [X + rng.uniform(0, 1000), 0.7 * X + rng.uniform(0, 1000), rng.uniform(-50, 50)]
        xj = [xi[k] + L * u[k] for k in range(3)]
        d = [xj[k] - xi[k] for k in range(3)]
        Lb_ = math.sqrt(sum(v * v for v in d))
        R = radius(Lb_, phi)
        if R <= Lb_ / 2:
            return
        if near_parallel is not None:
            w = [rng.gauss(0, 1) for _ in range(3)]
            dh = [v / Lb_ for v in d]
            wd = sum(w[k] * dh[k] for k in range(3))
            w = [w[k] - wd * dh[k] for k in range(3)]
            nw = math.sqrt(sum(v * v for v in w))
            w = [v / nw for v in w]
            y = [dh[k] + near_parallel * w[k] for k in range(3)]
        else:
            y = ydir or [rng.gauss(0, 1) for _ in range(3)]
        out.append((tag, xi, xj, R, y))
    for _ in range(150):
        case("random", rng.choice([0.0, 5e5, 5e6, 7.3e6]), 10 ** rng.uniform(-3, 2), 10 ** rng.uniform(-9, 0.3))
    for _ in range(80):
        case("near-pi", rng.choice([0.0, 7.3e6]), 10 ** rng.uniform(-2, 1), math.pi - 10 ** rng.uniform(-9, -1))
    for _ in range(80):
        case("floor", rng.choice([0.0, 7.3e6]), 10 ** rng.uniform(-3, 2), 1e-9 * (1 + rng.uniform(0, 1)))
    for _ in range(60):
        case("y-near-parallel", rng.choice([0.0, 7.3e6]), 10 ** rng.uniform(-2, 1), 10 ** rng.uniform(-9, 0.3),
             near_parallel=10 ** rng.uniform(-9, -7))
    return out


def main():
    getcontext().prec = 80
    parts = {}
    # A
    rows = []
    for tag, xi, xj, R, y in rv131_inputs():
        e = evaluate(xi, xj, R, y)
        d = [xj[k] - xi[k] for k in range(3)]
        e.update(tag=tag, yr=yratio(d, y))
        rows.append(e)
    print("A. RV131's adversarial set regenerated: %d admissible inputs (RV131: 353)" % len(rows))
    rv131_printed = {"random": "1.02e-30", "near-pi": "2.87e-38", "floor": "1.34e-30", "y-near-parallel": "4.40e-30"}
    summ = {}
    for tag in ("random", "near-pi", "floor", "y-near-parallel"):
        sel = [r for r in rows if r["tag"] == tag]
        w = max(sel, key=lambda r: r["cancel"])
        wb = max(r["benign"] for r in sel)
        print("  %-16s n=%3d  max|dc|/L cancelling %.2e at phi %.3e (RV131 printed %s) | benign %.2e" % (
            tag, len(sel), w["cancel"], w["phi"], rv131_printed[tag], wb))
        summ[tag] = dict(n=len(sel), max_cancel="%.2e" % w["cancel"], max_benign="%.2e" % wb, rv131_printed=rv131_printed[tag])
    parts["A_rv131_set"] = summ
    # B
    rng = random.Random(20261010)
    ext = []
    for _ in range(3000):
        X = rng.choice([0.0, 5e5, 2e6, 5e6, 7.3e6])
        L = 10 ** rng.uniform(-3, 2)
        band = rng.random()
        if band < 0.4:
            phi = 10 ** rng.uniform(-9, 0)
        elif band < 0.7:
            phi = rng.uniform(0.01, math.pi - 0.01)
        else:
            phi = math.pi - 10 ** rng.uniform(-7.5, -1)
        uu = [rng.gauss(0, 1) for _ in range(3)]
        nu = math.sqrt(sum(v * v for v in uu))
        xi = [X + rng.uniform(0, 1000), 0.7 * X + rng.uniform(0, 1000), rng.uniform(-50, 50)]
        xj = [xi[k] + L * uu[k] / nu for k in range(3)]
        d = [xj[k] - xi[k] for k in range(3)]
        Ld = math.sqrt(sum(v * v for v in d))
        R = radius(Ld, phi)
        if not (R > Ld / 2):
            continue
        y = [rng.gauss(0, 1) for _ in range(3)]
        e = evaluate(xi, xj, R, y)
        e.update(yr=yratio(d, y), X=X)
        if not (1e-9 <= e["phi"] <= math.pi - 1e-9):
            continue
        ext.append(e)
    allr = rows + ext
    print("B. extension: %d further admissible inputs; overall max |dc|/L cancelling %.2e, benign %.2e" % (
        len(ext), max(r["cancel"] for r in allr), max(r["benign"] for r in allr)))
    parts["B_extension"] = dict(n=len(ext), seed=20261010, max_cancel_all="%.2e" % max(r["cancel"] for r in allr),
                                max_benign_all="%.2e" % max(r["benign"] for r in allr))
    # C. constants
    small = [r for r in allr if r["phi"] < 1e-4 and r["yr"] < 10]
    c1_u = max(r["cancel_x"] / (U / (2 * r["s"])) for r in small)
    mid = [r for r in allr if 0.01 <= r["phi"] <= math.pi - 0.01 and r["yr"] < 10]
    c0b = max(r["benign"] / U for r in mid)
    c0c = max((r["cancel"] - 0.5 * U / (2 * r["s"])) / U for r in mid)
    nearpi = [r for r in allr if r["phi"] > math.pi - 1e-3 and r["yr"] < 10]
    c0pi = max(r["cancel"] / U for r in nearpi)
    print("C. constants (u = 2^-128):")
    print("  c1: max |dc_x|/L / (u / (2 sin(phi/2))) over %d inputs with phi < 1e-4, |y|/|y_perp| < 10 (cancelling) = %.6f  [derivation: c1 <= 1/2]" % (len(small), c1_u))
    print("  c0: max |dc|/L / u over %d inputs with phi in [0.01, pi-0.01], |y|/|y_perp| < 10: benign %.2f; cancelling minus its c1 term %.2f  [derivation: c0 <= 30]" % (len(mid), c0b, c0c))
    print("      near pi (phi > pi - 1e-3, %d inputs): cancelling %.2f" % (len(nearpi), c0pi))
    # c2 sweep: d = K1-IP chord direction (irrational components), y = big * dhat_b64 + perpendicular
    dK = [0.25980762112885714, 0.15000000037252903, 0.0]
    Lk = math.sqrt(sum(v * v for v in dK))
    dh = [v / Lk for v in dK]
    c2rows = []
    for ratio in (1e3, 1e6, 1e9, 1e12, 1e15, 1e18, 1e20, 1e22, 1e25, 1e28):
        for perp in ([-dh[1], dh[0], 0.0], [0.0, 0.0, 1.0]):
            y = [ratio * dh[k] + perp[k] for k in range(3)]
            for phi in (math.pi / 2, 1e-6):
                R = radius(Lk, phi)
                e = evaluate([0.0, 0.0, 0.0], dK, R, y)
                yr = yratio(dK, y)
                c2rows.append((ratio, yr, phi, e["cancel"], e["benign"], e["cancel"] / (U * yr)))
    c2 = max(r[5] for r in c2rows if r[1] > 1e6)
    for r in c2rows:
        print("  y sweep: nominal %.0e exact |y|/|y_perp| %.3e phi %.2e  |dc|/L cancelling %.2e benign %.2e  -> (|dc|/L)/(u |y|/|y_perp|) = %.3f" % r)
    print("  c2: max (|dc|/L) / (u |y|/|y_perp|) for |y|/|y_perp| > 1e6: %.3f  [derivation: c2 <= 11]" % c2)
    worst = max(r["cancel"] / (U * (30 + 0.5 / (2 * r["s"]) + 11 * r["yr"])) for r in allr)
    print("  bound check on all %d inputs: max |dc|/L / ((30 + 1/(4 sin(phi/2)) + 11 |y|/|y_perp|) u) = %.6f (<= 1 required)" % (len(allr), worst))
    parts["C_constants"] = dict(u="2^-128", c1_measured="%.6f" % c1_u, c1_derived="1/2 (term c1 u / (2 sin(phi/2)) <= (pi/4) u / phi)",
                                c0_measured_benign="%.2f" % c0b, c0_measured_cancelling="%.2f" % c0c, c0_measured_near_pi="%.2f" % c0pi,
                                c0_derived="<= 30 (first order)", c2_measured="%.3f" % c2, c2_derived="<= 11 (first order)",
                                bound_check_max_ratio="%.6f" % worst,
                                y_sweep=[dict(nominal=r[0], exact_ratio="%.3e" % r[1], phi=r[2], cancel="%.2e" % r[3], benign="%.2e" % r[4]) for r in c2rows])
    # D. K-D5 level
    S = Lb.F122_SECTION
    print("D. K-D5 triggers, correct check vs M31b0 (p = 128, stable spelling; u = exact intended solution rounded once):")
    drows = []
    G30 = 2.0 ** -30
    for plane, dhat, yv in (("IP", [math.cos(math.pi / 6), math.sin(math.pi / 6), 0.0], [0.0, 1.0, 0.0]),
                            ("SK", [1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0], [1.0, -1.0, 0.5])):
        d = [round(0.3 * c / G30) * G30 for c in dhat]
        Ld = math.sqrt(sum(v * v for v in d))
        for phi in (1e-9, 1e-8, 1e-6, math.pi / 36):
            R = radius(Ld, phi)
            xi, xj = [0.0, 0.0, 0.0], d
            el = Lb.exact_element(xi, xj, R, yv)
            springs = [(3, 1e6), (4, 1e6), (5, 1e6)]
            loads = {9: 1.0, 10: 1.0, 11: 1.0}
            free = [3, 4, 5, 6, 7, 8, 9, 10, 11]
            u = Lb.solve_exact(Lb.assemble(2, [(0, 1, el["K"])], springs), loads, free)
            ub = {q: float(v) for q, v in u.items()}
            Kc, _ = Lb.kd5_b1_at_p(AR, xi, xj, R, yv, S, 1.0, 1.0, "stable", "actual")
            K0, _ = Lb.kd5_b1_at_p(AR, xi, xj, R, yv, S, 1.0, 1.0, "stable", "formula_p")
            tc, _ = Lb.kd5_trigger(Kc, springs, loads, free, ub, [xi, xj], K_solve=el["K"])
            t0, _ = Lb.kd5_trigger(K0, springs, loads, free, ub, [xi, xj], K_solve=el["K"])
            drows.append(("K1-" + plane, phi, tc, t0))
            print("  K1-%s phi %.3e  correct %.6e  M31b0 %.6e  difference %.1e" % (plane, phi, tc, t0, abs(tc - t0)))
    # O4 model
    dO = [round(30.0 * c / G30) * G30 for c in (math.cos(math.pi / 6), math.sin(math.pi / 6), 0.0)]
    LO = math.sqrt(sum(v * v for v in dO))
    R = radius(LO, 1e-9)
    xi, xj = [0.0, 0.0, 0.0], dO
    el = Lb.exact_element(xi, xj, R, [0.0, 1.0, 0.0])
    free = [6, 7, 8, 9, 10, 11]
    f = {6: dO[0] / LO, 7: dO[1] / LO, 8: dO[2] / LO}
    u = Lb.solve_exact(Lb.assemble(2, [(0, 1, el["K"])], []), f, free)
    ub = {q: float(v) for q, v in u.items()}
    Kc, _ = Lb.kd5_b1_at_p(AR, xi, xj, R, [0.0, 1.0, 0.0], S, 1.0, 1.0, "stable", "actual")
    K0, _ = Lb.kd5_b1_at_p(AR, xi, xj, R, [0.0, 1.0, 0.0], S, 1.0, 1.0, "stable", "formula_p")
    tc, _ = Lb.kd5_trigger(Kc, [], f, free, ub, [xi, xj], K_solve=el["K"])
    t0, _ = Lb.kd5_trigger(K0, [], f, free, ub, [xi, xj], K_solve=el["K"])
    drows.append(("O4-L30", 1e-9, tc, t0))
    print("  O4 L 30 m phi 1e-9  correct %.6e  M31b0 %.6e  difference %.1e" % (tc, t0, abs(tc - t0)))
    parts["D_kd5_level"] = [dict(model=r[0], phi=r[1], correct="%.6e" % r[2], m31b0="%.6e" % r[3]) for r in drows]
    with open(os.path.join(HERE, "parts", "i7_m31b0.json"), "w") as fh:
        json.dump({"m31b0_evidence": parts}, fh, indent=1)


if __name__ == "__main__":
    main()
