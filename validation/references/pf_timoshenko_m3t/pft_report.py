"""PF M3-T judging pass: the whole collection of raw runner outputs judged directly against the reference, every
time (refutation round 3). It reads only the reference set and raw runner outputs; evaluation records are an
output (--eval-out), never an input. kd5t_report's M3 tables (../kd5_timoshenko, read-only) are printed over the
judged records. The judge is total and fail-closed (refutation round 4): any input it cannot fully validate gives
a named OVERALL: FAIL, never a pass and never a traceback.

Usage: python3 -I -B pft_report.py <set.jsonl> <raw runner output.jsonl> [...] --runner-sha256 <hex64>
                                   --product-head <hex40> --checkout <git checkout> [--ref own|twin] [--workers N]
                                   [--eval-out <new file>] [--json <new file>] [--by family|theory|placement|section]
                                   [--phi-table]

Steps, all in this one invocation (it may chunk internally, from the raw outputs, for speed):
1. Every JSON input is read strictly (pft_compare.strict_loads: a duplicate key, NaN or Infinity is refused). The
   reference set is validated exactly, every field the judge reads typed (pft_compare.reference_problems); with an
   invalid reference nothing is judged. Every raw record must match pft_compare.RAW_SCHEMA exactly, including its
   nested fields and its single outcome (one recognised standing per load case, or one recognised refusal code), and
   bind to its model (its rows' cases and entities, its evidence's cases and pipes).
2. PROVENANCE: --runner-sha256 (64 lowercase hex) and --product-head (40 lowercase hex) are required; the head must
   resolve with `git rev-parse --verify <head>^{commit}` in --checkout; every record carries exactly that runner and
   head; every schema-valid record's request_sha256 equals the sha256 of its model's runner-input line formed from the
   reference request (pft_compare.request_line); the reference set is valid. The record fields are assertions of the
   local build, bound by run.sh, not authentication (README, "Threat model").
3. INVENTORY: every model x entry (typed, captured) x mode (SparseInteractive, DenseScrutiny) exactly once, as a
   successful, schema-valid, bound record, and nothing else. A runner error (any `error` key, null included), a
   schema-invalid or misbound record, or a run with no outcome is unsuccessful: it fails the inventory and C0 and is
   never skipped (3,624 runs for the full set).
4. M3 VERDICT (the K-D5 M3 criterion and the soundness of its judging: C0-C4, C6, C8, C9): every record judged by
   pft_compare.work from its raw outputs, all of a model's records together, so C4 and C4b come from the canonical
   published fields of the raw records. It passes only with provenance and inventory.
5. DESIGN-CONFORMANCE VERDICT (admission, pf-timoshenko.md section 2: C10; separate from M3): every record carries
   an empty design_failures; unjudged records fail it. It passes only with provenance and inventory.
6. OVERALL: PASS only when all four pass. The exit status is 0 only then.
Outputs (--eval-out, --json) are created exclusively: a path that exists, or that aliases an input (the same file,
through a symlink or a hard link), is refused before anything is written.
Information clauses (C3lit, C4b, C5, C6i, C7, C11, C11tau_b, C11ops) are counted beside the verdicts.
"""

from __future__ import annotations

import collections
import json
import os
import re
import subprocess
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
    parts = data.split(b"\n")
    for i, line in enumerate(parts):
        if not line and i == len(parts) - 1:
            continue
        try:
            recs.append(PC.strict_loads(line.decode("utf-8")))
        except (ValueError, UnicodeDecodeError) as e:
            probs.append(f"{path}:{i + 1}: not a strict JSON value ({e})")
    return recs, probs


def head_problems(head, checkout):
    """The stated head must be 40 lowercase hex and resolve to that commit in the checkout."""
    if not isinstance(head, str) or not re.fullmatch(r"[0-9a-f]{40}", head):
        return ["--product-head is not 40 lowercase hex"]
    if not checkout or not os.path.isdir(checkout):
        return ["--checkout is not a directory"]
    try:
        p = subprocess.run(["git", "-C", checkout, "rev-parse", "--verify", "--quiet", head + "^{commit}"], capture_output=True, text=True, timeout=60)
    except Exception as e:
        return [f"git rev-parse failed ({type(e).__name__})"]
    if p.returncode != 0 or p.stdout.strip() != head:
        return [f"--product-head {head} is not a commit of {checkout}"]
    return []


def output_problems(outputs, inputs):
    """Outputs are new files: refused if the path exists (a dangling symlink included) or aliases any input."""
    out = []
    real_in = {os.path.realpath(p) for p in inputs}
    for o in outputs:
        if os.path.lexists(o):
            out.append(f"output {o} exists")
        elif os.path.realpath(o) in real_in:
            out.append(f"output {o} aliases an input")
    if len(set(map(os.path.realpath, outputs))) != len(outputs):
        out.append("two outputs name one file")
    return out


def write_new(path, text):
    with open(path, "x") as f:
        f.write(text)


def identity(r):
    return (r.get("tag"), r.get("entry"), r.get("mode")) if isinstance(r, dict) else (None, None, None)


def failing(r, why):
    tag, entry, mode = identity(r)
    return {"tag": tag, "entry": entry, "mode": mode, "harness_failure": why, "verdict_failures": {"C0": 1}, "info": {},
            "design_failures": {"unjudged": 1}, "verdict": "fail", "design_verdict": "fail"}


def judge_collection(set_path, raw_paths, variant="own", workers=6, runner_sha=None, head=None, checkout=None):
    """Judge the whole collection; return (report dict, judged records). Total: an unforeseen exception is a named failure."""
    try:
        return _judge_collection(set_path, raw_paths, variant, workers, runner_sha, head, checkout)
    except Exception as e:
        import traceback
        return ({"pass": False, "internal_error": f"{type(e).__name__}: {e}", "traceback": traceback.format_exc()[-2000:], "provenance": ["internal error"],
                 "provenance_pass": False, "inventory": PC.inventory([], [], [("internal error", None, None)]), "m3_pass": False, "design_pass": False,
                 "m3_failing_records": 0, "design_failing_records": 0, "records": 0, "info_records": {}, "c4_failures": 0, "runner_errors": 0,
                 "schema_invalid": 0, "runners": {}, "product_heads": {}}, [])


def _judge_collection(set_path, raw_paths, variant, workers, runner_sha, head, checkout):
    lines, ref_parse = read_jsonl(set_path)
    ref_probs = ref_parse + PC.reference_problems(lines, twins=variant == "twin")
    gens = {x["tag"]: x for x in lines if isinstance(x, dict) and isinstance(x.get("tag"), str)} if not ref_probs else {}
    raws, raw_parse = [], []
    for p in raw_paths:
        rs, probs = read_jsonl(p)
        raws += rs
        raw_parse += probs
    valid, invalid = [], []
    for r in raws:
        probs = PC.raw_problems(r)
        if not probs and "error" not in r and r["tag"] in gens:
            probs = PC.binding_problems(r, gens[r["tag"]])
        (invalid.append((r, probs)) if probs else valid.append(r))
    # a schema-valid record without an error key has exactly one outcome (pft_compare.outcome_problems)
    errored = [r for r in valid if "error" in r]
    successful = [r for r in valid if "error" not in r]
    # provenance
    runners = collections.Counter(r["runner_sha256"] for r in valid)
    heads = collections.Counter(r["product_head"] for r in valid)
    mismatch = sorted(identity(r) for r in valid if r["tag"] in gens and r["request_sha256"] != PC.request_sha256(r["tag"], gens[r["tag"]]["request"]))
    prov = [f"reference: {p}" for p in ref_probs[:20]] + raw_parse[:20]
    if not isinstance(runner_sha, str) or not re.fullmatch(r"[0-9a-f]{64}", runner_sha):
        prov.append("--runner-sha256 is required (64 lowercase hex)")
    prov += head_problems(head, checkout)
    if len(runners) != 1 or set(runners) != {runner_sha}:
        prov.append(f"records carry runner sha256 {sorted(runners)[:3]}, not exactly --runner-sha256")
    if len(heads) != 1 or set(heads) != {head}:
        prov.append(f"records carry product heads {sorted(heads)[:3]}, not exactly --product-head")
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
           "runner_errors": len(errored), "schema_invalid": len(invalid),
           "schema_problems": sorted({p for _, ps in invalid for p in ps})[:20]}
    out["pass"] = prov_pass and inv["pass"] and m3_pass and design_pass
    return out, records


def report(set_path, raw_paths, variant="own", workers=6, runner_sha=None, head=None, by=None, phi=False, quiet=False, eval_out=None,
           checkout=None, json_out=None):
    """The judging pass, printed; returns the report dict (with 'pass'). Outputs are checked before anything is
    judged or written, and created exclusively."""
    say = (lambda *a: None) if quiet else print
    outs = [o for o in (eval_out, json_out) if o]
    bad_out = output_problems(outs, [set_path] + list(raw_paths))
    if bad_out:
        for p in bad_out:
            say(f"refused: {p}")
        say("OVERALL: FAIL (outputs refused; nothing judged or written)")
        return {"pass": False, "outputs_refused": bad_out}
    out, records = judge_collection(set_path, raw_paths, variant, workers, runner_sha, head, checkout)
    if out.get("internal_error"):
        say(f"internal error: {out['internal_error']}")
    if eval_out:
        write_new(eval_out, "".join(json.dumps(r) + "\n" for r in records))
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
    say(f"runner errors {out['runner_errors']}, schema-invalid or misbound records {out['schema_invalid']}, C4 failures {out['c4_failures']}")
    for p in out.get("schema_problems", [])[:10]:
        say(f"  schema: {p}")
    say(f"M3 VERDICT (K-D5 M3 criterion and judging soundness: C0-C4, C6, C8, C9; requires provenance and inventory): "
        f"{'PASS' if out['m3_pass'] else 'FAIL'} ({out['m3_failing_records']} of {out['records']} records fail an acceptance clause)")
    say(f"DESIGN-CONFORMANCE VERDICT (admission, pf-timoshenko.md section 2, C10; separate from M3; requires provenance and inventory): "
        f"{'PASS' if out['design_pass'] else 'FAIL'} ({out['design_failing_records']} records)")
    say(f"information clauses, records raising each: {dict(sorted(out['info_records'].items()))}")
    if out["info_records"].get("C11tau_b"):
        say("  C11tau_b is the known tau_v,max defect on main (RN(1 + nu_d) cancels as E/(2G) -> 0), ruled by Agent 0 a separate correction (option (b))")
    if json_out:
        write_new(json_out, json.dumps(out, indent=1, sort_keys=True, default=list))
    say(f"OVERALL: {'PASS' if out['pass'] else 'FAIL'}")
    return out


def main():
    try:
        args = sys.argv[1:]
        opts = {"--ref": "own", "--runner-sha256": None, "--product-head": None, "--checkout": None, "--workers": "6", "--eval-out": None,
                "--json": None, "--by": None}
        for flag in list(opts):
            if flag in args:
                i = args.index(flag)
                opts[flag] = args[i + 1]
                del args[i:i + 2]
        phi = "--phi-table" in args
        if phi:
            args.remove("--phi-table")
        if (len(args) < 2 or opts["--ref"] not in ("own", "twin") or any(a.startswith("--") for a in args)
                or not opts["--runner-sha256"] or not opts["--product-head"] or not opts["--checkout"] or not opts["--workers"].isdigit()
                or opts["--by"] not in (None, "family", "theory", "placement", "section")):
            print(__doc__)
            print("OVERALL: FAIL (usage)")
            raise SystemExit(2)
        out = report(args[0], args[1:], opts["--ref"], int(opts["--workers"]), opts["--runner-sha256"], opts["--product-head"], opts["--by"], phi,
                     eval_out=opts["--eval-out"], checkout=opts["--checkout"], json_out=opts["--json"])
    except SystemExit:
        raise
    except BaseException as e:
        print(f"OVERALL: FAIL (internal error: {type(e).__name__})")
        raise SystemExit(1)
    raise SystemExit(0 if out["pass"] else 1)


if __name__ == "__main__":
    main()
