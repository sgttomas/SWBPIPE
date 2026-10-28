"""The M3 judging pass: a whole collection of raw runner outputs judged against the reference set, every time, in one
invocation, for either route. It reads only the reference set (and, for the exact route's control, its controls
file) and raw runner outputs; evaluation records are an output (--eval-out), never an input. The judge is total and
fail-closed: any input it cannot fully validate gives a named OVERALL: FAIL, never a pass and never a traceback.

Usage: python3 -I -B m3_report.py --route pf|exact <set.jsonl> <raw runner output.jsonl> [...] --runner-sha256 <hex64>
                                  --product-head <hex40> --checkout <git checkout> [--ref own|twin|control]
                                  [--controls <controls.jsonl>] [--workers N] [--eval-out <new file>] [--json <new file>]
                                  [--by family|theory|placement|section]

Steps:
1. Every JSON input is read strictly (m3_json). The reference set is validated exactly and becomes the models' views
   and expected references (m3_view.reference_views); with any problem nothing is judged. Every raw record must
   validate into the canonical view (m3_view.run_view: RAW_SCHEMA, bound to its model); a record that does not is
   refused whole.
2. PROVENANCE: the reference file (and the controls file) must have a sha256 committed for the route in
   REFERENCE_SHA256SUMS, itself tracked and unchanged in the judge's checkout; --runner-sha256 (64 lowercase hex) and --product-head (40 lowercase hex) are required; the head must
   resolve with `git rev-parse --verify <head>^{commit}` in --checkout; every record carries exactly that runner and
   head; every record's request_sha256 equals the sha256 of its model's runner-input line formed from the reference
   request; the reference set is admitted (m3_view.reference_views: geometry, ids and length scales from the request,
   expected values from the hash-verified file, nothing else read). The record fields are assertions of the local build, bound by run.sh, not
   authentication (README, "Threat model").
3. INVENTORY: every model x entry (typed, captured) x mode (SparseInteractive, DenseScrutiny) exactly once, as a
   successful view, and nothing else. A runner error, a refused (schema-invalid or misbound) record or a duplicate
   identity fails the inventory and C0. With --ref control, the models without a control are not part of the
   collection: their records are set aside and counted.
4. M3 VERDICT (C0-C4, C6, C8, C9): every view judged by m3_compare.judge_model, all of a model's runs together. It
   passes only with provenance and inventory.
5. DESIGN-CONFORMANCE VERDICT (PF admission, C10; separate from M3): passes only with provenance and inventory.
6. NONVACUITY (Agent 0): the judged groups on Passed cases, overall and per family, are counted beside the Passed and
   Sensitive runs; it fails when the overall count is 0 or when any family of the set (other than a family named in
   EXEMPT with its reason) has 0.
7. OVERALL: PASS only when all five pass; the exit status is 0 only then.
Outputs (--eval-out, --json) are created exclusively: a path that exists, or that aliases an input (the same file,
through a symlink or a hard link), is refused before anything is written.
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
import m3_compare as MC  # noqa: E402
import m3_json  # noqa: E402
import m3_view as MV  # noqa: E402

# Families exempt from the per-family nonvacuity rule, each with its reason (README, "Nonvacuity"). None: no family of
# either set is all-Sensitive or all-refused by construction.
EXEMPT = {"pf": {}, "exact": {}}


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
    return tuple(r.get(k) if isinstance(r.get(k), str) else None for k in ("tag", "entry", "mode")) if isinstance(r, dict) else (None, None, None)


def failing(ident, why):
    tag, entry, mode = ident
    return {"tag": tag, "entry": entry, "mode": mode, "cls": "refused-record", "harness_failure": why, "verdict_failures": {"C0": 1}, "info": {},
            "design_failures": {"unjudged": 1}, "verdict": "fail", "design_verdict": "fail"}


def inventory(expected_tags, identities, unsuccessful=()):
    """Every expected model x entry x mode exactly once as a successful view, and nothing else."""
    want = {(t, e, m) for t in expected_tags for e in MV.ENTRIES for m in MV.MODES}
    got = collections.Counter(identities)
    out = {"expected": len(want), "received": sum(got.values()) + len(unsuccessful), "missing": sorted(want - set(got)),
           "duplicate": sorted(k for k, n in got.items() if n > 1), "unexpected": sorted(k for k in got if k not in want),
           "unsuccessful": sorted(unsuccessful, key=repr)}
    out["pass"] = not (out["missing"] or out["duplicate"] or out["unexpected"] or out["unsuccessful"])
    return out


def inventory_text(inv):
    return (f"INVENTORY (model x entry x mode): {'PASS' if inv['pass'] else 'FAIL'} ({inv['received']} records against {inv['expected']} expected; "
            f"{len(inv['missing'])} missing, {len(inv['duplicate'])} duplicate, {len(inv['unexpected'])} unexpected, "
            f"{len(inv['unsuccessful'])} unsuccessful or refused)")


def nonvacuity(records, families, route):
    """Passed and Sensitive runs and judged groups on Passed cases, per family and overall; the verdict (Agent 0)."""
    t = {f: collections.Counter() for f in families}
    for r in records:
        f = r.get("family")
        if f not in t:
            continue
        c = t[f]
        cls = r.get("cls", "")
        c["runs"] += 1
        c["passed_runs"] += cls == "passed"
        c["sensitive_runs"] += cls.startswith("sensitive")
        c["other_standing_runs"] += cls.startswith("other:")
        c["refused_runs"] += cls == "refused"
        c["passed_cases"] += r.get("passed_cases", 0)
        c["judged_passed_groups"] += r.get("n_judged_passed", 0)
    fam = {f: dict(c) for f, c in sorted(t.items())}
    total = collections.Counter()
    for c in t.values():
        total.update(c)
    exempt = EXEMPT[route]
    vacuous = sorted(f for f, c in t.items() if c["judged_passed_groups"] == 0 and f not in exempt)
    ok = total["judged_passed_groups"] > 0 and not vacuous
    return {"pass": ok, "families": fam, "overall": dict(total), "vacuous_families": vacuous, "exempt": dict(exempt)}


def judge_collection(route, set_path, raw_paths, variant="own", workers=6, runner_sha=None, head=None, checkout=None, controls_path=None,
                     references=None):
    """Judge the whole collection; return (report dict, judged records). Total: an unforeseen exception is a named failure.
    references: the accepted reference digests ({(route, kind): {sha256: name}}); None, always so from the CLI, reads
    the committed REFERENCE_SHA256SUMS (m3_view.pinned_references). The self-test passes its own for synthetic sets."""
    try:
        return _judge_collection(route, set_path, raw_paths, variant, workers, runner_sha, head, checkout, controls_path, references)
    except Exception as e:
        import traceback
        return ({"pass": False, "internal_error": f"{type(e).__name__}: {e}", "traceback": traceback.format_exc()[-2000:], "provenance": ["internal error"],
                 "provenance_pass": False, "inventory": inventory([], [], [("internal error", None, None)]), "m3_pass": False, "design_pass": False,
                 "nonvacuity": {"pass": False, "families": {}, "overall": {}, "vacuous_families": [], "exempt": {}}, "m3_failing_records": 0,
                 "design_failing_records": 0, "records": 0, "info_records": {}, "c4_failures": 0, "runner_errors": 0, "schema_invalid": 0,
                 "runners": {}, "product_heads": {}, "set_aside": 0}, [])


def _judge_collection(route, set_path, raw_paths, variant, workers, runner_sha, head, checkout, controls_path, references):
    if route not in MV.JUDGED:
        raise ValueError(f"unknown route {route!r}")
    # the reference files: their sha256 must be committed in REFERENCE_SHA256SUMS (Agent 0, the judge's final round)
    pins, ref_file_probs = (MV.pinned_references() if references is None else (references, []))
    ref_file_probs = ref_file_probs + MV.reference_file_problems(set_path, route, "set", pins)
    if controls_path:
        ref_file_probs += MV.reference_file_problems(controls_path, route, "controls", pins)
    lines, ref_parse = m3_json.read_jsonl(set_path)
    controls, ctl_parse = (m3_json.read_jsonl(controls_path) if controls_path else (None, []))
    models, ref_probs = MV.reference_views(lines, route, variant, controls) if not ref_parse and not ctl_parse else ({}, [])
    ref_probs = ref_parse + ctl_parse + ref_probs
    all_tags = {x["tag"] for x in lines if isinstance(x, dict) and isinstance(x.get("tag"), str)}
    contract_sha = MV.contract(route)[1]
    raws, raw_parse = [], []
    for p in raw_paths:
        rs, probs = m3_json.read_jsonl(p)
        raws += rs
        raw_parse += probs
    set_aside = 0
    valid, invalid = [], []
    for r in raws:
        tag = r.get("tag") if isinstance(r, dict) else None
        if variant == "control" and not ref_probs and tag in all_tags and tag not in models:
            set_aside += 1            # a model without a control: not part of the control collection
            continue
        m = models.get(tag) if isinstance(tag, str) else None
        view, probs = MV.run_view(r, route, m.facts if m else None)
        (invalid.append((identity(r), probs)) if probs else valid.append(view))
    errored = [v for v in valid if v.error is not None]
    successful = [v for v in valid if v.error is None]
    runners = collections.Counter(v.runner_sha256 for v in valid)
    heads = collections.Counter(v.product_head for v in valid)
    mismatch = sorted(v.identity for v in valid if v.tag in models and v.request_sha256 != models[v.tag].view.request_sha256)
    prov = ref_file_probs + [f"reference: {p}" for p in ref_probs[:20]] + raw_parse[:20]
    if not isinstance(runner_sha, str) or not re.fullmatch(r"[0-9a-f]{64}", runner_sha):
        prov.append("--runner-sha256 is required (64 lowercase hex)")
    prov += head_problems(head, checkout)
    if len(runners) != 1 or set(runners) != {runner_sha}:
        prov.append(f"records carry runner sha256 {sorted(runners)[:3]}, not exactly --runner-sha256")
    if len(heads) != 1 or set(heads) != {head}:
        prov.append(f"records carry product heads {sorted(heads)[:3]}, not exactly --product-head")
    if mismatch:
        prov.append(f"{len(mismatch)} runs of a request other than the reference's (request sha256 differs), first {mismatch[:3]}")
    inv = inventory(list(models), [v.identity for v in successful], [v.identity for v in errored] + [i for i, _ in invalid])
    records = [failing(i, "refused by the view: " + "; ".join(p[:5])) for i, p in invalid]
    by_tag = collections.defaultdict(list)
    for v in valid:
        if v.tag in models:
            by_tag[v.tag].append(v)
        else:
            records.append(failing(v.identity, "unexpected model"))
    jobs = []
    for tag, vs in by_tag.items():
        ident = collections.Counter(v.identity for v in vs)
        dup = {k for k, n in ident.items() if n > 1}
        # a run identity received more than once is rejected before judging: no record of it is judged, so a later
        # record can never replace an earlier failure (refutation round 1); the inventory fails as well
        records += [dict(failing(k, f"duplicate run identity ({ident[k]} records)"), duplicate_identity=ident[k]) for k in sorted(dup)]
        keep = tuple(v for v in vs if v.identity not in dup)
        if keep:
            jobs.append((models[tag].view, models[tag].expected, keep))
    if workers <= 1 or len(jobs) <= 1:
        results = [MC.judge_model(*j) for j in jobs]
    else:
        import multiprocessing as mp
        with mp.get_context("fork").Pool(workers) as pool:
            results = pool.starmap(MC.judge_model, jobs, chunksize=2)
    for res in results:
        records += res
    for r in records:
        r["ref_variant"] = variant
    prov_pass = not prov
    m3_failing = sum(1 for r in records if r.get("verdict_failures") or not isinstance(r.get("verdict_failures"), dict))
    design_failing = [r for r in records if not isinstance(r.get("design_failures"), dict) or r["design_failures"]]
    m3_pass = prov_pass and inv["pass"] and m3_failing == 0
    design_pass = prov_pass and inv["pass"] and not design_failing
    nv = nonvacuity(records, sorted({m.view.family for m in models.values()}), route)
    out = {"route": route, "variant": variant, "contract_sha256": contract_sha, "provenance": prov, "provenance_pass": prov_pass,
           "runners": dict(runners), "product_heads": dict(heads), "inventory": inv, "m3_pass": m3_pass, "m3_failing_records": m3_failing,
           "design_pass": design_pass, "design_failing_records": len(design_failing), "nonvacuity": nv, "records": len(records),
           "info_records": dict(collections.Counter(c for r in records for c in (r.get("info") or {}))),
           "c4_failures": sum(1 for r in records if (r.get("verdict_failures") or {}).get("C4")),
           "runner_errors": len(errored), "schema_invalid": len(invalid), "set_aside": set_aside,
           "schema_problems": sorted({p for _, ps in invalid for p in ps})[:20]}
    out["pass"] = prov_pass and inv["pass"] and m3_pass and design_pass and nv["pass"]
    return out, records


def table(recs):
    """The M3 counts of judged records (after kd5t_report.table)."""
    t = collections.Counter()
    worst = {}
    for r in recs:
        t["runs"] += 1
        t["entry:" + str(r.get("entry"))] += 1
        t["cls:" + str(r.get("cls"))] += 1
        if r.get("cls") == "passed":
            t["Passed"] += 1
            t["Passed: judged groups"] += r.get("n_judged", 0)
            t["Passed: judged member/section/support/stress breaches (groups)"] += len(r.get("breaches", []))
            t["Passed: of which Lame groups"] += len(r.get("lame_breaches", []))
            t["Passed: groups judged only on the reference scales (F1)"] += r.get("n_judged_ref_only", 0)
            t["Passed: displacement/rotation groups beyond S*_kind (C3)"] += len(r.get("disp_beyond_kd5", []))
            t["Passed: displacement/rotation groups beyond the separate scale (C3lit)"] += len(r.get("disp_beyond_literal", []))
            for g, w in (r.get("worst_by_group") or {}).items():
                if g not in worst or w["ratio"] > worst[g][0]:
                    worst[g] = (w["ratio"], r["tag"], r["entry"], r["mode"], w["entity"], w["loc"], w["basis"])
            for key, val in (("disp_kd5_worst (C3, err/(c S*_kind))", r.get("worst_disp_kd5")),):
                if val is not None and (key not in worst or val > worst[key][0]):
                    worst[key] = (val, r["tag"], r["entry"], r["mode"], None, None, None)
        if str(r.get("cls", "")).startswith("sensitive"):
            t["Sensitive"] += 1
        t["coverage failures (C2 maxima)"] += len(r.get("max_fail", []))
        t["max rows checked"] += r.get("n_max", 0)
        t["unmapped rows"] += r.get("n_unmapped", 0)
        t["reference rows unpublished"] += r.get("n_missing", 0)
        t["rows of kinds the reference does not model (not judged)"] += r.get("n_other_rows", 0)
        t["headline findings"] += bool(r.get("headline_findings"))
        vf = r.get("verdict_failures") or {}
        t["VERDICT runs failing (C0-C4, C6, C8, C9)"] += bool(vf)
        for c, n in vf.items():
            t[f"VERDICT {c} failures"] += n
        for c, n in (r.get("info") or {}).items():
            t[f"INFO {c}"] += n
    return t, worst


def report(route, set_path, raw_paths, variant="own", workers=6, runner_sha=None, head=None, by=None, quiet=False, eval_out=None, checkout=None,
           json_out=None, controls_path=None, references=None):
    """The judging pass, printed; returns the report dict (with 'pass'). Outputs are checked before anything is judged."""
    say = (lambda *a: None) if quiet else print
    outs = [o for o in (eval_out, json_out) if o]
    bad_out = output_problems(outs, [set_path] + list(raw_paths) + ([controls_path] if controls_path else []))
    if bad_out:
        for p in bad_out:
            say(f"refused: {p}")
        say("OVERALL: FAIL (outputs refused; nothing judged or written)")
        return {"pass": False, "outputs_refused": bad_out}
    out, records = judge_collection(route, set_path, raw_paths, variant, workers, runner_sha, head, checkout, controls_path, references)
    if out.get("internal_error"):
        say(f"internal error: {out['internal_error']}")
    if eval_out:
        write_new(eval_out, "".join(json.dumps(r, default=list) + "\n" for r in records))
    judged = [r for r in records if r.get("cls") not in (None, "refused-record")]
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
            t, worst = table(rs)
            say(f"===== {name}: {len(rs)} judged records ({len({r['tag'] for r in rs})} models)")
            for k in sorted(t):
                say(f"  {k:78s} {t[k]:>9}")
            for g, w in sorted(worst.items()):
                say(f"    worst {g:30s} {w[0]:.4g}  {w[1:]}")
    say(f"route {out.get('route')}, reference {out.get('variant')}, contract sha256 {out.get('contract_sha256')}")
    say(f"runner sha256: {out['runners']}")
    say(f"product head: {out['product_heads']}")
    say(f"PROVENANCE (committed reference files, reference, raw view, request sha256, one runner, one product head): "
        f"{'PASS' if out['provenance_pass'] else 'FAIL'}")
    for p in out["provenance"][:10]:
        say(f"  {p}")
    inv = out["inventory"]
    say(inventory_text(inv))
    for k in ("missing", "duplicate", "unexpected", "unsuccessful"):
        if inv[k]:
            say(f"  {k} (first 5): {inv[k][:5]}")
    say(f"runner errors {out['runner_errors']}, records refused by the view {out['schema_invalid']}, C4 failures {out['c4_failures']}"
        + (f", set aside (no control) {out['set_aside']}" if out.get("set_aside") else ""))
    for p in out.get("schema_problems", [])[:10]:
        say(f"  refused: {p}")
    say(f"M3 VERDICT (K-D5 M3 criterion and judging soundness: C0-C4, C6, C8, C9; requires provenance and inventory): "
        f"{'PASS' if out['m3_pass'] else 'FAIL'} ({out['m3_failing_records']} of {out['records']} records fail an acceptance clause)")
    say(f"DESIGN-CONFORMANCE VERDICT (PF admission, C10; separate from M3; requires provenance and inventory): "
        f"{'PASS' if out['design_pass'] else 'FAIL'} ({out['design_failing_records']} records)")
    nv = out["nonvacuity"]
    say("NONVACUITY by family: runs, Passed runs, Sensitive runs, judged groups on Passed cases")
    for f, c in nv["families"].items():
        say(f"  {f:24s} {c.get('runs', 0):6d} {c.get('passed_runs', 0):6d} {c.get('sensitive_runs', 0):6d} {c.get('judged_passed_groups', 0):9d}"
            + ("  (exempt: " + nv["exempt"][f] + ")" if f in nv["exempt"] else ""))
    o = nv["overall"]
    say(f"  {'overall':24s} {o.get('runs', 0):6d} {o.get('passed_runs', 0):6d} {o.get('sensitive_runs', 0):6d} {o.get('judged_passed_groups', 0):9d}")
    say(f"NONVACUITY VERDICT (judged groups on Passed cases > 0 overall and in every family): {'PASS' if nv['pass'] else 'FAIL'}"
        + (f" (vacuous: {nv['vacuous_families']})" if nv["vacuous_families"] else ""))
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
        opts = {"--route": None, "--ref": "own", "--runner-sha256": None, "--product-head": None, "--checkout": None, "--workers": "6",
                "--eval-out": None, "--json": None, "--by": None, "--controls": None}
        for flag in list(opts):
            if flag in args:
                i = args.index(flag)
                if i + 1 >= len(args):
                    raise ValueError(flag)
                opts[flag] = args[i + 1]
                del args[i:i + 2]
        if (len(args) < 2 or opts["--route"] not in ("pf", "exact") or opts["--ref"] not in ("own", "twin", "control")
                or (opts["--ref"] == "control") != bool(opts["--controls"]) or (opts["--ref"] == "control" and opts["--route"] != "exact")
                or any(a.startswith("--") for a in args) or not opts["--runner-sha256"] or not opts["--product-head"] or not opts["--checkout"]
                or not opts["--workers"].isdigit() or opts["--by"] not in (None, "family", "theory", "placement", "section")):
            raise ValueError("usage")
    except ValueError:
        print(__doc__)
        print("OVERALL: FAIL (usage)")
        raise SystemExit(2)
    try:
        out = report(opts["--route"], args[0], args[1:], opts["--ref"], int(opts["--workers"]), opts["--runner-sha256"], opts["--product-head"],
                     opts["--by"], eval_out=opts["--eval-out"], checkout=opts["--checkout"], json_out=opts["--json"], controls_path=opts["--controls"])
    except BaseException as e:
        print(f"OVERALL: FAIL (internal error: {type(e).__name__})")
        raise SystemExit(1)
    raise SystemExit(0 if out["pass"] else 1)


if __name__ == "__main__":
    main()
