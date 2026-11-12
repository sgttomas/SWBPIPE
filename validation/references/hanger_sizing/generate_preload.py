#!/usr/bin/env python3
"""Independent reference generator: the variable spring preload (DEL-04-11 REQ-014).

Writes preload.json: closed-form cases for a variable spring hanger modelled as
a spring of its rate plus its cold load as a constant force along its degree of
freedom (DEL-04-03 REQ-014 and AC-009; DEL-04-11 REQ-014 and AC-014). Each case
gives the hanger's displacement and every published support force, the spring's
including the preload, F_cold - k (u - g).

Independence: written from execution/PKG-04/DEL-04-11/ScopeOfWork.md (REQ-014,
AC-014), execution/PKG-04/DEL-04-03/ScopeOfWork.md (REQ-014, AC-009),
DEL-05-01's definitions of the test condition and held support, and the task
brief only. It reads, searches and imports no product code and no other
generator, and shares no code with any. Standard library only. Every value is
invented.

Method: each expected value comes from a closed form written below for a
simply supported span with springs (flexibility method) or, for a held spring,
the three-moment equation. Each closed form is checked exactly against an
exact-rational stiffness solve of horizontal Euler-Bernoulli beam elements
written here; equilibrium of every solve is asserted, and so is the
force-plus-spring identity (each case re-solved with the hanger as a plain
spring and a separate force equal to its cold load). pi enters through E I
only; it is bounded by Machin's formula and every emitted binary64 must agree
at both bounds and with binary64-replaced inputs.

Usage:
    python3 generate_preload.py          # write preload.json
    python3 generate_preload.py --check  # exit 1 if the frozen JSON differs
"""

import argparse
import json
import math
import sys
from fractions import Fraction
from pathlib import Path

SCHEMA = "swbpipe.hanger_sizing_reference.preload/1"
OUT = Path(__file__).resolve().parent / "preload.json"


def machin_pi(digits=120):
    """(lower, middle, upper) rational bounds of pi from Machin's formula."""
    one = 10 ** digits

    def series(n):
        acc, p, j = 0, one // n, 0
        while p:
            acc += (p // (2 * j + 1)) * (-1 if j & 1 else 1)
            p //= n * n
            j += 1
        return acc, j

    s5, j5 = series(5)
    s239, j239 = series(239)
    mid = 16 * s5 - 4 * s239
    slack = 16 * (2 * j5 + 1) + 4 * (2 * j239 + 1)
    out = Fraction(mid - slack, one), Fraction(mid, one), Fraction(mid + slack, one)
    assert out[0] < out[1] < out[2] and float(out[1]) == math.pi
    return out


PI = machin_pi()


# ---------------------------------------------------------------- beam solve


def solve_linear(m, r):
    n = len(r)
    a = [list(m[i]) + [r[i]] for i in range(n)]
    for i in range(n):
        piv = next(j for j in range(i, n) if a[j][i])
        a[i], a[piv] = a[piv], a[i]
        inv = 1 / a[i][i]
        a[i] = [v * inv for v in a[i]]
        for j in range(n):
            if j != i and a[j][i]:
                fac = a[j][i]
                a[j] = [x - fac * y for x, y in zip(a[j], a[i])]
    return [a[i][n] for i in range(n)]


def beam_solve(xs, EA, EI, fixed, springs, loads):
    """Horizontal beam along X through nodes at xs (ascending), DOFs ux, uz, ry.

    fixed: {(node index, dof): prescribed}; springs: [(name, node index, k,
    base g, preload)] on uz, force on the pipe preload - k (uz - g);
    loads: [(node index, dof, value)]. Returns uz per node, the force on the
    pipe at each fixed DOF and each spring's force on the pipe.
    """
    n = 3 * len(xs)
    K = [[Fraction(0)] * n for _ in range(n)]
    P = [Fraction(0)] * n
    for e in range(len(xs) - 1):
        L = xs[e + 1] - xs[e]
        ka = EA / L
        c1, c2, c3, c4 = 12 * EI / L ** 3, 6 * EI / L ** 2, 4 * EI / L, 2 * EI / L
        d = [3 * e, 3 * e + 1, 3 * e + 2, 3 * e + 3, 3 * e + 4, 3 * e + 5]
        ke = [[ka, 0, 0, -ka, 0, 0], [0, c1, c2, 0, -c1, c2], [0, c2, c3, 0, -c2, c4],
              [-ka, 0, 0, ka, 0, 0], [0, -c1, -c2, 0, c1, -c2], [0, c2, c4, 0, -c2, c3]]
        for i in range(6):
            for j in range(6):
                K[d[i]][d[j]] += ke[i][j]
    col = {"ux": 0, "uz": 1, "ry": 2}
    for node, dof, v in loads:
        P[3 * node + col[dof]] += v
    for _, node, k, g, pre in springs:
        K[3 * node + 1][3 * node + 1] += k
        P[3 * node + 1] += k * g + pre
    pres = {3 * nd + col[dof]: Fraction(v) for (nd, dof), v in fixed.items()}
    free = [i for i in range(n) if i not in pres]
    sol = solve_linear([[K[i][j] for j in free] for i in free],
                       [P[i] - sum(K[i][j] * v for j, v in pres.items()) for i in free])
    u = dict(pres)
    u.update(zip(free, sol))
    reac = {key: sum(K[3 * key[0] + col[key[1]]][j] * u[j] for j in range(n))
            - P[3 * key[0] + col[key[1]]] for key in fixed}
    spr = {name: pre - k * (u[3 * node + 1] - g) for name, node, k, g, pre in springs}
    tot = (sum(v for _, dof, v in loads if dof == "uz")
           + sum(v for key, v in reac.items() if key[1] == "uz") + sum(spr.values()))
    assert tot == 0, tot
    assert sum(v for _, dof, v in loads if dof == "ux") + sum(
        v for key, v in reac.items() if key[1] == "ux") == 0
    return [u[3 * i + 1] for i in range(len(xs))], reac, spr


# ---------------------------------------------------------------- inputs

SEC = {"D": "0.25", "t": "0.01", "E": "200000000000", "nu": "0.3"}

# Model P-A: one span of 8 m on rigid ends, one spring hanger at midspan.
PA = {
    "x": ["0", "2", "4", "6", "8"],
    "rigid": [("S-1", 0), ("S-5", 4)],
    "hangers": [("SH-3", 2, "150000", "6000")],
    "loads": [("F-A", 1, "-5000"), ("F-B", 3, "-7000"), ("F-C", 3, "-3000"),
              ("F-H1", 1, "-4000"), ("F-H2", 3, "-4000")],
    "test_pressure": "6000000",
    # id, included loads, base displacement {hanger: g}, test condition
    "cases": [
        ("PA-WT", ["F-A", "F-B"], {}, None),
        ("PA-EMPTY", [], {}, None),
        ("PA-BASE", ["F-A", "F-B"], {"SH-3": "-0.004"}, None),
        ("PA-B", ["F-C"], {}, None),
        ("PA-AB", ["F-A", "F-B", "F-C"], {}, None),
        ("PA-HT", ["F-A", "F-B", "F-H1", "F-H2"], {}, "springs_and_constant_effort"),
        ("PA-HT-NONE", ["F-A", "F-B", "F-H1", "F-H2"], {}, "none"),
    ],
}

# Model P-B: one 12 m span on rigid ends, two spring hangers.
PB = {
    "x": ["0", "2", "4", "6", "8", "10", "12"],
    "rigid": [("S-1", 0), ("S-7", 6)],
    "hangers": [("SH-3", 2, "120000", "5000"), ("SH-5", 4, "200000", "7500")],
    "loads": [("F-1", 1, "-6000"), ("F-2", 3, "-9000"), ("F-3", 5, "-4000")],
    "test_pressure": None,
    "cases": [
        ("PB-WT", ["F-1", "F-2", "F-3"], {}, None),
        ("PB-EMPTY", [], {}, None),
        ("PB-BASE", ["F-1", "F-2", "F-3"], {"SH-5": "0.003"}, None),
    ],
}


class Inputs:
    def __init__(self, pi, binary):
        self.pi, self.binary = pi, binary

    def v(self, s):
        x = Fraction(s)
        return Fraction(float(x)) if self.binary else x


def props(inp):
    D, t, E = inp.v(SEC["D"]), inp.v(SEC["t"]), inp.v(SEC["E"])
    d = D - 2 * t
    return E * inp.pi * (D * D - d * d) / 4, E * inp.pi * (D ** 4 - d ** 4) / 64


def evaluate(model, inp):
    EA, EI = props(inp)
    xs = [inp.v(s) for s in model["x"]]
    Ltot = xs[-1] - xs[0]
    loads = {lid: (nd, inp.v(v)) for lid, nd, v in model["loads"]}
    hang = [(h, nd, inp.v(k), inp.v(fc)) for h, nd, k, fc in model["hangers"]]

    def flex(xa, xb):
        """Simply supported span: deflection at xa per unit upward force at xb."""
        lo, hi = min(xa, xb), max(xa, xb)
        b = Ltot - hi
        return b * lo * (Ltot ** 2 - b ** 2 - lo ** 2) / (6 * EI * Ltot)

    out = {}
    for cid, inc, base, test in model["cases"]:
        fl = [(loads[l][0], loads[l][1]) for l in inc]
        g = {h: inp.v(base.get(h, "0")) for h, _, _, _ in hang}
        held = test == "springs_and_constant_effort"
        fixed = {}
        for _, nd in model["rigid"]:
            fixed[(nd, "uz")] = 0
        fixed[(0, "ux")] = 0
        springs = []
        if held:
            for h, nd, _, _ in hang:
                fixed[(nd, "uz")] = 0
        else:
            springs = [(h, nd, k, g[h], fc) for h, nd, k, fc in hang]
        uz, reac, spr = beam_solve(xs, EA, EI, fixed, springs,
                                   [(nd, "uz", v) for nd, v in fl])
        # Pressure on the closed run with one axial restraint: an end-cap
        # thrust pair along X changes no vertical quantity.
        if test is not None:
            D, t = inp.v(SEC["D"]), inp.v(SEC["t"])
            pa = inp.v(model["test_pressure"]) * inp.pi * (D - 2 * t) ** 2 / 4
            uz2, reac2, spr2 = beam_solve(
                xs, EA, EI, fixed, springs,
                [(nd, "uz", v) for nd, v in fl]
                + [(0, "ux", -pa), (len(xs) - 1, "ux", pa)])
            assert uz2 == uz and spr2 == spr and all(
                reac2[k] == v for k, v in reac.items() if k[1] == "uz")
        # Closed forms.
        if held:
            # Two equal spans with the hanger rigid, point loads at midspans:
            # three-moment equation, M_B = -3 L (P1 + P2) / 32.
            assert len(hang) == 1 and len(xs) == 5
            Ls = xs[2] - xs[0]
            P1 = -sum(v for nd, v in fl if nd == 1)
            P2 = -sum(v for nd, v in fl if nd == 3)
            assert xs[1] - xs[0] == Ls / 2 and xs[3] - xs[2] == Ls / 2
            MB = -3 * Ls * (P1 + P2) / 32
            cf = {"S-1": P1 / 2 + MB / Ls, hang[0][0]: P1 / 2 + P2 / 2 - 2 * MB / Ls,
                  "S-5": P2 / 2 + MB / Ls}
            cu = {hang[0][0]: Fraction(0)}
            assert reac[(hang[0][1], "uz")] == cf[hang[0][0]]
            # Every node: the 8 m span under the loads and the held reaction.
            for i, x in enumerate(xs):
                assert uz[i] == sum(v * flex(x, xs[nd]) for nd, v in fl) + (
                    cf[hang[0][0]] * flex(x, xs[hang[0][1]]))
        else:
            # Flexibility: u_i = sum_j f_ij F_j + sum_h f_ih (F_cold,h - k_h (u_h - g_h)).
            # Solve the hanger equations (one or two) by Cramer's rule.
            H = len(hang)
            A = [[(1 if i == j else 0) + flex(xs[hang[i][1]], xs[hang[j][1]]) * hang[j][2]
                  for j in range(H)] for i in range(H)]
            r = [sum(v * flex(xs[hang[i][1]], xs[nd]) for nd, v in fl)
                 + sum(flex(xs[hang[i][1]], xs[hang[j][1]])
                       * (hang[j][3] + hang[j][2] * g[hang[j][0]]) for j in range(H))
                 for i in range(H)]
            if H == 1:
                sol = [r[0] / A[0][0]]
            else:
                det = A[0][0] * A[1][1] - A[0][1] * A[1][0]
                sol = [(r[0] * A[1][1] - A[0][1] * r[1]) / det,
                       (A[0][0] * r[1] - A[1][0] * r[0]) / det]
            cu = {hang[i][0]: sol[i] for i in range(H)}
            cf = {h: fc - k * (cu[h] - g[h]) for h, _, k, fc in hang}
            allf = [(xs[nd], v) for nd, v in fl] + [(xs[nd], cf[h]) for h, nd, _, _ in hang]
            cf[model["rigid"][0][0]] = -sum(v * (Ltot - x) for x, v in allf) / Ltot
            cf[model["rigid"][1][0]] = -sum(v * x for x, v in allf) / Ltot
            for h, nd, _, _ in hang:
                assert uz[nd] == cu[h] and spr[h] == cf[h]
            # Every node's displacement from the flexibility of the span.
            for i, x in enumerate(xs):
                assert uz[i] == sum(v * flex(x, xp) for xp, v in allf)
            # Force-plus-spring identity: a plain spring (no preload) and a
            # separately applied force equal to the cold load.
            uz_t, reac_t, spr_t = beam_solve(
                xs, EA, EI, fixed, [(h, nd, k, g[h], 0) for h, nd, k, _ in hang],
                [(nd, "uz", v) for nd, v in fl] + [(nd, "uz", fc) for _, nd, _, fc in hang])
            assert uz_t == uz and reac_t == reac
            for h, nd, k, fc in hang:
                assert spr_t[h] == -k * (cu[h] - g[h]) and spr_t[h] + fc == cf[h]
        for name, nd in model["rigid"]:
            assert reac[(nd, "uz")] == cf[name]
        assert reac[(0, "ux")] == 0
        out[cid] = {"u_nodes": uz, "u_hanger": cu, "forces": cf, "g": g,
                    "held": held, "test": test, "loads": fl}
    return out


# ---------------------------------------------------------------- emit

BASIS = (
    "Independent reference for the variable spring preload (DEL-04-03 REQ-014 and "
    "AC-009; DEL-04-11 REQ-014 and AC-014). A variable spring hanger is a spring "
    "of its rate k on UZ plus its cold load F_cold as a constant force along +Z, "
    "acting in every load case. It carries F_cold - k (u - g), where u is the "
    "node's UZ displacement from the drawn geometry and g the displacement of the "
    "spring's base along its axis (zero unless stated); that is its published "
    "support force. A held spring (a test case whose test condition is "
    "springs_and_constant_effort) is a rigid UZ restraint at zero displacement, "
    "and neither its stiffness nor its cold load acts; its published force is "
    "the restraint's reaction. Straight horizontal Euler-Bernoulli beams along "
    "+X with axial deformation, shear neglected, small displacements, one "
    "circular tube section: d = D - 2t, A = pi (D^2 - d^2)/4, I = pi (D^4 - d^4)/64. "
    "Z is vertical, positive up; SI units (m, N, Pa, N/m). Every load is a stored "
    "nodal force along Z; no self-weight, contents weight or line load is "
    "generated. Every published force is the force the support exerts on the "
    "pipe, positive along +Z. Values containing E I contain pi; they are "
    "computed exactly at rational lower and upper bounds of pi (Machin's "
    "formula, 120 digits) and emitted rounded to the nearest binary64, which "
    "agrees at both bounds and when every input is replaced by its binary64. "
    "Each closed form agrees exactly with an exact-rational stiffness solve. "
    "Every value is invented. See README.md for the derivations.")

DESC = {
    "P-A": (
        "Span with one spring hanger. Nodes N1..N5 at x = 0, 2, 4, 6, 8 m; members "
        "M1..M4; S-1 (N1) and S-5 (N5) rigid UZ; G-1 (N1: UX, UY, RX) and G-5 (N5: "
        "UY) restrain the out-of-plane and axial motion, UX at N1 only. Spring "
        "hanger SH-3 at N3 (UZ): k = 150000 N/m, cold load 6000 N. Stored loads "
        "F-A -5000 N at N2, F-B -7000 N at N4, F-C -3000 N at N4, F-H1 -4000 N at "
        "N2 and F-H2 -4000 N at N4 (test water). Closed form: simply supported "
        "8 m span, f(x, a) = b x (L^2 - b^2 - x^2) / (6 E I L) with b = L - a and "
        "x <= a (symmetric); u_3 = (sum_j f(4, x_j) F_j + f(4, 4)(F_cold + k g)) / "
        "(1 + k f(4, 4)); spring force F_cold - k (u_3 - g); end reactions by "
        "statics. Held: two 4 m spans, three-moment M_B = -3 L (P1 + P2)/32, "
        "R_S1 = R_S5 = P/2 + M_B/L, R_SH3 = (P1 + P2)/2 - 2 M_B/L, with P1 and P2 "
        "the downward loads at N2 and N4."),
    "P-B": (
        "Two spring hangers on one span. Nodes N1..N7 at x = 0, 2, ..., 12 m; S-1 "
        "(N1) and S-7 (N7) rigid UZ; G-1 (N1: UX, UY, RX), G-7 (N7: UY). SH-3 at N3 "
        "(x = 4): k = 120000 N/m, cold load 5000 N; SH-5 at N5 (x = 8): "
        "k = 200000 N/m, cold load 7500 N. Stored loads F-1 -6000 N at N2, F-2 "
        "-9000 N at N4, F-3 -4000 N at N6. Closed form: the two hanger equations "
        "u_i + sum_j f_ij k_j u_j = sum_l f_il F_l + sum_j f_ij (F_cold,j + k_j g_j) "
        "for the simply supported 12 m span, solved by Cramer's rule."),
}

CASE_DESC = {
    "PA-WT": "Weight: F-A and F-B; the cold load acts.",
    "PA-EMPTY": "No load of its own; the cold load still acts and lifts the span.",
    "PA-BASE": "F-A and F-B with SH-3's base moved g = -0.004 m along +Z.",
    "PA-B": "Case B of the superposition statement: F-C only.",
    "PA-AB": "The loads of PA-WT and PA-B solved directly, with one cold load.",
    "PA-HT": ("Test case holding springs (springs_and_constant_effort), test "
              "pressure 6 MPa: SH-3 is a rigid UZ restraint at zero; neither its "
              "stiffness nor its cold load acts."),
    "PA-HT-NONE": ("The same test case with held supports 'none': SH-3 acts as in "
                   "any case, with its cold load."),
    "PB-WT": "Weight: F-1, F-2 and F-3; both cold loads act.",
    "PB-EMPTY": "No load of its own; both cold loads act.",
    "PB-BASE": "PB-WT's loads with SH-5's base moved g = +0.003 m along +Z.",
}


def fl(x):
    return float(x)


def emit(res):
    models = []
    for mid, model in (("P-A", PA), ("P-B", PB)):
        r = res[mid]
        xs = model["x"]
        hang = model["hangers"]
        cases = []
        for cid, inc, base, test in model["cases"]:
            c = r[cid]
            entry = {
                "id": cid, "description": CASE_DESC[cid],
                "includes": [{"load": l, "factor": 1.0} for l in inc],
                "spring_base_displacements": [
                    {"support": h, "dof": "UZ", "value_m": fl(Fraction(g))}
                    for h, g in base.items()],
                "test_condition": ({"held_supports": test,
                                    "test_pressure_pa": fl(Fraction(model["test_pressure"]))}
                                   if test else None),
                "expected": {
                    "u_hanger_m": {h: fl(v) for h, v in c["u_hanger"].items()},
                    "u_nodes_m": {"N%d" % (i + 1): fl(v)
                                  for i, v in enumerate(c["u_nodes"])},
                    "published_support_force_n": {k: fl(v) for k, v in c["forces"].items()},
                },
            }
            if not c["held"]:
                entry["expected"]["force_plus_spring_twin"] = {
                    h: {"plain_spring_force_n": fl(c["forces"][h] - r["_fc"][h]),
                        "applied_force_n": fl(r["_fc"][h]),
                        "displacements": "identical to u_nodes_m"}
                    for h, _, _, _ in hang}
            cases.append(entry)
        m = {
            "id": mid, "description": DESC[mid],
            "section": {"outside_diameter_m": fl(Fraction(SEC["D"])),
                        "wall_thickness_m": fl(Fraction(SEC["t"])),
                        "elastic_modulus_pa": fl(Fraction(SEC["E"])),
                        "poisson_ratio": fl(Fraction(SEC["nu"]))},
            "nodes": {"N%d" % (i + 1): [fl(Fraction(x)), 0.0, 0.0] for i, x in enumerate(xs)},
            "members": [{"id": "M%d" % (i + 1), "end_nodes": ["N%d" % (i + 1), "N%d" % (i + 2)]}
                        for i in range(len(xs) - 1)],
            "supports": (
                [{"id": s, "node": "N%d" % (nd + 1), "kind": "rigid", "dofs": ["UZ"]}
                 for s, nd in model["rigid"]]
                + [{"id": "G-1", "node": "N1", "kind": "rigid", "dofs": ["UX", "UY", "RX"]},
                   {"id": "G-%d" % len(xs), "node": "N%d" % len(xs), "kind": "rigid",
                    "dofs": ["UY"]}]
                + [{"id": h, "node": "N%d" % (nd + 1), "kind": "variable_spring_hanger",
                    "dofs": ["UZ"], "stiffness_n_per_m": fl(Fraction(k)),
                    "cold_load_n": fl(Fraction(fc))} for h, nd, k, fc in hang]),
            "stored_loads": [{"id": l, "kind": "nodal_force", "node": "N%d" % (nd + 1),
                              "direction": "+Z", "value_n": fl(Fraction(v))}
                             for l, nd, v in model["loads"]],
            "load_cases": cases,
        }
        models.append(m)
    # Superposition statement on P-A: A = PA-WT, B = PA-B.
    r = res["P-A"]
    A, B, AB, E = r["PA-WT"], r["PA-B"], r["PA-AB"], r["PA-EMPTY"]
    naive_u = [a + b for a, b in zip(A["u_nodes"], B["u_nodes"])]
    naive_f = {k: A["forces"][k] + B["forces"][k] for k in A["forces"]}
    diff_u = [n - d for n, d in zip(naive_u, AB["u_nodes"])]
    diff_f = {k: naive_f[k] - AB["forces"][k] for k in naive_f}
    assert diff_u == E["u_nodes"] and diff_f == E["forces"]
    sup = {
        "model": "P-A", "case_a": "PA-WT", "case_b": "PA-B", "direct_case": "PA-AB",
        "factors": {"PA-WT": 1.0, "PA-B": 1.0},
        "statement": (
            "The direct solve of the loads of A plus B carries one cold load. The "
            "naive sum of the solved states of A and B (factor sum 2) carries it "
            "twice. Their difference, naive minus direct, is exactly the response "
            "to one extra cold load alone, which is case PA-EMPTY."),
        "direct_u_nodes_m": {"N%d" % (i + 1): fl(v) for i, v in enumerate(AB["u_nodes"])},
        "naive_sum_u_nodes_m": {"N%d" % (i + 1): fl(v) for i, v in enumerate(naive_u)},
        "naive_minus_direct_u_nodes_m": {"N%d" % (i + 1): fl(v)
                                         for i, v in enumerate(diff_u)},
        "direct_published_support_force_n": {k: fl(v) for k, v in AB["forces"].items()},
        "naive_sum_published_support_force_n": {k: fl(v) for k, v in naive_f.items()},
        "naive_minus_direct_published_support_force_n": {k: fl(v)
                                                          for k, v in diff_f.items()},
        "equals_case": "PA-EMPTY",
    }
    return {"schema": SCHEMA, "basis": BASIS, "models": models, "superposition": sup}


def compute(inp):
    res = {}
    for mid, model in (("P-A", PA), ("P-B", PB)):
        r = evaluate(model, inp)
        r["_fc"] = {h: inp.v(fc) for h, _, _, fc in model["hangers"]}
        res[mid] = r
    return res


def stated_checks(res):
    ht = res["P-A"]["PA-HT"]["forces"]
    assert ht == {"S-1": 2625, "SH-3": 13750, "S-5": 3625}
    for mid in ("P-A", "P-B"):
        for cid, c in res[mid].items():
            if cid.startswith("_"):
                continue
            if c["held"]:
                assert all(v == 0 for v in c["u_hanger"].values())
            else:
                # The published force includes the cold load: it differs from
                # the plain-spring force -k (u - g) by exactly F_cold.
                for h, v in c["forces"].items():
                    if h in res[mid]["_fc"]:
                        assert v > 0
    # The empty case lifts the span: positive displacement at the hanger.
    assert res["P-A"]["PA-EMPTY"]["u_hanger"]["SH-3"] > 0


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--check", action="store_true",
                    help="exit 1 if the frozen JSON differs from a fresh generation")
    args = ap.parse_args()
    res = compute(Inputs(PI[1], False))
    stated_checks(res)
    doc = emit(res)
    for p in (PI[0], PI[2]):
        assert emit(compute(Inputs(p, False))) == doc, "a value differs between pi bounds"
    for p in PI:
        # No discrete outcome here; the stated properties must survive
        # binary64-replaced inputs.
        stated_checks(compute(Inputs(p, True)))
    text = json.dumps(doc, indent=2, ensure_ascii=False) + "\n"
    if args.check:
        if not OUT.exists() or OUT.read_text(encoding="utf-8") != text:
            print("preload.json differs from a fresh generation", file=sys.stderr)
            return 1
        print("preload.json matches")
        return 0
    OUT.write_text(text, encoding="utf-8")
    print("wrote", OUT.name)
    return 0


if __name__ == "__main__":
    sys.exit(main())
