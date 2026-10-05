#!/usr/bin/env python3
"""I61 U5: the milestone successor's published rows against I50's named oracle.

A records/test lane. It imports no product code. It reuses, by exact text, the
pinned I50 oracle (R/I50/first_publishing_component_02/named_oracle.py):
  - its module-level exact-arithmetic helpers (everything before `def check`);
  - `check`'s input derivation up to and including the nested `truth(r, geometric)`
    (frame, equilibrium, relative torsion, supports, section bracket), which this
    lane returns instead of running I50's private-verdict comparison;
  - `check`'s closing observable checks (support norm guards, extrema midpoint,
    headline identities), run on the successor envelope.
The published classes, normalized bits, S* and bounds come from the successor's
receipt as read by the accepted, unchanged Python reader. Nothing is reinterpreted:
a row kind the oracle does not map, or any mapping that needs a reading, stops.

Usage: u5_compare.py ORACLE I50_RECORD_LOG READER_ROOT OUT_JSON SUCCESSOR_FILE...
"""
import hashlib, json, sys
from fractions import Fraction as F
from pathlib import Path

ORACLE_SHA256 = "b1b586392f6bd91ed1b25c6589ca679602f83f920dc8b1ce2b073457a8d44d56"
RECORD_LOG_SHA256 = "5ced66b5e030db563298dac2d4385345a0d3fcd2c05514f86df0b43c18270b69"
EXTRACT_SHA256 = "a66a8a49422907556660cfdb4300d587845795160d2c5d0c76e07e682ef94fda"
oracle_path, record_log, reader_root, out_path, *successor_files = sys.argv[1:]
assert hashlib.sha256(Path(oracle_path).read_bytes()).hexdigest() == ORACLE_SHA256, "pinned oracle"
assert hashlib.sha256(Path(record_log).read_bytes()).hexdigest() == EXTRACT_SHA256, "pinned extract"
oracle = Path(oracle_path).read_text()

# 1. The oracle's helpers, unchanged.
helpers_end = oracle.index("def check(record):")
ns: dict = {}
exec(oracle[:helpers_end], ns)
# 2. Its input derivation through `truth`, unchanged, returning `truth`.
derive_end = oracle.index("    rows=record['envelope']['results'];vs=record['verdicts']")
exec(oracle[helpers_end:derive_end] + "    return truth\n", ns)
oracle_truth = ns["check"]
# 3. Its closing observable checks, unchanged, as a function of (record, rows, m).
obs_start = oracle.index("    byid={r['id']:r for r in rows};assert len(byid)==len(rows)")
obs_end = oracle.index("    return {'mode':record['mode']")
exec("def observables(record,rows,m):\n" + oracle[obs_start:obs_end] + "    return True\n", ns)
observables = ns["observables"]
bits, dec, distance, norm_value, raw_truth = ns["bits"], ns["dec"], ns["distance"], ns["norm_value"], ns["raw_truth"]

# The accepted Python reader (unchanged) supplies the published classes.
sys.path.insert(0, reader_root)
from core.analysis_runs import retained_precision as reader  # noqa: E402

i50 = {r["mode"]: r for r in (json.loads(line.split("I50_RECORD ", 1)[1]) for line in Path(record_log).read_text().splitlines() if "I50_RECORD " in line)}
assert sorted(i50) == ["dense_scrutiny", "sparse_interactive"]

def verdict(lo, hi, allowance):
    return "pass" if hi <= allowance else "fail" if lo > allowance else "unproved"

report = {"oracle_sha256": ORACLE_SHA256, "i50_record_log_sha256": RECORD_LOG_SHA256, "modes": []}
stops = []
for path in successor_files:
    doc = json.loads(Path(path).read_text())
    file_sha = hashlib.sha256(Path(path).read_bytes()).hexdigest()
    source, invocation = doc["source"], doc["invocation"]
    mode = invocation["solver_mode"]
    record = dict(i50[mode])
    # The oracle's derivation applies to this invocation only if it is I50's request.
    assert invocation["request"] == record["request"], "the successor's invocation is I50's named request"
    # The public entry refuses at G0 until U7 switches `_IMPLEMENTATION_COMPLETE` on;
    # its ordered checks are `_validate_draft` (eligibility off), as U1's lane used.
    validation = reader._validate_draft(source, invocation)
    classes = {c["result_id"]: c for c in validation["classifications"]}
    envelope = {k: v for k, v in source.items() if k != "retained_precision"}
    rows = envelope["results"]
    assert len(classes) == len(rows) and all(r["id"] in classes for r in rows), "one class per published row"
    truth = oracle_truth(record)  # runs the oracle's own input assertions
    selection = source["retained_precision"]["body"]["cases"][0]["selection"]
    absolute_receipt = {a["result_id"]: a["bound"] for a in selection["absolute_verified"]}
    out_rows = []
    counts: dict = {}
    for r in rows:
        c = classes[r["id"]]
        cls = c["class"]
        counts[cls] = counts.get(cls, 0) + 1
        n = norm_value(r)
        assert bits(n) == c["normalized_bits"], ("normalized bits", r["id"])
        entry = {"id": r["id"], "kind": r["kind"], "class": cls, "value_bits": bits(r["value"]), "unit": r["unit"],
                 "normalized_bits": c["normalized_bits"], "scale_bits": c["scale_bits"], "bound_bits": c["bound_bits"],
                 "recovery_method": r.get("recovery_method")}
        if cls == "non_quantity":
            entry["readouts"] = "not a quantity (no reference)"
            out_rows.append(entry)
            continue
        if cls == "not_covered":
            stops.append([mode, r["id"], "not_covered row present"])
        try:
            readouts = {"source_annulus": truth(r, True), "represented": truth(r, False)}
        except AssertionError as error:
            stops.append([mode, r["id"], f"the oracle does not map kind {r['kind']}: {error}"])
            out_rows.append(entry)
            continue
        entry["readouts"] = {}
        for name, t in readouts.items():
            lo, hi = distance(n, t)
            raw = raw_truth(t, r)
            rlo, rhi = distance(r["value"], raw)
            tests = {}
            if cls == "input_derived":
                tests["InputDerived"] = verdict(lo, hi, F(0))
            elif cls == "absolute_verified":
                assert absolute_receipt.get(r["id"]) == c["bound_bits"], ("receipt bound", r["id"])
                tests["AbsoluteBound"] = verdict(lo, hi, F(dec(c["bound_bits"])))
            elif cls == "relative_verified":
                # The published class: relative 1e-9 on the published value (D1 §4.1.6),
                # in SI and in the published unit (the oracle's DecimalSi/DecimalRaw).
                tests["Relative1e-9Si"] = verdict(lo, hi, abs(F(n)) / 10**9)
                tests["Relative1e-9Raw"] = verdict(rlo, rhi, abs(F(r["value"])) / 10**9)
                # Information only (not the class claim): the stop rule's bound on the
                # published S* plus the publication rounding (the oracle's SharperExact).
                scale = F(dec(c["scale_bits"]))
                sharper = F(2)**-64 * max(abs(F(n)), scale) * (1 + F(2)**-21) + F(2)**-53 * abs(F(n)) + F(2)**-1074
                tests["info:StopRuleSharper"] = verdict(lo, hi, sharper)
            else:
                stops.append([mode, r["id"], f"unexpected class {cls}"])
            entry["readouts"][name] = {"truth": [str(t[0]), str(t[1])], "error_lower": str(lo), "error_upper": str(hi), "tests": tests}
            for test, v in tests.items():
                if not test.startswith("info:") and v != "pass":
                    stops.append([mode, r["id"], f"{name} {test} {v}"])
        out_rows.append(entry)
    # The prepared route's maxima patches, overlay headlines and support rows, by the
    # oracle's own observable checks (mapped through the oracle, not reinterpreted).
    m = record["request"]["model"]
    observable_ok = observables({"envelope": envelope, "mode": mode}, rows, m)
    summary = {"mode": mode, "successor_file_sha256": file_sha, "receipt_sha256": source["retained_precision"]["receipt_sha256"],
               "reader": {k: validation[k] for k in ["invocation_bound", "numerical_eligible", "standing", "publication_sha256"]},
               "rows": len(rows), "classes": counts, "observable_checks": observable_ok,
               "class_claims_checked": sum(1 for e in out_rows if isinstance(e.get("readouts"), dict))}
    tallies: dict = {}
    for e in out_rows:
        if isinstance(e.get("readouts"), dict):
            for name, rd in e["readouts"].items():
                for test, v in rd["tests"].items():
                    key = f"{name}:{test}:{v}"
                    tallies[key] = tallies.get(key, 0) + 1
    summary["tallies"] = dict(sorted(tallies.items()))
    # Negative controls (non-vacuity): the same comparison refuses a value moved
    # just outside its class, for the first relative and the first absolute row.
    controls = []
    for cls_name in ["relative_verified", "absolute_verified"]:
        r = next(x for x in rows if classes[x["id"]]["class"] == cls_name)
        t = truth(r, True)
        n = F(norm_value(r))
        if cls_name == "relative_verified":
            moved = n * (1 + F(2, 10**9))
            lo, hi = distance(moved, t)
            v = verdict(lo, hi, abs(moved) / 10**9)
        else:
            moved = t[1] + 2 * F(dec(classes[r["id"]]["bound_bits"])) + F(2)**-1074
            lo, hi = distance(moved, t)
            v = verdict(lo, hi, F(dec(classes[r["id"]]["bound_bits"])))
        controls.append({"row": r["id"], "class": cls_name, "moved_verdict": v})
        if v != "fail":
            stops.append([mode, r["id"], f"negative control not refused ({cls_name}: {v})"])
    summary["negative_controls"] = controls
    report["modes"].append({"summary": summary, "rows": out_rows})
    print(json.dumps(summary))
report["stops"] = stops
Path(out_path).write_text(json.dumps(report, indent=1) + "\n")
print("STOPS", json.dumps(stops))
sys.exit(1 if stops else 0)
