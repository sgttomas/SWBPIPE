"""PF M3-T judging pass: the whole collection of raw runner outputs judged directly against the reference, every
time (refutation round 3). It reads only the reference set and raw runner outputs; evaluation records are an
output (--eval-out), never an input. kd5t_report's M3 tables (../kd5_timoshenko, read-only) are printed over the
judged records.

Usage: python3 -I -B pft_report.py <set.jsonl> <raw runner output.jsonl> [...] [--ref own|twin]
                                   [--runner-sha256 <hex>] [--product-head <sha>] [--workers N]
                                   [--eval-out <records.jsonl>] [--json <report.json>]
                                   [--by family|theory|placement|section] [--phi-table]

Steps, all in this one invocation (it may chunk internally, from the raw outputs, for speed):
1. The reference set is validated exactly (pft_compare.reference_problems); every raw line must parse, and every raw
   record must match pft_compare.RAW_SCHEMA exactly (required keys, types, no unknown keys).
2. PROVENANCE: every schema-valid record's request_sha256 equals the sha256 of its model's runner-input line formed
   from the reference request (pft_compare.request_line), so a run of a changed request under the same tag fails;
   the collection carries exactly one runner_sha256 and one product_head (printed), equal to --runner-sha256 and
   --product-head where given, and the head is a commit (not "unknown" or "-dirty"); the reference set is valid.
3. INVENTORY: every model x entry (typed, captured) x mode (SparseInteractive, DenseScrutiny) exactly once, as a
   successful, schema-valid record, and nothing else. A runner error (any `error` key, null included) or a
   schema-invalid record is unsuccessful, as is a run with neither an integrity standing nor a blocking refusal: it
   fails the inventory and is never skipped (3,624 runs for the full set).
4. M3 VERDICT (the K-D5 M3 criterion and the soundness of its judging: C0-C4, C6, C8, C9): every record judged by
   pft_compare.work from its raw outputs, all of a model's records together, so C4 and C4b come from the canonical
   published fields of the raw records. Runner errors, schema-invalid records, duplicates, unexpected models and
   successful runs with neither an integrity standing nor a refusal fail C0. It passes only with provenance and
   inventory.
5. DESIGN-CONFORMANCE VERDICT (admission, pf-timoshenko.md section 2: C10; separate from M3): every record carries
   an empty design_failures; unjudged records fail it. It passes only with provenance and inventory.
6. OVERALL: PASS only when all four pass. The exit status is 0 only then.
Information clauses (C3lit, C4b, C5, C6i, C7, C11, C11tau_b, C11ops) are counted beside the verdicts.
"""

from __future__ import annotations

import collections
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.dont_write_bytecode = True
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, "..", "kd5_timoshenko"))
import kd5t_report as KR  # noqa: E402  (read-only reuse)
import pft_compare as PC  # noqa: E402


def read_jsonl(path):
    """(records, parse problems) of a JSON-lines file; a blank line is a problem too."""
    recs, probs = [], []
    if not os.path.exists(path):
        return recs, [f"{path}: missing"]
    with open(path, "rb") as f:
        data = f.read()
    for i, line in enumerate(data.split(b"\n")):
        if not line and i == len(data.split(b"\n")) - 1:
            continue
        try:
            recs.append(json.loads(line))
        except ValueError:
            probs.append(f"{path}:{i + 1}: not a JSON value")
    return recs, probs


def identity(r):
    return (r.get("tag"), r.get("entry"), r.get("mode")) if isinstance(r, dict) else (None, None, None)


def failing(r, why):
    tag, entry, mode = identity(r)
    return {"tag": tag, "entry": entry, "mode": mode, "harness_failure": why, "verdict_failures": {"C0": 1}, "info": {},
            "design_failures": {"unjudged": 1}, "verdict": "fail", "design_verdict": "fail"}


def judge_collection(set_path, raw_paths, variant="own", workers=6, runner_sha=None, head=None):
    """Judge the whole collection; return (report dict, judged records)."""
    lines, ref_parse = read_jsonl(set_path)
    ref_probs = ref_parse + PC.reference_problems(lines, twins=variant == "twin")
    gens = {x["tag"]: x for x in lines if isinstance(x, dict) and isinstance(x.get("tag"), str)}
    raws, raw_parse = [], []
    for p in raw_paths:
        rs, probs = read_jsonl(p)
        raws += rs
        raw_parse += probs
    valid, invalid = [], []
    for r in raws:
        probs = PC.raw_problems(r)
        (invalid.append((r, probs)) if probs else valid.append(r))
    # a run is successful when it carries no error and has an outcome: an integrity standing or a blocking refusal
    outcome = lambda r: any(d["code"].startswith("NUMERICAL_INTEGRITY") or d["severity"] == "blocking" for d in r["diagnostics"])
    errored = [r for r in valid if "error" in r or not outcome(r)]
    successful = [r for r in valid if "error" not in r and outcome(r)]
    # provenance
    runners = collections.Counter(r["runner_sha256"] for r in valid)
    heads = collections.Counter(r["product_head"] for r in valid)
    mismatch = sorted(identity(r) for r in valid if r["tag"] in gens and r["request_sha256"] != PC.request_sha256(r["tag"], gens[r["tag"]]["request"]))
    prov = [f"reference: {p}" for p in ref_probs[:20]] + raw_parse[:20]
    if len(runners) != 1:
        prov.append(f"{len(runners)} runner sha256 values in the collection")
    if len(heads) != 1:
        prov.append(f"{len(heads)} product heads in the collection")
    if runner_sha and set(runners) != {runner_sha}:
        prov.append("runner sha256 differs from --runner-sha256")
    if head and set(heads) != {head}:
        prov.append("product head differs from --product-head")
    if any(h == "unknown" or h.endswith("-dirty") for h in heads):
        prov.append("a product head that is not a commit (unknown or -dirty)")
    if mismatch:
        prov.append(f"{len(mismatch)} runs of a request other than the reference's (request sha256 differs), first {mismatch[:3]}")
    inv = PC.inventory(list(gens), [identity(r) for r in successful], [identity(r) for r in errored] + [identity(r) for r, _ in invalid])
    # judging: every valid record of a known model, all of a model's records in one job
    by_tag = collections.defaultdict(list)
    records = [dict(failing(r, "schema: " + "; ".join(p[:5]))) for r, p in invalid]
    for r in valid:
        if r["tag"] in gens:
            by_tag[r["tag"]].append(r)
        else:
            records.append(failing(r, "unexpected model"))
    jobs = []
    for tag, rs in by_tag.items():
        g = gens[tag]
        ref = g["ref"] if variant == "own" else gens[g["meta"]["twin"]]["ref"]
        jobs.append((g, ref, rs))
    if workers <= 1 or len(jobs) <= 1:
        results = list(map(PC.work, jobs))
    else:
        import multiprocessing as mp
        with mp.get_context("fork").Pool(workers) as pool:
            results = pool.map(PC.work, jobs, chunksize=2)
    for res in results:
        records += res
    for r in records:
        r["ref_variant"] = variant
    prov_pass = not prov
    m3_failing = sum(1 for r in records if r.get("verdict_failures") or not isinstance(r.get("verdict_failures"), dict))
    design_failing = [r for r in records if not isinstance(r.get("design_failures"), dict) or r["design_failures"]]
    m3_pass = prov_pass and inv["pass"] and m3_failing == 0
    design_pass = prov_pass and inv["pass"] and not design_failing
    out = {"provenance": prov, "provenance_pass": prov_pass, "runners": dict(runners), "product_heads": dict(heads),
           "inventory": inv, "m3_pass": m3_pass, "m3_failing_records": m3_failing, "design_pass": design_pass,
           "design_failing_records": len(design_failing), "records": len(records),
           "info_records": dict(collections.Counter(c for r in records for c in (r.get("info") or {}))),
           "c4_failures": sum(1 for r in records if (r.get("verdict_failures") or {}).get("C4")),
           "runner_errors": sum(1 for r in errored if "error" in r), "no_outcome": sum(1 for r in errored if "error" not in r),
           "schema_invalid": len(invalid)}
    out["pass"] = prov_pass and inv["pass"] and m3_pass and design_pass
    return out, records


def report(set_path, raw_paths, variant="own", workers=6, runner_sha=None, head=None, by=None, phi=False, quiet=False, eval_out=None):
    """The judging pass, printed; returns the report dict (with 'pass')."""
    say = (lambda *a: None) if quiet else print
    out, records = judge_collection(set_path, raw_paths, variant, workers, runner_sha, head)
    if eval_out:
        with open(eval_out, "w") as f:
            f.writelines(json.dumps(r) + "\n" for r in records)
    judged = [r for r in records if "cls" in r]
    if not quiet:
        groups = {"all": judged}
        if by:
            key = {"family": "family", "theory": "theory", "placement": "placement", "section": "section_variant"}[by]
            groups = collections.defaultdict(list)
            for r in judged:
                groups[str((r.get("meta") or {}).get(key))].append(r)
            groups = dict(sorted(groups.items()))
            groups["all"] = judged
        for name, rs in groups.items():
            t, worst, ex = KR.table(rs)
            say(f"===== {name}: {len(rs)} judged records ({len({r['tag'] for r in rs})} models)")
            for k in sorted(t):
                say(f"  {k:78s} {t[k]:>9}")
            for g, w in sorted(worst.items()):
                say(f"    worst {g:30s} {w[0]:.4g}  {w[1:]}")
            for k, v in ex.items():
                say(f"  examples [{k}] ({len(v)}): {v[:4]}")
        out["standings_vs_twins"] = KR.standings_vs_twins(judged)
        if phi:
            out["phi_table"] = KR.phi_table(judged, set_path)
    say(f"runner sha256: {out['runners']}")
    say(f"product head: {out['product_heads']}")
    say(f"PROVENANCE (reference schema, raw schema, request sha256, one runner, one product head): {'PASS' if out['provenance_pass'] else 'FAIL'}")
    for p in out["provenance"][:10]:
        say(f"  {p}")
    inv = out["inventory"]
    say(PC.inventory_text(inv))
    for k in ("missing", "duplicate", "unexpected", "unsuccessful"):
        if inv[k]:
            say(f"  {k} (first 5): {inv[k][:5]}")
    say(f"runner errors {out['runner_errors']}, runs with no outcome {out['no_outcome']}, schema-invalid records {out['schema_invalid']}, "
        f"C4 failures {out['c4_failures']}")
    say(f"M3 VERDICT (K-D5 M3 criterion and judging soundness: C0-C4, C6, C8, C9; requires provenance and inventory): "
        f"{'PASS' if out['m3_pass'] else 'FAIL'} ({out['m3_failing_records']} of {out['records']} records fail an acceptance clause)")
    say(f"DESIGN-CONFORMANCE VERDICT (admission, pf-timoshenko.md section 2, C10; separate from M3; requires provenance and inventory): "
        f"{'PASS' if out['design_pass'] else 'FAIL'} ({out['design_failing_records']} records)")
    say(f"information clauses, records raising each: {dict(sorted(out['info_records'].items()))}")
    if out["info_records"].get("C11tau_b"):
        say("  C11tau_b is the known tau_v,max defect on main (RN(1 + nu_d) cancels as E/(2G) -> 0), ruled by Agent 0 a separate correction (option (b))")
    say(f"OVERALL: {'PASS' if out['pass'] else 'FAIL'}")
    return out


def main():
    args = sys.argv[1:]
    opts = {"--ref": "own", "--runner-sha256": None, "--product-head": None, "--workers": "6", "--eval-out": None, "--json": None, "--by": None}
    for flag in list(opts):
        if flag in args:
            i = args.index(flag)
            opts[flag] = args[i + 1]
            del args[i:i + 2]
    phi = "--phi-table" in args
    if phi:
        args.remove("--phi-table")
    if len(args) < 2 or opts["--ref"] not in ("own", "twin") or any(a.startswith("--") for a in args):
        raise SystemExit(__doc__)
    out = report(args[0], args[1:], opts["--ref"], int(opts["--workers"]), opts["--runner-sha256"], opts["--product-head"], opts["--by"], phi,
                 eval_out=opts["--eval-out"])
    if opts["--json"]:
        json.dump(out, open(opts["--json"], "w"), indent=1, sort_keys=True, default=list)
    raise SystemExit(0 if out["pass"] else 1)


if __name__ == "__main__":
    main()
