#!/usr/bin/env python3
"""Test vectors for `structural/arc_maximum.rs` (T4-U4b, DEL-04-07 Design
`u4b-u5b-implementation.md` section 2.3).

Standard library only. None of the expected values comes from the code under
test or from any product output. Everything is computed here in `decimal` at
110 significant digits (about 365 bits, above the 256 bits the design names),
and every argument is an exact binary64 or dyadic value.

Usage:  python3 -I -B gen_arc_maximum_vectors.py [--check]

Writes, beside this script:
  trig.txt        sine and cosine at 23 exact arguments near 0, pi/2 and pi,
                  at the t4_u4b arcs' binary64 included angles and around 1:
                  the 60-digit decimal values (provenance, human-readable) and
                  `Wide<2>` tokens f <= value <= c, the 128-bit floor of an
                  exact rational lower bracket and the ceiling of an exact
                  upper bracket (Taylor partial sum -/+ the Lagrange bound
                  x^n/n! < 2^-400, in `fractions`), which the Rust test uses for
                  its containment check. The decimal values are computed two
                  independent ways (the Taylor series at the argument, and the
                  series at x/2^12 followed by 12 doubling steps); they must
                  agree to 1e-95 and lie in the exact bracket.
  two_peaks.txt   operands of a symmetric arc whose two interior peaks lie on
                  the 4,096-cell grid, and two variants perturbed by an
                  asymmetric end moment so that one peak is higher by about
                  1e-9 relative, with each variant's peak location and value.
  end_controls.txt  two arcs for the design's location order, tuned by
                  Newton in (M_x, M_z) at end j: a governing end j whose
                  interior peak, on the grid, is 1e-14 lower; and a governing
                  interior peak, on the grid, with end i 1e-14 lower.
  directed.txt    the measure and its derivative at nine exact points of every
                  t4_u4b arc and basis (both measures) and of the two-peak
                  variants, and the maximum of each represented problem, as the
                  binary64 floor and ceiling of the 110-digit value; and
                  Codex's rotated small-angle torsion control (the maximum of
                  the transposed chain and of the direct-local function).
The generator also checks its decimal model against the oracle's 40-digit
station resultants at all five stations of every arc and basis (within 1e-13
of the zero scale; the worst is printed).
  SHA256SUMS      sha256 of the three files above.
With --check, nothing is written; the files are regenerated in memory and
compared byte for byte with the committed ones.

The represented problem of an oracle arc is formed from the binary64 values in
`validation/references/t4_u4b/oracle.json` and `cases.json` exactly as the
Rust test forms its operands: the end-j section action
(`resultants_tangent_frame_j_side`, S on pressurized arcs), the end-j tangent
frame rows (`tangent_frames_global_components.end_j`, decimal strings rounded to
binary64), the arc's uniform load per arc length split into its global
components, R, the binary64 included angle, p_Ai, and the basis A and Z. The
measures and closed forms are those of the t4_u4b README item 4 and the U4b
design section 2.6.
"""
import hashlib
import json
import math
import sys
from decimal import Decimal as D, getcontext
from fractions import Fraction
from pathlib import Path

getcontext().prec = 110
HERE = Path(__file__).resolve().parent
REFS = HERE.parents[4] / "validation" / "references" / "t4_u4b"
FULL = 1 << 60
GRID = FULL // 4096


# ---------------------------------------------------------------------------
# Decimal sine and cosine
# ---------------------------------------------------------------------------

def sincos_series(x):
    s = D(0)
    c = D(0)
    term = D(1)
    k = 0
    eps = D(10) ** -140
    while True:
        r = k % 4
        if r == 0:
            c += term
        elif r == 1:
            s += term
        elif r == 2:
            c -= term
        else:
            s -= term
        k += 1
        term = term * x / k
        if k > 4 and abs(term) < eps:
            return s, c


def sincos_doubling(x):
    s, c = sincos_series(x / (D(2) ** 12))
    for _ in range(12):
        s, c = 2 * s * c, c * c - s * s
    return s, c


def sincos(x):
    return sincos_series(x)


# ---------------------------------------------------------------------------
# Tokens
# ---------------------------------------------------------------------------

def f64hex(x):
    import struct
    return "%016x" % struct.unpack("<Q", struct.pack("<d", x))[0]


def floor_ceil64(v):
    """Binary64 floor and ceiling of a Decimal."""
    f = float(v)
    if D(f) > v:
        lo, hi = math.nextafter(f, -math.inf), f
    elif D(f) < v:
        lo, hi = f, math.nextafter(f, math.inf)
    else:
        lo = hi = f
    return lo, hi


def wide_token(negative, e, m):
    return ("-" if negative else "+") + "%032x" % m + "p" + str(e)


def _bracket_token(a, upward):
    """Token of the 128-bit floor (upward=False) or ceiling of a Fraction a > 0."""
    e = a.numerator.bit_length() - a.denominator.bit_length()
    while Fraction(2) ** e > a:
        e -= 1
    while Fraction(2) ** (e + 1) <= a:
        e += 1
    scaled = a * Fraction(2) ** (127 - e)
    m = scaled.numerator // scaled.denominator
    if upward and m * scaled.denominator != scaled.numerator:
        m += 1
    if m == 1 << 128:
        m, e = 1 << 127, e + 1
    return m, e


def floor_ceil128(lo, hi):
    """`Wide<2>` tokens f <= lo and c >= hi for Fractions lo <= hi of one sign."""
    assert lo <= hi
    if lo == 0 and hi == 0:
        return "Z+", "Z+"
    assert lo > 0 or hi < 0, (lo, hi)
    if lo > 0:
        f = _bracket_token(lo, False)
        c = _bracket_token(hi, True)
        return wide_token(False, f[1], f[0]), wide_token(False, c[1], c[0])
    f = _bracket_token(-lo, True)
    c = _bracket_token(-hi, False)
    return wide_token(True, f[1], f[0]), wide_token(True, c[1], c[0])


def sincos_exact_bracket(x):
    """Exact rational brackets of sin x and cos x for a dyadic 0 <= x <= 4:
    the Taylor polynomial of degree n - 1 plus or minus the Lagrange bound
    x^n/n!, with n the first index >= 4 whose term is below 2^-400 min(1, x)."""
    if x == 0:
        return (Fraction(0), Fraction(0)), (Fraction(1), Fraction(1))
    target = Fraction(1, 1 << 400) * min(Fraction(1), x)
    s = Fraction(0)
    c = Fraction(0)
    term = Fraction(1)
    k = 0
    while True:
        r = k % 4
        if r == 0:
            c += term
        elif r == 1:
            s += term
        elif r == 2:
            c -= term
        else:
            s -= term
        k += 1
        term = term * x / k
        if k >= 4 and term < target:
            return (s - term, s + term), (c - term, c + term)


def arg_token(x):
    """Exact token of a nonnegative dyadic Fraction."""
    if x == 0:
        return "Z+"
    e = x.numerator.bit_length() - x.denominator.bit_length()
    while Fraction(2) ** e > x:
        e -= 1
    while Fraction(2) ** (e + 1) <= x:
        e += 1
    scaled = x * Fraction(2) ** (127 - e)
    assert scaled.denominator == 1
    return wide_token(False, e, scaled.numerator)


# ---------------------------------------------------------------------------
# trig.txt
# ---------------------------------------------------------------------------

def trig_lines():
    pi_half = 1.5707963267948966
    pi = 3.141592653589793
    args = [
        ("zero", Fraction(0)),
        ("min_subnormal", Fraction(2) ** -1074),
        ("min_normal", Fraction(2) ** -1022),
        ("tiny_1e-300", Fraction(1e-300)),
        ("two_pow_-30", Fraction(2) ** -30),
        ("near_zero_1e-8", Fraction(1e-8)),
        ("quarter", Fraction(1, 4)),
        ("half", Fraction(1, 2)),
        ("one", Fraction(1)),
        ("one_minus_2^-60", 1 - Fraction(2) ** -60),
        ("one_plus_2^-60", 1 + Fraction(2) ** -60),
        ("pi_half_down", Fraction(math.nextafter(pi_half, 0))),
        ("pi_half", Fraction(pi_half)),
        ("pi_half_up", Fraction(math.nextafter(pi_half, 4))),
        ("two", Fraction(2)),
        ("three", Fraction(3)),
        ("pi_down", Fraction(math.nextafter(pi, 0))),
        ("pi", Fraction(pi)),
        ("arc_ARC106_phi", Fraction(1.8545904360032244)),
        ("arc_MOVED_phi", Fraction(1.5707963267948792)),
        ("arc_RISE74_phi", Fraction(1.2870022175865687)),
        ("two_peaks_phi", Fraction(3.093337152652465)),
        ("two_peaks_half_phi", Fraction(3.093337152652465) / 2),
    ]
    out = [
        "# T4-U4b enclosed sine and cosine: name arg sin(60 digits) cos(60 digits)"
        " sin_floor128 sin_ceil128 cos_floor128 cos_ceil128",
        "# generated by gen_arc_maximum_vectors.py (decimal, 110 digits, two methods)",
    ]
    for name, x in args:
        xd = D(x.numerator) / D(x.denominator)
        s1, c1 = sincos_series(xd)
        s2, c2 = sincos_doubling(xd)
        assert abs(s1 - s2) < D(10) ** -95 and abs(c1 - c2) < D(10) ** -95, name
        assert abs(s1 * s1 + c1 * c1 - 1) < D(10) ** -100, name
        (sl, sh), (cl, ch) = sincos_exact_bracket(x)
        assert Fraction(s1) - sl > -Fraction(1, 10 ** 100) and sh - Fraction(s1) > -Fraction(1, 10 ** 100), name
        assert Fraction(c1) - cl > -Fraction(1, 10 ** 100) and ch - Fraction(c1) > -Fraction(1, 10 ** 100), name
        s_lo, s_hi = floor_ceil128(sl, sh)
        c_lo, c_hi = floor_ceil128(cl, ch)
        text = [("0" if v == 0 else f"{v:.59e}") for v in (s1, c1)]
        out.append(" ".join([name, arg_token(x)] + text + [s_lo, s_hi, c_lo, c_hi]))
    return out


# ---------------------------------------------------------------------------
# The represented problem
# ---------------------------------------------------------------------------

def cross(a, v):
    return [a[1] * v[2] - a[2] * v[1], a[2] * v[0] - a[0] * v[2], a[0] * v[1] - a[1] * v[0]]


class Problem:
    """Decimal evaluation of the represented problem (module docs of arc_maximum.rs)."""

    def __init__(self, R, phi, axes, force, intensities, pa, area, z):
        self.R = D(R)
        self.phi = D(phi)
        # The chain of design section 2.3: the end-j rows through the frame
        # transpose to global, then into the same rounded frame's coordinates.
        E = [[D(x) for x in row] for row in axes]
        def chained(local):
            g = [sum(E[a][i] * D(local[a]) for a in range(3)) for i in range(3)]
            return [sum(E[a][i] * g[i] for i in range(3)) for a in range(3)]
        self.Fj = chained(force[:3])
        self.Mj = chained(force[3:])
        self.w = [D(0)] * 3
        for g, scale in intensities:
            for a in range(3):
                self.w[a] += D(scale) * sum(D(axes[a][i]) * D(g[i]) for i in range(3))
        self.pa = D(pa) if pa is not None else D(0)
        self.area = D(area)
        self.z = D(z)

    def peak(self, k0, intensified):
        """The stationary point of the measure near k0 (secant on the slope in
        psi), as an exact Decimal fraction u and the value there."""
        def slope_at(u):
            return self.measure_at(u, intensified)[1]
        u0 = D(k0) / D(FULL)
        u1 = u0 + D(10) ** -9
        s0, s1 = slope_at(u0), slope_at(u1)
        for _ in range(200):
            if s1 == s0:
                break
            u2 = u1 - s1 * (u1 - u0) / (s1 - s0)
            u0, s0 = u1, s1
            u1, s1 = u2, slope_at(u2)
            if abs(u1 - u0) < D(10) ** -80:
                break
        return u1, self.measure_at(u1, intensified)[0]

    def measure_at(self, u, intensified):
        return self._measure_k(u * D(FULL), intensified)

    def section_k(self, k):
        """[N_eff, V_y, V_z, T, M_y, M_z] at psi = (k/2^60) phi, k a Decimal."""
        delta = (D(FULL) - k) * self.phi / D(FULL)
        s, c = sincos(delta)
        R = self.R
        omc = 1 - c
        F = [self.Fj[i] + R * delta * self.w[i] for i in range(3)]
        d = [R * s, -R * omc, D(0)]
        b = [delta * s - omc, delta * c - s, D(0)]
        dF = cross(d, self.Fj)
        bw = cross(b, self.w)
        M = [self.Mj[i] + dF[i] + R * R * bw[i] for i in range(3)]
        return [c * F[0] - s * F[1] + self.pa, s * F[0] + c * F[1], F[2],
                c * M[0] - s * M[1], s * M[0] + c * M[1], M[2]]

    def _measure_k(self, k, intensified):
        """(value, slope in psi) at psi = (k/2^60) phi, k a Decimal."""
        delta = (D(FULL) - k) * self.phi / D(FULL)
        s, c = sincos(delta)
        R = self.R
        omc = 1 - c
        F = [self.Fj[i] + R * delta * self.w[i] for i in range(3)]
        d = [R * s, -R * omc, D(0)]
        b = [delta * s - omc, delta * c - s, D(0)]
        dF = cross(d, self.Fj)
        bw = cross(b, self.w)
        M = [self.Mj[i] + dF[i] + R * R * bw[i] for i in range(3)]
        N = c * F[0] - s * F[1] + self.pa
        Vy = s * F[0] + c * F[1]
        Vz = F[2]
        T = c * M[0] - s * M[1]
        My = s * M[0] + c * M[1]
        Mz = M[2]
        wt = c * self.w[0] - s * self.w[1]
        dN = -R * wt + Vy
        dMy = R * Vz - T
        dMz = -R * Vy
        if intensified:
            i_ip, i_op = intensified
            a, cc, da, dc = D(i_op) * My, D(i_ip) * Mz, D(i_op) * dMy, D(i_ip) * dMz
        else:
            a, cc, da, dc = My, Mz, dMy, dMz
        h = (a * a + cc * cc).sqrt()
        value = h / self.z
        # The derivative does not exist at a kink (zero bending or N_eff = 0).
        slope = (a * da + cc * dc) / (h * self.z) if h != 0 else None
        if not intensified:
            value += abs(N) / self.area
            if slope is not None and N != 0:
                slope += (1 if N > 0 else -1) * dN / self.area
            else:
                slope = None
        return value, slope


# ---------------------------------------------------------------------------
# Oracle arcs
# ---------------------------------------------------------------------------

STATION_FRACTIONS = (("end_i", 0), ("quarter_1", 1), ("midspan", 2), ("quarter_3", 3), ("end_j", 4))
WORST_STATION_RESULTANT = [D(0)]


def check_station_resultants(case, member, problem):
    """The decimal model against the oracle's 40-digit station resultants:
    within 1e-13 of the force or moment zero scale (the operands are the
    oracle's binary64 values, so about 1e-16 is expected)."""
    scales = case["zero_scales"][member]
    keys = ("V_y", "V_z", "T", "M_y", "M_z")
    for station, quarter in STATION_FRACTIONS:
        r = case["quantities"]["resultants_tangent_frame_j_side"][member][station]
        expected = [D((r["N_w"] if "N_w" in r else r["N"])[0])] + [D(r[k][0]) for k in keys]
        got = problem.section_k(D(quarter * (FULL // 4)))
        for i, (g, e) in enumerate(zip(got, expected)):
            scale = D(scales["force" if i < 3 else "moment"][0])
            err = abs(g - e) / max(abs(e), scale)
            WORST_STATION_RESULTANT[0] = max(WORST_STATION_RESULTANT[0], err)
            assert err < D(10) ** -13, (case["case_id"], member, station, i, err)

def oracle_arcs():
    oracle = json.loads((REFS / "oracle.json").read_text())
    cases = {c["case_id"]: c for c in json.loads((REFS / "cases.json").read_text())["cases"]}
    for cid, case in oracle["cases"].items():
        for mid, derived in case["derived"].items():
            if derived["kind"] != "realized_arc":
                continue
            r = case["quantities"]["resultants_tangent_frame_j_side"][mid]["end_j"]
            axial = (r["S"] if "S" in r else r["N"])[1]
            force = [axial, r["V_y"][1], r["V_z"][1], r["T"][1], r["M_y"][1], r["M_z"][1]]
            frame = derived["tangent_frames_global_components"]["end_j"]
            axes = [[float(x) for x in frame[a]] for a in ("x", "y", "z")]
            intensities = []
            for load in cases[cid]["load_case"].get("member_uniform_forces", []):
                if load["member"] != mid:
                    continue
                for i, value in enumerate(load["global_components_per_length"]):
                    if value != 0.0:
                        g = [0.0, 0.0, 0.0]
                        g[i] = value
                        intensities.append((g, 1.0))
            pa = derived["p_Ai"][1] if "p_Ai" in derived else None
            sif = derived["directional_sif"]
            factors = (sif["i_ip"][1], sif["i_op"][1])
            for basis, section in derived["section_bases"].items():
                problem = Problem(
                    derived["arc"]["R"][1],
                    derived["arc"]["included_angle_rad"][1],
                    axes, force, intensities, pa,
                    section["A"][1], section["Z"][1],
                )
                check_station_resultants(case, mid, problem)
                q = case["quantities"]
                for measure, key, inten in (
                    ("elastic", "member_maximum", None),
                    ("intensified", "intensified_bending_maximum", factors),
                ):
                    record = q[key][mid][basis]
                    yield cid, mid, basis, measure, inten, problem, record


# ---------------------------------------------------------------------------
# Two peaks
# ---------------------------------------------------------------------------

def two_peak_problem(m2, eps):
    """Family 2 of the symmetric search: w along the mid-arc inward normal,
    F_j = -R phi w / 2 + alpha t_mid, M_j = m3 t_mid + m2 z (+ eps n_mid,
    which breaks the symmetry). Operands rounded to binary64 here."""
    R = 1.0
    phi = 3.093337152652465
    sh, ch = sincos(D(phi) / 2)
    nmid = [float(sh), float(ch), 0.0]
    tmid = [float(ch), float(-sh), 0.0]
    a, alpha, m3 = 163.22959234518578, 461.7590055018404, -2121.8483799736186
    w = [float(D(a) * D(nmid[i])) for i in range(3)]
    fj = [float(-D(R) * D(phi) * D(w[i]) / 2 + D(alpha) * D(tmid[i])) for i in range(3)]
    mj = [float(D(m3) * D(tmid[i]) + D(eps) * D(nmid[i])) for i in range(2)] + [m2]
    axes = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
    operands = dict(R=R, phi=phi, w=w, force=fj + mj, pa=-4904.24761379587, area=0.01, z=1e-4)
    problem = Problem(R, phi, axes, fj + mj, [(w, 1.0)], operands["pa"], 0.01, 1e-4)
    return operands, problem


LEFT_PEAK_K = 430 * GRID


def tuned_m2():
    """m2 with the slope zero at the grid point 430/4096 (secant, then binary64)."""
    m0, m1 = -2747.353077413066, -2747.0
    def s(m):
        return two_peak_problem(m, 0.0)[1]._measure_k(D(LEFT_PEAK_K), None)[1]
    s0, s1 = s(m0), s(m1)
    for _ in range(100):
        m2 = float(D(m1) - s1 * (D(m1) - D(m0)) / (s1 - s0))
        if m2 == m1:
            break
        m0, s0, m1, s1 = m1, s1, m2, s(m2)
    return m1


def two_peak_lines():
    m2 = tuned_m2()
    out = [
        "# T4-U4b arc maximum location controls: a symmetric arc with two interior peaks on the grid",
        "# generated by gen_arc_maximum_vectors.py; operands in the end-j frame (identity axes)",
        "# variant name eps; operand name hex...; peak fraction value_floor value_ceil (left peak, then right)",
    ]
    variants = []
    for name, eps in (("tie", 0.0), ("left_high", -2.0e-5), ("right_high", 2.0e-5)):
        operands, problem = two_peak_problem(m2, eps)
        out.append(f"variant {name} {f64hex(eps)}")
        out.append("operand R " + f64hex(operands["R"]))
        out.append("operand phi " + f64hex(operands["phi"]))
        out.append("operand w " + " ".join(f64hex(x) for x in operands["w"]))
        out.append("operand force " + " ".join(f64hex(x) for x in operands["force"]))
        out.append("operand pa " + f64hex(operands["pa"]))
        out.append("operand area " + f64hex(operands["area"]))
        out.append("operand z " + f64hex(operands["z"]))
        peaks = []
        for k0 in (LEFT_PEAK_K, FULL - LEFT_PEAK_K):
            u, v = problem.peak(k0, None)
            lo, hi = floor_ceil64(v)
            peaks.append((u, v))
            out.append(f"peak {f64hex(float(u))} {f64hex(lo)} {f64hex(hi)}")
        variants.append((name, problem, peaks))
        # The peaks are the global maximum: dense check.
        n = 4096
        dense = max(problem._measure_k(D(j * GRID), None)[0] for j in range(0, n + 1, 4))
        assert dense <= max(p[1] for p in peaks), name
    tie = variants[0][2]
    assert abs(tie[0][1] - tie[1][1]) / tie[0][1] < D(10) ** -14
    assert abs(tie[0][0] - D(LEFT_PEAK_K) / D(FULL)) < D(10) ** -14
    assert abs(tie[1][0] - D(FULL - LEFT_PEAK_K) / D(FULL)) < D(10) ** -14
    left, right = variants[1][2], variants[2][2]
    for (lo_peak, hi_peak) in ((left[1], left[0]), (right[0], right[1])):
        rel = (hi_peak[1] - lo_peak[1]) / hi_peak[1]
        assert D(10) ** -10 < rel < D(10) ** -8, rel
    return out, variants


# ---------------------------------------------------------------------------
# Codex's rotated small-angle torsion control (K2 review)
# ---------------------------------------------------------------------------

CODEX_FRAME = [
    [0.7816391739070251, -0.4829292842142122, 0.3947397981737998],
    [-0.5501172307043584, -0.8320301337746346, 0.07139249941787586],
    [0.29395787843858057, -0.2729563388883143, -0.9160150668873173],
]


def represented_maximum(problem, intensified=None, n=256):
    """The maximum of a represented problem: the best of n + 1 points, refined
    by the secant on the slope when interior."""
    best = max((problem._measure_k(D(j * (FULL // n)), intensified)[0], j) for j in range(n + 1))
    if best[1] in (0, n):
        return D(best[1]) / n, best[0]
    return problem.peak(best[1] * (FULL // n), intensified)


def codex_lines():
    force = [0.0, 0.0, 0.0, 1.0e8, 0.0, 0.0]
    chained = Problem(1.0, 1e-5, CODEX_FRAME, force, [], None, 1.0, 1.0)
    direct = Problem(1.0, 1e-5, [[1.0, 0, 0], [0, 1.0, 0], [0, 0, 1.0]], force, [], None, 1.0, 1.0)
    u, v = represented_maximum(chained)
    ud, vd = represented_maximum(direct)
    # Codex's figures: 999.999999980514384... and 999.999999983333415...
    assert abs(v - D("999.999999980514384")) < D(10) ** -15, v
    assert abs(vd - D("999.999999983333415")) < D(10) ** -15, vd
    out = []
    lo, hi = floor_ceil64(v)
    out.append(" ".join(["codex", "transposed_chain", f64hex(float(u)), f64hex(lo), f64hex(hi)]))
    lo, hi = floor_ceil64(vd)
    out.append(" ".join(["codex", "direct_local", f64hex(float(ud)), f64hex(lo), f64hex(hi)]))
    return out


# ---------------------------------------------------------------------------
# end_controls.txt: the design's location order
# ---------------------------------------------------------------------------

END_BASES = {
    # A governing end j (f'(phi) > 0) with an interior peak 1e-14 lower.
    "end_j_governs": dict(
        force=[-1187.5279061990345, -1625.7677763103652, 1044.7702996564012,
               210.34051754725851, -789.0080039458173, 1568.616112410351],
        w=[615.3633628910527, 66.3571634702912, 781.4929415006511],
        R=1.0, phi=2.4829425449852236, pa=2015.7797600477497, peak=1432, end="j"),
    # A governing interior peak with end i (f'(0) < 0) 1e-14 lower.
    "interior_governs_over_end_i": dict(
        force=[-1170.3589391669382, 917.5522148115087, -1993.350818531865,
               577.9170482784316, 1264.5180475014668, -1039.81846768052],
        w=[-567.5169258943125, -355.4631254699995, 136.81989992541105],
        R=0.5, phi=2.4995316041719082, pa=1747.280593977035, peak=2488, end="i"),
}
IDENTITY = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
END_SPLIT = D(10) ** -14


def end_problem(base, mx, mz):
    force = list(base["force"])
    force[3], force[5] = mx, mz
    return Problem(base["R"], base["phi"], IDENTITY, force, [(base["w"], 1.0)], base["pa"], 0.01, 1e-4), force


def end_residual(base, mx, mz):
    problem, _ = end_problem(base, mx, mz)
    kp = D(base["peak"] * GRID)
    value, slope = problem._measure_k(kp, None)
    end_value = problem._measure_k(D(FULL) if base["end"] == "j" else D(0), None)[0]
    if base["end"] == "j":
        return [slope, end_value - (1 + END_SPLIT) * value]
    return [slope, value - (1 + END_SPLIT) * end_value]


def tune_end_control(base):
    """Newton on (M_x, M_z) at end j: the slope is zero at the grid point
    `peak`/4096 and the value split is 1e-14 relative; then binary64."""
    p = [D(base["force"][3]), D(base["force"][5])]
    for _ in range(60):
        r = end_residual(base, *p)
        jac = []
        for i in range(2):
            h = abs(p[i]) * D(10) ** -40 + D(10) ** -40
            q = list(p)
            q[i] += h
            rq = end_residual(base, *q)
            jac.append([(rq[j] - r[j]) / h for j in range(2)])
        # Solve J^T-layout: jac[i][j] = d r_j / d p_i.
        det = jac[0][0] * jac[1][1] - jac[1][0] * jac[0][1]
        dp0 = (-r[0] * jac[1][1] + r[1] * jac[1][0]) / det
        dp1 = (-r[1] * jac[0][0] + r[0] * jac[0][1]) / det
        p = [p[0] + dp0, p[1] + dp1]
        if abs(dp0) + abs(dp1) < D(10) ** -60:
            break
    return float(p[0]), float(p[1])


def end_control_lines():
    out = [
        "# T4-U4b arc maximum location order controls (design section 2.3 step 6)",
        "# generated by gen_arc_maximum_vectors.py; operands in the end-j frame (identity axes)",
        "# variant name; operand name hex...; expect kind fraction_hex",
    ]
    for name, base in END_BASES.items():
        mx, mz = tune_end_control(base)
        problem, force = end_problem(base, mx, mz)
        kp = D(base["peak"] * GRID)
        value, slope = problem._measure_k(kp, None)
        end_k = D(FULL) if base["end"] == "j" else D(0)
        end_value, end_slope = problem._measure_k(end_k, None)
        split = (end_value - value) / value if base["end"] == "j" else (value - end_value) / end_value
        assert D("0.5e-14") < split < D("2e-14"), (name, split)
        assert abs(slope) < D(10) ** -6, (name, slope)
        assert (end_slope > 0) if base["end"] == "j" else (end_slope < 0), name
        top = max(value, end_value)
        for j in range(4097):
            if j * GRID not in (int(kp), int(end_k)):
                assert problem._measure_k(D(j * GRID), None)[0] < top, (name, j)
        out.append(f"variant {name}")
        out.append("operand R " + f64hex(base["R"]))
        out.append("operand phi " + f64hex(base["phi"]))
        out.append("operand w " + " ".join(f64hex(x) for x in base["w"]))
        out.append("operand force " + " ".join(f64hex(x) for x in force))
        out.append("operand pa " + f64hex(base["pa"]))
        out.append("operand area " + f64hex(0.01))
        out.append("operand z " + f64hex(1e-4))
        if base["end"] == "j":
            out.append("expect end_j " + f64hex(1.0))
        else:
            out.append("expect interior " + f64hex(base["peak"] / 4096))
    return out


# ---------------------------------------------------------------------------
# directed.txt
# ---------------------------------------------------------------------------

def directed_lines(variants):
    out = [
        "# T4-U4b arc maximum directedness: point case member basis measure k value_floor value_ceil slope_floor slope_ceil",
        "#   and: maximum case member basis measure fraction value_floor value_ceil",
        "# generated by gen_arc_maximum_vectors.py (decimal, 110 digits, the represented binary64 problem)",
    ]
    def emit(cid, mid, basis, measure, inten, problem):
        for j in range(9):
            k = j * (FULL // 8)
            value, slope = problem._measure_k(D(k), inten)
            v_lo, v_hi = floor_ceil64(value)
            if slope is None:
                slope_tokens = ["none", "none"]
            else:
                s_lo, s_hi = floor_ceil64(slope)
                slope_tokens = [f64hex(s_lo), f64hex(s_hi)]
            out.append(" ".join(["point", cid, mid, basis, measure, str(k), f64hex(v_lo), f64hex(v_hi)] + slope_tokens))
    for cid, mid, basis, measure, inten, problem, record in oracle_arcs():
        emit(cid, mid, basis, measure, inten, problem)
        loc = record["location"]
        if loc["kind"] == "end_i":
            u, v = D(0), problem._measure_k(D(0), inten)[0]
        elif loc["kind"] == "end_j":
            u, v = D(1), problem._measure_k(D(FULL), inten)[0]
        else:
            k0 = int(D(loc["fraction"][0]) * D(FULL))
            u, v = problem.peak(k0, inten)
        expected = D(record["value"][0])
        assert abs(v - expected) <= D(10) ** -12 * expected, (cid, mid, basis, measure, v, expected)
        lo, hi = floor_ceil64(v)
        out.append(" ".join(["maximum", cid, mid, basis, measure, f64hex(float(u)), f64hex(lo), f64hex(hi)]))
    for name, problem, peaks in variants:
        emit("TWO-PEAKS", name, "synthetic", "elastic", None, problem)
    out.extend(codex_lines())
    return out


def generate():
    files = {}
    files["trig.txt"] = "\n".join(trig_lines()) + "\n"
    peaks, variants = two_peak_lines()
    files["two_peaks.txt"] = "\n".join(peaks) + "\n"
    files["directed.txt"] = "\n".join(directed_lines(variants)) + "\n"
    files["end_controls.txt"] = "\n".join(end_control_lines()) + "\n"
    print(f"decimal model vs oracle 40-digit station resultants: worst {float(WORST_STATION_RESULTANT[0]):.3e} of the zero scale")
    sums = "".join(f"{hashlib.sha256(files[n].encode()).hexdigest()}  {n}\n" for n in sorted(files))
    files["SHA256SUMS"] = sums
    return files


def main():
    check = "--check" in sys.argv[1:]
    files = generate()
    if check:
        bad = [n for n, text in files.items() if (HERE / n).read_text() != text]
        if bad:
            print("MISMATCH:", ", ".join(bad))
            sys.exit(1)
        print("check: all files match")
        return
    for name, text in files.items():
        (HERE / name).write_text(text)
    print("wrote", ", ".join(sorted(files)))


if __name__ == "__main__":
    main()
