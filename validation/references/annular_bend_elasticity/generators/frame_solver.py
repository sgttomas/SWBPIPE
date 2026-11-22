#!/usr/bin/env python3
"""A three-dimensional linear frame of straight members and circular arcs, for comparing the formulations
of a realized bend. It imports nothing but the Python standard library and numpy, and no product code.

Library use

    import frame_solver as fs
    result = fs.solve(model, straight="timoshenko", arc="thin_shear")

`model` is a model document of this repository's benchmark shape (validation/caepipe_benchmark/models/*.json),
either the whole document or the object under its "model" key (`load_model` reads either from a file). SI
units throughout (m, N, Pa, degC); the reader refuses any other unit it meets. `solve` returns, per load case,
the support reactions, the node displacements and rotations, and the member end actions (see Output).

Options of `solve` (keyword only; the command line takes the same names with dashes)

  straight   theory of the straight members:
               "model"       (default) the document's model_settings.shear_model; absent means Euler-Bernoulli
               "eb"          Euler-Bernoulli
               "timoshenko"  Timoshenko, with the energy-matched shear factor of an annulus (`kappa_annulus`,
                             DEL-05-03's factor in its stated operation order)
  arc        formulation of each realized arc, by name (FORMULATIONS):
               "thin"        the end flexibility of a thin circular curved beam: bending multiplied by the
                             in-plane and out-of-plane flexibility factors, axial and torsional flexibility.
                             This is DEL-03-03's method (AC-002's closed form, here by quadrature)
               "thin_shear"  "thin" with transverse shear added, unscaled by the flexibility factors
               "candidate"   "thin_shear" with the curved bar's coupling of axial force with in-plane moment,
                             1/(E A R), added unscaled; the factor keeps its meaning as the bend's pure
                             in-plane bending flexibility relative to the thin curved beam
             `arc_shear` and `arc_curved_bar` set the same parts separately and override `arc`:
  arc_shear       "off" | "unscaled" | "scaled" (in-plane shear times k_in, out-of-plane times k_out)
  arc_curved_bar  "off" | "a" | "b" | "c" (curved-bar terms of a section whose planes stay plane, with the
                  neutral-axis offset of the annulus in closed form):
                    "a" the factor multiplies the curved bar's bending term, coupling added
                    "b" the factor multiplies the thin bending term, the curvature correction and the
                        coupling added unscaled
                    "c" the factor multiplies the thin bending term and only the coupling is added
                  With factors fitted to one pure-bending flexibility the three give one section matrix.
  k_in, k_out     the bend's in-plane and out-of-plane factors, overriding the document's
                  flexibility_factor_user_value (which is used for both otherwise)
  compliance      a 6 x 6 section compliance per unit arc length in the local basis (N, V_r, V_n, T, M_r,
                  M_n), replacing the arc's law: used for the elasticity reference (torus_elasticity.py)
  arc_polyline    replace each realized arc by this many straight members of the straight theory, with
                  nodes on the arc (0, the default, keeps the arc)
  anchor_stiffness  six stiffnesses (kx, ky, kz in N/m; krx, kry, krz in N*m/rad, global axes) that make
                  every anchor elastic instead of rigid
  quad            Gauss points along an arc (default 48)

Supported models (anything else is refused with ValueError, never solved approximately)
  - pressure-free models only: a model with a pressure_contract, a load case with any member naming pressure (such
    as its pressure regions) or a condition_use entry naming pressure, a load category naming pressure, or any pressure setting (such as pressure
    elongation) in model_settings is refused;
  - every material states shear_modulus explicitly (G is never derived from poisson_ratio);
  - supports: family "anchor", "rigid", "guide", "stop", "restraint" or none, each holding its listed
    restraints rigidly; "spring" (one stiffness on one dof); or any support with "stiffness6". Other families,
    such as "line_stop", are refused;
  - load categories: concentrated_force, concentrated_moment, weight, uniform_load and thermal. Others, such as
    imposed_displacement, are refused;
  - an arc's section compliance must be positive definite on its active terms: the factors must be finite and
    positive, and with the curved bar's coupling k_in must exceed the bound the coupling sets (for "c",
    (r_g / R)^2); a compliance given must be finite, exactly symmetric and 6 x 6, with every zero-diagonal row
    and column entirely zero.

Output: {"units", "straight_theory", "arc", "cases": {case id: {"reactions": {support id: {"node", "Fx", "Fy",
"Fz", "Mx", "My", "Mz"}}, "displacements": {node id: {"ux", "uy", "uz", "rx", "ry", "rz"}},
"member_end_actions": {member id: {"end_i", "end_j"}}}}}. Reactions are the support's action on the pipe in
global axes, about the support node. End actions are the action on the member at its end node, global axes,
moments about that node.

Conventions. A straight member's local x runs from "from" to "to". A realized arc spans its pipe's two nodes;
its plane is spanned by the chord and the pipe's y_reference, and it bows toward the positive y_reference
side: centre C = mid-chord - p sqrt(R^2 - c^2/4), p the unit in-plane normal to the chord along y_reference.
Arc frame at angle theta: e_r radial outward, t = n x e_r the tangent toward the far node, n the plane normal.
Section resultants (positive face, normal t): N, V_r, V_n, T (about t), M_r (about e_r, out-of-plane
bending), M_n (about n, in-plane). End flexibility by Castigliano: F = int B^T C B ds; the 12 x 12 stiffness
follows from the inverse of the tip flexibility. Thermal: free strain alpha dT along the member, whose free
end displacement is alpha dT times its chord, with no rotation, for a straight member and an arc alike.
Weight and uniform load: a force per unit length (of member, of arc) in a global direction.

Command line: python3 frame_solver.py MODEL.json [--straight ...] [--arc ...] ... > result.json
"""
from __future__ import annotations

import argparse
import json
import math
import sys

import numpy as np

UNITS = {"force": "N", "moment": "N*m", "displacement": "m", "rotation": "rad"}
DOFS = ("UX", "UY", "UZ", "RX", "RY", "RZ")
FORCES = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
FORMULATIONS = {"thin": ("off", "off"), "thin_shear": ("unscaled", "off"), "candidate": ("unscaled", "c")}
STRAIGHT = ("model", "eb", "timoshenko")
ARC_SHEAR = ("off", "unscaled", "scaled")
ARC_CURVED_BAR = ("off", "a", "b", "c")


# ---------------------------------------------------------------- section
def kappa_annulus(ri: float, ro: float, nu: float) -> float:
    """DEL-05-03's energy-matched annular shear factor, in its stated operation order."""
    m = ri / ro
    m2 = m * m
    a = 1 + m2
    b = 1 - m2
    n1 = 1 + nu
    c = (8 * nu + 14) * nu + 7
    D = c * (b * b) + 48 * (m2 * (n1 * n1))
    return (6 * (n1 * n1)) * (a * a) / D


class Section:
    """A circular annulus of outside diameter D and wall t, with moduli E and G (nu = E / 2G - 1)."""

    def __init__(self, D: float, t: float, E: float, G: float):
        self.D, self.t, self.E, self.G = D, t, E, G
        self.ro = D / 2
        self.ri = self.ro - t
        self.A = math.pi * (self.ro ** 2 - self.ri ** 2)
        self.I = math.pi * (self.ro ** 4 - self.ri ** 4) / 4
        self.J = 2 * self.I
        self.nu = E / (2 * G) - 1
        self.kappa = kappa_annulus(self.ri, self.ro, self.nu)
        self.As = self.kappa * self.A

    def neutral_offset(self, R: float) -> float:
        """e = R - r_n for an annulus whose centroid is at radius R (plane sections), R > r_o."""
        if R <= self.ro:
            raise ValueError("curved-bar law needs R > r_o")
        rn = (math.sqrt(R * R - self.ro ** 2) + math.sqrt(R * R - self.ri ** 2)) / 2
        return R - rn


def skew(d):
    return np.array([[0, -d[2], d[1]], [d[2], 0, -d[0]], [-d[1], d[0], 0]], dtype=float)


def unit(v):
    v = np.asarray(v, dtype=float)
    return v / np.linalg.norm(v)


def H_of(d):
    """P_i = H P_j for a member with no load between its ends; d = x_j - x_i."""
    H = np.zeros((6, 6))
    H[:3, :3] = -np.eye(3)
    H[3:, 3:] = -np.eye(3)
    H[3:, :3] = -skew(d)
    return H


def assemble_from_tip(Kjj, d):
    H = H_of(d)
    K = np.zeros((12, 12))
    K[:6, :6] = H @ Kjj @ H.T
    K[:6, 6:] = H @ Kjj
    K[6:, :6] = Kjj @ H.T
    K[6:, 6:] = Kjj
    return K


# ---------------------------------------------------------------- members
class Straight:
    """Two-node straight pipe, Euler-Bernoulli or Timoshenko, closed-form cantilever flexibility."""

    def __init__(self, xi, xj, sec: Section, timoshenko: bool, yref=None):
        self.xi, self.xj = np.asarray(xi, float), np.asarray(xj, float)
        self.sec, self.timo = sec, timoshenko
        d = self.xj - self.xi
        self.L = float(np.linalg.norm(d))
        ex = d / self.L
        y = np.asarray(yref if yref is not None else [0, 0, 1], float)
        y = y - (y @ ex) * ex
        if np.linalg.norm(y) < 1e-9:
            y = np.array([0.0, 1.0, 0.0]) if abs(ex[1]) < 0.9 else np.array([1.0, 0.0, 0.0])
            y = y - (y @ ex) * ex
        ey = unit(y)
        ez = np.cross(ex, ey)
        self.Lam = np.vstack([ex, ey, ez])  # rows: local axes in global
        self.d = d

    def section_compliance(self):
        s = self.sec
        cs = 1 / (s.G * s.As) if self.timo else 0.0
        return np.diag([1 / (s.E * s.A), cs, cs, 1 / (s.G * s.J), 1 / (s.E * s.I), 1 / (s.E * s.I)])

    def tip_flexibility_local(self):
        s, L = self.sec, self.L
        EI, GAs = s.E * s.I, s.G * s.As
        sh = L / GAs if self.timo else 0.0
        F = np.zeros((6, 6))
        F[0, 0] = L / (s.E * s.A)
        F[1, 1] = L ** 3 / (3 * EI) + sh
        F[2, 2] = L ** 3 / (3 * EI) + sh
        F[3, 3] = L / (s.G * s.J)
        F[4, 4] = L / EI
        F[5, 5] = L / EI
        F[1, 5] = F[5, 1] = L ** 2 / (2 * EI)
        F[2, 4] = F[4, 2] = -L ** 2 / (2 * EI)
        return F

    def tip_flexibility(self):
        T = np.zeros((6, 6))
        T[:3, :3] = self.Lam
        T[3:, 3:] = self.Lam
        return T.T @ self.tip_flexibility_local() @ T

    def stiffness(self):
        return assemble_from_tip(np.linalg.inv(self.tip_flexibility()), self.d)

    def load_tip_deflection(self, w, nq=8):
        """Free-tip generalized displacement at j (i clamped) under a uniform global load w per length."""
        g, wt = np.polynomial.legendre.leggauss(nq)
        s_ = 0.5 * self.L * (g + 1)
        C = self.section_compliance()
        ex = self.Lam[0]
        delta = np.zeros(6)
        for s, a in zip(s_, wt * 0.5 * self.L):
            r = self.xj - (self.xi + s * ex)
            B = np.zeros((6, 6))
            B[:3, :3] = self.Lam
            B[3:, 3:] = self.Lam
            B[3:, :3] = self.Lam @ skew(r)
            F = w * (self.L - s)
            M = np.cross(ex * (self.L - s) ** 2 / 2, w)
            S = np.concatenate([self.Lam @ F, self.Lam @ M])
            delta += a * (B.T @ (C @ S))
        return delta

    def load_resultant(self, w):
        W = w * self.L
        Mi = np.cross(self.d / 2, W)
        return W, Mi

    def length(self):
        return self.L


class Arc:
    """Circular arc between two nodes, formed from its end flexibility by Castigliano (quadrature)."""

    def __init__(self, xi, xj, R, yref, sec: Section, k_in=1.0, k_out=1.0, shear="off",
                 curved="off", compliance=None, nq=48):
        self.xi, self.xj = np.asarray(xi, float), np.asarray(xj, float)
        self.R, self.sec = float(R), sec
        self.k_in, self.k_out, self.shear, self.curved = k_in, k_out, shear, curved
        self.C_user = None if compliance is None else np.asarray(compliance, float)
        c = self.xj - self.xi
        cl = float(np.linalg.norm(c))
        if R < cl / 2 * (1 - 1e-12):
            raise ValueError("bend radius cannot span the chord")
        cu = c / cl
        y = np.asarray(yref, float)
        p = y - (y @ cu) * cu
        p = unit(p)
        mid = 0.5 * (self.xi + self.xj)
        self.Cc = mid - p * math.sqrt(max(R * R - cl * cl / 4, 0.0))
        self.phi = 2 * math.asin(min(cl / (2 * R), 1.0))
        self.e1 = unit(self.xi - self.Cc)
        # rotation from e1 toward the arc's midpoint direction p (also defined at 180 degrees)
        self.n = unit(np.cross(self.e1, p))
        self.e2 = np.cross(self.n, self.e1)
        self.d = c
        g, wt = np.polynomial.legendre.leggauss(nq)
        self.th = 0.5 * self.phi * (g + 1)
        self.wt = wt * 0.5 * self.phi
        self._admissible()

    def _admissible(self):
        """Refuses a section compliance that is not positive definite on its active terms. For the arc's own law
        the active terms are the formulation's (all six with shear, else N, T, M_r and M_n), and both factors must be
        finite and positive. A compliance given must be a finite, exactly symmetric 6 x 6 matrix whose zero-diagonal
        rows and columns are entirely zero, positive definite on the rest."""
        if self.C_user is not None:
            C = self.C_user
            if C.shape != (6, 6) or not np.all(np.isfinite(C)) or not np.array_equal(C, C.T):
                raise ValueError("the section compliance given must be a finite, exactly symmetric 6 x 6 matrix")
            act = [i for i in range(6) if C[i, i] != 0.0]
            rest = [i for i in range(6) if i not in act]
            if not act or np.any(C[rest, :] != 0.0) or np.any(C[:, rest] != 0.0):
                raise ValueError("the section compliance given has a zero diagonal term with a nonzero coupling, or "
                                 "no nonzero term")
            try:
                np.linalg.cholesky(C[np.ix_(act, act)])
            except np.linalg.LinAlgError:
                raise ValueError("the section compliance given is not positive definite") from None
            return
        if not (math.isfinite(self.k_in) and math.isfinite(self.k_out) and self.k_in > 0 and self.k_out > 0):
            raise ValueError(f"flexibility factors must be finite and positive: k_in = {self.k_in:g}, "
                             f"k_out = {self.k_out:g}")
        C = self.section_compliance()
        act = [0, 3, 4, 5] if self.shear == "off" else list(range(6))
        sub = C[np.ix_(act, act)]
        if not np.all(np.isfinite(sub)):
            raise ValueError(f"arc (shear {self.shear}, curved bar {self.curved}): section compliance not finite")
        try:
            np.linalg.cholesky(sub)
            return
        except np.linalg.LinAlgError:
            pass
        # only C[5, 5] depends on k_in, with slope s; positive definiteness needs C55 > C05^2 / C00
        s = self.sec
        slope = 1 / (s.E * s.A * s.neutral_offset(self.R) * self.R) if self.curved == "a" else 1 / (s.E * s.I)
        bound = self.k_in - (C[5, 5] - C[0, 5] ** 2 / C[0, 0]) / slope
        named = " = (r_g/R)^2" if self.curved == "c" else ""
        raise ValueError(f"arc (shear {self.shear}, curved bar {self.curved}) at R = {self.R:g}: k_in = "
                         f"{self.k_in:g} must exceed {bound:.6g}{named} for a positive definite section compliance")

    def pos(self, th):
        return self.Cc + self.R * (math.cos(th) * self.e1 + math.sin(th) * self.e2)

    def frame(self, th):
        er = math.cos(th) * self.e1 + math.sin(th) * self.e2
        t = np.cross(self.n, er)
        return np.vstack([t, er, self.n])

    def section_compliance(self):
        if self.C_user is not None:
            return self.C_user
        s, R = self.sec, self.R
        E, G, A, I, J, As = s.E, s.G, s.A, s.I, s.J, s.As
        C = np.zeros((6, 6))
        C[0, 0] = 1 / (E * A)
        if self.shear != "off":
            si = self.k_in if self.shear == "scaled" else 1.0
            so = self.k_out if self.shear == "scaled" else 1.0
            C[1, 1] = si / (G * As)
            C[2, 2] = so / (G * As)
        C[3, 3] = 1 / (G * J)
        C[4, 4] = self.k_out / (E * I)
        if self.curved == "off":
            C[5, 5] = self.k_in / (E * I)
        elif self.curved == "c":
            C[5, 5] = self.k_in / (E * I)
            C[0, 5] = C[5, 0] = 1 / (E * A * R)
        else:
            e = s.neutral_offset(R)
            winkler = 1 / (E * A * e * R)
            if self.curved == "a":
                C[5, 5] = self.k_in * winkler
            else:
                C[5, 5] = self.k_in / (E * I) + (winkler - 1 / (E * I))
            C[0, 5] = C[5, 0] = 1 / (E * A * R)
        return C

    def B(self, th):
        Lam = self.frame(th)
        r = self.xj - self.pos(th)
        B = np.zeros((6, 6))
        B[:3, :3] = Lam
        B[3:, 3:] = Lam
        B[3:, :3] = Lam @ skew(r)
        return B

    def tip_flexibility(self):
        C = self.section_compliance()
        F = np.zeros((6, 6))
        for th, a in zip(self.th, self.wt):
            B = self.B(th)
            F += a * self.R * (B.T @ C @ B)
        return 0.5 * (F + F.T)

    def stiffness(self):
        return assemble_from_tip(np.linalg.inv(self.tip_flexibility()), self.d)

    def _Xint(self, th):
        """int_th^phi x(th') R dth'."""
        p = self.phi
        return self.R * (self.Cc * (p - th) + self.R * ((math.sin(p) - math.sin(th)) * self.e1
                                                        + (math.cos(th) - math.cos(p)) * self.e2))

    def load_tip_deflection(self, w):
        C = self.section_compliance()
        delta = np.zeros(6)
        for th, a in zip(self.th, self.wt):
            Lam = self.frame(th)
            seg = self.R * (self.phi - th)
            F = w * seg
            M = np.cross(self._Xint(th) - self.pos(th) * seg, w)
            S = np.concatenate([Lam @ F, Lam @ M])
            delta += a * self.R * (self.B(th).T @ (C @ S))
        return delta

    def load_resultant(self, w):
        W = w * self.R * self.phi
        Mi = np.cross(self._Xint(0.0) - self.xi * self.R * self.phi, w)
        return W, Mi

    def length(self):
        return self.R * self.phi


def fixed_end_actions(mem, Kjj, delta_j, W=None, Mi=None):
    """End actions on the member (global, moments about each end node) with both ends held."""
    fj = -Kjj @ delta_j
    fi = np.zeros(6)
    W = np.zeros(3) if W is None else W
    Mi = np.zeros(3) if Mi is None else Mi
    fi[:3] = -W - fj[:3]
    fi[3:] = -Mi - fj[3:] - np.cross(mem.d, fj[:3])
    return np.concatenate([fi, fj])


# ---------------------------------------------------------------- options
class Options:
    """The options of `solve`, validated (see the module docstring)."""

    def __init__(self, straight="model", arc="thin", arc_shear=None, arc_curved_bar=None, k_in=None, k_out=None,
                 compliance=None, arc_polyline=0, anchor_stiffness=None, quad=48):
        if straight not in STRAIGHT:
            raise ValueError(f"straight {straight!r}: one of {STRAIGHT}")
        if arc not in FORMULATIONS:
            raise ValueError(f"arc {arc!r}: one of {tuple(FORMULATIONS)}")
        shear, curved = FORMULATIONS[arc]
        self.straight = straight
        self.arc = arc
        self.arc_shear = shear if arc_shear is None else arc_shear
        self.arc_curved_bar = curved if arc_curved_bar is None else arc_curved_bar
        if self.arc_shear not in ARC_SHEAR or self.arc_curved_bar not in ARC_CURVED_BAR:
            raise ValueError(f"arc_shear {self.arc_shear!r} or arc_curved_bar {self.arc_curved_bar!r} not known")
        self.k_in, self.k_out = k_in, k_out
        self.compliance = None if compliance is None else np.asarray(compliance, float).tolist()
        self.arc_polyline = int(arc_polyline)
        if anchor_stiffness is not None:
            anchor_stiffness = [float(v) for v in anchor_stiffness]
            if len(anchor_stiffness) != 6:
                raise ValueError("anchor_stiffness needs six values")
        self.anchor_stiffness = anchor_stiffness
        self.quad = int(quad)

    def formulation(self):
        named = [n for n, v in FORMULATIONS.items() if v == (self.arc_shear, self.arc_curved_bar)]
        return named[0] if named else f"shear {self.arc_shear}, curved bar {self.arc_curved_bar}"


# ---------------------------------------------------------------- model reading
def q(obj, unit_expected):
    if obj is None:
        return None
    u = obj.get("unit")
    if u != unit_expected:
        raise ValueError(f"unit {u!r} where {unit_expected!r} expected")
    return float(obj["value"])


DIRS = {"global_x": np.array([1.0, 0, 0]), "global_y": np.array([0, 1.0, 0]), "global_z": np.array([0, 0, 1.0])}


LOAD_CATEGORIES = ("concentrated_force", "concentrated_moment", "weight", "uniform_load", "thermal")
RIGID_FAMILIES = (None, "anchor", "rigid", "guide", "stop", "restraint")


def refuse_unsupported(model):
    """Raises ValueError for a model outside the supported subset (see the module docstring)."""
    if "pressure_contract" in model:
        raise ValueError("pressure_contract: pressure is not supported; this solver takes pressure-free models only")
    for k in (model.get("model_settings") or {}):
        if "pressure" in k:
            raise ValueError(f"model_settings.{k}: pressure settings are not supported")
    for lc in model["load_cases"]:
        for k in lc:
            if "pressure" in k:
                raise ValueError(f"load case {lc['id']}: {k}: pressure is not supported")
        for k in (lc.get("condition_use") or {}):
            if "pressure" in k:
                raise ValueError(f"load case {lc['id']}: condition_use.{k}: pressure is not supported")
        for ld in lc["primitive_loads"]:
            if "pressure" in ld["category"] or ld["category"] not in LOAD_CATEGORIES:
                raise ValueError(f"load case {lc['id']}: load category {ld['category']!r} not supported")
    for m in model["materials"]:
        if m.get("shear_modulus") is None:
            raise ValueError(f"material {m['id']}: shear_modulus must be given (G is not derived from poisson_ratio)")
    for s in model.get("supports", []):
        if s.get("family") not in (*RIGID_FAMILIES, "spring") and "stiffness6" not in s:
            raise ValueError(f"support {s['id']}: family {s.get('family')!r} not supported")


def build(model, o: Options):
    refuse_unsupported(model)
    nodes = {n["id"]: np.array([n["position"]["x"], n["position"]["y"], n["position"]["z"]], float)
             for n in model["nodes"]}
    mats = {m["id"]: m for m in model["materials"]}
    settings = model.get("model_settings") or {}
    if o.straight == "model":
        timo = settings.get("shear_model") == "timoshenko_annulus_energy_v1"
    else:
        timo = o.straight == "timoshenko"
    bends = {}
    for c in model.get("components", []):
        if c.get("kind") in ("bend", "elbow") and (c.get("mechanics_interface") or {}).get(
                "solver_consumption") == "curved_bend_macro_element":
            g = c["geometry"]
            k = q(c["modifiers"]["flexibility_factor_user_value"], "none")
            bends[g["bend_pipe_ref"]] = {"R": q(g["bend_radius"], "m"), "k": k}
    node_ids = list(nodes)
    members = []  # (id, member, from, to, material)
    for p in model["pipe_segments"]:
        m = mats[p["material"]]
        E = q(m["elastic_modulus"], "Pa")
        G = q(m["shear_modulus"], "Pa")
        sec = Section(q(p["section"]["outside_diameter"], "m"), q(p["section"]["wall_thickness"], "m"), E, G)
        yref = [p["y_reference"][a] for a in "xyz"] if p.get("y_reference") else None
        xi, xj = nodes[p["from"]], nodes[p["to"]]
        if p["id"] in bends:
            b = bends[p["id"]]
            kin = o.k_in if o.k_in is not None else b["k"]
            kout = o.k_out if o.k_out is not None else b["k"]
            arc = Arc(xi, xj, b["R"], yref, sec, kin, kout, o.arc_shear, o.arc_curved_bar, o.compliance, o.quad)
            if o.arc_polyline:
                N = o.arc_polyline
                prev = p["from"]
                for s in range(1, N + 1):
                    if s < N:
                        nid = f"{p['id']}#poly{s}"
                        nodes[nid] = arc.pos(arc.phi * s / N)
                        node_ids.append(nid)
                    else:
                        nid = p["to"]
                    st = Straight(nodes[prev], nodes[nid], sec, timo, arc.n)
                    members.append((f"{p['id']}#seg{s}", st, prev, nid, m))
                    prev = nid
                continue
            members.append((p["id"], arc, p["from"], p["to"], m))
        else:
            members.append((p["id"], Straight(xi, xj, sec, timo, yref), p["from"], p["to"], m))
    return nodes, node_ids, members, timo


def solve(model, **options):
    """Solves every load case of `model` (see the module docstring for the options and the output)."""
    o = Options(**options)
    if "model" in model and "nodes" in model["model"]:
        model = model["model"]
    nodes, node_ids, members, timo = build(model, o)
    idx = {n: i for i, n in enumerate(node_ids)}
    ndof = 6 * len(node_ids)
    K = np.zeros((ndof, ndof))
    Ke = {}
    for mid, mem, a, b, _ in members:
        k = mem.stiffness()
        Ke[mid] = k
        dof = list(range(6 * idx[a], 6 * idx[a] + 6)) + list(range(6 * idx[b], 6 * idx[b] + 6))
        K[np.ix_(dof, dof)] += k
    fixed = {}
    springs = []  # (support id, dof index, k)
    for s in model.get("supports", []):
        base = 6 * idx[s["node"]]
        fam = s.get("family")
        if fam == "spring":
            st = s["stiffness"]
            j = DOFS.index(st["dof"])
            springs.append((s["id"], base + j, q(st["value"], "N/m" if j < 3 else "N*m/rad")))
        elif "stiffness6" in s:
            for j, kv in enumerate(s["stiffness6"]):
                springs.append((s["id"], base + j, float(kv)))
        elif fam == "anchor" and o.anchor_stiffness is not None:
            for j in range(6):
                springs.append((s["id"], base + j, o.anchor_stiffness[j]))
        else:
            if fam not in RIGID_FAMILIES:
                raise ValueError(f"support family {fam!r} not handled")
            for r in s["restraints"]:
                fixed.setdefault(base + DOFS.index(r), []).append(s["id"])
    Ks = K.copy()
    for _, j, kv in springs:
        Ks[j, j] += kv
    free = [i for i in range(ndof) if i not in fixed]
    mem_of = {m[0]: m for m in members}
    node_of = {s["id"]: s["node"] for s in model.get("supports", [])}
    out = {"units": UNITS, "straight_theory": "timoshenko_annulus_energy" if timo else "euler_bernoulli",
           "arc": {"formulation": o.formulation(), "shear": o.arc_shear, "curved_bar": o.arc_curved_bar,
                   "polyline": o.arc_polyline, "compliance": o.compliance is not None},
           "cases": {}}
    for lc in model["load_cases"]:
        P = np.zeros(ndof)
        fea = {mid: np.zeros(12) for mid in Ke}
        for ld in lc["primitive_loads"]:
            cat = ld["category"]
            tgt = ld["target"]
            if cat in ("concentrated_force", "concentrated_moment"):
                v = q(ld["magnitude"], "N" if cat == "concentrated_force" else "N*m")
                base = 6 * idx[tgt["node"]] + (0 if cat == "concentrated_force" else 3)
                P[base:base + 3] += v * DIRS[ld["direction"]]
            elif cat in ("weight", "uniform_load"):
                w = q(ld["magnitude"], "N/m") * DIRS[ld["direction"]]
                for mid in [m for m in mem_of if m == tgt["pipe"] or m.startswith(tgt["pipe"] + "#seg")]:
                    _, mem, a, b, _ = mem_of[mid]
                    W, Mi = mem.load_resultant(w)
                    fea[mid] += fixed_end_actions(mem, Ke[mid][6:, 6:], mem.load_tip_deflection(w), W, Mi)
            elif cat == "thermal":
                dT = q(ld["magnitude"], "degC")
                for mid in [m for m in mem_of if m == tgt["pipe"] or m.startswith(tgt["pipe"] + "#seg")]:
                    _, mem, a, b, mat = mem_of[mid]
                    alpha = q(mat["thermal_expansion_coefficient"], "1/degC")
                    delta = np.concatenate([alpha * dT * mem.d, np.zeros(3)])
                    fea[mid] += fixed_end_actions(mem, Ke[mid][6:, 6:], delta)
            else:
                raise ValueError(f"load category {cat!r} not handled")
        for mid, f in fea.items():
            _, mem, a, b, _ = mem_of[mid]
            dof = list(range(6 * idx[a], 6 * idx[a] + 6)) + list(range(6 * idx[b], 6 * idx[b] + 6))
            P[dof] -= f
        u = np.zeros(ndof)
        u[free] = np.linalg.solve(Ks[np.ix_(free, free)], P[free])
        Rfull = Ks @ u - P
        reac = {}
        for j, sids in fixed.items():
            sid = sids[0]
            reac.setdefault(sid, {"node": node_of[sid], **{c: 0.0 for c in FORCES}})
            reac[sid][FORCES[j % 6]] += float(Rfull[j])
        for sid, j, kv in springs:
            reac.setdefault(sid, {"node": node_of[sid], **{c: 0.0 for c in FORCES}})
            reac[sid][FORCES[j % 6]] += float(-kv * u[j])
        disp = {n: dict(zip(("ux", "uy", "uz", "rx", "ry", "rz"), map(float, u[6 * i:6 * i + 6])))
                for n, i in idx.items()}
        ends = {}
        for mid, mem, a, b, _ in members:
            dof = list(range(6 * idx[a], 6 * idx[a] + 6)) + list(range(6 * idx[b], 6 * idx[b] + 6))
            f = Ke[mid] @ u[dof] + fea[mid]
            ends[mid] = {"end_i": {"node": a, **dict(zip(FORCES, map(float, f[:6])))},
                         "end_j": {"node": b, **dict(zip(FORCES, map(float, f[6:])))}}
        out["cases"][lc["id"]] = {"reactions": reac, "displacements": disp, "member_end_actions": ends}
    return out


def load_model(path):
    with open(path, encoding="utf-8") as fh:
        doc = json.load(fh)
    return doc["model"] if "model" in doc and "nodes" in doc["model"] else doc


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("model")
    ap.add_argument("--straight", choices=STRAIGHT, default="model")
    ap.add_argument("--arc", choices=tuple(FORMULATIONS), default="thin")
    ap.add_argument("--arc-shear", choices=ARC_SHEAR)
    ap.add_argument("--arc-curved-bar", choices=ARC_CURVED_BAR)
    ap.add_argument("--k-in", type=float)
    ap.add_argument("--k-out", type=float)
    ap.add_argument("--compliance", help='JSON file {"C": 6 x 6}')
    ap.add_argument("--arc-polyline", type=int, default=0)
    ap.add_argument("--anchor-stiffness", help="kx,ky,kz,krx,kry,krz")
    ap.add_argument("--quad", type=int, default=48)
    a = ap.parse_args(argv)
    compliance = None
    if a.compliance:
        with open(a.compliance, encoding="utf-8") as fh:
            compliance = json.load(fh)["C"]
    anchors = [float(v) for v in a.anchor_stiffness.split(",")] if a.anchor_stiffness else None
    result = solve(load_model(a.model), straight=a.straight, arc=a.arc, arc_shear=a.arc_shear,
                   arc_curved_bar=a.arc_curved_bar, k_in=a.k_in, k_out=a.k_out, compliance=compliance,
                   arc_polyline=a.arc_polyline, anchor_stiffness=anchors, quad=a.quad)
    json.dump(result, sys.stdout, indent=1)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
