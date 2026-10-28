"""PF M3-T comparison: the product's published PF rows against the pft exact reference, and the M3 verdict.

The judging and coverage logic is kd5t_compare.py's reconstructed M3 harness, imported unchanged from
../kd5_timoshenko (read-only); see its docstring for every rule and its source. This module adapts it to
the pressure-free (PF) route and adds the PF-only rows and expectations. No product code is read.

Usage:
  python3 -I -B pft_compare.py <set.jsonl> <runs.jsonl> <out.jsonl> [--ref own|twin] [--expect <runner input>] [--workers N]
      writes the judged records to <out.jsonl> and the collection inventory to <out.jsonl>.inventory.json;
      exits 3 when the inventory fails
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

Inventory [refuter P1]: every expected model x entry x mode (typed, captured; SparseInteractive,
DenseScrutiny) exactly once and nothing else; expected models are the set's or a chunk's (`--expect`). A run
identity received more than once is rejected before judging (C0 on that identity; no record of it is judged,
so a later record cannot replace an earlier failure); a record of a model outside the set fails C0.

M3 acceptance: kd5t_compare's clauses C0-C4, C6, C8 and C9 (the K-D5 M3 criterion and the soundness of its
judging), applied to every run, whatever the model's admission expectation: a Passed publication on a
model the design refuses is judged against the exact reference like any other.

Design conformance (separate from M3; `design_failures`, `design_verdict`):
- C10 [design pf-timoshenko.md section 2, admission]: a Timoshenko model the design refuses by name
  (binary64 nu_d outside (-1, 1/2), or the moduli-form factor not representable) that is not refused,
  or is refused without `PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE`; or a model the design admits that is
  refused.

Information clauses added for PF (counted per run in `info`):
- C11 [own; DEL-04-07 section 2.3, 2.4; DEL-04-01 global end actions]: on Passed cases, a derived
  row group (global end force or moment vector per end, sigma+- pair, tau_v,max) beyond RD(1e-9) of
  its body's coupled scale in the row's unit (SF; SM; SM/Z; SF c), unless it is the known tau defect
  below; or an expected derived row unpublished; or the `TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN`
  warning missing or unexpected (a tau row on an out-of-domain member is C6i).
- C11tau_b: a tau_v,max group beyond that scale which equals section 2.4's prescribed binary64 operations:
  the known defect on main (RN(1 + nu_d) cancels as E/(2G) -> 0), ruled by Agent 0 a separate correction
  (option (b)).
- C11ops: the derived rows against the design's prescribed binary64 operations on the published operands:
  sigma+- bit for bit (section 2.3: norm2 correctly rounded, then RN(sigma_N +- h)); tau_v,max within 2^-40
  (section 2.4's sequence; only I_b's binary64 order is not prescribed). Global end rows are judged against
  the ideal target only (their rotation's binary64 order is not prescribed).
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
ENTRIES = ("typed", "captured")
MODES = ("SparseInteractive", "DenseScrutiny")
TAU_OPS_TOL = Q(1, 2 ** 40)    # tau against the prescribed binary64 operations: I_b's binary64 order is not prescribed
CLAUSES = KC.CLAUSES
DESIGN = {"C10": "admission differs from design section 2 (an expected refusal published or refused under another code, or an admitted model refused)"}
INFO = dict(KC.INFO)
INFO.update({"C11": "derived row (global end action, sigma+-, tau_v,max) beyond 1e-9 of its coupled scale and not the known tau defect, "
                    "unpublished, or its tau-domain warning differs (Passed)",
             "C11tau_b": "tau_v,max beyond 1e-9 of its coupled scale but equal to the prescribed binary64 operations: the known defect on main "
                         "(RN(1 + nu_d) cancels), ruled by Agent 0 a separate correction (option (b)) (Passed)",
             "C11ops": "sigma+- not bit for bit RN(sigma_N +- norm2(sigma_y, sigma_z)) of the station's published rows, or tau_v,max not within "
                       "2^-40 of the prescribed operations on the published shear rows (Passed)"})


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
    """C10: the run's admission against the design's (design-conformance verdict, separate from M3)."""
    if run.get("error"):
        return 0
    codes = {d["code"] for d in run.get("diagnostics") or [] if d.get("severity") == "blocking"}
    if ref.get("expect") == "refused":
        return 0 if REFUSAL_CODE in codes else 1
    return 1 if codes else 0


def rn_norm2(a: float, b: float) -> float:
    """The correctly rounded binary64 sqrt(a^2 + b^2) of two binary64 values (the shared norm2, E15)."""
    X = Q(a) ** 2 + Q(b) ** 2
    if X == 0:
        return 0.0
    c = math.hypot(a, b)              # within an ulp or so; corrected exactly below
    if not math.isfinite(c):
        return c
    while True:
        lo, hi = (Q(math.nextafter(c, 0.0)) + Q(c)) / 2, (Q(c) + Q(math.nextafter(c, math.inf))) / 2
        if X < lo * lo:
            c = math.nextafter(c, 0.0)
        elif X > hi * hi:
            c = math.nextafter(c, math.inf)
        else:
            return c


def sigma_ops(sN: float, sy: float, sz: float):
    """u4b-u5b-implementation.md section 2.3's fixed sequence: h = norm2(sigma_y, sigma_z); RN(sigma_N + h), RN(sigma_N - h)."""
    h = rn_norm2(sy, sz)
    return sN + h, sN - h


def tau_cb(sec):
    """Section 2.4's c_b in its fixed binary64 sequence, from the model's E, G, OD, wall and mill tolerance; I_b is the
    exact I rounded once (the product's binary64 I_b order is not prescribed; it agrees to ulps)."""
    E, G = float(sec["E"]), float(sec["G"])
    nu = E / (2.0 * G) - 1.0
    ro = float(sec["od"]) / 2.0
    ri = ro - (float(sec["wall"]) - float(sec["mill"]))
    num = (3.0 + 2.0 * nu) * (ro * ro) + (1.0 + 2.0 * nu) * (ri * ri)
    den = (4.0 * (1.0 + nu)) * float(Decimal(sec["I"]))
    return num / den


def tau_ops(sec, vy: float, vz: float) -> float:
    """Section 2.4: tau = RN(RN(norm2(V_y, V_z) * c_b)/1e6), on the station's published shear rows."""
    return (rn_norm2(vy, vz) * tau_cb(sec)) / 1e6


def derived_info(ref, run, rec, derived_rows, core_rows):
    """On a Passed case: C11 (derived groups beyond 1e-9 of their coupled scale, unpublished derived rows, the tau
    warning), C11tau_b (the known tau defect: beyond, but equal to the prescribed operations) and C11ops (the derived
    rows against the design's prescribed binary64 operations on the published operands)."""
    sections = ref["sections"]
    out = {"n_groups": 0, "beyond": [], "tau_b": [], "ops": [], "missing": 0, "unexpected": 0, "warning_mismatch": 0, "worst": 0.0}
    codes = {d["code"] for d in run.get("diagnostics") or []}
    out["warning_mismatch"] = int((TAU_WARNING in codes) != (TAU_WARNING in ref.get("expected_warnings", [])))
    for cid, c in ref["cases"].items():
        cs = rec.get("cases", {}).get(cid)
        if cs is None:
            continue
        SF = max(Q(cs["SF"]), Q(cs["SF_ref"]))
        SM = max(Q(cs["SM"]), Q(cs["SM_ref"]))
        refrows = {(k, e, l, comp): fq(v) for k, e, l, comp, _, v in c["rows"] if k in DERIVED}
        operands = {}
        for r in core_rows:
            if r.get("case") == cid and KC.finite_value(r.get("value")):
                operands.setdefault((r["kind"], r["entity_ref"], r.get("location")), r["value"])
        pub = {}
        for r in derived_rows:
            if r.get("case") != cid:
                continue
            key = (r["kind"], r["entity_ref"], r.get("location"), r.get("component"))
            if key not in refrows or key in pub or not KC.finite_value(r.get("value")):
                out["unexpected"] += 1
                continue
            pub[key] = r["value"]
        out["missing"] += sum(1 for k in refrows if k not in pub)
        groups = collections.defaultdict(list)
        for key, v in pub.items():
            kind, pid, loc, comp = key
            groups[(kind, pid, loc)].append((comp, v, refrows[key]))
        for (kind, pid, loc), vals in groups.items():
            s = sections[pid]
            op = lambda k: operands.get((k, pid, loc))
            consistent = None
            if kind == "element_global_end_force_v1":
                scale = SF
            elif kind == "element_global_end_moment_v1":
                scale = SM
            elif kind == "element_local_normal_stress_extreme":
                scale = SM / (fq(s["Z"]) * KC.MPA)
                ops = (op("element_local_axial_normal_stress"), op("element_local_bending_normal_stress_y"), op("element_local_bending_normal_stress_z"))
                if None not in ops:
                    sp, sm = sigma_ops(*ops)
                    consistent = all(v == (sp if comp == "sigma_plus" else sm) for comp, v, _ in vals)
            else:
                scale = SF * fq(s["tau_c"]) / KC.MPA
                ops = (op("element_local_shear_force_y"), op("element_local_shear_force_z"))
                if None not in ops:
                    t = tau_ops(s, *ops)
                    consistent = all(abs(Q(v) - Q(t)) <= TAU_OPS_TOL * abs(Q(t)) for _, v, _ in vals)
            e2 = sum((Q(v) - t) ** 2 for _, v, t in vals)
            out["n_groups"] += 1
            ratio = KC.qratio(e2, CRIT ** 2 * scale * scale)
            item = [cid, kind, pid, loc, ratio]
            if consistent is False:
                out["ops"].append(item)
            if e2 > CRIT ** 2 * scale * scale:
                if kind == "element_local_transverse_shear_stress_maximum" and consistent:
                    out["tau_b"].append(item)
                else:
                    out["beyond"].append(item)
                    out["worst"] = max(out["worst"], ratio)
    return out


def judge(gen, ref, run):
    """kd5t_compare's judgement of one run on the PF reference, with the PF information clauses."""
    core, derived = split_run(run)
    rec = KC.judge_run(gen, kd5_ref(ref), core, PRODUCER)
    rec["producer"] = "pf"
    info = collections.Counter(rec.get("info") or {})
    rec["expect"] = ref.get("expect", "rows")
    rec["design_failures"] = {"C10": admission_info(ref, run)}
    if rec.get("cls") == "passed" and run.get("results") and not refused(run):
        d = derived_info(ref, run, rec, derived, core["results"])
        info["C11"] += len(d["beyond"]) + d["missing"] + d["warning_mismatch"]
        info["C11tau_b"] += len(d["tau_b"])
        info["C11ops"] += len(d["ops"])
        info["C6i"] += d["unexpected"]
        for k in ("beyond", "tau_b", "ops"):
            d["n_" + k] = len(d[k])
            d[k] = d[k][:20]
        rec["derived"] = d
    rec["info"] = {k: v for k, v in info.items() if v}
    return rec


def work(job):
    """kd5t_compare.work's typed/captured pairing (C4, C4b) around the PF judge."""
    gen, ref, runs = job
    ident = collections.Counter((r.get("entry"), r.get("mode")) for r in runs)
    dup = {k for k, n in ident.items() if n > 1}
    if dup:
        # a run identity received more than once is rejected before judging: no record of it is judged, so a later
        # record can never replace an earlier failure (refuter P1); the collection inventory fails as well
        return [{"tag": gen["tag"], "entry": e, "mode": m, "duplicate_identity": n, "verdict_failures": {"C0": 1}, "info": {}, "verdict": "fail",
                 "meta": {k: gen["meta"].get(k) for k in ("design", "family", "theory", "placement", "section_variant", "twin", "E", "G")}}
                for (e, m), n in sorted(ident.items()) if (e, m) in dup] + work((gen, ref, [r for r in runs if (r.get("entry"), r.get("mode")) not in dup]))
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
        rec["design_failures"] = {k: v for k, v in (rec.get("design_failures") or {}).items() if v}
        rec["verdict"] = "fail" if rec["verdict_failures"] else "pass"
        rec["design_verdict"] = "fail" if rec["design_failures"] else "pass"
        out.append(rec)
    return out


def inventory(expected_tags, identities):
    """The collection check (refuter P1): every expected model x entry x mode exactly once, and nothing else.
    `identities` are (tag, entry, mode) tuples of the received records."""
    want = {(t, e, m) for t in expected_tags for e in ENTRIES for m in MODES}
    got = collections.Counter(identities)
    out = {"expected": len(want), "received": sum(got.values()),
           "missing": sorted(want - set(got)), "duplicate": sorted(k for k, n in got.items() if n > 1),
           "unexpected": sorted(k for k in got if k not in want)}
    out["pass"] = not (out["missing"] or out["duplicate"] or out["unexpected"])
    return out


def inventory_text(inv):
    return (f"INVENTORY (model x entry x mode): {'PASS' if inv['pass'] else 'FAIL'} ({inv['received']} records against {inv['expected']} expected; "
            f"{len(inv['missing'])} missing, {len(inv['duplicate'])} duplicate, {len(inv['unexpected'])} unexpected)")


# ----------------------------------------------------------------- self-test
def synthetic_run(gen, upper_scale=1.0 + 1e-12, entry="typed", mode="SparseInteractive"):
    """A PF-shaped run that publishes the reference rows themselves (rounded to binary64), with the derived sigma+-
    and tau_v,max formed by the design's binary64 operations from those rows, Passed."""
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
    # sigma+- and tau_v,max as the design prescribes them: binary64 operations on the published operand rows
    val = {(r["case"], r["kind"], r["entity_ref"], r["location"]): r["value"] for r in rows}
    for r in rows:
        k = (r["case"], r["entity_ref"], r["location"])
        if r["kind"] == "element_local_normal_stress_extreme":
            sp, sm = sigma_ops(*(val[(k[0], kind, k[1], k[2])] for kind in ("element_local_axial_normal_stress", "element_local_bending_normal_stress_y",
                                                                            "element_local_bending_normal_stress_z")))
            r["value"] = sp if r["component"] == "sigma_plus" else sm
        elif r["kind"] == "element_local_transverse_shear_stress_maximum":
            r["value"] = tau_ops(ref["sections"][r["entity_ref"]], val[(k[0], "element_local_shear_force_y", k[1], k[2])],
                                 val[(k[0], "element_local_shear_force_z", k[1], k[2])])
    mx = [r["value"] for r in rows if r["kind"] == "pipe_elastic_normal_stress_maximum_v2"]
    diags = [{"code": "NUMERICAL_INTEGRITY_CHECKS_PASSED", "severity": "info", "message": "synthetic"}]
    diags += [{"code": w, "severity": "warning", "message": "synthetic"} for w in ref.get("expected_warnings", [])]
    if ref.get("expect") == "refused":
        diags = [{"code": REFUSAL_CODE, "severity": "blocking", "message": "synthetic"}]
        rows, cases_ev, mx = [], [], []
    return {"tag": gen["tag"], "entry": entry, "mode": mode, "results": rows,
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

    def onepnu_error(g):
        """Relative error of RN(1 + nu_d) against the exact 1 + nu = E/(2G), the known tau defect's size."""
        sec = next(iter(g["ref"]["sections"].values()))
        E, G = float(sec["E"]), float(sec["G"])
        x = Q(E) / (2 * Q(G))
        return abs(Q(1.0 + (E / (2.0 * G) - 1.0)) - x) / x

    chosen = [pick("CANT03"), pick("CANT03", th="eb"), pick("FF05"), pick("L13"), pick("SPAN04"), pick("CANT09", variant="cm"),
              pick("CANT07", "utmskew"), pick("NCcodexS8"), pick("NCauxfree", "utmskew"), pick("NCtwoxe-12S8")]
    pinned = set()
    for g in chosen:
        name = g["tag"].split("|", 1)[1]
        run = synthetic_run(g)
        rec = J(g, run)
        tau_b = rec["info"].get("C11tau_b", 0)
        check(f"{name}: the reference published as itself (sigma+- and tau by the prescribed operations) passes every clause; "
              f"its only information is the known tau defect, exactly where RN(1 + nu_d) is off by more than 1e-12 ({tau_b} groups)",
              not rec["verdict_failures"] and not rec["design_failures"].get("C10") and set(rec["info"]) <= {"C11tau_b"}
              and (tau_b > 0) == (onepnu_error(g) > Q(1, 10 ** 12)) and rec["n_judged"] > 0 and rec["n_judged_ref_only"] == 0
              and rec["derived"]["n_groups"] > 0 and not rec["derived"]["beyond"] and not rec["derived"]["ops"])
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
        # C5 and C10: a refusal of an admitted model fails no M3 acceptance clause, raises C5 and fails design conformance (C10)
        r2 = copy.deepcopy(run)
        r2["diagnostics"] = [{"code": REFUSAL_CODE, "severity": "blocking", "message": "x"}]
        r2["results"] = []
        rr = W(g, [r2])[0]
        check(f"{name}: refusing an admitted model fails design conformance (C10) and raises C5; M3 passes",
              rr["verdict"] == "pass" and rr["info"].get("C5") == 1 and rr["design_verdict"] == "fail" and rr["design_failures"].get("C10") == 1)
        # C11: derived rows from both sides of 1e-9 of their coupled scale; unpublished; unexpected
        d = J(g, run)
        SF, SM = Q(d["cases"]["case:1"]["SF"]), Q(d["cases"]["case:1"]["SM"])
        for kind, comp, scale_of in (("element_global_end_force_v1", "Fy", lambda s: SF), ("element_global_end_moment_v1", "Mz", lambda s: SM),
                                     ("element_local_normal_stress_extreme", "sigma_plus", lambda s: SM / (fq(s["Z"]) * KC.MPA)),
                                     ("element_local_transverse_shear_stress_maximum", "tau_v_max", lambda s: SF * fq(s["tau_c"]) / KC.MPA)):
            ii = [i for i, r in enumerate(run["results"]) if r["kind"] == kind and r["component"] == comp]
            if not ii or (kind == "element_local_transverse_shear_stress_maximum" and tau_b):
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
        rr = W(g, [run])[0]
        check(f"{name}: refused by name passes M3 and design conformance, C5 1", rr["verdict"] == "pass" and rr["design_verdict"] == "pass" and rr["info"].get("C5") == 1)
        r2 = copy.deepcopy(run)
        r2["diagnostics"][0]["code"] = "MODEL_SECTION_INVALID"
        check(f"{name}: refused under another code fails design conformance (C10)", W(g, [r2])[0]["design_failures"].get("C10", 0) == 1)
        g2 = copy.deepcopy(g)
        g2["ref"]["expect"] = "rows"
        pub = synthetic_run(g2)
        rr = W(g, [pub])[0]
        check(f"{name}: publishing the exact rows of an expected refusal fails design conformance (C10) and passes M3",
              rr["design_verdict"] == "fail" and rr["verdict"] == "pass")
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
    # the known tau defect (C11tau_b) and the prescribed operations (C11ops), on Codex's case
    g = pick("NCcodexS8")
    run = synthetic_run(g)
    rr = J(g, run)
    check("Codex's case: tau by the prescribed operations is the known defect (C11tau_b > 0), not C11, and passes the operations check",
          rr["info"].get("C11tau_b", 0) > 0 and not rr["info"].get("C11") and not rr["info"].get("C11ops"))
    ideal = {(c[1], c[2]): float(Decimal(c[5])) for c in g["ref"]["cases"]["case:1"]["rows"] if c[0] == "element_local_transverse_shear_stress_maximum"}
    r2 = copy.deepcopy(run)
    for r in r2["results"]:
        if r["kind"] == "element_local_transverse_shear_stress_maximum":
            r["value"] = ideal[(r["entity_ref"], r["location"])]
    rr = J(g, r2)
    check("Codex's case: tau published at the ideal target is not the prescribed operations (C11ops), and is neither C11 nor C11tau_b",
          rr["info"].get("C11ops", 0) > 0 and not rr["info"].get("C11") and not rr["info"].get("C11tau_b") and not rr["verdict_failures"])
    for g in (pick("NCcodexS8"), pick("CANT03")):
        run = synthetic_run(g)
        r2 = copy.deepcopy(run)
        i = next(i for i, r in enumerate(r2["results"]) if r["kind"] == "element_local_normal_stress_extreme" and r["component"] == "sigma_minus" and r["value"] != 0)
        r2["results"][i]["value"] = math.nextafter(r2["results"][i]["value"], math.inf)
        rr = J(g, r2)
        check(f"{g['tag'].split('|', 1)[1]}: sigma- one ulp off the prescribed operations raises C11ops (bit for bit), not C11",
              rr["info"].get("C11ops", 0) == 1 and not rr["info"].get("C11"))
    check("rn_norm2 is correctly rounded on hard cases", rn_norm2(3.0, 4.0) == 5.0 and rn_norm2(1.0, 2.0 ** -27) == 1.0
          and rn_norm2(1e-200, 1e-200) == float(Decimal(2).sqrt() * Decimal("1e-200")) and rn_norm2(1e200, 1e200) == float(Decimal(2).sqrt() * Decimal("1e200")))
    # the collection (refuter P1): duplicates, missing runs and unexpected records fail; the complete collection passes
    import tempfile
    import pft_report
    g = pick("NCcodexS8")
    clean = synthetic_run(g)
    bad = copy.deepcopy(clean)
    for r in bad["results"]:
        if r["kind"] in ("global_nodal_displacement_y", "displacement_magnitude") and r["entity_ref"] == "node:N1":
            r["value"] *= 1 - 0.556
    for label, runs in (("duplicate-then-correct", [bad, clean]), ("correct-then-duplicate", [clean, bad])):
        recs = W(g, runs)
        check(f"{label} (r1's 55.6%-low UY and a correct record of one identity): the identity is rejected (C0), never judged PASS",
              len(recs) == 1 and recs[0]["verdict"] == "fail" and recs[0]["verdict_failures"] == {"C0": 1} and recs[0]["duplicate_identity"] == 2)
    full_tags = [t for t, _, _ in __import__("pft_generate").model_list()]
    check(f"the full set's inventory is 3,624 runs ({len(full_tags)} models x 2 entries x 2 modes)", inventory(full_tags, [])["expected"] == 3624)
    frozen_tags = [x["tag"] for x in gens]
    check(f"the frozen subset's inventory is {4 * len(frozen_tags)} runs", inventory(frozen_tags, [])["expected"] == 4 * len(frozen_tags) == 400)
    complete = [synthetic_run(x, entry=e, mode=m) for x in gens for e in ENTRIES for m in MODES]
    scenarios = {"complete (positive control)": (complete, True),
                 "duplicate-then-correct": ([bad, clean] + [r for r in complete if (r["tag"], r["entry"], r["mode"]) != (g["tag"], "typed", "SparseInteractive")], False),
                 "correct-then-duplicate": (complete + [bad], False),
                 "a single run": ([clean], False),
                 "an empty file": ([], False),
                 "an unexpected tag": (complete + [dict(clean, tag="pft|NOSUCH|S8NC|o|m0|ts")], False),
                 "unpaired typed sparse and captured dense": ([clean, dict(clean, entry="captured", mode="DenseScrutiny")], False)}
    with tempfile.TemporaryDirectory() as tmp:
        for label, (runs, want) in scenarios.items():
            rp, ep = os.path.join(tmp, "runs.jsonl"), os.path.join(tmp, "eval.jsonl")
            with open(rp, "w") as f:
                f.writelines(json.dumps(r) + "\n" for r in runs)
            inv, _ = compare(set_path, rp, ep, workers=4)
            rep_ = pft_report.report(set_path, [ep], quiet=True)
            check(f"collection '{label}': compare inventory {'passes' if want else 'fails'} and the report's M3 verdict {'passes' if want else 'fails'}",
                  inv["pass"] == want and rep_["inventory"]["pass"] == want and rep_["m3_pass"] == want and rep_["pass"] == want)
        # the CLIs: the compare exits 3 and the report exits 1 on an empty collection
        import subprocess
        rp, ep = os.path.join(tmp, "runs.jsonl"), os.path.join(tmp, "eval.jsonl")
        open(rp, "w").close()
        c1 = subprocess.run([sys.executable, "-I", "-B", os.path.join(HERE, "pft_compare.py"), set_path, rp, ep], capture_output=True, text=True)
        c2 = subprocess.run([sys.executable, "-I", "-B", os.path.join(HERE, "pft_report.py"), set_path, ep], capture_output=True, text=True)
        check("empty collection through the CLIs: pft_compare exits 3 and pft_report exits 1 with INVENTORY FAIL and M3 VERDICT FAIL",
              c1.returncode == 3 and c2.returncode == 1 and "INVENTORY (model x entry x mode): FAIL" in c2.stdout and "M3 VERDICT" in c2.stdout
              and ": FAIL" in c2.stdout.split("M3 VERDICT", 1)[1].split("\n", 1)[0])
        # an admission mismatch fails the design-conformance verdict and leaves M3 passing
        adm = [copy.deepcopy(r) for r in complete]
        for r in adm:
            if r["tag"] == pick("CANT03")["tag"]:
                r["diagnostics"], r["results"] = [{"code": REFUSAL_CODE, "severity": "blocking", "message": "x"}], []
        with open(rp, "w") as f:
            f.writelines(json.dumps(r) + "\n" for r in adm)
        compare(set_path, rp, ep, workers=4)
        rep_ = pft_report.report(set_path, [ep], quiet=True)
        check("an admitted model refused in a complete collection: M3 passes, design conformance fails (4 records)",
              rep_["m3_pass"] and not rep_["design_pass"] and rep_["design_failing_records"] == 4 and not rep_["pass"])
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


def compare(set_path, runs_path, out_path, variant="own", workers=6, expect_paths=None):
    """Judge every received run, write the records, and return (inventory, summary). Expected tags are the set's,
    or those of the given runner inputs (a chunk). Records of tags outside the set are not judged (unexpected)."""
    gens = {}
    for line in open(set_path):
        x = json.loads(line)
        gens[x["tag"]] = x
    expected = list(gens)
    if expect_paths:
        expected = [json.loads(line)["tag"] for p in expect_paths for line in open(p) if line.strip()]
    runs = collections.defaultdict(list)
    idents = []
    for line in open(runs_path):
        if not line.strip():
            continue
        x = json.loads(line)
        idents.append((x.get("tag"), x.get("entry"), x.get("mode")))
        runs[x.get("tag")].append(x)
    inv = inventory(expected, idents)
    jobs = []
    stray = []
    for tag, rs in runs.items():
        if tag not in gens:
            # a record of a model outside the set is not judged; it is written as a failing record (C0) so that
            # the report's inventory sees it too
            stray += [{"tag": tag, "entry": r.get("entry"), "mode": r.get("mode"), "unexpected_tag": True, "verdict_failures": {"C0": 1},
                       "info": {}, "verdict": "fail", "ref_variant": variant} for r in rs]
            continue
        gen = gens[tag]
        ref = gen["ref"] if variant == "own" else gens[gen["meta"]["twin"]]["ref"]
        jobs.append((gen, ref, rs))
    import multiprocessing as mp
    summary = collections.Counter()
    with open(out_path, "w") as f:
        for rec in stray:
            summary["records"] += 1
            summary["M3 failing"] += 1
            f.write(json.dumps(rec) + "\n")
        if workers <= 1 or len(jobs) <= 1:
            results = map(work, jobs)
        else:
            pool = mp.get_context("fork").Pool(workers)
            results = pool.imap_unordered(work, jobs, chunksize=2)
        for result in results:
            for rec in result:
                rec["ref_variant"] = variant
                summary["records"] += 1
                summary["M3 failing"] += bool(rec.get("verdict_failures"))
                summary["design failing"] += bool(rec.get("design_failures"))
                f.write(json.dumps(rec) + "\n")
    with open(out_path + ".inventory.json", "w") as f:
        json.dump(inv, f, indent=1)
    return inv, summary


def main():
    args = sys.argv[1:]
    if args and args[0] == "--selftest":
        selftest(args[1])
        return
    if args and args[0] == "--probe-format":
        probe_format(args[1], args[2])
        return
    variant, workers, expect = "own", 6, None
    if "--expect" in args:
        i = args.index("--expect")
        expect = [args[i + 1]]
        del args[i:i + 2]
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
    inv, summary = compare(set_path, runs_path, out_path, variant, workers, expect)
    print(inventory_text(inv))
    print(f"M3 records failing an acceptance clause: {summary['M3 failing']} of {summary['records']}; "
          f"design-conformance (admission) records failing: {summary['design failing']}")
    if not inv["pass"]:
        raise SystemExit(3)


if __name__ == "__main__":
    main()
