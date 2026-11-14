#!/usr/bin/env python3
"""Exact element references for a Timoshenko straight member's consistent mass (DEL-04-09 REQ-001, AC-012).

What is referenced. The consistent mass of a two-node straight frame member whose bending stiffness is the
Timoshenko element with shear area A_s: in each bending plane, the mass of the interpolation that stiffness
implies, M = mbar * int(Nw^T Nw) + ibar * int(Nt^T Nt), with linear axial (mbar) and torsional (jbar) mass,
assembled to 12x12 in member axes and turned to global axes. The interpolation is the exact homogeneous
static solution of the Timoshenko beam (the interdependent-interpolation family of Friedman and Kosmatka,
Computers & Structures 47(3), 1993; derived here, not copied).

Derivation, per plane, with phi = 12 E I / (G A_s L^2) and th the section rotation:
  EI th'' + G A_s (w' - th) = 0 and (G A_s (w' - th))' = 0 give th = c1 + c2 x + c3 x^2 and
  w' = th - c3 phi L^2 / 6, so w = c0 + c1 x + c2 x^2/2 + c3 x^3/3 - c3 phi L^2 x / 6.
  The four coefficients are solved from the nodal values (w_i, th_i, w_j, th_j), and the products of the
  resulting shape polynomials are integrated exactly over [0, L]. No closed-form mass entry is used.

Conventions (the frame kernel's, checked against core/solver/frame_kernel/src/lib.rs `timoshenko_phi` and
core/solver/modal_eigen/src/reformation/mass.rs module docs at main bbf13d190):
  - local DOFs per node: u_x, u_y, u_z, r_x, r_y, r_z; node i's six then node j's;
  - the x-y plane acts on [u_y, r_z] with I_z (phi_z); the x-z plane acts on [u_z, -r_y] with I_y (phi_y),
    so its entry (a, b) carries the sign s_a s_b with s = (1, -1, 1, -1);
  - member axes: x the chord's unit vector, y the unit part of the y reference normal to x, z = x cross y;
    the 12x12 is turned to global axes as T^T M T, T = diag(R, R, R, R), R's rows the member axes;
  - one rotary inertia per length ibar serves both planes, as the product forms it today.
Each case's coordinates and y reference are chosen so that the chord length and member axes are rational,
so every value here is exact.

Self-checks, each exact (the build fails otherwise):
  1. the shapes reproduce the kernel's Timoshenko stiffness plane EI/(L^3(1+phi)) [12, 6L, (4+phi)L^2,
     (2-phi)L^2; ...] in each plane;
  2. at phi = 0 the plane mass is the Hermitian mbar L/420 [156, 22L, 54, -13L; ...] plus the Rayleigh
     ibar/(30 L) [36, 3L, -36, 3L; ...];
  3. for each global unit translation e_k, the global M times the rigid translation equals mbar times the
     consistent uniform-load vector [L/2, L^2/12, L/2, -L^2/12] in each plane, turned to global axes, and the
     total is mbar L;
  4. the global matrix is symmetric and positive definite (every pivot of an exact symmetric elimination positive).

Independence. Python 3 standard library only (fractions, decimal, json, hashlib, argparse, pathlib). It
reads, imports and runs no product code, product output, fixture or schema.

Outputs: references.json (operands as binary64 bit patterns and decimals; phi_y and phi_z; both plane
matrices; the global 12x12 upper triangle; each value as a 40-digit decimal and as a double-double pair
`hi lo`, hi the value rounded once to binary64 and lo the remainder rounded once) and SHA256SUMS over the
README, this generator and references.json.

Usage:  python3 -I -B modal_timoshenko_mass_reference.py [--check]  # default: rebuild in memory, compare bytes
        python3 -I -B modal_timoshenko_mass_reference.py --write    # rewrite references.json and SHA256SUMS
"""

import argparse
import hashlib
import json
import struct
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as F
from pathlib import Path

getcontext().prec = 60


class CheckFailure(Exception):
    pass


def check(condition, what):
    if not condition:
        raise CheckFailure(what)


# ---- exact helpers ---------------------------------------------------------------------------------------

def exact(value):
    """The exact rational value of a binary64."""
    check(isinstance(value, float) and value == value and abs(value) != float("inf"), f"not finite: {value!r}")
    return F(value)


def bits(value):
    return struct.pack(">d", value).hex()


def rational_sqrt(q):
    """The square root of a nonnegative rational that is a perfect square, else a CheckFailure."""
    check(q >= 0, "negative square")
    from math import isqrt
    n, d = q.numerator, q.denominator
    rn, rd = isqrt(n), isqrt(d)
    check(rn * rn == n and rd * rd == d, f"not a rational square: {q}")
    return F(rn, rd)


def dec40(q):
    return format(Decimal(q.numerator) / Decimal(q.denominator), ".40e")


def dd_pair(q):
    hi = float(q)  # Fraction -> float rounds once to nearest
    lo = float(q - F(hi))
    return [bits(hi), bits(lo)]


# polynomials in x, coefficient lists
def padd(a, b):
    n = max(len(a), len(b))
    return [(a[i] if i < len(a) else F(0)) + (b[i] if i < len(b) else F(0)) for i in range(n)]


def pscale(a, k):
    return [k * c for c in a]


def pmul(a, b):
    r = [F(0)] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            r[i + j] += x * y
    return r


def pder(a):
    return [i * a[i] for i in range(1, len(a))] or [F(0)]


def pint(a, length):
    return sum(c * length ** (i + 1) / (i + 1) for i, c in enumerate(a))


def peval(a, x):
    return sum(c * x ** i for i, c in enumerate(a))


def solve(a, b):
    n = len(a)
    m = [row[:] + [b[i]] for i, row in enumerate(a)]
    for c in range(n):
        p = next(r for r in range(c, n) if m[r][c] != 0)
        m[c], m[p] = m[p], m[c]
        for r in range(n):
            if r != c and m[r][c] != 0:
                f = m[r][c] / m[c][c]
                m[r] = [m[r][k] - f * m[c][k] for k in range(n + 1)]
    return [m[i][n] / m[i][i] for i in range(n)]


def ldl_pivots(m):
    """The pivots of an exact symmetric elimination without row exchange; all positive iff m is positive definite."""
    a = [row[:] for row in m]
    n = len(a)
    pivots = []
    for c in range(n):
        p = a[c][c]
        pivots.append(p)
        if p <= 0:
            return pivots
        for r in range(c + 1, n):
            f = a[r][c] / p
            if f:
                a[r] = [a[r][k] - f * a[c][k] for k in range(n)]
    return pivots


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


# ---- the element ----------------------------------------------------------------------------------------

def shapes(length, phi):
    """Nw[k], Nt[k] for unit nodal value k of (w_i, th_i, w_j, th_j)."""
    a = -phi * length * length / 6
    w_basis = [[F(1)], [F(0), F(1)], [F(0), F(0), F(1, 2)], [F(0), a, F(0), F(1, 3)]]
    t_basis = [[F(0)], [F(1)], [F(0), F(1)], [F(0), F(0), F(1)]]
    rows = [[peval(w_basis[j], F(0)) for j in range(4)], [peval(t_basis[j], F(0)) for j in range(4)],
            [peval(w_basis[j], length) for j in range(4)], [peval(t_basis[j], length) for j in range(4)]]
    nw, nt = [], []
    for k in range(4):
        c = solve(rows, [F(int(i == k)) for i in range(4)])
        w, t = [F(0)], [F(0)]
        for j in range(4):
            w = padd(w, pscale(w_basis[j], c[j]))
            t = padd(t, pscale(t_basis[j], c[j]))
        nw.append(w)
        nt.append(t)
    return nw, nt


def plane(length, phi, mbar, ibar, ei, check_stiffness=True):
    """The plane mass on (w_i, th_i, w_j, th_j), with its exact self-checks 1 and 2."""
    nw, nt = shapes(length, phi)
    m = [[mbar * pint(pmul(nw[a], nw[b]), length) + ibar * pint(pmul(nt[a], nt[b]), length)
          for b in range(4)] for a in range(4)]
    if check_stiffness:
        f = ei / (length ** 3 * (1 + phi))
        lk = length
        kk = [[12, 6 * lk, -12, 6 * lk], [6 * lk, (4 + phi) * lk * lk, -6 * lk, (2 - phi) * lk * lk],
              [-12, -6 * lk, 12, -6 * lk], [6 * lk, (2 - phi) * lk * lk, -6 * lk, (4 + phi) * lk * lk]]
        for a in range(4):
            for b in range(4):
                v = ei * pint(pmul(pder(nt[a]), pder(nt[b])), length)
                if phi != 0:
                    gas = 12 * ei / (phi * length * length)
                    ga = padd(pder(nw[a]), pscale(nt[a], -1))
                    gb = padd(pder(nw[b]), pscale(nt[b], -1))
                    v += gas * pint(pmul(ga, gb), length)
                check(v == f * kk[a][b], f"self-check 1: stiffness entry ({a},{b}) at phi {phi}")
    return m


def phi_zero_self_check():
    """Self-check 2, on symbolic-free exact samples of length."""
    for length in (F(1), F(3, 2), F(7, 3)):
        lk = length
        h = [[156, 22 * lk, 54, -13 * lk], [22 * lk, 4 * lk * lk, 13 * lk, -3 * lk * lk],
             [54, 13 * lk, 156, -22 * lk], [-13 * lk, -3 * lk * lk, -22 * lk, 4 * lk * lk]]
        r = [[36, 3 * lk, -36, 3 * lk], [3 * lk, 4 * lk * lk, -3 * lk, -lk * lk],
             [-36, -3 * lk, 36, -3 * lk], [3 * lk, -lk * lk, -3 * lk, 4 * lk * lk]]
        mt = plane(length, F(0), F(1), F(0), F(1))
        mr = plane(length, F(0), F(0), F(1), F(1))
        for a in range(4):
            for b in range(4):
                check(mt[a][b] == length / 420 * h[a][b], f"self-check 2: Hermitian ({a},{b}) at L {length}")
                check(mr[a][b] == r[a][b] / (30 * length), f"self-check 2: Rayleigh ({a},{b}) at L {length}")


def element(case):
    xi = [exact(v) for v in case["xi"]]
    xj = [exact(v) for v in case["xj"]]
    yr = [exact(v) for v in case["y_reference"]]
    e, g, a_s = (exact(case[k]) for k in ("E", "G", "A_s"))
    iy, iz = exact(case["I_y"]), exact(case["I_z"])
    mbar, ibar, jbar = (exact(case[k]) for k in ("mbar", "ibar", "jbar"))
    d = [b - a for a, b in zip(xi, xj)]
    length = rational_sqrt(dot(d, d))
    check(length > 0, "zero length")
    ex = [v / length for v in d]
    yc = [y - dot(yr, ex) * u for y, u in zip(yr, ex)]
    ym = rational_sqrt(dot(yc, yc))
    check(ym > 0, "y reference parallel to the chord")
    ey = [v / ym for v in yc]
    ez = cross(ex, ey)
    phi_y = 12 * e * iy / (g * a_s * length * length)
    phi_z = 12 * e * iz / (g * a_s * length * length)
    pxy = plane(length, phi_z, mbar, ibar, e * iz)
    pxz = plane(length, phi_y, mbar, ibar, e * iy)
    local = [[F(0)] * 12 for _ in range(12)]
    for (p, q, x) in ((0, 6, mbar), (3, 9, jbar)):
        local[p][p] = local[q][q] = x * length / 3
        local[p][q] = local[q][p] = x * length / 6
    xy, xz, s = (1, 5, 7, 11), (2, 4, 8, 10), (1, -1, 1, -1)
    for r in range(4):
        for c in range(4):
            local[xy[r]][xy[c]] = pxy[r][c]
            local[xz[r]][xz[c]] = s[r] * s[c] * pxz[r][c]
    axes = [ex, ey, ez]
    glob = [[F(0)] * 12 for _ in range(12)]
    for bi in range(4):
        for bj in range(4):
            for r in range(3):
                for c in range(3):
                    glob[3 * bi + r][3 * bj + c] = sum(axes[p][r] * local[3 * bi + p][3 * bj + q] * axes[q][c]
                                                       for p in range(3) for q in range(3))
    # Self-check 3: rigid translations
    total = F(0)
    for k in range(3):
        rvec = [F(0)] * 12
        rvec[k] = rvec[6 + k] = F(1)
        mr = [sum(glob[a][b] * rvec[b] for b in range(12)) for a in range(12)]
        # expected: mbar times the consistent load, local then global
        tk = [axes[p][k] for p in range(3)]  # the unit translation in member axes
        loc = [F(0)] * 12
        loc[0] = loc[6] = mbar * length / 2 * tk[0]
        for (wi, ti, wj, tj), comp, sgn in (((1, 5, 7, 11), 1, 1), ((2, 4, 8, 10), 2, -1)):
            loc[wi] += mbar * length / 2 * tk[comp]
            loc[wj] += mbar * length / 2 * tk[comp]
            loc[ti] += sgn * mbar * length * length / 12 * tk[comp]
            loc[tj] -= sgn * mbar * length * length / 12 * tk[comp]
        expect = [sum(axes[p][r] * loc[3 * b + p] for p in range(3)) for b in range(4) for r in range(3)]
        check(mr == expect, f"self-check 3: rigid translation {k}")
        total += sum(rvec[a] * mr[a] for a in range(12))
    check(total == 3 * mbar * length, "self-check 3: total mass")
    # Self-check 4: symmetry and positive definiteness
    check(all(glob[a][b] == glob[b][a] for a in range(12) for b in range(12)), "self-check 4: symmetry")
    check(all(p > 0 for p in ldl_pivots(glob)), "self-check 4: positive definite")
    return {"length": length, "phi_y": phi_y, "phi_z": phi_z, "plane_xy": pxy, "plane_xz": pxz, "global": glob}


# ---- cases ----------------------------------------------------------------------------------------------
# Section values are binary64 inputs; each case's geometry keeps the chord and axes rational.
# Annulus D = 0.3239 m, t = 0.0103 m (illustrative), steel E = 2.03e11, G = 7.8e10; A_s = kappa A with an
# illustrative kappa of 0.53. Stubby cases raise phi to discriminate the shear terms.
CASES = [
    {"id": "T-AXIS-SLENDER", "what": "chord along +x, annulus, slender (phi about 0.08)",
     "xi": [0.0, 0.0, 0.0], "xj": [3.0, 0.0, 0.0], "y_reference": [0.0, 1.0, 0.0],
     "E": 2.03e11, "G": 7.8e10, "A_s": 5.540e-3, "I_y": 1.2847e-4, "I_z": 1.2847e-4,
     "mbar": 81.5, "ibar": 1.035, "jbar": 2.07},
    {"id": "T-SKEW-STUBBY", "what": "chord along (3,4,12)/13, y reference not normal to it, annulus, stubby (phi about 1)",
     "xi": [1.0, 2.0, 3.0], "xj": [1.1875, 2.25, 3.75], "y_reference": [67.0, -19.0, 60.0],
     "E": 2.03e11, "G": 7.8e10, "A_s": 5.540e-3, "I_y": 1.2847e-4, "I_z": 1.2847e-4,
     "mbar": 81.5, "ibar": 1.035, "jbar": 2.07},
    {"id": "T-SKEW-UNEQUAL", "what": "chord along (2,3,6)/7, I_y four times I_z (plane mapping), phi_y about 4 and phi_z about 1",
     "xi": [0.0, 0.0, 0.0], "xj": [0.25, 0.375, 0.75], "y_reference": [6.0, 2.0, -3.0],
     "E": 2.0e11, "G": 8.0e10, "A_s": 4.0e-4, "I_y": 4.0e-5, "I_z": 1.0e-5,
     "mbar": 50.0, "ibar": 0.25, "jbar": 0.5},
    {"id": "T-AXIS-NEAR-EB", "what": "chord along +z, very large A_s (phi about 1e-12): continuity with the Euler-Bernoulli element",
     "xi": [0.0, 0.0, 0.0], "xj": [0.0, 0.0, 2.0], "y_reference": [1.0, 0.0, 0.0],
     "E": 2.03e11, "G": 7.8e10, "A_s": 5.540e9, "I_y": 1.2847e-4, "I_z": 1.2847e-4,
     "mbar": 81.5, "ibar": 1.035, "jbar": 2.07},
]


def render(results):
    out = {"package": "modal_timoshenko_mass", "deliverable": "DEL-04-09 (REQ-001, AC-012)",
           "format": "each value: [40-digit decimal, [hi bits, lo bits]]; matrices on (w_i, th_i, w_j, th_j) for "
                     "planes and the 12 global DOFs (node i u_x..r_z, node j) for `global_upper`",
           "cases": []}
    for case, res in zip(CASES, results):
        val = lambda q: [dec40(q), dd_pair(q)]
        out["cases"].append({
            "id": case["id"], "what": case["what"],
            "operands": {k: {"bits": bits(float(v)) if not isinstance(v, list) else [bits(x) for x in v],
                             "value": v} for k, v in case.items() if k not in ("id", "what")},
            "length": val(res["length"]), "phi_y": val(res["phi_y"]), "phi_z": val(res["phi_z"]),
            "plane_xy": [[val(x) for x in row] for row in res["plane_xy"]],
            "plane_xz": [[val(x) for x in row] for row in res["plane_xz"]],
            "global_upper": [[r, c, *val(res["global"][r][c])] for r in range(12) for c in range(r, 12)
                             if res["global"][r][c] != 0],
        })
    return out


def build():
    phi_zero_self_check()
    return [element(c) for c in CASES]


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--write", action="store_true", help="rewrite references.json and SHA256SUMS")
    mode.add_argument("--check", action="store_true", help="rebuild in memory and compare bytes (the default)")
    args = parser.parse_args()
    try:
        results = build()
    except CheckFailure as exc:
        print(f"check failed: {exc}", file=sys.stderr)
        return 1
    root = Path(__file__).resolve().parents[1]
    outputs = {"references.json": (json.dumps(render(results), indent=1) + "\n").encode("utf-8")}
    files = {"README.md": (root / "README.md").read_bytes(),
             "generators/modal_timoshenko_mass_reference.py": Path(__file__).read_bytes(), **outputs}
    outputs["SHA256SUMS"] = "".join(f"{hashlib.sha256(d).hexdigest()}  {n}\n"
                                    for n, d in sorted(files.items())).encode("ascii")
    if args.write:
        for name, data in outputs.items():
            (root / name).parent.mkdir(parents=True, exist_ok=True)
            (root / name).write_bytes(data)
    else:
        diff = [n for n, d in outputs.items() if not (root / n).is_file() or (root / n).read_bytes() != d]
        if diff:
            print("reference bytes differ: " + ", ".join(diff), file=sys.stderr)
            return 1
    print(f"{len(results)} cases; self-checks 1-4 exact; frozen bytes {'written' if args.write else 'match'}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
