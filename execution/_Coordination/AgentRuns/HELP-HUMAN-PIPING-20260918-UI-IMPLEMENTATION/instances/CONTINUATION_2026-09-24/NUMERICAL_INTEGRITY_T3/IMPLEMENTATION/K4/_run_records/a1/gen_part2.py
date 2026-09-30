# ----------------------------------------------------------------------------
# E: K4's formation, emulated bit for bit (assemble.rs), and exact rational
# solves of the intended models (the controls' expectations)
# ----------------------------------------------------------------------------
from math import isqrt

r1 = load_module("references", R1_PY)
PI_Q = r1.PI_Q


def rp(x, p):
    """Fraction x rounded once to p bits (nearest, ties to even); 0 -> 0."""
    if x == 0:
        return Fr(0)
    neg = x < 0
    a = -x if neg else x
    e = k3.floor_log2(a)
    sh = p - 1 - e
    n, d = a.numerator, a.denominator
    if sh >= 0:
        num, den = n << sh, d
    else:
        num, den = n, d << (-sh)
    q, r = divmod(num, den)
    if 2 * r > den or (2 * r == den and q & 1):
        q += 1
    v = Fr(q, 1 << sh) if sh >= 0 else Fr(q << (-sh))
    return -v if neg else v


def sqrt_p(x, p):
    """The correctly rounded square root of a Fraction x > 0 at p bits."""
    assert x > 0
    n, d = x.numerator, x.denominator
    k = (p + 4) - (n.bit_length() - d.bit_length()) // 2
    num = n << (2 * k) if k >= 0 else n
    den = d if k >= 0 else d << (-2 * k)
    t = num // den
    s = isqrt(t)
    exact = s * s == t and t * den == num
    val = Fr(2 * s + (0 if exact else 1), 2)
    val = val / (Fr(2) ** k)
    return rp(val, p)


def W_of(x, L):
    """The W holding the Fraction x (which must fit 64L bits exactly)."""
    if x == 0:
        return W.zero(L, False)
    neg = x < 0
    a = -x if neg else x
    n, d = a.numerator, a.denominator
    assert d & (d - 1) == 0
    e = -(d.bit_length() - 1)
    while n % 2 == 0 and n:
        n //= 2
        e += 1
    return W.from_int(L, neg, n, e)


def width_of(p):
    return 4 if p <= 256 else (8 if p <= 512 else 16)


UPPER = [(a, b) for a in range(12) for b in range(a, 12)]


def form_member_em(nodes, m, p):
    """assemble.rs `form_member` at precision p (every rounding mirrored)."""
    F = Fr
    xi, xj = nodes[m["i"]], nodes[m["j"]]

    def dot(a, b):
        return rp(sum((x * y for x, y in zip(a, b)), Fr(0)), p)

    def normalize(v):
        n = sqrt_p(dot(v, v), p)
        return [rp(c / n, p) for c in v], n

    d = [rp(F(xj[k]) - F(xi[k]), p) for k in range(3)]
    ex, L = normalize(d)
    yr = [F(c) for c in m["y"]]
    proj = dot(yr, ex)
    yc = [rp(yr[k] - proj * ex[k], p) for k in range(3)]
    ey, _ = normalize(yc)
    zc = [rp(ex[a] * ey[b] - ex[b] * ey[a], p) for a, b in ((1, 2), (2, 0), (0, 1))]
    ez, _ = normalize(zc)
    inv = rp(Fr(1) / L, p)
    axial = rp(rp(F(m["E"]) * F(m["A"]), p) / L, p)
    torsion = rp(rp(F(m["G"]) * F(m["J"]), p) / L, p)
    bz = rp(rp(F(m["E"]) * F(m["Iz"]), p) / L, p)
    by = rp(rp(F(m["E"]) * F(m["Iy"]), p) / L, p)
    B = [[Fr(0)] * 12 for _ in range(6)]
    for k in range(3):
        iy = rp(inv * ey[k], p)
        iz = rp(inv * ez[k], p)
        B[0][k], B[0][6 + k] = -ex[k], ex[k]
        B[1][3 + k], B[1][9 + k] = -ex[k], ex[k]
        for row, rot in ((2, 3), (3, 9)):
            B[row][k], B[row][6 + k], B[row][rot + k] = iy, -iy, ez[k]
        for row, rot in ((4, 3), (5, 9)):
            B[row][k], B[row][6 + k], B[row][rot + k] = -iz, iz, ey[k]
    drows = [[(0, axial)], [(1, torsion)], [(2, 4 * bz), (3, 2 * bz)], [(2, 2 * bz), (3, 4 * bz)],
             [(4, 4 * by), (5, 2 * by)], [(4, 2 * by), (5, 4 * by)]]
    DB = [[rp(sum((c * B[s][col] for s, c in drows[r]), Fr(0)), p) for col in range(12)] for r in range(6)]
    ke = {}
    for a, b in UPPER:
        ke[(a, b)] = rp(sum((B[r][a] * DB[r][b] for r in range(6)), Fr(0)), p)
    return dict(axes=[ex, ey, ez], L=L, inv=inv, axial=axial, torsion=torsion, bz=bz, by=by, B=B, ke=ke)


def directional_em(s, p):
    n = [Fr(c) for c in s["n"]]
    norm2 = rp(sum((c * c for c in n), Fr(0)), p)
    k = Fr(s["k"])
    out = [[Fr(0)] * 3 for _ in range(3)]
    for a in range(3):
        for b in range(a, 3):
            m = rp(n[a] * n[b], p)
            v = rp(rp(k * m, p) / norm2, p)
            out[a][b] = out[b][a] = v
    return out


def kind_offset(kind):
    return 0 if kind == "t" else 3


def assemble_em(model, p):
    """The assembled K at p: {(r, c) upper: value} (one exact sum per entry)."""
    nodes = model["nodes"]
    contrib = {}

    def add(r, c, v):
        if r <= c:
            contrib.setdefault((r, c), []).append(v)

    members = [form_member_em(nodes, m, p) for m in model["members"]]
    for m, op in zip(model["members"], members):
        dofs = [6 * m["i"] + k for k in range(6)] + [6 * m["j"] + k for k in range(6)]
        for a in range(12):
            for b in range(12):
                r, c = dofs[a], dofs[b]
                if r <= c:
                    contrib.setdefault((r, c), []).append(op["ke"][(min(a, b), max(a, b))])
    for s in model["springs"]:
        d = 6 * s["node"] + s["c"]
        contrib.setdefault((d, d), []).append(Fr(s["k"]))
    for s in model["dsprings"]:
        blk = directional_em(s, p)
        base = 6 * s["node"] + kind_offset(s["kind"])
        for a in range(3):
            for b in range(a, 3):
                contrib.setdefault((base + a, base + b), []).append(blk[a][b])
    return {rc: rp(sum(vs, Fr(0)), p) for rc, vs in contrib.items()}, members


# ---- exact rational solves (the controls' expectations)

def qsqrt_or_none(q):
    n, d = q.numerator, q.denominator
    rn, rd = isqrt(n), isqrt(d)
    if rn * rn == n and rd * rd == d:
        return Fr(rn, rd)
    return None


def skew(e):
    return [[Fr(0), -e[2], e[1]], [e[2], Fr(0), -e[0]], [-e[1], e[0], Fr(0)]]


def local_matrix(m, L):
    F = Fr
    EA, GJ = F(m["E"]) * F(m["A"]) / L, F(m["G"]) * F(m["J"]) / L
    k = [[Fr(0)] * 12 for _ in range(12)]

    def sym(a, b, v):
        k[a][b] = v
        k[b][a] = v
    k[0][0] = k[6][6] = EA
    sym(0, 6, -EA)
    k[3][3] = k[9][9] = GJ
    sym(3, 9, -GJ)
    for idx, I, sgn in (((1, 5, 7, 11), F(m["Iz"]), 1), ((2, 4, 8, 10), F(m["Iy"]), -1)):
        c = F(m["E"]) * I / L ** 3
        pat = [[12, 6 * L * sgn, -12, 6 * L * sgn], [6 * L * sgn, 4 * L * L, -6 * L * sgn, 2 * L * L],
               [-12, -6 * L * sgn, 12, -6 * L * sgn], [6 * L * sgn, 2 * L * L, -6 * L * sgn, 4 * L * L]]
        for a in range(4):
            for b in range(4):
                k[idx[a]][idx[b]] = c * pat[a][b]
    return k


def exact_member(nodes, m):
    """(ke global 12x12, e, L, axes or None) of a member, exact."""
    xi = [Fr(c) for c in nodes[m["i"]]]
    xj = [Fr(c) for c in nodes[m["j"]]]
    d = [b - a for a, b in zip(xi, xj)]
    L = qsqrt_or_none(sum(c * c for c in d))
    assert L is not None, "irrational member length: no exact control"
    e = [c / L for c in d]
    yr = [Fr(c) for c in m["y"]]
    proj = sum(a * b for a, b in zip(yr, e))
    yc = [yr[k] - proj * e[k] for k in range(3)]
    ny = qsqrt_or_none(sum(c * c for c in yc))
    axes = None
    if ny is not None:
        ey = [c / ny for c in yc]
        ez = [e[1] * ey[2] - e[2] * ey[1], e[2] * ey[0] - e[0] * ey[2], e[0] * ey[1] - e[1] * ey[0]]
        axes = [e, ey, ez]
        kl = local_matrix(m, L)
        T = [[Fr(0)] * 12 for _ in range(12)]
        for blk in range(4):
            for r in range(3):
                for c in range(3):
                    T[3 * blk + r][3 * blk + c] = axes[r][c]
        KT = [[sum(kl[i][k] * T[k][j] for k in range(12)) for j in range(12)] for i in range(12)]
        ke = [[sum(T[k][i] * KT[k][j] for k in range(12)) for j in range(12)] for i in range(12)]
    else:
        assert m["Iy"] == m["Iz"], "irrational axes need an isotropic section"
    if ny is None or m["Iy"] == m["Iz"]:
        EA, GJ = Fr(m["E"]) * Fr(m["A"]) / L, Fr(m["G"]) * Fr(m["J"]) / L
        EI = Fr(m["E"]) * Fr(m["Iz"])
        a, b, f4, f2 = 12 * EI / L ** 3, 6 * EI / L ** 2, 4 * EI / L, 2 * EI / L
        P = [[(1 if r == c else 0) - e[r] * e[c] for c in range(3)] for r in range(3)]
        E3 = [[e[r] * e[c] for c in range(3)] for r in range(3)]
        S = skew(e)
        blocks = {}
        Kt = [[EA * E3[r][c] + a * P[r][c] for c in range(3)] for r in range(3)]
        Kri = [[GJ * E3[r][c] + f4 * P[r][c] for c in range(3)] for r in range(3)]
        Krj = [[-GJ * E3[r][c] + f2 * P[r][c] for c in range(3)] for r in range(3)]
        neg = lambda M: [[-x for x in row] for row in M]
        tr = lambda M: [[M[c][r] for c in range(3)] for r in range(3)]
        mS = [[-b * S[r][c] for c in range(3)] for r in range(3)]
        pS = [[b * S[r][c] for c in range(3)] for r in range(3)]
        blocks[(0, 0)], blocks[(0, 2)], blocks[(2, 2)] = Kt, neg(Kt), Kt
        blocks[(1, 1)], blocks[(1, 3)], blocks[(3, 3)] = Kri, Krj, Kri
        blocks[(0, 1)], blocks[(0, 3)], blocks[(2, 1)], blocks[(2, 3)] = mS, mS, pS, pS
        full = [[Fr(0)] * 12 for _ in range(12)]
        for (bi, bj), M in list(blocks.items()):
            for r in range(3):
                for c in range(3):
                    full[3 * bi + r][3 * bj + c] = M[r][c]
                    full[3 * bj + c][3 * bi + r] = M[r][c]
        if ny is not None:
            assert full == ke, "projector form disagrees with T^T K T"
        ke = full
    return ke, e, L, axes


def solve_exact(model):
    """Exact published quantities of a model (keys as in models.txt)."""
    nodes = model["nodes"]
    nn = len(nodes)
    n = 6 * nn
    K = {}
    mem = []
    for m in model["members"]:
        ke, e, L, axes = exact_member(nodes, m)
        dofs = [6 * m["i"] + k for k in range(6)] + [6 * m["j"] + k for k in range(6)]
        for a in range(12):
            for b in range(12):
                if ke[a][b] != 0:
                    K[(dofs[a], dofs[b])] = K.get((dofs[a], dofs[b]), Fr(0)) + ke[a][b]
        mem.append((m, ke, e, L, axes, dofs))
    for s in model["springs"]:
        d = 6 * s["node"] + s["c"]
        K[(d, d)] = K.get((d, d), Fr(0)) + Fr(s["k"])
    for s in model["dsprings"]:
        nv = [Fr(c) for c in s["n"]]
        n2 = sum(c * c for c in nv)
        base = 6 * s["node"] + kind_offset(s["kind"])
        for a in range(3):
            for b in range(3):
                v = Fr(s["k"]) * nv[a] * nv[b] / n2
                if v != 0:
                    K[(base + a, base + b)] = K.get((base + a, base + b), Fr(0)) + v
    f = [Fr(0)] * n
    for l in model["loads"]:
        f[6 * l["node"] + l["c"]] += Fr(l["v"])
    fixed = {6 * c["node"] + c["c"]: Fr(c["v"]) for c in model["constraints"]}
    free = [g for g in range(n) if g not in fixed]
    pos = {g: a for a, g in enumerate(free)}
    nf = len(free)
    A = [[Fr(0)] * nf for _ in range(nf)]
    rhs = [f[g] for g in free]
    for (r, c), v in K.items():
        if r in pos and c in pos:
            A[pos[r]][pos[c]] += v
        elif r in pos and c in fixed:
            rhs[pos[r]] -= v * fixed[c]
    # Gaussian elimination (exact).
    M = [row[:] + [rhs[i]] for i, row in enumerate(A)]
    for col in range(nf):
        piv = next(r for r in range(col, nf) if M[r][col] != 0)
        M[col], M[piv] = M[piv], M[col]
        for r in range(nf):
            if r != col and M[r][col] != 0:
                fac = M[r][col] / M[col][col]
                M[r] = [x - fac * y for x, y in zip(M[r], M[col])]
    u = [Fr(0)] * n
    for g, v in fixed.items():
        u[g] = v
    for a, g in enumerate(free):
        u[g] = M[a][nf] / M[a][a]
    out = {}
    for g in range(n):
        out["u.%d.%d" % (g // 6, g % 6)] = u[g]
    for node in range(nn):
        out["mag.%d" % node] = ("sqrt", sum(u[6 * node + k] ** 2 for k in range(3)))
    for (m, ke, e, L, axes, dofs) in mem:
        ue = [u[d] for d in dofs]
        Fe = [sum(ke[a][b] * ue[b] for b in range(12)) for a in range(12)]
        Fi, Mi, Fj, Mj = Fe[0:3], Fe[3:6], Fe[6:9], Fe[9:12]
        dot3 = lambda a, b: sum(x * y for x, y in zip(a, b))
        perp2 = lambda v: dot3(v, v) - dot3(v, e) ** 2
        mid = m["id"]
        out["N.%d" % mid] = dot3(Fj, e)
        out["T.%d" % mid] = dot3(Mj, e)
        out["Mb.%d.i" % mid] = ("sqrt", perp2(Mi))
        out["Mb.%d.j" % mid] = ("sqrt", perp2(Mj))
        if axes is not None:
            for end, (Fv, Mv) in (("i", (Fi, Mi)), ("j", (Fj, Mj))):
                for c in range(3):
                    out["end.%d.%s.%d" % (mid, end, c)] = dot3(axes[c], Fv)
                    out["end.%d.%s.%d" % (mid, end, 3 + c)] = dot3(axes[c], Mv)
        for st in model["stations"]:
            if st["member"] != mid:
                continue
            t = Fr(st["t"])
            xi = [Fr(c) for c in nodes[m["i"]]]
            xj = [Fr(c) for c in nodes[m["j"]]]
            arm = [t * (b - a) for a, b in zip(xi, xj)]
            cross = [arm[1] * Fi[2] - arm[2] * Fi[1], arm[2] * Fi[0] - arm[0] * Fi[2], arm[0] * Fi[1] - arm[1] * Fi[0]]
            Mx = [-Mi[k] + cross[k] for k in range(3)]
            Fx = [-c for c in Fi]
            out["Mbs.%d" % st["id"]] = ("sqrt", perp2(Mx))
            if axes is not None:
                for c in range(3):
                    out["st.%d.%d" % (st["id"], c)] = dot3(axes[c], Fx)
                    out["st.%d.%d" % (st["id"], 3 + c)] = dot3(axes[c], Mx)
    spring_action = {}
    for s in model["springs"]:
        v = -Fr(s["k"]) * u[6 * s["node"] + s["c"]]
        spring_action[s["id"]] = {s["c"]: v}
        out["spr.%d.%d" % (s["id"], s["c"])] = v
    for s in model["dsprings"]:
        nv = [Fr(c) for c in s["n"]]
        n2 = sum(c * c for c in nv)
        base = 6 * s["node"] + kind_offset(s["kind"])
        proj = sum(nv[b] * u[base + b] for b in range(3))
        acts = {}
        for a in range(3):
            v = -Fr(s["k"]) * proj * nv[a] / n2
            acts[kind_offset(s["kind"]) + a] = v
            out["dspr.%d.%d" % (s["id"], kind_offset(s["kind"]) + a)] = v
        spring_action[s["id"]] = acts
    reaction = {}
    for g in sorted(fixed):
        r = sum(v * u[c] for (rr, c), v in K.items() if rr == g) - f[g]
        reaction[g] = r
        out["R.%d.%d" % (g // 6, g % 6)] = r
    for grp in model["supports"]:
        comp = [Fr(0)] * 6
        for c in range(6):
            g = 6 * grp["node"] + c
            if grp["r"][c]:
                comp[c] += reaction[g]
            for sid in grp["springs"] + grp["dsprings"]:
                comp[c] += spring_action[sid].get(c, Fr(0))
        out["sf.%d" % grp["id"]] = ("sqrt", sum(x * x for x in comp[:3]))
        out["sm.%d" % grp["id"]] = ("sqrt", sum(x * x for x in comp[3:]))
    return out


def to_f64_bits(v):
    if isinstance(v, tuple):
        x = v[1]
        if x == 0:
            return 0
        return b64(float(sqrt_p(x, 90)))
    if v == 0:
        return 0
    return b64(float(v))


# ---- model definitions (invented inputs; D1's probe models and K4's own controls)

def n_section():
    """The N-series section on the intended basis: fl() of R1's exact values."""
    E, G = 200e9, 80e9
    od, idd = Fr("0.2"), Fr("0.18")
    A = PI_Q * (od * od - idd * idd) / 4
    I = PI_Q * (od ** 4 - idd ** 4) / 64
    return dict(E=E, G=G, A=float(A), Iy=float(I), Iz=float(I), J=float(2 * I))


def new_model(name, nodes):
    return dict(name=name, nodes=[tuple(float(c) for c in p) for p in nodes], members=[], springs=[],
                dsprings=[], constraints=[], loads=[], stations=[], supports=[])


def add_member(model, mid, i, j, y=(0.0, 0.0, 1.0), section=None, **over):
    s = dict(section or n_section())
    s.update(over)
    model["members"].append(dict(id=mid, i=i, j=j, y=tuple(float(c) for c in y), **s))


def fix(model, node, comps, value=0.0):
    for c in comps:
        model["constraints"].append(dict(node=node, c=c, v=float(value)))


def spring(model, sid, node, c, k):
    model["springs"].append(dict(id=sid, node=node, c=c, k=float(k)))


def load(model, node, c, v, src=None):
    model["loads"].append(dict(node=node, c=c, v=float(v), src=src or "l%d" % len(model["loads"])))


def pin_case(name, direction, k, moment, members=1, y=(0.0, 0.0, 1.0)):
    """D1's probe `pin_case`: root translations fixed, root rotations on three
    global springs k, a run of members along `direction`, a tip moment."""
    nodes = [tuple(c * s for c in direction) for s in range(members + 1)]
    m = new_model(name, nodes)
    for s in range(members):
        add_member(m, s + 1, s, s + 1, y)
    fix(m, 0, (0, 1, 2))
    for c in (3, 4, 5):
        spring(m, c, 0, c, k)
    for c in range(3):
        if moment[c] != 0:
            load(m, members, 3 + c, moment[c])
    return m


def models():
    out = []
    sec = n_section()
    # N01, N08 (four torques), N09 (bending and torsion).
    m = new_model("N01", [(0, 0, 0), (2, 0, 0)])
    add_member(m, 1, 0, 1)
    fix(m, 0, range(6))
    load(m, 1, 1, 1000.0)
    m["stations"].append(dict(id=1, member=1, t=0.25))
    m["stations"].append(dict(id=2, member=1, t=0.5))
    out.append(m)
    for idx, T in enumerate((1.0, -1.0, 0.1, -0.1)):
        m = new_model("N08-%d" % idx, [(0, 0, 0), (2, 0, 0)])
        add_member(m, 1, 0, 1)
        fix(m, 0, range(6))
        load(m, 1, 3, T)
        out.append(m)
    m = new_model("N09-B", [(0, 0, 0), (10, 0, 0)])
    add_member(m, 1, 0, 1)
    fix(m, 0, range(6))
    load(m, 1, 1, 100.0)
    out.append(m)
    m = new_model("N09-T", [(0, 0, 0), (10, 0, 0)])
    add_member(m, 1, 0, 1)
    fix(m, 0, range(6))
    load(m, 1, 3, 0.1)
    out.append(m)
    # N05, N06, and N05 with a transverse tip force (T3's N05 item).
    for name, k, T, fy in (("N05", 1e-4, 1e-8, None), ("N06", 1e-12, 1e-16, None), ("N05-TRANSVERSE", 1e-4, 1e-8, 1.0)):
        m = new_model(name, [(0, 0, 0), (2, 0, 0)])
        add_member(m, 1, 0, 1)
        fix(m, 0, (0, 1, 2, 4, 5))
        spring(m, 1, 0, 3, k)
        load(m, 1, 3, T)
        if fy is not None:
            load(m, 1, 1, fy)
        m["stations"].append(dict(id=1, member=1, t=0.5))
        m["supports"].append(dict(id=1, node=0, r=[1, 1, 1, 0, 1, 1], springs=[1], dsprings=[]))
        out.append(m)
    # D1's probes (§3.1, §3.2).
    skew = (3.0, 4.0, 0.0)
    mom = lambda k: (1e-8 * k / 1e-4, 2e-8 * k / 1e-4, 0.0)
    out.append(pin_case("SKEW-K1E-28", skew, 1e-28, mom(1e-28)))
    out.append(pin_case("SKEW6-K1E-12", skew, 1e-12, mom(1e-12), members=6))
    out.append(pin_case("SKEW-K1E-12", skew, 1e-12, mom(1e-12)))
    out.append(pin_case("SKEW-K1E-4", skew, 1e-4, mom(1e-4)))
    out.append(pin_case("AXIS-K1E-4", (5.0, 0.0, 0.0), 1e-4, mom(1e-4)))
    out.append(pin_case("OBLIQUE-K1E-4", (2.0, 3.0, 6.0), 1e-4, mom(1e-4), y=(6.0, 2.0, -3.0)))
    m = pin_case("B1-L", skew, 1e-4, (0.0, 0.0, 0.0))
    for v in (1e80, 1e-8, -1e80):
        load(m, 1, 3, v)
    load(m, 1, 4, 2e-8)
    out.append(m)
    for name, loads in (("B1-C-A", [1e80]), ("B1-C-B", [1e-8]), ("B1-C-A2", [1e80]),
                        ("B1-E-A", [1e-8, 1e-53]), ("B1-E-B", [1e-8]), ("B1-E-NET", [1e-53]),
                        ("B1-C-NET", [1e-8])):
        m = pin_case(name, skew, 1e-4, (0.0, 0.0, 0.0))
        for v in loads:
            load(m, 1, 3, v)
        out.append(m)
    for s in (1e-10, 1e-14):
        m = new_model("S8-W-%g" % s, [(0, 0, 0), (3, 4, 0), (6, 8, 0)])
        add_member(m, 1, 0, 1)
        add_member(m, 2, 1, 2, E=sec["E"] * s, G=sec["G"] * s)
        fix(m, 0, range(6))
        for c in range(6):
            spring(m, 1 + c, 2, c, 1e12)
        load(m, 1, 1, 1000.0)
        load(m, 1, 5, 300.0)
        out.append(m)
    # §7.3-16's duplicate-operand control: two identical collinear members meet
    # at node 1 (their node-1 couplings cancel exactly) and a third member,
    # 2^-300 as stiff, leaves node 1 along y. The member list places the weak
    # member between the two, the order in which a sequential fold loses it.
    m = new_model("DUPLICATE", [(0, 0, 0), (2, 0, 0), (4, 0, 0), (2, 2, 0)])
    add_member(m, 1, 0, 1)
    add_member(m, 3, 1, 3, E=sec["E"] * 2.0 ** -300, G=sec["G"] * 2.0 ** -300)
    add_member(m, 2, 1, 2)
    fix(m, 0, range(6))
    fix(m, 2, range(6))
    fix(m, 3, range(6))
    load(m, 1, 5, 1.0)
    out.append(m)
    # A prescribed-motion control (KREV-02 analogue): a two-member cantilever
    # whose root rotates by a prescribed 1e-3 rad about z and whose far end
    # settles by 2e-4 m; no load.
    m = new_model("PRESCRIBED", [(0, 0, 0), (2, 0, 0), (4, 0, 0)])
    add_member(m, 1, 0, 1)
    add_member(m, 2, 1, 2)
    fix(m, 0, (0, 1, 2, 3, 4))
    fix(m, 0, (5,), 1e-3)
    fix(m, 2, (1,), -2e-4)
    fix(m, 2, (0, 2))
    out.append(m)
    # The pivot-escalation control: N05 with k = 2^-120·(GJ/L) (fails at 128,
    # passes at 256).
    a = float(Fr(sec["G"]) * Fr(sec["J"]) / 2)
    m = new_model("PIVOT", [(0, 0, 0), (2, 0, 0)])
    add_member(m, 1, 0, 1)
    fix(m, 0, (0, 1, 2, 4, 5))
    spring(m, 1, 0, 3, a * 2.0 ** -120)
    load(m, 1, 3, a * 2.0 ** -120 * 1e-4)
    out.append(m)
    # The reactions-only control (K4-M16): a member whose two nodes are
    # prescribed the same large translation (a rigid motion: its end actions
    # are exact zeros), and a separate tiny load on a stiff cantilever.
    m = new_model("REACTIONS-ONLY", [(0, 0, 0), (3, 4, 0), (0, 0, 5), (0, 0, 7)])
    add_member(m, 1, 0, 1)
    fix(m, 0, (0,), 1.0)
    fix(m, 1, (0,), 1.0)
    fix(m, 0, (1, 2, 3, 4, 5))
    fix(m, 1, (1, 2, 3, 4, 5))
    add_member(m, 2, 2, 3, y=(1.0, 0.0, 0.0))
    fix(m, 2, range(6))
    load(m, 3, 0, 1e-30)
    out.append(m)
    # K4-M11's control: two collinear spans of lengths 1 and 1 + 2^-40 between
    # fixed ends, a moment at the shared node.
    m = new_model("TWO-SPAN", [(0, 0, 0), (1, 0, 0), (2 + 2.0 ** -40, 0, 0)])
    add_member(m, 1, 0, 1)
    add_member(m, 2, 1, 2)
    fix(m, 0, range(6))
    fix(m, 2, range(6))
    load(m, 1, 5, 1.0)
    out.append(m)
    # A structural-zero kind: a (3,4,0) cantilever under a torque along its axis
    # (every force and bending moment is an exact zero).
    m = new_model("ZERO-TORSION-345", [(0, 0, 0), (3, 4, 0)])
    add_member(m, 1, 0, 1)
    fix(m, 0, range(6))
    load(m, 1, 3, 3.0)
    load(m, 1, 4, 4.0)
    out.append(m)
    # An all-zero body beside a loaded one.
    m = new_model("ALL-ZERO-BODY", [(0, 0, 0), (2, 0, 0), (0, 5, 0), (2, 5, 0)])
    add_member(m, 1, 0, 1)
    add_member(m, 2, 2, 3)
    fix(m, 0, range(6))
    fix(m, 2, range(6))
    load(m, 1, 2, 10.0)
    out.append(m)
    # Directional springs spanning R³ exactly although a binary64 determinant
    # is zero (K4-M26): a cantilever root held by three directional springs.
    eps = 2.0 ** -52
    m = new_model("DIRECTIONAL-SPAN", [(0, 0, 0), (0, 0, 3)])
    add_member(m, 1, 0, 1, y=(1.0, 0.0, 0.0))
    fix(m, 0, (0, 1, 2))
    for sid, n in ((1, (1.0, 1.0, 1.0)), (2, (1.0, 1.0 + eps, 1.0)), (3, (1.0, 1.0, 1.0 + eps))):
        m["dsprings"].append(dict(id=sid, node=0, kind="r", n=n, k=1e6))
    load(m, 1, 3, 1.0)
    out.append(m)
    # A combination-precision control: the k = 1e-28 geometry under a load that
    # does not excite its soft mode (selected at 128), and one that does.
    m = pin_case("SKEW-K1E-28-AXIAL", skew, 1e-28, (0.0, 0.0, 0.0))
    load(m, 1, 0, 3.0)
    load(m, 1, 1, 4.0)
    out.append(m)
    # The ceiling-combination finding: operands P + ε and P with ε/P = 2^-1060.
    for name, loads in (("CEIL-A", [2.0 ** 1000, 2.0 ** -60]), ("CEIL-B", [2.0 ** 1000]), ("CEIL-NET", [2.0 ** -60])):
        m = pin_case(name, skew, 1e-4, (0.0, 0.0, 0.0))
        for v in loads:
            load(m, 1, 3, v)
        out.append(m)
    return out


COMBOS = (
    ("B1-C", ((1.0, "B1-C-A"), (1.0, "B1-C-B"), (-1.0, "B1-C-A2")), "B1-C-NET"),
    ("B1-E", ((1.0, "B1-E-A"), (-1.0, "B1-E-B")), "B1-E-NET"),
    ("PRECISION-RULE", ((1.0, "SKEW-K1E-28-AXIAL"), (1.0, "SKEW-K1E-28")), None),
    ("CEILING", ((1.0, "CEIL-A"), (-1.0, "CEIL-B")), "CEIL-NET"),
)


def model_lines(m, expectations=True):
    lines = ["model %s" % m["name"]]
    for p in m["nodes"]:
        lines.append("node %s %s %s" % tuple(hexf(c) for c in p))
    for mm in m["members"]:
        lines.append("member %d %d %d %s %s %s %s %s %s %s %s %s" % (
            mm["id"], mm["i"], mm["j"], hexf(mm["E"]), hexf(mm["G"]), hexf(mm["A"]), hexf(mm["Iy"]),
            hexf(mm["Iz"]), hexf(mm["J"]), hexf(mm["y"][0]), hexf(mm["y"][1]), hexf(mm["y"][2])))
    for s in m["springs"]:
        lines.append("spring %d %d %d %s" % (s["id"], s["node"], s["c"], hexf(s["k"])))
    for s in m["dsprings"]:
        lines.append("dspring %d %d %s %s %s %s %s" % (s["id"], s["node"], s["kind"], hexf(s["n"][0]),
                                                     hexf(s["n"][1]), hexf(s["n"][2]), hexf(s["k"])))
    for c in m["constraints"]:
        lines.append("constraint %d %d %s" % (c["node"], c["c"], hexf(c["v"])))
    for l in m["loads"]:
        lines.append("load %d %d %s %s" % (l["node"], l["c"], hexf(l["v"]), l["src"]))
    for s in m["stations"]:
        lines.append("station %d %d %s" % (s["id"], s["member"], hexf(s["t"])))
    for g in m["supports"]:
        lines.append("support %d %d %s %s %s" % (g["id"], g["node"], "".join(str(int(x)) for x in g["r"]),
                                                ",".join(map(str, g["springs"])) or "-",
                                                ",".join(map(str, g["dsprings"])) or "-"))
    if expectations:
        try:
            ex = solve_exact(m)
        except AssertionError:
            ex = None
        if ex is not None:
            for key in sorted(ex):
                lines.append("expect %s %016x" % (key, to_f64_bits(ex[key])))
    lines.append("end")
    return lines


FORMATION_MODELS = ("M-AX", "M-345", "M-236", "M-122", "M-OFF", "DUPLICATE", "DIRECTIONAL-SPAN")


def formation_models():
    sec = n_section()
    out = {}
    for name, xj, y in (("M-AX", (2, 0, 0), (0.0, 1.0, 0.0)), ("M-345", (3, 4, 0), (0.0, 0.0, 1.0)),
                        ("M-236", (2, 3, 6), (6.0, -2.0 + 4.0, -3.0)), ("M-122", (1, 2, 2), (1.0, 0.0, 0.0))):
        m = new_model(name, [(0, 0, 0), xj])
        add_member(m, 1, 0, 1, y=y)
        out[name] = m
    m = new_model("M-OFF", [(1.1, -0.7, 2.3), (4.9, 3.3, -1.7)])
    add_member(m, 1, 0, 1, y=(0.3, 1.0, 0.2), Iy=sec["Iy"] * 1.5)
    out["M-OFF"] = m
    for mm in models():
        if mm["name"] in ("DUPLICATE", "DIRECTIONAL-SPAN"):
            out[mm["name"]] = mm
    return out


def formation_lines():
    lines = []
    fm = formation_models()
    for name in FORMATION_MODELS:
        m = fm[name]
        lines += model_lines(m, expectations=False)
        for p, L in K4_PRECISIONS + ((53, 4),):
            K, _ = assemble_em(m, p)
            for (r, c) in sorted(K):
                lines.append("k %s %d %d %d %d %s" % (name, p, L, r, c, W_of(K[(r, c)], L).token()))
    return lines
