#!/usr/bin/env python3
"""RV104 scratch comparison of base and candidate dumps (not repository content).

Usage: rv104_compare.py ee BASE CAND OUT_JSON
       rv104_compare.py run BASE CAND OUT_JSON

ee:  point lines (P) may differ only where the base panicked; the candidate
     line must then be a blocked result whose last finding is the
     NonFiniteInput finding for the panic's site. Interval lines (I) must be
     identical, and no line on either side may panic in interval mode.
run: the point run (p) may differ only where the base panicked; the
     candidate must then carry a blocking NonFiniteInput finding and no panic.
     b0 must equal p on each side (bytes, by hash). Bounded modes may differ
     only where the base panicked. Multi-check packs: each check's outcome in
     the three-check run equals the same check run alone.
"""
import json
import re
import sys
from collections import Counter, defaultdict

FINDING = re.compile(
    r'EvaluationFinding \{ code: (\w+), subject_id: "((?:[^"\\]|\\.)*)", message: "((?:[^"\\]|\\.)*)" \}'
)
RATIO_MSG = "same-dimension quotient (ratio) must be finite"
NAN_MSG = ("table argument must be finite: a NaN argument is neither inside nor outside "
           "the table range")


def read(path):
    out = []
    with open(path, encoding="utf-8") as f:
        for raw in f:
            parts = raw.rstrip("\n").split("\t", 2)
            out.append(parts)
    return out


def site_of(panic):
    if "ratio unit reference is non-empty" in panic:
        return "ratio"
    if "in-range step lookup always has a governing row" in panic:
        return "step"
    if "in-range interpolation always has a bracketing pair" in panic:
        return "interpolate"
    return "other:" + panic[:120]


def ee(base_path, cand_path):
    base, cand = read(base_path), read(cand_path)
    report = {"lines": len(base), "cand_lines": len(cand)}
    assert len(base) == len(cand), "line counts differ"
    counts = Counter()
    sites = Counter()
    unexpected = []
    examples = defaultdict(list)
    for b, c in zip(base, cand):
        assert b[0] == c[0] and b[1] == c[1], f"label mismatch {b[:2]} {c[:2]}"
        label, mode = b[0], b[1]
        bb, cb = b[2], c[2]
        if mode == "I":
            counts["I_total"] += 1
            if bb.startswith("PANIC"):
                counts["I_base_panic"] += 1
            if cb.startswith("PANIC"):
                counts["I_cand_panic"] += 1
            if bb != cb:
                unexpected.append({"label": label, "mode": mode, "base": bb[:400], "cand": cb[:400]})
            else:
                counts["I_identical"] += 1
            continue
        counts["P_total"] += 1
        if cb.startswith("PANIC"):
            counts["P_cand_panic"] += 1
            unexpected.append({"label": label, "mode": mode, "why": "candidate panics", "cand": cb[:400]})
            continue
        if bb == cb:
            counts["P_identical"] += 1
            if bb.startswith("none|"):
                counts["P_identical_blocked"] += 1
            continue
        if not bb.startswith("PANIC"):
            unexpected.append({"label": label, "mode": mode, "why": "non-panic input differs",
                               "base": bb[:600], "cand": cb[:600]})
            continue
        site = site_of(bb)
        sites[site] += 1
        family = label.split("_")[0]
        sites[f"{family}:{site}"] += 1
        findings = FINDING.findall(cb)
        ok = cb.startswith("none|") and "value: None" in cb and findings
        if ok:
            code, subject, message = findings[-1]
            if site == "ratio":
                ok = (code, subject, message) == ("NonFiniteInput", "divide", RATIO_MSG)
            elif site in ("step", "interpolate"):
                ok = code == "NonFiniteInput" and message == NAN_MSG and subject not in ("divide", "literal")
            else:
                ok = False
        if ok:
            counts["P_base_panic_now_blocked"] += 1
            if len(examples[site]) < 3:
                examples[site].append({"label": label, "base": bb[:200], "cand": cb[:500]})
            # Earlier findings, if any (e.g. a malformed table, a status boundary).
            if len(findings) > 1:
                counts["P_base_panic_with_earlier_findings"] += 1
                prior = tuple(sorted({f[0] for f in findings[:-1]}))
                counts["prior:" + ",".join(prior)] += 1
        else:
            unexpected.append({"label": label, "mode": mode, "why": "base panic, candidate not the site's finding",
                               "base": bb[:300], "cand": cb[:600]})
    report["counts"] = dict(sorted(counts.items()))
    report["base_panic_sites"] = dict(sorted(sites.items()))
    report["examples"] = examples
    report["unexpected_count"] = len(unexpected)
    report["unexpected"] = unexpected[:200]
    return report


def run(base_path, cand_path):
    base, cand = read(base_path), read(cand_path)
    assert len(base) == len(cand), "line counts differ"
    counts = Counter()
    unexpected = []
    examples = []
    # Per label: mode -> (hash or PANIC, full text or None)
    per = {"base": defaultdict(dict), "cand": defaultdict(dict)}

    def split(body):
        if body.startswith("PANIC"):
            return ("PANIC", body)
        if "\t" in body:
            h, text = body.split("\t", 1)
            return (h, text)
        return (body, None)

    for b, c in zip(base, cand):
        assert b[0] == c[0] and b[1] == c[1], f"label mismatch {b[:2]} {c[:2]}"
        label, mode = b[0], b[1]
        bh, bt = split(b[2])
        ch, ct = split(c[2])
        per["base"][label][mode] = (bh, bt)
        per["cand"][label][mode] = (ch, ct)
        counts[f"{mode}_total"] += 1
        if ch == "PANIC":
            counts[f"{mode}_cand_panic"] += 1
            unexpected.append({"label": label, "mode": mode, "why": "candidate panics", "cand": c[2][:300]})
            continue
        if bh == "PANIC":
            counts[f"{mode}_base_panic"] += 1
            counts[f"{mode}_base_panic_site:{site_of(bt)}"] += 1
        if bh == ch:
            counts[f"{mode}_identical"] += 1
            continue
        if bh != "PANIC":
            unexpected.append({"label": label, "mode": mode, "why": "non-panic run differs",
                               "base": (bt or bh)[:600], "cand": (ct or ch)[:600]})
            continue
        if mode == "p" or ct is not None:
            result = json.loads(ct)
            blocked = [ch_ for ch_ in result["checks"]
                       if any(f["code"] == "NonFiniteInput" and f["severity"] == "blocking"
                              and (f["subject_id"] == "divide" or f["message"] == NAN_MSG)
                              for f in ch_["evaluator_findings"])]
            if blocked and all(x["status"] == "RULE_INPUTS_INCOMPLETE" and x["computed_value"] is None
                               if "computed_value" in x else x["status"] == "RULE_INPUTS_INCOMPLETE"
                               for x in blocked):
                counts[f"{mode}_base_panic_now_blocked"] += 1
                if len(examples) < 4:
                    examples.append({"label": label, "base": bt[:200], "cand_blocked_check": blocked[0]})
            else:
                unexpected.append({"label": label, "mode": mode, "why": "base panic, candidate lacks the block",
                                   "cand": ct[:800]})
        else:
            counts[f"{mode}_base_panic_cand_hash_only"] += 1

    # b0 equals p, per side.
    for side in ("base", "cand"):
        for label, modes in per[side].items():
            if "p" in modes and "b0" in modes:
                if modes["p"][0] == "PANIC" or modes["b0"][0] == "PANIC":
                    if modes["p"][0] != modes["b0"][0] and (modes["p"][0] == "PANIC") != (modes["b0"][0] == "PANIC"):
                        unexpected.append({"label": label, "why": f"{side}: p and b0 disagree on panic"})
                    continue
                counts[f"{side}_p_vs_b0_checked"] += 1
                if modes["p"][0] != modes["b0"][0]:
                    unexpected.append({"label": label, "why": f"{side}: b0 bytes differ from p"})

    # Multi-check independence on the candidate, and on the base where it ran.
    for side in ("base", "cand"):
        for label, modes in per[side].items():
            if not label.startswith("m_") or "_single" in label:
                continue
            p = modes.get("p")
            if p is None or p[0] == "PANIC":
                continue
            result = json.loads(p[1])
            for ci, outcome in enumerate(result["checks"]):
                single = per[side][f"{label}_single{ci}"]["p"]
                if single[0] == "PANIC":
                    unexpected.append({"label": label, "why": f"{side}: single {ci} panics but multi ran"})
                    continue
                alone = json.loads(single[1])["checks"][0]
                counts[f"{side}_multi_vs_single_checked"] += 1
                if alone != outcome:
                    unexpected.append({"label": label, "why": f"{side}: check {ci} differs from its single run"})
            statuses = [c["status"] for c in result["checks"]]
            rank = {"USER_RULE_CHECKED": 0, "RULE_INPUTS_INCOMPLETE": 1, "USER_RULE_FAILED": 2}
            worst = max(statuses, key=lambda s: rank[s])
            if result["aggregate_status"] != worst:
                unexpected.append({"label": label, "why": f"{side}: aggregate is not worst-of"})
    # Where the base multi run panicked: the candidate's other checks equal
    # the base's single runs of those checks.
    for label, modes in per["base"].items():
        if not label.startswith("m_") or "_single" in label or modes["p"][0] != "PANIC":
            continue
        counts["multi_base_panic"] += 1
        cand_result = json.loads(per["cand"][label]["p"][1])
        for ci, outcome in enumerate(cand_result["checks"]):
            base_single = per["base"][f"{label}_single{ci}"]["p"]
            if base_single[0] == "PANIC":
                counts["multi_base_panic_panicking_check"] += 1
                if not any(f["code"] == "NonFiniteInput" for f in outcome["evaluator_findings"]):
                    unexpected.append({"label": label, "why": f"check {ci} panicked on base but has no NonFiniteInput"})
                continue
            counts["multi_base_panic_other_checks_compared"] += 1
            if json.loads(base_single[1])["checks"][0] != outcome:
                unexpected.append({"label": label, "why": f"check {ci} differs from the base single run"})
    return {"lines": len(base), "counts": dict(sorted(counts.items())), "examples": examples,
            "unexpected_count": len(unexpected), "unexpected": unexpected[:200]}


if __name__ == "__main__":
    kind, base_path, cand_path, out_path = sys.argv[1:5]
    report = ee(base_path, cand_path) if kind == "ee" else run(base_path, cand_path)
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(report, f, indent=1, sort_keys=True)
    print(json.dumps({k: v for k, v in report.items() if k not in ("unexpected", "examples")}, indent=1))
