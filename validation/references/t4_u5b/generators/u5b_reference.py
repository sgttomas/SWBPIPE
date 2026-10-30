#!/usr/bin/env python3
"""T4-U5b independent references: explicit elastic tie rods on a pressurized
expansion joint authored as an objective connector.

Standard-library Python 3 only. This generator never imports, reads or runs
product code or product output. Besides its own fixture it reads, as data
bound by SHA-256, t4_u5a's oracle.json and nine of its request documents.

Every value is exact over Q(pi): a rational function of pi with rational
coefficients, held in lowest terms (integer coefficients, no common content,
positive leading denominator coefficient), so equality is exact. Decimals are
rounded from the exact forms at 40 significant digits, binary64 values from
those.

Physics source: the Joint Reference contract (JR, archived at 11718e61a2^,
CORRECTNESS_DESIGN/JOINT_REFERENCE/CONTRACT.md sections 2, 4, 5, 6 and 7) and
its independent refutation (sections 2 to 4), and DEL-04-07 Design
joint-pressure-thrust.md section 9, with HELP_HUMAN's two rulings of
2026-10-17 (non-parallel rods admitted, S_joint by the projected rule; rods
admitted with p = 0 at the joint). Re-derived here; no equation is taken
from an extracted equation artifact (DEC-043).

Two solvers, sharing only the fixture and this file's exact arithmetic:
  * a nodal stiffness solve: 3D Euler-Bernoulli frame members, the
    connector's B^T K B from JR's midpoint kinematics, each rod's C^T k C
    with C evaluated from e = n.(v_bj - v_bi), the representation-B ledger
    as nodal loads, the Poisson and thermal strains as equivalent eigenload
    pairs and the rod reference load +C^T k e_ref; fraction-free (Bareiss)
    elimination over Z[pi];
  * a force method: the chain is a cantilever from A (with the six actions at
    the far anchor as redundants when it is anchored); section actions from
    statics, displacements by the unit-load method (axial, torsion and EB
    bending integrals, the Poisson and thermal strains as initial strains), the connector and
    its rods as one generalized spring K + sum k c c^T in the connector's
    midpoint basis with the rod reference as an initial generalized
    deformation, and the connector's end actions from JR's end-action
    formulas.
Closed forms (rows 2 and 4 as functions of the total rod stiffness, J4,
e_ref, the offset pairs, the asymmetric free chain, and the p = 0 controls:
the mechanical-only free chain, the thermal anchored relation extended with
the rods and J3-R's remote-region chain) are a third check.

Modes:
  python3 -I u5b_reference.py            check: rebuild every file in memory,
                                          run every cross-check, compare byte
                                          for byte, fail on any disagreement
                                          or unexpected file
  python3 -I u5b_reference.py --write    rewrite cases/, oracle.json and
                                          SHA256SUMS (intentional changes only)
"""
import argparse
import hashlib
import json
import sys
from decimal import Decimal, getcontext, localcontext
from fractions import Fraction as F
from math import gcd, isqrt
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
U5A = ROOT.parent / "t4_u5a"
getcontext().prec = 120

U5A_PINNED = {
    "oracle.json": "72cd5afa4b417f561c2c357aa3014de22d2dd3e64caf5574455aa6d7d9aba1a8",
    "requests/J3-1.request.json": "e64cbeae5b9f25a25b1d38566579e3d9ccabd7e69304e9637af20033efa49e47",
    "requests/J3-3.request.json": "7598c7b47dac284ac0d84de634682f04b537b71e14fefd11fe01376266816732",
    "requests/J3-5U.request.json": "ba56d91991ebaa09bc5e9310d6872b9d0f9241a94c47b8928c5a1047fb231530",
    "requests/J3-5UA.request.json": "8cabf999bf9ab5a7e7602af73c6e900b381ea2414499f205d2ec308923908960",
    "requests/J3-1R.request.json": "c72059741eff1d0d63cbae3add491099cfa0e62b97656d33a59d1d96b3623dd4",
    "requests/J3-1V.request.json": "bba8f7b4650c9258edd6670b2a09c711b15a866d102c10050a9a491e9bc6344f",
    "requests/J3-1T.request.json": "829fc390290b7b0ed392f50078c01a9f7f8a4e56f23c13ee067309c05f36307d",
    "requests/J3-R.request.json": "481bf60266f064655fdd4a439b7e269d5f114610c320d638d6a70b4f11a1dd80",
    "requests/J3-RP.request.json": "c3267c0e631126cfb2afc0efdb4df06fb8eca083435c1d8da9687c23cc20acca",
}

CHECKS = []  # (name, passed) records of every cross-check


def check(cond, name):
    CHECKS.append(name)
    if not cond:
        raise SystemExit("CROSS-CHECK FAILED: " + name)


# ---------------------------------------------------------------------------
# Z[t]: integer polynomials, lowest degree first.


def ztrim(p):
    p = list(p)
    while p and p[-1] == 0:
        p.pop()
    return p


def zadd(a, b):
    if len(a) < len(b):
        a, b = b, a
    r = list(a)
    for i, c in enumerate(b):
        r[i] += c
    return ztrim(r)


def zsub(a, b):
    r = list(a) + [0] * max(0, len(b) - len(a))
    for i, c in enumerate(b):
        r[i] -= c
    return ztrim(r)


def zmul(a, b):
    if not a or not b:
        return []
    r = [0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        if x:
            for j, y in enumerate(b):
                r[i + j] += x * y
    return ztrim(r)


def zdivexact(a, b):
    a = list(a)
    if not a:
        return []
    db = len(b) - 1
    lb = b[-1]
    if len(a) - 1 < db:
        raise ArithmeticError("inexact polynomial division")
    q = [0] * (len(a) - db)
    for i in range(len(a) - 1 - db, -1, -1):
        c = a[i + db]
        if c:
            if c % lb:
                raise ArithmeticError("inexact polynomial division")
            qc = c // lb
            q[i] = qc
            for j, y in enumerate(b):
                a[i + j] -= qc * y
    if ztrim(a):
        raise ArithmeticError("inexact polynomial division")
    return ztrim(q)


def zcontent(p):
    g = 0
    for c in p:
        g = gcd(g, c)
    return g


def zprim(p):
    p = ztrim(p)
    if not p:
        return []
    g = zcontent(p)
    if p[-1] < 0:
        g = -g
    return [c // g for c in p]


def zprem(a, b):
    a = list(a)
    db = len(b) - 1
    lb = b[-1]
    while a and len(a) - 1 >= db:
        da = len(a) - 1
        la = a[-1]
        a = [lb * c for c in a]
        for j, y in enumerate(b):
            a[da - db + j] -= la * y
        a = ztrim(a)
    return a


def zgcd(a, b):
    """Primitive gcd in Z[t] (positive leading coefficient)."""
    a, b = zprim(a), zprim(b)
    if not a:
        return b
    if not b:
        return a
    if len(a) < len(b):
        a, b = b, a
    while b:
        r = zprem(a, b)
        a, b = b, zprim(r)
    return zprim(a)


def zlcm(a, b):
    if len(a) == 1 and len(b) == 1:
        return [abs(a[0] * b[0]) // gcd(a[0], b[0])]
    return zdivexact(zmul(a, b), zgcd(a, b))


# ---------------------------------------------------------------------------
# Q(pi): quotients of integer polynomials in pi, canonical.


def _qnorm(n, d):
    n = ztrim(n)
    d = ztrim(d)
    if not d:
        raise ZeroDivisionError("Q(pi) division by zero")
    if not n:
        return (), (1,)
    if len(d) > 1:
        g = zgcd(n, d)
        if len(g) > 1:
            n = zdivexact(n, g)
            d = zdivexact(d, g)
    c = 0
    for x in n:
        c = gcd(c, x)
    for x in d:
        c = gcd(c, x)
    if d[-1] < 0:
        c = -c
    if c != 1:
        n = [x // c for x in n]
        d = [x // c for x in d]
    return tuple(n), tuple(d)


class Qp:
    __slots__ = ("n", "d")

    @staticmethod
    def make(n, d=(1,)):
        q = Qp.__new__(Qp)
        q.n, q.d = _qnorm(n, d)
        return q

    @staticmethod
    def of(x):
        if isinstance(x, Qp):
            return x
        x = F(x)
        return Qp.make([x.numerator], [x.denominator])

    def __add__(self, o):
        o = Qp.of(o)
        if self.d == o.d:
            return Qp.make(zadd(self.n, o.n), self.d)
        return Qp.make(zadd(zmul(self.n, o.d), zmul(o.n, self.d)), zmul(self.d, o.d))

    __radd__ = __add__

    def __neg__(self):
        q = Qp.__new__(Qp)
        q.n = tuple(-c for c in self.n)
        q.d = self.d
        return q

    def __sub__(self, o):
        return self + (-Qp.of(o))

    def __rsub__(self, o):
        return Qp.of(o) + (-self)

    def __mul__(self, o):
        o = Qp.of(o)
        if not self.n or not o.n:
            return ZERO
        return Qp.make(zmul(self.n, o.n), zmul(self.d, o.d))

    __rmul__ = __mul__

    def __truediv__(self, o):
        o = Qp.of(o)
        if not o.n:
            raise ZeroDivisionError("Q(pi) division by zero")
        return Qp.make(zmul(self.n, o.d), zmul(self.d, o.n))

    def __rtruediv__(self, o):
        return Qp.of(o) / self

    def __pow__(self, k):
        if not isinstance(k, int) or k < 0:
            raise ValueError("nonnegative integer powers only")
        out = ONE
        for _ in range(k):
            out = out * self
        return out

    def __eq__(self, o):
        o = Qp.of(o)
        return self.n == o.n and self.d == o.d

    def __hash__(self):
        return hash((self.n, self.d))

    def is_zero(self):
        return not self.n

    def dec(self):
        return peval(self.n, PI_DEC) / peval(self.d, PI_DEC)

    def text(self):
        return qtext(self)

    def __repr__(self):
        return "Qp(" + self.text() + ")"


def peval(p, x):
    s = Decimal(0)
    for c in reversed(p):
        s = s * x + c
    return s


def ptext(p):
    terms = []
    for k, c in enumerate(p):
        if c == 0:
            continue
        mono = "" if k == 0 else ("pi" if k == 1 else "pi^%d" % k)
        a = abs(c)
        if k == 0:
            s = str(a)
        elif a == 1:
            s = mono
        else:
            s = "%d*%s" % (a, mono)
        terms.append((c < 0, s))
    if not terms:
        return "0"
    out = ("-" if terms[0][0] else "") + terms[0][1]
    for neg, s in terms[1:]:
        out += (" - " if neg else " + ") + s
    return out


def qtext(q):
    n, d = q.n, q.d
    if not n:
        return "0"
    nt = ptext(n)
    nterms = sum(1 for c in n if c)
    if d == (1,):
        return nt
    dterms = sum(1 for c in d if c)
    if nterms > 1:
        nt = "(" + nt + ")"
    dt = ptext(d)
    if dterms > 1 or len(d) > 1:
        dt = "(" + dt + ")"
    return nt + "/" + dt


def _pi_decimal(digits):
    with localcontext() as ctx:
        ctx.prec = digits + 20

        def arctan_inv(x):
            x = Decimal(x)
            x2 = x * x
            power = Decimal(1) / x
            total = power
            k = 1
            eps = Decimal(10) ** (-(digits + 15))
            while True:
                power /= x2
                term = power / (2 * k + 1)
                if term < eps:
                    break
                total += -term if k % 2 else term
                k += 1
            return total

        value = 4 * (4 * arctan_inv(5) - arctan_inv(239))
    with localcontext() as ctx:
        ctx.prec = digits
        return +value


PI_DEC = _pi_decimal(115)
check(str(PI_DEC).startswith("3.14159265358979323846264338327950288419716939937510582097494459"), "pi by Machin")
ZERO = Qp.make([])
ONE = Qp.of(1)
PI = Qp.make([0, 1])


def parse_exact(text):
    """Parse t4_u5a's exact text (integers, pi, pi^k, + - * / and parentheses)."""
    toks = []
    i = 0
    while i < len(text):
        ch = text[i]
        if ch.isspace():
            i += 1
        elif ch.isdigit():
            j = i
            while j < len(text) and text[j].isdigit():
                j += 1
            toks.append(("n", int(text[i:j])))
            i = j
        elif text.startswith("pi", i):
            toks.append(("pi", None))
            i += 2
        elif ch in "+-*/^()":
            toks.append((ch, None))
            i += 1
        else:
            raise ValueError("unparsable exact text: " + text)
    pos = [0]

    def peek():
        return toks[pos[0]][0] if pos[0] < len(toks) else None

    def take():
        t = toks[pos[0]]
        pos[0] += 1
        return t

    def expr():
        v = term()
        while peek() in ("+", "-"):
            op = take()[0]
            w = term()
            v = v + w if op == "+" else v - w
        return v

    def term():
        v = factor()
        while peek() in ("*", "/"):
            op = take()[0]
            w = factor()
            v = v * w if op == "*" else v / w
        return v

    def factor():
        t = peek()
        if t == "-":
            take()
            return -factor()
        if t == "n":
            return Qp.of(take()[1])
        if t == "pi":
            take()
            if peek() == "^":
                take()
                k = take()[1]
                return Qp.make([0] * k + [1])
            return PI
        if t == "(":
            take()
            v = expr()
            if take()[0] != ")":
                raise ValueError("unbalanced: " + text)
            return v
        raise ValueError("unexpected token in: " + text)

    v = expr()
    if pos[0] != len(toks):
        raise ValueError("trailing tokens in: " + text)
    return v


# ---------------------------------------------------------------------------
# vectors and matrices over Q(pi)


def V(*xs):
    return [Qp.of(x) for x in xs]


def vadd(a, b):
    return [x + y for x, y in zip(a, b)]


def vsub(a, b):
    return [x - y for x, y in zip(a, b)]


def vsc(s, a):
    return [x * s for x in a]


def dot(a, b):
    s = ZERO
    for x, y in zip(a, b):
        if not x.is_zero() and not y.is_zero():
            s = s + x * y
    return s


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def mvec(m, v):
    return [dot(row, v) for row in m]


def mtvec(m, v):
    """M^T v (M row-major)."""
    return [dot([m[r][c] for r in range(len(m))], v) for c in range(len(m[0]))]


def mT(m):
    return [[m[r][c] for r in range(len(m))] for c in range(len(m[0]))]


def mmul(a, b):
    bt = mT(b)
    return [[dot(row, col) for col in bt] for row in a]


def zeros(n, m):
    return [[ZERO] * m for _ in range(n)]


def qmat(rows):
    return [[Qp.of(x) for x in row] for row in rows]


def unit(k, n=12):
    v = [ZERO] * n
    v[k] = ONE
    return v


def exact_sqrt(q):
    """Square root of a rational Q(pi) constant that is a perfect square."""
    if len(q.n) > 1 or len(q.d) > 1:
        raise ValueError("not rational")
    num = q.n[0] if q.n else 0
    den = q.d[0]
    rn, rd = isqrt(num), isqrt(den)
    if rn * rn != num or rd * rd != den:
        raise ValueError("not a perfect square")
    return Qp.of(F(rn, rd))


def norm(v):
    return exact_sqrt(dot(v, v))


def qsolve(A, rhs_cols):
    """Solve A x = b exactly for each column b; A, b over Q(pi).

    Each row is scaled by the lcm of its denominators to land in Z[pi]; the
    fraction-free (Bareiss) elimination is then exact. Pivots are leading
    principal minors, nonzero for the positive-definite systems used here."""
    n = len(A)
    m = len(rhs_cols)
    rows = []
    for i in range(n):
        entries = list(A[i]) + [rhs_cols[c][i] for c in range(m)]
        L = [1]
        for e in entries:
            if e.n and e.d != (1,):
                L = zlcm(L, list(e.d))
        rows.append([zmul(list(e.n), zdivexact(L, list(e.d))) if e.n else [] for e in entries])
    prev = [1]
    for k in range(n - 1):
        piv = rows[k][k]
        if not piv:
            raise ArithmeticError("zero leading minor")
        rk = rows[k]
        for i in range(k + 1, n):
            ri = rows[i]
            mik = ri[k]
            for j in range(k + 1, n + m):
                t = zmul(piv, ri[j])
                if mik and rk[j]:
                    t = zsub(t, zmul(mik, rk[j]))
                ri[j] = zdivexact(t, prev) if t else []
            ri[k] = []
        prev = piv
    det = rows[n - 1][n - 1]
    if not det:
        raise ArithmeticError("singular system")
    out = []
    for c in range(m):
        Y = [None] * n
        for i in range(n - 1, -1, -1):
            s = zmul(det, rows[i][n + c])
            for j in range(i + 1, n):
                if rows[i][j] and Y[j]:
                    s = zsub(s, zmul(rows[i][j], Y[j]))
            Y[i] = zdivexact(s, rows[i][i]) if s else []
        out.append([Qp.make(y, det) if y else ZERO for y in Y])
    return out


# ---------------------------------------------------------------------------
# Fixture (JR J3, exact decimal values as written; invented data)

PROVENANCE = "invented T4-U5b reference fixture (JR J3 values and invented tie rods); not library, catalog, manufacturer or code-rule data"
E_PA = F(200000000000)
NU = F(3, 10)
ALPHA = F(3, 250000)
OD = F(3, 50)
WALL = F(1, 200)
RO = OD / 2
RI = RO - WALL
P_PA = F(200000)
AE = F(1, 250)
K_DIAG = [F(200000), F(80000), F(80000), F(600), F(900), F(900)]
L_STUB = F(2)
L_JOINT = F(3, 10)
G_PA = E_PA / (2 * (1 + NU))
A_S = PI * (RO ** 2 - RI ** 2)
I_B = PI * ((RO ** 4 - RI ** 4) / 4)
J_T = I_B * 2
A_I = PI * (RI ** 2)
EA = A_S * E_PA
EI = I_B * E_PA
GJ = J_T * G_PA
P_I = A_I * P_PA
H_P = P_I * (2 * NU)
P_E = Qp.of(P_PA * AE)
KP = EA / L_STUB
K_AX = Qp.of(K_DIAG[0])
D0_ANCH = K_AX + KP / 2
A_OFF = F(1, 10)       # rod transverse offset a
S_OFF = F(1, 20)       # rod axial attachment offset beyond the connector ends
E_REF = F(-1, 40000)   # pretension: rod stress-free length shorter than the gap
K_ROD_PAIR = F(10000000)

check(P_I == Qp.make([0, 125]), "Pi = 125 pi")
check(H_P == Qp.make([0, 75]), "h = 2 nu Pi = 75 pi")
check(P_E == Qp.of(800), "Pe = 800")
check(KP == Qp.make([0, 27500000]), "kp = 27500000 pi")
check(A_S == Qp.make([0, 11], [40000]), "As = 11 pi/40000")

NODES = ["node:A", "node:B", "node:C", "node:D"]
NODES_R = NODES + ["node:E"]   # J3-R's chain: a remote region over pipe:X beyond D
BASE_POS = {"node:A": (0, 0, 0), "node:B": (2, 0, 0), "node:C": (F(23, 10), 0, 0), "node:D": (F(43, 10), 0, 0),
            "node:E": (F(63, 10), 0, 0)}
PIPES = [("pipe:L", "node:A", "node:B"), ("pipe:R", "node:C", "node:D")]
PIPES_R = PIPES + [("pipe:X", "node:D", "node:E")]
DT_STUBS = F(10)               # MO-T: J3-1T's uniform stub temperature change (degC)
F_MO = F(1010)                 # MO-F: axial force at D (N)
M_MO = F(100)                  # MO-F: moment about z at D (N*m)
MODEL_CONST = "constant_effective_area_reference_v1"
MODEL_UNPR = "unpressurized"
I3 = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
ROT = [[F(-3, 5), 0, F(4, 5)], [F(16, 25), F(-3, 5), F(12, 25)], [F(12, 25), F(4, 5), F(9, 25)]]
TRANS = (10, 20, 5)
FLIP = [[-1, 0, 0], [0, 1, 0], [0, 0, -1]]
T_REV = [1, -1, 1, 1, -1, 1]  # q' = T q under end reversal, T = blockdiag(-J, -J)

# ---------------------------------------------------------------------------
# Case configurations


def rod(rid, o_b, o_c, k, e_ref=F(0)):
    return {"id": rid, "o_B": tuple(F(x) for x in o_b), "o_C": tuple(F(x) for x in o_c), "k": F(k), "e_ref": F(e_ref)}


def centreline_rod(k):
    return [rod("rod:c0", (0, 0, 0), (0, 0, 0), k)]


def pair_y(k_each, axial=False, e_ref=F(0)):
    s = S_OFF if axial else 0
    return [rod("rod:yp", (-s, A_OFF, 0), (s, A_OFF, 0), k_each, e_ref),
            rod("rod:ym", (-s, -A_OFF, 0), (s, -A_OFF, 0), k_each, e_ref)]


def pair_z(k_each):
    return [rod("rod:zp", (-S_OFF, 0, A_OFF), (S_OFF, 0, A_OFF), k_each),
            rod("rod:zm", (-S_OFF, 0, -A_OFF), (S_OFF, 0, -A_OFF), k_each)]


def single_y(k, e_ref=F(0)):
    return [rod("rod:yp", (-S_OFF, A_OFF, 0), (S_OFF, A_OFF, 0), k, e_ref)]


LADDER = [("2E3", F(2000)), ("2E5", F(200000)), ("2E7", F(20000000)), ("2E9", F(2000000000)),
          ("2E11", F(200000000000))]
EXTRA_K = [F(0), F(2 * 10 ** 13), F(2 * 10 ** 15), F(2 * 10 ** 17), F(2 * 10 ** 19)]


def config(cid, chain, rods, description, family, base, loads=(), rotated=False, reversed_=False, total_k=None,
           realization=None, pressure="listed", model=MODEL_CONST, dT=F(0)):
    """pressure: 'listed' (J3's region over pipes L and R lists the connector), 'remote' (J3-R's region over
    pipe:X only; the connector is in no region) or 'none' (no pressure region). model: the connector's
    pressure model. dT: uniform temperature change of pipes L and R (degC)."""
    return {"id": cid, "chain": chain, "rods": rods, "loads": list(loads), "rotated": rotated,
            "reversed": reversed_, "description": description, "family": family, "base": base,
            "total_k": total_k, "realization": realization, "pressure": pressure, "model": model, "dT": F(dT)}


def build_configs():
    cfgs = []
    for row, chain, base in (("L2", "anchored", "J3-1"), ("L4", "free", "J3-3")):
        for real in ("CL", "PR"):
            for tag, k in LADDER:
                rods = centreline_rod(k) if real == "CL" else pair_y(k / 2)
                where = ("a single centreline rod" if real == "CL"
                         else "a symmetric pair at +/-0.1 m along connector y, each k_R/2")
                rowtxt = ("JR row 2 (anchored outer ends)" if row == "L2"
                          else "JR row 4 (free closed chain, axial datum at A)")
                cfgs.append(config("%s-%s-%s" % (row, real, tag), chain, rods,
                                   "%s with finite rods, total k_R = %s N/m, as %s" % (rowtxt, tag.replace("E", "e"), where),
                                   "row2_ladder" if row == "L2" else "row4_ladder", base, total_k=k, realization=real))
    mz = [{"id": "load:moment-D", "node": "node:D", "force": (0, 0, 0), "moment": (0, 0, 100)}]
    my = [{"id": "load:moment-D", "node": "node:D", "force": (0, 0, 0), "moment": (0, -150, 0)}]
    m4 = [{"id": "load:moment-D", "node": "node:D", "force": (0, 0, 0), "moment": (12, -150, 100)}]
    cfgs.append(config("OP-Z", "free", pair_y(K_ROD_PAIR, axial=True),
                       "free pressurized chain, rod pair at +/-0.1 m along connector y (1e7 N/m each), moment 100 N*m about z at D",
                       "offset_pair", "J3-3", loads=mz, total_k=2 * K_ROD_PAIR))
    cfgs.append(config("OP-Y", "free", pair_z(K_ROD_PAIR),
                       "free pressurized chain, rod pair at +/-0.1 m along connector z (1e7 N/m each), moment -150 N*m about y at D",
                       "offset_pair", "J3-3", loads=my, total_k=2 * K_ROD_PAIR))
    cfgs.append(config("OP-4", "free", pair_y(K_ROD_PAIR / 2, axial=True) + pair_z(K_ROD_PAIR / 2),
                       "free pressurized chain, four rods at +/-0.1 m along connector y and z (5e6 N/m each), moment (12, -150, 100) N*m at D",
                       "offset_pair", "J3-3", loads=m4, total_k=2 * K_ROD_PAIR))
    for chain, base in (("free", "J3-3"), ("anchored", "J3-1")):
        c = "F" if chain == "free" else "A"
        what = "free pressurized chain" if chain == "free" else "anchored pressurized chain"
        cfgs.append(config("AS-%s" % c, chain, single_y(K_ROD_PAIR),
                           "%s with one rod at +0.1 m along connector y (1e7 N/m): asymmetric control" % what,
                           "asymmetric", base))
        cfgs.append(config("AS-%s-V" % c, chain, single_y(K_ROD_PAIR),
                           "AS-%s with the connector's ends reversed (end_i at C, Q' = Q J, rod ends swapped)" % c,
                           "asymmetric", base, reversed_=True))
        cfgs.append(config("AS-%s-R" % c, chain, single_y(K_ROD_PAIR),
                           "AS-%s rigidly rotated (J3-1R's rotation, every axis skew) and translated" % c,
                           "asymmetric", base, rotated=True))
        cfgs.append(config("AS-%s-ER" % c, chain, single_y(K_ROD_PAIR, E_REF),
                           "AS-%s with the rod pretensioned, e_ref = -2.5e-5 m (reference load with an offset moment)" % c,
                           "asymmetric_reference_extension", base))
        cfgs.append(config("ER-%s" % c, chain, pair_y(K_ROD_PAIR, axial=True, e_ref=E_REF),
                           "%s, symmetric rod pair (1e7 N/m each) pretensioned, e_ref = -2.5e-5 m each" % what,
                           "reference_extension", base, total_k=2 * K_ROD_PAIR))
    cfgs.append(config("SK-F", "free",
                       [rod("rod:sk", (-S_OFF, A_OFF, 0), (S_OFF, A_OFF, F(3, 10)), K_ROD_PAIR)],
                       "free pressurized chain with one rod not parallel to the connector axis (direction "
                       "(0.8, 0, 0.6)); admitted by HELP_HUMAN's ruling 1 of 2026-10-17, S_joint by the projected rule",
                       "non_parallel", "J3-3"))
    # p = 0 at the joint (HELP_HUMAN's ruling 2 of 2026-10-17)
    fm = [{"id": "load:mech-D", "node": "node:D", "force": (F_MO, 0, 0), "moment": (0, 0, M_MO)}]
    cfgs.append(config("MO-F", "free", pair_y(K_ROD_PAIR, axial=True),
                       "mechanical-only tied control (JR J-C): free chain, no pressure region, unpressurized "
                       "connector, rod pair at +/-0.1 m along connector y (1e7 N/m each), force 1010 N along x and "
                       "moment 100 N*m about z at D",
                       "mechanical_only_free", "J3-3", loads=fm, total_k=2 * K_ROD_PAIR, pressure="none", model=MODEL_UNPR))
    cfgs.append(config("MO-T", "anchored", pair_y(K_ROD_PAIR, axial=True, e_ref=E_REF),
                       "mechanical-only tied thermal control: anchored chain, no pressure region, unpressurized "
                       "connector, stubs +10 degC (J3-1T's thermal loads), rod pair at +/-0.1 m along connector y "
                       "(1e7 N/m each) pretensioned, e_ref = -2.5e-5 m each",
                       "mechanical_only_thermal", "J3-1", total_k=2 * K_ROD_PAIR, pressure="none", model=MODEL_UNPR,
                       dT=DT_STUBS))
    cfgs.append(config("RR-U", "anchored", pair_y(K_ROD_PAIR, axial=True),
                       "J3-R with rods: unpressurized connector beside the remote region over pipe:X (p = 0 at the "
                       "joint), rod pair at +/-0.1 m along connector y (1e7 N/m each)",
                       "remote_region", "J3-R", total_k=2 * K_ROD_PAIR, pressure="remote", model=MODEL_UNPR))
    cfgs.append(config("RR-P", "anchored", pair_y(K_ROD_PAIR, axial=True),
                       "J3-RP with rods: constant-effective-area connector that no region lists (Pe = 0) beside the "
                       "remote region over pipe:X, rod pair at +/-0.1 m along connector y (1e7 N/m each)",
                       "remote_region", "J3-RP", total_k=2 * K_ROD_PAIR, pressure="remote", model=MODEL_CONST))
    return cfgs


def realize(c):
    R = qmat(ROT if c["rotated"] else I3)
    t = V(*(TRANS if c["rotated"] else (0, 0, 0)))
    remote = c["pressure"] == "remote"
    nodes = NODES_R if remote else NODES
    pipes = PIPES_R if remote else PIPES
    pos = {n: vadd(mvec(R, V(*BASE_POS[n])), t) for n in nodes}
    frame = mmul(R, qmat(FLIP)) if c["reversed"] else R
    ci, cj = ("node:C", "node:B") if c["reversed"] else ("node:B", "node:C")
    rods = []
    for r in c["rods"]:
        ob = mvec(R, V(*r["o_B"]))
        oc = mvec(R, V(*r["o_C"]))
        if c["reversed"]:
            ni, oi, nj, oj = "node:C", oc, "node:B", ob
        else:
            ni, oi, nj, oj = "node:B", ob, "node:C", oc
        rods.append({"id": r["id"], "ni": ni, "nj": nj, "oi": oi, "oj": oj, "k": Qp.of(r["k"]),
                     "e_ref": Qp.of(r["e_ref"])})
    loads = [{"id": ld["id"], "node": ld["node"], "force": mvec(R, V(*ld["force"])),
              "moment": mvec(R, V(*ld["moment"]))} for ld in c["loads"]]
    last = nodes[-1]
    supports = {"support:A": "node:A"}
    if c["chain"] == "anchored":
        supports["support:" + last[len("node:"):]] = last
    # pressure: which pipes a region pressurizes, its transferring terminals, and whether it lists the joint
    pressurized = {"listed": ("pipe:L", "pipe:R"), "remote": ("pipe:X",), "none": ()}[c["pressure"]]
    terminals = {"listed": ("node:A", "node:D"), "remote": ("node:D", "node:E"), "none": ()}[c["pressure"]]
    listed = c["pressure"] == "listed"
    check(c["model"] in (MODEL_CONST, MODEL_UNPR) and (c["model"] == MODEL_CONST or not listed),
          "%s: only a constant-effective-area connector is listed by a region" % c["id"])
    Pi = {pid: (P_I if pid in pressurized else ZERO) for pid, _, _ in pipes}
    H = {pid: (H_P if pid in pressurized else ZERO) for pid, _, _ in pipes}
    epsT = {pid: Qp.of(ALPHA * c["dT"] if pid in ("pipe:L", "pipe:R") else 0) for pid, _, _ in pipes}
    return {"cfg": c, "R": R, "pos": pos, "frame": frame, "ci": ci, "cj": cj, "rods": rods, "loads": loads,
            "supports": supports, "yref": mvec(R, V(0, 1, 0)), "axis": mvec(R, V(1, 0, 0)),
            "nodes": nodes, "pipes": pipes, "last": last, "terminals": terminals, "listed": listed,
            "Pi": Pi, "H": H, "epsT": epsT, "Pe": P_E if listed else ZERO}


# ---------------------------------------------------------------------------
# Stiffness method


def dof(node, k):
    return NODES_R.index(node) * 6 + k


def connector_q(xi, xj, ai, aj, frame, d):
    """JR section 2: symmetric-midpoint generalized deformation, evaluated directly."""
    ui, ti, uj, tj = d[0:3], d[3:6], d[6:9], d[9:12]
    vi = vadd(ui, cross(ti, ai))
    vj = vadd(uj, cross(tj, aj))
    r = vsub(vadd(xj, aj), vadd(xi, ai))
    tc = vsc(F(1, 2), vadd(ti, tj))
    qt = mtvec(frame, vsub(vsub(vj, vi), cross(tc, r)))
    qr = mtvec(frame, vsub(tj, ti))
    return qt + qr


def connector_B(xi, xj, ai, aj, frame):
    cols = [connector_q(xi, xj, ai, aj, frame, unit(k)) for k in range(12)]
    return [[cols[c][r] for c in range(12)] for r in range(6)]


def skew(v):
    return [[ZERO, -v[2], v[1]], [v[2], ZERO, -v[0]], [-v[1], v[0], ZERO]]


def connector_B_matrix_form(xi, xj, ai, aj, frame):
    """JR's printed B (Bt = Q^T[-I, S(ai)+S(r)/2, I, -S(aj)+S(r)/2], Br = Q^T[0,-I,0,I])."""
    r = vsub(vadd(xj, aj), vadd(xi, ai))
    I = qmat(I3)
    Sr2 = [[x * F(1, 2) for x in row] for row in skew(r)]
    Sai, Saj = skew(ai), skew(aj)
    blocks_t = [[[-x for x in row] for row in I],
                [[Sai[a][b] + Sr2[a][b] for b in range(3)] for a in range(3)],
                I,
                [[-Saj[a][b] + Sr2[a][b] for b in range(3)] for a in range(3)]]
    Z = zeros(3, 3)
    blocks_r = [Z, [[-x for x in row] for row in I], Z, I]
    QT = mT(frame)
    out = []
    for blocks in (blocks_t, blocks_r):
        prods = [mmul(QT, b) for b in blocks]
        for a in range(3):
            out.append([prods[blk][a][b] for blk in range(4) for b in range(3)])
    return out


def rod_extension(xi, xj, oi, oj, d, kin="correct"):
    """e = n.(v_bj - v_bi), v_b = u + theta x offset (JR section 5)."""
    if kin == "lumped":
        oi = oj = V(0, 0, 0)
    bi, bj = vadd(xi, oi), vadd(xj, oj)
    span = vsub(bj, bi)
    n = vsc(1 / norm(span), span)
    ui, ti, uj, tj = d[0:3], d[3:6], d[6:9], d[9:12]
    arm_i = cross(ti, oi)
    arm_j = cross(tj, oj)
    if kin == "offset_sign":
        arm_i, arm_j = vsc(-1, arm_i), vsc(-1, arm_j)
    elif kin == "arm_sign_end_i":
        arm_i = vsc(-1, arm_i)
    return dot(n, vsub(vadd(uj, arm_j), vadd(ui, arm_i))), n


def rod_C(xi, xj, oi, oj, kin="correct"):
    row = []
    n = None
    for k in range(12):
        e, n = rod_extension(xi, xj, oi, oj, unit(k), kin)
        row.append(e)
    return row, n


def eb_local(L):
    """3D Euler-Bernoulli frame member, local DOFs (u, v, w, rx, ry, rz) at i then j."""
    K = zeros(12, 12)
    a = EA / L
    t = GJ / L
    b12 = EI * 12 / L ** 3
    b6 = EI * 6 / L ** 2
    b4 = EI * 4 / L
    b2 = EI * 2 / L

    def put(i, j, v):
        K[i][j] = v
        K[j][i] = v

    put(0, 0, a); put(6, 6, a); put(0, 6, -a)
    put(3, 3, t); put(9, 9, t); put(3, 9, -t)
    # x-y plane: v and rz (theta_z = dv/dx)
    put(1, 1, b12); put(7, 7, b12); put(1, 7, -b12)
    put(1, 5, b6); put(1, 11, b6); put(5, 7, -b6); put(7, 11, -b6)
    put(5, 5, b4); put(11, 11, b4); put(5, 11, b2)
    # x-z plane: w and ry (theta_y = -dw/dx)
    put(2, 2, b12); put(8, 8, b12); put(2, 8, -b12)
    put(2, 4, -b6); put(2, 10, -b6); put(4, 8, b6); put(8, 10, b6)
    put(4, 4, b4); put(10, 10, b4); put(4, 10, b2)
    return K


def pipe_frame(xi, xj, yref):
    span = vsub(xj, xi)
    L = norm(span)
    e1 = vsc(1 / L, span)
    check(dot(yref, e1).is_zero() and dot(yref, yref) == ONE, "pipe y_reference is a unit vector normal to the member")
    e3 = cross(e1, yref)
    return L, [e1, list(yref), e3]


_PIPE_CACHE = {}


def pipe_global(xi, xj, yref):
    key = (xi, xj, yref)
    if key not in _PIPE_CACHE:
        L, lam = pipe_frame(list(xi), list(xj), list(yref))
        T = blockdiag4(lam)
        _PIPE_CACHE[key] = (L, lam, mmul(mT(T), mmul(eb_local(L), T)))
    return _PIPE_CACHE[key]


def blockdiag4(lam):
    T = zeros(12, 12)
    for b in range(4):
        for a in range(3):
            for c in range(3):
                T[3 * b + a][3 * b + c] = lam[a][c]
    return T


def ledger_B(g, double_count=False):
    """Representation B without the Poisson pairs: physical caps at the region's transferring
    terminals and, at a joint the region lists, the pair -/+ p(Ae - Ai) along the connector axis
    (double count: -/+ p Ae). The Poisson pairs are added to form the pressure RHS."""
    pos = g["pos"]
    out = {n: [V(0, 0, 0), V(0, 0, 0)] for n in g["nodes"]}
    for pid, ni, nj in g["pipes"]:
        if g["Pi"][pid].is_zero():
            continue
        d = vsc(1 / norm(vsub(pos[nj], pos[ni])), vsub(pos[nj], pos[ni]))
        if ni in g["terminals"]:
            out[ni][0] = vadd(out[ni][0], vsc(-g["Pi"][pid], d))
        if nj in g["terminals"]:
            out[nj][0] = vadd(out[nj][0], vsc(g["Pi"][pid], d))
    if g["listed"]:
        qx = [g["frame"][r][0] for r in range(3)]
        pair = P_E if double_count else P_E - P_I
        out[g["ci"]][0] = vadd(out[g["ci"]][0], vsc(-pair, qx))
        out[g["cj"]][0] = vadd(out[g["cj"]][0], vsc(pair, qx))
    return out


def ledger_A(g):
    """Representation A: every pressurized pipe's full cap pair, its Poisson pair (separately,
    second element of the return) and, at a listed joint, the joint's B^T g_p."""
    pos = g["pos"]
    out = {n: [V(0, 0, 0), V(0, 0, 0)] for n in g["nodes"]}
    poisson = {n: V(0, 0, 0) for n in g["nodes"]}
    for pid, ni, nj in g["pipes"]:
        if g["Pi"][pid].is_zero():
            continue
        d = vsc(1 / norm(vsub(pos[nj], pos[ni])), vsub(pos[nj], pos[ni]))
        out[ni][0] = vadd(out[ni][0], vsc(-g["Pi"][pid], d))
        out[nj][0] = vadd(out[nj][0], vsc(g["Pi"][pid], d))
        # Poisson pair: the pipe law Nw = EA eps + 2 nu Pi as an eigenload, +h d at node i, -h d at node j
        poisson[ni] = vadd(poisson[ni], vsc(g["H"][pid], d))
        poisson[nj] = vadd(poisson[nj], vsc(-g["H"][pid], d))
    if g["listed"]:
        zero = V(0, 0, 0)
        Bm = connector_B(pos[g["ci"]], pos[g["cj"]], zero, zero, g["frame"])
        gp = [P_E] + [ZERO] * 5
        f = mtvec(Bm, gp)
        for blk, node in ((0, g["ci"]), (1, g["cj"])):
            out[node][0] = vadd(out[node][0], f[6 * blk:6 * blk + 3])
            out[node][1] = vadd(out[node][1], f[6 * blk + 3:6 * blk + 6])
    return out, poisson


def sm_solve(g, mut=None):
    """Nodal stiffness solve. mut: dict with optional keys ledger ('double_count'),
    kin ('lumped', 'offset_sign', 'arm_sign_end_i') and ref ('ignored',
    'load_sign', 'load_omitted')."""
    mut = mut or {}
    pos = g["pos"]
    nodes = g["nodes"]
    ndof = 6 * len(nodes)
    K = zeros(ndof, ndof)
    f = [ZERO] * ndof

    def assemble(nodes, ke):
        idx = [dof(n, k) for n in nodes for k in range(6)]
        for a, ia in enumerate(idx):
            for b, ib in enumerate(idx):
                if not ke[a][b].is_zero():
                    K[ia][ib] = K[ia][ib] + ke[a][b]

    pipes = {}
    for pid, ni, nj in g["pipes"]:
        L, lam, kg = pipe_global(tuple(pos[ni]), tuple(pos[nj]), tuple(g["yref"]))
        assemble((ni, nj), kg)
        dvec = lam[0]
        h = g["H"][pid]
        th = EA * g["epsT"][pid]
        # Poisson eigenload pair (pressure RHS) and the thermal eigenload pair (not pressure)
        feq_p = vsc(h, dvec) + V(0, 0, 0) + vsc(-h, dvec) + V(0, 0, 0)
        feq = vadd(feq_p, vsc(-th, dvec) + V(0, 0, 0) + vsc(th, dvec) + V(0, 0, 0))
        for a, ia in enumerate([dof(ni, k) for k in range(6)] + [dof(nj, k) for k in range(6)]):
            f[ia] = f[ia] + feq[a]
        pipes[pid] = (ni, nj, L, lam, kg, feq, feq_p)
    zero = V(0, 0, 0)
    ci, cj = g["ci"], g["cj"]
    Bm = connector_B(pos[ci], pos[cj], zero, zero, g["frame"])
    Kd = [[Qp.of(K_DIAG[r]) if r == c else ZERO for c in range(6)] for r in range(6)]
    kc = mmul(mT(Bm), mmul(Kd, Bm))
    assemble((ci, cj), kc)
    rods = []
    kin = mut.get("kin", "correct")
    refm = mut.get("ref", "correct")
    for r in g["rods"]:
        C, n = rod_C(pos[r["ni"]], pos[r["nj"]], r["oi"], r["oj"], kin)
        kr = [[r["k"] * C[a] * C[b] for b in range(12)] for a in range(12)]
        assemble((r["ni"], r["nj"]), kr)
        e_ref = ZERO if refm == "ignored" else r["e_ref"]
        load_scale = {"correct": ONE, "ignored": ONE, "load_sign": -ONE, "load_omitted": ZERO}[refm]
        idx = [dof(r["ni"], k) for k in range(6)] + [dof(r["nj"], k) for k in range(6)]
        for a, ia in enumerate(idx):
            f[ia] = f[ia] + C[a] * r["k"] * e_ref * load_scale
        rods.append((r, C, n, e_ref))
    led = ledger_B(g, mut.get("ledger") == "double_count")
    for node in nodes:
        for k in range(3):
            f[dof(node, k)] = f[dof(node, k)] + led[node][0][k]
            f[dof(node, 3 + k)] = f[dof(node, 3 + k)] + led[node][1][k]
    for ld in g["loads"]:
        for k in range(3):
            f[dof(ld["node"], k)] = f[dof(ld["node"], k)] + ld["force"][k]
            f[dof(ld["node"], 3 + k)] = f[dof(ld["node"], 3 + k)] + ld["moment"][k]
    fixed = set()
    for sid, node in g["supports"].items():
        fixed.update(dof(node, k) for k in range(6))
    free = [i for i in range(ndof) if i not in fixed]
    Kff = [[K[i][j] for j in free] for i in free]
    ff = [f[i] for i in free]
    x = qsolve(Kff, [ff])[0]
    u = [ZERO] * ndof
    for i, xi in zip(free, x):
        u[i] = xi
    res = {"u": {n: (u[dof(n, 0):dof(n, 3)], u[dof(n, 3):dof(n, 6)]) for n in nodes}}
    Ku = [dot(K[i], u) for i in range(ndof)]
    res["reactions"] = {sid: ([Ku[dof(node, k)] - f[dof(node, k)] for k in range(3)],
                              [Ku[dof(node, 3 + k)] - f[dof(node, 3 + k)] for k in range(3)])
                        for sid, node in g["supports"].items()}
    dc = u[dof(ci, 0):dof(ci, 6)] + u[dof(cj, 0):dof(cj, 6)]
    q = mvec(Bm, dc)
    gq = [q[k] * K_DIAG[k] for k in range(6)]
    fend = mtvec(Bm, gq)
    res["q"] = q
    res["g"] = gq
    res["endpoint"] = {"end_i": (ci, fend[0:3], fend[3:6]), "end_j": (cj, fend[6:9], fend[9:12])}
    qx = [g["frame"][rr][0] for rr in range(3)]
    res["rods"] = {}
    sumT = ZERO
    sumT_axial = ZERO
    for r, C, n, e_ref in rods:
        d = u[dof(r["ni"], 0):dof(r["ni"], 6)] + u[dof(r["nj"], 0):dof(r["nj"], 6)]
        e = dot(C, d)
        T = r["k"] * (e - e_ref)
        act = vsc(T, C)
        nax = dot(n, qx)
        res["rods"][r["id"]] = {"e": e, "T": T, "n": n, "n_axial": nax,
                                "end_i": (r["ni"], act[0:3], act[3:6]), "end_j": (r["nj"], act[6:9], act[9:12])}
        sumT = sumT + T
        sumT_axial = sumT_axial + T * nax
    res["sumT"] = sumT
    res["sumT_axial"] = sumT_axial
    res["Tb"] = gq[0]
    # HELP_HUMAN ruling 1 (2026-10-17): the projected rule in every case (= Tb + sum T_r - Pe for parallel rods)
    res["S_joint"] = gq[0] + sumT_axial - g["Pe"]
    res["pipes"] = {}
    poisson = {n: V(0, 0, 0) for n in nodes}
    for pid, (ni, nj, L, lam, kg, feq, feq_p) in pipes.items():
        d = u[dof(ni, 0):dof(ni, 6)] + u[dof(nj, 0):dof(nj, 6)]
        fe = [dot(kg[a], d) - feq[a] for a in range(12)]
        loc = []
        for blk in range(4):
            loc += mvec(lam, fe[3 * blk:3 * blk + 3])
        Nw = loc[6]
        check(Nw == -loc[0], "pipe axial end forces balance (%s)" % pid)
        elong = dot(lam[0], vsub(d[6:9], d[0:3]))
        res["pipes"][pid] = {"Nw": Nw, "S": Nw - g["Pi"][pid], "elong": elong,
                             "end_i": (ni, fe[0:3], fe[3:6], loc[0:3], loc[3:6]),
                             "end_j": (nj, fe[6:9], fe[9:12], loc[6:9], loc[9:12])}
        check(all(x.is_zero() for x in feq_p[3:6] + feq_p[9:12]), "Poisson pair carries no moment (%s)" % pid)
        poisson[ni] = vadd(poisson[ni], feq_p[0:3])
        poisson[nj] = vadd(poisson[nj], feq_p[6:9])
    res["ledger"] = led
    # the pressure RHS in t4_u5a's convention (its pressure_rhs.global_n, the product's
    # assembled_pressure_rhs_global): caps, joint pair and Poisson pairs; rod reference and thermal loads excluded
    res["pressure_rhs"] = {n: vadd(led[n][0], poisson[n]) for n in nodes}
    return res


# ---------------------------------------------------------------------------
# Force method (independent of the stiffness assembly)


def fm_solve(g):
    pos = g["pos"]
    frame = g["frame"]
    nodes = g["nodes"]
    last = g["last"]
    idx = {n: i for i, n in enumerate(nodes)}
    xA, xZ = pos["node:A"], pos[last]
    tch = vsc(1 / norm(vsub(xZ, xA)), vsub(xZ, xA))
    # physical loads from the free body along the chain direction: the region's closed ends at its two
    # terminals (outward caps Pi) and, at a listed joint, the face pair -/+(Pe - Pi)
    base_loads = {n: [V(0, 0, 0), V(0, 0, 0)] for n in nodes}
    if g["terminals"]:
        t0, t1 = g["terminals"]
        base_loads[t0][0] = vsc(-P_I, tch)
        base_loads[t1][0] = vsc(P_I, tch)
    if g["listed"]:
        base_loads["node:B"][0] = vsc(-(P_E - P_I), tch)
        base_loads["node:C"][0] = vsc(P_E - P_I, tch)
    press = {n: [list(v[0]), list(v[1])] for n, v in base_loads.items()}
    for ld in g["loads"]:
        base_loads[ld["node"]][0] = vadd(base_loads[ld["node"]][0], ld["force"])
        base_loads[ld["node"]][1] = vadd(base_loads[ld["node"]][1], ld["moment"])
    members = [("node:A", "node:B"), ("node:C", "node:D")] + ([("node:D", "node:E")] if "node:E" in nodes else [])
    # initial strain of each member: thermal minus the Poisson term (Nw = EA(eps - eps_T) + 2 nu Pi)
    eps0 = {}
    rhs = {n: list(v[0]) for n, v in press.items()}
    for (pid, ni, nj), (n0, n1) in zip(g["pipes"], members):
        check(ni == n0 and nj == n1, "force method: pipes authored from the A side")
        eps0[(n0, n1)] = g["epsT"][pid] - g["H"][pid] / EA
        t = vsc(1 / norm(vsub(pos[n1], pos[n0])), vsub(pos[n1], pos[n0]))
        rhs[n0] = vadd(rhs[n0], vsc(g["H"][pid], t))
        rhs[n1] = vadd(rhs[n1], vsc(-g["H"][pid], t))
    # connector plus rods as one generalized spring in the connector basis
    toD = "j" if g["cj"] == "node:C" else "i"
    xi_c, xj_c = pos[g["ci"]], pos[g["cj"]]
    rvec = vsub(xj_c, xi_c)
    cs = []
    for r in g["rods"]:
        check(r["ni"] == g["ci"] and r["nj"] == g["cj"], "rod ends are the connector's ends, in order")
        span = vsub(vadd(pos[r["nj"]], r["oj"]), vadd(pos[r["ni"]], r["oi"]))
        n = vsc(1 / norm(span), span)
        m = vsc(F(1, 2), vadd(r["oi"], r["oj"]))
        c = mtvec(frame, n) + mtvec(frame, cross(m, n))
        cs.append((r, c, n))
    Kp = [[Qp.of(K_DIAG[a]) if a == b else ZERO for b in range(6)] for a in range(6)]
    for r, c, n in cs:
        for a in range(6):
            for b in range(6):
                Kp[a][b] = Kp[a][b] + r["k"] * c[a] * c[b]
    Kinv_cols = qsolve(Kp, [unit(k, 6) for k in range(6)])
    Kinv = [[Kinv_cols[c][r] for c in range(6)] for r in range(6)]
    ref_gen = [ZERO] * 6
    for r, c, n in cs:
        ref_gen = vadd(ref_gen, vsc(r["k"] * r["e_ref"], c))

    def resultant(loads, from_index, about):
        RF, RM = V(0, 0, 0), V(0, 0, 0)
        for node, (Fv, Mv) in loads.items():
            if idx[node] >= from_index:
                RF = vadd(RF, Fv)
                RM = vadd(RM, vadd(Mv, cross(vsub(pos[node], about), Fv)))
        return RF, RM

    def gen_action(loads):
        about = pos["node:C"]
        RF, RM = resultant(loads, 2, about)
        if toD == "j":
            gt = mtvec(frame, RF)
            gr = mtvec(frame, vadd(RM, cross(vsc(F(1, 2), rvec), RF)))
        else:
            Fv = vsc(-1, RF)
            Mv = vadd(vsc(-1, RM), cross(vsc(F(1, 2), rvec), RF))
            gt = mtvec(frame, Fv)
            gr = mtvec(frame, Mv)
        return gt + gr

    def vw(real, virt, initial):
        total = ZERO
        for n0, n1 in members:
            x0, x1 = pos[n0], pos[n1]
            L = norm(vsub(x1, x0))
            t = vsc(1 / L, vsub(x1, x0))
            acc = ZERO
            for s, w in ((ZERO, 1), (L / 2, 4), (L, 1)):
                xs = vadd(x0, vsc(s, t))
                vF, vM = resultant(virt, idx[n1], xs)
                if all(x.is_zero() for x in vF + vM):
                    continue
                RF, RM = resultant(real, idx[n1], xs)
                Nr, nv = dot(RF, t), dot(vF, t)
                Tr, tv = dot(RM, t), dot(vM, t)
                fval = (nv * (Nr / EA + (eps0[(n0, n1)] if initial else ZERO)) + tv * Tr / GJ
                        + (dot(vM, RM) - tv * Tr) / EI)
                acc = acc + fval * w
            total = total + acc * L / 6
        gv = gen_action(virt)
        if any(not x.is_zero() for x in gv):
            gr = gen_action(real)
            if initial:
                gr = vadd(gr, ref_gen)
            qv = mvec(Kinv, gr)
            total = total + dot(gv, qv)
        return total

    def unit_load(node, k):
        ld = {n: [V(0, 0, 0), V(0, 0, 0)] for n in nodes}
        ld[node][0 if k < 3 else 1] = unit(k % 3, 3)
        return ld

    loads = {n: [list(v[0]), list(v[1])] for n, v in base_loads.items()}
    X = None
    if g["cfg"]["chain"] == "anchored":
        Fm = [[vw(unit_load(last, l), unit_load(last, k), False) for l in range(6)] for k in range(6)]
        d0 = [vw(loads, unit_load(last, k), True) for k in range(6)]
        X = qsolve(Fm, [[-x for x in d0]])[0]
        loads[last][0] = vadd(loads[last][0], X[0:3])
        loads[last][1] = vadd(loads[last][1], X[3:6])
    res = {"u": {"node:A": (V(0, 0, 0), V(0, 0, 0))}}
    for node in nodes[1:]:
        vals = [vw(loads, unit_load(node, k), True) for k in range(6)]
        res["u"][node] = (vals[0:3], vals[3:6])
    if X is not None:
        check(all(x.is_zero() for x in res["u"][last][0] + res["u"][last][1]),
              "force method: the anchored far end has zero displacement after the redundants")
    totF, totM = V(0, 0, 0), V(0, 0, 0)
    for node, (Fv, Mv) in loads.items():
        totF = vadd(totF, Fv)
        totM = vadd(totM, vadd(Mv, cross(vsub(pos[node], xA), Fv)))
    res["reactions"] = {"support:A": (vsc(-1, totF), vsc(-1, totM))}
    if X is not None:
        sid = [s for s, n in g["supports"].items() if n == last][0]
        res["reactions"][sid] = (X[0:3], X[3:6])
    gprime = gen_action(loads)
    q = mvec(Kinv, vadd(gprime, ref_gen))
    gq = [q[k] * K_DIAG[k] for k in range(6)]
    res["q"], res["g"] = q, gq
    Fv = mvec(frame, gq[0:3])
    Mv = mvec(frame, gq[3:6])
    half_r = vsc(F(1, 2), rvec)
    res["endpoint"] = {"end_i": (g["ci"], vsc(-1, Fv), vsub(vsc(-1, cross(half_r, Fv)), Mv)),
                       "end_j": (g["cj"], Fv, vadd(vsc(-1, cross(half_r, Fv)), Mv))}
    qx = [frame[r][0] for r in range(3)]
    res["rods"] = {}
    sumT_axial = ZERO
    sumT = ZERO
    for r, c, n in cs:
        e = dot(c, q)
        T = r["k"] * (e - r["e_ref"])
        Fi = vsc(-T, n)
        Fj = vsc(T, n)
        res["rods"][r["id"]] = {"e": e, "T": T, "end_i": (r["ni"], Fi, cross(r["oi"], Fi)),
                                "end_j": (r["nj"], Fj, cross(r["oj"], Fj))}
        sumT = sumT + T
        sumT_axial = sumT_axial + T * dot(n, qx)
    res["sumT"] = sumT
    res["S_joint"] = gq[0] + sumT_axial - g["Pe"]
    res["pipes"] = {}
    for (pid, ni, nj), (n0, n1) in zip(g["pipes"], members):
        L = norm(vsub(pos[n1], pos[n0]))
        t = vsc(1 / L, vsub(pos[n1], pos[n0]))
        RFj, RMj = resultant(loads, idx[n1], pos[n1])
        RFi, RMi = resultant(loads, idx[n1], pos[n0])
        res["pipes"][pid] = {"Nw": dot(RFj, t), "end_i": (n0, vsc(-1, RFi), vsc(-1, RMi)),
                             "end_j": (n1, RFj, RMj)}
        # shear force in the member (EB vs Timoshenko dependence)
        res["pipes"][pid]["shear_zero"] = all(x.is_zero() for x in vsub(RFj, vsc(dot(RFj, t), t)))
    res["ledger"] = press
    res["pressure_rhs"] = rhs
    return res


def compare_sm_fm(cid, sm, fm):
    for node in sm["u"]:
        for a in range(2):
            check(sm["u"][node][a] == fm["u"][node][a], "%s: node %s %s, stiffness = force method" %
                  (cid, node, "translation" if a == 0 else "rotation"))
    for sid in sm["reactions"]:
        check(sm["reactions"][sid][0] == fm["reactions"][sid][0] and sm["reactions"][sid][1] == fm["reactions"][sid][1],
              "%s: reaction %s, stiffness = force method" % (cid, sid))
    check(sm["q"] == fm["q"] and sm["g"] == fm["g"], "%s: q and g, stiffness = force method" % cid)
    for end in ("end_i", "end_j"):
        check(sm["endpoint"][end] == fm["endpoint"][end],
              "%s: connector endpoint %s, B^T g = JR end-action formulas" % (cid, end))
    for rid in sm["rods"]:
        a, b = sm["rods"][rid], fm["rods"][rid]
        check(a["e"] == b["e"] and a["T"] == b["T"], "%s: rod %s e and T, stiffness = force method" % (cid, rid))
        check(a["end_i"] == b["end_i"] and a["end_j"] == b["end_j"],
              "%s: rod %s nodal action C^T T = statics" % (cid, rid))
    check(sm["S_joint"] == fm["S_joint"], "%s: S_joint, stiffness = force method" % cid)
    for pid in sm["pipes"]:
        a, b = sm["pipes"][pid], fm["pipes"][pid]
        check(a["Nw"] == b["Nw"], "%s: %s Nw, stiffness = force method" % (cid, pid))
        check(a["end_i"][:3] == b["end_i"] and a["end_j"][:3] == b["end_j"],
              "%s: %s end actions, stiffness = force method" % (cid, pid))
    check(set(sm["u"]) == set(fm["u"]) and set(sm["ledger"]) == set(fm["ledger"]) == set(sm["u"]),
          "%s: both methods cover the same nodes" % cid)
    for node in sm["ledger"]:
        check(sm["ledger"][node] == fm["ledger"][node], "%s: ledger at %s, stiffness = force method" % (cid, node))
        check(sm["pressure_rhs"][node] == fm["pressure_rhs"][node],
              "%s: pressure RHS (caps, joint pair and Poisson pairs) at %s, stiffness = force method" % (cid, node))


# ---------------------------------------------------------------------------
# Closed forms in a variable k (total rod stiffness k_R, or one rod's k_r):
# value(k) = sum num[i] k^i / sum den[i] k^i, coefficients in Q(pi).


def kr_eval(cf, k):
    num, den = cf
    kq = Qp.of(k)
    nv, dv, p = ZERO, ZERO, ONE
    for i in range(max(len(num), len(den))):
        if i < len(num):
            nv = nv + num[i] * p
        if i < len(den):
            dv = dv + den[i] * p
        p = p * kq
    return nv / dv


def kr_lim_inf(cf):
    num, den = cf
    dn = max([i for i, c in enumerate(num) if not c.is_zero()], default=-1)
    dd = max(i for i, c in enumerate(den) if not c.is_zero())
    if dn < dd:
        return ZERO
    check(dn == dd, "closed form has a finite k -> infinity limit")
    return num[dn] / den[dd]


def kr_lim_zero(cf):
    return kr_eval(cf, 0)


def kr_text(cf, var):
    def poly(cs):
        parts = []
        for i, c in enumerate(cs):
            if c.is_zero():
                continue
            ct = c.text()
            if i == 0:
                parts.append("(" + ct + ")")
            elif i == 1:
                parts.append(var if c == ONE else "(" + ct + ")*" + var)
            else:
                parts.append("(" + ct + ")*" + var + "^" + str(i))
        return " + ".join(parts) if parts else "0"
    return "[" + poly(cf[0]) + "] / [" + poly(cf[1]) + "]"


def cf_add(a, b):
    """Sum of two closed forms with the same denominator."""
    check(a[1] == b[1], "closed forms share a denominator")
    n = max(len(a[0]), len(b[0]))
    pad = lambda p: p + [ZERO] * (n - len(p))
    return ([x + y for x, y in zip(pad(a[0]), pad(b[0]))], a[1])


def cf_scale(s, a):
    return ([x * s for x in a[0]], a[1])


def cf_const(c, den):
    return ([Qp.of(c) * d for d in den], den)


def axial_forms(chain, eps=F(0), double_count=False, eps_T=F(0), pressurized=True):
    """Rows 2/4 as functions of the total rod stiffness k_R (equal e_ref = eps on every rod).
    Derived from the axial free bodies: node B equilibrium, the pipe law
    Nw = EA(eps - eps_T) + 2 nu Pi and symmetry (anchored) or the free end (free).
    Anchored, this is t4_u5a's thermal relation extended with the rods:
    (k + k_R + kp/2) q = Pe - (1 - 2 nu) Pi - EA eps_T + k_R e_ref (q_ref = 0).
    pressurized=False: no pressure anywhere (p = 0: Pi = h = Pe = 0)."""
    k = K_AX
    eps = Qp.of(eps)
    p_i, h_p, p_e = (P_I, H_P, P_E) if pressurized else (ZERO, ZERO, ZERO)
    load = p_e if double_count else p_e - p_i
    forms = {}
    if chain == "anchored":
        N0 = h_p + load - EA * Qp.of(eps_T)
        den = [D0_ANCH, ONE]
        q = ([N0, eps], den)
        forms["q_m"] = q
        forms["Tb_n"] = cf_scale(k, q)
        forms["sum_rod_tension_n"] = ([ZERO, N0 - eps * D0_ANCH], den)
        S_joint = cf_add(cf_add(forms["Tb_n"], forms["sum_rod_tension_n"]), cf_const(-p_e, den))
        forms["S_joint_n"] = S_joint
        if double_count:
            Nw = S_joint                      # node B: Nw - (Tb + sum T) = -Pe
        else:
            Nw = cf_add(S_joint, cf_const(p_i, den))
        forms["Nw_n"] = Nw
        forms["S_pipe_n"] = cf_add(Nw, cf_const(-p_i, den))
        forms["reaction_A_axial_n"] = cf_add(cf_const(p_i, den), cf_scale(-1, Nw))
        forms["reaction_D_axial_n"] = cf_scale(-1, forms["reaction_A_axial_n"])
        forms["interface_identity_S_pipe_minus_S_joint_n"] = cf_add(forms["S_pipe_n"], cf_scale(-1, S_joint))
        forms["u_B_axial_m"] = cf_scale(F(-1, 2), q)
        forms["u_C_axial_m"] = cf_scale(F(1, 2), q)
        forms["u_D_axial_m"] = ([ZERO], den)
    else:
        check(eps_T == 0 and pressurized, "free-chain axial forms: pressurized, no thermal strain")
        load_free = P_E + (P_I if double_count else ZERO)
        den = [K_AX, ONE]
        q = ([load_free, eps], den)
        forms["q_m"] = q
        forms["Tb_n"] = cf_scale(k, q)
        forms["sum_rod_tension_n"] = ([ZERO, load_free - eps * K_AX], den)
        forms["S_joint_n"] = cf_add(cf_add(forms["Tb_n"], forms["sum_rod_tension_n"]), cf_const(-P_E, den))
        forms["Nw_n"] = cf_const(P_I, den)
        forms["S_pipe_n"] = cf_const(ZERO, den)
        forms["reaction_A_axial_n"] = cf_const(ZERO, den)
        delta = (P_I - H_P) / KP
        forms["u_B_axial_m"] = cf_const(delta, den)
        forms["u_C_axial_m"] = cf_add(cf_const(delta, den), q)
        forms["u_D_axial_m"] = cf_add(cf_const(delta * 2, den), q)
        forms["outer_end_separation_change_m"] = forms["u_D_axial_m"]
        forms["interface_identity_S_pipe_minus_S_joint_n"] = cf_add(forms["S_pipe_n"], cf_scale(-1, forms["S_joint_n"]))
    return forms


def mo_free_forms(Fx):
    """MO-F (no pressure, free chain, axial force F_x at D on the axis), as functions of k_R: the joint
    and its rods carry F_x in parallel, (k + k_R) q = F_x."""
    den = [K_AX, ONE]
    q = ([Qp.of(Fx)], den)
    return {"q_m": q, "Tb_n": cf_scale(K_AX, q), "sum_rod_tension_n": ([ZERO, Qp.of(Fx)], den)}


def remote_forms(eps=F(0)):
    """J3-R's chain with rods, as functions of k_R (equal e_ref = eps on every rod): A - L - B - joint - C - R
    - D - X - E, A and E anchored, a region over pipe:X only (caps at D and E), p = 0 at the joint (Pe = 0)
    and in pipes L and R. Free bodies: one axial force N in L, the joint (Tb + sum T_r) and R; node D gives
    Nw_X = N + Pi; the pipe laws give elongations N/kp (L, R) and (N + (1 - 2 nu) Pi)/kp (X); anchored ends
    give zero total elongation: 3N + kp q = -(1 - 2 nu) Pi with N = (k + k_R) q - k_R eps, so
    (3k + kp + 3k_R) q = -(1 - 2 nu) Pi + 3 k_R eps."""
    k = K_AX
    eps = Qp.of(eps)
    lame = P_I - H_P                      # pipe X's net Lame term (1 - 2 nu) Pi
    d0 = k * 3 + KP
    den = [d0, Qp.of(3)]
    forms = {}
    q = ([-lame, eps * 3], den)
    forms["q_m"] = q
    forms["Tb_n"] = cf_scale(k, q)
    forms["sum_rod_tension_n"] = ([ZERO, -lame - eps * d0], den)
    S = cf_add(forms["Tb_n"], forms["sum_rod_tension_n"])
    forms["S_joint_n"] = S                # Pe = 0
    for p in ("L", "R"):
        forms["pipe_%s_Nw_n" % p] = S
        forms["pipe_%s_S_n" % p] = S
        forms["pipe_%s_elongation_m" % p] = cf_scale(1 / KP, S)
    forms["pipe_X_Nw_n"] = cf_add(S, cf_const(P_I, den))
    forms["pipe_X_S_n"] = S
    forms["pipe_X_elongation_m"] = cf_scale(1 / KP, cf_add(S, cf_const(lame, den)))
    forms["u_B_axial_m"] = forms["pipe_L_elongation_m"]
    forms["u_C_axial_m"] = cf_add(forms["u_B_axial_m"], q)
    forms["u_D_axial_m"] = cf_add(forms["u_C_axial_m"], forms["pipe_R_elongation_m"])
    forms["u_E_axial_m"] = ([ZERO], den)
    forms["reaction_A_axial_n"] = cf_scale(-1, S)
    forms["reaction_E_axial_n"] = S
    forms["interface_identity_S_pipe_minus_S_joint_n"] = ([ZERO], den)
    return forms


def remote_vals(g, sm):
    p = sm["pipes"]
    vals = {"q_m": sm["q"][0], "Tb_n": sm["Tb"], "sum_rod_tension_n": sm["sumT"], "S_joint_n": sm["S_joint"],
            "u_B_axial_m": axial_disp(g, sm, "node:B"), "u_C_axial_m": axial_disp(g, sm, "node:C"),
            "u_D_axial_m": axial_disp(g, sm, "node:D"), "u_E_axial_m": axial_disp(g, sm, "node:E"),
            "reaction_A_axial_n": dot(sm["reactions"]["support:A"][0], g["axis"]),
            "reaction_E_axial_n": dot(sm["reactions"]["support:E"][0], g["axis"]),
            "interface_identity_S_pipe_minus_S_joint_n": p["pipe:L"]["S"] - sm["S_joint"]}
    check(p["pipe:R"]["S"] == p["pipe:L"]["S"], "remote chain: both face pipes carry the same S")
    for x in ("L", "R", "X"):
        pv = p["pipe:" + x]
        vals["pipe_%s_Nw_n" % x] = pv["Nw"]
        vals["pipe_%s_S_n" % x] = pv["S"]
        vals["pipe_%s_elongation_m" % x] = pv["elong"]
    check(set(vals) == set(remote_forms()), "every remote-chain closed form has a solved counterpart")
    return vals


def asym_free_forms(a, eps=F(0)):
    """Free chain, one rod parallel to the axis at transverse offset a along connector y,
    as functions of the rod's k_r. The chain beyond the joint carries only the axial
    resultant Pe on the axis, so the connector-and-rod generalized action is (Pe, 0, ...):
      g_x: k qt_x + T = Pe;  g_rz: K_rz qr_z - a T = 0;  T = k_r (qt_x - a qr_z - e_ref)."""
    k = K_AX
    Krz = Qp.of(K_DIAG[5])
    a = Qp.of(a)
    eps = Qp.of(eps)
    den = [k * Krz, k * a * a + Krz]
    forms = {"qt_x_m": ([P_E * Krz, P_E * a * a + Krz * eps], den),
             "T_n": ([ZERO, Krz * (P_E - eps * k)], den),
             "qr_z_rad": ([ZERO, a * (P_E - eps * k)], den)}
    return forms


# ---------------------------------------------------------------------------
# Output formatting


def dec_text(d):
    if d == 0:
        return "0"
    with localcontext() as ctx:
        ctx.prec = 40
        r = +d
    s = format(r, "f")
    if "." in s:
        s = s.rstrip("0").rstrip(".")
    return s


def num(q):
    q = Qp.of(q)
    d = q.dec()
    return {"exact": q.text(), "decimal": dec_text(d), "value": float(d)}


def nums(v):
    return [num(x) for x in v]


def dumps(obj):
    return (json.dumps(obj, indent=1, ensure_ascii=False) + "\n").encode("utf-8")


FLOORS = {"force": Decimal(1), "moment": Decimal(1), "length": Decimal("1e-5"), "rotation": Decimal("1e-5")}
REL = Decimal("1e-9")


def distance(exp, wrong, family):
    e = exp.dec()
    w = wrong.dec()
    tol = REL * max(abs(e), FLOORS[family])
    return abs(w - e) / tol


def dist_text(d):
    with localcontext() as ctx:
        ctx.prec = 3
        r = +d
    return "%.2e" % r


# ---------------------------------------------------------------------------
# Observables


def observables(g, res):
    """Flattened (key, value, family) list of the frozen observables."""
    out = []
    ax = ("x", "y", "z")
    for k in range(3):
        out.append(("q.translation." + ax[k], res["q"][k], "length"))
    for k in range(3):
        out.append(("q.rotation." + ax[k], res["q"][3 + k], "rotation"))
    for k in range(3):
        out.append(("g.force." + ax[k], res["g"][k], "force"))
    for k in range(3):
        out.append(("g.moment." + ax[k], res["g"][3 + k], "moment"))
    out.append(("Tb", res["Tb"], "force"))
    for rid, rv in res["rods"].items():
        out.append(("rod.%s.extension" % rid, rv["e"], "length"))
        out.append(("rod.%s.tension" % rid, rv["T"], "force"))
        for end in ("end_i", "end_j"):
            for k in range(3):
                out.append(("rod.%s.%s.force.%s" % (rid, end, ax[k]), rv[end][1][k], "force"))
            for k in range(3):
                out.append(("rod.%s.%s.moment.%s" % (rid, end, ax[k]), rv[end][2][k], "moment"))
    out.append(("sum_rod_tension", res["sumT"], "force"))
    out.append(("S_joint", res["S_joint"], "force"))
    for pid, pv in res["pipes"].items():
        out.append(("pipe.%s.Nw" % pid, pv["Nw"], "force"))
        out.append(("pipe.%s.S" % pid, pv["Nw"] - g["Pi"][pid], "force"))
        for end in ("end_i", "end_j"):
            for k in range(3):
                out.append(("pipe.%s.%s.force.%s" % (pid, end, ax[k]), pv[end][1][k], "force"))
            for k in range(3):
                out.append(("pipe.%s.%s.moment.%s" % (pid, end, ax[k]), pv[end][2][k], "moment"))
    for face, node in (("face_i", g["ci"]), ("face_j", g["cj"])):
        pid = face_pipe(node)
        out.append(("identity.%s.S_pipe_minus_S_joint" % face,
                    res["pipes"][pid]["Nw"] - g["Pi"][pid] - res["S_joint"], "force"))
    for node in g["nodes"]:
        for k in range(3):
            out.append(("node.%s.translation.%s" % (node, ax[k]), res["u"][node][0][k], "length"))
        for k in range(3):
            out.append(("node.%s.rotation.%s" % (node, ax[k]), res["u"][node][1][k], "rotation"))
    for sid, (Fv, Mv) in res["reactions"].items():
        for k in range(3):
            out.append(("reaction.%s.force.%s" % (sid, ax[k]), Fv[k], "force"))
        for k in range(3):
            out.append(("reaction.%s.moment.%s" % (sid, ax[k]), Mv[k], "moment"))
    for end in ("end_i", "end_j"):
        for k in range(3):
            out.append(("connector_endpoint.%s.force.%s" % (end, ax[k]), res["endpoint"][end][1][k], "force"))
        for k in range(3):
            out.append(("connector_endpoint.%s.moment.%s" % (end, ax[k]), res["endpoint"][end][2][k], "moment"))
    return out


def face_pipe(node):
    return "pipe:L" if node == "node:B" else "pipe:R"


# ---------------------------------------------------------------------------
# Case evaluation


def evaluate(c):
    g = realize(c)
    sm = sm_solve(g)
    fm = fm_solve(g)
    compare_sm_fm(c["id"], sm, fm)
    # ledger representations A and B are equal (H-2) and rods carry no pressure load
    (la, poisson_a), lb = ledger_A(g), sm["ledger"]
    check(all(la[n] == lb[n] for n in g["nodes"]), "%s: representation A = representation B exactly" % c["id"])
    check(all(x.is_zero() for n in g["nodes"] for x in la[n][1]),
          "%s: the pressure RHS carries no nodal moment (coaxial, zero-offset joint)" % c["id"])
    check(all(vadd(la[n][0], poisson_a[n]) == sm["pressure_rhs"][n] for n in g["nodes"]),
          "%s: pressure RHS with the Poisson pairs, representation A = the stiffness solve's" % c["id"])
    # interface identity at both faces
    for node in (g["ci"], g["cj"]):
        pid = face_pipe(node)
        check(sm["pipes"][pid]["Nw"] - g["Pi"][pid] == sm["S_joint"],
              "%s: interface identity S_pipe = Tb + sum T_r (n.Qx) - Pe at %s" % (c["id"], node))
    # ruling 1: the projected rule reduces to section 9's literal Tb + sum T_r - Pe exactly when every rod is parallel
    literal = sm["Tb"] + sm["sumT"] - g["Pe"]
    if all(rv["n_axial"] == ONE for rv in sm["rods"].values()):
        check(literal == sm["S_joint"], "%s: parallel rods: projected S_joint = section 9's literal form" % c["id"])
    else:
        check(literal != sm["S_joint"], "%s: non-parallel rod: the literal form differs from the projected" % c["id"])
    # global equilibrium of the physical loads and reactions
    totF, totM = V(0, 0, 0), V(0, 0, 0)
    for node in g["nodes"]:
        Fv, Mv = sm["ledger"][node]
        totF = vadd(totF, Fv)
        totM = vadd(totM, vadd(Mv, cross(g["pos"][node], Fv)))
    for ld in g["loads"]:
        totF = vadd(totF, ld["force"])
        totM = vadd(totM, vadd(ld["moment"], cross(g["pos"][ld["node"]], ld["force"])))
    for sid, (Fv, Mv) in sm["reactions"].items():
        node = g["supports"][sid]
        totF = vadd(totF, Fv)
        totM = vadd(totM, vadd(Mv, cross(g["pos"][node], Fv)))
    check(all(x.is_zero() for x in totF + totM), "%s: global force and moment balance" % c["id"])
    return g, sm, fm


def axial_disp(g, res, node):
    return dot(res["u"][node][0], g["axis"])


def case_entry(c, g, sm, fm, case_sha):
    ent = {
        "description": c["description"],
        "family": c["family"],
        "case_description": "cases/%s.case.json" % c["id"],
        "case_sha256": case_sha,
        "chain": c["chain"],
        "orientation": "reversed" if c["reversed"] else ("rotated_translated" if c["rotated"] else "base"),
        "shear_model": "euler_bernoulli",
        "shear_dependence": ("none: every pipe shear force is exactly zero, so the Timoshenko model gives the same values"
                             if all(fm["pipes"][p]["shear_zero"] for p in fm["pipes"])
                             else "euler_bernoulli: pipe shear forces are nonzero; values hold for shear_model = euler_bernoulli only"),
        "method": "exact over Q(pi); nodal stiffness solve and force method agree exactly",
        "chain_axis_global": nums(g["axis"]),
        "connector_axis_x_global": nums([g["frame"][r][0] for r in range(3)]),
        "connector_end_i_node": g["ci"],
        "connector_end_j_node": g["cj"],
        "connector_pressure_model": c["model"],
        "connector_listed_by_region": g["listed"],
        "generalized": {"q_translation_m": nums(sm["q"][0:3]), "q_rotation_rad": nums(sm["q"][3:6]),
                        "g_force_n": nums(sm["g"][0:3]), "g_moment_nm": nums(sm["g"][3:6])},
        "Tb_gx_n": num(sm["Tb"]),
        "rods": {},
        "sum_rod_tension_n": num(sm["sumT"]),
        "Pe_n": num(g["Pe"]),
        "S_joint_n": num(sm["S_joint"]),
        "pipes": {},
        "interface_identity": {},
        "nodes": {},
        "reactions_support_on_model": {},
        "connector_endpoint_elastic_node_on_element": {},
        "pressure_rhs_global_n": {n: nums(sm["pressure_rhs"][n]) for n in g["nodes"]},
        "pressure_rhs_without_poisson_pairs_global_n": {n: nums(sm["ledger"][n][0]) for n in g["nodes"]},
    }
    if any(rv["n_axial"] != ONE for rv in sm["rods"].values()):
        ent["S_joint_rule"] = ("Tb + sum T_r (n_r . Q_x) - Pe, the projected rule (HELP_HUMAN ruling 1, 2026-10-17); "
                               "section 9's former literal Tb + sum T_r - Pe is the discriminator s_joint_section9_literal")
    for rid, rv in sm["rods"].items():
        ent["rods"][rid] = {
            "extension_m": num(rv["e"]), "tension_n": num(rv["T"]),
            "direction_global": nums(rv["n"]), "direction_dot_connector_x": num(rv["n_axial"]),
            "nodal_action_node_on_rod": {end: {"node": rv[end][0], "force_global_n": nums(rv[end][1]),
                                               "moment_global_nm": nums(rv[end][2])} for end in ("end_i", "end_j")}}
    for pid, pv in sm["pipes"].items():
        ent["pipes"][pid] = {
            "Nw_n": num(pv["Nw"]), "S_n": num(pv["S"]), "elongation_m": num(pv["elong"]),
            "end_actions_node_on_element": {end: {"node": pv[end][0], "force_global_n": nums(pv[end][1]),
                                                  "moment_global_nm": nums(pv[end][2]),
                                                  "force_local_n": nums(pv[end][3]),
                                                  "moment_local_nm": nums(pv[end][4])} for end in ("end_i", "end_j")}}
    for face, node in (("face_i", g["ci"]), ("face_j", g["cj"])):
        pid = face_pipe(node)
        ent["interface_identity"][face] = {"node": node, "pipe": pid, "S_pipe_n": num(sm["pipes"][pid]["S"]),
                                           "S_joint_n": num(sm["S_joint"]), "holds_exactly": True}
    for node in g["nodes"]:
        ent["nodes"][node] = {"translation_global_m": nums(sm["u"][node][0]),
                              "rotation_global_rad": nums(sm["u"][node][1]),
                              "axial_displacement_m": num(axial_disp(g, sm, node))}
    for sid, (Fv, Mv) in sm["reactions"].items():
        ent["reactions_support_on_model"][sid] = {"node": g["supports"][sid], "force_global_n": nums(Fv),
                                                  "moment_global_nm": nums(Mv)}
    for end in ("end_i", "end_j"):
        node, Fv, Mv = sm["endpoint"][end]
        ent["connector_endpoint_elastic_node_on_element"][end] = {"node": node, "force_global_n": nums(Fv),
                                                                  "moment_global_nm": nums(Mv)}
    if c["chain"] == "free":
        ent["outer_end_separation_change_m"] = num(axial_disp(g, sm, "node:D") - axial_disp(g, sm, "node:A"))
    return ent


# ---------------------------------------------------------------------------
# Closed-form checks per case


def axial_vals(chain, g, sm):
    vals = {"q_m": sm["q"][0], "Tb_n": sm["Tb"], "sum_rod_tension_n": sm["sumT"], "S_joint_n": sm["S_joint"],
            "Nw_n": sm["pipes"]["pipe:L"]["Nw"], "S_pipe_n": sm["pipes"]["pipe:L"]["S"],
            "reaction_A_axial_n": dot(sm["reactions"]["support:A"][0], g["axis"]),
            "u_B_axial_m": axial_disp(g, sm, "node:B"), "u_C_axial_m": axial_disp(g, sm, "node:C"),
            "u_D_axial_m": axial_disp(g, sm, "node:D"),
            "interface_identity_S_pipe_minus_S_joint_n": sm["pipes"]["pipe:L"]["S"] - sm["S_joint"]}
    if chain == "anchored":
        vals["reaction_D_axial_n"] = dot(sm["reactions"]["support:D"][0], g["axis"])
    else:
        vals["outer_end_separation_change_m"] = axial_disp(g, sm, "node:D") - axial_disp(g, sm, "node:A")
    check(sm["pipes"]["pipe:R"]["Nw"] == sm["pipes"]["pipe:L"]["Nw"], "both stubs carry the same Nw")
    check(set(vals) == set(axial_forms(chain)), "every axial closed form has a solved counterpart")
    return vals


def closed_form_entries(c, g, sm):
    out = []
    fam = c["family"]
    if fam in ("row2_ladder", "row4_ladder", "reference_extension"):
        eps = c["rods"][0]["e_ref"]
        forms = axial_forms(c["chain"], eps)
        K = c["total_k"]
        vals = axial_vals(c["chain"], g, sm)
        nr = len(c["rods"])
        for rid, rv in sm["rods"].items():
            check(rv["e"] == sm["q"][0], "%s: symmetric rods: each rod's extension equals qt_x" % c["id"])
            check(rv["T"] * nr == sm["sumT"], "%s: equal rods share the tension equally" % c["id"])
        for key, form in forms.items():
            v = kr_eval(form, K)
            check(v == vals[key], "%s: closed form %s at k_R" % (c["id"], key))
            out.append({"quantity": key, "variable": "k_R = total rod stiffness (N/m)"
                        + ("; e_ref = %s m on every rod" % Qp.of(eps).text() if eps else ""),
                        "form": kr_text(form, "k_R"), "value": num(v),
                        "limit_k_R_to_infinity": num(kr_lim_inf(form)), "limit_k_R_to_zero": num(kr_lim_zero(form))})
    if fam in ("offset_pair", "mechanical_only_free"):
        mo = fam == "mechanical_only_free"
        Kp = [Qp.of(K_DIAG[k]) for k in range(6)]
        Ksum = ZERO
        for r in c["rods"]:
            y, z = Qp.of(r["o_B"][1]), Qp.of(r["o_B"][2])
            Kp[0] = Kp[0] + r["k"]
            Kp[4] = Kp[4] + r["k"] * z * z
            Kp[5] = Kp[5] + r["k"] * y * y
            Ksum = Ksum + r["k"]
        ld = c["loads"][0]
        M = V(*ld["moment"])
        # the axial resultant beyond the joint, on the axis: Pe (pressurized free chain) or the applied F_x
        check(all(x == 0 for x in ld["force"][1:]) and (mo or ld["force"][0] == 0),
              "%s: the applied force, if any, is axial" % c["id"])
        Fx = Qp.of(ld["force"][0]) if mo else P_E
        gprime = [Fx, ZERO, ZERO] + M
        qcf = [gprime[k] / Kp[k] for k in range(6)]
        check(sm["q"] == qcf, "%s: q = diag(K + sum k c c^T)^-1 (%s, 0, 0, M): cross terms cancel" %
              (c["id"], "F_x" if mo else "Pe"))
        for r in c["rods"]:
            y, z = Qp.of(r["o_B"][1]), Qp.of(r["o_B"][2])
            check(sm["rods"][r["id"]]["e"] == qcf[0] + z * qcf[4] - y * qcf[5] and
                  sm["rods"][r["id"]]["T"] == r["k"] * (qcf[0] + z * qcf[4] - y * qcf[5] - r["e_ref"]),
                  "%s: %s e = qt_x + z qr_y - y qr_z and T = k_r (e - e_ref)" % (c["id"], r["id"]))
        # node kinematics of the free chain: pipe end-moment cantilevers and the midpoint basis
        L = Qp.of(L_STUB)
        Lj = Qp.of(L_JOINT)
        Nw_free = Fx if mo else P_I          # free end D: the wall carries the axial resultant (and the cap Pi)
        check(all(sm["pipes"][p]["Nw"] == Nw_free for p in sm["pipes"]), "%s: free chain Nw from statics" % c["id"])
        delta = (Nw_free - (ZERO if mo else H_P)) / KP
        ex = V(1, 0, 0)

        def pipe_tip(M):
            rot = vadd(vsc(L / GJ, [M[0], ZERO, ZERO]), vsc(L / EI, [ZERO, M[1], M[2]]))
            trans = vadd(vsc(delta, ex), vsc(L * L / (EI * 2), cross([ZERO, M[1], M[2]], ex)))
            return trans, rot
        uB, thB = pipe_tip(M)
        thC = vadd(thB, qcf[3:6])
        tc = vsc(F(1, 2), vadd(thB, thC))
        uC = vadd(vadd(uB, qcf[0:3]), cross(tc, vsc(Lj, ex)))
        dtrans, drot = pipe_tip(M)
        thD = vadd(thC, drot)
        uD = vadd(vadd(uC, dtrans), cross(thC, vsc(L, ex)))
        check(sm["u"]["node:B"] == (uB, thB) and sm["u"]["node:C"] == (uC, thC) and sm["u"]["node:D"] == (uD, thD),
              "%s: node translations and rotations equal the cantilever-and-midpoint closed forms" % c["id"])
        for k, nm in ((4, "qr_y_rad"), (5, "qr_z_rad")):
            if gprime[k].is_zero():
                continue
            kdiag = Qp.of(K_DIAG[k])
            # per-rod k_r as the variable (all rods equal here)
            kr = c["rods"][0]["k"]
            coef = ZERO
            for r in c["rods"]:
                o = Qp.of(r["o_B"][2 if k == 4 else 1])
                coef = coef + o * o * (r["k"] / kr)
            form = ([gprime[k]], [kdiag, coef])
            check(kr_eval(form, kr) == sm["q"][k], "%s: %s closed form" % (c["id"], nm))
            out.append({"quantity": nm, "variable": "k_r = each rod's stiffness (N/m)", "form": kr_text(form, "k_r"),
                        "value": num(kr_eval(form, kr)), "limit_k_r_to_infinity": num(kr_lim_inf(form)),
                        "limit_k_r_to_zero": num(kr_lim_zero(form)),
                        "meaning": "relative rotation uses K_connector + sum k_r o^2 (= 2 k_r a^2 per pair)"})
        if not mo:
            axial = axial_forms("free")
            check(kr_eval(axial["q_m"], Ksum) == sm["q"][0] and kr_eval(axial["sum_rod_tension_n"], Ksum) == sm["sumT"],
                  "%s: axial result equals row 4 at the same total rod stiffness (moment does not couple)" % c["id"])
            out.append({"quantity": "q_m", "variable": "k_R = total rod stiffness (N/m)",
                        "form": kr_text(axial["q_m"], "k_R"), "value": num(sm["q"][0]),
                        "meaning": "unaffected by the moment: row 4's q at the same k_R"})
        else:
            forms = mo_free_forms(Fx)
            check(kr_eval(forms["q_m"], Ksum) == sm["q"][0] and kr_eval(forms["Tb_n"], Ksum) == sm["Tb"] and
                  kr_eval(forms["sum_rod_tension_n"], Ksum) == sm["sumT"],
                  "%s: axial closed forms at k_R (the moment does not couple)" % c["id"])
            check(sm["S_joint"] == Fx and sm["Tb"] + sm["sumT"] == Fx and g["Pe"].is_zero(),
                  "%s: S_joint = Tb + sum T_r = F_x (Pe = 0)" % c["id"])
            check(sm["reactions"]["support:A"] == (vsc(-1, [Fx, ZERO, ZERO]), vsc(-1, M)),
                  "%s: support A reacts -F_x and -M (the force acts on the axis through A)" % c["id"])
            check(axial_disp(g, sm, "node:D") - axial_disp(g, sm, "node:A") == sm["q"][0] + Fx * 2 / KP,
                  "%s: separation = q + 2 F_x/kp" % c["id"])
            for key in ("q_m", "Tb_n", "sum_rod_tension_n"):
                out.append({"quantity": key, "variable": "k_R = total rod stiffness (N/m)",
                            "form": kr_text(forms[key], "k_R"), "value": num(kr_eval(forms[key], Ksum)),
                            "limit_k_R_to_infinity": num(kr_lim_inf(forms[key])),
                            "limit_k_R_to_zero": num(kr_lim_zero(forms[key]))})
            out.append({"quantity": "statics",
                        "form": "Nw = S_pipe = S_joint = Tb + sum T_r = F_x (p = 0 everywhere, Pe = 0); support A "
                                "reacts (-F_x, 0, 0) and -M; each rod T = k_r (qt_x - y qr_z) with y = +/-a; "
                                "separation = q + F_x (2 L)/(E A_s)",
                        "value": num(Fx),
                        "meaning": "JR J-C's mechanical-only tied case: the rods and bellows share F_x in proportion "
                                   "to stiffness; as k_r -> infinity qt_x -> 0 and qr_z -> 0 (two ideal rods fix both)"})
    if fam in ("asymmetric", "asymmetric_reference_extension") and c["chain"] == "free":
        r = c["rods"][0]
        forms = asym_free_forms(r["o_B"][1], r["e_ref"])
        qloc = [sm["q"][k] * (T_REV[k] if c["reversed"] else 1) for k in range(6)]
        check(kr_eval(forms["qt_x_m"], r["k"]) == qloc[0], "%s: asymmetric free qt_x closed form" % c["id"])
        check(kr_eval(forms["T_n"], r["k"]) == sm["rods"][r["id"]]["T"], "%s: asymmetric free T closed form" % c["id"])
        check(kr_eval(forms["qr_z_rad"], r["k"]) == qloc[5], "%s: asymmetric free qr_z closed form" % c["id"])
        check(all(qloc[k].is_zero() for k in (1, 2, 3, 4)), "%s: asymmetric free: other q components zero" % c["id"])
        for key, form in forms.items():
            out.append({"quantity": key, "variable": "k_r = the rod's stiffness (N/m)"
                        + ("; e_ref = %s m" % Qp.of(r["e_ref"]).text() if r["e_ref"] else ""),
                        "form": kr_text(form, "k_r"), "value": num(kr_eval(form, r["k"])),
                        "limit_k_r_to_infinity": num(kr_lim_inf(form)), "limit_k_r_to_zero": num(kr_lim_zero(form))})
        out.append({"quantity": "series stiffness", "form": "k_s = k_r K_rz/(K_rz + k_r a^2); qt_x = (Pe + k_s e_ref)/(k + k_s); "
                    "T = k_s (qt_x - e_ref); qr_z = a T/K_rz (connector-axis components; reversed: q' = T q)",
                    "meaning": "an offset rod acts on the axis through the connector's rotational stiffness at lever a; "
                    "as k_r -> infinity the joint is not axially rigid"})
    if fam in ("mechanical_only_thermal", "remote_region"):
        eps = c["rods"][0]["e_ref"]
        check(all(r["e_ref"] == eps for r in c["rods"]), "%s: equal e_ref on every rod" % c["id"])
        if fam == "mechanical_only_thermal":
            epsT = ALPHA * c["dT"]
            forms = axial_forms("anchored", eps, eps_T=epsT, pressurized=False)
            vals = axial_vals("anchored", g, sm)
            var = "k_R = total rod stiffness (N/m); e_ref = %s m on every rod; eps_T = %s" % (
                Qp.of(eps).text(), Qp.of(epsT).text())
        else:
            forms = remote_forms(eps)
            vals = remote_vals(g, sm)
            var = "k_R = total rod stiffness (N/m)" + ("; e_ref = %s m on every rod" % Qp.of(eps).text() if eps else "")
        K = c["total_k"]
        for rid, rv in sm["rods"].items():
            check(rv["e"] == sm["q"][0], "%s: symmetric rods: each rod's extension equals qt_x" % c["id"])
            check(rv["T"] * len(c["rods"]) == sm["sumT"], "%s: equal rods share the tension equally" % c["id"])
        check(all(x.is_zero() for x in sm["q"][1:]) and all(x.is_zero() for n in g["nodes"] for x in sm["u"][n][1]),
              "%s: no non-axial response" % c["id"])
        for key, form in forms.items():
            v = kr_eval(form, K)
            check(v == vals[key], "%s: closed form %s at k_R" % (c["id"], key))
            out.append({"quantity": key, "variable": var, "form": kr_text(form, "k_R"), "value": num(v),
                        "limit_k_R_to_infinity": num(kr_lim_inf(form)), "limit_k_R_to_zero": num(kr_lim_zero(form))})
        if fam == "mechanical_only_thermal":
            out.append({"quantity": "relation",
                        "form": "(k + k_R + kp/2) q = Pe - (1 - 2 nu) Pi - EA eps_T + k q_ref + k_R e_ref: t4_u5a's "
                                "thermal anchored relation extended with the rods; here p = 0 (Pi = Pe = 0) and q_ref = 0",
                        "meaning": "positive eps_T closes the joint; the rods add k_R to the joint's stiffness and "
                                   "k_R e_ref to its load (pretension e_ref < 0 also closes it)"})
        else:
            out.append({"quantity": "relation",
                        "form": "(3k + kp + 3 k_R) q = -(1 - 2 nu) Pi + 3 k_R e_ref; N = (k + k_R) q - k_R e_ref is "
                                "the axial force of L, the joint and R; Nw_X = N + Pi",
                        "meaning": "p = 0 at the joint (Pe = 0); its only load is pipe X's net Lame elongation "
                                   "(1 - 2 nu) Pi/kp carried through the anchored chain; as k_R -> 0 every form "
                                   "equals t4_u5a's %s" % c["base"]})
    return out


# ---------------------------------------------------------------------------
# Element-level controls


def element_controls():
    out = {}
    origin = V(7, -3, 5)
    rigid = []
    # generic skew rod and connector with offsets
    xi = V(1, 2, 3)
    xj = V(F(19, 10), F(23, 10), F(13, 5))
    oi = V(F(1, 5), F(2, 5), F(-1, 5))
    oj = V(F(-1, 10), F(3, 10), F(1, 2))
    samples = [("skew rod with offsets at both ends", xi, xj, oi, oj)]
    for c in build_configs():
        g = realize(c)
        for r in g["rods"]:
            samples.append(("%s %s" % (c["id"], r["id"]), g["pos"][r["ni"]], g["pos"][r["nj"]], r["oi"], r["oj"]))
    count = 0
    for name, a, b, o1, o2 in samples:
        C, n = rod_C(a, b, o1, o2)
        for mode in range(6):
            t = unit(mode, 3) if mode < 3 else V(0, 0, 0)
            w = unit(mode - 3, 3) if mode >= 3 else V(0, 0, 0)
            d = vadd(t, cross(w, vsub(a, origin))) + w + vadd(t, cross(w, vsub(b, origin))) + w
            check(dot(C, d).is_zero(), "rigid mode %d leaves %s unextended" % (mode, name))
            count += 1
    out["rod_rigid_modes"] = {"checked": count, "result": "C d = 0 exactly for all six infinitesimal rigid modes "
                              "(three translations, three rotations about the arbitrary origin (7, -3, 5) m) of every rod "
                              "in every case and of a generic skew rod with offsets at both ends"}
    # the end-i-only arm sign error breaks a rigid mode
    C, n = rod_C(xi, xj, oi, oj, "arm_sign_end_i")
    bad = []
    for mode in range(3, 6):
        w = unit(mode - 3, 3)
        d = cross(w, vsub(xi, origin)) + w + cross(w, vsub(xj, origin)) + w
        bad.append(dot(C, d))
    check(any(not x.is_zero() for x in bad), "arm-sign error at end i breaks a rigid rotation")
    out["arm_sign_end_i_breaks_rigid_modes"] = {
        "rod": {"x_i": nums(xi), "x_j": nums(xj), "offset_i": nums(oi), "offset_j": nums(oj)},
        "extension_under_unit_rigid_rotation_about_x_y_z_m": nums(bad),
        "meaning": "a rod whose offset arm has the wrong sign at one end extends under rigid rotation"}
    # connector B: direct kinematics equals JR's printed matrix; rigid modes with offsets
    ai = V(F(1, 5), F(2, 5), F(-1, 5))
    aj = V(F(-1, 10), F(3, 10), F(1, 2))
    xi2 = V(1, 2, 3)
    xj2 = vsub(vadd(vadd(xi2, ai), V(2, 0, 0)), aj)
    for frame in (qmat(I3), qmat(ROT)):
        Bd = connector_B(xi2, xj2, ai, aj, frame)
        Bm = connector_B_matrix_form(xi2, xj2, ai, aj, frame)
        check(Bd == Bm, "connector B from the kinematics equals JR's printed B")
        for mode in range(6):
            t = unit(mode, 3) if mode < 3 else V(0, 0, 0)
            w = unit(mode - 3, 3) if mode >= 3 else V(0, 0, 0)
            d = vadd(t, cross(w, vsub(xi2, origin))) + w + vadd(t, cross(w, vsub(xj2, origin))) + w
            check(all(x.is_zero() for x in mvec(Bd, d)), "connector rigid mode %d gives q = 0" % mode)
    # the JR refutation's arithmetic case
    Bd = connector_B(xi2, xj2, ai, aj, qmat(I3))
    gq = V(2, -3, 5, 7, -11, 13)
    fint = mtvec(Bd, gq)
    check(fint == V(-2, 3, -5, F(-42, 5), F(87, 5), F(-43, 5), 2, -3, 5, 10, F(-9, 2), F(157, 10)),
          "connector end actions equal the JR refutation's arithmetic case")
    dd = [Qp.of(F(k - 4, 17)) for k in range(12)]
    check(mvec(Bd, dd) == V(F(73, 170), F(-26, 85), F(29, 34), F(6, 17), F(6, 17), F(6, 17)),
          "connector q equals the JR refutation's arithmetic case")
    out["connector_B"] = {"result": "B evaluated from JR's midpoint kinematics equals JR's printed B (identity and "
                          "skew frames, offsets at both ends); q = 0 for all six rigid modes; the JR refutation's "
                          "end actions and q for g = (2, -3, 5, 7, -11, 13) and d_k = (k - 4)/17 are reproduced exactly"}
    # pipe element rigid modes
    L, lam = pipe_frame(V(0, 0, 0), V(2, 0, 0), V(0, 1, 0))
    T = blockdiag4(lam)
    kg = mmul(mT(T), mmul(eb_local(L), T))
    for mode in range(6):
        t = unit(mode, 3) if mode < 3 else V(0, 0, 0)
        w = unit(mode - 3, 3) if mode >= 3 else V(0, 0, 0)
        d = vadd(t, cross(w, vsub(V(0, 0, 0), origin))) + w + vadd(t, cross(w, vsub(V(2, 0, 0), origin))) + w
        check(all(x.is_zero() for x in mvec(kg, d)), "pipe element rigid mode %d is force-free" % mode)
    # generalized-basis forms: C = c B and B^T (sum k c c^T) B = sum C^T k C
    forms = {}
    zero = V(0, 0, 0)
    xB, xC = V(2, 0, 0), V(F(23, 10), 0, 0)
    Bq = connector_B(xB, xC, zero, zero, qmat(I3))
    k = Qp.of(K_ROD_PAIR)
    a = Qp.of(A_OFF)
    layouts = {"pair_y": pair_y(K_ROD_PAIR, axial=True), "pair_z": pair_z(K_ROD_PAIR),
               "four_rods": pair_y(K_ROD_PAIR, axial=True) + pair_z(K_ROD_PAIR),
               "single_y": single_y(K_ROD_PAIR),
               "skew_single": [rod("rod:sk", (-S_OFF, A_OFF, 0), (S_OFF, A_OFF, F(3, 10)), K_ROD_PAIR)]}
    for name, rods in layouts.items():
        Kg = zeros(6, 6)
        K12 = zeros(12, 12)
        for r in rods:
            o1, o2 = V(*r["o_B"]), V(*r["o_C"])
            C, n = rod_C(xB, xC, o1, o2)
            m = vsc(F(1, 2), vadd(o1, o2))
            c = list(n) + cross(m, n)
            check(C == mtvec(Bq, c), "%s %s: C = c B with c = (n, ((o_i + o_j)/2) x n)" % (name, r["id"]))
            kr = Qp.of(r["k"])
            for i in range(6):
                for j in range(6):
                    Kg[i][j] = Kg[i][j] + kr * c[i] * c[j]
            for i in range(12):
                for j in range(12):
                    K12[i][j] = K12[i][j] + kr * C[i] * C[j]
        check(mmul(mT(Bq), mmul(Kg, Bq)) == K12, "%s: B^T K_rods B = sum C^T k C" % name)
        forms[name] = {"rods": [{"id": r["id"], "offset_i_m": nums(V(*r["o_B"])), "offset_j_m": nums(V(*r["o_C"])),
                                 "k_n_per_m": num(r["k"])} for r in rods],
                       "generalized_rod_stiffness_6x6": [[num(x) for x in row] for row in Kg]}
    pairy = layouts["pair_y"]
    out["generalized_basis"] = {
        "rule": "for rods whose end nodes are the connector's (zero connector offsets), e = c . q with "
                "c = (Q^T n, Q^T(((o_i + o_j)/2) x n)); for a rod parallel to the axis at transverse offset (y, z) in "
                "connector axes c = (1, 0, 0, 0, z, -y), so k c c^T adds k to K_xx, k z^2 to K_ry, k y^2 to K_rz, "
                "and cross terms k z (x, ry), -k y (x, rz), -k y z (ry, rz) that cancel for a pair at +/-a",
        "pair_at_plus_minus_a_y": "K_xx = 2 k_r, K_rz = 2 k_r a^2, all cross terms zero",
        "pair_at_plus_minus_a_z": "K_xx = 2 k_r, K_ry = 2 k_r a^2, all cross terms zero",
        "four_rods": "K_xx = 4 k_r, K_ry = K_rz = 2 k_r a^2: both relative bending rotations stiffened; K_rx unchanged "
                     "(rods parallel to the axis add no torsional stiffness at first order)",
        "single_rod_at_plus_a_y": "K_xx = k_r, K_rz = k_r a^2, K_x,rz = -k_r a: axial load couples into relative rotation",
        "checked": "C = c B and B^T (sum k c c^T) B = sum C^T k C exactly for every layout below "
                   "(a = 0.1 m, axial attachment offsets -/+0.05 m)",
        "layouts": forms}
    # explicit structure checks of the pair and four-rod forms
    Kpy = [[Qp.of(0)] * 6 for _ in range(6)]
    for r in pairy:
        y = Qp.of(r["o_B"][1])
        c = V(1, 0, 0, 0, 0, 0)
        c[5] = -y
        for i in range(6):
            for j in range(6):
                Kpy[i][j] = Kpy[i][j] + Qp.of(r["k"]) * c[i] * c[j]
    expect = zeros(6, 6)
    expect[0][0] = 2 * k
    expect[5][5] = 2 * k * a * a
    check(Kpy == expect, "pair at +/-a e_y: K_xx = 2 k_r, K_rz = 2 k_r a^2, cross terms zero")
    return out


# ---------------------------------------------------------------------------
# t4_u5a bindings (data only)


def sha256(b):
    return hashlib.sha256(b).hexdigest()


def load_u5a():
    sums = (U5A / "SHA256SUMS").read_text().splitlines()
    listed = {}
    for line in sums:
        h, p = line.split(None, 1)
        listed[p.strip()] = h
    data = {}
    for rel, h in U5A_PINNED.items():
        b = (U5A / rel).read_bytes()
        check(sha256(b) == h, "t4_u5a %s SHA-256 equals the pinned value" % rel)
        check(listed.get(rel) == h, "t4_u5a SHA256SUMS lists the pinned %s" % rel)
        data[rel] = json.loads(b)
    return data


def bind_fixture(u5a):
    req = u5a["requests/J3-1.request.json"]["model"]
    pos = {n["id"]: (n["position"]["x"], n["position"]["y"], n["position"]["z"]) for n in req["nodes"]}
    for n in NODES:
        check(pos[n] == tuple(float(F(x)) for x in BASE_POS[n]), "J3-1 node %s position equals the fixture" % n)
    for p in req["pipe_segments"]:
        check(p["section"]["outside_diameter"]["value"] == float(OD) and p["section"]["wall_thickness"]["value"] == float(WALL),
              "J3-1 %s section equals the fixture" % p["id"])
    mat = req["materials"][0]
    check(mat["elastic_modulus"]["value"] == float(E_PA) and mat["poisson_ratio"]["value"] == float(NU),
          "J3-1 material equals the fixture")
    oc = req["components"][0]["objective_connector"]
    ut = oc["stiffness"]["upper_triangle"]
    diag_idx = [0, 6, 11, 15, 18, 20]
    check([ut[i] for i in diag_idx] == [float(x) for x in K_DIAG] and
          all(ut[i] == 0.0 for i in range(21) if i not in diag_idx), "J3-1 connector 6x6 equals the fixture diagonal")
    check(oc["pressure_model"]["effective_area"]["value"] == float(AE), "J3-1 effective area equals the fixture")
    check(req["load_cases"][0]["pressure_regions"][0]["pressure"]["value"] == float(P_PA), "J3-1 pressure equals the fixture")
    check(oc["connector_axes_global"] == [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]], "J3-1 connector axes")
    r1r = u5a["requests/J3-1R.request.json"]["model"]
    gR = realize(config("x", "anchored", [], "", "", "", rotated=True))
    for nd in r1r["nodes"]:
        check((nd["position"]["x"], nd["position"]["y"], nd["position"]["z"]) ==
              tuple(float(x.dec()) for x in gR["pos"][nd["id"]]), "J3-1R node %s equals the rotated fixture" % nd["id"])
    check(r1r["components"][0]["objective_connector"]["connector_axes_global"] ==
          [[float(x) for x in row] for row in ROT], "J3-1R connector axes equal the rotation (row-major, columns = axes)")
    r1v = u5a["requests/J3-1V.request.json"]["model"]["components"][0]["objective_connector"]
    check(r1v["connector_axes_global"] == [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]] and
          r1v["end_i"]["node_ref"] == "node:C", "J3-1V reversal: end_i at C, Q' = Q J")
    r5u = u5a["requests/J3-5U.request.json"]["model"]["components"][0]["objective_connector"]["stiffness"]["upper_triangle"]
    check(r5u[0] == 20200000.0, "J3-5U axial stiffness is k + k_rod = 2.02e7")
    # J3-1T: the thermal pattern of MO-T
    r1t = u5a["requests/J3-1T.request.json"]["model"]
    th = r1t["load_cases"][0]["primitive_loads"]
    check(sorted((x["id"], x["category"], x["target"]["pipe"], x["magnitude"]["value"], x["magnitude"]["unit"])
                 for x in th) == [("load:T-L", "thermal", "pipe:L", float(DT_STUBS), "degC"),
                                  ("load:T-R", "thermal", "pipe:R", float(DT_STUBS), "degC")],
          "J3-1T thermal loads: +10 degC on pipe:L and pipe:R")
    check(r1t["materials"][0]["thermal_expansion_coefficient"]["value"] == float(ALPHA), "J3-1T alpha equals the fixture")
    # J3-R and J3-RP: the remote-region chain of RR-U and RR-P
    gr = realize(config("x", "anchored", [], "", "", "", pressure="remote"))
    for rel, model in (("requests/J3-R.request.json", MODEL_UNPR), ("requests/J3-RP.request.json", MODEL_CONST)):
        m = u5a[rel]["model"]
        check([(n["id"], (n["position"]["x"], n["position"]["y"], n["position"]["z"])) for n in m["nodes"]] ==
              [(n, tuple(float(x.dec()) for x in gr["pos"][n])) for n in NODES_R], "%s nodes equal the fixture" % rel)
        check([(p["id"], p["from"], p["to"]) for p in m["pipe_segments"]] ==
              [("pipe:L", "node:A", "node:B"), ("pipe:J", "node:B", "node:C"), ("pipe:R", "node:C", "node:D"),
               ("pipe:X", "node:D", "node:E")] and
              all(p["section"]["outside_diameter"]["value"] == float(OD) and
                  p["section"]["wall_thickness"]["value"] == float(WALL) for p in m["pipe_segments"]),
              "%s pipes and sections equal the fixture" % rel)
        check(sorted((s["id"], s["node"], s["family"]) for s in m["supports"]) ==
              [("support:A", "node:A", "anchor"), ("support:E", "node:E", "anchor")], "%s anchors A and E" % rel)
        reg = m["load_cases"][0]["pressure_regions"]
        check(len(reg) == 1 and reg[0]["member_pipe_ids"] == ["pipe:X"] and not reg[0].get("member_connector_ids") and
              reg[0]["pressure"]["value"] == float(P_PA) and
              [(t["node_ref"], t["closure_transfer"]) for t in reg[0]["terminals"]] ==
              [("node:D", "transfers_to_wall"), ("node:E", "transfers_to_wall")] and
              not m["load_cases"][0]["primitive_loads"],
              "%s: one region over pipe:X only, terminals D and E transferring, not listing the connector" % rel)
        oc = m["components"][0]["objective_connector"]
        check(oc["pressure_model"]["kind"] == model and oc["hardware"]["kind"] == "untied" and
              oc["end_i"]["node_ref"] == "node:B" and oc["end_j"]["node_ref"] == "node:C" and
              oc["connector_axes_global"] == [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] and
              [oc["stiffness"]["upper_triangle"][i] for i in (0, 6, 11, 15, 18, 20)] == [float(x) for x in K_DIAG],
              "%s connector: %s, untied, B to C, the fixture 6x6" % (rel, model))
        if model == MODEL_CONST:
            check(oc["pressure_model"]["effective_area"]["value"] == float(AE), "%s effective area" % rel)


def u5a_compare(u5a, label, case, res, g, combined_tb=False):
    """Compare a solve with a t4_u5a oracle case's exact values."""
    o = u5a["oracle.json"]["cases"][case]
    sc = o["scalars"]
    n = 0

    def eq(exact_text, value, what):
        nonlocal n
        check(parse_exact(exact_text) == value, "%s vs t4_u5a %s: %s" % (label, case, what))
        n += 1
    eq(sc["q_m"]["exact"], res["q"][0], "q")
    tb = res["Tb"] + res["sumT"] if combined_tb else res["Tb"]
    eq(sc["Tb_gt_x_n"]["exact"], tb, "Tb (U5a's untied g_x)" + (" = Tb + sum T_r" if combined_tb else ""))
    # an unpressurized connector (J3-R) publishes neither Pe nor S_joint in t4_u5a
    check(("S_joint_n" in sc) == ("Pe_n" in sc) == (o["connector_pressure_model"] == MODEL_CONST),
          "t4_u5a %s publishes Pe and S_joint exactly for a constant-effective-area connector" % case)
    if "S_joint_n" in sc:
        eq(sc["S_joint_n"]["exact"], res["S_joint"], "S_joint")
        eq(sc["Pe_n"]["exact"], g["Pe"], "Pe")
    check(list(o["pipes"]) == [p for p, _, _ in g["pipes"]] and list(o["nodes"]) == g["nodes"] and
          set(o["reactions_support_on_model"]) == set(g["supports"]), "%s vs t4_u5a %s: same members" % (label, case))
    for pid in o["pipes"]:
        eq(o["pipes"][pid]["Nw_n"]["exact"], res["pipes"][pid]["Nw"], pid + " Nw")
        eq(o["pipes"][pid]["S_n"]["exact"], res["pipes"][pid]["S"], pid + " S")
        eq(o["pipes"][pid]["elongation_m"]["exact"], res["pipes"][pid]["elong"], pid + " elongation")
    for node in o["nodes"]:
        eq(o["nodes"][node]["axial_displacement_m"]["exact"], axial_disp(g, res, node), node + " axial displacement")
        for k in range(3):
            eq(o["nodes"][node]["displacement_global_m"][k]["exact"], res["u"][node][0][k], node + " translation")
            eq(o["nodes"][node]["rotation_global_rad"][k]["exact"], res["u"][node][1][k], node + " rotation")
    for sid, rv in o["reactions_support_on_model"].items():
        for k in range(3):
            eq(rv["force_global_n"][k]["exact"], res["reactions"][sid][0][k], sid + " force")
            eq(rv["moment_global_nm"][k]["exact"], res["reactions"][sid][1][k], sid + " moment")
    if "outer_end_separation_change_m" in sc:
        eq(sc["outer_end_separation_change_m"]["exact"], axial_disp(g, res, "node:D") - axial_disp(g, res, "node:A"),
           "outer-end separation")
    return n


def u5a_rhs_compare(u5a, label, case, rhs, R):
    """Compare a pressure RHS (caps, joint pair and Poisson pairs) with R times t4_u5a's pressure_rhs.global_n."""
    o = u5a["oracle.json"]["cases"][case]["pressure_rhs"]["global_n"]
    check(list(o) == list(rhs), "%s vs t4_u5a %s: pressure RHS covers the same nodes" % (label, case))
    n = 0
    for node, vec in o.items():
        exp = mvec(R, [parse_exact(x["exact"]) for x in vec])
        check(rhs[node] == exp, "%s vs t4_u5a %s: pressure RHS at %s" % (label, case, node))
        n += 3
    return n


# ---------------------------------------------------------------------------
# Case descriptions (request-agnostic)


def inp(x):
    return num(Qp.of(x))


BASE_REQUEST = {"J3-1": "requests/J3-1.request.json", "J3-3": "requests/J3-3.request.json",
                "J3-R": "requests/J3-R.request.json", "J3-RP": "requests/J3-RP.request.json"}
PRESSURE_MODEL_CONST = {"kind": "constant_effective_area_reference_v1", "effective_area_m2": None,
                        "pressure_basis": "internal_differential_zero_external_v1",
                        "source_reference": "invented effective area (JR J3); not manufacturer data"}


def case_description(c):
    g = realize(c)
    base = c["base"]
    pres = c["pressure"]
    pattern = None
    if c["reversed"]:
        pattern = {"request": "requests/J3-1V.request.json", "request_sha256": U5A_PINNED["requests/J3-1V.request.json"],
                   "applied": "J3-1V's connector end reversal (end_i at node:C, end_j at node:B, Q' = Q J with "
                              "J = diag(-1, 1, -1)); each rod's ends are swapped with the connector's"}
    elif c["rotated"]:
        pattern = {"request": "requests/J3-1R.request.json", "request_sha256": U5A_PINNED["requests/J3-1R.request.json"],
                   "applied": "J3-1R's rigid rotation (rows below) and translation (10, 20, 5) m applied to every "
                              "position, axis, y_reference, rod offset and load"}
    changes = ["connector hardware: untied -> explicit_axial_rods_v1, bilateral_linear (%d rod%s, below)" %
               (len(c["rods"]), "" if len(c["rods"]) == 1 else "s")]
    if pres == "none":
        changes.append("connector pressure_model: constant_effective_area_reference_v1 -> unpressurized")
        changes.append("pressure region region:U5A removed: the load case has no pressure region (p = 0 everywhere)")
    if c["dT"]:
        changes.append("added thermal loads (below): J3-1T's load:T-L and load:T-R, +10 degC on pipe:L and pipe:R")
    if c["loads"]:
        changes.append("added nodal loads (below), in the same load case as the pressure region" if pres != "none"
                       else "added nodal loads (below), in load case load:U5A")
    elif not c["dT"]:
        changes.append("no added loads")
    changes.append("supports unchanged from the base")
    if pattern:
        changes.append("orientation: " + pattern["applied"])
    pos = g["pos"]
    frame = g["frame"]
    rods = []
    for r in g["rods"]:
        bi = vadd(pos[r["ni"]], r["oi"])
        bj = vadd(pos[r["nj"]], r["oj"])
        span = vsub(bj, bi)
        L = norm(span)
        rods.append({
            "id": r["id"],
            "end_i": {"node_ref": r["ni"], "offset_connector_axes_m": nums(mtvec(frame, r["oi"])),
                      "offset_global_m": nums(r["oi"])},
            "end_j": {"node_ref": r["nj"], "offset_connector_axes_m": nums(mtvec(frame, r["oj"])),
                      "offset_global_m": nums(r["oj"])},
            "law": {"kind": "bilateral_linear", "stiffness_n_per_m": num(r["k"]),
                    "reference_extension_m": num(r["e_ref"])},
            "source_reference": "invented T4-U5b tie rod (analytical control); not manufacturer data",
            "derived": {"attachment_i_global_m": nums(bi), "attachment_j_global_m": nums(bj), "length_m": num(L),
                        "direction_global": nums(vsc(1 / L, span))}})
    loads = [{"id": ld["id"], "node": ld["node"], "force_global_n": nums(ld["force"]),
              "moment_global_nm": nums(ld["moment"])} for ld in g["loads"]]
    supports = [{"id": sid, "node": node, "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]}
                for sid, node in g["supports"].items()]
    face = {"listed": "pressurized member", "none": "unpressurized member (no pressure region)",
            "remote": "unpressurized member (outside the region)"}[pres]
    spans = [("pipe:L", "node:A", "node:B", "%s; face pipe at node:B" % face),
             ("pipe:J", "node:B", "node:C", "replaced span (connector replaces_span); no stiffness, load or result"),
             ("pipe:R", "node:C", "node:D", "%s; face pipe at node:C" % face)]
    if pres == "remote":
        spans.append(("pipe:X", "node:D", "node:E", "pressurized member of region:U5A (closed ends at node:D and node:E)"))
    pipes = []
    for pid, ni, nj, role in spans:
        pipes.append({"id": pid, "from": ni, "to": nj, "outside_diameter_m": inp(OD), "wall_thickness_m": inp(WALL),
                      "mill_tolerance_m": inp(0), "corrosion_allowance_m": inp(0), "material": "material:invented-steel",
                      "y_reference": nums(g["yref"]), "role": role})
    if c["model"] == MODEL_CONST:
        pmodel = dict(PRESSURE_MODEL_CONST, effective_area_m2=inp(AE))
    else:
        pmodel = {"kind": "unpressurized"}
    if pres == "listed":
        region = {"id": "region:U5A", "member_pipe_ids": ["pipe:L", "pipe:R"],
                  "member_connector_ids": ["component:C-J"], "pressure_pa": inp(P_PA),
                  "pressure_basis": "internal_differential_zero_external_v1",
                  "terminals": [{"node_ref": "node:A", "closure_transfer": "transfers_to_wall"},
                                {"node_ref": "node:D", "closure_transfer": "transfers_to_wall"}]}
    elif pres == "remote":
        region = {"id": "region:U5A", "member_pipe_ids": ["pipe:X"], "member_connector_ids": [],
                  "pressure_pa": inp(P_PA), "pressure_basis": "internal_differential_zero_external_v1",
                  "terminals": [{"node_ref": "node:D", "closure_transfer": "transfers_to_wall"},
                                {"node_ref": "node:E", "closure_transfer": "transfers_to_wall"}]}
    else:
        region = None
    derives = {"reference": "validation/references/t4_u5a", "request": BASE_REQUEST[base],
               "request_sha256": U5A_PINNED[BASE_REQUEST[base]], "oracle_case": base,
               "orientation_pattern": pattern}
    if c["dT"]:
        derives["thermal_pattern"] = {
            "request": "requests/J3-1T.request.json", "request_sha256": U5A_PINNED["requests/J3-1T.request.json"],
            "applied": "J3-1T's thermal loads load:T-L and load:T-R (+10 degC on pipe:L and pipe:R) only; its "
                       "pressure region is not applied"}
    doc = {
        "schema": "openpipestress.t4_u5b_case_description/1",
        "id": c["id"],
        "status": ("request-agnostic case description for T4-U5b; not a model document of any version. A builder "
                   "authors the model 0.6.0 request from it once 0.6.0 defines explicit_axial_rods_v1"),
        "description": c["description"],
        "derives_from": derives,
        "changes_from_base": changes,
        "unchanged_from_base": [
            "model 0.5.0 project fields re-expressed in the 0.6.0 form by the builder (project id renamed per case)",
            "connector installed_reference_temperature, temperature_applicability, reference_state and stiffness provenance",
            ("load case id load:U5A and region id region:U5A, no other primitive loads" if pres != "none" else
             "load case id load:U5A (no pressure region); no primitive loads other than those added here"),
            "every id other than the rods' and the added loads'"],
        "physical_inputs": {
            "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa"},
            "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": "euler_bernoulli"},
            "material": {"id": "material:invented-steel", "elastic_modulus_pa": inp(E_PA), "poisson_ratio": inp(NU),
                         "thermal_expansion_per_degC": inp(ALPHA),
                         "note": "alpha used by the thermal loads" if c["dT"] else "no thermal load; alpha unused"},
            "nodes": [{"id": n, "position_m": nums(pos[n])} for n in g["nodes"]],
            "pipes": pipes,
            "connector": {
                "id": "component:C-J", "kind": "expansion_joint", "objective_connector_version": "1.0.0",
                "motion_basis": "symmetric_midpoint_small_rotation_v1",
                "end_i": {"node_ref": g["ci"], "offset_m": nums(V(0, 0, 0))},
                "end_j": {"node_ref": g["cj"], "offset_m": nums(V(0, 0, 0))},
                "connector_axes_global": [nums(row) for row in frame],
                "connector_axes_convention": "the matrix Q whose columns are the connector axes in global coordinates, "
                                             "stored row-major as in t4_u5a's requests (inferred from J3-1R)",
                "stiffness_diagonal": {"tx_n_per_m": inp(K_DIAG[0]), "ty_n_per_m": inp(K_DIAG[1]),
                                       "tz_n_per_m": inp(K_DIAG[2]), "rx_nm_per_rad": inp(K_DIAG[3]),
                                       "ry_nm_per_rad": inp(K_DIAG[4]), "rz_nm_per_rad": inp(K_DIAG[5]),
                                       "off_diagonal": "zero",
                                       "representation": "as the base: scaled_work_coefficients_v1, translation and rotation scales 1"},
                "q_ref": "zero (stress_free)",
                "topology": {"type": "replaces_span", "span_ref": "pipe:J"},
                "calibration": "constant_structural_elasticity_v1, includes_pressure_dependent_tangent false (as the base)",
                "pressure_model": pmodel,
                "hardware": {
                    "kind": "explicit_axial_rods_v1",
                    "shape_note": "neutral description in JR's proposed vocabulary (CONTRACT.md section 5); not the fixed 0.6.0 shape",
                    "rods": rods}},
            "pressure_region": region,
            "supports": supports,
            "loads": loads},
        "provenance": PROVENANCE,
    }
    if c["dT"]:
        doc["physical_inputs"]["thermal_loads"] = [
            {"id": "load:T-%s" % p[-1], "pipe": p, "temperature_change_degC": inp(c["dT"]),
             "uniform_strain": num(Qp.of(ALPHA * c["dT"]))} for p in ("pipe:L", "pipe:R")]
    if c["rotated"]:
        doc["physical_inputs"]["rotation_applied"] = [nums(row) for row in qmat(ROT)]
        doc["physical_inputs"]["translation_applied_m"] = nums(V(*TRANS))
    if c["family"] == "non_parallel":
        doc["admitted"] = ("HELP_HUMAN ruling 1 (2026-10-17): rods not parallel to the connector axis are admitted, "
                           "S_joint = Tb + sum T_r (n_r . Q_x) - Pe; a rod nearly perpendicular to the joint axis is "
                           "for the U5b implementation design (a named refusal if justified)")
    if not g["listed"]:
        doc["admitted"] = ("HELP_HUMAN ruling 2 (2026-10-17): rods are admitted with p = 0 at the joint (%s), so "
                           "Pe = 0 and S_joint = Tb + sum T_r (n_r . Q_x)" %
                           ("an unpressurized connector" if c["model"] == MODEL_UNPR
                            else "a constant-effective-area connector that no region lists"))
    return doc


# ---------------------------------------------------------------------------
# Discriminators


def ideal_values(c, g):
    """The k_R -> infinity values (JR's ideal tie) in place of the finite-rod values."""
    forms = axial_forms(c["chain"], c["rods"][0]["e_ref"])
    lim = {k: kr_lim_inf(v) for k, v in forms.items()}
    return lim


def mutant_record(g, sm, wrong, keys_filter=None):
    exp = {k: (v, fam) for k, v, fam in observables(g, sm)}
    wro = {k: v for k, v, fam in observables(g, wrong)}
    moved, unmoved, within = {}, [], []
    for k, (v, fam) in exp.items():
        if keys_filter and not keys_filter(k):
            continue
        w = wro[k]
        if w == v:
            unmoved.append(k)
            continue
        dd = distance(v, w, fam)
        if dd <= 1:
            within.append(k)
        moved[k] = {"wrong": num(w), "distance_tolerances": dist_text(dd), "_d": dd}
    return moved, unmoved, within


def folded(sm, g):
    """Rod forces folded into the connector's generalized action and its endpoint rows."""
    w = dict(sm)
    pos = g["pos"]
    frame = g["frame"]
    gf = list(sm["g"])
    for rid, rv in sm["rods"].items():
        r = [x for x in g["rods"] if x["id"] == rid][0]
        m = vsc(F(1, 2), vadd(r["oi"], r["oj"]))
        c = mtvec(frame, rv["n"]) + mtvec(frame, cross(m, rv["n"]))
        gf = vadd(gf, vsc(rv["T"], c))
    zero = V(0, 0, 0)
    Bm = connector_B(pos[g["ci"]], pos[g["cj"]], zero, zero, frame)
    fend = mtvec(Bm, gf)
    w["g"] = gf
    w["Tb"] = gf[0]
    w["endpoint"] = {"end_i": (g["ci"], fend[0:3], fend[3:6]), "end_j": (g["cj"], fend[6:9], fend[9:12])}
    w["S_joint"] = gf[0] + sm["sumT_axial"] - g["Pe"]     # the projected rule, on the folded g'_x
    return w


def apply_ideal(sm, g, c):
    lim = ideal_values(c, g)
    w = dict(sm)
    axis = g["axis"]
    w["q"] = [lim["q_m"]] + list(sm["q"][1:])
    w["g"] = [K_AX * lim["q_m"]] + list(sm["g"][1:])
    w["Tb"] = lim["Tb_n"]
    w["sumT"] = lim["sum_rod_tension_n"]
    nr = len(sm["rods"])
    w["rods"] = {rid: dict(rv, e=lim["q_m"], T=lim["sum_rod_tension_n"] / nr) for rid, rv in sm["rods"].items()}
    w["S_joint"] = lim["S_joint_n"]
    w["pipes"] = {pid: dict(pv, Nw=lim["Nw_n"]) for pid, pv in sm["pipes"].items()}
    u = dict(sm["u"])
    for node, key in (("node:B", "u_B_axial_m"), ("node:C", "u_C_axial_m"), ("node:D", "u_D_axial_m")):
        u[node] = (vsc(lim[key], axis), sm["u"][node][1])
    w["u"] = u
    reac = {"support:A": (vsc(lim["reaction_A_axial_n"], axis), sm["reactions"]["support:A"][1])}
    if "support:D" in sm["reactions"]:
        reac["support:D"] = (vsc(lim["reaction_D_axial_n"], axis), sm["reactions"]["support:D"][1])
    w["reactions"] = reac
    return w


MUTANTS = {
    "j4_double_count": "JR J4: the joint face caps dropped, so the attachments carry -/+ p Ae instead of -/+ p(Ae - Ai) "
                       "(terminal caps plus the full joint pair); formed only where a region lists the joint",
    "rods_lumped_on_centreline": "rod offsets ignored: each rod's attachments moved onto the connector centreline at its "
                                 "end nodes (o_i = o_j = 0), so the rod has no moment arm and its direction becomes "
                                 "(x_j - x_i)/|x_j - x_i| = Q_x; SK-F's rod direction (0.8, 0, 0.6) becomes Q_x = "
                                 "(1, 0, 0) and n . Q_x becomes 1",
    "offset_sign_error": "the rod offset arm with the wrong sign at both ends (v_b = u - theta x o): a mirror of every rod",
    "arm_sign_end_i_only": "the rod offset arm with the wrong sign at end i only (breaks the rigid modes)",
    "e_ref_ignored": "the reference extension ignored in both the reference load and the recovery (T = k e)",
    "reference_load_sign_flipped": "the reference load assembled as -C^T k e_ref; recovery T = k(e - e_ref) kept",
    "reference_load_omitted": "the reference load omitted; recovery T = k(e - e_ref) kept",
    "rod_force_folded_into_g": "rod forces folded into the connector's generalized action, g' = g + sum c_r T_r, and so "
                               "into its elastic rows (g, Tb = g'_x and the endpoint rows B^T g'); S_joint then formed "
                               "by the projected rule g'_x + sum T_r (n_r . Q_x) - Pe, which is g'_x + sum T_r - Pe for "
                               "parallel rods and uses n . Q_x = 4/5 on SK-F",
    "s_joint_without_rods": "S_joint published as Tb - Pe (sum T_r left out)",
    "ideal_in_place_of_finite": "the k_R -> infinity (ideal tie) values published in place of the finite-rod values; "
                                "scored only on q_x, g_x, Tb, sum T_r, S_joint, each rod's e and T_r, each pipe's Nw "
                                "and S, the identity at both faces, every node translation and every reaction force "
                                "(31 observables in L2-CL, 33 in L2-PR, 28 in L4-CL, 30 in L4-PR; per case "
                                "scored_count); unmoved_count counts only those (16 anchored, 20 free)",
    "s_joint_section9_literal": "section 9's former literal rule S_joint = Tb + sum T_r - Pe (no projection) in place of "
                                "the projected Tb + sum T_r (n_r . Q_x) - Pe (HELP_HUMAN ruling 1, 2026-10-17); "
                                "identical for parallel rods, so formed only where a rod is not parallel (SK-F)",
}


def IDEAL_KEYS(k):
    if k in ("q.translation.x", "g.force.x", "Tb", "sum_rod_tension", "S_joint"):
        return True
    if k.startswith("rod.") and (k.endswith(".extension") or k.endswith(".tension")):
        return True
    if k.startswith("pipe.") and (k.endswith(".Nw") or k.endswith(".S")):
        return True
    if k.startswith("identity."):
        return True
    if k.startswith("node.") and ".translation." in k:
        return True
    return k.startswith("reaction.") and ".force." in k


def run_discriminators(results):
    out = {name: {"mutant": desc, "cases": {}} for name, desc in MUTANTS.items()}
    for cid, (c, g, sm, fm) in results.items():
        applicable = (["j4_double_count"] if g["listed"] else []) + ["rod_force_folded_into_g", "s_joint_without_rods"]
        offs = any(any(x != 0 for x in r["o_B"][1:]) or any(x != 0 for x in r["o_C"][1:]) for r in c["rods"])
        if offs:
            applicable += ["rods_lumped_on_centreline", "offset_sign_error", "arm_sign_end_i_only"]
        if any(r["e_ref"] != 0 for r in c["rods"]):
            applicable += ["e_ref_ignored", "reference_load_sign_flipped", "reference_load_omitted"]
        if c["family"] in ("row2_ladder", "row4_ladder"):
            applicable.append("ideal_in_place_of_finite")
        if any(rv["n_axial"] != ONE for rv in sm["rods"].values()):
            applicable.append("s_joint_section9_literal")
        for name in applicable:
            if name == "j4_double_count":
                wrong = sm_solve(g, {"ledger": "double_count"})
            elif name == "rods_lumped_on_centreline":
                wrong = sm_solve(g, {"kin": "lumped"})
            elif name == "offset_sign_error":
                wrong = sm_solve(g, {"kin": "offset_sign"})
            elif name == "arm_sign_end_i_only":
                wrong = sm_solve(g, {"kin": "arm_sign_end_i"})
            elif name == "e_ref_ignored":
                wrong = sm_solve(g, {"ref": "ignored"})
            elif name == "reference_load_sign_flipped":
                wrong = sm_solve(g, {"ref": "load_sign"})
            elif name == "reference_load_omitted":
                wrong = sm_solve(g, {"ref": "load_omitted"})
            elif name == "rod_force_folded_into_g":
                wrong = folded(sm, g)
            elif name == "s_joint_without_rods":
                wrong = dict(sm, S_joint=sm["Tb"] - g["Pe"])
            elif name == "s_joint_section9_literal":
                wrong = dict(sm, S_joint=sm["Tb"] + sm["sumT"] - g["Pe"])
            else:
                wrong = apply_ideal(sm, g, c)
            moved, unmoved, within = mutant_record(g, sm, wrong,
                                                   IDEAL_KEYS if name == "ideal_in_place_of_finite" else None)
            dmin = min((v["_d"] for v in moved.values()), default=None)
            for v in moved.values():
                del v["_d"]
            out[name]["cases"][cid] = {"moved": moved,
                                       "unmoved_count": len(unmoved),
                                       "smallest_distance_tolerances": dist_text(dmin) if dmin is not None else None,
                                       "moved_within_criterion": within}
            if name == "ideal_in_place_of_finite":
                out[name]["cases"][cid]["scored_count"] = len(moved) + len(unmoved)
            check(not within, "%s: %s moves no observable by less than the criterion" % (cid, name))
            # closed-form confirmation of the J4 values
            if name == "j4_double_count" and c["family"] in ("row2_ladder", "row4_ladder", "reference_extension"):
                forms = axial_forms(c["chain"], c["rods"][0]["e_ref"], double_count=True)
                K = c["total_k"]
                check(kr_eval(forms["q_m"], K) == wrong["q"][0] and
                      kr_eval(forms["sum_rod_tension_n"], K) == wrong["sumT"] and
                      kr_eval(forms["Nw_n"], K) == wrong["pipes"]["pipe:L"]["Nw"] and
                      kr_eval(forms["S_joint_n"], K) == wrong["S_joint"],
                      "%s: J4 double count equals its closed forms" % cid)
            if name in ("rods_lumped_on_centreline", "offset_sign_error") and c["family"] in (
                    "asymmetric", "asymmetric_reference_extension") and c["chain"] == "free":
                r = c["rods"][0]
                aa = 0 if name == "rods_lumped_on_centreline" else -r["o_B"][1]
                forms = asym_free_forms(aa, r["e_ref"])
                qloc = [wrong["q"][k] * (T_REV[k] if c["reversed"] else 1) for k in range(6)]
                check(kr_eval(forms["qt_x_m"], r["k"]) == qloc[0] and kr_eval(forms["qr_z_rad"], r["k"]) == qloc[5],
                      "%s: %s equals the asymmetric closed form with a -> %s" % (cid, name, aa))
    return out


# ---------------------------------------------------------------------------
# Oracle assembly


def flat_entry(v, path=""):
    """Flatten an oracle entry to {path: exact text or scalar}."""
    out = {}
    if isinstance(v, dict) and "exact" in v:
        out[path] = v["exact"]
    elif isinstance(v, dict):
        for k, x in v.items():
            out.update(flat_entry(x, path + "/" + k))
    elif isinstance(v, list):
        for i, x in enumerate(v):
            out.update(flat_entry(x, path + "/" + str(i)))
    else:
        out[path] = v
    return out


def build(u5a):
    cfgs = build_configs()
    cases_bytes = {}
    results = {}
    for c in cfgs:
        desc = case_description(c)
        cases_bytes[c["id"]] = dumps(desc)
    for c in cfgs:
        g, sm, fm = evaluate(c)
        results[c["id"]] = (c, g, sm, fm)
    entries = {}
    for cid, (c, g, sm, fm) in results.items():
        ent = case_entry(c, g, sm, fm, sha256(cases_bytes[cid]))
        cfs = closed_form_entries(c, g, sm)
        if cfs:
            ent["closed_forms"] = cfs
        entries[cid] = ent
    # local equivalence of the reversed and rotated copies
    for chain in ("F", "A"):
        base = results["AS-%s" % chain]
        rev = results["AS-%s-V" % chain]
        rot = results["AS-%s-R" % chain]
        b_sm, v_sm, r_sm = base[2], rev[2], rot[2]
        R = rot[1]["R"]
        check(v_sm["q"] == [b_sm["q"][k] * T_REV[k] for k in range(6)] and
              v_sm["g"] == [b_sm["g"][k] * T_REV[k] for k in range(6)], "AS-%s-V: q' = T q, g' = T g" % chain)
        check(all(v_sm["u"][n] == b_sm["u"][n] for n in NODES), "AS-%s-V: node results identical" % chain)
        check(v_sm["reactions"] == b_sm["reactions"], "AS-%s-V: reactions identical" % chain)
        check(v_sm["endpoint"]["end_i"][1:] == b_sm["endpoint"]["end_j"][1:] and
              v_sm["endpoint"]["end_j"][1:] == b_sm["endpoint"]["end_i"][1:], "AS-%s-V: endpoint blocks exchanged" % chain)
        check(all(v_sm["rods"][k]["e"] == b_sm["rods"][k]["e"] and v_sm["rods"][k]["T"] == b_sm["rods"][k]["T"]
                  for k in b_sm["rods"]), "AS-%s-V: rod e and T identical" % chain)
        check(v_sm["S_joint"] == b_sm["S_joint"] and v_sm["Tb"] == b_sm["Tb"], "AS-%s-V: Tb and S_joint identical" % chain)
        for k in b_sm["rods"]:
            bv, vv = b_sm["rods"][k], v_sm["rods"][k]
            check(vv["n"] == vsc(-1, bv["n"]), "AS-%s-V: rod %s direction negated" % (chain, k))
            check(vv["end_i"] == bv["end_j"] and vv["end_j"] == bv["end_i"],
                  "AS-%s-V: rod %s end labels swapped; each node's action on the rod unchanged" % (chain, k))
        # literal: every published difference between the reversed copy and its base is one of these
        fb, fv = flat_entry(entries["AS-%s" % chain]), flat_entry(entries["AS-%s-V" % chain])
        check(set(fb) == set(fv), "AS-%s-V: the same published keys as AS-%s" % (chain, chain))
        allowed = ("/description", "/case_description", "/case_sha256", "/orientation", "/connector_end_i_node",
                   "/connector_end_j_node", "/connector_axis_x_global/", "/connector_endpoint_elastic_node_on_element/",
                   "/interface_identity/face_i/node", "/interface_identity/face_i/pipe",
                   "/interface_identity/face_j/node", "/interface_identity/face_j/pipe",
                   "/generalized/q_translation_m/1", "/generalized/q_rotation_rad/1",
                   "/generalized/g_force_n/1", "/generalized/g_moment_nm/1")
        diff = [k for k in fb if fb[k] != fv[k]]
        check(all(k.startswith(allowed) or (k.startswith("/rods/") and ("/direction_global/" in k or
                                                                       "/nodal_action_node_on_rod/" in k))
                  for k in diff), "AS-%s-V: differs from AS-%s only by the named exceptions" % (chain, chain))
        check(any("/direction_global/" in k for k in diff) and any("/nodal_action_node_on_rod/" in k for k in diff),
              "AS-%s-V: the rod direction and nodal-action labels do differ" % chain)
        check(all(fv[k] == ("-" + fb[k] if not fb[k].startswith("-") else fb[k][1:]) or fb[k] == "0"
                  for k in diff if "/direction_global/" in k or "/connector_axis_x_global/" in k),
              "AS-%s-V: the rod direction and Q_x are negated" % chain)
        check(r_sm["q"] == b_sm["q"] and r_sm["g"] == b_sm["g"] and r_sm["S_joint"] == b_sm["S_joint"],
              "AS-%s-R: local q, g, S_joint identical" % chain)
        check(all(r_sm["rods"][k]["T"] == b_sm["rods"][k]["T"] for k in b_sm["rods"]), "AS-%s-R: rod T identical" % chain)
        check(all(r_sm["pipes"][p]["Nw"] == b_sm["pipes"][p]["Nw"] for p in b_sm["pipes"]),
              "AS-%s-R: Nw identical" % chain)
        check(all(r_sm["u"][n][0] == mvec(R, b_sm["u"][n][0]) and r_sm["u"][n][1] == mvec(R, b_sm["u"][n][1])
                  for n in NODES), "AS-%s-R: global translations and rotations = R x base" % chain)
        check(all(r_sm["reactions"][s][0] == mvec(R, b_sm["reactions"][s][0]) and
                  r_sm["reactions"][s][1] == mvec(R, b_sm["reactions"][s][1]) for s in b_sm["reactions"]),
              "AS-%s-R: reactions = R x base" % chain)
        for pid in ("pipe:L", "pipe:R"):
            check(r_sm["pipes"][pid]["end_j"][3:] == b_sm["pipes"][pid]["end_j"][3:] and
                  r_sm["pipes"][pid]["end_i"][3:] == b_sm["pipes"][pid]["end_i"][3:],
                  "AS-%s-R: %s local end actions identical" % (chain, pid))
        entries["AS-%s-V" % chain]["local_equivalence"] = (
            "checked exactly against AS-%s: q' = T q and g' = T g with T = diag(1, -1, 1, 1, -1, 1); node results, "
            "reactions, rod e and T, Tb and S_joint identical; connector endpoint blocks exchanged. Every other "
            "published difference is one of: Q_x (connector_axis_x_global) negated and the connector's end nodes "
            "swapped; each rod's direction_global negated; each rod's end labels swapped (end_i at node:C, carrying "
            "the base's end_j action unchanged); the interface identity's face labels swapped" % chain)
        entries["AS-%s-R" % chain]["local_equivalence"] = (
            "checked exactly against AS-%s: q, g, rod e and T, Tb, S_joint, Nw and pipe local end actions identical; "
            "global translations, rotations and reactions are R times AS-%s's" % (chain, chain))
    # J4 control: ladder closed forms and limits
    disc = run_discriminators(results)
    return cfgs, cases_bytes, results, entries, disc


def closed_form_block():
    out = {}
    for row, chain in (("row2_anchored", "anchored"), ("row4_free", "free")):
        blk = {}
        for dc in (False, True):
            forms = axial_forms(chain, double_count=dc)
            blk["j4_double_count" if dc else "correct"] = {
                key: {"form": kr_text(f, "k_R"), "limit_k_R_to_infinity": num(kr_lim_inf(f)),
                      "limit_k_R_to_zero": num(kr_lim_zero(f))} for key, f in forms.items()}
        out[row] = blk
    out["variable"] = "k_R = total rod stiffness sum k_r (N/m); e_ref = 0; centreline rod or symmetric pair (identical axial results)"
    out["derivation"] = ("axial free bodies: node B equilibrium Nw - (Tb + sum T_r) = -(Pe - Pi) (J4: -Pe), the pipe law "
                         "Nw = EA eps + 2 nu Pi and, anchored, symmetry u_B = -q/2; free, Nw = Pi from the free end D. "
                         "Anchored: (k + k_R + kp/2) q = 2 nu Pi + Pe - Pi; free: (k + k_R) q = Pe")
    return out


def invariance_block(results):
    """A single centreline rod and a symmetric pair at +/-a with the same total stiffness."""
    out = []
    for row in ("L2", "L4"):
        for tag, K in LADDER:
            a = results["%s-CL-%s" % (row, tag)][2]
            b = results["%s-PR-%s" % (row, tag)][2]
            same = (a["q"] == b["q"] and a["g"] == b["g"] and a["Tb"] == b["Tb"] and a["sumT"] == b["sumT"]
                    and a["S_joint"] == b["S_joint"] and all(a["u"][n] == b["u"][n] for n in NODES)
                    and a["reactions"] == b["reactions"] and a["endpoint"] == b["endpoint"]
                    and all(a["pipes"][p]["Nw"] == b["pipes"][p]["Nw"] and a["pipes"][p]["end_i"] == b["pipes"][p]["end_i"]
                            and a["pipes"][p]["end_j"] == b["pipes"][p]["end_j"] for p in a["pipes"]))
            check(same, "%s-%s: centreline rod and symmetric pair agree exactly" % (row, tag))
            check(all(rv["T"] * 2 == a["sumT"] for rv in b["rods"].values()),
                  "%s-PR-%s: each rod of the pair carries half the total" % (row, tag))
            out.append({"centreline_case": "%s-CL-%s" % (row, tag), "pair_case": "%s-PR-%s" % (row, tag),
                        "k_R_n_per_m": num(K), "identical_exactly": True})
    return {"rule": "q, g, Tb, sum T_r, S_joint, every node translation and rotation, every reaction, the "
                    "connector endpoint actions, Nw and every pipe end action are identical exactly; each rod of "
                    "the pair carries sum T_r / 2. Only the rods' own nodal-action moments differ (the pair's "
                    "offset arms)", "pairs": out}


def j4_block():
    out = {}
    for row, chain in (("row2_anchored", "anchored"), ("row4_free", "free")):
        c = axial_forms(chain)
        w = axial_forms(chain, double_count=True)
        diffs = {}
        for key in c:
            if key not in w:
                continue
            d = cf_add(w[key], cf_scale(-1, c[key]))
            diffs[key] = {"wrong_minus_correct": kr_text(d, "k_R"), "limit_k_R_to_infinity": num(kr_lim_inf(d)),
                          "limit_k_R_to_zero": num(kr_lim_zero(d))}
        out[row] = diffs
    a, aw = axial_forms("anchored"), axial_forms("anchored", double_count=True)
    f, fw = axial_forms("free"), axial_forms("free", double_count=True)
    K = Qp.of(F(123456789, 7))  # a generic k_R for the identity checks below
    den_a = D0_ANCH + K
    check(kr_eval(aw["q_m"], K) - kr_eval(a["q_m"], K) == P_I / den_a, "J4 anchored: q_w - q = Pi/(k + k_R + kp/2)")
    check(kr_eval(aw["Nw_n"], K) - kr_eval(a["Nw_n"], K) == -P_I * KP / 2 / den_a,
          "J4 anchored: Nw_w - Nw = -Pi (kp/2)/(k + k_R + kp/2)")
    check(kr_eval(aw["reaction_A_axial_n"], K) - kr_eval(a["reaction_A_axial_n"], K) == P_I * KP / 2 / den_a,
          "J4 anchored: R_A,w - R_A = Pi (kp/2)/(k + k_R + kp/2)")
    check(kr_eval(aw["sum_rod_tension_n"], K) - kr_eval(a["sum_rod_tension_n"], K) == P_I * K / den_a,
          "J4 anchored: sum T_w - sum T = Pi k_R/(k + k_R + kp/2)")
    check(kr_eval(fw["q_m"], K) - kr_eval(f["q_m"], K) == P_I / (K_AX + K), "J4 free: q_w - q = Pi/(k + k_R)")
    check(kr_eval(fw["Nw_n"], K) == kr_eval(f["Nw_n"], K) and
          kr_eval(fw["reaction_A_axial_n"], K) == kr_eval(f["reaction_A_axial_n"], K), "J4 free: Nw and reactions unchanged")
    check(kr_eval(aw["interface_identity_S_pipe_minus_S_joint_n"], K) == -P_I and
          kr_eval(fw["interface_identity_S_pipe_minus_S_joint_n"], K) == -P_I, "J4 breaks the interface identity by -Pi")
    out["statement"] = (
        "Double count (face caps dropped; the attachments carry -/+ p Ae). Anchored, at finite k_R: "
        "q_w - q = Pi/(k + k_R + kp/2), Nw_w - Nw = -Pi (kp/2)/(k + k_R + kp/2), R_A,w - R_A = -(R_D,w - R_D) = "
        "Pi (kp/2)/(k + k_R + kp/2) and sum T_w - sum T = Pi k_R/(k + k_R + kp/2); as k_R -> infinity q, Nw and the "
        "reactions coincide with the correct values while sum T_r -> 800 + 75 pi instead of 800 - 50 pi (wrong by "
        "exactly Pi). Free, at every k_R: Nw = Pi and the reactions (zero) are unchanged, q_w - q = Pi/(k + k_R), "
        "sum T_r -> Pe + Pi instead of Pe. In both chains S_pipe - S_joint = -Pi: the interface identity is broken "
        "at every k_R, including both limits")
    return out


def jr_literal_checks(results, u5a):
    lits = []

    def lit(name, value, expected, how="exact"):
        check(value == expected, "JR literal: " + name)
        lits.append({"check": name, "value": num(value), "jr": num(expected), "agrees": how})

    def litdec(name, value, text):
        v = value.dec()
        lit_v = Decimal(text)
        exp_ = lit_v.as_tuple().exponent
        half_ulp = Decimal(5) * Decimal(10) ** (exp_ - 1)
        check(abs(v - lit_v) <= half_ulp, "JR refutation decimal: " + name)
        lits.append({"check": name, "value": num(value), "jr_refutation_decimal": text, "agrees": "to the digits given"})
    a = axial_forms("anchored")
    f = axial_forms("free")
    aw = axial_forms("anchored", double_count=True)
    fw = axial_forms("free", double_count=True)
    lit("row 2 limit: Tt = 800 - 50 pi", kr_lim_inf(a["sum_rod_tension_n"]), Qp.of(800) - PI * 50)
    lit("row 2 limit: Tb = 0", kr_lim_inf(a["Tb_n"]), ZERO)
    lit("row 2 limit: q = 0", kr_lim_inf(a["q_m"]), ZERO)
    lit("row 2 limit: Nw = 75 pi", kr_lim_inf(a["Nw_n"]), PI * 75)
    lit("row 2 limit: S = -50 pi", kr_lim_inf(a["S_joint_n"]), PI * -50)
    lit("row 2 limit: support A (on model) = +50 pi", kr_lim_inf(a["reaction_A_axial_n"]), PI * 50)
    lit("row 2 limit: support D (on model) = -50 pi", kr_lim_inf(a["reaction_D_axial_n"]), PI * -50)
    lit("row 4 limit: Tt = 800", kr_lim_inf(f["sum_rod_tension_n"]), Qp.of(800))
    lit("row 4 limit: Nw = 125 pi", kr_lim_inf(f["Nw_n"]), PI * 125)
    lit("row 4 limit: S = 0", kr_lim_inf(f["S_joint_n"]), ZERO)
    lit("row 4 limit: separation = 1/275000", kr_lim_inf(f["outer_end_separation_change_m"]), Qp.of(F(1, 275000)))
    K5 = F(20000000)
    q5 = kr_eval(f["q_m"], K5)
    lit("row 5: q = 800/20200000", q5, Qp.of(F(800, 20200000)))
    lit("row 5: Tb = 200000 q", kr_eval(f["Tb_n"], K5), q5 * 200000)
    lit("row 5: Tt = 20000000 q", kr_eval(f["sum_rod_tension_n"], K5), q5 * 20000000)
    lit("row 5: separation = q + 1/275000", kr_eval(f["outer_end_separation_change_m"], K5), q5 + F(1, 275000))
    lit("J4 tied anchored limit: Tt = 800 + 75 pi", kr_lim_inf(aw["sum_rod_tension_n"]), Qp.of(800) + PI * 75)
    lit("J4 tied anchored limit: Tt exceeds the correct 800 - 50 pi by exactly Pi",
        kr_lim_inf(aw["sum_rod_tension_n"]) - kr_lim_inf(a["sum_rod_tension_n"]), P_I)
    lit("J4 tied free limit: Tt = Pe + Pi", kr_lim_inf(fw["sum_rod_tension_n"]), P_E + P_I)
    lit("J4 tied anchored limit: q unchanged", kr_lim_inf(aw["q_m"]), kr_lim_inf(a["q_m"]))
    lit("J4 tied anchored limit: Nw unchanged", kr_lim_inf(aw["Nw_n"]), kr_lim_inf(a["Nw_n"]))
    lit("J4 tied anchored limit: reaction A unchanged", kr_lim_inf(aw["reaction_A_axial_n"]),
        kr_lim_inf(a["reaction_A_axial_n"]))
    lit("J4 untied (k_R -> 0) anchored: q = (h + Pe)/(k + kp/2)", kr_lim_zero(aw["q_m"]), (H_P + P_E) / D0_ANCH)
    lit("J4 untied (k_R -> 0) free: q = (Pe + Pi)/k", kr_lim_zero(fw["q_m"]), (P_E + P_I) / K_AX)
    litdec("row 2 ideal Tt = h + dP", kr_lim_inf(a["sum_rod_tension_n"]), "642.920367320510")
    litdec("row 2 ideal Nw = 75 pi", kr_lim_inf(a["Nw_n"]), "235.619449019234")
    litdec("row 2 ideal S = -50 pi", kr_lim_inf(a["S_joint_n"]), "-157.079632679490")
    litdec("row 5 q", q5, "0.0000396039603960396")
    litdec("row 5 Tb", kr_eval(f["Tb_n"], K5), "7.92079207920792")
    litdec("row 5 Tt", kr_eval(f["sum_rod_tension_n"], K5), "792.079207920792")
    litdec("J4 tied anchored Tt_wrong = Pe + h", kr_lim_inf(aw["sum_rod_tension_n"]), "1035.61944901923")
    litdec("J4 untied anchored q_wrong", kr_lim_zero(aw["q_m"]), "0.0000238639044078429")
    litdec("J4 untied free q_wrong", kr_lim_zero(fw["q_m"]), "0.00596349540849362")
    # t4_u5a's untied J4 mutant values at k_R -> 0
    m = u5a["oracle.json"]["j4_and_ledger_mutants"]
    lit("k_R -> 0 J4 anchored q equals t4_u5a J3-1 double_count q", kr_lim_zero(aw["q_m"]),
        parse_exact(m["J3-1"]["double_count"]["q_m"]["exact"]))
    lit("k_R -> 0 J4 anchored S_joint equals t4_u5a J3-1 double_count S_joint", kr_lim_zero(aw["S_joint_n"]),
        parse_exact(m["J3-1"]["double_count"]["S_joint_n"]["exact"]))
    lit("k_R -> 0 J4 anchored Nw equals t4_u5a J3-1 double_count Nw_L", kr_lim_zero(aw["Nw_n"]),
        parse_exact(m["J3-1"]["double_count"]["Nw_L_n"]["exact"]))
    lit("k_R -> 0 J4 free q equals t4_u5a J3-3 double_count q", kr_lim_zero(fw["q_m"]),
        parse_exact(m["J3-3"]["double_count"]["q_m"]["exact"]))
    lit("k_R -> 0 J4 free S_joint equals t4_u5a J3-3 double_count S_joint", kr_lim_zero(fw["S_joint_n"]),
        parse_exact(m["J3-3"]["double_count"]["S_joint_n"]["exact"]))
    return lits


def convergence_block():
    """Exact 3D stiffness solves on the extended ladder; each equals the closed form."""
    out = {}
    for row, chain, base in (("row2_anchored", "anchored", "J3-1"), ("row4_free", "free", "J3-3")):
        forms = axial_forms(chain)
        lim_q = kr_lim_inf(forms["q_m"])
        lim_t = kr_lim_inf(forms["sum_rod_tension_n"])
        rows = []
        for K in [k for _, k in LADDER] + EXTRA_K:
            for real in ("CL", "PR"):
                if K == 0:
                    rods = []
                else:
                    rods = centreline_rod(K) if real == "CL" else pair_y(K / 2)
                if K == 0 and real == "PR":
                    continue
                c = config("conv", chain, rods, "", "", base)
                g = realize(c)
                sm = sm_solve(g)
                vals = axial_vals(chain, g, sm)
                for key, form in forms.items():
                    check(vals[key] == kr_eval(form, K),
                          "%s ladder k_R = %s (%s): 3D solve equals the closed form %s" % (row, K, real, key))
                check(all(x.is_zero() for x in sm["q"][1:]) and
                      all(x.is_zero() for n in NODES for x in sm["u"][n][1]),
                      "%s ladder k_R = %s (%s): no non-axial response" % (row, K, real))
                if real == "CL":
                    rows.append({"k_R_n_per_m": num(K), "q_m": num(sm["q"][0]), "sum_rod_tension_n": num(sm["sumT"]),
                                 "Nw_n": num(sm["pipes"]["pipe:L"]["Nw"]), "S_joint_n": num(sm["S_joint"]),
                                 "q_minus_limit_m": dec_text((sm["q"][0] - lim_q).dec()),
                                 "sum_rod_tension_minus_limit_n": dec_text((sm["sumT"] - lim_t).dec()),
                                 "product_case": K in [k for _, k in LADDER]})
        out[row] = {"limit_q_m": num(lim_q), "limit_sum_rod_tension_n": num(lim_t), "ladder": rows,
                    "note": "every row is an exact 3D stiffness solve for both realizations (centreline rod and "
                            "symmetric pair; identical axial results); every closed form of this row equals the "
                            "solve at all ten k_R values (nine for the pair), more than the degree bound needs "
                            "(number of rods + 3 points); k_R = 0 is the untied joint; the differences from the "
                            "limit fall as 1/k_R"}
    return out


def closed_form_identity_block():
    """Every closed form checked against exact 3D solves at enough stiffness values to prove identity:
    the solves are rational in the stiffness with numerator degree <= rods + 1 and denominator degree
    <= rods, the forms have degree <= 1, so agreement at rods + 3 distinct values proves the identity."""
    n = 0
    Ks = [k for _, k in LADDER] + EXTRA_K[1:]
    for chain, base in (("anchored", "J3-1"), ("free", "J3-3")):
        for eps in (F(0), E_REF):
            for dc in (False, True):
                forms = axial_forms(chain, eps, double_count=dc)
                for K in Ks:
                    for rods in (centreline_rod(K) if eps == 0 else None, pair_y(K / 2, axial=True, e_ref=eps)):
                        if rods is None:
                            continue
                        if eps:
                            for r in rods:
                                check(r["e_ref"] == eps, "e_ref sweep rods carry e_ref")
                        g = realize(config("cfid", chain, rods, "", "", base))
                        sm = sm_solve(g, {"ledger": "double_count"} if dc else None)
                        vals = axial_vals(chain, g, sm)
                        for key, form in forms.items():
                            check(vals[key] == kr_eval(form, K), "%s e_ref=%s J4=%s k_R=%s: closed form %s" %
                                  (chain, eps, dc, K, key))
                            n += 1
    krs = [F(10 ** 3), F(10 ** 4), F(10 ** 5), F(10 ** 6), F(10 ** 7), F(10 ** 9), F(10 ** 11)]
    for eps in (F(0), E_REF):
        forms = asym_free_forms(A_OFF, eps)
        for kr in krs:
            g = realize(config("cfid", "free", single_y(kr, eps), "", "", "J3-3"))
            sm = sm_solve(g)
            for key, idx in (("qt_x_m", 0), ("qr_z_rad", 5)):
                check(sm["q"][idx] == kr_eval(forms[key], kr), "one offset rod k_r=%s e_ref=%s: %s" % (kr, eps, key))
                n += 1
            check(sm["rods"]["rod:yp"]["T"] == kr_eval(forms["T_n"], kr), "one offset rod k_r=%s: T" % kr)
            n += 1
    mz = [{"id": "m", "node": "node:D", "force": (0, 0, 0), "moment": (12, -150, 100)}]
    for layout in ("pair_y", "pair_z", "four"):
        for kr in krs:
            rods = {"pair_y": pair_y(kr, axial=True), "pair_z": pair_z(kr),
                    "four": pair_y(kr, axial=True) + pair_z(kr)}[layout]
            g = realize(config("cfid", "free", rods, "", "", "J3-3", loads=mz))
            sm = sm_solve(g)
            coef_y = sum((Qp.of(r["o_B"][2]) ** 2 for r in rods), ZERO)
            coef_z = sum((Qp.of(r["o_B"][1]) ** 2 for r in rods), ZERO)
            for idx, coef, M in ((3, ZERO, 12), (4, coef_y, -150), (5, coef_z, 100)):
                form = ([Qp.of(M)], [Qp.of(K_DIAG[idx]), coef])
                check(sm["q"][idx] == kr_eval(form, kr), "%s k_r=%s: relative rotation %d closed form" % (layout, kr, idx))
                n += 1
            check(sm["q"][0] == kr_eval(axial_forms("free")["q_m"], kr * len(rods)),
                  "%s k_r=%s: axial result is row 4's at the same total stiffness" % (layout, kr))
            n += 1
    # p = 0 at the joint (ruling 2): MO-F's forms at seven k_r values for its pair
    fm_ = [{"id": "m", "node": "node:D", "force": (F_MO, 0, 0), "moment": (0, 0, M_MO)}]
    mo = mo_free_forms(F_MO)
    for kr in krs:
        rods = pair_y(kr, axial=True)
        g = realize(config("cfid", "free", rods, "", "", "J3-3", loads=fm_, pressure="none", model=MODEL_UNPR))
        sm = sm_solve(g)
        for key in ("q_m", "Tb_n", "sum_rod_tension_n"):
            v = {"q_m": sm["q"][0], "Tb_n": sm["Tb"], "sum_rod_tension_n": sm["sumT"]}[key]
            check(v == kr_eval(mo[key], kr * 2), "MO-F pair k_r=%s: %s closed form" % (kr, key))
            n += 1
        form = ([Qp.of(M_MO)], [Qp.of(K_DIAG[5]), Qp.of(A_OFF) ** 2 * 2])
        check(sm["q"][5] == kr_eval(form, kr), "MO-F pair k_r=%s: qr_z closed form" % kr)
        check(sm["S_joint"] == Qp.of(F_MO) and all(sm["pipes"][p]["Nw"] == Qp.of(F_MO) for p in sm["pipes"]),
              "MO-F pair k_r=%s: S_joint = Nw = F_x" % kr)
        n += 2
    # MO-T's forms (no pressure, stubs +10 degC, with and without e_ref) and the remote-region forms (both
    # connector models) at nine k_R values for the pair
    for eps in (F(0), E_REF):
        forms = axial_forms("anchored", eps, eps_T=ALPHA * DT_STUBS, pressurized=False)
        for K in Ks:
            g = realize(config("cfid", "anchored", pair_y(K / 2, axial=True, e_ref=eps), "", "", "J3-1",
                               pressure="none", model=MODEL_UNPR, dT=DT_STUBS))
            sm = sm_solve(g)
            vals = axial_vals("anchored", g, sm)
            for key, form in forms.items():
                check(vals[key] == kr_eval(form, K), "MO-T e_ref=%s k_R=%s: closed form %s" % (eps, K, key))
                n += 1
    forms = remote_forms()
    for model in (MODEL_UNPR, MODEL_CONST):
        for K in Ks:
            g = realize(config("cfid", "anchored", pair_y(K / 2, axial=True), "", "", "J3-R", pressure="remote",
                               model=model))
            sm = sm_solve(g)
            vals = remote_vals(g, sm)
            for key, form in forms.items():
                check(vals[key] == kr_eval(form, K), "remote %s k_R=%s: closed form %s" % (model, K, key))
                n += 1
    return {"rule": "every closed form above is checked against exact 3D stiffness solves at enough stiffness "
                    "values to prove it an identity in the stiffness (the solves are rational with numerator "
                    "degree <= rods + 1 and denominator degree <= rods; agreement at rods + 3 distinct values "
                    "suffices): rows 2 and 4, with and without e_ref, correct and J4, at nine k_R values for the "
                    "centreline rod and the pair; the single offset rod (with and without e_ref) and the "
                    "offset-pair and four-rod relative rotations at seven k_r values; for p = 0 at the joint, "
                    "MO-F's axial forms and relative rotation at seven k_r values for its pair, MO-T's forms (with "
                    "and without e_ref) and the remote-region forms (unpressurized and unlisted "
                    "constant-effective-area connector) at nine k_R values for the pair",
            "values_checked": n}


def u5a_block(u5a):
    out = {"bound_files": {rel: h for rel, h in U5A_PINNED.items()}, "comparisons": []}
    for chain, case in (("anchored", "J3-1"), ("free", "J3-3")):
        g = realize(config("k0", chain, [], "", "", case))
        sm = sm_solve(g)
        n = u5a_compare(u5a, "k_R = 0 solve", case, sm, g)
        forms = axial_forms(chain)
        o = u5a["oracle.json"]["cases"][case]
        check(parse_exact(o["scalars"]["q_m"]["exact"]) == kr_lim_zero(forms["q_m"]) and
              parse_exact(o["scalars"]["S_joint_n"]["exact"]) == kr_lim_zero(forms["S_joint_n"]) and
              parse_exact(o["pipes"]["pipe:L"]["Nw_n"]["exact"]) == kr_lim_zero(forms["Nw_n"]) and
              parse_exact(o["scalars"]["Tb_gt_x_n"]["exact"]) == kr_lim_zero(forms["Tb_n"]),
              "k_R -> 0 closed-form limits equal t4_u5a %s" % case)
        out["comparisons"].append({"t4_u5a_case": case, "against": "k_R -> 0: the closed-form limit and an exact solve "
                                   "with no rods", "values_compared_exactly": n + 4, "result": "all equal"})
    # p = 0 at the joint (ruling 2): J3-1T binds MO-T's thermal path and relation; J3-R and J3-RP are the
    # k_R -> 0 limits of RR-U and RR-P
    R0 = qmat(I3)
    rhs_rows = []
    for chain, case in (("anchored", "J3-1"), ("free", "J3-3")):
        g = realize(config("k0", chain, [], "", "", case))
        rhs_rows.append({"t4_u5a_case": case, "against": "an exact solve with no rods",
                         "values_compared_exactly": u5a_rhs_compare(u5a, "k_R = 0 solve", case,
                                                                    sm_solve(g)["pressure_rhs"], R0)})
    g = realize(config("k0T", "anchored", [], "", "", "J3-1", dT=DT_STUBS))
    sm = sm_solve(g)
    compare_sm_fm("J3-1T k_R = 0", sm, fm_solve(g))
    n = u5a_compare(u5a, "k_R = 0 thermal solve", "J3-1T", sm, g)
    rel = axial_forms("anchored", eps_T=ALPHA * DT_STUBS)
    o = u5a["oracle.json"]["cases"]["J3-1T"]
    check(parse_exact(o["scalars"]["q_m"]["exact"]) == kr_lim_zero(rel["q_m"]) and
          parse_exact(o["scalars"]["S_joint_n"]["exact"]) == kr_lim_zero(rel["S_joint_n"]) and
          parse_exact(o["pipes"]["pipe:L"]["Nw_n"]["exact"]) == kr_lim_zero(rel["Nw_n"]),
          "the rod-extended thermal relation at k_R -> 0 and J3-1T's inputs equals t4_u5a J3-1T")
    mot = axial_forms("anchored", eps_T=ALPHA * DT_STUBS, pressurized=False)
    j31 = axial_forms("anchored")
    check(kr_lim_zero(mot["q_m"]) + kr_lim_zero(j31["q_m"]) == parse_exact(o["scalars"]["q_m"]["exact"]),
          "superposition: MO-T's k_R -> 0 q (p = 0) + J3-1's q = t4_u5a J3-1T's q")
    out["comparisons"].append({
        "t4_u5a_case": "J3-1T", "for": "MO-T",
        "against": "an exact solve of J3-1T (pressurized, stubs +10 degC, no rods) by both methods; the rod-extended "
                   "thermal relation at k_R -> 0 with J3-1T's inputs (q, S_joint, Nw); superposition: MO-T's "
                   "k_R -> 0 q at p = 0 plus J3-1's q equals J3-1T's q",
        "values_compared_exactly": n + 4, "result": "all equal"})
    rhs_rows.append({"t4_u5a_case": "J3-1T", "against": "an exact solve with no rods",
                     "values_compared_exactly": u5a_rhs_compare(u5a, "k_R = 0 thermal solve", "J3-1T",
                                                                sm["pressure_rhs"], R0)})
    forms = remote_forms()
    for case, model, cid in (("J3-R", MODEL_UNPR, "RR-U"), ("J3-RP", MODEL_CONST, "RR-P")):
        g = realize(config("k0", "anchored", [], "", "", case, pressure="remote", model=model))
        sm = sm_solve(g)
        compare_sm_fm("%s k_R = 0" % case, sm, fm_solve(g))
        n = u5a_compare(u5a, "k_R = 0 solve", case, sm, g)
        o = u5a["oracle.json"]["cases"][case]
        lim = {k: kr_lim_zero(f) for k, f in forms.items()}
        pairs = [(o["scalars"]["q_m"], lim["q_m"]), (o["scalars"]["Tb_gt_x_n"], lim["Tb_n"])]
        if "S_joint_n" in o["scalars"]:
            pairs.append((o["scalars"]["S_joint_n"], lim["S_joint_n"]))
        for x in ("L", "R", "X"):
            pv = o["pipes"]["pipe:" + x]
            pairs += [(pv["Nw_n"], lim["pipe_%s_Nw_n" % x]), (pv["S_n"], lim["pipe_%s_S_n" % x]),
                      (pv["elongation_m"], lim["pipe_%s_elongation_m" % x])]
        for nd in ("B", "C", "D", "E"):
            pairs.append((o["nodes"]["node:" + nd]["axial_displacement_m"], lim["u_%s_axial_m" % nd]))
        for s in ("A", "E"):
            pairs.append((o["reactions_support_on_model"]["support:" + s]["force_global_n"][0],
                          lim["reaction_%s_axial_n" % s]))
        for exp_, val in pairs:
            check(parse_exact(exp_["exact"]) == val, "%s: remote closed-form limit k_R -> 0 equals t4_u5a" % case)
        out["comparisons"].append({
            "t4_u5a_case": case, "for": cid,
            "against": "k_R -> 0: the remote-region closed-form limits and an exact solve with no rods (both methods)",
            "values_compared_exactly": n + len(pairs), "result": "all equal"})
        rhs_rows.append({"t4_u5a_case": case, "against": "an exact solve with no rods",
                         "values_compared_exactly": u5a_rhs_compare(u5a, "k_R = 0 solve", case, sm["pressure_rhs"], R0)})
    out["pressure_rhs_comparisons"] = rhs_rows
    return out


def pressure_rhs_binding_block(u5a, results):
    """Finding 1: every case's pressure RHS (caps, joint pair and Poisson pairs) equals t4_u5a's pressure_rhs.global_n
    of its base, rotated with the case; rods never change it."""
    rows = []
    for cid, (c, g, sm, fm) in results.items():
        rhs = sm["pressure_rhs"]
        if c["pressure"] == "none":
            check(all(x.is_zero() for n in g["nodes"] for x in rhs[n]), "%s: no pressure region: pressure RHS zero" % cid)
            rows.append({"case": cid, "t4_u5a_case": None, "values_compared_exactly": 3 * len(g["nodes"]),
                         "result": "no pressure region: every entry exactly zero"})
            continue
        n = u5a_rhs_compare(u5a, cid, c["base"], rhs, g["R"])
        row = {"case": cid, "t4_u5a_case": c["base"], "transform": "R" if c["rotated"] else "identity",
               "values_compared_exactly": n, "result": "all equal"}
        direct = "J3-1R" if (c["rotated"] and c["chain"] == "anchored") else (
            "J3-1V" if (c["reversed"] and c["chain"] == "anchored") else None)
        if direct:
            row["also_equal_to"] = direct
            row["values_compared_exactly"] += u5a_rhs_compare(u5a, cid, direct, rhs, qmat(I3))
        rows.append(row)
    return {"rule": "pressure_rhs_global_n is t4_u5a's convention (its pressure_rhs.global_n, which Design section 8 "
                    "binds to the product's assembled_pressure_rhs_global): the caps at the region's transferring "
                    "terminals, the joint pair -/+ p(Ae - Ai) along Q_x at a joint the region lists, and every "
                    "pressurized pipe's Poisson pair +/- 2 nu Pi along the member; rod reference loads, thermal "
                    "eigenloads and applied loads are not pressure terms. Every case equals its t4_u5a base's vector "
                    "(rotated with the case), so rods never change it; AS-A-R and AS-A-V also equal J3-1R's and "
                    "J3-1V's directly", "cases": rows}


def row5_block(u5a, results):
    out = []
    for cid, case in (("L4-CL-2E7", "J3-5U"), ("L4-PR-2E7", "J3-5U"), ("L2-CL-2E7", "J3-5UA"), ("L2-PR-2E7", "J3-5UA")):
        c, g, sm, fm = results[cid]
        n = u5a_compare(u5a, cid, case, sm, g, combined_tb=True)
        q = sm["q"][0]
        check(sm["Tb"] == q * 200000 and sm["sumT"] == q * 20000000, "%s: row 5 split Tb = 2e5 q, sum T = 2e7 q" % cid)
        if c["chain"] == "free":
            check(q == Qp.of(F(1, 25250)) and sm["S_joint"].is_zero(), "%s: q = 1/25250 and S_joint = 0" % cid)
        out.append({"case": cid, "t4_u5a_case": case,
                    "t4_u5a_request_sha256": U5A_PINNED["requests/%s.request.json" % case],
                    "q_m": num(q), "Tb_n": num(sm["Tb"]), "sum_rod_tension_n": num(sm["sumT"]),
                    "S_joint_n": num(sm["S_joint"]),
                    "split": "Tb = 2e5 q and sum T_r = 2e7 q exactly; Tb + sum T_r equals %s's untied g_x" % case,
                    "values_compared_exactly": n})
    return out


def build_oracle(u5a):
    cfgs, cases_bytes, results, entries, disc = build(u5a)
    lits = jr_literal_checks(results, u5a)
    oracle = {
        "schema": "openpipestress.t4_u5b_reference/1",
        "unit": "T4-U5b (explicit elastic tie rods, explicit_axial_rods_v1 with bilateral_linear)",
        "status": ("Refuted on 2026-10-17 by a separate TASK that shares no code with the generator (a 100-digit 3D "
                   "nodal solve with its own B and C, and hand free bodies in reduced coordinates), which confirmed "
                   "every value. Its one MAJOR and four MINOR findings and HELP_HUMAN's two rulings of 2026-10-17 "
                   "are applied in this revision. Confirmation of the repairs and the new cases is pending."),
        "consumer": "T4-U5b, not implemented",
        "claim_fence": "Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).",
        "basis": ("exact arithmetic over Q(pi) (rational functions of pi with rational coefficients, canonical); "
                  "decimals are rounded from the exact forms at 40 significant digits, binary64 values from those; "
                  "mill tolerance = corrosion allowance = 0; shear_model = euler_bernoulli"),
        "independence": ("generators/u5b_reference.py imports, reads and runs no product code or output; it reads "
                         "t4_u5a's oracle.json and nine of its requests as data, bound by SHA-256"),
        "physics_source": ("JR CONTRACT.md sections 2, 4, 5, 6 and 7 (J-C) and its independent refutation sections "
                           "2-4 (archived at 11718e61a2^); DEL-04-07 Design joint-pressure-thrust.md sections 1, 2, 4 "
                           "and 9; HELP_HUMAN's rulings of 2026-10-17; re-derived here"),
        "rulings": {
            "1_non_parallel_rods": {
                "ruling": "HELP_HUMAN, 2026-10-17: non-parallel rods are admitted; S_joint = Tb + sum T_r (n_r . Q_x) - Pe "
                          "in every case; SK-F is an admitted reference case; section 9's former literal form "
                          "Tb + sum T_r - Pe is the discriminator s_joint_section9_literal",
                "basis": "section 9's hardware defines a general rod direction, e = n . (v_bj - v_bi); the literal form "
                         "holds only when n . Q_x = 1 and breaks SK-F's interface identity by 28800/1661 N; the "
                         "projected form is the face equilibrium and reduces to the literal form for parallel rods; "
                         "refusing non-parallel rods would need a parallelism tolerance with no physical basis",
                "applied": "every case's S_joint uses the projected rule; on every parallel case it equals the literal "
                           "form exactly (checked); no value moved (SK-F's S_joint was already the projected 0)",
                "open": "a rod nearly perpendicular to the joint axis (n . Q_x -> 0) is for the U5b implementation "
                        "design to handle, with a named refusal if justified"},
            "2_p_zero_at_the_joint": {
                "ruling": "HELP_HUMAN, 2026-10-17: rods are admitted on an unpressurized connector and on a "
                          "constant-effective-area connector that no region lists; with p = 0 at the joint, Pe = 0 "
                          "and the projected rule applies unchanged",
                "basis": "rods are hardware whose stiffness and reference extension act whatever the pressure; JR "
                         "J-C's mechanical-only tied qualification is exactly this case",
                "applied": "controls MO-F, MO-T, RR-U and RR-P"}},
        "fixture": {
            "E_pa": inp(E_PA), "nu": inp(NU), "outside_diameter_m": inp(OD), "wall_thickness_m": inp(WALL),
            "inner_radius_m": inp(RI), "Ai_m2": num(A_I), "As_m2": num(A_S), "I_m4": num(I_B), "J_m4": num(J_T),
            "G_pa": inp(G_PA), "EA_n": num(EA), "EI_nm2": num(EI), "GJ_nm2": num(GJ), "p_pa": inp(P_PA),
            "Ae_m2": inp(AE), "Pi_n": num(P_I), "h_2nuPi_n": num(H_P), "Pe_n": num(P_E),
            "stub_axial_stiffness_n_per_m": num(KP), "connector_diagonal": [inp(x) for x in K_DIAG],
            "stub_length_m": inp(L_STUB), "joint_length_m": inp(L_JOINT), "rod_transverse_offset_a_m": inp(A_OFF),
            "rod_axial_attachment_offset_m": inp(S_OFF), "rod_reference_extension_m": inp(E_REF),
            "rotation_for_R_cases": [nums(row) for row in qmat(ROT)], "translation_for_R_cases_m": nums(V(*TRANS)),
            "thermal_expansion_per_degC": inp(ALPHA), "MO_T_stub_temperature_change_degC": inp(DT_STUBS),
            "MO_T_stub_thermal_strain": num(Qp.of(ALPHA * DT_STUBS)), "MO_F_force_x_n": inp(F_MO),
            "MO_F_moment_z_nm": inp(M_MO), "remote_pipe_X_length_m": inp(2)},
        "criterion": {
            "rule": "an entry passes when |obs - exp| <= 1e-9 * max(|exp|, F), F the floor of its family; both solver modes",
            "relative": 1e-09,
            "floors": {"force_n": 1.0, "moment_nm": 1.0, "length_m": 1e-05, "rotation_rad": 1e-05},
            "unitless": "direction components and n . Q_x: floor 1"},
        "sign_conventions": {
            "q": "JR's symmetric-midpoint generalized deformation in connector axes (Q columns)",
            "g": "elastic generalized action K (q - q_ref) only; rod forces are never in g",
            "Tb": "g_x",
            "rod_extension": "e = n . (v_bj - v_bi), n = (b_j - b_i)/l, v_b = u + theta x offset; positive = longer",
            "rod_tension": "T_r = k_r (e - e_ref), tension positive; e_ref < 0 is pretension",
            "rod_reference_load": "+C^T k_r e_ref on the right-hand side",
            "rod_nodal_action": "C^T T_r: the action of each node on the rod (node on element), global, moments about the node",
            "connector_endpoint": "B^T g, node on element, global (as t4_u5a's endpoint rows)",
            "pipe_end_actions": "K_e d - f_eq, node on element, global and member-local (x along the member, y = y_reference)",
            "S_joint": "Tb + sum T_r (n_r . Q_x) - Pe in every case (HELP_HUMAN ruling 1, 2026-10-17); equal to "
                       "Tb + sum T_r - Pe for parallel rods; Pe = 0 where the joint has p = 0 (ruling 2)",
            "Pe": "p Ae at a joint a region lists; 0 for an unpressurized connector and for a constant-effective-area "
                  "connector that no region lists",
            "Nw": "pipe wall tension EA (eps - eps_T) + 2 nu Pi (Pi = 0 outside every region; eps_T only in MO-T)",
            "S_pipe": "Nw - Pi of that pipe",
            "interface_identity": "S_pipe = S_joint at both connector faces",
            "reactions": "support on model, R = K u - f, moments about the support node",
            "pressure_rhs": "pressure_rhs_global_n, t4_u5a's convention (its pressure_rhs.global_n; Design section 8 "
                            "binds it to the product's assembled_pressure_rhs_global): representation B's physical caps "
                            "at the region's transferring terminals, -/+ p(Ae - Ai) along Q_x at the end_i/end_j "
                            "attachments of a joint the region lists, and every pressurized pipe's Poisson pair "
                            "(+2 nu Pi at its node i, -2 nu Pi at its node j, along the member); rods, thermal "
                            "eigenloads and applied loads are not pressure terms",
            "pressure_rhs_without_poisson_pairs": "pressure_rhs_without_poisson_pairs_global_n: the same vector without "
                                                  "the Poisson pairs (caps and joint pair only); the first freeze's "
                                                  "pressure_rhs_global_n, kept under this name; no product field"},
        "element_controls": element_controls(),
        "cases": entries,
        "closed_forms_rows_2_and_4": closed_form_block(),
        "convergence": convergence_block(),
        "row5_split": row5_block(u5a, results),
        "closed_form_identity": closed_form_identity_block(),
        "invariance_control_centreline_vs_pair": invariance_block(results),
        "j4_tied_control": j4_block(),
        "t4_u5a_binding": u5a_block(u5a),
        "pressure_rhs_binding": pressure_rhs_binding_block(u5a, results),
        "jr_literal_checks": lits,
        "discriminators": disc,
    }
    return cfgs, cases_bytes, oracle


# ---------------------------------------------------------------------------
# Files


def expected_files(cases_bytes, oracle_bytes):
    files = {"oracle.json": oracle_bytes}
    for cid, b in cases_bytes.items():
        files["cases/%s.case.json" % cid] = b
    return files


def sums_bytes(files):
    allf = dict(files)
    allf["README.md"] = (ROOT / "README.md").read_bytes()
    allf["generators/u5b_reference.py"] = (ROOT / "generators/u5b_reference.py").read_bytes()
    lines = ["%s  %s" % (sha256(allf[p]), p) for p in sorted(allf)]
    return ("\n".join(lines) + "\n").encode()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--write", action="store_true")
    args = ap.parse_args()
    u5a = load_u5a()
    bind_fixture(u5a)
    cfgs, cases_bytes, oracle = build_oracle(u5a)
    oracle["cross_checks"] = {"count": len(CHECKS), "all_passed": True,
                              "note": "every exact assertion the generator ran while building this oracle; "
                                      "any failure stops the generator"}
    oracle_bytes = dumps(oracle)
    files = expected_files(cases_bytes, oracle_bytes)
    if args.write:
        (ROOT / "cases").mkdir(exist_ok=True)
        for p in list((ROOT / "cases").iterdir()):
            if p.name not in {"%s.case.json" % c["id"] for c in cfgs}:
                p.unlink()
        for rel, b in files.items():
            (ROOT / rel).write_bytes(b)
        (ROOT / "SHA256SUMS").write_bytes(sums_bytes(files))
        print("wrote %d files; %d cross-checks passed" % (len(files) + 1, len(CHECKS)))
        return 0
    bad = []
    for rel, b in files.items():
        p = ROOT / rel
        if not p.exists() or p.read_bytes() != b:
            bad.append(rel)
    if (ROOT / "SHA256SUMS").read_bytes() != sums_bytes(files):
        bad.append("SHA256SUMS")
    allowed = set(files) | {"README.md", "SHA256SUMS", "generators/u5b_reference.py"}
    present = {str(p.relative_to(ROOT)) for p in ROOT.rglob("*") if p.is_file()}
    extra = sorted(present - allowed)
    if bad or extra:
        print("MISMATCH: " + ", ".join(bad))
        if extra:
            print("UNEXPECTED FILES: " + ", ".join(extra))
        return 1
    print("ok: %d files identical; %d cross-checks passed" % (len(files) + 1, len(CHECKS)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
