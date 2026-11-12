"""Hydrostatic test references: held supports, test contents and test pressure on the exact route.

Independent reference generator. It imports, reads and runs no product code and
uses the Python standard library only. Every input is an invented exact
rational. Every derived value is an element of Q(pi): section areas and second
moments carry pi, spring rates and forces do not, so a reaction is in general
a ratio of polynomials in pi. The generator works exactly in that field
(polynomials in pi with Fraction coefficients, reduced by their gcd) and
evaluates a value only at output: lower and upper rational bounds of pi
(Machin's formula in integer arithmetic, 100 digits) give a rigorous interval,
the binary64 value is emitted only when both ends of the interval round to the
same double, and the decimal is printed to 40 significant digits.

Each value is computed by a closed form and checked exactly against a second
method written here: a beam stiffness solve (Euler-Bernoulli or exact
Timoshenko two-node elements, work-consistent loads, exact Gaussian
elimination), the flexibility method on the simply supported base span, the
three-moment equation, and for the thick-wall stresses the contract form of
DEL-05-03 and the hoop-force equilibrium of the half cylinder. Vertical force
and moment equilibrium are checked for every case. Finally every input is
replaced, one at a time and all together, by its binary64 value, and the sign
of every emitted row must not change.

Modes:
  python3 -I hydrostatic_test_reference.py           check: rebuild every output in memory
                                                      and compare byte for byte with the
                                                      committed files, SHA256SUMS included
  python3 -I hydrostatic_test_reference.py --write   write references.json and SHA256SUMS
"""
import argparse
import hashlib
import json
import sys
from decimal import Decimal, localcontext
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
GENERATOR = "generators/hydrostatic_test_reference.py"
FIXED_FILES = ("README.md", GENERATOR)


# ============================================================================ pi bounds
def _arctan_inv_scaled(x, scale):
    """floor-ish of arctan(1/x) * scale by the alternating Gregory series in integers."""
    total, power, n, sign = 0, scale // x, 1, 1
    x2 = x * x
    while power:
        total += sign * (power // n)
        power //= x2
        n += 2
        sign = -sign
    return total


def _pi_bounds(digits):
    guard = 12
    scale = 10 ** (digits + guard)
    p = 16 * _arctan_inv_scaled(5, scale) - 4 * _arctan_inv_scaled(239, scale)
    # each integer division loses less than one unit; a few hundred terms lose far less than 10**guard units
    mid = F(p, scale)
    return mid - F(1, 10 ** digits), mid + F(1, 10 ** digits)


PI_LO, PI_HI = _pi_bounds(100)
_KNOWN = F(31415926535897932384626433832795028841971693993751, 10 ** 49)  # pi truncated to 49 decimals
assert PI_LO < _KNOWN + F(1, 10 ** 49) and PI_HI > _KNOWN, "pi bounds disagree with the known digits"
assert PI_HI - PI_LO == F(2, 10 ** 100)


# ============================================================================ the field Q(pi)
def _trim(seq):
    p = [F(c) for c in seq]
    while p and p[-1] == 0:
        p.pop()
    return tuple(p)


def _padd(a, b):
    n = max(len(a), len(b))
    return _trim((a[i] if i < len(a) else 0) + (b[i] if i < len(b) else 0) for i in range(n))


def _pmul(a, b):
    if not a or not b:
        return ()
    out = [F(0)] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] += x * y
    return _trim(out)


def _pdivmod(a, b):
    quotient = [F(0)] * max(len(a) - len(b) + 1, 1)
    rem = list(a)
    while rem and len(rem) >= len(b):
        c = rem[-1] / b[-1]
        shift = len(rem) - len(b)
        quotient[shift] = c
        for i, y in enumerate(b):
            rem[shift + i] -= c * y
        rem = list(_trim(rem))
    return _trim(quotient), _trim(rem)


def _pgcd(a, b):
    while b:
        a, b = b, _pdivmod(a, b)[1]
    return tuple(c / a[-1] for c in a)


class Q:
    """An element N(pi)/D(pi) of Q(pi), kept with gcd(N, D) = 1 and D monic."""
    __slots__ = ("n", "d")

    def __init__(self, n, d=(1,)):
        n, d = _trim(n), _trim(d)
        if not d:
            raise ZeroDivisionError("denominator polynomial is zero")
        if not n:
            self.n, self.d = (), (F(1),)
            return
        g = _pgcd(n, d)
        if len(g) > 1:
            n, d = _pdivmod(n, g)[0], _pdivmod(d, g)[0]
        lead = d[-1]
        self.n = tuple(c / lead for c in n)
        self.d = tuple(c / lead for c in d)

    @staticmethod
    def of(x):
        return x if isinstance(x, Q) else Q((F(x),))

    def __add__(self, o):
        o = Q.of(o)
        return Q(_padd(_pmul(self.n, o.d), _pmul(o.n, self.d)), _pmul(self.d, o.d))

    __radd__ = __add__

    def __neg__(self):
        return Q(tuple(-c for c in self.n), self.d)

    def __sub__(self, o):
        return self + (-Q.of(o))

    def __rsub__(self, o):
        return Q.of(o) + (-self)

    def __mul__(self, o):
        o = Q.of(o)
        return Q(_pmul(self.n, o.n), _pmul(self.d, o.d))

    __rmul__ = __mul__

    def __truediv__(self, o):
        o = Q.of(o)
        if not o.n:
            raise ZeroDivisionError("division by zero in Q(pi)")
        return Q(_pmul(self.n, o.d), _pmul(self.d, o.n))

    def __rtruediv__(self, o):
        return Q.of(o) / self

    def __pow__(self, k):
        out = Q.of(1)
        for _ in range(k):
            out = out * self
        return out

    def __eq__(self, o):
        o = Q.of(o)
        return self.n == o.n and self.d == o.d

    __hash__ = None

    def is_zero(self):
        return not self.n


PI = Q((0, 1))
ZERO = Q.of(0)


def _poly_bounds(p):
    lo = hi = F(0)
    for i, c in enumerate(p):
        x, y = c * PI_LO ** i, c * PI_HI ** i
        lo += min(x, y)
        hi += max(x, y)
    return lo, hi


def bounds(x):
    x = Q.of(x)
    nlo, nhi = _poly_bounds(x.n)
    dlo, dhi = _poly_bounds(x.d)
    if dlo <= 0 <= dhi:
        raise SystemExit("denominator interval contains zero")
    cands = [nlo / dlo, nlo / dhi, nhi / dlo, nhi / dhi]
    return min(cands), max(cands)


def sign(x):
    x = Q.of(x)
    if x.is_zero():
        return 0
    lo, hi = bounds(x)
    if lo > 0:
        return 1
    if hi < 0:
        return -1
    raise SystemExit("sign not decided by the pi bounds")


def binary64(x):
    lo, hi = bounds(x)
    if float(lo) != float(hi):
        raise SystemExit("binary64 rounding not decided by the pi bounds")
    return float(lo)


def decimal40(x):
    x = Q.of(x)
    if x.is_zero():
        return "0"
    lo, hi = bounds(x)
    mid = (lo + hi) / 2
    with localcontext() as ctx:
        ctx.prec = 60
        v = Decimal(mid.numerator) / Decimal(mid.denominator)
        return format(v, ".39e")


def _poly_text(p):
    terms = []
    for i, c in enumerate(p):
        if c == 0:
            continue
        terms.append(str(c) if i == 0 else f"{c}*pi" if i == 1 else f"{c}*pi^{i}")
    return " + ".join(terms).replace("+ -", "- ")


def exact_text(x):
    x = Q.of(x)
    if x.is_zero():
        return "0"
    if x.d == (1,):
        nonzero = [i for i, c in enumerate(x.n) if c != 0]
        if len(nonzero) == 1:
            i = nonzero[0]
            c = x.n[i]
            return str(c) if i == 0 else f"{c}*pi" if i == 1 else f"{c}*pi^{i}"
        return _poly_text(x.n)
    if len(x.n) == 1 and all(c == 0 for c in x.d[:-1]):
        k = len(x.d) - 1
        return f"{x.n[0]}/pi" if k == 1 else f"{x.n[0]}/pi^{k}"
    return f"({_poly_text(x.n)})/({_poly_text(x.d)})"


def value(x, **meta):
    """An emitted value: metadata, then the exact element, its 40-digit decimal and its nearest binary64."""
    rec = dict(meta)
    rec.update({"exact": exact_text(x), "decimal": decimal40(x), "value": binary64(x)})
    return rec


# ============================================================================ inputs (invented)
# Every input is the user's. None is a standard, catalogue or code value; the
# dimensions are deliberately off any schedule and the two pressures are just
# two authored pressures (no factor relates them).
INPUTS = {
    "D": (F(29, 100), "m", "outside diameter"),
    "t": (F(27, 2000), "m", "nominal wall"),
    "c_m": (F(17, 10000), "m", "mill tolerance (absolute thickness)"),
    "c_a": (F(21, 10000), "m", "corrosion allowance (absolute thickness)"),
    "t_i": (F(13, 200), "m", "insulation thickness"),
    "rho_m": (F(7830), "kg/m^3", "metal density"),
    "rho_c_section": (F(865), "kg/m^3", "the section's own contents density (ordinary cases)"),
    "rho_c_test": (F(1015), "kg/m^3", "contents density of the condition the test case selects"),
    "rho_i": (F(135), "kg/m^3", "insulation density"),
    "E": (F(203 * 10 ** 9), "Pa", "elastic modulus"),
    "nu": (F(29, 100), "1", "Poisson's ratio (exact route); G = E/(2(1 + nu))"),
    "g": (F(979, 100), "m/s^2", "gravity magnitude, along global -Z"),
    "p_ordinary": (F(4700000), "Pa", "authored pressure of the ordinary cases"),
    "p_test": (F(7300000), "Pa", "authored pressure of the hydrostatic test cases"),
    "a": (F(43, 10), "m", "span length of the two- and four-span models"),
    "L1": (F(57, 10), "m", "span length of the single-span model"),
    "k_2s": (F(1350000), "N/m", "two-span model: middle spring rate"),
    "kv_2s": (F(960000), "N/m", "two-span model: middle variable spring rate"),
    "Fc_2s": (F(4800), "N", "two-span model: middle variable spring cold load"),
    "Fce_2s": (F(5600), "N", "two-span model: middle constant-effort force"),
    "k_4s_b": (F(1180000), "N/m", "four-span model: spring rate at B"),
    "Fce_4s_c": (F(6900), "N", "four-span model: constant-effort force at C"),
    "kv_4s_d": (F(870000), "N/m", "four-span model: variable spring rate at D"),
    "Fc_4s_d": (F(5300), "N", "four-span model: variable spring cold load at D"),
}
NOMINAL = {k: v[0] for k, v in INPUTS.items()}


# ============================================================================ checks
class Checks:
    def __init__(self, record):
        self.record = record
        self.items = []
        self.failed = 0

    def eq(self, name, x, y):
        ok = Q.of(x) == Q.of(y)
        if not ok:
            self.failed += 1
        if self.record:
            self.items.append({"check": name, "agrees": ok})
        return ok


# ============================================================================ section, mass, Lame
def derive(inp, chk):
    s = {}
    R = inp["D"] / 2
    t_r = (inp["t"] - inp["c_m"]) - inp["c_a"]
    r_n = R - inp["t"]
    r_p = R - t_r
    s.update(R=R, t_r=t_r, r_n=r_n, r_p=r_p)
    s["A_nominal"] = PI * (R * R - r_n * r_n)
    s["A_reduced"] = PI * (R * R - r_p * r_p)
    s["A_bore"] = PI * (r_p * r_p)
    s["A_insulation"] = PI * ((R + inp["t_i"]) ** 2 - R * R)
    s["I_nominal"] = PI * (R ** 4 - r_n ** 4) / 4
    s["EI"] = inp["E"] * s["I_nominal"]
    nu = inp["nu"]
    s["G"] = inp["E"] / (2 * (1 + nu))
    m = r_n / R
    m2 = m * m
    aa, bb, n1 = 1 + m2, 1 - m2, 1 + nu
    c = (8 * nu + 14) * nu + 7
    s["kappa_E"] = (6 * (n1 * n1)) * (aa * aa) / (c * (bb * bb) + 48 * (m2 * (n1 * n1)))
    s["GA_s"] = s["G"] * s["kappa_E"] * s["A_nominal"]
    # mass per length on the exact route's basis source_od_nominal_wall_pressure_bore_areas/v1
    s["mu_metal"] = inp["rho_m"] * s["A_nominal"]
    s["mu_insulation"] = inp["rho_i"] * s["A_insulation"]
    s["mu_contents_section"] = inp["rho_c_section"] * s["A_bore"]
    s["mu_contents_test"] = inp["rho_c_test"] * s["A_bore"]
    g = inp["g"]
    s["w_ordinary"] = g * (s["mu_metal"] + s["mu_contents_section"] + s["mu_insulation"])
    s["w_test"] = g * (s["mu_metal"] + s["mu_contents_test"] + s["mu_insulation"])
    s["w_double_counted"] = g * (s["mu_metal"] + s["mu_contents_section"] + s["mu_contents_test"] + s["mu_insulation"])
    s["w_contents_omitted"] = g * (s["mu_metal"] + s["mu_insulation"])
    # second route to the weights: the whole outside-insulation disc less the voids, by area composition
    disc = PI * (R + inp["t_i"]) ** 2
    chk.eq("areas: insulation + nominal metal + nominal bore = outer disc",
           s["A_insulation"] + s["A_nominal"] + PI * r_n * r_n, disc)
    chk.eq("areas: nominal metal + (bore growth by c_m + c_a) + pressure bore = nominal metal + pi r_p^2",
           s["A_nominal"] + PI * (r_p * r_p - r_n * r_n) + PI * r_n * r_n, s["A_nominal"] + s["A_bore"])
    chk.eq("weights: test - ordinary = g (rho_test - rho_section) A_bore",
           s["w_test"] - s["w_ordinary"], g * (inp["rho_c_test"] - inp["rho_c_section"]) * s["A_bore"])
    chk.eq("weights: double counted - test = section contents weight",
           s["w_double_counted"] - s["w_test"], g * s["mu_contents_section"])
    # alternative reading (README, open reading 4): contents on the nominal bore pi r_n^2
    s["alt_w_ordinary_nominal_bore"] = g * (s["mu_metal"] + inp["rho_c_section"] * PI * r_n * r_n + s["mu_insulation"])
    s["alt_w_test_nominal_bore"] = g * (s["mu_metal"] + inp["rho_c_test"] * PI * r_n * r_n + s["mu_insulation"])
    return s


def lame(p, R, r_i):
    den = R * R - r_i * r_i
    return {
        "hoop_bore": p * (R * R + r_i * r_i) / den,
        "hoop_outside": 2 * p * r_i * r_i / den,
        "radial_bore": -p,
        "radial_outside": F(0),
    }


def lame_checks(p, R, r_i, area_b, chk, tag):
    v = lame(p, R, r_i)
    # DEL-05-03 contract form: outer hoop 2 p pi r_i^2 / A_b, inner hoop that plus p
    chk.eq(f"{tag}: outer hoop = 2 p pi r_i^2 / A_b", v["hoop_outside"], 2 * p * PI * r_i * r_i / area_b)
    chk.eq(f"{tag}: bore hoop = outer hoop + p", v["hoop_bore"], 2 * p * PI * r_i * r_i / area_b + p)
    # Lame: sigma = A -/+ B/r^2, A = p r_i^2/(R^2 - r_i^2), B = A R^2; boundary tractions
    A_l = p * r_i * r_i / (R * R - r_i * r_i)
    B_l = A_l * R * R
    chk.eq(f"{tag}: radial(r_i) = A - B/r_i^2 = -p", A_l - B_l / (r_i * r_i), v["radial_bore"])
    chk.eq(f"{tag}: radial(R) = A - B/R^2 = 0", A_l - B_l / (R * R), v["radial_outside"])
    chk.eq(f"{tag}: hoop(r_i) = A + B/r_i^2", A_l + B_l / (r_i * r_i), v["hoop_bore"])
    # equilibrium of a half cylinder of unit length: wall hoop force int_{r_i}^{R} sigma_h dr = p r_i
    chk.eq(f"{tag}: int sigma_h dr over the wall = p r_i", A_l * (R - r_i) + B_l * (1 / r_i - 1 / R), p * r_i)
    # hoop + radial is the same constant 2A at both surfaces
    chk.eq(f"{tag}: hoop + radial equal at bore and outside",
           v["hoop_bore"] + v["radial_bore"], v["hoop_outside"] + v["radial_outside"])
    return v


# ============================================================================ beam solvers
# A straight run along global X of n equal spans of length a, nodes 0..n, pinned for
# bending at both ends (vertical restraint, rotation free), loaded by a uniform load
# per length w along global -Z. Interior node laws (the support's effect on the pipe):
#   ("rigid",)            vertical displacement 0
#   ("at", v)             vertical displacement v (a negative control only)
#   ("spring", k)         force on pipe -k u
#   ("vspring", k, Fc)    force on pipe Fc - k u
#   ("ce", Fce)           force on pipe Fce
#   ("free",)             nothing
# Reactions are the vertical forces of the supports on the pipe, + along +Z.


def solve_linear(A, b):
    n = len(b)
    M = [list(row) + [b[i]] for i, row in enumerate(A)]
    for col in range(n):
        piv = next(r for r in range(col, n) if not Q.of(M[r][col]).is_zero())
        M[col], M[piv] = M[piv], M[col]
        p = M[col][col]
        for r in range(col + 1, n):
            if Q.of(M[r][col]).is_zero():
                continue
            fct = M[r][col] / p
            M[r] = [M[r][j] - fct * M[col][j] for j in range(n + 1)]
    x = [ZERO] * n
    for r in reversed(range(n)):
        acc = M[r][n]
        for j in range(r + 1, n):
            acc = acc - M[r][j] * x[j]
        x[r] = acc / M[r][r]
    return x


def stiffness_solve(n, a, w, EI, GAs, laws):
    """Two-node beam elements (v, theta) per node; exact for these loads in both theories."""
    N = 2 * (n + 1)
    K = [[ZERO] * N for _ in range(N)]
    P = [ZERO] * N
    phi = ZERO if GAs is None else 12 * EI / (GAs * a * a)
    c = EI / ((1 + phi) * a ** 3)
    L = Q.of(a)
    ke = [[12, 6 * L, -12, 6 * L],
          [6 * L, (4 + phi) * L * L, -6 * L, (2 - phi) * L * L],
          [-12, -6 * L, 12, -6 * L],
          [6 * L, (2 - phi) * L * L, -6 * L, (4 + phi) * L * L]]
    q = -w
    fe = [q * a / 2, q * a * a / 12, q * a / 2, -q * a * a / 12]
    for e in range(n):
        dofs = [2 * e, 2 * e + 1, 2 * e + 2, 2 * e + 3]
        for i in range(4):
            P[dofs[i]] = P[dofs[i]] + fe[i]
            for j in range(4):
                K[dofs[i]][dofs[j]] = K[dofs[i]][dofs[j]] + c * ke[i][j]
    fixed = {0: ZERO, 2 * n: ZERO}
    for node, law in laws.items():
        dof = 2 * node
        if law[0] == "rigid":
            fixed[dof] = ZERO
        elif law[0] == "at":
            fixed[dof] = Q.of(law[1])
        elif law[0] == "spring":
            K[dof][dof] = K[dof][dof] + law[1]
        elif law[0] == "vspring":
            K[dof][dof] = K[dof][dof] + law[1]
            P[dof] = P[dof] + law[2]
        elif law[0] == "ce":
            P[dof] = P[dof] + law[1]
    free = [i for i in range(N) if i not in fixed]
    A = [[K[i][j] for j in free] for i in free]
    b = [P[i] - sum((K[i][j] * fixed[j] for j in fixed), ZERO) for i in free]
    uf = solve_linear(A, b)
    u = [ZERO] * N
    for i, dof in enumerate(free):
        u[dof] = uf[i]
    for dof, val in fixed.items():
        u[dof] = val
    reactions = []
    for node in range(n + 1):
        dof = 2 * node
        law = laws.get(node, ("rigid",))
        if dof in fixed:
            reactions.append(sum((K[dof][j] * u[j] for j in range(N)), ZERO) - P[dof])
        elif law[0] == "spring":
            reactions.append(-law[1] * u[dof])
        elif law[0] == "vspring":
            reactions.append(law[2] - law[1] * u[dof])
        elif law[0] == "ce":
            reactions.append(Q.of(law[1]))
        else:
            reactions.append(ZERO)
    return reactions, [u[2 * i] for i in range(n + 1)]


def flexibility_solve(n, a, w, EI, GAs, laws):
    """Base: simple span of length n a; interior forces X_j as redundants (downward deflection positive)."""
    L = n * a

    def d0(x):
        v = w * x * (L ** 3 - 2 * L * x * x + x ** 3) / (24 * EI)
        return v if GAs is None else v + w * x * (L - x) / (2 * GAs)

    def fl(xi, xj):
        if xi > xj:
            xi, xj = xj, xi
        b = L - xj
        v = b * xi * (L * L - b * b - xi * xi) / (6 * EI * L)
        return v if GAs is None else v + b * xi / (L * GAs)

    nodes = list(range(1, n))
    xs = {i: i * a for i in nodes}
    A, rhs = [], []
    for i in nodes:
        law = laws.get(i, ("free",))
        if law[0] in ("rigid", "at"):
            A.append([fl(xs[i], xs[j]) for j in nodes])
            rhs.append(d0(xs[i]) + (law[1] if law[0] == "at" else 0))
        elif law[0] in ("spring", "vspring"):
            k = law[1]
            A.append([(1 if i == j else 0) + k * fl(xs[i], xs[j]) for j in nodes])
            rhs.append(k * d0(xs[i]) + (law[2] if law[0] == "vspring" else 0))
        elif law[0] == "ce":
            A.append([Q.of(1 if i == j else 0) for j in nodes])
            rhs.append(Q.of(law[1]))
        else:
            A.append([Q.of(1 if i == j else 0) for j in nodes])
            rhs.append(ZERO)
    X = solve_linear(A, rhs) if nodes else []
    Xd = dict(zip(nodes, X))
    R0 = (w * L * L / 2 - sum((Xd[j] * (L - xs[j]) for j in nodes), ZERO)) / L
    Rn = w * L - R0 - sum(X, ZERO)
    reactions = [R0] + X + [Rn]
    u = [ZERO] + [-(d0(xs[i]) - sum((fl(xs[i], xs[j]) * Xd[j] for j in nodes), ZERO)) for i in nodes] + [ZERO]
    return reactions, u


def three_moment_rigid(n, a, w):
    """Euler-Bernoulli, equal spans, all supports rigid and level: M_{i-1} + 4 M_i + M_{i+1} = -w a^2 / 2."""
    if n == 1:
        return [w * a / 2, w * a / 2]
    m = n - 1
    A = [[Q.of(4 if i == j else 1 if abs(i - j) == 1 else 0) for j in range(m)] for i in range(m)]
    M = [ZERO] + solve_linear(A, [-w * a * a / 2] * m) + [ZERO]
    R = []
    for i in range(n + 1):
        left = w * a / 2 - (M[i] - M[i - 1]) / a if i > 0 else ZERO
        right = w * a / 2 + (M[i + 1] - M[i]) / a if i < n else ZERO
        R.append(left + right)
    return R


def two_span_closed(a, w, EI, GAs, law):
    """Closed form of the two-span beam: delta0 and f of the simple span 2a at midspan."""
    d0 = 5 * w * a ** 4 / (24 * EI) + (ZERO if GAs is None else w * a * a / (2 * GAs))
    f = a ** 3 / (6 * EI) + (ZERO if GAs is None else a / (2 * GAs))
    kind = law[0]
    if kind == "rigid":
        RB = d0 / f
    elif kind == "at":
        RB = (d0 + law[1]) / f
    elif kind == "spring":
        RB = d0 / (f + Q.of(1) / law[1])
    elif kind == "vspring":
        RB = (d0 + Q.of(law[2]) / law[1]) / (f + Q.of(1) / law[1])
    elif kind == "ce":
        RB = Q.of(law[1])
    else:
        raise ValueError(kind)
    RA = w * a - RB / 2
    uB = -(d0 - f * RB)
    return [RA, RB, RA], [ZERO, uB, ZERO], d0, f


# ============================================================================ models and cases
NODE_NAMES = "abcde"
HELD_FAMILIES = ("spring", "variable_spring", "constant_effort")


def model_laws(model, inp):
    """Interior supports of a model: node -> (support id, family, law when acting)."""
    if model == "M1":
        return {}
    if model == "M2-SPRING":
        return {1: ("support:b", "spring", ("spring", inp["k_2s"]))}
    if model == "M2-VSPRING":
        return {1: ("support:b", "variable_spring", ("vspring", inp["kv_2s"], inp["Fc_2s"]))}
    if model == "M2-CE":
        return {1: ("support:b", "constant_effort", ("ce", inp["Fce_2s"]))}
    if model == "M4":
        return {1: ("support:b", "spring", ("spring", inp["k_4s_b"])),
                2: ("support:c", "constant_effort", ("ce", inp["Fce_4s_c"])),
                3: ("support:d", "variable_spring", ("vspring", inp["kv_4s_d"], inp["Fc_4s_d"]))}
    raise ValueError(model)


MODELS = {
    "M1": {"spans": 1, "length": "L1", "item": "3, 4"},
    "M2-SPRING": {"spans": 2, "length": "a", "item": "1, 2, 4"},
    "M2-VSPRING": {"spans": 2, "length": "a", "item": "1, 2, 4"},
    "M2-CE": {"spans": 2, "length": "a", "item": "1, 2, 4"},
    "M4": {"spans": 4, "length": "a", "item": "5"},
}

# documents: one shear-model selection each; every case of a document is listed with it
DOCUMENTS = [
    ("DOC-1S", "M1", "euler_bernoulli", [
        ("HT-1S-ORDINARY", None),
        ("HT-1S-TEST", "springs_and_constant_effort"),
    ]),
    ("DOC-2S-SPRING", "M2-SPRING", "euler_bernoulli", [
        ("HT-2S-SPRING-ORDINARY", None),
        ("HT-2S-SPRING-HELD", "springs_and_constant_effort"),
        ("HT-2S-SPRING-NONE", "none"),
    ]),
    ("DOC-2S-SPRING-TIMOSHENKO", "M2-SPRING", "timoshenko_annulus_energy_v1", [
        ("HT-2S-SPRING-HELD-TIMOSHENKO", "springs_and_constant_effort"),
        ("HT-2S-SPRING-NONE-TIMOSHENKO", "none"),
    ]),
    ("DOC-2S-VSPRING", "M2-VSPRING", "euler_bernoulli", [
        ("HT-2S-VSPRING-HELD", "springs_and_constant_effort"),
        ("HT-2S-VSPRING-NONE", "none"),
    ]),
    ("DOC-2S-CE", "M2-CE", "euler_bernoulli", [
        ("HT-2S-CE-HELD", "springs_and_constant_effort"),
        ("HT-2S-CE-NONE", "none"),
    ]),
    ("DOC-4S", "M4", "euler_bernoulli", [
        ("HT-4S-ORDINARY", None),
        ("HT-4S-TEST-HELD", "springs_and_constant_effort"),
        ("HT-4S-TEST-NONE", "none"),
    ]),
]


def solve_all_methods(n, a, w, EI, GAs, laws, chk, tag, closed2=True):
    """Primary values by the closed form (two spans) or the flexibility method, checked by the others."""
    Rs, us = stiffness_solve(n, a, w, EI, GAs, laws)
    Rf, uf = flexibility_solve(n, a, w, EI, GAs, laws)
    if n == 2 and closed2:
        Rc, uc, _, _ = two_span_closed(a, w, EI, GAs, laws.get(1, ("free",)))
        for i in range(3):
            chk.eq(f"{tag}: reaction {NODE_NAMES[i]} closed form = stiffness solve", Rc[i], Rs[i])
        chk.eq(f"{tag}: u_z(b) closed form = stiffness solve", uc[1], us[1])
    for i in range(n + 1):
        chk.eq(f"{tag}: reaction {NODE_NAMES[i]} flexibility = stiffness solve", Rf[i], Rs[i])
        chk.eq(f"{tag}: u_z({NODE_NAMES[i]}) flexibility = stiffness solve", uf[i], us[i])
    L = n * a
    chk.eq(f"{tag}: vertical equilibrium, sum of reactions = w n a", sum(Rf, ZERO), w * L)
    chk.eq(f"{tag}: moment equilibrium about node a", sum((Rf[i] * (i * a) for i in range(n + 1)), ZERO), w * L * L / 2)
    all_rigid = all(l[0] == "rigid" for l in laws.values())
    if all_rigid and GAs is None:
        Rm = three_moment_rigid(n, a, w)
        for i in range(n + 1):
            chk.eq(f"{tag}: reaction {NODE_NAMES[i]} three-moment = flexibility", Rm[i], Rf[i])
        table = {1: [F(1, 2), F(1, 2)], 2: [F(3, 8), F(10, 8), F(3, 8)],
                 4: [F(11, 28), F(32, 28), F(26, 28), F(32, 28), F(11, 28)]}[n]
        for i in range(n + 1):
            chk.eq(f"{tag}: reaction {NODE_NAMES[i]} = tabulated coefficient x w a", Rf[i], table[i] * w * a)
    return Rf, uf


def reaction_rows(n, R, u, interior, label_elongation=True):
    el = {"pressure_elongation": "independent"} if label_elongation else {}
    rows = []
    a_id, end_id = "support:a", f"support:{NODE_NAMES[n]}"
    for comp in ("fx", "fy", "fz", "mx"):
        val = R[0] if comp == "fz" else ZERO
        rows.append(value(val, id=f"{a_id}.{comp}", quantity="support_reaction", unit="N" if comp != "mx" else "N*m", **el))
    for i in range(1, n):
        sid = interior[i][0]
        rows.append(value(R[i], id=f"{sid}.fz", quantity="support_reaction", unit="N", **el))
    for comp in ("fy", "fz"):
        rows.append(value(R[n] if comp == "fz" else ZERO, id=f"{end_id}.{comp}", quantity="support_reaction", unit="N", **el))
    for i in range(1, n):
        rows.append(value(u[i], id=f"node:{NODE_NAMES[i]}.uz", quantity="nodal_translation", unit="m", **el))
    return rows


def stress_rows(lame_by_basis):
    rows = []
    for basis, vals in lame_by_basis.items():
        for key in ("hoop_bore", "hoop_outside", "radial_bore", "radial_outside"):
            comp, surf = key.split("_")
            rows.append(value(vals[key], id=f"{basis}.{comp}.{surf}", quantity="pressure_stress",
                              basis=basis, component=comp, surface="inner" if surf == "bore" else "outer",
                              unit="Pa", applies_to="every station of every pipe of the case",
                              pressure_elongation="independent"))
    return rows


def compute(inp, record):
    chk = Checks(record)
    s = derive(inp, chk)
    R = s["R"]
    lame_sets = {}
    for pname in ("p_ordinary", "p_test"):
        p = inp[pname]
        lame_sets[pname] = {
            "nominal_section": lame_checks(p, R, s["r_n"], s["A_nominal"], chk, f"Lame {pname} nominal"),
            "reduced_section": lame_checks(p, R, s["r_p"], s["A_reduced"], chk, f"Lame {pname} reduced"),
        }
    cases = []
    for doc_id, model, shear, doc_cases in DOCUMENTS:
        n = MODELS[model]["spans"]
        a = inp[MODELS[model]["length"]]
        GAs = None if shear == "euler_bernoulli" else s["GA_s"]
        interior = model_laws(model, inp)
        for case_id, held in doc_cases:
            test = held is not None
            w = s["w_test"] if test else s["w_ordinary"]
            held_ids = []
            laws = {}
            for node, (sid, family, law) in interior.items():
                if held == "springs_and_constant_effort" and family in HELD_FAMILIES:
                    laws[node] = ("rigid",)
                    held_ids.append(sid)
                else:
                    laws[node] = law
            Rv, uv = solve_all_methods(n, a, w, s["EI"], GAs, laws, chk, case_id)
            controls = []
            if test and held == "springs_and_constant_effort" and n == 2:
                node_law = interior[1][2]
                # the unheld twin of this case, solved again here
                Rn_, un_ = solve_all_methods(n, a, w, s["EI"], GAs, {1: node_law}, chk, case_id + " unheld twin")
                Rc, uc = solve_all_methods(n, a, w, s["EI"], GAs, {1: ("at", un_[1])}, chk,
                                           case_id + " control: replacement at the unheld position")
                for i in range(3):
                    chk.eq(f"{case_id}: replacement at the unheld position reproduces the unheld reaction {NODE_NAMES[i]}",
                           Rc[i], Rn_[i])
                controls.append({"id": "held_at_unheld_position",
                                 "label": "negative control: replacement held at the unheld support's solved position "
                                          "instead of zero displacement from the authored geometry",
                                 "rows": reaction_rows(n, Rc, uc, interior)})
                if case_id == "HT-2S-SPRING-HELD":
                    Ro, uo = solve_all_methods(n, a, s["w_ordinary"], s["EI"], GAs, {1: node_law}, chk,
                                               case_id + " ordinary twin")
                    Rp, up = solve_all_methods(n, a, w, s["EI"], GAs, {1: ("at", uo[1])}, chk,
                                               case_id + " control: replacement at the ordinary case's position")
                    controls.append({"id": "held_at_ordinary_case_position",
                                     "label": "negative control: replacement held at the spring's solved position "
                                              "in the ordinary case of the same document (a previously solved state)",
                                     "rows": reaction_rows(n, Rp, up, interior)})
                if node_law[0] in ("vspring", "ce"):
                    force = node_law[2] if node_law[0] == "vspring" else node_law[1]
                    rep = Rv[1] - force
                    Rs_, _ = stiffness_solve(n, a, w, s["EI"], GAs, {1: ("rigid",)})
                    chk.eq(f"{case_id}: control replacement reaction = rigid reaction - support force", rep, Rs_[1] - force)
                    controls.append({"id": "support_force_still_acting",
                                     "label": "negative control: the support's own force still applied beside the "
                                              "rigid replacement; the replacement then reports less by that force",
                                     "rows": [value(rep, id="support:b.fz.replacement_only", quantity="support_reaction", unit="N"),
                                              value(force, id="support:b.applied_force", quantity="support_applied_force", unit="N")]})
            if case_id == "HT-1S-TEST":
                for wid, wv, lab in (("contents_double_counted", s["w_double_counted"],
                                      "negative control: section contents and test contents both counted"),
                                     ("contents_omitted", s["w_contents_omitted"],
                                      "negative control: no contents counted")):
                    Rx, ux = solve_all_methods(n, a, wv, s["EI"], GAs, laws, chk, case_id + " control " + wid)
                    controls.append({"id": wid, "label": lab,
                                     "rows": [value(wv, id="weight_per_length", quantity="load_per_length", unit="N/m")]
                                     + reaction_rows(n, Rx, ux, interior)})
            pkey = "p_test" if test else "p_ordinary"
            ce_applied = []
            for node, (sid, family, law) in interior.items():
                if family == "constant_effort":
                    ce_applied.append({"support": sid,
                                       "applied_force": value(ZERO if sid in held_ids else law[1], unit="N")})
            cases.append({
                "id": case_id,
                "document": doc_id,
                "model": model,
                "shear_model": shear,
                "route": "exact",
                "test_condition": None if not test else {"kind": "hydrostatic", "held_supports": held},
                "pressure": {"input": pkey, **value(inp[pkey], unit="Pa")},
                "contents_density": {"input": "rho_c_test" if test else "rho_c_section",
                                     **value(inp["rho_c_test" if test else "rho_c_section"], unit="kg/m^3")},
                "weight_per_length": value(w, unit="N/m", direction="global -Z"),
                "expected_outcome": {
                    "solved": True,
                    "marked_hydrostatic_test": test,
                    "held_supports_choice": held,
                    "held_support_ids": sorted(held_ids),
                    "constant_effort_applied_force": ce_applied,
                },
                "rows": reaction_rows(n, Rv, uv, interior) + stress_rows(lame_sets[pkey]),
                "negative_controls": controls,
            })
    return s, lame_sets, cases, chk


# ============================================================================ document
def signature(cases):
    sig = []
    for c in cases:
        for r in c["_q_rows"]:
            sig.append((c["id"], r[0], sign(r[1])))
    return sig


def compute_q(inp, chk):
    """A lean recomputation of every case, returning exact elements, for the binary64 input check."""
    s = derive(inp, chk)
    out = []
    for doc_id, model, shear, doc_cases in DOCUMENTS:
        n = MODELS[model]["spans"]
        a = inp[MODELS[model]["length"]]
        GAs = None if shear == "euler_bernoulli" else s["GA_s"]
        interior = model_laws(model, inp)
        for case_id, held in doc_cases:
            test = held is not None
            w = s["w_test"] if test else s["w_ordinary"]
            laws = {node: (("rigid",) if held == "springs_and_constant_effort" and fam in HELD_FAMILIES else law)
                    for node, (sid, fam, law) in interior.items()}
            Rf, uf = flexibility_solve(n, a, w, s["EI"], GAs, laws)
            Rs, us = stiffness_solve(n, a, w, s["EI"], GAs, laws)
            for i in range(n + 1):
                chk.eq("perturbed agreement", Rf[i], Rs[i])
                chk.eq("perturbed agreement", uf[i], us[i])
            rows = [(f"R{i}", Rf[i]) for i in range(n + 1)] + [(f"u{i}", uf[i]) for i in range(1, n)]
            p = inp["p_test" if test else "p_ordinary"]
            for basis, ri in (("nominal", s["r_n"]), ("reduced", s["r_p"])):
                for k, v in lame(p, s["R"], ri).items():
                    rows.append((f"{basis}.{k}", Q.of(v)))
            out.append({"id": case_id, "_q_rows": rows})
    out.append({"id": "weights", "_q_rows": [(k, s[k]) for k in ("w_ordinary", "w_test", "w_double_counted", "w_contents_omitted")]})
    return s, None, out, chk


def binary64_input_check():
    base_chk = Checks(False)
    _, _, base, _ = compute_q(NOMINAL, base_chk)
    base_sig = signature(base)
    base_vals = {(c["id"], r[0]): r[1] for c in base for r in c["_q_rows"]}
    variants = [[k] for k in INPUTS] + [list(INPUTS)]
    changed, max_rel, failures = [], F(0), base_chk.failed
    for keys in variants:
        inp = dict(NOMINAL)
        for k in keys:
            inp[k] = F(float(NOMINAL[k]))
        chk = Checks(False)
        _, _, cases, chk = compute_q(inp, chk)
        failures += chk.failed
        if signature(cases) != base_sig:
            changed.append("+".join(keys) if len(keys) == 1 else "all inputs")
        for c in cases:
            for rid, x in c["_q_rows"]:
                b = base_vals[(c["id"], rid)]
                if b.is_zero():
                    continue
                lo, hi = bounds((x - b) / b)
                max_rel = max(max_rel, abs(lo), abs(hi))
    return {
        "method": "each input replaced by Fraction(float(input)), one at a time and then all together; "
                  "every case re-solved exactly by the flexibility and stiffness methods; the sign (+, 0, -) "
                  "of every reaction, displacement, stress and weight compared with the nominal solve",
        "variants": len(variants),
        "inputs_exactly_binary64": sorted(k for k in INPUTS if F(float(NOMINAL[k])) == NOMINAL[k]),
        "discrete_outcomes_unchanged": not changed,
        "variants_with_changed_outcomes": changed,
        "method_disagreements": failures,
        "max_relative_value_change": format(float(max_rel), ".3e"),
    }


def document():
    s, lame_sets, cases, chk = compute(NOMINAL, True)
    b64 = binary64_input_check()
    chk.items.append({"check": "binary64 inputs: discrete outcomes unchanged and both methods agree",
                      "agrees": b64["discrete_outcomes_unchanged"] and b64["method_disagreements"] == 0})
    if not (b64["discrete_outcomes_unchanged"] and b64["method_disagreements"] == 0):
        chk.failed += 1
    derived_keys = [("R", "m"), ("t_r", "m"), ("r_n", "m"), ("r_p", "m"),
                    ("A_nominal", "m^2"), ("A_reduced", "m^2"), ("A_bore", "m^2"), ("A_insulation", "m^2"),
                    ("I_nominal", "m^4"), ("EI", "N*m^2"), ("G", "Pa"), ("kappa_E", "1"), ("GA_s", "N"),
                    ("mu_metal", "kg/m"), ("mu_insulation", "kg/m"), ("mu_contents_section", "kg/m"),
                    ("mu_contents_test", "kg/m"), ("w_ordinary", "N/m"), ("w_test", "N/m")]
    derived = {k: value(s[k], unit=u) for k, u in derived_keys}
    a = NOMINAL["a"]
    for shear, GAs in (("euler_bernoulli", None), ("timoshenko_annulus_energy_v1", s["GA_s"])):
        _, _, d0, f = two_span_closed(a, s["w_test"], s["EI"], GAs, ("rigid",))
        derived[f"two_span_delta0_test_{shear}"] = value(d0, unit="m")
        derived[f"two_span_f_{shear}"] = value(f, unit="m/N")
    negative_weights = {
        "w_double_counted": value(s["w_double_counted"], unit="N/m",
                                  label="negative control: g (metal + section contents + test contents + insulation)"),
        "w_contents_omitted": value(s["w_contents_omitted"], unit="N/m",
                                    label="negative control: g (metal + insulation)"),
    }
    pressure_stresses = {p: {b: {k: value(v, unit="Pa", pressure_elongation="independent") for k, v in vals.items()}
                             for b, vals in bases.items()} for p, bases in lame_sets.items()}
    alternative = {
        "open_reading_4_contents_on_nominal_bore": {
            "w_ordinary": value(s["alt_w_ordinary_nominal_bore"], unit="N/m"),
            "w_test": value(s["alt_w_test_nominal_bore"], unit="N/m"),
        },
    }
    refusals = [
        {"id": "REFUSE-TEST-ON-PRESSURE-FREE-ROUTE",
         "scenario": "DOC-2S-SPRING authored without the pressure contract (pressure-free route), its case "
                     "HT-2S-SPRING-HELD keeping its test condition",
         "expected": "the test case is refused by name; it is not solved as an ordinary case and publishes no result"},
        {"id": "REFUSE-HELD-DUPLICATE-ON-ONE-DOF",
         "scenario": "DOC-2S-SPRING with a second support at node:b, a constant-effort support on uz, beside the spring "
                     "on uz; case HT-2S-SPRING-HELD",
         "expected": "both are held, giving two rigid restraints on one degree of freedom whose split of the reaction "
                     "is not determined by mechanics; the case is refused by name with no partial result"},
        {"id": "REFUSE-HELD-ON-RESTRAINED-DOF",
         "scenario": "DOC-2S-SPRING with a rigid vertical support (uz) at node:b beside the spring on uz; case "
                     "HT-2S-SPRING-HELD",
         "expected": "the replacement duplicates an existing rigid restraint, so its reaction under the spring's "
                     "identity is not determined; the case is refused by name with no partial result"},
    ]
    if chk.failed:
        raise SystemExit(f"{chk.failed} cross-checks disagree")
    rows = sum(len(c["rows"]) for c in cases)
    ctrl_rows = sum(len(x["rows"]) for c in cases for x in c["negative_controls"])
    return {
        "document": "swbpipe.hydrostatic_test_references",
        "status": "candidate frozen references; derived and cross-checked by the author, not yet independently refuted",
        "generator": GENERATOR,
        "units": {"force": "N", "moment": "N*m", "translation": "m", "stress": "Pa", "load_per_length": "N/m"},
        "theory": "small-displacement linear elasticity; straight runs along global X; Euler-Bernoulli flexure, or "
                  "Timoshenko with A_s = kappa_E A_nominal where the document selects it; thick-wall (Lame) "
                  "annulus under internal pressure",
        "conventions": {
            "global_axes": "X along the run, Z up; gravity along -Z",
            "support_reaction": "global force (moment) of the support on the pipe at its node; + along the positive "
                                "global axis; a held support's row is its replacement restraint's reaction, under the "
                                "support's own identity (open reading 1)",
            "nodal_translation": "global translation from the authored geometry",
            "pressure_stress": "Lame hoop and radial stress on the named section basis, tension positive",
            "pressure_elongation": "'independent': the row is the same with the case's pressure elongation off or on "
                                   "(README, 'Pressure elongation')",
            "exact": "an element of Q(pi): a rational, q*pi, q/pi or a ratio of polynomials in pi",
        },
        "counts": {"cases": len(cases), "case_rows": rows, "negative_control_rows": ctrl_rows,
                   "refusal_scenarios": len(refusals), "cross_checks": len(chk.items)},
        "inputs": {k: {"unit": u, "meaning": m, **value(v)} for k, (v, u, m) in INPUTS.items()},
        "models": {k: {"spans": m["spans"], "span_length_input": m["length"], "items": m["item"],
                       "interior_supports": {f"node:{NODE_NAMES[node]}": {"support": sid, "family": fam, "dof": "uz"}
                                             for node, (sid, fam, _) in model_laws(k, NOMINAL).items()}}
                   for k, m in MODELS.items()},
        "derived": derived,
        "weights_negative_controls": negative_weights,
        "pressure_stresses": pressure_stresses,
        "alternative_readings": alternative,
        "binary64_input_check": b64,
        "refusals": refusals,
        "crosscheck": chk.items,
        "cases": cases,
    }


def sha256sums(files):
    return ("".join(f"{hashlib.sha256(d).hexdigest()}  {p}\n" for p, d in sorted(files.items()))).encode()


def build():
    doc = document()
    text = json.dumps(doc, indent=1, ensure_ascii=False) + "\n"
    files = {"references.json": text.encode()}
    for fixed in FIXED_FILES:
        path = OUT / fixed
        if path.exists():
            files[fixed] = path.read_bytes()
    files["SHA256SUMS"] = sha256sums({k: v for k, v in files.items() if k != "SHA256SUMS"})
    return files, doc


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--write", action="store_true", help="rewrite references.json and SHA256SUMS")
    args = parser.parse_args()
    files, doc = build()
    c = doc["counts"]
    print(f"{c['cases']} cases, {c['case_rows']} case rows, {c['negative_control_rows']} negative-control rows, "
          f"{c['refusal_scenarios']} refusal scenarios")
    print(f"cross-checks: {c['cross_checks']}, all agree")
    print(f"binary64 inputs: {doc['binary64_input_check']['variants']} variants, discrete outcomes unchanged: "
          f"{doc['binary64_input_check']['discrete_outcomes_unchanged']}, largest relative value change "
          f"{doc['binary64_input_check']['max_relative_value_change']}")
    if args.write:
        for path, data in files.items():
            if path in FIXED_FILES:
                continue
            (OUT / path).write_bytes(data)
        print("wrote references.json and SHA256SUMS")
        return 0
    bad = [p for p, d in files.items() if not (OUT / p).exists() or (OUT / p).read_bytes() != d]
    committed = {str(p.relative_to(OUT)) for p in OUT.rglob("*") if p.is_file() and "__pycache__" not in p.parts}
    extra = sorted(committed - set(files))
    if bad or extra:
        print("MISMATCH: " + ", ".join(bad + [f"unexpected {e}" for e in extra]))
        return 1
    print(f"verified {len(files)} files byte for byte (SHA256SUMS included)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
