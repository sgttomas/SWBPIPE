#!/usr/bin/env python3
"""S0 independent derivation of the T0R hand-statics references.

Written by the S0 checker from the case definitions in DESIGN.md §9.1, the T0
probe source, the T0R design-probe source, STRESS_REFERENCE §2-§7 and the
invented preview fixture. Standard library only. No product code and no code
from references.py is used.

Conventions (product, PP:2108-2170 and append_signed_support_results PP:8712):
  support_reaction_component_v2 = support-on-pipe action, global frame,
  forces N, couples N*m about the attachment node; rigid DOFs from K u - f,
  springs -k u, consuming constant effort +P along the declared DOF's axis.
Section actions (STRESS_REFERENCE §1): action of the positive-x (outboard)
material on the negative-x free body = resultant of the outboard loads,
moments taken about the cut point.
Beam theory: Euler-Bernoulli (frame_kernel README; curved_bend lib.rs:80).
"""
import math
from fractions import Fraction

PI = math.pi
E = 200e9


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def add(a, b):
    return tuple(x + y for x, y in zip(a, b))


def sub(a, b):
    return tuple(x - y for x, y in zip(a, b))


def scale(s, a):
    return tuple(s * x for x in a)


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def norm(a):
    return math.sqrt(sum(x * x for x in a))


def annulus(od, wall):
    ro = od / 2.0
    ri = ro - wall
    A = PI * (ro ** 2 - ri ** 2)
    I = PI * (ro ** 4 - ri ** 4) / 4.0
    return dict(ro=ro, ri=ri, A=A, I=I, Z=I / ro, J=2.0 * I)


def anchor_reaction(origin, forces=(), couples=(), dist=()):
    """Support-on-pipe action of a single anchor at `origin` (the only support).

    forces: [(point, F)], couples: [C], dist: [(p0, p1, w)] uniform w (N/m) on
    the straight segment p0->p1 (resultant |p1-p0|*w at the midpoint; exact).
    """
    F = (0.0, 0.0, 0.0)
    M = (0.0, 0.0, 0.0)
    for p, f in forces:
        F = add(F, f)
        M = add(M, cross(sub(p, origin), f))
    for c in couples:
        M = add(M, c)
    for p0, p1, w in dist:
        L = norm(sub(p1, p0))
        R = scale(L, w)
        mid = scale(0.5, add(p0, p1))
        F = add(F, R)
        M = add(M, cross(sub(mid, origin), R))
    return scale(-1.0, F) + scale(-1.0, M)


def section_action(cut, outboard_forces=(), outboard_couples=()):
    F = (0.0, 0.0, 0.0)
    M = (0.0, 0.0, 0.0)
    for p, f in outboard_forces:
        F = add(F, f)
        M = add(M, cross(sub(p, cut), f))
    for c in outboard_couples:
        M = add(M, c)
    return F, M


def circ_max(N, My, Mz, s):
    return abs(N / s["A"]) + math.hypot(My, Mz) / s["Z"]


def abs_sum(N, My, Mz, s):
    return abs(N / s["A"]) + abs(My) / s["Z"] + abs(Mz) / s["Z"]


def unit_load_flexibility(L, EI, n=2000):
    """Tip translational flexibility of a cantilever by the unit-load integral
    int_0^L m(x)^2/EI dx with m = L-x, composite Simpson (exact for quadratics)."""
    h = L / n
    tot = 0.0
    for k in range(n + 1):
        x = k * h
        wgt = 1 if k in (0, n) else (4 if k % 2 else 2)
        tot += wgt * (L - x) ** 2
    return tot * h / 3.0 / EI


out = {}
secA = annulus(0.12, 0.01)   # T0 probe / L model / B1 section
secS = annulus(0.10, 0.01)   # STRESS_REFERENCE S1 section

# ---------------------------------------------------------------- REF-M14-A
L = 1.0
tip = (L, 0.0, 0.0)
F, M = section_action((0, 0, 0), [(tip, (0.0, 1000.0, 1000.0))])
out["M14-A root maximum [Pa]"] = circ_max(F[0], M[1], M[2], secA)
out["M14-A abs-sum control [Pa]"] = abs_sum(F[0], M[1], M[2], secA)

# ---------------------------------------------------------------- REF-M14-S1
# Assumed L = 1 m (S1's moments are then exactly the stated tip forces).
F, M = section_action((0, 0, 0), [(tip, (1800 * PI, 73.8 * PI, 55.35 * PI))])
out["S1 normal maximum [Pa]"] = circ_max(F[0], M[1], M[2], secS)
out["S1 signed min normal [Pa]"] = F[0] / secS["A"] - math.hypot(M[1], M[2]) / secS["Z"]
out["S1 abs-sum control [Pa]"] = abs_sum(F[0], M[1], M[2], secS)
F, M = section_action((0, 0, 0), [(tip, (1800 * PI, 0.0, 92.25 * PI))])
out["S1 rotated normal maximum [Pa]"] = circ_max(F[0], M[1], M[2], secS)
out["S1 rotated abs-sum [Pa]"] = abs_sum(F[0], M[1], M[2], secS)
F, M = section_action((0, 0, 0), [(tip, (1800 * PI, 73.8 * PI, 55.35 * PI))], [(221.4 * PI, 0, 0)])
out["S1+Mx normal maximum [Pa]"] = circ_max(F[0], M[1], M[2], secS)
out["S1+Mx torsional shear [Pa]"] = abs(M[0]) * secS["ro"] / secS["J"]
F, M = section_action((0, 0, 0), [], [(221.4 * PI, 0, 0)])
out["S1 pure torque normal maximum [Pa]"] = circ_max(F[0], M[1], M[2], secS)
out["S1 pure torque shear [Pa]"] = abs(M[0]) * secS["ro"] / secS["J"]

# ---------------------------------------------------------------- REF-M14-X1
# Pin at i (x=0), roller at j (x=1), w_z = 8e6*Z, couple 1e6*Z about z at j.
Zs = secS["Z"]
w = 8e6 * Zs
C0 = 1e6 * Zs
Lx = 1.0
# Support reactions of the simply supported beam (vertical z): R_i = R_j = w L/2.
# Section at x (outboard x..L): loads = w over (x,L), R_j at L, couple at L.
# For the y-z plane moment about the cut:
def x1_moments(t):
    x = t * Lx
    # outboard: distributed w (+z) over [x, L], roller reaction Rz_j = -w L/2 at L,
    # couple (0,0,C0) at j; couple balance about y needs no y-force; couple about
    # z is reacted by a y-force pair: R_yi = +C0/L, R_yj = -C0/L (sign fixed so that
    # the couple is balanced about the pin: L*Ryj + C0 = 0).
    Rzj = -w * Lx / 2.0
    Ryj = -C0 / Lx
    Fo, Mo = section_action((x, 0, 0), [((Lx, 0, 0), (0.0, Ryj, Rzj))], [(0.0, 0.0, C0)])
    # distributed part, exact resultant w*(L-x) at midpoint of (x, L)
    R = (0.0, 0.0, w * (Lx - x))
    Mo = add(Mo, cross(((Lx - x) / 2.0, 0.0, 0.0), R))
    return Mo


def x1_f(t):
    Mo = x1_moments(t)
    return math.hypot(Mo[1], Mo[2]) / Zs


def x1_abs(t):
    Mo = x1_moments(t)
    return (abs(Mo[1]) + abs(Mo[2])) / Zs


# check the equilibrium assumption: moments vanish at both ends apart from the couple
assert abs(x1_moments(0.0)[1]) < 1e-9 and abs(x1_moments(0.0)[2]) < 1e-9
# analytic: f^2 = (4t(1-t))^2 + t^2 (MPa^2); D' = 2t(17 - 48t + 32t^2)
roots = [(6 - math.sqrt(2)) / 8, (6 + math.sqrt(2)) / 8]
cands = [0.0, 1.0] + roots
t_star = max(cands, key=x1_f)
# golden-section refinement from a dense scan (independent of the analytic root)
best = max((x1_f(k / 200000), k / 200000) for k in range(200001))
a_, b_ = best[1] - 1e-5, best[1] + 1e-5
g = (math.sqrt(5) - 1) / 2
for _ in range(200):
    c_, d_ = b_ - g * (b_ - a_), a_ + g * (b_ - a_)
    if x1_f(c_) > x1_f(d_):
        b_ = d_
    else:
        a_ = c_
out["X1 t* analytic"] = t_star
out["X1 t* golden"] = (a_ + b_) / 2
out["X1 maximum [Pa]"] = x1_f(t_star)
out["X1 closed form sqrt(71+8sqrt2)/8 MPa [Pa]"] = math.sqrt(71 + 8 * math.sqrt(2)) / 8 * 1e6
out["X1 eight-sign candidates control [Pa]"] = max(x1_f(t) for t in (0.0, 1.0, 3 / 8, 5 / 8))
out["X1 abs-sum objective control [Pa]"] = max(x1_abs(k / 100000) for k in range(100001))

# ---------------------------------------------------------------- REF-M33-G
EI_A = E * secA["I"]
F, M = section_action((0, 0, 0), [(tip, (0.0, 3000.0, 0.0))])
sA = out["M14-A root maximum [Pa]"]
sB = circ_max(F[0], M[1], M[2], secA)
out["M33 case B root maximum [Pa]"] = sB
out["M33 headline (max over A,B) [Pa]"] = max(sA, sB)
fl = 1.0 / (3 * EI_A)   # tip flexibility, L = 1
fl_num = unit_load_flexibility(1.0, EI_A)
uA = math.hypot(1000 * fl, 1000 * fl)
uB = 3000 * fl
out["M33 case A tip displacement [m]"] = uA
out["M33 displacement headline [m]"] = max(uA, uB)
out["M33 displacement headline (unit-load integral) [m]"] = 3000 * fl_num
out["M33 first-case control [Pa]"] = sA

# ---------------------------------------------------------------- REF-M05-T
out["M05-T anchor"] = anchor_reaction((0, 0, 0), couples=[(500.0, 0, 0)])

# ---------------------------------------------------------------- REF-M05-R1
R1a = anchor_reaction((0, 0, 0), forces=[((2, 0, 0), (10, -20, 30))], couples=[(4, 5, -6)])
R1b = anchor_reaction((0, 0, 0), forces=[((2, 0, 0), (10, -20, 30))], couples=[(4, 5, -6)],
                      dist=[((0, 0, 0), (2, 0, 0), (0, -3, 0))])
out["R1 point loads"] = R1a
out["R1 with distributed"] = R1b


def rot(Q, v):
    return tuple(sum(Q[i][j] * v[j] for j in range(3)) for i in range(3))


def det3(Q):
    return (Q[0][0] * (Q[1][1] * Q[2][2] - Q[1][2] * Q[2][1])
            - Q[0][1] * (Q[1][0] * Q[2][2] - Q[1][2] * Q[2][0])
            + Q[0][2] * (Q[1][0] * Q[2][1] - Q[1][1] * Q[2][0]))


c30, s30 = math.cos(math.radians(30)), math.sin(math.radians(30))
rotations = {
    "cyclic x->y->z->x": ((0, 0, 1), (1, 0, 0), (0, 1, 0)),
    "Rz(30deg)": ((c30, -s30, 0), (s30, c30, 0), (0, 0, 1)),
    "swap x<->y (improper, det -1)": ((0, 1, 0), (1, 0, 0), (0, 0, 1)),
}
for name, Q in rotations.items():
    tipQ = rot(Q, (2, 0, 0))
    re = anchor_reaction((0, 0, 0), forces=[(tipQ, rot(Q, (10, -20, 30)))],
                         couples=[rot(Q, (4, 5, -6))],   # couples mapped as plain vectors
                         dist=[((0, 0, 0), tipQ, rot(Q, (0, -3, 0)))])
    qr = rot(Q, R1b[:3]) + rot(Q, R1b[3:])
    out[f"R1 rotated {name}: direct"] = re
    out[f"R1 rotated {name}: Q.R"] = qr
    out[f"R1 rotated {name}: det"] = det3(Q)

# ---------------------------------------------------------------- REF-M05-SPRING
# 1 m cantilever (T0 section, E=200 GPa), tip spring k=1e6 N/m on UY, tip Fy=1000.
k = 1e6
kb = 3 * EI_A / 1.0 ** 3
kb_num = 1.0 / fl_num
u = 1000.0 / (kb + k)
spring = -k * u
anchor = anchor_reaction((0, 0, 0), forces=[(tip, (0, 1000.0 + spring, 0))])
out["SPRING spring-on-pipe Fy [N]"] = spring
out["SPRING spring-on-pipe Fy via unit-load flexibility [N]"] = -k * 1000.0 / (kb_num + k)
out["SPRING anchor"] = anchor
out["SPRING global force balance"] = anchor[1] + spring + 1000.0
out["SPRING global moment balance about origin"] = anchor[5] + 1.0 * spring + 1.0 * 1000.0

# ---------------------------------------------------------------- REF-M05-COMB
A1 = anchor_reaction((0, 0, 0), forces=[(tip, (0, 1000.0, 0))])
T = anchor_reaction((0, 0, 0), couples=[(500.0, 0, 0)])
c1 = tuple(2 * a - t for a, t in zip(A1, T))
c2 = tuple(a - t for a, t in zip(A1, T))
out["COMB A1"] = A1
out["COMB T"] = T
out["COMB 2A1-T"] = c1
out["COMB 2A1-T |F|"] = norm(c1[:3])
out["COMB 2A1-T |M|"] = norm(c1[3:])
out["COMB A1-T |M|"] = norm(c2[3:])
out["COMB envelope(A1,T) |F|"] = max(norm(A1[:3]), norm(T[:3]))
out["COMB envelope(A1,T) |M|"] = max(norm(A1[3:]), norm(T[3:]))
out["COMB magnitude-algebra control 2|M_A1|-|M_T|"] = 2 * norm(A1[3:]) - norm(T[3:])
out["COMB magnitude-algebra control |M_A1|-|M_T|"] = norm(A1[3:]) - norm(T[3:])

# ---------------------------------------------------------------- REF-CE
# DEC-049 consumption: one declared translational DOF, positive constant load,
# applied as a constant nodal force along the +axis of that DOF (PP:9506).
out["CE CE-120 support-on-pipe"] = (0.0, 375.0, 0.0, 0.0, 0.0, 0.0)

# ---------------------------------------------------------------- REF-M08-L
a, b, c = (0, 0, 0), (1, 0, 0), (1, 1, 0)
for case, fz in (("up", 1000.0), ("down", -1000.0)):
    P = (0, 0, fz)
    Fa, Ma = section_action(a, [(c, P)])       # a-b at a (local x = global x)
    Fb1, Mb1 = section_action(b, [(c, P)])     # a-b at b and b-c at b
    # a-b local x = +x: T = Mx, bending = (My, Mz); b-c local x = +y: T = My, bending = (Mx, Mz)
    out[f"L {case} a-b at a: T, |Mb|"] = (Ma[0], math.hypot(Ma[1], Ma[2]))
    out[f"L {case} a-b at b: T, |Mb|"] = (Mb1[0], math.hypot(Mb1[1], Mb1[2]))
    out[f"L {case} b-c at b: T, |Mb|"] = (Mb1[1], math.hypot(Mb1[0], Mb1[2]))
    out[f"L {case} a-b max normal [Pa]"] = max(math.hypot(Ma[1], Ma[2]), math.hypot(Mb1[1], Mb1[2])) / secA["Z"]
    out[f"L {case} b-c max normal [Pa]"] = math.hypot(Mb1[0], Mb1[2]) / secA["Z"]
    out[f"L {case} anchor"] = anchor_reaction(a, forces=[(c, P)])
out["L headline [Pa]"] = 1000.0 / secA["Z"]
out["L combination up+down anchor"] = tuple(x + y for x, y in zip(out["L up anchor"], out["L down anchor"]))
out["L main SIF row i*k*sigma (k=1.08) [Pa]"] = 1.15 * 1.08 * out["L headline [Pa]"]
out["L main SIF row i*k*sigma (k=2.16) [Pa]"] = 1.15 * 2.16 * out["L headline [Pa]"]

# ---------------------------------------------------------------- REF-B1
Rb = 0.2
centre = (1.0, 0.2, 0.0)
d_pt = (1.2, 1.2, 0.0)
Pz = (0.0, 0.0, 1000.0)
for label, phi in (("b", 0.0), ("quarter_1", PI / 8), ("midspan", PI / 4), ("quarter_3", 3 * PI / 8), ("c", PI / 2)):
    p = add(centre, (Rb * math.sin(phi), -Rb * math.cos(phi), 0.0))
    t_ = (math.cos(phi), math.sin(phi), 0.0)
    y_ = scale(1.0 / Rb, sub(centre, p))       # toward the centre
    z_ = cross(t_, y_)
    Fs, Ms = section_action(p, [(d_pt, Pz)])
    out[f"B1 {label}: (N,Vy,Vz)"] = (dot(Fs, t_), dot(Fs, y_), dot(Fs, z_))
    out[f"B1 {label}: (T,My,Mz)"] = (dot(Ms, t_), dot(Ms, y_), dot(Ms, z_))
    out[f"B1 {label}: z axis"] = z_
out["B1 midspan T closed form"] = 1000 * (math.sqrt(2) / 2 + 0.2 - 0.2 * math.sqrt(2) / 2 * 1)  # see notes
out["B1 midspan |Mo| closed form"] = 1000 * (math.sqrt(2) / 2 + 0.2 * math.sqrt(2) / 2)
out["B1 anchor"] = anchor_reaction((0, 0, 0), forces=[(d_pt, Pz)])

# ---------------------------------------------------------------- extras (recommended additions)
# B1 endpoint rows in the chord frame (x=(1,1,0)/sqrt2, y=(1,-1,0)/sqrt2, z=x cross y=(0,0,-1)):
cx = (1 / math.sqrt(2), 1 / math.sqrt(2), 0.0)
cy = (1 / math.sqrt(2), -1 / math.sqrt(2), 0.0)
cz = cross(cx, cy)
for label, pnt in (("b", (1.0, 0.0, 0.0)), ("c", (1.2, 0.2, 0.0))):
    Fs, Ms = section_action(pnt, [(d_pt, Pz)])
    out[f"B1 chord-frame cut-face at {label}: (N,Vy,Vz,T,My,Mz)"] = tuple(dot(Fs, e) for e in (cx, cy, cz)) + tuple(dot(Ms, e) for e in (cx, cy, cz))
# True nominal out-of-plane bending maximum on the arc: |Mo| = 1000 (sin phi + 0.2 cos phi)
out["B1 continuous arc max |Mo| [N*m]"] = 1000 * math.sqrt(1 + 0.2 ** 2)
out["B1 continuous arc max phi [deg]"] = math.degrees(math.atan2(1, 0.2))
# X1 support-on-pipe actions: pin i (UX,UY,UZ,RX), roller j (UY,UZ)
out["X1 pin i support-on-pipe"] = (0.0, C0 / Lx, -w * Lx / 2, 0.0, 0.0, 0.0)
out["X1 roller j support-on-pipe"] = (0.0, -C0 / Lx, -w * Lx / 2, 0.0, 0.0, 0.0)
# R1 with the anchor translated to O=(5,-3,2) (whole model translated): moments about the
# attachment node are unchanged; moments about the global origin would differ.
O = (5.0, -3.0, 2.0)
R1t = anchor_reaction(O, forces=[(add(O, (2, 0, 0)), (10, -20, 30))], couples=[(4, 5, -6)],
                      dist=[(O, add(O, (2, 0, 0)), (0, -3, 0))])
out["R1 translated to (5,-3,2): about attachment node"] = R1t
out["R1 translated: moment about global origin (wrong-origin control)"] = add(R1t[3:], cross(O, R1t[:3]))

# ---------------------------------------------------------------- section data
out["secA A,I,Z"] = (secA["A"], secA["I"], secA["Z"])
out["secS A,I,Z,J"] = (secS["A"], secS["I"], secS["Z"], secS["J"])

if __name__ == "__main__":
    for kk, vv in out.items():
        if isinstance(vv, tuple):
            print(f"{kk}: (" + ", ".join(repr(float(x)) for x in vv) + ")")
        else:
            print(f"{kk}: {vv!r}")
