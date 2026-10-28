"""PF M3-T comparison: the product's published PF rows against the pft exact reference, and the M3 verdict.

The judging and coverage logic is kd5t_compare.py's reconstructed M3 harness, imported unchanged from
../kd5_timoshenko (read-only); see its docstring for every rule and its source. This module adapts it to
the pressure-free (PF) route and adds the PF-only rows and expectations. No product code is read.

The judging library of pft_report.py, which is the one judging pass (refutation round 3): it judges a whole
collection of raw runner outputs directly against the reference, every time, and nothing intermediate is read
back. This module's own CLI is its self-test and the format-probe summary:
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

Raw outputs and provenance [refutation round 3]: every raw record is validated exactly against RAW_SCHEMA
(required keys, types, no unknown keys); a record with an `error` key (null included) is a runner failure.
Each record carries request_sha256 (the sha256 of the runner-input line {tag, request} it solved, which must
equal that of the reference's request: request_line), runner_sha256 and product_head; a collection must carry
one runner and one product head. The reference set is validated exactly as well (reference_problems).

Inventory [refuter P1]: every expected model x entry x mode (typed, captured; SparseInteractive,
DenseScrutiny) exactly once, successful and schema-valid, and nothing else. A run
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
import hashlib
import json
import re
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
EULER, TIMOSHENKO = "euler_bernoulli", "timoshenko_annulus_energy_v1"
TAU_WARNING = "TRANSVERSE_SHEAR_POISSON_OUT_OF_DOMAIN"
DERIVED = {"element_global_end_force_v1", "element_global_end_moment_v1", "element_local_normal_stress_extreme",
           "element_local_transverse_shear_stress_maximum"}
JUDGING_FORMAT = "pft-m3t-judging/2"   # the record and sidecar format; bump on any change to what a record means
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
    """kd5t_compare's judgement of one run on the PF reference, with the PF information clauses.

    A run that carries an `error` key (null included: kd5t_compare.judge_run returns no failure for any `error`
    key, refutation round 3, P1) is a runner failure: C0, never judged or skipped. So is a successful run that
    carries neither an integrity standing nor a blocking refusal."""
    if "error" in run:
        return {"tag": gen["tag"], "entry": run.get("entry"), "mode": run.get("mode"), "runner_error": repr(run["error"]),
                "verdict_failures": {"C0": 1}, "info": {}, "design_failures": {"unjudged": 1}}
    core, derived = split_run(run)
    rec = KC.judge_run(gen, kd5_ref(ref), core, PRODUCER)
    rec["producer"] = "pf"
    if not rec.get("integrity") and not refused(run):
        rec["no_standing"] = True
        rec["verdict_failures"]["C0"] = rec["verdict_failures"].get("C0", 0) + 1
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
                 "design_failures": {"unjudged": 1}, "design_verdict": "fail",
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
        # C4 and C4b from the raw runs themselves: the judging pass gives work() every record of a model at once
        if entry == "captured":
            other, orec = by.get(("typed", mode)), judged.get(("typed", mode))
            if (other is not None and orec is not None and not {"reference_error", "runner_error"} & (set(orec) | set(rec))):
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


HEX64 = re.compile(r"\A[0-9a-f]{64}\Z")
HEX40 = re.compile(r"\A[0-9a-f]{40}\Z")
RAW_COMMON = {"tag", "entry", "mode", "ns", "request_sha256", "runner_sha256", "product_head"}
RAW_SUCCESS = RAW_COMMON | {"diagnostics", "results", "results_count", "numerical_quality", "summary", "contract_evidence", "publication"}
RAW_ERROR = RAW_COMMON | {"error"}
ROW_KEYS = {"kind", "value", "unit", "entity_ref", "case", "location", "component", "id", "metadata"}
ROW_METADATA = {"basis", "component", "coordinate_system", "location", "sign_convention"}
DIAG_REQUIRED = {"code", "id", "message", "severity", "source"}
DIAG_KEYS = DIAG_REQUIRED | {"affected_refs"}
SEVERITIES = {"info", "warning", "blocking"}
# The closed sets of outcomes (refutation round 4, P1). Standings: DEL-07-05 numerical-health-indicator.md section 1's
# table; only CHECKS_PASSED is Passed. Refusals: the PF refusals pf-timoshenko.md names (sections 1-3).
STANDINGS = {"NUMERICAL_INTEGRITY_CHECKS_PASSED", "NUMERICAL_INTEGRITY_SENSITIVE", "NUMERICAL_INTEGRITY_UNRESOLVED",
             "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED", "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED",
             "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM", "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY", "NUMERICAL_INTEGRITY_FAILED"}
REFUSALS = {"PF_TIMOSHENKO_SHEAR_FACTOR_UNAVAILABLE", "PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED", "MODEL-SETTING-SHEAR-MODEL-UNKNOWN",
            "MODEL-SETTING-SHEAR-MODEL-WITH-PRESSURE-CONTRACT", "MODEL-SETTING-SHEAR-MODEL-TIMOSHENKO-UNSUPPORTED"}
SOLVER_ENTITY = "solver:linear_static_preview"
SUMMARY_KEYS = {"component_stress_modifier_count", "curved_bend_macro_element_count", "load_case_count", "max_displacement",
                "max_open_formula_stress", "node_count", "segment_count", "spring_hanger_user_input_count", "support_count"}
HEADLINE_KEYS = {"location_ref", "result_ref", "unit", "value"}
EVIDENCE_KEYS = {"combination_gates", "preview_cases"}
CASE_EVIDENCE_KEYS = {"load_case_id", "pipe_stress_extrema", "intensified_measures", "stress_maximum_coverage", "support_attribution"}
EXTREMA_KEYS = {"pipe_id", "value_lower_pa", "value_upper_pa", "approximation", "certified_gap_pa", "coefficient_basis", "enclosure_scope",
                "global_upper_bound_pa", "local_fraction", "result_id", "span_index", "station_fraction", "subdivisions", "section_basis"}
RAW_SCHEMA = """A raw record is exactly one of
  success: {tag, entry, mode, ns, diagnostics, results, results_count, numerical_quality, summary, contract_evidence,
            publication, request_sha256, runner_sha256, product_head}
  runner error: {tag, entry, mode, ns, error, request_sha256, runner_sha256, product_head}
with tag a string, entry in ENTRIES, mode in MODES, ns a nonnegative integer, the two sha256 lowercase hex, product_head
40 lowercase hex; error a nonempty string; results_count = len(results); publication "ordinary". Nested, as the judge
reads them:
  diagnostics: objects with code, id, message, severity (info, warning or blocking) and source strings, optional
    affected_refs (strings), nothing else. Exactly one outcome: either one standing per load case (a code of STANDINGS,
    not blocking, affected_refs exactly [case]) and no blocking diagnostic, or blocking diagnostics of exactly one code,
    of REFUSALS, and no standing. Any other NUMERICAL_INTEGRITY_* code is refused. A refusal publishes no rows, a
    standing at least one;
  results: rows of exactly ROW_KEYS (kind, unit, entity_ref and id strings; case, location and component strings or
    null; value a number or null; metadata null or an object of strings over ROW_METADATA), each row's case a case of
    the model and its entity_ref one of its nodes, pipes or supports (the linear_solver_mode_basis row names
    SOLVER_ENTITY);
  summary: an object over SUMMARY_KEYS; max_open_formula_stress present when not refused, null or an object over
    HEADLINE_KEYS with a numeric or null value. The other summary members are not read;
  contract_evidence: null when refused; otherwise {combination_gates: [], preview_cases: [...]}, each case an object over
    CASE_EVIDENCE_KEYS with a load_case_id string and pipe_stress_extrema a list of objects over EXTREMA_KEYS, each with
    a pipe_id string and value_lower_pa and value_upper_pa numbers or null. intensified_measures,
    stress_maximum_coverage and support_attribution are not read;
  numerical_quality: null or an object; not read.
JSON is read strictly: a duplicate object key, NaN or Infinity is refused (strict_loads)."""


class StrictJSONError(ValueError):
    pass


def _no_duplicates(pairs):
    out = {}
    for k, v in pairs:
        if k in out:
            raise StrictJSONError(f"duplicate key {k!r}")
        out[k] = v
    return out


def _no_constant(name):
    raise StrictJSONError(f"non-JSON constant {name}")


def strict_loads(text):
    """json.loads refusing duplicate object keys (the last member would silently win) and NaN/Infinity."""
    return json.loads(text, object_pairs_hook=_no_duplicates, parse_constant=_no_constant)


def is_int(x):
    return isinstance(x, int) and not isinstance(x, bool)


def is_num(x):
    return isinstance(x, (int, float)) and not isinstance(x, bool)


def raw_problems(r):
    """Every way a raw record departs from RAW_SCHEMA ([] when it conforms)."""
    if not isinstance(r, dict):
        return ["not an object"]
    want = RAW_ERROR if "error" in r else RAW_SUCCESS
    out = [f"missing {k}" for k in sorted(want - set(r))] + [f"unknown key {k}" for k in sorted(set(r) - want)]
    g = r.get
    if not isinstance(g("tag"), str) or not g("tag"):
        out.append("tag")
    if g("entry") not in ENTRIES:
        out.append("entry")
    if g("mode") not in MODES:
        out.append("mode")
    if not is_int(g("ns")) or g("ns") < 0:
        out.append("ns")
    for k in ("request_sha256", "runner_sha256"):
        if not isinstance(g(k), str) or not HEX64.match(g(k)):
            out.append(k)
    if not isinstance(g("product_head"), str) or not HEX40.match(g("product_head")):
        out.append("product_head is not 40 lowercase hex")
    if "error" in r:
        if not isinstance(r["error"], str) or not r["error"]:
            out.append("error is not a nonempty string")
        return out
    ds = g("diagnostics")
    if not isinstance(ds, list):
        out.append("diagnostics")
    else:
        for d in ds:
            if (not isinstance(d, dict) or not DIAG_REQUIRED <= set(d) or not set(d) <= DIAG_KEYS
                    or not all(isinstance(d[k], str) for k in DIAG_REQUIRED) or d["severity"] not in SEVERITIES
                    or ("affected_refs" in d and not (isinstance(d["affected_refs"], list) and all(isinstance(x, str) for x in d["affected_refs"])))):
                out.append("a diagnostic")
                break
    rows = g("results")
    if not isinstance(rows, list):
        out.append("results")
    else:
        for x in rows:
            md = x.get("metadata") if isinstance(x, dict) else None
            if (not isinstance(x, dict) or set(x) != ROW_KEYS or not all(isinstance(x[k], str) for k in ("kind", "unit", "entity_ref", "id"))
                    or not all(x[k] is None or isinstance(x[k], str) for k in ("case", "location", "component"))
                    or not (x["value"] is None or is_num(x["value"]))
                    or not (md is None or (isinstance(md, dict) and set(md) <= ROW_METADATA and all(isinstance(v, str) for v in md.values())))):
                out.append("a result row")
                break
        if not is_int(g("results_count")) or g("results_count") != len(rows):
            out.append("results_count")
    if g("publication") != "ordinary":
        out.append("publication")
    if not (g("numerical_quality") is None or isinstance(g("numerical_quality"), dict)):
        out.append("numerical_quality")
    if out:
        return out
    return out + outcome_problems(r)


def outcome_problems(r):
    """The outcome, summary and evidence of a structurally valid successful record (RAW_SCHEMA)."""
    out = []
    ds = r["diagnostics"]
    standings = [d for d in ds if d["code"].startswith("NUMERICAL_INTEGRITY")]
    blocking = [d for d in ds if d["severity"] == "blocking"]
    unknown = sorted({d["code"] for d in standings} - STANDINGS)
    if unknown:
        out.append(f"unrecognised standing {unknown}")
    if blocking:
        codes = sorted({d["code"] for d in blocking})
        if len(codes) != 1 or codes[0] not in REFUSALS:
            out.append(f"refusal codes {codes}: exactly one recognised refusal code required")
        if standings:
            out.append("a refusal with a standing")
        if r["results"]:
            out.append("a refusal that publishes rows")
    else:
        if not standings:
            out.append("no outcome: neither a standing nor a refusal")
        if any(d["severity"] == "blocking" for d in standings):
            out.append("a blocking standing")
        if not r["results"]:
            out.append("a standing with no rows")
    sm = r["summary"]
    if not (sm is None or isinstance(sm, dict)) or (isinstance(sm, dict) and not set(sm) <= SUMMARY_KEYS):
        out.append("summary")
    elif not blocking:
        h = sm.get("max_open_formula_stress", "absent") if isinstance(sm, dict) else "absent"
        if h == "absent" or not (h is None or (isinstance(h, dict) and set(h) <= HEADLINE_KEYS and "value" in h
                                                and (h["value"] is None or is_num(h["value"])))):
            out.append("summary.max_open_formula_stress")
    ev = r["contract_evidence"]
    if blocking:
        if ev is not None and not isinstance(ev, dict):
            out.append("contract_evidence")
    elif (not isinstance(ev, dict) or set(ev) != EVIDENCE_KEYS or ev["combination_gates"] != [] or not isinstance(ev["preview_cases"], list)
          or not all(isinstance(c, dict) and {"load_case_id", "pipe_stress_extrema"} <= set(c) <= CASE_EVIDENCE_KEYS
                     and isinstance(c["load_case_id"], str) and isinstance(c["pipe_stress_extrema"], list)
                     and all(isinstance(e, dict) and {"pipe_id", "value_lower_pa", "value_upper_pa"} <= set(e) <= EXTREMA_KEYS
                             and isinstance(e["pipe_id"], str)
                             and all(e[k] is None or is_num(e[k]) for k in ("value_lower_pa", "value_upper_pa"))
                             for e in c["pipe_stress_extrema"])
                     for c in ev["preview_cases"])):
        out.append("contract_evidence")
    return out


def binding_problems(r, gen):
    """A successful record against its model: one standing per load case naming it, rows of the model's cases and
    entities, evidence of its cases and pipes."""
    model = gen["request"]["model"]
    cases = [c["id"] for c in model["load_cases"]]
    entities = {x["id"] for k in ("nodes", "pipe_segments", "supports") for x in model[k]}
    pipes = {x["id"] for x in model["pipe_segments"]}
    out = []
    if not any(d["severity"] == "blocking" for d in r["diagnostics"]):
        standings = [d for d in r["diagnostics"] if d["code"].startswith("NUMERICAL_INTEGRITY")]
        for cid in cases:
            if sum(1 for d in standings if d.get("affected_refs") == [cid]) != 1:
                out.append(f"not exactly one standing for {cid}")
        if len(standings) != len(cases):
            out.append(f"{len(standings)} standings for {len(cases)} load cases")
    # the solver-mode row names the solver, not a model entity (observed on every genuine probe record)
    solver = lambda x: x["kind"] == "linear_solver_mode_basis" and x["entity_ref"] == SOLVER_ENTITY
    bad = [(x["case"], x["entity_ref"]) for x in r["results"] if x["case"] not in cases or not (x["entity_ref"] in entities or solver(x))]
    if bad:
        out.append(f"{len(bad)} rows of another model's case or entity, first {bad[0]}")
    if isinstance(r["contract_evidence"], dict) and not any(d["severity"] == "blocking" for d in r["diagnostics"]):
        for c in r["contract_evidence"]["preview_cases"]:
            if c["load_case_id"] not in cases or any(e["pipe_id"] not in pipes for e in c["pipe_stress_extrema"]):
                out.append("evidence of another model's case or pipe")
                break
    return out


REF_LINE = {"tag", "meta", "ref", "request"}
REF_META = {"design", "family", "section", "material", "placement", "theory", "section_variant", "twin", "E", "G", "harness_validation",
            "notes", "self_weight_N_per_m"}
REF_REF = {"theory", "sections", "nodes", "cases", "expect", "expected_warnings", "phi", "phi_minus_2"}


REF_SECTION = {"od", "wall", "mill", "E", "G", "ro", "ri", "A", "I", "J", "Z", "L", "nu", "E_over_2G", "kappa_E", "shear_area", "tau_c",
               "phi_timoshenko", "phi_used", "exact_nu_in_domain", "admitted", "refusal_reasons"}


def _decimal_text(x):
    try:
        return isinstance(x, str) and Decimal(x).is_finite()
    except Exception:
        return False


def _opt_str(x):
    return x is None or isinstance(x, str)


def reference_problems(lines, twins=False):
    """The reference set validated exactly, every field the judge reads typed: line, meta and ref keys; sections; node
    coordinates; case rows and maxima; the request's nodes, pipes, supports and load cases; unique tags; and, when
    judging against the twins, every twin present."""
    out = []
    tags = collections.Counter(x.get("tag") if isinstance(x, dict) else None for x in lines)
    out += [f"duplicate reference tag {t}" for t, n in tags.items() if n > 1]
    if not lines:
        out.append("an empty reference set")
    for x in lines:
        t = x.get("tag") if isinstance(x, dict) else None
        try:
            out += [f"{t}: {p}" for p in _reference_line_problems(x, tags, twins)]
        except Exception as e:  # a shape the checks below did not anticipate is itself a problem, never a crash
            out.append(f"{t}: unreadable ({type(e).__name__})")
    return out


def _reference_line_problems(x, tags, twins):
    if not isinstance(x, dict) or set(x) != REF_LINE or not isinstance(x["tag"], str):
        return ["line keys"]
    out = []
    m, r, q = x["meta"], x["ref"], x["request"]
    if not isinstance(m, dict) or set(m) != REF_META or not isinstance(m["twin"], str) or (twins and m["twin"] not in tags):
        out.append("meta")
    if not isinstance(r, dict) or set(r) != REF_REF | ({"refusal"} if r.get("expect") == "refused" else set()) or r["expect"] not in ("rows", "refused"):
        return out + ["ref keys"]
    if r["theory"] not in (EULER, TIMOSHENKO) or not (isinstance(r["expected_warnings"], list) and all(isinstance(w, str) for w in r["expected_warnings"])):
        out.append("ref theory or warnings")
    if r["expect"] == "refused" and not (isinstance(r["refusal"], dict) and r["refusal"].get("code") in REFUSALS):
        out.append("ref refusal")
    if not (isinstance(r["sections"], dict) and r["sections"] and all(isinstance(s, dict) and set(s) == REF_SECTION for s in r["sections"].values())
            and all(_decimal_text(s[k]) for s in r["sections"].values() for k in ("ro", "A", "I", "J", "Z", "tau_c"))
            and all(isinstance(s[k], str) and _decimal_text(repr(float(s[k]))) for s in r["sections"].values() for k in ("od", "wall", "mill", "E", "G"))):
        out.append("ref sections")
    if not (isinstance(r["nodes"], list) and r["nodes"] and all(isinstance(n, list) and len(n) == 2 and isinstance(n[0], str) and isinstance(n[1], list)
                                                           and len(n[1]) == 3 and all(_decimal_text(c) for c in n[1]) for n in r["nodes"])):
        out.append("ref nodes")
    if not (isinstance(r["phi"], dict) and isinstance(r["phi_minus_2"], dict)):
        out.append("ref phi")
    if not (isinstance(q, dict) and set(q) == {"model", "materials"} and isinstance(q["model"], dict)):
        return out + ["request"]
    mdl = q["model"]
    lists = all(isinstance(mdl.get(k), list) and all(isinstance(e, dict) and isinstance(e.get("id"), str) for e in mdl[k])
                for k in ("nodes", "pipe_segments", "supports", "load_cases"))
    if not lists or not isinstance(mdl.get("components"), list) or not all(isinstance(p.get("from"), str) and isinstance(p.get("to"), str)
                                                                          for p in mdl["pipe_segments"]):
        return out + ["request model"]
    cases = [c["id"] for c in mdl["load_cases"]]
    pipes = set(p["id"] for p in mdl["pipe_segments"])
    if not isinstance(r["cases"], dict) or sorted(r["cases"]) != sorted(cases):
        return out + ["ref cases differ from the request's load cases"]
    for cid, c in r["cases"].items():
        if not (isinstance(c, dict) and set(c) == {"rows", "maxima"} and isinstance(c["rows"], list) and isinstance(c["maxima"], list)
                and all(isinstance(w, list) and len(w) == 6 and isinstance(w[0], str) and isinstance(w[1], str) and _opt_str(w[2]) and _opt_str(w[3])
                        and w[4] is None and _decimal_text(w[5]) for w in c["rows"])
                and all(isinstance(w, list) and len(w) == 3 and w[0] in pipes and w[1] is None and _decimal_text(w[2]) for w in c["maxima"])):
            out.append(f"case {cid}")
    return out


def request_line(tag, request):
    """The runner-input line of a model, exactly as pft_generate.py split writes it and the runner hashes it."""
    return json.dumps({"tag": tag, "request": request}, sort_keys=True, separators=(",", ":"))


def request_lines(gens):
    return "".join(request_line(g["tag"], g["request"]) + "\n" for g in gens)


def request_sha256(tag, request):
    return hashlib.sha256(request_line(tag, request).encode()).hexdigest()


def file_sha256(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def inventory(expected_tags, identities, unsuccessful=()):
    """The collection check: every expected model x entry x mode exactly once, successful and schema-valid, and
    nothing else. `identities` are the (tag, entry, mode) of the schema-valid, successful records; `unsuccessful`
    those of runner errors and schema-invalid records (identifiable or not), which fail the inventory."""
    want = {(t, e, m) for t in expected_tags for e in ENTRIES for m in MODES}
    got = collections.Counter(identities)
    out = {"expected": len(want), "received": sum(got.values()) + len(unsuccessful),
           "missing": sorted(want - set(got)), "duplicate": sorted(k for k, n in got.items() if n > 1),
           "unexpected": sorted(k for k in got if k not in want), "unsuccessful": sorted(unsuccessful, key=repr)}
    out["pass"] = not (out["missing"] or out["duplicate"] or out["unexpected"] or out["unsuccessful"])
    return out


def inventory_text(inv):
    return (f"INVENTORY (model x entry x mode): {'PASS' if inv['pass'] else 'FAIL'} ({inv['received']} records against {inv['expected']} expected; "
            f"{len(inv['missing'])} missing, {len(inv['duplicate'])} duplicate, {len(inv['unexpected'])} unexpected, "
            f"{len(inv['unsuccessful'])} unsuccessful or schema-invalid)")


# ----------------------------------------------------------------- self-test
SYNTH_RUNNER = "5" * 64          # a synthetic runner's sha256 (no product runs in the self-test)
SYNTH_HEAD = "0" * 40            # a placeholder head for runs judged in memory; collection tests use the checkout's HEAD


def synthetic_run(gen, upper_scale=1.0 + 1e-12, entry="typed", mode="SparseInteractive", runner=SYNTH_RUNNER, head=SYNTH_HEAD):
    """A raw runner record (RAW_SCHEMA) that publishes the reference rows themselves (rounded to binary64), with the
    derived sigma+- and tau_v,max formed by the design's binary64 operations from those rows, Passed, bound to the
    reference request."""
    ref = gen["ref"]
    rows, cases_ev = [], []
    for cid, c in ref["cases"].items():
        for kind, ent, loc, comp, _, val in c["rows"]:
            rows.append({"kind": kind, "value": float(Decimal(val)), "unit": "synthetic", "entity_ref": ent, "case": cid, "location": loc,
                         "component": comp, "id": f"result:{len(rows)}", "metadata": {"basis": "recovered_from_open_mechanics_stress_components"}})
        ev = []
        for pid, _, val in c["maxima"]:
            rows.append({"kind": "pipe_elastic_normal_stress_maximum_v2", "value": float(Decimal(val)), "unit": "Pa", "entity_ref": pid, "case": cid,
                         "location": "governing_station", "component": "maximum_absolute_normal_stress", "id": f"result:{len(rows)}",
                         "metadata": {"basis": "x"}})
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
    diags = [diag("NUMERICAL_INTEGRITY_CHECKS_PASSED", "info")] + [diag(w, "warning") for w in ref.get("expected_warnings", [])]
    if ref.get("expect") == "refused":
        diags = [diag(REFUSAL_CODE, "blocking")]
        rows, cases_ev, mx = [], [], []
    return {"tag": gen["tag"], "entry": entry, "mode": mode, "ns": 1, "diagnostics": diags, "results": rows, "results_count": len(rows),
            "numerical_quality": {}, "summary": {"max_open_formula_stress": {"value": max(mx), "unit": "Pa"}} if mx else
            ({} if ref.get("expect") == "refused" else {"max_open_formula_stress": None}),
            "contract_evidence": {"combination_gates": [], "preview_cases": cases_ev} if not ref.get("expect") == "refused" else None,
            "publication": "ordinary",
            "request_sha256": request_sha256(gen["tag"], gen["request"]), "runner_sha256": runner, "product_head": head}


def diag(code, severity, message="synthetic", case="case:1"):
    return {"affected_refs": [case], "code": code, "id": "diagnostic:" + code.lower(), "message": message, "severity": severity, "source": "synthetic"}


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
    collection_scenarios(set_path, gens, pick, check)
    bad = [n for n, ok in out if not ok]
    print(f"compare self-test: {len(out) - len(bad)} of {len(out)} checks pass")
    if bad:
        raise SystemExit(1)


def collection_scenarios(set_path, gens, pick, check):
    """Every refutation round's attacks (1-4) as committed regression cases of the judging pass (pft_report.py) and
    run.sh, on synthetic records only: run.sh is driven with a synthetic builder whose runners are Python scripts, so no
    product can run. Each case prints a TABLE line: round | attack | case | outcome."""
    import shutil
    import subprocess
    import tempfile
    import pft_report as PR
    checkout = subprocess.run(["git", "-C", HERE, "rev-parse", "--show-toplevel"], capture_output=True, text=True).stdout.strip()
    head = subprocess.run(["git", "-C", HERE, "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip()
    check(f"the self-test's checkout {checkout} resolves HEAD to a commit", bool(re.fullmatch(r"[0-9a-f]{40}", head)))
    py = [sys.executable, "-I", "-B"]
    R = SYNTH_RUNNER
    codex, ordinary = pick("NCcodexS8"), pick("CANT03")
    by_tag = {x["tag"]: x for x in gens}
    run = lambda g, e, m, **kw: synthetic_run(g, entry=e, mode=m, head=head, **kw)
    four = [run(codex, e, m) for e in ENTRIES for m in MODES]
    eight = four + [run(ordinary, e, m) for e in ENTRIES for m in MODES]
    bad = copy.deepcopy(four[0])
    for r in bad["results"]:
        if r["kind"] == "global_nodal_displacement_y" and r["entity_ref"] == "node:N1":
            r["value"] *= 0.444
    complete = [run(x, e, m) for x in gens for e in ENTRIES for m in MODES]
    check(f"the full set's inventory is 3,624 runs, the frozen subset's {inventory([x['tag'] for x in gens], [])['expected']}",
          inventory([t for t, _, _ in __import__("pft_generate").model_list()], [])["expected"] == 3624 and len(complete) == 400)
    rows = []

    def table(rnd, attack, case, outcome, ok):
        rows.append((rnd, attack, case, outcome))
        print(f"TABLE {rnd} | {attack} | {case} | {outcome}")
        check(f"round {rnd} '{case}' ({attack}): {outcome}", ok)

    with tempfile.TemporaryDirectory() as tmp:
        T = lambda name: os.path.join(tmp, name)

        def write(path, xs, raw=False):
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "w") as f:
                f.write(xs if raw else "".join(json.dumps(x) + "\n" for x in xs))
            return path

        small = write(T("small-set.jsonl"), [codex, ordinary])
        one = write(T("one-set.jsonl"), [codex])

        def judge(rnd, attack, case, st, files, want, why=lambda j: True, label="", **kw):
            args = dict(quiet=True, workers=kw.pop("workers", 1), runner_sha=kw.pop("runner_sha", R), head=kw.pop("head", head),
                        checkout=kw.pop("checkout", checkout), **kw)
            j = PR.report(st, files, **args)
            outcome = ("OVERALL: PASS" if j["pass"] else "OVERALL: FAIL") + (f" ({label})" if label else "")
            table(rnd, attack, case, outcome, j["pass"] == want and why(j))
            return j

        def judge_runs(rnd, attack, case, st, runs, want, why=lambda j: True, label="", **kw):
            return judge(rnd, attack, case, st, [write(T(case + ".raw.jsonl"), runs)], want, why, label, **kw)

        def mut(runs, f):
            xs = copy.deepcopy(runs)
            for r in xs:
                f(r)
            return xs

        schema = lambda j: j.get("schema_invalid", 0) > 0 and not j["inventory"]["pass"]
        prov = lambda j: j.get("provenance_pass") is False
        inv = lambda j: j["inventory"]["pass"] is False
        m3 = lambda j: j["provenance_pass"] and j["inventory"]["pass"] and not j["m3_pass"]
        sens = lambda r: r.update(diagnostics=[diag("NUMERICAL_INTEGRITY_SENSITIVE", "warning")])
        err_only = lambda r: [r.pop(k) for k in ("diagnostics", "results", "results_count", "numerical_quality", "summary", "contract_evidence",
                                                  "publication")] and r.update(error="synthetic runner failure")
        # ---- positive controls
        judge(0, "positive control", "small_positive", small, [write(T("small_positive.raw.jsonl"), eight)], True)
        judge(0, "positive control", "frozen_positive_400", set_path, [write(T("frozen.raw.jsonl"), complete)], True,
              lambda j: j["inventory"]["expected"] == 400, workers=4)
        judge(0, "positive control", "chunks_complete_positive (two raw files)", small, [write(T("a.raw.jsonl"), four), write(T("b.raw.jsonl"), eight[4:])], True)
        judge(4, "key order is not meaning", "key_order_positive", small,
              [write(T("ko.raw.jsonl"), "".join(json.dumps(dict(reversed(list(r.items())))) + "\n" for r in eight), raw=True)], True)
        judge(4, "an escaped key is the same key", "escaped_key_positive", small,
              [write(T("esc.raw.jsonl"), open(T("small_positive.raw.jsonl")).read().replace('"tag":', '"\\u0074ag":'), raw=True)], True)
        judge(4, "a twin control judged against the twins", "frozen_twin_control", set_path, [T("frozen.raw.jsonl")], False, m3,
              "M3 fails, provenance and inventory pass: the control behaves", variant="twin", workers=4)
        # ---- round 1
        for case, runs in (("duplicate_bad_then_good", [bad] + eight), ("duplicate_good_then_bad", eight + [bad]), ("single", [four[0]]),
                           ("empty", []), ("unexpected_tag", eight + [dict(four[0], tag="invented-unexpected-tag")]), ("unpaired", [four[0], four[3]])):
            judge_runs(1, case.replace("_", " "), case, small, runs, False, inv, "inventory")
        judge_runs(1, "r1's 55.6%-low UY on a Passed run", "bad_uy_55_6_percent_low", small, [bad] + eight[1:], False, m3, "M3: C3")
        # ---- round 2 (sidecars and caches no longer exist; their raw-output equivalents)
        split = [four[:2], mut(four[2:], sens)]
        s0, s1 = write(T("s0.raw.jsonl"), split[0]), write(T("s1.raw.jsonl"), split[1])
        judge(2, "split C4 (typed Passed, captured Sensitive) across comparisons", "r2_c4_across_raw_files", one, [s0, s1], False,
              lambda j: j["c4_failures"] == 2, "C4 = 2 from the raw records")
        side = T("s0.raw.jsonl.inventory.json")
        write(side, json.dumps({"pass": True, "expected": 4, "received": 4}), raw=True)
        judge(2, "split C4 with a stale passing sidecar beside the raw files", "r2_stale_sidecar_not_read", one, [s0, s1], False,
              lambda j: j["c4_failures"] == 2, "C4 = 2; sidecars are not read")
        judge(2, "a sidecar (mismatched) given as a raw input", "r2_sidecar_as_input", one, [s0, s1, side], False, schema, "schema")
        judge(2, "split C4 with missing sidecars", "r2_split_c4_no_sidecars", one, [s0, write(T("s1b.raw.jsonl"), split[1])], False,
              lambda j: j["c4_failures"] == 2, "C4 = 2")
        judge_runs(2, "split C4 on the full frozen subset", "r2_full_frozen_c4", set_path,
                   [r for r in complete if r["tag"] != codex["tag"]] + split[0] + split[1], False, lambda j: j["c4_failures"] == 2, "C4 = 2", workers=4)
        evs = []
        PR.report(small, [T("small_positive.raw.jsonl")], quiet=True, workers=1, runner_sha=R, head=head, checkout=checkout, eval_out=T("small.eval.jsonl"))
        evs = [strict_loads(line) for line in open(T("small.eval.jsonl"))]
        pre_repair = mut(evs, lambda r: (r.pop("design_failures", None), r.update(info=dict(r.get("info") or {}, C10=1))))
        judge_runs(2, "a pre-repair evaluation cache given as input", "r2_pre_repair_cache_as_input", small, pre_repair, False, schema, "schema")
        # ---- round 3
        rnull = lambda r: r.update(error=None, results=[], results_count=0)
        judge_runs(3, "null error, no rows", "r3_error_null_only", small, mut(eight[:1], rnull) + eight[1:], False, schema, "schema")
        judge_runs(3, "null error on the 55.6%-low Passed record", "r3_error_null_bad_rows", small, [dict(bad, error=None)] + eight[1:], False, schema, "schema")
        judge_runs(3, "an error string beside results", "r3_error_string_plus_results", small, [dict(four[0], error="x")] + eight[1:], False, schema, "schema")
        judge_runs(3, "a runner error", "r3_error_string_only", small, mut(eight[:1], err_only) + eight[1:], False,
                   lambda j: j["runner_errors"] == 1 and inv(j), "C0, inventory")
        judge_runs(3, "a run with no outcome", "r3_no_outcome", small, mut(eight, lambda r: r.update(results=[], results_count=0, diagnostics=[])), False,
                   schema, "schema: no outcome")
        judge_runs(3, "400 runner errors", "r3_frozen_400_errors", set_path, mut(complete, err_only), False,
                   lambda j: j["runner_errors"] == 400 and len(j["inventory"]["unsuccessful"]) == 400 and j["m3_failing_records"] == 400, "C0 x400", workers=4)
        judge_runs(3, "a forged pairing", "r3_unknown_pairing", small, mut(eight[:1], lambda r: r.update(pairing={"integrity": ["x"]})) + eight[1:],
                   False, schema, "schema")
        for case, f in (("r3_replay_pairing_lie_rehashed", lambda r: r.update(pairing={"integrity": r.get("integrity"), "row_digest": "0" * 64})),
                        ("r3_replay_missing_verdict_failures", lambda r: r.pop("verdict_failures", None)),
                        ("r3_replay_null_design_failures", lambda r: r.update(design_failures=None)),
                        ("r3_replay_legacy_c4_currentstamp", lambda r: r.update(stamp={"format": "pft-m3t-judging/2"}))):
            judge_runs(3, "an evaluation record given as input", case, small, mut(evs, f), False, schema, "schema")
        for case, f in (("r3_missing_clause_list", lambda r: r.pop("results")), ("r3_null_clause_list", lambda r: r.update(results=None))):
            judge_runs(3, "a missing or null clause list", case, small, mut(eight[:1], f) + eight[1:], False, schema, "schema")
        changed = copy.deepcopy(codex)
        changed["request"]["model"]["nodes"][1]["position"]["x"] = 2.0
        judge_runs(3, "a changed request (L = 2 m) under the same tag", "r3_changed_L_hash", small,
                   [dict(r, request_sha256=request_sha256(changed["tag"], changed["request"])) for r in four] + eight[4:], False, prov, "provenance")
        judge(3, "mixed runner", "r3_mixed_runner", small, [T("a.raw.jsonl"), write(T("b6.raw.jsonl"), [dict(r, runner_sha256="6" * 64) for r in eight[4:]])],
              False, prov, "provenance")
        judge(3, "mixed head", "r3_mixed_head", small, [T("a.raw.jsonl"), write(T("b7.raw.jsonl"), [dict(r, product_head="1" * 40) for r in eight[4:]])],
              False, prov, "provenance")
        judge(3, "--runner-sha256 differs", "r3_expect_runner_mismatch", small, [T("small_positive.raw.jsonl")], False, prov, "provenance", runner_sha="7" * 64)
        judge(3, "--product-head differs", "r3_expect_head_mismatch", small, [T("small_positive.raw.jsonl")], False, prov, "provenance", head="7" * 40)
        for case, h in (("r3_dirty", head + "-dirty"), ("r3_unknown", "unknown")):
            judge_runs(3, "a head that is not a commit", case, small, [dict(r, product_head=h) for r in eight], False, schema, "schema")
        # ---- round 4
        for case, h in (("arbitrary_noncommit_head", "this-is-not-a-commit"), ("unicode_head_suffix", head + "-dirty\u200b")):
            judge_runs(4, "a head that is not 40 hex", case, small, [dict(r, product_head=h) for r in eight], False, lambda j: schema(j) and prov(j),
                       "schema and provenance", head=h)
        judge_runs(4, "a 40-hex head that is not a commit", "nonexistent_hex_head", small, [dict(r, product_head="0" * 40) for r in eight], False, prov,
                   "provenance: not a commit", head="0" * 40)
        judge(4, "a head without a checkout", "head_without_checkout", small, [T("small_positive.raw.jsonl")], False, prov, "provenance",
              checkout=T("no-such-dir"))
        judge(4, "no --runner-sha256", "missing_runner_sha", small, [T("small_positive.raw.jsonl")], False, prov, "provenance", runner_sha=None)
        judge_runs(4, "identical forged runner sha256 at the real head", "forged_identical_provenance", small, [dict(r, runner_sha256="f" * 64) for r in eight],
                   True, lambda j: True, "outside the model: assertions", runner_sha="f" * 64)
        for case, f in (("unknown_integrity_code_bad_rows", lambda r: r.update(diagnostics=[diag("NUMERICAL_INTEGRITY_NOT_A_STANDING", "info")])),
                        ("sensitive_before_passed_bad_rows", lambda r: r["diagnostics"].insert(0, diag("NUMERICAL_INTEGRITY_SENSITIVE", "warning"))),
                        ("passed_before_sensitive_bad_rows", lambda r: r["diagnostics"].append(diag("NUMERICAL_INTEGRITY_SENSITIVE", "warning"))),
                        ("standing_naming_another_case", lambda r: r.update(diagnostics=[diag("NUMERICAL_INTEGRITY_CHECKS_PASSED", "info", case="case:2")])),
                        ("refusal_plus_standing", lambda r: r["diagnostics"].append(diag(REFUSAL_CODE, "blocking"))),
                        ("unrecognised_refusal_code", lambda r: r.update(diagnostics=[diag("SOMETHING_BLOCKING", "blocking")], results=[], results_count=0)),
                        ("two_refusal_codes", lambda r: r.update(diagnostics=[diag(REFUSAL_CODE, "blocking"), diag("PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED", "blocking")],
                                                                results=[], results_count=0))):
            judge_runs(4, "contradictory or unknown standing or refusal", case, small, mut([bad] + eight[1:], f), False, schema, "schema")
        judge_runs(4, "every run Sensitive, bad rows", "bad_rows_sensitive_only", small, mut([bad] + eight[1:], sens), True, lambda j: True,
                   "by the criterion: Sensitive cases are not judged")
        judge_runs(4, "a standing with no rows", "sensitive_no_results", small, mut(eight, lambda r: (sens(r), r.update(results=[], results_count=0))), False,
                   schema, "schema")

        def swap(xs):
            for i in range(4):
                for k in RAW_SUCCESS - RAW_COMMON:
                    xs[i][k], xs[i + 4][k] = copy.deepcopy(xs[i + 4][k]), copy.deepcopy(xs[i][k])
            return xs
        judge_runs(4, "another model's content under a matching hash", "different_model_content_matching_hash", small, swap(copy.deepcopy(eight)), False,
                   schema, "schema: misbound rows")
        judge_runs(4, "another model's content, Sensitive", "different_model_content_sensitive", small, swap(mut(eight, sens)), False, schema,
                   "schema: misbound rows")
        twin_content = []
        for r in eight:
            t = run(by_tag[by_tag[r["tag"]]["meta"]["twin"]], r["entry"], r["mode"])
            twin_content.append(dict(r, **{k: t[k] for k in RAW_SUCCESS - RAW_COMMON}))
        judge_runs(4, "the twin's rows under the own hash", "mixed_theory_content_own_hash", small, twin_content, False, m3, "M3")
        for case, f in (("unicode_confusable_key", lambda r: r.update({"t\u0430g": r.pop("tag")})), ("bool_ns", lambda r: r.update(ns=True)),
                        ("bool_row_value", lambda r: r["results"][0].update(value=True)), ("raw_with_ref_variant", lambda r: r.update(ref_variant="twin")),
                        ("nested_summary_arbitrary", lambda r: r.update(summary={"arbitrary": True})),
                        ("nested_headline_wrong_type", lambda r: r["summary"].update(max_open_formula_stress="high")),
                        ("nested_evidence_wrong_type", lambda r: r.update(contract_evidence={"combination_gates": [], "preview_cases": "not-an-array"})),
                        ("nested_extrema_wrong_type", lambda r: r["contract_evidence"]["preview_cases"][0]["pipe_stress_extrema"][0].update(value_upper_pa="1")),
                        ("evidence_of_another_pipe", lambda r: r["contract_evidence"]["preview_cases"][0]["pipe_stress_extrema"][0].update(pipe_id="pipe:ZZ"))):
            judge_runs(4, "a malformed nested field", case, small, mut(eight[:1], f) + eight[1:], False, schema, "schema")
        solver_row = lambda ent: (lambda r: (r["results"].append({"kind": "linear_solver_mode_basis", "value": 1.0, "unit": "mode_code", "entity_ref": ent,
                                                                 "case": "case:1", "location": "case:1", "component": "linear_solver_mode", "id": "result:solver",
                                                                 "metadata": None}), r.update(results_count=len(r["results"]))))
        judge_runs(0, "positive control: the product's solver-mode row", "solver_row_genuine", small, mut(eight, solver_row(SOLVER_ENTITY)), True)
        judge_runs(4, "a solver-mode row naming another entity", "solver_row_other_entity", small, mut(eight[:1], solver_row("solver:other")) + eight[1:], False,
                   schema, "schema: misbound row")
        nan = write(T("nan.raw.jsonl"), json.dumps(dict(four[0], results=[dict(four[0]["results"][0], value=float("nan"))] + four[0]["results"][1:])) + "\n"
                    + "".join(json.dumps(r) + "\n" for r in eight[1:]), raw=True)
        judge(4, "a NaN token", "nan_row_value", small, [nan], False, prov, "strict JSON: refused")
        dd = write(T("dd.raw.jsonl"), "".join('{"diagnostics":' + json.dumps([diag("NUMERICAL_INTEGRITY_CHECKS_PASSED", "info")]) + ","
                                              + json.dumps(dict(r, diagnostics=[diag("NUMERICAL_INTEGRITY_SENSITIVE", "warning")]))[1:] + "\n"
                                              for r in [bad] + eight[1:]), raw=True)
        judge(4, "a duplicate diagnostics key", "duplicate_diagnostics_hides_passed", small, [dd], False, prov, "strict JSON: refused")
        dr = write(T("dr.raw.jsonl"), "".join('{"results":' + json.dumps((bad if i == 0 else r)["results"]) + "," + json.dumps(r)[1:] + "\n"
                                              for i, r in enumerate(eight)), raw=True)
        judge(4, "a duplicate results key", "duplicate_results_hides_bad_value", small, [dr], False, prov, "strict JSON: refused")
        bad_ref = copy.deepcopy(codex)
        bad_ref["ref"]["cases"]["case:1"]["rows"] = None
        judge(4, "reference rows of the wrong type", "reference_rows_wrong_type", write(T("bad-ref.jsonl"), [bad_ref, ordinary]), [T("small_positive.raw.jsonl")],
              False, prov, "provenance: reference; no traceback")
        dup_ref = write(T("dup-ref.jsonl"), json.dumps(codex)[:-1] + ', "tag": "x"}\n' + json.dumps(ordinary) + "\n", raw=True)
        judge(4, "a duplicate key in the reference", "reference_duplicate_key", dup_ref, [T("small_positive.raw.jsonl")], False, prov, "strict JSON: refused")
        judge_runs(4, "half the runs", "partial_out", small, eight[:4], False, inv, "inventory")
        judge(4, "an empty raw output", "empty_out", small, [write(T("empty.raw.jsonl"), "", raw=True)], False, lambda j: inv(j) and prov(j))
        # outputs never alias inputs
        for case, target in (("eval_out_alias_raw", "raw"), ("eval_out_alias_reference", "ref"), ("eval_out_alias_symlink_raw", "symlink"),
                             ("eval_out_hardlink_input", "hardlink"), ("json_out_alias_input", "json"), ("eval_out_existing_file", "existing")):
            raw_p, ref_p = write(T(case + ".raw.jsonl"), eight), write(T(case + ".set.jsonl"), [codex, ordinary])
            alias = {"raw": raw_p, "ref": ref_p, "json": raw_p, "existing": write(T(case + ".other"), "keep", raw=True)}.get(target)
            if target == "symlink":
                alias = T(case + ".link")
                os.symlink(raw_p, alias)
            if target == "hardlink":
                alias = T(case + ".hard")
                os.link(raw_p, alias)
            before = {p: open(p, "rb").read() for p in (raw_p, ref_p)}
            kw = {"json_out": alias} if target == "json" else {"eval_out": alias}
            j = PR.report(ref_p, [raw_p], quiet=True, workers=1, runner_sha=R, head=head, checkout=checkout, **kw)
            kept = all(open(p, "rb").read() == b for p, b in before.items()) and (target != "existing" or open(alias).read() == "keep")
            table(4, "an output aliasing an input or an existing file", case, "OVERALL: FAIL (outputs refused, inputs intact)",
                  not j["pass"] and bool(j.get("outputs_refused")) and kept)
        # ---- run.sh: a synthetic builder whose runner is a Python script
        def runner(name, runs, exit_code=0):
            data = T(name + ".runs.json")
            json.dump(runs, open(data, "w"))
            src = T(name + ".runner")
            open(src, "w").write(
                f"#!{sys.executable} -I\n"
                "import hashlib, json, os, subprocess, sys\n"
                "me = hashlib.sha256(open(sys.argv[0], 'rb').read()).hexdigest()\n"
                f"head = subprocess.run(['git', '-C', {checkout!r}, 'rev-parse', 'HEAD'], capture_output=True, text=True).stdout.strip()\n"
                f"runs = json.load(open({data!r}))\n"
                "out = open(os.environ['KD5_OUT'], 'w')\n"
                "for line in open(os.environ['KD5_IN'], 'rb').read().split(b'\\n'):\n"
                "    if not line: continue\n"
                "    tag = json.loads(line)['tag']\n"
                "    for r in runs:\n"
                "        if r['tag'] == tag:\n"
                "            out.write(json.dumps(dict(r, request_sha256=hashlib.sha256(line).hexdigest(), runner_sha256=me, product_head=head)) + '\\n')\n"
                f"sys.exit({exit_code})\n")
            os.chmod(src, 0o755)
            return src

        builder = T("builder.sh")
        open(builder, "w").write('#!/bin/bash\nset -eu\ncp "$PFM3T_FAKE_RUNNER" "$3"\nchmod +x "$3"\n')
        os.chmod(builder, 0o755)
        good, badr, exit7 = runner("good", eight), runner("bad", [bad] + eight[1:]), runner("exit7", eight, 7)
        twins = [by_tag[codex["meta"]["twin"]], by_tag[ordinary["meta"]["twin"]]]
        good4 = runner("good4", eight + [run(t, e, m) for t in twins for e in ENTRIES for m in MODES])
        sd, sd4 = T("driver-set"), T("driver-set4")
        write(os.path.join(sd, "set.jsonl"), [codex, ordinary])
        write(os.path.join(sd4, "set.jsonl"), [codex, ordinary] + twins)

        def driver(rnd, attack, case, runner_bin, rundir, chunks, variants, want, setdir=sd, why=lambda d: True):
            for i, tags in enumerate(chunks):
                write(os.path.join(rundir, "chunks", f"c.{i:03d}.in.jsonl"), request_lines(tags), raw=True)
            env = dict(os.environ, PFM3T_PY=sys.executable, PFM3T_WORKERS="1", PFM3T_BUILD=builder, PFM3T_FAKE_RUNNER=runner_bin)
            d = subprocess.run(["bash", os.path.join(HERE, "run.sh"), checkout, setdir, rundir, *variants], capture_output=True, text=True, env=env)
            last = d.stdout.strip().splitlines()[-1] if d.stdout.strip() else ""
            table(rnd, attack, case, f"run.sh exit {d.returncode}, final line '{last}'",
                  (d.returncode == 0) == want and last == ("OVERALL: PASS" if want else "OVERALL: FAIL") and why(rundir))

        driver(0, "positive control", "driver_positive", good, T("d/positive"), [[codex], [ordinary]], ["own"], True)
        driver(0, "positive control", "driver_positive_with_twin_control", good4, T("d/twin-ok"), [[codex, ordinary], twins], ["own", "twin"], True, sd4)
        driver(1, "a model missing from the chunks", "driver_missing_model", good, T("d/missing"), [[codex]], ["own"], False)
        driver(1, "a chunk given twice", "driver_duplicate_across_chunks", good, T("d/dup"), [[codex], [codex], [ordinary]], ["own"], False)
        driver(3, "a changed request under the same tag", "driver_changed_request", good, T("d/changed"), [[changed, ordinary]], ["own"], False)
        driver(3, "a replaced runner", "driver_resume_good_then_bad_runner (good first)", good, T("d/resume"), [[codex], [ordinary]], ["own"], True)
        driver(3, "a replaced runner", "driver_resume_bad_new_runner", badr, T("d/resume"), [[codex], [ordinary]], ["own"], False)
        write(os.path.join(T("d/partial"), "out", "c.000.out.jsonl"), [bad])          # a partial earlier output of the same chunk
        driver(4, "a partial earlier output", "driver_recover_good_clear_partial", good, T("d/partial"), [[codex], [ordinary]], ["own"], True,
               why=lambda d: sorted(os.listdir(os.path.join(d, "out"))) == ["c.000.out.jsonl", "c.001.out.jsonl"])
        write(os.path.join(T("d/partial2"), "out", "c.007.out.jsonl"), [bad])
        driver(4, "a stale output from an earlier chunking", "driver_stale_out_removed", good, T("d/partial2"), [[codex], [ordinary]], ["own"], True,
               why=lambda d: not os.path.exists(os.path.join(d, "out", "c.007.out.jsonl")))
        driver(4, "a runner that exits 7 after valid records", "driver_valid_records_exit_nonzero", exit7, T("d/exit7"), [[codex], [ordinary]], ["own"], False)
        driver(4, "a twin control that cannot complete", "driver_twin_runtime_error_accepted", good, T("d/twin-bad"), [[codex], [ordinary]], ["own", "twin"], False)
        driver(4, "twin only, no own judgement", "driver_twin_only_no_own", good, T("d/twin-only"), [[codex], [ordinary]], ["twin"], False)
        # build.sh: a dirty build input refuses the build before cargo runs (a throwaway repository; cargo is `false`)
        repo = T("build-repo")
        rel = os.path.join("projects", "chirality-piping")
        for f in (os.path.join(rel, "core", "product_physics", "Cargo.toml"), os.path.join(rel, "fixtures", "x.json"), os.path.join(rel, "schemas", "x.json"),
                  os.path.join(rel, "validation", "references", "pf_timoshenko_m3t", "runner", "Cargo.toml"),
                  os.path.join(rel, "validation", "references", "pf_timoshenko_m3t", "runner", "Cargo.lock"),
                  os.path.join(rel, "validation", "references", "pf_timoshenko_m3t", "runner", "build.rs"), os.path.join(rel, "apps", "x.txt")):
            write(os.path.join(repo, f), "x\n", raw=True)
        g = lambda *a: subprocess.run(["git", "-C", repo, *a], capture_output=True, text=True)
        g("init", "-q")
        g("add", "-A")
        g("-c", "user.name=selftest", "-c", "user.email=selftest@invalid", "commit", "-q", "-m", "x")
        benv = dict(os.environ, PFM3T_CARGO="false")
        for case, path, want in (("build_clean (cargo then refuses: false)", None, 1), ("build_dirty_core", os.path.join(rel, "core", "new.rs"), 3),
                                 ("build_dirty_runner_lock", os.path.join(rel, "validation", "references", "pf_timoshenko_m3t", "runner", "Cargo.lock"), 3),
                                 ("build_dirty_fixtures", os.path.join(rel, "fixtures", "x.json"), 3), ("build_apps_not_an_input", os.path.join(rel, "apps", "x.txt"), 1)):
            g("checkout", "-q", "--", ".")
            g("clean", "-qfd")
            if path:
                write(os.path.join(repo, path), "changed\n", raw=True)
            b = subprocess.run(["bash", os.path.join(HERE, "runner", "build.sh"), repo, T("bt"), T("bt.bin")], capture_output=True, text=True, env=benv)
            table(4, "the runner built from a dirty input", case, f"build.sh exit {b.returncode}" + (" (refused: dirty input)" if b.returncode == 3 else ""),
                  b.returncode == want and not os.path.exists(T("bt.bin")))
        # the CLI: one final OVERALL line, exit 0 only on PASS; usage and internal errors fail by name
        for case, argv, rc in (("cli_passing", [small, T("small_positive.raw.jsonl")], 0),
                               ("cli_failing", [small, T("bad_uy_55_6_percent_low.raw.jsonl")], 1),
                               ("cli_missing_mandatory_flags", [small, T("small_positive.raw.jsonl"), "--no-provenance"], 2)):
            flags = ["--runner-sha256", R, "--product-head", head, "--checkout", checkout] if case != "cli_missing_mandatory_flags" else []
            c = subprocess.run(py + [os.path.join(HERE, "pft_report.py"), *argv, "--workers", "1", *flags], capture_output=True, text=True)
            last = c.stdout.strip().splitlines()[-1] if c.stdout.strip() else ""
            table(4 if rc == 2 else 0, "the CLI's final line and exit", case, f"exit {c.returncode}, final line '{last}'",
                  c.returncode == rc and last.startswith("OVERALL: PASS" if rc == 0 else "OVERALL: FAIL") and "Traceback" not in c.stderr)
    check(f"{len(rows)} regression cases across rounds 1-4 and the positive controls", len(rows) >= 80)


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
    elif args and args[0] == "--probe-format":
        probe_format(args[1], args[2])
    else:
        raise SystemExit(__doc__)


if __name__ == "__main__":
    main()
