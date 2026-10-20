#!/usr/bin/env python3
"""O4 retained-source exact references: both stiffness readouts of request-agnostic straight-member cases.

Standard library only. This generator imports, reads and runs no product code and no product output.
Its one dependency is the frozen, independent M3-T engine
`../../kd5_timoshenko/kd5t_engine.py`, imported unchanged and bound by SHA-256 (the engine is refused if
its bytes change). Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Every model is
invented.

Readouts (o4-source-extension.md section 5, DEL-04-08 Design):
- annular: the engine's own exact element (SHEAR_REFERENCE's, kappa_E formed exactly from the authored
  OD, wall and nu on the nominal section; D-5 bases with t_r = fl(fl(t - c_m) - c_a); exact A, I, J, Z).
- admitted: the same model solved exactly with the prepared binary64 leaves: A, I, J, Z per basis the
  correctly rounded exact annulus values; kappa-hat in U8 section 3.1's fixed binary64 order from the
  leaves r_o = D/2 and r_i = fl(D/2 - t); A_s,prep = RN(kappa-hat * A_prep);
  G-hat = RN64(E / RN64(2 * RN64(1 + nu))). Its bending stress rows and circular maxima use the represented
  section-modulus interval hull(Z-hat, I-hat/c), c = r_o (DEF-O/DEF-E rows.component_stress), and are
  written as the interval's two endpoints.

Usage (from the unit directory):
  python3 -I -B generators/o4_reference.py            check: rebuild at 110 and 150 digits, rerun every
                                                      cross-check, compare the frozen files byte for byte
  python3 -I -B generators/o4_reference.py --write    intentional re-freeze only
  python3 -I -B generators/o4_reference.py --solve DOC.json   both readouts of one case document (not frozen)
"""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
import struct
import sys
from decimal import Decimal, getcontext, localcontext
from fractions import Fraction

sys.dont_write_bytecode = True

HERE = os.path.dirname(os.path.abspath(__file__))
UNIT = os.path.dirname(HERE)
ENGINE_PATH = os.path.join(os.path.dirname(UNIT), "kd5_timoshenko", "kd5t_engine.py")
ENGINE_SHA256 = "c91f28c6bce4a6777c63357fc82b8a0d1cd5a3ca48d0b1a756ef29706be8314e"


def _load_engine():
    with open(ENGINE_PATH, "rb") as fh:
        digest = hashlib.sha256(fh.read()).hexdigest()
    if digest != ENGINE_SHA256:
        raise SystemExit(f"kd5t_engine.py changed (sha256 {digest}); this unit binds {ENGINE_SHA256}")
    sys.path.insert(0, os.path.dirname(ENGINE_PATH))
    import kd5t_engine  # noqa: E402  (bound by hash above, used unchanged)
    return kd5t_engine


E = _load_engine()
D = Decimal
ZERO = D(0)

WORK_PREC = 110
CHECK_PREC = 150
SNAP = D("1e-60")            # residue snap, SI units
MIN_WRITTEN = D("1e-30")     # no written nonzero below this
RECHECK_SNAP = D("1e-80")    # every snapped residue must fall below this at CHECK_PREC
TOL_REL = D("1e-9")          # REQ-001 relative_limit
SEP_MIN = D(1000)            # discriminator separation, tolerances
LISTED = 3                   # listed rows per (case, mutant, readout)
UNMOVED = D("1e-50")         # a row within 1e-50 tolerances (1e-59 relative) of the correct value is unmoved
FRACTION_MAX_FREE = 72       # exact rational re-solve only up to this many free DOFs

TIMO = "timoshenko_annulus_energy_v1"
EB = "euler_bernoulli"
THEORIES = {"eb": EB, "timo": TIMO}
BASES = ("nominal_section", "reduced_section")
STATIONS = (("end_i", D(0)), ("quarter_1", D("0.25")), ("midspan", D("0.5")), ("quarter_3", D("0.75")), ("end_j", D(1)))
NODAL = ("ux", "uy", "uz", "rx", "ry", "rz")
END = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
SECTION = ("N", "Vy", "Vz", "T", "My", "Mz")
REACTION = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
STRESS = ("axial", "bending_y", "bending_z", "torsion")
DOF_INDEX = {"UX": 0, "UY": 1, "UZ": 2, "RX": 3, "RY": 4, "RZ": 5}
ALL6 = ("UX", "UY", "UZ", "RX", "RY", "RZ")

MUTANTS = (
    "phi_with_metal_area",
    "kappa_on_reduced_section",
    "stiffness_on_t_minus_cm",
    "reduced_rows_copied_from_nominal",
    "reduced_maximum_on_nominal_section",
    "euler_bernoulli_member_in_timoshenko_document",
    "kappa_cowper",
    "kappa_five_sixths",
    "annular_lane_reuses_kappa_hat",
)
STRESS_MUTANTS = ("reduced_rows_copied_from_nominal", "reduced_maximum_on_nominal_section")


def set_precision(prec):
    """Run the unchanged engine at `prec` digits: its module PREC, the decimal context and its pi."""
    E.PREC = prec
    getcontext().prec = prec
    E.PI = E._machin_pi()


# ------------------------------------------------------------------ binary64 helpers
def rn(x: Decimal) -> float:
    """Correctly rounded binary64 of a working-precision value, refusing a near-midpoint input."""
    f = float(x)
    if not math.isfinite(f) or f == 0.0:
        raise AssertionError("prepared leaf is zero or non-finite")
    nb = math.nextafter(f, math.inf if D(f) < x else -math.inf)
    mid = (D(f) + D(nb)) / 2
    if abs(x - mid) <= abs(x) * D(10) ** (-(getcontext().prec - 15)):
        raise AssertionError("prepared leaf within working precision of a binary64 midpoint")
    return f


def kappa_b64(m: float, nu: float) -> float:
    """U8 section 3.1's fixed binary64 order (no FMA): kappa_E from the binary64 leaves."""
    m2 = m * m
    a = 1.0 + m2
    b = 1.0 - m2
    n1 = 1.0 + nu
    c = (8.0 * nu + 14.0) * nu + 7.0
    den = c * (b * b) + 48.0 * (m2 * (n1 * n1))
    k = (6.0 * (n1 * n1)) * (a * a) / den
    if k != E.kappa_energy(m, nu):
        raise AssertionError("fixed-order kappa disagrees with the engine's form on binary64")
    return k


def cowper_b64(m: float, nu: float) -> float:
    m2 = m * m
    return 6.0 * (1.0 + nu) * (1.0 + m2) ** 2 / ((7.0 + 6.0 * nu) * (1.0 + m2) ** 2 + (20.0 + 12.0 * nu) * m2)


def g_hat(E_f: float, nu_f: float) -> float:
    return E_f / (2.0 * (1.0 + nu_f))


def section_b64(od, wall):
    ro = od / 2.0
    ri = ro - wall
    A = math.pi * (ro * ro - ri * ri)
    I = math.pi * (ro ** 4 - ri ** 4) / 4.0
    return ro, ri, A, I, 2.0 * I


# ------------------------------------------------------------------ the judged set
MATERIALS = {"steel": (2.0e11, 0.3), "nu_0p125": (2.0e11, 0.125), "nu_0p03125": (2.0e11, 0.03125),
             "stainless": (1.93e11, 0.31)}
SECTIONS = {"S1": (0.2, 0.01), "S2": (0.1143, 0.00602), "S3": (0.6096, 0.0127), "S4": (0.02, 0.002),
            "S5": (1.0, 0.05),
            # phi = 2 exactly (section 2 "including phi = 2 exactly where representable"):
            "SA": (0.3125, 0.140625),          # ro = 10/64, ri = 1/64, nu = 1/8
            "SB": (0.2421875, 0.04296875)}     # ro = 31/256, ri = 20/256, nu = 1/32
VARIANTS_FULL = ("zero", "mill", "ca", "both", "roundaway")


def SRC(extra):
    """Source-control-style designs: zero, both nonzero and one rotating further class."""
    return ("zero", "both", extra)


def allowance(variant, wall):
    ca = 0.0015 if wall >= 0.006 else 0.25 * wall
    if variant == "zero":
        return 0.0, 0.0
    if variant == "mill":
        return 0.125 * wall, 0.0
    if variant == "ca":
        return 0.0, ca
    if variant == "both":
        return 0.125 * wall, ca
    if variant == "roundaway":
        return wall * 2.0 ** -80, 0.0
    raise ValueError(variant)


def N(i, x, y, z):
    return {"id": i, "xyz_m": [float(x), float(y), float(z)]}


def M(i, a, b, sec, yref=(0.0, 0.0, 1.0)):
    return {"id": i, "from": a, "to": b, "section": sec, "y_reference": [float(c) for c in yref]}


def RIGID(i, n, dofs=ALL6):
    return {"id": i, "node": n, "kind": "rigid", "restraints": list(dofs)}


def SPRING(i, n, dof, k):
    return {"id": i, "node": n, "kind": "spring", "dof": dof, "stiffness": float(k),
            "unit": "N/m" if dof[0] == "U" else "N*m/rad"}


def F(i, n, axis, v):
    return {"id": i, "node": n, "kind": "force", "direction": axis, "value": float(v), "unit": "N"}


def MOM(i, n, axis, v):
    return {"id": i, "node": n, "kind": "moment", "direction": axis, "value": float(v), "unit": "N*m"}


def tip6(n, s=1.0, tag="L"):
    return [F(tag + "1", n, "x", 1000 * s), F(tag + "2", n, "y", -2000 * s), F(tag + "3", n, "z", 1500 * s),
            MOM(tag + "4", n, "x", 300 * s), MOM(tag + "5", n, "y", -400 * s), MOM(tag + "6", n, "z", 250 * s)]


def phi_length(sec, mat, phi):
    """Dyadic length (2^-12 grid) giving roughly the target annular phi on the nominal section."""
    od, wall = SECTIONS[sec]
    E_f, nu = MATERIALS[mat]
    ro, ri, _, _, _ = section_b64(od, wall)
    k = kappa_b64(ri / ro, nu)
    L = math.sqrt(6.0 * (1.0 + nu) * (ro * ro + ri * ri) / (k * phi))
    return round(L * 4096.0) / 4096.0


def phi2_length(sec, mat):
    """The exact annular phi = 2 length from the exact kappa_E (working-precision decimal)."""
    od, wall = SECTIONS[sec]
    _, nu = MATERIALS[mat]
    ro = E.exact(od) / 2
    ri = ro - E.exact(wall)
    k = E.kappa_energy(ri / ro, E.exact(nu))
    return (3 * (1 + E.exact(nu)) * (ro * ro + ri * ri) / k).sqrt()


def coeff(sec, mat, L, kind):
    od, wall = SECTIONS[sec]
    E_f, nu = MATERIALS[mat]
    _, _, A, I, J = section_b64(od, wall)
    if kind == "torsion":
        return g_hat(E_f, nu) * J / L
    if kind == "transverse":
        return 12.0 * E_f * I / L ** 3
    if kind == "axial":
        return E_f * A / L
    raise ValueError(kind)


def designs():
    out = []

    def add(design, family, mat, nodes, members, supports, loads, tags, variants, *, independent=False,
            mirror=(), note="", ratios=None, source_style=False):
        out.append({"design": design, "family": family, "material": mat, "nodes": nodes, "members": members,
                    "supports": supports, "loads": loads, "tags": list(tags), "variants": list(variants),
                    "theory_independent_actions": independent, "mirror_reactions": [list(p) for p in mirror],
                    "note": note, "spring_ratios": ratios or {}, "source_control_style": source_style})

    # cantilevers across the phi range (statically determinate)
    cants = [("CANT-P1E-6", "S4", 1e-6, 0.01, "mill"), ("CANT-P1E-4", "S4", 1e-4, 0.01, "ca"),
             ("CANT-P1E-2", "S1", 1e-2, 1.0, "both"), ("CANT-P3E-1", "S1", 0.3, 1.0, None),
             ("CANT-P3", "S1", 3.0, 1.0, "roundaway"), ("CANT-P30", "S1", 30.0, 1.0, "mill")]
    for name, sec, phi, s, var in cants:
        L = phi_length(sec, "steel", phi)
        add(name, "cantilever", "steel", [N("N0", 0, 0, 0), N("N1", L, 0, 0)], [M("P1", "N0", "N1", sec)],
            [RIGID("A0", "N0")], tip6("N1", s), ["axis_aligned", "origin"],
            SRC("mill") if var is None else ("zero", var), independent=True, source_style=var is None,
            note=f"target annular phi {phi:g}")

    # fixed-pinned members with an end moment: the near-end moment is M(2 - phi)/(4 + phi)
    r = 2.0 ** -6
    add("PROP-PHI2-EXACT-A", "phi2_prop", "nu_0p125",
        [N("N0", 0, 0, 0), N("N1", 39 * r / 2, 9 * r / 2, 6 * r / 2)], [M("P1", "N0", "N1", "SA")],
        [RIGID("A0", "N0"), RIGID("A1", "N1", ("UX", "UY", "UZ"))],
        [MOM("L1", "N1", "x", 500), MOM("L2", "N1", "y", -300), MOM("L3", "N1", "z", 800)],
        ["skew_other", "origin", "phi2_exact"], ("zero", "both"),
        note="annular phi = 2 exactly: nu = 1/8, r_i/r_o = 1/10, L^2 = 1638 (r/2)^2 along (39, 9, 6)")
    r = 2.0 ** -8
    add("PROP-PHI2-EXACT-B", "phi2_prop", "nu_0p03125",
        [N("N0", 0, 0, 0), N("N1", 353 * r / 4, 23 * r / 4, 8 * r / 4)], [M("P1", "N0", "N1", "SB")],
        [RIGID("A0", "N0"), RIGID("A1", "N1", ("UX", "UY", "UZ"))],
        [MOM("L1", "N1", "x", -200), MOM("L2", "N1", "y", 650), MOM("L3", "N1", "z", 400)],
        ["skew_other", "origin", "phi2_exact"], ("zero", "ca"),
        note="annular phi = 2 exactly: nu = 1/32, r_i/r_o = 20/31, L^2 = 125202 (r/4)^2 along (353, 23, 8)")
    L2 = float(phi2_length("S1", "steel"))
    add("PROP-PHI2-NEAR-X", "phi2_prop", "steel", [N("N0", 0, 0, 0), N("N1", L2, 0, 0)],
        [M("P1", "N0", "N1", "S1")], [RIGID("A0", "N0"), RIGID("A1", "N1", ("UX", "UY", "UZ"))],
        [MOM("L1", "N1", "y", 700), MOM("L2", "N1", "z", -900), MOM("L3", "N1", "x", 100)],
        ["axis_aligned", "origin", "phi2_near"], ("zero", "mill"),
        note="L is the binary64 nearest the annular phi = 2 length")
    s5 = round(float(phi2_length("S2", "steel")) / 5.0 * 2.0 ** 40) / 2.0 ** 40
    o = (1024.0, 2048.0, 16.0)
    add("PROP-PHI2-NEAR-345-OFF", "phi2_prop", "steel",
        [N("N0", *o), N("N1", o[0] + 3 * s5, o[1] + 4 * s5, o[2])], [M("P1", "N0", "N1", "S2")],
        [RIGID("A0", "N0"), RIGID("A1", "N1", ("UX", "UY", "UZ"))],
        [MOM("L1", "N1", "z", 600), MOM("L2", "N1", "x", -250), MOM("L3", "N1", "y", 350)],
        ["skew_345", "offset", "phi2_near"], ("zero", "both"),
        note="(3,4,0) member, L = 5s with s on a 2^-40 grid nearest the phi = 2 length")

    # fixed-fixed runs
    add("FF-RUN-LONG", "fixed_run", "steel", [N("N0", 0, 0, 0), N("N1", 3, 0, 0), N("N2", 7, 0, 0), N("N3", 12, 0, 0)],
        [M("P1", "N0", "N1", "S2"), M("P2", "N1", "N2", "S2"), M("P3", "N2", "N3", "S2")],
        [RIGID("A0", "N0"), RIGID("A3", "N3")],
        [F("L1", "N1", "y", 1000), F("L2", "N1", "z", 500), MOM("L3", "N1", "x", 100), F("L4", "N2", "y", -700),
         MOM("L5", "N2", "y", 250)], ["axis_aligned", "origin"], SRC("mixed"), source_style=True)
    add("FF-RUN-SHORT", "fixed_run", "steel",
        [N("N0", 0, 0, 0), N("N1", 0.15, 0, 0), N("N2", 0.35, 0, 0), N("N3", 0.6, 0, 0)],
        [M("P1", "N0", "N1", "S1"), M("P2", "N1", "N2", "S1"), M("P3", "N2", "N3", "S1")],
        [RIGID("A0", "N0"), RIGID("A3", "N3")],
        [F("L1", "N1", "y", 10000), F("L2", "N1", "z", 5000), MOM("L3", "N1", "x", 1000), F("L4", "N2", "y", -7000),
         MOM("L5", "N2", "y", 2500)], ["axis_aligned", "origin"], ("zero", "mixed"))
    add("FF-SYM", "fixed_run", "steel", [N("N0", 0, 0, 0), N("N1", 1.5, 0, 0), N("N2", 3.0, 0, 0)],
        [M("P1", "N0", "N1", "S1"), M("P2", "N1", "N2", "S1")], [RIGID("A0", "N0"), RIGID("A2", "N2")],
        [F("L1", "N1", "y", 2000), F("L2", "N1", "z", -1200)], ["axis_aligned", "origin", "symmetric"],
        ("zero", "ca"), independent=True,
        mirror=(("A0", "A2", "Fy", 1), ("A0", "A2", "Fz", 1), ("A0", "A2", "My", -1), ("A0", "A2", "Mz", -1),
                ("A0", "A2", "Fx", 1), ("A0", "A2", "Mx", 1)),
        note="symmetric: zero slope at the centre makes each half a guided cantilever")

    # L, Z and portal lines
    add("LLINE-3D", "l_line", "steel", [N("N0", 0, 0, 0), N("N1", 4, 0, 0), N("N2", 4, 3, 0)],
        [M("P1", "N0", "N1", "S2"), M("P2", "N1", "N2", "S2")], [RIGID("A0", "N0"), RIGID("A2", "N2")],
        tip6("N1"), ["axis_aligned", "origin"], SRC("ca"), source_style=True)
    add("LLINE-SHORT", "l_line", "steel", [N("N0", 0, 0, 0), N("N1", 0.4, 0, 0), N("N2", 0.4, 0.3, 0)],
        [M("P1", "N0", "N1", "S1"), M("P2", "N1", "N2", "S1")], [RIGID("A0", "N0"), RIGID("A2", "N2")],
        tip6("N1"), ["axis_aligned", "origin"], ("zero", "both"))
    add("ZLINE", "z_line", "steel", [N("N0", 0, 0, 0), N("N1", 3, 0, 0), N("N2", 3, 2, 0), N("N3", 3, 2, 1.5)],
        [M("P1", "N0", "N1", "S3"), M("P2", "N1", "N2", "S3"), M("P3", "N2", "N3", "S3", (1, 0, 0))],
        [RIGID("A0", "N0"), RIGID("A3", "N3")],
        [F("L1", "N1", "z", -5000), F("L2", "N1", "y", 1000), F("L3", "N2", "x", 2000), MOM("L4", "N2", "z", 500)],
        ["axis_aligned", "origin"], SRC("roundaway"), source_style=True)
    add("UPORTAL", "portal", "stainless", [N("N0", 0, 0, 0), N("N1", 0, 0, 3), N("N2", 4, 0, 3), N("N3", 4, 0, 0)],
        [M("P1", "N0", "N1", "S2", (0, 1, 0)), M("P2", "N1", "N2", "S2"), M("P3", "N2", "N3", "S2", (0, 1, 0))],
        [RIGID("A0", "N0"), RIGID("A3", "N3")],
        [F("L1", "N1", "x", 3000), F("L2", "N2", "z", -4000), F("L3", "N2", "y", 800), MOM("L4", "N2", "y", 200)],
        ["axis_aligned", "origin"], SRC("mill"), source_style=True)

    # RF-style congruence members
    o = (512.0, 256.0, 8.0)
    add("SKEW345-CANT-OFF", "skew", "steel", [N("N0", *o), N("N1", o[0] + 1.5, o[1] + 2.0, o[2])],
        [M("P1", "N0", "N1", "S2")], [RIGID("A0", "N0")], tip6("N1"), ["skew_345", "offset"], SRC("ca"),
        independent=True, source_style=True)
    add("SKEW345-L-OFF", "skew", "steel",
        [N("N0", *o), N("N1", o[0] + 1.5, o[1] + 2.0, o[2]), N("N2", o[0] - 0.5, o[1] + 3.5, o[2])],
        [M("P1", "N0", "N1", "S2"), M("P2", "N1", "N2", "S2")], [RIGID("A0", "N0"), RIGID("A2", "N2")],
        tip6("N1"), ["skew_345", "offset"], ("zero", "mixed"))
    add("SKEW122-CANT", "skew", "steel", [N("N0", 0, 0, 0), N("N1", 1, 2, 2)], [M("P1", "N0", "N1", "S1", (-2, -1, 2))],
        [RIGID("A0", "N0")], tip6("N1"), ["skew_122", "origin"], ("zero", "ca"), independent=True)
    o = (100.0, 200.0, 300.0)
    add("SKEW122-FRAME-OFF", "skew", "steel",
        [N("N0", *o), N("N1", o[0] + 1, o[1] + 2, o[2] + 2), N("N2", o[0] + 3, o[1] + 3, o[2])],
        [M("P1", "N0", "N1", "S1", (-2, -1, 2)), M("P2", "N1", "N2", "S1", (2, -2, 1))],
        [RIGID("A0", "N0"), RIGID("A2", "N2")], tip6("N1"), ["skew_122", "offset"], SRC("mixed"),
        source_style=True)
    add("SKEW122-YREF-Z", "skew", "steel", [N("N0", 0, 0, 0), N("N1", 0.5, 1, 1)], [M("P1", "N0", "N1", "S2")],
        [RIGID("A0", "N0")], tip6("N1"), ["skew_122", "origin"], ("zero", "both"), independent=True,
        note="y_reference (0,0,1): an irrational local frame")

    # RF-style skew tee on pins with a soft torsional spring (invented from the RF design's description)
    for ratio, tag in ((1e-8, "r1e-08"), (1e-12, "r1e-12")):
        o = (1000.0, 2000.0, 10.0)
        k = ratio * coeff("S2", "steel", 2.5, "torsion")
        add(f"RFLIKE-SKEWT-PIN-OFF-345-{tag}", "rf_style_skew_tee", "steel",
            [N("N1", *o), N("N0", o[0] + 1.5, o[1] + 2.0, o[2]), N("N2", o[0] + 3.0, o[1] + 4.0, o[2]),
             N("N3", o[0] - 0.5, o[1] + 3.5, o[2])],
            [M("P1", "N1", "N0", "S2"), M("P2", "N0", "N2", "S2"), M("P3", "N0", "N3", "S2")],
            [RIGID("A1", "N1", ("UX", "UY", "UZ")), RIGID("A2", "N2", ("UX", "UY", "UZ")), SPRING("K0", "N0", "RX", k)],
            [F("L1", "N3", "z", 10), F("L2", "N0", "y", 2000), MOM("L3", "N3", "z", 150)],
            ["skew_345", "offset", "sensitive_design", "soft_spring_" + tag], VARIANTS_FULL + ("mixed",),
            ratios={"K0": [ratio, "G J / L of P1 (binary64), L = 2.5 m"]}, source_style=True,
            note="the run N1-N0-N2 spins about its own axis, restrained only by the soft RX spring")

    # soft grounded springs, stiff contrast, short members
    for ratio, tag, var in ((1e-8, "r1e-08", None), (1e-12, "r1e-12", "both")):
        k = ratio * coeff("S2", "steel", 2.0, "transverse")
        nodes = [N(f"N{i}", 2 * i, 0, 0) for i in range(5)]
        sup = [RIGID("A0", "N0", ("UX", "RX"))]
        for i in range(5):
            sup += [SPRING(f"KY{i}", f"N{i}", "UY", k), SPRING(f"KZ{i}", f"N{i}", "UZ", k)]
        add(f"SOFT-RUN-{tag}", "soft_spring_run", "steel", nodes,
            [M(f"P{i + 1}", f"N{i}", f"N{i + 1}", "S2") for i in range(4)], sup,
            [F("L1", "N2", "y", 1000), F("L2", "N3", "z", -1500), MOM("L3", "N4", "y", 300), MOM("L4", "N1", "z", 200)],
            ["axis_aligned", "origin", "sensitive_design", "soft_spring_" + tag],
            SRC("roundaway") if var is None else ("zero", var), source_style=var is None,
            ratios={"KY*, KZ*": [ratio, "12 E I / L^3 of one member (binary64), L = 2 m"]})
    for ratio, tag, var in ((1e-8, "r1e-08", "mill"), (1e-12, "r1e-12", "ca")):
        k = ratio * coeff("S2", "steel", 3.0, "torsion")
        add(f"SOFT-TORSION-{tag}", "soft_torsion", "steel", [N("N0", 0, 0, 0), N("N1", 3, 0, 0), N("N2", 3, 2, 0)],
            [M("P1", "N0", "N1", "S2"), M("P2", "N1", "N2", "S2")],
            [RIGID("A0", "N0", ("UX", "UY", "UZ", "RY", "RZ")), SPRING("K0", "N0", "RX", k)],
            [F("L1", "N2", "z", 1000), F("L2", "N1", "y", 500), MOM("L3", "N2", "x", 100)],
            ["axis_aligned", "origin", "sensitive_design", "soft_spring_" + tag], ("zero", var),
            ratios={"K0": [ratio, "G J / L of P1 (binary64), L = 3 m"]})
    add("CONTRAST-STIFF", "contrast", "steel",
        [N("N0", 0, 0, 0), N("N1", 4, 0, 0), N("N2", 4.5, 0, 0), N("N3", 8.5, 0, 0)],
        [M("P1", "N0", "N1", "S4"), M("P2", "N1", "N2", "S5"), M("P3", "N2", "N3", "S4")],
        [RIGID("A0", "N0"), RIGID("A3", "N3")],
        [F("L1", "N1", "y", 50), F("L2", "N1", "z", 20), F("L3", "N2", "y", -10), MOM("L4", "N2", "x", 5)],
        ["axis_aligned", "origin", "sensitive_design", "stiff_contrast"], SRC("mill"), source_style=True,
        note="a short OD 1.0 m member held only by two long OD 0.02 m members (12EI/L^3 ratio about 2e9)")
    h = 2.0 ** -10
    add("SHORT-MEMBER", "short_member", "steel",
        [N("N0", 0, 0, 0), N("N1", 3, 0, 0), N("N2", 3 + h, 0, 0), N("N3", 6 + h, 0, 0)],
        [M("P1", "N0", "N1", "S1"), M("P2", "N1", "N2", "S1"), M("P3", "N2", "N3", "S1")],
        [RIGID("A0", "N0"), RIGID("A3", "N3")],
        [F("L1", "N1", "y", 2000), F("L2", "N1", "z", -1000), MOM("L3", "N1", "x", 300), F("L4", "N2", "z", 500),
         MOM("L5", "N2", "y", 200)], ["axis_aligned", "origin", "sensitive_design", "short_member"],
        SRC("ca"), source_style=True, note="the middle member is 2^-10 m long")

    # partial rigid restraints and continuous beams
    add("GUIDED", "guided", "steel", [N("N0", 0, 0, 0), N("N1", 1.25, 0, 0), N("N2", 2.5, 0, 0)],
        [M("P1", "N0", "N1", "S1"), M("P2", "N1", "N2", "S1")], [RIGID("A0", "N0"), RIGID("A2", "N2", ("UY", "RZ"))],
        [F("L1", "N1", "y", 3000), F("L2", "N1", "z", 1000), F("L3", "N2", "x", 500), F("L4", "N2", "z", -800),
         MOM("L5", "N2", "x", 200)], ["axis_aligned", "origin"], SRC("mixed"), source_style=True)
    xs = [0, 1, 2, 3.5, 5, 6.25, 7.5]
    add("CONT-BEAM", "continuous_beam", "steel", [N(f"N{i}", x, 0, 0) for i, x in enumerate(xs)],
        [M(f"P{i + 1}", f"N{i}", f"N{i + 1}", "S2") for i in range(6)],
        [RIGID("A0", "N0", ("UX", "UY", "UZ", "RX")), RIGID("A2", "N2", ("UY", "UZ")), RIGID("A4", "N4", ("UY", "UZ")),
         RIGID("A6", "N6", ("UY", "UZ"))],
        [F("L1", "N1", "y", -1200), F("L2", "N3", "z", -2000), F("L3", "N5", "y", 800), F("L4", "N5", "z", -600),
         MOM("L5", "N5", "x", 150)], ["axis_aligned", "origin"], SRC("roundaway"), source_style=True)

    # D1 caps: 32 nodes, 32 members (one closed loop), 32 supports
    pts = [(x, 0) for x in range(10)] + [(10, y) for y in range(6)] + [(x, 6) for x in range(10, 0, -1)] + \
          [(0, y) for y in range(6, 0, -1)]
    assert len(pts) == 32
    nodes = [N(f"N{i}", 0.5 * p[0], 0.5 * p[1], 0) for i, p in enumerate(pts)]
    members = [M(f"P{i + 1}", f"N{i}", f"N{(i + 1) % 32}", "S2") for i in range(32)]
    sup = [RIGID("A0", "N0")] + [SPRING(f"K{i}", f"N{i}", "UZ", 2.0e5) for i in range(1, 32)]
    add("CAPS-LOOP-32", "d1_caps", "steel", nodes, members, sup,
        [F("L1", "N5", "z", -800), F("L2", "N13", "z", 600), F("L3", "N21", "z", -400), F("L4", "N28", "z", 900),
         F("L5", "N10", "x", 1500), F("L6", "N16", "y", -1100), MOM("L7", "N24", "x", 120), MOM("L8", "N7", "z", -90)],
        ["axis_aligned", "origin", "d1_caps"], ("zero", "both"), note="32 nodes, 32 members, 32 supports")
    return out


def expand(design):
    cases = []
    for variant in design["variants"]:
        for th in ("eb", "timo"):
            mems = []
            for idx, m in enumerate(design["members"]):
                od, wall = SECTIONS[m["section"]]
                v = ("both" if idx % 2 == 0 else "zero") if variant == "mixed" else variant
                cm, ca = allowance(v, wall)
                if not (cm >= 0 and ca >= 0 and cm + ca < wall and (wall - cm) - ca > 0):
                    raise AssertionError("allowance outside the D-5 domain")
                mems.append({"id": m["id"], "from": m["from"], "to": m["to"], "section_key": m["section"],
                             "outside_diameter_m": od, "wall_thickness_m": wall, "mill_tolerance_m": cm,
                             "corrosion_allowance_m": ca, "y_reference": m["y_reference"]})
            E_f, nu = MATERIALS[design["material"]]
            doc = {"shear_model": THEORIES[th], "material": {"id": design["material"], "elastic_modulus_Pa": E_f,
                                                             "poisson_ratio": nu},
                   "nodes": design["nodes"], "members": mems, "supports": design["supports"],
                   "loads": design["loads"], "pressure_regions": []}
            cases.append({"id": f"{design['design']}|{variant}|{th}", "design": design["design"],
                          "family": design["family"], "variant": variant, "theory": th, "tags": design["tags"],
                          "source_control_style": design["source_control_style"],
                          "theory_independent_actions": design["theory_independent_actions"],
                          "mirror_reactions": design["mirror_reactions"], "spring_ratios": design["spring_ratios"],
                          "note": design["note"], "document": doc})
    return cases


def validate_document(doc):
    """D1 caps and the straight-member nodal-load family."""
    ids = [n["id"] for n in doc["nodes"]]
    assert len(ids) == len(set(ids)) and len(ids) <= 32
    assert len(doc["members"]) <= 32 and len(doc["supports"]) <= 32
    assert doc["pressure_regions"] == []
    for m in doc["members"]:
        assert m["from"] in ids and m["to"] in ids and m["from"] != m["to"]
    seen = set()
    for s in doc["supports"]:
        dofs = s["restraints"] if s["kind"] == "rigid" else [s["dof"]]
        for d in dofs:
            assert (s["node"], d) not in seen, "two supports on one DOF"
            seen.add((s["node"], d))
    for ld in doc["loads"]:
        dof = ("U" if ld["kind"] == "force" else "R") + ld["direction"].upper()
        assert not any(s["node"] == ld["node"] and s["kind"] == "rigid" and dof in s["restraints"]
                       for s in doc["supports"]), "load on a rigidly restrained DOF"


# ------------------------------------------------------------------ engine requests and leaves
def engine_request(doc):
    """The engine's input (its 0.5.0 request shape); request-agnostic documents map onto it one to one."""
    q = lambda v, u: {"value": v, "unit": u}  # noqa: E731
    sup = []
    for s in doc["supports"]:
        if s["kind"] == "rigid":
            sup.append({"id": s["id"], "node": s["node"], "family": "anchor", "restraints": list(s["restraints"])})
        else:
            sup.append({"id": s["id"], "node": s["node"], "family": "spring", "restraints": [s["dof"]],
                        "stiffness": {"dof": s["dof"], "value": q(s["stiffness"], s["unit"])}})
    loads = []
    for ld in doc["loads"]:
        if ld["kind"] == "force":
            loads.append({"id": ld["id"], "category": "concentrated_force", "target": {"type": "node", "node": ld["node"]},
                          "direction": "global_" + ld["direction"], "magnitude": q(ld["value"], "N")})
        else:
            loads.append({"id": ld["id"], "category": "concentrated_moment", "target": {"type": "node", "node": ld["node"]},
                          "direction": "rotation_" + ld["direction"], "magnitude": q(ld["value"], "N*m")})
    mat = doc["material"]
    return {"model": {
        "schema_version": "0.5.0",
        "pressure_contract": {"version": "3.0.0", "mode": "exact_pressure_v3", "shear_model": doc["shear_model"]},
        "nodes": [{"id": n["id"], "position": dict(zip("xyz", n["xyz_m"]))} for n in doc["nodes"]],
        "pipe_segments": [{"id": m["id"], "from": m["from"], "to": m["to"], "material": "material:1",
                           "section": {"outside_diameter": q(m["outside_diameter_m"], "m"),
                                       "wall_thickness": q(m["wall_thickness_m"], "m"),
                                       "mill_tolerance": q(m["mill_tolerance_m"], "m"),
                                       "corrosion_allowance": q(m["corrosion_allowance_m"], "m")},
                           "y_reference": dict(zip("xyz", m["y_reference"]))} for m in doc["members"]],
        "materials": [{"id": "material:1", "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
                       "elastic_modulus": q(mat["elastic_modulus_Pa"], "Pa"), "poisson_ratio": q(mat["poisson_ratio"], "1")}],
        "supports": sup, "components": [],
        "load_cases": [{"id": "case:1", "primitive_loads": loads, "pressure_regions": []}]}}


def prepared_leaves(od, wall, t_r, nu_f, E_f):
    """The admitted readout's binary64 leaves (o4-source-extension.md section 5)."""
    ro = E.exact(od) / 2
    ro_f = od / 2.0
    assert ro_f * 2.0 == od
    out = {}
    for b, w in (("nominal_section", wall), ("reduced_section", t_r)):
        s = E.annulus(ro, E.exact(w))
        out[b] = {k: rn(s[k]) for k in ("A", "I", "J", "Z")}
    ri_hat = ro_f - wall
    m_hat = ri_hat / ro_f
    kap = kappa_b64(m_hat, nu_f)
    out["ri_hat"] = ri_hat
    out["m_hat"] = m_hat
    out["kappa_hat"] = kap
    out["shear_area"] = kap * out["nominal_section"]["A"]
    out["G_hat"] = g_hat(E_f, nu_f)
    return out


def leaves_on_wall(od, wall, nu_f):
    ro = E.exact(od) / 2
    ro_f = od / 2.0
    s = E.annulus(ro, E.exact(wall))
    lv = {k: rn(s[k]) for k in ("A", "I", "J")}
    lv["kappa_hat"] = kappa_b64((ro_f - wall) / ro_f, nu_f)
    return lv


class LeafReference(E.Reference):
    """The engine's Reference with its member leaves replaced before the solve (the engine is unchanged)."""

    def __init__(self, request, leaf):
        self._leaf = leaf
        self._leafed = False
        super().__init__(request)

    def _solve_case(self, case):
        if not self._leafed:
            self._leafed = True
            for m in self.members:
                self._leaf(self, m)
        return super()._solve_case(case)


def member_leaves(m):
    if not hasattr(m, "o4_prepared"):
        m.o4_prepared = prepared_leaves(m.od, m.wall, m.sec["t_r_binary64"], float(m.nu), float(m.E))
        assert D(float(m.nu)) == m.nu and D(float(m.E)) == m.E
    return m.o4_prepared


def annular_terms(m, mutant):
    ro = m.sec["ro"]
    nom = m.sec["nominal_section"]
    A, I, J, ri = nom["A"], nom["I"], nom["J"], nom["ri"]
    kappa = m.kappa
    if mutant == "stiffness_on_t_minus_cm":
        s = E.annulus(ro, E.exact(m.wall - m.mill))
        A, I, J, ri = s["A"], s["I"], s["J"], s["ri"]
        kappa = E.kappa_energy(ri / ro, m.nu)
    As = kappa * A
    if mutant == "kappa_on_reduced_section":
        red = m.sec["reduced_section"]
        As = E.kappa_energy(red["ri"] / ro, m.nu) * red["A"]
    elif mutant == "kappa_cowper":
        As = E.kappa_cowper(ri / ro, m.nu) * A
    elif mutant == "kappa_five_sixths":
        As = D(5) / D(6) * A
    elif mutant == "annular_lane_reuses_kappa_hat":
        As = D(member_leaves(m)["kappa_hat"]) * A
    elif mutant == "phi_with_metal_area":
        As = A
    return m.E * A, m.E * I, m.G * J, m.G * As


def admitted_terms(m, mutant):
    lv = member_leaves(m)
    E_f, nu_f, G_f = float(m.E), float(m.nu), lv["G_hat"]
    ro_f = m.od / 2.0
    An, In, Jn = lv["nominal_section"]["A"], lv["nominal_section"]["I"], lv["nominal_section"]["J"]
    kap = lv["kappa_hat"]
    if mutant == "stiffness_on_t_minus_cm":
        w = leaves_on_wall(m.od, m.wall - m.mill, nu_f)
        An, In, Jn, kap = w["A"], w["I"], w["J"], w["kappa_hat"]
    As = kap * An
    if mutant == "kappa_on_reduced_section":
        As = kappa_b64((ro_f - m.sec["t_r_binary64"]) / ro_f, nu_f) * lv["reduced_section"]["A"]
    elif mutant == "kappa_cowper":
        As = cowper_b64(lv["m_hat"], nu_f) * An
    elif mutant == "kappa_five_sixths":
        As = (5.0 / 6.0) * An
    elif mutant == "phi_with_metal_area":
        As = An
    return D(E_f) * D(An), D(E_f) * D(In), D(G_f) * D(Jn), D(G_f) * D(As)


def leaf_factory(readout, mutant=None, target=None):
    def leaf(ref, m):
        timo = ref.theory == TIMO
        if readout == "annular":
            EA, EI, GJ, Ks = annular_terms(m, mutant)
            stress = {b: {"A": m.sec[b]["A"], "J": m.sec[b]["J"], "Zlo": m.sec[b]["Z"], "Zhi": m.sec[b]["Z"],
                          "interval": False} for b in BASES}
        else:
            EA, EI, GJ, Ks = admitted_terms(m, mutant)
            lv = member_leaves(m)
            stress = {}
            for b in BASES:
                z_hat = D(lv[b]["Z"])
                i_c = D(lv[b]["I"]) / m.sec["ro"]
                stress[b] = {"A": D(lv[b]["A"]), "J": D(lv[b]["J"]), "Zlo": min(z_hat, i_c), "Zhi": max(z_hat, i_c),
                             "interval": True}
        phi = 12 * EI / (Ks * m.L * m.L) if timo else ZERO
        if mutant == "euler_bernoulli_member_in_timoshenko_document" and m.id == target:
            phi = ZERO
        if readout == "annular" and mutant is None:
            # the unmutated annular readout is the engine's own element, bit for bit
            if not (EA == m.EA and EI == m.EI and GJ == m.GJ and phi == m.phi):
                raise AssertionError("annular leaves differ from the engine's element")
        m.EA, m.EI, m.GJ, m.phi = EA, EI, GJ, phi
        m.o4_phi_timoshenko = 12 * EI / (Ks * m.L * m.L)
        m.k_local = E.local_stiffness(EA, GJ, EI, EI, m.L, phi, phi)
        m.o4_stress = stress
    return leaf


def solve(doc, readout, mutant=None, target=None):
    return LeafReference(engine_request(doc), leaf_factory(readout, mutant, target))


# ------------------------------------------------------------------ rows
class Snap:
    def __init__(self):
        self.snapped = 0
        self.max_snapped = ZERO
        self.exact_zero = 0

    def __call__(self, x):
        if x == 0:
            return ZERO
        if abs(x) < SNAP:
            self.snapped += 1
            self.max_snapped = max(self.max_snapped, abs(x))
            return ZERO
        return x


def extract(ref, snap):
    """Request-agnostic rows in SI units, residues snapped to exact zero."""
    cid = "case:1"
    case = ref.cases[cid]
    u = case["u"]
    rows = {"nodal": {}, "member_end_force": {}, "section_action": {}, "stress": {}, "circular_maximum": {},
            "support_reaction": {}}
    for nid in ref.node_order:
        b = 6 * ref.index[nid]
        rows["nodal"][nid] = {c: snap(u[b + k]) for k, c in enumerate(NODAL)}
    for m in ref.members:
        f = case["members"][m.id]["f"]
        rows["member_end_force"][m.id] = {"end_i": {c: snap(f[k]) for k, c in enumerate(END)},
                                          "end_j": {c: snap(f[6 + k]) for k, c in enumerate(END)}}
        rows["section_action"][m.id] = {}
        rows["stress"][m.id] = {}
        ends = {}
        for loc, fr in STATIONS:
            Fc, Mc = ref.cut(cid, m.id, fr)
            Fc = [snap(x) for x in Fc]
            Mc = [snap(x) for x in Mc]
            rows["section_action"][m.id][loc] = dict(zip(SECTION, Fc + Mc))
            rows["stress"][m.id][loc] = {}
            for b in BASES:
                ss = m.o4_stress[b]
                rows["stress"][m.id][loc][b] = {
                    "axial": snap(Fc[0] / ss["A"]),
                    "bending_y": span(Mc[1], ss, snap),
                    "bending_z": span(Mc[2], ss, snap),
                    "torsion": snap(Mc[0] * m.sec["ro"] / ss["J"])}
            if loc in ("end_i", "end_j"):
                ends[loc] = (abs(Fc[0]), (Mc[1] * Mc[1] + Mc[2] * Mc[2]).sqrt())
        rows["circular_maximum"][m.id] = {}
        for b in BASES:
            ss = m.o4_stress[b]
            lo = max(n / ss["A"] + mm / ss["Zhi"] for n, mm in ends.values())
            hi = max(n / ss["A"] + mm / ss["Zlo"] for n, mm in ends.values())
            rows["circular_maximum"][m.id][b] = (snap(lo), snap(hi)) if ss["interval"] else snap(lo)
    for s in ref.supports:
        r = case["reactions"][s["id"]]
        rows["support_reaction"][s["id"]] = {c: snap(r[k]) for k, c in enumerate(REACTION)}
    return rows


def span(Mv, ss, snap):
    if not ss["interval"]:
        return snap(Mv / ss["Zlo"])
    a, b = snap(Mv / ss["Zhi"]), snap(Mv / ss["Zlo"])
    return (min(a, b), max(a, b))


def governing(ref, m_id, basis):
    cid = "case:1"
    m = ref.member_by_id[m_id]
    ss = m.o4_stress[basis]
    g = []
    for fr in (D(0), D(1)):
        Fc, Mc = ref.cut(cid, m_id, fr)
        g.append(abs(Fc[0]) / ss["A"] + (Mc[1] * Mc[1] + Mc[2] * Mc[2]).sqrt() / ss["Zhi"])
    if abs(g[0] - g[1]) <= D("1e-50") * max(g[0], g[1]):
        return "tie"
    return "end_i" if g[0] > g[1] else "end_j"


def flatten(rows, prefix=""):
    for k, v in rows.items():
        p = f"{prefix}{k}" if not prefix else f"{prefix}:{k}"
        if isinstance(v, dict):
            yield from flatten(v, p)
        else:
            yield p, v


def b64(x):
    """The binary64 of an exact value from its working-precision approximant.

    A value within working precision of a binary64 midpoint is an exact tie (sums of binary64 inputs
    land on midpoints); it is rounded half to even, as the exact value is. The rational re-solve checks
    this rule independently wherever it reaches."""
    f = float(x)
    if D(f) == x:
        return f
    nb = math.nextafter(f, math.inf if D(f) < x else -math.inf)
    mid = (D(f) + D(nb)) / 2
    if abs(x - mid) <= abs(x) * D(10) ** (-(getcontext().prec - 20)):
        bits = struct.unpack("<Q", struct.pack("<d", f))[0]
        return f if bits % 2 == 0 else nb
    return f


def fmt(x, snap_check=True):
    """[decimal with 40 significant digits, binary64]; exact zero as ["0", 0.0]."""
    if x == 0:
        return ["0", 0.0]
    if snap_check and abs(x) < MIN_WRITTEN:
        raise AssertionError(f"written nonzero below {MIN_WRITTEN}: {x}")
    return [format(x, ".39e"), b64(x)]


def fmt_value(v, snap_check=True):
    if isinstance(v, tuple):
        return {"lower": fmt(v[0], snap_check), "upper": fmt(v[1], snap_check)}
    return fmt(v, snap_check)


def format_rows(rows):
    def walk(x):
        if isinstance(x, dict):
            return {k: walk(v) for k, v in x.items()}
        return fmt_value(x)
    return walk(rows)


# ------------------------------------------------------------------ internal cross-checks
def check_solution(ref, doc):
    """Equilibrium at every node and support DOF, member statics, K_ff symmetric positive definite."""
    cid = "case:1"
    case = ref.cases[cid]
    prec = getcontext().prec
    tol = D(10) ** (-(prec - 30))
    n = 6 * len(ref.node_order)
    nodal = [ZERO] * n
    for ld in doc["loads"]:
        k = DOF_INDEX[("U" if ld["kind"] == "force" else "R") + ld["direction"].upper()]
        nodal[6 * ref.index[ld["node"]] + k] += E.exact(ld["value"])
    node_force = [ZERO] * n
    sf = sm = ZERO
    for m in ref.members:
        fe = case["members"][m.id]["f"]
        dofs = [6 * ref.index[m.i] + k for k in range(6)] + [6 * ref.index[m.j] + k for k in range(6)]
        fg = []
        for blk in range(4):
            fg += m.to_global(fe[3 * blk:3 * blk + 3])
        for r in range(12):
            node_force[dofs[r]] += fg[r]
            if r % 6 < 3:
                sf = max(sf, abs(fg[r]))
            else:
                sm = max(sm, abs(fg[r]))
        # member statics: f_j from f_i
        L = m.L
        want = [-fe[0], -fe[1], -fe[2], -fe[3], -fe[4] - L * fe[2], -fe[5] + L * fe[1]]
        scale = max([abs(x) for x in fe[:3]] + [abs(x) / L for x in fe[3:12]] + [ZERO])
        for k in range(6):
            lim = tol * (scale if k < 3 else scale * L)
            if abs(fe[6 + k] - want[k]) > lim:
                raise AssertionError(f"member statics {m.id} component {k}")
    react = [ZERO] * n
    for s in ref.supports:
        r = case["reactions"][s["id"]]
        b = 6 * ref.index[s["node"]]
        for k in range(6):
            react[b + k] += r[k]
    for g in range(n):
        if g % 6 < 3:
            sf = max(sf, abs(nodal[g]), abs(react[g]))
        else:
            sm = max(sm, abs(nodal[g]), abs(react[g]))
    pts = [ref.nodes[nid] for nid in ref.node_order]
    Lb = E.vnorm([max(p[a] for p in pts) - min(p[a] for p in pts) for a in range(3)])
    SF, SM = max(sf, sm / Lb), max(sm, Lb * sf)
    for g in range(n):
        res = node_force[g] - nodal[g] - react[g]
        if abs(res) > tol * (SF if g % 6 < 3 else SM):
            raise AssertionError(f"nodal equilibrium fails at DOF {g}: {res}")
    # K_ff symmetric and positive definite (no mechanism)
    K = case["Kff"]
    nf = len(K)
    kmax = max((abs(K[r][c]) for r in range(nf) for c in range(nf)), default=ZERO)
    for r in range(nf):
        for c in range(r + 1, nf):
            if abs(K[r][c] - K[c][r]) > tol * kmax:
                raise AssertionError("K_ff not symmetric")
    a = [row[:] for row in K]
    for col in range(nf):
        p = a[col][col]
        if not p > 0:
            raise AssertionError("K_ff not positive definite: mechanism")
        for r in range(col + 1, nf):
            if a[r][col] != 0:
                f = a[r][col] / p
                for c in range(col, nf):
                    if a[col][c] != 0:
                        a[r][c] -= f * a[col][c]
    return nf


# ------------------------------------------------------------------ discriminators and readout gaps
INF = "inf"


def separation(c, w):
    clo, chi = c if isinstance(c, tuple) else (c, c)
    wlo, whi = w if isinstance(w, tuple) else (w, w)
    if whi < clo:
        d, ref = clo - whi, abs(clo)
    elif wlo > chi:
        d, ref = wlo - chi, abs(chi)
    else:
        return ZERO
    if ref == 0:
        return INF
    return d / (TOL_REL * ref)


def sep_str(s):
    return "%.3e" % float(s)


def compare(correct, wrong):
    cw = dict(flatten(wrong))
    moved, below, zero_moved, n = [], 0, 0, 0
    for path, c in flatten(correct):
        n += 1
        w = cw[path]
        if w == c:
            continue
        s = separation(c, w)
        if s == INF:
            zero_moved += 1
        elif s >= SEP_MIN:
            moved.append((sep_str(s), path, c, w))
        elif s > UNMOVED:
            below += 1
    moved.sort(key=lambda t: (-float(t[0]), t[1]))
    rec = {"rows_compared": n, "rows_moved_1e3": len(moved), "rows_moved_below_1e3": below,
           "zero_rows_moved": zero_moved}
    if moved:
        groups = {}
        for _, p, _, _ in moved:
            g = p.split(":")[0]
            groups[g] = groups.get(g, 0) + 1
        rec["moved_1e3_by_group"] = {g: groups[g] for g in ("nodal", "member_end_force", "section_action", "stress",
                                                          "circular_maximum", "support_reaction") if g in groups}
        rec["largest_tolerances"] = moved[0][0]
        rec["smallest_moved_tolerances"] = min(moved, key=lambda t: float(t[0]))[0]
        rec["listed"] = [{"row": p, "correct": fmt_value(c), "wrong": fmt_value(w, False), "tolerances": s}
                         for s, p, c, w in moved[:LISTED]]
        for item in rec["listed"]:
            assert D(item["tolerances"]) >= SEP_MIN
    return rec


def stress_mutant(rows, mutant, differ):
    wrong = json_copy(rows)
    for mid, sd in differ.items():
        if not sd:
            continue
        if mutant == "reduced_rows_copied_from_nominal":
            for loc in wrong["stress"][mid]:
                wrong["stress"][mid][loc]["reduced_section"] = dict(wrong["stress"][mid][loc]["nominal_section"])
        else:
            wrong["circular_maximum"][mid]["reduced_section"] = wrong["circular_maximum"][mid]["nominal_section"]
    return wrong


def json_copy(x):
    if isinstance(x, dict):
        return {k: json_copy(v) for k, v in x.items()}
    return x


def readout_gap(rows):
    adm = dict(flatten(rows["admitted"]))
    beyond, zero_vs_nonzero, worst = [], 0, ZERO
    for path, a in flatten(rows["annular"]):
        b = adm[path]
        s = separation(a if not isinstance(a, tuple) else a, b)
        if s == INF:
            zero_vs_nonzero += 1
            beyond.append(("inf", path))
            continue
        worst = max(worst, s)
        if s > 1:
            beyond.append((sep_str(s), path))
    beyond.sort(key=lambda t: (-(math.inf if t[0] == "inf" else float(t[0])), t[1]))
    return {"rows_beyond_1_tolerance": len(beyond), "annular_zero_admitted_nonzero": zero_vs_nonzero,
            "worst_finite_tolerances": sep_str(worst), "listed": [{"row": p, "tolerances": s} for s, p in beyond[:5]]}


# ------------------------------------------------------------------ binary64 proxy (not the product's gate)
def binary64_proxy(doc, exact_rows):
    """A plain binary64 formation and partial-pivot solve, judged against the exact annular readout.

    It indicates which designs are hard for an ordinary binary64 solve; the product's Passed/Sensitive
    gate is product-side and is not computed here."""
    timo = doc["shear_model"] == TIMO
    E_f, nu = doc["material"]["elastic_modulus_Pa"], doc["material"]["poisson_ratio"]
    G = g_hat(E_f, nu)
    nodes = {n["id"]: n["xyz_m"] for n in doc["nodes"]}
    order = [n["id"] for n in doc["nodes"]]
    idx = {nid: k for k, nid in enumerate(order)}
    n = 6 * len(order)
    K = [[0.0] * n for _ in range(n)]
    f = [0.0] * n
    for ld in doc["loads"]:
        f[6 * idx[ld["node"]] + DOF_INDEX[("U" if ld["kind"] == "force" else "R") + ld["direction"].upper()]] += ld["value"]
    mem = []
    for m in doc["members"]:
        xi, xj = nodes[m["from"]], nodes[m["to"]]
        d = [xj[k] - xi[k] for k in range(3)]
        L = math.sqrt(d[0] * d[0] + d[1] * d[1] + d[2] * d[2])
        ex = [c / L for c in d]
        y = m["y_reference"]
        dot = y[0] * ex[0] + y[1] * ex[1] + y[2] * ex[2]
        p = [y[k] - dot * ex[k] for k in range(3)]
        pn = math.sqrt(p[0] * p[0] + p[1] * p[1] + p[2] * p[2])
        ey = [c / pn for c in p]
        ez = [ex[1] * ey[2] - ex[2] * ey[1], ex[2] * ey[0] - ex[0] * ey[2], ex[0] * ey[1] - ex[1] * ey[0]]
        ro, ri, A, I, J = section_b64(m["outside_diameter_m"], m["wall_thickness_m"])
        As = kappa_b64(ri / ro, nu) * A
        phi = 12.0 * E_f * I / (G * As * L * L) if timo else 0.0
        kl = E.local_stiffness(E_f * A, G * J, E_f * I, E_f * I, L, phi, phi)
        R = [ex, ey, ez]
        kg = [[0.0] * 12 for _ in range(12)]
        for bi in range(4):
            for bj in range(4):
                for r in range(3):
                    for c in range(3):
                        kg[3 * bi + r][3 * bj + c] = sum(R[s][r] * sum(kl[3 * bi + s][3 * bj + t] * R[t][c] for t in range(3))
                                                         for s in range(3))
        dofs = [6 * idx[m["from"]] + k for k in range(6)] + [6 * idx[m["to"]] + k for k in range(6)]
        for r in range(12):
            for c in range(12):
                K[dofs[r]][dofs[c]] += kg[r][c]
        mem.append((m["id"], kl, R, dofs))
    fixed = set()
    for s in doc["supports"]:
        if s["kind"] == "rigid":
            for dname in s["restraints"]:
                fixed.add(6 * idx[s["node"]] + DOF_INDEX[dname])
        else:
            g = 6 * idx[s["node"]] + DOF_INDEX[s["dof"]]
            K[g][g] += s["stiffness"]
    free = [g for g in range(n) if g not in fixed]
    a = [[K[r][c] for c in free] + [f[r]] for r in free]
    nf = len(free)
    for col in range(nf):
        piv = max(range(col, nf), key=lambda r: abs(a[r][col]))
        a[col], a[piv] = a[piv], a[col]
        if a[col][col] == 0.0:
            return {"worst_scaled_error": "singular in binary64", "exceeds_1e-9": True}
        for r in range(col + 1, nf):
            if a[r][col] != 0.0:
                fac = a[r][col] / a[col][col]
                for c in range(col, nf + 1):
                    a[r][c] -= fac * a[col][c]
    x = [0.0] * nf
    for i in range(nf - 1, -1, -1):
        s = a[i][nf] - sum(a[i][c] * x[c] for c in range(i + 1, nf))
        x[i] = s / a[i][i]
    u = [0.0] * n
    for g, v in zip(free, x):
        u[g] = v
    groups = {"translation": [], "rotation": [], "force": [], "moment": []}
    for nid in order:
        b = 6 * idx[nid]
        for k, c in enumerate(NODAL):
            groups["translation" if k < 3 else "rotation"].append((u[b + k], exact_rows["nodal"][nid][c]))
    for mid, kl, R, dofs in mem:
        dl = []
        for blk in range(4):
            v = [u[dofs[3 * blk + t]] for t in range(3)]
            dl += [sum(R[s][t] * v[t] for t in range(3)) for s in range(3)]
        fe = [sum(kl[r][c] * dl[c] for c in range(12)) for r in range(12)]
        for k, c in enumerate(END):
            groups["force" if k < 3 else "moment"].append((fe[k], exact_rows["member_end_force"][mid]["end_i"][c]))
            groups["force" if k < 3 else "moment"].append((fe[6 + k], exact_rows["member_end_force"][mid]["end_j"][c]))
    worst = 0.0
    for g in groups.values():
        scale = max((abs(float(e)) for _, e in g), default=0.0)
        if scale == 0.0:
            continue
        for v, e in g:
            if not math.isfinite(v):
                return {"worst_scaled_error": "non-finite", "exceeds_1e-9": True}
            worst = max(worst, abs(float(D(v) - e)) / scale)
    return {"worst_scaled_error": "%.2e" % worst, "exceeds_1e-9": worst > 1e-9}


# ------------------------------------------------------------------ exact rational re-solve (independent code)
def frac_sqrt(q):
    if q < 0:
        return None
    n, d = q.numerator, q.denominator
    a, b = math.isqrt(n), math.isqrt(d)
    return Fraction(a, b) if a * a == n and b * b == d else None


def kappa_closed(m, nu):
    """kappa_E as o4-source-extension.md section 4.3 writes it (not the engine's arrangement)."""
    m2 = m * m
    return 6 * (1 + nu) ** 2 * (1 + m2) ** 2 / ((8 * nu * nu + 14 * nu + 7) * (1 - m2) ** 2 + 48 * m2 * (1 + nu) ** 2)


def own_local(EA, GJ, EI, L, phi):
    k = [[Fraction(0)] * 12 for _ in range(12)]
    for p, q, s in ((0, 0, 1), (0, 6, -1), (6, 0, -1), (6, 6, 1)):
        k[p][q] = s * EA / L
    for p, q, s in ((3, 3, 1), (3, 9, -1), (9, 3, -1), (9, 9, 1)):
        k[p][q] = s * GJ / L
    c = EI / (L ** 3 * (1 + phi))
    kvv, kvt, ktt, ktt2 = 12 * c, 6 * L * c, (4 + phi) * L * L * c, (2 - phi) * L * L * c
    blk = [[kvv, kvt, -kvv, kvt], [kvt, ktt, -kvt, ktt2], [-kvv, -kvt, kvv, -kvt], [kvt, ktt2, -kvt, ktt]]
    for idx, sgn in (([1, 5, 7, 11], [1, 1, 1, 1]), ([2, 4, 8, 10], [1, -1, 1, -1])):
        for r in range(4):
            for c2 in range(4):
                k[idx[r]][idx[c2]] = sgn[r] * sgn[c2] * blk[r][c2]
    return k


def frac_solve(a, b):
    n = len(b)
    m = [row[:] + [b[i]] for i, row in enumerate(a)]
    for col in range(n):
        piv = next(r for r in range(col, n) if m[r][col] != 0)
        m[col], m[piv] = m[piv], m[col]
        p = m[col]
        for r in range(col + 1, n):
            if m[r][col] != 0:
                f = m[r][col] / p[col]
                for c in range(col, n + 1):
                    if p[c] != 0:
                        m[r][c] -= f * p[c]
    x = [Fraction(0)] * n
    for i in range(n - 1, -1, -1):
        s = m[i][n] - sum((m[i][c] * x[c] for c in range(i + 1, n)), Fraction(0))
        x[i] = s / m[i][i]
    return x


def frac_text(q, pi_div=False, sqrt_extra=None):
    """40-digit text of an exact value (optionally divided by pi), for comparison with the written rows.

    A rational value also carries its correctly rounded binary64 (Fraction rounds half to even)."""
    if q == 0 and not sqrt_extra:
        return ("0", 0.0)
    with localcontext() as ctx:
        ctx.prec = getcontext().prec + 20
        x = D(q.numerator) / D(q.denominator)
        if pi_div:
            x = x / E.PI
        text = format(x, ".39e") if x != 0 else "0"
    return text if pi_div else (text, float(q))


def fraction_check(case, written):
    """Re-solve exactly in rationals with code independent of the engine's assembly and recovery.

    admitted readout: every leaf, length and frame entry is rational, so the solution is exact.
    annular readout (spring-free only): the members' stiffness is pi times a rational matrix, so
    u = u'/pi, end actions are rational and stresses are rational/pi."""
    doc = case["document"]
    timo = doc["shear_model"] == TIMO
    nodes = {n["id"]: [Fraction(c) for c in n["xyz_m"]] for n in doc["nodes"]}
    order = [n["id"] for n in doc["nodes"]]
    idx = {nid: k for k, nid in enumerate(order)}
    frames = {}
    for m in doc["members"]:
        d = [nodes[m["to"]][k] - nodes[m["from"]][k] for k in range(3)]
        L = frac_sqrt(sum(c * c for c in d))
        if L is None:
            return {"eligible": False, "reason": "irrational member length"}
        ex = [c / L for c in d]
        y = [Fraction(c) for c in m["y_reference"]]
        dot = sum(y[k] * ex[k] for k in range(3))
        p = [y[k] - dot * ex[k] for k in range(3)]
        pn = frac_sqrt(sum(c * c for c in p))
        if pn is None:
            return {"eligible": False, "reason": "irrational member frame"}
        ey = [c / pn for c in p]
        ez = [ex[1] * ey[2] - ex[2] * ey[1], ex[2] * ey[0] - ex[0] * ey[2], ex[0] * ey[1] - ex[1] * ey[0]]
        frames[m["id"]] = (L, [ex, ey, ez])
    n = 6 * len(order)
    fixed = set()
    for s in doc["supports"]:
        if s["kind"] == "rigid":
            for dname in s["restraints"]:
                fixed.add(6 * idx[s["node"]] + DOF_INDEX[dname])
    free = [g for g in range(n) if g not in fixed]
    if len(free) > FRACTION_MAX_FREE:
        return {"eligible": False, "reason": f"{len(free)} free DOFs"}
    springs = [s for s in doc["supports"] if s["kind"] == "spring"]
    Ef, nuf = doc["material"]["elastic_modulus_Pa"], doc["material"]["poisson_ratio"]
    Eq, nuq = Fraction(Ef), Fraction(nuf)
    out = {"eligible": True, "values_compared": 0, "binary64_compared": 0, "readouts": []}
    for readout in ("admitted", "annular"):
        if readout == "annular" and springs:
            continue
        pi = readout == "annular"
        K = [[Fraction(0)] * n for _ in range(n)]
        P = [Fraction(0)] * n
        for ld in doc["loads"]:
            P[6 * idx[ld["node"]] + DOF_INDEX[("U" if ld["kind"] == "force" else "R") + ld["direction"].upper()]] += Fraction(ld["value"])
        mdata = {}
        for m in doc["members"]:
            L, R = frames[m["id"]]
            od, wall = m["outside_diameter_m"], m["wall_thickness_m"]
            t_r = (wall - m["mill_tolerance_m"]) - m["corrosion_allowance_m"]
            ro = Fraction(od) / 2
            if readout == "admitted":
                lv = prepared_leaves(od, wall, t_r, nuf, Ef)
                G = Fraction(lv["G_hat"])
                A, I, J = (Fraction(lv["nominal_section"][k]) for k in ("A", "I", "J"))
                As = Fraction(lv["shear_area"])
                sec = {}
                for b in BASES:
                    zh, ic = Fraction(lv[b]["Z"]), Fraction(lv[b]["I"]) / ro
                    sec[b] = {"A": Fraction(lv[b]["A"]), "J": Fraction(lv[b]["J"]), "Zlo": min(zh, ic), "Zhi": max(zh, ic)}
            else:
                G = Eq / (2 * (1 + nuq))
                sec = {}
                for b, w in (("nominal_section", Fraction(wall)), ("reduced_section", Fraction(t_r))):
                    ri = ro - w
                    Ip = (ro ** 4 - ri ** 4) / 4
                    sec[b] = {"A": ro * ro - ri * ri, "J": 2 * Ip, "Zlo": Ip / ro, "Zhi": Ip / ro, "I": Ip, "ri": ri}
                A, I, J = sec["nominal_section"]["A"], sec["nominal_section"]["I"], sec["nominal_section"]["J"]
                As = kappa_closed(sec["nominal_section"]["ri"] / ro, nuq) * A
            phi = 12 * Eq * I / (G * As * L * L) if timo else Fraction(0)
            kl = own_local(Eq * A, G * J, Eq * I, L, phi)
            dofs = [6 * idx[m["from"]] + k for k in range(6)] + [6 * idx[m["to"]] + k for k in range(6)]
            for bi in range(4):
                for bj in range(4):
                    blk = [[kl[3 * bi + r][3 * bj + c] for c in range(3)] for r in range(3)]
                    tmp = [[sum((blk[r][s] * R[s][c] for s in range(3)), Fraction(0)) for c in range(3)] for r in range(3)]
                    for r in range(3):
                        for c in range(3):
                            K[dofs[3 * bi + r]][dofs[3 * bj + c]] += sum((R[s][r] * tmp[s][c] for s in range(3)), Fraction(0))
            mdata[m["id"]] = (kl, R, dofs, L, ro, sec)
        for s in springs:
            g = 6 * idx[s["node"]] + DOF_INDEX[s["dof"]]
            K[g][g] += Fraction(s["stiffness"])
        uf = frac_solve([[K[r][c] for c in free] for r in free], [P[g] for g in free])
        u = [Fraction(0)] * n
        for g, v in zip(free, uf):
            u[g] = v
        exact = {}
        for nid in order:
            b = 6 * idx[nid]
            for k, c in enumerate(NODAL):
                exact[f"nodal:{nid}:{c}"] = frac_text(u[b + k], pi)
        node_force = [Fraction(0)] * n
        for mid, (kl, R, dofs, L, ro, sec) in mdata.items():
            dl = []
            for blk in range(4):
                v = [u[dofs[3 * blk + t]] for t in range(3)]
                dl += [sum((R[s][t] * v[t] for t in range(3)), Fraction(0)) for s in range(3)]
            fe = [sum((kl[r][c] * dl[c] for c in range(12)), Fraction(0)) for r in range(12)]
            for blk in range(4):
                v = fe[3 * blk:3 * blk + 3]
                for t in range(3):
                    node_force[dofs[3 * blk + t]] += sum((R[s][t] * v[s] for s in range(3)), Fraction(0))
            for k, c in enumerate(END):
                exact[f"member_end_force:{mid}:end_i:{c}"] = frac_text(fe[k])
                exact[f"member_end_force:{mid}:end_j:{c}"] = frac_text(fe[6 + k])
            ends = {}
            for loc, fr in STATIONS:
                x = Fraction(fr) * L
                Fc = [-fe[0], -fe[1], -fe[2]]
                Mc = [-fe[3], -fe[4] - x * fe[2], -fe[5] + x * fe[1]]
                for c, v in zip(SECTION, Fc + Mc):
                    exact[f"section_action:{mid}:{loc}:{c}"] = frac_text(v)
                for b in BASES:
                    ss = sec[b]
                    key = f"stress:{mid}:{loc}:{b}"
                    exact[key + ":axial"] = frac_text(Fc[0] / ss["A"], pi)
                    exact[key + ":torsion"] = frac_text(Mc[0] * ro / ss["J"], pi)
                    for comp, mv in (("bending_y", Mc[1]), ("bending_z", Mc[2])):
                        if pi:
                            exact[f"{key}:{comp}"] = frac_text(mv / ss["Zlo"], True)
                        else:
                            lo, hi = sorted((mv / ss["Zhi"], mv / ss["Zlo"]))
                            exact[f"{key}:{comp}:lower"] = frac_text(lo)
                            exact[f"{key}:{comp}:upper"] = frac_text(hi)
                if loc in ("end_i", "end_j"):
                    ends[loc] = (abs(Fc[0]), Mc[1] * Mc[1] + Mc[2] * Mc[2])
            for b in BASES:
                ss = sec[b]
                with localcontext() as ctx:
                    ctx.prec = getcontext().prec + 20

                    def g(nn, q2, z):
                        return D(nn.numerator) / D(nn.denominator) / D(ss["A"].numerator) * D(ss["A"].denominator) + \
                            (D(q2.numerator) / D(q2.denominator)).sqrt() / (D(z.numerator) / D(z.denominator))
                    lo = max(g(nn, q2, ss["Zhi"]) for nn, q2 in ends.values())
                    hi = max(g(nn, q2, ss["Zlo"]) for nn, q2 in ends.values())
                    if pi:
                        lo, hi = lo / E.PI, hi / E.PI
                    lo_s = format(lo, ".39e") if lo != 0 else "0"
                    hi_s = format(hi, ".39e") if hi != 0 else "0"
                if pi:
                    exact[f"circular_maximum:{mid}:{b}"] = lo_s
                else:
                    exact[f"circular_maximum:{mid}:{b}:lower"] = lo_s
                    exact[f"circular_maximum:{mid}:{b}:upper"] = hi_s
        for s in doc["supports"]:
            b = 6 * idx[s["node"]]
            for k, c in enumerate(REACTION):
                if s["kind"] == "rigid":
                    v = node_force[b + k] - P[b + k] if ALL6[k] in s["restraints"] else Fraction(0)
                else:
                    v = -Fraction(s["stiffness"]) * u[b + k] if ALL6[k] == s["dof"] else Fraction(0)
                exact[f"support_reaction:{s['id']}:{c}"] = frac_text(v)
        got = {}
        for path, v in flatten(written[readout]):
            got[path] = v
        if set(got) != set(exact):
            raise AssertionError("rational re-solve row universe differs")
        for k, v in exact.items():
            g = got[k]
            if isinstance(v, tuple):
                ok = g[0] == v[0] and (g[1] == v[1] or (v[0] == "0" and g[1] == 0.0))
                out["binary64_compared"] += 1
            else:
                ok = g[0] == v
            if not ok:
                raise AssertionError(f"rational re-solve differs at {case['id']} {readout} {k}: {g} vs {v}")
        out["values_compared"] += len(exact)
        out["readouts"].append(readout)
    return out


# ------------------------------------------------------------------ one case
def closed_form_check(ref, doc):
    """Single-member closed forms in the readout's own leaves (independent of the engine's assembly).

    Cantilever with tip loads: u = Fx L/EA; v = Fy L^3/(3EI)(1 + phi/4) + Mz L^2/(2EI);
    w = Fz L^3/(3EI)(1 + phi/4) - My L^2/(2EI); rx = Mx L/GJ; ry = -Fz L^2/(2EI) + My L/EI;
    rz = Fy L^2/(2EI) + Mz L/EI (local; section rotations, so free of phi).
    Fixed-pinned member with an end moment: the near-end bending moments are (2 - phi)/(4 + phi) of the
    far-end ones, and the torsion is carried whole."""
    if len(ref.members) != 1:
        return 0
    m = ref.members[0]
    sup = doc["supports"]
    if not any(s["kind"] == "rigid" and s["node"] == m.i and len(s["restraints"]) == 6 for s in sup):
        return 0
    if any(ld["node"] != m.j for ld in doc["loads"]):
        return 0
    Fg, Mg = [ZERO] * 3, [ZERO] * 3
    for ld in doc["loads"]:
        (Fg if ld["kind"] == "force" else Mg)["xyz".index(ld["direction"])] += E.exact(ld["value"])
    Fl, Ml = m.to_local(Fg), m.to_local(Mg)
    case = ref.cases["case:1"]
    L, EA, EI, GJ, phi = m.L, m.EA, m.EI, m.GJ, m.phi
    tol = D(10) ** (-(getcontext().prec - 30))
    if len(sup) == 1:
        b = 6 * ref.index[m.j]
        got = m.to_local(case["u"][b:b + 3]) + m.to_local(case["u"][b + 3:b + 6])
        want = [Fl[0] * L / EA,
                Fl[1] * L ** 3 / (3 * EI) * (1 + phi / 4) + Ml[2] * L * L / (2 * EI),
                Fl[2] * L ** 3 / (3 * EI) * (1 + phi / 4) - Ml[1] * L * L / (2 * EI),
                Ml[0] * L / GJ, -Fl[2] * L * L / (2 * EI) + Ml[1] * L / EI, Fl[1] * L * L / (2 * EI) + Ml[2] * L / EI]
        st = max(abs(x) for x in want[:3])
        sr = max(abs(x) for x in want[3:])
        sc = max(st, sr * L)
        for k in range(6):
            if abs(got[k] - want[k]) > tol * (sc if k < 3 else sc / L):
                raise AssertionError(f"cantilever closed form fails, component {k}")
        return 6
    if len(sup) == 2 and any(s["node"] == m.j and s["restraints"] == ["UX", "UY", "UZ"] for s in sup):
        f = case["members"][m.id]["f"]
        ratio = (2 - phi) / (4 + phi)
        sc = max(abs(x) for x in Ml)
        checks = [(f[9], Ml[0]), (f[10], Ml[1]), (f[11], Ml[2]), (f[3], -Ml[0]), (f[4], ratio * Ml[1]), (f[5], ratio * Ml[2])]
        for got, want in checks:
            if abs(got - want) > tol * sc:
                raise AssertionError("propped-member carry-over closed form fails")
        return 6
    return 0


def applicable(mutant, case, ref):
    timo = case["theory"] == "timo"
    differ = any(m.sec["t_r_binary64"] != m.wall for m in ref.members)
    mill = any(m.wall - m.mill != m.wall for m in ref.members)
    if mutant in ("phi_with_metal_area", "euler_bernoulli_member_in_timoshenko_document", "kappa_cowper",
                  "kappa_five_sixths", "annular_lane_reuses_kappa_hat"):
        return (True, None) if timo else (False, "Euler-Bernoulli document")
    if mutant == "kappa_on_reduced_section":
        if not timo:
            return False, "Euler-Bernoulli document"
        return (True, None) if differ else (False, "sections coincide")
    if mutant == "stiffness_on_t_minus_cm":
        return (True, None) if mill else (False, "fl(t - c_m) = t on every member")
    if mutant in STRESS_MUTANTS:
        return (True, None) if differ else (False, "sections coincide")
    raise ValueError(mutant)


def process_case(args):
    case, prec, do_fraction = args
    set_precision(prec)
    doc = case["document"]
    snap = Snap()
    refs, rows = {}, {}
    nfree = None
    closed = 0
    for readout in ("annular", "admitted"):
        ref = solve(doc, readout)
        nfree = check_solution(ref, doc)
        closed += closed_form_check(ref, doc)
        refs[readout] = ref
        rows[readout] = extract(ref, snap)
    ann, adm = refs["annular"], refs["admitted"]
    derived = {"free_dofs": nfree, "members": {}}
    for m in ann.members:
        ma = adm.member_by_id[m.id]
        lv = member_leaves(ma)
        rec = {"L_m": fmt(m.L), "sections_differ": m.sec["t_r_binary64"] != m.wall,
               "t_r_binary64": m.sec["t_r_binary64"], "kappa_E": fmt(m.kappa),
               "kappa_hat": [lv["kappa_hat"], float.hex(lv["kappa_hat"])],
               "shear_area_exact_m2": fmt(m.kappa * m.sec["nominal_section"]["A"]),
               "shear_area_prepared_m2": [lv["shear_area"], float.hex(lv["shear_area"])],
               "G_exact_Pa": fmt(m.G), "G_hat_Pa": [lv["G_hat"], float.hex(lv["G_hat"])],
               "phi_timoshenko": {"annular": fmt(m.o4_phi_timoshenko), "admitted": fmt(ma.o4_phi_timoshenko)},
               "phi_minus_2": {"annular": fmt(snap(m.o4_phi_timoshenko - 2)), "admitted": fmt(snap(ma.o4_phi_timoshenko - 2))},
               "phi_used": {"annular": fmt(m.phi), "admitted": fmt(ma.phi)},
               "governing_end": {b: governing(ann, m.id, b) for b in BASES},
               "sections": {}}
        for b in BASES:
            rec["sections"][b] = {"exact": {k: fmt(m.sec[b][k]) for k in ("wall", "ri", "A", "I", "J", "Z")},
                                  "prepared": {k: [lv[b][k], float.hex(lv[b][k])] for k in ("A", "I", "J", "Z")},
                                  "admitted_Z_hull": fmt_value((ma.o4_stress[b]["Zlo"], ma.o4_stress[b]["Zhi"]))}
        derived["members"][m.id] = rec
    target = max(ann.members, key=lambda mm: mm.o4_phi_timoshenko).id
    differ = {m.id: m.sec["t_r_binary64"] != m.wall for m in ann.members}
    disc = {}
    for mutant in MUTANTS:
        ok, why = applicable(mutant, case, ann)
        if not ok:
            disc[mutant] = {"not_formed": why}
            continue
        rec = {}
        if mutant == "euler_bernoulli_member_in_timoshenko_document":
            rec["member"] = target
        for readout in (("annular",) if mutant == "annular_lane_reuses_kappa_hat" else ("annular", "admitted")):
            if mutant in STRESS_MUTANTS:
                wrong = stress_mutant(rows[readout], mutant, differ)
            else:
                wrong = extract(solve(doc, readout, mutant, target), Snap())
            rec[readout] = compare(rows[readout], wrong)
        disc[mutant] = rec
    content = {"derived": derived,
               "readouts": {r: format_rows(rows[r]) for r in ("annular", "admitted")},
               "readout_gap": readout_gap(rows),
               "binary64_proxy": binary64_proxy(doc, rows["annular"]),
               "discriminators": disc}
    frac = fraction_check(case, content["readouts"]) if do_fraction else None
    stats = {"snapped": snap.snapped, "max_snapped": str(snap.max_snapped), "closed_form": closed}
    return case["id"], content, stats, frac


def exact_readouts(document, prec=WORK_PREC):
    """Both readouts of one request-agnostic case document (for in-memory re-authorings; not frozen)."""
    validate_document(document)
    _, content, _, _ = process_case(({"id": "adhoc", "theory": "timo" if document["shear_model"] == TIMO else "eb",
                                      "document": document}, prec, False))
    return content


# ------------------------------------------------------------------ assembly, census, cross-checks
def run_all(cases, prec, workers, do_fraction):
    tasks = [(c, prec, do_fraction) for c in cases]
    if workers > 1:
        import multiprocessing as mp
        with mp.get_context("spawn").Pool(workers) as pool:
            results = pool.map(process_case, tasks, chunksize=1)
    else:
        results = [process_case(t) for t in tasks]
    return {r[0]: r for r in results}


def phi_band(phi):
    if phi < 1e-4:
        return "[1e-6, 1e-4)" if phi >= 1e-6 else "below 1e-6"
    if phi < 1e-2:
        return "[1e-4, 1e-2)"
    if phi < 1:
        return "[1e-2, 1)"
    if phi < 10:
        return "[1, 10)"
    return "[10, inf)"


def census(cases, contents):
    def count(key):
        out = {}
        for c in cases:
            k = key(c)
            out[k] = out.get(k, 0) + 1
        return dict(sorted(out.items()))

    member_bands, doc_bands, phi2_exact, phi2_near = {}, {}, 0, 0
    for c in cases:
        if c["theory"] != "timo":
            continue
        top = 0.0
        for mid, rec in contents[c["id"]]["derived"]["members"].items():
            phi = float(D(rec["phi_timoshenko"]["annular"][0]))
            top = max(top, phi)
            b = phi_band(phi)
            member_bands[b] = member_bands.get(b, 0) + 1
            if rec["phi_minus_2"]["annular"][0] == "0":
                phi2_exact += 1
            elif abs(D(rec["phi_minus_2"]["annular"][0])) < D("1e-9"):
                phi2_near += 1
        b = phi_band(top)
        doc_bands[b] = doc_bands.get(b, 0) + 1
    sens = [c for c in cases if "sensitive_design" in c["tags"]]
    proxy = [c for c in cases if contents[c["id"]]["binary64_proxy"]["exceeds_1e-9"]]
    return {
        "documents": len(cases),
        "designs": len({c["design"] for c in cases}),
        "by_shear_model": count(lambda c: THEORIES[c["theory"]]),
        "by_allowance_class": count(lambda c: c["variant"]),
        "by_family": count(lambda c: c["family"]),
        "by_placement": count(lambda c: "offset" if "offset" in c["tags"] else "origin"),
        "by_member_direction": count(lambda c: next(t for t in c["tags"] if t in ("axis_aligned", "skew_345", "skew_122", "skew_other"))),
        "timoshenko_members_by_annular_phi_band": dict(sorted(member_bands.items())),
        "timoshenko_documents_by_largest_annular_phi_band": dict(sorted(doc_bands.items())),
        "timoshenko_members_phi_exactly_2": phi2_exact,
        "timoshenko_members_phi_within_1e-9_of_2": phi2_near,
        "sensitive_designs": {"documents": len(sens),
                              "by_tag": count(lambda c: "|".join(t for t in c["tags"] if t.startswith(("soft_spring", "stiff", "short"))) or "none"),
                              "binary64_proxy_exceeds_1e-9": sum(1 for c in sens if c in proxy)},
        "binary64_proxy_exceeds_1e-9_all_documents": len(proxy),
        "source_control_style_documents": sum(1 for c in cases if c["source_control_style"]),
        "rf_style_documents": sum(1 for c in cases if c["family"] == "rf_style_skew_tee"),
        "euler_bernoulli_zero_allowance_harness_twins": sum(1 for c in cases if c["theory"] == "eb" and c["variant"] == "zero"),
        "d1_caps_documents": sum(1 for c in cases if "d1_caps" in c["tags"]),
    }


def string_rows(content, readout, groups):
    out = {}
    for g in groups:
        for path, v in flatten({g: content["readouts"][readout][g]}):
            out[path] = v
    return out


def twin_checks(cases, contents):
    """Cross-document checks: allowance invariance, round-away identity, zero-allowance coincidence,
    theory independence of determinate actions, mirror symmetry."""
    by = {(c["design"], c["variant"], c["theory"]): c for c in cases}
    rec = {"round_away_identical": 0, "allowance_invariant_values": 0, "zero_allowance_bases_coincide": 0,
           "theory_independent_values": 0, "mirror_pairs": 0}
    shared = ("nodal", "member_end_force", "section_action", "support_reaction")
    for c in cases:
        k = (c["design"], c["variant"], c["theory"])
        cont = contents[c["id"]]
        for readout in ("annular", "admitted"):
            if c["variant"] == "roundaway":
                z = contents[by[(c["design"], "zero", c["theory"])]["id"]]
                assert cont["readouts"][readout] == z["readouts"][readout], f"round-away differs {c['id']}"
                rec["round_away_identical"] += 1
            if c["variant"] != "zero":
                z = contents[by[(c["design"], "zero", c["theory"])]["id"]]
                a = string_rows(cont, readout, shared)
                b = string_rows(z, readout, shared)
                assert a == b, f"allowance moved a basis-free row {c['id']}"
                for mid in cont["readouts"][readout]["stress"]:
                    for loc in cont["readouts"][readout]["stress"][mid]:
                        assert cont["readouts"][readout]["stress"][mid][loc]["nominal_section"] == \
                            z["readouts"][readout]["stress"][mid][loc]["nominal_section"], f"nominal stress moved {c['id']}"
                    assert cont["readouts"][readout]["circular_maximum"][mid]["nominal_section"] == \
                        z["readouts"][readout]["circular_maximum"][mid]["nominal_section"]
                rec["allowance_invariant_values"] += len(a)
            else:
                for mid in cont["readouts"][readout]["stress"]:
                    for loc in cont["readouts"][readout]["stress"][mid]:
                        s = cont["readouts"][readout]["stress"][mid][loc]
                        assert s["nominal_section"] == s["reduced_section"]
                    mx = cont["readouts"][readout]["circular_maximum"][mid]
                    assert mx["nominal_section"] == mx["reduced_section"]
                    rec["zero_allowance_bases_coincide"] += 1
            if c["theory_independent_actions"] and c["theory"] == "timo":
                e = contents[by[(c["design"], c["variant"], "eb")]["id"]]
                groups = ("member_end_force", "section_action", "support_reaction", "stress", "circular_maximum")
                a, b = string_rows(cont, readout, groups), string_rows(e, readout, groups)
                assert a == b, f"theory-dependent actions in {c['id']}"
                rec["theory_independent_values"] += len(a)
            for s1, s2, comp, sign in c["mirror_reactions"]:
                r = cont["readouts"][readout]["support_reaction"]
                v1, v2 = D(r[s1][comp][0]), D(r[s2][comp][0])
                assert v1 == sign * v2, f"mirror {c['id']} {comp}"
                rec["mirror_pairs"] += 1
        # phi = 2 exactly: the annular near-end moment of the propped member vanishes, the admitted does not
        if "phi2_exact" in c["tags"] and c["theory"] == "timo":
            for mid, mr in cont["derived"]["members"].items():
                assert mr["phi_minus_2"]["annular"][0] == "0"
                assert mr["phi_minus_2"]["admitted"][0] != "0"
    return rec


def phi2_exact_rational_checks(cases):
    """phi = 2 exactly for the PHI2-EXACT designs, in rationals (pi cancels from I/A)."""
    n = 0
    for c in cases:
        if "phi2_exact" not in c["tags"]:
            continue
        doc = c["document"]
        nu = Fraction(doc["material"]["poisson_ratio"])
        for m in doc["members"]:
            ro = Fraction(m["outside_diameter_m"]) / 2
            ri = ro - Fraction(m["wall_thickness_m"])
            xi = next(x["xyz_m"] for x in doc["nodes"] if x["id"] == m["from"])
            xj = next(x["xyz_m"] for x in doc["nodes"] if x["id"] == m["to"])
            L2 = sum((Fraction(b) - Fraction(a)) ** 2 for a, b in zip(xi, xj))
            phi = 6 * (1 + nu) * (ro * ro + ri * ri) / (kappa_closed(ri / ro, nu) * L2)
            assert phi == 2, f"phi = {phi} in {c['id']}"
            n += 1
    return n


def discriminator_summary(cases, contents):
    out = {}
    for mutant in MUTANTS:
        rec = {"discriminable_in": {}, "not_discriminable_in": {}, "not_formed_in": 0, "smallest_listed_tolerances": {}}
        for c in cases:
            d = contents[c["id"]]["discriminators"][mutant]
            if "not_formed" in d:
                rec["not_formed_in"] += 1
                continue
            for readout in ("annular", "admitted"):
                if readout not in d:
                    continue
                r = d[readout]
                if r["rows_moved_1e3"]:
                    rec["discriminable_in"][readout] = rec["discriminable_in"].get(readout, 0) + 1
                    s = r["listed"][-1]["tolerances"]
                    cur = rec["smallest_listed_tolerances"].get(readout)
                    if cur is None or float(s) < float(cur):
                        rec["smallest_listed_tolerances"][readout] = s
                else:
                    rec["not_discriminable_in"].setdefault(readout, []).append(c["id"])
        for readout in (("annular",) if mutant == "annular_lane_reuses_kappa_hat" else ("annular", "admitted")):
            if not rec["discriminable_in"].get(readout):
                raise AssertionError(f"{mutant} is discriminable in no case on the {readout} readout")
        rec["not_discriminable_count"] = {k: len(v) for k, v in rec["not_discriminable_in"].items()}
        out[mutant] = rec
    return out


def build(workers):
    set_precision(WORK_PREC)
    checks = E.self_check(verbose=False)
    assert len(checks) == 58 and all(ok for _, ok in checks)
    ds = designs()
    cases = [c for d in ds for c in expand(d)]
    for c in cases:
        validate_document(c["document"])
    ids = [c["id"] for c in cases]
    assert len(ids) == len(set(ids))
    n_phi2 = phi2_exact_rational_checks(cases)
    work = run_all(cases, WORK_PREC, workers, True)
    hi = run_all(cases, CHECK_PREC, workers, False)
    set_precision(WORK_PREC)
    contents = {cid: work[cid][1] for cid in ids}
    compared = 0
    for cid in ids:
        a = json.dumps(work[cid][1], sort_keys=False)
        b = json.dumps(hi[cid][1], sort_keys=False)
        if a != b:
            raise AssertionError(f"precision recheck differs in {cid}")
        compared += sum(1 for _ in flatten(work[cid][1]["readouts"]))
    snapped = sum(work[cid][2]["snapped"] for cid in ids)
    max110 = max(D(work[cid][2]["max_snapped"]) for cid in ids)
    max150 = max(D(hi[cid][2]["max_snapped"]) for cid in ids)
    snapped_hi = sum(hi[cid][2]["snapped"] for cid in ids)
    assert max150 < RECHECK_SNAP, f"a snapped residue does not shrink: {max150}"
    frac = [work[cid][3] for cid in ids]
    twins = twin_checks(cases, contents)
    summary = discriminator_summary(cases, contents)
    cases_doc = {"schema": "o4_retained_source_cases_v1",
                 "status": "Frozen; not yet refuted.",
                 "claim_fence": "Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). Every model is invented.",
                 "conventions": "SI units; binary64 inputs taken exactly; request-agnostic documents (see README, Binding)",
                 "cases": cases}
    cases_bytes = dump_lines(cases_doc, "cases", lambda c: c["id"])
    oracle = {
        "schema": "o4_retained_source_oracle_v1",
        "status": "Frozen; not yet refuted.",
        "claim_fence": "Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).",
        "cases_sha256": hashlib.sha256(cases_bytes).hexdigest(),
        "engine": {"path": "../kd5_timoshenko/kd5t_engine.py", "sha256": ENGINE_SHA256, "used": "unchanged, by import",
                   "self_check": f"{len(checks)} checks pass"},
        "conventions": {
            "units": "m, rad, N, N*m, Pa",
            "value": "[decimal with 40 significant digits, binary64]; an interval is {lower, upper}",
            "exact_zero": "\"0\": known exactly, or a residue below 1e-60 SI snapped to zero (README)",
            "nodal": "global displacement and rotation components",
            "member_end_force": "node-on-element end actions in the member's local frame (x chord, y y_reference projected, z = x cross y)",
            "section_action": "j-side (+x face) section action in the local frame at the five stations",
            "stress": "per basis: axial N/A, bending My/Z and Mz/Z, torsion T*r_o/J (Pa); admitted bending over hull(Z-hat, I-hat/r_o)",
            "circular_maximum": "per basis: max over the two ends of |N|/A + hypot(My, Mz)/Z (an unloaded member's maximum is at an end)",
            "support_reaction": "support-on-pipe, global components; a spring's reaction is -k u on its DOF",
            "readouts": "annular: the engine's exact element; admitted: prepared binary64 leaves solved exactly"},
        "criterion": {"relative_limit": "1e-9 * |value| (REQ-001); an admitted interval row passes within 1e-9 of any point of the interval",
                      "discriminator_separation": "|wrong - correct| / (1e-9 |correct|) >= 1e3 on every listed row; exact-zero rows are counted, not listed"},
        "census": census(cases, contents),
        "discriminator_summary": summary,
        "cross_checks": {
            "precision": {"working_digits": WORK_PREC, "recheck_digits": CHECK_PREC, "values_compared": compared,
                          "identical": True, "snapped_residues_at_working": snapped, "snapped_residues_at_recheck": snapped_hi,
                          "largest_snapped_at_working": "%.1e" % max110, "largest_snapped_at_recheck": "%.1e" % max150},
            "rational_resolve": {"documents_eligible": sum(1 for f in frac if f["eligible"]),
                                 "documents_ineligible": sum(1 for f in frac if not f["eligible"]),
                                 "readouts_resolved": sum(len(f.get("readouts", [])) for f in frac),
                                 "values_compared": sum(f.get("values_compared", 0) for f in frac),
                                 "binary64_compared": sum(f.get("binary64_compared", 0) for f in frac),
                                 "all_identical_at_40_digits": True},
            "phi_exactly_2_rational": n_phi2,
            "closed_form_values": sum(work[cid][2]["closed_form"] for cid in ids),
            "twins": twins,
            "per_solve": "engine whole-body balance; equilibrium at every node and support DOF; member end statics; "
                         "K_ff symmetric and positive definite; the unmutated annular element equals the engine's bit for bit"},
        "cases": contents,
    }
    oracle_bytes = dump_lines(oracle, "cases", None)
    return cases_bytes, oracle_bytes


def dump_lines(doc, key, _unused):
    """Indented header members; the `key` collection is written one compact entry per line."""
    j = lambda x: json.dumps(x, ensure_ascii=True, allow_nan=False)  # noqa: E731
    jc = lambda x: json.dumps(x, ensure_ascii=True, allow_nan=False, separators=(",", ":"))  # noqa: E731
    parts = []
    for k, v in doc.items():
        if k != key:
            parts.append(f" {j(k)}: " + json.dumps(v, indent=1, ensure_ascii=True, allow_nan=False).replace("\n", "\n "))
    coll = doc[key]
    if isinstance(coll, dict):
        body = ",\n".join(f"  {j(cid)}: {jc(v)}" for cid, v in coll.items())
        parts.append(f" {j(key)}: {{\n{body}\n }}")
    else:
        body = ",\n".join(f"  {jc(v)}" for v in coll)
        parts.append(f" {j(key)}: [\n{body}\n ]")
    text = "{\n" + ",\n".join(parts) + "\n}\n"
    if json.loads(text) != json.loads(json.dumps(doc)):
        raise AssertionError("line-per-entry serialization does not round-trip")
    return text.encode()


FROZEN = ("cases.json", "oracle.json")
SUMMED = ("README.md", "cases.json", "oracle.json", "run.sh", "generators/o4_reference.py",
          "generators/o4_source_twins.py", "source_twins_cases.json", "source_twins_oracle.json")


def sha256sums():
    lines = []
    for name in SUMMED:
        with open(os.path.join(UNIT, name), "rb") as fh:
            lines.append(f"{hashlib.sha256(fh.read()).hexdigest()}  {name}\n")
    return "".join(lines).encode()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--workers", type=int, default=min(12, os.cpu_count() or 1))
    ap.add_argument("--solve")
    a = ap.parse_args()
    if a.solve:
        with open(a.solve) as fh:
            doc = json.load(fh)
        set_precision(WORK_PREC)
        print(json.dumps(exact_readouts(doc), indent=1))
        return
    cases_bytes, oracle_bytes = build(a.workers)
    if a.write:
        for name, data in zip(FROZEN, (cases_bytes, oracle_bytes)):
            with open(os.path.join(UNIT, name), "wb") as fh:
                fh.write(data)
        with open(os.path.join(UNIT, "SHA256SUMS"), "wb") as fh:
            fh.write(sha256sums())
        print("written")
        return
    bad = []
    for name, data in zip(FROZEN, (cases_bytes, oracle_bytes)):
        with open(os.path.join(UNIT, name), "rb") as fh:
            if fh.read() != data:
                bad.append(name)
    with open(os.path.join(UNIT, "SHA256SUMS"), "rb") as fh:
        if fh.read() != sha256sums():
            bad.append("SHA256SUMS")
    present = sorted(os.path.relpath(os.path.join(dp, f), UNIT) for dp, _, fs in os.walk(UNIT) for f in fs)
    expected = sorted(SUMMED + ("SHA256SUMS",))
    if present != expected:
        bad.append(f"unexpected or missing files: {sorted(set(present) ^ set(expected))}")
    if bad:
        raise SystemExit(f"check FAILED: {bad}")
    print("check passed: frozen files reproduced byte for byte at 110 digits, identical at 150, every cross-check holds")


if __name__ == "__main__":
    main()
