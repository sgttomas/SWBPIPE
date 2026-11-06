#!/usr/bin/env python3
"""RT-A3 §4 combination-certificate references (DEL-05-02 Work `combination-certificate.md` §3).

Standard-library Python 3 only. This generator imports, reads and runs no product
code and reads no file other than the ones it writes. Every value is exact: a
rational q0 plus q1 * pi, with pi by Machin's formula in integer arithmetic,
enclosed in [pi_lo, pi_hi] with pi_hi - pi_lo = 10^-120.

What it states, per reference combination (all models invented):

1. The intended value of each derived group's components, as magnitudes |D_c|
   (closed forms; the reader-side test enforces | |v_D,c| - |D_c| | <= B_D for
   every component, and for a magnitude row (v_M - B_D)^2 <= sum D_c^2 <= (v_M + B_D)^2).
2. The verdict class the hand analysis decides from closed forms and binary64
   publication rounding alone:
   - `not_established` where the operands' own publication gaps already exceed the
     criterion: B_D >= B_O + B_S >= |fl(m_O) - m_O| + |fl(m_S) - m_S| - 2^-100 (|m_O| + |m_S|)
     > RD(1e-9) * q_D (a group bound encloses its published value's distance from
     the intended value, K-D5 §3.1; the 2^-100 term allows for the 128-bit
     intended system), stated together with the assessment condition
     q_D >= 2^-34 * S*_upper;
   - `below_floor` where every intended difference is exactly zero (the published
     difference is exactly zero), conditional on B_D < 2^-34 * S* (stated);
   - `not_assessed` for a combination of gap-bearing operands (R1 standing);
   - D-6: the wall groups of an equal-pressure member name the elastic axial group
     at the same location; the intended wall and elastic differences are equal.

Models:
- the cantilever (EX with a pressure region over its member and caps transferring
  to the wall, and PF without): node:a anchored at the origin, node:b at
  (L, 0, 0), L = fl(0.3); OD = fl(0.2), t = fl(0.01), c_m = fl(0.00125),
  c_a = fl(0.002), p = 2 MPa, E = 200 GPa, nu = fl(0.3); tip forces Fx, Fy.
  Closed forms (statically determinate, EB): N = S = Fx along the member;
  N_w = P + Fx with P = p * pi * (OD/2 - t_r)^2, t_r = fl(fl(t - c_m) - c_a)
  (the binary64 pressure-bore wall leaf); shear |Fy|; bending |Fy| (1 - xi) L at
  xi = 0, 1/4, 1/2, 3/4, 0 at end j; torsion 0; anchor reaction (|Fx|, |Fy|, 0)
  and moment (0, 0, |Fy| L). With y_reference = global z, a global-Y load is
  the local shear z and the local bending y.
- op1a V-2 (DEL-05-02 `pf-nonlinear-operand-standing.md` §4): a propped
  cantilever, L = 4, P = fl(808.6253305030623) at midspan; sustained R = 5P/16,
  M_A = 3PL/16; lifted (prop inactive) R = 0, M_A = PL/2; O - S: dR = 5P/16,
  dM_A = 5PL/16, anchor dFz = 5P/16 (EI-independent statics of a uniform
  propped cantilever and of a cantilever).
- op1a V-1: both operands gap-bearing, so the subtraction is not assessed.

Modes:
  python3 -I certificate_reference.py           check the committed files byte for byte
  python3 -I certificate_reference.py --write   write them (intentional changes only)
"""
import argparse
import hashlib
import json
import struct
import sys
from fractions import Fraction as F
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
FIXED_FILES = ("README.md", "generators/certificate_reference.py")

# ----------------------------------------------------------------------------- pi
DIGITS = 130


def _arctan_inverse(x, scale):
    total, term, n, sign, x2 = 0, scale // x, 1, 1, x * x
    while term:
        total += sign * (term // n)
        term //= x2
        n += 2
        sign = -sign
    return total


_SCALE = 10 ** DIGITS
_PI_SCALED = 4 * (4 * _arctan_inverse(5, _SCALE) - _arctan_inverse(239, _SCALE))
# Truncation of each series term loses < 1 unit per term; 10^-120 encloses it.
PI_LO = F(_PI_SCALED, _SCALE) - F(1, 10 ** 120)
PI_HI = F(_PI_SCALED, _SCALE) + F(1, 10 ** 120)


class V:
    """q0 + q1 * pi, exact."""

    def __init__(self, q0=0, q1=0):
        self.q0, self.q1 = F(q0), F(q1)

    def __add__(self, o):
        return V(self.q0 + o.q0, self.q1 + o.q1)

    def __sub__(self, o):
        return V(self.q0 - o.q0, self.q1 - o.q1)

    def scale(self, k):
        return V(self.q0 * k, self.q1 * k)

    def bounds(self):
        a, b = self.q0 + self.q1 * PI_LO, self.q0 + self.q1 * PI_HI
        return (min(a, b), max(a, b))

    def magnitude(self):
        """|v| as an enclosure, refused where the sign is not decided."""
        lo, hi = self.bounds()
        if lo >= 0:
            return V(self.q0, self.q1)
        if hi <= 0:
            return V(-self.q0, -self.q1)
        if self.q1 == 0 and self.q0 == 0:
            return V()
        raise ValueError("sign undecided")

    def json(self):
        lo, _ = self.bounds()
        return {"q0": str(self.q0), "q1_pi": str(self.q1), "decimal": f"{float(lo):.17e}"}


def nearest(v):
    """The binary64 nearest the exact value, decided at both pi bounds."""
    lo, hi = v.bounds()
    a, b = float(lo), float(hi)
    if a != b:
        raise ValueError("rounding undecided")
    return a


def bits(x):
    return struct.unpack(">Q", struct.pack(">d", x))[0]


CRITERION = struct.unpack(">d", struct.pack(">Q", bits(1e-9) - 1))[0]
FLOOR = F(1, 2 ** 34)
INTENDED_MARGIN = F(1, 2 ** 100)

# ----------------------------------------------------------------------------- cantilever
L = F(0.3)
OD, T, CM, CA, P_PA = F(0.2), F(0.01), F(0.00125), F(0.002), F(2.0e6)
T_R = F((0.01 - 0.00125) - 0.002)  # the binary64 pressure-bore wall leaf
R_P = OD / 2 - T_R
P = V(0, P_PA * R_P * R_P)  # p * pi * r_p^2
FACTOR = F(1.3)  # the represented binary64 factor
TINY = {"-16": F(1, 2 ** 16), "-10": F(1, 2 ** 10)}
CASES = {
    "case:S": (F(1000), F(600)),
    "case:S2": (F(1000), F(600)),
    "case:O2": (F(2000), F(1200)),
    "case:Oc": (F(1000), F(600) + TINY["-16"]),
    "case:Oa": (F(1000) + TINY["-10"], F(600)),
    "case:Q": (F(-400), F(200)),
}
SUBTRACTIONS = {"comb:double": ("case:O2", "case:S"), "comb:cancel": ("case:Oc", "case:S"),
                "comb:same": ("case:S2", "case:S"), "comb:axial": ("case:Oa", "case:S")}
STATIONS = [("end_i", F(0)), ("quarter_1", F(1, 4)), ("midspan", F(1, 2)), ("quarter_3", F(3, 4))]


def case_groups(fx, fy, pressure, exact_route):
    """Signed intended components of every covered group of one load state.
    `pressure` is the coefficient of P in the wall force (1 for a case)."""
    fx, fy = V(fx), V(fy)
    wall = P.scale(pressure) + fx
    out = {}
    for location, xi in STATIONS + [("end_j", F(1))]:
        out[("pipe:m", location, "shear")] = [V(), fy]
        out[("pipe:m", location, "torsion")] = [V()]
        out[("pipe:m", location, "bending")] = [fy.scale(L * (1 - xi)), V()]
        if not exact_route:
            out[("pipe:m", location, "axial")] = [fx]
    if exact_route:
        for location in ["cut_i", "quarter_1", "midspan", "quarter_3", "cut_j"]:
            out[("pipe:m", location, "axial")] = [fx]
            out[("pipe:m", "wall_" + location, "pipe_wall")] = [wall]
        for location in ["wall_end_i", "wall_end_j"]:
            out[("pipe:m", location, "pipe_wall")] = [wall]
    out[("support:a", "node", "support_force")] = [fx, fy, V()]
    out[("support:a", "node", "support_moment")] = [V(), V(), fy.scale(L)]
    return out


def magnitudes(components):
    return [c.magnitude().json() for c in components]


def published(v):
    return nearest(v)


def scale_upper(states, exact_route):
    """Coupled floor scales (force, moment) of a case, K-D5 §3.3, as an upper bound:
    the largest published force and moment rows, coupled through the body extent L."""
    force = max(abs(float(c.bounds()[1])) for k, comps in states.items() if k[2] in ("axial", "shear", "pipe_wall", "support_force") for c in comps)
    fx, fy = (states[("support:a", "node", "support_force")][i] for i in (0, 1))
    force = max(force, float((fx.q0 ** 2 + fy.q0 ** 2) ** 1) ** 0.5 * (1 + 2 ** -50))
    moment = max(abs(float(c.bounds()[1])) for k, comps in states.items() if k[2] in ("bending", "torsion", "support_moment") for c in comps)
    lf = float(L)
    return (max(force, moment / lf) * (1 + 2 ** -40), max(moment, lf * force) * (1 + 2 ** -40))


def cantilever(exact_route):
    states = {case: case_groups(fx, fy, 1, exact_route) for case, (fx, fy) in CASES.items()}
    out = []
    for cid, (minuend, subtrahend) in SUBTRACTIONS.items():
        o, s = states[minuend], states[subtrahend]
        so, ss = scale_upper(o, exact_route), scale_upper(s, exact_route)
        groups = []
        for key in o:
            d = [a - b for a, b in zip(o[key], s[key])]
            moment = key[2] in ("bending", "torsion", "support_moment")
            star = max(so[1], ss[1]) if moment else max(so[0], ss[0])
            entry = {"entity": key[0], "location": key[1], "group": key[2], "components": magnitudes(d)}
            # The published rows and their differences, binary64 (publication, not product code).
            vo = [published(c) for c in o[key]]
            vs = [published(c) for c in s[key]]
            vd = [a - b for a, b in zip(vo, vs)]
            entry["operand_published"] = [vo, vs]
            entry["derived_published"] = vd
            if all(c.q0 == 0 and c.q1 == 0 for c in d):
                entry["expected"] = {"verdict": "not_assessed", "reason": "below_floor",
                                     "condition": "every intended difference is exactly zero; holds when B_D < 2^-34 * S*"}
            else:
                q = F(max(abs(x) for x in vd)) if len(vd) == 1 else None
                gaps = F(0)
                for comps, pub in ((o[key], vo), (s[key], vs)):
                    for c, v in zip(comps, pub):
                        lo, hi = c.bounds()
                        gap = min(abs(F(v) - lo), abs(F(v) - hi)) if not (lo <= F(v) <= hi) else F(0)
                        gaps += gap - INTENDED_MARGIN * abs(hi)
                qd = sum(F(x) ** 2 for x in vd)
                # q_D^2 <= qd (exact): compare gaps^2 > CRITERION^2 * qd * (1 + 2^-50)^2.
                assessed = qd >= (FLOOR * F(star)) ** 2
                if gaps > 0 and gaps * gaps > F(CRITERION) ** 2 * qd * F(1 + 2 ** -50) ** 2 and assessed:
                    entry["expected"] = {"verdict": "not_established", "reason": "bound_exceeds_criterion",
                                         "gap_lower_bound": f"{float(gaps):.17e}", "q_d": f"{float(qd) ** 0.5:.17e}",
                                         "floor_scale_upper": f"{star:.17e}"}
            if exact_route and key[2] == "pipe_wall":  # every subtraction here keeps the same pressure
                elastic = key[1][len("wall_"):]
                entry["equal_pressure"] = {"elastic_group": None if elastic in ("end_i", "end_j") else {"entity": "pipe:m", "location": elastic, "group": "axial"},
                                           "identity": "D(N_w) = D(N_el): both operands carry the same P"}
            groups.append(entry)
        out.append({"combination_id": cid, "basis": "result_state_subtraction", "minuend": minuend, "subtrahend": subtrahend, "groups": groups})
    # The mechanics sum S + 1.3 Q, with the represented factor.
    s, q = CASES["case:S"], CASES["case:Q"]
    d = case_groups(s[0] + FACTOR * q[0], s[1] + FACTOR * q[1], 1 + FACTOR, exact_route)
    out.append({"combination_id": "comb:sum", "basis": "mechanics", "terms": [["case:S", "1.0"], ["case:Q", str(FACTOR)]],
                "groups": [{"entity": k[0], "location": k[1], "group": k[2], "components": magnitudes(v)} for k, v in d.items()]})
    return {"route": "exact_pressure_v3" if exact_route else "preview_physics", "combinations": out}


# ----------------------------------------------------------------------------- op1a
def op1a():
    p, l = F(808.6253305030623), F(4)
    return {
        "v2": {"combination_id": "comb:lift", "basis": "result_state_subtraction", "minuend": "case:O-LIFT", "subtrahend": "case:S",
               "groups": [
                   {"entity": "support:PROP", "location": "node", "group": "support_force", "components": magnitudes([V(), V(), V(5 * p / 16)])},
                   {"entity": "support:ANCHOR", "location": "node", "group": "support_moment", "components": magnitudes([V(), V(5 * p * l / 16), V()])},
                   {"entity": "support:ANCHOR", "location": "node", "group": "support_force", "components": magnitudes([V(), V(), V(5 * p / 16)])},
               ],
               "operands": "both confirmed (checks_passed): the combination is assessed"},
        "v1": {"combination_id": "comb:o-s", "expected": {"assessment": "not_assessed",
               "reason": "operand case(s) case:O, case:S carry no certified rows",
               "basis": "both operands are gap-bearing and NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED (R1); no member bound is formed until G2"}},
    }


def references():
    return {
        "reference": "RT-A3 §4 combination certificate (DEL-05-02 Work combination-certificate.md §3)",
        "generator": "generators/certificate_reference.py",
        "pi": {"digits": DIGITS, "enclosure": "Machin, +-1e-120"},
        "criterion": {"rd_1e_9_hex": f"{bits(CRITERION):016x}", "floor": "2^-34", "intended_margin": "2^-100"},
        "inputs": {"L": str(L), "OD": str(OD), "t": str(T), "c_m": str(CM), "c_a": str(CA), "t_r": str(T_R), "p": str(P_PA),
                   "factor_1_3": str(FACTOR), "cases": {k: [str(a), str(b)] for k, (a, b) in CASES.items()}},
        "P": P.json(),
        "cantilever_exact": cantilever(True),
        "cantilever_preview": cantilever(False),
        "op1a": op1a(),
    }


def render():
    text = json.dumps(references(), indent=1, ensure_ascii=True) + "\n"
    files = {"references.json": text.encode("ascii")}
    for name in FIXED_FILES:
        files[name] = (OUT / name).read_bytes()
    sums = "".join(f"{hashlib.sha256(files[name]).hexdigest()}  {name}\n" for name in sorted(files))
    files["SHA256SUMS"] = sums.encode("ascii")
    return files


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    files = render()
    if args.write:
        for name, data in files.items():
            if name not in FIXED_FILES:
                (OUT / name).write_bytes(data)
        print("written")
        return 0
    stale = [name for name, data in files.items() if not (OUT / name).exists() or (OUT / name).read_bytes() != data]
    if stale:
        print("stale: " + ", ".join(sorted(stale)))
        return 1
    print("current")
    return 0


if __name__ == "__main__":
    sys.exit(main())
