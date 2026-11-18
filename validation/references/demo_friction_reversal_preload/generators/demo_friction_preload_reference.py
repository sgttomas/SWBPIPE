#!/usr/bin/env python3
"""Demo three-way friction reversal with stop release, under the variable-spring-hanger preload law.

Supplements the independent expectations held inline in the EXPECTED table of
core/product_physics/tests/friction_reversal_stop_release_runtime.rs (lines 61-66; frozen, unchanged),
which were derived with the demo's hanger support:SH-140 as a bare 42000 N/m UZ spring although its record
carries an entered cold load of 430 N. No generator for them is committed. This generator rebuilds that
reference by the method its header states and derives the same scenarios under DEL-04-03/REQ-014 and
DEL-04-11/REQ-014:

  - SH-140 is a spring of its entered rate k on UZ plus its entered cold load F_cold as a constant force
    on the pipe along +Z at N-140, in every load case and every variant (a variant negates the case's
    primitive loads only; the cold load is the support's, not a primitive load);
  - its published support-on-pipe Fz is F_cold - k*u_z (no case moves its base);
  - the cold load is also published separately as SH-140's applied force (recorded per row, outside the
    EXPECTED fields);
  - installed_load and hot_load are review data and are not applied.

The reference model (the test header's stated basis, read from the demo model's own leaves):
  - exact-rational small-displacement 3D Euler-Bernoulli frame of the four pipes; annular A = pi(ro^2 - ri^2),
    I = pi(ro^4 - ri^4)/4, J = 2I; E and G from the material record; pi a 60-decimal rational (Machin);
  - consistent line loads; a thermal interval as the initial strain alpha*dT (axial pair +-E A alpha dT);
  - anchor S-100, guide S-120 (UX, UZ) and S-130 (UY) rigid; SH-140 a UZ spring at zero reference (plus,
    under the law, its cold load); components (C-110, C-130, C-140) and CE-120 not consumed;
  - NL-140 one-way on UY (contact for a negative reaction); NL-130-FRIC zero-reference Coulomb friction on UZ,
    mu from the record, on N = |S-130 UY action| of the same state.
Every state NL-140 {active, released} x friction {stick, slide +UZ, slide -UZ} x sign of the S-130 action
is solved exactly; a state is admissible when every condition holds (stop active iff its action is
negative, released iff N-140 UY <= 0; stick iff |F| <= mu N; slide iff F = -sign(UZ) mu N with UZ in the
slide's sense; the S-130 action of the stated sign). Strict: every margin positive. Exactly one strict
state is required in each scenario; a weak state, none or more than one is reported, not chosen.

Standard-library Python 3 only. It imports, reads and runs no product code and no product output. Its
default check reads only this package: staged/invented_demo_model.json (the demo model fixture as committed
at a named commit) and staged/frozen_expected_table.json (the test file's EXPECTED lines verbatim, with the
author's transcription, checked against those lines number for number). Only --check-live opens the live
fixture and the test file.

Modes (run from this package's directory):
  python3 -I generators/demo_friction_preload_reference.py --check   regenerate into a temporary directory and
                                                                     compare byte for byte (changes no file)
  python3 -I generators/demo_friction_preload_reference.py --write   write the outputs (intentional changes only)
  python3 -I generators/demo_friction_preload_reference.py --check-live
                                       compare the staged inputs with the live fixture's derivation leaves and the
                                       live EXPECTED table, found by its name, naming any that differ (not run by --check)
"""
import argparse
import fnmatch
import hashlib
import json
import re
import sys
import tempfile
from fractions import Fraction as Fr
from itertools import product
from pathlib import Path

HERE = Path(__file__).resolve().parent
OUT = HERE.parent
ROOT = OUT.parents[2]
STAGED_MODEL = "staged/invented_demo_model.json"
STAGED_TABLE = "staged/frozen_expected_table.json"
DEMO = OUT / STAGED_MODEL
LIVE_DEMO = ROOT / "fixtures" / "product_preview" / "invented_demo_model.json"  # --check-live only
LIVE_TEST = ROOT / "core" / "product_physics" / "tests" / "friction_reversal_stop_release_runtime.rs"  # --check-live only
OUTPUT = "demo_friction_reversal_preload.json"
FIXED_FILES = ("README.md", "generators/demo_friction_preload_reference.py", STAGED_MODEL, STAGED_TABLE)
DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
VARIANTS = ["Original", "ReverseZ", "ReverseAll"]

# The frozen EXPECTED table (the test file's lines 61-66): the stiffness-only result, SH-140 a bare 42000 N/m
# spring with its cold load not applied. The staged file holds those lines verbatim and the author's
# transcription; used only for the reproduction check and to stand beside each changed value; build() checks
# that every number of the transcription appears verbatim on its staged line.
TABLE = json.loads((OUT / STAGED_TABLE).read_text())
TRANSCRIBED_LINES = {row["line_at_staging_commit"]: (row["case"], row["variant"]) for row in TABLE["transcription"]}
FROZEN = [{k: v for k, v in row.items() if k != "line_at_staging_commit"} for row in TABLE["transcription"]]
STAGED_ROWS = {row["line_at_staging_commit"]: row["text"] for row in TABLE["rows"]}
FIELDS = ["case", "variant", "applied_y_n", "stop_active", "stop_uy_mm", "stop_reaction_n", "slip_mm", "friction_n",
          "normal_n", "s130_fy_n", "sh140_fz_n"]
CROSS = []


def check(name, ok):
    CROSS.append({"check": name, "agrees": bool(ok)})


def q(x):
    """An exact rational from a JSON leaf (its shortest decimal text, as authored)."""
    return Fr(repr(x)) if isinstance(x, float) else Fr(x)


def machin_pi(digits):
    """pi to `digits` decimals as an exact rational: 16 atan(1/5) - 4 atan(1/239) in scaled integers."""
    scale = 10 ** (digits + 10)

    def atan_inv(n):
        total, term, k, sign = 0, scale // n, 1, 1
        while term:
            total += sign * (term // k)
            term //= n * n
            k += 2
            sign = -sign
        return total
    return Fr(16 * atan_inv(5) - 4 * atan_inv(239), scale).limit_denominator(10 ** (digits + 5))


PI = machin_pi(60)


# ----------------------------------------------------------------------------- the frame, from the leaves
def solve_linear(a, b):
    n = len(b)
    m = [list(r) + [v] for r, v in zip(a, b)]
    for c in range(n):
        piv = next((r for r in range(c, n) if m[r][c] != 0), None)
        if piv is None:
            return None
        m[c], m[piv] = m[piv], m[c]
        p = m[c][c]
        m[c] = [x / p for x in m[c]]
        for r in range(n):
            if r != c and m[r][c] != 0:
                f = m[r][c]
                m[r] = [x - f * y for x, y in zip(m[r], m[c])]
    return [r[-1] for r in m]


def axes(xi, xj, yref):
    d = [b - a for a, b in zip(xi, xj)]
    nz = [k for k in range(3) if d[k] != 0]
    assert len(nz) == 1, "the demo's members are axis-aligned"
    L = abs(d[nz[0]])
    ex = [v / L for v in d]
    p = sum(a * b for a, b in zip(yref, ex))
    yc = [a - p * b for a, b in zip(yref, ex)]
    ny = [k for k in range(3) if yc[k] != 0]
    assert len(ny) == 1
    ey = [v / abs(yc[ny[0]]) for v in yc]
    ez = [ex[1] * ey[2] - ex[2] * ey[1], ex[2] * ey[0] - ex[0] * ey[2], ex[0] * ey[1] - ex[1] * ey[0]]
    return L, [ex, ey, ez]


def local_k(E, G, A, I, J, L):
    k = [[Fr(0)] * 12 for _ in range(12)]

    def sym(i, j, v):
        k[i][j] = k[j][i] = v
    a, t = E * A / L, G * J / L
    sym(0, 0, a); sym(6, 6, a); sym(0, 6, -a)
    sym(3, 3, t); sym(9, 9, t); sym(3, 9, -t)
    z12, z6, z4, z2 = 12 * E * I / L ** 3, 6 * E * I / L ** 2, 4 * E * I / L, 2 * E * I / L
    sym(1, 1, z12); sym(1, 5, z6); sym(1, 7, -z12); sym(1, 11, z6)
    sym(5, 5, z4); sym(5, 7, -z6); sym(5, 11, z2); sym(7, 7, z12); sym(7, 11, -z6); sym(11, 11, z4)
    sym(2, 2, z12); sym(2, 4, -z6); sym(2, 8, -z12); sym(2, 10, -z6)
    sym(4, 4, z4); sym(4, 8, z6); sym(4, 10, z2); sym(8, 8, z12); sym(8, 10, z6); sym(10, 10, z4)
    return k


def block_t(R):
    T = [[Fr(0)] * 12 for _ in range(12)]
    for b in range(4):
        for i in range(3):
            for j in range(3):
                T[3 * b + i][3 * b + j] = R[i][j]
    return T


def mm(a, b):
    bt = list(zip(*b))
    return [[sum(x * y for x, y in zip(r, c)) for c in bt] for r in a]


def mv(a, v):
    return [sum(x * y for x, y in zip(r, v)) for r in a]


class Demo:
    """The reference model of the demo, formed from the fixture's leaves alone."""

    def __init__(self, doc):
        self.doc = doc
        self.nodes = [n["id"] for n in doc["nodes"]]
        self.pos = {n["id"]: [q(n["position"][c]) for c in "xyz"] for n in doc["nodes"]}
        self.n = 6 * len(self.nodes)
        mats = {m["id"]: m for m in doc["materials"]}
        self.K = [[Fr(0)] * self.n for _ in range(self.n)]
        self.geo = {}
        for p in doc["pipe_segments"]:
            m = mats[p["material"]]
            E, G, alpha = q(m["elastic_modulus"]["value"]), q(m["shear_modulus"]["value"]), q(m["thermal_expansion_coefficient"]["value"])
            od, t = q(p["section"]["outside_diameter"]["value"]), q(p["section"]["wall_thickness"]["value"])
            ro, ri = od / 2, od / 2 - t
            A, I = PI * (ro ** 2 - ri ** 2), PI * (ro ** 4 - ri ** 4) / 4
            L, R = axes(self.pos[p["from"]], self.pos[p["to"]], [q(p["y_reference"][c]) for c in "xyz"])
            T = block_t(R)
            kg = mm(mm(list(map(list, zip(*T))), local_k(E, G, A, I, 2 * I, L)), T)
            idx = self.dof(p["from"]) + self.dof(p["to"])
            for a, ia in enumerate(idx):
                for b, ib in enumerate(idx):
                    self.K[ia][ib] += kg[a][b]
            self.geo[p["id"]] = {"L": L, "R": R, "E": E, "A": A, "I": I, "alpha": alpha, "idx": idx}
        sup = {s["id"]: s for s in doc["supports"]}
        self.rigid = []
        for sid in ("support:S-100", "support:S-120", "support:S-130"):
            s = sup[sid]
            assert s["family"] in ("anchor", "guide") and not s.get("nonlinear")
            self.rigid += [self.dof(s["node"])[DOFS.index(r)] for r in s["restraints"]]
        self.s130 = self.dof(sup["support:S-130"]["node"])[DOFS.index(sup["support:S-130"]["restraints"][0])]
        stop = sup["support:NL-140"]["nonlinear"]
        assert stop["behavior"] == "one_way" and stop["active_when"] == "negative_reaction"
        self.stop = self.dof(sup["support:NL-140"]["node"])[DOFS.index(stop["dof"])]
        fr = sup["support:NL-130-FRIC"]["nonlinear"]
        assert fr["behavior"] == "friction" and fr["normal_reaction_source"] == {"support_ref": "support:S-130", "dof": "UY"}
        self.fric = self.dof(sup["support:NL-130-FRIC"]["node"])[DOFS.index(fr["dof"])]
        self.mu = q(fr["friction_coefficient"]["value"])
        h = sup["support:SH-140"]["hanger"]
        assert sup["support:SH-140"]["family"] == "variable_spring_hanger" and h["stiffness"]["value"]["unit"] == "N/m"
        self.spring = self.dof(sup["support:SH-140"]["node"])[DOFS.index(h["stiffness"]["dof"])]
        assert h["stiffness"]["dof"] in ("UX", "UY", "UZ")
        self.k = q(h["stiffness"]["value"]["value"])
        assert h["cold_load"]["unit"] == "N"
        self.cold = q(h["cold_load"]["value"])
        self.K[self.spring][self.spring] += self.k
        self.anchor = self.dof(sup["support:S-100"]["node"])

    def dof(self, node):
        b = 6 * self.nodes.index(node)
        return list(range(b, b + 6))

    def loads(self, case_id, variant, cold):
        """f for the case under the variant, plus the cold load (scaled) at SH-140 along +DOF."""
        (case,) = [c for c in self.doc["load_cases"] if c["id"] == case_id]
        f = [Fr(0)] * self.n
        applied_y = Fr(0)
        for ld in case["primitive_loads"]:
            v = q(ld["magnitude"]["value"])
            if variant == "ReverseAll" or (variant == "ReverseZ" and ld["direction"] == "global_z" and ld["dimension"] == "force_per_length"):
                v = -v
            if ld["dimension"] == "force_per_length":
                g = self.geo[ld["target"]["pipe"]]
                w = [Fr(0)] * 3
                w[{"global_x": 0, "global_y": 1, "global_z": 2}[ld["direction"]]] = v
                wl = mv(g["R"], w)
                L = g["L"]
                fl = [Fr(0)] * 12
                fl[0] = fl[6] = wl[0] * L / 2
                fl[1] = fl[7] = wl[1] * L / 2
                fl[5], fl[11] = wl[1] * L * L / 12, -wl[1] * L * L / 12
                fl[2] = fl[8] = wl[2] * L / 2
                fl[4], fl[10] = -wl[2] * L * L / 12, wl[2] * L * L / 12
                fg = mv(list(map(list, zip(*block_t(g["R"])))), fl)
                for a, ia in enumerate(g["idx"]):
                    f[ia] += fg[a]
            elif ld["dimension"] == "force":
                axis = {"global_x": 0, "global_y": 1, "global_z": 2}[ld["direction"]]
                f[self.dof(ld["target"]["node"])[axis]] += v
                if axis == 1:
                    applied_y += v
            elif ld["dimension"] == "temperature_interval":
                g = self.geo[ld["target"]["pipe"]]
                nt = g["E"] * g["A"] * g["alpha"] * v
                for c in range(3):
                    f[g["idx"][c]] -= nt * g["R"][0][c]
                    f[g["idx"][6 + c]] += nt * g["R"][0][c]
            else:
                raise ValueError(ld["dimension"])
        f[self.spring] += cold
        return f, applied_y

    def solve_state(self, f, stop_active, fric, beta):
        """One state: stop active (UY = 0) or released; friction stick (UZ = 0) or slide (+1/-1) with N of sign beta."""
        cons = set(self.rigid)
        if stop_active:
            cons.add(self.stop)
        if fric == "stick":
            cons.add(self.fric)
        free = [i for i in range(self.n) if i not in cons]
        A, b = [], []
        for i in free:
            row = [self.K[i][j] for j in free]
            rhs = f[i]
            if fric != "stick" and i == self.fric:
                # F = -dir mu beta (K_s u - f_s): moved to the left-hand side.
                c = -fric * self.mu * beta
                row = [r - c * self.K[self.s130][j] for r, j in zip(row, free)]
                rhs = rhs - c * f[self.s130]
            A.append(row)
            b.append(rhs)
        x = solve_linear(A, b)
        if x is None:
            return None
        u = [Fr(0)] * self.n
        for i, v in zip(free, x):
            u[i] = v
        r = [sum(self.K[i][j] * u[j] for j in range(self.n)) - f[i] for i in range(self.n)]
        return u, r


def enumerate_scenario(model, case_id, variant, cold):
    f, applied_y = model.loads(case_id, variant, cold)
    states = []
    for stop_active, fric, beta in product((True, False), ("stick", 1, -1), (1, -1)):
        solved = model.solve_state(f, stop_active, fric, beta)
        if solved is None:
            states.append({"state": (stop_active, fric, beta), "solvable": False})
            continue
        u, r = solved
        R_s = r[model.s130]
        N = beta * R_s
        F = r[model.fric] if fric == "stick" else -fric * model.mu * N
        if fric != "stick":
            assert r[model.fric] == F  # the coupled row holds the friction force exactly
        margins = {"normal_sign beta*R_S130": N}
        if stop_active:
            margins["stop action -R_NL140"] = -r[model.stop]
        else:
            margins["stop release -UY_N140"] = -u[model.stop]
        if fric == "stick":
            margins["stick capacity mu*N - |F|"] = model.mu * N - abs(F)
        else:
            margins["slide sense dir*UZ_N130"] = fric * u[model.fric]
        worst = min(margins.values())
        standing = "strict" if worst > 0 else ("weak" if worst == 0 else "inadmissible")
        states.append({"state": (stop_active, fric, beta), "solvable": True, "standing": standing,
                       "margins": margins, "u": u, "r": r, "F": F, "N": N, "applied_y": applied_y})
    return states, f, applied_y


def name(state):
    stop_active, fric, beta = state
    return {"NL-140": "active" if stop_active else "released",
            "NL-130-FRIC": "stick" if fric == "stick" else ("slide +UZ" if fric == 1 else "slide -UZ"),
            "S-130 action sign": "+" if beta == 1 else "-"}


def row_of(model, case_id, variant, st, cold):
    u, r = st["u"], st["r"]
    return {"case": case_id, "variant": variant, "applied_y_n": st["applied_y"], "stop_active": st["state"][0],
            "stop_uy_mm": u[model.stop] * 1000, "stop_reaction_n": r[model.stop] if st["state"][0] else Fr(0),
            "slip_mm": u[model.fric] * 1000, "friction_n": st["F"], "normal_n": st["N"], "s130_fy_n": r[model.s130],
            "sh140_fz_n": cold - model.k * u[model.spring]}


def as_float_row(row):
    return {k: (float(v) if isinstance(v, Fr) else v) for k, v in row.items()}


def reproduces(row, frozen):
    """The test's own criteria (its header and close/zero helpers)."""
    out = []
    for key in FIELDS:
        exp, obs = frozen[key], row[key]
        if key in ("case", "variant", "stop_active"):
            out.append(obs == exp)
        elif key == "friction_n":
            out.append(abs(obs - Fr(exp)) <= Fr(1, 10 ** 9) * Fr(frozen["normal_n"]))
        elif exp == 0:
            out.append(abs(obs) <= Fr(1, 10 ** 9) * abs(Fr(frozen["applied_y_n"])))
        else:
            out.append(abs(obs - Fr(exp)) <= Fr(1, 10 ** 9) * abs(Fr(exp)))
    return all(out)


def transcription_matches():
    ok = True
    for (ln, (case_id, variant)), frozen in zip(TRANSCRIBED_LINES.items(), FROZEN):
        text = STAGED_ROWS[ln]
        ok &= (frozen["case"], frozen["variant"]) == (case_id, variant)
        ok &= f'case: "{case_id}"' in text and f"Variant::{variant}," in text
        for key in FIELDS[2:]:
            v = frozen[key]
            lit = ("true" if v else "false") if isinstance(v, bool) else repr(v)
            ok &= f"{key}: {lit}," in text or f"{key}: {lit} }}" in text
    return ok


def fmt_margins(m):
    return {k: {"exact": str(v), "value": float(v)} for k, v in m.items()}


def build():
    CROSS.clear()
    doc = json.loads(DEMO.read_text())
    model = Demo(doc)
    check("the transcribed frozen table matches the staged lines 61-66 of the test file, number for number", transcription_matches())
    check("pi (Machin, 60 decimals) begins 3.14159265358979323846264338327950288419716939937510",
          str(PI.numerator * 10 ** 50 // PI.denominator).startswith("314159265358979323846264338327950288419716939937510"))
    rows, notes, findings = [], [], []
    for scale, tag in ((Fr(0), "reproduction"), (Fr(1), "law")):
        cold = model.cold * scale
        for frozen in FROZEN:
            case_id, variant = frozen["case"], frozen["variant"]
            states, f, applied_y = enumerate_scenario(model, case_id, variant, cold)
            strict = [s for s in states if s.get("standing") == "strict"]
            weak = [s for s in states if s.get("standing") == "weak"]
            label = f"{tag} {case_id} {variant}"
            check(f"{label}: exactly one strictly admissible state and no weakly admissible state",
                  len(strict) == 1 and not weak)
            if len(strict) != 1 or weak:
                findings.append({"scenario": label, "strict": [name(s["state"]) for s in strict],
                                 "weak": [name(s["state"]) for s in weak]})
                continue
            (st,) = strict
            row = row_of(model, case_id, variant, st, cold)
            u, r = st["u"], st["r"]
            check(f"{label}: the anchor carries no Y force (exactly)", r[model.anchor[1]] == 0)
            check(f"{label}: Y balance: the S-130 and stop actions carry the applied Y load (exactly)",
                  r[model.s130] + (r[model.stop] if st["state"][0] else 0) == -applied_y)
            if not st["state"][0]:
                check(f"{label}: a released stop leaves N = |applied Y| (exactly)", st["N"] == abs(applied_y))
            check(f"{label}: Coulomb: friction opposes the slip and |F| = mu N",
                  st["state"][1] == "stick" or (st["F"] * u[model.fric] < 0 and abs(st["F"]) == model.mu * st["N"]))
            if tag == "reproduction":
                check(f"{label}: with F_cold = 0 every field reproduces the frozen table at the test's own criteria",
                      reproduces(row, frozen))
                continue
            check(f"{label}: SH-140's published Fz is F_cold - k u_z", row["sh140_fz_n"] == model.cold - model.k * u[model.spring])
            rows.append(as_float_row(row))
            changed = {}
            for key in FIELDS[2:]:
                law_v, frz = as_float_row(row)[key], frozen[key]
                if law_v != frz:
                    changed[key] = {"law": law_v, "stiffness_only": frz}
            notes.append({
                "case": case_id, "variant": variant,
                "admissible_state": name(st["state"]),
                "margins_of_the_admissible_state": fmt_margins(st["margins"]),
                "every_state": [{"state": name(s["state"]), "standing": s.get("standing", "singular"),
                                 "margins": fmt_margins(s["margins"]) if s["solvable"] else None} for s in states],
                "stiffness_only_state": "NL-140 " + ("active" if frozen["stop_active"] else "released")
                                        + "; NL-130-FRIC slide " + ("+UZ" if frozen["slip_mm"] > 0 else "-UZ"),
                "sh140_applied_force_n": float(model.cold),
                "changed": changed,
                "changed_basis": "stiffness_only: the frozen EXPECTED entry (test file line "
                                 + str([ln for ln, cv in TRANSCRIBED_LINES.items() if cv == (case_id, variant)][0])
                                 + "), SH-140 a bare 42000 N/m spring, its cold load not applied",
                "cold_load_n": float(model.cold),
                "exact": {k: str(v) for k, v in row.items() if isinstance(v, Fr)},
            })
    out = {
        "document": "demo_friction_reversal_preload",
        "status": "candidate references under the variable-spring-hanger preload law; derived by their author TASK "
                  "without sight of any product output; not yet independently refuted",
        "supplements": "core/product_physics/tests/friction_reversal_stop_release_runtime.rs EXPECTED (lines 61-66; frozen, unchanged)",
        "model": {"file": STAGED_MODEL,
                  # Both inputs were staged at the same commit, which the staged table records.
                  "staged_from": "fixtures/product_preview/invented_demo_model.json at " + TABLE["commit"],
                  "sha256": hashlib.sha256(DEMO.read_bytes()).hexdigest(),
                  "basis": "the test header's: C-130 (and every component) and CE-120 not consumed; the test's request "
                           "declares C-130 not_solver_consumed"},
        "law": "DEL-04-03/REQ-014 and DEL-04-11/REQ-014: SH-140 a spring of its entered rate on UZ plus its entered cold "
               "load as a constant force along +Z at N-140 in every case and variant; published Fz = F_cold - k u_z; "
               "the cold load published separately as SH-140's applied force; installed_load and hot_load not applied",
        "units": "N and mm, support actions act on the structure (the test's)",
        "criteria": "the test's own, unchanged: frame quantities |obs - exp| <= 1e-9 |exp|; an exactly-zero expectation "
                    "at 1e-9 of the case's authored Y-force scale; the friction force |obs - exp| <= 1e-9 * normal_n, "
                    "one billionth of the scenario's normal force (not a fixed number of newtons)",
        "margin_values": "each margin's 'exact' is the exact rational and governs; its 'value' is the nearest binary64, "
                         "which can exceed the exact rational by up to half a unit in the last place; read lower bounds "
                         "from 'exact'",
        "rows": rows,
        "notes": notes,
        "findings": findings,
        "crosscheck": list(CROSS),
    }
    return {OUTPUT: (json.dumps(out, indent=1) + "\n").encode()}


# The leaves of the model document the derivation reads (Demo and loads()); descriptive strings, components and
# CE-120 are not among them. Paths key list members by their id.
DERIVATION_LEAVES = [
    "nodes/*/position/*",
    "materials/*/elastic_modulus/*", "materials/*/shear_modulus/*", "materials/*/thermal_expansion_coefficient/*",
    "pipe_segments/*/from", "pipe_segments/*/to", "pipe_segments/*/material", "pipe_segments/*/y_reference/*",
    "pipe_segments/*/section/outside_diameter/*", "pipe_segments/*/section/wall_thickness/*",
    "supports/support:S-1?0/family", "supports/support:S-1?0/node", "supports/support:S-1?0/restraints/*",
    "supports/support:S-1?0/nonlinear*",
    "supports/support:NL-140/node", "supports/support:NL-140/nonlinear/*",
    "supports/support:NL-130-FRIC/node", "supports/support:NL-130-FRIC/nonlinear/*",
    "supports/support:SH-140/family", "supports/support:SH-140/node", "supports/support:SH-140/hanger/stiffness/*",
    "supports/support:SH-140/hanger/cold_load/*",
    "load_cases/*/primitive_loads/*/direction", "load_cases/*/primitive_loads/*/dimension",
    "load_cases/*/primitive_loads/*/magnitude/*", "load_cases/*/primitive_loads/*/target/*",
]


def flatten(o, path=""):
    if isinstance(o, dict):
        return {k: v for key, x in o.items() for k, v in flatten(x, f"{path}/{key}" if path else key).items()}
    if isinstance(o, list):
        keyed = all(isinstance(x, dict) and "id" in x for x in o)
        return {k: v for i, x in enumerate(o) for k, v in flatten(x, f"{path}/{x['id'] if keyed else i}").items()}
    return {path: o}


def derivation_leaves(doc):
    return {k: v for k, v in flatten(doc).items() if any(fnmatch.fnmatchcase(k, p) for p in DERIVATION_LEAVES)}


def live_table_rows(text, name):
    """The rows of the table declared as the constant `name`, in order, found by the name wherever it sits.

    Returns (rows, None), or (None, reason) when the declaration is not found exactly once."""
    decl = re.compile(r"^\s*(pub(\([^)]*\))?\s+)?(const|static)\s+" + re.escape(name) + r"\s*:")
    lines = text.splitlines()
    at = [i for i, line in enumerate(lines) if decl.match(line)]
    if len(at) != 1:
        return None, f"the table {name} is declared {len(at)} times in the file, not once"
    rows = []
    for line in lines[at[0] + 1:]:
        if line.strip().startswith("];"):
            break
        if line.strip().startswith("Expected {"):
            rows.append(line.strip())
    return rows, None


def check_live():
    """The staged inputs against their live sources: the fixture on its derivation leaves only, and the frozen
    table found by its name in the live test file."""
    differ = []
    staged, live = derivation_leaves(json.loads(DEMO.read_text())), derivation_leaves(json.loads(LIVE_DEMO.read_text()))
    for k in sorted(set(staged) | set(live)):
        if staged.get(k, "<absent>") != live.get(k, "<absent>"):
            differ.append(f"{LIVE_DEMO.relative_to(ROOT)} {k}: staged {staged.get(k, '<absent>')!r}, live {live.get(k, '<absent>')!r}")
    test = LIVE_TEST.relative_to(ROOT)
    rows, reason = live_table_rows(LIVE_TEST.read_text(), TABLE["table"])
    if rows is None:
        print(f"SOURCE MOVED ON: {test}: {reason}; the live check cannot find the staged table and does not guess. "
              "A decision is needed on the staged input.")
        return 1
    expected = [r["text"].strip() for r in TABLE["rows"]]
    for i in range(max(len(expected), len(rows))):
        a = expected[i] if i < len(expected) else "<absent>"
        b = rows[i] if i < len(rows) else "<absent>"
        if a != b:
            differ.append(f"{test} table {TABLE['table']} row {i + 1}: staged {a!r}, live {b!r}")
    for d in differ:
        print("DIFFERS:", d)
    print(f"live inputs: {len(staged)} model derivation leaves and the {len(expected)} rows of table {TABLE['table']} "
          f"compared, {len(differ)} differ")
    if differ:
        print("SOURCE MOVED ON: a live source no longer matches the staged input in what is named above; a decision is "
              "needed on whether to re-stage it. The committed output stands for its staged inputs.")
    return 1 if differ else 0


def with_sums(files):
    sums = []
    for path in sorted(list(files) + list(FIXED_FILES)):
        data = files[path] if path in files else (OUT / path).read_bytes()
        sums.append(f"{hashlib.sha256(data).hexdigest()}  {path}")
    files["SHA256SUMS"] = ("\n".join(sums) + "\n").encode()
    return files


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--check-live", action="store_true")
    args = parser.parse_args()
    if args.check_live:
        return check_live()
    files = with_sums(build())
    bad = [c for c in CROSS if not c["agrees"]]
    print(f"cross-checks: {len(CROSS)} recorded, {len(bad)} disagreeing")
    for c in bad:
        print("DISAGREES:", c["check"])
    if bad:
        return 1
    if args.write:
        for path, data in files.items():
            (OUT / path).parent.mkdir(parents=True, exist_ok=True)
            (OUT / path).write_bytes(data)
        print(f"wrote {len(files)} files")
        return 0
    with tempfile.TemporaryDirectory() as tmp:
        for path, data in files.items():
            (Path(tmp) / path).write_bytes(data)
        failures = [p for p in files if not (OUT / p).exists() or (OUT / p).read_bytes() != (Path(tmp) / p).read_bytes()]
    committed = {str(p.relative_to(OUT)) for p in OUT.rglob("*") if p.is_file() and "__pycache__" not in p.parts}
    extra = sorted(committed - set(files) - set(FIXED_FILES))
    if failures or extra:
        print("MISMATCH: " + ", ".join(failures + [f"unexpected {e}" for e in extra]))
        return 1
    print(f"verified {len(files)} files byte for byte (SHA256SUMS included)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
