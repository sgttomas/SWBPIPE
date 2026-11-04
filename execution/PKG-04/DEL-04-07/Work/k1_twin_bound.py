"""RT-A3 increment 4, K-1 amendment basis (DEL-04-07 `route-unification-r4.md` §3.5, option (a); the note
`k1-twin-bound.md`): independent forward-error enclosures of the K-D5 controls' PF and EX loop rows, and the per-family
bound |x_EX - x_PF| <= a_f + r_f * S_f they justify.

For every control and load case that `rt_a3_nonlinear_k1_twins.rs` solves, this script forms each route's intended
system in exact rationals from that route's own binary64 section leaves: for every enumerated nonsingular support
state the DEC-067 classification admits as final, and for the iterates that precede the published final iterate
(the seed solve and, for three-iterate cases, the second iterate derived from it; `seed_combo`, `second_iterate`). It reuses the independent kd5 reference
(`validation/references/kd5_nonlinear/reference.py`: element formation, transformation and assembly), which calls no
product code. PF's leaves are the kd5 oracles' `normalized_sections`. EX's are the measured D-5 leaves, A 3 ulp and
I, J 1 ulp below PF's (pinned by `kd5_twin_section_constants_differ_from_pf_in_the_last_place`).

Per published value x it bounds |x_EX - x_PF| <= |x*_EX - x*_PF| + E_EX(x) + E_PF(x) + P(x):
- x*_EX - x*_PF: the exact response of the intended system to the section difference (the "reduced-system
  perturbation"), computed exactly, not estimated;
- E_route(x): a forward-error enclosure of the route's binary64 value about its own exact x*. Every structural solve
  returns only when each free row's original-equation residual meets 64*gamma(n) of its row denominator
  (`frame_kernel::structural::finish_structural` over `evaluate_original_residual`), for every iterate. By Oettli and
  Prager the computed u then solves (K + dK) u = f + df with |dK| <= w|K|, |df| <= w|f|, w = 64*gamma(n_max), and
  w_tot = w + e_form also absorbs each route's binary64 formation of K about the exact-leaves K (e_form = gamma(16)).
  The free-DOF error obeys e <= w_tot*|A^-1|*(|K|_abs*(|u*|+e) + |f| + |K_fc|*|p|) (Skeel), closed by a weighted
  fixed point. The enclosure includes:
  - the prescribed terms |K_fc|*|p| (gap closure);
  - an applied friction force's own error E_t, directly and through the certificate, in the same fixed point;
  - each recovered reaction's cancellation and its row's formation, sum_j |K_ij|*e_j
    + gamma(16)*sum_j |K|_abs,ij*(|u_j|+e_j) + gamma(n+1)*(1+gamma(16))*(|K|_abs*(|u|+e) + |f|)_i;
  - each free row's residual (an inactive support's reaction);
  - the spring actions;
  - and, for a sliding derived-normal friction support, the loop's coupled force phi = -c*R_b/(1 + c*g) from its base
    and unit solves. Its error is (|c|*E_Rb + |c|*E_g*|phi*|)/(|1 + c*g| - |c|*E_g), with E_g the cancellation of
    g = R_u - R_b, plus gamma(4)*|phi|; it enters the final solve as an applied force's own error;
- P(x): publication rounding, the mm conversion and the norm.

An iterate-delta row max_i |x_n,i - x_n-1,i| is bounded on the derived preceding iterate (`delta_bound`), under the
published iteration count (`ITERATIONS`) and the absence of the seed recovery, both pinned by the K-1 test; each
first-iterate decision the derivation uses must exceed its enclosure on both routes (`second_iterate`).

The bound's scales S_f are formed from the case's INPUTS only (loads, explicit friction normals, member lengths, the
authored section, E, G, spring stiffness and gap clearance), never from either route's outputs. The script reports, per
family, the largest enclosure / S_f over every case and the a_f, r_f this justifies. It also reports the
final states' active-set margins against the enclosure of their deciding quantity, and the coupled denominators.

Run: python3 execution/PKG-04/DEL-04-07/Work/k1_twin_bound.py [--json]. Standard library only; no product code.
"""
from __future__ import annotations

import argparse
import importlib.util
import json
import math
import sys
from fractions import Fraction as F
from itertools import product
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
REF = ROOT / "validation/references/kd5_nonlinear"
sys.dont_write_bytecode = True  # leave no cache beside the frozen reference
_spec = importlib.util.spec_from_file_location("kd5_reference", REF / "reference.py")
kd5 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(kd5)

U = F(1, 2**53)  # unit roundoff
PF_LEAVES = {"As_m2": 0.003540574920595698, "I_m4": 1.1493591335983782e-05, "J_m4": 2.2987182671967564e-05}
# EX's measured D-5 leaves for the controls' 0.168 x 0.007 m section (pinned by the K-1 test).
EX_LEAVES = {"As_m2": 0.003540574920595697, "I_m4": 1.149359133598378e-05, "J_m4": 2.298718267196756e-05}
DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
# (label, request stem, load cases, gap edit), as `rt_a3_nonlinear_k1_twins.rs` CONTROLS.
CONTROLS = [
    ("F-1 (derived normal)", "f1", ["load:SLIDE-POS", "load:SLIDE-NEG", "load:STICK", "load:BOUNDARY"], False),
    ("F-1 explicit normal", "f1_explicit", ["load:SLIDE", "load:STICK"], False),
    ("F-1 mu = 0", "f1_mu0", ["load:FREE"], False),
    ("F-2", "f2", ["load:SLIDE"], False),
    ("U-1", "u1", ["load:PRESSED", "load:LIFTED", "load:IN-PLANE"], False),
    ("U-1 parallel spring", "u1_spring", ["load:PRESSED", "load:LIFTED"], False),
    ("U-1 rotational (RX)", "u1_rx", ["load:PRESSED", "load:LIFTED"], False),
    ("G-1 gap containment (U-1 as a gap)", "u1", ["load:PRESSED", "load:LIFTED"], True),
]
FAMILIES = ["translation", "rotation", "force", "moment", "residual_force", "residual_moment", "work"]
# The loop's iterate-delta rows (max |x_k - x_k-1| of the final and previous iterates): their base family's scale.
DELTAS = {"translation_delta": "translation", "rotation_delta": "rotation", "force_delta": "force",
          "moment_delta": "moment"}
UNITS = {"translation": "m", "rotation": "rad", "force": "N", "moment": "N*m", "residual_force": "N",
         "residual_moment": "N*m", "work": "N*m", "translation_delta": "m", "rotation_delta": "rad",
         "force_delta": "N", "moment_delta": "N*m"}
GAP = {"behavior": "gap", "dof": "UZ", "initial_state": "inactive", "closes_when": "negative_displacement",
       "gap": {"value": 1e-5, "unit": "m"}}


def gamma(n: int) -> F:
    return n * U / (1 - n * U)


# --------------------------------------------------------------------------------------------- the model
def request(stem: str, gap: bool) -> dict:
    r = json.loads((REF / f"{stem}_request.json").read_text())
    if gap:
        next(s for s in r["model"]["supports"] if s["id"] == "support:NL-U")["nonlinear"] = dict(GAP)
    return r


class Route:
    """One route's intended exact-leaves system: kd5's `Model`, assembled from the given leaves, with |K|_abs."""

    def __init__(self, req: dict, leaves: dict):
        saved = dict(kd5.LEAVES)
        kd5.LEAVES[(0.168, 0.007)] = leaves
        gap = None
        for s in req["model"]["supports"]:
            if (s.get("nonlinear") or {}).get("behavior") == "gap":
                gap = s
        try:
            if gap is not None:  # kd5's Model reads one-way and friction only; form it as one-way, then restate.
                saved_nl = gap["nonlinear"]
                gap["nonlinear"] = {"behavior": "one_way", "dof": saved_nl["dof"], "active_when": "positive_reaction"}
                self.m = kd5.Model(req)
                gap["nonlinear"] = saved_nl
                e = next(e for e in self.m.nonlinear if e["id"] == gap["id"])
                e.clear()
                e.update(id=gap["id"], dof=6 * self.m.nodes.index(gap["node"]) + DOFS.index(saved_nl["dof"]), behavior="gap",
                         sense=-1 if saved_nl["closes_when"].startswith("negative") else 1,
                         clearance=F(float(saved_nl["gap"]["value"])))
            else:
                self.m = kd5.Model(req)
        finally:
            kd5.LEAVES.clear()
            kd5.LEAVES.update(saved)
        n = self.m.n
        self.abs = [[F(0)] * n for _ in range(n)]
        for _, kg, _, _, dofs in self.m.members:
            for a, ga in enumerate(dofs):
                for b, gb in enumerate(dofs):
                    self.abs[ga][gb] += abs(kg[a][b])
        for _, d, k in self.m.springs:
            self.abs[d][d] += abs(k)
        # n = 2*count + 2 per row (count: the row's nonzero entries), its largest over the rows.
        count = max(sum(1 for v in row if v != 0) for row in self.m.K)
        self.n_max = 2 * count + 2
        self.w = 64 * gamma(self.n_max) + gamma(16) + 64 * gamma(self.n_max) * gamma(16)


def states_for(entry):
    if entry["behavior"] == "gap":
        return [("inactive", None, None), ("active", None, None)]
    return kd5.states_for(entry)


def inverse(A):
    """Exact inverse by Gauss-Jordan with partial pivoting (kd5's `solve`, every right-hand side at once)."""
    n = len(A)
    aug = [list(row) + [F(int(i == k)) for k in range(n)] for i, row in enumerate(A)]
    for col in range(n):
        pivot = max(range(col, n), key=lambda r: abs(aug[r][col]))
        aug[col], aug[pivot] = aug[pivot], aug[col]
        p = aug[col][col]
        if p == 0:
            return None
        aug[col] = [x / p for x in aug[col]]
        for row in range(n):
            if row != col and aug[row][col]:
                factor = aug[row][col]
                aug[row] = [a - factor * c for a, c in zip(aug[row], aug[col])]
    return [row[n:] for row in aug]


def mat_vec(M, v):
    return [sum(a * b for a, b in zip(row, v)) for row in M]


class Solved:
    """One route, one case, one support state: the exact solution and every enclosure."""

    def __init__(self, route: Route, f, combo):
        m, K, Kabs, w = route.m, route.m.K, route.abs, route.w
        n = m.n
        constrained = {d: F(0) for dofs in m.rigid.values() for d in dofs}
        explicit, derived = {}, {}
        for entry, (state, direction, beta) in zip(m.nonlinear, combo):
            d = entry["dof"]
            if entry["behavior"] == "gap":
                if state == "active":
                    constrained[d] = entry["sense"] * entry["clearance"]
            elif state in ("active", "sticking"):
                constrained[d] = F(0)
            elif state == "sliding" and entry["mu"] != 0 and direction != 0:
                # direction 0: a sliding seed on the loop's first iterate, which has no previous iterate to orient
                # or scale its force, so none is applied (`nonlinear_integration` `sliding_friction_candidates`).
                if "source" in entry:
                    derived[d] = (entry, direction, beta)
                else:
                    explicit[d] = -direction * entry["mu"] * abs(entry["normal"])
        free = [i for i in range(n) if i not in constrained]
        A = [[K[i][j] for j in free] for i in free]
        Ainv = inverse(A)
        self.ok = Ainv is not None
        if not self.ok:
            return
        p = [constrained.get(j, F(0)) for j in range(n)]
        base = [f[i] + explicit.get(i, F(0)) for i in range(n)]  # the loop's base force (explicit friction applied)

        def exact(force):
            rhs = [force[i] - sum(K[i][j] * p[j] for j in constrained) for i in free]
            x = mat_vec(Ainv, rhs)
            u = list(p)
            for i, v in zip(free, x):
                u[i] = v
            return u

        def residual(u, force):
            return [sum(K[i][j] * u[j] for j in range(n)) - force[i] for i in range(n)]

        absAinv = [[abs(v) for v in row] for row in Ainv]
        Kff_abs = [[Kabs[i][j] for j in free] for i in free]
        M = [[w * sum(absAinv[i][k] * Kff_abs[k][j] for k in range(len(free))) for j in range(len(free))]
             for i in range(len(free))]

        def solve_enclosure(u, force, applied=None):
            """Free-DOF forward-error enclosure (full-length, zero at constrained DOFs) of a certified solve about the
            exact solution u of the exact force. `applied` maps a DOF to the error bound E_t of the computed force
            there (an applied friction force): the computed solve solves (K + dK) u_hat = f_hat + df with
            f_hat = f + delta_t e_t, so e <= |A^-1| E_t e_t + w |A^-1| (|K|_abs (|u| + e) + |f| + E_t e_t). Both
            the direct term and its feedback through the certificate are closed by the same weighted fixed point."""
            applied = applied or {}
            h = [sum(Kabs[i][j] * abs(u[j]) for j in range(n)) + abs(force[i]) + applied.get(i, F(0)) for i in free]
            b = [w * v for v in mat_vec(absAinv, h)]
            for t, ep in applied.items():
                k = free.index(t)
                for i in range(len(free)):
                    b[i] += absAinv[i][k] * ep
            tiny = F(1, 2**1074)
            s = [v + tiny for v in b]
            theta = max(sum(M[i][j] * s[j] for j in range(len(free))) / s[i] for i in range(len(free)))
            assert theta < 1, "the certificate's Skeel fixed point needs theta < 1"
            e = [F(0)] * n
            for i, v in zip(free, s):
                e[i] = v / (1 - theta)
            return e, theta

        def recovery(u, e, force, i):
            """|R_hat_i - R*_i| for a recovered reaction (or a free row's residual). R_hat_i = fl(K_hat_i . u_hat - f_i)
            with the route's rounded assembled row K_hat_i, |K_hat - K| <= gamma(16) |K|_abs (the formation allowance
            inside w). R_hat_i - R*_i = [rounding] + (K_hat_i - K_i) u_hat + K_i (u_hat - u), so the bound is
            sum_j |K_ij| e_j + gamma(16) sum_j |K|_abs,ij (|u_j| + e_j) + gamma(n + 1) (1 + gamma(16)) (...)_i: the
            direct formation term covers a constrained-constrained entry times a prescribed value, which no
            displacement error can (the nl3 review)."""
            stiff = sum(Kabs[i][j] * (abs(u[j]) + e[j]) for j in range(n))
            terms = stiff + abs(force[i])
            count = sum(1 for j in range(n) if K[i][j] != 0)
            cancel = (sum(abs(K[i][j]) * e[j] for j in range(n)) + gamma(16) * stiff
                      + gamma(count + 1) * (1 + gamma(16)) * terms)
            if i in constrained:
                return cancel
            # A free row's computed residual: the certificate bounds it directly, plus its evaluation.
            return min(cancel, (w + gamma(count + 1)) * terms)

        self.coupling = {}
        # The explicit forces' own rounding, fl(mu*|N|), known before any solve.
        e_explicit = {t: U * abs(value) for t, value in explicit.items()}
        e_phi = {}
        force = list(base)
        if derived:
            assert len(derived) == 1, "the controls have at most one sliding derived-normal support"
            (t, (entry, direction, beta)), = derived.items()
            src, c = entry["source"], direction * entry["mu"] * beta
            ub = exact(base)
            Rb = residual(ub, base)[src]
            unit = list(base)
            unit[t] += 1
            uu = exact(unit)
            Ru = residual(uu, unit)[src]
            g = Ru - Rb
            denominator = 1 + c * g
            phi = -c * Rb / denominator
            eb, _ = solve_enclosure(ub, base, e_explicit)
            eu, _ = solve_enclosure(uu, unit, e_explicit)
            E_Rb = recovery(ub, eb, base, src)
            E_g = E_Rb + recovery(uu, eu, unit, src) + U * abs(g)
            assert abs(denominator) > abs(c) * E_g
            e_phi[t] = (abs(c) * E_Rb + abs(c) * E_g * abs(phi)) / (abs(denominator) - abs(c) * E_g)
            e_phi[t] += gamma(4) * (abs(phi) + e_phi[t])
            force[t] += phi
            self.coupling[entry["id"]] = {"c": float(c), "g": float(g), "1+c*g": float(denominator),
                                          "phi_N": float(phi), "E_phi_N": float(e_phi[t])}
        e_phi.update(e_explicit)
        u = exact(force)
        # The applied forces' own errors enter the fixed point (directly and through the certificate's feedback).
        e, self.theta = solve_enclosure(u, force, e_phi)
        r = residual(u, force)
        R = [r[i] + (force[i] - f[i] if i in e_phi else 0) for i in range(n)]  # the loop's reported reactions
        E_R = [recovery(u, e, force, i) + e_phi.get(i, 0) for i in range(n)]
        self.route, self.combo, self.free, self.constrained = route, combo, free, constrained
        self.u, self.e, self.R, self.E_R, self.force = u, e, R, E_R, force
        # Free-row residual records (an evaluation of each free row) and the work residual.
        self.res = {"force": F(0), "moment": F(0), "work": F(0)}
        for i in free:
            count = sum(1 for j in range(n) if K[i][j] != 0)
            bound = (w + gamma(count + 1)) * (sum(Kabs[i][j] * (abs(u[j]) + e[j]) for j in range(n)) + abs(force[i]))
            family = "force" if i % 6 < 3 else "moment"
            self.res[family] = max(self.res[family], bound)
            self.res["work"] = max(self.res["work"], (abs(u[i]) + e[i]) * bound)

    def published(self):
        """The K-1 loop rows of this state: key -> (family, exact value, enclosure, publication rounding)."""
        m, out = self.route.m, {}
        for node_index, node in enumerate(m.nodes):
            for k in range(6):
                d = 6 * node_index + k
                family = "translation" if k < 3 else "rotation"
                x, e = self.u[d], self.e[d]
                out[f"nodal {node} {DOFS[k]}"] = (family, x, e, 2 * U * (abs(x) + e))
        slots = {}
        for entry, (state, _, _) in zip(m.nonlinear, self.combo):
            slots[entry["id"]] = entry["dof"]
        for sid, node in m.supports:
            comps = []
            for k in range(6):
                d = 6 * node + k
                if d in m.rigid.get(sid, []) or slots.get(sid) == d:
                    x, e = self.R[d], self.E_R[d]
                elif any(s == sid and dd == d for s, dd, _ in m.springs):
                    kk = next(kk for s, dd, kk in m.springs if s == sid and dd == d)
                    x, e = -kk * self.u[d], abs(kk) * self.e[d] + U * abs(kk) * (abs(self.u[d]) + self.e[d])
                else:
                    x, e = F(0), F(0)
                comps.append((x, e))
                out[f"support {sid} {DOFS[k]}"] = ("force" if k < 3 else "moment", x, e, U * (abs(x) + e))
            # | |a| - |b| | <= sum_k |a_k - b_k|: the difference is taken over the components (`run`).
            for name, part, family, keys in (("force_magnitude", comps[:3], "force", DOFS[:3]),
                                             ("moment_magnitude", comps[3:], "moment", DOFS[3:])):
                x = F(math.sqrt(float(sum(v * v for v, _ in part))))
                e = sum(err for _, err in part)
                out[f"support {sid} {name}"] = (family, x, e, U * (abs(x) + e), [f"support {sid} {k}" for k in keys])
        for entry in m.nonlinear:
            d = entry["dof"]
            fam_u, fam_r = ("translation", "force") if d % 6 < 3 else ("rotation", "moment")
            out[f"nonlinear {entry['id']} displacement"] = (fam_u, self.u[d], self.e[d], 2 * U * (abs(self.u[d]) + self.e[d]))
            out[f"nonlinear {entry['id']} reaction"] = (fam_r, self.R[d], self.E_R[d], U * (abs(self.R[d]) + self.E_R[d]))
            if "source" in entry:
                s = entry["source"]
                out[f"nonlinear {entry['id']} normal"] = ("force", abs(self.R[s]), self.E_R[s], U * (abs(self.R[s]) + self.E_R[s]))
        return out


# --------------------------------------------------------------------------------------------- the scales
def section(od: float, t: float):
    """A, I and J of the authored annulus, in binary64 (an input scale, not a published leaf)."""
    d = od - 2.0 * t
    od2, d2 = od * od, d * d
    area = math.pi / 4.0 * (od2 - d2)
    inertia = math.pi / 64.0 * (od2 * od2 - d2 * d2)  # the Rust test forms these by the same operations
    return area, inertia, 2.0 * inertia


def scales(req: dict, case_id: str) -> dict:
    """S_f from the case's inputs only (the note's definitions; the Rust test forms the same)."""
    m = req["model"]
    pos = {n["id"]: [float(n["position"][a]) for a in "xyz"] for n in m["nodes"]}
    mats = {x["id"]: x for x in m["materials"]}
    lengths, areas, inertias, polars, e_mod, g_mod = [], [], [], [], 0.0, 0.0
    for pipe in m["pipe_segments"]:
        a, b = pos[pipe["from"]], pos[pipe["to"]]
        dx, dy, dz = b[0] - a[0], b[1] - a[1], b[2] - a[2]
        lengths.append(math.sqrt(dx * dx + dy * dy + dz * dz))
        area, inertia, polar = section(pipe["section"]["outside_diameter"]["value"], pipe["section"]["wall_thickness"]["value"])
        areas.append(area); inertias.append(inertia); polars.append(polar)
        mat = mats[pipe["material"]]
        e_mod, g_mod = float(mat["elastic_modulus"]["value"]), float(mat["shear_modulus"]["value"])
    total, shortest = sum(lengths), min(lengths)
    case = next(c for c in m["load_cases"] if c["id"] == case_id)
    force = sum(abs(float(load["magnitude"]["value"])) for load in case["primitive_loads"])
    clearance, springs = 0.0, 0.0
    for s in m["supports"]:
        nl = s.get("nonlinear") or {}
        if nl.get("behavior") == "friction" and "normal_reaction" in nl:
            force += abs(float(nl["friction_coefficient"]["value"])) * abs(float(nl["normal_reaction"]["value"]))
        if nl.get("behavior") == "gap":
            clearance += abs(float(nl["gap"]["value"]))
        if (s.get("hanger") or {}).get("stiffness"):
            springs += abs(float(s["hanger"]["stiffness"]["value"]["value"]))
    cube, square = total * total * total, total * total
    compliance_t = total / (e_mod * min(areas)) + cube / (e_mod * min(inertias)) + cube / (g_mod * min(polars))
    compliance_r = square / (e_mod * min(inertias)) + square / (g_mod * min(polars))
    stiffness = e_mod * max(areas) / shortest + 12.0 * e_mod * max(inertias) / (shortest * shortest * shortest) + springs
    s_force = force + clearance * stiffness
    s_moment = s_force * total
    s_translation = s_force * compliance_t + clearance
    s_rotation = s_force * compliance_r + clearance / shortest
    return {"translation": s_translation, "rotation": s_rotation, "force": s_force, "moment": s_moment,
            "residual_force": s_force, "residual_moment": s_moment,
            "work": s_force * s_translation + s_moment * s_rotation}


# --------------------------------------------------------------------------------------------- the basis
def admissible(sol: Solved) -> str | None:
    """'strict', 'weak' or None: kd5's DEC-067 classification, with a gap's (separation, contact) added."""
    m = sol.route.m
    gaps = [(e, st) for e, st in zip(m.nonlinear, sol.combo) if e["behavior"] == "gap"]
    others = [(e, st) for e, st in zip(m.nonlinear, sol.combo) if e["behavior"] != "gap"]
    weak = False
    for entry, (state, _, _) in gaps:
        d = entry["dof"]
        value = -entry["sense"] * sol.R[d] if state == "active" else entry["clearance"] - entry["sense"] * sol.u[d]
        if value < 0:
            return None
        weak |= value == 0
    if others:
        r = [sol.R[i] if i in sol.constrained else sol.R[i] for i in range(sol.route.m.n)]
        saved = m.nonlinear
        m.nonlinear = [e for e, _ in others]
        try:
            kind = kd5.classify(m, [st for _, st in others], sol.u, r)
        finally:
            m.nonlinear = saved
        if kind is None:
            return None
        weak |= kind == "weak"
    return "weak" if weak else "strict"


def margins(sol: Solved):
    """Final-state DEC-067 margins (exact) against the enclosure of their deciding quantity."""
    out = []
    m = sol.route.m
    for entry, (state, direction, beta) in zip(m.nonlinear, sol.combo):
        d = entry["dof"]
        if entry["behavior"] in ("one_way", "gap"):
            sense = (1 if entry.get("positive", True) else -1) if entry["behavior"] == "one_way" else -entry["sense"]
            if state == "active":
                out.append((entry["id"], "contact", sense * sol.R[d], sol.E_R[d]))
            else:
                sep = sense * sol.u[d] if entry["behavior"] == "one_way" else entry["clearance"] - entry["sense"] * sol.u[d]
                out.append((entry["id"], "separation", sep, sol.e[d]))
        else:
            normal = abs(sol.R[entry["source"]]) if "source" in entry else entry["normal"]
            e_n = sol.E_R[entry["source"]] if "source" in entry else F(0)
            out.append((entry["id"], "normal", normal, e_n))
            if state == "sticking":
                out.append((entry["id"], "stick capacity", entry["mu"] * normal - abs(sol.R[d]), entry["mu"] * e_n + sol.E_R[d]))
            elif state == "sliding":
                out.append((entry["id"], "slip", direction * sol.u[d], sol.e[d]))
    return out


# The loop's iteration count per case, published identically by both routes in both modes and pinned exactly by the
# K-1 test (`K1_ITERATIONS`): a discrete premise of the iterate-delta derivation, not a consequence of it.
ITERATIONS = {
    ("F-1 (derived normal)", "load:SLIDE-POS"): 2, ("F-1 (derived normal)", "load:SLIDE-NEG"): 2,
    ("F-1 (derived normal)", "load:STICK"): 3, ("F-1 (derived normal)", "load:BOUNDARY"): 3,
    ("F-1 explicit normal", "load:SLIDE"): 2, ("F-1 explicit normal", "load:STICK"): 3,
    ("F-1 mu = 0", "load:FREE"): 2, ("F-2", "load:SLIDE"): 2,
    ("U-1", "load:PRESSED"): 1, ("U-1", "load:LIFTED"): 2, ("U-1", "load:IN-PLANE"): 2,
    ("U-1 parallel spring", "load:PRESSED"): 1, ("U-1 parallel spring", "load:LIFTED"): 2,
    ("U-1 rotational (RX)", "load:PRESSED"): 1, ("U-1 rotational (RX)", "load:LIFTED"): 2,
    ("G-1 gap containment (U-1 as a gap)", "load:PRESSED"): 2, ("G-1 gap containment (U-1 as a gap)", "load:LIFTED"): 1,
}
# An iterate-delta row's base family and its vector: (family, displacements (True) or reactions, translational DOFs).
DELTA_ROWS = {"translation_delta": ("translation", True, True), "rotation_delta": ("rotation", True, False),
              "force_delta": ("force", False, True), "moment_delta": ("moment", False, False)}


def seed_combo(m, req) -> tuple:
    """The first iterate's states: each support's authored `initial_state` (`normalized_initial_states`; the twin keeps
    the PF request's supports, seeds included). A sliding seed has no previous iterate, so no force is applied
    (`sliding_friction_candidates`: no previous, no candidate): direction 0."""
    seeds = {s["id"]: s["nonlinear"]["initial_state"] for s in req["model"]["supports"] if s.get("nonlinear")}
    return tuple(("sliding", 0, None) if seeds[e["id"]] == "sliding" else (seeds[e["id"]], None, None)
                 for e in m.nonlinear)


def second_iterate(first: tuple) -> tuple:
    """The second iterate's input for each friction support seeded sliding, from the first iterate on both routes
    (`first` = (PF, EX) Solved), by the product's rules: the deferred first iterate keeps it sliding unless its normal
    is <= 0 (`resolve_sliding_friction_states`); the slip direction is the sign of the first iterate's displacement at
    the support DOF when nonzero (`sliding_direction`); the derived-normal branch is the sign of the first iterate's
    source reaction when nonzero (`solve_iteration_with_sliding_friction_evidence`). Each deciding quantity must exceed
    its enclosure on both routes and have the same exact sign, so both routes' binary64 decisions are that sign.
    Returns (combo, decisions); a non-friction support's entry is None (the classifier's, published where used)."""
    m = first[0].route.m
    combo, decisions = [], []

    def decide(sid, what, pick):
        values = [pick(sol) for sol in first]
        signs = {(v > 0) - (v < 0) for v, _ in values}
        assert len(signs) == 1 and 0 not in signs, f"{sid} {what}: no common nonzero exact sign"
        for v, e in values:
            assert abs(v) > e, f"{sid} {what}: margin {float(v)} within its enclosure {float(e)}"
        decisions.append((sid, what, float(values[0][0]), max(float(e) for _, e in values)))
        return signs.pop()

    for entry, (state, _, _) in zip(m.nonlinear, first[0].combo):
        if entry["behavior"] != "friction" or state != "sliding":
            combo.append(None)
            continue
        t = entry["dof"]
        if "source" in entry:
            branch = decide(entry["id"], "source reaction (normal > 0, branch)", lambda s: (s.R[entry["source"]], s.E_R[entry["source"]]))
        else:
            assert entry["normal"] > 0, "an explicit normal > 0 keeps the deferred seed sliding"
            branch = None
        direction = decide(entry["id"], "slip (direction)", lambda s: (s.u[t], s.e[t]))
        combo.append(("sliding", direction, branch if "source" in entry and entry["mu"] != 0 else None))
    return tuple(combo), decisions


def delta_bound(final: tuple, previous: tuple, displacement: bool, translational: bool) -> F:
    """|D_EX - D_PF| for one iterate-delta row, D = max_i |x_n,i - x_n-1,i| over the family's DOFs (the loop's
    `max_abs_delta_by_dof_group`, in SI, then published): |max|a| - max|b|| <= max_i |a_i - b_i|, and per DOF the
    exact difference of the routes' exact deltas, the four iterate enclosures, and per route gamma(2) of the delta's
    operands for its subtraction and the row's unit conversion. `final` and `previous` are (PF, EX) Solved pairs."""
    pick = (lambda s: (s.u, s.e)) if displacement else (lambda s: (s.R, s.E_R))
    (fa, ea_n), (fb, eb_n) = pick(final[0]), pick(final[1])
    (pa, ea_p), (pb, eb_p) = pick(previous[0]), pick(previous[1])
    best = F(0)
    for i in range(final[0].route.m.n):
        if (i % 6 < 3) != translational:
            continue
        diff = abs((fb[i] - pb[i]) - (fa[i] - pa[i]))
        rounding = gamma(2) * (abs(fa[i]) + ea_n[i] + abs(pa[i]) + ea_p[i] + abs(fb[i]) + eb_n[i] + abs(pb[i]) + eb_p[i])
        best = max(best, diff + ea_n[i] + ea_p[i] + eb_n[i] + eb_p[i] + rounding)
    return best


def run():
    families = {f: {"ratio": 0.0, "at": None, "final_ratio": 0.0, "final_at": None} for f in FAMILIES}
    families.update({d: {"ratio": 0.0, "at": None, "final_ratio": 0.0, "final_at": None} for d in DELTAS})
    cases, coupled, margin_rows, histories = [], [], [], []
    for label, stem, case_ids, gap in CONTROLS:
        req = request(stem, gap)
        pf, ex = Route(req, PF_LEAVES), Route(req, EX_LEAVES)
        for case_id in case_ids:
            case = next(c for c in pf.m.cases if c["id"] == case_id)
            f = pf.m.force(case)
            s = scales(req, case_id)
            combos = list(product(*[states_for(e) for e in pf.m.nonlinear]))
            row = {"control": label, "case": case_id, "states": 0, "scales": s, "omega": float(pf.w), "n_max": pf.n_max}
            finals = []
            for combo in combos:
                a, b = Solved(pf, f, combo), Solved(ex, f, combo)
                if not (a.ok and b.ok):
                    continue
                row["states"] += 1
                names = {e["id"]: st for e, (st, _, _) in zip(pf.m.nonlinear, combo)}
                standing = admissible(a)
                if standing is None:
                    continue  # not a state the loop can converge in; the value rows are the final iterate's
                assert admissible(b) == standing, "the routes' exact systems admit the same states"
                row.setdefault("final", []).append((names, standing))
                finals.append((a, b))
                pa, pb = a.published(), b.published()
                bounds = {}
                for key, (family, xa, ea, ra, *parts) in pa.items():
                    _, xb, eb, rb, *_ = pb[key]
                    diff = sum(abs(pb[k][1] - pa[k][1]) for k in parts[0]) if parts else abs(xb - xa)
                    bounds.setdefault(family, F(0))
                    bounds[family] = max(bounds[family], diff + ea + eb + ra + rb)
                for family, (pf_bound, ex_bound) in (("residual_force", (a.res["force"], b.res["force"])),
                                                     ("residual_moment", (a.res["moment"], b.res["moment"])),
                                                     ("work", (a.res["work"], b.res["work"]))):
                    bounds[family] = max(pf_bound, ex_bound)
                for family, bound in bounds.items():
                    ratio = float(bound) / s[family] if s[family] > 0 else (0.0 if bound == 0 else math.inf)
                    if ratio > families[family]["final_ratio"]:
                        families[family].update(final_ratio=ratio, final_at=f"{label} {case_id} {names}")
                if a.coupling:
                    coupled.append({"control": label, "case": case_id, **next(iter(a.coupling.values()))})
                for sid, what, margin, enclosure in margins(a):
                    margin_rows.append((label, case_id, sid, what, float(margin), float(enclosure)))
            # The iterate history (the note's section 3, "Iterate deltas"): the preceding iterate of the published
            # delta rows, from the seeds and the first iterate by the product's rules, under the published count.
            n_iter = ITERATIONS[(label, case_id)]
            seed = seed_combo(pf.m, req)
            first = (Solved(pf, f, seed), Solved(ex, f, seed))
            assert first[0].ok and first[1].ok, "the seed solve is nonsingular (no all-active recovery)"
            second, decisions = second_iterate(first)
            history = {"control": label, "case": case_id, "iterations": n_iter, "seed": seed, "decisions": decisions}
            if n_iter == 2:
                previous = first
                # The final iterate is the second: its derived slip direction and branch are an admissible final's.
                for k, entry in enumerate(second):
                    if entry is not None:
                        assert any(fa.combo[k] == entry for fa, _ in finals), f"{label} {case_id}: second iterate {entry}"
            elif n_iter == 3:
                assert all(entry is not None for entry in second), "a three-iterate case has only friction supports"
                previous = (Solved(pf, f, second), Solved(ex, f, second))
                assert previous[0].ok and previous[1].ok
            else:
                assert n_iter == 1
                previous = None  # one iterate: no delta rows are published
            history["previous"] = None if previous is None else previous[0].combo
            if previous is not None:
                for delta, (base, displacement, translational) in DELTA_ROWS.items():
                    bound = max(delta_bound(fin, previous, displacement, translational) for fin in finals)
                    ratio = float(bound) / s[base] if s[base] > 0 else (0.0 if bound == 0 else math.inf)
                    history[delta] = ratio
                    if ratio > families[delta]["final_ratio"]:
                        families[delta].update(final_ratio=ratio, final_at=f"{label} {case_id}")
            histories.append(history)
            cases.append(row)
    # a_f = 0: a zero scale means no load, no applied friction and no gap, so u* = 0 and the certificate forces
    # u_hat = 0 exactly in both routes. r_f: the next power of two at or above twice the largest enclosure / S_f over
    # every case: the final states for value and residual rows (the final iterate's), and the final and the derived
    # preceding iterate for a delta row.
    pow2 = lambda x: 2.0 ** math.ceil(math.log2(x)) if x > 0 else 0.0
    for family, entry in families.items():
        entry["a_f"] = 0.0
        entry["r_f"] = pow2(2.0 * entry["final_ratio"])
    return families, cases, coupled, margin_rows, histories


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--json", action="store_true")
    args = parser.parse_args()
    families, cases, coupled, margin_rows, histories = run()
    if args.json:
        print(json.dumps({"families": families, "coupled": coupled, "margins": margin_rows, "histories": histories,
                          "cases": [{**c, "scales": c["scales"]} for c in cases]}, indent=1, default=str))
        return
    print("family             unit  largest enclosure/S_f   a_f  r_f")
    for family, e in families.items():
        print(f"{family:18s} {UNITS[family]:5s} {e['final_ratio']:.3e}  {e['a_f']}  "
              f"2^{int(math.log2(e['r_f']))} = {e['r_f']:.3e}   ({e['final_at']})")
    print("\ncases (nonsingular states enumerated, admissible finals, omega = 64*gamma(n_max) + formation, S_f):")
    for c in cases:
        print(f"  {c['control']} {c['case']}: {c['states']} states, final {c.get('final')}, n_max {c['n_max']}, omega {c['omega']:.3e}, "
              + ", ".join(f"{k} {v:.3e}" for k, v in c["scales"].items()))
    print("\niterate histories (published count; seed; first-iterate decisions: exact value, largest enclosure; preceding iterate):")
    for h in histories:
        print(f"  {h['control']} {h['case']}: n = {h['iterations']}, seed {h['seed']}, preceding {h['previous']}")
        for sid, what, value, enclosure in h["decisions"]:
            print(f"      {sid} {what}: {value:.6e} against {enclosure:.3e} ({abs(value) / enclosure:.2e}x)")
        if h["previous"] is not None:
            print("      delta enclosure/S_f: " + ", ".join(f"{d} {h[d]:.3e}" for d in DELTA_ROWS))
    print("\ncoupled friction (admissible final states): c, g, 1+c*g, phi, enclosure of phi")
    for c in coupled:
        print(f"  {c['control']} {c['case']}: c {c['c']:+.3f} g {c['g']:+.6f} 1+c*g {c['1+c*g']:.6f} phi {c['phi_N']:.6f} N  E_phi {c['E_phi_N']:.3e} N")
    print("\nactive-set margins (final states, PF leaves): margin against its enclosure")
    for label, case_id, sid, what, margin, enclosure in margin_rows:
        ratio = margin / enclosure if enclosure > 0 else math.inf
        print(f"  {label} {case_id} {sid} {what}: {margin:.6e} vs {enclosure:.3e} ({'weak, zero margin' if margin == 0 else f'{ratio:.2e}x'})")


if __name__ == "__main__":
    main()
