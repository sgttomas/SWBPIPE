"""PF M3-T comparison: the product's published PF rows against the pft exact reference, and the M3 verdict.

The judging and coverage logic is kd5t_compare.py's reconstructed M3 harness, imported unchanged from
../kd5_timoshenko (read-only); see its docstring for every rule and its source. This module adapts it to
the pressure-free (PF) route and adds the PF-only rows and expectations. No product code is read.

Usage:
  python3 -I -B pft_compare.py <set.jsonl> <runs.jsonl> <out.jsonl> [--ref own|twin] [--workers N]
  python3 -I -B pft_compare.py --selftest <set.jsonl>
  python3 -I -B pft_compare.py --probe-format <probes.in.jsonl> <probes.out.jsonl>

What PF publishes, established from the public result format on Euler-Bernoulli probe models
(`--probe-format`; README "Runner and probes"):
- one section family: stress rows carry no `section_basis`, and PF's single section is the
  annulus r_i = OD/2 - (t - c_m). kd5t_compare's single-family producer ("pre-u6") judges exactly
  that: unsuffixed stress rows on one section. The adapter names PF's section `reduced_section`
  (wall less mill tolerance, no allowance on PF), which is that producer's lookup key;
- the K-D5 standing in the `NUMERICAL_INTEGRITY_*` diagnostic (its message carries the gate's
  reciprocal_condition_estimate, scale_exponents and global_dof_map, as on the exact route), and
  the circular maxima with `pipe_stress_extrema` evidence (value_lower_pa, value_upper_pa) and the
  `max_open_formula_stress` headline;
- no group bound B, est, ||K~_s||_1 or epsilon_E. As on the exact route, "B >= actual group error"
  is judged through the decision: a Passed case asserts that no judged group fired, so a judged
  group whose actual error exceeds RD(1e-9) q refutes the bound;
- three derived kinds the exact-route harness never saw: `element_global_end_force_v1` and
  `_moment_v1` (F = R^T f; DEL-04-01 global-end-actions.md), `element_local_normal_stress_extreme`
  (sigma+-) and `element_local_transverse_shear_stress_maximum` (tau_v,max; DEL-04-07
  u4b-u5b-implementation.md sections 2.3, 2.4). These carry no certificate ("No certificate";
  "receive no certificate of their own"), so they are judged here as information (C11), not as C1.

Acceptance: kd5t_compare's clauses C0-C4, C6, C8 and C9 (the K-D5 M3 criterion and the soundness of its
judging), applied to every run, whatever the model's admission expectation: a Passed publication on a
model the design refuses is judged against the exact reference like any other.

Information clauses added for PF (counted per run in `info`, reported beside the verdict):
- C10 [design pf-timoshenko.md section 2, admission]: a Timoshenko model the design refuses by name
  (binary64 nu_d outside (-1, 1/2), or the moduli-form factor not representable) that is not refused,
  or is refused without `PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE`; or a model the design admits that is
  refused.
- C11 [own; DEL-04-07 section 2.3, 2.4; DEL-04-01 global end actions]: on Passed cases, a derived
  row group (global end force or moment vector per end, sigma+- pair, tau_v,max) beyond RD(1e-9) of
  its body's coupled scale in the row's unit (SF; SM; SM/Z; SF c), or an expected derived row
  unpublished; a tau row on a member whose nu_d is out of domain, or the
  `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN` warning missing or unexpected.
kd5t_compare's information clauses C3lit, C4b, C5, C6i and C7 are reported as there.
"""

from __future__ import annotations

import collections
import copy
import json
import math
import os
import sys
from decimal import Decimal
from fractions import Fraction as Q

HERE = os.path.dirname(os.path.abspath(__file__))
sys.dont_write_bytecode = True
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, "..", "kd5_timoshenko"))
import kd5t_compare as KC  # noqa: E402  (read-only reuse of the reconstructed M3 judge)

CRIT, FLOOR = KC.CRIT, KC.FLOOR
PRODUCER = "pre-u6"            # kd5t_compare's single-family producer: unsuffixed stress rows on one section
SECTION = "reduced_section"    # its lookup key; PF's single section (wall less mill tolerance)
REFUSAL_CODE = "PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE"
TAU_WARNING = "TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN"
DERIVED = {"element_global_end_force_v1", "element_global_end_moment_v1", "element_local_normal_stress_extreme",
           "element_local_transverse_shear_stress_maximum"}
INFO = dict(KC.INFO)
INFO.update({"C10": "admission differs from design section 2 (an expected refusal published or refused under another code, or an admitted model refused)",
             "C11": "derived row (global end action, sigma+-, tau_v,max) beyond 1e-9 of its coupled scale, unpublished, or its tau-domain warning differs (Passed)"})
CLAUSES = KC.CLAUSES


def fq(text):
    return Q(Decimal(text))


def kd5_ref(ref):
    """The PF reference in kd5t_compare's single-family shape: PF's section under the producer's lookup key."""
    sections = {pid: {"ro": s["ro"], SECTION: {"A": s["A"], "Z": s["Z"], "J": s["J"]}} for pid, s in ref["sections"].items()}
    cases = {}
    for cid, c in ref["cases"].items():
        rows = [[k, e, l, comp, (SECTION if k in KC.STRESS_KINDS else None), v] for k, e, l, comp, _, v in c["rows"] if k not in DERIVED]
        cases[cid] = {"rows": rows, "maxima": [[p, SECTION, v] for p, _, v in c["maxima"]]}
    return {"sections": sections, "nodes": ref["nodes"], "cases": cases}


def split_run(run):
    """The run without its derived rows (for kd5t_compare's judge), and those rows."""
    if not run.get("results"):
        return run, []
    core = dict(run)
    core["results"] = [r for r in run["results"] if r["kind"] not in DERIVED]
    return core, [r for r in run["results"] if r["kind"] in DERIVED]


def refused(run):
    return any(d.get("severity") == "blocking" for d in run.get("diagnostics") or [])


def admission_info(ref, run):
    """C10: the run's admission against the design's (information)."""
    if run.get("error"):
        return 0
    codes = {d["code"] for d in run.get("diagnostics") or [] if d.get("severity") == "blocking"}
    if ref.get("expect") == "refused":
        return 0 if REFUSAL_CODE in codes else 1
    return 1 if codes else 0


def derived_info(ref, run, rec, derived_rows):
    """C11 on a Passed case: derived groups beyond 1e-9 of their coupled scale, unpublished derived rows, and the tau warning."""
    sections = ref["sections"]
    out = {"n_groups": 0, "beyond": [], "missing": 0, "unexpected": 0, "warning_mismatch": 0, "worst": 0.0}
    codes = {d["code"] for d in run.get("diagnostics") or []}
    out["warning_mismatch"] = int((TAU_WARNING in codes) != (TAU_WARNING in ref.get("expected_warnings", [])))
    for cid, c in ref["cases"].items():
        cs = rec.get("cases", {}).get(cid)
        if cs is None:
            continue
        SF = max(Q(cs["SF"]), Q(cs["SF_ref"]))
        SM = max(Q(cs["SM"]), Q(cs["SM_ref"]))
        refrows = {(k, e, l, comp): fq(v) for k, e, l, comp, _, v in c["rows"] if k in DERIVED}
        pub = {}
        for r in derived_rows:
            if r.get("case") != cid:
                continue
            key = (r["kind"], r["entity_ref"], r.get("location"), r.get("component"))
            if key not in refrows or key in pub or not KC.finite_value(r.get("value")):
                out["unexpected"] += 1
                continue
            pub[key] = Q(r["value"])
        out["missing"] += sum(1 for k in refrows if k not in pub)
        groups = collections.defaultdict(list)
        for key, v in pub.items():
            kind, pid, loc, comp = key
            groups[(kind, pid, loc)].append((v, refrows[key]))
        for (kind, pid, loc), vals in groups.items():
            s = sections[pid]
            if kind == "element_global_end_force_v1":
                scale = SF
            elif kind == "element_global_end_moment_v1":
                scale = SM
            elif kind == "element_local_normal_stress_extreme":
                scale = SM / (fq(s["Z"]) * KC.MPA)
            else:
                scale = SF * fq(s["tau_c"]) / KC.MPA
            e2 = sum((a - b) ** 2 for a, b in vals)
            out["n_groups"] += 1
            ratio = KC.qratio(e2, CRIT ** 2 * scale * scale)
            out["worst"] = max(out["worst"], ratio)
            if e2 > CRIT ** 2 * scale * scale:
                out["beyond"].append([cid, kind, pid, loc, ratio])
    out["beyond"] = out["beyond"][:20] + ([["...", len(out["beyond"]) - 20]] if len(out["beyond"]) > 20 else [])
    return out


def judge(gen, ref, run):
    """kd5t_compare's judgement of one run on the PF reference, with the PF information clauses."""
    core, derived = split_run(run)
    rec = KC.judge_run(gen, kd5_ref(ref), core, PRODUCER)
    rec["producer"] = "pf"
    info = collections.Counter(rec.get("info") or {})
    adm = admission_info(ref, run)
    rec["expect"] = ref.get("expect", "rows")
    info["C10"] += adm
    if rec.get("cls") == "passed" and run.get("results") and not refused(run):
        d = derived_info(ref, run, rec, derived)
        rec["derived"] = d
        info["C11"] += len([b for b in d["beyond"] if b[0] != "..."]) + max(0, len(d["beyond"]) - 20) + d["missing"] + d["warning_mismatch"]
        info["C6i"] += d["unexpected"]
    rec["info"] = {k: v for k, v in info.items() if v}
    return rec


def work(job):
    """kd5t_compare.work's typed/captured pairing (C4, C4b) around the PF judge."""
    gen, ref, runs = job
    by = {(r["entry"], r["mode"]): r for r in runs}
    judged = {}
    for run in runs:
        try:
            judged[(run["entry"], run["mode"])] = judge(gen, ref, run)
        except Exception:
            import traceback
            judged[(run["entry"], run["mode"])] = {"tag": gen["tag"], "entry": run["entry"], "mode": run["mode"], "reference_error": traceback.format_exc(),
                                                   "verdict_failures": {"C0": 1}}
    out = []
    for (entry, mode), rec in judged.items():
        if entry == "captured":
            other, orec = by.get(("typed", mode)), judged.get(("typed", mode))
            if other is not None and orec is not None and "reference_error" not in orec and "reference_error" not in rec:
                run = by[(entry, mode)]
                rec["typed_captured_rows_identical"] = KC.row_fingerprint(other) == KC.row_fingerprint(run)
                a = [d["code"] for d in other.get("diagnostics") or []]
                b = [d["code"] for d in run.get("diagnostics") or []]
                rec["captured_extra_codes"] = sorted(set(b) - set(a))
                rec["typed_extra_codes"] = sorted(set(a) - set(b))
                rec["typed_captured_disagree"] = (orec.get("integrity") != rec.get("integrity") or len(orec.get("breaches", [])) != len(rec.get("breaches", [])))
                vf = rec["verdict_failures"]
                vf["C4"] = vf.get("C4", 0) + int(rec["typed_captured_disagree"])
                inf = rec.setdefault("info", {})
                inf["C4b"] = inf.get("C4b", 0) + int(not rec["typed_captured_rows_identical"])
        rec.pop("_groups", None)
        rec["meta"] = {k: gen["meta"].get(k) for k in ("design", "family", "theory", "placement", "section_variant", "twin", "E", "G")}
        rec["verdict_failures"] = {k: v for k, v in rec["verdict_failures"].items() if v}
        rec["info"] = {k: v for k, v in rec.get("info", {}).items() if v}
        rec["verdict"] = "fail" if rec["verdict_failures"] else "pass"
        out.append(rec)
    return out


# ----------------------------------------------------------------- self-test
def synthetic_run(gen, upper_scale=1.0 + 1e-12, entry="typed"):
    """A PF-shaped run that publishes the reference rows themselves (rounded to binary64), Passed."""
    ref = gen["ref"]
    rows, cases_ev = [], []
    for cid, c in ref["cases"].items():
        for kind, ent, loc, comp, _, val in c["rows"]:
            rows.append({"kind": kind, "value": float(Decimal(val)), "entity_ref": ent, "case": cid, "location": loc, "component": comp,
                         "metadata": {"basis": "recovered_from_open_mechanics_stress_components"}})
        ev = []
        for pid, _, val in c["maxima"]:
            rows.append({"kind": "pipe_elastic_normal_stress_maximum_v2", "value": float(Decimal(val)), "entity_ref": pid, "case": cid,
                         "location": "governing_station", "component": "maximum_absolute_normal_stress", "metadata": {"basis": "x"}})
            ev.append({"pipe_id": pid, "value_upper_pa": float(Decimal(val)) * upper_scale, "value_lower_pa": float(Decimal(val)) / upper_scale})
        cases_ev.append({"load_case_id": cid, "pipe_stress_extrema": ev})
    mx = [r["value"] for r in rows if r["kind"] == "pipe_elastic_normal_stress_maximum_v2"]
    diags = [{"code": "NUMERICAL_INTEGRITY_CHECKS_PASSED", "severity": "info", "message": "synthetic"}]
    diags += [{"code": w, "severity": "warning", "message": "synthetic"} for w in ref.get("expected_warnings", [])]
    if ref.get("expect") == "refused":
        diags = [{"code": REFUSAL_CODE, "severity": "blocking", "message": "synthetic"}]
        rows, cases_ev, mx = [], [], []
    return {"tag": gen["tag"], "entry": entry, "mode": "SparseInteractive", "results": rows,
            "summary": {"max_open_formula_stress": {"value": max(mx), "unit": "Pa"}} if mx else {},
            "diagnostics": diags, "contract_evidence": {"preview_cases": cases_ev}}


def selftest(set_path):
    """Exercise every rule and clause on synthetic PF runs built from the reference itself."""
    gens = [json.loads(l) for l in open(set_path)]
    by_tag = {g["tag"]: g for g in gens}
    out = []

    def check(name, cond):
        out.append((name, bool(cond)))
        print(("PASS " if cond else "FAIL ") + name)

    def J(g, run, ref=None):
        return judge(g, ref or g["ref"], run)

    def W(g, runs, ref=None):
        return work((g, ref or g["ref"], runs))

    def evidence(run):
        return [e for c in run["contract_evidence"]["preview_cases"] for e in c["pipe_stress_extrema"]]

    # predicates at their exact boundaries (kd5t_compare's, used unchanged)
    check("CRIT is RD(1e-9) = 0x1.12e0be826d694p-30", CRIT == Q(float.fromhex("0x1.12e0be826d694p-30")) and CRIT <= Q(1, 10 ** 9))
    q2 = Q(7, 3)
    check("breach is strict at e^2 = CRIT^2 q^2", not KC.group_breach(CRIT ** 2 * q2, q2) and KC.group_breach(CRIT ** 2 * q2 * (1 + Q(1, 2 ** 60)), q2))
    s, conv = Q(5), Q(3)
    t2 = FLOOR ** 2 * s ** 2 / conv ** 2
    check("floor inclusive, guard 2^-988 inclusive", KC.group_judged(Q(0), t2, conv, s) and not KC.group_judged(t2 * (1 - Q(1, 2 ** 60)), Q(0), conv, s)
          and KC.group_judged(Q(1), Q(1), Q(1), KC.GUARD) and not KC.group_judged(Q(1), Q(1), Q(1), KC.GUARD * (1 - Q(1, 2 ** 60))))

    def pick(design, placement="o", variant="m0", th="ts"):
        return next(g for g in gens if g["meta"]["design"] == design and g["meta"]["placement"] == placement
                    and g["meta"]["section_variant"] == variant and g["tag"].endswith("|" + th))

    chosen = [pick("CANT03"), pick("CANT03", th="eb"), pick("FF05"), pick("L13"), pick("SPAN04"), pick("CANT09", variant="cm"),
              pick("CANT07", "utmskew"), pick("NCcodexS8"), pick("NCauxfree", "utmskew"), pick("NCtwoxe-12S8")]
    pinned = set()
    for g in chosen:
        name = g["tag"].split("|", 1)[1]
        run = synthetic_run(g)
        rec = J(g, run)
        check(f"{name}: the reference published as itself passes every clause and raises no information clause",
              not rec["verdict_failures"] and not rec["info"] and rec["n_judged"] > 0 and rec["n_judged_ref_only"] == 0
              and rec["derived"]["n_groups"] > 0 and not rec["derived"]["beyond"])
        # CRIT from both sides on the largest judged group of every group kind present
        idx = collections.defaultdict(list)
        for i, r in enumerate(run["results"]):
            if r["kind"] in KC.SKIP or r["kind"] in DERIVED or r["kind"] == "pipe_elastic_normal_stress_maximum_v2":
                continue
            idx[KC._gkey_of(dict(r, metadata={}))].append(i)
        judged = {(x["group"], x["entity"], x["loc"], x["basis"]) for x in J(g, run)["_groups"] if x["judged"]}
        done = set()
        for gk in sorted(idx, key=lambda k: -sum(run["results"][i]["value"] ** 2 for i in idx[k])):
            if gk[0] in done or gk not in judged:
                continue
            done.add(gk[0])
            ii = idx[gk]
            qn = math.sqrt(sum(run["results"][i]["value"] ** 2 for i in ii))
            big = max(ii, key=lambda i: abs(run["results"][i]["value"]))
            for factor, expect in ((1 + 1e-5, True), (1 - 1e-5, False)):
                r2 = copy.deepcopy(run)
                v = r2["results"][big]["value"]
                r2["results"][big]["value"] = v + math.copysign(factor * float(CRIT) * qn, v)
                rr = J(g, r2)
                hit = any((b["group"], b["entity"], b["loc"], b["basis"]) == gk for b in rr["breaches"] + rr["disp_breaches"])
                check(f"{name}: {gk[0]} at {factor:.5f} CRIT {'breaches' if expect else 'does not breach'}", hit == expect)
        pinned |= done
        cs = J(g, run)["cases"]["case:1"]
        # C3 from both sides: a component error of (1 +- 1e-5) CRIT S*_kind
        for prefix, star in (("global_nodal_displacement", "Sstar_t"), ("global_nodal_rotation", "Sstar_r")):
            if not cs[star]:
                continue
            ii = [i for i, r in enumerate(run["results"]) if r["kind"].startswith(prefix)]
            i = min(ii, key=lambda i: abs(run["results"][i]["value"]))
            for factor, expect in ((1 + 1e-5, True), (1 - 1e-5, False)):
                r2 = copy.deepcopy(run)
                r2["results"][i]["value"] += factor * float(CRIT) * cs[star]
                check(f"{name}: {prefix} error {factor:.5f} CRIT S*_kind {'fails' if expect else 'passes'} C3",
                      (J(g, r2)["verdict_failures"].get("C3", 0) > 0) == expect)
        # F1: an all-zero publication fails C1 on the reference scales
        r2 = copy.deepcopy(run)
        for r in r2["results"]:
            r["value"] = 0.0
        vf = W(g, [r2])[0]["verdict_failures"]
        check(f"{name}: an all-zero publication fails C1 (reference scales), no C0", "C0" not in vf and vf.get("C1", 0) > 0)
        # F2: a null and an infinite value fail C1 and C6, no exception; a null evidence upper fails C2
        r2 = copy.deepcopy(run)
        kinds = [r["kind"] for r in r2["results"]]
        r2["results"][kinds.index("element_local_bending_moment_z")]["value"] = None
        r2["results"][kinds.index("element_local_shear_force_y")]["value"] = math.inf
        vf = W(g, [r2])[0]["verdict_failures"]
        check(f"{name}: a null and an infinite published value fail C1 and C6", "C0" not in vf and vf.get("C1", 0) == 2 and vf.get("C6", 0) == 2)
        r2 = copy.deepcopy(run)
        evidence(r2)[0]["value_upper_pa"] = None
        check(f"{name}: a null evidence upper fails C2", W(g, [r2])[0]["verdict_failures"].get("C2", 0) > 0)
        # C2 and C7: an upper bound 0.1% low; a headline below its maximum; missing evidence
        r2 = copy.deepcopy(run)
        e0 = max(evidence(r2), key=lambda e: e["value_upper_pa"])
        e0["value_upper_pa"] *= 0.999
        rr = J(g, r2)
        check(f"{name}: an upper bound 0.1% low fails C2 and raises C7", rr["verdict_failures"].get("C2", 0) > 0 and rr["info"].get("C7", 0) > 0)
        r2 = copy.deepcopy(run)
        r2["summary"]["max_open_formula_stress"]["value"] *= 0.5
        check(f"{name}: a headline below the maximum raises C7", J(g, r2)["info"].get("C7", 0) > 0)
        r2 = copy.deepcopy(run)
        r2["contract_evidence"]["preview_cases"][0]["pipe_stress_extrema"] = []
        check(f"{name}: missing maximum evidence raises C7", J(g, r2)["info"].get("C7", 0) > 0)
        # F3 and C6: interior station rows dropped
        r2 = copy.deepcopy(run)
        n0 = len(r2["results"])
        r2["results"] = [r for r in r2["results"] if not (r["kind"] not in DERIVED and r.get("location") in ("quarter_1", "midspan", "quarter_3"))]
        nd = n0 - len(r2["results"])
        rr = J(g, r2)
        check(f"{name}: dropped interior station rows ({nd}) fail C6", nd > 0 and rr["verdict_failures"].get("C6", 0) == nd)
        # C8: a Passed run with no rows; a gross support moment
        r2 = copy.deepcopy(run)
        r2["results"] = []
        check(f"{name}: a Passed run with no rows fails C8", J(g, r2)["verdict_failures"].get("C8", 0) == 1)
        r2 = copy.deepcopy(run)
        i = next(i for i, r in enumerate(r2["results"]) if r["kind"] == "support_reaction_component_v2" and r["component"] in ("Mx", "My", "Mz"))
        r2["results"][i]["value"] = 1e20
        rr = J(g, r2)
        check(f"{name}: a gross support moment fails C1 and C8", rr["verdict_failures"].get("C1", 0) > 0 and rr["verdict_failures"].get("C8", 0) > 0)
        # C6i: an unknown row; C6: a dropped row; a basis-suffixed stress row on PF's single family
        r2 = copy.deepcopy(run)
        r2["results"].append({"kind": "element_local_bending_moment_q", "value": 1.0, "entity_ref": "x", "case": "case:1", "location": "midspan", "component": "q", "metadata": {}})
        i = next(i for i, r in enumerate(r2["results"]) if r["kind"] == "element_local_axial_force")
        r2["results"].pop(i)
        rr = J(g, r2)
        check(f"{name}: an unknown row raises C6i and a dropped row fails C6", rr["info"].get("C6i", 0) == 1 and rr["verdict_failures"].get("C6", 0) == 1)
        r2 = copy.deepcopy(run)
        i = next(i for i, r in enumerate(r2["results"]) if r["kind"] == "element_local_torsional_shear_stress")
        r2["results"][i]["metadata"]["basis"] += "; section_basis=reduced_section"
        check(f"{name}: a basis-suffixed stress row on PF's single family raises C6i", J(g, r2)["info"].get("C6i", 0) > 0)
        # C9: a second value of one identity (four orders); on a Sensitive run C6i; a same-value repeat; a second maximum
        kinds = [r["kind"] for r in run["results"]]
        i = max((j for j, k in enumerate(kinds) if k.startswith("element_local_bending_moment_")), key=lambda j: abs(run["results"][j]["value"]))
        v = run["results"][i]["value"]
        for label, value, before, c1 in (("one ulp off, after", math.nextafter(v, math.inf), False, False), ("one ulp off, before", math.nextafter(v, math.inf), True, False),
                                         ("doubled, after", 2 * v, False, False), ("doubled, before", 2 * v, True, True)):
            r2 = copy.deepcopy(run)
            dup = copy.deepcopy(r2["results"][i])
            dup["value"] = value
            r2["results"].insert(i if before else i + 1, dup)
            vf = W(g, [r2])[0]["verdict_failures"]
            check(f"{name}: a second value of one row identity ({label}) fails C9 once" + (" and C1" if c1 else ", C1 clean"),
                  vf.get("C9", 0) == 1 and (vf.get("C1", 0) > 0) == c1 and "C0" not in vf)
            if label == "doubled, after":
                r2["diagnostics"] = [{"code": "NUMERICAL_INTEGRITY_SENSITIVE", "severity": "warning", "message": "synthetic"}]
                res = W(g, [r2])[0]
                check(f"{name}: the same conflict on a Sensitive run is C6i information, not C9", not res["verdict_failures"] and res["info"].get("C6i", 0) == 2)
        r2 = copy.deepcopy(run)
        r2["results"].insert(i + 1, copy.deepcopy(r2["results"][i]))
        res = W(g, [r2])[0]
        check(f"{name}: a same-value repeat raises C6i and fails no clause", not res["verdict_failures"] and res["info"].get("C6i", 0) == 1)
        r2 = copy.deepcopy(run)
        dup = copy.deepcopy(next(r for r in r2["results"] if r["kind"] == "pipe_elastic_normal_stress_maximum_v2"))
        dup["value"] *= 2
        r2["results"].append(dup)
        check(f"{name}: a second, different maximum row fails C9 once", W(g, [r2])[0]["verdict_failures"].get("C9", 0) == 1)
        # C4 and C4b: typed/captured
        cap = copy.deepcopy(run)
        cap["entry"] = "captured"
        j = kinds.index("element_local_bending_moment_z")
        cap["results"][j]["value"] = math.nextafter(cap["results"][j]["value"], math.inf)
        res = {r["entry"]: r for r in W(g, [run, cap])}
        check(f"{name}: a captured row one ulp off raises C4b", res["captured"]["info"].get("C4b", 0) == 1)
        cap["diagnostics"] = [{"code": "NUMERICAL_INTEGRITY_SENSITIVE", "severity": "warning", "message": "synthetic"}]
        res = {r["entry"]: r for r in W(g, [run, cap])}
        check(f"{name}: a captured integrity code that differs fails C4", res["captured"]["verdict_failures"].get("C4", 0) == 1)
        # C5 and C10: a refusal of an admitted model fails no acceptance clause and raises C5 and C10
        r2 = copy.deepcopy(run)
        r2["diagnostics"] = [{"code": REFUSAL_CODE, "severity": "blocking", "message": "x"}]
        r2["results"] = []
        rr = J(g, r2)
        check(f"{name}: refusing an admitted model raises C5 and C10, fails no acceptance clause", not rr["verdict_failures"] and rr["info"].get("C5") == 1 and rr["info"].get("C10") == 1)
        # C11: derived rows from both sides of 1e-9 of their coupled scale; unpublished; unexpected
        d = J(g, run)
        SF, SM = Q(d["cases"]["case:1"]["SF"]), Q(d["cases"]["case:1"]["SM"])
        for kind, comp, scale_of in (("element_global_end_force_v1", "Fy", lambda s: SF), ("element_global_end_moment_v1", "Mz", lambda s: SM),
                                     ("element_local_normal_stress_extreme", "sigma_plus", lambda s: SM / (fq(s["Z"]) * KC.MPA)),
                                     ("element_local_transverse_shear_stress_maximum", "tau_v_max", lambda s: SF * fq(s["tau_c"]) / KC.MPA)):
            ii = [i for i, r in enumerate(run["results"]) if r["kind"] == kind and r["component"] == comp]
            if not ii:
                continue
            i = ii[0]
            sc = scale_of(g["ref"]["sections"][run["results"][i]["entity_ref"]])
            for factor, expect in ((1 + 1e-5, True), (1 - 1e-5, False)):
                r2 = copy.deepcopy(run)
                r2["results"][i]["value"] = float(Q(r2["results"][i]["value"]) + Q(factor) * CRIT * sc)
                rr = J(g, r2)
                check(f"{name}: {kind} {comp} error {factor:.5f} CRIT scale {'raises' if expect else 'does not raise'} C11, fails no acceptance clause",
                      (rr["info"].get("C11", 0) > 0) == expect and not rr["verdict_failures"])
            r2 = copy.deepcopy(run)
            r2["results"].pop(i)
            check(f"{name}: an unpublished {kind} row raises C11", J(g, r2)["info"].get("C11", 0) == 1)
    every = {"disp", "rot", "dispmag", "reactF", "reactM", "reactFmag", "reactMmag", "axial", "shear", "torsion", "bending", "s_axial", "s_bending", "s_torsion"}
    check(f"CRIT pinned from both sides on every PF group kind (missing: {sorted(every - pinned)})", every <= pinned)
    # Codex's defect: the guided cantilever's UY published 55.6% low on a Passed case fails C3 (forces are phi-free here)
    g = pick("NCcodexS8")
    run = synthetic_run(g)
    r2 = copy.deepcopy(run)
    for r in r2["results"]:
        if r["kind"] in ("global_nodal_displacement_y", "displacement_magnitude") and r["entity_ref"] == "node:N1":
            r["value"] *= 1 - 0.556
    vf = W(g, [r2])[0]["verdict_failures"]
    check("Codex's case: UY published 55.6% low (r1's defect) on a Passed case fails C3", vf.get("C3", 0) > 0)
    # the twin control: a high-phi Timoshenko model judged against its Euler-Bernoulli twin fails
    g = pick("CANT09", variant="cm")
    check("sensitivity control: the Timoshenko reference published as itself fails against the Euler-Bernoulli twin (CANT09, phi 12)",
          W(g, [synthetic_run(g)], by_tag[g["meta"]["twin"]]["ref"])[0]["verdict"] == "fail")
    # an exactly zero rotation field (guided tip): rotation noise passes C3 and raises C3lit
    g = pick("NCcodexS8")
    run = synthetic_run(g)
    r2 = copy.deepcopy(run)
    i = next(i for i, r in enumerate(r2["results"]) if r["kind"] == "global_nodal_rotation_z" and r["entity_ref"] == "node:N1")
    r2["results"][i]["value"] = 1.15e-15 * max(abs(r["value"]) for r in run["results"] if r["kind"].startswith("global_nodal_displacement")) / 1000.0
    rr = J(g, r2)
    check("rotation noise in an exactly zero rotation field passes C3 and raises C3lit", not rr["verdict_failures"] and rr["info"].get("C3lit", 0) > 0)
    # no displacement-noise floor: an all-zero displacement field published as exact zeros passes, as noise fails C3
    g = copy.deepcopy(pick("CANT03"))
    for r in g["ref"]["cases"]["case:1"]["rows"]:
        if r[0].startswith("global_nodal_") or r[0] == "displacement_magnitude":
            r[5] = "0"
    run = synthetic_run(g)
    check("an all-zero displacement field published as exact zeros passes C3", J(g, run)["verdict_failures"].get("C3", 0) == 0)
    r2 = copy.deepcopy(run)
    for r in r2["results"]:
        if r["kind"].startswith("global_nodal_"):
            r["value"] = 1e-15
    check("an all-zero displacement field published as noise fails C3", J(g, r2)["verdict_failures"].get("C3", 0) > 0)
    # a second body is a harness error (C0)
    g = copy.deepcopy(pick("CANT03"))
    g["request"]["model"]["nodes"].append({"id": "node:island", "position": {"x": 0.0, "y": 0.0, "z": 0.0}})
    check("a model with a second body is a harness error (C0)", W(g, [synthetic_run(g)])[0]["verdict_failures"].get("C0", 0) == 1)
    # admission (C10): expected refusals refused by name, refused by another code, or published
    gs = [x for x in gens if x["ref"].get("expect") == "refused"]
    check(f"the frozen set has expected refusals ({len(gs)})", len(gs) > 0)
    for g in gs[:3]:
        name = g["tag"].split("|", 1)[1]
        run = synthetic_run(g)
        rr = J(g, run)
        check(f"{name}: refused by name passes, C10 0, C5 1", not rr["verdict_failures"] and rr["info"].get("C10", 0) == 0 and rr["info"].get("C5") == 1)
        r2 = copy.deepcopy(run)
        r2["diagnostics"][0]["code"] = "MODEL_SECTION_INVALID"
        check(f"{name}: refused under another code raises C10", J(g, r2)["info"].get("C10", 0) == 1)
        g2 = copy.deepcopy(g)
        g2["ref"]["expect"] = "rows"
        pub = synthetic_run(g2)
        rr = J(g, pub)
        check(f"{name}: publishing the exact rows of an expected refusal raises C10 and passes the M3 clauses", rr["info"].get("C10", 0) == 1 and not rr["verdict_failures"])
        r2 = copy.deepcopy(pub)
        for r in r2["results"]:
            if r["kind"] == "global_nodal_displacement_y":
                r["value"] *= 0.5
        check(f"{name}: publishing wrong rows of an expected refusal as Passed fails C3", J(g, r2)["verdict_failures"].get("C3", 0) > 0)
    # the tau domain warning (C11): an out-of-domain EB twin publishes no tau rows and the warning
    g = next(x for x in gens if x["ref"].get("expected_warnings") and x["meta"]["theory"] == "euler_bernoulli")
    run = synthetic_run(g)
    rr = J(g, run)
    check(f"{g['tag'].split('|', 1)[1]}: no tau rows and the out-of-domain warning pass with C11 0", not rr["verdict_failures"] and not rr["info"].get("C11"))
    r2 = copy.deepcopy(run)
    r2["diagnostics"] = [d for d in r2["diagnostics"] if d["code"] != TAU_WARNING]
    check(f"{g['tag'].split('|', 1)[1]}: the warning missing raises C11", J(g, r2)["info"].get("C11", 0) == 1)
    r2 = copy.deepcopy(run)
    r2["results"].append({"kind": "element_local_transverse_shear_stress_maximum", "value": 1.0, "entity_ref": "pipe:N01", "case": "case:1",
                          "location": "midspan", "component": "tau_v_max", "metadata": {}})
    check(f"{g['tag'].split('|', 1)[1]}: a tau row on an out-of-domain member raises C6i", J(g, r2)["info"].get("C6i", 0) == 1)
    bad = [n for n, ok in out if not ok]
    print(f"compare self-test: {len(out) - len(bad)} of {len(out)} checks pass")
    if bad:
        raise SystemExit(1)


# ------------------------------------------------------------- format probes
def probe_format(in_path, out_path):
    """Format facts from the probe runs: kinds, units, locations and components; the integrity message's gate
    fields; and, on the Euler-Bernoulli probes only, each kind's agreement with this engine (units, signs and the
    single section basis). The Timoshenko probe's values are not read."""
    import pft_engine as PE
    reqs = {json.loads(l)["tag"]: json.loads(l)["request"] for l in open(in_path)}
    worst = collections.defaultdict(float)
    units = collections.defaultdict(set)
    gate = set()
    unmatched = collections.Counter()
    refs = {}
    for line in open(out_path):
        run = json.loads(line)
        for r in run.get("results") or []:
            units[r["kind"]].add((r["unit"], (r.get("metadata") or {}).get("coordinate_system")))
        for d in run.get("diagnostics") or []:
            if d["code"].startswith("NUMERICAL_INTEGRITY"):
                gate.add(tuple(f in d["message"] for f in ("reciprocal_condition_estimate: ", "scale_exponents: [", "global_dof_map=[")))
        if not run["tag"].endswith("|eb"):
            continue
        if run["tag"] not in refs:
            refs[run["tag"]] = PE.Reference(reqs[run["tag"]])
        ref = refs[run["tag"]]
        exp = {(k[0], k[1], k[2], k[3]): v for k, v in ref.rows("case:1").items()}
        mx = ref.maxima("case:1")
        scale = collections.defaultdict(float)
        for k, v in exp.items():
            scale[k[0]] = max(scale[k[0]], abs(float(v)))
        for r in run["results"]:
            key = (r["kind"], r["entity_ref"], r.get("location"), r.get("component"))
            if r["kind"] == "pipe_elastic_normal_stress_maximum_v2":
                worst[r["kind"]] = max(worst[r["kind"]], abs(r["value"] - float(mx[r["entity_ref"]])) / float(mx[r["entity_ref"]]))
            elif key in exp:
                worst[r["kind"]] = max(worst[r["kind"]], abs(r["value"] - float(exp[key])) / (scale[r["kind"]] or 1.0))
            else:
                unmatched[r["kind"]] += 1
    print("gate fields in the integrity message (r_hat, scale exponents, dof map):", sorted(gate))
    for k in sorted(units):
        print(f"{k:50s} units {sorted(units[k], key=str)}  worst |product - engine|/kind scale on EB probes {worst.get(k, float('nan')):.2e}")
    print("unmatched EB probe rows by kind:", dict(unmatched))


def main():
    args = sys.argv[1:]
    if args and args[0] == "--selftest":
        selftest(args[1])
        return
    if args and args[0] == "--probe-format":
        probe_format(args[1], args[2])
        return
    variant, workers = "own", 6
    for flag in ("--ref", "--workers"):
        if flag in args:
            i = args.index(flag)
            val = args[i + 1]
            del args[i:i + 2]
            if flag == "--ref":
                variant = val
            else:
                workers = int(val)
    if variant not in ("own", "twin"):
        raise SystemExit("--ref own|twin")
    set_path, runs_path, out_path = args
    gens = {}
    for line in open(set_path):
        x = json.loads(line)
        gens[x["tag"]] = x
    runs = collections.defaultdict(list)
    for line in open(runs_path):
        x = json.loads(line)
        runs[x["tag"]].append(x)
    jobs = []
    for tag, rs in runs.items():
        gen = gens[tag]
        ref = gen["ref"] if variant == "own" else gens[gen["meta"]["twin"]]["ref"]
        jobs.append((gen, ref, rs))
    import multiprocessing as mp
    with open(out_path, "w") as f:
        if workers <= 1:
            results = map(work, jobs)
        else:
            pool = mp.get_context("fork").Pool(workers)
            results = pool.imap_unordered(work, jobs, chunksize=2)
        for result in results:
            for rec in result:
                rec["ref_variant"] = variant
                f.write(json.dumps(rec) + "\n")


if __name__ == "__main__":
    main()
