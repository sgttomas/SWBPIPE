"""PF M3-T judged-set generator (standard-library Python only; imports and reads no product code).

Builds invented straight-member pressure-free (PF) models (model document 0.8.0, no
`pressure_contract`), each as a Timoshenko document (`model_settings.shear_model =
timoshenko_annulus_energy_v1`) and its Euler-Bernoulli twin (member absent), places each topology
at the origin, at a UTM-scale offset and skewed at a UTM-scale offset, adds a nonzero mill
tolerance on a subset, and freezes the exact expected rows from pft_engine. The design helpers
(Design, chain, line_points, the placements and the skew rotation) are imported unchanged from
../kd5_timoshenko/kd5t_generate.py (read-only).

Usage (from this directory; heavy modes run under the host's slot wrapper):
  python3 -I -B pft_generate.py full <out_dir> [workers]   full set: set.jsonl and manifest.json
  python3 -I -B pft_generate.py freeze [workers]           rewrite frozen/pft_frozen_subset.jsonl
  python3 -I -B pft_generate.py check [workers]            engine self-check; regenerate the frozen subset in
                                                           memory and compare it byte for byte; compare
                                                           self-test; verify SHA256SUMS
  python3 -I -B pft_generate.py split <set.jsonl> <dir> <all|eb|ts|probe> [per_chunk]
  python3 -I -B pft_generate.py list | composition

Each set line: {"tag", "meta", "ref", "request"}. `ref.expect` is "rows" or "refused" (a Timoshenko
document the design refuses by name; `ref.refusal` gives the code and reasons). `ref.cases[case].rows`
are [kind, entity_ref, location, component, null, value] with values as 45-significant-digit
decimal strings in published units; `ref.cases[case].maxima` are [pipe, null, value_Pa];
`ref.sections` is PF's single section per member with nu, kappa_E and phi as exact-rational decimals.
"""

from __future__ import annotations

import collections
import hashlib
import json
import math
import os
import sys
from fractions import Fraction as Q

HERE = os.path.dirname(os.path.abspath(__file__))
sys.dont_write_bytecode = True
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, "..", "kd5_timoshenko"))
import pft_engine as E  # noqa: E402
import kd5t_generate as KG  # noqa: E402  (read-only reuse of the design helpers and placements)

K = E.K
PROV = "invented PF M3-T judged-set model; not library, catalog, manufacturer or code-rule data"
FROZEN = os.path.join(HERE, "frozen", "pft_frozen_subset.jsonl")
DIGITS = 45

# Invented sections (OD, wall) in m and PF materials (E Pa, G Pa, alpha 1/degC); not catalog data.
SECTIONS = dict(KG.SECTIONS)
SECTIONS.update({"S7": (0.28125, 0.09375), "S8": (0.2, 0.01), "S9": (0.5, 0.00125)})
MATERIALS = {"M1": (2.0e11, 7.7e10, 1.2e-5), "M2": (1.93e11, 7.6e10, 1.17e-5), "M3": (2.07e11, 8.0e10, 1.25e-5)}
RHO_S, RHO_C, G_ACC = 7850, 998, K.D("9.80665")
OFFSETS = KG.OFFSETS
Design, chain, line_points, rot, sig4 = KG.Design, KG.chain, KG.line_points, KG.rot, KG.sig4


def q(v, u):
    return {"value": v, "unit": u}


def mill_for(sec):
    """A nonzero mill tolerance near 0.125 t with t - c_m exact in binary64: c_m = t - d, d = fl(t - c0),
    exact by Sterbenz (d lies in [t/2, t])."""
    od, t = SECTIONS[sec]
    d = t - float(f"{0.125 * t:.3g}")
    cm = t - d
    assert Q(t) - Q(cm) == Q(d) and Q(t - cm) == Q(d) and 0 < cm < t
    return cm


def section_q(sec, mill=0.0):
    od, t = SECTIONS[sec]
    return E.pf_section(od, t, mill)


def phi_coef(sec, mat, mill=0.0):
    """phi * L^2 (exact): 3 E (r_o^2 + r_i^2)/(kappa_E G)."""
    ro, ri = section_q(sec, mill)
    E_, G_, _ = MATERIALS[mat] if isinstance(mat, str) else mat
    kap = E.kappa_moduli(ri / ro, Q(E_), Q(G_))
    return 3 * Q(E_) * (ro * ro + ri * ri) / (kap * Q(G_))


def length_for_phi(sec, mat, phi):
    return math.sqrt(float(phi_coef(sec, mat)) / phi)


def bending_stiffness(sec, mat):
    ro, ri = section_q(sec)
    return float(Q(MATERIALS[mat][0]) * (ro ** 4 - ri ** 4)) * math.pi / 4


def torsion_stiffness(sec, mat):
    ro, ri = section_q(sec)
    return float(Q(MATERIALS[mat][1]) * (ro ** 4 - ri ** 4)) * math.pi / 2


# ----------------------------------------------------------------- designs
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
        Le = sig4(length_for_phi(sec, mat, phi * nsub * nsub))
        ids, mids = chain(d, "N", line_points([0, 0, 0], [1, 0, 0], nsub, Le))
        d.anchor("support:root", ids[0])
        L = nsub * Le
        EI = bending_stiffness(sec, mat)
        scale = EI / L ** 2
        if "tip" in var or "mix" in var:
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
    # ---- B. fixed-fixed runs (anchors at both ends; prescribed motions at the far anchor)
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
            d.motion["support:b"] = [L * 2e-4, -L * 1.5e-4, L * 3e-4, 4e-4, -2e-4, 6e-4]
        d.notes.append(f"phi_member_target={phi:g}; nsub={nsub}; variant={var}")
        out.append(d)
    # ---- C. L-lines with a sharp corner
    C = [("S3", "M1", 0.01, 2, 1.0, "aa", "th"), ("S3", "M1", 0.01, 2, 0.6, "aa", "sw"), ("S4", "M2", 0.5, 1, 1.5, "aa", "th+sw"),
         ("S4", "M1", 3.0, 2, 0.8, "af", "tip"), ("S5", "M3", 8.0, 1, 1.0, "aa", "th"), ("S2", "M1", 1e-4, 2, 2.0, "af", "sw+tip"),
         ("S6", "M1", 0.3, 2, 1.0, "aa", "th+corner"), ("S5", "M1", 1.0, 3, 0.5, "aa", "sw+mot"), ("S1", "M2", 1e-4, 1, 3.0, "aa", "th"),
         ("S3", "M3", 4.0, 2, 1.2, "af", "tip+k"), ("S4", "M3", 0.05, 3, 0.7, "aa", "th+corner"), ("S6", "M2", 10.0, 1, 2.0, "aa", "sw"),
         ("S2", "M2", 0.002, 2, 1.0, "aa", "mot"), ("S5", "M2", 30.0, 2, 1.0, "af", "tip+g")]
    for n_, (sec, mat, phi, nsub, ratio, sup, var) in enumerate(C):
        d = Design(f"L{n_:02d}", "l_line", sec, mat)
        Le = sig4(length_for_phi(sec, mat, phi * nsub * nsub))
        Le2 = sig4(Le * ratio)
        ids1, m1 = chain(d, "A", line_points([0, 0, 0], [1, 0, 0], nsub, Le))
        corner = d.nodes[-1][1]
        pts2 = line_points(corner, [0, 1, 0], nsub, Le2)[1:]
        ids2, m2, prev = [], [], ids1[-1]
        for k, p in enumerate(pts2):
            ids2.append(d.node(f"node:B{k + 1}", p))
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
        if "g" in var.split("+"):
            d.guide("support:g", ids1[-1], ["UZ"])
        if "mot" in var:
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
        yrefs = [(0.0, 0.0, 1.0) if dv[2] == 0 else (1.0, 0.0, 0.0) for dv in dirs for _ in range(nsub)]
        ids, mids = chain(d, "N", pts, yrefs)
        d.anchor("support:a", ids[0])
        d.anchor("support:b", ids[-1])
        if "th" in var:
            for m in mids:
                d.thermal[m] = 90.0
        if "sw" in var:
            d.gravity = True
            d.contents = sec in ("S3", "S4", "S5")
        if "mot" in var:
            d.motion["support:b"] = [Le * 1e-4, Le * 2e-4, -Le * 1e-4, 2e-4, 0.0, -3e-4]
        d.notes.append(f"shape={shape}; phi_member_target={phi:g}; nsub={nsub}; variant={var}")
        out.append(d)
    # ---- E. skew multi-span runs on grounded springs and guides (anchored ends)
    Ee = [("S3", "M1", 0.01, 3, "sw"), ("S3", "M2", 0.2, 4, "sw+th"), ("S4", "M1", 1.0, 3, "sw+lat"), ("S2", "M3", 1e-4, 5, "sw"),
          ("S5", "M1", 4.0, 3, "sw+th+g"), ("S6", "M3", 0.6, 4, "sw+lat"), ("S1", "M1", 1e-5, 4, "sw+th"), ("S5", "M2", 0.05, 5, "sw+g"),
          ("S4", "M3", 9.0, 3, "lat+th"), ("S2", "M1", 2e-3, 3, "sw+lat+mot")]
    for n_, (sec, mat, phi, nspan, var) in enumerate(Ee):
        d = Design(f"SPAN{n_:02d}", "multi_span", sec, mat)
        Le = sig4(length_for_phi(sec, mat, phi * 4))
        ids, mids = chain(d, "N", line_points([0, 0, 0], [1, 0, 0], 2 * nspan, Le))
        d.anchor("support:a", ids[0])
        d.anchor("support:b", ids[-1])
        EI = bending_stiffness(sec, mat)
        Ls = 2 * Le
        for s in range(1, nspan):
            if "g" in var.split("+") and s % 2 == 1:
                d.guide(f"support:gz{s}", ids[2 * s], ["UY", "UZ"])
            else:
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
        if "mot" in var:
            d.motion["support:b"] = [0.0, Ls * 1e-4, -Ls * 1e-4, 0.0, 0.0, 0.0]
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
    # ---- H. near the Sensitive boundary: soft springs against stiff members, short members
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
    out += near_cancellation_designs()
    counter = {}
    for d in out:
        k = counter.get(d.family, 0)
        counter[d.family] = k + 1
        if d.family == "phi2_cluster":
            d.mill_variant = d.name.endswith(("+0", "FF"))
        else:
            d.mill_variant = d.family in MILL_FAMILIES and k % 3 == 0
    return out


MILL_FAMILIES = {"cantilever", "fixed_fixed", "l_line", "multi_leg", "multi_span", "phi2_cluster"}


def _guide(self, sid, node, restraints):
    self.supports.append({"id": sid, "node": node, "kind": "guide", "restraints": list(restraints)})


Design.guide = _guide

# ---- N. the near-cancellation family: E/(2G) from ordinary values to far below 2^-54 at E = 1 GPa
NC_E = 1.0e9


def _ulps(x, k):
    for _ in range(abs(k)):
        x = math.nextafter(x, math.inf if k > 0 else -math.inf)
    return x


def nc_moduli():
    """[(label, G)] at E = NC_E. G is authored; E/(2G) is exact from the two binary64 values."""
    E_ = NC_E
    out = [("codex", 6755399441055744.0e9)]                                     # E/(2G) = 1/(3 2^52), Codex's case
    for lab, x in (("x1p3", 1.3), ("x1", 1.0), ("x0p65", 0.65), ("x0p5", 0.5), ("x0p3", 0.3)):
        out.append((lab, E_ / (2 * x)))
    for e in range(1, 17):                                                       # decades 1e-1 .. 1e-16
        out.append((f"xe-{e}", E_ / (2 * 10.0 ** -e)))
    for lab, x in (("xe-17", 1e-17), ("xe-20", 1e-20), ("xe-50", 1e-50), ("xe-100", 1e-100), ("xe-154", 1e-154), ("xe-200", 1e-200)):
        out.append((lab, E_ / (2 * x)))
    out.append(("g1e300", 1e300))                                                # G = 1e300 Pa: E/(2G) = 5e-292
    out.append(("g2inf", 1e308))                                                 # fl(2G) overflows
    for k in (-2, -1, 0, 1, 2):                                                  # within ulps of 2G/E = 1 (nu ~ 0)
        out.append((f"nu0{k:+d}", _ulps(E_ / 2, k)))
    for k in (-2, -1, 0, 1, 2):                                                  # within ulps of the Sterbenz edge G = E (E/(2G) = 1/2)
        out.append((f"sterbenz{k:+d}", _ulps(E_, k)))
    g15 = E_ / 3.0                                                               # within ulps of the domain edge E/(2G) = 3/2
    for k in (-2, -1, 0, 1, 2):
        out.append((f"edge1p5{k:+d}", _ulps(g15, k)))
    gu = E_ * 2.0 ** 53                                                          # E/(2G) = 2^-54: nu = fl(E/(2G)) - 1 rounds to -1
    for k in (-2, -1, 0, 1, 2):
        out.append((f"edge2m54{k:+d}", _ulps(gu, k)))
    gt = E_ / (2 * 7.458340731200207e-155)                                       # t*t overflow edge (|t| = sqrt(DBL_MAX))
    for k in (-1, 0, 1):
        out.append((f"ttover{k:+d}", _ulps(gt, k)))
    return out


def near_cancellation_designs():
    out = []
    sections = {"S8": "codex annulus (m = 9/10)", "S7": "thick annulus (m = 1/3)", "S9": "thin annulus (m = 0.995)"}
    for label, G in nc_moduli():
        secs = ["S8", "S7", "S9"] if label in ("codex", "xe-16", "x1", "edge2m54+1", "sterbenz+0") else ["S8"]
        for sec in secs:
            mat = (NC_E, G, 1.2e-5)
            name = f"NC{label}{sec}"
            d = Design(name, "near_cancellation", sec, "NC")
            d.material = mat
            L = 1.0
            ids, mids = chain(d, "N", [[0.0, 0.0, 0.0], [L, 0.0, 0.0]])
            d.anchor("support:root", ids[0])
            d.anchor("support:tip", ids[1], ["UX", "UZ", "RX", "RY", "RZ"])
            if label == "codex" and sec == "S8":
                d.load(ids[1], [0.0, 1e-18, 0.0, 0.0, 0.0, 0.0])
                d.notes.append("Codex's case: OD 0.2 m, wall 0.01 m, L 1 m, fixed root, tip restrained except UY, E 1 GPa, "
                               "G 6755399441055744 GPa, tip force 1e-18 N; Codex's independent energy value UY = 8.31392979802745e-12 m")
            else:
                d.load(ids[1], [3.0, 2.5, -4.0, 0.5, -0.25, 0.75])
                d.udl[mids[0]] = [0.0, -1.5, 0.0]
            d.notes.append(f"near-cancellation {label}; one free DOF (tip UY)")
            out.append(d)
    # ordinary-to-small E/(2G) on free cantilevers (6 free DOFs) and a two-element cantilever: conditioning grows as 1/x
    for label, x in (("x1p3", 1.3), ("x0p5", 0.5), ("xe-3", 1e-3), ("xe-6", 1e-6), ("xe-9", 1e-9), ("xe-12", 1e-12), ("xe-16", 1e-16)):
        G = NC_E / (2 * x)
        for shape in ("free", "two"):
            d = Design(f"NC{shape}{label}S8", "near_cancellation", "S8", "NC")
            d.material = (NC_E, G, 1.2e-5)
            pts = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]] if shape == "free" else [[0.0, 0.0, 0.0], [0.5, 0.0, 0.0], [1.0, 0.0, 0.0]]
            ids, mids = chain(d, "N", pts)
            d.anchor("support:root", ids[0])
            d.load(ids[-1], [3.0, 2.5, -4.0, 0.5, -0.25, 0.75])
            d.notes.append(f"near-cancellation {label}; {'free cantilever' if shape == 'free' else 'two-element cantilever'}")
            out.append(d)
    # the auxetic phi = 2 point: OD 9/32, wall 3/32 (m = 1/3), L 7/32, E 8 GPa, G 9 GPa: nu = -5/9, kappa_E = 30/49, phi = 2 exactly
    for shape in ("guided", "free", "two", "ff"):
        d = Design(f"NCaux{shape}", "near_cancellation", "S7", "NC")
        d.material = (8.0e9, 9.0e9, 1.2e-5)
        Lq = 7 / 32
        if shape in ("two", "ff"):
            ids, mids = chain(d, "N", [[0.0, 0.0, 0.0], [Lq, 0.0, 0.0], [2 * Lq, 0.0, 0.0]])
        else:
            ids, mids = chain(d, "N", [[0.0, 0.0, 0.0], [Lq, 0.0, 0.0]])
        d.anchor("support:root", ids[0])
        if shape == "guided":
            d.anchor("support:tip", ids[-1], ["UX", "UZ", "RX", "RY", "RZ"])
        if shape == "ff":
            d.anchor("support:tip", ids[-1])
            d.load(ids[1], [3.0, 2.5, -4.0, 0.5, -0.25, 0.75])
        else:
            d.load(ids[-1], [3.0, 2.5, -4.0, 0.5, -0.25, 0.75])
        d.notes.append(f"auxetic phi = 2 point, {shape}")
        out.append(d)
    return out


# --------------------------------------------------------------- placement
def placements_for(d):
    if d.family == "near_cancellation":
        return ["o", "utmskew"] if d.name in ("NCcodexS8", "NCauxfree", "NCxe-16S8", "NCfreexe-6S8") else ["o"]
    if d.family == "phi2_cluster":
        return ["o", "utmskew"] if d.name.endswith(("+0", "FF", "L")) else ["o"]
    return ["o", "utm", "utmskew"]


def material_of(d):
    return getattr(d, "material", None) or MATERIALS[d.mat]


def build_request(d, placement, theory, mill):
    skew = placement == "utmskew"
    off = OFFSETS.get(placement, (0.0, 0.0, 0.0))
    nodes = []
    for nid, p in d.nodes:
        r = rot(p, skew)
        x = [off[c] + r[c] for c in range(3)] if placement != "o" else [float(c) for c in p]
        nodes.append({"id": nid, "position": {"x": x[0], "y": x[1], "z": x[2]}, "provenance": PROV})
    E_, G_, alpha = material_of(d)
    pipes, sect_of = [], {}
    for m in d.members:
        sec = d.member_sections.get(m["id"], d.sec)
        od, t = SECTIONS[sec]
        cm = mill_for(sec) if mill else 0.0
        s = {"outside_diameter": q(od, "m"), "wall_thickness": q(t, "m")}
        if cm != 0.0:
            s["mill_tolerance"] = q(cm, "m")
        y = rot(m["yref"], skew)
        sect_of[m["id"]] = (od, t, cm)
        pipes.append({"id": m["id"], "from": m["i"], "to": m["j"], "material": "mat:m", "y_reference": {"x": y[0], "y": y[1], "z": y[2]},
                      "section": s, "provenance": PROV})
    supports = []
    for s in d.supports:
        if s["kind"] in ("anchor", "guide"):
            supports.append({"id": s["id"], "node": s["node"], "family": s["kind"], "restraints": s["restraints"], "provenance": PROV})
        else:
            unit = "N/m" if E.DIRS[s["dof"]] < 3 else "N*m/rad"
            supports.append({"id": s["id"], "node": s["node"], "family": "spring", "restraints": [s["dof"]],
                             "stiffness": {"dof": s["dof"], "value": q(s["k"], unit)}, "provenance": PROV})
    loads = []
    axes = ["global_x", "global_y", "global_z"]
    raxes = ["rotation_x", "rotation_y", "rotation_z"]
    for nid, v in d.nodal.items():
        F, M = rot(v[:3], skew), rot(v[3:], skew)
        for k in range(3):
            if F[k] != 0.0:
                loads.append({"id": f"load:{nid[5:]}:f{'xyz'[k]}", "category": "concentrated_force", "target": {"type": "node", "node": nid},
                              "direction": axes[k], "magnitude": q(F[k], "N"), "dimension": "force", "provenance": PROV})
            if M[k] != 0.0:
                loads.append({"id": f"load:{nid[5:]}:m{'xyz'[k]}", "category": "concentrated_moment", "target": {"type": "node", "node": nid},
                              "direction": raxes[k], "magnitude": q(M[k], "N*m"), "dimension": "moment", "provenance": PROV})
    for mid, w in d.udl.items():
        W = rot(w, skew)
        for k in range(3):
            if W[k] != 0.0:
                loads.append({"id": f"load:{mid[5:]}:w{'xyz'[k]}", "category": "distributed_force", "target": {"type": "element", "pipe": mid},
                              "direction": axes[k], "magnitude": q(W[k], "N/m"), "dimension": "force_per_length", "provenance": PROV})
    weights = {}
    if d.gravity:
        for m in d.members:
            od, t, cm = sect_of[m["id"]]
            ro, ri = E.pf_section(od, t, cm)
            ro, ri = E.qd(ro), E.qd(ri)
            w = G_ACC * (RHO_S * K.PI * (ro * ro - ri * ri) + (RHO_C * K.PI * ri * ri if d.contents else 0))
            weights[m["id"]] = float(w)
            loads.append({"id": f"load:{m['id'][5:]}:weight", "category": "weight", "target": {"type": "element", "pipe": m["id"]},
                          "direction": "global_z", "magnitude": q(-float(w), "N/m"), "dimension": "force_per_length",
                          "provenance": PROV + "; self-weight w = g (rho_s A" + (" + rho_c pi r_i^2" if d.contents else "") +
                          ") on PF's single section, authored directly (rho_s 7850, rho_c 998 kg/m3, g 9.80665 m/s2, along -Z)"})
    for mid, dT in d.thermal.items():
        loads.append({"id": f"load:{mid[5:]}:thermal", "category": "thermal", "target": {"type": "element", "pipe": mid}, "direction": "global_z",
                      "magnitude": q(dT, "degC"), "dimension": "temperature_interval", "provenance": PROV})
    for sid, v in d.motion.items():
        T, Rr = rot(v[:3], skew), rot(v[3:], skew)
        for k, val in enumerate(T + Rr):
            if val != 0.0:
                dof = E.DOFS[k]
                loads.append({"id": f"load:{sid[8:]}:{dof.lower()}", "category": "imposed_displacement",
                              "target": {"type": "support", "support": sid, "dof": dof}, "direction": dof,
                              "magnitude": q(val, "m" if k < 3 else "rad"), "dimension": "displacement" if k < 3 else "rotation", "provenance": PROV})
    model = {"schema_version": "0.8.0", "document_kind": "openpipestress.product_preview.model",
             "analysis_status": {"mechanics": "ready_for_preview_diagnostics", "rule_check": "not_performed_user_rule_inputs_missing", "professional_acceptance": "not_provided"},
             "project": {"id": f"invented:pf-m3t:{d.name.lower()}", "units": {"length": "m", "force": "N", "angle": "rad", "pressure": "Pa", "stress": "Pa", "temperature": "degC"}},
             "nodes": nodes, "pipe_segments": pipes,
             "materials": [{"id": "mat:m", "elastic_modulus": q(E_, "Pa"), "shear_modulus": q(G_, "Pa"), "thermal_expansion_coefficient": q(alpha, "1/degC"), "provenance": PROV}],
             "supports": supports, "components": [],
             "load_cases": [{"id": "case:1", "kind": "primitive_user_load", "primitive_loads": loads, "provenance": PROV}],
             "combinations": []}
    if theory == E.TIMOSHENKO:
        model = {k: model[k] for k in ("schema_version", "document_kind", "analysis_status", "project")} | {"model_settings": {"shear_model": theory}} | \
            {k: v for k, v in model.items() if k not in ("schema_version", "document_kind", "analysis_status", "project")}
    return {"model": model, "materials": []}, weights


THEORIES = {"eb": E.EULER, "ts": E.TIMOSHENKO}


def probe_requests():
    """Format probes: every load, support and section feature of the judged set on small invented models."""
    out = []
    d = Design("PROBE-CANT", "probe", "S3", "M1")
    ids, mids = chain(d, "N", line_points([0, 0, 0], [1, 0, 0], 2, 0.75))
    d.anchor("support:root", ids[0])
    d.load(ids[-1], [110.0, 220.0, -330.0, 44.0, -55.0, 66.0])
    d.udl[mids[0]] = [0.0, 70.0, -90.0]
    d.gravity = d.contents = True
    for placement in ("o", "utmskew"):
        for mill in (False, True):
            out.append((f"probe|cant|{placement}|{'cm' if mill else 'm0'}|eb", build_request(d, placement, E.EULER, mill)[0]))
    out.append(("probe|cant|o|m0|ts-format-only", build_request(d, "o", E.TIMOSHENKO, False)[0]))
    d = Design("PROBE-L", "probe", "S4", "M2")
    ids1, m1 = chain(d, "A", line_points([0, 0, 0], [1, 0, 0], 2, 1.5))
    n3 = d.node("node:B1", [3.0, 2.0, 0.0])
    m2 = d.member("pipe:B01", ids1[-1], n3, (1.0, 0.0, 0.3))
    d.anchor("support:a", ids1[0])
    d.anchor("support:b", n3)
    d.guide("support:g", ids1[1], ["UZ"])
    d.spring("support:k", ids1[2], "UY", 2.5e6)
    d.spring("support:kr", ids1[1], "RX", 4.0e5)
    d.thermal[m1[0]] = 60.0
    d.motion["support:b"] = [0.002, -0.001, 0.003, 0.0004, 0.0, -0.0002]
    for placement in ("o", "utmskew"):
        out.append((f"probe|l|{placement}|m0|eb", build_request(d, placement, E.EULER, False)[0]))
    return out


def model_list():
    """Every (tag, meta, request) of the judged set, in a fixed order."""
    out = []
    for d in designs():
        variants = ["m0"] + (["cm"] if d.mill_variant else [])
        for variant in variants:
            places = placements_for(d) if variant == "m0" else ["o", "utmskew"]
            for placement in places:
                for th in ("eb", "ts"):
                    tag = f"pft|{d.name}|{d.sec}{d.mat}|{placement}|{variant}|{th}"
                    twin = f"pft|{d.name}|{d.sec}{d.mat}|{placement}|{variant}|{'ts' if th == 'eb' else 'eb'}"
                    req, weights = build_request(d, placement, THEORIES[th], variant == "cm")
                    E_, G_, _ = material_of(d)
                    meta = {"design": d.name, "family": d.family, "section": d.sec, "material": d.mat, "placement": placement,
                            "theory": THEORIES[th], "section_variant": variant, "twin": twin, "E": repr(E_), "G": repr(G_),
                            "harness_validation": th == "eb" and variant == "m0", "notes": d.notes,
                            "self_weight_N_per_m": {k: repr(v) for k, v in weights.items()}}
                    out.append((tag, meta, req))
    return out


# --------------------------------------------------------------- references
def reference_of(request, theory=None):
    ref = E.Reference(request, theory=theory)
    cases = {}
    for cid in ref.cases:
        rows = ref.rows(cid)
        mx = ref.maxima(cid)
        cases[cid] = {"rows": [[k[0], k[1], k[2], k[3], k[4], K.dstr(v, DIGITS)] for k, v in sorted(rows.items(), key=lambda kv: tuple("" if x is None else x for x in kv[0]))],
                      "maxima": [[p, None, K.dstr(v, DIGITS)] for p, v in sorted(mx.items())]}
    phis = {m.id: m.phiQ for m in ref.members}
    out = {"theory": ref.theory, "sections": ref.section_record(), "nodes": [[n, [K.dstr(c, DIGITS) for c in ref.point(n)]] for n in ref.node_order],
           "cases": cases, "expect": "rows" if ref.admitted else "refused", "expected_warnings": ref.expected_warnings(),
           "phi": {m: (E.qtext(p) if ref.theory == E.TIMOSHENKO else "0") for m, p in phis.items()},
           "phi_minus_2": {m: E.qtext(p - 2, 20) if p != 2 else "0" for m, p in phis.items() if ref.theory == E.TIMOSHENKO and abs(p - 2) < Q(1, 10 ** 6)}}
    if not ref.admitted:
        out["refusal"] = {"code": E.REFUSAL_CODE, "reasons": {m.id: m.refusal_reasons for m in ref.members if m.refusal_reasons}}
    return out


def work(item):
    tag, meta, req = item
    line = {"tag": tag, "meta": meta, "ref": reference_of(req), "request": req}
    return json.dumps(line, sort_keys=True, separators=(",", ":"))


def run_pool(items, workers):
    if workers <= 1:
        return [work(x) for x in items]
    import multiprocessing as mp
    with mp.get_context("fork").Pool(workers) as pool:
        return pool.map(work, items, chunksize=1)


FROZEN_WANT = {("CANT03", "o", "m0"), ("CANT07", "utmskew", "m0"), ("CANT09", "o", "cm"), ("CANT13", "utm", "m0"), ("CANT20", "o", "m0"),
               ("FF00", "o", "m0"), ("FF05", "o", "m0"), ("FF06", "utmskew", "cm"), ("FF07", "utmskew", "m0"),
               ("L00", "o", "m0"), ("L03", "utmskew", "m0"), ("L09", "utmskew", "cm"), ("L13", "o", "m0"),
               ("U00", "utm", "m0"), ("Z07", "utmskew", "m0"), ("3D06", "o", "m0"),
               ("SPAN02", "utmskew", "m0"), ("SPAN04", "o", "m0"), ("SPAN09", "utmskew", "m0"),
               ("PHI2S5+0", "o", "m0"), ("PHI2S5+1", "o", "m0"), ("PHI2S4FF", "o", "m0"), ("PHI2S5L", "utmskew", "m0"),
               ("SOFTRX03", "o", "m0"), ("SOFTUY01", "utmskew", "m0"), ("SHORT02", "o", "m0"), ("SHORTL01", "utmskew", "m0"),
               ("NCcodexS8", "o", "m0"), ("NCcodexS8", "utmskew", "m0"), ("NCcodexS7", "o", "m0"), ("NCxe-16S8", "o", "m0"),
               ("NCxe-17S8", "o", "m0"), ("NCxe-200S8", "o", "m0"), ("NCg1e300S8", "o", "m0"), ("NCg2infS8", "o", "m0"),
               ("NCedge2m54+0S8", "o", "m0"), ("NCedge2m54+1S8", "o", "m0"), ("NCttover+0S8", "o", "m0"),
               ("NCsterbenz-1S8", "o", "m0"), ("NCsterbenz+0S8", "o", "m0"), ("NCnu0+1S8", "o", "m0"),
               ("NCedge1p5-1S8", "o", "m0"), ("NCedge1p5+0S8", "o", "m0"), ("NCx0p3S8", "o", "m0"),
               ("NCfreexe-6S8", "o", "m0"), ("NCtwoxe-12S8", "o", "m0"), ("NCauxguided", "o", "m0"), ("NCauxfree", "utmskew", "m0"),
               ("NCauxtwo", "o", "m0"), ("NCauxff", "o", "m0")}


def frozen_selection(items):
    """A fixed subset: both theories of chosen designs and placements."""
    return [x for x in items if (x[1]["design"], x[1]["placement"], x[1]["section_variant"]) in FROZEN_WANT]


def frozen_bytes(workers):
    items = frozen_selection(model_list())
    lines = run_pool(items, workers)
    return ("\n".join(lines) + "\n").encode()


def sha256(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def precision(set_path, families, digits):
    """Re-solve the chosen families at `digits` significant digits: the worst difference of a frozen 45-digit row
    from the re-solved one, relative to the case's largest re-solved value of the same unit, and of a maximum
    relative to itself."""
    from decimal import getcontext

    def unit(k):
        return ("mm" if "displacement" in k else "rad" if "rotation" in k else "MPa" if "stress" in k else "N*m" if "moment" in k else "N")

    gens = [json.loads(line) for line in open(set_path)]
    sel = [g for g in gens if g["meta"]["family"] in families]
    old = getcontext().prec
    getcontext().prec = digits
    worst, n = (K.ZERO, None), 0
    try:
        for g in sel:
            r2 = reference_of(g["request"])
            for cid, c in g["ref"]["cases"].items():
                a = {tuple(x[:5]): K.D(x[5]) for x in c["rows"]}
                b = {tuple(x[:5]): K.D(x[5]) for x in r2["cases"][cid]["rows"]}
                sc = collections.defaultdict(lambda: K.ZERO)
                for k, v in b.items():
                    sc[unit(k[0])] = max(sc[unit(k[0])], abs(v))
                for k in a:
                    if sc[unit(k[0])]:
                        n += 1
                        e = abs(a[k] - b[k]) / sc[unit(k[0])]
                        worst = max(worst, (e, (g["tag"], k)), key=lambda w: w[0])
                for (p, _, v), (_, _, v2) in zip(c["maxima"], r2["cases"][cid]["maxima"]):
                    if K.D(v2):
                        worst = max(worst, (abs(K.D(v) - K.D(v2)) / K.D(v2), (g["tag"], "maximum", p)), key=lambda w: w[0])
    finally:
        getcontext().prec = old
    return f"{len(sel)} models, {n} rows re-solved at {digits} digits; worst row error per unit scale {float(worst[0]):.2e} at {worst[1]}"


def composition(items):
    c = collections.Counter()
    for tag, meta, _ in items:
        c[("family", meta["family"])] += 1
        c[("placement", meta["placement"])] += 1
        c[("variant", meta["section_variant"])] += 1
        c[("theory", meta["theory"])] += 1
    return c


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "check"
    if mode == "full":
        out_dir = sys.argv[2]
        workers = int(sys.argv[3]) if len(sys.argv) > 3 else 8
        os.makedirs(out_dir, exist_ok=True)
        items = model_list()
        res = run_pool(items, workers)
        with open(os.path.join(out_dir, "set.jsonl"), "w") as f:
            for line in res:
                f.write(line + "\n")
        manifest = {"generator_sha256": sha256(__file__), "engine_sha256": sha256(E.__file__), "models": len(items),
                    "set_sha256": sha256(os.path.join(out_dir, "set.jsonl"))}
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
        print(f"PF engine self-check: {n} checks pass")
        data = frozen_bytes(workers)
        cur = open(FROZEN, "rb").read()
        if data != cur:
            raise SystemExit("frozen subset differs from a fresh generation")
        print(f"frozen subset regenerated byte for byte ({data.count(b'\n')} models)")
        import subprocess
        subprocess.run([sys.executable, "-I", "-B", os.path.join(HERE, "pft_compare.py"), "--selftest", FROZEN], check=True)
        sums = os.path.join(HERE, "SHA256SUMS")
        bad = []
        for line in open(sums):
            h, name = line.split(None, 1)
            name = name.strip()
            if sha256(os.path.join(HERE, name)) != h:
                bad.append(name)
        if bad:
            raise SystemExit(f"SHA256SUMS mismatch: {bad}")
        print(f"SHA256SUMS verified ({sum(1 for _ in open(sums))} files)")
    elif mode == "split":
        set_path, out_dir, sel = sys.argv[2], sys.argv[3], sys.argv[4]
        per = int(sys.argv[5]) if len(sys.argv) > 5 else 60
        os.makedirs(out_dir, exist_ok=True)
        chosen = []
        for line in open(set_path):
            x = json.loads(line)
            th = x["meta"]["theory"]
            if sel == "all" or (sel == "eb" and th == E.EULER) or (sel == "ts" and th == E.TIMOSHENKO):
                chosen.append(json.dumps({"tag": x["tag"], "request": x["request"]}, sort_keys=True, separators=(",", ":")))
        for k in range(0, len(chosen), per):
            with open(os.path.join(out_dir, f"c.{k // per:03d}.in.jsonl"), "w") as f:
                f.write("\n".join(chosen[k:k + per]) + "\n")
        print(f"{len(chosen)} models in {(len(chosen) + per - 1) // per} chunks under {out_dir}")
    elif mode == "probes":
        # format probes (harness): invented models outside the judged set, Euler-Bernoulli except one Timoshenko
        # probe used for the envelope's format only (identity, kinds, diagnostics), never for values
        out_path = sys.argv[2]
        with open(out_path, "w") as f:
            for tag, req in probe_requests():
                f.write(json.dumps({"tag": tag, "request": req}, sort_keys=True, separators=(",", ":")) + "\n")
        print(f"wrote {out_path}")
    elif mode == "precision":
        # precision <set.jsonl> [family,...] [digits]: re-solve at more digits; worst 110-digit row error per unit scale
        set_path = sys.argv[2]
        fams = set(sys.argv[3].split(",")) if len(sys.argv) > 3 else {"near_cancellation", "near_sensitive", "phi2_cluster"}
        digits = int(sys.argv[4]) if len(sys.argv) > 4 else 200
        print(precision(set_path, fams, digits))
    elif mode == "list":
        for tag, meta, _ in model_list():
            print(tag, meta["family"])
    elif mode == "composition":
        items = model_list()
        print(len(items), "models")
        for k, v in sorted(composition(items).items()):
            print(k, v)
    else:
        raise SystemExit(__doc__)


if __name__ == "__main__":
    main()
