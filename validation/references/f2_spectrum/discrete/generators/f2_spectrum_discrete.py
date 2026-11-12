#!/usr/bin/env python3
"""F2 response-spectrum references, discrete frame models (DEL-04-10 AC-002, AC-003).

Standard-library Python 3 only. This generator imports, reads and runs no
product code. Besides the files it writes, it reads only the frozen f1_modal
mass_discrete generator modules elements.py, numeric.py, eigen.py and model.py
(reference-side code, imported unmodified; their SHA-256 values are pinned
below and checked before import). All model data are invented
(spectrum_cases.py). Method, premises and cases: ../README.md.

  python3 -I -B generators/f2_spectrum_discrete.py           check (default): regenerate in
                                                             memory, compare byte for byte
  python3 -I -B generators/f2_spectrum_discrete.py --write   write the outputs
"""
import hashlib
import json
import os
import sys
from decimal import Decimal
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
# The pinned f1 directory goes first on the path, so that no same-named file elsewhere (this directory
# included) can shadow a pinned module; each imported module's file is then asserted.
sys.path[:] = [str(F1), str(HERE)] + [p for p in sys.path if p and Path(p).resolve() not in (F1.resolve(), HERE)]

import elements as el  # noqa: E402  (f1_modal)
import eigen as eg  # noqa: E402  (f1_modal)
import model as _f1_model  # noqa: E402  (f1_modal)
import numeric as _f1_numeric  # noqa: E402  (f1_modal)
from model import DOFS, Model, form_mass, form_stiffness, rigid_translation  # noqa: E402  (f1_modal)
from numeric import QPi, QPiField, PointField, fmt, pi_decimal, set_point_precision  # noqa: E402  (f1_modal)
import spectrum_cases  # noqa: E402
for _mod in (el, eg, _f1_model, _f1_numeric):
    assert Path(_mod.__file__).resolve() == (F1 / Path(_mod.__file__).name).resolve(), _mod.__file__
    assert Path(_mod.__file__).name in F1_PINS
assert Path(spectrum_cases.__file__).resolve().parent == HERE

PREC = 80
DIG = 40
FLUSH = Decimal("1e-50")
GRAY = Decimal("1e-35")
TAU_T = Decimal("1e-6")
TIE_REL = Decimal("1e-25")
AXES = ["X", "Y", "Z"]
QF = QPiField()
PF = PointField()
FIXED_FILES = ("README.md", "generators/f2_spectrum_discrete.py", "generators/spectrum_cases.py")
# Files verified by hash but not generated (as f1_modal/mass_discrete does with its REFUTATION.md).
RECORDS = ("REFUTATION.md",)
OUTPUTS = {  # output file -> case ids
    "c1_one_element.json": ["C1-ONE-ELEMENT", "C1C-ONE-ELEMENT-TRANSVERSE"],
    "c2_ground_spring.json": ["C2-GROUND-SPRING"],
    "c3_cantilever_3d.json": ["C3-CANTILEVER-3D"],
    "c4_close_pair.json": ["C4-CLOSE-PAIR"],
    "c5_stiff_and_l_line.json": ["C5A-STIFF", "C5B-L-LINE"],
    "c6_snubbers.json": ["C6A-SNUBBERS-RIGID", "C6B-SNUBBERS-FINITE"],
    "c7_repeated_pair.json": ["C7-REPEATED-PAIR"],
    "c8_pipe_x.json": ["C8-PIPE-X"],
    "c9_pipe_diagonal.json": ["C9-PIPE-DIAGONAL", "C9-CHAIN"],
    "c10_cluster_witnesses.json": ["C10-W2-CLOSE-DISTINCT", "C10-W3-STRADDLING-CLUSTER"],
}


# ============================================================================ numbers
def D(x):
    if isinstance(x, Decimal):
        return +x
    if isinstance(x, QPi):
        return x.to_decimal(PREC)
    x = Fraction(x)
    return Decimal(x.numerator) / Decimal(x.denominator)


def exact(x):
    """Exact rational of an input number (a binary64 value)."""
    if isinstance(x, bool):
        raise TypeError
    if isinstance(x, int):
        return Fraction(x)
    return Fraction(float(x))


def dstr(x):
    x = Decimal(x)
    return "0" if x == 0 else fmt(x, DIG)


class Fmt:
    """Formatter with flushing of working-precision noise relative to a per-unit scale."""

    def __init__(self, scales):
        self.scales = scales

    def __call__(self, x, unit):
        x = Decimal(x)
        s = self.scales.get(unit, Decimal(0))
        if s > 0 and abs(x) < FLUSH * s:
            x = Decimal(0)
        elif s > 0 and abs(x) < GRAY * s:
            raise ArithmeticError(f"value {x} in the flush gray zone for unit {unit}")
        return [dstr(x), float(x)]


def plain(x):
    return [dstr(x), float(Decimal(x))]


def maxabs(v):
    return max((abs(a) for a in v), default=Decimal(0))


def dot(a, b):
    return sum((x * y for x, y in zip(a, b)), Decimal(0))


def mv(A, x):
    return [sum((a * b for a, b in zip(row, x) if a), Decimal(0)) for row in A]


def rel(a, b):
    a, b = Decimal(a), Decimal(b)
    s = max(abs(a), abs(b))
    return abs(a - b) / s if s else Decimal(0)


def show(x):
    return "0" if x == 0 else fmt(x, 3)


# ============================================================================ the system
class System:
    """Full K, M on all DOFs; restraint partition; skew constraint basis; element matrices."""

    def __init__(self, spec):
        self.spec = spec
        m = Model(spec)
        self.model = m
        self.n = m.n
        self.nodes = m.node_ids
        self.xyz = m.xyz
        # exact field a + b*pi when every member length is rational, else 80-digit Decimal
        from model import exact_sqrt
        rational = all(exact_sqrt(sum((m.xyz[m.idx[mm["j"]]][k] - m.xyz[m.idx[mm["i"]]][k]) ** 2 for k in range(3)))
                       is not None for mm in spec["members"])
        F = QF if rational else PF
        self.field = "exact" if rational else "decimal"
        M, contribs, _ = form_mass(m, F, "PF")
        Kstruct = el.zeros(F, self.n, self.n)
        self.elems = []
        for c in contribs:
            assert c["kind"] == "straight"
            mem = m.members[c["id"]]
            mat = spec["materials"][mem["material"]]
            sb = m.section(mem, "PF")
            pi = F.pi()
            E, G = F.q(exact(mat["elastic_modulus_Pa"])), F.q(exact(mat["shear_modulus_Pa"]))
            e, L, Le = m.straight_axis(F, mem)
            Ke = el.straight_stiffness(F, e, L, E * F.q(sb["A"]) * pi, E * F.q(sb["I"]) * pi,
                                       G * F.q(sb["J"]) * pi)
            for a, ra in enumerate(c["dofs"]):
                for b, rb in enumerate(c["dofs"]):
                    Kstruct[ra][rb] = Kstruct[ra][rb] + Ke[a][b]
            if rational:
                ex = [QPi._c(v).a for v in e]
                yr = [exact(v) for v in mem["y_reference"]]
            else:
                ex = list(e)
                yr = [D(exact(v)) for v in mem["y_reference"]]
            p = sum(a * b for a, b in zip(yr, ex))
            y = [yr[k] - p * ex[k] for k in range(3)]
            ny2 = sum(v * v for v in y)
            if rational:
                assert ny2 == 1, "frames are kept exact: y_reference must be a unit vector normal to the member"
            else:
                y = [v / ny2.sqrt() for v in y]
                Le = L
            z = [ex[1] * y[2] - ex[2] * y[1], ex[2] * y[0] - ex[0] * y[2], ex[0] * y[1] - ex[1] * y[0]]
            self.elems.append({"id": c["id"], "i": mem["i"], "j": mem["j"], "dofs": c["dofs"],
                               "Ke": [[D(x) for x in r] for r in Ke],
                               "Me": [[D(x) for x in r] for r in c["matrix"]],
                               "frame": [ex, y, z], "length": Le})
        # restraint partition
        restrained = {}
        for s in spec["supports"]:
            for d in s["restrained"]:
                restrained[6 * m.idx[s["node"]] + DOFS.index(d)] = s["id"]
        self.constraints = []   # rigid skew snubbers: (id, node, d)
        self.compliant = []     # (id, node, kind, d (3 rationals), k)
        Kcomp = el.zeros(F, self.n, self.n)

        def add_rank_one(node, d, k):
            dd = sum(v * v for v in d)
            b = 6 * m.idx[node]
            for r in range(3):
                for c in range(3):
                    if d[r] * d[c]:
                        Kcomp[b + r][b + c] = Kcomp[b + r][b + c] + F.q(k * d[r] * d[c] / dd)

        for sp in spec.get("springs", []):
            d = [Fraction(int(DOFS.index(sp["dof"]) == q)) for q in range(3)]
            assert DOFS.index(sp["dof"]) < 3, "only translational springs in these references"
            add_rank_one(sp["node"], d, exact(sp["stiffness"]))
            self.compliant.append((sp["id"], sp["node"], "spring", d, exact(sp["stiffness"])))
        for sp in spec.get("skew_springs", []):
            d = [exact(v) for v in sp["direction"]]
            add_rank_one(sp["node"], d, exact(sp["stiffness"]))
            self.compliant.append((sp["id"], sp["node"], "skew_spring", d, exact(sp["stiffness"])))
        for sn in spec.get("snubbers", []):
            if isinstance(sn["direction"], str):
                q = DOFS.index(sn["direction"])
                assert q < 3
                d = [Fraction(int(q == r)) for r in range(3)]
            else:
                d = [exact(v) for v in sn["direction"]]
            axis = isinstance(sn["direction"], str)
            if sn["stiffness"] is None:
                if axis:
                    restrained[6 * m.idx[sn["node"]] + q] = sn["id"]
                else:
                    self.constraints.append((sn["id"], sn["node"], d))
            else:
                add_rank_one(sn["node"], d, exact(sn["stiffness"]))
                self.compliant.append((sn["id"], sn["node"], "snubber", d, exact(sn["stiffness"])))
        self.restrained = restrained
        self.free = [k for k in range(self.n) if k not in restrained]
        Ktot = [[Kstruct[i][j] + Kcomp[i][j] for j in range(self.n)] for i in range(self.n)]
        # assembly cross-check against f1's form_stiffness (elements + axis springs on the diagonal)
        Kf1 = form_stiffness(m, F)
        axis_springs = el.zeros(F, self.n, self.n)
        for sp in spec.get("springs", []):
            dd = 6 * m.idx[sp["node"]] + DOFS.index(sp["dof"])
            axis_springs[dd][dd] = F.q(exact(sp["stiffness"]))
        if rational:
            self.assembly_check = all(QPi._c(Kf1[i][j]) == QPi._c(Kstruct[i][j]) + QPi._c(axis_springs[i][j])
                                      for i in range(self.n) for j in range(self.n))
            self.assembly_rel = None
            assert self.assembly_check
        else:
            kmax = max(abs(x) for r in Kf1 for x in r)
            self.assembly_rel = max(abs(Kf1[i][j] - Kstruct[i][j] - axis_springs[i][j])
                                    for i in range(self.n) for j in range(self.n)) / kmax
            self.assembly_check = None
            assert self.assembly_rel < Decimal(10) ** (10 - PREC)
        for cid, node, _, _, _ in self.compliant:
            b = 6 * m.idx[node]
            assert not any(b + r in restrained for r in range(3)), "compliant support on a restrained node"
        # constraint basis T: reduced coordinates -> free DOFs (exact rationals)
        cols = []
        cnodes = {}
        for cid, node, d in self.constraints:
            b = 6 * m.idx[node]
            assert all(b + r not in restrained for r in range(3)) and node not in cnodes
            cnodes[node] = d
        fpos = {g: k for k, g in enumerate(self.free)}
        done = set()
        for g in self.free:
            node = self.nodes[g // 6]
            if node in cnodes and g % 6 < 3:
                if node in done:
                    continue
                done.add(node)
                d = cnodes[node]
                mi = min(range(3), key=lambda r: (abs(d[r]), r))
                ek = [Fraction(int(r == mi)) for r in range(3)]
                w1 = [d[1] * ek[2] - d[2] * ek[1], d[2] * ek[0] - d[0] * ek[2], d[0] * ek[1] - d[1] * ek[0]]
                w2 = [d[1] * w1[2] - d[2] * w1[1], d[2] * w1[0] - d[0] * w1[2], d[0] * w1[1] - d[1] * w1[0]]
                b = 6 * m.idx[node]
                for w in (w1, w2):
                    cols.append({fpos[b + r]: w[r] for r in range(3) if w[r]})
            else:
                cols.append({fpos[g]: Fraction(1)})
        self.T = cols
        self.nr = len(cols)
        nf = len(self.free)

        def reduce_exact(A):
            Af = [[A[i][j] for j in self.free] for i in self.free]
            out = []
            for ca in cols:
                row = []
                for cb in cols:
                    if rational:
                        s = QPi(0)
                        for ia, ta in ca.items():
                            for ib, tb in cb.items():
                                v = QPi._c(Af[ia][ib])
                                if not v.is_zero():
                                    s = s + v * (ta * tb)
                    else:
                        s = Decimal(0)
                        for ia, ta in ca.items():
                            for ib, tb in cb.items():
                                if Af[ia][ib]:
                                    s += Af[ia][ib] * D(ta * tb)
                    row.append(D(s))
                out.append(row)
            return out

        self.Kr = reduce_exact(Ktot)
        self.Mr = reduce_exact(M)
        self.K = [[D(x) for x in r] for r in Ktot]
        self.M = [[D(x) for x in r] for r in M]
        self.nf = nf
        self.R = {ax: [D(x) for x in rigid_translation(m, QF, k)] for k, ax in enumerate(AXES)}
        self.Theta = {}
        for k, ax in enumerate(AXES):
            th = [Decimal(0)] * self.n
            ek = [Decimal(int(k == q)) for q in range(3)]
            for ni, x in enumerate(self.xyz):
                xd = [D(v) for v in x]
                cr = [ek[1] * xd[2] - ek[2] * xd[1], ek[2] * xd[0] - ek[0] * xd[2], ek[0] * xd[1] - ek[1] * xd[0]]
                for q in range(3):
                    th[6 * ni + q] = cr[q]
                    th[6 * ni + 3 + q] = ek[q]
            self.Theta[ax] = th
        self.MR = {ax: mv(self.M, self.R[ax]) for ax in AXES}
        self.Ttot = {ax: dot(self.R[ax], self.MR[ax]) for ax in AXES}
        lo = [min(p[k] for p in self.xyz) for k in range(3)]
        hi = [max(p[k] for p in self.xyz) for k in range(3)]
        self.ell = D(sum((hi[k] - lo[k]) ** 2 for k in range(3))).sqrt()

    def lift(self, q):
        """Reduced coordinates -> full DOF vector (zeros at restrained DOFs)."""
        u = [Decimal(0)] * self.n
        for a, col in enumerate(self.T):
            if q[a] == 0:
                continue
            for i, t in col.items():
                u[self.free[i]] += D(t) * q[a]
        return u

    def reduce_vec(self, v):
        """T^T v_f for a full DOF vector v."""
        return [sum((D(t) * v[self.free[i]] for i, t in col.items()), Decimal(0)) for col in self.T]


# ============================================================================ modal basis
def blocks_of(S):
    """Connected components of the coupling pattern of K_r and M_r (exact zeros decouple)."""
    n = S.nr
    seen = [False] * n
    comps = []
    for s0 in range(n):
        if seen[s0]:
            continue
        stack, comp = [s0], []
        seen[s0] = True
        while stack:
            i = stack.pop()
            comp.append(i)
            for j in range(n):
                if not seen[j] and (S.Kr[i][j] != 0 or S.Mr[i][j] != 0):
                    seen[j] = True
                    stack.append(j)
        comps.append(sorted(comp))
    return comps


def modal_basis(S):
    comps = blocks_of(S)
    if len(comps) == 1:
        pairs, _ = eg.route_b(S.Kr, S.Mr)
    else:
        pairs = []
        for comp in comps:
            sub, _ = eg.route_b([[S.Kr[i][j] for j in comp] for i in comp], [[S.Mr[i][j] for j in comp] for i in comp])
            for lam, v in sub:
                q = [Decimal(0)] * S.nr
                for a, i in enumerate(comp):
                    q[i] = v[a]
                pairs.append((lam, q))
    S.blocks = comps
    modes = []
    for lam, q in pairs:
        mq = dot(q, mv(S.Mr, q))
        q = [x / mq.sqrt() for x in q]
        lam = dot(q, mv(S.Kr, q))
        modes.append([lam, q])
    modes.sort(key=lambda t: t[0])
    lams = [t[0] for t in modes]
    clusters = eg.clusters_of(lams)
    for s, d in clusters:
        if d > 1:
            Q = eg.m_orthonormalize(S.Mr, [modes[s + k][1] for k in range(d)])
            Q = eg.canonical_basis(S.Mr, Q)
            for k in range(d):
                modes[s + k][1] = Q[k]
            lam = sum((dot(v, mv(S.Kr, v)) for v in Q), Decimal(0)) / d
            for k in range(d):
                modes[s + k][0] = lam
    pi2 = 2 * pi_decimal(PREC)
    out = []
    for ci, (s, d) in enumerate(clusters):
        for k in range(d):
            lam, q = modes[s + k]
            omega = lam.sqrt()
            phi = S.lift(q)
            out.append({"index": s + k + 1, "cluster": ci, "multiplicity": d, "lam": lam, "omega": omega,
                        "f": omega / pi2, "q": q, "phi": phi})
    # normalization of simple modes (DEL-04-09 REQ-005); M-normalized vectors kept for clusters
    for md in out:
        phi = md["phi"]
        tr = [(abs(phi[g]), g) for g in S.free if g % 6 < 3]
        rt = [(abs(phi[g]), g) for g in S.free if g % 6 >= 3]
        tmax = max((a for a, _ in tr), default=Decimal(0))
        rmax = max((a for a, _ in rt), default=Decimal(0))
        case_b = tmax <= TAU_T * S.ell * rmax
        pool = rt if case_b else tr
        big = max(a for a, _ in pool)
        ties = sorted(g for a, g in pool if a >= big * (1 - TIE_REL))
        g0 = ties[0]
        if md["multiplicity"] == 1:
            sc = phi[g0]
            md["phi"] = [x / sc for x in phi]
            md["q"] = [x / sc for x in md["q"]]
            md["case"] = "unit_max_rotation" if case_b else "unit_max_translation"
            md["norm_dof"] = g0
            md["ties"] = ties if len(ties) > 1 else []
        md["m"] = dot(md["phi"], mv(S.M, md["phi"]))
        md["gamma"] = {ax: dot(md["phi"], S.MR[ax]) / md["m"] for ax in AXES}
        md["meff"] = {ax: dot(md["phi"], S.MR[ax]) ** 2 / md["m"] for ax in AXES}
    return out, clusters


def scope_clusters(modes):
    """DEL-04-09 REQ-002 clusters: adjacent eigenvalues chained by lambda_{i+1} - lambda_i <= delta_c lambda_{i+1}."""
    out = []
    k = 0
    while k < len(modes):
        d = 1
        while k + d < len(modes) and modes[k + d]["lam"] - modes[k + d - 1]["lam"] <= DELTA_C * modes[k + d]["lam"]:
            d += 1
        out.append((k, d))
        for j in range(k, k + d):
            modes[j]["scl"] = len(out) - 1
        k += d
    return out


def modal_checks(S, modes, clusters):
    res = Decimal(0)
    knorm = max(sum(abs(x) for x in r) for r in S.Kr)
    for md in modes:
        q = md["q"]
        r = [a - md["lam"] * b for a, b in zip(mv(S.Kr, q), mv(S.Mr, q))]
        res = max(res, maxabs(r) / (knorm * maxabs(q)))
    orth = Decimal(0)
    for i, a in enumerate(modes):
        Ma = mv(S.Mr, a["q"])
        for b in modes[i + 1:]:
            orth = max(orth, abs(dot(b["q"], Ma)) / (a["m"] * b["m"]).sqrt())
    lams = [modes[s]["lam"] for s, _ in clusters]
    counts = []
    shifts = [lams[0] / 2] + [(lams[k] * lams[k + 1]).sqrt() for k in range(len(lams) - 1)] + [2 * lams[-1]]
    expect = [0]
    acc = 0
    for _, d in clusters:
        acc += d
        expect.append(acc)
    ok = True
    for sh, ex in zip(shifts, expect):
        c, _ = eg.sturm_count(PF, S.Kr, S.Mr, sh)
        ok = ok and c == ex
        counts.append(c)
    # sum of effective masses over all modes vs p^T M^-1 p (reduced coordinates)
    fac = eg.lu_factor(S.Mr)
    meff_all = {}
    for ax in AXES:
        pr = S.reduce_vec(S.MR[ax])
        direct = dot(pr, eg.lu_solve(fac, pr))
        sm = sum((md["meff"][ax] for md in modes), Decimal(0))
        meff_all[ax] = (sm, direct)
    # all modes periodic -> no residual load on the free DOFs
    resid = Decimal(0)
    for ax in AXES:
        pr = S.reduce_vec(S.MR[ax])
        acc = list(pr)
        for md in modes:
            Mq = mv(S.Mr, md["q"])
            acc = [a - md["gamma"][ax] * b for a, b in zip(acc, Mq)]
        scale = maxabs(pr)
        if scale:
            resid = max(resid, maxabs(acc) / scale)
    gaps = [(lams[k + 1].sqrt() - lams[k].sqrt()) / lams[k].sqrt() for k in range(len(lams) - 1)]
    return {
        "eigen_residual": res, "m_orthogonality": orth, "sturm_ok": ok, "sturm_counts": counts,
        "sturm_expected": expect, "meff_all": meff_all, "all_periodic_free_load": resid,
        "min_relative_frequency_gap": min(gaps) if gaps else None,
    }


# ============================================================================ spectra
def ordinate(spec, f):
    pts = [(D(exact(a)), D(exact(b))) for a, b in spec["points"]]
    x = f if spec["abscissa"] == "frequency_Hz" else 1 / f
    if x < pts[0][0] or x > pts[-1][0]:
        return None
    for (x1, s1), (x2, s2) in zip(pts, pts[1:]):
        if x1 <= x <= x2:
            break
    if x == x1:
        return s1
    if x == x2:
        return s2
    if spec["interpolation"]["abscissa"] == "linear":
        t = (x - x1) / (x2 - x1)
    else:
        t = (x.ln() - x1.ln()) / (x2.ln() - x1.ln())
    if spec["interpolation"]["ordinate"] == "linear":
        return s1 + t * (s2 - s1)
    return (s1.ln() + t * (s2.ln() - s1.ln())).exp()


def rho(zeta, wi, wj):
    r = wj / wi
    return 8 * zeta ** 2 * (1 + r) * r * r.sqrt() / ((1 - r * r) ** 2 + 4 * zeta ** 2 * r * (1 + r) ** 2)


# ============================================================================ recovery
class Recovery:
    def __init__(self, S):
        self.S = S
        self.qs = []   # (key, unit)
        for ni, nid in enumerate(S.nodes):
            for k, d in enumerate(DOFS):
                self.qs.append((f"{nid}.{d}", "m" if k < 3 else "rad"))
        comp = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"]
        for e in S.elems:
            for end in ("i", "j"):
                for k, c in enumerate(comp):
                    self.qs.append((f"{e['id']}.{end}.{c}", "N" if k < 3 else "N*m"))
        self.member_axes = getattr(S, "member_axes", False)
        if self.member_axes:
            for e in S.elems:
                for end in ("i", "j"):
                    for k, c in enumerate(comp):
                        self.qs.append((f"{e['id']}.{end}.{c}_m", "N" if k < 3 else "N*m"))
        self.supports = []  # (key, unit, kind, data)
        for s in S.spec["supports"]:
            for d in s["restrained"]:
                g = 6 * S.model.idx[s["node"]] + DOFS.index(d)
                k = DOFS.index(d)
                self.supports.append((f"{s['id']}.{comp[k]}", "N" if k < 3 else "N*m", "rigid", g))
        for sn in S.spec.get("snubbers", []):
            if sn["stiffness"] is None and isinstance(sn["direction"], str):
                k = DOFS.index(sn["direction"])
                g = 6 * S.model.idx[sn["node"]] + k
                self.supports.append((f"{sn['id']}.{comp[k]}", "N", "rigid", g))
        for cid, node, d in S.constraints:
            b = 6 * S.model.idx[node]
            for k in range(3):
                self.supports.append((f"{cid}.{comp[k]}", "N", "constraint", (b, k, d)))
            self.supports.append((f"{cid}.Fd", "N", "constraint_d", (b, d)))
        # motion at a rigid skew snubber's node: along d (zero) and along two perpendicular unit vectors
        self.motion = []
        self.motion_axes = {}
        for cid, node, d in S.constraints:
            b = 6 * S.model.idx[node]
            mi = min(range(3), key=lambda r: (abs(d[r]), r))
            ek = [Fraction(int(r == mi)) for r in range(3)]
            w1 = [d[1] * ek[2] - d[2] * ek[1], d[2] * ek[0] - d[0] * ek[2], d[0] * ek[1] - d[1] * ek[0]]
            w2 = [d[1] * w1[2] - d[2] * w1[1], d[2] * w1[0] - d[0] * w1[2], d[0] * w1[1] - d[1] * w1[0]]
            axes = {}
            for name, w in (("u_along_d", d), ("u_perp1", w1), ("u_perp2", w2)):
                nw = D(sum(x * x for x in w)).sqrt()
                axes[name] = [D(x) / nw for x in w]
                self.motion.append((f"{cid}.{name}", b, axes[name]))
            self.motion_axes[cid] = {k: [dstr(x) for x in v] for k, v in axes.items()}
        for cid, node, kind, d, kk in S.compliant:
            b = 6 * S.model.idx[node]
            for k in range(3):
                if d[k]:
                    self.supports.append((f"{cid}.{comp[k]}", "N", "compliant", (b, k, d, kk)))
            if sum(1 for v in d if v) > 1:
                self.supports.append((f"{cid}.Fd", "N", "compliant_d", (b, d, kk)))
        self.qs += [(k, u) for k, u, _, _ in self.supports]
        self.qs += [(k, "m") for k, _, _ in self.motion]
        self.unit = dict(self.qs)
        self.keys = [k for k, _ in self.qs]

    def recover(self, u, g):
        """Quantities from displacement u (all DOFs) and inertia acceleration field g (F = M g)."""
        S = self.S
        out = {}
        for ni, nid in enumerate(S.nodes):
            for k, d in enumerate(DOFS):
                out[f"{nid}.{d}"] = u[6 * ni + k]
        comp = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"]
        assembled = [Decimal(0)] * S.n
        for e in S.elems:
            ue = [u[d] for d in e["dofs"]]
            ge = [g[d] for d in e["dofs"]]
            fe = [a - b for a, b in zip(mv(e["Ke"], ue), mv(e["Me"], ge))]
            for a, d in enumerate(e["dofs"]):
                assembled[d] += fe[a]
            for a in range(12):
                out[f"{e['id']}.{'ij'[a // 6]}.{comp[a % 6]}"] = fe[a]
            if self.member_axes:
                lam = [[D(x) for x in row] for row in e["frame"]]
                for blk in range(4):
                    v = fe[3 * blk:3 * blk + 3]
                    for r in range(3):
                        out[f"{e['id']}.{'ij'[blk // 2]}.{comp[3 * (blk % 2) + r]}_m"] = sum(
                            (lam[r][q] * v[q] for q in range(3)), Decimal(0))
        Mg = mv(S.M, g)
        resid = [a - b for a, b in zip(mv(S.K, u), Mg)]
        support_force = [Decimal(0)] * S.n   # force on the piping, per DOF
        for key, unit, kind, data in self.supports:
            if kind == "rigid":
                v = resid[data]
                support_force[data] += v
            elif kind == "constraint":
                b, k, d = data
                v = resid[b + k]
                support_force[b + k] += v
            elif kind == "constraint_d":
                b, d = data
                nd = D(sum(x * x for x in d)).sqrt()
                v = sum((D(d[k]) * resid[b + k] for k in range(3)), Decimal(0)) / nd
            elif kind == "compliant":
                b, k, d, kk = data
                dd = sum(x * x for x in d)
                du = sum((D(d[r]) * u[b + r] for r in range(3)), Decimal(0))
                v = -D(kk) * D(d[k]) * du / D(dd)
                support_force[b + k] += v
            else:
                b, d, kk = data
                nd = D(sum(x * x for x in d)).sqrt()
                du = sum((D(d[r]) * u[b + r] for r in range(3)), Decimal(0))
                v = -D(kk) * du / nd
            out[key] = v
        for key, b, w in self.motion:
            out[key] = sum((w[k] * u[b + k] for k in range(3)), Decimal(0))
        # checks: nodal balance and global equilibrium (forces and moments)
        bal = maxabs([a - b for a, b in zip(assembled, support_force)])
        bal_scale = maxabs(assembled)
        eq = Decimal(0)
        for ax in AXES:
            for vec in (S.R[ax], S.Theta[ax]):
                lhs = dot(vec, support_force)
                rhs = -dot(vec, Mg)
                eq = max(eq, abs(lhs - rhs) / max(abs(rhs), maxabs(Mg) * S.ell, Decimal("1e-300")))
        skew = Decimal(0)
        for cid, node, d in S.constraints:
            b = 6 * S.model.idx[node]
            nd = D(sum(x * x for x in d)).sqrt()
            v = abs(sum((D(d[k]) * u[b + k] for k in range(3)), Decimal(0))) / nd
            skew = max(skew, v / max(maxabs(u[b:b + 3]), Decimal("1e-300")))
        return out, {"nodal_balance": bal / bal_scale if bal_scale else Decimal(0), "equilibrium": eq,
                     "skew_constraint": skew}


# ============================================================================ combinations
def combine(rule, units, key, zeta):
    """units: [(omega, f, {key: R})] in ascending frequency."""
    R = [u[2][key] for u in units]
    if rule == "SRSS":
        return sum((r * r for r in R), Decimal(0)).sqrt()
    if rule == "ABSOLUTE_SUM":
        return sum((abs(r) for r in R), Decimal(0))
    if rule == "CQC":
        s = Decimal(0)
        for i, ui in enumerate(units):
            for j, uj in enumerate(units):
                s += (Decimal(1) if i == j else rho(zeta, ui[0], uj[0])) * R[i] * R[j]
        scale = sum((r * r for r in R), Decimal(0))
        assert s >= -Decimal("1e-60") * scale
        return max(s, Decimal(0)).sqrt()
    if rule == "GROUPING":
        s = sum((r * r for r in R), Decimal(0))
        for grp in groups([u[1] for u in units]):
            for a in range(len(grp)):
                for b in range(a + 1, len(grp)):
                    s += 2 * abs(R[grp[a]] * R[grp[b]])
        return s.sqrt()
    if rule == "TEN_PERCENT":
        # every pair i < j with (f_j - f_i)/f_i <= 0.1, whether or not the two fall in one group
        s = sum((r * r for r in R), Decimal(0))
        for i in range(len(units)):
            for j in range(i + 1, len(units)):
                x = (units[j][1] - units[i][1]) / units[i][1]
                assert abs(x - Decimal("0.1")) > Decimal("1e-6")
                if x <= Decimal("0.1"):
                    s += 2 * abs(R[i] * R[j])
        return s.sqrt()
    raise ValueError(rule)


def groups(fs):
    out = []
    k = 0
    while k < len(fs):
        g = [k]
        while k + len(g) < len(fs):
            x = (fs[k + len(g)] - fs[k]) / fs[k]
            assert abs(x - Decimal("0.1")) > Decimal("1e-6"), "a frequency lies on the grouping boundary"
            if x <= Decimal("0.1"):
                g.append(k + len(g))
            else:
                break
        out.append(g)
        k += len(g)
    return out


def directional(rule, vals):
    vals = list(vals)
    if rule == "SRSS":
        return sum((v * v for v in vals), Decimal(0)).sqrt()
    if rule == "ABSOLUTE_SUM":
        return sum(vals, Decimal(0))
    if rule == "100-40-40":
        return max(vals[a] + Decimal(2) / 5 * sum((vals[b] for b in range(len(vals)) if b != a), Decimal(0))
                   for a in range(len(vals)))
    raise ValueError(rule)


def total(rule, p, r):
    if rule is None:
        return p
    if rule == "SRSS":
        return (p * p + r * r).sqrt()
    return p + r


# ============================================================================ one request
def analyse(S, rec, modes, clusters, spectra, req, mutations, grid=None):
    zeta = D(exact(spectra[req["directions"][0]["spectrum"]]["damping_ratio"]))
    for dd in req["directions"]:
        assert spectra[dd["spectrum"]]["damping_ratio"] == spectra[req["directions"][0]["spectrum"]]["damping_ratio"]
    n_ext = req["extracted_modes"]
    ext = modes[:n_ext]
    sclusters = scope_clusters(modes)
    assert n_ext == len(modes) or modes[n_ext]["scl"] != modes[n_ext - 1]["scl"], "partial cluster"
    for k in range(1, min(n_ext + 1, len(modes))):
        gap = (modes[k]["lam"] - modes[k - 1]["lam"]) / modes[k]["lam"]
        assert not (DELTA_C / 2 < gap < 2 * DELTA_C), "an eigenvalue gap too near the cluster tolerance"
    lowest_f = {md["scl"]: modes[[s for s, _ in sclusters][md["scl"]]]["f"] for md in modes}
    mm = req["missing_mass"]
    fr = D(exact(mm["rigid_frequency_Hz"])) if mm["applied"] else None
    if mm["applied"]:
        # REQ-001: the lowest frequency of the highest completed cluster is at or above f_r, or the whole spectrum
        top = [s0 for s0, _ in sclusters if s0 < n_ext][-1]
        assert modes[top]["f"] >= fr or n_ext == len(modes), "extraction premise"
        for f0 in lowest_f.values():
            assert rel(f0, fr) > Decimal("1e-9"), "rigid frequency on a cluster's lowest frequency"
    # a cluster is periodic as a whole when its lowest frequency is below f_r
    periodic = [md for md in ext if fr is None or lowest_f[md["scl"]] < fr]
    pidx = {md["index"] for md in periodic}
    keys = rec.keys
    dirs = [dd["direction"] for dd in req["directions"]]
    checks = {"nodal_balance": Decimal(0), "equilibrium": Decimal(0), "skew_constraint": Decimal(0)}

    def upd(c):
        for k in checks:
            checks[k] = max(checks[k], c[k])

    ords = {}
    contrib = {}
    for dd in req["directions"]:
        ax, spec, s = dd["direction"], spectra[dd["spectrum"]], D(exact(dd["scale_factor"]))
        ords[ax] = []
        contrib[ax] = {}
        for md in ext:
            Sv = ordinate(spec, md["f"])
            status = "periodic" if md["index"] in pidx else "rigid"
            if status == "periodic":
                assert Sv is not None, "periodic mode outside the tabulated range"
            ords[ax].append((md, Sv, None if Sv is None else s * Sv, status))
            if Sv is None:
                continue
            amp = md["gamma"][ax] * s * Sv / md["lam"]
            u = [amp * x for x in md["phi"]]
            g = [md["lam"] * x for x in u]
            vals, c = rec.recover(u, g)
            upd(c)
            contrib[ax][md["index"]] = vals
    # units: scope clusters (delta_c) summed algebraically, each mode with its own ordinate and omega^2;
    # the cluster's lowest frequency is its frequency for rho, the closeness tests and periodic/rigid
    def units_for(ax, mlist):
        out = []
        for s, d in sclusters:
            members = [md for md in mlist if s < md["index"] <= s + d]
            if not members:
                continue
            assert len(members) == d
            vals = {k: sum((contrib[ax][md["index"]][k] for md in members), Decimal(0)) for k in keys}
            out.append((members[0]["omega"], members[0]["f"], vals, members))
        return out

    # rigid response
    rigid = {}
    rigid_checks = {}
    if mm["applied"]:
        for dd in req["directions"]:
            ax, spec, s = dd["direction"], spectra[dd["spectrum"]], D(exact(dd["scale_factor"]))
            Sr = ordinate(spec, fr)
            assert Sr is not None, "rigid frequency outside the tabulated range"
            ar = s * Sr
            g = [ar * x for x in S.R[ax]]
            for md in periodic:
                g = [a - ar * md["gamma"][ax] * b for a, b in zip(g, md["phi"])]
            F = mv(S.M, g)
            ur = eg.lu_solve(eg.lu_factor(S.Kr), S.reduce_vec(F))
            u = S.lift(ur)
            vals, c = rec.recover(u, g)
            upd(c)
            # second route: spectral expansion over every finite mode that is not periodic
            alt = [Decimal(0)] * S.n
            for md in modes:
                if md["index"] in pidx:
                    continue
                a = ar * md["gamma"][ax] / md["lam"]
                alt = [x + a * y for x, y in zip(alt, md["phi"])]
            # scale: the static response to the full rigid inertia a_r M R_k (no periodic modes removed)
            u_full = S.lift(eg.lu_solve(eg.lu_factor(S.Kr), S.reduce_vec([ar * x for x in S.MR[ax]])))
            d_alt = maxabs([a - b for a, b in zip(u, alt)]) / maxabs(u_full)
            tot = sum((vals[k] for k, _, kind, data in rec.supports
                       if (kind in ("rigid",) and data % 6 == AXES.index(ax))
                       or (kind in ("constraint", "compliant") and data[1] == AXES.index(ax))), Decimal(0))
            expect = -ar * (S.Ttot[ax] - sum((md["meff"][ax] for md in periodic), Decimal(0)))
            rigid[ax] = {"a_r": ar, "S_r": Sr, "vals": vals, "F": F, "u": u}
            rigid_checks[ax] = {"static_vs_modal_expansion": d_alt, "total_reaction": tot, "expected": expect,
                                "total_reaction_rel": rel(tot, expect) if expect else abs(tot)}
    # combinations
    rules_modal = ["SRSS", "CQC", "GROUPING"]
    extra_modal = ["ABSOLUTE_SUM", "TEN_PERCENT"] if mutations or grid else []
    per_rule = {}
    for rule in rules_modal + extra_modal:
        per_rule[rule] = {ax: {k: combine(rule, units_for(ax, periodic), k, zeta) for k in keys} for ax in dirs}
    off = None
    if mm["applied"] and all(o[1] is not None for ax in dirs for o in ords[ax]):
        off = {rule: {ax: {k: combine(rule, units_for(ax, ext), k, zeta) for k in keys} for ax in dirs}
               for rule in rules_modal + extra_modal}
    rig_mag = {ax: {k: abs(rigid[ax]["vals"][k]) for k in keys} for ax in rigid}

    def result(modal, rrule, drule, use_off=False):
        src = (off if use_off else per_rule)[modal]
        perdir = {ax: {k: total(None if use_off or not mm["applied"] else rrule, src[ax][k],
                                rig_mag[ax][k] if ax in rig_mag else 0) for k in keys} for ax in dirs}
        comb = {k: directional(drule, [perdir[ax][k] for ax in dirs]) for k in keys}
        return perdir, comb

    base_rr = mm["rigid_periodic_combination"] if mm["applied"] else None
    base_perdir, base_comb = result(req["modal_combination"], base_rr, req["directional_combination"],
                                    use_off=False)
    muts = []
    if mutations:
        for modal in rules_modal:
            for rr in (["SRSS", "ABSOLUTE_SUM", "OFF"] if mm["applied"] else ["OFF"]):
                for dr in ["SRSS", "100-40-40"]:
                    if rr == "OFF" and mm["applied"]:
                        if off is None:
                            continue
                        _, comb = result(modal, None, dr, use_off=True)
                    else:
                        _, comb = result(modal, None if rr == "OFF" else rr, dr)
                    muts.append(({"modal_combination": modal, "missing_mass": rr, "directional_combination": dr},
                                 comb))
        bm, bd = req["modal_combination"], req["directional_combination"]
        for modal in extra_modal:
            _, comb = result(modal, base_rr, bd)
            muts.append(({"modal_combination": modal, "missing_mass": base_rr or "OFF",
                          "directional_combination": bd}, comb))
        _, comb = result(bm, base_rr, "ABSOLUTE_SUM")
        muts.append(({"modal_combination": bm, "missing_mass": base_rr or "OFF",
                      "directional_combination": "ABSOLUTE_SUM"}, comb))
    if grid:
        for modal in grid["modal"]:
            for rr in grid["rigid"]:
                for dr in grid["directional"]:
                    if rr == "OFF" and mm["applied"]:
                        if off is None:
                            continue
                        _, comb = result(modal, None, dr, use_off=True)
                    else:
                        _, comb = result(modal, None if rr == "OFF" else rr, dr)
                    muts.append(({"modal_combination": modal, "missing_mass": rr, "directional_combination": dr},
                                 comb))
    # reading B for repeated clusters: SRSS range as the basis of each cluster rotates
    reading_b = None
    if any(d > 1 for s, d in clusters if any(s < md["index"] <= s + d for md in periodic)):
        reading_b = cluster_ranges(S, rec, periodic, clusters, ords, req, spectra, contrib, keys)
    # disclosures
    disc = {}
    for ax in dirs:
        Tk = S.Ttot[ax]
        sp = sum((md["meff"][ax] for md in periodic), Decimal(0))
        se = sum((md["meff"][ax] for md in ext), Decimal(0))
        sa = sum((md["meff"][ax] for md in modes), Decimal(0))
        disc[ax] = {"T": Tk, "periodic": sp, "extracted": se, "all": sa}
    return {"sclusters": sclusters, "zeta": zeta, "fr": fr, "ext": ext, "periodic": periodic, "ords": ords, "contrib": contrib,
            "rigid": rigid, "rigid_checks": rigid_checks, "pidx": pidx, "per_rule": per_rule, "off": off, "rig_mag": rig_mag,
            "base_perdir": base_perdir, "base_comb": base_comb, "muts": muts, "checks": checks,
            "reading_b": reading_b, "disc": disc, "units_for": units_for}


def cluster_ranges(S, rec, periodic, clusters, ords, req, spectra, contrib, keys):
    """Plain SRSS over individual vectors of a repeated pair, as the pair's basis rotates.

    For a pair with M-orthonormal basis (a, b), contributions are R(v) = C (v^T p)(L v) for unit v,
    C = s S / omega^2 and L the linear recovery at omega^2. With alpha = (a^T p, b^T p),
    beta = (L a, L b) and A = (alpha beta^T + beta alpha^T)/2, a rotated basis (u, w) gives
    R_u^2 + R_w^2 = C^2 ((u^T A u)^2 + (w^T A w)^2), whose range over rotations is
    [C^2 (tr A)^2 / 2, C^2 |A|_F^2]: the minimum where u^T A u = w^T A w (45 degrees from A's
    eigenvectors), the maximum on A's eigenvectors.
    """
    out = {}
    errs = []
    for dd in req["directions"]:
        ax = dd["direction"]
        simple_sq = {k: Decimal(0) for k in keys}
        lo = {k: Decimal(0) for k in keys}
        hi = {k: Decimal(0) for k in keys}
        for s, d in clusters:
            members = [md for md in periodic if s < md["index"] <= s + d]
            if not members:
                continue
            if d == 1:
                for k in keys:
                    simple_sq[k] += contrib[ax][members[0]["index"]][k] ** 2
                continue
            assert d == 2, "ranges are implemented for pairs"
            a, b = members
            Sv = [o for o in ords[ax] if o[0]["index"] == a["index"]][0][2]
            C = Sv / a["lam"]
            # contributions of the M-normalized basis vectors: R_a = C (a^T p)^2/m ... use unit-mass vectors
            va = [x / a["m"].sqrt() for x in a["phi"]]
            vb = [x / b["m"].sqrt() for x in b["phi"]]
            pa, pb = dot(va, S.MR[ax]), dot(vb, S.MR[ax])
            La, _ = rec.recover(va, [a["lam"] * x for x in va])
            Lb, _ = rec.recover(vb, [a["lam"] * x for x in vb])
            for k in keys:
                al = (pa, pb)
                be = (La[k], Lb[k])
                A11, A22 = al[0] * be[0], al[1] * be[1]
                A12 = (al[0] * be[1] + al[1] * be[0]) / 2
                tr = A11 + A22
                fro = A11 * A11 + A22 * A22 + 2 * A12 * A12
                lo[k] += C * C * tr * tr / 2
                hi[k] += C * C * fro
                # second route: explicit rotated bases through the full recovery
                if fro > 0:
                    half = (A11 - A22) / 2
                    rad = (half * half + A12 * A12).sqrt()
                    if A12 != 0:
                        e1 = (A12, rad - half)
                    else:
                        e1 = (Decimal(1), Decimal(0)) if A11 >= A22 else (Decimal(0), Decimal(1))
                    n1 = (e1[0] ** 2 + e1[1] ** 2).sqrt()
                    e1 = (e1[0] / n1, e1[1] / n1)
                    e2 = (-e1[1], e1[0])
                    h = Decimal(2).sqrt() / 2
                    for basis, target in (((e1, e2), C * C * fro),
                                          ((((e1[0] + e2[0]) * h, (e1[1] + e2[1]) * h),
                                            ((e2[0] - e1[0]) * h, (e2[1] - e1[1]) * h)), C * C * tr * tr / 2)):
                        ssum = Decimal(0)
                        for c in basis:
                            v = [c[0] * x + c[1] * y for x, y in zip(va, vb)]
                            R, _ = rec.recover(v, [a["lam"] * x for x in v])
                            ssum += (C * dot(v, S.MR[ax]) * R[k]) ** 2
                        errs.append((rec.unit[k], abs(ssum - target), C * C * fro))
        out[ax] = {"min": {k: (simple_sq[k] + lo[k]).sqrt() for k in keys},
                   "max": {k: (simple_sq[k] + hi[k]).sqrt() for k in keys}}
    umax = {}
    for u, _, t in errs:
        umax[u] = max(umax.get(u, Decimal(0)), t)
    worst = max((e / umax[u] for u, e, _ in errs if umax[u] > 0), default=Decimal(0))
    return {"ranges": out, "rotated_basis_check": worst}


# ============================================================================ publication
def inputs_table(*objs):
    seen = {}

    def walk(o):
        if isinstance(o, bool) or o is None:
            return
        if isinstance(o, (int, float)):
            fr = exact(o)
            f = float(fr)
            seen[repr(f)] = {"shortest_decimal": repr(f), "hex": f.hex(),
                             "exact_rational": str(fr)}
        elif isinstance(o, dict):
            for v in o.values():
                walk(v)
        elif isinstance(o, (list, tuple)):
            for v in o:
                walk(v)

    for o in objs:
        walk(o)
    return [seen[k] for k in sorted(seen, key=lambda s: (float(s), s))]


def publish_modes(S, modes, clusters, n_shapes):
    out = []
    cum = {ax: Decimal(0) for ax in AXES}
    for s, d in clusters:
        mds = modes[s:s + d]
        a = mds[0]
        ent = {"modes": [md["index"] for md in mds], "multiplicity": d,
               "eigenvalue_rad2_per_s2": plain(a["lam"]), "circular_frequency_rad_per_s": plain(a["omega"]),
               "frequency_Hz": plain(a["f"]), "period_s": plain(1 / a["f"])}
        if d == 1:
            ent["normalization"] = a["case"]
            ent["normalizing_dof"] = [S.nodes[a["norm_dof"] // 6], DOFS[a["norm_dof"] % 6]]
            if a["ties"]:
                ent["normalization_tie_dofs"] = [[S.nodes[g // 6], DOFS[g % 6]] for g in a["ties"]]
            unit = "kg" if a["case"] == "unit_max_translation" else "kg*m^2*rad^-2"
            ent["generalized_mass"] = {"unit": unit, "value": plain(a["m"])}
            ent["participation_factor"] = {ax: plain(a["gamma"][ax]) for ax in AXES}
            if a["index"] <= n_shapes:
                sh = {}
                for ni, nid in enumerate(S.nodes):
                    sh[nid] = [plain(a["phi"][6 * ni + k]) if abs(a["phi"][6 * ni + k]) > FLUSH else ["0", 0.0]
                               for k in range(6)]
                ent["shape"] = sh
        else:
            ent["basis"] = ("exactly repeated: any M-orthogonal basis of the cluster is a valid modal basis; "
                            "only basis-invariant quantities are published")
        me = {ax: sum((md["meff"][ax] for md in mds), Decimal(0)) for ax in AXES}
        for ax in AXES:
            cum[ax] += me[ax]
        ent["effective_mass_kg"] = {ax: plain(me[ax]) for ax in AXES}
        ent["effective_mass_fraction"] = {ax: plain(me[ax] / S.Ttot[ax]) for ax in AXES}
        ent["cumulative_effective_mass_fraction"] = {ax: plain(cum[ax] / S.Ttot[ax]) for ax in AXES}
        out.append(ent)
    return out


def case_result(case):
    S = System(case["model"])
    S.member_axes = bool(case.get("member_axes"))
    rec = Recovery(S)
    modes, clusters = modal_basis(S)
    mc = modal_checks(S, modes, clusters)
    spectra = {s["id"]: s for s in case["spectra"]}
    n_shapes = max(r["extracted_modes"] for r in case["requests"])
    # the scope's delta_c clusters against the exact-repeat groups: they coincide except where stated
    scl = scope_clusters(modes)
    exc = case.get("scope_cluster_exception")
    if exc is None:
        assert scl == list(clusters), case["id"]
    elif exc == "unextracted":
        assert [c for c in scl if c[0] < n_shapes] == [c for c in clusters if c[0] < n_shapes], case["id"]
        assert scl != list(clusters)
    else:
        assert exc == "distinct modes inside clusters" and scl != list(clusters)
    out = {"id": case["id"], "serves": case["serves"], "purpose": case["purpose"], "model": case["model"],
           "derived_model": derived_model(S, rec),
           "spectra": case["spectra"]}
    out["modal_basis"] = {
        "basis": "DEL-04-09 REQ-005: eigenproblem on the free DOFs with dynamic-only restraints included; "
                 "excitation p_k = (M R_k)_f; every finite mode listed, shapes for the modes any request extracts",
        "total_mass_by_direction_kg": {ax: plain(S.Ttot[ax]) for ax in AXES},
        "bounding_box_diagonal_m": plain(S.ell),
        "n_reduced_dofs": S.nr,
        "clusters": publish_modes(S, modes, clusters, n_shapes),
        "scope_clusters": [{"modes": [md["index"] for md in modes[s0:s0 + d]],
                            "standing_frequency_Hz": plain(modes[s0]["f"])} for s0, d in scl],
    }
    if case.get("without_snubbers"):
        spec2 = dict(case["model"])
        spec2["snubbers"] = []
        S2 = System(spec2)
        m2, c2 = modal_basis(S2)
        out["modal_basis_without_dynamic_only_restraints"] = {
            "same_as_case": case["without_snubbers"],
            "frequency_Hz": [plain(m2[s]["f"]) for s, _ in c2]}
    analyses = []
    cross = {
        **({"stiffness_assembly_equals_f1_form_stiffness_exactly": S.assembly_check} if S.assembly_rel is None else
           {"stiffness_assembly_vs_f1_form_stiffness_rel": show(S.assembly_rel)}),
        "eigen_scaled_residual_max": show(mc["eigen_residual"]),
        "m_orthogonality_max": show(mc["m_orthogonality"]),
        "sturm_counts_at_gap_shifts": mc["sturm_counts"],
        "sturm_counts_expected": mc["sturm_expected"],
        "sturm_counts_agree": mc["sturm_ok"],
        "min_relative_gap_between_distinct_frequencies": show(mc["min_relative_frequency_gap"]),
        "sum_effective_mass_all_modes_vs_pT_Minv_p_rel": {ax: show(rel(*mc["meff_all"][ax])) for ax in AXES},
        "all_modes_periodic_free_load_rel": show(mc["all_periodic_free_load"]),
    }
    assert mc["sturm_ok"] and mc["eigen_residual"] < Decimal("1e-60") and mc["m_orthogonality"] < Decimal("1e-55")
    out["modal_cross_checks"] = cross
    bm = case.get("benchmark")
    if bm:
        out["frequency_list"] = frequency_list(S, modes, bm.get("planes"), n_shapes)
    for req in case["requests"]:
        grid = case.get("grids", {}).get(req["id"])
        an = analyse(S, rec, modes, clusters, spectra, req, case.get("mutations", False), grid)
        pa = publish_analysis(S, rec, req, an, clusters, case.get("publish_quantities"))
        if bm and bm.get("single_direction_check"):
            pa["single_direction_check"] = single_direction(rec, req, an)
        if bm and bm.get("close_pair"):
            pa["close_pair"] = close_pair(rec, req, an, modes, bm["close_pair"])
        if bm and bm.get("chain"):
            pa["chain"] = chain_summary(rec, req, an, modes, bm["chain"], case.get("publish_quantities"))
        wit = case.get("witness", {})
        if wit.get("cluster_sums"):
            pa.update(cluster_witness(S, rec, req, an, wit))
            pa["comparability"] = comparability(modes, scl, pa)
        if wit.get("basis") and req["id"] == wit["basis"]["request"]:
            pa["basis_witness"] = basis_witness(S, rec, req, an, modes, wit["basis"])
        analyses.append(pa)
    out["analyses"] = analyses
    if case.get("mesh_series"):
        out["mesh_series"] = mesh_series(case)
    return out


DELTA_C = Decimal(1) / Decimal(2 ** 20)


def plane_of(S, md):
    """Plane of a mode of a straight pipe along X: Y (uy, rz), Z (uz, ry), axial (ux), torsion (rx)."""
    grp = {"XY plane (uy, rz)": (1, 5), "XZ plane (uz, ry)": (2, 4), "axial (ux)": (0,), "torsion (rx)": (3,)}
    e = {k: sum((md["phi"][6 * n + d] ** 2 for n in range(len(S.nodes)) for d in v), Decimal(0))
         for k, v in grp.items()}
    best = max(e, key=lambda k: e[k])
    assert all(v <= FLUSH * e[best] for k, v in e.items() if k != best), "mode is not in one plane"
    return best


def frequency_list(S, modes, planes, n_ext):
    lst = []
    cl = 0
    gaps = []
    near = []
    for k, md in enumerate(modes):
        if k > 0:
            lo, hi = modes[k - 1]["lam"], md["lam"]
            g = (hi - lo) / hi
            gaps.append((g, k))
            if g <= DELTA_C:
                pass
            else:
                cl += 1
            fr = (md["f"] - modes[k - 1]["f"]) / modes[k - 1]["f"]
            if fr <= Decimal("0.01"):
                near.append([k, k + 1, show(fr)])
        ent = {"mode": md["index"], "frequency_Hz": plain(md["f"]), "scope_cluster": cl}
        if planes:
            ent["plane"] = plane_of(S, md)
        lst.append(ent)
    ext_gaps = [g for g, k in gaps if k < n_ext]
    return {
        "scope_cluster_tolerance": "delta_c = 2^-20 on eigenvalues, lambda_{i+1} - lambda_i <= delta_c lambda_{i+1} "
                                   "(DEL-04-09 REQ-003)",
        "modes": lst,
        "min_relative_eigenvalue_gap_all_modes": show(min(g for g, _ in gaps)),
        "min_relative_eigenvalue_gap_extracted_modes": show(min(ext_gaps)) if ext_gaps else None,
        "any_two_eigenvalues_within_delta_c": any(g <= DELTA_C for g, _ in gaps),
        "adjacent_pairs_within_1_percent_in_frequency": near,
        "adjacent_pairs_within_delta_c": [[k, k + 1, show(g)] for g, k in gaps if g <= DELTA_C],
    }


def is_local(k):
    """Element end-force keys in member axes (published only in their own block)."""
    return k.endswith("_m")


def unit_scales(an, units, local=False):
    scales = {}
    for per in an["contrib"].values():
        for vals in per.values():
            for k, v in vals.items():
                if is_local(k) == local:
                    scales[units[k]] = max(scales.get(units[k], Decimal(0)), abs(v))
    for r in an["rigid"].values():
        for k, v in r["vals"].items():
            if is_local(k) == local:
                scales[units[k]] = max(scales.get(units[k], Decimal(0)), abs(v))
    return scales


def single_direction(rec, req, an):
    scales = unit_scales(an, rec.unit)
    dirs = [d["direction"] for d in req["directions"]]
    multi = []
    for k in rec.keys:
        s = scales.get(rec.unit[k], Decimal(0))
        nz = [ax for ax in dirs if an["base_perdir"][ax][k] > FLUSH * s]
        if len(nz) > 1:
            multi.append(k)
    worst = Decimal(0)
    by = {}
    for lab, comb in an["muts"]:
        by[(lab["modal_combination"], lab["missing_mass"], lab["directional_combination"])] = comb
    pairs = 0
    for (mo, rr, dr), comb in by.items():
        if dr == "SRSS" and (mo, rr, "ABSOLUTE_SUM") in by:
            other = by[(mo, rr, "ABSOLUTE_SUM")]
            pairs += 1
            for k in rec.keys:
                s = scales.get(rec.unit[k], Decimal(0))
                worst = max(worst, abs(comb[k] - other[k]) / s if s else Decimal(0))
    assert not multi and worst < Decimal("1e-60")
    return {"statement": "every response quantity receives contributions from one direction only (Y-plane "
                         "quantities from Y, Z-plane quantities from Z; axial and torsional quantities from neither), "
                         "so every directional rule gives the same combined value",
            "quantities_with_more_than_one_direction": multi,
            "directional_SRSS_vs_ABSOLUTE_SUM_max_difference_rel_to_unit_scale": show(worst),
            "rule_pairs_compared": pairs}


def close_pair(rec, req, an, modes, idx):
    ax = req["directions"][0]["direction"]
    i, j = idx
    mi, mj = modes[i - 1], modes[j - 1]
    scales = unit_scales(an, rec.unit)
    comps = {}
    for k in rec.keys:
        if not (k.startswith("B.") or k.startswith("SA.")):
            continue
        a, b = an["contrib"][ax][i][k], an["contrib"][ax][j][k]
        s = scales.get(rec.unit[k], Decimal(0))
        za, zb = abs(a) <= FLUSH * s, abs(b) <= FLUSH * s
        rel_ = "one or both zero" if za or zb else ("same sign" if (a > 0) == (b > 0) else "opposite sign")
        srss, cqc = an["per_rule"]["SRSS"][ax][k], an["per_rule"]["CQC"][ax][k]
        cmp_ = "equal" if abs(srss - cqc) <= FLUSH * s else ("CQC < SRSS" if cqc < srss else "CQC > SRSS")
        comps[k] = {"relation": rel_, "CQC_vs_SRSS_over_extracted_modes": cmp_}
    return {"modes": idx, "frequency_Hz": [plain(mi["f"]), plain(mj["f"])],
            "relative_frequency_gap": plain((mj["f"] - mi["f"]) / mi["f"]),
            "damping_ratio": plain(an["zeta"]), "rho": plain(rho(an["zeta"], mi["omega"], mj["omega"])),
            "components_tip_B_and_anchor_SA": comps}


def chain_summary(rec, req, an, modes, idx, prefixes):
    ax = req["directions"][0]["direction"]
    f = [modes[i - 1]["f"] for i in idx]
    diff = [k for k in rec.keys if any(k.startswith(p) for p in prefixes)
            if rel(an["per_rule"]["GROUPING"][ax][k], an["per_rule"]["TEN_PERCENT"][ax][k]) > Decimal("1e-30")]
    assert f[1] <= f[0] * Decimal("1.1") < f[2] <= f[1] * Decimal("1.1") and diff
    return {"modes": idx, "frequency_Hz": [plain(x) for x in f],
            "ratios": {"f22/f21": plain(f[1] / f[0]), "f23/f21": plain(f[2] / f[0]), "f23/f22": plain(f[2] / f[1])},
            "effect": "GROUPING puts the first two in one group and starts a new group at the third; the ten "
                      "percent method also couples the second and third",
            "quantities_where_GROUPING_differs_from_TEN_PERCENT": diff}


def cluster_witness(S, rec, req, an, wit):
    """Scope clusters of distinct modes: each mode's own data, the cluster sums, and mode-by-mode alternatives."""
    keys = [k for k in rec.keys if any(k.startswith(p) for p in wit["quantities"])]
    F = Fmt(unit_scales(an, rec.unit))
    dirs = [d["direction"] for d in req["directions"]]
    pidx = an["pidx"]
    out = {}
    cl = []
    for s0, d in an["sclusters"]:
        mem = [md for md in an["ext"] if s0 < md["index"] <= s0 + d]
        if len(mem) < 2:
            continue
        ent = {"modes": [md["index"] for md in mem],
               "eigenvalue_rad2_per_s2": [plain(md["lam"]) for md in mem],
               "frequency_Hz": [plain(md["f"]) for md in mem],
               "relative_eigenvalue_gaps": [show((b["lam"] - a["lam"]) / b["lam"]) for a, b in zip(mem, mem[1:])],
               "delta_c": "2^-20 = 9.54e-7",
               "cluster_frequency_Hz (lowest; for rho, the closeness tests and periodic/rigid only)": plain(mem[0]["f"]),
               "status": "periodic" if mem[0]["index"] in pidx else "rigid (extracted)",
               "per_direction": {}}
        for ax in dirs:
            od = {o[0]["index"]: o for o in an["ords"][ax]}
            per = {"own_scaled_ordinates_m_per_s2": [plain(od[md["index"]][2]) for md in mem],
                   "own_contributions_signed": {str(md["index"]): {k: F(an["contrib"][ax][md["index"]][k], rec.unit[k])
                                                                   for k in keys} for md in mem},
                   "cluster_sum_signed": {k: F(sum((an["contrib"][ax][md["index"]][k] for md in mem), Decimal(0)),
                                              rec.unit[k]) for k in keys}}
            ent["per_direction"][ax] = per
        cl.append(ent)
    out["cluster_witness"] = cl
    if not wit.get("mode_by_mode"):
        if wit.get("rigid_load") and an["rigid"]:
            out["rigid_load_signed"] = {
                ax: {f"{S.nodes[g // 6]}.{DOFS[g % 6]}": F(an["rigid"][ax]["F"][g], "N" if g % 6 < 3 else "N*m")
                     for g in range(S.n)} for ax in an["rigid"]}
            out["rigid_load_note"] = ("F_k = a_r (M R_k - sum over the periodic modes, the whole straddling cluster "
                                      "included, of Gamma_ik M phi_i) on all DOFs; moments in N*m")
        return out
    # the same modes combined mode by mode (each its own frequency): not the adopted reading
    alt = {"label": "not the adopted reading",
           "rule": "every periodic mode a separate unit with its own frequency; plain SRSS, and CQC with "
                   "rho from the modes' own frequencies; the request's directional rule",
           "SRSS": {}, "CQC": {}}
    for rule in ("SRSS", "CQC"):
        perdir = {}
        for ax in dirs:
            units = [(md["omega"], md["f"], an["contrib"][ax][md["index"]], [md]) for md in an["periodic"]]
            perdir[ax] = {k: combine(rule, units, k, an["zeta"]) for k in keys}
        if an["rig_mag"]:
            rr = req["missing_mass"]["rigid_periodic_combination"]
            perdir = {ax: {k: total(rr, perdir[ax][k], an["rig_mag"][ax][k]) for k in keys} for ax in dirs}
        comb = {k: directional(req["directional_combination"], [perdir[ax][k] for ax in dirs]) for k in keys}
        alt[rule] = {"per_direction": {ax: {k: F(perdir[ax][k], rec.unit[k]) for k in keys} for ax in dirs},
                     "combined": {k: F(comb[k], rec.unit[k]) for k in keys}}
    adopted = {}
    for rule in ("SRSS", "CQC"):
        perdir = {ax: an["per_rule"][rule][ax] for ax in dirs}
        if an["rig_mag"]:
            rr = req["missing_mass"]["rigid_periodic_combination"]
            perdir = {ax: {k: total(rr, perdir[ax][k], an["rig_mag"][ax][k]) for k in keys} for ax in dirs}
        adopted[rule] = {k: F(directional(req["directional_combination"], [perdir[ax][k] for ax in dirs]), rec.unit[k])
                         for k in keys}
    alt["adopted_reading_for_comparison"] = adopted
    out["not_the_adopted_reading_mode_by_mode"] = alt
    if wit.get("rigid_load") and an["rigid"]:
        out["rigid_load_signed"] = {
            ax: {f"{S.nodes[g // 6]}.{DOFS[g % 6]}": F(an["rigid"][ax]["F"][g], "N" if g % 6 < 3 else "N*m")
                 for g in range(S.n)} for ax in an["rigid"]}
        out["rigid_load_note"] = ("F_k = a_r (M R_k - sum over the periodic modes, the whole straddling cluster "
                                  "included, of Gamma_ik M phi_i) on all DOFs; moments in N*m")
    return out


def comparability(modes, scl, pa):
    """Which published blocks of a C10 analysis a product can be compared against at the verification tolerance."""
    near = [[md["index"] for md in modes[s0:s0 + d]] for s0, d in scl if d > 1]
    ok = ["frequency_list and the eigenvalues, frequencies and periods of every mode",
          "spectral_ordinates and cluster_witness[*].per_direction[*].own_scaled_ordinates_m_per_s2",
          "cluster_witness[*].per_direction[*].cluster_sum_signed",
          "cqc_correlation and grouping_groups (by cluster)",
          "periodic_modal_combined", "per_direction_combined", "combined", "mutations",
          "missing_mass_disclosure",
          "not_the_adopted_reading_mode_by_mode.CQC (rho = 1 - O(1e-12) across each pair)"]
    if "periodic_modal_combined_correction_off" in pa:
        ok.append("periodic_modal_combined_correction_off")
    if "rigid_response_signed" in pa:
        ok += ["rigid_acceleration_m_per_s2", "rigid_response_signed", "rigid_magnitude", "rigid_load_signed"]
    return {
        "comparable at the verification tolerance": ok,
        "not comparable per mode: basis within a near-degenerate cluster is ill conditioned": [
            "contributions_signed entries of the modes listed below",
            "cluster_witness[*].per_direction[*].own_contributions_signed",
            "modal_basis.clusters entries of the modes listed below: shape, normalization, normalizing_dof, "
            "generalized_mass, participation_factor, effective_mass_kg, effective_mass_fraction, and a cumulative "
            "fraction taken after a mode that does not complete its scope cluster",
            "not_the_adopted_reading_mode_by_mode.SRSS"],
        "modes_in_near_degenerate_scope_clusters": near,
        "observation": "REFUTATION.md (R5) reports, from 16-digit decimal arithmetic used as a binary64 stand-in, a "
                       "rotation of mode 1 within its pair of about 1.7e-9 on a route through Cholesky of K and "
                       "about 4.2e-6 on a route through Cholesky of M",
    }


def normalize_unit_max_translation(S, v):
    tr = [(abs(v[g]), g) for g in S.free if g % 6 < 3]
    big = max(a for a, _ in tr)
    g0 = sorted(g for a, g in tr if a >= big * (1 - TIE_REL))[0]
    return [x / v[g0] for x in v]


def basis_witness(S, rec, req, an, modes, bw):
    """W1: two M-orthogonal bases of one exactly repeated pair give the same signed cluster sums."""
    keys = [k for k in rec.keys if any(k.startswith(p) for p in bw["quantities"])]
    F = Fmt(unit_scales(an, rec.unit))
    a, b = modes[bw["modes"][0] - 1], modes[bw["modes"][1] - 1]
    assert a["cluster"] == b["cluster"] and a["multiplicity"] == 2
    va = [x / a["m"].sqrt() for x in a["phi"]]
    vb = [x / b["m"].sqrt() for x in b["phi"]]
    c, s_ = Decimal(3) / 5, Decimal(4) / 5
    lam = a["lam"]
    bases = {
        "basis_1": [normalize_unit_max_translation(S, va), normalize_unit_max_translation(S, vb)],
        "basis_2": [normalize_unit_max_translation(S, [c * x + s_ * y for x, y in zip(va, vb)]),
                    normalize_unit_max_translation(S, [-s_ * x + c * y for x, y in zip(va, vb)])],
    }
    p1, p2 = bases["basis_2"]
    bases["euclidean_rotation_of_basis_2"] = [[c * x + s_ * y for x, y in zip(p1, p2)],
                                              [-s_ * x + c * y for x, y in zip(p1, p2)]]
    bases["sheared_basis_not_M_orthogonal"] = [p1, normalize_unit_max_translation(
        S, [x + y / 2 for x, y in zip(p2, p1)])]
    dirs = [d["direction"] for d in req["directions"]]
    sc = {ax: [o for o in an["ords"][ax] if o[0]["index"] == a["index"]][0][2] for ax in dirs}
    out = {"modes": bw["modes"], "frequency_Hz": plain(a["f"]), "bases": {}}
    sums = {}
    for name, vecs in bases.items():
        ent = {"vectors": []}
        Mv = [mv(S.M, v) for v in vecs]
        mx = dot(vecs[0], Mv[1])
        mref = (dot(vecs[0], Mv[0]) * dot(vecs[1], Mv[1])).sqrt()
        ent["M_cross_product_v1_M_v2_kg"] = plain(0 if abs(mx) < FLUSH * mref else mx)
        sm = {ax: {k: Decimal(0) for k in rec.keys} for ax in dirs}
        for v, Mvv in zip(vecs, Mv):
            m = dot(v, Mvv)
            gam = {ax: dot(v, S.MR[ax]) / m for ax in AXES}
            vent = {"generalized_mass_kg": plain(m), "participation_factor": {ax: plain(gam[ax]) for ax in AXES},
                    "tip_translation_uy_uz": [plain(v[6 * S.model.idx[bw["tip"]] + 1]),
                                              plain(v[6 * S.model.idx[bw["tip"]] + 2])]}
            vent["contributions_signed"] = {}
            for ax in dirs:
                u = [gam[ax] * sc[ax] / lam * x for x in v]
                R, _ = rec.recover(u, [lam * x for x in u])
                vent["contributions_signed"][ax] = {k: F(R[k], rec.unit[k]) for k in keys}
                for k in rec.keys:
                    sm[ax][k] += R[k]
            ent["vectors"].append(vent)
        ent["cluster_sum_signed"] = {ax: {k: F(sm[ax][k], rec.unit[k]) for k in keys} for ax in dirs}
        sums[name] = sm
        out["bases"][name] = ent
    scales = unit_scales(an, rec.unit)

    def maxdiff(x, y):
        return max(abs(x[ax][k] - y[ax][k]) / scales[rec.unit[k]] for ax in dirs for k in rec.keys
                   if scales.get(rec.unit[k]))

    out["bases"]["basis_1"]["construction"] = (
        "the canonical M-orthonormal basis of the pair, each vector then normalized to unit maximum translation "
        "(the published normalization)")
    out["bases"]["basis_2"]["construction"] = (
        "the same M-orthonormal basis rotated by the angle with cos = 3/5, sin = 4/5 (an M-orthonormal "
        "rotation), each vector then normalized to unit maximum translation")
    out["bases"]["euclidean_rotation_of_basis_2"]["construction"] = (
        "basis_2's normalized vectors rotated by cos = 3/5, sin = 4/5 in the Euclidean sense. In this pair the "
        "y- and z-plane modes are isotropic (equal generalized masses and equal Euclidean norms in every "
        "normalized M-orthogonal basis), so the rotated basis stays M-orthogonal and gives the same sums")
    out["bases"]["sheared_basis_not_M_orthogonal"]["construction"] = (
        "counter-witness, outside the premise: basis_2's first vector and its second vector plus half the first, "
        "renormalized; it spans the eigenspace but is not M-orthogonal, and its sum differs")
    out["comparability"] = {
        "comparable at the verification tolerance": [
            "cluster_sum_signed of basis_1, basis_2 and euclidean_rotation_of_basis_2 (equal to each other and to "
            "the published cluster sum)", "frequency_Hz",
            "combined_every_modal_rule of basis_1 and basis_2 (formed on the cluster sums; equal to each other)"],
        "not comparable per mode: the basis of an exactly repeated cluster is arbitrary": [
            "vectors[*]: generalized_mass_kg, participation_factor, tip_translation_uy_uz, contributions_signed, and "
            "M_cross_product_v1_M_v2_kg, in every basis"]}
    out["sum_difference_basis_1_vs_basis_2_rel_to_unit_scale"] = show(maxdiff(sums["basis_1"], sums["basis_2"]))
    out["sum_difference_basis_1_vs_euclidean_rotation_rel"] = show(
        maxdiff(sums["basis_1"], sums["euclidean_rotation_of_basis_2"]))
    out["sum_difference_basis_1_vs_sheared_rel (outside the premise)"] = show(
        maxdiff(sums["basis_1"], sums["sheared_basis_not_M_orthogonal"]))
    # the cluster sum equals the published reading-A sum
    pub = {ax: {k: sum((an["contrib"][ax][md][k] for md in bw["modes"]), Decimal(0)) for k in rec.keys} for ax in dirs}
    out["sum_difference_basis_1_vs_published_cluster_sum_rel"] = show(maxdiff(sums["basis_1"], pub))
    assert maxdiff(sums["basis_1"], sums["basis_2"]) < Decimal("1e-60")
    assert maxdiff(sums["basis_1"], pub) < Decimal("1e-60")
    assert maxdiff(sums["basis_1"], sums["sheared_basis_not_M_orthogonal"]) > Decimal("1e-3")
    # every modal rule of the scope on the cluster sums, from each basis: the pair's unit takes the basis's
    # cluster sum, every other periodic cluster its published sum; then the request's directional rule
    rules = ["SRSS", "CQC", "GROUPING", "TEN_PERCENT", "ABSOLUTE_SUM"]
    pair = set(bw["modes"])
    comb_all = {}
    for name in ("basis_1", "basis_2"):
        per_rule = {}
        for rule in rules:
            perdir = {}
            for ax in dirs:
                units = []
                for u in an["units_for"](ax, an["periodic"]):
                    members = {m["index"] for m in u[3]}
                    units.append((u[0], u[1], sums[name][ax] if members == pair else u[2], u[3]))
                perdir[ax] = {k: combine(rule, units, k, an["zeta"]) for k in rec.keys}
            if an["rig_mag"]:
                rr = req["missing_mass"]["rigid_periodic_combination"]
                perdir = {ax: {k: total(rr, perdir[ax][k], an["rig_mag"][ax][k]) for k in rec.keys} for ax in dirs}
            comb = {k: directional(req["directional_combination"], [perdir[ax][k] for ax in dirs]) for k in rec.keys}
            per_rule[rule] = (perdir, comb)
        comb_all[name] = per_rule
        out["bases"][name]["combined_every_modal_rule"] = {
            rule: {"per_direction": {ax: {k: F(pd[ax][k], rec.unit[k]) for k in keys} for ax in dirs},
                   "combined": {k: F(c[k], rec.unit[k]) for k in keys}}
            for rule, (pd, c) in per_rule.items()}

    def maxdiff_comb(x, y):
        worst = Decimal(0)
        for k in rec.keys:
            s = scales.get(rec.unit[k])
            if not s:
                continue
            worst = max([worst, abs(x[1][k] - y[1][k]) / s] + [abs(x[0][ax][k] - y[0][ax][k]) / s for ax in dirs])
        return worst

    d12 = max(maxdiff_comb(comb_all["basis_1"][r], comb_all["basis_2"][r]) for r in rules)
    # against the published combinations of this analysis, for the rules it publishes
    pubc = {r: ({ax: an["per_rule"][r][ax] for ax in dirs},
                {k: directional(req["directional_combination"], [an["per_rule"][r][ax][k] for ax in dirs])
                 for k in rec.keys}) for r in rules if r in an["per_rule"]}
    dpub = max(maxdiff_comb(comb_all["basis_1"][r], pubc[r]) for r in pubc)
    out["combined_every_modal_rule_cross_check"] = {
        "statement": "for every modal rule of the scope (SRSS, CQC, GROUPING, TEN_PERCENT, ABSOLUTE_SUM) and every "
                     "quantity, per direction and over directions, the combined values from basis_1 and basis_2 "
                     "agree; both are formed on cluster sums (CLM-001 Repeated modes, REQ-003)",
        "basis_1_vs_basis_2_max_difference_rel_to_unit_scale": show(d12),
        "basis_1_vs_published_periodic_modal_combined_max_difference_rel_to_unit_scale": show(dpub),
        "published_rules_compared": sorted(pubc),
    }
    assert d12 < Decimal("1e-60") and dpub < Decimal("1e-60")
    return out


def mesh_series(case):
    ms = case["mesh_series"]
    req = [r for r in case["requests"] if r["id"] == ms["request"]][0]
    spectra = {s["id"]: s for s in case["spectra"]}
    out = {"request": ms["request"],
           "note": "the same pipe, supports, spectra and request as the named analysis, with N equal members; "
                   "every quantity receives contributions from one direction only, so the directional rule does "
                   "not change these values (directional SRSS shown)",
           "series": []}
    models = dict(ms["models"])
    models[8] = case["model"]
    for n in sorted(models):
        S = System(models[n])
        rec = Recovery(S)
        modes, clusters = modal_basis(S)
        mc = modal_checks(S, modes, clusters)
        assert mc["sturm_ok"] and mc["eigen_residual"] < Decimal("1e-60")
        grid = {"modal": ["SRSS", "CQC", "GROUPING", "ABSOLUTE_SUM", "TEN_PERCENT"], "rigid": ["OFF"],
                "directional": ["SRSS"]}
        an = analyse(S, rec, modes, clusters, spectra, req, False, grid)
        assert an["checks"]["nodal_balance"] < Decimal("1e-60") and an["checks"]["equilibrium"] < Decimal("1e-60")
        F = Fmt(unit_scales(an, rec.unit))
        comb = {}
        for lab, c in an["muts"]:
            comb[lab["modal_combination"]] = {k: F(c[k], rec.unit[k]) for k in ms["quantities"]}
        out["series"].append({
            "members": n, "model": models[n] if n != 8 else "the model of this case",
            "frequencies": [{"mode": md["index"], "frequency_Hz": plain(md["f"]), "plane": plane_of(S, md)}
                            for md in modes[:req["extracted_modes"]]],
            "combined": comb,
            "cross_checks": {"sturm_counts_agree": mc["sturm_ok"], "eigen_scaled_residual_max": show(mc["eigen_residual"]),
                             "nodal_balance_rel_max": show(an["checks"]["nodal_balance"]),
                             "global_equilibrium_rel_max": show(an["checks"]["equilibrium"])}})
    return out


def derived_model(S, rec):
    return {
        "restrained_dofs": [[S.nodes[g // 6], DOFS[g % 6], sid] for g, sid in sorted(S.restrained.items())],
        "skew_rigid_constraints": [{"id": c, "node": n, "direction": [str(v) for v in d]} for c, n, d in S.constraints],
        "member_frames": {e["id"]: {"length_m": str(e["length"]) if not isinstance(e["length"], Decimal)
                                    else dstr(e["length"]),
                                    "rows_x_y_z_in_global": [[str(v) if not isinstance(v, Decimal) else dstr(v)
                                                              for v in r] for r in e["frame"]]}
                          for e in S.elems},
        "skew_snubber_motion_axes_unit": rec.motion_axes,
        "quantities": [[k, u] for k, u in rec.qs if not is_local(k)],
        **({"member_axes_quantities": [[k, u] for k, u in rec.qs if is_local(k)]} if rec.member_axes else {}),
    }


def publish_analysis(S, rec, req, an, clusters, prefixes=None):
    keys = [k for k in rec.keys if not is_local(k) and (not prefixes or any(k.startswith(p) for p in prefixes))]
    units = rec.unit
    # flush scales per unit: largest magnitude over contributions and rigid responses
    scales = unit_scales(an, units)
    F = Fmt(scales)

    def qmap(vals):
        return {k: F(vals[k], units[k]) for k in keys}

    dirs = [d["direction"] for d in req["directions"]]
    repeated = {s for s, d in clusters if d > 1}
    out = {"request": req}
    out["rigid_frequency_Hz"] = plain(an["fr"]) if an["fr"] is not None else None
    out["mode_status"] = [{"mode": md["index"], "frequency_Hz": plain(md["f"]),
                           "status": "periodic" if md["index"] in an["pidx"] else "rigid (extracted)"}
                          for md in an["ext"]]
    ords = {}
    for ax in dirs:
        ords[ax] = [{"mode": md["index"],
                     "spectral_ordinate_m_per_s2": plain(Sv) if Sv is not None else None,
                     "scaled_ordinate_m_per_s2": plain(sS) if sS is not None else None,
                     "note": None if Sv is not None else "outside the tabulated range (rigid mode: not used)"}
                    for md, Sv, sS, st in an["ords"][ax]]
    out["spectral_ordinates"] = ords
    if an["fr"] is not None:
        out["rigid_acceleration_m_per_s2"] = {ax: plain(an["rigid"][ax]["a_r"]) for ax in dirs}
    # contributions
    contrib = {}
    for ax in dirs:
        lst = []
        done = set()
        for md in an["ext"]:
            if md["index"] not in an["contrib"][ax]:
                continue
            c0 = md["cluster"]
            if md["multiplicity"] == 1:
                lst.append({"mode": md["index"], "status": "periodic" if md["index"] in an["pidx"] else "rigid (extracted)",
                            "values": qmap(an["contrib"][ax][md["index"]])})
            elif c0 not in done:
                done.add(c0)
                mem = [m for m in an["ext"] if m["cluster"] == c0]
                vals = {k: sum((an["contrib"][ax][m["index"]][k] for m in mem), Decimal(0)) for k in keys}
                lst.append({"modes": [m["index"] for m in mem],
                            "status": "periodic" if md["index"] in an["pidx"] else "rigid (extracted)",
                            "values_signed_sum_over_repeated_cluster": qmap(vals)})
        contrib[ax] = lst
    out["contributions_signed"] = contrib
    ax0 = dirs[0]
    ext_units = an["units_for"](ax0, an["ext"]) if an["off"] is not None else an["units_for"](ax0, an["periodic"])
    labels = [[m["index"] for m in u[3]] for u in ext_units]
    if an["zeta"] > 0 and ext_units:
        out["cqc_correlation"] = {
            "units": labels,
            "note": "rho_ij of the units of the combination (a repeated cluster is one unit), for the extracted "
                    "modes; the periodic set uses its leading block",
            "rho": [[plain(Decimal(1) if i == j else rho(an["zeta"], ui[0], uj[0])) for j, uj in enumerate(ext_units)]
                    for i, ui in enumerate(ext_units)]}
    pu = an["units_for"](ax0, an["periodic"])
    out["grouping_groups"] = {"periodic": [[labels[k] for k in g] for g in groups([u[1] for u in pu])]}
    if an["off"] is not None:
        out["grouping_groups"]["correction_off"] = [[labels[k] for k in g] for g in groups([u[1] for u in ext_units])]
    if an["rigid"]:
        out["rigid_response_signed"] = {ax: qmap(an["rigid"][ax]["vals"]) for ax in dirs}
    out["periodic_modal_combined"] = {rule: {ax: qmap(v[ax]) for ax in dirs} for rule, v in an["per_rule"].items()}
    if an["off"] is not None:
        out["periodic_modal_combined_correction_off"] = {rule: {ax: qmap(v[ax]) for ax in dirs}
                                                         for rule, v in an["off"].items()}
    if an["rig_mag"]:
        out["rigid_magnitude"] = {ax: qmap(an["rig_mag"][ax]) for ax in dirs}
    out["per_direction_combined"] = {ax: qmap(an["base_perdir"][ax]) for ax in dirs}
    out["combined"] = qmap(an["base_comb"])
    if an["muts"]:
        out["mutations"] = [{**lab, "combined": qmap(comb)} for lab, comb in an["muts"]]
    if an["reading_b"] is not None:
        rb = an["reading_b"]
        out["repeated_cluster_readings"] = {
            "reading_A": "modes of one repeated cluster are summed algebraically first (the signed sums above), "
                         "then the rule is applied across clusters; basis-invariant. Every *_combined block of "
                         "this analysis is under reading A",
            "reading_B": "the rule is applied over the individual vectors of whatever basis the solver returns; "
                         "under SRSS the result depends on the basis. Published: its minimum and maximum per "
                         "direction and quantity as the basis of each repeated pair rotates (each bound taken "
                         "separately per quantity)",
            "reading_B_SRSS_range": {ax: {"min": qmap(rb["ranges"][ax]["min"]), "max": qmap(rb["ranges"][ax]["max"])}
                                     for ax in dirs},
            "note_CQC": "CQC with rho = 1 within an exactly repeated cluster equals reading A under CQC for any "
                        "basis",
        }
    disc = {}
    for ax in dirs:
        d = an["disc"][ax]
        disc[ax] = {
            "total_mass_T_k_kg": plain(d["T"]),
            "periodic_effective_mass_kg": plain(d["periodic"]),
            "periodic_cumulative_fraction": plain(d["periodic"] / d["T"]),
            "fraction_not_represented_by_periodic_modes": plain(1 - d["periodic"] / d["T"]),
            "extracted_cumulative_fraction": plain(d["extracted"] / d["T"]),
            "fraction_not_represented_correction_off": plain(1 - d["extracted"] / d["T"]),
            "all_finite_modes_cumulative_fraction": plain(d["all"] / d["T"]),
            "fraction_moving_with_supports": plain(1 - d["all"] / d["T"]),
        }
    out["missing_mass_disclosure"] = disc
    cc = {"nodal_balance_rel_max": show(an["checks"]["nodal_balance"]),
          "global_equilibrium_forces_and_moments_rel_max": show(an["checks"]["equilibrium"])}
    if S.constraints:
        cc["skew_constraint_dT_u_rel_max"] = show(an["checks"]["skew_constraint"])
    for ax, r in an["rigid_checks"].items():
        cc[f"rigid_{ax}"] = {
            "static_solve_vs_expansion_over_nonperiodic_modes_rel": show(r["static_vs_modal_expansion"]),
            "total_reaction_along_direction_N": plain(r["total_reaction"]),
            "expected_minus_a_r_times_T_k_minus_periodic_meff_N": plain(r["expected"]),
            "difference_rel": show(r["total_reaction_rel"])}
        assert r["static_vs_modal_expansion"] < Decimal("1e-50") and r["total_reaction_rel"] < Decimal("1e-60"), (req["id"], ax, r)
    if an["reading_b"] is not None:
        cc["reading_B_range_vs_rotated_basis_recovery_rel"] = show(an["reading_b"]["rotated_basis_check"])
        assert an["reading_b"]["rotated_basis_check"] < Decimal("1e-50"), (req["id"], an["reading_b"]["rotated_basis_check"])
    assert an["checks"]["nodal_balance"] < Decimal("1e-60") and an["checks"]["equilibrium"] < Decimal("1e-60")
    out["cross_checks"] = cc
    lkeys = [k for k in rec.keys if is_local(k)]
    if lkeys:
        out["element_end_forces_member_axes"] = member_axes_block(S, rec, req, an, lkeys)
    return out


def member_axes_block(S, rec, req, an, lkeys):
    """The element end forces of every published block, in each member's own axes (new block, C9 and C9-CHAIN)."""
    units = rec.unit
    F = Fmt(unit_scales(an, units, local=True))
    dirs = [d["direction"] for d in req["directions"]]

    def qmap(vals):
        return {k: F(vals[k], units[k]) for k in lkeys}

    out = {
        "frame": "member axes: x from node i to node j, y = y_reference made normal to x and unit, z = x cross y; "
                 "the rows of each member's frame in global axes are in `frames` (and in derived_model.member_frames)",
        "keys": "<member>.<i|j>.<Fx|Fy|Fz|Mx|My|Mz>_m: the end forces and moments acting on the element at that end, "
                "components along the member's x, y, z",
        "note": "these are the signed contributions of the global-axis blocks rotated into member axes, and magnitudes "
                "combined from those rotated contributions; they differ from the global-axis values because a combined "
                "magnitude is a property of the axes it is combined in, and these frames are not signed permutations "
                "of the global axes",
        "frames": {e["id"]: [[dstr(D(x)) for x in row] for row in e["frame"]] for e in S.elems},
    }
    contrib = {}
    for ax in dirs:
        lst = []
        for md in an["ext"]:
            if md["index"] in an["contrib"][ax]:
                lst.append({"mode": md["index"],
                            "status": "periodic" if md["index"] in an["pidx"] else "rigid (extracted)",
                            "values": qmap(an["contrib"][ax][md["index"]])})
        contrib[ax] = lst
    out["contributions_signed"] = contrib
    if an["rigid"]:
        out["rigid_response_signed"] = {ax: qmap(an["rigid"][ax]["vals"]) for ax in dirs}
    out["periodic_modal_combined"] = {rule: {ax: qmap(v[ax]) for ax in dirs} for rule, v in an["per_rule"].items()}
    out["per_direction_combined"] = {ax: qmap(an["base_perdir"][ax]) for ax in dirs}
    out["combined"] = qmap(an["base_comb"])
    if an["muts"]:
        out["mutations"] = [{**lab, "combined": qmap(comb)} for lab, comb in an["muts"]]
    return out


# ============================================================================ driver
def dumps(obj):
    return json.dumps(obj, indent=1, ensure_ascii=False) + "\n"


def build():
    set_point_precision(PREC)
    allc = {c["id"]: c for c in spectrum_cases.cases()}
    files = {}
    for name, ids in OUTPUTS.items():
        cs = [allc[i] for i in ids]
        doc = {
            "document": f"openpipestress.f2_spectrum.discrete.{name[:-5]}",
            "status": "references with a refutation record: REFUTATION.md in this directory",
            "deliverable": "DEL-04-10 (response-spectrum analysis)",
            "units": {"length": "m", "mass": "kg", "force": "N", "moment": "N*m", "time": "s",
                      "frequency": "Hz", "acceleration": "m/s^2", "rotation": "rad"},
            "dof_order": DOFS,
            "number_format": {"pair": f"[decimal with {DIG} significant digits, nearest binary64]",
                              "flush": "a value below 1e-50 of the largest magnitude of its unit in the "
                                       "analysis is printed as 0 (zero by symmetry or decoupling)"},
            "conventions": "README.md in this directory, section \"Conventions a product must match or map\"",
            "inputs_binary64": inputs_table([c["model"] for c in cs], [c["spectra"] for c in cs],
                                            [c["requests"] for c in cs]),
            "cases": [case_result(c) for c in cs],
        }
        files[name] = dumps(doc)
    return files


def sums(files):
    lines = []
    for name in sorted(list(FIXED_FILES) + list(RECORDS) + list(files)):
        data = files[name].encode() if name in files else (OUT / name).read_bytes()
        lines.append(f"{hashlib.sha256(data).hexdigest()}  {name}")
    return "\n".join(lines) + "\n"


def main():
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--check", action="store_true", help="default; regenerate and compare")
    args = ap.parse_args()
    files = build()
    files["SHA256SUMS"] = sums(files)
    if args.write:
        for name, text in files.items():
            (OUT / name).write_text(text)
        print("wrote", ", ".join(sorted(files)))
        return 0
    bad = [n for n, t in files.items() if not (OUT / n).exists() or (OUT / n).read_text() != t]
    if bad:
        print("MISMATCH:", ", ".join(bad))
        return 1
    print("check passed:", ", ".join(sorted(files)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
