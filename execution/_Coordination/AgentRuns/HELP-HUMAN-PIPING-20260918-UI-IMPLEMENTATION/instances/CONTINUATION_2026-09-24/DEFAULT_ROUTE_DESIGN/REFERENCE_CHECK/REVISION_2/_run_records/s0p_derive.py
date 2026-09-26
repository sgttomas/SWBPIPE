#!/usr/bin/env python3
"""S0' independent derivation of the T0R revision-2 references.

Written by the S0' checker from the case definitions (DESIGN.md rev 2 §9.1 and
the definition strings under revision_2 / case_definitions), the product's
documented laws (support-on-pipe, K u - f, -k u, DEC-049 +axis constant force,
gap law nonlinear_supports lib.rs:604-633, arc construction PP:4928-4935,
curved_bend end_flexibility lib.rs:199-238) and STRESS_REFERENCE. Standard
library only; no product code and no references.py code.
Section actions: action of the outboard (j-side) material on the cut,
moments about the cut point. Euler-Bernoulli throughout.
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


def annulus(od, wall):
    ro = od / 2.0
    ri = ro - wall
    I = PI * (ro ** 4 - ri ** 4) / 4.0
    return dict(ro=ro, A=PI * (ro ** 2 - ri ** 2), I=I, Z=I / ro, J=2 * I)


def resultant(about, forces=(), couples=()):
    F, M = (0.0, 0.0, 0.0), (0.0, 0.0, 0.0)
    for p, f in forces:
        F = add(F, f)
        M = add(M, cross(sub(p, about), f))
    for c in couples:
        M = add(M, c)
    return F, M


def on_pipe(about, forces=(), couples=()):
    """Support-on-pipe action of the sole support at `about` carrying all loads."""
    F, M = resultant(about, forces, couples)
    return sc(-1, F) + sc(-1, M)


C1 = annulus(0.12, 0.01)
EI, EA = E * C1["I"], E * C1["A"]
kb = 3 * EI            # tip stiffness of the 1 m cantilever (rotation free)
out = {}
O0 = (0.0, 0.0, 0.0)
tip = (1.0, 0.0, 0.0)

# ---- R1 translated: anchor at O=(5,-3,2), geometry and loads carried with it
O = (5.0, -3.0, 2.0)
F, M = resultant(O, [(add(O, (2, 0, 0)), (10, -20, 30)), (add(O, (1, 0, 0)), (0, -6, 0))], [(4, 5, -6)])
R = sc(-1, F) + sc(-1, M)
out["R1T anchor F"] = R[:3]
out["R1T anchor M about node"] = R[3:]
out["R1T control M about global origin"] = add(R[3:], cross(O, R[:3]))

# ---- COMB-2: A1 tip Fy=+1000, A2 tip Fz=+1000
A1 = on_pipe(O0, [(tip, (0, 1000.0, 0))])
A2 = on_pipe(O0, [(tip, (0, 0, 1000.0))])
d = 1000.0 / kb
uA1, uA2 = (0.0, d, 0.0), (0.0, 0.0, d)
out["COMB2 envelope max |M|"] = max(norm(A1[3:]), norm(A2[3:]))
out["COMB2 control sum |M|"] = norm(A1[3:]) + norm(A2[3:])
out["COMB2 A1+A2 anchor"] = add(A1, A2)
out["COMB2 A1+A2 |u| mm"] = norm(add(uA1, uA2)) * 1e3
out["COMB2 A1-A2 |u| mm"] = norm(sub(uA1, uA2)) * 1e3
out["COMB2 control magnitude sum mm"] = (norm(uA1) + norm(uA2)) * 1e3
out["COMB2 control magnitude difference mm"] = (norm(uA1) - norm(uA2)) * 1e3

# ---- X1 supports (S1 section, L=1, pin i UX UY UZ RX, roller j UY UZ, w_z=8e6 Z, couple +z at j)
S1 = annulus(0.10, 0.01)
w, cz = 8e6 * S1["Z"], 1e6 * S1["Z"]
# equilibrium: sum Fz: Rzi+Rzj+w=0, symmetric -> -w/2 each; moment z about i: 1*Ryj + cz = 0
out["X1 pin i on pipe"] = (0.0, cz, -w / 2, 0.0, 0.0, 0.0)
out["X1 roller j on pipe"] = (0.0, -cz, -w / 2, 0.0, 0.0, 0.0)

# ---- M14-TH: fixed-fixed 1 m, dT=+100 K, alpha=1.2e-5
Nth = -EA * 1.2e-5 * 100.0
out["TH wall axial force N"] = Nth
out["TH max normal Pa"] = abs(Nth) / C1["A"]
out["TH anchor a on pipe"] = (-Nth, 0, 0, 0, 0, 0)     # pushes the expanding pipe back in +x
out["TH anchor b on pipe"] = (Nth, 0, 0, 0, 0, 0)

# ---- CE: cantilever, CE at tip on UY, 375 N (+y on pipe); W1 tip Fy=-1000, W2 tip Fy=-500
P = 375.0
def ce_case(fy, consumed=True):
    net = fy + (P if consumed else 0.0)
    return on_pipe(O0, [(tip, (0, net, 0))]), net / kb
W1, u1 = ce_case(-1000.0)
W2, u2 = ce_case(-500.0)
out["CE W1 anchor"] = W1; out["CE W1 tip uy"] = u1
out["CE W2 anchor"] = W2; out["CE W2 tip uy"] = u2
out["CE support on pipe"] = (0.0, P, 0.0, 0.0, 0.0, 0.0)
out["CE balance W1 Fy"] = W1[1] + P - 1000.0
out["CE 0.5/0.5 linear anchor Fy"] = 0.5 * W1[1] + 0.5 * W2[1]
out["CE 0.5/0.5 true anchor Fy"] = ce_case(-750.0)[0][1]
out["CE 0.5/0.5 applied"] = 0.5 * P + 0.5 * P
out["CE 1.0/0.5 linear anchor Fy (control)"] = W1[1] + 0.5 * W2[1]
out["CE 1.0/0.5 applied (control)"] = P + 0.5 * P
out["CE 1.0/0.5 true anchor Fy"] = ce_case(-1250.0)[0][1]
out["CE W1-W2 anchor Fy"] = W1[1] - W2[1]
out["CE W1-W2 applied"] = P - P
out["CE non-consuming W1 anchor"] = ce_case(-1000.0, consumed=False)[0]

# ---- SPRING2: spring UY k=1e6 and guide UX at tip; tip Fx=+2000, Fy=+1000
k = 1e6
us = 1000.0 / (kb + k)
spring = -k * us
out["SPRING2 spring on pipe"] = (0.0, spring, 0.0, 0.0, 0.0, 0.0)
out["SPRING2 guide on pipe"] = (-2000.0, 0.0, 0.0, 0.0, 0.0, 0.0)   # tip UX held: bar unstrained
out["SPRING2 anchor"] = on_pipe(O0, [(tip, (2000.0 - 2000.0, 1000.0 + spring, 0))])

# ---- SPRING-GAP: spring UY k=1e6 + gap on UY (1 mm, inactive); tip Fy=+350
ug = 350.0 / (kb + k)
out["GAP tip uy"] = ug
out["GAP inactive (|u| < 1 mm)"] = abs(ug) < 1e-3
out["GAP spring on pipe Fy"] = -k * ug
out["GAP gap on pipe"] = 0.0
ag = on_pipe(O0, [(tip, (0, 350.0 - k * ug, 0))])
out["GAP anchor Fy, Mz"] = (ag[1], ag[5])

# ---- ATTR: rigid guide UY + one-way UY at tip; tip Fy=-1000
out["ATTR tip nodal total on pipe Fy"] = 1000.0
out["ATTR anchor"] = on_pipe(O0, [(tip, (0, -1000.0 + 1000.0, 0))])   # tip held at uy=0: beam unstrained
out["ATTR zero-fill imbalance"] = -1000.0

# ---- NL-C3: gap stop at tip closing on +UY at 0.4 mm; P1=P2= tip Fy=+800
g = 0.4e-3
uc = 800.0 / kb
out["NLC3 case uy"] = uc
out["NLC3 case stop active"] = uc >= g
out["NLC3 superposed uy"] = 2 * uc
out["NLC3 superposed penetrates"] = 2 * uc > g
u_free = 1600.0 / kb
out["NLC3 true uy"] = min(u_free, g)
out["NLC3 true stop on pipe Fy"] = -(1600.0 - kb * g) if u_free > g else 0.0
# STRESS_REFERENCE C3 1-D numbers
out["C3 1d superposed u, true u, lambda"] = (2 * 800 / 1e6, 0.001, 1600 - 1e6 * 0.001)

# ---- Intensified equal-factor measure i*hypot(Mb)/Z at member ends
def bend_about_axis(M, axis):
    ax = unit(axis)
    T = dot(M, ax)
    return T, norm(sub(M, sc(T, ax)))

a, b, c = (0, 0, 0), (1, 0, 0), (1, 1, 0)
i_L = 1.15
for name, Fc in (("L1", (0, 0, 1000.0)), ("L2", (1000.0, 0, 1000.0))):
    Fb, Mb = resultant(b, [(c, Fc)])          # cut at b: outboard is c
    T_ab, Mab = bend_about_axis(Mb, (1, 0, 0))
    T_bc, Mbc = bend_about_axis(Mb, (0, 1, 0))
    out[f"IL {name} ab end j T"] = T_ab
    out[f"IL {name} ab end j Pa"] = i_L * Mab / C1["Z"]
    out[f"IL {name} bc end i Pa"] = i_L * Mbc / C1["Z"]
    out[f"IL {name} ab axial N"] = dot(Fb, (1, 0, 0))
    out[f"IL {name} anchor"] = on_pipe(a, [(c, Fc)])
    if name == "L2":
        Mvec = Mb
        out["IL L2 control abs-sum bc Pa"] = i_L * (abs(Mvec[0]) + abs(Mvec[2])) / C1["Z"]
        out["IL L2 control with k bc Pa"] = i_L * 1.08 * Mbc / C1["Z"]
        out["IL L2 control axial un-intensified ab Pa"] = i_L * Mab / C1["Z"] + abs(dot(Fb, (1, 0, 0))) / C1["A"]
        out["IL L2 control axial intensified ab Pa"] = i_L * (Mab / C1["Z"] + abs(dot(Fb, (1, 0, 0))) / C1["A"])

# I-T: header a-b (SIF 1.3), continuation b-e (unreferenced), branch b-c (SIF 2.0)
e = (2, 0, 0)
loads = [(c, (0, 0, 1000.0)), (e, (0, 0, 500.0))]
_, Mh = resultant(b, loads)                  # a-b cut at b sees c and e
_, Mbr = resultant(b, [loads[0]])            # b-c cut at b sees c
_, Mbe = resultant(b, [loads[1]])            # b-e cut at b sees e
out["IT header ab end j Pa"] = 1.3 * bend_about_axis(Mh, (1, 0, 0))[1] / C1["Z"]
out["IT branch bc end i Pa"] = 2.0 * bend_about_axis(Mbr, (0, 1, 0))[1] / C1["Z"]
out["IT be bending Nm (info)"] = bend_about_axis(Mbe, (1, 0, 0))[1]

# ---- B1 signed (arc b->c, centre (1,0.2,0), R=0.2, load Fz=+1000 at d(1.2,1.2,0))
ctr, Rb, dpt, Pz = (1.0, 0.2, 0.0), 0.2, (1.2, 1.2, 0.0), (0.0, 0.0, 1000.0)
zn = unit(cross(sub((1, 0, 0), ctr), sub((1.2, 0.2, 0), ctr)))   # unit(radial_i x radial_j)
out["B1 bend-plane normal"] = zn
for label, phi in (("end_i_b", 0.0), ("quarter_1", PI / 8), ("midspan", PI / 4), ("quarter_3", 3 * PI / 8), ("end_j_c", PI / 2)):
    p = add(ctr, (Rb * math.sin(phi), -Rb * math.cos(phi), 0.0))
    t = (math.cos(phi), math.sin(phi), 0.0)
    y = unit(sub(ctr, p))
    z = cross(t, y)
    F, M = resultant(p, [(dpt, Pz)])
    out[f"B1 {label} (N,Vy,Vz,T,My,Mz)"] = tuple(dot(F, v) for v in (t, y, z)) + tuple(dot(M, v) for v in (t, y, z))
cx = unit((0.2, 0.2, 0.0))
yref = (1 / math.sqrt(2), -1 / math.sqrt(2), 0.0)
cz_ = unit(cross(cx, yref))
cy = cross(cz_, cx)
for label, p in (("b", (1.0, 0.0, 0.0)), ("c", (1.2, 0.2, 0.0))):
    F, M = resultant(p, [(dpt, Pz)])
    out[f"B1 chord cut-face {label}"] = tuple(dot(F, v) for v in (cx, cy, cz_)) + tuple(dot(M, v) for v in (cx, cy, cz_))
out["B1 chord axes x,y,z"] = cx + cy + cz_
out["B1 product end_i rows (node-on-element = -cut face)"] = sc(-1, out["B1 chord cut-face b"])
out["B1 product end_j rows (node-on-element = +cut face)"] = out["B1 chord cut-face c"]
sb = out["B1 end_i_b (N,Vy,Vz,T,My,Mz)"]
sc_ = out["B1 end_j_c (N,Vy,Vz,T,My,Mz)"]
out["B1 end_i bending normal y Pa (My/Z)"] = sb[4] / C1["Z"]
out["B1 end_i torsional shear Pa (T ro/J)"] = sb[3] * C1["ro"] / C1["J"]
out["B1 end_j bending normal y Pa"] = sc_[4] / C1["Z"]

# ---- B1 tangency: product construction (centre on the -y_reference side of the chord midpoint)
def arc_angles(yr):
    pb, pc = (1.0, 0.0, 0.0), (1.2, 0.2, 0.0)
    ch = sub(pc, pb)
    u = unit(ch)
    yp = unit(sub(yr, sc(dot(yr, u), u)))
    half = norm(ch) / 2
    h = math.sqrt(Rb ** 2 - half ** 2)
    cen = sub(add(pb, sc(0.5, ch)), sc(h, yp))
    res = []
    for p, adj in ((pb, (1, 0, 0)), (pc, (0, 1, 0))):
        r = unit(sub(p, cen))
        t = unit(sub(u, sc(dot(u, r), r)))
        res.append(math.degrees(math.acos(max(-1, min(1, dot(t, adj))))))
    return cen, res
for name, yr in (("consistent", yref), ("kinked (0,0,1)", (0.0, 0.0, 1.0))):
    cen, ang = arc_angles(yr)
    out[f"B1T {name} centre"] = cen
    out[f"B1T {name} angles b,c deg"] = tuple(ang)

# ---- B2: arc only, anchor b, roller UX at c, Fy=+1000 at c; unknown X = roller Fx on pipe
def b2(kf, n=None):
    a_, b_ = kf / EI, 1.0 / EA
    if n is None:   # closed form: int cos = 1, int sin cos = 1/2, int cos^2 = pi/4
        return (2000.0 / PI) * (a_ * Rb ** 2 - b_) / (a_ * Rb ** 2 + b_)
    # Simpson quadrature of the two unit-load integrals d11 X + d10 = 0
    h = (PI / 2) / n
    d11 = d10 = 0.0
    for j in range(n + 1):
        ph = j * h
        wt = 1 if j in (0, n) else (4 if j % 2 else 2)
        s, co = math.sin(ph), math.cos(ph)
        m0, m1 = Rb * (1 - s) * 1000.0, -Rb * co        # M from Fy=1000 and from unit X
        n0, n1 = 1000.0 * s, co                          # N from Fy=1000 and from unit X
        d11 += wt * (a_ * m1 * m1 + b_ * n1 * n1)
        d10 += wt * (a_ * m1 * m0 + b_ * n1 * n0)
    return -d10 / d11

for kf in (1.0, 2.0, 4.0):
    X = b2(kf)
    out[f"B2 k={kf:g} roller on pipe Fx"] = X
    out[f"B2 k={kf:g} section at b (N, Vy_inward, Mz)"] = (X, 1000.0, Rb * (1000.0 - X))
    out[f"B2 k={kf:g} anchor on pipe"] = on_pipe((1.0, 0, 0), [((1.2, 0.2, 0), (X, 1000.0, 0))])
out["B2 k=2 quadrature n=40000"] = b2(2.0, 40000)
out["B2 sensitivity k1->k4"] = b2(4.0) / b2(1.0) - 1.0

# ---- M33-TIE: cantilevers o->p(1,0,0), o->q(-1,0,0); Fz=+1000 at p and q
out["TIE |u| mm at p and q"] = (1000.0 / kb * 1e3, 1000.0 / kb * 1e3)
out["TIE anchor"] = on_pipe(O0, [((1, 0, 0), (0, 0, 1000.0)), ((-1, 0, 0), (0, 0, 1000.0))])

if __name__ == "__main__":
    for kk, vv in out.items():
        if isinstance(vv, tuple):
            print(f"{kk}: (" + ", ".join(repr(float(x)) for x in vv) + ")")
        else:
            print(f"{kk}: {vv!r}")
