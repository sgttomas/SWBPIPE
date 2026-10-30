"""M3-T judged-set generator (standard-library Python only; imports and reads no product code).

Builds invented straight-member exact-route models (model document 0.8.0, the current version since
main retired 0.5.0; `3.0.0/exact_pressure_v3`; re-authored from 0.5.0 by the version alone), each in an Euler-Bernoulli and a Timoshenko twin,
places each topology at the origin, at a UTM-scale offset and skewed at a
UTM-scale offset, adds nonzero mill tolerance and corrosion allowance on a
subset, and freezes the exact expected rows from kd5t_engine.

Usage (from this directory; heavy modes run under the host's slot wrapper):
  python3 -I -B kd5t_generate.py full <out_dir> [workers]   full set: set.jsonl, controls.jsonl, manifest.json
  python3 -I -B kd5t_generate.py freeze [workers]           rewrite frozen/kd5t_frozen_subset.jsonl
  python3 -I -B kd5t_generate.py check [workers]            engine self-check; regenerate the frozen subset in
                                                            memory and compare it byte for byte; compare self-test;
                                                            verify SHA256SUMS
  python3 -I -B kd5t_generate.py split <set.jsonl> <dir> <hv|pending|stiffness_control|all> [per_chunk]
Section variants per design: zero (every design); and, on every third design of a family, both mill
tolerance and allowance ("mill"), mill tolerance only ("cm") and allowance only ("ca").
Each set line: {"tag", "meta", "ref", "request"}. `ref.cases[case].rows` are
[kind, entity_ref, location, component, section_basis, value] with values as
45-significant-digit decimal strings in published units; `ref.cases[case].maxima`
are [pipe, section_basis, value_Pa]; `ref.sections` are the D-5 bases.
"""

from __future__ import annotations

import hashlib
import json
import math
import os
import sys
from decimal import Decimal as D

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import kd5t_engine as E  # noqa: E402

PROV = "invented M3-T judged-set model; not library, catalog, manufacturer or code-rule data"
FROZEN = os.path.join(HERE, "frozen", "kd5t_frozen_subset.jsonl")
DIGITS = 45

# Invented sections (OD, wall) in m and materials (E Pa, nu, alpha 1/degC); not catalog data.
SECTIONS = {"S1": (0.0221, 0.0029), "S2": (0.0611, 0.0041), "S3": (0.1702, 0.0072), "S4": (0.3301, 0.0098),
            "S5": (0.6103, 0.0131), "S6": (0.2, 0.021)}
MATERIALS = {"M1": (2.0e11, 0.3, 1.2e-5), "M2": (1.93e11, 0.27, 1.17e-5), "M3": (2.07e11, 0.29, 1.25e-5)}
RHO_S, RHO_C, G_ACC = 7850, 998, D("9.80665")
OFFSETS = {"utm": (431907.3125, 7312250.6875, 218.25), "utmskew": (5000000.37, 7300000.13, 1200000.5)}
SKEW_AXIS, SKEW_ANGLE = (0.3, -0.5, 0.81), 0.63


def q(v, u):
    return {"value": v, "unit": u}


def rotation_matrix():
    ax = [c / math.sqrt(sum(a * a for a in SKEW_AXIS)) for c in SKEW_AXIS]
    c, s = math.cos(SKEW_ANGLE), math.sin(SKEW_ANGLE)
    x, y, z = ax
    return [[c + x * x * (1 - c), x * y * (1 - c) - z * s, x * z * (1 - c) + y * s],
            [y * x * (1 - c) + z * s, c + y * y * (1 - c), y * z * (1 - c) - x * s],
            [z * x * (1 - c) - y * s, z * y * (1 - c) + x * s, c + z * z * (1 - c)]]


ROT = rotation_matrix()
# Exactly collinear skew for pressure chains (the product's collinearity guard is 64*eps*span): the design x axis
# maps to (2, 3, 6)/7, nodes sit on the dyadic grid 2^-10 (2, 3, 6) and the offset is dyadic, so every
# coordinate is exact; other vectors use the orthonormal frame [(2,3,6)/7, (-3,2,0)/sqrt(13), u x v].
PGRID = 7 * 2.0 ** -10
PTRIPLE = (2, 3, 6)
POFF = (5000000.375, 7300000.125, 1200000.5)


def _pframe():
    u = [2 / 7, 3 / 7, 6 / 7]
    s = math.sqrt(13.0)
    v = [-3 / s, 2 / s, 0.0]
    w = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]]
    return [[u[r], v[r], w[r]] for r in range(3)]


PROT = _pframe()


def rot(v, skew, frame=None):
    if not skew:
        return [float(a) for a in v]
    R = frame or ROT
    return [R[r][0] * v[0] + R[r][1] * v[1] + R[r][2] * v[2] for r in range(3)]


def sig4(x):
    return float(f"{x:.4g}")


def section_props(sec, mat):
    od, t = SECTIONS[sec]
    E_, nu, _ = MATERIALS[mat]
    s = E.d5_sections(od, t, 0.0, 0.0)
    nom = s["nominal_section"]
    kap = E.kappa_energy(nom["ri"] / s["ro"], E.exact(nu))
    G = E.exact(E_) / (2 * (1 + E.exact(nu)))
    return nom, kap, G, E.exact(E_)


def length_for_phi(sec, mat, phi):
    nom, kap, G, E_ = section_props(sec, mat)
    return float((12 * E_ * nom["I"] / (kap * G * nom["A"] * D(phi))).sqrt())


def phi_of(sec, mat, L):
    nom, kap, G, E_ = section_props(sec, mat)
    return 12 * E_ * nom["I"] / (kap * G * nom["A"] * E.exact(L) ** 2)


# ----------------------------------------------------------------- designs
class Design:
    """A model in its design frame (before placement)."""

    def __init__(self, name, family, sec, mat):
        self.name, self.family, self.sec, self.mat = name, family, sec, mat
        self.nodes, self.members, self.supports = [], [], []
        self.nodal = {}          # node -> [Fx, Fy, Fz, Mx, My, Mz] (design frame)
        self.udl = {}            # member -> [wx, wy, wz] (design frame)
        self.gravity = False     # self-weight (+ contents) along global -z after placement
        self.contents = False
        self.thermal = {}        # member -> dT (degC) or explicit strain (load-state)
        self.regions = []        # [{"members": [...], "p": Pa, "closures": {node: kind}}]
        self.motion = {}         # anchor id -> [ux, uy, uz, rx, ry, rz] (design frame)
        self.load_state = False
        self.factor = 1.0
        self.member_sections = {}  # member -> section name override
        self.notes = []
        self.mill_variant = False
        self.axis_chain = False  # every node on the design x axis (pressure regions)

    def node(self, nid, xyz):
        self.nodes.append((nid, [float(c) for c in xyz]))
        return nid

    def member(self, mid, i, j, yref=(0.0, 0.0, 1.0), sec=None):
        self.members.append({"id": mid, "i": i, "j": j, "yref": yref})
        if sec:
            self.member_sections[mid] = sec
        return mid

    def anchor(self, sid, node, restraints=None):
        self.supports.append({"id": sid, "node": node, "kind": "anchor", "restraints": restraints or E.DOFS[:]})

    def spring(self, sid, node, dof, k):
        self.supports.append({"id": sid, "node": node, "kind": "spring", "dof": dof, "k": float(k)})

    def load(self, node, vec6):
        cur = self.nodal.setdefault(node, [0.0] * 6)
        for k in range(6):
            cur[k] += float(vec6[k])


def chain(d, prefix, points, yrefs=None, sec=None):
    """Members joining consecutive points; returns (node ids, member ids)."""
    ids = []
    for k, p in enumerate(points):
        ids.append(d.node(f"node:{prefix}{k}", p))
    mids = []
    for k in range(len(points) - 1):
        y = yrefs[k] if yrefs else (0.0, 0.0, 1.0)
        mids.append(d.member(f"pipe:{prefix}{k}{k + 1}", ids[k], ids[k + 1], y, sec))
    return ids, mids


def line_points(start, direction, n, Le):
    return [[start[c] + direction[c] * (k * Le) for c in range(3)] for k in range(n + 1)]


def bending_stiffness(sec, mat):
    nom, _, _, E_ = section_props(sec, mat)
    return float(E_ * nom["I"])


def torsion_stiffness(sec, mat):
    nom, _, G, _ = section_props(sec, mat)
    return float(G * nom["J"])


def designs():
    out = []
    # ---- A. cantilevers: root anchored, free tip
    A = [("S2", "M1", 1e-4, 1, "tip"), ("S2", "M1", 1e-4, 2, "sw"), ("S3", "M1", 1e-3, 1, "tip"), ("S3", "M2", 1e-2, 2, "mix"),
         ("S3", "M1", 0.1, 4, "tip+midk"), ("S4", "M1", 0.3, 1, "sw+tipk"), ("S4", "M3", 1.0, 2, "mix"), ("S5", "M1", 3.0, 1, "tip"),
         ("S5", "M2", 5.0, 2, "sw"), ("S5", "M1", 12.0, 1, "mix+tipk"), ("S6", "M1", 0.5, 4, "tip"), ("S6", "M3", 20.0, 1, "tip"),
         ("S5", "M1", 40.0, 2, "tip+midk"), ("S1", "M1", 1e-5, 1, "tip"), ("S1", "M2", 1e-3, 4, "th+tip"), ("S2", "M3", 0.05, 2, "th+tip"),
         ("S4", "M2", 8.0, 4, "sw"), ("S6", "M2", 2.5, 2, "mix"), ("S3", "M3", 0.6, 1, "sw+tipk"), ("S4", "M1", 0.02, 2, "tip"),
         ("S1", "M3", 1e-6, 1, "tip"), ("S2", "M2", 3e-6, 1, "th+tip")]
    for n_, (sec, mat, phi, nsub, var) in enumerate(A):
        d = Design(f"CANT{n_:02d}", "cantilever", sec, mat)
        Le = sig4(length_for_phi(sec, mat, phi * nsub * nsub))  # element phi = phi_member * nsub^2
        ids, mids = chain(d, "N", line_points([0, 0, 0], [1, 0, 0], nsub, Le))
        d.anchor("support:root", ids[0])
        L = nsub * Le
        EI = bending_stiffness(sec, mat)
        scale = EI / L ** 2           # loads giving rotations ~ 1e-3
        if "tip" in var or var == "mix" or "mix" in var:
            d.load(ids[-1], [0.1 * scale * 1e-3 * L, 0.8 * scale * 1e-3, -1.5 * scale * 1e-3, 0.12 * scale * 1e-3 * L, -0.06 * scale * 1e-3 * L, 0.09 * scale * 1e-3 * L])
        if "sw" in var or "mix" in var:
            d.gravity = True
            d.contents = sec in ("S3", "S4", "S5")
        if "mix" in var:
            for m in mids:
                d.udl[m] = [0.0, 0.3 * scale * 1e-3 / L, 0.0]
        if "th" in var:
            for m in mids:
                d.thermal[m] = 80.0
        if "tipk" in var:
            d.spring("support:tipk", ids[-1], "UZ", sig4(3 * EI / L ** 3 * 2.5))
        if "midk" in var:
            d.spring("support:midk", ids[len(ids) // 2], "UY", sig4(48 * EI / L ** 3 * 0.7))
        d.notes.append(f"phi_member_target={phi:g}; nsub={nsub}; variant={var}")
        out.append(d)
    # ---- B. fixed-fixed runs (anchors at both ends)
    B = [("S1", "M1", 1e-6, 2, "sw"), ("S2", "M2", 1e-5, 3, "pt"), ("S3", "M1", 1e-3, 2, "sw+th"), ("S3", "M3", 0.05, 4, "pt+midk"),
         ("S4", "M1", 0.4, 2, "sw+pt"), ("S4", "M2", 2.0, 3, "mot"), ("S5", "M1", 6.0, 2, "pt"), ("S5", "M3", 15.0, 4, "sw+mot"),
         ("S6", "M1", 1.2, 2, "pt+mot"), ("S6", "M2", 0.08, 3, "sw+th"), ("S2", "M1", 3e-4, 4, "mot"), ("S1", "M3", 2e-5, 3, "pt+midk"),
         ("S5", "M2", 60.0, 2, "pt"), ("S3", "M2", 0.25, 3, "sw+th+mot"), ("S4", "M3", 25.0, 2, "sw"), ("S2", "M2", 0.01, 2, "th+mot")]
    for n_, (sec, mat, phi, nsub, var) in enumerate(B):
        d = Design(f"FF{n_:02d}", "fixed_fixed", sec, mat)
        Le = sig4(length_for_phi(sec, mat, phi * nsub * nsub))
        ids, mids = chain(d, "N", line_points([0, 0, 0], [1, 0, 0], nsub, Le))
        d.anchor("support:a", ids[0])
        d.anchor("support:b", ids[-1])
        L = nsub * Le
        EI = bending_stiffness(sec, mat)
        scale = EI / L ** 2
        if "pt" in var:
            for k in range(1, nsub):
                d.load(ids[k], [0.2 * scale * 1e-2, -0.9 * scale * 1e-2, 0.6 * scale * 1e-2, 0.05 * scale * 1e-2 * L, 0.0, -0.03 * scale * 1e-2 * L])
        if "sw" in var:
            d.gravity = True
            d.contents = sec in ("S3", "S4", "S5")
        if "th" in var:
            for m in mids:
                d.thermal[m] = 45.0
        if "midk" in var:
            d.spring("support:midk", ids[nsub // 2], "UZ", sig4(192 * EI / L ** 3 * 0.4))
        if "mot" in var:
            d.load_state = True
            d.motion["support:b"] = [L * 2e-4, -L * 1.5e-4, L * 3e-4, 4e-4, -2e-4, 6e-4]
            d.factor = 1.1
        d.notes.append(f"phi_member_target={phi:g}; nsub={nsub}; variant={var}")
        out.append(d)
    # ---- C. L-lines with a sharp corner
    C = [("S3", "M1", 0.01, 2, 1.0, "aa", "th"), ("S3", "M1", 0.01, 2, 0.6, "aa", "sw"), ("S4", "M2", 0.5, 1, 1.5, "aa", "th+sw"),
         ("S4", "M1", 3.0, 2, 0.8, "af", "tip"), ("S5", "M3", 8.0, 1, 1.0, "aa", "th"), ("S2", "M1", 1e-4, 2, 2.0, "af", "sw+tip"),
         ("S6", "M1", 0.3, 2, 1.0, "aa", "th+corner"), ("S5", "M1", 1.0, 3, 0.5, "aa", "sw+mot"), ("S1", "M2", 1e-4, 1, 3.0, "aa", "th"),
         ("S3", "M3", 4.0, 2, 1.2, "af", "tip+k"), ("S4", "M3", 0.05, 3, 0.7, "aa", "th+corner"), ("S6", "M2", 10.0, 1, 2.0, "aa", "sw"),
         ("S2", "M2", 0.002, 2, 1.0, "aa", "mot"), ("S5", "M2", 30.0, 2, 1.0, "af", "tip")]
    for n_, (sec, mat, phi, nsub, ratio, sup, var) in enumerate(C):
        d = Design(f"L{n_:02d}", "l_line", sec, mat)
        Le = sig4(length_for_phi(sec, mat, phi * nsub * nsub))
        Le2 = sig4(Le * ratio)
        ids1, m1 = chain(d, "A", line_points([0, 0, 0], [1, 0, 0], nsub, Le))
        corner = d.nodes[-1][1]
        pts2 = line_points(corner, [0, 1, 0], nsub, Le2)[1:]
        ids2 = []
        for k, p in enumerate(pts2):
            ids2.append(d.node(f"node:B{k + 1}", p))
        m2 = []
        prev = ids1[-1]
        for k, nid in enumerate(ids2):
            m2.append(d.member(f"pipe:B{k}{k + 1}", prev, nid, (0.0, 0.0, 1.0) if k % 2 == 0 else (1.0, 0.0, 0.3)))
            prev = nid
        d.anchor("support:a", ids1[0])
        if sup == "aa":
            d.anchor("support:b", ids2[-1])
        L = nsub * Le
        EI = bending_stiffness(sec, mat)
        scale = EI / L ** 2
        if "th" in var:
            for m in m1 + m2:
                d.thermal[m] = 60.0
        if "sw" in var:
            d.gravity = True
            d.contents = sec in ("S4", "S5")
        if "tip" in var:
            d.load(ids2[-1], [0.4 * scale * 1e-3, 0.2 * scale * 1e-3, -0.9 * scale * 1e-3, 0.05 * scale * 1e-3 * L, 0.1 * scale * 1e-3 * L, 0.0])
        if "corner" in var:
            d.load(ids1[-1], [0.0, 0.0, -2.0 * scale * 1e-2, 0.0, 0.0, 0.0])
        if "k" in var.split("+"):
            d.spring("support:k", ids1[-1], "UZ", sig4(3 * EI / L ** 3 * 5))
        if "mot" in var:
            d.load_state = True
            d.motion["support:a"] = [0.0, L * 1e-4, -L * 2e-4, 0.0, 3e-4, -1e-4]
        d.notes.append(f"phi_member_target={phi:g}; nsub={nsub}; leg_ratio={ratio}; supports={sup}; variant={var}")
        out.append(d)
    # ---- D. U, Z and non-planar 3D lines
    Dd = [("U", "S3", "M1", 0.05, 1, "th"), ("U", "S5", "M2", 2.0, 2, "sw+th"), ("Z", "S4", "M1", 0.2, 1, "th"), ("Z", "S2", "M3", 1e-3, 2, "sw"),
          ("3D", "S3", "M2", 0.5, 1, "th+sw"), ("3D", "S6", "M1", 5.0, 2, "th"), ("3D", "S5", "M3", 0.02, 1, "sw+mot"), ("Z", "S5", "M1", 12.0, 1, "th+sw"),
          ("U", "S1", "M1", 1e-5, 1, "th"), ("3D", "S4", "M2", 1.5, 2, "mot")]
    for n_, (shape, sec, mat, phi, nsub, var) in enumerate(Dd):
        d = Design(f"{shape}{n_:02d}", "multi_leg", sec, mat)
        Le = sig4(length_for_phi(sec, mat, phi * nsub * nsub))
        dirs = {"U": [[1, 0, 0], [0, 1, 0], [-1, 0, 0]], "Z": [[1, 0, 0], [0, 1, 0], [1, 0, 0]], "3D": [[1, 0, 0], [0, 0, 1], [0, 1, 0]]}[shape]
        lens = [Le, sig4(Le * 0.7), sig4(Le * 1.3)]
        pts = [[0.0, 0.0, 0.0]]
        for leg, dv in enumerate(dirs):
            for k in range(nsub):
                p = pts[-1]
                pts.append([p[c] + dv[c] * lens[leg] for c in range(3)])
        yrefs = []
        for leg, dv in enumerate(dirs):
            for k in range(nsub):
                yrefs.append((0.0, 0.0, 1.0) if dv[2] == 0 else (1.0, 0.0, 0.0))
        ids, mids = chain(d, "N", pts, yrefs)
        d.anchor("support:a", ids[0])
        d.anchor("support:b", ids[-1])
        EI = bending_stiffness(sec, mat)
        if "th" in var:
            for m in mids:
                d.thermal[m] = 90.0
        if "sw" in var:
            d.gravity = True
            d.contents = sec in ("S3", "S4", "S5")
        if "mot" in var:
            d.load_state = True
            d.motion["support:b"] = [Le * 1e-4, Le * 2e-4, -Le * 1e-4, 2e-4, 0.0, -3e-4]
        d.notes.append(f"shape={shape}; phi_member_target={phi:g}; nsub={nsub}; variant={var}")
        out.append(d)
    # ---- E. multi-span runs on grounded springs (anchored ends)
    Ee = [("S3", "M1", 0.01, 3, "sw"), ("S3", "M2", 0.2, 4, "sw+th"), ("S4", "M1", 1.0, 3, "sw+lat"), ("S2", "M3", 1e-4, 5, "sw"),
          ("S5", "M1", 4.0, 3, "sw+th"), ("S6", "M3", 0.6, 4, "sw+lat"), ("S1", "M1", 1e-5, 4, "sw+th"), ("S5", "M2", 0.05, 5, "sw"),
          ("S4", "M3", 9.0, 3, "lat+th"), ("S2", "M1", 2e-3, 3, "sw+lat")]
    for n_, (sec, mat, phi, nspan, var) in enumerate(Ee):
        d = Design(f"SPAN{n_:02d}", "multi_span", sec, mat)
        Le = sig4(length_for_phi(sec, mat, phi * 4))       # two elements per span
        ids, mids = chain(d, "N", line_points([0, 0, 0], [1, 0, 0], 2 * nspan, Le))
        d.anchor("support:a", ids[0])
        d.anchor("support:b", ids[-1])
        EI = bending_stiffness(sec, mat)
        Ls = 2 * Le
        for s in range(1, nspan):
            d.spring(f"support:kz{s}", ids[2 * s], "UZ", sig4(48 * EI / Ls ** 3 * 30))
            d.spring(f"support:ky{s}", ids[2 * s + 1] if s % 2 else ids[2 * s], "UY", sig4(48 * EI / Ls ** 3 * 3))
        if "sw" in var:
            d.gravity = True
            d.contents = sec in ("S3", "S4", "S5")
        if "th" in var:
            for m in mids:
                d.thermal[m] = 35.0
        if "lat" in var:
            for m in mids:
                d.udl[m] = [0.0, 0.002 * EI / Ls ** 3, 0.0]
        d.notes.append(f"phi_member_target={phi:g}; spans={nspan}; variant={var}")
        out.append(d)
    # ---- F. phi = 2 cluster (element lengths within ulps of the phi = 2 length)
    for sec, mat in (("S5", "M1"), ("S4", "M2")):
        L2 = length_for_phi(sec, mat, 2.0)
        for k in (-3, -2, -1, 0, 1, 2, 3):
            Lk = L2
            for _ in range(abs(k)):
                Lk = math.nextafter(Lk, math.inf if k > 0 else -math.inf)
            d = Design(f"PHI2{sec}{k:+d}", "phi2_cluster", sec, mat)
            ids, mids = chain(d, "N", [[0.0, 0.0, 0.0], [Lk, 0.0, 0.0]])
            d.anchor("support:root", ids[0])
            EI = bending_stiffness(sec, mat)
            sc = EI / Lk ** 2 * 1e-3
            d.load(ids[-1], [0.0, 0.7 * sc, -1.1 * sc, 0.2 * sc * Lk, 0.0, 0.0])
            if k == 0:
                d.gravity = True
            d.notes.append(f"phi2 ulp offset={k}")
            out.append(d)
        # two-element fixed-fixed and an L line with elements at the phi = 2 length
        d = Design(f"PHI2{sec}FF", "phi2_cluster", sec, mat)
        ids, mids = chain(d, "N", [[0.0, 0.0, 0.0], [L2, 0.0, 0.0], [2 * L2, 0.0, 0.0]])
        d.anchor("support:a", ids[0])
        d.anchor("support:b", ids[-1])
        EI = bending_stiffness(sec, mat)
        d.load(ids[1], [0.0, -EI / L2 ** 2 * 1e-3, EI / L2 ** 2 * 2e-3, 0.0, 0.0, 0.0])
        d.gravity = True
        out.append(d)
        d = Design(f"PHI2{sec}L", "phi2_cluster", sec, mat)
        ids = [d.node("node:N0", [0.0, 0.0, 0.0]), d.node("node:N1", [L2, 0.0, 0.0]), d.node("node:N2", [L2, L2, 0.0])]
        d.member("pipe:N01", ids[0], ids[1])
        d.member("pipe:N12", ids[1], ids[2])
        d.anchor("support:a", ids[0])
        d.anchor("support:b", ids[2])
        for m in ("pipe:N01", "pipe:N12"):
            d.thermal[m] = 70.0
        out.append(d)
    # ---- G. pressure subfamily (straight collinear regions)
    G = [("S3", "M1", 0.01, 2, "cant", "tt", "p"), ("S4", "M2", 0.5, 2, "cant", "tt", "p+sw+tip"), ("S5", "M1", 3.0, 1, "cant", "tt", "p+tip"),
         ("S3", "M3", 0.1, 3, "ff", "tt", "p+th"), ("S4", "M1", 1.0, 2, "ff", "ts", "p+sw"), ("S5", "M2", 8.0, 2, "ff", "tt", "p+th+sw"),
         ("S6", "M1", 0.3, 2, "ff", "st", "p+pt"), ("S2", "M1", 1e-3, 2, "cant", "ts", "p+sw"), ("S6", "M3", 15.0, 1, "ff", "tt", "p"),
         ("S5", "M3", 0.05, 4, "ff", "tt", "p+sw+th+mot"), ("S4", "M3", 2.0, 2, "cant", "tt", "p+th"), ("S3", "M2", 6.0, 2, "ff", "ss", "p+pt")]
    for n_, (sec, mat, phi, nsub, top, closures, var) in enumerate(G):
        d = Design(f"PR{n_:02d}", "pressure", sec, mat)
        # element lengths on the grid 7*2^-10 m so that the skewed chain is exactly collinear in binary64
        Le = max(1, round(length_for_phi(sec, mat, phi * nsub * nsub) / PGRID)) * PGRID
        d.axis_chain = True
        ids, mids = chain(d, "N", line_points([0, 0, 0], [1, 0, 0], nsub, Le))
        d.anchor("support:a", ids[0])
        if top == "ff":
            d.anchor("support:b", ids[-1])
        kinds = {"t": "transfers_to_wall", "s": "separately_supported_or_compensated"}
        p = {"S2": 6.0e6, "S3": 4.0e6, "S4": 2.5e6, "S5": 1.6e6, "S6": 8.0e6}[sec] * (1 + 0.1 * n_)
        d.regions.append({"members": mids, "p": sig4(p), "closures": {ids[0]: kinds[closures[0]], ids[-1]: kinds[closures[1]]}})
        L = nsub * Le
        EI = bending_stiffness(sec, mat)
        scale = EI / L ** 2
        if "tip" in var:
            d.load(ids[-1], [0.0, 0.5 * scale * 1e-3, -0.8 * scale * 1e-3, 0.0, 0.0, 0.0])
        if "pt" in var:
            for k in range(1, nsub):
                d.load(ids[k], [0.0, 0.3 * scale * 1e-2, 0.4 * scale * 1e-2, 0.0, 0.0, 0.0])
        if "sw" in var:
            d.gravity = True
            d.contents = True
        if "th" in var:
            for m in mids:
                d.thermal[m] = 50.0
        if "mot" in var:
            d.load_state = True
            d.motion["support:b"] = [L * 1e-4, 0.0, -L * 1e-4, 0.0, 2e-4, 0.0]
        d.notes.append(f"phi_member_target={phi:g}; nsub={nsub}; topology={top}; closures={closures}; variant={var}")
        out.append(d)
    # ---- H. near the Sensitive boundary
    # ratios chosen so that the gate's reciprocal condition estimate straddles 2^-26 (measured on today's producer)
    for n_, ratio in enumerate((3e-5, 1e-5, 3e-6, 1e-6, 6e-7, 4e-7, 2e-7)):
        sec, mat = ("S3", "M1") if n_ % 2 == 0 else ("S4", "M2")
        d = Design(f"SOFTRX{n_:02d}", "near_sensitive", sec, mat)
        Le = sig4(length_for_phi(sec, mat, 0.2 * 4))
        ids, mids = chain(d, "N", line_points([0, 0, 0], [1, 0, 0], 2, Le))
        d.anchor("support:root", ids[0], ["UX", "UY", "UZ", "RY", "RZ"])
        GJ = torsion_stiffness(sec, mat)
        d.spring("support:krx", ids[0], "RX", sig4(ratio * GJ / (2 * Le)))
        EI = bending_stiffness(sec, mat)
        sc = EI / (2 * Le) ** 2 * 1e-3
        d.load(ids[-1], [0.0, 0.6 * sc, -0.9 * sc, 1e-6 * sig4(ratio * GJ / (2 * Le)), 0.0, 0.0])
        d.notes.append(f"soft torsion spring ratio={ratio:g}")
        out.append(d)
    for n_, ratio in enumerate((3e-5, 1e-5, 5e-6, 3e-6, 2e-6)):
        sec, mat = ("S5", "M1") if n_ % 2 == 0 else ("S3", "M3")
        d = Design(f"SOFTUY{n_:02d}", "near_sensitive", sec, mat)
        Le = sig4(length_for_phi(sec, mat, 1.0 * 4))
        ids, mids = chain(d, "N", line_points([0, 0, 0], [1, 0, 0], 2, Le))
        d.anchor("support:root", ids[0], ["UX", "UZ", "RX", "RY", "RZ"])
        EI = bending_stiffness(sec, mat)
        L = 2 * Le
        k = sig4(ratio * 12 * EI / L ** 3)
        d.spring("support:kuy", ids[-1], "UY", k)
        d.load(ids[-1], [0.0, 1e-6 * k, -EI / L ** 2 * 1e-3, 0.0, 0.0, 0.0])
        d.notes.append(f"soft translational spring ratio={ratio:g}")
        out.append(d)
    for n_, Ls in enumerate((0.05, 0.03, 0.02, 0.017, 0.014, 0.01)):
        sec, mat = ("S3", "M1") if n_ % 2 == 0 else ("S5", "M2")
        d = Design(f"SHORT{n_:02d}", "near_sensitive", sec, mat)
        Ll = 4.0
        ids, mids = chain(d, "N", [[0, 0, 0], [Ll, 0, 0], [Ll + Ls, 0, 0], [2 * Ll + Ls, 0, 0]])
        d.anchor("support:a", ids[0])
        d.anchor("support:b", ids[-1])
        EI = bending_stiffness(sec, mat)
        d.load(ids[1], [0.0, EI / Ll ** 2 * 1e-3, -EI / Ll ** 2 * 2e-3, 0.0, 0.0, 0.0])
        d.gravity = True
        d.notes.append(f"short member {Ls} m between 4 m members")
        out.append(d)
    for n_, Ls in enumerate((0.03, 0.012, 0.006, 0.004)):
        sec, mat = "S4", "M1"
        d = Design(f"SHORTL{n_:02d}", "near_sensitive", sec, mat)
        ids = [d.node("node:N0", [0.0, 0.0, 0.0]), d.node("node:N1", [3.0, 0.0, 0.0]), d.node("node:N2", [3.0, Ls, 0.0]), d.node("node:N3", [3.0, Ls, 2.5])]
        d.member("pipe:N01", ids[0], ids[1])
        d.member("pipe:N12", ids[1], ids[2])
        d.member("pipe:N23", ids[2], ids[3], (1.0, 0.0, 0.0))
        d.anchor("support:a", ids[0])
        d.anchor("support:b", ids[3])
        for m in ("pipe:N01", "pipe:N12", "pipe:N23"):
            d.thermal[m] = 60.0
        d.gravity = True
        d.notes.append(f"L line with a {Ls} m sharp-corner leg")
        out.append(d)
    # nonzero mill tolerance and allowance on every third design of each family (and two phi = 2 designs)
    counter = {}
    for d in out:
        k = counter.get(d.family, 0)
        counter[d.family] = k + 1
        if d.family == "phi2_cluster":
            d.mill_variant = d.name.endswith(("+0", "FF"))
        else:
            d.mill_variant = d.family in MILL_FAMILIES and k % 3 == 0
    return out


# --------------------------------------------------------------- placement
def placements_for(d):
    if d.family == "phi2_cluster":
        return ["o", "utmskew"] if d.name.endswith(("+0", "FF", "L")) else ["o"]
    return ["o", "utm", "utmskew"]


MILL_FAMILIES = {"cantilever", "fixed_fixed", "l_line", "multi_leg", "multi_span", "pressure", "phi2_cluster"}


def mill_values(sec):
    od, t = SECTIONS[sec]
    cm = float(f"{0.125 * t:.3g}")
    ca = 0.0005 if t < 0.005 else 0.0015
    assert cm + ca < t
    return cm, ca


def wants_mill(d, idx):
    return d.mill_variant


def build_request(d, placement, theory, mill):
    skew = placement == "utmskew"
    off = OFFSETS.get(placement, (0.0, 0.0, 0.0))
    frame = PROT if (skew and d.axis_chain) else None
    pos = {}
    nodes = []
    for nid, p in d.nodes:
        if frame is not None:
            j = p[0] / PGRID
            assert p[1] == 0.0 and p[2] == 0.0 and j == int(j)
            x = [POFF[c] + int(j) * 2.0 ** -10 * PTRIPLE[c] for c in range(3)]
            assert all(x[c] - POFF[c] == int(j) * 2.0 ** -10 * PTRIPLE[c] for c in range(3)), "inexact pressure-chain coordinate"
        else:
            r = rot(p, skew)
            x = [off[c] + r[c] for c in range(3)] if placement != "o" else [float(c) for c in p]
        pos[nid] = x
        nodes.append({"id": nid, "position": {"x": x[0], "y": x[1], "z": x[2]}, "provenance": PROV})
    E_, nu, alpha = MATERIALS[d.mat]
    pipes = []
    sect_of = {}
    for m in d.members:
        sec = d.member_sections.get(m["id"], d.sec)
        od, t = SECTIONS[sec]
        cm, ca = mill_values(sec) if mill in ("both", True) else (0.0, 0.0)
        if mill == "mill_only":
            cm, ca = mill_values(sec)[0], 0.0
        elif mill == "allowance_only":
            cm, ca = 0.0, mill_values(sec)[1]
        s = {"outside_diameter": q(od, "m"), "wall_thickness": q(t, "m"), "corrosion_allowance": q(ca, "m")}
        if cm != 0.0:
            s["mill_tolerance"] = q(cm, "m")
        y = rot(m["yref"], skew, frame)
        sect_of[m["id"]] = (od, t, cm, ca)
        pipes.append({"id": m["id"], "from": m["i"], "to": m["j"], "material": "material:m", "section": s,
                      "y_reference": {"x": y[0], "y": y[1], "z": y[2]}, "provenance": PROV})
    supports = []
    for s in d.supports:
        if s["kind"] == "anchor":
            supports.append({"id": s["id"], "node": s["node"], "family": "anchor", "restraints": s["restraints"], "provenance": PROV})
        else:
            unit = "N/m" if E.DIRS[s["dof"]] < 3 else "N*m/rad"
            supports.append({"id": s["id"], "node": s["node"], "family": "spring", "restraints": [s["dof"]],
                             "stiffness": {"dof": s["dof"], "value": q(s["k"], unit)}, "provenance": PROV})
    loads = []
    axes = ["global_x", "global_y", "global_z"]
    raxes = ["rotation_x", "rotation_y", "rotation_z"]
    for nid, v in d.nodal.items():
        F = rot(v[:3], skew, frame)
        M = rot(v[3:], skew, frame)
        for k in range(3):
            if F[k] != 0.0:
                loads.append({"id": f"load:{nid[5:]}:f{'xyz'[k]}", "category": "concentrated_force", "target": {"type": "node", "node": nid},
                              "direction": axes[k], "magnitude": q(F[k], "N"), "dimension": "force", "provenance": PROV})
            if M[k] != 0.0:
                loads.append({"id": f"load:{nid[5:]}:m{'xyz'[k]}", "category": "concentrated_moment", "target": {"type": "node", "node": nid},
                              "direction": raxes[k], "magnitude": q(M[k], "N*m"), "dimension": "moment", "provenance": PROV})
    for mid, w in d.udl.items():
        W = rot(w, skew, frame)
        for k in range(3):
            if W[k] != 0.0:
                loads.append({"id": f"load:{mid[5:]}:w{'xyz'[k]}", "category": "distributed_force", "target": {"type": "element", "pipe": mid},
                              "direction": axes[k], "magnitude": q(W[k], "N/m"), "dimension": "force_per_length", "provenance": PROV})
    weights = {}
    if d.gravity:
        for m in d.members:
            od, t, cm, ca = sect_of[m["id"]]
            s = E.d5_sections(od, t, cm, ca)
            w = G_ACC * (RHO_S * s["nominal_section"]["A"] + (RHO_C * s["bore"]["Ai"] if d.contents else 0))
            weights[m["id"]] = float(w)
            loads.append({"id": f"load:{m['id'][5:]}:weight", "category": "weight", "target": {"type": "element", "pipe": m["id"]},
                          "direction": "global_z", "magnitude": q(-float(w), "N/m"), "dimension": "force_per_length",
                          "provenance": PROV + "; self-weight w = g (rho_s A_nominal" + (" + rho_c Ai_pressure_bore" if d.contents else "") +
                          ") authored directly (rho_s 7850, rho_c 998 kg/m3, g 9.80665 m/s2, along -Z)"})
    case = {"id": "case:1", "kind": "primitive_user_load", "primitive_loads": [], "pressure_regions": [], "provenance": PROV}
    regions = []
    for k, reg in enumerate(d.regions):
        regions.append({"id": f"region:{k}", "member_pipe_ids": reg["members"], "pressure_basis": "internal_differential_zero_external_v1",
                        "pressure": q(reg["p"], "Pa"),
                        "terminals": [{"node_ref": n, "closure_transfer": c, "provenance": PROV} for n, c in reg["closures"].items()],
                        "provenance": PROV})
    case["pressure_regions"] = regions
    model = {"schema_version": "0.8.0", "document_kind": "openpipestress.product_preview.model",
             "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": theory},
             "project": {"id": f"project:m3t-{d.name.lower()}", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
             "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
             "nodes": nodes, "pipe_segments": pipes,
             "materials": [{"id": "material:m", "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "elastic_modulus": q(E_, "Pa"),
                            "poisson_ratio": q(nu, "1"), "thermal_expansion_coefficient": q(alpha, "1/degC"), "provenance": PROV}],
             "supports": supports, "components": [], "load_cases": [case], "combinations": []}
    if d.load_state:
        model["reference_configurations"] = [{"id": "reference:m3t", "geometry_ref": {"kind": "authored_model_geometry"},
                                              "member_references": [{"pipe_ref": m["id"], "basis": {"kind": "direct_strain_reference"}, "fit": {"kind": "none"}, "provenance": PROV} for m in d.members],
                                              "provenance": PROV}]
        es = []
        for m in d.members:
            st = {"pipe_ref": m["id"], "material_selection": {"kind": "explicit_base_properties", "material_ref": "material:m", "applicability_reference": "invented M3-T basis"}}
            if m["id"] in d.thermal:
                st["thermal_state"] = {"kind": "explicit_interval_strain", "strain": q(alpha * d.thermal[m["id"]], "1"), "interval_reference": "invented M3-T interval", "provenance": PROV}
            else:
                st["thermal_state"] = {"kind": "unchanged_reference", "provenance": PROV}
            es.append(st)
        ss = []
        for s in d.supports:
            st = {"support_ref": s["id"], "participation": {"kind": "active_model_device"}}
            if s["id"] in d.motion:
                v = d.motion[s["id"]]
                T = rot(v[:3], skew, frame)
                Rr = rot(v[3:], skew, frame)
                st["boundary_motion"] = [{"dof": E.DOFS[k], "value": q((T + Rr)[k], "m" if k < 3 else "rad"), "meaning": "absolute_reference_displacement"}
                                         for k in range(6) if (T + Rr)[k] != 0.0]
            ss.append(st)
        case["status"] = "user_entered"
        case["primitive_loads"] = loads
        case["analysis_state"] = {"contract": "openpipestress.load_reference_state/1.0.0", "reference_configuration_ref": "reference:m3t",
                                  "element_states": es, "support_states": ss,
                                  "load_sources": [{"source_ref": l["id"], "factor": d.factor} for l in loads],
                                  "history": {"kind": "independent_equilibrium"}, "provenance": PROV}
    else:
        for mid, dT in d.thermal.items():
            loads.append({"id": f"load:{mid[5:]}:thermal", "category": "thermal", "target": {"type": "element", "pipe": mid}, "direction": "global_x",
                          "magnitude": q(dT, "degC"), "dimension": "temperature_interval", "provenance": PROV})
        case["primitive_loads"] = loads
    return {"model": model, "materials": []}, weights


THEORIES = {"eb": "euler_bernoulli", "ts": "timoshenko_annulus_energy_v1"}


def model_list():
    """Every (tag, meta, request) of the judged set, in a fixed order."""
    out = []
    seg = {"zero": "m0", "both": "mill", "mill_only": "cm", "allowance_only": "ca"}
    for idx, d in enumerate(designs()):
        variants = ["zero"] + (["both", "mill_only", "allowance_only"] if wants_mill(d, idx) else [])
        for variant in variants:
            places = {"zero": placements_for(d), "both": ["o", "utmskew"]}.get(variant, ["o"])
            for placement in places:
                for th in ("eb", "ts"):
                    tag = f"m3t|{d.name}|{d.sec}{d.mat}|{placement}|{seg[variant]}|{th}"
                    twin = f"m3t|{d.name}|{d.sec}{d.mat}|{placement}|{seg[variant]}|{'ts' if th == 'eb' else 'eb'}"
                    req, weights = build_request(d, placement, THEORIES[th], variant)
                    meta = {"design": d.name, "family": d.family, "section": d.sec, "material": d.mat, "placement": placement,
                            "theory": THEORIES[th], "mill_and_allowance": variant == "both", "section_variant": variant, "twin": twin,
                            "load_state": d.load_state, "harness_validation": th == "eb" and variant == "zero",
                            "stiffness_basis_control": th == "eb" and variant == "mill_only", "notes": d.notes,
                            "self_weight_N_per_m": {k: repr(v) for k, v in weights.items()}}
                    out.append((tag, meta, req))
    return out


# --------------------------------------------------------------- references
def reference_of(request, theory=None, e_scale=None):
    ref = E.Reference(request, theory=theory, e_scale=e_scale if e_scale is not None else E.ONE)
    cases = {}
    for cid in ref.cases:
        rows = ref.rows(cid)
        mx = ref.maxima(cid)
        cases[cid] = {"rows": [[k[0], k[1], k[2], k[3], k[4], E.dstr(v, DIGITS)] for k, v in sorted(rows.items(), key=lambda kv: tuple("" if x is None else x for x in kv[0]))],
                      "maxima": [[p, b, E.dstr(v, DIGITS)] for (p, b), v in sorted(mx.items())],
                      "region_members": sorted(ref.cases[cid]["region_of"].keys())}
    return {"theory": ref.theory, "sections": ref.section_record(), "nodes": [[n, [E.dstr(c, DIGITS) for c in ref.nodes[n]]] for n in ref.node_order],
            "cases": cases, "phi": {m.id: E.dstr(m.phi, 30) for m in ref.members},
            "phi_minus_2": {m.id: E.dstr(m.phi - 2, 20) for m in ref.members if m.phi != 0 and abs(m.phi - 2) < D("1e-6")}}


PERT = E.ONE + E.D(2) ** -20


def work(item):
    tag, meta, req = item
    line = {"tag": tag, "meta": meta, "ref": reference_of(req), "request": req}
    controls = None
    if meta["harness_validation"]:
        controls = {"tag": tag, "pertE": reference_of(req, e_scale=PERT)}
    return json.dumps(line, sort_keys=True, separators=(",", ":")), (json.dumps(controls, sort_keys=True, separators=(",", ":")) if controls else None)


def run_pool(items, workers):
    if workers <= 1:
        return [work(x) for x in items]
    import multiprocessing as mp
    with mp.get_context("fork").Pool(workers) as pool:
        return pool.map(work, items, chunksize=1)


def frozen_selection(items):
    """A fixed subset: both theories of chosen designs and placements."""
    want = {("CANT03", "o", "m0"), ("CANT07", "utmskew", "m0"), ("CANT09", "o", "mill"), ("CANT13", "utm", "m0"),
            ("FF00", "o", "m0"), ("FF04", "utmskew", "m0"), ("FF05", "o", "m0"), ("FF06", "utmskew", "mill"),
            ("L00", "o", "m0"), ("L03", "utmskew", "m0"), ("L07", "o", "m0"), ("L09", "utmskew", "mill"),
            ("U00", "utm", "m0"), ("Z07", "utmskew", "m0"), ("3D06", "o", "m0"),
            ("SPAN02", "utmskew", "m0"), ("SPAN06", "o", "mill"),
            ("PHI2S5+0", "o", "m0"), ("PHI2S5+1", "o", "m0"), ("PHI2S4FF", "o", "m0"), ("PHI2S5L", "utmskew", "m0"),
            ("PR00", "o", "m0"), ("PR01", "utmskew", "m0"), ("PR09", "o", "mill"), ("PR09", "utmskew", "m0"), ("PR11", "o", "m0"),
            ("SOFTRX03", "o", "m0"), ("SOFTUY01", "utmskew", "m0"), ("SHORT02", "o", "m0"), ("SHORTL01", "utmskew", "m0")}
    want |= {("FF06", "o", "cm"), ("PR09", "o", "ca")}
    seg = {"zero": "m0", "both": "mill", "mill_only": "cm", "allowance_only": "ca"}
    return [x for x in items if (x[1]["design"], x[1]["placement"], seg[x[1]["section_variant"]]) in want]


def frozen_bytes(workers):
    items = frozen_selection(model_list())
    lines = [r[0] for r in run_pool(items, workers)]
    return ("\n".join(lines) + "\n").encode()


def sha256(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "check"
    if mode == "full":
        out_dir = sys.argv[2]
        workers = int(sys.argv[3]) if len(sys.argv) > 3 else 8
        os.makedirs(out_dir, exist_ok=True)
        items = model_list()
        res = run_pool(items, workers)
        with open(os.path.join(out_dir, "set.jsonl"), "w") as f:
            for line, _ in res:
                f.write(line + "\n")
        with open(os.path.join(out_dir, "controls.jsonl"), "w") as f:
            for _, c in res:
                if c:
                    f.write(c + "\n")
        manifest = {"generator_sha256": sha256(__file__), "engine_sha256": sha256(E.__file__), "models": len(items),
                    "harness_validation": sum(1 for x in items if x[1]["harness_validation"]),
                    "set_sha256": sha256(os.path.join(out_dir, "set.jsonl")), "controls_sha256": sha256(os.path.join(out_dir, "controls.jsonl"))}
        with open(os.path.join(out_dir, "manifest.json"), "w") as f:
            json.dump(manifest, f, indent=1, sort_keys=True)
        print(json.dumps(manifest, indent=1, sort_keys=True))
    elif mode == "freeze":
        workers = int(sys.argv[2]) if len(sys.argv) > 2 else 8
        os.makedirs(os.path.dirname(FROZEN), exist_ok=True)
        data = frozen_bytes(workers)
        with open(FROZEN, "wb") as f:
            f.write(data)
        print(f"wrote {FROZEN}: {data.count(b'\n')} models, sha256 {hashlib.sha256(data).hexdigest()}")
    elif mode == "check":
        workers = int(sys.argv[2]) if len(sys.argv) > 2 else 8
        n = len(E.self_check(verbose=False))
        print(f"engine self-check: {n} checks pass")
        data = frozen_bytes(workers)
        cur = open(FROZEN, "rb").read()
        if data != cur:
            raise SystemExit("frozen subset differs from a fresh generation")
        print(f"frozen subset regenerated byte for byte ({data.count(b'\n')} models)")
        import subprocess
        subprocess.run([sys.executable, "-I", "-B", os.path.join(HERE, "kd5t_compare.py"), "--selftest", FROZEN], check=True)
        sums = os.path.join(HERE, "SHA256SUMS")
        bad = []
        for line in open(sums):
            h, name = line.split(None, 1)
            name = name.strip()
            if sha256(os.path.join(HERE, name)) != h:
                bad.append(name)
        if bad:
            raise SystemExit(f"SHA256SUMS mismatch: {bad}")
        print("SHA256SUMS verified")
    elif mode == "split":
        # split <set.jsonl> <out_dir> <hv|pending|all> [models_per_chunk]: runner inputs {tag, request}
        set_path, out_dir, sel = sys.argv[2], sys.argv[3], sys.argv[4]
        per = int(sys.argv[5]) if len(sys.argv) > 5 else 60
        os.makedirs(out_dir, exist_ok=True)
        chosen = []
        for line in open(set_path):
            x = json.loads(line)
            hv = x["meta"]["harness_validation"]
            ctl = x["meta"].get("stiffness_basis_control", False)
            th = x["meta"]["theory"]
            if (sel == "all" or (sel == "hv" and hv) or (sel == "pending" and not hv) or (sel == "stiffness_control" and ctl)
                    or (sel == "eb" and th == "euler_bernoulli") or (sel == "ts" and th == "timoshenko_annulus_energy_v1")):
                chosen.append(json.dumps({"tag": x["tag"], "request": x["request"]}, sort_keys=True, separators=(",", ":")))
        for k in range(0, len(chosen), per):
            with open(os.path.join(out_dir, f"c.{k // per:03d}.in.jsonl"), "w") as f:
                f.write("\n".join(chosen[k:k + per]) + "\n")
        print(f"{len(chosen)} models in {(len(chosen) + per - 1) // per} chunks under {out_dir}")
    elif mode == "list":
        for tag, meta, _ in model_list():
            print(tag, meta["family"], meta["harness_validation"])
    else:
        raise SystemExit(__doc__)


if __name__ == "__main__":
    main()
