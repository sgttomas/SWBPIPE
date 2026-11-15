#!/usr/bin/env python3
"""W2 response-spectrum recovery references: realized arc, massive face pipes, massless connector and rods.

Standard-library Python 3 only. Imports, reads and runs no product code. Besides
the files it writes, it reads only the frozen f1_modal mass_discrete generator
modules elements.py, numeric.py, eigen.py and model.py (reference-side code,
imported unmodified; SHA-256 pinned and checked before import, import origin
asserted). All model data are invented (w2_cases.py). Method: ../DERIVATION.md.

  python3 -I -B generators/w2_recovery.py            check (default): regenerate in memory,
                                                     compare byte for byte, verify SHA256SUMS
  python3 -I -B generators/w2_recovery.py --write    write the outputs and SHA256SUMS
"""
import hashlib
import json
import sys
from decimal import Decimal, localcontext
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
F1 = OUT.parent.parent / "f1_modal" / "mass_discrete" / "generators"
F1_PINS = {
    "elements.py": "c2a7b890c90d8c2d016fc72036aad36165173cb9f146158bea7629364bd696f0",
    "numeric.py": "e08ff24066f636f0d6c76e38ae95de2ef42e9e1c0f651714910034575d5dd1e7",
    "eigen.py": "22ae605c36ae47ad7e80369ecbee422646bef67acb1d46294a744228589d17c6",
    "model.py": "c219d158aa2aa260153dff303514508a4c41e4983b2b1c5236a2beb79c046761",
}
for _name, _h in F1_PINS.items():
    _got = hashlib.sha256((F1 / _name).read_bytes()).hexdigest()
    if _got != _h:
        sys.exit(f"f1_modal generator {_name} changed (sha256 {_got}); these references pin {_h}")
sys.path[:] = [str(F1), str(HERE)] + [p for p in sys.path if p and Path(p).resolve() not in (F1.resolve(), HERE)]

import elements as el  # noqa: E402  (f1_modal)
import eigen as eg  # noqa: E402  (f1_modal)
import model as f1m  # noqa: E402  (f1_modal)
import numeric as f1n  # noqa: E402  (f1_modal)
from model import DOFS, Model, form_mass, form_stiffness, section_basis  # noqa: E402
from numeric import QPi, QPiField, PointField, exact_double, pi_decimal, set_point_precision  # noqa: E402
import w2_cases as cases  # noqa: E402

for _mod in (el, eg, f1m, f1n):
    assert Path(_mod.__file__).resolve() == (F1 / Path(_mod.__file__).name).resolve(), _mod.__file__
assert Path(cases.__file__).resolve().parent == HERE

QF = QPiField()
PF = PointField()
AXES = ["X", "Y", "Z"]
STATE = {"prec": 80}


# ============================================================================ numbers
def fr(s):
    return exact_double(s)


def zeta_exact():
    """The damping ratio is the binary64 nearest 0.05 (3602879701896397/72057594037927936), taken exactly."""
    z = Fraction(float(cases.ZETA))
    assert z == Fraction(3602879701896397, 72057594037927936)
    return z


def D(x):
    """Any exact or point value -> Decimal at the working precision."""
    if isinstance(x, Decimal):
        return +x
    if isinstance(x, QPi):
        return x.to_decimal(STATE["prec"])
    if isinstance(x, (int, Fraction)):
        return f1n.to_decimal(Fraction(x))
    raise TypeError(type(x))


def Dmat(A):
    return [[D(x) for x in r] for r in A]


def zeros(n, m=None):
    m = n if m is None else m
    return [[Decimal(0)] * m for _ in range(n)]


def mv(A, x):
    return [sum((a * b for a, b in zip(r, x) if a != 0), Decimal(0)) for r in A]


def dot(a, b):
    return sum((x * y for x, y in zip(a, b)), Decimal(0))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def tr(A):
    return [list(r) for r in zip(*A)]


def mm(A, B):
    Bt = tr(B)
    return [[sum((a * b for a, b in zip(r, c)), type(r[0])(0)) for c in Bt] for r in A]


def maxabs(v):
    return max((abs(x) for x in v), default=Decimal(0))


def skew(v):
    z = type(v[0])(0)
    return [[z, -v[2], v[1]], [v[2], z, -v[0]], [-v[1], v[0], z]]


# ============================================================================ connector and rods (exact, in Q)
def connector_frame(r, guide):
    """Exact rational frame: x along r; y from the integer guide made normal to x; z = x cross y.

    Requires |r| and the normalized y to be rational (true here: r is axis-aligned and the
    guide's normal part has an integer-square norm). Returns Q with columns the axes.
    """
    rr = sum(x * x for x in r)
    nr = f1m.exact_sqrt(rr)
    assert nr is not None, "connector chord length must be rational for the exact frame"
    x = [v / nr for v in r]
    g = [Fraction(v) for v in guide]
    p = sum(a * b for a, b in zip(g, x))
    y = [g[k] - p * x[k] for k in range(3)]
    ny = f1m.exact_sqrt(sum(v * v for v in y))
    assert ny is not None, "guide's normal part must have a rational norm for the exact frame"
    y = [v / ny for v in y]
    z = cross(x, y)
    Q = [[x[k], y[k], z[k]] for k in range(3)]  # columns are axes
    QtQ = mm(tr(Q), Q)
    assert QtQ == [[Fraction(int(a == b)) for b in range(3)] for a in range(3)], "frame not orthonormal"
    assert x[0] * (y[1] * z[2] - y[2] * z[1]) - x[1] * (y[0] * z[2] - y[2] * z[0]) + x[2] * (y[0] * z[1] - y[1] * z[0]) == 1
    return Q


def connector_B_closed(ai, aj, r, Q):
    """q = B d, d = (u_i, th_i, u_j, th_j): q_t = Q^T[(u_j + th_j x a_j) - (u_i + th_i x a_i) - th_c x r],
    q_r = Q^T (th_j - th_i), th_c = (th_i + th_j)/2. With th x a = -[a]x th and -th_c x r = [r]x th_c:
    dq_t/du_i = -Q^T, dq_t/dth_i = Q^T([a_i]x + [r]x/2), dq_t/du_j = Q^T, dq_t/dth_j = Q^T(-[a_j]x + [r]x/2).
    """
    QT = tr(Q)
    I3 = [[Fraction(int(a == b)) for b in range(3)] for a in range(3)]
    Z3 = [[Fraction(0)] * 3 for _ in range(3)]
    sa, sb, sr = skew(ai), skew(aj), skew([v / 2 for v in r])
    neg = lambda A: [[-x for x in row] for row in A]  # noqa: E731
    add = lambda A, B: [[x + y for x, y in zip(ra, rb)] for ra, rb in zip(A, B)]  # noqa: E731
    blocks_t = [neg(I3), add(sa, sr), I3, add(neg(sb), sr)]
    blocks_r = [Z3, neg(I3), Z3, I3]
    Bt = [[] for _ in range(3)]
    Br = [[] for _ in range(3)]
    for blk_t, blk_r in zip(blocks_t, blocks_r):
        mt, mr = mm(QT, blk_t), mm(QT, blk_r)
        for k in range(3):
            Bt[k] += mt[k]
            Br[k] += mr[k]
    return Bt + Br


def connector_q_kinematic(d, ai, aj, r, Q):
    ui, ti, uj, tj = d[0:3], d[3:6], d[6:9], d[9:12]
    vi = [ui[k] + cross(ti, ai)[k] for k in range(3)]
    vj = [uj[k] + cross(tj, aj)[k] for k in range(3)]
    tc = [(ti[k] + tj[k]) / 2 for k in range(3)]
    tcr = cross(tc, r)
    QT = tr(Q)
    qt = [sum(QT[a][b] * (vj[b] - vi[b] - tcr[b]) for b in range(3)) for a in range(3)]
    qr = [sum(QT[a][b] * (tj[b] - ti[b]) for b in range(3)) for a in range(3)]
    return qt + qr


def connector(xi, xj, spec):
    ai = [fr(v) for v in spec["offset_i_global_m"]]
    aj = [fr(v) for v in spec["offset_j_global_m"]]
    assert ai == [0, 0, 0] and aj == [0, 0, 0], "W2 uses zero attachment offsets"
    pi_, pj_ = [xi[k] + ai[k] for k in range(3)], [xj[k] + aj[k] for k in range(3)]
    r = [pj_[k] - pi_[k] for k in range(3)]
    Q = connector_frame(r, spec["y_guide_integer"])
    B = connector_B_closed(ai, aj, r, Q)
    cols = []
    for k in range(12):
        e = [Fraction(0)] * 12
        e[k] = Fraction(1)
        cols.append(connector_q_kinematic(e, ai, aj, r, Q))
    assert tr(cols) == B, "connector B: closed form differs from the kinematic definition"
    Ls = fr(spec["translation_scale_m"])
    dsc = [Ls, Ls, Ls, Fraction(1), Fraction(1), Fraction(1)]
    H = [fr(v) for v in spec["H_diagonal"]]
    assert all(h > 0 for h in H)
    Kc = [[(H[a] / (dsc[a] * dsc[b]) if a == b else Fraction(0)) for b in range(6)] for a in range(6)]
    Ke = mm(mm(tr(B), Kc), B)
    assert Ke == tr(Ke)
    # rigid-body motions give q = 0 exactly
    for k in range(3):
        t = [Fraction(int(c == k)) for c in range(3)]
        assert [sum(B[a][c] * v for c, v in enumerate(t + [0] * 3 + t + [0] * 3)) for a in range(6)] == [0] * 6
        w = t
        ui, uj = cross(w, xi), cross(w, xj)
        d = ui + w + uj + w
        assert [sum(B[a][c] * d[c] for c in range(12)) for a in range(6)] == [0] * 6
    return {"Q": Q, "r": r, "B": B, "K_conn": Kc, "Ke": Ke, "H": H, "Ls": Ls, "a_i": ai, "a_j": aj}


def rod(xi, xj, offset, k):
    """Linear axial rod between rigid offsets a at both ends: e = n.(v_j - v_i), v = u + th x a;
    n.(th x a) = th.(a x n), so b = (-n, -(a x n), n, a x n), K_rod = k b b^T, T = k (e - e_ref)."""
    pi_ = [xi[c] + offset[c] for c in range(3)]
    pj_ = [xj[c] + offset[c] for c in range(3)]
    d = [pj_[c] - pi_[c] for c in range(3)]
    L = f1m.exact_sqrt(sum(v * v for v in d))
    assert L is not None
    n = [v / L for v in d]
    an = cross(offset, n)
    b = [-v for v in n] + [-v for v in an] + n + an
    Kr = [[k * b[p] * b[q] for q in range(12)] for p in range(12)]
    return {"n": n, "length": L, "b": b, "K": Kr, "p_i": pi_, "p_j": pj_, "offset": offset, "k": k}


# ============================================================================ the model
class System:
    def __init__(self, vid, prec):
        self.vid = vid
        var = cases.VARIANTS[vid]
        self.var = var
        self.node_ids = list(var["nodes"])
        spec_members = []
        for mid in var["members"]:
            m = dict(cases.PIPE_MEMBERS[mid])
            m.update({"id": mid, "section": "s3", "material": "steel"})
            spec_members.append(m)
        pipe_spec = {
            "nodes": [{"id": n, "xyz": cases.NODES[n]} for n in self.node_ids],
            "materials": cases.MATERIAL,
            "sections": cases.SECTION,
            "members": spec_members,
            "supports": [{"node": a, "restrained": list(DOFS)} for a in var["anchors"]],
            "springs": [],
        }
        full_members = list(spec_members)
        if var["connector"]:
            full_members.append({"id": cases.CONNECTOR["id"], "kind": "connector",
                                 "i": cases.CONNECTOR["i"], "j": cases.CONNECTOR["j"]})
        self.pipe_model = Model(pipe_spec)
        self.model = Model(dict(pipe_spec, members=full_members))
        m = self.model
        self.n = m.n
        self.free = m.free
        self.restrained = sorted(m.restrained)
        self.xyz = {nid: m.xyz[m.idx[nid]] for nid in self.node_ids}
        self.dofs = {nid: m.dofs_of(nid) for nid in self.node_ids}
        self._mass()
        self._stiffness()

    # ---------------------------------------------------------------- mass
    def _mass(self):
        M, contribs, cats = form_mass(self.model, PF, "EX")
        M_pf, _, _ = form_mass(self.model, PF, "PF")
        assert M == M_pf, "PF and EX mass bases must coincide here (c_m = c_a = 0)"
        self.M = M
        self.cats = cats
        self.mass_constituents = []
        for c in contribs:
            if c.get("massless"):
                self.mass_constituents.append({"id": c["id"], "kind": c["kind"], "massless": True})
            elif c["kind"] == "straight":
                self.mass_constituents.append({"id": c["id"], "kind": "straight", "dofs": c["dofs"],
                                               "M": c["matrix"], "mbar": c["mbar"], "length": c["length"]})
            elif c["kind"] == "arc":
                for side, lump in zip(("i", "j"), c["lumps"]):
                    self.mass_constituents.append({
                        "id": f"{c['id']}.lump_{side}", "kind": "arc_half_lump", "member": c["id"], "side": side,
                        "node": lump["node"], "dofs": lump["dofs"], "M": lump["matrix"], "mass": lump["mass"],
                        "first_moment": lump["first_moment"],
                        "centreline_second_moment": lump["centreline_second_moment"],
                        "section_rotary_inertia": lump["section_rotary_inertia"]})
                self.arc_geo = c["geometry"]
                self.arc_mbar, self.arc_jbar, self.arc_ibar = c["mbar"], c["jbar"], c["ibar"]
            else:
                raise ValueError(c)

    # ---------------------------------------------------------------- stiffness
    def _stiffness(self):
        m = self.model
        n = self.n
        K = zeros(n)
        self.k_constituents = []
        sec, mat = cases.SECTION["s3"], cases.MATERIAL["steel"]
        sb = section_basis(sec, mat, "EX")
        sb_pf = section_basis(sec, mat, "PF")
        assert all(sb[k] == sb_pf[k] for k in ("A", "I", "J", "metal", "contents", "insulation", "mbar", "jbar",
                                               "ibar")), "PF and EX section bases must coincide here"
        self.section = sb
        E, G = fr(mat["elastic_modulus_Pa"]), fr(mat["shear_modulus_Pa"])
        for mid in self.var["members"]:
            mem = m.members[mid]
            dofs = m.dofs_of(mem["i"]) + m.dofs_of(mem["j"])
            if mem["kind"] == "straight":
                e, L, Le = m.straight_axis(QF, mem)
                assert Le is not None
                pi = QF.pi()
                blk = el.straight_stiffness(QF, e, L, QF.q(E * sb["A"]) * pi, QF.q(E * sb["I"]) * pi,
                                            QF.q(G * sb["J"]) * pi)
                blkD = Dmat(blk)
                self.k_constituents.append({"id": mid, "kind": "straight", "dofs": dofs, "K": blkD,
                                            "K_exact": blk, "axis": [Fraction(v.a) for v in e], "length": Le})
            else:
                geo = m.arc_geometry(PF, mem)
                pi = PF.pi()
                k = PF.q(fr(mem["flexibility_factor"]))
                blkD = el.arc_stiffness(PF, geo, D(E * sb["A"]) * pi, D(E * sb["I"]) * pi, D(G * sb["J"]) * pi, k)
                self.k_constituents.append({"id": mid, "kind": "arc", "dofs": dofs, "K": blkD, "geometry": geo,
                                            "k": fr(mem["flexibility_factor"]), "R": fr(mem["radius_m"])})
            f1m.add_block(K, dofs, blkD)
        if self.var["connector"]:
            cs = cases.CONNECTOR
            xi, xj = self.xyz[cs["i"]], self.xyz[cs["j"]]
            con = connector(xi, xj, cs)
            dofs = m.dofs_of(cs["i"]) + m.dofs_of(cs["j"])
            con.update({"id": cs["id"], "kind": "connector", "dofs": dofs, "dofs_nodes": (cs["i"], cs["j"])})
            self.connector = con
            f1m.add_block(K, dofs, Dmat(con["Ke"]))
            self.k_constituents.append(dict(con, K=Dmat(con["Ke"])))
        else:
            self.connector = None
        self.rods = []
        if self.var["rods"]:
            Q = self.connector["Q"]
            ydir = [Q[k][1] for k in range(3)]
            rho = fr(cases.ROD_OFFSET_M)
            kr = fr(cases.ROD_STIFFNESS_N_PER_M)
            for rs in cases.RODS:
                off = [rs["offset_sign"] * rho * v for v in ydir]
                rd = rod(self.xyz[rs["i"]], self.xyz[rs["j"]], off, kr)
                dofs = m.dofs_of(rs["i"]) + m.dofs_of(rs["j"])
                rd.update({"id": rs["id"], "kind": "rod", "dofs": dofs})
                self.rods.append(rd)
                f1m.add_block(K, dofs, Dmat(rd["K"]))
                self.k_constituents.append(dict(rd, K=Dmat(rd["K"])))
        self.K = K
        # cross-check: pipe part against f1's form_stiffness (route through f1's own assembly)
        Kp = form_stiffness(self.pipe_model, PF, "EX")
        Kmine = zeros(n)
        for c in self.k_constituents:
            if c["kind"] in ("straight", "arc"):
                f1m.add_block(Kmine, c["dofs"], c["K"])
        scale = max(maxabs(r) for r in Kp)
        self.check_pipe_K = max(abs(a - b) for ra, rb in zip(Kp, Kmine) for a, b in zip(ra, rb)) / scale


def build_systems(prec):
    set_point_precision(prec)
    STATE["prec"] = prec
    return {vid: System(vid, prec) for vid in cases.VARIANTS}


# ============================================================================ modal basis
def reduce(A, idx):
    return [[A[i][j] for j in idx] for i in idx]


def modal(S):
    Kr, Mr = reduce(S.K, S.free), reduce(S.M, S.free)
    pairs, sweeps = eg.route_b(Kr, Mr)
    modes = []
    for lam0, phi in pairs:
        Mphi, Kphi = mv(Mr, phi), mv(Kr, phi)
        lam = dot(phi, Kphi) / dot(phi, Mphi)
        modes.append({"lam": lam, "phi_r": phi})
    modes.sort(key=lambda t: t["lam"])
    return modes, Kr, Mr




TAU_T = Decimal("1e-6")
TIE_REL = Decimal("1e-25")
REPEAT_REL = Decimal("1e-30")
DELTA_C = Decimal(1) / Decimal(2 ** 20)


def two_pi():
    return 2 * pi_decimal(STATE["prec"])


def solve_spd(A, b):
    R = eg.cholesky(A)
    return eg.backward_t(R, eg.forward(R, b))


def expand(S, vr):
    v = [Decimal(0)] * S.n
    for k, d in enumerate(S.free):
        v[d] = vr[k]
    return v


def rigid_translation(S, k):
    r = [Decimal(0)] * S.n
    for nid in S.node_ids:
        r[S.dofs[nid][k]] = Decimal(1)
    return r


def rigid_rotation(S, j):
    """Rigid small rotation about global axis j through the origin, over all DOFs."""
    w = [Decimal(int(c == j)) for c in range(3)]
    r = [Decimal(0)] * S.n
    for nid in S.node_ids:
        x = [D(v) for v in S.xyz[nid]]
        u = cross(w, x)
        for c in range(3):
            r[S.dofs[nid][c]] = u[c]
            r[S.dofs[nid][3 + c]] = w[c]
    return r


def bbox_diag(S):
    return D(S.model.bbox_diagonal_sq()).sqrt()


def modal_basis(S):
    modes, Kr, Mr = modal(S)
    lams = [m["lam"] for m in modes]
    for a, b in zip(lams, lams[1:]):
        assert (b - a) > REPEAT_REL * b, f"{S.vid}: exactly repeated eigenvalue; W2 models are designed without one"
    ell = bbox_diag(S)
    Mfull = S.M
    T = [dot(rigid_translation(S, k), mv(Mfull, rigid_translation(S, k))) for k in range(3)]
    p = [[mv(Mfull, rigid_translation(S, k))[d] for d in S.free] for k in range(3)]
    out = []
    tp = two_pi()
    for i, m in enumerate(modes):
        phi = expand(S, m["phi_r"])
        trans = [(d, phi[d]) for nid in S.node_ids for d in S.dofs[nid][:3]]
        rots = [(d, phi[d]) for nid in S.node_ids for d in S.dofs[nid][3:]]
        tmax = max(abs(v) for _, v in trans)
        rmax = max(abs(v) for _, v in rots)
        case = "a" if tmax > TAU_T * ell * rmax else "b"
        pool = trans if case == "a" else rots
        big = max(abs(v) for _, v in pool)
        ties = [d for d, v in pool if abs(v) >= big * (1 - TIE_REL)]
        dn = min(ties)
        scale = phi[dn]
        phi = [v / scale for v in phi]
        phi_r = [phi[d] for d in S.free]
        gm = dot(phi_r, mv(Mr, phi_r))
        gam = [dot(phi_r, p[k]) / gm for k in range(3)]
        meff = [dot(phi_r, p[k]) ** 2 / gm for k in range(3)]
        lam = m["lam"]
        omega = lam.sqrt()
        out.append({
            "index": i + 1, "lam": lam, "omega": omega, "f": omega / tp, "T": tp / omega,
            "phi": phi, "phi_r": phi_r, "case": case, "norm_dof": dn, "ties": sorted(ties) if len(ties) > 1 else [],
            "gm": gm, "gamma": gam, "meff": meff,
        })
    # residual and orthogonality checks (80-digit; recorded)
    res = Decimal(0)
    knorm = max(sum(abs(x) for x in r) for r in Kr)
    for md in out:
        r = [a - md["lam"] * b for a, b in zip(mv(Kr, md["phi_r"]), mv(Mr, md["phi_r"]))]
        res = max(res, maxabs(r) / (knorm * maxabs(md["phi_r"])))
    orth = Decimal(0)
    for a in range(len(out)):
        for b in range(a + 1, len(out)):
            va, vb = out[a]["phi_r"], out[b]["phi_r"]
            orth = max(orth, abs(dot(va, mv(Mr, vb))) / (out[a]["gm"] * out[b]["gm"]).sqrt())
    # Sturm counts at lam_1/2, the geometric mean of each gap and 2 lam_max
    shifts = [lams[0] / 2] + [(lams[k] * lams[k + 1]).sqrt() for k in range(len(lams) - 1)] + [2 * lams[-1]]
    expected = list(range(len(lams) + 1))
    counts = [eg.sturm_count(PF, Kr, Mr, s)[0] for s in shifts]
    assert counts == expected, f"{S.vid}: Sturm counts {counts} != {expected}"
    # sum of effective masses over every finite mode vs p^T M_ff^-1 p
    sumchk = Decimal(0)
    pMp = []
    for k in range(3):
        y = solve_spd(Mr, p[k])
        v = dot(p[k], y)
        pMp.append(v)
        s = sum((md["meff"][k] for md in out), Decimal(0))
        sumchk = max(sumchk, abs(s - v) / T[k])
    return {"modes": out, "Kr": Kr, "Mr": Mr, "T": T, "p": p, "pMp": pMp, "ell": ell,
            "checks": {"eigen_residual": res, "m_orthogonality": orth, "sturm_counts": counts,
                       "sum_meff_vs_pMp": sumchk}}


def scope_clusters(modes):
    """DEL-04-09 clusters: adjacent eigenvalues chained by lam_{i+1} - lam_i <= delta_c lam_{i+1}."""
    groups = [[modes[0]["index"]]]
    for a, b in zip(modes, modes[1:]):
        gap = (b["lam"] - a["lam"]) / b["lam"]
        assert not (DELTA_C / 2 <= gap <= 2 * DELTA_C), "gap too close to delta_c to classify robustly"
        if gap <= DELTA_C:
            groups[-1].append(b["index"])
        else:
            groups.append([b["index"]])
    return groups


# ============================================================================ spectra
def table(spec):
    return [(fr(x), fr(y)) for x, y in spec["points"]]


def ordinate(spec, f):
    """Ordinate at frequency f (Decimal). Period tables are located by the period T = 1/f."""
    pts = table(spec)
    x = f if spec["abscissa"] == "frequency_Hz" else 1 / f
    xs = [D(a) for a, _ in pts]
    ys = [D(b) for _, b in pts]
    if x < xs[0] or x > xs[-1]:
        return None
    for k in range(len(xs) - 1):
        if xs[k] <= x <= xs[k + 1]:
            break
    for xx in xs:
        assert abs(x - xx) > Decimal("1e-30") * xx, "query too close to a knot"
    x1, x2, y1, y2 = xs[k], xs[k + 1], ys[k], ys[k + 1]
    if spec["x_interp"] == "linear":
        t = (x - x1) / (x2 - x1)
    else:
        t = (x.ln() - x1.ln()) / (x2.ln() - x1.ln())
    if spec["y_interp"] == "linear":
        return y1 + t * (y2 - y1)
    return (y1.ln() + t * (y2.ln() - y1.ln())).exp()


def rho_cqc(zeta, wi, wj):
    r = wj / wi
    z2 = zeta * zeta
    return 8 * z2 * (1 + r) * r * r.sqrt() / ((1 - r * r) ** 2 + 4 * z2 * r * (1 + r) ** 2)


# ============================================================================ recovery
class Recovery:
    """Keys, units and signed recovery of every published quantity from (u, g): u the displacement,
    g the acceleration field whose inertia load is F = M g (g = w^2 u for a mode)."""

    def __init__(self, S):
        self.S = S
        self.keys, self.unit = [], {}

        def add(k, u):
            self.keys.append(k)
            self.unit[k] = u

        for nid in S.node_ids:
            for c, dname in enumerate(DOFS):
                add(f"{nid}.{dname}", "m" if c < 3 else "rad")
        comps = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"]
        self.end_keys = {}
        for c in S.k_constituents:
            if c["kind"] in ("straight", "arc", "connector", "rod"):
                ks = [f"{c['id']}.{e}.{q}" for e in ("i", "j") for q in comps]
                self.end_keys[c["id"]] = ks
                for k in ks:
                    add(k, "N" if k.split(".")[-1].startswith("F") else "N*m")
            if c["kind"] == "arc":
                for part in ("elastic", "lump_load"):
                    for e in ("i", "j"):
                        for q in comps:
                            add(f"{c['id']}.{part}.{e}.{q}", "N" if q.startswith("F") else "N*m")
                for e in ("i", "j"):
                    for q in comps:
                        add(f"{c['id']}.{e}.{q}_t", "N" if q.startswith("F") else "N*m")
            if c["kind"] == "connector":
                for q, u in (("N", "N"), ("Vy", "N"), ("Vz", "N"), ("T", "N*m"), ("My", "N*m"), ("Mz", "N*m")):
                    add(f"{c['id']}.g.{q}", u)
            if c["kind"] == "rod":
                add(f"{c['id']}.T", "N")
        for a in S.var["anchors"]:
            for q in comps:
                add(f"{a}.{q}", "N" if q.startswith("F") else "N*m")
        self.Bd = Dmat(S.connector["B"]) if S.connector else None
        self.Kcd = Dmat(S.connector["K_conn"]) if S.connector else None
        if any(c["kind"] == "arc" for c in S.k_constituents):
            self.tangent = arc_tangent_frames(S)

    def __call__(self, u, g):
        S = self.S
        F = mv(S.M, g)
        out = {}
        for nid in S.node_ids:
            for c, dname in enumerate(DOFS):
                out[f"{nid}.{dname}"] = u[S.dofs[nid][c]]
        lumps = {mc["side"]: mc for mc in S.mass_constituents if mc["kind"] == "arc_half_lump"}
        mstraight = {mc["id"]: mc for mc in S.mass_constituents if mc["kind"] == "straight"}
        ends_by = {}
        for c in S.k_constituents:
            ue = [u[d] for d in c["dofs"]]
            elastic = mv(c["K"], ue)
            if c["kind"] == "straight":
                mc = mstraight[c["id"]]
                inert = mv(mc["M"], [g[d] for d in mc["dofs"]])
                total = [a - b for a, b in zip(elastic, inert)]
            elif c["kind"] == "arc":
                li = mv(lumps["i"]["M"], [g[d] for d in lumps["i"]["dofs"]])
                lj = mv(lumps["j"]["M"], [g[d] for d in lumps["j"]["dofs"]])
                inert = li + lj
                total = [a - b for a, b in zip(elastic, inert)]
                for part, vec in (("elastic", elastic), ("lump_load", inert)):
                    for k, key in enumerate(self.end_keys[c["id"]]):
                        out[key.replace(f"{c['id']}.", f"{c['id']}.{part}.", 1)] = vec[k]
                rot = rotate_ends(self.tangent, total)
                for k, key in enumerate(self.end_keys[c["id"]]):
                    out[key + "_t"] = rot[k]
            elif c["kind"] == "connector":
                q = mv(self.Bd, ue)
                gq = mv(self.Kcd, q)
                total = mv(tr(self.Bd), gq)
                for k, nm in enumerate(("N", "Vy", "Vz", "T", "My", "Mz")):
                    out[f"{c['id']}.g.{nm}"] = gq[k]
                ends_by[c["id"] + ".chk"] = max(abs(a - b) for a, b in zip(total, elastic))
            elif c["kind"] == "rod":
                b = [D(x) for x in c["b"]]
                T = D(c["k"]) * dot(b, ue)
                out[f"{c['id']}.T"] = T
                total = [x * T for x in b]
                ends_by[c["id"] + ".chk"] = max(abs(a - bb) for a, bb in zip(total, elastic))
            for k, key in enumerate(self.end_keys[c["id"]]):
                out[key] = total[k]
            ends_by[c["id"]] = total
        Ku = mv(S.K, u)
        resid = [a - b for a, b in zip(Ku, F)]
        for a in S.var["anchors"]:
            for c, q in enumerate(["Fx", "Fy", "Fz", "Mx", "My", "Mz"]):
                out[f"{a}.{q}"] = resid[S.dofs[a][c]]
        return out, F, resid, ends_by


def arc_tangent_frames(S):
    """U4a end-tangent frames, formed two ways: from the chord frame by the stated rotations, and from
    the arc's own geometry (traversal tangent, toward-centre normal, binormal)."""
    c = next(c for c in S.k_constituents if c["kind"] == "arc")
    mem = S.model.members[c["id"]]
    xi = [D(v) for v in S.xyz[mem["i"]]]
    xj = [D(v) for v in S.xyz[mem["j"]]]
    R = D(c["R"])
    d = [b - a for a, b in zip(xi, xj)]
    L = dot(d, d).sqrt()
    xc = [v / L for v in d]
    yr = [D(fr(v)) for v in mem["y_reference"]]
    pr = dot(yr, xc)
    yc = [a - pr * b for a, b in zip(yr, xc)]
    ny = dot(yc, yc).sqrt()
    yc = [v / ny for v in yc]
    zc = cross(xc, yc)
    s = L / (2 * R)
    co = (1 - s * s).sqrt()
    rot_i = [[co, s, Decimal(0)], [s, -co, Decimal(0)], [Decimal(0), Decimal(0), Decimal(-1)]]
    rot_j = [[co, -s, Decimal(0)], [-s, -co, Decimal(0)], [Decimal(0), Decimal(0), Decimal(-1)]]
    chord = [xc, yc, zc]
    Ti = [[sum(rot_i[a][m] * chord[m][k] for m in range(3)) for k in range(3)] for a in range(3)]
    Tj = [[sum(rot_j[a][m] * chord[m][k] for m in range(3)) for k in range(3)] for a in range(3)]
    # geometric construction, independent of the rotation rows
    h = (R * R - L * L / 4).sqrt()
    centre = [(a + b) / 2 - h * y for a, b, y in zip(xi, xj, yc)]
    e1 = [(a - cc) / R for a, cc in zip(xi, centre)]
    ej = [(a - cc) / R for a, cc in zip(xj, centre)]
    nrm = cross(e1, ej)
    nn = dot(nrm, nrm).sqrt()
    e3 = [v / nn for v in nrm]
    t_i = cross(e3, e1)
    t_j = cross(e3, ej)
    Gi = [t_i, [-v for v in e1], cross(t_i, [-v for v in e1])]
    Gj = [t_j, [-v for v in ej], cross(t_j, [-v for v in ej])]
    dev = max(abs(Ti[a][k] - Gi[a][k]) for a in range(3) for k in range(3))
    dev = max(dev, max(abs(Tj[a][k] - Gj[a][k]) for a in range(3) for k in range(3)))
    orth = Decimal(0)
    for T in (Ti, Tj):
        for a in range(3):
            for b in range(3):
                orth = max(orth, abs(dot(T[a], T[b]) - int(a == b)))
        det = dot(T[0], cross(T[1], T[2]))
        orth = max(orth, abs(det - 1))
    return {"i": Ti, "j": Tj, "chord": chord, "rot_i": rot_i, "rot_j": rot_j, "s": s, "c": co,
            "centre": centre, "e1": e1, "ej": ej, "e3": e3,
            "checks": {"rotation_rows_vs_geometry": dev, "orthonormal_right_handed": orth}}


def rotate_ends(tf, v):
    """(F_i, M_i, F_j, M_j) global -> each in its end-tangent frame (rows of the frame matrix)."""
    out = []
    for blk, T in ((0, tf["i"]), (3, tf["i"]), (6, tf["j"]), (9, tf["j"])):
        w = v[blk:blk + 3]
        out += [dot(T[a], w) for a in range(3)]
    return out


# ============================================================================ independent half-arc quadrature
def own_pi(prec):
    """Machin's formula with its own arctan series (independent of f1's pi)."""
    with localcontext() as ctx:
        ctx.prec = prec + 15

        def atan_inv(n):
            x = Decimal(1) / n
            x2 = x * x
            s, term, k = x, x, 1
            eps = Decimal(10) ** (-(prec + 12))
            while abs(term) > eps:
                term = -term * x2
                k += 2
                s += term / k
            return s

        v = 16 * atan_inv(5) - 4 * atan_inv(239)
    return +v


def own_sincos(x):
    with localcontext() as ctx:
        ctx.prec = STATE["prec"] + 20
        x2 = x * x
        eps = Decimal(10) ** (-(STATE["prec"] + 18))
        s, term, k = x, x, 1
        while abs(term) > eps:
            term = -term * x2 / ((k + 1) * (k + 2))
            k += 2
            s += term
        c, term, k = Decimal(1), Decimal(1), 0
        while abs(term) > eps:
            term = -term * x2 / ((k + 1) * (k + 2))
            k += 2
            c += term
    return +s, +c


def own_asin(s):
    """Newton on sin a = s from a binary64-free start (s itself), own sin/cos."""
    a = s
    for _ in range(200):
        sa, ca = own_sincos(a)
        da = (sa - s) / ca
        a -= da
        if abs(da) < Decimal(10) ** (-(STATE["prec"] + 5)):
            break
    return a


def gauss_legendre(n):
    """Nodes and weights on [-1, 1] by Newton on P_n, initial guesses from the own cosine."""
    pi = own_pi(STATE["prec"] + 10)
    out = []
    with localcontext() as ctx:
        ctx.prec = STATE["prec"] + 20
        for i in range(1, n + 1):
            x = own_sincos(pi * (Decimal(i) - Decimal("0.25")) / (Decimal(n) + Decimal("0.5")))[1]
            for _ in range(100):
                p0, p1 = Decimal(1), x
                for k in range(2, n + 1):
                    p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
                dp = n * (x * p1 - p0) / (x * x - 1)
                dx = p1 / dp
                x -= dx
                if abs(dx) < Decimal(10) ** (-(STATE["prec"] + 15)):
                    break
            p0, p1 = Decimal(1), x
            for k in range(2, n + 1):
                p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
            dp = n * (x * p1 - p0) / (x * x - 1)
            out.append((+x, +(2 / ((1 - x * x) * dp * dp))))
    return out


class HalfArcQuadrature:
    """The rigid half-arc's inertia resultant about its end node, integrated directly: for node
    acceleration (g_t, g_th), force density mbar (g_t + g_th x rho), moment density
    rho x f + (jbar t t^T + ibar (I - t t^T)) g_th, ds = R dpsi. Geometry from the arc's own inputs."""

    def __init__(self, S, rec, n=48):
        tf = rec.tangent
        c = next(c for c in S.k_constituents if c["kind"] == "arc")
        self.R = D(c["R"])
        self.alpha = own_asin(tf["s"])
        e1, e3 = tf["e1"], tf["e3"]
        e2 = cross(e3, e1)
        pi = own_pi(STATE["prec"])
        sb = S.section
        self.mbar, self.jbar, self.ibar = D(sb["mbar"]) * pi, D(sb["jbar"]) * pi, D(sb["ibar"]) * pi
        mem = S.model.members[c["id"]]
        self.nodes = {"i": mem["i"], "j": mem["j"]}
        self.xn = {"i": [D(v) for v in S.xyz[mem["i"]]], "j": [D(v) for v in S.xyz[mem["j"]]]}
        gl = gauss_legendre(n)
        self.pts = {}
        for side, lo, hi in (("i", Decimal(0), self.alpha), ("j", self.alpha, 2 * self.alpha)):
            pts = []
            for x, w in gl:
                psi = (lo + hi) / 2 + (hi - lo) / 2 * x
                sp, cp = own_sincos(psi)
                pos = [tf["centre"][k] + self.R * (cp * e1[k] + sp * e2[k]) for k in range(3)]
                t = [-sp * e1[k] + cp * e2[k] for k in range(3)]
                rho = [pos[k] - self.xn[side][k] for k in range(3)]
                pts.append((rho, t, w * (hi - lo) / 2 * self.R))
            self.pts[side] = pts
        self.e1, self.ej = tf["e1"], tf["ej"]
        self.centre = tf["centre"]

    def wrench(self, side, gt, gth):
        F = [Decimal(0)] * 3
        Mm = [Decimal(0)] * 3
        for rho, t, w in self.pts[side]:
            a = [gt[k] + cross(gth, rho)[k] for k in range(3)]
            f = [self.mbar * v for v in a]
            mom = cross(rho, f)
            tg = dot(t, gth)
            sec = [self.jbar * tg * t[k] + self.ibar * (gth[k] - tg * t[k]) for k in range(3)]
            for k in range(3):
                F[k] += w * f[k]
                Mm[k] += w * (mom[k] + sec[k])
        return F + Mm

    def closed_form_weight(self):
        """Both halves under unit translation along each axis: total mass and centroid in closed form."""
        mtot = self.mbar * self.R * 2 * self.alpha
        bis = [a + b for a, b in zip(self.e1, self.ej)]
        nb = dot(bis, bis).sqrt()
        sa, _ = own_sincos(self.alpha)
        cen = [self.centre[k] + self.R * sa / self.alpha * bis[k] / nb for k in range(3)]
        return mtot, cen


def half_arc_check(S, hq, g):
    """Max relative difference between the lumps' M g and the direct integral, both halves."""
    worst = Decimal(0)
    for mc in S.mass_constituents:
        if mc["kind"] != "arc_half_lump":
            continue
        gn = [g[d] for d in mc["dofs"]]
        lump = mv(mc["M"], gn)
        quad = hq.wrench(mc["side"], gn[:3], gn[3:])
        sc = max(maxabs(lump), maxabs(quad))
        if sc > 0:
            worst = max(worst, maxabs([a - b for a, b in zip(lump, quad)]) / sc)
    return worst


# ============================================================================ combinations
def combine(rule, units, zeta):
    """units: list of (f_low, omega_low, value) per cluster."""
    v = [x for _, _, x in units]
    if not units:
        return Decimal(0)
    if rule == "SRSS":
        return sum((x * x for x in v), Decimal(0)).sqrt()
    if rule == "ABSOLUTE_SUM":
        return sum((abs(x) for x in v), Decimal(0))
    if rule == "CQC":
        s = Decimal(0)
        for a in range(len(units)):
            for b in range(len(units)):
                s += rho_cqc(zeta, units[a][1], units[b][1]) * v[a] * v[b]
        return s.sqrt() if s > 0 else Decimal(0)
    if rule == "GROUPING":
        s = sum((x * x for x in v), Decimal(0))
        for grp in groups_of([f for f, _, _ in units]):
            for ia in range(len(grp)):
                for ib in range(ia + 1, len(grp)):
                    s += 2 * abs(v[grp[ia]] * v[grp[ib]])
        return s.sqrt()
    if rule == "TEN_PERCENT":
        s = sum((x * x for x in v), Decimal(0))
        fs = [f for f, _, _ in units]
        for a in range(len(units)):
            for b in range(a + 1, len(units)):
                lo, hi = min(fs[a], fs[b]), max(fs[a], fs[b])
                if (hi - lo) / lo <= Decimal("0.1"):
                    s += 2 * abs(v[a] * v[b])
        return s.sqrt()
    raise ValueError(rule)


def groups_of(fs):
    order = sorted(range(len(fs)), key=lambda k: fs[k])
    groups, cur, f0 = [], [], None
    for k in order:
        if cur and (fs[k] - f0) / f0 <= Decimal("0.1"):
            cur.append(k)
        else:
            if cur:
                groups.append(cur)
            cur, f0 = [k], fs[k]
    groups.append(cur)
    return [g for g in groups if len(g) > 1]


def total_rule(rule, p, r):
    if rule == "SRSS":
        return (p * p + r * r).sqrt()
    if rule == "ABSOLUTE_SUM":
        return p + abs(r)
    raise ValueError(rule)


def directional(rule, vals):
    if rule == "SRSS":
        return sum((x * x for x in vals), Decimal(0)).sqrt()
    if rule == "ABSOLUTE_SUM":
        return sum(vals, Decimal(0))
    if rule == "100-40-40":
        f = Decimal(2) / Decimal(5)
        return max(vals[a] + f * sum((vals[b] for b in range(len(vals)) if b != a), Decimal(0))
                   for a in range(len(vals)))
    raise ValueError(rule)


MODAL_RULES = ["SRSS", "CQC", "GROUPING", "TEN_PERCENT", "ABSOLUTE_SUM"]
RIGID_RULES = ["SRSS", "ABSOLUTE_SUM", "OFF"]
DIR_RULES = ["SRSS", "100-40-40", "ABSOLUTE_SUM"]


# ============================================================================ analysis
def analyse(S, mb, rec, req):
    modes = mb["modes"]
    zeta = D(zeta_exact())
    fc = D(fr(req["cutoff_Hz"]))
    f_r = D(fr(req["rigid_frequency_Hz"]))
    clusters = scope_clusters(modes)
    extracted = [m for m in modes if m["f"] <= fc]
    nx = len(extracted)
    assert nx < len(modes) and modes[nx]["f"] > fc
    ext_ids = {m["index"] for m in extracted}
    ext_clusters = [c for c in clusters if c[0] in ext_ids]
    assert all(set(c) <= ext_ids for c in ext_clusters), "cut-off inside a cluster"
    by = {m["index"]: m for m in modes}
    low = {tuple(c): by[c[0]] for c in ext_clusters}
    periodic_clusters = [c for c in ext_clusters if low[tuple(c)]["f"] < f_r]
    assert low[tuple(ext_clusters[-1])]["f"] >= f_r, "extraction premise: the extracted set must reach f_r"
    periodic = [i for c in periodic_clusters for i in c]
    for m in extracted:  # 10 % boundary and f_r margins
        assert abs(m["f"] - f_r) > Decimal("1e-6") * f_r
    lows = [low[tuple(c)]["f"] for c in ext_clusters]
    for a in range(len(lows)):
        for b in range(a + 1, len(lows)):
            assert abs((lows[b] - lows[a]) / lows[a] - Decimal("0.1")) > Decimal("1e-6")
    an = {"_by": by, "periodic": periodic, "extracted": [m["index"] for m in extracted], "clusters": ext_clusters,
          "f_r": f_r, "f_c": fc, "zeta": zeta}
    contrib, rigid, rigid_load, ords, checks = {}, {}, {}, {}, {}
    hq = HalfArcQuadrature(S, rec) if any(c["kind"] == "arc" for c in S.k_constituents) else None
    eq = {"nodal_balance_free": Decimal(0), "nodal_balance_restrained": Decimal(0), "global_force": Decimal(0),
          "global_moment": Decimal(0), "connector_BTg_vs_Ke_u": Decimal(0), "rod_bT_vs_Krod_u": Decimal(0),
          "half_arc_lump_vs_quadrature": Decimal(0), "rigid_total_reaction": Decimal(0),
          "rigid_vs_modal_expansion": Decimal(0), "rigid_load_shares_sum": Decimal(0)}

    def equilibrium(out, F, resid, ends_by, u):
        scF = max(maxabs(F), maxabs(resid), Decimal("1e-300"))
        A = [Decimal(0)] * S.n
        for c in S.k_constituents:
            for k, d in enumerate(c["dofs"]):
                A[d] += ends_by[c["id"]][k]
        for d in range(S.n):
            if d in S.free:
                eq["nodal_balance_free"] = max(eq["nodal_balance_free"], abs(A[d]) / scF)
            else:
                eq["nodal_balance_restrained"] = max(eq["nodal_balance_restrained"], abs(A[d] - resid[d]) / scF)
        for k in range(3):
            tot = sum((resid[d] for a in S.var["anchors"] for d in [S.dofs[a][k]]), Decimal(0))
            exp = -dot(rigid_translation(S, k), F)
            eq["global_force"] = max(eq["global_force"], abs(tot - exp) / scF)
            mom = Decimal(0)
            for a in S.var["anchors"]:
                x = [D(v) for v in S.xyz[a]]
                fv = [resid[S.dofs[a][c]] for c in range(3)]
                mv_ = [resid[S.dofs[a][3 + c]] for c in range(3)]
                mom += cross(x, fv)[k] + mv_[k]
            expm = -dot(rigid_rotation(S, k), F)
            eq["global_moment"] = max(eq["global_moment"], abs(mom - expm) / scF)
        for key, v in ends_by.items():
            if key.endswith(".chk"):
                tag = "connector_BTg_vs_Ke_u" if key.startswith("J") else "rod_bT_vs_Krod_u"
                eq[tag] = max(eq[tag], v / scF)

    for kdir, (ax, sid, sc) in enumerate(cases.DIRECTIONS):
        spec = cases.SPECTRA[sid]
        s = D(fr(sc))
        ords[ax] = {}
        contrib[ax] = {}
        for m in extracted:
            S_f = ordinate(spec, m["f"])
            assert S_f is not None, "every extracted mode lies inside the tabulated range by design"
            ords[ax][m["index"]] = (S_f, s * S_f)
            amp = m["gamma"][kdir] * s * S_f / m["lam"]
            u = [amp * v for v in m["phi"]]
            g = [m["lam"] * v for v in u]
            out, F, resid, ends_by = rec(u, g)
            equilibrium(out, F, resid, ends_by, u)
            if hq:
                eq["half_arc_lump_vs_quadrature"] = max(eq["half_arc_lump_vs_quadrature"], half_arc_check(S, hq, g))
            contrib[ax][m["index"]] = out
        S_r = ordinate(spec, f_r)
        a_r = s * S_r
        ords[ax]["rigid"] = (S_r, a_r)
        Rk = rigid_translation(S, kdir)
        g = list(Rk)
        for i in periodic:
            m = by[i]
            g = [a - m["gamma"][kdir] * b for a, b in zip(g, m["phi"])]
        g = [a_r * v for v in g]
        F = mv(S.M, g)
        uf = solve_spd(mb["Kr"], [F[d] for d in S.free])
        u = expand(S, uf)
        out, F2, resid, ends_by = rec(u, g)
        equilibrium(out, F2, resid, ends_by, u)
        if hq:
            eq["half_arc_lump_vs_quadrature"] = max(eq["half_arc_lump_vs_quadrature"], half_arc_check(S, hq, g))
        rigid[ax] = out
        # total reaction along k vs -a_r (T_k - sum_periodic m_eff)
        tot = sum((resid[S.dofs[a][kdir]] for a in S.var["anchors"]), Decimal(0))
        exp = -a_r * (mb["T"][kdir] - sum((by[i]["meff"][kdir] for i in periodic), Decimal(0)))
        eq["rigid_total_reaction"] = max(eq["rigid_total_reaction"], abs(tot - exp) / abs(a_r * mb["T"][kdir]))
        # static solve vs the modal expansion over every non-periodic finite mode
        ue = [Decimal(0)] * S.n
        for m in modes:
            if m["index"] in periodic:
                continue
            c = a_r * m["gamma"][kdir] / m["lam"]
            ue = [a + c * b for a, b in zip(ue, m["phi"])]
        eq["rigid_vs_modal_expansion"] = max(eq["rigid_vs_modal_expansion"],
                                             maxabs([a - b for a, b in zip(ue, u)]) / maxabs(u))
        # rigid load over all DOFs and its shares per mass-bearing constituent
        shares = {}
        ssum = [Decimal(0)] * S.n
        for mc in S.mass_constituents:
            if mc.get("massless"):
                continue
            v = mv(mc["M"], [g[d] for d in mc["dofs"]])
            shares[mc["id"]] = (mc["dofs"], v)
            for k, d in enumerate(mc["dofs"]):
                ssum[d] += v[k]
        eq["rigid_load_shares_sum"] = max(eq["rigid_load_shares_sum"],
                                          maxabs([a - b for a, b in zip(ssum, F)]) / maxabs(F))
        rigid_load[ax] = {"g": g, "F": F, "shares": shares}

    # combinations
    keys = rec.keys

    def units_for(ax, key, ids_clusters):
        us = []
        for c in ids_clusters:
            lowm = by[c[0]]
            us.append((lowm["f"], lowm["omega"], sum((contrib[ax][i][key] for i in c), Decimal(0))))
        return us

    periodic_combined = {}
    correction_off = {}
    for rule in MODAL_RULES:
        periodic_combined[rule] = {ax: {k: combine(rule, units_for(ax, k, periodic_clusters), zeta) for k in keys}
                                   for ax, _, _ in cases.DIRECTIONS}
        correction_off[rule] = {ax: {k: combine(rule, units_for(ax, k, ext_clusters), zeta) for k in keys}
                                for ax, _, _ in cases.DIRECTIONS}

    def per_dir(mrule, rrule):
        res = {}
        for ax, _, _ in cases.DIRECTIONS:
            if rrule == "OFF":
                res[ax] = dict(correction_off[mrule][ax])
            else:
                res[ax] = {k: total_rule(rrule, periodic_combined[mrule][ax][k], rigid[ax][k]) for k in keys}
        return res

    grid = []
    for mr in MODAL_RULES:
        for rr in RIGID_RULES:
            pdv = per_dir(mr, rr)
            for dr in DIR_RULES:
                comb = {k: directional(dr, [pdv[ax][k] for ax, _, _ in cases.DIRECTIONS]) for k in keys}
                grid.append(({"modal": mr, "rigid": rr, "directional": dr}, comb))
    base = per_dir(cases.BASE_RULES["modal"], cases.BASE_RULES["rigid"])
    base_comb = {k: directional(cases.BASE_RULES["directional"], [base[ax][k] for ax, _, _ in cases.DIRECTIONS])
                 for k in keys}
    rhos = {}
    for a in periodic_clusters:
        for b in periodic_clusters:
            if a[0] < b[0]:
                rhos[f"{a[0]}-{b[0]}"] = rho_cqc(zeta, by[a[0]]["omega"], by[b[0]]["omega"])
    groups_p = [[periodic_clusters[k][0] for k in g] for g in groups_of([by[c[0]]["f"] for c in periodic_clusters])]
    groups_o = [[ext_clusters[k][0] for k in g] for g in groups_of([by[c[0]]["f"] for c in ext_clusters])]
    # missing-mass disclosure
    mm_disc = {}
    for kdir, ax in enumerate(AXES):
        T = mb["T"][kdir]
        per = sum((by[i]["meff"][kdir] for i in periodic), Decimal(0)) / T
        allx = sum((by[i]["meff"][kdir] for i in an["extracted"]), Decimal(0)) / T
        allf = sum((m["meff"][kdir] for m in modes), Decimal(0)) / T
        mm_disc[ax] = {"T_k_kg": T, "periodic_cumulative_fraction": per, "periodic_unrepresented_fraction": 1 - per,
                       "correction_off_cumulative_fraction": allx, "correction_off_unrepresented_fraction": 1 - allx,
                       "all_finite_modes_fraction": allf, "fraction_moving_with_supports": 1 - mb["pMp"][kdir] / T}
    an.update({"ordinates": ords, "contrib": contrib, "rigid": rigid, "rigid_load": rigid_load,
               "periodic_combined": periodic_combined, "correction_off": correction_off, "base_per_direction": base,
               "base_combined": base_comb, "grid": grid, "rho": rhos, "groups_periodic": groups_p,
               "groups_correction_off": groups_o, "missing_mass": mm_disc, "equilibrium": eq,
               "periodic_clusters": periodic_clusters})
    return an


# ============================================================================ controls
def fixed_state_control(S, an):
    """Affine pressure/reference terms vanish in the increment at fixed K, B and geometry (exact in Q).

    g_state(d) = K_c (B d - q_ref); joint face action f(d) = B^T g_state(d) + thrust pair P (-x, +x),
    P = p Ae; rod T(d) = k (b.d - e_ref) with e_ref = 0 in this family. The increments from any held
    state d0 are K_c B delta, B^T K_c B delta and k b.delta, whatever p, Ae, q_ref and d0.
    """
    con = S.connector
    hs = cases.HELD_STATE
    P = fr(hs["pressure_Pa"]) * fr(hs["effective_area_m2"])
    qref = [fr(v) for v in hs["q_ref"]]
    d0 = [fr(v) for v in hs["d0_joint_ends"]]
    B, Kc = con["B"], con["K_conn"]
    x = [con["Q"][k][0] for k in range(3)]
    thrust = [-P * v for v in x] + [Fraction(0)] * 3 + [P * v for v in x] + [Fraction(0)] * 3

    def g_state(d, q):
        q_ = [sum(B[a][c] * d[c] for c in range(12)) - q[a] for a in range(6)]
        return [sum(Kc[a][b] * q_[b] for b in range(6)) for a in range(6)]

    def face(d, q, th):
        gg = g_state(d, q)
        return [sum(B[a][c] * gg[a] for a in range(6)) + th[c] for c in range(12)]

    checked, equal = 0, 0
    for ax in an["contrib"]:
        responses = list(an["contrib"][ax].values()) + [an["rigid"][ax]]
        for out in responses:
            delta = [Fraction(out[f"{nid}.{dn}"]) for nid in (con["dofs_nodes"]) for dn in DOFS]
            inc = [Fraction(0)] * 6
            for a in range(6):
                inc[a] = sum(Kc[a][b] * sum(B[b][c] * delta[c] for c in range(12)) for b in range(6))
            for q, th, dd in ((qref, thrust, d0), ([Fraction(0)] * 6, [Fraction(0)] * 12, [Fraction(0)] * 12)):
                g1 = g_state([a + b for a, b in zip(dd, delta)], q)
                g0 = g_state(dd, q)
                f1 = face([a + b for a, b in zip(dd, delta)], q, th)
                f0 = face(dd, q, th)
                incf = [sum(B[a][c] * inc[a] for a in range(6)) for c in range(12)]
                checked += 1
                if [a - b for a, b in zip(g1, g0)] == inc and [a - b for a, b in zip(f1, f0)] == incf:
                    equal += 1
            for rd in S.rods:
                T1 = rd["k"] * sum(b * (d + e) for b, d, e in zip(rd["b"], d0, delta))
                T0 = rd["k"] * sum(b * d for b, d in zip(rd["b"], d0))
                checked += 1
                equal += int(T1 - T0 == rd["k"] * sum(b * e for b, e in zip(rd["b"], delta)))
    g0 = g_state(d0, qref)
    names = ("N", "Vy", "Vz", "T", "My", "Mz")
    keys = [f"{con['id']}.g.{nm}" for nm in names]
    face_keys = [f"{con['id']}.{e}.{q}" for e in ("i", "j") for q in ("Fx", "Fy", "Fz", "Mx", "My", "Mz")]
    f0 = face(d0, qref, thrust)

    def base_with(adds):
        zeta = an["zeta"]
        by_ax = {}
        for ax, _, _ in cases.DIRECTIONS:
            res = {}
            for key, add in adds.items():
                units = []
                for c in an["periodic_clusters"]:
                    lowm = an["_by"][c[0]]
                    units.append((lowm["f"], lowm["omega"],
                                  sum((an["contrib"][ax][i][key] + add for i in c), Decimal(0))))
                p_ = combine(cases.BASE_RULES["modal"], units, zeta)
                res[key] = total_rule(cases.BASE_RULES["rigid"], p_, an["rigid"][ax][key] + add)
            by_ax[ax] = res
        return {k: directional(cases.BASE_RULES["directional"], [by_ax[ax][k] for ax, _, _ in cases.DIRECTIONS])
                for k in adds}

    mutants = {
        "static_connector_force_added_to_each_contribution": {k: D(v) for k, v in zip(keys, g0)},
        "thrust_folded_into_connector_axial_force": {keys[0]: D(P)},
        "static_face_action_with_thrust_added_to_each_contribution": {k: D(v) for k, v in zip(face_keys, f0)},
    }
    mres = {}
    for name, adds in mutants.items():
        mut = base_with(adds)
        rels = {k: abs(mut[k] - an["base_combined"][k]) / an["base_combined"][k] for k in adds
                if an["base_combined"][k] != 0}
        mres[name] = {"added_constant": adds, "mutant_combined": mut,
                      "reference_combined": {k: an["base_combined"][k] for k in adds},
                      "smallest_relative_distance": min(rels.values()) if rels else None}
    return {"P_N": P, "q_ref": qref, "d0": d0, "g_state_at_d0": g0, "face_action_at_d0": f0,
            "increments_checked": checked, "increments_exactly_equal": equal, "mutants": mres}


def replaced_span_mutants(S0, mb0, req):
    """Double ownership of the replaced span D-E: its pipe mass, its pipe stiffness, or both, added
    (forbidden by DEL-03-06 / DEL-04-09 REQ-001; shown here as discriminators)."""
    import copy
    cs = cases.CONNECTOR
    out = {}
    sb = S0.section
    E, G = fr(cases.MATERIAL["steel"]["elastic_modulus_Pa"]), fr(cases.MATERIAL["steel"]["shear_modulus_Pa"])
    xi, xj = S0.xyz[cs["i"]], S0.xyz[cs["j"]]
    d = [b - a for a, b in zip(xi, xj)]
    L = f1m.exact_sqrt(sum(v * v for v in d))
    e = [QF.q(v / L) for v in d]
    pi = QF.pi()
    Ks = Dmat(el.straight_stiffness(QF, e, QF.q(L), QF.q(E * sb["A"]) * pi, QF.q(E * sb["I"]) * pi,
                                    QF.q(G * sb["J"]) * pi))
    Ms = Dmat(el.straight_mass(QF, e, QF.q(L), QF.q(sb["mbar"]) * pi, QF.q(sb["ibar"]) * pi,
                               QF.q(sb["jbar"]) * pi))
    dofs = S0.dofs[cs["i"]] + S0.dofs[cs["j"]]
    for name, add_m, add_k in (("span_mass_added", True, False), ("span_stiffness_added", False, True),
                               ("span_mass_and_stiffness_added", True, True)):
        S = copy.copy(S0)
        S.K = [list(r) for r in S0.K]
        S.M = [list(r) for r in S0.M]
        S.mass_constituents = list(S0.mass_constituents)
        S.k_constituents = list(S0.k_constituents)
        if add_m:
            f1m.add_block(S.M, dofs, Ms)
        if add_k:
            f1m.add_block(S.K, dofs, Ks)
        S.mass_constituents.append({"id": "SPAN-DE", "kind": "straight", "dofs": dofs,
                                    "M": Ms if add_m else zeros(12)})
        if add_k:
            S.k_constituents.append({"id": "SPAN-DE", "kind": "straight", "dofs": dofs, "K": Ks})
        mb = modal_basis(S)
        rec = {"T_k_kg": mb["T"], "first_frequencies_Hz": [m["f"] for m in mb["modes"][:8]]}
        try:
            an = analyse(S, mb, Recovery(S), req)
            rec["admissible_under_the_base_request"] = True
            rec["base_combined_selected"] = {}
            sel = [k for k in an["base_combined"] if k.startswith(("J1.g.", "A.", "F.", "D.u", "E.u"))]
            for k in sel:
                rec["base_combined_selected"][k] = an["base_combined"][k]
        except AssertionError as exc:
            rec["admissible_under_the_base_request"] = False
            rec["reason"] = str(exc)
        out[name] = rec
    return out


QUANTITY_FAMILIES = {
    "<node>.<ux|uy|uz|rx|ry|rz>": "displacement or rotation relative to the supports, global axes",
    "<P1|P2|P3>.<i|j>.<Fx..Mz>": ("generalized force acting on the straight element at its end node, global "
                                  "axes: (K_e - w^2 M_e) u per mode, K_e u - M_e g in the rigid response"),
    "BEND1.elastic.<i|j>.<Fx..Mz>": "K_arc u: the arc's elastic end action on the element, global axes",
    "BEND1.lump_load.<i|j>.<Fx..Mz>": ("M_lump g: the half-arc lump's share of the inertia load, attributed to "
                                       "that end node, moments about the end node, global axes"),
    "BEND1.<i|j>.<Fx..Mz>": "total arc end action on the element, elastic minus lump_load, global axes",
    "BEND1.<i|j>.<Fx..Mz>_t": ("the same total in that end's U4a end-tangent frame (x the traversal tangent, y "
                               "toward the centre, z = x cross y), node-on-element sign"),
    "J1.g.<N|Vy|Vz|T|My|Mz>": ("incremental work-conjugate connector force g = K_c B u in connector axes "
                               "(axial, lateral y, lateral z, torsion, bending y, bending z); it excludes static "
                               "pressure thrust, preload and reference load"),
    "J1.<i|j>.<Fx..Mz>": "B^T g: incremental connector action on the connector at its end node, global axes",
    "<R1|R2>.T": "incremental rod axial force k b.u, tension positive (no pretension in this family)",
    "<R1|R2>.<i|j>.<Fx..Mz>": "b T: incremental rod action on the rod at its end node, global axes, moment about the node",
    "<A|C|D|F>.<Fx..Mz>": "reaction applied by the anchor to the piping, global axes: (K u - M g) at its DOFs",
    "signed blocks": "contributions_signed and rigid_response_signed are signed contributions, not peak responses",
    "combined blocks": "every *_combined value and every mutation entry is an unsigned combined magnitude",
}


# ============================================================================ publication
FLUSH = Decimal("1e-50")
GRAY = Decimal("1e-35")
DIG = 40
ROUNDING = {"count": 0, "min_margin": None}


def b64_margin(x):
    """Relative distance of x from the nearest binary64 rounding boundary (a midpoint)."""
    import math
    d = float(x)
    lo, hi = math.nextafter(d, -math.inf), math.nextafter(d, math.inf)
    mids = [(Decimal(d) + Decimal(lo)) / 2, (Decimal(d) + Decimal(hi)) / 2]
    return min(abs(x - m) for m in mids) / abs(x)


def pv(x, scale):
    """[40-digit decimal, nearest binary64], or '0' below 1e-50 of the unit scale."""
    x = D(x) if not isinstance(x, Decimal) else x
    if scale == 0 or abs(x) < FLUSH * scale:
        return "0"
    if abs(x) < GRAY * scale:
        raise ValueError(f"value {x} lies between 1e-50 and 1e-35 of its scale {scale}")
    m = b64_margin(x)
    ROUNDING["count"] += 1
    if ROUNDING["min_margin"] is None or m < ROUNDING["min_margin"]:
        ROUNDING["min_margin"] = m
    assert m > Decimal("1e-60"), "binary64 rounding undecided at the working precision"
    return [format(x, f".{DIG - 1}e"), float(x)]


def pexact(x):
    if isinstance(x, QPi):
        return x.exact_str()
    return str(Fraction(x))


def res(x):
    return None if x is None else f"{float(x):.2e}"


def pub_map(d, unit_of):
    scale = {}
    for k, v in d.items():
        u = unit_of(k)
        scale[u] = max(scale.get(u, Decimal(0)), abs(v))
    return {k: pv(v, scale[unit_of(k)]) for k, v in d.items()}


def pub_nested(dd, unit_of):
    """{outer: {key: value}} with one scale per unit over the whole block."""
    scale = {}
    for inner in dd.values():
        for k, v in inner.items():
            u = unit_of(k)
            scale[u] = max(scale.get(u, Decimal(0)), abs(v))
    return {o: {k: pv(v, scale[unit_of(k)]) for k, v in inner.items()} for o, inner in dd.items()}


def pub_matrix(A):
    sc = max(maxabs(r) for r in A)
    return [[pv(x, sc) for x in r] for r in A]


def dof_names(S, idx):
    out = []
    for d in idx:
        nid = S.node_ids[d // 6]
        out.append(f"{nid}.{DOFS[d % 6]}")
    return out


def dof_unit(k):
    q = k.split(".")[-1]
    return {"ux": "m", "uy": "m", "uz": "m", "rx": "rad", "ry": "rad", "rz": "rad"}[q]


def force_unit(k):
    q = k.split(".")[-1]
    return "N" if q in ("ux", "uy", "uz") or q.startswith("F") else "N*m"


def inputs_binary64():
    rows = {}

    def add(label, s):
        x = fr(s)
        rows[label] = {"decimal": s, "hex": float(x).hex(), "rational": str(x)}

    for k, v in cases.SECTION["s3"].items():
        if v is not None:
            add(f"section.s3.{k}", v)
    for k, v in cases.MATERIAL["steel"].items():
        add(f"material.steel.{k}", v)
    for nid, xyz in cases.NODES.items():
        for c, v in zip("xyz", xyz):
            add(f"node.{nid}.{c}", v)
    for mid, m in cases.PIPE_MEMBERS.items():
        for c, v in zip("xyz", m["y_reference"]):
            add(f"member.{mid}.y_reference.{c}", v)
        for k in ("radius_m", "flexibility_factor"):
            if k in m:
                add(f"member.{mid}.{k}", m[k])
    cs = cases.CONNECTOR
    for k, v in enumerate(cs["H_diagonal"]):
        add(f"connector.H_diagonal.{k}", v)
    add("connector.translation_scale_m", cs["translation_scale_m"])
    add("rod.offset_m", cases.ROD_OFFSET_M)
    add("rod.stiffness_N_per_m", cases.ROD_STIFFNESS_N_PER_M)
    z = zeta_exact()
    rows["zeta"] = {"decimal": cases.ZETA, "hex": float(z).hex(), "rational": str(z),
                    "note": "binary64 nearest 0.05, taken exactly"}
    for sid, sp in cases.SPECTRA.items():
        for n_, (x, y) in enumerate(sp["points"]):
            add(f"spectrum.{sid}.{n_}.x", x)
            add(f"spectrum.{sid}.{n_}.y", y)
    for ax, sid, sc in cases.DIRECTIONS:
        add(f"direction.{ax}.scale", sc)
    for vid, r in cases.REQUESTS.items():
        add(f"request.{vid}.cutoff_Hz", r["cutoff_Hz"])
        add(f"request.{vid}.rigid_frequency_Hz", r["rigid_frequency_Hz"])
    hs = cases.HELD_STATE
    add("held_state.pressure_Pa", hs["pressure_Pa"])
    add("held_state.effective_area_m2", hs["effective_area_m2"])
    for k, v in enumerate(hs["q_ref"]):
        add(f"held_state.q_ref.{k}", v)
    for k, v in enumerate(hs["d0_joint_ends"]):
        add(f"held_state.d0.{k}", v)
    return rows


def publish_inputs(systems):
    S = systems["V-COMB-R"]
    con = S.connector
    return {
        "schema": "f2_spectrum.recovery_w2.inputs/1",
        "status": "candidate reference; no refutation record yet (see README.md)",
        "units": "SI: m, rad, kg, N, N*m, s, Hz, m/s^2",
        "section": cases.SECTION, "material": cases.MATERIAL, "nodes": cases.NODES,
        "pipe_members": cases.PIPE_MEMBERS,
        "connector": dict(cases.CONNECTOR, derived_exact={
            "r_m": [pexact(v) for v in con["r"]],
            "Q_columns_are_axes_row_major": [[pexact(v) for v in r] for r in con["Q"]],
            "K_conn_diagonal": [pexact(con["K_conn"][k][k]) for k in range(6)],
            "B_row_major": [[pexact(v) for v in r] for r in con["B"]],
            "Ke_row_major": [[pexact(v) for v in r] for r in con["Ke"]],
        }),
        "rods": {"offset_m": cases.ROD_OFFSET_M, "stiffness_N_per_m": cases.ROD_STIFFNESS_N_PER_M,
                 "reference_extension_m": "0", "law": "linear, tension and compression",
                 "members": cases.RODS,
                 "derived_exact": {rd["id"]: {"offset_global_m": [pexact(v) for v in rd["offset"]],
                                              "attachment_i_m": [pexact(v) for v in rd["p_i"]],
                                              "attachment_j_m": [pexact(v) for v in rd["p_j"]],
                                              "unit_direction_n": [pexact(v) for v in rd["n"]],
                                              "length_m": pexact(rd["length"]),
                                              "b": [pexact(v) for v in rd["b"]]} for rd in S.rods}},
        "variants": cases.VARIANTS,
        "spectra": cases.SPECTRA, "directions": cases.DIRECTIONS, "zeta": cases.ZETA,
        "requests": cases.REQUESTS, "base_rules": cases.BASE_RULES,
        "mutation_grid": {"modal": MODAL_RULES, "rigid": RIGID_RULES, "directional": DIR_RULES},
        "held_state_for_the_exclusion_control": cases.HELD_STATE,
        "inputs_binary64": inputs_binary64(),
    }


def publish_variant(S, mb, rec, an):
    allidx = list(range(S.n))
    dm = {
        "dof_order": dof_names(S, allidx),
        "free_dofs": dof_names(S, S.free),
        "restrained_dofs": dof_names(S, S.restrained),
        "section_EX_equals_PF": {k: pv(D(S.section[k]) * pi_decimal(STATE["prec"]),
                                       abs(D(S.section[k]) * pi_decimal(STATE["prec"])))
                                 for k in ("A", "I", "J", "metal", "contents", "mbar", "jbar", "ibar")},
        "section_EX_equals_PF_exact_over_pi": {k: pexact(S.section[k])
                                               for k in ("A", "I", "J", "metal", "contents", "mbar", "jbar", "ibar")},
        "section_units": {"A": "m^2", "I": "m^4", "J": "m^4", "metal": "kg/m", "contents": "kg/m", "mbar": "kg/m",
                          "jbar": "kg*m", "ibar": "kg*m"},
        "stiffness_constituents": [],
        "mass_constituents": [],
        "massless": [c["id"] for c in S.k_constituents if c["kind"] in ("connector", "rod")],
        "K_full": pub_matrix(S.K),
        "M_full": pub_matrix(S.M),
    }
    for c in S.k_constituents:
        ent = {"id": c["id"], "kind": c["kind"], "dofs": dof_names(S, c["dofs"])}
        if c["kind"] == "straight":
            ent.update({"axis": [pexact(v) for v in c["axis"]], "length_m": pexact(c["length"]),
                        "K_exact": [[pexact(x) for x in r] for r in c["K_exact"]]})
        if c["kind"] == "arc":
            tf = rec.tangent
            ent.update({
                "radius_m": pexact(c["R"]), "flexibility_factor": pexact(c["k"]),
                "chord_frame_rows_x_y_z": [[pv(v, Decimal(1)) for v in r] for r in tf["chord"]],
                "s_eq_L_over_2R": pv(tf["s"], Decimal(1)), "c_eq_sqrt_1_minus_s2": pv(tf["c"], Decimal(1)),
                "chord_to_tangent_rotation_end_i": [[pv(v, Decimal(1)) for v in r] for r in tf["rot_i"]],
                "chord_to_tangent_rotation_end_j": [[pv(v, Decimal(1)) for v in r] for r in tf["rot_j"]],
                "tangent_frame_rows_end_i": [[pv(v, Decimal(1)) for v in r] for r in tf["i"]],
                "tangent_frame_rows_end_j": [[pv(v, Decimal(1)) for v in r] for r in tf["j"]],
                "centre_m": [pv(v, Decimal(1)) for v in tf["centre"]],
            })
        ent["K"] = pub_matrix(c["K"])
        dm["stiffness_constituents"].append(ent)
    for mc in S.mass_constituents:
        if mc.get("massless"):
            dm["mass_constituents"].append({"id": mc["id"], "kind": mc["kind"], "massless": True})
            continue
        ent = {"id": mc["id"], "kind": mc["kind"], "dofs": dof_names(S, mc["dofs"]), "M": pub_matrix(mc["M"])}
        if mc["kind"] == "arc_half_lump":
            sc = abs(mc["mass"])
            ent.update({"node": mc["node"], "mass_kg": pv(mc["mass"], sc),
                        "first_moment_kg_m": [pv(v, maxabs(mc["first_moment"])) for v in mc["first_moment"]],
                        "centroid_offset_m": [pv(v / mc["mass"], maxabs(mc["first_moment"]) / mc["mass"])
                                              for v in mc["first_moment"]],
                        "centreline_second_moment_kg_m2": pub_matrix(mc["centreline_second_moment"]),
                        "section_rotary_inertia_kg_m2": pub_matrix(mc["section_rotary_inertia"])})
        dm["mass_constituents"].append(ent)
    cats = {k: pv(v, max(abs(x) for x in S.cats.values())) for k, v in S.cats.items()}
    dm["mass_by_category_kg"] = cats
    # modal basis
    modes = mb["modes"]
    clusters = scope_clusters(modes)
    mbo = {"T_k_kg": {ax: pv(mb["T"][k], mb["T"][k]) for k, ax in enumerate(AXES)},
           "bounding_box_diagonal_m": pv(mb["ell"], mb["ell"]),
           "scope_clusters": clusters, "modes": []}
    cum = [Decimal(0)] * 3
    for m in modes:
        fracs = [m["meff"][k] / mb["T"][k] for k in range(3)]
        cum = [a + b for a, b in zip(cum, fracs)]
        shape = {nid: {dn: m["phi"][S.dofs[nid][c]] for c, dn in enumerate(DOFS)} for nid in S.node_ids}
        sc_t = max(abs(m["phi"][d]) for nid in S.node_ids for d in S.dofs[nid][:3])
        sc_r = max(abs(m["phi"][d]) for nid in S.node_ids for d in S.dofs[nid][3:])
        mbo["modes"].append({
            "index": m["index"], "lambda_rad2_per_s2": pv(m["lam"], m["lam"]),
            "omega_rad_per_s": pv(m["omega"], m["omega"]), "frequency_Hz": pv(m["f"], m["f"]),
            "period_s": pv(m["T"], m["T"]),
            "normalization": "unit_max_translation" if m["case"] == "a" else "unit_max_rotation",
            "normalizing_dof": dof_names(S, [m["norm_dof"]])[0],
            "normalization_tie_dofs": dof_names(S, m["ties"]),
            "generalized_mass": pv(m["gm"], m["gm"]),
            "participation_factor": {ax: pv(m["gamma"][k], maxabs(m["gamma"])) for k, ax in enumerate(AXES)},
            "effective_mass_kg": {ax: pv(m["meff"][k], max(mb["T"])) for k, ax in enumerate(AXES)},
            "effective_mass_fraction": {ax: pv(fracs[k], Decimal(1)) for k, ax in enumerate(AXES)},
            "cumulative_fraction": {ax: pv(cum[k], Decimal(1)) for k, ax in enumerate(AXES)},
            "shape": {nid: {dn: pv(v, sc_t if c < 3 else sc_r) for c, (dn, v) in enumerate(d.items())}
                      for nid, d in shape.items()},
        })
    # analysis
    unit = rec.unit.get
    by = an["_by"]
    ao = {
        "request": dict(cases.REQUESTS[S.vid], directions=cases.DIRECTIONS, zeta=cases.ZETA,
                        base_rules=cases.BASE_RULES),
        "extracted_modes": an["extracted"],
        "extracted_clusters": an["clusters"],
        "periodic_modes": an["periodic"],
        "mode_status": {str(i): ("periodic" if i in an["periodic"] else "rigid (extracted)")
                        for i in an["extracted"]},
        "spectral_ordinates_m_per_s2": {
            ax: {str(k): {"S": pv(v[0], v[0]), "scaled": pv(v[1], v[1])} for k, v in d.items()}
            for ax, d in an["ordinates"].items()},
        "cqc_correlation_periodic_clusters": {k: pv(v, Decimal(1)) for k, v in an["rho"].items()},
        "grouping_groups_periodic": an["groups_periodic"],
        "grouping_groups_correction_off": an["groups_correction_off"],
        "quantities": {k: rec.unit[k] for k in rec.keys},
        "quantity_families": QUANTITY_FAMILIES,
        "contributions_signed": {ax: pub_nested({str(i): v for i, v in d.items()}, unit)
                                 for ax, d in an["contrib"].items()},
        "rigid_response_signed": {ax: pub_map(v, unit) for ax, v in an["rigid"].items()},
        "rigid_load": {},
        "periodic_modal_combined": {r: {ax: pub_map(v, unit) for ax, v in d.items()}
                                    for r, d in an["periodic_combined"].items()},
        "periodic_modal_combined_correction_off": {r: {ax: pub_map(v, unit) for ax, v in d.items()}
                                                   for r, d in an["correction_off"].items()},
        "per_direction_combined": {ax: pub_map(v, unit) for ax, v in an["base_per_direction"].items()},
        "combined": pub_map(an["base_combined"], unit),
        "mutations": [dict(rules, combined=pub_map(c, unit)) for rules, c in an["grid"]],
        "missing_mass_disclosure": {ax: {k: pv(v, v if k == "T_k_kg" else Decimal(1)) for k, v in d.items()}
                                    for ax, d in an["missing_mass"].items()},
    }
    for ax, rl in an["rigid_load"].items():
        names = dof_names(S, allidx)
        F = {n_: v for n_, v in zip(names, rl["F"])}
        sh = {}
        for cid, (dofs, v) in rl["shares"].items():
            sh[cid] = dict(zip(dof_names(S, dofs), v))
        allvals = dict(F)
        block = {"F_all_dofs": F}
        block.update({f"share.{cid}": d for cid, d in sh.items()})
        pubd = pub_nested(block, force_unit)
        ao["rigid_load"][ax] = {"acceleration_field_g": pub_map(dict(zip(names, rl["g"])), lambda k: (
            "m/s^2" if dof_unit(k) == "m" else "rad/s^2")),
            "F_all_dofs": pubd["F_all_dofs"],
            "shares_by_constituent": {cid: pubd[f"share.{cid}"] for cid in sh},
            "massless_constituents_share_zero": dm["massless"]}
        del allvals
    eq = an["equilibrium"]
    chk = {k: res(v) for k, v in mb["checks"].items() if k != "sturm_counts"}
    chk["sturm_counts_at_lam1_over_2_gaps_and_2lam_max"] = mb["checks"]["sturm_counts"]
    chk.update({k: res(v) for k, v in eq.items()})
    chk["pipe_K_vs_f1_form_stiffness"] = res(S.check_pipe_K)
    if hasattr(rec, "tangent"):
        chk.update({f"tangent_frame_{k}": res(v) for k, v in rec.tangent["checks"].items()})
    return {
        "schema": "f2_spectrum.recovery_w2.variant/1",
        "variant": S.vid,
        "purpose": S.var["purpose"],
        "status": "candidate reference; no refutation record yet (see README.md)",
        "derived_model": dm,
        "modal_basis": mbo,
        "analysis": ao,
        "cross_checks": chk,
    }


def publish_controls(systems, mbs, recs, ans, conv=None):
    out = {"schema": "f2_spectrum.recovery_w2.controls/1",
           "status": "candidate reference; no refutation record yet (see README.md)"}
    # rods absent / present and arc / joint / combined: frequency ladders side by side
    out["frequency_ladders_Hz"] = {vid: [pv(m["f"], m["f"]) for m in mbs[vid]["modes"]] for vid in systems}
    # fixed-state exclusion
    fs = {}
    for vid in ("V-JOINT-R", "V-COMB-R"):
        r = fixed_state_control(systems[vid], ans[vid])
        unit = recs[vid].unit.get
        fs[vid] = {
            "statement": ("Increments at fixed K, B and geometry: the affine pressure-thrust, reference-load and "
                          "held-displacement terms cancel exactly. This control does not establish an EX static "
                          "equilibrium, a pressure-region mapping or a same-run nonlinear state confirmation."),
            "thrust_P_eq_p_Ae_N": pexact(r["P_N"]),
            "g_state_at_held_state": [pexact(v) for v in r["g_state_at_d0"]],
            "face_action_at_held_state_with_thrust": [pexact(v) for v in r["face_action_at_d0"]],
            "increments_checked": r["increments_checked"],
            "increments_exactly_equal": r["increments_exactly_equal"],
            "mutants": {name: {"added_constant": {k: pv(v, abs(v)) for k, v in m["added_constant"].items()},
                               "reference_combined": pub_map(m["reference_combined"], unit),
                               "mutant_combined": pub_map(m["mutant_combined"], unit),
                               "smallest_relative_distance": res(m["smallest_relative_distance"])}
                        for name, m in r["mutants"].items()},
        }
        assert r["increments_exactly_equal"] == r["increments_checked"]
    out["fixed_state_exclusion"] = fs
    # replaced span
    S = systems["V-COMB-R"]
    rs = replaced_span_mutants(S, mbs["V-COMB-R"], cases.REQUESTS["V-COMB-R"])
    unit = recs["V-COMB-R"].unit.get
    base_sel = {k: v for k, v in ans["V-COMB-R"]["base_combined"].items()
                if k.startswith(("J1.g.", "A.", "F.", "D.u", "E.u"))}
    out["replaced_span_double_ownership"] = {
        "reference": {"T_k_kg": [pv(v, v) for v in mbs["V-COMB-R"]["T"]],
                      "first_frequencies_Hz": [pv(m["f"], m["f"]) for m in mbs["V-COMB-R"]["modes"][:8]],
                      "base_combined_selected": pub_map(base_sel, unit),
                      "member_keys_for_the_span": [k for k in recs["V-COMB-R"].keys if k.startswith("SPAN")]},
        "mutants": {name: {"T_k_kg": [pv(v, v) for v in r["T_k_kg"]],
                           "first_frequencies_Hz": [pv(v, v) for v in r["first_frequencies_Hz"]],
                           "admissible_under_the_base_request": r["admissible_under_the_base_request"],
                           **({"base_combined_selected": pub_map(r["base_combined_selected"], unit)}
                              if r["admissible_under_the_base_request"] else {"reason": r["reason"]})}
                    for name, r in rs.items()},
    }
    # whole-arc closed-form weight check (unit translation): lumps' resultant vs m R phi and centroid
    S = systems["V-ARC"]
    hq = HalfArcQuadrature(S, recs["V-ARC"])
    mtot, cen = hq.closed_form_weight()
    worst = Decimal(0)
    for k in range(3):
        g = rigid_translation(S, k)
        Fs, Ms = [Decimal(0)] * 3, [Decimal(0)] * 3
        for mc in S.mass_constituents:
            if mc["kind"] != "arc_half_lump":
                continue
            w = mv(mc["M"], [g[d] for d in mc["dofs"]])
            x = [D(v) for v in S.xyz[mc["node"]]]
            for c in range(3):
                Fs[c] += w[c]
            mm_ = cross(x, w[:3])
            for c in range(3):
                Ms[c] += mm_[c] + w[3 + c]
        ek = [Decimal(int(c == k)) for c in range(3)]
        Fexp = [mtot * v for v in ek]
        Mexp = cross(cen, Fexp)
        worst = max(worst, maxabs([a - b for a, b in zip(Fs + Ms, Fexp + Mexp)]) / (mtot * maxabs(cen)))
    out["arc_weight_closed_form"] = {
        "arc_mass_kg": pv(mtot, mtot), "arc_centroid_m": [pv(v, maxabs(cen)) for v in cen]}
    # total mass along each axis vs mbar times the mass-bearing length (span D-E excluded), own pi
    Sc = systems["V-COMB-R"]
    pi = own_pi(STATE["prec"])
    mbar = D(Sc.section["mbar"]) * pi
    length = Decimal(0)
    for c in Sc.k_constituents:
        if c["kind"] == "straight":
            length += D(c["length"])
    length += hq.R * 2 * hq.alpha
    tk = max(abs(v - mbar * length) / (mbar * length) for v in mbs["V-COMB-R"]["T"])
    out["mass_bearing_length_V_COMB_m"] = pv(length, length)
    out["cross_checks"] = {"arc_lumps_resultant_vs_closed_form_weight": res(worst),
                           "T_k_vs_mbar_times_mass_bearing_length_span_excluded": res(tk),
                           "binary64_rounding_published_values": ROUNDING["count"],
                           "binary64_rounding_min_relative_margin": res(ROUNDING["min_margin"])}
    if conv is not None:
        out["cross_checks"]["precision_convergence"] = conv
    return out


# ============================================================================ build, check, write
OUTPUT_FILES = ["w2_inputs.json", "w2_v_arc.json", "w2_v_joint_0.json", "w2_v_joint_r.json", "w2_v_comb_0.json",
                "w2_v_comb_r.json", "w2_controls.json"]
VARIANT_FILE = {"V-ARC": "w2_v_arc.json", "V-JOINT-0": "w2_v_joint_0.json", "V-JOINT-R": "w2_v_joint_r.json",
                "V-COMB-0": "w2_v_comb_0.json", "V-COMB-R": "w2_v_comb_r.json"}
FIXED_FILES = ["README.md", "DERIVATION.md", "generators/w2_recovery.py", "generators/w2_cases.py"]
RECORDS = ["REFUTATION.md"]


def build(prec, conv=None):
    ROUNDING.update(count=0, min_margin=None)
    systems = build_systems(prec)
    mbs, recs, ans = {}, {}, {}
    files = {"w2_inputs.json": publish_inputs(systems)}
    for vid, S in systems.items():
        mbs[vid] = modal_basis(S)
        recs[vid] = Recovery(S)
        ans[vid] = analyse(S, mbs[vid], recs[vid], cases.REQUESTS[vid])
        files[VARIANT_FILE[vid]] = publish_variant(S, mbs[vid], recs[vid], ans[vid])
    files["w2_controls.json"] = publish_controls(systems, mbs, recs, ans, conv)
    return files


def strip_checks(o):
    if isinstance(o, dict):
        return {k: strip_checks(v) for k, v in o.items() if k != "cross_checks"}
    if isinstance(o, list):
        return [strip_checks(v) for v in o]
    return o


def count_leaves(o):
    if isinstance(o, dict):
        return sum(count_leaves(v) for v in o.values())
    if isinstance(o, list):
        if len(o) == 2 and isinstance(o[0], str) and isinstance(o[1], float):
            return 1
        return sum(count_leaves(v) for v in o)
    return 1


def dumps(o):
    return json.dumps(o, indent=1, ensure_ascii=False) + "\n"


def generate():
    hi = build(100)
    lo = build(80)
    a, b = strip_checks(lo), strip_checks(hi)
    conv = {"working_precisions_digits": [80, 100],
            "published_values_compared": count_leaves(a),
            "identical_at_40_digits_and_binary64": a == b}
    assert a == b, "published values differ between 80 and 100 digits"
    lo["w2_controls.json"]["cross_checks"]["precision_convergence"] = conv
    return {name: dumps(lo[name]) for name in OUTPUT_FILES}


def sums(texts):
    lines = []
    names = sorted(set(FIXED_FILES) | set(OUTPUT_FILES) | {r for r in RECORDS if (OUT / r).exists()})
    for name in names:
        data = texts[name].encode() if name in texts else (OUT / name).read_bytes()
        lines.append(f"{hashlib.sha256(data).hexdigest()}  {name}")
    return "\n".join(lines) + "\n"


def membership():
    """Every file in the package, except SHA256SUMS and caches, must be listed, and vice versa."""
    present = sorted(str(p.relative_to(OUT)) for p in OUT.rglob("*")
                     if p.is_file() and "__pycache__" not in p.parts and p.name != "SHA256SUMS"
                     and p.name != ".DS_Store")
    return present


def main():
    write = "--write" in sys.argv[1:]
    if "--scratch" in sys.argv[1:]:  # development aid: one 80-digit build to a scratch directory
        dest = Path(sys.argv[sys.argv.index("--scratch") + 1])
        dest.mkdir(parents=True, exist_ok=True)
        for name, o in build(80).items():
            (dest / name).write_text(dumps(o))
        print("scratch build written to", dest)
        return 0
    texts = generate()
    if write:
        for name, t in texts.items():
            (OUT / name).write_text(t)
        (OUT / "SHA256SUMS").write_text(sums(texts))
        print("written:", ", ".join(OUTPUT_FILES), "and SHA256SUMS")
        return 0
    bad = []
    for name, t in texts.items():
        p = OUT / name
        if not p.exists() or p.read_text() != t:
            bad.append(name)
    want = sums(texts)
    got = (OUT / "SHA256SUMS").read_text() if (OUT / "SHA256SUMS").exists() else ""
    if got != want:
        bad.append("SHA256SUMS")
    listed = sorted(l.split("  ", 1)[1] for l in got.splitlines() if l.strip())
    if listed != membership():
        bad.append(f"membership: listed {listed} vs present {membership()}")
    if bad:
        print("MISMATCH:", "; ".join(bad))
        return 1
    print("check passed: every output regenerates byte for byte; SHA256SUMS and file membership agree")
    return 0


if __name__ == "__main__":
    sys.exit(main())
