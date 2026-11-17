"""W1 recovery references: inertia-inclusive signed modal and rigid-response recovery.

Generator, check and write driver of validation/references/f2_spectrum/recovery_w1.

  python3 -I -B generators/w1_recovery.py            # check: regenerate in memory, compare byte for byte
  python3 -I -B generators/w1_recovery.py --write    # rewrite the JSON files and SHA256SUMS

Formation (mass, stiffness, section bases, Cholesky-Jacobi, Sturm counts and
bisection) uses four frozen reference-side modules of
../../../f1_modal/mass_discrete/generators/, imported unmodified, their
SHA-256 pinned and their import origin asserted. Everything after formation
(modal normalization and participation, spectra, signed recovery with the
inertia terms, the rigid load and its constituent and member/body shares,
balances, combinations, member-axis rotation, variant witnesses and the
replay of two frozen f2 discrete analyses) is written here from DEL-04-10
CLM-001 and DEL-04-09 REQ-005. No product code is imported, read or run.
"""
import argparse
import copy
import hashlib
import json
import os
import sys
from concurrent.futures import ProcessPoolExecutor
from decimal import Decimal, getcontext
from fractions import Fraction
from pathlib import Path

HERE = Path(__file__).resolve().parent
PKG = HERE.parent
F1_DIR = (PKG.parent.parent / "f1_modal" / "mass_discrete" / "generators").resolve()
F2_DISCRETE = (PKG.parent / "discrete").resolve()

# Frozen reference-side modules this generator imports unmodified (f1_modal/mass_discrete, refuted).
F1_PINS = {
    "elements.py": "c2a7b890c90d8c2d016fc72036aad36165173cb9f146158bea7629364bd696f0",
    "model.py": "c219d158aa2aa260153dff303514508a4c41e4983b2b1c5236a2beb79c046761",
    "numeric.py": "e08ff24066f636f0d6c76e38ae95de2ef42e9e1c0f651714910034575d5dd1e7",
    "eigen.py": "22ae605c36ae47ad7e80369ecbee422646bef67acb1d46294a744228589d17c6",
}
# Frozen f2 discrete outputs read (inputs and published values only) by the replay control.
F2_PINS = {
    "c3_cantilever_3d.json": "54860437fd3df398d732595bd152716872e8ec0d05e94265064adbac01a1b1aa",
    "c9_pipe_diagonal.json": "c669ceea4182b2c20b1cacf5d587b5d3151cde55a9fb33f2c112179a48c3e0ba",
}


def sha256_file(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()


for _name, _h in F1_PINS.items():
    if sha256_file(F1_DIR / _name) != _h:
        sys.exit(f"pinned f1 module changed: {F1_DIR / _name}")
# python3 -I implies -P, so neither directory is on the path; the f1 directory goes first, ahead of this
# one, so that no same-named file can shadow a pinned module.
sys.path[:0] = [str(F1_DIR), str(HERE)]

import elements as el  # noqa: E402  (f1_modal, pinned)
import eigen as eg  # noqa: E402  (f1_modal, pinned)
import model as f1m  # noqa: E402  (f1_modal, pinned)
import numeric as f1n  # noqa: E402  (f1_modal, pinned)

for _mod in (el, eg, f1m, f1n):
    assert Path(_mod.__file__).resolve().parent == F1_DIR, f"{_mod.__name__} not loaded from {F1_DIR}"

import w1_cases as cases  # noqa: E402

assert Path(cases.__file__).resolve().parent == HERE

DOFS = f1m.DOFS
FN = {"ux": "Fx", "uy": "Fy", "uz": "Fz", "rx": "Mx", "ry": "My", "rz": "Mz"}
COMP6 = ["Fx", "Fy", "Fz", "Mx", "My", "Mz"]
PREC = 80          # working precision of the published values
PREC_CHECK = 110   # convergence rerun
DIGITS = 40
DELTA_C = Fraction(1, 2 ** 20)
TAU_T = Decimal("1e-6")
TIE_REL = Decimal("1e-25")
FLUSH = Decimal("1e-50")
GUARD = Decimal("1e-35")
CONSTITUENTS = ["metal_translational", "metal_rotary", "contents", "insulation_translational",
                "insulation_rotary", "component_body"]
OUTPUTS = ["w1_base.json", "w1_variants.json", "w1_controls.json"]
RECORDS = ["README.md", "generators/w1_recovery.py", "generators/w1_cases.py"]
OPTIONAL_RECORDS = ["REFUTATION.md"]   # hashed when present; never generated here
STATUS = ("candidate reference, derived and self-checked by its author TASK; no independent refutation "
          "has been made yet")


# ==========================================================================
# Small Decimal helpers
# ==========================================================================
D0 = Decimal(0)


def ctx():
    return getcontext()


def mv(A, x):
    out = []
    for row in A:
        s = D0
        for a, b in zip(row, x):
            if a and b:
                s += a * b
        out.append(s)
    return out


def vdot(x, y):
    s = D0
    for a, b in zip(x, y):
        if a and b:
            s += a * b
    return s


def cross3(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dec_of(x, prec):
    if isinstance(x, f1n.QPi):
        return x.to_decimal(prec)
    if isinstance(x, Decimal):
        return +x
    return f1n.to_decimal(x)


def fq(x):
    return f1n.exact_double(x) if not isinstance(x, Fraction) else x


def cholesky_solve(R, b):
    return eg.backward_t(R, eg.forward(R, b))


def pi_dec():
    return f1n.pi_decimal(ctx().prec)


# ==========================================================================
# Inputs
# ==========================================================================
def binary64_record(v):
    fr = Fraction(v)
    fl = float(fr)
    assert Fraction(fl) == fr, f"input {v!r} is not a binary64 value"
    return {"decimal": repr(fl) if not float(fl).is_integer() or abs(fl) >= 1e16 else str(int(fl)),
            "hex": fl.hex(), "rational": str(fr)}


def collect_numbers(obj, out):
    if isinstance(obj, bool) or obj is None or isinstance(obj, str):
        return
    if isinstance(obj, (int, float)):
        out.add(Fraction(obj))
        return
    if isinstance(obj, dict):
        for v in obj.values():
            collect_numbers(v, out)
    elif isinstance(obj, (list, tuple)):
        for v in obj:
            collect_numbers(v, out)


def inputs_table(objs):
    nums = set()
    for o in objs:
        collect_numbers(o, nums)
    return [binary64_record(v) for v in sorted(nums)]


def solver_spec(spec):
    """Copy of the authored model with each material's shear modulus derived from E and nu.

    Homogeneous isotropic: G = E / (2 (1 + nu)), exact in rationals. A material
    that states shear_modulus_Pa directly (the f2 replay) is used as stated.
    """
    s = copy.deepcopy(spec)
    derivation = {}
    for name, mat in s["materials"].items():
        if "poisson_ratio" in mat:
            E, nu = fq(mat["elastic_modulus_Pa"]), fq(mat["poisson_ratio"])
            G = E / (2 * (1 + nu))
            assert Fraction(float(G)) == G, "derived G is not a binary64 value"
            assert G == fq(cases.STATED_SHEAR_MODULUS_PA), "derived G differs from the stated binary64 value"
            mat["shear_modulus_Pa"] = int(G) if G.denominator == 1 else float(G)
            derivation[name] = {"elastic_modulus_Pa": str(E), "poisson_ratio": str(nu),
                                "shear_modulus_Pa_derived": str(G),
                                "rule": "G = E / (2 (1 + nu)), exact; equals the stated binary64 value "
                                        f"{cases.STATED_SHEAR_MODULUS_PA}"}
    return s, derivation


# ==========================================================================
# Frames
# ==========================================================================
def exact_frame(xi, xj, yref):
    """Member frame x, y, z as Fractions when every coefficient is rational, else None."""
    d = [b - a for a, b in zip(xi, xj)]
    L = f1m.exact_sqrt(sum(v * v for v in d))
    if L is None:
        return None
    x = [v / L for v in d]
    yr = [Fraction(v) for v in yref]
    p = sum(a * b for a, b in zip(yr, x))
    y = [a - p * b for a, b in zip(yr, x)]
    ny = f1m.exact_sqrt(sum(v * v for v in y))
    if ny is None or ny == 0:
        return None
    y = [v / ny for v in y]
    z = cross3(x, y)
    return [x, y, z]


def decimal_frame(xi, xj, yref):
    d = [f1n.to_decimal(b - a) for a, b in zip(xi, xj)]
    L = vdot(d, d).sqrt()
    x = [v / L for v in d]
    yr = [f1n.to_decimal(Fraction(v)) for v in yref]
    p = vdot(yr, x)
    y = [a - p * b for a, b in zip(yr, x)]
    ny = vdot(y, y).sqrt()
    y = [v / ny for v in y]
    return [x, y, cross3(x, y)]


# ==========================================================================
# The system: formation through f1, own element and constituent assembly
# ==========================================================================
class System:
    def __init__(self, spec, basis, prec):
        f1n.set_point_precision(prec)
        self.prec = prec
        self.authored = spec
        self.spec, self.material_derivation = solver_spec(spec)
        self.basis = basis
        m = self.model = f1m.Model(self.spec)
        for key in ("skew_springs", "snubbers"):
            assert not self.spec.get(key), f"{key} are outside this generator"
        self.exact = all(f1m.exact_sqrt(sum((b - a) ** 2 for a, b in zip(m.xyz[m.idx[mm["i"]]],
                                                                        m.xyz[m.idx[mm["j"]]]))) is not None
                         for mm in self.spec["members"])
        F = self.F = f1n.QPiField() if self.exact else f1n.PointField()
        M_full, contribs, self.categories = f1m.form_mass(m, F, basis)
        K_full = f1m.form_stiffness(m, F, basis)
        pi = F.pi()
        n = m.n
        # ---- own element and constituent assembly
        self.members = []
        Mc = {c: el.zeros(F, n, n) for c in CONSTITUENTS}
        Ksum = el.zeros(F, n, n)
        for mid in m.member_order:
            mem = m.members[mid]
            assert mem["kind"] == "straight", "only straight members in this family"
            dofs = m.dofs_of(mem["i"]) + m.dofs_of(mem["j"])
            sb = m.section(mem, basis)
            mat = self.spec["materials"][mem["material"]]
            e, L, Lexact = m.straight_axis(F, mem)
            E, G = F.q(fq(mat["elastic_modulus_Pa"])), F.q(fq(mat["shear_modulus_Pa"]))
            s = m.scale_of.get(mid)
            if s is not None:
                E, G = E * F.q(s), G * F.q(s)
            Ke = el.straight_stiffness(F, e, L, E * F.q(sb["A"]) * pi, E * F.q(sb["I"]) * pi,
                                       G * F.q(sb["J"]) * pi)
            f1m.add_block(Ksum, dofs, Ke)
            rho_m = fq(mat["density_kg_per_m3"])
            jm = rho_m * sb["J"]
            ji = sb["jbar"] - jm
            replaced = mid in m.body_of
            z = F.q(0)
            parts = {}
            if not replaced:
                parts = {
                    "metal_translational": el.straight_mass(F, e, L, F.q(sb["metal"]) * pi, z, z),
                    "metal_rotary": el.straight_mass(F, e, L, z, F.q(jm / 2) * pi, F.q(jm) * pi),
                    "contents": el.straight_mass(F, e, L, F.q(sb["contents"]) * pi, z, z),
                    "insulation_translational": el.straight_mass(F, e, L, F.q(sb["insulation"]) * pi, z, z),
                    "insulation_rotary": el.straight_mass(F, e, L, z, F.q(ji / 2) * pi, F.q(ji) * pi),
                }
                for c, blk in parts.items():
                    f1m.add_block(Mc[c], dofs, blk)
            self.members.append({"id": mid, "i": mem["i"], "j": mem["j"], "dofs": dofs, "Ke": Ke,
                                 "parts": parts, "replaced_by": m.body_of[mid]["id"] if replaced else None,
                                 "scale": s, "length": L, "section": sb, "y_reference": mem["y_reference"]})
        self.bodies = []
        for c in contribs:
            if c.get("kind") == "component_body":
                f1m.add_block(Mc["component_body"], c["dofs"], c["matrix"])
                comp = next(x for x in self.spec["components"] if x["id"] == c["id"])
                self.bodies.append({"id": c["id"], "node": c["node"], "dofs": c["dofs"], "matrix": c["matrix"],
                                    "mass": c["mass"], "cog": c["cog"], "maps_member": comp["maps_member"]})
        for sp in self.spec.get("springs", []):
            d = 6 * m.idx[sp["node"]] + DOFS.index(sp["dof"])
            Ksum[d][d] = Ksum[d][d] + F.q(fq(sp["stiffness"]))
        # ---- formation identities: own assembly = f1 assembly
        self.formation_identity = self._identity(M_full, Mc, K_full, Ksum)
        # ---- T_k = R_k^T M R_k in the formation field (exact where formation is exact): the rotary
        # constituents give exactly 0, since their M R_k vanishes identically
        def tk(A, k):
            idx = [6 * nd + k for nd in range(len(m.node_ids))]
            acc = F.q(0)
            for a in idx:
                for b in idx:
                    acc = acc + A[a][b]
            return dec_of(acc, prec)
        self.T_formed = [tk(M_full, k) for k in range(3)]
        self.Tc_formed = {c: [tk(Mc[c], k) for k in range(3)] for c in CONSTITUENTS}
        # ---- Decimal at the working precision
        D = lambda A: [[dec_of(v, prec) for v in r] for r in A]  # noqa: E731
        self.K = D(K_full)
        self.M = D(M_full)
        self.Mc = {c: D(A) for c, A in Mc.items()}
        for mb in self.members:
            mb["Ke_d"] = D(mb["Ke"])
            mb["parts_d"] = {c: D(b) for c, b in mb["parts"].items()}
            mb["Me_d"] = ([[sum((mb["parts_d"][c][a][b] for c in mb["parts_d"]), D0) for b in range(12)]
                           for a in range(12)] if mb["parts_d"] else None)
        for bd in self.bodies:
            bd["matrix_d"] = D(bd["matrix"])
        self.xyz = [[f1n.to_decimal(v) for v in p] for p in m.xyz]
        self.free = list(m.free)
        self.restr = sorted(m.restrained)
        self.n = n
        self._quantities()

    def _identity(self, M_full, Mc, K_full, Ksum):
        n = self.model.n
        if self.exact:
            okm = all(sum((Mc[c][a][b] for c in CONSTITUENTS), f1n.QPi(0)) == M_full[a][b]
                      for a in range(n) for b in range(n))
            okk = all(Ksum[a][b] == K_full[a][b] for a in range(n) for b in range(n))
            assert okm and okk, "own assembly differs from f1 assembly"
            return {"mass": "exactly equal in Q + Q*pi, every entry", "stiffness": "exactly equal in Q + Q*pi"}
        dm = max(abs(sum((Mc[c][a][b] for c in CONSTITUENTS), D0) - M_full[a][b]) for a in range(n) for b in range(n))
        dk = max(abs(Ksum[a][b] - K_full[a][b]) for a in range(n) for b in range(n))
        sm = max(abs(v) for r in M_full for v in r)
        sk = max(abs(v) for r in K_full for v in r)
        assert dm <= sm * Decimal(10) ** (-(self.prec - 8)) and dk <= sk * Decimal(10) ** (-(self.prec - 8))
        return {"mass_relative": f1n.fmt(dm / sm, 2), "stiffness_relative": f1n.fmt(dk / sk, 2)}

    # ---------------------------------------------------------------- keys
    def _quantities(self):
        m = self.model
        self.disp_keys = [f"{nid}.{d}" for nid in m.node_ids for d in DOFS]
        self.force_keys = []
        self.axes_keys = []
        for mb in self.members:
            for end in ("i", "j"):
                self.force_keys += [f"{mb['id']}.{end}.{c}" for c in COMP6]
                self.axes_keys += [f"{mb['id']}.{end}.{c}_m" for c in COMP6]
        self.reaction_keys = []   # (key, global dof)
        seen = set()
        for s in self.spec["supports"]:
            for d in DOFS:
                if d in s["restrained"]:
                    g = 6 * m.idx[s["node"]] + DOFS.index(d)
                    assert g not in seen, "a DOF restrained by two supports"
                    seen.add(g)
                    self.reaction_keys.append((f"{s['id']}.{FN[d]}", g))
        self.spring_keys = [(f"{sp['id']}.{FN[sp['dof']]}", 6 * m.idx[sp["node"]] + DOFS.index(sp["dof"]),
                             f1n.to_decimal(fq(sp["stiffness"]))) for sp in self.spec.get("springs", [])]
        # frames
        self.frames = {}
        for mb in self.members:
            mem = m.members[mb["id"]]
            xi, xj = m.xyz[m.idx[mem["i"]]], m.xyz[m.idx[mem["j"]]]
            fr = exact_frame(xi, xj, mem["y_reference"])
            if fr is not None:
                self.frames[mb["id"]] = {"exact": fr, "dec": [[f1n.to_decimal(v) for v in r] for r in fr]}
            else:
                self.frames[mb["id"]] = {"exact": None, "dec": decimal_frame(xi, xj, mem["y_reference"])}

    def node_x(self, gdof):
        return self.xyz[gdof // 6]


def unit_of(key):
    tail = key.split(".")[-1].replace("_m", "")
    if tail in ("ux", "uy", "uz"):
        return "m"
    if tail in ("rx", "ry", "rz", "u_perp1"):
        return "rad"
    if tail.startswith("F"):
        return "N"
    if tail.startswith("M"):
        return "N*m"
    raise KeyError(key)


# ==========================================================================
# Modal basis (DEL-04-09 REQ-005 conventions)
# ==========================================================================
def rigid_R(sys_, k):
    r = [D0] * sys_.n
    for node in range(len(sys_.model.node_ids)):
        r[6 * node + k] = Decimal(1)
    return r


def modal_basis(sys_, n_extract, route_a=False):
    free = sys_.free
    Kff = [[sys_.K[a][b] for b in free] for a in free]
    Mff = [[sys_.M[a][b] for b in free] for a in free]
    pairs, sweeps = eg.route_b(Kff, Mff)
    nf = len(free)
    pairs = sorted(pairs, key=lambda t: t[0])
    lam_all = []
    vecs = []
    for lam0, phi in pairs:
        lam = vdot(phi, mv(Kff, phi)) / vdot(phi, mv(Mff, phi))   # Rayleigh quotient
        lam_all.append(lam)
        vecs.append(phi)
    assert len(lam_all) == nf, "M_ff is singular in this family"
    two_pi = 2 * pi_dec()
    bbox = sys_.model.bbox_diagonal_sq()
    ell = f1n.to_decimal(bbox).sqrt()
    # scope clusters (DEL-04-09 REQ-002) over every computed mode
    dc = f1n.to_decimal(DELTA_C)
    clusters = [[0]]
    for i in range(1, nf):
        if lam_all[i] - lam_all[i - 1] <= dc * lam_all[i]:
            clusters[-1].append(i)
        else:
            clusters.append([i])
    for c in clusters:
        if c[0] <= n_extract:
            assert len(c) == 1, "extracted or next cluster has more than one mode"
    gaps = [(lam_all[i + 1] - lam_all[i]) / lam_all[i + 1] for i in range(min(n_extract, nf - 1))]
    assert min(gaps) > 2 * dc, "an extracted gap is near the cluster tolerance"
    R = [rigid_R(sys_, k) for k in range(3)]
    MR = [mv(sys_.M, R[k]) for k in range(3)]
    p = [[MR[k][a] for a in free] for k in range(3)]
    T = list(sys_.T_formed)
    Tc = sys_.Tc_formed
    assert all(abs(T[k] - vdot(R[k], MR[k])) < T[k] * Decimal("1e-70") for k in range(3))
    modes = []
    full_all = []
    for i in range(nf):
        phi = vecs[i]
        full = [D0] * sys_.n
        for a, g in enumerate(free):
            full[g] = phi[a]
        trans = [abs(full[g]) for g in range(sys_.n) if g % 6 < 3]
        rots = [abs(full[g]) for g in range(sys_.n) if g % 6 >= 3]
        t, th = max(trans), max(rots)
        case_a = t > TAU_T * ell * th
        pool = [g for g in range(sys_.n) if (g % 6 < 3) == case_a]
        big = max(abs(full[g]) for g in pool)
        ties = [g for g in pool if abs(full[g]) >= big * (1 - TIE_REL)]
        g0 = ties[0]
        sc = 1 / full[g0]
        full = [v * sc for v in full]
        phi = [full[g] for g in free]
        mi = vdot(phi, mv(Mff, phi))
        lam = lam_all[i]
        om = lam.sqrt()
        rec = {"index": i + 1, "lambda": lam, "omega": om, "f": om / two_pi, "phi": full, "phi_f": phi, "m": mi,
               "case": "unit_max_translation" if case_a else "unit_max_rotation", "norm_dof": g0, "ties": ties}
        rec["L"] = [vdot(phi, p[k]) for k in range(3)]
        rec["Gamma"] = [rec["L"][k] / mi for k in range(3)]
        rec["meff"] = [rec["L"][k] ** 2 / mi for k in range(3)]
        full_all.append(rec)
        if i < n_extract + 1:
            modes.append(rec)
    # checks
    Rch = eg.cholesky(Mff)
    xs = [cholesky_solve(Rch, p[k]) for k in range(3)]
    pMp = [vdot(p[k], xs[k]) for k in range(3)]
    sum_meff = [sum((r["meff"][k] for r in full_all), D0) for k in range(3)]
    resid = D0
    knorm = max(sum(abs(v) for v in row) for row in Kff)
    for r in full_all:
        kr = mv(Kff, r["phi_f"])
        mr = mv(Mff, r["phi_f"])
        res = max(abs(a - r["lambda"] * b) for a, b in zip(kr, mr))
        resid = max(resid, res / (knorm * max(abs(v) for v in r["phi_f"])))
    orth = D0
    for a in range(nf):
        Ma = mv(Mff, full_all[a]["phi_f"])
        for b in range(a + 1, nf):
            orth = max(orth, abs(vdot(full_all[b]["phi_f"], Ma)) / (full_all[a]["m"] * full_all[b]["m"]).sqrt())
    # completeness of the excitation: p_k = sum over every mode of Gamma M phi
    comp = D0
    for k in range(3):
        acc = [D0] * nf
        for r in full_all:
            mphi = mv(Mff, r["phi_f"])
            acc = [x + r["Gamma"][k] * y for x, y in zip(acc, mphi)]
        sc = max(abs(v) for v in p[k]) or Decimal(1)
        comp = max(comp, max(abs(x - y) for x, y in zip(acc, p[k])) / sc)
    # Sturm counts (f1 eigen.sturm_count) at the geometric mean of each gap and at lambda_1/2
    PF = f1n.PointField()
    sturm = []
    shifts = [(lam_all[0] / 2, 0)] + [((lam_all[i] * lam_all[i + 1]).sqrt(), i + 1)
                                      for i in range(min(n_extract + 1, nf - 1))]
    for s, expect in shifts:
        c, _ = eg.sturm_count(PF, Kff, Mff, s)
        assert c == expect, f"Sturm count {c} != {expect}"
        sturm.append({"shift": s, "count": c})
    route_a_dev = None
    if route_a:
        bis = eg.Bisection(PF, Kff, Mff)
        route_a_dev = max(abs(bis.eigenvalue(k + 1) - lam_all[k]) / lam_all[k] for k in range(n_extract + 1))
        assert route_a_dev < Decimal("1e-40")
    checks = {
        "residual_scaled_max": resid, "m_orthogonality_max": orth, "excitation_completeness_rel": comp,
        "sum_meff_vs_pMinvp_rel": max(abs(sum_meff[k] - pMp[k]) / T[k] for k in range(3)),
        "jacobi_sweeps": sweeps, "route_a_bisection_rel": route_a_dev,
    }
    for key in ("residual_scaled_max", "m_orthogonality_max", "excitation_completeness_rel", "sum_meff_vs_pMinvp_rel"):
        assert checks[key] < Decimal("1e-60"), (key, checks[key])
    return {"modes": modes, "all": full_all, "clusters": clusters, "p": p, "T": T, "Tc": Tc, "pMp": pMp,
            "R": R, "MR": MR, "sturm": sturm, "checks": checks, "ell": ell, "gaps": gaps, "Mff": Mff, "Kff": Kff}


# ==========================================================================
# Spectra
# ==========================================================================
def ordinate(sp, f):
    pts = [(f1n.to_decimal(fq(a)), f1n.to_decimal(fq(b))) for a, b in sp["points"]]
    x = f if sp["abscissa"] == "frequency_Hz" else 1 / f
    for a, _ in pts:
        assert abs(x - a) > a * Decimal("1e-9"), "query too close to a knot"
    if x < pts[0][0] or x > pts[-1][0]:
        return None
    for (x1, s1), (x2, s2) in zip(pts, pts[1:]):
        if x1 <= x <= x2:
            if sp["interpolation"]["abscissa"] == "linear":
                t = (x - x1) / (x2 - x1)
            else:
                t = (x.ln() - x1.ln()) / (x2.ln() - x1.ln())
            if sp["interpolation"]["ordinate"] == "linear":
                return s1 + t * (s2 - s1)
            return (s1.ln() + t * (s2.ln() - s1.ln())).exp()
    raise AssertionError


# ==========================================================================
# Recovery
# ==========================================================================
def resultant(sys_, q):
    """Force and moment about the global origin of a full-DOF generalized load vector."""
    F = [D0, D0, D0]
    Mo = [D0, D0, D0]
    for node in range(len(sys_.model.node_ids)):
        b = 6 * node
        f = q[b:b + 3]
        mm = q[b + 3:b + 6]
        x = sys_.xyz[node]
        rxf = cross3(x, f)
        for a in range(3):
            F[a] += f[a]
            Mo[a] += mm[a] + rxf[a]
    return F + Mo


def scatter(sys_, dofs, v):
    out = [D0] * sys_.n
    for d, x in zip(dofs, v):
        out[d] += x
    return out


def recover(sys_, u, g):
    """Signed recovery of displacement u under the inertia field g (load per unit mass, M g).

    End forces on each element f_e = K_e u_e - M_e g_e; reactions at rigid
    restraints r = (K u - M g) at the restrained DOFs; compliant supports -k u.
    """
    vals = {}
    for key, v in zip(sys_.disp_keys, u):
        vals[key] = v
    ends = {}
    for mb in sys_.members:
        ue = [u[d] for d in mb["dofs"]]
        ge = [g[d] for d in mb["dofs"]]
        f = mv(mb["Ke_d"], ue)
        if mb["Me_d"] is not None:
            f = [a - b for a, b in zip(f, mv(mb["Me_d"], ge))]
        ends[mb["id"]] = f
        for k, key in enumerate(sys_.force_keys[12 * sys_.members.index(mb): 12 * sys_.members.index(mb) + 12]):
            vals[key] = f[k]
    Mg = mv(sys_.M, g)
    r_full = [a - b for a, b in zip(mv(sys_.K, u), Mg)]
    for key, gd in sys_.reaction_keys:
        vals[key] = r_full[gd]
    for key, gd, k in sys_.spring_keys:
        vals[key] = -k * u[gd]
    axes = {}
    for mb in sys_.members:
        Lam = sys_.frames[mb["id"]]["dec"]
        f = ends[mb["id"]]
        rot = []
        for blk in range(4):
            v = f[3 * blk:3 * blk + 3]
            rot += [vdot(Lam[r], v) for r in range(3)]
        for end in range(2):
            for c in range(6):
                axes[f"{mb['id']}.{'ij'[end]}.{COMP6[c]}_m"] = rot[6 * end + c]
    return vals, axes, ends, r_full, Mg


def shares(sys_, g):
    """Inertia load M g split by constituent, by member and by body; resultants about the origin."""
    out = {"constituents": {}, "members": {}, "bodies": {}}
    for c in CONSTITUENTS:
        out["constituents"][c] = mv(sys_.Mc[c], g)
    for mb in sys_.members:
        if mb["Me_d"] is None:
            out["members"][mb["id"]] = None
            continue
        ge = [g[d] for d in mb["dofs"]]
        out["members"][mb["id"]] = scatter(sys_, mb["dofs"], mv(mb["Me_d"], ge))
    for bd in sys_.bodies:
        gb = [g[d] for d in bd["dofs"]]
        out["bodies"][bd["id"]] = scatter(sys_, bd["dofs"], mv(bd["matrix_d"], gb))
    return out


def balances(sys_, u, g, ends, r_full, Mg, vals):
    """Element, nodal, branch-node, body-node and global balance residuals (relative)."""
    m = sys_.model
    scale = max(abs(v) for v in Mg) or Decimal(1)
    # element: resultant of f_e equals minus the resultant of its own inertia share
    el_res = D0
    for mb in sys_.members:
        fe = scatter(sys_, mb["dofs"], ends[mb["id"]])
        rf = resultant(sys_, fe)
        if mb["Me_d"] is not None:
            ge = [g[d] for d in mb["dofs"]]
            ri = resultant(sys_, scatter(sys_, mb["dofs"], mv(mb["Me_d"], ge)))
        else:
            ri = [D0] * 6
        el_res = max(el_res, max(abs(a + b) for a, b in zip(rf, ri)))
    # nodal: sum of element end forces at a node minus the body inertia there equals the support force there
    body_q = [D0] * sys_.n
    for bd in sys_.bodies:
        gb = [g[d] for d in bd["dofs"]]
        for d, v in zip(bd["dofs"], mv(bd["matrix_d"], gb)):
            body_q[d] += v
    sum_e = [D0] * sys_.n
    for mb in sys_.members:
        for d, v in zip(mb["dofs"], ends[mb["id"]]):
            sum_e[d] += v
    # at every DOF: sum_e f_e - (body inertia) + (spring term k u) = support force (0 at a free DOF)
    support = [D0] * sys_.n
    for _, gd in sys_.reaction_keys:
        support[gd] = r_full[gd]
    ksu = [D0] * sys_.n
    for _, gd, k in sys_.spring_keys:
        ksu[gd] += k * u[gd]
    nodal = max(abs(sum_e[d] - body_q[d] + ksu[d] - support[d]) for d in range(sys_.n))
    free_resid = max(abs(r_full[d]) for d in sys_.free)
    # global: reactions + spring forces = - resultant of the inertia load
    ext = [D0] * sys_.n
    for _, gd in sys_.reaction_keys:
        ext[gd] += r_full[gd]
    for _, gd, k in sys_.spring_keys:
        ext[gd] += -k * u[gd]
    rext = resultant(sys_, ext)
    rin = resultant(sys_, Mg)
    glob = max(abs(a + b) for a, b in zip(rext, rin))
    lscale = scale * (f1n.to_decimal(m.bbox_diagonal_sq()).sqrt() + 1)
    return {"element_resultant": el_res / lscale, "nodal": nodal / scale, "free_dof_residual": free_resid / scale,
            "global": glob / lscale}, sum_e, body_q


# ==========================================================================
# Combinations (DEL-04-10 CLM-001)
# ==========================================================================
def rho_cqc(wi, wj, zeta):
    r = wj / wi
    num = 8 * zeta * zeta * (1 + r) * r * r.sqrt()
    den = (1 - r * r) ** 2 + 4 * zeta * zeta * r * (1 + r) ** 2
    return num / den


def groups_of(freqs):
    out = []
    for i, f in enumerate(freqs):
        if out and (f - freqs[out[-1][0]]) / freqs[out[-1][0]] <= Decimal("0.1"):
            out[-1].append(i)
        else:
            out.append([i])
    return out


def ten_pairs(freqs):
    return [(i, j) for i in range(len(freqs)) for j in range(i + 1, len(freqs))
            if (freqs[j] - freqs[i]) / freqs[i] <= Decimal("0.1")]


def modal_combine(rule, vals, freqs, omegas, zeta):
    if not vals:
        return D0
    if rule == "SRSS":
        return sum((v * v for v in vals), D0).sqrt()
    if rule == "ABSOLUTE_SUM":
        return sum((abs(v) for v in vals), D0)
    if rule == "CQC":
        s = D0
        for i, a in enumerate(vals):
            for j, b in enumerate(vals):
                s += (Decimal(1) if i == j else rho_cqc(omegas[i], omegas[j], zeta)) * a * b
        return max(s, D0).sqrt()
    if rule == "GROUPING":
        s = sum((v * v for v in vals), D0)
        for grp in groups_of(freqs):
            for x in range(len(grp)):
                for y in range(x + 1, len(grp)):
                    s += 2 * abs(vals[grp[x]] * vals[grp[y]])
        return s.sqrt()
    if rule == "TEN_PERCENT":
        s = sum((v * v for v in vals), D0)
        for i, j in ten_pairs(freqs):
            s += 2 * abs(vals[i] * vals[j])
        return s.sqrt()
    raise ValueError(rule)


def rigid_combine(rule, rp, rr):
    if rule == "SRSS":
        return (rp * rp + rr * rr).sqrt()
    if rule == "ABSOLUTE_SUM":
        return rp + abs(rr)
    raise ValueError(rule)


def directional_combine(rule, vals):
    if rule == "SRSS":
        return sum((v * v for v in vals), D0).sqrt()
    if rule == "ABSOLUTE_SUM":
        return sum(vals, D0)
    if rule == "100-40-40":
        f = f1n.to_decimal(Fraction(2, 5))
        return max(vals[a] + f * sum((vals[b] for b in range(len(vals)) if b != a), D0) for a in range(len(vals)))
    raise ValueError(rule)


# ==========================================================================
# Formatting with the flush rule
# ==========================================================================
class Flusher:
    """Values below 1e-50 of the largest magnitude of their unit in a block print as 0; values between
    1e-50 and 1e-35 of it are refused (they would be ambiguous)."""

    def __init__(self, unit_fn):
        self.unit_fn = unit_fn
        self.scale = {}

    def see(self, key, v):
        u = self.unit_fn(key)
        self.scale[u] = max(self.scale.get(u, D0), abs(v))

    def see_all(self, d):
        for k, v in d.items():
            self.see(k, v)

    def pair(self, key, v):
        sc = self.scale.get(self.unit_fn(key), D0)
        a = abs(v)
        if sc == 0 or a < FLUSH * sc:
            return ["0", 0.0]
        if a < GUARD * sc:
            raise AssertionError(f"value {key} = {v} lies between 1e-50 and 1e-35 of its unit scale")
        return [f1n.fmt(v, DIGITS), float(v)]

    def block(self, d):
        return {k: self.pair(k, v) for k, v in d.items()}


def pair(v):
    if v == 0:
        return ["0", 0.0]
    return [f1n.fmt(v, DIGITS), float(v)]


def small(v):
    return f1n.fmt(v, 2) if v != 0 else "0"


def vec6(v):
    return {c: x for c, x in zip(COMP6, v)}


def tree_unit(key):
    if key in COMP6:
        return "N" if key.startswith("F") else "N*m"
    return unit_of(key)


def flush_tree(tree):
    """Format a tree of raw Decimals with the flush rule, one scale per unit over the whole tree."""
    fl = Flusher(tree_unit)

    def see(t):
        if isinstance(t, dict):
            for k, v in t.items():
                if isinstance(v, Decimal):
                    fl.see(k, v)
                else:
                    see(v)
        elif isinstance(t, list):
            for v in t:
                see(v)

    def put(t):
        if isinstance(t, dict):
            return {k: (fl.pair(k, v) if isinstance(v, Decimal) else put(v)) for k, v in t.items()}
        if isinstance(t, list):
            return [put(v) for v in t]
        return t

    see(tree)
    return put(tree)


# ==========================================================================
# Analysis
# ==========================================================================
def analyse(sys_, mb_, request, spectra, grid=False, detail=True):
    """One spectrum analysis. detail adds the per-mode inertia shares, node balances and per-mode member-axis
    contributions (published for W1-BASE; the variants publish the rigid shares and combined member-axis
    values only)."""
    zeta = f1n.to_decimal(fq(spectra[0]["damping_ratio"]))
    for sp in spectra:
        assert sp["damping_ratio"] == spectra[0]["damping_ratio"]
    sp_by = {s["id"]: s for s in spectra}
    n_ext = request["extracted_modes"]
    modes = mb_["modes"][:n_ext]
    mm = request["missing_mass"]
    fr = f1n.to_decimal(fq(mm["rigid_frequency_Hz"])) if mm["applied"] else None
    if mm["applied"]:
        assert modes[-1]["f"] >= fr, "extraction premise: the extracted set does not reach f_r"
    units = [[i] for i in range(n_ext)]       # every extracted scope cluster is a single mode (asserted)
    periodic = [u for u in units if fr is None or modes[u[0]]["f"] < fr]
    dirs = request["directions"]
    kidx = {"X": 0, "Y": 1, "Z": 2}
    out = {"request": request, "spectral_ordinates": {}, "mode_status": [], "contributions_signed": {},
           "contributions_signed_member_axes": {}}
    for i, md in enumerate(modes):
        out["mode_status"].append({"mode": md["index"], "f_Hz": pair(md["f"]),
                                   "status": "periodic" if [i] in periodic else "rigid (extracted)"})
    contrib = {}
    contrib_ax = {}
    inertia = {}
    bal_modes = []
    for dd in dirs:
        k = kidx[dd["direction"]]
        sp = sp_by[dd["spectrum"]]
        s = f1n.to_decimal(fq(dd["scale_factor"]))
        ords = []
        contrib[dd["direction"]] = []
        contrib_ax[dd["direction"]] = []
        inertia[dd["direction"]] = []
        for i, md in enumerate(modes):
            S = ordinate(sp, md["f"])
            if S is None:
                # outside the table: allowed only for a rigid extracted mode, which then has no contribution
                assert [i] not in periodic, "a periodic mode lies outside its spectrum's range"
                ords.append({"mode": md["index"], "S": None, "sS": None})
                contrib[dd["direction"]].append(None)
                contrib_ax[dd["direction"]].append(None)
                if detail:
                    inertia[dd["direction"]].append(None)
                continue
            ords.append({"mode": md["index"], "S": pair(S), "sS": pair(s * S)})
            amp = md["Gamma"][k] * s * S / md["lambda"]
            u = [amp * v for v in md["phi"]]
            g = [md["lambda"] * v for v in u]
            vals, axes, ends, r_full, Mg = recover(sys_, u, g)
            b, sum_e, body_q = balances(sys_, u, g, ends, r_full, Mg, vals)
            bal_modes.append(b)
            contrib[dd["direction"]].append(vals)
            contrib_ax[dd["direction"]].append(axes)
            if detail:
                inertia[dd["direction"]].append(share_record(sys_, g, Mg, sum_e, body_q, r_full, ends,
                                                               full_vectors=False))
        out["spectral_ordinates"][dd["direction"]] = ords
    # ---- rigid response
    rigid = {}
    rigid_ax = {}
    rigid_detail = {}
    bal_rigid = []
    rigid_checks = {}
    if mm["applied"]:
        out["rigid_frequency_Hz"] = pair(fr)
        out["rigid_acceleration_m_per_s2"] = {}
        free = sys_.free
        Rk = eg.cholesky(mb_["Kff"])
        for dd in dirs:
            k = kidx[dd["direction"]]
            s = f1n.to_decimal(fq(dd["scale_factor"]))
            ar = s * ordinate(sp_by[dd["spectrum"]], fr)
            out["rigid_acceleration_m_per_s2"][dd["direction"]] = pair(ar)
            g = list(mb_["R"][k])
            for u_ in periodic:
                md = modes[u_[0]]
                g = [a - md["Gamma"][k] * b for a, b in zip(g, md["phi"])]
            g = [ar * v for v in g]
            Fk = mv(sys_.M, g)
            uf = cholesky_solve(Rk, [Fk[a] for a in free])
            u = [D0] * sys_.n
            for a, gd in enumerate(free):
                u[gd] = uf[a]
            vals, axes, ends, r_full, Mg = recover(sys_, u, g)
            b, sum_e, body_q = balances(sys_, u, g, ends, r_full, Mg, vals)
            bal_rigid.append(b)
            rigid[dd["direction"]] = vals
            rigid_ax[dd["direction"]] = axes
            rigid_detail[dd["direction"]] = share_record(sys_, g, Mg, sum_e, body_q, r_full, ends,
                                                       full_vectors=True)
            # identity: the static solve equals the expansion over every non-periodic finite mode
            pset = {modes[u_[0]]["index"] for u_ in periodic}
            exp_ = [D0] * sys_.n
            for md in mb_["all"]:
                if md["index"] in pset:
                    continue
                c = ar * md["Gamma"][k] / md["lambda"]
                exp_ = [x + c * y for x, y in zip(exp_, md["phi"])]
            usc = max(abs(v) for v in u)
            dev = max(abs(a - b) for a, b in zip(u, exp_)) / usc
            # identity: total reaction along k = -a_r (T_k - sum_periodic m_eff)
            tot = sum((r_full[gd] for _, gd in sys_.reaction_keys if gd % 6 == k), D0)
            tot += sum((-kk * u[gd] for _, gd, kk in sys_.spring_keys if gd % 6 == k), D0)
            want = -ar * (mb_["T"][k] - sum((modes[u_[0]]["meff"][k] for u_ in periodic), D0))
            rigid_checks[dd["direction"]] = {"static_vs_modal_expansion_rel": dev,
                                             "total_reaction_vs_closed_form_rel": abs(tot - want) / abs(want)}
            assert dev < Decimal("1e-55") and abs(tot - want) < abs(want) * Decimal("1e-60")
    # ---- flushing scales
    fl_g = Flusher(unit_of)
    fl_a = Flusher(unit_of)
    for d in contrib:
        for v in contrib[d]:
            if v is not None:
                fl_g.see_all(v)
        for v in contrib_ax[d]:
            if v is not None:
                fl_a.see_all(v)
    for d in rigid:
        fl_g.see_all(rigid[d])
        fl_a.see_all(rigid_ax[d])
    for d in contrib:
        out["contributions_signed"][d] = [{"mode": modes[i]["index"], "status": out["mode_status"][i]["status"],
                                           "values": fl_g.block(contrib[d][i])}
                                          for i in range(n_ext) if contrib[d][i] is not None]
        if detail:
            out["contributions_signed_member_axes"][d] = [{"mode": modes[i]["index"],
                                                           "values": fl_a.block(contrib_ax[d][i])}
                                                          for i in range(n_ext) if contrib_ax[d][i] is not None]
    if not detail:
        out.pop("contributions_signed_member_axes")
    if mm["applied"]:
        out["rigid_response_signed"] = {d: fl_g.block(rigid[d]) for d in rigid}
        out["rigid_response_signed_member_axes"] = {d: fl_a.block(rigid_ax[d]) for d in rigid_ax}
        out["rigid_load_and_shares"] = rigid_detail
        out["rigid_response_identities"] = {d: {k2: small(v) for k2, v in x.items()} for d, x in rigid_checks.items()}
    if detail:
        out["modal_inertia_shares"] = {d: [{"mode": modes[i]["index"], **inertia[d][i]} for i in range(n_ext)
                                           if inertia[d][i] is not None] for d in inertia}
    # ---- combinations
    freqs_u = [modes[u_[0]]["f"] for u_ in units]
    oms_u = [modes[u_[0]]["omega"] for u_ in units]
    per_idx = [units.index(u_) for u_ in periodic]
    out["cqc_correlation"] = [[pair(Decimal(1) if i == j else rho_cqc(oms_u[i], oms_u[j], zeta))
                               for j in range(len(units))] for i in range(len(units))]
    out["grouping_groups"] = {
        "periodic": [[modes[per_idx[x]]["index"] for x in grp] for grp in groups_of([freqs_u[i] for i in per_idx])],
        "correction_off": [[modes[x]["index"] for x in grp] for grp in groups_of(freqs_u)]}
    out["ten_percent_pairs"] = {
        "periodic": [[modes[per_idx[a]]["index"], modes[per_idx[b]]["index"]]
                     for a, b in ten_pairs([freqs_u[i] for i in per_idx])],
        "correction_off": [[modes[a]["index"], modes[b]["index"]] for a, b in ten_pairs(freqs_u)]}
    for i in range(len(freqs_u)):
        for j in range(i + 1, len(freqs_u)):
            assert abs((freqs_u[j] - freqs_u[i]) / freqs_u[i] - Decimal("0.1")) > Decimal("1e-6")

    def combos(cset, rset, keys, modal_rules, rigid_rules, dir_rules):
        res = {}
        for mr in modal_rules:
            for rr in rigid_rules:
                per_dir = {}
                for dd in dirs:
                    d = dd["direction"]
                    row = {}
                    for key in keys:
                        if rr == "OFF" or not mm["applied"]:
                            assert all(cset[d][i] is not None for i in range(n_ext)), "OFF needs every ordinate"
                            vals = [cset[d][i][key] for i in range(n_ext)]
                            row[key] = modal_combine(mr, vals, freqs_u, oms_u, zeta)
                        else:
                            vals = [cset[d][i][key] for i in per_idx]
                            rp = modal_combine(mr, vals, [freqs_u[i] for i in per_idx],
                                               [oms_u[i] for i in per_idx], zeta)
                            row[key] = rigid_combine(rr, rp, rset[d][key])
                    per_dir[d] = row
                for dr in dir_rules:
                    res[(mr, rr, dr)] = (per_dir, {key: directional_combine(dr, [per_dir[dd["direction"]][key]
                                                                               for dd in dirs]) for key in keys})
        return res

    base_mr = request["modal_combination"]
    base_rr = mm["rigid_periodic_combination"] if mm["applied"] else "OFF"
    base_dr = request["directional_combination"]
    keys_g = list(contrib[dirs[0]["direction"]][per_idx[0]].keys())
    keys_a = list(contrib_ax[dirs[0]["direction"]][per_idx[0]].keys())
    if grid:
        mrs, rrs, drs = cases.MODAL_RULES, cases.RIGID_RULES, cases.DIRECTIONAL_RULES
    else:
        mrs, rrs, drs = [base_mr], [base_rr], [base_dr]
    cg = combos(contrib, rigid, keys_g, mrs, rrs, drs)
    ca = combos(contrib_ax, rigid_ax, keys_a, mrs, rrs, drs)
    pdir, comb = cg[(base_mr, base_rr, base_dr)]
    pdir_a, comb_a = ca[(base_mr, base_rr, base_dr)]
    out["per_direction_combined"] = {d: fl_g.block(pdir[d]) for d in pdir}
    out["combined"] = fl_g.block(comb)
    out["per_direction_combined_member_axes"] = {d: fl_a.block(pdir_a[d]) for d in pdir_a}
    out["combined_member_axes"] = fl_a.block(comb_a)
    if grid:
        out["method_grid"] = [{"modal_combination": a, "missing_mass": b, "directional_combination": c,
                               "combined": fl_g.block(cg[(a, b, c)][1]),
                               "combined_member_axes": fl_a.block(ca[(a, b, c)][1])}
                              for (a, b, c) in cg]
    # ---- missing-mass disclosure
    disc = {}
    for dd in dirs:
        k = kidx[dd["direction"]]
        T = mb_["T"][k]
        per = sum((modes[i]["meff"][k] for i in per_idx), D0) / T
        ext = sum((md["meff"][k] for md in modes), D0) / T
        allf = sum((md["meff"][k] for md in mb_["all"]), D0) / T
        moving = 1 - mb_["pMp"][k] / T
        disc[dd["direction"]] = {"T_k_kg": pair(T), "periodic_cumulative_fraction": pair(per),
                                 "not_represented_by_periodic_modes": pair(1 - per),
                                 "extracted_cumulative_fraction": pair(ext),
                                 "not_represented_by_extracted_modes": pair(1 - ext),
                                 "all_finite_modes_fraction": pair(allf),
                                 "moving_with_supports_fraction": pair(moving)}
        assert abs(allf + moving - 1) < Decimal("1e-60")
    out["missing_mass_disclosure"] = disc
    worst = {}
    for b in bal_modes + bal_rigid:
        for k2, v in b.items():
            worst[k2] = max(worst.get(k2, D0), v)
    for k2, v in worst.items():
        assert v < Decimal("1e-60"), (k2, v)
    out["balance_residuals_max"] = {k2: small(v) for k2, v in worst.items()}
    out["_raw"] = {"contrib": contrib, "rigid": rigid, "comb": comb, "comb_a": comb_a, "contrib_ax": contrib_ax,
                   "rigid_ax": rigid_ax}
    return out


def node_vec(sys_, q, node_id):
    b = 6 * sys_.model.idx[node_id]
    return q[b:b + 6]


def share_record(sys_, g, Mg, sum_e, body_q, r_full, ends, full_vectors):
    """Inertia-load shares and the branch-node and body-node balances of one signed response."""
    sh = shares(sys_, g)
    restr = set(sys_.restr)
    rec = {"resultant_about_origin": {"total": vec6(resultant(sys_, Mg))}}
    rec["resultant_about_origin"]["constituents"] = {c: vec6(resultant(sys_, v)) for c, v in sh["constituents"].items()}
    rec["resultant_about_origin"]["members"] = {m: (vec6(resultant(sys_, v)) if v is not None else None)
                                                for m, v in sh["members"].items()}
    rec["resultant_about_origin"]["bodies"] = {b: vec6(resultant(sys_, v)) for b, v in sh["bodies"].items()}
    fr = [v if d not in restr else D0 for d, v in enumerate(Mg)]
    rs = [v if d in restr else D0 for d, v in enumerate(Mg)]
    rec["resultant_about_origin"]["free_dof_part"] = vec6(resultant(sys_, fr))
    rec["resultant_about_origin"]["restrained_dof_part"] = vec6(resultant(sys_, rs))
    rec["restrained_dof_load"] = {}
    for c, v in list(sh["constituents"].items()) + [("total", Mg)]:
        rec["restrained_dof_load"][c] = {f"{sys_.model.node_ids[d // 6]}.{FN[DOFS[d % 6]]}": v[d]
                                         for d in sys_.restr}
    # branch junction N2 and body nodes: member end forces there, their sum, the body's inertia there
    nodes = ["N2"] + [bd["node"] for bd in sys_.bodies]
    nb = {}
    for nid in nodes:
        if nid not in sys_.model.idx:
            continue
        b = 6 * sys_.model.idx[nid]
        ent = {"member_end_forces": {}}
        for mb in sys_.members:
            for end, key in ((0, "i"), (1, "j")):
                if mb[key] == nid:
                    f = ends[mb["id"]][6 * end:6 * end + 6]
                    ent["member_end_forces"][f"{mb['id']}.{key}"] = vec6(f)
        ent["sum_of_member_end_forces"] = vec6(sum_e[b:b + 6])
        ent["body_inertia_at_node"] = vec6(body_q[b:b + 6])
        ent["support_force_at_node"] = vec6([r_full[d] if d in restr else D0 for d in range(b, b + 6)])
        nb[nid] = ent
    rec["node_balances"] = nb
    if full_vectors:
        rec["full_dof_load"] = {}
        for c, v in list(sh["constituents"].items()) + [("total", Mg)]:
            rec["full_dof_load"][c] = {f"{sys_.model.node_ids[d // 6]}.{FN[DOFS[d % 6]]}": v[d]
                                       for d in range(sys_.n)}
        rec["full_dof_load_members"] = {
            m: ({f"{sys_.model.node_ids[d // 6]}.{FN[DOFS[d % 6]]}": v[d]
                 for mb in sys_.members if mb["id"] == m for d in mb["dofs"]} if v is not None else None)
            for m, v in sh["members"].items()}
        rec["full_dof_load_bodies"] = {
            b_: {f"{sys_.model.node_ids[d // 6]}.{FN[DOFS[d % 6]]}": v[d]
                 for bd in sys_.bodies if bd["id"] == b_ for d in bd["dofs"]} for b_, v in sh["bodies"].items()}
    return flush_tree(rec)


# ==========================================================================
# Publication of one variant
# ==========================================================================
def publish_model_block(sys_, mb_, n_ext):
    m = sys_.model
    derived = {"material_derivation": sys_.material_derivation, "basis": sys_.basis,
               "formation_field": "exact Q + Q*pi (every member's unit axis is rational)" if sys_.exact
               else f"{sys_.prec}-digit Decimal",
               "formation_identity": sys_.formation_identity}
    secs = {}
    for name, sec in sys_.spec["sections"].items():
        mat = next(iter(sys_.spec["materials"].values()))
        sb = f1m.section_basis(sec, mat, sys_.basis)
        rho = fq(mat["density_kg_per_m3"])
        jm = rho * sb["J"]
        secs[name] = {k: {"exact": f"({sb[k]})*pi", "value": pair(f1n.QPi(0, sb[k]).to_decimal(sys_.prec))}
                      for k in ("metal", "contents", "insulation", "mbar", "jbar", "A", "I", "J")}
        secs[name]["jbar_metal"] = {"exact": f"({jm})*pi", "value": pair(f1n.QPi(0, jm).to_decimal(sys_.prec))}
        secs[name]["jbar_insulation"] = {"exact": f"({sb['jbar'] - jm})*pi",
                                         "value": pair(f1n.QPi(0, sb["jbar"] - jm).to_decimal(sys_.prec))}
        secs[name]["method"] = sb["method"]
        secs[name]["inner_diameter_metal_m"] = str(sb["inner_diameter_metal"])
        secs[name]["inner_diameter_bore_m"] = str(sb["inner_diameter_bore"])
    derived["section_bases"] = secs
    derived["members"] = []
    for mb in sys_.members:
        fr = sys_.frames[mb["id"]]
        L = mb["length"]
        ent = {"id": mb["id"], "i": mb["i"], "j": mb["j"],
               "length_m": str(L.a) if isinstance(L, f1n.QPi) else pair(L),
               "mass_replaced_by_body": mb["replaced_by"],
               "stiffness_scale": str(mb["scale"]) if mb["scale"] is not None else None,
               "y_reference": mb["y_reference"],
               "frame_rows_xyz_decimal": [[f1n.fmt(v, DIGITS) if v != 0 else "0" for v in r] for r in fr["dec"]],
               "frame_rows_xyz_nearest_binary64": [[float(v) for v in r] for r in fr["dec"]]}
        if fr["exact"] is not None:
            ent["frame_rows_xyz_exact"] = [[str(v) for v in r] for r in fr["exact"]]
        derived["members"].append(ent)
    derived["bodies"] = [{"id": bd["id"], "node": bd["node"], "maps_member": bd["maps_member"],
                          "mass_kg_exact": str(bd["mass"].a), "mass_kg": pair(dec_of(bd["mass"], sys_.prec)),
                          "cog_offset_m": [str(v.a) for v in bd["cog"]]} for bd in sys_.bodies]
    derived["component_status"] = m.component_status
    cats = {k: pair(dec_of(v, sys_.prec)) for k, v in sys_.categories.items()}
    derived["mass_by_category_kg"] = cats
    derived["free_dofs"] = [f"{m.node_ids[d // 6]}.{DOFS[d % 6]}" for d in sys_.free]
    derived["restrained_dofs"] = [f"{m.node_ids[d // 6]}.{DOFS[d % 6]}" for d in sys_.restr]
    derived["quantities"] = {
        "displacements": {"keys": "<node>.<dof> for every node, zero at restrained DOFs", "units": "m, rad"},
        "element_end_forces_global": sys_.force_keys,
        "element_end_forces_member_axes": sys_.axes_keys,
        "reactions": [k for k, _ in sys_.reaction_keys] + [k for k, _, _ in sys_.spring_keys]}
    basis = {"finite_mode_count": len(mb_["all"]),
             "all_frequencies_Hz": [pair(md["f"]) for md in mb_["all"]],
             "scope_clusters": [[mb_["all"][i]["index"] for i in c] for c in mb_["clusters"]],
             "T_k_kg": {d: pair(mb_["T"][k]) for k, d in enumerate("XYZ")},
             "T_k_kg_by_constituent": {c: {d: pair(mb_["Tc"][c][k]) for k, d in enumerate("XYZ")}
                                       for c in CONSTITUENTS},
             "tau_t": "1e-6", "ell_m": pair(mb_["ell"]),
             "smallest_relative_eigenvalue_gap_extracted": small(min(mb_["gaps"])),
             "modes": []}
    cum = [D0, D0, D0]
    for md in mb_["modes"]:
        for k in range(3):
            cum[k] += md["meff"][k]
        nd = md["norm_dof"]
        ent = {"mode": md["index"], "lambda_rad2_per_s2": pair(md["lambda"]), "omega_rad_per_s": pair(md["omega"]),
               "f_Hz": pair(md["f"]), "T_s": pair(1 / md["f"]),
               "normalization": md["case"], "normalizing_dof": f"{m.node_ids[nd // 6]}.{DOFS[nd % 6]}",
               "normalization_tie_dofs": [f"{m.node_ids[t // 6]}.{DOFS[t % 6]}" for t in md["ties"]]
               if len(md["ties"]) > 1 else [],
               "generalized_mass": pair(md["m"]),
               "participation": {d: pair(md["Gamma"][k]) for k, d in enumerate("XYZ")},
               "effective_mass_kg": {d: pair(md["meff"][k]) for k, d in enumerate("XYZ")},
               "effective_mass_fraction": {d: pair(md["meff"][k] / mb_["T"][k]) for k, d in enumerate("XYZ")},
               "cumulative_fraction": {d: pair(cum[k] / mb_["T"][k]) for k, d in enumerate("XYZ")},
               "extracted": md["index"] <= n_ext}
        fl = Flusher(lambda key: "rad" if key.split(".")[-1].startswith("r") else "m")
        shape = {f"{m.node_ids[g // 6]}.{DOFS[g % 6]}": md["phi"][g] for g in range(sys_.n)}
        fl.see_all(shape)
        ent["shape"] = fl.block(shape)
        basis["modes"].append(ent)
    ch = mb_["checks"]
    basis["cross_checks"] = {k: (small(v) if isinstance(v, Decimal) else v) for k, v in ch.items()}
    basis["sturm_counts"] = [{"shift_lambda": pair(s["shift"]), "count": s["count"]} for s in mb_["sturm"]]
    return derived, basis


def run_variant(args):
    vid, prec = args
    var = next(v for v in cases.VARIANTS if v["id"] == vid)
    sys_ = System(var["model"], var["basis"], prec)
    req = cases.request(vid)
    n_ext = req["extracted_modes"]
    mb_ = modal_basis(sys_, n_ext, route_a=(vid == "W1-BASE" and prec == PREC))
    an = analyse(sys_, mb_, req, cases.SPECTRA, grid=(vid == "W1-BASE"), detail=(vid == "W1-BASE"))
    raw = an.pop("_raw")
    derived, basis = publish_model_block(sys_, mb_, n_ext)
    rec = {"id": vid, "purpose": var["purpose"], "model": var["model"], "derived_model": derived,
           "modal_basis": basis, "analysis": an}
    summary = variant_summary(sys_, mb_, raw, n_ext)
    return vid, prec, rec, summary


SUMMARY_KEYS = None


def variant_summary(sys_, mb_, raw, n_ext):
    keys = ([f"{n}.{d}" for n in ("N3", "N4", "N6", "N7") for d in DOFS]
            + [k for k, _ in sys_.reaction_keys]
            + [k for k in sys_.axes_keys if k.split(".")[0] in ("RD", "BV")])
    comb = dict(raw["comb"])
    comb.update(raw["comb_a"])
    return {"frequencies_Hz": [md["f"] for md in mb_["modes"][:n_ext]],
            "T_k": mb_["T"], "combined": {k: comb[k] for k in keys}}


# ==========================================================================
# Replay of frozen f2 discrete analyses (implementation control for the fresh recovery)
# ==========================================================================
def replay(args):
    fname, case_id, analysis_id = args
    data = json.loads((F2_DISCRETE / fname).read_text())
    case = next(c for c in data["cases"] if c["id"] == case_id)
    an_ref = next(a for a in case["analyses"] if a["request"]["id"] == analysis_id)
    sys_ = System(case["model"], "PF", PREC)
    req = an_ref["request"]
    mb_ = modal_basis(sys_, req["extracted_modes"])
    an = analyse(sys_, mb_, req, case["spectra"], grid=False, detail=False)
    raw = an["_raw"]
    res = {"file": fname, "case": case_id, "analysis": analysis_id, "blocks": {}}

    def compare(name, ref_block, mine):
        scale = {}
        for k, v in ref_block.items():
            u = unit_of(k)
            scale[u] = max(scale.get(u, D0), abs(Decimal(v[0])))
        worst = D0
        same = 0
        n = 0
        for k, v in ref_block.items():
            if k not in mine:
                continue
            n += 1
            mv_ = mine[k]
            d = abs(mv_ - Decimal(v[0]))
            sc = scale[unit_of(k)]
            worst = max(worst, d / sc if sc else d)
            if (f1n.fmt(mv_, DIGITS) if abs(mv_) >= FLUSH * sc else "0") == v[0]:
                same += 1
        res["blocks"][name] = {"values_compared": n, "identical_40_digit_strings": same,
                               "largest_difference_relative_to_unit_scale": small(worst)}
        assert n > 0 and worst < Decimal("1e-36"), (name, worst)

    for d, lst in an_ref["contributions_signed"].items():
        for ent in lst:
            if ent.get("status") != "periodic":
                continue
            i = ent["mode"] - 1
            compare(f"contributions_signed.{d}.mode{ent['mode']}", ent["values"], raw["contrib"][d][i])
    for d, blk in an_ref.get("rigid_response_signed", {}).items():
        compare(f"rigid_response_signed.{d}", blk, raw["rigid"][d])
    compare("combined", an_ref["combined"], raw["comb"])
    ax = an_ref.get("element_end_forces_member_axes")
    if ax:
        compare("member_axes.combined", ax["combined"], raw["comb_a"])
        for d, lst in ax["contributions_signed"].items():
            for ent in lst:
                compare(f"member_axes.contributions_signed.{d}.mode{ent['mode']}", ent["values"],
                        raw["contrib_ax"][d][ent["mode"] - 1])
    return res


# ==========================================================================
# Assembly
# ==========================================================================
def header(doc):
    return {
        "document": doc,
        "status": STATUS,
        "deliverable": "DEL-04-10 (response spectrum: CLM-001 recovery), on DEL-04-09's modal basis",
        "units": {"length": "m", "mass": "kg", "force": "N", "moment": "N*m", "time": "s", "frequency": "Hz",
                  "acceleration": "m/s^2"},
        "dof_order": DOFS,
        "number_format": {"pair": "[decimal with 40 significant digits, nearest binary64]",
                          "small": "a two-significant-digit check residual",
                          "flush": "a value below 1e-50 of the largest magnitude of its unit in its block is "
                                   "printed as 0; a value between 1e-50 and 1e-35 of it is refused at generation",
                          "working_precision_digits": PREC},
        "provenance": {"f1_modules_imported_unmodified": {k: v for k, v in F1_PINS.items()},
                       "f1_directory": "validation/references/f1_modal/mass_discrete/generators",
                       "product_code": "none imported, read or run"},
    }


SKIP_MARKERS = ("cross_checks", "identities", "balance_residuals", "formation_identity",
                "smallest_relative_eigenvalue_gap_extracted")


def compare_payloads(a, b, path, acc):
    """Every published pair of the 80-digit run against the 110-digit rerun. Check residuals (two-digit
    values that measure the working precision itself) are skipped by design."""
    if any(mk in path for mk in SKIP_MARKERS):
        return
    if isinstance(a, dict):
        assert isinstance(b, dict) and a.keys() == b.keys(), path
        for k in a:
            compare_payloads(a[k], b[k], f"{path}.{k}", acc)
    elif isinstance(a, list):
        assert isinstance(b, list) and len(a) == len(b), path
        if len(a) == 2 and isinstance(a[0], str) and isinstance(a[1], float):
            acc["pairs"] += 1
            if a[0] != b[0]:
                acc["differ"] += 1
                da, db = Decimal(a[0]), Decimal(b[0])
                assert da != 0 and db != 0, f"flush differs at {path}"
                ulp = abs(da).adjusted() - (DIGITS - 1)
                units = abs(da - db) / Decimal(10) ** ulp
                assert units <= 1, f"{path}: {a[0]} vs {b[0]}"
                acc["max_units"] = max(acc["max_units"], units)
            return
        for k, (x, y) in enumerate(zip(a, b)):
            compare_payloads(x, y, f"{path}[{k}]", acc)
    else:
        assert a == b, f"{path}: {a!r} vs {b!r}"


def build(jobs):
    tasks = [(v["id"], p) for v in cases.VARIANTS for p in (PREC, PREC_CHECK)]
    rtasks = [("c3_cantilever_3d.json", "C3-CANTILEVER-3D", "C3-BASE"), ("c9_pipe_diagonal.json",
                                                                        "C9-PIPE-DIAGONAL", "C9-Y")]
    for name, h in F2_PINS.items():
        assert sha256_file(F2_DISCRETE / name) == h, f"pinned f2 file changed: {name}"
    results = {}
    summaries = {}
    if jobs > 1:
        with ProcessPoolExecutor(max_workers=jobs) as ex:
            fv = [ex.submit(run_variant, t) for t in tasks]
            fr = [ex.submit(replay, t) for t in rtasks]
            vres = [f.result() for f in fv]
            rres = [f.result() for f in fr]
    else:
        vres = [run_variant(t) for t in tasks]
        rres = [replay(t) for t in rtasks]
    for vid, prec, rec, summ in vres:
        results[(vid, prec)] = rec
        summaries[(vid, prec)] = summ
    f1n.set_point_precision(PREC)
    # ---- precision convergence: every published pair at 80 digits equals the 110-digit rerun
    conv = {"pairs": 0, "differ": 0, "max_units": D0}
    for v in cases.VARIANTS:
        compare_payloads(results[(v["id"], PREC)], results[(v["id"], PREC_CHECK)], v["id"], conv)
    # ---- EX: only c_m + c_a enters, so W1-MILL and W1-CORR agree in every computed value
    a, b = copy.deepcopy(results[("W1-MILL", PREC)]), copy.deepcopy(results[("W1-CORR", PREC)])
    for r in (a, b):
        for k in ("id", "purpose", "model"):
            r.pop(k)
        r["analysis"]["request"].pop("id")
    assert json.dumps(a, sort_keys=True) == json.dumps(b, sort_keys=True), "W1-MILL and W1-CORR differ on EX"
    base = results[("W1-BASE", PREC)]
    inputs = inputs_table([v["model"] for v in cases.VARIANTS] + [cases.SPECTRA, cases.request("W1-BASE"),
                                                                  cases.STATED_SHEAR_MODULUS_PA])
    conventions = CONVENTIONS
    out = {}
    doc = header("f2_spectrum/recovery_w1: W1-BASE, with the full method grid")
    doc.update({"conventions": conventions, "inputs_binary64": inputs, "spectra": cases.SPECTRA,
                "variant": base})
    out["w1_base.json"] = doc
    doc = header("f2_spectrum/recovery_w1: diagnostic variants of the W1 family (base request only)")
    doc.update({"conventions": "as w1_base.json", "spectra": cases.SPECTRA,
                "variants": [results[(v["id"], PREC)] for v in cases.VARIANTS if v["id"] != "W1-BASE"]})
    out["w1_variants.json"] = doc
    doc = header("f2_spectrum/recovery_w1: controls")
    doc["provenance"]["f2_discrete_outputs_read_by_the_replay"] = dict(F2_PINS)
    doc.update({
        "precision_convergence": {
            "rerun_working_precision_digits": PREC_CHECK,
            "published_pairs_compared": conv["pairs"],
            "pairs_whose_40_digit_strings_differ": conv["differ"],
            "largest_difference_in_units_of_the_40th_digit": str(conv["max_units"]),
            "statement": "Every variant was recomputed in full at 110 digits. Each published 40-digit decimal "
                         "equals the 110-digit result's, or differs from it by at most one unit in the 40th "
                         "digit where the value lies at a rounding boundary; no flush decision differs. Check "
                         "residuals (two-digit values) depend on the working precision by design and are not "
                         "compared."},
        "ex_mill_corrosion_witness": {
            "statement": "W1-MILL (S1 c_m 1/1024 -> 2/1024 m) and W1-CORR (S1 c_a 3/1024 -> 4/1024 m) give "
                         "identical published values in every block except their own inputs: on the EX basis "
                         "metal and stiffness are on the nominal wall and only the pressure bore, through "
                         "t_r = t - c_m - c_a, sees either allowance.",
            "asserted": True},
        "section_contrast": section_contrast(),
        "variant_summary": summary_table(summaries),
        "f2_replay": {
            "statement": "The fresh recovery and combination code of this generator, run on the frozen f2 "
                         "discrete inputs of C3-BASE (rigid response, every direction) and C9-Y (inclined "
                         "members, member axes), reproduces their published values. This is an "
                         "implementation control: both packages share the pinned f1 formation modules, so it "
                         "is not independent evidence for them. Differences are reported, never fitted.",
            "results": rres},
    })
    out["w1_controls.json"] = doc
    return out


def section_contrast():
    rows = []
    for vid in ("W1-BASE", "W1-MILL", "W1-CORR"):
        var = next(v for v in cases.VARIANTS if v["id"] == vid)
        sec = var["model"]["sections"]["S1"]
        mat = dict(var["model"]["materials"]["steel_state"])
        for basis in ("EX", "PF"):
            sb = f1m.section_basis(sec, mat, basis)
            rows.append({"variant": vid, "section": "S1", "basis": basis, "method": sb["method"],
                         "mill_tolerance_m": str(fq(sec["mill_tolerance_m"])),
                         "corrosion_allowance_m": str(fq(sec["corrosion_allowance_m"])),
                         **{f"{k}_coefficient_of_pi": str(sb[k]) for k in
                            ("metal", "contents", "insulation", "mbar", "jbar", "A", "I", "J")}})
    return {"statement": "S1 section bases on EX and PF for the mill and corrosion variants (exact rational "
                         "coefficients of pi). EX: metal, jbar and A, I, J on the nominal wall; contents on "
                         "the pressure bore. PF: metal, contents and A, I, J on t - c_m; c_a absent. No PF "
                         "modal or spectrum analysis is published here.",
            "rows": rows}


def summary_table(summaries):
    base = summaries[("W1-BASE", PREC)]
    out = []
    for v in cases.VARIANTS:
        s = summaries[(v["id"], PREC)]
        ent = {"variant": v["id"], "frequencies_Hz": [pair(f) for f in s["frequencies_Hz"]],
               "T_k_kg": [pair(t) for t in s["T_k"]], "combined_base_request": {}}
        for k, val in s["combined"].items():
            b = base["combined"].get(k)
            ent["combined_base_request"][k] = {"value": pair(val),
                                               "ratio_to_W1_BASE": pair(val / b) if b else None}
        out.append(ent)
    return {"statement": "Selected combined magnitudes under the base request (CQC, missing mass on with SRSS, "
                         "directional SRSS) for every variant, with the ratio to W1-BASE.",
            "rows": out}


CONVENTIONS = {
    "excitation": "p_k = (M R_k)_f, R_k the rigid unit translation of every DOF, consistent-mass coupling to "
                  "restrained DOFs included; Gamma = phi^T p_k / m, m_eff = (phi^T p_k)^2 / m, "
                  "T_k = R_k^T M R_k over all DOFs.",
    "modal_response": "u_ik = Gamma_ik s_k S_k(f_i) / omega_i^2 phi_i on all DOFs, relative to the supports; "
                      "inertia field g = omega_i^2 u_ik; inertia load M g.",
    "rigid_response": "g = a_r,k (R_k - sum over periodic modes of Gamma_ik phi_i), F_k = M g over all DOFs; "
                      "K_ff u_f = F_k,f; a_r,k = s_k S_k(f_r).",
    "element_end_forces": "f_e = K_e u_e - M_e g_e: the 12 generalized forces acting on the element at node i "
                          "then node j, Fx Fy Fz Mx My Mz. M_e is the member's own consistent mass (absent "
                          "where an eligible body replaces it); a body's inertia acts at its node, not in "
                          "any element.",
    "reactions": "the generalized force the support applies to the piping, global axes: (K u - M g) at each "
                 "restrained DOF, which includes mass at restrained DOFs, consistent coupling and a body "
                 "coupled to a restrained DOF.",
    "member_axes": "for a member from node i to node j: x = (x_j - x_i)/|x_j - x_i|, y = y_reference less "
                   "its x part, normalized, z = x cross y; Lambda has rows x, y, z; each 3-vector of a global "
                   "end force is rotated by Lambda. Combined member-axis magnitudes combine the rotated "
                   "signed contributions; they are not rotations of combined global magnitudes.",
    "constituents": "metal_translational and contents and insulation_translational: Hermitian consistent "
                    "translation mass plus linear axial mass of that part's mass per length; metal_rotary and "
                    "insulation_rotary: Rayleigh rotary inertia plus linear torsional mass of that part's "
                    "j_bar (i_bar = j_bar / 2); contents carry no rotary or torsional inertia; "
                    "component_body: the body's point mass W/a at its centre of gravity, no rotary inertia of "
                    "its own. The six constituent matrices sum exactly to the assembled M.",
    "shares": "an inertia share is M_c g (constituent), M_e g_e (member) or M_b g_b (body); its resultant is "
              "the total force and the moment about the global origin, sum over nodes of (q_M + x cross q_F).",
    "combinations": "per response quantity on signed values: SRSS, CQC (Der Kiureghian rho, r = omega_j / "
                    "omega_i, zeta of the spectra), GROUPING (ascending, a group takes each next mode within "
                    "10 % of its lowest, inclusive), TEN_PERCENT (every pair within 10 %, inclusive), "
                    "ABSOLUTE_SUM; rigid with periodic by SRSS or ABSOLUTE_SUM, or OFF (every extracted mode "
                    "periodic, no rigid response); directions by SRSS, 100-40-40 (factor exactly 2/5) or "
                    "ABSOLUTE_SUM. Every extracted scope cluster here is a single mode.",
    "spectra": "interpolated on the table's own abscissa (period for a period table, x = 1/f); no "
               "extrapolation; ln and exp from Decimal at the working precision.",
}


# ==========================================================================
# Driver
# ==========================================================================
def dump(obj):
    return (json.dumps(obj, separators=(",", ":"), ensure_ascii=False) + "\n").encode()


def expected_membership():
    allowed = set(OUTPUTS) | set(RECORDS) | {"SHA256SUMS"} | set(OPTIONAL_RECORDS)
    found = set()
    for root, dirs, files in os.walk(PKG):
        dirs[:] = [d for d in dirs if d != "__pycache__"]
        for f in files:
            found.add(str((Path(root) / f).relative_to(PKG)))
    extra = sorted(found - allowed)
    missing = sorted((set(OUTPUTS) | set(RECORDS) | {"SHA256SUMS"}) - found)
    return extra, missing


def sums_text(blobs):
    names = sorted(set(OUTPUTS) | set(RECORDS) | {n for n in OPTIONAL_RECORDS if (PKG / n).exists()})
    lines = []
    for n in names:
        data = blobs[n] if n in blobs else (PKG / n).read_bytes()
        lines.append(f"{hashlib.sha256(data).hexdigest()}  {n}")
    return ("\n".join(lines) + "\n").encode()


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--write", action="store_true", help="rewrite the JSON files and SHA256SUMS")
    ap.add_argument("--jobs", type=int, default=min(8, os.cpu_count() or 1))
    a = ap.parse_args()
    docs = build(a.jobs)
    blobs = {n: dump(d) for n, d in docs.items()}
    if a.write:
        for n, b in blobs.items():
            (PKG / n).write_bytes(b)
        (PKG / "SHA256SUMS").write_bytes(sums_text(blobs))
        extra, missing = expected_membership()
        assert not extra and not missing, (extra, missing)
        print("written:", ", ".join(sorted(blobs)) + ", SHA256SUMS")
        return 0
    ok = True
    extra, missing = expected_membership()
    if extra or missing:
        print("membership: unexpected", extra, "missing", missing)
        ok = False
    for n, b in blobs.items():
        p = PKG / n
        if not p.exists() or p.read_bytes() != b:
            print("differs:", n)
            ok = False
    if (PKG / "SHA256SUMS").exists() and (PKG / "SHA256SUMS").read_bytes() != sums_text(blobs):
        print("differs: SHA256SUMS")
        ok = False
    print("check:", "all files regenerate byte for byte; SHA256SUMS complete" if ok else "FAILED")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
