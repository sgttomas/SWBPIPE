#!/usr/bin/env python3
"""S0'' independent derivation of the T0R revision-3 reference changes.

Items: COMB-2 split, I-L axial variants, I-T with Fx=+800 at c, B2 chord-frame
cut face at b (k = 1, 2, 4), TH moment zero scale. Standard library only; no
product code and no references.py code. Conventions as in S0/S0': support on
pipe; section action = outboard (j-side) loads reduced to the cut point.
"""
import math

PI = math.pi
E = 200e9


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def add(*vs):
    return tuple(sum(c) for c in zip(*vs))


def sub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def sc(s, a):
    return tuple(s * x for x in a)


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def norm(a):
    return math.sqrt(dot(a, a))


def unit(a):
    return sc(1.0 / norm(a), a)


def resultant(about, forces=(), couples=()):
    F, M = (0.0, 0.0, 0.0), (0.0, 0.0, 0.0)
    for p, f in forces:
        F = add(F, f)
        M = add(M, cross(sub(p, about), f))
    for c in couples:
        M = add(M, c)
    return F, M


def bend(M, axis):
    ax = unit(axis)
    T = dot(M, ax)
    return T, norm(sub(M, sc(T, ax)))


ro, ri = 0.06, 0.05
I = PI * (ro ** 4 - ri ** 4) / 4
Z, A = I / ro, PI * (ro ** 2 - ri ** 2)
EI, EA = E * I, E * A
out = {}
tip = (1.0, 0.0, 0.0)

# 1. COMB-2 split (1 m C1 cantilever, anchor at origin)
MA1 = sc(-1, resultant((0, 0, 0), [(tip, (0, 1000.0, 0))])[1])
MT = sc(-1, resultant((0, 0, 0), [], [(500.0, 0, 0)])[1])
MA2 = sc(-1, resultant((0, 0, 0), [(tip, (0, 0, 1000.0))])[1])
out["COMB2 A1/T envelope |M|"] = max(norm(MA1), norm(MT))
out["COMB2 A1/T sum control"] = norm(MA1) + norm(MT)
out["COMB2 A1/A2 envelope |M|"] = max(norm(MA1), norm(MA2))
out["COMB2 A1/A2 sum control"] = norm(MA1) + norm(MA2)

# 2. I-L L2 axial variants at a-b end j (b): load (1000,0,1000) at c(1,1,0), SIF 1.15
b, c, e = (1.0, 0, 0), (1.0, 1.0, 0), (2.0, 0, 0)
Fb, Mb = resultant(b, [(c, (1000.0, 0, 1000.0))])
T_ab, Mab = bend(Mb, (1, 0, 0))
Nab = dot(Fb, (1, 0, 0))
out["IL L2 ab correct Pa"] = 1.15 * Mab / Z
out["IL L2 ab axial un-intensified Pa"] = 1.15 * Mab / Z + abs(Nab) / A
out["IL L2 ab axial intensified Pa"] = 1.15 * (Mab / Z + abs(Nab) / A)

# 3. I-T: header a-b (1.3), continuation b-e (none), branch b-c (2.0); loads (800,0,1000) at c, (0,0,500) at e
Lc, Le = (c, (800.0, 0, 1000.0)), (e, (0, 0, 500.0))
_, Mh = resultant(b, [Lc, Le])
_, Mbr = resultant(b, [Lc])
_, Mbe = resultant(b, [Le])
out["IT header ab end j Pa"] = 1.3 * bend(Mh, (1, 0, 0))[1] / Z
out["IT branch bc end i Pa"] = 2.0 * bend(Mbr, (0, 1, 0))[1] / Z
out["IT header torsion Nm"] = bend(Mh, (1, 0, 0))[0]
out["IT b-e bending Nm (unreferenced)"] = bend(Mbe, (1, 0, 0))[1]
out["IT control: header SIF on b-e moment Pa"] = 1.3 * bend(Mbe, (1, 0, 0))[1] / Z
out["IT control: old value without Fx (1.3*500/Z) Pa"] = 1.3 * 500.0 / Z
out["IT control: SIFs swapped (header, branch) Pa"] = (2.0 * bend(Mh, (1, 0, 0))[1] / Z, 1.3 * bend(Mbr, (0, 1, 0))[1] / Z)

# 4. B2 chord frame at b (arc b(1,0,0)->c(1.2,0.2,0), R 0.2, anchor b, roller UX at c, Fy=+1000 at c)
R = 0.2


def X_roller(k):
    a_, b_ = k / EI, 1.0 / EA
    return (2000.0 / PI) * (a_ * R ** 2 - b_) / (a_ * R ** 2 + b_)


cx = unit((0.2, 0.2, 0.0))
yref = (1 / math.sqrt(2), -1 / math.sqrt(2), 0.0)
cz = unit(cross(cx, yref))
cy = cross(cz, cx)
for k in (1.0, 2.0, 4.0):
    X = X_roller(k)
    F, M = resultant(b, [((1.2, 0.2, 0.0), (X, 1000.0, 0.0))])
    cut = tuple(dot(F, v) for v in (cx, cy, cz)) + tuple(dot(M, v) for v in (cx, cy, cz))
    out[f"B2 k={k:g} chord cut face b"] = cut
    out[f"B2 k={k:g} product end_i (negated)"] = sc(-1, cut)

# 7. TH moment zero scale candidates
Nth = EA * 1.2e-5 * 100.0
out["TH |N|"] = Nth
out["TH |N|*ro (candidate moment scale)"] = Nth * ro
out["TH |N|*1 m (length arm)"] = Nth * 1.0

if __name__ == "__main__":
    for kk, vv in out.items():
        if isinstance(vv, tuple):
            print(f"{kk}: (" + ", ".join(repr(float(x)) for x in vv) + ")")
        else:
            print(f"{kk}: {vv!r}")
