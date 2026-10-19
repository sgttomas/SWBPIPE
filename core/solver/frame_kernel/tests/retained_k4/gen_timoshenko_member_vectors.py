#!/usr/bin/env python3
"""Independent 1200-bit outward reference for the retained Timoshenko member.

O4 (DEL-04-08 Design o4-source-extension.md section 4.4 item 7, section 6): the
exact Timoshenko element of the exact binary64 leaves at the exact frame,
(EI/(L(1+phi)))[[4+phi, 2-phi], [2-phi, 4+phi]] per bending plane with
phi = 12EI/(G*A_s*L^2), its global element stiffness K* = B*^T D* B*, and the
exact recovery of a prescribed displacement vector u: end actions, stations,
reactions K*u (no loads) and the two nodes' support magnitudes. It imports no
producer, retained emulator or test oracle; the interval model keeps every
higher-order term by directed rounding at 1200 bits.
"""
from fractions import Fraction as F
from math import isqrt
from pathlib import Path
import hashlib, struct, sys

R = 1200


def two(e):
    return F(1 << e) if e >= 0 else F(1, 1 << -e)


def exponent(x):
    x = abs(x)
    e = x.numerator.bit_length() - x.denominator.bit_length()
    return e - int(x < two(e))


def rnd(x, mode):
    x = F(x)
    if not x:
        return F(0)
    h = two(exponent(x) - R + 1)
    q, r = divmod(x / h, 1)
    q = int(q)
    if mode == 1:
        q += int(bool(r))
    return q * h


def sqrtr(x, mode):
    assert x >= 0
    if not x:
        return F(0)
    h = two(exponent(x) // 2 - R + 1)
    z = x / (h * h)
    q = isqrt(z.numerator // z.denominator)
    if mode == 1:
        q += int(F(q * q) != z)
    return q * h


class I:
    def __init__(self, a, b=None):
        self.a = F(a)
        self.b = F(a if b is None else b)

    def __add__(self, o):
        o = iv(o)
        return I(rnd(self.a + o.a, -1), rnd(self.b + o.b, 1))

    __radd__ = __add__

    def __neg__(self):
        return I(-self.b, -self.a)

    def __sub__(self, o):
        return self + -iv(o)

    def __rsub__(self, o):
        return iv(o) + -self

    def __mul__(self, o):
        o = iv(o)
        a = [x * y for x in (self.a, self.b) for y in (o.a, o.b)]
        return I(rnd(min(a), -1), rnd(max(a), 1))

    __rmul__ = __mul__

    def __truediv__(self, o):
        o = iv(o)
        assert not o.a <= 0 <= o.b
        a = [x / y for x in (self.a, self.b) for y in (o.a, o.b)]
        return I(rnd(min(a), -1), rnd(max(a), 1))

    def sqrt(self):
        assert self.a >= 0
        return I(sqrtr(self.a, -1), sqrtr(self.b, 1))


def iv(x):
    return x if isinstance(x, I) else I(x)


def dot(a, b):
    return sum((x * y for x, y in zip(a, b)), I(0))


def frame(xi, xj, yr):
    def normalize(v):
        n = dot(v, v).sqrt()
        return [a / n for a in v], n

    x, length = normalize([I(b - a) for a, b in zip(xi, xj)])
    p = dot([I(c) for c in yr], x)
    y, _ = normalize([I(a) - p * b for a, b in zip(yr, x)])
    z, _ = normalize([x[j] * y[k] - x[k] * y[j] for j, k in ((1, 2), (2, 0), (0, 1))])
    return x, y, z, length


def element(case):
    """B (global, 6x12), D (6x6) and K* (12x12) of the exact Timoshenko element."""
    x, y, z, length = frame(case["xi"], case["xj"], case["yr"])
    inv = I(1) / length
    b = [[I(0)] * 12 for _ in range(6)]
    for k in range(3):
        iy, iz = inv * y[k], inv * z[k]
        b[0][k], b[0][6 + k] = -x[k], x[k]
        b[1][3 + k], b[1][9 + k] = -x[k], x[k]
        for row, rot in ((2, 3), (3, 9)):
            b[row][k], b[row][6 + k], b[row][rot + k] = iy, -iy, z[k]
        for row, rot in ((4, 3), (5, 9)):
            b[row][k], b[row][6 + k], b[row][rot + k] = -iz, iz, y[k]
    e, g, a, j, a_s = (F(case[n]) for n in ("E", "G", "A", "J", "As"))
    d = [[I(0)] * 6 for _ in range(6)]
    d[0][0] = I(e * a) / length
    d[1][1] = I(g * j) / length
    ks = g * a_s
    for rows, second in (((2, 3), F(case["Iz"])), ((4, 5), F(case["Iy"]))):
        ei = e * second
        c = I(ei) / length
        phi = I(12 * ei) / (I(ks) * length * length)
        cs = 3 * c / (1 + phi)
        d1, d2 = c + cs, cs - c
        r0, r1 = rows
        d[r0][r0], d[r0][r1], d[r1][r0], d[r1][r1] = d1, d2, d2, d1
    db = [[dot([d[r][s] for s in range(6)], [b[s][c] for s in range(6)]) for c in range(12)] for r in range(6)]
    k = [[dot([b[r][aa] for r in range(6)], [db[r][bb] for r in range(6)]) for bb in range(12)] for aa in range(12)]
    return b, d, k, (x, y, z), inv


def recovery(case):
    b, d, k, axes, inv = element(case)
    u = [F(v) for v in case["U"]]
    loc = [dot(axes[r % 3], [I(v) for v in u[3 * (r // 3):3 * (r // 3) + 3]]) for r in range(12)]
    e = [loc[6] - loc[0], loc[9] - loc[3],
         loc[5] + inv * loc[1] - inv * loc[7], loc[11] + inv * loc[1] - inv * loc[7],
         loc[4] - inv * loc[2] + inv * loc[8], loc[10] - inv * loc[2] + inv * loc[8]]
    q = [dot(d[r], e) for r in range(6)]
    vy = inv * q[2] + inv * q[3]
    vz = -(inv * q[4] + inv * q[5])
    ends = [-q[0], vy, vz, -q[1], q[4], q[2], q[0], -vy, -vz, q[1], q[5], q[3]]
    out = list(ends)
    for t in TS:
        out.extend([ends[6], ends[7], ends[8], ends[9],
                    t * q[5] + t * q[4] - q[4], t * q[3] + t * q[2] - q[2]])
    reactions = [dot(k[c], [I(v) for v in u]) for c in range(12)]
    out.extend(reactions)
    for node in (0, 1):
        for first in (0, 3):
            comps = reactions[6 * node + first:6 * node + first + 3]
            out.append(dot(comps, comps).sqrt())
    return k, out


TS = [F(0), F(1, 4), F(float(1 / 3)), F(1) - two(-52), F(1)]
U_DEFAULT = [F((i % 5) - 2, 16) for i in range(12)]
# e2 = e4 = 0 on the aligned frame, so Q2 and Q4 carry only the d2 terms.
U_PHI2 = [F(1, 16), F(0), F(0), F(1, 32), F(-1, 8), F(1, 16),
          F(-1, 16), F(1, 16), F(1, 8), F(-1, 32), F(1, 16), F(-1, 8)]


def f64(x):
    return F(float(x))


def cases():
    unit = dict(E=F(1), G=F(1), A=F(1), J=F(1))
    aligned = dict(xi=[F(0)] * 3, xj=[F(1), F(0), F(0)], yr=[F(0), F(1), F(0)])
    out = []
    out.append(("phi2", "128,256,512,1024", dict(aligned, **unit, As=F(3), Iz=F(1, 2), Iy=F(1, 2) + two(-41), U=U_PHI2)))
    out.append(("phi2_minus", "128,256,512,1024", dict(aligned, **unit, As=F(3), Iz=F(1, 2) - two(-42), Iy=F(1, 2) - two(-41), U=U_PHI2)))
    out.append(("phi2_plus", "128,256,512,1024", dict(aligned, **unit, As=F(3), Iz=F(1, 2) + two(-42), Iy=F(1, 2) + two(-43), U=U_PHI2)))
    out.append(("phi_small", "128,256,512,1024", dict(aligned, **unit, As=F(3), Iz=f64(2.5e-7), Iy=f64(3e-7), U=U_DEFAULT)))
    out.append(("phi_large", "128,256,512,1024", dict(aligned, **unit, As=F(3), Iz=F(3), Iy=F(7), U=U_DEFAULT)))
    out.append(("skew", "128,256,512,1024", dict(xi=[F(0)] * 3, xj=[F(1), F(2), F(3)], yr=[F(0), F(1), F(0)],
                E=F(2), G=F(5), A=F(3), J=F(2), As=F(1), Iz=f64(35 / 12), Iy=F(2), U=U_DEFAULT)))
    out.append(("skew345", "128,256,512,1024", dict(xi=[F(0)] * 3, xj=[F(3), F(4), F(0)], yr=[F(0), F(0), F(1)],
                **unit, As=F(2), Iz=F(4), Iy=F(1), U=U_DEFAULT)))
    out.append(("skew122_phi2", "128,256,512,1024", dict(xi=[F(0)] * 3, xj=[F(1), F(2), F(2)], yr=[F(1), F(0), F(0)],
                **unit, As=F(1), Iz=F(3, 2), Iy=F(3, 4), U=U_DEFAULT)))
    for p in (128, 256, 512, 1024):
        for kind in ("g_boundary", "g_above"):
            k = p - 17 + (kind == "g_above")
            out.append(("%s_%d" % (kind, p), str(p), dict(xi=[F(0)] * 3, xj=[F(1), F(1), two(-k)], yr=[F(1), F(1), F(0)],
                        **unit, As=F(3), Iz=F(1), Iy=F(1, 4), U=U_DEFAULT)))
    return out


FIELDS = ("xi", "xj", "yr", "E", "G", "A", "Iy", "Iz", "J", "As", "U")


def bits(x):
    return "%016x" % struct.unpack("<Q", struct.pack("<d", float(x)))[0]


def dyad(x):
    if not x:
        return "+0@0"
    n, d = x.numerator, x.denominator
    assert d & (d - 1) == 0
    e = -(d.bit_length() - 1)
    while n % 2 == 0:
        n //= 2
        e += 1
    return ("-" if n < 0 else "+") + format(abs(n), "x") + "@" + str(e)


def build():
    lines = []
    for name, ps, case in cases():
        values = []
        for f in FIELDS:
            v = case[f]
            values.extend(v if isinstance(v, list) else [v])
        for v in values:
            assert F(float(v)) == v, (name, v)
        lines.append("case %s %s %s" % (name, ps, " ".join(bits(v) for v in values)))
        k, rows = recovery(case)
        for a in range(12):
            for b in range(a, 12):
                lines.append("ke %d %d %s %s" % (a, b, dyad(k[a][b].a), dyad(k[a][b].b)))
        for i, row in enumerate(rows):
            lines.append("row %d %s %s" % (i, dyad(row.a), dyad(row.b)))
        lines.append("end")
    return "\n".join(lines) + "\n"


HERE = Path(__file__).resolve().parent

if __name__ == "__main__":
    value = build()
    path = HERE / "timoshenko_member.txt"
    sums = HERE / "SHA256SUMS"
    entries = {line.split()[1]: line.split()[0] for line in sums.read_text().splitlines()}
    digest = hashlib.sha256(value.encode()).hexdigest()
    if "--check" in sys.argv:
        if not path.exists() or path.read_text() != value:
            raise SystemExit("DIFF timoshenko_member.txt")
        if entries.get(path.name) != digest:
            raise SystemExit("DIFF timoshenko_member.txt checksum")
        print("OK timoshenko_member.txt")
    else:
        path.write_text(value)
        entries[path.name] = digest
        sums.write_text("".join(f"{entries[name]}  {name}\n" for name in sorted(entries)))
