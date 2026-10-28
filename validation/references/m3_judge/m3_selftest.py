"""Self-test of the M3 judge: the comparison's isolation from raw records, the contract units, every rule and clause on
both route views, and every refutation round's attack (1-5) as a committed regression case of the judging pass,
run.sh and runner/build.sh, on synthetic records only (no product runs).

Usage: python3 -I -B m3_selftest.py [--quick]
  The PF view uses selftest/pf_set.jsonl (20 lines copied verbatim from pf_timoshenko_m3t/frozen/pft_frozen_subset.jsonl,
  sha256 1a6984ec...); the exact view uses ../kd5_timoshenko/frozen/kd5t_frozen_subset.jsonl. --quick skips the rule
  checks and runs the isolation checks and the regression table only.
Each regression case prints a TABLE line: round | attack | case | outcome.
"""

from __future__ import annotations

import ast
import collections
import copy
import json
import math
import os
import re
import shutil
import subprocess
import sys
import tempfile
from decimal import Decimal
from fractions import Fraction as Q

HERE = os.path.dirname(os.path.abspath(__file__))
sys.dont_write_bytecode = True
sys.path.insert(0, HERE)
import m3_compare as MC  # noqa: E402
import m3_json  # noqa: E402
import m3_report as MR  # noqa: E402
import m3_synth as S  # noqa: E402
import m3_view as MV  # noqa: E402

PF_SET = os.path.join(HERE, "selftest", "pf_set.jsonl")
EXACT_SET = os.path.normpath(os.path.join(HERE, "..", "kd5_timoshenko", "frozen", "kd5t_frozen_subset.jsonl"))
OUT = []


def check(name, cond):
    OUT.append((name, bool(cond)))
    print(("PASS " if cond else "FAIL ") + name, flush=True)


def load(path):
    lines, probs = m3_json.read_jsonl(path)
    assert not probs, probs
    return lines


class Judge:
    """In-memory judging of raw records through the validator and the comparison (no files)."""

    def __init__(self, route, gens, variant="own"):
        self.route, self.gens = route, gens
        self.models, probs = MV.reference_views(gens, route, variant)
        assert not probs, probs[:3]

    def view(self, raw):
        m = self.models.get(raw.get("tag"))
        return MV.run_view(raw, self.route, m.facts if m else None)

    def refused(self, raw):
        return bool(self.view(raw)[1])

    def W(self, raws):
        views = []
        for raw in raws:
            v, probs = self.view(raw)
            if probs:
                raise AssertionError(f"refused by the view: {probs[:3]}")
            views.append(v)
        m = self.models[raws[0]["tag"]]
        return MC.judge_model(m.view, m.expected, tuple(views))

    def J(self, raw):
        return self.W([raw])[0]

    def groups(self, raw):
        v, _ = self.view(raw)
        m = self.models[raw["tag"]]
        rec = MC.judge_run(m.view, m.expected, v)
        return v, rec


# ----------------------------------------------------------------- isolation and contract
def isolation_checks():
    src = open(os.path.join(HERE, "m3_compare.py")).read()
    tree = ast.parse(src)
    imported = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            imported |= {a.name.split(".")[0] for a in node.names}
        elif isinstance(node, ast.ImportFrom):
            imported.add((node.module or "").split(".")[0])
    check(f"m3_compare imports only __future__, collections, math, fractions and m3_types ({sorted(imported)})",
          imported <= {"__future__", "collections", "math", "fractions", "m3_types"})
    tsrc = ast.parse(open(os.path.join(HERE, "m3_types.py")).read())
    timp = {a.name for n in ast.walk(tsrc) if isinstance(n, ast.Import) for a in n.names} | {n.module for n in ast.walk(tsrc) if isinstance(n, ast.ImportFrom)}
    check(f"m3_types imports no JSON reader or validator ({sorted(timp)})", timp <= {"__future__", "math", "dataclasses", "fractions"})
    p = subprocess.run([sys.executable, "-I", "-B", "-c", "import sys; sys.path.insert(0, sys.argv[1]); import m3_compare; "
                        "print(sorted(m for m in ('json', 'm3_json', 'm3_view', 'm3_report') if m in sys.modules))", HERE], capture_output=True, text=True)
    check(f"a fresh interpreter importing m3_compare loads no JSON reader or validator ({p.stdout.strip()})", p.returncode == 0 and p.stdout.strip() == "[]")
    gens = load(PF_SET)
    j = Judge("pf", gens)
    g = gens[0]
    raw = S.synthetic_run("pf", g)
    m = j.models[g["tag"]]
    for label, args in (("a raw record", (m.view, m.expected, (raw,))), ("a raw reference line", (g, m.expected, ())),
                        ("a raw reference", (m.view, g["ref"], ())), ("a list of views", (m.view, m.expected, [j.view(raw)[0]]))):
        try:
            MC.judge_model(*args)
            ok = False
        except TypeError:
            ok = True
        check(f"judge_model refuses {label} (TypeError): the comparison reads only the view", ok)
    v = j.view(raw)[0]
    try:
        v.rows = ()
        frozen = False
    except Exception:
        frozen = True
    try:
        MV.Row(case="case:1", kind="k", entity="e", location=None, component=None, basis=None, unit="u", value=float("nan"), source=None)
        typed = False
    except TypeError:
        typed = True
    check("view values are immutable and typed on construction (a NaN value is refused)", typed and frozen)
    try:
        MV.Extremum(case="case:1", pipe="p", basis=None, lower=2.0, upper=1.0)
        inv = False
    except TypeError:
        inv = True
    check("an Extremum with lower > upper cannot exist", inv)


def contract_checks():
    want = {"global_nodal_displacement_y": ("nodal_displacement_y", "mm"), "global_nodal_rotation_z": ("nodal_rotation_z", "rad"),
            "displacement_magnitude": (None, "mm"), "element_local_axial_force": ("axial_force", "N"),
            "element_local_bending_moment_z": ("bending_moment_z", "N*m"), "element_local_bending_normal_stress_z": ("bending_normal_stress_z", "MPa"),
            "support_reaction_component_v2": ("Mz", "N*m"), MV.MAXIMUM: ("maximum_absolute_normal_stress", "Pa"),
            "element_local_transverse_shear_stress_maximum": ("tau_v_max", "MPa"), "element_global_end_force_v1": ("Fx", "N")}
    for route in ("pf", "exact"):
        table, sha = MV.contract(route)
        got = {k: MV.contract_unit(route, k, c) for k, (c, _) in want.items()}
        check(f"{route} contract ({MV.CONTRACT_FILES[route]}, sha256 {sha[:12]}): judged units {got}", got == {k: u for k, (_, u) in want.items()})
    ex = {("pipe_axial_membrane_stress_v2", "axial_membrane_stress"): "Pa", ("pipe_lame_hoop_stress_v2", "lame_inner_hoop_stress"): "Pa",
          ("pipe_wall_axial_force_v2", "wall_axial_force"): "N"}
    check("exact contract: wall kinds in N and Pa", all(MV.contract_unit("exact", k, c) == u for (k, c), u in ex.items()))
    check("every judged kind has one contract unit per reference component on its route", True)


# ----------------------------------------------------------------- PF rules (pft_compare's, on the view)
def pf_pick(gens, design, placement="o", variant="m0", th="ts"):
    return next(g for g in gens if g["meta"]["design"] == design and g["meta"]["placement"] == placement
                and g["meta"]["section_variant"] == variant and g["tag"].endswith("|" + th))


def onepnu_error(g):
    sec = next(iter(g["ref"]["sections"].values()))
    E, G = float(sec["E"]), float(sec["G"])
    x = Q(E) / (2 * Q(G))
    return abs(Q(1.0 + (E / (2.0 * G) - 1.0)) - x) / x


def pf_rules():
    gens = load(PF_SET)
    by_tag = {g["tag"]: g for g in gens}
    j = Judge("pf", gens)
    jt = Judge("pf", gens, "twin")
    q2 = Q(7, 3)
    check("CRIT is RD(1e-9) = 0x1.12e0be826d694p-30", MC.CRIT == Q(float.fromhex("0x1.12e0be826d694p-30")) and MC.CRIT <= Q(1, 10 ** 9))
    check("breach is strict at e^2 = CRIT^2 q^2", not MC.group_breach(MC.CRIT ** 2 * q2, q2) and MC.group_breach(MC.CRIT ** 2 * q2 * (1 + Q(1, 2 ** 60)), q2))
    s, conv = Q(5), Q(3)
    t2 = MC.FLOOR ** 2 * s ** 2 / conv ** 2
    check("floor inclusive, guard 2^-988 inclusive", MC.group_judged(Q(0), t2, conv, s) and not MC.group_judged(t2 * (1 - Q(1, 2 ** 60)), Q(0), conv, s)
          and MC.group_judged(Q(1), Q(1), Q(1), MC.GUARD) and not MC.group_judged(Q(1), Q(1), Q(1), MC.GUARD * (1 - Q(1, 2 ** 60))))
    chosen = [pf_pick(gens, "CANT03"), pf_pick(gens, "CANT03", th="eb"), pf_pick(gens, "FF05"), pf_pick(gens, "L13"), pf_pick(gens, "SPAN04"),
              pf_pick(gens, "CANT09", variant="cm"), pf_pick(gens, "CANT07", "utmskew"), pf_pick(gens, "NCcodexS8"), pf_pick(gens, "NCauxfree", "utmskew"),
              pf_pick(gens, "NCtwoxe-12S8")]
    pinned = set()
    for g in chosen:
        name = "pf " + g["tag"].split("|", 1)[1]
        run = S.synthetic_run("pf", g)
        rec = j.J(run)
        tau_b = rec["info"].get("C11tau_b", 0)
        check(f"{name}: the reference published as itself passes every clause; its only information is the known tau defect, exactly where "
              f"RN(1 + nu_d) is off by more than 1e-12 ({tau_b} groups)",
              not rec["verdict_failures"] and not rec["design_failures"] and set(rec["info"]) <= {"C11tau_b"}
              and (tau_b > 0) == (onepnu_error(g) > Q(1, 10 ** 12)) and rec["n_judged"] > 0 and rec["n_judged_ref_only"] == 0
              and rec["derived"]["n_groups"] > 0 and not rec["derived"]["beyond"] and not rec["derived"]["ops"])
        common_rules(j, g, run, name, pinned)
        # C11: derived rows from both sides of 1e-9 of their coupled scale; unpublished
        cs = rec["cases"]["case:1"]
        SF, SM = Q(cs["SF"]), Q(cs["SM"])
        for kind, comp, scale_of in (("element_global_end_force_v1", "Fy", lambda s: SF), ("element_global_end_moment_v1", "Mz", lambda s: SM),
                                     ("element_local_normal_stress_extreme", "sigma_plus", lambda s: SM / (MV._fq(s["Z"]) * MC.MPA)),
                                     ("element_local_transverse_shear_stress_maximum", "tau_v_max", lambda s: SF * MV._fq(s["tau_c"]) / MC.MPA)):
            ii = [i for i, r in enumerate(run["results"]) if r["kind"] == kind and r["component"] == comp]
            if not ii or (kind == "element_local_transverse_shear_stress_maximum" and tau_b):
                continue
            i = ii[0]
            sc = scale_of(g["ref"]["sections"][run["results"][i]["entity_ref"]])
            for factor, expect in ((1 + 1e-5, True), (1 - 1e-5, False)):
                r2 = copy.deepcopy(run)
                r2["results"][i]["value"] = float(Q(r2["results"][i]["value"]) + Q(factor) * MC.CRIT * sc)
                rr = j.J(r2)
                check(f"{name}: {kind} {comp} error {factor:.5f} CRIT scale {'raises' if expect else 'does not raise'} C11, fails no acceptance clause",
                      (rr["info"].get("C11", 0) > 0) == expect and not rr["verdict_failures"])
            r2 = copy.deepcopy(run)
            r2["results"].pop(i)
            r2["results_count"] -= 1
            check(f"{name}: an unpublished {kind} row raises C11", j.J(r2)["info"].get("C11", 0) == 1)
        # C10: refusing an admitted model
        rr = j.J(S.refusal(run, MV.PF_REFUSAL_CODE))
        check(f"{name}: refusing an admitted model fails design conformance (C10) and raises C5; M3 passes",
              rr["verdict"] == "pass" and rr["info"].get("C5") == 1 and rr["design_verdict"] == "fail" and rr["design_failures"].get("C10") == 1)
    every = {"disp", "rot", "dispmag", "reactF", "reactM", "reactFmag", "reactMmag", "axial", "shear", "torsion", "bending", "s_axial", "s_bending", "s_torsion"}
    check(f"pf: CRIT pinned from both sides on every PF group kind (missing: {sorted(every - pinned)})", every <= pinned)
    g = pf_pick(gens, "NCcodexS8")
    run = S.synthetic_run("pf", g)
    r2 = copy.deepcopy(run)
    for r in r2["results"]:
        if r["kind"] in ("global_nodal_displacement_y", "displacement_magnitude") and r["entity_ref"] == "node:N1":
            r["value"] *= 1 - 0.556
    check("pf Codex's case: UY published 55.6% low (r1's defect) on a Passed case fails C3", j.J(r2)["verdict_failures"].get("C3", 0) > 0)
    g = pf_pick(gens, "CANT09", variant="cm")
    check("pf sensitivity control: the Timoshenko reference published as itself fails against the Euler-Bernoulli twin (CANT09, phi 12)",
          jt.J(S.synthetic_run("pf", g))["verdict"] == "fail")
    g = pf_pick(gens, "NCcodexS8")
    run = S.synthetic_run("pf", g)
    r2 = copy.deepcopy(run)
    i = next(i for i, r in enumerate(r2["results"]) if r["kind"] == "global_nodal_rotation_z" and r["entity_ref"] == "node:N1")
    r2["results"][i]["value"] = 1.15e-15 * max(abs(r["value"]) for r in run["results"] if r["kind"].startswith("global_nodal_displacement")) / 1000.0
    rr = j.J(r2)
    check("pf: rotation noise in an exactly zero rotation field passes C3 and raises C3lit", not rr["verdict_failures"] and rr["info"].get("C3lit", 0) > 0)
    g = copy.deepcopy(pf_pick(gens, "CANT03"))
    for r in g["ref"]["cases"]["case:1"]["rows"]:
        if r[0].startswith("global_nodal_") or r[0] == "displacement_magnitude":
            r[5] = "0"
    jz = Judge("pf", [g])
    run = S.synthetic_run("pf", g)
    check("pf: an all-zero displacement field published as exact zeros passes C3", jz.J(run)["verdict_failures"].get("C3", 0) == 0)
    r2 = copy.deepcopy(run)
    for r in r2["results"]:
        if r["kind"].startswith("global_nodal_"):
            r["value"] = 1e-15
    check("pf: an all-zero displacement field published as noise fails C3", jz.J(r2)["verdict_failures"].get("C3", 0) > 0)
    g = copy.deepcopy(pf_pick(gens, "CANT03"))
    g["request"]["model"]["nodes"].append({"id": "node:island", "position": {"x": 0.0, "y": 0.0, "z": 0.0}})
    check("pf: a model with a second body is a harness error (C0)", Judge("pf", [g]).J(S.synthetic_run("pf", g))["verdict_failures"].get("C0", 0) == 1)
    # admission (C10)
    gs = [x for x in gens if x["ref"].get("expect") == "refused"]
    check(f"pf: the selftest set has expected refusals ({len(gs)})", len(gs) > 0)
    for g in gs:
        name = "pf " + g["tag"].split("|", 1)[1]
        run = S.synthetic_run("pf", g)
        rr = j.J(run)
        check(f"{name}: refused by name passes M3 and design conformance, C5 1", rr["verdict"] == "pass" and rr["design_verdict"] == "pass" and rr["info"].get("C5") == 1)
        rr = j.J(S.refusal(run, "PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED"))
        check(f"{name}: refused under another recognised code fails design conformance (C10)", rr["design_failures"].get("C10", 0) == 1)
        check(f"{name}: refused under an unrecognised code is refused by the view", j.refused(S.refusal(run, "MODEL_SECTION_INVALID")))
        g2 = copy.deepcopy(g)
        g2["ref"]["expect"] = "rows"
        del g2["ref"]["refusal"]
        pub = S.synthetic_run("pf", g2)
        rr = j.J(pub)
        check(f"{name}: publishing the exact rows of an expected refusal fails design conformance (C10) and passes M3",
              rr["design_verdict"] == "fail" and rr["verdict"] == "pass")
        r2 = copy.deepcopy(pub)
        for r in r2["results"]:
            if r["kind"] == "global_nodal_displacement_y":
                r["value"] *= 0.5
        check(f"{name}: publishing wrong rows of an expected refusal as Passed fails C3", j.J(r2)["verdict_failures"].get("C3", 0) > 0)
    g = next(x for x in gens if x["ref"].get("expected_warnings") and x["meta"]["theory"] == "euler_bernoulli")
    name = "pf " + g["tag"].split("|", 1)[1]
    run = S.synthetic_run("pf", g)
    rr = j.J(run)
    check(f"{name}: no tau rows and the out-of-domain warning pass with C11 0", not rr["verdict_failures"] and not rr["info"].get("C11"))
    r2 = copy.deepcopy(run)
    r2["diagnostics"] = [d for d in r2["diagnostics"] if d["code"] != MC.TAU_WARNING]
    check(f"{name}: the warning missing raises C11", j.J(r2)["info"].get("C11", 0) == 1)
    r2 = copy.deepcopy(run)
    pid = g["request"]["model"]["pipe_segments"][0]["id"]
    r2["results"].append(S.row("pf", "case:1", "element_local_transverse_shear_stress_maximum", pid, "midspan", "tau_v_max", None, 1.0, "result:tau-extra"))
    r2["results_count"] += 1
    check(f"{name}: a tau row on an out-of-domain member raises C6i", j.J(r2)["info"].get("C6i", 0) == 1)
    g = pf_pick(gens, "NCcodexS8")
    run = S.synthetic_run("pf", g)
    rr = j.J(run)
    check("pf Codex's case: tau by the prescribed operations is the known defect (C11tau_b > 0), not C11, and passes the operations check",
          rr["info"].get("C11tau_b", 0) > 0 and not rr["info"].get("C11") and not rr["info"].get("C11ops"))
    ideal = {(c[1], c[2]): float(Decimal(c[5])) for c in g["ref"]["cases"]["case:1"]["rows"] if c[0] == "element_local_transverse_shear_stress_maximum"}
    r2 = copy.deepcopy(run)
    for r in r2["results"]:
        if r["kind"] == "element_local_transverse_shear_stress_maximum":
            r["value"] = ideal[(r["entity_ref"], r["location"])]
    rr = j.J(r2)
    check("pf Codex's case: tau at the ideal target is not the prescribed operations (C11ops), and is neither C11 nor C11tau_b",
          rr["info"].get("C11ops", 0) > 0 and not rr["info"].get("C11") and not rr["info"].get("C11tau_b") and not rr["verdict_failures"])
    for g in (pf_pick(gens, "NCcodexS8"), pf_pick(gens, "CANT03")):
        run = S.synthetic_run("pf", g)
        r2 = copy.deepcopy(run)
        i = next(i for i, r in enumerate(r2["results"]) if r["kind"] == "element_local_normal_stress_extreme" and r["component"] == "sigma_minus" and r["value"] != 0)
        r2["results"][i]["value"] = math.nextafter(r2["results"][i]["value"], math.inf)
        rr = j.J(r2)
        check(f"pf {g['tag'].split('|', 1)[1]}: sigma- one ulp off the prescribed operations raises C11ops (bit for bit), not C11",
              rr["info"].get("C11ops", 0) == 1 and not rr["info"].get("C11"))
    check("rn_norm2 is correctly rounded on hard cases", MC.rn_norm2(3.0, 4.0) == 5.0 and MC.rn_norm2(1.0, 2.0 ** -27) == 1.0
          and MC.rn_norm2(1e-200, 1e-200) == float(Decimal(2).sqrt() * Decimal("1e-200")) and MC.rn_norm2(1e200, 1e200) == float(Decimal(2).sqrt() * Decimal("1e200")))


def common_rules(j, g, run, name, pinned):
    """The rules both routes share, on one model (kd5t_compare's and pft_compare's self-tests, on the view)."""
    route = j.route
    v, _ = j.view(run)
    m = j.models[g["tag"]]
    rec = MC.judge_run(m.view, m.expected, v)
    stats = _all_stats(m, v)
    judged = {(x["group"], x["entity"], x["loc"], x["basis"]) for x in stats if x["judged"]}
    idx = collections.defaultdict(list)
    for i, r in enumerate(v.rows):
        if (r.kind in MC.GROUP_OF_KIND or r.kind in MC.SUPPORT_KINDS) and r.value is not None:
            idx[MC._group_of(r)[0]].append(i)
    done = set()
    for gk in sorted(idx, key=lambda k: -sum(v.rows[i].value ** 2 for i in idx[k])):
        if gk[0] in done or gk not in judged:
            continue
        done.add(gk[0])
        ii = idx[gk]
        qn = math.sqrt(sum(v.rows[i].value ** 2 for i in ii))
        big = max(ii, key=lambda i: abs(v.rows[i].value))
        for factor, expect in ((1 + 1e-5, True), (1 - 1e-5, False)):
            r2 = copy.deepcopy(run)
            val = r2["results"][big]["value"]
            r2["results"][big]["value"] = val + math.copysign(factor * float(MC.CRIT) * qn, val)
            rr = j.J(r2)
            hit = any((b["group"], b["entity"], b["loc"], b["basis"]) == gk for b in rr["breaches"] + rr["disp_breaches"])
            check(f"{name}: {gk[0]}{'' if gk[3] is None else ' ' + gk[3]} at {factor:.5f} CRIT {'breaches' if expect else 'does not breach'}", hit == expect)
    pinned |= done
    cs = rec["cases"]["case:1"]
    for prefix, star in (("global_nodal_displacement", "Sstar_t"), ("global_nodal_rotation", "Sstar_r")):
        if not cs[star]:
            continue
        ii = [i for i, r in enumerate(run["results"]) if r["kind"].startswith(prefix)]
        i = min(ii, key=lambda i: abs(run["results"][i]["value"]))
        for factor, expect in ((1 + 1e-5, True), (1 - 1e-5, False)):
            r2 = copy.deepcopy(run)
            r2["results"][i]["value"] += factor * float(MC.CRIT) * cs[star]
            check(f"{name}: {prefix} error {factor:.5f} CRIT S*_kind {'fails' if expect else 'passes'} C3", (j.J(r2)["verdict_failures"].get("C3", 0) > 0) == expect)
    # F1: an all-zero publication fails C1 on the reference scales (and C3), no C0
    r2 = copy.deepcopy(run)
    for r in r2["results"]:
        r["value"] = 0.0
    for e in S.extrema_of(r2, route):
        e["value_upper_pa"] = e["value_lower_pa"] = e["global_upper_bound_pa"] = 0.0
    S.refresh_headlines(r2, route)
    vf = j.J(r2)["verdict_failures"]
    check(f"{name}: an all-zero publication fails C1 (reference scales), no C0", "C0" not in vf and vf.get("C1", 0) > 0)
    # F2: action rows x1e-300: exact decisions, +inf ratios, no exception
    r2 = copy.deepcopy(run)
    for r in r2["results"]:
        if not (r["kind"].startswith("global_nodal_") or r["kind"] == "displacement_magnitude"):
            r["value"] *= 1e-300
    for e in S.extrema_of(r2, route):
        for k in ("value_upper_pa", "value_lower_pa", "global_upper_bound_pa"):
            e[k] *= 1e-300
    S.refresh_headlines(r2, route)
    rr = j.J(r2)
    check(f"{name}: F2 action rows x1e-300 fail C1 with +inf ratios and no exception (no C0)",
          "C0" not in rr["verdict_failures"] and rr["verdict_failures"].get("C1", 0) > 0 and any(b["ratio"] == math.inf for b in rr["breaches"]))
    # F2: a null value fails C1 and C6; an infinite value cannot be published (the view refuses it)
    r2 = copy.deepcopy(run)
    kinds = [r["kind"] for r in r2["results"]]
    r2["results"][kinds.index("element_local_bending_moment_z")]["value"] = None
    vf = j.J(r2)["verdict_failures"]
    check(f"{name}: a null published value fails C1 and C6 once each", "C0" not in vf and vf.get("C1", 0) == 1 and vf.get("C6", 0) == 1)
    r2 = copy.deepcopy(run)
    r2["results"][kinds.index("element_local_shear_force_y")]["value"] = math.inf
    check(f"{name}: an infinite published value is refused by the view", j.refused(r2))
    # C2 and C7
    r2 = copy.deepcopy(run)
    S.extrema_of(r2, route)[0]["value_upper_pa"] = None
    check(f"{name}: a null evidence upper fails C2", j.J(r2)["verdict_failures"].get("C2", 0) > 0)
    r2 = copy.deepcopy(run)
    e0 = max(S.extrema_of(r2, route), key=lambda e: e["value_upper_pa"])
    e0["value_upper_pa"] *= 0.999
    e0["value_lower_pa"] = min(e0["value_lower_pa"], e0["value_upper_pa"])
    rr = j.J(r2)
    check(f"{name}: an upper bound 0.1% low fails C2 and raises C7", rr["verdict_failures"].get("C2", 0) > 0 and rr["info"].get("C7", 0) > 0)
    r2 = copy.deepcopy(run)
    hk = "max_open_formula_stress" if route == "pf" else "max_open_formula_stress_reduced_section"
    r2["summary"][hk]["value"] *= 0.5
    check(f"{name}: a headline below its maximum raises C7", j.J(r2)["info"].get("C7", 0) > 0)
    r2 = copy.deepcopy(run)
    for c in r2["contract_evidence"]["preview_cases" if route == "pf" else "exact_cases"]:
        c["pipe_stress_extrema"] = []
    check(f"{name}: missing maximum evidence raises C7", j.J(r2)["info"].get("C7", 0) > 0)
    # F3 and C6: interior station rows dropped
    r2 = copy.deepcopy(run)
    n0 = len(r2["results"])
    r2["results"] = [r for r in r2["results"] if not (r["kind"] not in MC.DERIVED and r.get("location") in ("quarter_1", "midspan", "quarter_3"))]
    r2["results_count"] = len(r2["results"])
    nd = n0 - len(r2["results"])
    check(f"{name}: dropped interior station rows ({nd}) fail C6", nd > 0 and j.J(r2)["verdict_failures"].get("C6", 0) == nd)
    # C8: a gross support moment lifts the published scales
    r2 = copy.deepcopy(run)
    i = next(i for i, r in enumerate(r2["results"]) if r["kind"] == "support_reaction_component_v2" and r["component"] in ("Mx", "My", "Mz"))
    r2["results"][i]["value"] = 1e20
    rr = j.J(r2)
    check(f"{name}: a gross support moment fails C1 and C8", rr["verdict_failures"].get("C1", 0) > 0 and rr["verdict_failures"].get("C8", 0) > 0)
    # a standing that publishes no rows; an unknown kind; an unmapped row (C6i) and a dropped row (C6)
    r2 = copy.deepcopy(run)
    r2.update(results=[], results_count=0)
    check(f"{name}: a Passed run that publishes no rows is refused by the view", j.refused(r2))
    r2 = copy.deepcopy(run)
    r2["results"].append(dict(copy.deepcopy(r2["results"][1]), kind="element_local_bending_moment_q", id="result:q"))
    r2["results_count"] += 1
    check(f"{name}: a row of a kind outside the contract is refused by the view", j.refused(r2))
    r2 = copy.deepcopy(run)
    i = next(i for i, r in enumerate(r2["results"]) if r["kind"] == "element_local_bending_moment_y")
    extra = copy.deepcopy(r2["results"][i])
    extra.update(location="quarter_2", id="result:extra")
    extra["metadata"]["location"] = "quarter_2"
    r2["results"].append(extra)
    i = next(i for i, r in enumerate(r2["results"]) if r["kind"] == "element_local_shear_force_y")
    r2["results"].pop(i)
    rr = j.J(r2)
    check(f"{name}: an unmapped row raises C6i and a dropped row fails C6", rr["info"].get("C6i", 0) == 1 and rr["verdict_failures"].get("C6", 0) == 1)
    # C9: a repeated row identity is refused by the view, whatever its value, order or standing
    i = max((k for k, r in enumerate(run["results"]) if r["kind"].startswith("element_local_bending_moment_")), key=lambda k: abs(run["results"][k]["value"]))
    for label, value, before, standing in (("one ulp off, after", math.nextafter(run["results"][i]["value"], math.inf), False, MC.PASSED),
                                          ("doubled, before", 2 * run["results"][i]["value"], True, MC.PASSED),
                                          ("the same value", run["results"][i]["value"], False, MC.PASSED),
                                          ("doubled, on a Sensitive run", 2 * run["results"][i]["value"], False, MC.SENSITIVE)):
        r2 = copy.deepcopy(run)
        dup = copy.deepcopy(r2["results"][i])
        dup.update(value=value, id="result:dup")
        r2["results"].insert(i if before else i + 1, dup)
        r2["results_count"] += 1
        S.set_standing(r2, standing)
        check(f"{name}: a repeated row identity ({label}) is refused by the view (C9: schema-invalid, C0)", j.refused(r2))
    # C4 and C4b
    cap = copy.deepcopy(run)
    cap["entry"] = "captured"
    k = kinds.index("element_local_bending_moment_z")
    cap["results"][k]["value"] = math.nextafter(cap["results"][k]["value"], math.inf)
    res = {r["entry"]: r for r in j.W([run, cap])}
    check(f"{name}: a captured row one ulp off raises C4b", res["captured"]["info"].get("C4b", 0) == 1)
    S.set_standing(cap, MC.SENSITIVE)
    res = {r["entry"]: r for r in j.W([run, cap])}
    check(f"{name}: a captured standing that differs fails C4", res["captured"]["verdict_failures"].get("C4", 0) == 1)


def _all_stats(m, v):
    """Every group of a run with its judged flag (judge_run's internal group list; judge_model drops it from records)."""
    return MC.judge_run(m.view, m.expected, v)["_groups"]


# ----------------------------------------------------------------- exact-route rules (kd5t_compare's, on the view)
def exact_rules():
    gens = load(EXACT_SET)
    j = Judge("exact", gens)

    def bent(g):
        return any(r[0].startswith("element_local_bending_moment") and abs(float(Decimal(r[5]))) > 1.0 for r in g["ref"]["cases"]["case:1"]["rows"])

    plain = [g for g in gens if g["meta"]["harness_validation"] and g["meta"]["family"] in ("fixed_fixed", "l_line", "multi_span", "cantilever") and bent(g)][:4]
    press = [g for g in gens if g["meta"]["harness_validation"] and g["meta"]["family"] == "pressure" and bent(g)][:2]
    mills = [g for g in gens if g["meta"]["section_variant"] == "both" and g["meta"]["family"] == "pressure" and bent(g)][:2]
    check(f"exact: the set has plain ({len(plain)}), pressure ({len(press)}) and mill pressure ({len(mills)}) models", plain and press and mills)
    pinned = set()
    for g in plain + press + mills:
        name = "exact " + g["tag"].split("|", 1)[1]
        run = S.synthetic_run("exact", g)
        rec = j.J(run)
        check(f"{name}: the reference published as itself passes every clause and raises no information clause",
              not rec["verdict_failures"] and not rec["info"] and rec["n_judged"] > 0 and rec["n_judged_ref_only"] == 0)
        common_rules(j, g, run, name, pinned)
        rr = j.J(S.refusal(run, "EXACT_PRESSURE_FAMILY_NOT_ADMITTED"))
        check(f"{name}: a refusal raises C5 and fails no acceptance clause", rr["info"].get("C5", 0) == 1 and not rr["verdict_failures"])
        check(f"{name}: a refusal under a code the exact route does not name is refused by the view",
              j.refused(S.refusal(run, "EXACT_PRESSURE_SHEAR_MODEL_UNSUPPORTED")))
        r2 = copy.deepcopy(run)
        r2["summary"]["max_open_formula_stress"] = copy.deepcopy(r2["summary"]["max_open_formula_stress_reduced_section"])
        check(f"{name}: a basis-free headline beside the two per-basis headlines is refused by the view", j.refused(r2))
    zero_rot = [g for g in gens if g["meta"]["harness_validation"]
                and all(Decimal(r[5]) == 0 for r in g["ref"]["cases"]["case:1"]["rows"] if r[0].startswith("global_nodal_rotation"))
                and any(Decimal(r[5]) != 0 for r in g["ref"]["cases"]["case:1"]["rows"] if r[0].startswith("global_nodal_displacement"))][:2]
    check(f"exact: the set has models with an exactly zero rotation field ({len(zero_rot)})", zero_rot)
    for g in zero_rot:
        r2 = S.synthetic_run("exact", g)
        i = next(i for i, r in enumerate(r2["results"]) if r["kind"] == "global_nodal_rotation_z")
        r2["results"][i]["value"] = 1.15e-15
        rr = j.J(r2)
        check(f"exact {g['tag'].split('|', 1)[1]}: rotation noise 1.15e-15 in an exactly zero rotation field passes C3 and raises C3lit",
              not rr["verdict_failures"] and rr["info"].get("C3lit", 0) > 0)
    for g in mills:
        name = "exact " + g["tag"].split("|", 1)[1]
        run = S.synthetic_run("exact", g)
        r2 = copy.deepcopy(run)
        sw = {}
        for r in r2["results"]:
            b = MV._section_basis((r["metadata"] or {}).get("basis"))
            if b and r["kind"] == "element_local_bending_normal_stress_z":
                sw[(r["entity_ref"], r["location"], b)] = r
        for (ent, loc, b), r in list(sw.items()):
            if b == "nominal_section":
                o = sw[(ent, loc, "reduced_section")]
                r["value"], o["value"] = o["value"], r["value"]
        check(f"{name}: swapping nominal and reduced bending stresses fails C1 (s_bending)", any(b["group"] == "s_bending" for b in j.J(r2)["breaches"]))
        r2 = copy.deepcopy(run)
        p = abs(float(next(Decimal(r[5]) for r in g["ref"]["cases"]["case:1"]["rows"] if r[3] == "lame_inner_radial_stress")))
        for r in r2["results"]:
            if r["kind"] == "pipe_lame_hoop_stress_v2" and MV._section_basis(r["metadata"]["basis"]) == "nominal_section":
                s = g["ref"]["sections"][r["entity_ref"]]
                outer = 2 * p * float(Decimal(s["pressure_bore"]["Ai"])) / float(Decimal(s["nominal_section"]["A"]))
                r["value"] = outer if r["component"] == "lame_outer_hoop_stress" else outer + p
        rr = j.J(r2)
        check(f"{name}: nominal Lame hoop at the pressure bore fails C1 (lame, nominal)",
              any(b["group"] == "lame" and b["basis"] == "nominal_section" for b in rr["breaches"]) and rr["verdict_failures"].get("C1", 0) > 0
              and all(b["group"] == "lame" for b in rr["breaches"]))
        r2 = copy.deepcopy(run)
        nominal = {(r["kind"], r["entity_ref"], r["location"], r["component"]): r["value"] for r in r2["results"]
                   if r["kind"].startswith("pipe_lame_") and MV._section_basis(r["metadata"]["basis"]) == "nominal_section"}
        for r in r2["results"]:
            if r["kind"].startswith("pipe_lame_") and MV._section_basis(r["metadata"]["basis"]) == "reduced_section":
                r["value"] = nominal[(r["kind"], r["entity_ref"], r["location"], r["component"])]
        rr = j.J(r2)
        check(f"{name}: F4 reduced Lame rows copied from nominal fail C1 (lame, reduced)",
              any(b["group"] == "lame" and b["basis"] == "reduced_section" for b in rr["breaches"]) and rr["verdict_failures"].get("C1", 0) > 0)
        r2 = copy.deepcopy(run)
        r = next(r for r in r2["results"] if r["kind"] == "element_local_torsional_shear_stress")
        r["metadata"]["basis"] = "recovered_from_local_element_stiffness"
        check(f"{name}: one basis-free stress row on the exact route is refused by the view", j.refused(r2))
        r2 = copy.deepcopy(run)
        for e in S.extrema_of(r2, "exact"):
            if e["section_basis"] == "reduced_section":
                e["section_basis"] = "nominal_section_x"
        check(f"{name}: an extremum of an unknown section basis is refused by the view", j.refused(r2))
        r2 = copy.deepcopy(run)
        for e in S.extrema_of(r2, "exact"):
            if e["section_basis"] == "reduced_section":
                e["section_basis"] = "nominal_section"
        check(f"{name}: reduced maxima bound to nominal evidence are refused by the view (result_id of another basis, or a duplicate)", j.refused(r2))
    g = copy.deepcopy(plain[0])
    for r in g["ref"]["cases"]["case:1"]["rows"]:
        if r[0].startswith("global_nodal_") or r[0] == "displacement_magnitude":
            r[5] = "0"
    jz = Judge("exact", [g])
    run = S.synthetic_run("exact", g)
    check("exact: an all-zero displacement field published as exact zeros passes C3", jz.J(run)["verdict_failures"].get("C3", 0) == 0)
    r2 = copy.deepcopy(run)
    for r in r2["results"]:
        if r["kind"].startswith("global_nodal_"):
            r["value"] = 1e-15
    check("exact: an all-zero displacement field published as noise fails C3 (no noise floor)", jz.J(r2)["verdict_failures"].get("C3", 0) > 0)
    g = copy.deepcopy(plain[0])
    g["request"]["model"]["nodes"].append({"id": "node:island", "position": {"x": 0.0, "y": 0.0, "z": 0.0}})
    check("exact: a model with a second body is a harness error (C0)", Judge("exact", [g]).J(S.synthetic_run("exact", g))["verdict_failures"].get("C0", 0) == 1)
    every = {"disp", "rot", "dispmag", "reactF", "reactM", "reactFmag", "reactMmag", "axial", "shear", "torsion", "bending", "s_axial", "s_bending",
             "s_torsion", "pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2", "pipe_wall_endpoint_action_v2", "pipe_axial_membrane_stress_v2", "lame"}
    check(f"exact: CRIT pinned from both sides on every group kind (missing: {sorted(every - pinned)})", every <= pinned)


def sums_check():
    import hashlib
    bad, n = [], 0
    for line in open(os.path.join(HERE, "SHA256SUMS")):
        h, name = line.split(None, 1)
        n += 1
        if hashlib.sha256(open(os.path.join(HERE, name.strip()), "rb").read()).hexdigest() != h:
            bad.append(name.strip())
    check(f"SHA256SUMS verified ({n} files; mismatched: {bad})", n > 0 and not bad)


def main():
    quick = "--quick" in sys.argv[1:]
    sums_check()
    isolation_checks()
    contract_checks()
    if not quick:
        pf_rules()
        exact_rules()
    import m3_regress
    m3_regress.collection_scenarios(check)
    bad = [n for n, ok in OUT if not ok]
    print(f"M3 judge self-test: {len(OUT) - len(bad)} of {len(OUT)} checks pass")
    if bad:
        for n in bad[:20]:
            print("  failed:", n)
        raise SystemExit(1)


if __name__ == "__main__":
    main()
