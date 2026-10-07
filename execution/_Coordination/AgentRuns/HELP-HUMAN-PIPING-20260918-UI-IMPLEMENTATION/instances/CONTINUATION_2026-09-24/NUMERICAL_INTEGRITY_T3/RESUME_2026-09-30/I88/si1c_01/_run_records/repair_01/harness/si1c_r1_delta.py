#!/usr/bin/env python3
"""I88 T3-SI1c repair round 1: the candidate's dumps at the repaired head against
those at 7f233b2e01 (scratch tool, not repository content).

Every evaluator dump (point and interval) must be byte-identical. In every
runner dump, a line may differ only in `checks[i].bound_inputs[j].note`, and
only where the previous head wrote N-4's note alone and the base carried a
note: the repaired head writes "<base note>; <N-4 note>". Every such line must
be an N-4 line by the instrumented base's records. Hash-only lines (RV104's
bounded modes) are covered by the all-full variant.

Usage: si1c_r1_delta.py <old cand dir> <new cand dir> <base dir> <ibase dir> <report json>
"""
import collections
import json
import sys
from pathlib import Path

OLD, NEW, BASE, IBASE, REPORT = (Path(a) for a in sys.argv[1:6])
NOTE = "non-finite value (NaN or ±inf, after unit normalization): not bound"
EE = ["i79_dump.tsv", "i79_dump.tsv.interval", "i79_dump.tsv.panics", "i79_extreme.tsv",
      "i79_extreme.tsv.interval", "i79_extreme.tsv.panics", "i79_table.tsv", "i79_table.tsv.interval",
      "i79_table.tsv.panics", "rv104_ee.txt", "si1c_ee.txt"]
RUN = {"i79_run_plain.tsv": ("runner_crate.flags", "i73_runner_dump"),
       "i79_run_extreme.tsv": ("runner_crate.flags", "i79_runner_extreme_dump"),
       "rv104_run.txt": ("runner_crate.flags", "rv104_runner_dump"),
       "rv104_run_full.txt": ("runner_full.flags", "rv104_runner_dump"),
       "si1c_run.txt": ("runner_crate.flags", "si1c_runner_family")}
report = {"evaluator": {}, "runner": {}, "violations": collections.Counter(), "examples": []}


def bad(kind, detail):
    report["violations"][kind] += 1
    if len(report["examples"]) < 20:
        report["examples"].append(f"{kind}: {detail[:500]}")


for name in EE:
    a, b = (OLD / name).read_bytes(), (NEW / name).read_bytes()
    report["evaluator"][name] = {"identical": a == b, "lines": a.count(b"\n")}
    if a != b:
        bad("evaluator_dump_differs", name)


def json_of(dump, line):
    parts = line.split("\t")
    if dump == "rv104_run.txt" or dump == "rv104_run_full.txt":
        return parts[3] if len(parts) > 3 else None
    if dump == "i79_run_plain.tsv":
        return parts[4]
    if dump == "i79_run_extreme.tsv":
        return "\t".join(parts[5:])
    return parts[2]


for name, (flagfile, thread) in RUN.items():
    records = [l.rstrip("\n").split("\t") for l in (IBASE / flagfile).open() if l.split("\t", 1)[0] == thread]
    old = [l for l in (OLD / name).read_text().splitlines() if not l.startswith("#")]
    new = [l for l in (NEW / name).read_text().splitlines() if not l.startswith("#")]
    base = [l for l in (BASE / name).read_text().splitlines() if not l.startswith("#")]
    stats = collections.Counter()
    assert len(old) == len(new) == len(base) == len(records), (name, len(old), len(new), len(base), len(records))
    for rec, lo, ln, lb in zip(records, old, new, base):
        stats["lines"] += 1
        if lo == ln:
            stats["identical"] += 1
            continue
        stats["differ"] += 1
        n4_checks = {int(e.split("|")[0]) for e in (rec[2] if len(rec) > 2 else "").split(";")
                     if e and e.split("|")[1] == "n4in"}
        jo, jn, jb = json_of(name, lo), json_of(name, ln), json_of(name, lb)
        if jo is None or jn is None:
            stats["differ_hash_only"] += 1  # RV104's bounded modes: covered by rv104_run_full.txt
            if not n4_checks:
                bad("hash_only_non_n4_line_differs", f"{name} {ln[:120]}")
            continue
        do, dn, db = json.loads(jo), json.loads(jn), json.loads(jb)
        for i, (co, cn, cb) in enumerate(zip(do["checks"], dn["checks"], db["checks"])):
            if co == cn:
                continue
            if i not in n4_checks:
                bad("non_n4_check_differs", f"{name} check {i}: {ln[:200]}")
                continue
            for j, (bo, bn, bb) in enumerate(zip(co["bound_inputs"], cn["bound_inputs"], cb["bound_inputs"])):
                if bo == bn:
                    continue
                rest_o = {k: v for k, v in bo.items() if k != "note"}
                rest_n = {k: v for k, v in bn.items() if k != "note"}
                if rest_o != rest_n:
                    bad("record_differs_beyond_note", f"{name} {bo} {bn}")
                elif bo.get("note") == NOTE and bb.get("note") and bn.get("note") == f"{bb['note']}; {NOTE}":
                    kind = "interval" if bb["note"].startswith("interval ") else (
                        "library" if bb["note"].startswith("resolved from private library") else "other")
                    stats[f"note_appended_{kind}"] += 1
                else:
                    bad("note_not_appended_as_ruled", f"{name} old={bo.get('note')} new={bn.get('note')} base={bb.get('note')}")
            co2 = dict(co, bound_inputs=None)
            cn2 = dict(cn, bound_inputs=None)
            if co2 != cn2:
                bad("check_differs_beyond_records", f"{name} check {i}")
    report["runner"][name] = dict(stats)

report["violations"] = dict(report["violations"])
REPORT.write_text(json.dumps(report, indent=1, sort_keys=True, ensure_ascii=False))
print(json.dumps({"violations": report["violations"], "runner": report["runner"]}, indent=1))
