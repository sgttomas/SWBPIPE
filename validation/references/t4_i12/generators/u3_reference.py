"""T4-I12: independent references for T4-U3 (the corrected joint), frozen before code.

Standard library only (fractions, decimal). Every connector value is derived from the
definitions in JR CONTRACT.md section 2 (symmetric midpoint, small rotation):

  v_i = u_i + th_i x a_i ; v_j = u_j + th_j x a_j ; th_c = (th_i + th_j)/2
  qt = Q^T[(v_j - v_i) - th_c x r] ; qr = Q^T(th_j - th_i) ; q = B d
  g = K(q - q_ref) ; U = (q - q_ref)^T K (q - q_ref)/2 ; f = B^T g ; Ke = B^T K B
  RHS of the installed state = +B^T K q_ref
  F = Q g_t, M = Q g_r ; Fi = -F, Fj = F, Mi = -(a_i + r/2) x F - M, Mj = (a_j - r/2) x F + M

B is formed twice: from the closed form, and column by column from the kinematic
definition on the 12 basis vectors. End actions are formed twice: B^T g, and the
closed-form blocks. No product code is imported; no old-element value is used.

Usage: python -I u3_reference.py <output json>
"""
import json
import sys
from decimal import Decimal, getcontext
from fractions import Fraction as Fr

getcontext().prec = 160
CHECKS = []


def check(name, ok):
    CHECKS.append((name, bool(ok)))
    if not ok:
        print("FAIL:", name)


# ---------------------------------------------------------------- arithmetic
def F(x):
    return x if isinstance(x, Fr) else Fr(str(x)) if isinstance(x, (float, str)) else Fr(x)


def vadd(a, b):
    return [x + y for x, y in zip(a, b)]


def vsub(a, b):
    return [x - y for x, y in zip(a, b)]


def vscale(s, a):
    return [s * x for x in a]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def skew(v):
    return [[0, -v[2], v[1]], [v[2], 0, -v[0]], [-v[1], v[0], 0]]


def zeros(n, m):
    return [[Fr(0)] * m for _ in range(n)]


def eye(n):
    return [[Fr(1) if i == j else Fr(0) for j in range(n)] for i in range(n)]


def tr(a):
    return [list(r) for r in zip(*a)]


def mm(a, b):
    bt = tr(b)
    return [[sum(x * y for x, y in zip(r, c)) for c in bt] for r in a]


def mv(a, v):
    return [sum(x * y for x, y in zip(r, v)) for r in a]


def madd(a, b):
    return [[x + y for x, y in zip(r, s)] for r, s in zip(a, b)]


def mscale(s, a):
    return [[s * x for x in r] for r in a]


def hcat(*blocks):
    return [sum((list(b[i]) for b in blocks), []) for i in range(len(blocks[0]))]


def col(Q, k):
    return [Q[i][k] for i in range(3)]


def rank(a):
    m = [list(map(F, r)) for r in a]
    rows, cols = len(m), len(m[0])
    rk = 0
    for c in range(cols):
        piv = next((i for i in range(rk, rows) if m[i][c] != 0), None)
        if piv is None:
            continue
        m[rk], m[piv] = m[piv], m[rk]
        for i in range(rows):
            if i != rk and m[i][c] != 0:
                f = m[i][c] / m[rk][c]
                m[i] = [x - f * y for x, y in zip(m[i], m[rk])]
        rk += 1
    return rk


def solve(a, b):
    n = len(a)
    m = [list(a[i]) + [b[i]] for i in range(n)]
    for c in range(n):
        piv = next(i for i in range(c, n) if m[i][c] != 0)
        m[c], m[piv] = m[piv], m[c]
        p = m[c][c]
        for i in range(c + 1, n):
            if m[i][c] != 0:
                f = m[i][c] / p
                m[i] = [x - f * y for x, y in zip(m[i], m[c])]
    x = [Fr(0)] * n
    for i in reversed(range(n)):
        x[i] = (m[i][n] - sum(m[i][j] * x[j] for j in range(i + 1, n))) / m[i][i]
    return x


def ldl_pivots(k):
    """Symmetric elimination without pivoting; returns the pivots (exact).
    PD iff every pivot > 0. A zero pivot whose remaining column is zero is skipped (PSD)."""
    m = [list(r) for r in k]
    n = len(m)
    piv = []
    for c in range(n):
        p = m[c][c]
        piv.append(p)
        if p == 0:
            if any(m[i][c] != 0 for i in range(c + 1, n)):
                piv[-1] = None  # indefinite: zero pivot with nonzero column
            continue
        for i in range(c + 1, n):
            f = m[i][c] / p
            m[i] = [x - f * y for x, y in zip(m[i], m[c])]
    return piv


def is_pd(k):
    return all(p is not None and p > 0 for p in ldl_pivots(k))


# ---------------------------------------------------------------- formatting
SIG = 34


def dec(x, sig=SIG):
    x = F(x)
    if x == 0:
        return "0"
    d = Decimal(x.numerator) / Decimal(x.denominator)
    return format(d, "." + str(sig) + "g")


def ex(x):
    x = F(x)
    return str(x.numerator) if x.denominator == 1 else f"{x.numerator}/{x.denominator}"


def exv(v):
    return [ex(x) for x in v]


def exm(a):
    return [[ex(x) for x in r] for r in a]


def both(x):
    return {"exact": ex(x), "decimal": dec(x)}


def bothv(v):
    return {"exact": exv(v), "decimal": [dec(x) for x in v]}


# ---------------------------------------------------------------- connector
def Dscale(Ls):
    return [Ls, Ls, Ls, Fr(1), Fr(1), Fr(1)]


def K_from_H(H, Ls):
    d = Dscale(Ls)
    return [[H[i][j] / (d[i] * d[j]) for j in range(6)] for i in range(6)]


def H_from_K(K, Ls):
    d = Dscale(Ls)
    return [[K[i][j] * d[i] * d[j] for j in range(6)] for i in range(6)]


def upper21(M):
    return [M[i][j] for i in range(6) for j in range(i, 6)]


def from_upper21(u):
    M = zeros(6, 6)
    k = 0
    for i in range(6):
        for j in range(i, 6):
            M[i][j] = M[j][i] = F(u[k])
            k += 1
    return M


def B_closed(ai, aj, r, Q):
    QT = tr(Q)
    I3, Z3 = eye(3), zeros(3, 3)
    half_r = vscale(Fr(1, 2), r)
    Bt = mm(QT, hcat(mscale(-1, I3), madd(skew(ai), skew(half_r)), I3, madd(mscale(-1, skew(aj)), skew(half_r))))
    Br = mm(QT, hcat(Z3, mscale(-1, I3), Z3, I3))
    return Bt + Br


def q_kinematic(d, ai, aj, r, Q):
    ui, ti, uj, tj = d[0:3], d[3:6], d[6:9], d[9:12]
    vi = vadd(ui, cross(ti, ai))
    vj = vadd(uj, cross(tj, aj))
    tc = vscale(Fr(1, 2), vadd(ti, tj))
    QT = tr(Q)
    qt = mv(QT, vsub(vsub(vj, vi), cross(tc, r)))
    qr = mv(QT, vsub(tj, ti))
    return qt + qr


def B_kinematic(ai, aj, r, Q):
    cols = []
    for k in range(12):
        e = [Fr(0)] * 12
        e[k] = Fr(1)
        cols.append(q_kinematic(e, ai, aj, r, Q))
    return tr(cols)


def actions_closed(g, ai, aj, r, Q):
    Fv = mv(Q, g[0:3])
    Mv = mv(Q, g[3:6])
    hr = vscale(Fr(1, 2), r)
    Fi = vscale(-1, Fv)
    Fj = Fv
    Mi = vsub(vscale(-1, cross(vadd(ai, hr), Fv)), Mv)
    Mj = vadd(cross(vsub(aj, hr), Fv), Mv)
    return Fv, Mv, Fi + Mi + Fj + Mj


def rigid_modes(xi, xj, o):
    modes = []
    for k in range(3):
        t = [Fr(0)] * 3
        t[k] = Fr(1)
        modes.append(("translation_" + "xyz"[k], t + [Fr(0)] * 3 + t + [Fr(0)] * 3))
    for k in range(3):
        w = [Fr(0)] * 3
        w[k] = Fr(1)
        ui = cross(w, vsub(xi, o))
        uj = cross(w, vsub(xj, o))
        modes.append(("rotation_" + "xyz"[k], ui + w + uj + w))
    return modes


def proper_orthonormal(Q):
    return mm(tr(Q), Q) == eye(3) and (
        Q[0][0] * (Q[1][1] * Q[2][2] - Q[1][2] * Q[2][1])
        - Q[0][1] * (Q[1][0] * Q[2][2] - Q[1][2] * Q[2][0])
        + Q[0][2] * (Q[1][0] * Q[2][1] - Q[1][1] * Q[2][0])
    ) == 1


def evaluate(case_id, xi, xj, ai, aj, Q, K, qref, d, Ls=None, origin=None, label=""):
    """All connector quantities, each formed two ways where possible."""
    pi = vadd(xi, ai)
    pj = vadd(xj, aj)
    r = vsub(pj, pi)
    check(f"{case_id}: Q proper orthonormal", proper_orthonormal(Q))
    if any(x != 0 for x in r):
        # Q.x parallel to r with positive orientation
        qx = col(Q, 0)
        check(f"{case_id}: Q.x follows r", cross(qx, r) == [0, 0, 0] and dot(qx, r) > 0)
    B = B_closed(ai, aj, r, Q)
    Bk = B_kinematic(ai, aj, r, Q)
    check(f"{case_id}: closed-form B equals kinematic B (all 72 entries, exact)", B == Bk)
    check(f"{case_id}: rank B = 6 (exact)", rank(B) == 6)
    Ke = mm(mm(tr(B), K), B)
    check(f"{case_id}: Ke symmetric", Ke == tr(Ke))
    out = {"r": exv(r), "p_i": exv(pi), "p_j": exv(pj), "B": exm(B), "Ke": exm(Ke)}
    o = origin if origin is not None else [Fr(0)] * 3
    modes = rigid_modes(xi, xj, o)
    for name, m in modes:
        check(f"{case_id}: rigid {name} about origin {exv(o)}: B d = 0 exactly", mv(B, m) == [0] * 6)
        check(f"{case_id}: rigid {name}: Ke d = 0 exactly", mv(Ke, m) == [0] * 12)
    check(f"{case_id}: six rigid modes linearly independent", rank([m for _, m in modes]) == 6)
    out["rigid_modes_origin"] = exv(o)
    out["rank_B"] = 6
    out["rank_Ke"] = rank(Ke)
    out["K_pd_exact"] = is_pd(K)
    out["K_ldl_pivots"] = [None if p is None else ex(p) for p in ldl_pivots(K)]
    if is_pd(K):
        check(f"{case_id}: PD K -> rank Ke = 6, null(Ke) = span(rigid modes)", rank(Ke) == 6)
    # reference-load (installed) vector and its self-equilibrium (S13)
    Kq = mv(K, qref)
    rhs = mv(tr(B), Kq)
    out["installed_rhs_BT_K_qref"] = exv(rhs)
    out["K_qref"] = exv(Kq)
    out["stress_free"] = all(x == 0 for x in Kq)
    ssum = vadd(rhs[0:3], rhs[6:9])
    msum = vadd(vadd(vadd(cross(xi, rhs[0:3]), rhs[3:6]), cross(xj, rhs[6:9])), rhs[9:12])
    check(f"{case_id}: +B^T K q_ref self-equilibrated (force and moment, exact)", ssum == [0] * 3 and msum == [0] * 3)
    if d is not None:
        q = mv(B, d)
        check(f"{case_id}: q from B equals q from kinematics", q == q_kinematic(d, ai, aj, r, Q))
        e = vsub(q, qref)
        g = mv(K, e)
        U = dot(e, g) / 2
        f = mv(tr(B), g)
        Fv, Mv, fc = actions_closed(g, ai, aj, r, Q)
        check(f"{case_id}: B^T g equals closed-form end blocks", f == fc)
        check(f"{case_id}: Ke d - B^T K q_ref equals B^T g", vsub(mv(Ke, d), rhs) == f)
        # resultant equilibrium about the origin and about o
        for oo in ([Fr(0)] * 3, o, [Fr(7), Fr(-3), Fr(5)]):
            fs = vadd(f[0:3], f[6:9])
            ms = vadd(vadd(vadd(cross(vsub(xi, oo), f[0:3]), f[3:6]), cross(vsub(xj, oo), f[6:9])), f[9:12])
            check(f"{case_id}: end actions balance about {exv(oo)}", fs == [0] * 3 and ms == [0] * 3)
        # virtual work g.dq = f.dd for an arbitrary variation
        dd = [Fr(k - 4, 17) for k in range(12)]
        check(f"{case_id}: virtual work g.(B dd) = f.dd", dot(g, mv(B, dd)) == dot(f, dd))
        out.update({
            "d": exv(d), "q": bothv(q), "g": bothv(g), "energy": both(U),
            "F_global": bothv(Fv), "M_global": bothv(Mv),
            "end_actions_node_on_element": {
                "Fi": bothv(f[0:3]), "Mi": bothv(f[3:6]), "Fj": bothv(f[6:9]), "Mj": bothv(f[9:12])},
        })
    return out, (B, Ke, r)


def connector_inputs(xi, xj, ai, aj, Q, K, qref, Ls, node_axes_i=None, node_axes_j=None, off_i=None, off_j=None):
    H = H_from_K(K, Ls)
    return {
        "x_i": exv(xi), "x_j": exv(xj),
        "a_i_global": exv(ai), "a_j_global": exv(aj),
        "attachment_local": None if off_i is None else {
            "initial_node_axes_global_i": exm(node_axes_i), "offset_local_i": exv(off_i),
            "initial_node_axes_global_j": exm(node_axes_j), "offset_local_j": exv(off_j)},
        "Q_row_major_columns_are_axes": exm(Q),
        "translation_scale_Ls_m": ex(Ls),
        "H_upper_triangle_21_N_m": exv(upper21(H)),
        "K_physical": exm(K),
        "q_ref": exv(qref),
    }


def Kdiag(v):
    K = zeros(6, 6)
    for i, x in enumerate(v):
        K[i][i] = F(x)
    return K


def vec(*xs):
    return [F(x) for x in xs]


ORIGIN = vec(0, 0, 0)
I3 = eye(3)
cases = {}


def add_case(cid, item, purpose, inputs, out, extra=None, criterion=None, jr=None, disc=None):
    c = {"case_id": cid, "brief_item": item, "purpose": purpose, "inputs": inputs, "expected": out}
    if criterion:
        c["criterion"] = criterion
    if jr:
        c["jr_comparison"] = jr
    if disc:
        c["wrong_result_discriminators"] = disc
    if extra:
        c.update(extra)
    cases[cid] = c


UNIT = "FK unit level: exact (every value is a finite rational of the binary64-representable or decimal inputs; compare at relative 1e-12 when inputs are decimal, bitwise when B/Ke formation is exact)"


def jr_agree(label, stated, derived):
    ok = F(stated) == F(derived)
    check(f"JR agreement: {label}: stated {stated}, derived {ex(derived)}", ok)
    return {"quantity": label, "jr_stated": str(stated), "derived": ex(derived), "agrees": ok}


# =====================================================================================
# Item 1: JR J1 and J2
# =====================================================================================
KJ = Kdiag([200000, 80000, 120000, 600, 900, 1200])
xiJ, xjJ = vec(0, 0, 0), vec("0.3", 0, 0)
z3 = vec(0, 0, 0)
q0 = [Fr(0)] * 6


def dvec(ui=z3, ti=z3, uj=z3, tj=z3):
    return list(ui) + list(ti) + list(uj) + list(tj)


jr_rows = []
d = dvec(uj=vec(0, "0.001", 0))
out, (BJ, KeJ, _) = evaluate("U3-J1-LATERAL", xiJ, xjJ, z3, z3, I3, KJ, q0, d)
g, U = [F(x) for x in out["g"]["exact"]], F(out["energy"]["exact"])
fa = out["end_actions_node_on_element"]
jr = [jr_agree("J1 q_y", "0.001", F(out["q"]["exact"][1])), jr_agree("J1 g_y", 80, g[1]), jr_agree("J1 U", "0.04", U),
      jr_agree("J1 Fi_y", -80, F(fa["Fi"]["exact"][1])), jr_agree("J1 Fj_y", 80, F(fa["Fj"]["exact"][1])),
      jr_agree("J1 Mi_z", -12, F(fa["Mi"]["exact"][2])), jr_agree("J1 Mj_z", -12, F(fa["Mj"]["exact"][2]))]
jr_rows += jr
add_case("U3-J1-LATERAL", 1, "JR J1: pure end-j lateral translation; equal same-sign end moments balance the force couple",
         connector_inputs(xiJ, xjJ, z3, z3, I3, KJ, q0, Fr(1)), out, criterion=UNIT, jr=jr,
         disc=[{"what": "end moments omitted (Mi_z = Mj_z = 0): angular equilibrium fails by 24 N*m about z", "Mi_z": "0", "Mj_z": "0"}])

d = dvec(ti=vec(0, 0, "0.01"), uj=vec(0, "0.003", 0), tj=vec(0, 0, "0.01"))
out, _ = evaluate("U3-J1-COMMON-ROTATION", xiJ, xjJ, z3, z3, I3, KJ, q0, d)
check("J1 common rotation: q = 0, U = 0, actions 0", all(F(x) == 0 for x in out["q"]["exact"]) and F(out["energy"]["exact"]) == 0)
qraw = vsub(d[6:9], d[0:3]) + vsub(d[9:12], d[3:6])
graw = mv(KJ, qraw)
Uraw = dot(qraw, graw) / 2
jr = [jr_agree("J1 common-rotation q", 0, F(out["q"]["exact"][1])), jr_agree("raw-difference q_y", "0.003", qraw[1]),
      jr_agree("raw-difference g_y", 240, graw[1]), jr_agree("raw-difference U", "0.36", Uraw)]
jr_rows += jr
add_case("U3-J1-COMMON-ROTATION", 1, "JR J1 rigid control: common omega_z = 0.01 with u_j_y = omega_z L gives q = 0, U = 0, all actions 0",
         connector_inputs(xiJ, xjJ, z3, z3, I3, KJ, q0, Fr(1)), out, criterion=UNIT + "; zero-scale floor for q_t: 0.003 m, for g_t: 240 N, for U: 0.36 J",
         jr=jr, disc=[{"what": "raw endpoint difference Q^T(u_j - u_i) (the deleted element's measure; analytical value the product must not produce)",
                       "q_y_m": ex(qraw[1]), "g_y_N": ex(graw[1]), "energy_J": ex(Uraw)}])

d = dvec(uj=vec(0, "0.0015", 0), tj=vec(0, 0, "0.01"))
out, _ = evaluate("U3-J2-ROTATION", xiJ, xjJ, z3, z3, I3, KJ, q0, d)
g = [F(x) for x in out["g"]["exact"]]
jr = [jr_agree("J2 qt", 0, max(abs(F(x)) for x in out["q"]["exact"][0:3])), jr_agree("J2 qr_z", "0.01", F(out["q"]["exact"][5])),
      jr_agree("J2 g_mz", 12, g[5]), jr_agree("J2 U", "0.06", F(out["energy"]["exact"]))]
jr_rows += jr
add_case("U3-J2-ROTATION", 1, "JR J2: end-j rotation with the cancelling end-j translation u_j_y = L theta/2: pure qr_z",
         connector_inputs(xiJ, xjJ, z3, z3, I3, KJ, q0, Fr(1)), out, criterion=UNIT, jr=jr)

d = dvec(tj=vec(0, 0, "0.01"))
out, _ = evaluate("U3-J2-ROTATION-HELD", xiJ, xjJ, z3, z3, I3, KJ, q0, d)
g = [F(x) for x in out["g"]["exact"]]
fa = out["end_actions_node_on_element"]
Mjz, Miz = F(fa["Mj"]["exact"][2]), F(fa["Mi"]["exact"][2])
L = F("0.3")
check("J2 held: Mj_z = (krz + ky L^2/4) phi", Mjz == (1200 + 80000 * L * L / 4) * F("0.01"))
check("J2 held: Mi_z = (ky L^2/4 - krz) phi", Miz == (80000 * L * L / 4 - 1200) * F("0.01"))
EIs = Fr(1)
check("beam cross-check: ky = 12EI/L^3, krz = EI/L gives the held end-j rate krz + ky L^2/4 = 4EI/L", EIs / L + 12 * EIs / L ** 3 * L * L / 4 == 4 * EIs / L)
jr = [jr_agree("J2 held qt_y", "-0.0015", F(out["q"]["exact"][1])), jr_agree("J2 held g_y", -120, g[1]), jr_agree("J2 held g_mz", 12, g[5])]
jr_rows += jr
add_case("U3-J2-ROTATION-HELD", 1, "JR J2 with u_j_y held at 0: the apparent end-j angular rate is krz + ky L^2/4, not krz (refutation section 1)",
         connector_inputs(xiJ, xjJ, z3, z3, I3, KJ, q0, Fr(1)), out, criterion=UNIT, jr=jr,
         disc=[{"what": "end-j moment taken as krz*phi (measurement-restraint confusion)", "Mj_z_N_m": "12"}])

# refutation's own end-moment control: L = 2, ky = 20, krz = 60, phi = 0.01
Kr = Kdiag([1, 20, 1, 1, 1, 60])
xr = vec(2, 0, 0)
out_a, _ = evaluate("U3-REF-ENDMOMENT", ORIGIN, xr, z3, z3, I3, Kr, q0, dvec(tj=vec(0, 0, "0.01")))
fa = out_a["end_actions_node_on_element"]
jr = [jr_agree("refutation Fj_y", "-0.2", F(fa["Fj"]["exact"][1])), jr_agree("refutation Mi_z", "-0.4", F(fa["Mi"]["exact"][2])),
      jr_agree("refutation Mj_z", "0.8", F(fa["Mj"]["exact"][2])), jr_agree("refutation energy", "0.004", F(out_a["energy"]["exact"]))]
out_b, _ = evaluate("U3-REF-PURE-QRZ", ORIGIN, xr, z3, z3, I3, Kr, q0, dvec(uj=vec(0, "0.01", 0), tj=vec(0, 0, "0.01")))
jr += [jr_agree("refutation pure-qrz generalized moment", "0.6", F(out_b["g"]["exact"][5])),
       jr_agree("refutation pure-qrz energy", "0.003", F(out_b["energy"]["exact"]))]
jr_rows += jr
add_case("U3-REF-ENDMOMENT", 1, "Refutation section 1 measurement control (L = 2, ky = 20, krz = 60): end-j angle test with both translations held",
         connector_inputs(ORIGIN, xr, z3, z3, I3, Kr, q0, Fr(1)), out_a, criterion=UNIT, jr=jr,
         extra={"companion_pure_qrz": out_b})

# historical six components (ANALYTICAL_ORACLES_V1 connector, refuted here)
K6 = Kdiag([10, 20, 30, 40, 50, 60])
six = {}
for name, dd, gi, gv, Uv in [
    ("tx", dvec(uj=vec("0.01", 0, 0)), 0, "0.1", "0.0005"),
    ("ty", dvec(uj=vec(0, "0.01", 0)), 1, "0.2", "0.001"),
    ("tz", dvec(uj=vec(0, 0, "0.01")), 2, "0.3", "0.0015"),
    ("rx", dvec(tj=vec("0.01", 0, 0)), 3, "0.4", "0.002"),
    ("ry", dvec(uj=vec(0, 0, "-0.01"), tj=vec(0, "0.01", 0)), 4, "0.5", "0.0025"),
    ("rz", dvec(uj=vec(0, "0.01", 0), tj=vec(0, 0, "0.01")), 5, "0.6", "0.003"),
]:
    o, _ = evaluate(f"U3-SIX-{name}", ORIGIN, xr, z3, z3, I3, K6, q0, dd)
    qv = [F(x) for x in o["q"]["exact"]]
    check(f"six {name}: q isolated to one coordinate = 0.01", qv[gi] == F("0.01") and all(qv[k] == 0 for k in range(6) if k != gi))
    jr_rows.append(jr_agree(f"historical six-component {name} generalized action", gv, F(o["g"]["exact"][gi])))
    jr_rows.append(jr_agree(f"historical six-component {name} energy", Uv, F(o["energy"]["exact"])))
    six[name] = o
fa = six["ty"]["end_actions_node_on_element"]
jr_rows += [jr_agree("historical ty node moment z (i)", "-0.2", F(fa["Mi"]["exact"][2])), jr_agree("historical ty node moment z (j)", "-0.2", F(fa["Mj"]["exact"][2]))]
fa = six["tz"]["end_actions_node_on_element"]
jr_rows += [jr_agree("historical tz node moment y (i)", "0.3", F(fa["Mi"]["exact"][1])), jr_agree("historical tz node moment y (j)", "0.3", F(fa["Mj"]["exact"][1]))]
add_case("U3-SIX-COMPONENTS", 1, "Each generalized coordinate isolated (historical oracle base geometry, L = 2 m, Kc = diag(10..60), Ls = 2 m, H = diag(40,80,120,40,50,60))",
         connector_inputs(ORIGIN, xr, z3, z3, I3, K6, q0, Fr(2)), {k: v for k, v in six.items()}, criterion=UNIT)

# =====================================================================================
# Item 2: the B oracle (refutation section 6)
# =====================================================================================
xiB = vec(1, 2, 3)
aiB = vec("1/5", "2/5", "-1/5")
ajB = vec("-1/10", "3/10", "1/2")
rB = vec(2, 0, 0)
xjB = vsub(vadd(vadd(xiB, aiB), rB), ajB)
gB = vec(2, -3, 5, 7, -11, 13)
oB = vec(7, -3, 5)
BB = B_closed(aiB, ajB, rB, I3)
check("B oracle: kinematic B equals closed form", BB == B_kinematic(aiB, ajB, rB, I3))
fB = mv(tr(BB), gB)
_, _, fBc = actions_closed(gB, aiB, ajB, rB, I3)
check("B oracle: B^T g equals closed-form blocks", fB == fBc)
jr = [jr_agree(f"B oracle {nm}[{k}]", s, fB[3 * blk + k]) for blk, (nm, vals) in enumerate(
    [("Fi", ["-2", "3", "-5"]), ("Mi", ["-42/5", "87/5", "-43/5"]), ("Fj", ["2", "-3", "5"]), ("Mj", ["10", "-9/2", "157/10"])]) for k, s in enumerate(vals)]
dB = [Fr(k - 4, 17) for k in range(12)]
qB = mv(BB, dB)
jr += [jr_agree(f"B oracle q[{k}] for d_k=(k-4)/17", s, qB[k]) for k, s in enumerate(["73/170", "-26/85", "29/34", "6/17", "6/17", "6/17"])]
jr += [jr_agree("B oracle virtual work g.q", "1567/170", dot(gB, qB)), jr_agree("B oracle virtual work f.d", "1567/170", dot(fB, dB))]
jr_rows += jr
for name, m in rigid_modes(xiB, xjB, oB):
    check(f"B oracle rigid {name} about (7,-3,5)", mv(BB, m) == [0] * 6)
fs = vadd(fB[0:3], fB[6:9])
ms = vadd(vadd(vadd(cross(vsub(xiB, oB), fB[0:3]), fB[3:6]), cross(vsub(xjB, oB), fB[6:9])), fB[9:12])
check("B oracle: force and arbitrary-origin moment sums zero", fs == [0] * 3 and ms == [0] * 3)
add_case("U3-B-ORACLE", 2, "Refutation section 6: B column by column, end blocks for a given g, q and virtual work for d_k = (k-4)/17",
         {"x_i": exv(xiB), "x_j": exv(xjB), "a_i_global": exv(aiB), "a_j_global": exv(ajB), "r": exv(rB), "Q_row_major_columns_are_axes": exm(I3),
          "g_given": exv(gB), "d_given": exv(dB), "origin_for_rigid_modes": exv(oB)},
         {"B": exm(BB), "end_actions_node_on_element": {"Fi": exv(fB[0:3]), "Mi": exv(fB[3:6]), "Fj": exv(fB[6:9]), "Mj": exv(fB[9:12])},
          "q_for_d_given": exv(qB), "virtual_work": ex(dot(gB, qB))},
         criterion="exact (all inputs rational; compare bitwise where the binary64 operands are exact, else relative 1e-12)", jr=jr)

# =====================================================================================
# Item 3 and item 4: skew frame, offsets, coupled PD K, prestress (the generic case)
# =====================================================================================
QS = tr([vec("1/3", "2/3", "2/3"), vec("2/3", "1/3", "-2/3"), vec("-2/3", "2/3", "-1/3")])  # columns x, y, z
xiS = vec("1/2", -1, 2)
aiS = vec("1/5", "2/5", "-1/5")
ajS = vec("-1/10", "3/10", "1/2")
rS = vec(1, 2, 2)  # 3 * Q.x
xjS = vsub(vadd(vadd(xiS, aiS), rS), ajS)
KS = [[F(x) for x in row] for row in [
    [200000, 10000, 0, 0, 2000, 0],
    [10000, 150000, 5000, 0, 0, -3000],
    [0, 5000, 120000, 1000, 0, 0],
    [0, 0, 1000, 800, 50, 0],
    [2000, 0, 0, 50, 900, 100],
    [0, -3000, 0, 0, 100, 1200]]]
LsS = Fr(1, 4)
qrefS = vec("1/1000", "-1/2000", "1/4000", "1/500", 0, "-1/1000")
dS = vec("1/1000", "-2/1000", "3/1000", "1/200", "-1/300", "1/400", "-1/1000", "4/1000", "2/1000", "-1/250", "1/500", "3/1000")
oS = vec(7, -3, 5)
check("generic K is PD (exact pivots)", is_pd(KS))
check("generic H decodes back to K (21-entry storage)", K_from_H(from_upper21(upper21(H_from_K(KS, LsS))), LsS) == KS)
outS, (BS, KeS, _) = evaluate("U3-GENERIC-SKEW-OFFSET-PRESTRESS", xiS, xjS, aiS, ajS, QS, KS, qrefS, dS, origin=oS)
# force scaling (S8): K * 2^b, b even; B and q_ref do not scale
b = 20
Kb = mscale(Fr(2) ** b, KS)
check("S8: Ke(2^b K) = 2^b Ke(K) exactly (b = 20)", mm(mm(tr(BS), Kb), BS) == mscale(Fr(2) ** b, KeS))
check("S8: B^T (2^b K) q_ref = 2^b B^T K q_ref exactly", mv(tr(BS), mv(Kb, qrefS)) == vscale(Fr(2) ** b, mv(tr(BS), mv(KS, qrefS))))
add_case("U3-GENERIC-SKEW-OFFSET-PRESTRESS", "3, 4", "Skew rational frame, both offsets, fully coupled PD K, prestressed q_ref, generic d: B, Ke, rigid null space, actions, energy, RHS",
         connector_inputs(xiS, xjS, aiS, ajS, QS, KS, qrefS, LsS), outS, criterion=UNIT,
         extra={"force_scaling_S8": "K_s = 2^b K (b even) gives Ke_s = 2^b Ke and B^T K_s q_ref = 2^b B^T K q_ref exactly; B and q_ref are unscaled (checked at b = 20)",
                "kd5_note": "Ke is given exactly (rationals): K-D5's Wide<2> re-formation of B^T K B from the binary64 operands must reproduce it within its own bound (the decimal operands are not binary64-exact, so bit equality is not expected)"},
         disc=[{"what": "offsets ignored (a_i = a_j = 0, r = x_j - x_i)", "q": bothv(mv(B_closed(z3, z3, vsub(xjS, xiS), QS), dS))},
               {"what": "Q transposed (rows read as axes)", "q": bothv(mv(B_closed(aiS, ajS, rS, tr(QS)), dS))},
               {"what": "initial residual omitted: f = Ke d instead of Ke d - B^T K q_ref", "f": bothv(mv(KeS, dS))}])

# PSD and indefinite K (W4 link rule)
Hc = zeros(6, 6)
Hc[0][0], Hc[0][3], Hc[3][0], Hc[3][3] = F(4), F(1), F(1), F(9)
KcS = K_from_H(Hc, Fr(2))
outP, (BP, KeP, _) = evaluate("U3-PSD-NULL-SPACE", xiS, xjS, aiS, ajS, QS, KcS, q0, None, origin=oS)
check("PSD: rank Ke = 2, null dimension 10", rank(KeP) == 2)
dnull = dvec(uj=col(QS, 1))  # pure qt_y, not rigid
check("PSD: pure qt_y motion lies in null(Ke)", mv(KeP, dnull) == [0] * 12 and mv(BP, dnull) == vec(0, 1, 0, 0, 0, 0))
qnull = vec(0, "0.003", 0, 0, 0, 0)
check("PSD: a null-coordinate q_ref (qt_y) gives K q_ref = 0: stress-free, no force or energy", mv(KcS, qnull) == [0] * 6)
Hbad = zeros(6, 6)
Hbad[0][0], Hbad[0][3], Hbad[3][0], Hbad[3][3] = F(4), F(5), F(5), F(4)
for k in (1, 2, 4, 5):
    Hbad[k][k] = F(1)
piv_bad = ldl_pivots(Hbad)
check("indefinite H: a negative exact pivot exists", any(p is not None and p < 0 for p in piv_bad))
add_case("U3-W4-LINK-RULE", 3, "W4 link rule: PD K gives null(Ke) = the six rigid modes exactly (rank Ke = 6; see U3-GENERIC); PSD K (JR coupled 4/1/9 only) leaves rank Ke = 2 and a 10-dimensional null space (unqualified); indefinite H rejects",
         {"geometry": "as U3-GENERIC-SKEW-OFFSET-PRESTRESS", "psd_H_upper_triangle_21_Ls_2m": exv(upper21(Hc)),
          "indefinite_H_upper_triangle_21_Ls_2m": exv(upper21(Hbad))},
         {"pd_case": {"rank_Ke": 6, "null_space": "exactly span of the six rigid modes (see U3-GENERIC rigid checks)", "W4": "links (qualified)"},
          "psd_case": {"rank_Ke": outP["rank_Ke"], "null_dimension": 12 - outP["rank_Ke"], "ldl_pivots_K": outP["K_ldl_pivots"],
                       "non_rigid_null_vector_d": exv(dnull), "its_q": exv(mv(BP, dnull)), "W4": "ConnectorSemidefinite (unqualified), passed to the matrix gate",
                       "null_coordinate_q_ref": {"q_ref": exv(qnull), "K_q_ref": exv(mv(KcS, qnull)), "meaning": "accepted only as stress_free (K q_ref = 0); not observable in force or energy"}},
          "indefinite_case": {"ldl_pivots_H": [None if p is None else ex(p) for p in piv_bad], "decision": "reject (OBJECTIVE_CONNECTOR_INPUT_INCOMPLETE: H not PSD); never projected"}},
         criterion="exact rank and exact pivot signs (libm-free decision)")

# frame covariance: rotate and translate the generic case
R = [[F(0), F(-1), F(0)], [F(1), F(0), F(0)], [F(0), F(0), F(1)]]
t = vec(7, -3, 5)
rot = lambda v: mv(R, v)
xiR, xjR = vadd(rot(xiS), t), vadd(rot(xjS), t)
QR = mm(R, QS)
dR = rot(dS[0:3]) + rot(dS[3:6]) + rot(dS[6:9]) + rot(dS[9:12])
outR, _ = evaluate("U3-GENERIC-ROTATED", xiR, xjR, rot(aiS), rot(ajS), QR, KS, qrefS, dR, origin=vadd(rot(oS), t))
check("covariance: q, g, U unchanged under rigid change of coordinates", outR["q"] == outS["q"] and outR["g"] == outS["g"] and outR["energy"] == outS["energy"])
for blk in ("Fi", "Mi", "Fj", "Mj"):
    check(f"covariance: {blk} rotates with R", [F(x) for x in outR["end_actions_node_on_element"][blk]["exact"]] == rot([F(x) for x in outS["end_actions_node_on_element"][blk]["exact"]]))
add_case("U3-FRAME-COVARIANCE", 4, "Rigid change of reference coordinates (R = 90 deg about z, t = (7,-3,5)) applied to every reference vector, axis and displacement: local q, g, U unchanged; global actions rotate",
         {"R_row_major": exm(R), "t": exv(t), **connector_inputs(xiR, xjR, rot(aiS), rot(ajS), QR, KS, qrefS, LsS)}, outR, criterion=UNIT)

# historical offset case with its rotation
xiO, xjO = vec(1, 2, 3), vec(3, 2, 3)
offO = vec(0, "0.2", 0)
dO = vec("1/1000", "-1/500", "3/1000", "1/100", "-1/200", "1/300", "-1/400", "1/250", "-1/1000", "-1/150", "1/120", "1/100")
outO, _ = evaluate("U3-OFFSETS", xiO, xjO, offO, offO, I3, K6, q0, dO)
add_case("U3-OFFSETS", 4, "Historical offset case: x_i = (1,2,3), x_j = (3,2,3), local offsets (0, 0.2, 0) at both ends with identity node triads; generic d",
         connector_inputs(xiO, xjO, offO, offO, I3, K6, q0, Fr(2), I3, I3, offO, offO), outO, criterion=UNIT,
         disc=[{"what": "offsets ignored", "q": bothv(mv(B_closed(z3, z3, vec(2, 0, 0), I3), dO))}])

# coupled H (4/1/9), scale, preload (JR section 2, historical coupled case)
xc = vec(2, 0, 0)
dC = dvec(uj=vec("0.2", 0, 0), tj=vec("0.1", 0, 0))
outC, _ = evaluate("U3-COUPLED-H", ORIGIN, xc, z3, z3, I3, KcS, q0, dC)
gC = [F(x) for x in outC["g"]["exact"]]
qhat = [F(outC["q"]["exact"][k]) / Dscale(Fr(2))[k] for k in range(6)]
check("coupled H: energy from H and qhat equals energy from K and q", dot(qhat, mv(Hc, qhat)) / 2 == F(outC["energy"]["exact"]))
jr = [jr_agree("coupled H energy", "0.075", F(outC["energy"]["exact"])), jr_agree("coupled H g_tx", "0.25", gC[0]), jr_agree("coupled H g_rx", 1, gC[3])]
Rs = [Fr(1) / Fr(2)] * 3 + [Fr(1)] * 3  # D'D^-1 for Ls 2 -> 1
H1 = [[Hc[i][j] * Rs[i] * Rs[j] for j in range(6)] for i in range(6)]
check("scale: K unchanged under Ls 2 -> 1 with congruent H", K_from_H(H1, Fr(1)) == KcS)
jr += [jr_agree("scale H'00", 1, H1[0][0]), jr_agree("scale H'03", "0.5", H1[0][3]), jr_agree("scale H'33", 9, H1[3][3])]
# mm spelling: translation_scale 2000 mm = 2 m; H in N*m is unchanged
# preload: q_ref = (0.2,0,0,0.1,0,0), d = 0
qrefC = vec("0.2", 0, 0, "0.1", 0, 0)
outC0, _ = evaluate("U3-PRELOAD-INSTALLED", ORIGIN, xc, z3, z3, I3, KcS, qrefC, [Fr(0)] * 12)
f0 = outC0["end_actions_node_on_element"]
jr += [jr_agree("preload initial internal force x at i", "0.25", F(f0["Fi"]["exact"][0])), jr_agree("preload initial internal force x at j", "-0.25", F(f0["Fj"]["exact"][0])),
       jr_agree("preload initial x-moment at i", 1, F(f0["Mi"]["exact"][0])), jr_agree("preload initial x-moment at j", -1, F(f0["Mj"]["exact"][0])),
       jr_agree("preload initial energy", "0.075", F(outC0["energy"]["exact"]))]
jr_rows += jr
add_case("U3-COUPLED-H-SCALE-PRELOAD", 4, "JR coupled H (H00 = 4, H03 = 1, H33 = 9 at Ls = 2 m; PSD, rank 2): energy and actions; congruent rescaling to Ls = 1 m; prestressed installed state at d = 0",
         {"coupled": connector_inputs(ORIGIN, xc, z3, z3, I3, KcS, q0, Fr(2)), "rescaled_H_upper_triangle_Ls_1m": exv(upper21(H1)),
          "mm_spelling": "translation_scale {value 2000, unit mm} normalizes exactly to 2 m; H (N*m) is unchanged; a q_ref translation of 1 mm equals 0.001 m",
          "preload_q_ref": exv(qrefC)},
         {"coupled": outC, "qhat": exv(qhat), "preload_installed_d0": outC0,
          "installed_meaning": "at d = 0: internal action f = B^T g with g = -K q_ref; the assembly RHS is +B^T K q_ref = -f; with both nodes fully held the support-on-element reactions equal f"},
         criterion=UNIT, jr=jr,
         disc=[{"what": "q_ref read as stress-free (initial residual omitted)", "f_at_d0": "all zero"},
               {"what": "fixed 1 m used instead of the authored Ls = 2 m (H read as K)", "energy_J": ex(dot(vec("0.2", 0, 0, "0.1", 0, 0), mv(Hc, vec("0.2", 0, 0, "0.1", 0, 0))) / 2)}])

# preload relief with a PD K: node i held, node j free -> q = q_ref, g = 0, reactions 0;
# node j held axially only -> partial relief
KpP = [list(r) for r in KcS]
for k, v in ((1, 2), (2, 3), (4, 5), (5, 6)):
    KpP[k][k] = F(v)
check("preload PD K is PD", is_pd(KpP))
B0 = B_closed(z3, z3, xc, I3)
Ke0 = mm(mm(tr(B0), KpP), B0)
rhs0 = mv(tr(B0), mv(KpP, qrefC))
free = list(range(6, 12))
dj = solve([[Ke0[i][j] for j in free] for i in free], [rhs0[i] for i in free])
dfull = [Fr(0)] * 6 + dj
qP = mv(B0, dfull)
gP = mv(KpP, vsub(qP, qrefC))
check("preload relief: q = q_ref, g = 0", qP == qrefC and gP == [0] * 6)
reacP = vsub(mv(Ke0, dfull), rhs0)
check("preload relief: all reactions 0", reacP == [0] * 12)
free2 = [7, 8, 9, 10, 11]  # u_j_x held
dj2 = solve([[Ke0[i][j] for j in free2] for i in free2], [rhs0[i] for i in free2])
d2 = [Fr(0)] * 12
for k, v in zip(free2, dj2):
    d2[k] = v
q2 = mv(B0, d2)
g2 = mv(KpP, vsub(q2, qrefC))
reac2 = vsub(mv(Ke0, d2), rhs0)
check("preload partial: reaction sums zero", vadd(reac2[0:3], reac2[6:9]) == [0] * 3)
add_case("U3-PRELOAD-RELIEF", 4, "Prestress through q_ref on a PD K (coupled 4/1/9 block plus K11 = 2, K22 = 3, K44 = 5, K55 = 6, physical units), L = 2 m, Q = I: the +B^T K q_ref load and the recovered g after solving",
         {**connector_inputs(ORIGIN, xc, z3, z3, I3, KpP, qrefC, Fr(2)), "applied_external_load": "none"},
         {"installed_rhs_BT_K_qref": exv(rhs0),
          "node_i_anchored_node_j_free": {"d": exv(dfull), "q": exv(qP), "g_recovered": exv(gP), "energy": "0", "support_on_element_reactions": exv(reacP)},
          "node_i_anchored_node_j_ux_held": {"d": exv(d2), "q": exv(q2), "g_recovered": exv(g2), "energy": ex(dot(vsub(q2, qrefC), g2) / 2),
                                              "support_on_element_reactions": exv(reac2)},
          "both_nodes_held": {"g_recovered": exv(vscale(-1, mv(KpP, qrefC))), "support_on_element_reactions": exv(vscale(-1, rhs0))}},
         criterion=UNIT,
         disc=[{"what": "+B^T K q_ref omitted from the RHS: node j free gives d = 0, q = 0, g = -K q_ref", "g": exv(vscale(-1, mv(KpP, qrefC)))}])

# reversal of node order
J = [[F(-1), 0, 0], [0, F(1), 0], [0, 0, F(-1)]]
J = [[F(x) for x in r] for r in J]
Tm = zeros(6, 6)
for i in range(3):
    Tm[i][i] = -J[i][i]
    Tm[3 + i][3 + i] = -J[i][i]
QSr = mm(QS, J)
KSr = mm(mm(Tm, KS), tr(Tm))
qrefSr = mv(Tm, qrefS)
dSr = dS[6:12] + dS[0:6]
outRev, _ = evaluate("U3-REVERSAL", xjS, xiS, ajS, aiS, QSr, KSr, qrefSr, dSr, origin=oS)
check("reversal: q' = T q", [F(x) for x in outRev["q"]["exact"]] == mv(Tm, [F(x) for x in outS["q"]["exact"]]))
check("reversal: g' = T g", [F(x) for x in outRev["g"]["exact"]] == mv(Tm, [F(x) for x in outS["g"]["exact"]]))
check("reversal: energy unchanged", outRev["energy"] == outS["energy"])
ea, eb = outS["end_actions_node_on_element"], outRev["end_actions_node_on_element"]
check("reversal: end-action blocks exchanged", eb["Fi"] == ea["Fj"] and eb["Mi"] == ea["Mj"] and eb["Fj"] == ea["Fi"] and eb["Mj"] == ea["Mi"])
HSr = mm(mm(Tm, H_from_K(KS, LsS)), tr(Tm))
check("reversal: H' = T H T^T at unchanged Ls", HSr == H_from_K(KSr, LsS))
swapped_only, _ = evaluate("U3-REVERSAL-ID-SWAP-ONLY", xjS, xiS, ajS, aiS, mm(QS, J), KS, qrefS, dSr)
add_case("U3-REVERSAL", 4, "Canonical node-order reversal of U3-GENERIC: J = diag(-1,1,-1), Q' = QJ, attachments swapped, T = blockdiag(-J,-J), q' = Tq, q_ref' = T q_ref, K' = T K T^T",
         {"T": exm(Tm), **connector_inputs(xjS, xiS, ajS, aiS, QSr, KSr, qrefSr, LsS)}, outRev, criterion=UNIT,
         disc=[{"what": "node IDs swapped with Q' = QJ but K and q_ref not transformed", "energy": swapped_only["energy"],
                "Fj": swapped_only["end_actions_node_on_element"]["Fj"]}])

# =====================================================================================
# Item 5: finite rotation negative control
# =====================================================================================


def dsin(x):
    s, term, k = Decimal(0), x, 1
    while abs(term) > Decimal(10) ** -150:
        s += term
        term = -term * x * x / ((2 * k) * (2 * k + 1))
        k += 1
    return s


def dcos(x):
    s, term, k = Decimal(0), Decimal(1), 1
    while abs(term) > Decimal(10) ** -150:
        s += term
        term = -term * x * x / ((2 * k - 1) * (2 * k))
        k += 1
    return s


fin = []
for Lv, ph in (("2", "0.1"), ("0.3", "0.01"), ("2", "0.5")):
    Ld, pd = Decimal(Lv), Decimal(ph)
    qx = Ld * (dcos(pd) - 1)
    qy = Ld * (dsin(pd) - pd)
    ujx, ujy = Ld * (dcos(pd) - 1), Ld * dsin(pd)
    fin.append({"L_m": Lv, "phi_rad": ph, "d": {"u_i": ["0", "0", "0"], "theta_i": ["0", "0", ph],
                "u_j": [format(ujx, ".40g"), format(ujy, ".40g"), "0"], "theta_j": ["0", "0", ph]},
                "qt": [format(qx, ".40g"), format(qy, ".40g"), "0"], "qr": ["0", "0", "0"],
                "leading_axial_term_minus_L_phi2_over_2": format(-Ld * pd * pd / 2, ".40g")})
dx_ = abs(Decimal(fin[0]["qt"][0]) - Decimal("-0.00999166944394836"))
dy_ = abs(Decimal(fin[0]["qt"][1]) - Decimal("-0.000333166706343702"))
check("finite rotation L=2 phi=0.1: refutation's decimals agree to 13 significant digits (|diff| < 2e-16 m)", dx_ < Decimal("2e-16") and dy_ < Decimal("2e-16"))
check("finite rotation L=2 phi=0.1: refutation's decimals are NOT the exact values (binary64 cancellation in cos-1 and sin-phi)", dx_ > Decimal("1e-17") and dy_ > Decimal("1e-18"))
jr_rows.append({"quantity": "finite rotation qt at L = 2, phi = 0.1", "jr_stated": "[-0.00999166944394836, -0.000333166706343702, 0] (refutation section 1, 'approximately')",
                "derived": fin[0]["qt"], "agrees": True,
                "note": "agrees to 13 significant digits only: the stated decimals carry binary64 cancellation error (differences " + format(dx_, ".2e") + " and " + format(dy_, ".2e") + " m); the exact values are frozen here"})
add_case("U3-FINITE-ROTATION-NEGATIVE", 5, "A true finite common rotation Rz(phi) encoded as nodal displacement (R - I)x and nodal rotation vectors phi e_z (no offsets, Q = I, r = L e_x): the small-rotation connector reports qt = [L(cos phi - 1), L(sin phi - phi), 0], qr = 0. It is intentionally nonzero",
         {"Q": exm(I3), "offsets": "0", "x_i": ["0", "0", "0"], "x_j": "L e_x"}, {"rows": fin},
         criterion="|obs - exp| <= 1e-12 * L per component (the d inputs come from binary64 cos/sin; the qt_y cancellation loses about 3 digits); negative assertion: the product must report |qt_x| >= 0.99 * L*phi^2/2 * (1 - phi^2/12) and must not report q = 0 or suppress the output",
         disc=[{"what": "finite objectivity claimed (q forced to zero, or an evolving Q spliced into constant B)", "qt": ["0", "0", "0"]}])

# =====================================================================================
# Item 7 (connector-level): raw-difference analytical values
# =====================================================================================
Kraw = Kdiag([1, 20, 1, 1, 1, 1])
draw = dvec(ti=vec(0, 0, "0.01"), uj=vec(0, "0.02", 0), tj=vec(0, 0, "0.01"))
outRaw, _ = evaluate("U3-RAW-HISTORICAL", ORIGIN, vec(2, 0, 0), z3, z3, I3, Kraw, q0, draw)
qraw2 = vsub(draw[6:9], draw[0:3]) + vsub(draw[9:12], draw[3:6])
graw2 = mv(Kraw, qraw2)
jr_rows += [jr_agree("historical raw mutation g_y", "0.4", graw2[1]), jr_agree("historical raw mutation energy", "0.004", dot(qraw2, graw2) / 2),
            jr_agree("historical raw mutation correct energy", 0, F(outRaw["energy"]["exact"]))]
add_case("U3-RAW-DIFFERENCE-NEGATIVE", 7, "Raw endpoint differences (the deleted element's measure) under first-order rigid rotation: analytical values the product must never produce",
         {"cases": ["U3-J1-COMMON-ROTATION (240 N, 0.36 J)", "historical: L = 2, omega_z = 0.01, u_j_y = 0.02, Kyy = 20"]},
         {"historical_correct": outRaw, "historical_raw": {"q_y_m": ex(qraw2[1]), "g_y_N": ex(graw2[1]), "energy_J": ex(dot(qraw2, graw2) / 2)}},
         criterion="the correct values are exact zeros (zero-scale floors: 0.02 m, 0.4 N, 0.004 J); negative assertions at relative 1e-9",
         disc=[{"what": "J1 raw difference", "q_y_m": "0.003", "g_y_N": "240", "energy_J": "0.36"},
               {"what": "historical raw difference", "q_y_m": "0.02", "g_y_N": "0.4", "energy_J": "0.004"}])

# =====================================================================================
# Item 6: system case through PP (re-authored invented demo)
# =====================================================================================


def machin_pi(prec):
    getcontext().prec = prec + 20

    def arctan_inv(n):
        x = Decimal(1) / n
        x2 = x * x
        s, term, k = Decimal(0), x, 0
        while term != 0:
            s += term / (2 * k + 1) if k % 2 == 0 else -term / (2 * k + 1)
            term *= x2
            k += 1
            if term < Decimal(10) ** -(prec + 15):
                break
        return s
    p = 4 * (4 * arctan_inv(5) - arctan_inv(239))
    getcontext().prec = 160
    return p


PI_DEC = machin_pi(130)
PI = Fr(PI_DEC.quantize(Decimal(10) ** -125))
check("pi (Machin, 125 decimals) begins 3.14159265358979323846264338327950288", str(PI_DEC).startswith("3.14159265358979323846264338327950288419716939937510"))

E_ = F("2e11")
NU = F("0.3")
G_ = E_ / (2 * (1 + NU))
ALPHA = F("1.2e-5")
OD, WALL = F("0.168"), F("0.007")
ro, ri = OD / 2, OD / 2 - WALL
As_c = WALL * (OD - WALL)  # As / pi
I_c = As_c * (ro * ro + ri * ri) / 4  # I / pi
NODES = {"node:N-100": vec(0, 0, 0), "node:N-110": vec("3.2", 0, 0), "node:N-120": vec("3.2", "2.4", 0),
         "node:N-130": vec("7.6", "2.4", 0), "node:N-140": vec("7.6", "2.4", "2.2")}
NID = list(NODES)
PIPES = {"pipe:P-100": ("node:N-100", "node:N-110", vec(0, 0, 1)), "pipe:P-110": ("node:N-110", "node:N-120", vec(0, 0, 1)),
         "pipe:P-120": ("node:N-120", "node:N-130", vec(0, 0, 1)), "pipe:P-130": ("node:N-130", "node:N-140", vec(0, 1, 0))}
K_SPRING = F(42000)
KDEMO = Kdiag([3200000, 900000, 900000, 620000, 480000, 480000])
LS_DEMO = Fr(1, 2)
QD = [[F(0), F(0), F(-1)], [F(0), F(1), F(0)], [F(1), F(0), F(0)]]  # columns: e_z, e_y, -e_x


def sqrt_fr(x):
    # exact for perfect squares used here (axis-aligned members)
    n, d_ = x.numerator, x.denominator
    from math import isqrt
    rn, rd = isqrt(n), isqrt(d_)
    assert rn * rn == n and rd * rd == d_
    return Fr(rn, rd)


def frame_local_k(Ea, Gg, A, Iy, Iz, Jt, L):
    k = zeros(12, 12)
    a, tq = Ea * A / L, Gg * Jt / L

    def sym(i, j, v):
        k[i][j] = v
        k[j][i] = v
    sym(0, 0, a); sym(6, 6, a); sym(0, 6, -a)
    sym(3, 3, tq); sym(9, 9, tq); sym(3, 9, -tq)
    z12, z6, z4, z2 = 12 * Ea * Iz / L ** 3, 6 * Ea * Iz / L ** 2, 4 * Ea * Iz / L, 2 * Ea * Iz / L
    sym(1, 1, z12); sym(1, 5, z6); sym(1, 7, -z12); sym(1, 11, z6)
    sym(5, 5, z4); sym(5, 7, -z6); sym(5, 11, z2)
    sym(7, 7, z12); sym(7, 11, -z6); sym(11, 11, z4)
    y12, y6, y4, y2 = 12 * Ea * Iy / L ** 3, 6 * Ea * Iy / L ** 2, 4 * Ea * Iy / L, 2 * Ea * Iy / L
    sym(2, 2, y12); sym(2, 4, -y6); sym(2, 8, -y12); sym(2, 10, -y6)
    sym(4, 4, y4); sym(4, 8, y6); sym(4, 10, y2)
    sym(8, 8, y12); sym(8, 10, y6); sym(10, 10, y4)
    return k


def frame_axes(xi, xj, yref):
    dx = vsub(xj, xi)
    L = sqrt_fr(dot(dx, dx))
    ex_ = vscale(1 / L, dx)
    yc = vsub(yref, vscale(dot(yref, ex_), ex_))
    ny = sqrt_fr(dot(yc, yc))
    ey = vscale(1 / ny, yc)
    ez = cross(ex_, ey)
    return L, [ex_, ey, ez]


def blockT(Rm):
    T = zeros(12, 12)
    for b_ in range(4):
        for i in range(3):
            for j in range(3):
                T[3 * b_ + i][3 * b_ + j] = Rm[i][j]
    return T


def frame_global(pid, pi_val):
    a, bnode, yref = PIPES[pid]
    L, Rm = frame_axes(NODES[a], NODES[bnode], yref)
    A, I_ = As_c * pi_val, I_c * pi_val
    k = frame_local_k(E_, G_, A, I_, I_, 2 * I_, L)
    T = blockT(Rm)
    return mm(mm(tr(T), k), T), T, L, Rm


# frame self-tests (sign conventions): rigid modes in the null space; cantilever closed forms
def frame_selftest():
    pi_val = Fr(3)  # any positive value
    Kg, T, L, Rm = frame_global("pipe:P-110", pi_val)
    xi, xj = NODES["node:N-110"], NODES["node:N-120"]
    for name, m in rigid_modes(xi, xj, vec(1, -2, 3)):
        check(f"frame self-test: P-110 rigid {name} in null(K)", mv(Kg, m) == [0] * 12)
    # cantilever along x (P-100), fixed at i, tip load P in z: w = P L^3 / (3 E I); tip moment My: theta_y = M L/(E I)
    Kg, T, L, Rm = frame_global("pipe:P-100", pi_val)
    EI = E_ * I_c * pi_val
    fr_ = list(range(6, 12))
    Kff = [[Kg[i][j] for j in fr_] for i in fr_]
    u = solve(Kff, [0, 0, 1, 0, 0, 0])
    check("frame self-test: cantilever tip z under unit z load = L^3/(3EI)", u[2] == L ** 3 / (3 * EI))
    check("frame self-test: cantilever tip rotation y under unit z load = -L^2/(2EI)", u[4] == -L ** 2 / (2 * EI))
    # uniform load w (global z) consistent vector: tip deflection w L^4/(8EI)
    w = F(-190)
    fl = consistent_uniform(Rm, L, vec(0, 0, w))
    u = solve(Kff, fl[6:12])
    check("frame self-test: consistent uniform load tip deflection = w L^4/(8EI)", u[2] == w * L ** 4 / (8 * EI))
    check("frame self-test: consistent uniform load tip rotation = -w L^3/(6EI)", u[4] == -w * L ** 3 / (6 * EI))


def consistent_uniform(Rm, L, wg):
    wl = mv(Rm, wg)
    fl = [Fr(0)] * 12
    fl[0] = fl[6] = wl[0] * L / 2
    fl[1] = fl[7] = wl[1] * L / 2
    fl[5], fl[11] = wl[1] * L * L / 12, -wl[1] * L * L / 12
    fl[2] = fl[8] = wl[2] * L / 2
    fl[4], fl[10] = -wl[2] * L * L / 12, wl[2] * L * L / 12
    return mv(tr(blockT(Rm)), fl)


frame_selftest()

NDOF = 30
RESTR = {"support:S-100": ("node:N-100", [0, 1, 2, 3, 4, 5]), "support:S-120": ("node:N-120", [0, 2]), "support:S-130": ("node:N-130", [1])}
SPRING = ("support:SH-140", "node:N-140", 2)
CASES = {
    "load:L-100": {"weight": [("pipe:P-120", vec(0, 0, -190))], "nodal": [("node:N-140", vec(0, 350, 0))], "thermal": [("pipe:P-120", F("12.5"))]},
    "load:L-200": {"weight": [("pipe:P-120", vec(0, 0, -95))], "nodal": [("node:N-140", vec(0, 125, 0))], "thermal": []},
    "load:L-300": {"weight": [], "nodal": [("node:N-140", vec(200, 0, 0))], "thermal": [], "moments": [("node:N-140", vec(0, -80, 150))]},
}


def dofs(nid):
    b_ = 6 * NID.index(nid)
    return list(range(b_, b_ + 6))


def connector_for_demo(kind):
    xi, xj = NODES["node:N-130"], NODES["node:N-140"]
    r = vsub(xj, xi)
    if kind == "raw":
        QT = tr(QD)
        I3_, Z3 = eye(3), zeros(3, 3)
        Bm = mm(QT, hcat(mscale(-1, I3_), Z3, I3_, Z3)) + mm(QT, hcat(Z3, mscale(-1, I3_), Z3, I3_))
    else:
        Bm = B_closed(z3, z3, r, QD)
    return Bm


def solve_demo(case_id, pi_val, variant="connector"):
    Kg = zeros(NDOF, NDOF)
    f = [Fr(0)] * NDOF
    loads = CASES[case_id]
    for pid in ("pipe:P-100", "pipe:P-110", "pipe:P-120") + (("pipe:P-130",) if variant == "parallel" else ()):
        Ke_, T, L, Rm = frame_global(pid, pi_val)
        idx = dofs(PIPES[pid][0]) + dofs(PIPES[pid][1])
        for a_, ia in enumerate(idx):
            for b_, ib in enumerate(idx):
                Kg[ia][ib] += Ke_[a_][b_]
    Bm = connector_for_demo("raw" if variant == "raw" else "objective")
    Kc = mm(mm(tr(Bm), KDEMO), Bm)
    idx = dofs("node:N-130") + dofs("node:N-140")
    for a_, ia in enumerate(idx):
        for b_, ib in enumerate(idx):
            Kg[ia][ib] += Kc[a_][b_]
    sdof = dofs(SPRING[1])[SPRING[2]]
    Kg[sdof][sdof] += K_SPRING
    for pid, wg in loads["weight"]:
        _, T, L, Rm = frame_global(pid, pi_val)
        fl = consistent_uniform(Rm, L, wg)
        idx = dofs(PIPES[pid][0]) + dofs(PIPES[pid][1])
        for a_, ia in enumerate(idx):
            f[ia] += fl[a_]
    for nid, fv in loads["nodal"]:
        for k in range(3):
            f[dofs(nid)[k]] += fv[k]
    for nid, mvv in loads.get("moments", []):
        for k in range(3):
            f[dofs(nid)[3 + k]] += mvv[k]
    for pid, dT in loads["thermal"]:
        _, T, L, Rm = frame_global(pid, pi_val)
        NT = E_ * As_c * pi_val * ALPHA * dT
        for k in range(3):
            f[dofs(PIPES[pid][0])[k]] -= NT * Rm[0][k]
            f[dofs(PIPES[pid][1])[k]] += NT * Rm[0][k]
    restrained = sorted(sum(([dofs(n)[k] for k in ks] for n, ks in RESTR.values()), []))
    free = [i for i in range(NDOF) if i not in restrained]
    df = solve([[Kg[i][j] for j in free] for i in free], [f[i] for i in free])
    dfull = [Fr(0)] * NDOF
    for k, v in zip(free, df):
        dfull[k] = v
    Kd = mv(Kg, dfull)
    resid = [Kd[i] - f[i] for i in range(NDOF)]
    # spring internal force is inside Kg: its support-on-pipe reaction is -k u
    reactions = {}
    for sid, (nid, ks) in RESTR.items():
        reactions[sid] = [resid[dofs(nid)[k]] if k in ks else Fr(0) for k in range(6)]
    us = dfull[sdof]
    reactions[SPRING[0]] = [Fr(0), Fr(0), -K_SPRING * us, Fr(0), Fr(0), Fr(0)]
    # residual at free DOFs is zero; reactions at restrained DOFs include the spring? (no: spring DOF is free)
    check(f"{case_id}/{variant}: residual zero at free DOFs", all(resid[i] == 0 for i in free))
    # global balance of applied loads + reactions (support-on-pipe), reference geometry, origin 0
    Ftot, Mtot = [Fr(0)] * 3, [Fr(0)] * 3
    for nid in NID:
        x = NODES[nid]
        fn = f[dofs(nid)[0]:dofs(nid)[0] + 3]
        mn = f[dofs(nid)[3]:dofs(nid)[3] + 3]
        Ftot = vadd(Ftot, fn)
        Mtot = vadd(Mtot, vadd(cross(x, fn), mn))
    for sid, rv in reactions.items():
        nid = RESTR[sid][0] if sid in RESTR else SPRING[1]
        x = NODES[nid]
        Ftot = vadd(Ftot, rv[0:3])
        Mtot = vadd(Mtot, vadd(cross(x, rv[0:3]), rv[3:6]))
    res = {"d": dfull, "reactions": reactions, "f": f, "Ftot": Ftot, "Mtot": Mtot}
    dc = [dfull[i] for i in dofs("node:N-130") + dofs("node:N-140")]
    qc = mv(Bm, dc)
    gc = mv(KDEMO, qc)
    fc = mv(tr(Bm), gc)
    res.update({"q": qc, "g": gc, "U": dot(qc, gc) / 2, "fc": fc})
    return res


def sysfmt(x, sig=30):
    return dec(x, sig)


sys_out = {}
sys_disc = {}
for cid in CASES:
    res = solve_demo(cid, PI)
    res2 = solve_demo(cid, Fr(PI_DEC.quantize(Decimal(10) ** -115)))
    agree = all(abs(a - b_) <= Fr(1, 10 ** 95) * max(1, abs(a)) for a, b_ in zip(res["d"], res2["d"]))
    check(f"{cid}: solution stable to 95 digits under pi truncated at 115 vs 125 decimals", agree)
    check(f"{cid}: global force balance (applied + reactions) = 0 to 1e-100", max(abs(x) for x in res["Ftot"]) < Fr(1, 10 ** 100))
    check(f"{cid}: global moment balance about origin = 0 to 1e-100", max(abs(x) for x in res["Mtot"]) < Fr(1, 10 ** 100))
    xi, xj = NODES["node:N-130"], NODES["node:N-140"]
    Fv, Mv, fcc = actions_closed(res["g"], z3, z3, vsub(xj, xi), QD)
    check(f"{cid}: connector B^T g equals closed-form blocks", res["fc"] == fcc)
    fc = res["fc"]
    check(f"{cid}: connector end actions self-equilibrated", vadd(fc[0:3], fc[6:9]) == [0] * 3 and vadd(vadd(vadd(cross(xi, fc[0:3]), fc[3:6]), cross(xj, fc[6:9])), fc[9:12]) == [0] * 3)
    disp = {nid: {"u_m": [sysfmt(x) for x in res["d"][dofs(nid)[0]:dofs(nid)[0] + 3]],
                  "theta_rad": [sysfmt(x) for x in res["d"][dofs(nid)[3]:dofs(nid)[3] + 3]]} for nid in NID}
    reac = {sid: [sysfmt(x) for x in rv] for sid, rv in res["reactions"].items()}
    Fs = max(max(abs(x) for x in rv[0:3]) for rv in res["reactions"].values())
    Ms = max(max(abs(x) for x in rv[3:6]) for rv in res["reactions"].values())
    us = max(abs(x) for nid in NID for x in res["d"][dofs(nid)[0]:dofs(nid)[0] + 3])
    ts = max(abs(x) for nid in NID for x in res["d"][dofs(nid)[3]:dofs(nid)[3] + 3])
    qts, qrs = max(abs(x) for x in res["q"][0:3]), max(abs(x) for x in res["q"][3:6])
    gts, grs = max(abs(x) for x in res["g"][0:3]), max(abs(x) for x in res["g"][3:6])
    applied_F = sum(abs(x) for x in res["f"] if True)  # informational
    sys_out[cid] = {
        "displacements": disp,
        "reactions_support_on_pipe_global_Fx_Fy_Fz_Mx_My_Mz": reac,
        "connector": {"q_local": [sysfmt(x) for x in res["q"]], "g_local": [sysfmt(x) for x in res["g"]], "energy_J": sysfmt(res["U"]),
                      "F_global": [sysfmt(x) for x in Fv], "M_global": [sysfmt(x) for x in Mv],
                      "end_actions_node_on_element": {"Fi_at_N-130": [sysfmt(x) for x in fc[0:3]], "Mi_at_N-130": [sysfmt(x) for x in fc[3:6]],
                                                      "Fj_at_N-140": [sysfmt(x) for x in fc[6:9]], "Mj_at_N-140": [sysfmt(x) for x in fc[9:12]]}},
        "global_balance": {"sum_applied_plus_reactions_force_N": "0 (exact; reference residual < 1e-100)", "sum_moment_about_origin_N_m": "0 (exact; reference residual < 1e-100)"},
        "zero_scale_floors": {"force_N": sysfmt(Fs, 20), "moment_N_m": sysfmt(Ms, 20), "translation_m": sysfmt(us, 20), "rotation_rad": sysfmt(ts, 20),
                              "q_translation_m": sysfmt(qts, 20), "q_rotation_rad": sysfmt(qrs, 20), "g_force_N": sysfmt(gts, 20), "g_moment_N_m": sysfmt(grs, 20),
                              "rule": "|obs - exp| <= 1e-9 * max(|exp|, floor of the value's family); floors are the case's largest magnitude in that family"},
    }
    raw = solve_demo(cid, PI, "raw")
    par = solve_demo(cid, PI, "parallel")
    check(f"{cid}: parallel retained span changes the reactions", any(abs(a - b_) > Fr(1, 10 ** 6) * max(Fs, 1) for sid in res["reactions"] for a, b_ in zip(res["reactions"][sid], par["reactions"][sid])))
    check(f"{cid}: raw-difference element leaves a global moment imbalance", max(abs(x) for x in raw["Mtot"]) > 1)
    xi = NODES["node:N-130"]
    xj = NODES["node:N-140"]
    comps = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"]

    def discriminating(other):
        out_ = []
        for sid in res["reactions"]:
            for k in range(6):
                fam = Fs if k < 3 else Ms
                if abs(res["reactions"][sid][k] - other["reactions"][sid][k]) > Fr(1, 10 ** 6) * fam:
                    out_.append(f"{sid}.{comps[k]}")
        return out_
    sys_disc[cid] = {
        "use": "a discriminator is asserted only on the listed components, whose wrong values differ from the reference by more than 1e-6 of the family floor; the others coincide (statically determined)",
        "raw_difference_element_solve": {
            "what": "the connector replaced by six uncoupled relative springs on raw endpoint differences (the deleted element's measure) with the same K and Q",
            "reactions": {sid: [sysfmt(x, 20) for x in rv] for sid, rv in raw["reactions"].items()},
            "global_moment_imbalance_about_origin_N_m": [sysfmt(x, 20) for x in raw["Mtot"]],
            "global_force_imbalance_N": [sysfmt(x, 20) for x in raw["Ftot"]],
            "element_couple_r_x_Fj_N_m": [sysfmt(x, 20) for x in cross(vsub(xj, xi), raw["fc"][6:9])],
            "discriminating_components": discriminating(raw)},
        "retained_span_in_parallel_solve": {
            "what": "pipe P-130 kept in parallel with the connector (replaces_span not applied)",
            "reactions": {sid: [sysfmt(x, 20) for x in rv] for sid, rv in par["reactions"].items()},
            "connector_g_local": [sysfmt(x, 20) for x in par["g"]],
            "discriminating_components": discriminating(par)},
    }
    print(f"{cid}: connector g = {[sysfmt(x, 12) for x in res['g']]}")
    print(f"{cid}: raw-difference moment imbalance = {[sysfmt(x, 12) for x in raw['Mtot']]}")

PROV = "invented T4-I12 re-authoring of the invented demo; not library, catalog, manufacturer or code-rule data"
conn_record = {
    "version": "1.0.0", "motion_basis": "symmetric_midpoint_small_rotation_v1",
    "end_i": {"node_ref": "node:N-130", "initial_node_axes_global": [[1, 0, 0], [0, 1, 0], [0, 0, 1]], "offset_local": {"x": 0, "y": 0, "z": 0, "unit": "m"}},
    "end_j": {"node_ref": "node:N-140", "initial_node_axes_global": [[1, 0, 0], [0, 1, 0], [0, 0, 1]], "offset_local": {"x": 0, "y": 0, "z": 0, "unit": "m"}},
    "connector_axes_global": [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    "installed_reference_temperature": {"value": 20, "unit": "degC"},
    "temperature_applicability": "fixed_installed_parameters_v1",
    "reference_state": "stress_free",
    "q_ref": {"translation": {"x": 0, "y": 0, "z": 0, "unit": "m"}, "rotation": {"x": 0, "y": 0, "z": 0, "unit": "rad"}},
    "stiffness": {"version": "1.0.0", "representation": "scaled_work_coefficients_v1", "translation_scale": {"value": 0.5, "unit": "m"},
                  "rotation_scale": {"value": 1, "unit": "rad"}, "coefficient_unit": "N*m",
                  "upper_triangle": [int(x) for x in upper21(H_from_K(KDEMO, LS_DEMO))],
                  "coordinate_order": ["tx", "ty", "tz", "rx", "ry", "rz"],
                  "provenance": {"source_reference": "invented demo C-150 four rates, explicitly authored as an uncoupled isotropic 6x6 (T4-I12); not a migration",
                                 "measurement_restraints": "invented; symmetric midpoint basis declared by the author", "basis_transform_reference": "none (authored directly in the midpoint basis)"}},
    "topology": {"type": "replaces_span", "span_ref": "pipe:P-130"},
    "calibration": {"kind": "constant_structural_elasticity_v1", "includes_pressure_dependent_tangent": False, "installed_geometry": "authored model geometry"},
    "hardware": {"kind": "untied"},
    "pressure_model": {"kind": "unpressurized"},
    "provenance": {"source_reference": PROV, "measurement_restraints": "invented", "basis_transform_reference": "none", "validity_statement": "invented analytical control; no applicability claim"},
}


def demo_doc(extra_loads=None, regions=None, conn=True, annotation=False):
    mat = {"id": "material:invented-carbon-steel", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
           "elastic_modulus": {"value": 200000000000.0, "unit": "Pa"}, "poisson_ratio": {"value": 0.3, "unit": "1"},
           "thermal_expansion_coefficient": {"value": 1.2e-05, "unit": "1/degC"}, "provenance": PROV}
    pipes = []
    for pid, (a, b_, yref) in PIPES.items():
        pipes.append({"id": pid, "from": a, "to": b_, "section": {"outside_diameter": {"value": 0.168, "unit": "m"}, "wall_thickness": {"value": 0.007, "unit": "m"}},
                      "material": "material:invented-carbon-steel", "y_reference": {"x": float(yref[0]), "y": float(yref[1]), "z": float(yref[2])}, "provenance": PROV})
    nodes = [{"id": n, "position": {"x": float(v[0]), "y": float(v[1]), "z": float(v[2])}, "provenance": PROV} for n, v in NODES.items()]
    supports = [
        {"id": "support:S-100", "node": "node:N-100", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"], "provenance": PROV},
        {"id": "support:S-120", "node": "node:N-120", "family": "guide", "restraints": ["UX", "UZ"], "provenance": PROV},
        {"id": "support:S-130", "node": "node:N-130", "family": "guide", "restraints": ["UY"], "provenance": PROV},
        {"id": "support:SH-140", "node": "node:N-140", "family": "variable_spring_hanger", "restraints": [],
         "hanger": {"hanger_type": "variable_spring_hanger", "stiffness": {"dof": "UZ", "value": {"value": 42000, "unit": "N/m"}},
                    "manufacturer_reference": "invented_user_entered_variable_hanger_reference_no_catalog",
                    "source_reference": "invented_user_entered_spring_hanger_values_no_catalog",
                    "mechanics_consumption": "linear_spring_primitive_user_stiffness"}, "provenance": PROV}]
    comp = {"id": "component:C-150", "label": "Invented expansion joint (objective connector)", "kind": "expansion_joint", "node": "node:N-140", "provenance": PROV}
    if conn:
        comp["objective_connector"] = conn_record
    if annotation:
        comp["geometry"] = {"expansion_joint_pipe_ref": "pipe:P-130"}
        comp["mechanics_interface"] = {"solver_consumption": "not_solver_consumed", "rule_check_consumption": "user_rule_pack_inputs_only"}
    def load(i, cat, target, direction, value, unit, dim):
        return {"id": i, "category": cat, "target": target, "direction": direction, "magnitude": {"value": value, "unit": unit}, "dimension": dim, "provenance": PROV}
    cases_ = [
        {"id": "load:L-100", "primitive_loads": [
            load("load:L-100-Z", "weight", {"type": "element", "pipe": "pipe:P-120"}, "global_z", -190.0, "N/m", "force_per_length"),
            load("load:L-100-Y", "occasional", {"type": "node", "node": "node:N-140"}, "global_y", 350.0, "N", "force"),
            load("load:L-100-T", "thermal", {"type": "element", "pipe": "pipe:P-120"}, "global_z", 12.5, "degC", "temperature_interval")],
         "pressure_regions": [], "provenance": PROV},
        {"id": "load:L-200", "primitive_loads": [
            load("load:L-200-Z", "weight", {"type": "element", "pipe": "pipe:P-120"}, "global_z", -95.0, "N/m", "force_per_length"),
            load("load:L-200-Y", "occasional", {"type": "node", "node": "node:N-140"}, "global_y", 125.0, "N", "force")],
         "pressure_regions": [], "provenance": PROV},
        {"id": "load:L-300", "primitive_loads": [
            load("load:L-300-X", "occasional", {"type": "node", "node": "node:N-140"}, "global_x", 200.0, "N", "force"),
            load("load:L-300-MY", "occasional", {"type": "node", "node": "node:N-140"}, "rotation_y", -80.0, "N*m", "moment"),
            load("load:L-300-MZ", "occasional", {"type": "node", "node": "node:N-140"}, "rotation_z", 150.0, "N*m", "moment")],
         "pressure_regions": [], "provenance": "T4-I12 coverage case added to the re-authored demo (not in the original demo): exercises the connector's z-lateral, y-bending and torsion coordinates"}]
    if extra_loads:
        cases_[0]["primitive_loads"].append(extra_loads)
    if regions is not None:
        cases_[0]["pressure_regions"] = regions
    model = {"schema_version": "0.3.0", "document_kind": "openpipestress.product_preview.model",
             "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3"},
             "project": {"id": "project:invented-loop-01-v3-connector", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
             "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
             "nodes": nodes, "pipe_segments": pipes, "supports": supports, "components": [comp], "materials": [mat], "load_cases": cases_, "combinations": []}
    return {"model": model, "materials": []}


demo = demo_doc()
refusals = {
    "replaced_span_weight": {"document_patch": "load:L-100 gains a weight primitive on pipe:P-130",
                             "added_load": {"id": "load:L-100-Z-130", "category": "weight", "target": {"type": "element", "pipe": "pipe:P-130"}, "direction": "global_x",
                                            "magnitude": {"value": -150.0, "unit": "N/m"}, "dimension": "force_per_length", "provenance": PROV},
                             "expected": {"status_mechanics": "MODEL_INCOMPLETE", "results": "none", "blocking_code": "JOINT_REPLACED_SPAN_LOAD_UNOWNED",
                                          "affected_refs_include": ["component:C-150", "pipe:P-130", "load:L-100-Z-130"], "fallback": "none (never moved to the end nodes, never dropped)"}},
    "replaced_span_thermal": {"document_patch": "load:L-100 gains a thermal primitive on pipe:P-130",
                              "added_load": {"id": "load:L-100-T-130", "category": "thermal", "target": {"type": "element", "pipe": "pipe:P-130"}, "direction": "global_z",
                                             "magnitude": {"value": 12.5, "unit": "degC"}, "dimension": "temperature_interval", "provenance": PROV},
                              "expected": {"status_mechanics": "MODEL_INCOMPLETE", "results": "none", "blocking_code": "JOINT_REPLACED_SPAN_LOAD_UNOWNED",
                                           "affected_refs_include": ["component:C-150", "pipe:P-130", "load:L-100-T-130"], "fallback": "none (the removed pipe's alpha never becomes a joint thermal law)"}},
    "joint_case_with_pressure_region": {"document_patch": "load:L-100 gains a pressure region over pipe:P-120 (any non-empty pressure_regions)",
                                        "expected": {"status_mechanics": "MODEL_INCOMPLETE", "blocking_code": "JOINT_PRESSURE_INTERFACE_UNRESOLVED (until T4-U5)"}},
    "annotation_only_joint_on_exact_route": {"document_patch": "component:C-150 without objective_connector, with expansion_joint_pipe_ref pipe:P-130 and solver_consumption not_solver_consumed",
                                             "expected": {"status_mechanics": "MODEL_INCOMPLETE", "results": "none",
                                                          "rule": "ruling: annotation-only joints are not admitted on the exact route; never analysed as pipe P-130 there; the blocking code is T4-U3's choice"}},
    "series_or_parallel_topology": {"expected": {"blocking_code": "OBJECTIVE_CONNECTOR_TOPOLOGY_UNRESOLVED", "rule": "ruling: the connector topology is replaces_span only"}},
}
add_case("U3-SYS-DEMO-CONNECTOR-001", 6,
         "The invented demo (P/fixtures/product_preview/invented_preview_model.json) re-authored as a v3 exact document whose joint C-150 is an objective connector replacing pipe:P-130; solved through PP in both modes. The legacy element left a force couple unbalanced (658.44 N*m in the legacy demo's L-100, per PP/tests/preview_physics_runtime.rs:1036-1043@ed012c7ccf); the connector balances exactly",
         {"re_authoring": {
             "kept": "nodes, pipes P-100..P-130 (P-130 retained as the replaced record), anchor S-100, guides S-120 and S-130, variable spring hanger SH-140 (linear spring UZ 42000 N/m), weight and occasional loads of L-100 and L-200, the thermal load of L-100",
             "added": "load:L-300 (not in the demo; T4-I12 coverage): at N-140 a global-x force of 200 N and nodal moments -80 N*m about global y and 150 N*m about global z, so that the connector's z-lateral, y-bending and torsion coordinates are nonzero through PP (the demo's own cases load only q_tx, q_ty and q_rz)",
             "material": "E = 2e11 Pa, nu = 0.3 (G = E/(2(1+nu)) derived; the demo's G = 77 GPa is not used), alpha = 1.2e-5 /degC",
             "removed_and_why": {
                 "C-110 bend marker (geometry-only)": "refused on the exact route (D-2)",
                 "C-120 branch, C-130 valve, C-140 terminal markers": "components other than the connector are refused on the exact route (T7)",
                 "NL-140 one-way, NL-130-FRIC friction": "nonlinear supports refused on the exact route (T5)",
                 "CE-120 constant effort": "refused on the exact route",
                 "all pressure primitives (incl. L-100-P-EJ on P-130)": "legacy pressure primitives are retired; a joint case with pressure regions is refused until T4-U5",
                 "combination C-OPER-ALT": "exact-route combinations refused (T6)"},
             "connector": "Q columns (e_z, e_y, -e_x): x along r = N-140 - N-130 = (0, 0, 2.2) m, y = P-130's y_reference; offsets 0; K = diag(3.2e6, 9e5, 9e5 N/m; 6.2e5, 4.8e5, 4.8e5 N*m/rad) explicitly authored from the four demo rates (axial, lateral x2, torsional, angular x2); Ls = 0.5 m, so H = diag(800000, 225000, 225000, 620000, 480000, 480000) N*m; q_ref = 0, stress-free; untied; unpressurized",
             "section": "OD 0.168 m, wall 0.007 m, no mill tolerance: As = 0.001127 pi m^2, I = As (ro^2 + ri^2)/4 = 0.001127 pi * 0.012985/4 m^4, J = 2I; Euler-Bernoulli frames, consistent uniform loads, thermal pair +-E As alpha dT along local x"},
          "document_v3_0.3.0": demo, "connector_record": conn_record},
         sys_out,
         criterion="both modes; relative 1e-9 with the case's zero-scale floors (each value: |obs - exp| <= 1e-9 * max(|exp|, floor)); global balance of applied loads plus published reactions: |sum F| <= 1e-9 * force floor and |sum M about the origin| <= 1e-9 * force floor * 8.27 m (8.27 m = distance of the farthest node, N-140); displacements are published in mm (observed/1000 against the metre value)",
         extra={"refusal_variants": refusals, "numeric_basis": "pi to 125 decimals from Machin's formula, as an exact rational; the rest exact. Values are rational functions of pi, given to 30 significant digits (checked stable to 95 digits)"},
         disc=sys_disc)

# =====================================================================================
# Item 8: NI's four friction tests (replacement coupling: an ordinary frame element)
# =====================================================================================


def sgn(x):
    return (x > 0) - (x < 0)


def ni_run(Kxx, Kxy, Fx, Fy, mu, seed, max_it):
    """Model of NI's active-set loop for one friction row on Ux with a derived normal on the
    restrained Uy of node 1 (NI/src/lib.rs@ed012c7ccf: 608-780 loop, 1370-1500 sliding solve,
    1560-1660 resolve, 1737-1771 branch/direction, 2348-2410 trial; nonlinear_supports
    classify 644-670). Node 1: Ux free unless sticking; Uy, Uz, Rx, Ry, Rz restrained; node 0 fixed."""
    state, prev, its = seed, None, []
    for it in range(1, max_it + 1):
        deferred = (it == 1 and state == "Sliding")
        applied, badm = None, True
        if state == "Sticking":
            u, Rx, Ry = Fr(0), -Fx, -Fy
        else:
            s = 0 if prev is None else (sgn(prev["u"]) if prev["u"] != 0 else -sgn(prev["Rx"]))
            if prev is None or s == 0:
                u = Fx / Kxx
                Rx, Ry = Fr(0), Kxy * u - Fy
            else:
                ub = Fx / Kxx
                base_Ry = Kxy * ub - Fy
                sigma = sgn(prev["Ry"]) if prev["Ry"] != 0 else sgn(base_Ry)
                infl = Kxy / Kxx
                c = s * mu * sigma
                fforce = -c * base_Ry / (1 + c * infl)
                applied = fforce if fforce != 0 else None
                u = (Fx + (fforce if applied is not None else 0)) / Kxx
                Ry = Kxy * u - Fy
                Rx = applied if applied is not None else Fr(0)
                if mu == 0 or s == 0:
                    badm = True
                elif sigma == 0:
                    badm = Ry == 0
                else:
                    badm = sigma * Ry >= 0
        normal, tang = abs(Ry), Rx
        tadm = True
        if state == "Sliding":
            if normal <= 0:
                new = "Inactive"
            elif deferred or not badm:
                new = "Sliding"
            else:
                limit = mu * normal
                if limit == 0:
                    tadm = applied is None and u != 0
                else:
                    tadm = applied is not None and u != 0 and applied * u < 0 and tang * u < 0
                new = "Sliding" if tadm else "Sticking"
        else:
            new = "Inactive" if normal <= 0 else ("Sticking" if abs(tang) <= mu * normal else "Sliding")
        converged = new == state and not deferred and tadm and badm
        its.append({"iteration": it, "state_in": state, "state_out": new, "u": u, "Rx": Rx, "Ry": Ry, "applied": applied,
                    "branch_admissible": badm, "converged": converged})
        prev = {"u": u, "Rx": Rx, "Ry": Ry}
        state = new
        if converged:
            break
    return its


def ni_fixture_checks(label, Kxx, Kxy, expect):
    mu = Fr(3, 10)
    res = {}
    for Fx, Fy in ((10, -10), (-10, -10)):
        runs = {sd: ni_run(Kxx, Kxy, F(Fx), F(Fy), mu, sd, 4) for sd in ("Sticking", "Sliding")}
        a, b_ = runs["Sticking"][-1], runs["Sliding"][-1]
        ok = all(len(r) == 2 and r[-1]["converged"] and r[-1]["state_out"] == "Sliding" for r in runs.values())
        ok &= (a["u"], a["Ry"], a["Rx"]) == (b_["u"], b_["Ry"], b_["Rx"])
        ok &= a["applied"] == a["Rx"] and abs(a["Rx"]) == mu * abs(a["Ry"])
        check(f"NI {label} test 1 ({Fx},{Fy}): 2 iterations, Sliding, both seeds agree, |friction| = 0.3|normal|", ok)
        check(f"NI {label} test 1 ({Fx},{Fy}): equilibrium Kxx u = Fx + friction and Ry = Kxy u - Fy", Kxx * a["u"] == F(Fx) + a["Rx"] and a["Ry"] == Kxy * a["u"] - F(Fy))
        res[f"test1_{Fx}_{Fy}"] = {"u": a["u"], "normal": a["Ry"], "friction": a["Rx"]}
    for Fx, Fy in ((10, -1), (-10, 1)):
        r = ni_run(Kxx, Kxy, F(Fx), F(Fy), mu, "Sticking", 4)
        sign = sgn(Fx)
        br, fin_ = r[1], r[-1]
        ok = len(r) == 3 and fin_["converged"] and fin_["state_out"] == "Sliding"
        ok &= br["state_out"] == "Sliding" and not br["branch_admissible"] and br["applied"] is not None
        ok &= fin_["applied"] * fin_["u"] < 0 and abs(fin_["Rx"]) == mu * abs(fin_["Ry"])
        capped = ni_run(Kxx, Kxy, F(Fx), F(Fy), mu, "Sticking", 2)
        ok &= len(capped) == 2 and not capped[-1]["converged"] and not capped[-1]["branch_admissible"]
        check(f"NI {label} test 2 ({Fx},{Fy}): sign flip at iteration 2, 3 iterations, cap 2 fails on the derived-normal branch", ok)
        check(f"NI {label} test 2 ({Fx},{Fy}): equilibrium of the retry and final iterates", all(Kxx * it["u"] == F(Fx) + it["Rx"] and it["Ry"] == Kxy * it["u"] - F(Fy) for it in (br, fin_)))
        res[f"test2_{Fx}_{Fy}"] = {"branch_retry_u": br["u"], "branch_retry_applied": br["applied"], "branch_retry_Rx": br["Rx"], "branch_retry_Ry": br["Ry"],
                                   "final_u": fin_["u"], "final_friction": fin_["Rx"], "final_normal": fin_["Ry"], "sign": sign}
    r = ni_run(Kxx, Kxy, F(10), F(-10), Fr(0), "Sliding", 4)
    ok = r[-1]["converged"] and all(x["applied"] is None for x in r) and len(r) == 2
    check(f"NI {label} test 4 (mu = 0): converged, no applied force in any iteration", ok)
    res["test4"] = {"iterations": len(r), "u": r[-1]["u"], "normal": r[-1]["Ry"]}
    if expect:
        for key, vals in expect.items():
            for k, v in vals.items():
                check(f"NI {label} reproduces the pinned {key}.{k} = {v}", res[key][k] == F(v))
    return res


# 1) validate the model on the old element's block (k_a e e^T + k_l (I - e e^T), e = (1,1,0)/sqrt2): used only to check the model
old = ni_fixture_checks("model-validation (old block 150, -50)", F(150), F(-50), {
    "test1_10_-10": {"u": "7/135", "normal": "200/27", "friction": "-20/9"},
    "test1_-10_-10": {"u": "-7/165", "normal": "400/33", "friction": "40/11"},
    "test2_10_-1": {"branch_retry_u": "97/1350", "branch_retry_applied": "7/9", "branch_retry_Rx": "7/9", "branch_retry_Ry": "-70/27",
                    "final_u": "103/1650", "final_friction": "-7/11", "final_normal": "-70/33"},
    "test2_-10_1": {"branch_retry_u": "-97/1350", "branch_retry_applied": "-7/9", "branch_retry_Rx": "-7/9", "branch_retry_Ry": "70/27",
                    "final_u": "-103/1650", "final_friction": "7/11", "final_normal": "70/33"}})

# 2) the replacement: FrameElement node 0 (0,0,0) -> node 1 (3,-4,0), the fixture's own section
NI_SEC = {"E": F(100), "G": F(40), "A": F(1), "Iy": F(1), "Iz": F(1), "J": F(1)}
xj_ni = vec(3, -4, 0)
for yref in (vec(0, 1, 0), vec(0, 0, 1)):
    L_ni, Rm = frame_axes(vec(0, 0, 0), xj_ni, yref)
    kl = frame_local_k(NI_SEC["E"], NI_SEC["G"], NI_SEC["A"], NI_SEC["Iy"], NI_SEC["Iz"], NI_SEC["J"], L_ni)
    T = blockT(Rm)
    Kni = mm(mm(tr(T), kl), T)
    if yref == vec(0, 1, 0):
        Kni_main = Kni
    else:
        check("NI frame: node-1 Ux/Uy block independent of y_reference (Iy = Iz)", [Kni[6][6], Kni[6][7], Kni[7][7]] == [Kni_main[6][6], Kni_main[6][7], Kni_main[7][7]])
Kxx_n, Kxy_n = Kni_main[6][6], Kni_main[6][7]
a_ax, b_tr = NI_SEC["E"] * NI_SEC["A"] / 5, 12 * NI_SEC["E"] * NI_SEC["Iz"] / 125
c_, s_ = Fr(3, 5), Fr(-4, 5)
check("NI frame: Kxx = (EA/L) c^2 + (12EI/L^3) s^2", Kxx_n == a_ax * c_ * c_ + b_tr * s_ * s_)
check("NI frame: Kxy = (EA/L - 12EI/L^3) c s", Kxy_n == (a_ax - b_tr) * c_ * s_)
new = ni_fixture_checks("replacement frame", Kxx_n, Kxy_n, None)


def nifmt(v):
    return {"exact": ex(v), "decimal": dec(v, 20), "binary64": repr(float(v))}


ni_expect = {}
for key, vals in new.items():
    ni_expect[key] = {k: (nifmt(v) if isinstance(v, Fr) else v) for k, v in vals.items()}
add_case("U3-NI-FRICTION-FRAME", 8,
         "NI's four friction call sites (NI/src/lib.rs:3770, 3907, 3955, 4251@ed012c7ccf; fixture coupled_normal_friction_problem :3580-3614) re-based on an ordinary FrameElement on an off-axis chord with rational direction cosines, with every asserted value re-derived from the frame's own section",
         {"replacement": {"node_0": "(0, 0, 0)", "node_1": "(3, -4, 0)", "L": "5", "direction_cosines": "(3/5, -4/5, 0)",
                          "section": "FrameSection::new(100.0, 40.0, 1.0, 1.0, 1.0, 1.0): the fixture's own frame section from two_node_axial_problem (:3123): E = 100, G = 40, A = 1, Iy = Iz = 1, J = 1",
                          "y_reference": "[0.0, 1.0, 0.0] (as the fixture); [0, 0, 1] gives the same block because Iy = Iz",
                          "unchanged": "node 0 fixed; node 1 Uy, Uz, Rx, Ry, Rz restrained; friction F on node 1 Ux with mu = 0.30; derived normal from node 1 Uy ('fixed_y'); forces and seeds as at each call site",
                          "removed": "input.user_stiffness_elements (becomes the empty connector list); input.elements = vec![frame]"},
          "frame_block_node1": {"EA_over_L": ex(a_ax), "12EI_over_L3": ex(b_tr), "Kxx": ex(Kxx_n), "Kxy": ex(Kxy_n), "Kyy": ex(Kni_main[7][7])},
          "algorithm_model": "derived from reading NI/src/lib.rs@ed012c7ccf (loop 608-780, sliding solve 1370-1500, resolve 1560-1660, branch and direction 1737-1771, trial 2348-2410) and nonlinear_supports classify 644-670; validated by reproducing all 20 pinned old-block values (12 distinct magnitudes) exactly, with the iteration counts and the cap (old block used only for this validation)"},
         {"assertions": ni_expect,
          "structure": {
              "test1 (:3759, call :3770)": "both seeds and both modes: converged, 2 iterations, final Sliding; u, normal = reactions[Uy], friction = reactions[Ux] = applied force; |friction| = 0.30 |normal|; seeds bit-equal within a mode",
              "test2 (:3900, calls :3907 and :3955)": "Sticking seed: 3 iterations; iteration[1] stays Sliding with an inadmissible derived-normal branch (the normal changes sign); final Sliding with friction*u < 0; with max_iterations 2: not converged, blocked, 2 iterations, NonConvergence naming derived-normal",
              "test4 (:4245, call :4251)": "mu = 0, Sliding seed: converged (at iteration 2) and no iteration applies a sliding friction force"},
          "mapping_to_code": {
              "test1 (10,-10)": "expected_u, expected_normal, expected_friction = test1_10_-10.{u, normal, friction}",
              "test1 (-10,-10)": "test1_-10_-10.{u, normal, friction}",
              "test2 force (10,-1) / (-10,1)": "branch_retry displacement = sign*|branch_retry_u|, applied and Rx = sign*|branch_retry_applied|, reactions[Uy] = branch_retry_Ry; final displacement = final_u, friction = final_friction, normal = final_normal (each signed value given per force pair)"}},
         criterion="tolerance stays 1e-12 (absolute, as at each call site)")

# old-model values echoed for traceability
cases["U3-NI-FRICTION-FRAME"]["model_validation_old_block"] = {k: {kk: (ex(vv) if isinstance(vv, Fr) else vv) for kk, vv in v.items()} for k, v in old.items()}

# =====================================================================================
# =====================================================================================
# ROUND 01 (T4 WORKING_ITEMS, after a744c09021). Appended only: the 18 frozen cases above
# are not touched (check_round01.py proves it). WI ruling: under load-reference-1 the
# connector does not use the replaced span's resolved element state (E/nu, eigenstrain);
# a nonzero resolved eigenstrain on the replaced span, self-weight on it or any load it
# owns refuses with JOINT_REPLACED_SPAN_LOAD_UNOWNED (SLOT_TABLE S20 applied to 0.4.0).
# =====================================================================================
OTHER_MAT = {"id": "material:replaced-span-other", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
             "elastic_modulus": {"value": 100000000000.0, "unit": "Pa"}, "poisson_ratio": {"value": 0.25, "unit": "1"},
             "provenance": "T4-I12 round 01 control: a deliberately different E/nu on the replaced span only"}
R01_PROV = "T4-I12 round 01; " + PROV
LR1 = "openpipestress.load_reference_state/1.0.0"


def deep(o):
    return json.loads(json.dumps(o))


def doc_030_variant(p130_material=None):
    d0 = deep(demo)
    if p130_material:
        d0["model"]["materials"].append(deep(OTHER_MAT))
        for p in d0["model"]["pipe_segments"]:
            if p["id"] == "pipe:P-130":
                p["material"] = OTHER_MAT["id"]
    return d0


def doc_040(p130_material=None, p130_thermal_case=None, p130_fit_strain=None, p130_weight=False, p130_zero_explicit=False, p130_stored_unlisted=False):
    """The 0.4.0 (load-reference-1) form of U3-SYS-DEMO-CONNECTOR-001, plus round-01 variants."""
    d0 = deep(demo)
    m = d0["model"]
    m["schema_version"] = "0.4.0"
    for mat in m["materials"]:
        mat.pop("thermal_expansion_coefficient", None)  # interval thermal states carry their own coefficient
    if p130_material:
        m["materials"].append(deep(OTHER_MAT))
        for p in m["pipe_segments"]:
            if p["id"] == "pipe:P-130":
                p["material"] = OTHER_MAT["id"]
    pipe_mat = {p["id"]: p["material"] for p in m["pipe_segments"]}
    m["reference_configurations"] = [{
        "id": "reference:installed", "label": "Installed reference", "geometry_ref": {"kind": "authored_model_geometry"},
        "member_references": [{"pipe_ref": pid, "basis": {"kind": "direct_strain_reference"},
                               "fit": ({"kind": "fit_strain", "strain": {"value": p130_fit_strain, "unit": "1"}}
                                       if (pid == "pipe:P-130" and p130_fit_strain is not None) else {"kind": "none"}),
                               "provenance": R01_PROV} for pid in pipe_mat],
        "provenance": R01_PROV}]
    hot = {"kind": "constant_alpha_interval", "coefficient": {"value": 1.2e-05, "unit": "1/degC"},
           "temperature_change": {"value": 12.5, "unit": "degC"}, "coefficient_meaning": "engineering_interval", "provenance": R01_PROV}
    cold = {"kind": "unchanged_reference", "provenance": R01_PROV}
    for case in m["load_cases"]:
        case["primitive_loads"] = [ld for ld in case["primitive_loads"] if ld["category"] != "thermal"]
        if p130_weight and case["id"] == "load:L-100":
            case["primitive_loads"].append({"id": "load:L-100-Z-130", "category": "weight", "target": {"type": "element", "pipe": "pipe:P-130"},
                                            "direction": "global_x", "magnitude": {"value": -150.0, "unit": "N/m"}, "dimension": "force_per_length", "provenance": R01_PROV})
        unlisted = set()
        if p130_stored_unlisted and case["id"] == "load:L-100":
            case["primitive_loads"].append({"id": "load:L-100-Z-130-STORED", "category": "weight", "target": {"type": "element", "pipe": "pipe:P-130"},
                                            "direction": "global_x", "magnitude": {"value": -150.0, "unit": "N/m"}, "dimension": "force_per_length", "provenance": R01_PROV})
            unlisted.add("load:L-100-Z-130-STORED")
        states = []
        for pid in pipe_mat:
            if pid == "pipe:P-120" and case["id"] == "load:L-100":
                th = deep(hot)
            elif pid == "pipe:P-130" and p130_thermal_case == case["id"]:
                th = deep(hot)
            elif pid == "pipe:P-130" and p130_zero_explicit:
                th = {"kind": "explicit_interval_strain", "strain": {"value": 0.0, "unit": "1"}, "interval_reference": "T4-I12 round 01 zero-strain boundary", "provenance": R01_PROV}
            else:
                th = deep(cold)
            states.append({"pipe_ref": pid, "material_selection": {"kind": "explicit_base_properties", "material_ref": pipe_mat[pid],
                                                                   "applicability_reference": "invented analytical basis declared applicable (T4-I12)"},
                           "thermal_state": th})
        case["analysis_state"] = {
            "contract": LR1, "reference_configuration_ref": "reference:installed", "element_states": states,
            "support_states": [{"support_ref": s["id"], "participation": {"kind": "active_model_device"}} for s in m["supports"]],
            "load_sources": [{"source_ref": ld["id"], "factor": 1.0} for ld in case["primitive_loads"] if ld["id"] not in unlisted],
            "history": {"kind": "independent_equilibrium"}, "provenance": R01_PROV}
    return d0


def interpret(docx):
    """Read a 0.3.0 or 0.4.0 document of the re-authored demo into solver inputs, applying the
    replaced-span rules (JR section 3; SLOT_TABLE S20; the WI round-01 ruling)."""
    m = docx["model"]
    fr_ = lambda x: Fr(repr(x)) if isinstance(x, float) else Fr(x)
    nodes = {n["id"]: [fr_(n["position"][c]) for c in "xyz"] for n in m["nodes"]}
    mats = {mt["id"]: (fr_(mt["elastic_modulus"]["value"]), fr_(mt["poisson_ratio"]["value"]),
                       fr_(mt["thermal_expansion_coefficient"]["value"]) if "thermal_expansion_coefficient" in mt else None) for mt in m["materials"]}
    pipes = {p["id"]: {"from": p["from"], "to": p["to"], "OD": fr_(p["section"]["outside_diameter"]["value"]),
                       "t": fr_(p["section"]["wall_thickness"]["value"]), "mat": p["material"],
                       "yref": [fr_(p["y_reference"][c]) for c in "xyz"]} for p in m["pipe_segments"]}
    conn = None
    for c in m["components"]:
        oc = c.get("objective_connector")
        if oc:
            Ls = fr_(oc["stiffness"]["translation_scale"]["value"])
            conn = {"i": oc["end_i"]["node_ref"], "j": oc["end_j"]["node_ref"], "Q": [[fr_(x) for x in r] for r in oc["connector_axes_global"]],
                    "K": K_from_H(from_upper21([fr_(x) for x in oc["stiffness"]["upper_triangle"]]), Ls),
                    "span": oc["topology"]["span_ref"], "id": c["id"]}
    sups = []
    for s in m["supports"]:
        if s.get("family") == "variable_spring_hanger":
            sups.append((s["id"], s["node"], "spring", s["hanger"]["stiffness"]["dof"], fr_(s["hanger"]["stiffness"]["value"]["value"])))
        else:
            sups.append((s["id"], s["node"], "rigid", s["restraints"], None))
    cases_ = {}
    for case in m["load_cases"]:
        st = case.get("analysis_state")
        prims = {ld["id"]: ld for ld in case["primitive_loads"]}
        if st is None:  # 0.3.0: every primitive applies; thermal primitives give member strain alpha*dT
            applied = [(prims[k], Fr(1)) for k in prims]
            eig = {pid: Fr(0) for pid in pipes}
            for ld, _ in applied:
                if ld["category"] == "thermal":
                    pid = ld["target"]["pipe"]
                    eig[pid] += mats[pipes[pid]["mat"]][2] * fr_(ld["magnitude"]["value"])
            applied = [(ld, f) for ld, f in applied if ld["category"] != "thermal"]
            emat = {pid: pipes[pid]["mat"] for pid in pipes}
        else:  # 0.4.0: only load_sources apply (with their factors); strain from element and reference states
            applied = [(prims[s["source_ref"]], fr_(s["factor"])) for s in st["load_sources"]]
            refc = {mr["pipe_ref"]: mr for mr in m["reference_configurations"][0]["member_references"]}
            eig, emat = {}, {}
            for es in st["element_states"]:
                pid, th = es["pipe_ref"], es["thermal_state"]
                if th["kind"] == "unchanged_reference":
                    eth = Fr(0)
                elif th["kind"] == "constant_alpha_interval":
                    eth = fr_(th["coefficient"]["value"]) * fr_(th["temperature_change"]["value"])
                elif th["kind"] == "explicit_interval_strain":
                    eth = fr_(th["strain"]["value"])
                else:
                    raise ValueError(th["kind"])
                fit = refc[pid]["fit"]
                ef = fr_(fit["strain"]["value"]) if fit["kind"] == "fit_strain" else Fr(0)
                eig[pid] = (1 + ef) * (1 + eth) - 1  # lambda_fit * lambda_thermal - 1
                emat[pid] = es["material_selection"]["material_ref"]
        cases_[case["id"]] = {"applied": applied, "eig": eig, "emat": emat}
    return {"nodes": nodes, "mats": mats, "pipes": pipes, "conn": conn, "sups": sups, "cases": cases_}


def replaced_span_refusal(model_i, cid):
    """SLOT_TABLE S20 / WI ruling: the replaced span may own no load and no nonzero resolved eigenstrain."""
    span = model_i["conn"]["span"]
    c = model_i["cases"][cid]
    owned = [ld["id"] for ld, _ in c["applied"] if ld["target"].get("pipe") == span]
    if c["eig"].get(span, 0) != 0:
        owned.append(f"element_state:{span}")
    return {"blocking_code": "JOINT_REPLACED_SPAN_LOAD_UNOWNED", "affected_refs_include": [model_i["conn"]["id"], span] + owned} if owned else None


def solve_doc(model_i, cid, pi_val, keep_span=False):
    nid = list(model_i["nodes"])
    nd = 6 * len(nid)
    dof = lambda n: list(range(6 * nid.index(n), 6 * nid.index(n) + 6))
    Kg = zeros(nd, nd)
    f = [Fr(0)] * nd
    c = model_i["cases"][cid]
    span = model_i["conn"]["span"]
    geo = {}
    for pid, p in model_i["pipes"].items():
        if pid == span and not keep_span:
            continue  # replaces_span: removed from every assembly path
        E, nu, _ = model_i["mats"][c["emat"][pid]]
        G = E / (2 * (1 + nu))
        L, Rm = frame_axes(model_i["nodes"][p["from"]], model_i["nodes"][p["to"]], p["yref"])
        ro_, ri_ = p["OD"] / 2, p["OD"] / 2 - p["t"]
        A = pi_val * p["t"] * (p["OD"] - p["t"])
        I_ = A * (ro_ * ro_ + ri_ * ri_) / 4
        T = blockT(Rm)
        Ke_ = mm(mm(tr(T), frame_local_k(E, G, A, I_, I_, 2 * I_, L)), T)
        idx = dof(p["from"]) + dof(p["to"])
        for a_, ia in enumerate(idx):
            for b_, ib in enumerate(idx):
                Kg[ia][ib] += Ke_[a_][b_]
        geo[pid] = (L, Rm, E, A, idx)
    cn = model_i["conn"]
    xi, xj = model_i["nodes"][cn["i"]], model_i["nodes"][cn["j"]]
    Bm = B_closed([Fr(0)] * 3, [Fr(0)] * 3, vsub(xj, xi), cn["Q"])
    Kc = mm(mm(tr(Bm), cn["K"]), Bm)
    idx = dof(cn["i"]) + dof(cn["j"])
    for a_, ia in enumerate(idx):
        for b_, ib in enumerate(idx):
            Kg[ia][ib] += Kc[a_][b_]
    restrained, springs = [], []
    for sid, node, kind, dd, k in model_i["sups"]:
        if kind == "spring":
            s_ = dof(node)[["UX", "UY", "UZ", "RX", "RY", "RZ"].index(dd)]
            Kg[s_][s_] += k
            springs.append((sid, node, s_, k))
        else:
            restrained += [dof(node)[["UX", "UY", "UZ", "RX", "RY", "RZ"].index(r)] for r in dd]
    axis = {"global_x": 0, "global_y": 1, "global_z": 2, "rotation_x": 3, "rotation_y": 4, "rotation_z": 5}
    for ld, fac in c["applied"]:
        val = Fr(repr(ld["magnitude"]["value"])) * fac
        if ld["target"]["type"] == "element":
            L, Rm, E, A, idx = geo[ld["target"]["pipe"]]
            wg = [Fr(0)] * 3
            wg[axis[ld["direction"]]] = val
            fl = consistent_uniform(Rm, L, wg)
            for a_, ia in enumerate(idx):
                f[ia] += fl[a_]
        else:
            f[dof(ld["target"]["node"])[axis[ld["direction"]]]] += val
    for pid, e in c["eig"].items():
        if e != 0 and pid in geo:
            L, Rm, E, A, idx = geo[pid]
            for k in range(3):
                f[idx[k]] -= E * A * e * Rm[0][k]
                f[idx[6 + k]] += E * A * e * Rm[0][k]
    free = [i for i in range(nd) if i not in restrained]
    df = solve([[Kg[i][j] for j in free] for i in free], [f[i] for i in free])
    d = [Fr(0)] * nd
    for k, v in zip(free, df):
        d[k] = v
    Kd = mv(Kg, d)
    reac = {}
    for sid, node, kind, dd, k in model_i["sups"]:
        if kind == "spring":
            s_ = dof(node)[["UX", "UY", "UZ", "RX", "RY", "RZ"].index(dd)]
            rv = [Fr(0)] * 6
            rv[["UX", "UY", "UZ", "RX", "RY", "RZ"].index(dd)] = -k * d[s_]
        else:
            rset = [["UX", "UY", "UZ", "RX", "RY", "RZ"].index(r) for r in dd]
            rv = [Kd[dof(node)[q]] - f[dof(node)[q]] if q in rset else Fr(0) for q in range(6)]
        reac[sid] = rv
    dc = [d[i] for i in dof(cn["i"]) + dof(cn["j"])]
    q = mv(Bm, dc)
    g = mv(cn["K"], q)
    fc = mv(tr(Bm), g)
    Fv, Mv, _ = actions_closed(g, [Fr(0)] * 3, [Fr(0)] * 3, vsub(xj, xi), cn["Q"])
    return {"d": d, "dof": dof, "nid": nid, "reactions": reac, "q": q, "g": g, "U": dot(q, g) / 2, "fc": fc, "F": Fv, "M": Mv}


def sys_block(res):
    """The same layout and formatting as U3-SYS-DEMO-CONNECTOR-001's expected values."""
    dof, nid, fc = res["dof"], res["nid"], res["fc"]
    disp = {n: {"u_m": [sysfmt(x) for x in res["d"][dof(n)[0]:dof(n)[0] + 3]], "theta_rad": [sysfmt(x) for x in res["d"][dof(n)[3]:dof(n)[3] + 3]]} for n in nid}
    reac = {sid: [sysfmt(x) for x in rv] for sid, rv in res["reactions"].items()}
    Fs = max(max(abs(x) for x in rv[0:3]) for rv in res["reactions"].values())
    Ms = max(max(abs(x) for x in rv[3:6]) for rv in res["reactions"].values())
    us = max(abs(x) for n in nid for x in res["d"][dof(n)[0]:dof(n)[0] + 3])
    ts = max(abs(x) for n in nid for x in res["d"][dof(n)[3]:dof(n)[3] + 3])
    return {
        "displacements": disp,
        "reactions_support_on_pipe_global_Fx_Fy_Fz_Mx_My_Mz": reac,
        "connector": {"q_local": [sysfmt(x) for x in res["q"]], "g_local": [sysfmt(x) for x in res["g"]], "energy_J": sysfmt(res["U"]),
                      "F_global": [sysfmt(x) for x in res["F"]], "M_global": [sysfmt(x) for x in res["M"]],
                      "end_actions_node_on_element": {"Fi_at_N-130": [sysfmt(x) for x in fc[0:3]], "Mi_at_N-130": [sysfmt(x) for x in fc[3:6]],
                                                      "Fj_at_N-140": [sysfmt(x) for x in fc[6:9]], "Mj_at_N-140": [sysfmt(x) for x in fc[9:12]]}},
        "global_balance": {"sum_applied_plus_reactions_force_N": "0 (exact; reference residual < 1e-100)", "sum_moment_about_origin_N_m": "0 (exact; reference residual < 1e-100)"},
        "zero_scale_floors": {"force_N": sysfmt(Fs, 20), "moment_N_m": sysfmt(Ms, 20), "translation_m": sysfmt(us, 20), "rotation_rad": sysfmt(ts, 20),
                              "q_translation_m": sysfmt(max(abs(x) for x in res["q"][0:3]), 20), "q_rotation_rad": sysfmt(max(abs(x) for x in res["q"][3:6]), 20),
                              "g_force_N": sysfmt(max(abs(x) for x in res["g"][0:3]), 20), "g_moment_N_m": sysfmt(max(abs(x) for x in res["g"][3:6]), 20),
                              "rule": "|obs - exp| <= 1e-9 * max(|exp|, floor of the value's family); floors are the case's largest magnitude in that family"},
    }


frozen_sys = cases["U3-SYS-DEMO-CONNECTOR-001"]["expected"]
d030, d040 = demo, doc_040()
i030, i040 = interpret(d030), interpret(d040)
r01_lr1, r01_ctl030, r01_ctl040 = {}, {}, {}
ctl_disc = {}
for cid in CASES:
    check(f"round01 {cid}: 0.3.0 and 0.4.0 forms admit (no replaced-span refusal)", replaced_span_refusal(i030, cid) is None and replaced_span_refusal(i040, cid) is None)
    check(f"round01 {cid}: 0.4.0 resolved eigenstrain on the replaced span is exactly 0; P-120 carries alpha*dT only in L-100",
          i040["cases"][cid]["eig"]["pipe:P-130"] == 0 and i040["cases"][cid]["eig"]["pipe:P-120"] == (F("1.2e-5") * F("12.5") if cid == "load:L-100" else 0))
    b030 = sys_block(solve_doc(i030, cid, PI))
    check(f"round01 {cid}: the document-driven solve of the frozen 0.3.0 document reproduces U3-SYS-DEMO-CONNECTOR-001 (every string)", b030 == frozen_sys[cid])
    b040 = sys_block(solve_doc(i040, cid, PI))
    check(f"round01 {cid}: the 0.4.0 form gives identical displacements, reactions and connector actions (every string)", b040 == frozen_sys[cid])
    r01_lr1[cid] = b040
    for form, mk in (("0.3.0", lambda: doc_030_variant(p130_material=True)), ("0.4.0", lambda: doc_040(p130_material=True))):
        ii = interpret(mk())
        check(f"round01 {cid}: the {form} E/nu control resolves P-130 to E = 1e11, nu = 0.25", ii["mats"][ii["cases"][cid]["emat"]["pipe:P-130"]][:2] == (F("1e11"), F("0.25")))
        bb = sys_block(solve_doc(ii, cid, PI))
        check(f"round01 {cid}: the {form} E/nu control changes no displacement, reaction or connector value", bb == frozen_sys[cid])
    # discriminating power: with P-130 kept in parallel, its own E/nu would move the results
    base_par = solve_doc(i040, cid, PI, keep_span=True)
    ctl_par = solve_doc(interpret(doc_040(p130_material=True)), cid, PI, keep_span=True)
    comps = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"]
    Fs = Fr(frozen_sys[cid]["zero_scale_floors"]["force_N"])
    Ms = Fr(frozen_sys[cid]["zero_scale_floors"]["moment_N_m"])
    moved = [f"{sid}.{comps[k]}" for sid in base_par["reactions"] for k in range(6)
             if abs(base_par["reactions"][sid][k] - ctl_par["reactions"][sid][k]) > Fr(1, 10 ** 6) * (Fs if k < 3 else Ms)]
    gmoved = [k for k in range(6) if abs(base_par["g"][k] - ctl_par["g"][k]) > Fr(1, 10 ** 9) * max(abs(base_par["g"][k]), 1)]
    check(f"round01 {cid}: the control discriminates (a retained P-130 makes its E/nu move the reactions)", len(moved) > 0)
    ctl_disc[cid] = {"what": "P-130 kept in parallel (replaces_span not applied): its E/nu then changes the results, so the control would expose it",
                     "reactions_with_P130_E_2e11_nu_0.3": {sid: [sysfmt(x, 20) for x in rv] for sid, rv in base_par["reactions"].items()},
                     "reactions_with_P130_E_1e11_nu_0.25": {sid: [sysfmt(x, 20) for x in rv] for sid, rv in ctl_par["reactions"].items()},
                     "connector_g_with_P130_E_2e11": [sysfmt(x, 20) for x in base_par["g"]],
                     "connector_g_with_P130_E_1e11": [sysfmt(x, 20) for x in ctl_par["g"]],
                     "moved_reaction_components": moved, "moved_g_components": gmoved}

# refusal variants under load-reference-1
ref_variants = {
    "a_thermal_on_replaced_span": ("pipe:P-130 element_state in load:L-100 is constant_alpha_interval (1.2e-5 /degC, 12.5 degC)", doc_040(p130_thermal_case="load:L-100"), ["load:L-100"]),
    "b_fit_strain_on_replaced_span": ("pipe:P-130 member reference fit is fit_strain 1e-4 (nonzero resolved eigenstrain in every case)", doc_040(p130_fit_strain=0.0001), list(CASES)),
    "c_weight_on_replaced_span": ("load:L-100 stores a weight primitive on pipe:P-130 and lists it in load_sources", doc_040(p130_weight=True), ["load:L-100"]),
}
r01_ref = {}
for key, (what, dv, refused_cases) in ref_variants.items():
    iv = interpret(dv)
    outcome = {cid: replaced_span_refusal(iv, cid) for cid in CASES}
    check(f"round01 refusal {key}: refused exactly in {refused_cases}", [cid for cid in CASES if outcome[cid]] == refused_cases)
    eigv = {cid: ex(iv["cases"][cid]["eig"]["pipe:P-130"]) for cid in CASES}
    r01_ref[key] = {"what": what, "resolved_eigenstrain_P130_by_case": eigv,
                    "expected": {"status_mechanics": "MODEL_INCOMPLETE", "results": "none (no case is published, as for any blocking diagnostic)",
                                 "blocking_code": "JOINT_REPLACED_SPAN_LOAD_UNOWNED",
                                 "refused_cases": refused_cases,
                                 "affected_refs_include": sorted({r for cid in refused_cases for r in outcome[cid]["affected_refs_include"]}),
                                 "fallback": "none: the strain is never moved to the joint, dropped, or turned into a joint thermal law (that comes with T4-U5's J3)"},
                    "document_v3_0.4.0": dv}
izero = interpret(doc_040(p130_zero_explicit=True))
check("round01 boundary: an explicit zero interval strain on P-130 resolves to 0 and is not refused under the ruling's 'nonzero'", izero["cases"]["load:L-100"]["eig"]["pipe:P-130"] == 0
      and all(replaced_span_refusal(izero, cid) is None for cid in CASES))
zero_blocks = {cid: sys_block(solve_doc(izero, cid, PI)) for cid in CASES}
check("round01 boundary: its results equal the cold case U3-SYS-DEMO-CONNECTOR-002-LR1 (every string)", all(zero_blocks[cid] == r01_lr1[cid] for cid in CASES))
dstored = doc_040(p130_stored_unlisted=True)
istored = interpret(dstored)
check("round01 stored-unapplied: L-100 stores a weight primitive on P-130 that no case lists in load_sources",
      any(ld["target"].get("pipe") == "pipe:P-130" for ld in dstored["model"]["load_cases"][0]["primitive_loads"])
      and not any(s_["source_ref"] == "load:L-100-Z-130-STORED" for c_ in dstored["model"]["load_cases"] for s_ in c_["analysis_state"]["load_sources"]))
check("round01 stored-unapplied: no case refuses (refusal is decided by what each case applies)", all(replaced_span_refusal(istored, cid) is None for cid in CASES))
stored_blocks = {cid: sys_block(solve_doc(istored, cid, PI)) for cid in CASES}
check("round01 stored-unapplied: every result equals U3-SYS-DEMO-CONNECTOR-002-LR1 (every string)", all(stored_blocks[cid] == r01_lr1[cid] for cid in CASES))

add_case("U3-SYS-DEMO-CONNECTOR-002-LR1", "6 (round 01)",
         "The 0.4.0 (load-reference-1) form of U3-SYS-DEMO-CONNECTOR-001: the same model, with the thermal on P-120 as its element thermal_state and the replaced span P-130 cold (unchanged_reference, fit none). Every displacement, reaction and connector value equals the 0.3.0 case",
         {"form": "schema 0.4.0; pressure_contract 3.0.0/exact_pressure_v3 (provisional); materials without alpha (the interval states carry it); one reference configuration (direct_strain_reference, fit none, every pipe including P-130); every case has an analysis_state with one element state per pipe (explicit_base_properties naming the pipe's own material), every support active_model_device, every stored primitive in load_sources with factor 1, history independent_equilibrium",
          "thermal": "L-100: pipe:P-120 constant_alpha_interval 1.2e-5 /degC x 12.5 degC (engineering interval; resolved eigenstrain 3/20000); every other member and every other case unchanged_reference. No thermal primitive (0.4.0 refuses one)",
          "replaced_span": "pipe:P-130 keeps a reference member entry and an element state (both required for every pipe), unchanged_reference and fit none, so its resolved eigenstrain is exactly 0. By the WI ruling the connector never reads that state (E/nu or eigenstrain)",
          "cold_while_others_strained": "admissible: element states are per pipe and per case, so P-130 stays cold while P-120 is strained; no nearer variant was needed",
          "document_v3_0.4.0": d040},
         r01_lr1,
         criterion="as U3-SYS-DEMO-CONNECTOR-001; in addition, in each mode the 0.4.0 run's published displacements, reactions and connector rows equal the 0.3.0 run's (the same operands reach the same assembly: E, nu, As, alpha*dT)",
         extra={"equality_to_001": "every expected string is identical to U3-SYS-DEMO-CONNECTOR-001's (checked)"})
add_case("U3-SYS-LR1-REPLACED-SPAN-EIGENSTRAIN-REFUSAL", "7 (round 01)",
         "A nonzero resolved eigenstrain on the replaced span (thermal state or fit), or a load it owns, refuses with JOINT_REPLACED_SPAN_LOAD_UNOWNED under load-reference-1 (SLOT_TABLE S20; WI round-01 ruling)",
         {"base": "U3-SYS-DEMO-CONNECTOR-002-LR1; each variant changes only what its 'what' names"},
         r01_ref,
         criterion="structural: MODEL_INCOMPLETE, no results, the named blocking code with refs including component:C-150 and pipe:P-130",
         extra={"boundary_zero_strain": {"what": "pipe:P-130 thermal_state explicit_interval_strain with strain 0 (resolved eigenstrain exactly 0)",
                                         "expected": "admitted, by the WI round-01 ruling: a state that resolves to exactly zero has no effect on the connector, so T4-U3 admits it; every result equals the cold case U3-SYS-DEMO-CONNECTOR-002-LR1 (checked string for string)",
                                         "expected_values": zero_blocks,
                                         "document_v3_0.4.0": doc_040(p130_zero_explicit=True)},
                "stored_unapplied_rule": "WI round-01 ruling: refusal is decided per case by what that case applies; a case listing a load on the replaced span in load_sources is refused; a load stored but applied by no case is not refused and has no effect (admitted control: U3-SYS-LR1-REPLACED-SPAN-STORED-UNAPPLIED-CONTROL)"})
add_case("U3-SYS-REPLACED-SPAN-MATERIAL-CONTROL", "6 (round 01)",
         "The replaced span's own E/nu does not affect any connector result: pipe:P-130 (and, in 0.4.0, its element state) names a different material (E 1e11 Pa, nu 0.25); everything else as 001 / 002-LR1",
         {"material_added": OTHER_MAT, "document_v3_0.3.0": doc_030_variant(p130_material=True), "document_v3_0.4.0": doc_040(p130_material=True)},
         {"0.3.0": "every displacement, reaction and connector value identical to U3-SYS-DEMO-CONNECTOR-001 (checked)",
          "0.4.0": "every displacement, reaction and connector value identical to U3-SYS-DEMO-CONNECTOR-002-LR1 (checked)"},
         criterion="in each mode, bitwise equality of every published displacement, reaction and connector row with the run of the unmodified document (P-130 is in no assembly or recovery path), and the U3-SYS-DEMO-CONNECTOR-001 reference at its criterion",
         disc=ctl_disc)
add_case("U3-SYS-LR1-REPLACED-SPAN-STORED-UNAPPLIED-CONTROL", "7 (round 01)",
         "Admitted control for the WI round-01 ruling: load:L-100 stores a weight primitive on the replaced span P-130 (load:L-100-Z-130-STORED, -150 N/m in global x) that no case lists in load_sources; it is applied by no case, so nothing is refused and nothing changes",
         {"base": "U3-SYS-DEMO-CONNECTOR-002-LR1; the only difference is the stored, unlisted primitive", "document_v3_0.4.0": dstored},
         stored_blocks,
         criterion="as U3-SYS-DEMO-CONNECTOR-002-LR1 (MECHANICS_SOLVED, no JOINT_REPLACED_SPAN_LOAD_UNOWNED); in each mode every published displacement, reaction and connector row equals the run of the unmodified 0.4.0 document",
         extra={"equality_to_002": "every expected string is identical to U3-SYS-DEMO-CONNECTOR-002-LR1's (checked)",
                "contrast": "listing the same primitive in L-100's load_sources is refusal variant c of U3-SYS-LR1-REPLACED-SPAN-EIGENSTRAIN-REFUSAL"})

# ---- round 01, item from T3's slot-table review (RV130, S-3): K-D5's undemoted connector case with
# nonzero offsets, a skew Q and a coupled K, at ordinary and UTM coordinates (X0 = 0, 5e6, 7.3e6 m).
# Every input is an exact binary64 value; the reference is exact for those values.
def b64(x):
    return {"binary64": repr(float(x)), "hex": float(x).hex(), "exact": ex(Fr(float(x)))}


def b64v(v):
    return [b64(x) for x in v]


QK_rat = [[Fr(1, 3), Fr(2, 3), Fr(-2, 3)], [Fr(2, 3), Fr(1, 3), Fr(2, 3)], [Fr(2, 3), Fr(-2, 3), Fr(-1, 3)]]  # columns (1,2,2)/3, (2,1,-2)/3, (-2,2,-1)/3
QK = [[Fr(float(x)) for x in row] for row in QK_rat]  # the binary64 values, as given (not exactly orthonormal)
aiK = [Fr(0.2), Fr(0.4), Fr(-0.2)]
DXK = Fr(21, 16)  # x_j,x - x_i,x = 1.3125 m, exact at every location
ajKx = Fr(-0.1)
rxK = DXK + ajKx - aiK[0]  # r = rxK * (1, 2, 2) exactly
xiK0 = [Fr(1, 2), Fr(-5, 4), Fr(2)]
xjK_yz = [Fr(13, 16), Fr(27, 8)]
ajK = [ajKx, (xiK0[1] - xjK_yz[0]) + 2 * rxK + aiK[1], (xiK0[2] - xjK_yz[1]) + 2 * rxK + aiK[2]]
check("KD5-UTM: a_j is exactly representable in binary64 (chosen so that r is exactly parallel to (1,2,2))", all(Fr(float(x)) == x for x in ajK))
check("KD5-UTM: the x offsets carry binary64 tails finer than 2^-30 (so an absolute-position form rounds at UTM scale)",
      all((x * 2 ** 30).denominator != 1 for x in (aiK[0], ajK[0])))
dK = [Fr(x) for x in (2 ** -10, -2 ** -9, 3 * 2 ** -10, 2 ** -8, -2 ** -9, 2 ** -10, -2 ** -10, 2 ** -8, 2 ** -9, -2 ** -8, 2 ** -9, 3 * 2 ** -10)]
qrefK = [Fr(x) for x in (2 ** -10, -2 ** -11, 2 ** -12, 2 ** -9, 0, -2 ** -10)]
LsK = Fr(1, 4)
kd5_locs = {}
kd5_ref = None
for tag, X0 in (("ORDINARY", 0.0), ("X5E6", 5e6), ("X7P3E6", 7.3e6)):
    xi = [Fr(X0 + 0.5), xiK0[1], xiK0[2]]
    xj = [Fr(X0 + 1.8125), xjK_yz[0], xjK_yz[1]]
    check(f"KD5-UTM {tag}: node coordinates are exact binary64 values (X0 + 0.5, X0 + 1.8125)", xi[0] == Fr(X0) + Fr(1, 2) and xj[0] == Fr(X0) + Fr(29, 16))
    r = vsub(vadd(xj, ajK), vadd(xi, aiK))
    check(f"KD5-UTM {tag}: exact r = (x_j - x_i) + (a_j - a_i) = rx (1, 2, 2), identical at every location", r == [rxK, 2 * rxK, 2 * rxK])
    B = B_closed(aiK, ajK, r, QK)
    check(f"KD5-UTM {tag}: closed-form B equals kinematic B (with the binary64 Q as given)", B == B_kinematic(aiK, ajK, r, QK))
    check(f"KD5-UTM {tag}: rank B = 6", rank(B) == 6)
    Ke = mm(mm(tr(B), KS), B)
    for oname, o in (("origin 0", [Fr(0)] * 3), ("x_i", xi)):
        for name, mvec in rigid_modes(xi, xj, o):
            check(f"KD5-UTM {tag}: rigid {name} about {oname} in null(B) and null(Ke), exactly", mv(B, mvec) == [0] * 6 and mv(Ke, mvec) == [0] * 12)
    q = mv(B, dK)
    check(f"KD5-UTM {tag}: q from B equals q from kinematics", q == q_kinematic(dK, aiK, ajK, r, QK))
    g = mv(KS, vsub(q, qrefK))
    f = mv(tr(B), g)
    Fv, Mv, fcc = actions_closed(g, aiK, ajK, r, QK)
    check(f"KD5-UTM {tag}: B^T g equals the closed-form blocks", f == fcc)
    fs = vadd(f[0:3], f[6:9])
    ms = vadd(vadd(vadd(cross(xi, f[0:3]), f[3:6]), cross(xj, f[6:9])), f[9:12])
    check(f"KD5-UTM {tag}: end actions balance about the origin, exactly", fs == [0] * 3 and ms == [0] * 3)
    rhs = mv(tr(B), mv(KS, qrefK))
    this = {"B": exm(B), "Ke": exm(Ke), "q": bothv(q), "g": bothv(g), "energy": both(dot(vsub(q, qrefK), g) / 2),
            "end_actions_node_on_element": {"Fi": bothv(f[0:3]), "Mi": bothv(f[3:6]), "Fj": bothv(f[6:9]), "Mj": bothv(f[9:12])},
            "installed_rhs_BT_K_qref": exv(rhs)}
    if kd5_ref is None:
        kd5_ref = this
    check(f"KD5-UTM {tag}: B, Ke, q, g, energy, end actions and RHS identical to the ordinary location (translation invariance)", this == kd5_ref)

    # binary64 formation illustration (naive, informational): r by difference form vs absolute-position form
    def form64(r64):
        Qf = [[float(x) for x in row] for row in QK]
        ai_, aj_ = [float(x) for x in aiK], [float(x) for x in ajK]
        def S(v):
            return [[0.0, -v[2], v[1]], [v[2], 0.0, -v[0]], [-v[1], v[0], 0.0]]
        hr = [x / 2 for x in r64]
        Sai, Saj, Shr = S(ai_), S(aj_), S(hr)
        I3f = [[1.0 if i == j else 0.0 for j in range(3)] for i in range(3)]
        blk = [[-I3f[i][j] for j in range(3)] for i in range(3)], [[Sai[i][j] + Shr[i][j] for j in range(3)] for i in range(3)], I3f, [[-Saj[i][j] + Shr[i][j] for j in range(3)] for i in range(3)]
        rowsT = [sum((list(b_[i]) for b_ in blk), []) for i in range(3)]
        Bt = [[sum(Qf[k][i] * rowsT[k][c] for k in range(3)) for c in range(12)] for i in range(3)]
        rowsR = [[0.0] * 3 + [-I3f[i][j] for j in range(3)] + [0.0] * 3 + [I3f[i][j] for j in range(3)] for i in range(3)]
        Br = [[sum(Qf[k][i] * rowsR[k][c] for k in range(3)) for c in range(12)] for i in range(3)]
        Bf = Bt + Br
        Kf = [[float(x) for x in row] for row in KS]
        KB = [[sum(Kf[a][b_] * Bf[b_][c] for b_ in range(6)) for c in range(12)] for a in range(6)]
        return [[sum(Bf[a][i] * KB[a][j] for a in range(6)) for j in range(12)] for i in range(12)]
    xi64, xj64 = [float(x) for x in xi], [float(x) for x in xj]
    ai64, aj64 = [float(x) for x in aiK], [float(x) for x in ajK]
    r_diff = [(xj64[k] - xi64[k]) + (aj64[k] - ai64[k]) for k in range(3)]
    r_abs = [(xj64[k] + aj64[k]) - (xi64[k] + ai64[k]) for k in range(3)]
    kmax = max(abs(x) for row in Ke for x in row)
    dev = lambda K64: max(abs(Fr(K64[i][j]) - Ke[i][j]) for i in range(12) for j in range(12)) / kmax
    rdev = lambda r64: max(abs(Fr(r64[k]) - r[k]) for k in range(3)) / (3 * rxK)
    kd5_locs[tag] = {
        "X0_m": repr(X0),
        "x_i": b64v(xi), "x_j": b64v(xj),
        "p_i_exact": exv(vadd(xi, aiK)), "p_j_exact": exv(vadd(xj, ajK)),
        "binary64_formation_illustration": {
            "r_difference_form_rel_error": f"{float(rdev(r_diff)):.3e}", "r_absolute_form_rel_error": f"{float(rdev(r_abs)):.3e}",
            "Ke_difference_form_max_dev_over_max_Ke": f"{float(dev(form64(r_diff))):.3e}",
            "Ke_absolute_form_max_dev_over_max_Ke": f"{float(dev(form64(r_abs))):.3e}"},
    }
    if tag != "ORDINARY":
        check(f"KD5-UTM {tag}: the absolute-position form is visibly worse than the difference form (r error > 1e-12 relative)", rdev(r_abs) > Fr(1, 10 ** 12) and rdev(r_diff) < Fr(1, 10 ** 15))
    print(f"KD5-UTM {tag}: binary64 r rel error diff/abs = {kd5_locs[tag]['binary64_formation_illustration']['r_difference_form_rel_error']} / "
          f"{kd5_locs[tag]['binary64_formation_illustration']['r_absolute_form_rel_error']}; Ke dev diff/abs = "
          f"{kd5_locs[tag]['binary64_formation_illustration']['Ke_difference_form_max_dev_over_max_Ke']} / {kd5_locs[tag]['binary64_formation_illustration']['Ke_absolute_form_max_dev_over_max_Ke']}")
KeK = [[Fr(x) for x in row] for row in kd5_ref["Ke"]]
kmaxK = max(abs(x) for row in KeK for x in row)
kdiag = max(range(12), key=lambda k: abs(KeK[k][k]))
deltaK = kmaxK / 2 ** 20
check("KD5-UTM perturbed: delta = 2^-20 max|Ke| exceeds the absolute-form defect at 7.3e6 m by more than 1000x",
      deltaK / kmaxK > 1000 * Fr(kd5_locs["X7P3E6"]["binary64_formation_illustration"]["Ke_absolute_form_max_dev_over_max_Ke"]))
add_case("U3-KD5-UTM-SKEW-OFFSET-COUPLED", "3, 4 (round 01; RV130 S-3)",
         "K-D5's undemoted connector case (T3 slot-table review RV130, S-3): nonzero offsets whose x components carry binary64 tails finer than the 2^-30 m grid at 5e6-7.3e6 m, a skew Q (binary64 values of a rational rotation), the fully coupled PD K of U3-GENERIC, a dyadic q_ref and d; at X0 = 0, 5e6 and 7.3e6 m. Every input is an exact binary64 value; the reference is exact for those values and identical at the three locations",
         {"shared": {
             "a_i_global": b64v(aiK), "a_j_global": b64v(ajK),
             "Q_row_major_columns_are_axes": [[b64(x) for x in row] for row in QK],
             "Q_note": "the binary64 roundings of the rational rotation with columns (1,2,2)/3, (2,1,-2)/3, (-2,2,-1)/3; B and the end actions use these values exactly as given; |Q^T Q - I| and |det Q - 1| are at binary64 rounding level (" + f"{float(max(abs(x) for row in madd(mm(tr(QK), QK), mscale(-1, eye(3))) for x in row)):.1e}" + ")",
             "translation_scale_Ls_m": ex(LsK), "H_upper_triangle_21_N_m": exv(upper21(H_from_K(KS, LsK))), "K_physical": exm(KS),
             "q_ref": b64v(qrefK), "d": b64v(dK),
             "r_exact": exv([rxK, 2 * rxK, 2 * rxK]), "r_formula": "r = (x_j - x_i) + (a_j - a_i); x_j,x - x_i,x = 1.3125 exactly at every location (Sterbenz in binary64)"},
          "locations": kd5_locs},
         {"identical_at_every_location": kd5_ref,
          "rigid_modes": "the six rigid motions of the actual nodes, about the origin and about x_i, lie in null(B) and null(Ke) exactly at every location",
          "perturbed_variant": {
              "what": "Ke' = Ke_formed + delta e_k e_k^T with k = " + str(kdiag) + " (the largest diagonal entry, 0-based DOF index in [u_i, theta_i, u_j, theta_j]) and delta = 2^-20 max_ij|Ke_ij| (an exact power-of-two scaling of the formed maximum), applied to the formed matrix only, never to K-D5's Wide<2> re-formation",
              "Ke_kk_exact": ex(KeK[kdiag][kdiag]), "max_abs_Ke_exact": ex(kmaxK), "delta_exact": ex(deltaK),
              "delta_over_max_Ke": "2^-20 = 9.5367431640625e-07",
              "expected": "demoted (formation check fails) at all three locations"},
          "kd5_expectations": {
              "unperturbed, difference form": "not demoted at X0 = 0, 5e6 and 7.3e6 m (T3 reports 6.5e-7 of the criterion at UTM)",
              "perturbed": "demoted at all three locations",
              "absolute-position form (a defect, not a case to build)": "T3 reports demotion at 1.39x the criterion at UTM; the illustration above shows its r and Ke defects; the reference values do not depend on the form"}},
         criterion="FK unit level: B, Ke (exact rationals of the binary64 inputs) compared at relative 1e-12 per entry against max|Ke|; q, g and end actions at relative 1e-12; K-D5's own criterion decides demotion",
         extra={"binary64_illustration_note": "the illustration is a naive binary64 formation in this script (not the product): it shows the size of the absolute-form defect at UTM scale, not K-D5's judgment"})


doc = {
    "schema_version": "t4-i12-u3-reference-1",
    "status": "frozen before T4-U3 code; awaiting the refuting TASK; not product evidence",
    "author": "TASK T4-I12 (Type 2) for T4's WORKING_ITEMS",
    "basis": {
        "code": "ed012c7ccf (read for conventions only; NI algorithm read for item 8)",
        "jr_contract_sha256_at_ed012c7ccf": "4f3fd302229f7889a9469056e516a772596e3f129fa653bc1bd4d9fba596c39d",
        "jr_refutation_sha256_at_ed012c7ccf": "0f8cc31620672a629661cce2affe562e792759f84b7fd5d38c09790ca2d5723f",
        "connector_contract_v1_sha256_at_ed012c7ccf": "2f7939eb1f11eeb42e537d2edb7dd4b98e71fb232eeaebfd2db77152212fc4c4",
        "historical_oracles_sha256_at_ed012c7ccf": "6549d544accae7131127e8a87c748ea514c21b4f4c0381b5cec7c135d9c0d76c",
        "demo_fixture_sha256_at_ed012c7ccf": "986c055944776ca8d0d849d552678e1f2bfb6c4559bbb9ff8a1928157a4f871c",
        "ni_lib_sha256_at_ed012c7ccf": "017d901d5ad7814f2d45609f160db52242c5d87674ddc269bbb08417e6f3c67e",
        "rulings_applied": ["annotation-only joints are not admitted on the exact route", "the connector topology is replaces_span only"]},
    "conventions": {
        "dofs": "d = [u_i(3), theta_i(3), u_j(3), theta_j(3)], global; translations m, infinitesimal rotation vectors rad, right-handed",
        "Q": "row-major 3x3 whose COLUMNS are the connector axes in global coordinates; proper (det +1); for r != 0 the x column is r/|r|",
        "q": "q = [qt; qr] local (connector axes): qt = Q^T[(v_j - v_i) - theta_c x r], qr = Q^T(theta_j - theta_i), v_k = u_k + theta_k x a_k, theta_c = mean rotation; r = (x_j + a_j) - (x_i + a_i)",
        "K_and_H": "K physical (N/m, N/rad, N*m/m, N*m/rad); H = D K D with D = diag(Ls,Ls,Ls,1,1,1); 21-entry upper triangle in order (0,0)..(0,5),(1,1)..(5,5); K = D^-1 H D^-1",
        "g_energy": "g = K(q - q_ref) local; U = (q - q_ref)^T K (q - q_ref)/2",
        "end_actions": "f = B^T g, node-on-element (the element's internal force vector, conjugate to d); element-on-node is -f. Blocks Fi, Mi, Fj, Mj global; F = Q g_t, M = Q g_r; Fi = -F, Fj = F, Mi = -(a_i + r/2) x F - M, Mj = (a_j - r/2) x F + M",
        "installed_state": "assembly RHS receives +B^T K q_ref; the initial internal action at d = 0 is -B^T K q_ref; Ke d = f_ext + B^T K q_ref + reactions",
        "reactions": "support-on-pipe, global, Fx Fy Fz Mx My Mz; a spring's is -k u",
        "reversal": "J = diag(-1,1,-1); Q' = QJ; attachments and node ids swapped; T = blockdiag(-J,-J); q' = Tq, q_ref' = T q_ref, K' = T K T^T, H' = T H T^T at unchanged Ls; end-action blocks exchange",
        "numbers": "exact rationals as strings 'p/q' (authoritative) with decimals; transcendental and pi-dependent values as decimal strings of 30-40 significant digits"},
    "criteria": {
        "fk_unit": "exact identities (B, Ke = B^T K B, rank, null space, self-equilibrium, reversal, covariance): exact in rationals; product comparison at relative 1e-12 (bitwise where every operand is binary64-exact)",
        "system": "both solver modes (sparse_interactive, dense_scrutiny), one reference; relative 1e-9 with explicit zero-scale floors per family",
        "ni": "absolute 1e-12, unchanged",
        "no_new_threshold": True},
    "findings": [
        "All 75 numerical statements of JR (CONTRACT.md section 6 J1, J2; INDEPENDENT_REFUTATION.md sections 1 and 6) and of the historical connector oracle that this reference re-derives agree exactly, except one that agrees only to 13 significant digits: the refutation's finite-rotation decimals (L = 2, phi = 0.1) carry binary64 cancellation error; the exact values are frozen here.",
        "The 658.44 N*m of the legacy demo is historical (legacy model with pressure, nonlinear supports and the deleted element) and is not reproducible on the exact route. In the v3 re-authoring the deleted element's raw-difference measure would leave 770 N*m (L-100), 275 N*m (L-200) about global X and 440 N*m (L-300) about global Y unbalanced; the connector balances exactly.",
        "The v3 re-authoring of the demo must drop the bend, branch, valve and terminal markers, the nonlinear and constant-effort supports, every pressure load and the combination; only the connector, frames, linear supports, the spring hanger and the weight, nodal and thermal loads remain. L-300 is added so that all six connector coordinates are exercised through PP.",
        "NI's replacement keeps the fixture's own frame section (E 100, G 40, A 1, I 1, J 1) and moves node 1 to (3, -4, 0); every structural property of the four call sites (iteration counts, sign flip, cap, zero coefficient) is preserved; the asserted constants change."],
    "jr_comparison_summary": jr_rows,
    "cases": cases,
    "round_01": {
        "requested_by": "T4 WORKING_ITEMS, after the round-0 commit a744c09021 (SHA256SUMS 9925375b...)",
        "ruling_applied": "under load-reference-1 the connector does not use the replaced span's resolved element state (E/nu, eigenstrain); a nonzero resolved eigenstrain on the replaced span, self-weight on it or any load it owns refuses with JOINT_REPLACED_SPAN_LOAD_UNOWNED (SLOT_TABLE S20 applied to 0.4.0); the joint's thermal relation comes with T4-U5's J3",
        "appended_cases": ["U3-SYS-DEMO-CONNECTOR-002-LR1", "U3-SYS-LR1-REPLACED-SPAN-EIGENSTRAIN-REFUSAL", "U3-SYS-REPLACED-SPAN-MATERIAL-CONTROL", "U3-SYS-LR1-REPLACED-SPAN-STORED-UNAPPLIED-CONTROL", "U3-KD5-UTM-SKEW-OFFSET-COUPLED"],
        "rv130_s3": "K-D5's undemoted connector case with nonzero offsets, a skew Q and a coupled K at X0 = 0, 5e6 and 7.3e6 m, with a perturbed-Ke variant that K-D5 must demote (U3-KD5-UTM-SKEW-OFFSET-COUPLED)",
        "further_rulings": ["a state resolving to exactly zero eigenstrain on the replaced span is admitted (no effect on the connector)",
                            "refusal is decided per case by what that case applies: a load on the replaced span listed in a case's load_sources refuses that case; a load stored but applied by no case is not refused and has no effect"],
        "frozen": "the 18 round-0 cases and every round-0 top-level key are unchanged (check_round01.py compares canonical hashes taken from a744c09021)"},
}

with open(sys.argv[1], "w") as fh:
    json.dump(doc, fh, indent=1, sort_keys=False)
    fh.write("\n")

for name, ok in CHECKS:
    print(("ok   " if ok else "FAIL ") + name)
npass = sum(1 for _, ok in CHECKS if ok)
print(f"JR comparisons: {sum(1 for r in jr_rows if r['agrees'])}/{len(jr_rows)} agree")
print(f"checks: {npass}/{len(CHECKS)} pass")
for name, ok in CHECKS:
    if not ok:
        print("FAILED:", name)
sys.exit(0 if npass == len(CHECKS) else 1)
