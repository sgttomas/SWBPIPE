"""PF M3-T premise census: kd5t_census with its engine bound to the PF reference, and the bound census.

  python3 -I -B pft_census.py hager|probe|standings ...      kd5t_census's modes (read-only reuse)
  python3 -I -B pft_census.py bound <set.jsonl> --raw <raw.jsonl>... --dump <dump.jsonl>... [--tags <file>]
                                    [--workers N] [--hager typed|none] --out <summary.json>
  python3 -I -B pft_census.py bound --selftest

`bound` checks Agent 0's census rule, B >= |error|, on every judged group of every Passed case: B is the product's
own bound for the group, read from a census-probe dump (README "Census probe"), and |error| is the group's actual
error in the group's norm as the M3 judge defines it, against the exact solution of the product's intended system
(pft_intended.py: the solver's binary64 E, G, A, Iy, Iz, J and A_s from the dump's frames; Agent 0's ruling, option
2). The error against this set's physical rows (the annulus of OD and wall) is reported beside it as information.

- Each raw record becomes the shared judge's canonical view (../m3_judge/m3_view.py) and is judged by its own
  judge_run (../m3_judge/m3_compare.py): the groups, their norms and which are judged are the judge's. The error is
  recomputed exactly, sqrt(sum (published - exact)^2) over the same rows against the reference's 45-digit values;
  the judge's own q and err must agree with it.
- Each probe group (entity, location, kind) is matched to the judge's group: axial, shear, torsion and bending by
  name; support_force, support_moment and their magnitudes as reactF, reactM, reactFmag and reactMmag at `node`;
  stress groups (s_axial, s_bending, s_torsion) take the B of their resultant at the same entity and location,
  divided by the reference's conversion (A, Z or J/ro, times 10^6); cut_i and cut_j are checked with their own B
  against the error of end_i and end_j. Displacement and rotation groups have no product bound; they are counted.
- A run is bound to the dump by its labels (tag, entry, mode, case) and by its operands: every evaluation line of
  the model carries the same frames, one per pipe, with E and G bit-equal to the reference's Pa operands (meta E,
  G); those frames are the intended system's operands. Stress groups convert with the intended section.
- The groups line of a case is the last one written for it. Its `eval` must name an evaluation line of the same
  run and load case whose lambda_sigma equals the groups line's (the two lines' ID, case and lambda_sigma together).
- Alongside: the margin B/|error| (its distribution and the worst group) and, per evaluation line, kd5t_census's
  probe record: epsilon_E/||K~_s||_1 against 2^-40 (go) and 2^-36 (no-go) and the A1 Hager margin
  2^10 est/||K~_s^-1||_1 (typed entries only with --hager typed, the default: the gate is entry-independent).
"""

from __future__ import annotations

import collections
import json
import math
import os
import struct
import sys
from fractions import Fraction as Q

HERE = os.path.dirname(os.path.abspath(__file__))
sys.dont_write_bytecode = True
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, "..", "kd5_timoshenko"))
sys.path.insert(0, os.path.join(HERE, "..", "m3_judge"))
import pft_engine  # noqa: E402
import kd5t_census as KCE  # noqa: E402  (read-only reuse)
import m3_compare as MC  # noqa: E402
import m3_json  # noqa: E402
import m3_view as MV  # noqa: E402
import pft_generate as PG  # noqa: E402
import pft_intended as PI  # noqa: E402

KCE.E = pft_engine            # the census's exact engine is the PF reference
_corpus_of = KCE.corpus_of


def corpus_of(tag):
    return "PF M3-T" if tag.startswith(("pft|", "probe|")) else _corpus_of(tag)


KCE.corpus_of = corpus_of

# probe group kind -> the judge's group name
PROBE_GROUP = {"axial": "axial", "shear": "shear", "torsion": "torsion", "bending": "bending", "support_force": "reactF",
               "support_moment": "reactM", "support_force_magnitude": "reactFmag", "support_moment_magnitude": "reactMmag"}
STRESS_OF = {"s_axial": "axial", "s_bending": "bending", "s_torsion": "torsion"}
STRESS_KINDS = ("element_local_axial_normal_stress", "element_local_bending_normal_stress_y", "element_local_bending_normal_stress_z",
                "element_local_torsional_shear_stress")
STRESS_ULPS = 4           # Agent 0: the stress rule's allowance for Z's representation and the final rounding
CUT_END = {"cut_i": "end_i", "cut_j": "end_j"}


def bits(x):
    return struct.pack(">d", x).hex()


def exact_groups(model, run, reference=None):
    """{(case, group, entity, location, basis): (q2, e2)} exact, over the rows judge_run groups, Passed cases only.
    The error is against the set's exact (physical) rows, or against `reference` {case: {row key: value}}."""
    exp = model.expected
    standing = {s.case: s.code for s in run.standings}
    out = {}
    for ec in exp.cases:
        if standing.get(ec.case) != MC.PASSED:
            continue
        refrows = dict(ec.rows) if reference is None else reference[ec.case]
        acc = collections.defaultdict(lambda: [Q(0), Q(0)])
        for row in run.rows:
            if row.case != ec.case or row.value is None:
                continue
            if exp.derived_judged and row.kind in MC.DERIVED:
                continue
            if row.kind not in MC.GROUP_OF_KIND and row.kind not in MC.SUPPORT_KINDS:
                continue
            if row.key not in refrows:
                continue
            (g, ent, loc, basis), _ = MC._group_of(row)
            a = acc[(ec.case, g, ent, loc, basis)]
            a[0] += Q(row.value) ** 2
            a[1] += (Q(row.value) - refrows[row.key]) ** 2
        out.update({k: (v[0], v[1]) for k, v in acc.items()})
    return out


def qsqrt_upper(x):
    """An upper bound of sqrt(x) for exact x >= 0, as a Fraction (the float sqrt, raised one ulp if it falls short)."""
    r = Q(math.sqrt(float(x)))
    while r * r < x:
        r = Q(math.nextafter(float(r), math.inf))
    return r


FRAME_KEYS = ("theory", "E_bits", "G_bits", "A_bits", "Iy_bits", "Iz_bits", "J_bits", "A_s_bits")


def model_frames(model, runs, dumps):
    """({member id: frame}, problems): the solver's binary64 operands of the model's members. Every evaluation line of
    every run (entries, modes, cases) must carry the same frames, one per pipe of the request, with E and G the
    reference's Pa operands bit for bit and each value equal to its `_bits`."""
    seen, problems = {}, []
    E_pa, G_pa = (bits(float(x)) for x in PA_OPERANDS[model.view.tag])
    theory = "timoshenko" if model.view.theory == pft_engine.TIMOSHENKO else "euler_bernoulli"
    for run in runs:
        for ev in dumps.get((run.tag, run.entry, run.mode), ([], {}))[0]:
            fr = {}
            for f in ev.get("frames") or []:
                mid = PI.member_id(f["member"])
                if any(f.get(k[:-5]) is not None and bits(f[k[:-5]]) != f[k] for k in FRAME_KEYS[1:] if f.get(k) is not None):
                    problems.append(f"{run.tag}: frame {mid}: a value differs from its bits")
                fr[mid] = f
            sig = {m: tuple(f.get(k) for k in FRAME_KEYS) for m, f in fr.items()}
            if not seen:
                seen = dict(fr)
                first = sig
            elif sig != first:
                problems.append(f"{(run.tag, run.entry, run.mode, ev.get('case'))}: frames differ from the model's first evaluation")
    if seen:
        if set(seen) != set(model.facts.pipes):
            problems.append(f"{model.view.tag}: frames name {sorted(seen)[:3]}, not the request's pipes")
        for mid, f in seen.items():
            if f.get("E_bits") != E_pa or f.get("G_bits") != G_pa:
                problems.append(f"{model.view.tag}: frame {mid} resolved E/G {f.get('E')}/{f.get('G')} are not the reference's Pa operands")
            if f.get("theory") != theory:
                problems.append(f"{model.view.tag}: frame {mid} theory {f.get('theory')!r}, not the model's {theory!r}")
    return seen, problems


def judge_model_bounds(model, runs, dumps, intended=None):
    """(checks, counts, problems, stress rows) for one model's runs. `intended` is {case: {row key: value}} of the intended-system
    reference (the criterion); without it the criterion is against the set's physical rows (the self-test's case)."""
    checks, problems, stress = [], [], []
    counts = collections.Counter()
    pipes = {p.pipe: p for p in model.expected.sections}
    ipipes = INTENDED_SECTIONS.get(model.view.tag, pipes)
    for run in runs:
        rid = (run.tag, run.entry, run.mode)
        if run.error is not None or run.refusal is not None:
            counts["runs refused or errored (nothing judged)"] += 1
            continue
        rec = MC.judge_run(model.view, model.expected, run)
        stats = {(g["case"], g["group"], g["entity"], g["loc"], g["basis"]): g for g in rec["_groups"]}
        phys = exact_groups(model, run)
        ex = exact_groups(model, run, intended) if intended is not None else phys
        evals, glines = dumps.get(rid, ([], {}))
        if not phys:
            counts["runs with no Passed case (nothing judged)"] += 1
            continue
        if not evals:
            problems.append(f"{rid}: no evaluation line in the dumps")
            continue
        by_eval = {}
        for ev in evals:
            by_eval.setdefault(ev.get("eval"), []).append(ev)
        counts["runs bound"] += 1
        for case in {k[0] for k in ex}:
            gl = glines.get(case)
            if gl is None:
                problems.append(f"{rid} {case}: a Passed case without a groups line")
                continue
            named = by_eval.get(gl.get("eval"), [])
            if not (type(gl.get("eval")) is int and len(named) == 1 and named[0].get("case") == case == gl.get("case")
                    and named[0].get("lambda_sigma_bits") == gl.get("lambda_sigma_bits")):
                problems.append(f"{rid} {case}: the groups line's eval {gl.get('eval')!r} does not name one evaluation line of this "
                                "run and case with its lambda_sigma")
            probe = {}
            for g in gl["groups"]:
                kind = PROBE_GROUP.get(g["kind"])
                if kind is None:
                    problems.append(f"{rid} {case}: unknown probe group kind {g['kind']!r}")
                    continue
                probe[(kind, g["entity"], g["location"])] = g
            for key, (q2, e2) in sorted(ex.items(), key=repr):
                if key[0] != case:
                    continue
                _, grp, ent, loc, basis = key
                st = stats.get(key)
                if st is None:
                    problems.append(f"{rid}: exact group {key} absent from the judge's groups")
                    continue
                pe2 = phys[key][1]
                if st["q"] != math.sqrt(MC.qf(q2)) or st["err"] != math.sqrt(MC.qf(pe2)):
                    problems.append(f"{rid}: group {key}: the judge's q/err differ from the exact recomputation")
                if not st["judged"]:
                    counts["groups not judged (below the floor)"] += 1
                    continue
                if grp in MC.NONACT:
                    counts["judged displacement/rotation groups (no product bound; C3)"] += 1
                    continue
                conv = pconv = Q(1)
                pname = grp
                if grp in STRESS_OF:
                    # K-D5 (kd5-member-relative-bound.md:224): "Stress rows are scaled at p from the certified resultants
                    # and judged through the resultant they scale." Information: each row's error in ulps of its value.
                    res = probe.get((STRESS_OF[grp], ent, loc))
                    sec = dict(ipipes[ent].bases)[None]
                    for row in run.rows:
                        if row.case == case and row.entity == ent and row.location == loc and row.kind in STRESS_KINDS \
                                and row.value is not None and MC._group_of(row)[0][0] == grp:
                            exact = intended[case][row.key] if intended is not None else dict(next(c for c in model.expected.cases if c.case == case).rows)[row.key]
                            ulps = abs(Q(row.value) - exact) / Q(math.ulp(row.value))
                            # the stress rule (Agent 0; line 224 made quantitative): |error| <= B_resultant/conv + 4 ulps,
                            # conv the intended section's A, Z or J/r_o times 10^6 (MPa from N or N*m)
                            sconv = (sec.A if row.kind == STRESS_KINDS[0] else sec.J / ipipes[ent].ro if row.kind == STRESS_KINDS[3] else sec.Z) * MC.MPA
                            allow = None if res is None or res.get("B") is None else Q(res["B"]) / sconv + STRESS_ULPS * Q(math.ulp(row.value))
                            usage = None if allow is None else (float(abs(Q(row.value) - exact) / allow) if allow else (0.0 if Q(row.value) == exact else math.inf))
                            if allow is None:
                                problems.append(f"{rid} {case}: stress {row.kind} {ent} {loc}: no resultant bound")
                            stress.append({"usage": usage, "held": usage is not None and usage <= 1,"tag": run.tag, "entry": run.entry, "mode": run.mode, "case": case, "kind": row.kind, "entity": ent,
                                           "location": loc, "published": row.value, "intended_exact": float(exact), "ulps": float(ulps),
                                           "Z": float(sec.Z), "A": float(sec.A), "J_over_ro": float(sec.J / ipipes[ent].ro),
                                           "resultant_B": None if res is None else res.get("B")})
                    counts["judged stress groups (information)"] += 1
                    continue
                targets = [(pname, loc, loc)] + [(pname, cut, loc) for cut, end in CUT_END.items() if end == loc]
                for name, ploc, eloc in targets:
                    g = probe.get((name, ent, ploc))
                    if g is None:
                        if ploc == loc:
                            problems.append(f"{rid} {case}: judged group {grp} {ent} {loc}: no probe group {name} at {ploc}")
                        continue
                    if g.get("fires"):
                        problems.append(f"{rid} {case}: probe group {name} {ent} {ploc} fires on a Passed case")
                    if g.get("B") is None:
                        problems.append(f"{rid} {case}: probe group {name} {ent} {ploc} has no bound ({g.get('unavailable')})")
                        continue
                    B = Q(g["B"]) / conv                      # in the judged group's units (the intended section)
                    held = B * B >= e2                        # the criterion: B >= |published - intended exact|, exactly
                    err = qsqrt_upper(e2) if e2 else Q(0)
                    Bp = Q(g["B"]) / pconv                    # information: against the physical (OD/wall) reference
                    perr = qsqrt_upper(pe2) if pe2 else Q(0)
                    checks.append({"tag": run.tag, "entry": run.entry, "mode": run.mode, "case": case, "group": grp, "entity": ent,
                                   "location": ploc, "probe_kind": name, "B": float(B), "error": float(err), "held": held,
                                   "margin": (float(B / err) if err else math.inf), "q": math.sqrt(MC.qf(q2)),
                                   "physical_error": float(perr), "physical_held": Bp * Bp >= pe2,
                                   "physical_excess_over_q": float((perr - Bp) / qsqrt_upper(q2)) if q2 and perr > Bp else 0.0})
                    counts["checks (cut sections included)" if ploc != loc else "judged groups with a product bound"] += 1
    return checks, counts, problems, stress


PA_OPERANDS = {}      # tag -> the reference's Pa operands (meta E, G), which the dump's resolved moduli must equal
INTENDED_SECTIONS = {}  # tag -> {pipe: PipeSection} of the intended system (stress conversions of the criterion)


def stress_table(rows):
    """Per stress kind: rows, the maximum |error| in ulps of the published stress, and the distribution."""
    out = {}
    for kind in STRESS_KINDS:
        xs = sorted(r["ulps"] for r in rows if r["kind"] == kind)
        if xs:
            out[kind] = {"rows": len(xs), "max_ulps": xs[-1], "exact": sum(1 for x in xs if x == 0),
                         "<=0.5": sum(1 for x in xs if 0 < x <= 0.5), "(0.5,1]": sum(1 for x in xs if 0.5 < x <= 1),
                         "(1,2]": sum(1 for x in xs if 1 < x <= 2), "(2,4]": sum(1 for x in xs if 2 < x <= 4), ">4": sum(1 for x in xs if x > 4)}
    return out


def intended_rows(line, model, frames):
    """({case: {row key: Fraction}}, problems): the intended-system reference of a set line from its frames, and the
    intended sections (A, Z, J, r_o) for the stress conversions."""
    try:
        ref = PI.intended_reference(PG.pa_request(line), frames)
    except Exception as e:
        return None, [f"{line['tag']}: the intended system is not solved ({type(e).__name__}: {e})"]
    out = {cid: {k: Q(v) for k, v in ref.rows(cid).items()} for cid in ref.cases}
    secs = []
    for mbr in ref.members:
        props = MV.SectionProps(A=Q(mbr.sec["A"]), Z=Q(mbr.sec["Z"]), J=Q(mbr.sec["J"]))
        secs.append(MV.PipeSection(pipe=mbr.id, ro=Q(mbr.sec["ro"]), bases=((None, props),), tau=None))
    INTENDED_SECTIONS[line["tag"]] = {p.pipe: p for p in secs}
    return out, []


def _load_dumps(paths, tags):
    """{(tag, entry, mode): ([evaluation lines without Ks], {case: last groups line})}, and the evaluation lines for
    the Hager pass (as text, with Ks)."""
    out = collections.defaultdict(lambda: ([], {}))
    multi = collections.Counter()
    for p in paths:
        with open(p) as f:
            for line in f:
                x = m3_json.strict_loads(line)
                rid = (x["tag"], x["entry"], x["mode"])
                if tags and x["tag"] not in tags:
                    continue
                if x["record"] == "evaluation":
                    x.pop("Ks", None)
                    out[rid][0].append(x)
                elif x["record"] == "groups":
                    if x["case_id"] in out[rid][1]:
                        multi[rid] += 1
                    out[rid][1][x["case_id"]] = x
    return out, multi


def _hager_lines(paths, tags, which):
    for p in paths:
        with open(p) as f:
            for line in f:
                if '"record":"evaluation"' not in line[:40]:
                    continue
                x = json.loads(line)
                if (tags and x["tag"] not in tags) or (which == "typed" and x["entry"] != "typed"):
                    continue
                yield line


def _hager_job(line):
    return KCE.probe_record(json.loads(line), {})


def bound(set_path, raw_paths, dump_paths, tags=None, workers=6, hager="typed"):
    lines, probs = m3_json.read_jsonl(set_path)
    if probs:
        raise SystemExit(probs[:3])
    lines = [x for x in lines if not tags or x["tag"] in tags]
    models, probs = MV.reference_views(lines, "pf", "own")
    if probs:
        raise SystemExit(probs[:3])
    for x in lines:
        PA_OPERANDS[x["tag"]] = (x["meta"]["E"], x["meta"]["G"])
    runs = collections.defaultdict(list)
    problems = []
    for p in raw_paths:
        recs, probs = m3_json.read_jsonl(p)
        problems += probs
        for r in recs:
            if r.get("tag") in models:
                v, probs = MV.run_view(r, "pf", models[r["tag"]].facts)
                if v is None:
                    problems.append(f"{(r.get('tag'), r.get('entry'), r.get('mode'))}: refused by the view: {probs[:2]}")
                else:
                    runs[v.tag].append(v)
    dumps, multi = _load_dumps(dump_paths, set(models))
    checks, counts, stress = [], collections.Counter(), []
    by_line = {x["tag"]: x for x in lines}
    for tag, m in sorted(models.items()):
        want = {(e, md) for e in m.view.entries for md in MV.MODES}
        got = {(r.entry, r.mode) for r in runs[tag]}
        if got != want:
            problems.append(f"{tag}: runs {sorted(got)} are not the declared {sorted(want)}")
        intended = None
        frames, p = model_frames(m, runs[tag], dumps)
        problems += p
        if frames and not p:
            intended, p = intended_rows(by_line[tag], m, frames)
            problems += p
            counts["models solved as the intended system"] += intended is not None
        if intended is None:
            if any(r.error is None and r.refusal is None and exact_groups(m, r) for r in runs[tag]):
                problems.append(f"{tag}: Passed cases but no intended-system reference")
            continue
        c, n, p, st = judge_model_bounds(m, runs[tag], dumps, intended)
        stress += st
        checks += c
        counts += n
        problems += p
    failed = [c for c in checks if not c["held"]]
    finite = sorted(c["margin"] for c in checks if math.isfinite(c["margin"]))
    worst = min(checks, key=lambda c: c["margin"]) if checks else None
    hrecs = []
    if hager != "none":
        import multiprocessing as mp
        with mp.get_context("fork").Pool(workers) as pool:
            hrecs = list(pool.imap(_hager_job, _hager_lines(dump_paths, set(models), hager), chunksize=4))
    eps = [r["epsilon_E_over_norm"] for r in hrecs if "epsilon_E_over_norm" in r]
    a1 = [r["A1_margin"] for r in hrecs if "A1_margin" in r]
    q = lambda xs, f: xs[min(len(xs) - 1, int(f * len(xs)))] if xs else None
    summary = {
        "models": len(models), "runs": sum(len(v) for v in runs.values()), "counts": dict(counts),
        "judged_groups_with_bound": counts["judged groups with a product bound"],
        "checks": len(checks), "failed": failed[:50], "n_failed": len(failed),
        "error_zero": sum(1 for c in checks if c["error"] == 0.0),
        "margin": {"min": finite[0] if finite else None, "p01": q(finite, 0.01), "median": q(finite, 0.5), "max": finite[-1] if finite else None,
                   "log10_histogram": dict(sorted(collections.Counter(math.floor(math.log10(x)) for x in finite).items()))},
        "worst": worst, "groups_lines_superseded": sum(multi.values()),
        "physical_information": {"held": sum(1 for c in checks if c["physical_held"]), "of": len(checks),
                                 "excess_over_q_max": max((c["physical_excess_over_q"] for c in checks), default=0.0)},
        "stress_rule": {"rows": len(stress), "held": sum(1 for x in stress if x["held"]),
                        "worst_usage": max((x["usage"] for x in stress if x["usage"] is not None), default=None),
                        "worst": max((x for x in stress if x["usage"] is not None), key=lambda x: x["usage"], default=None)},
        "stress_beyond_rule": [x for x in stress if not x["held"]][:20],
        "stress_information": stress_table(stress),
        "stress_beyond_4_ulps": {"rows": sum(1 for x in stress if x["ulps"] > STRESS_ULPS),
                                 "intended_exact_zero": sum(1 for x in stress if x["ulps"] > STRESS_ULPS and x["intended_exact"] == 0.0),
                                 "max_abs_published": max((abs(x["published"]) for x in stress if x["ulps"] > STRESS_ULPS), default=None),
                                 "by_model": dict(sorted(collections.Counter(x["tag"] for x in stress if x["ulps"] > STRESS_ULPS).items()))},
        "evaluations": len(hrecs), "eps_go": sum(1 for r in hrecs if r.get("go")), "eps_no_go": sum(1 for r in hrecs if r.get("no_go")),
        "eps_over_norm_max": max(eps) if eps else None, "A1_held": sum(1 for r in hrecs if r.get("A1")), "A1_failed": sum(1 for r in hrecs if r.get("A1") is False),
        "A1_margin_min": min(a1) if a1 else None,
        "problems": problems[:50], "n_problems": len(problems),
    }
    judged_n = counts["judged groups with a product bound"]
    held_judged = sum(1 for c in checks if c["held"] and c["location"] not in CUT_END)
    summary["held_judged"] = held_judged
    summary["verdict"] = (f"bound held on {held_judged} of {judged_n} judged groups"
                          + (f" (and on {sum(1 for c in checks if c['held'] and c['location'] in CUT_END)} of "
                             f"{sum(1 for c in checks if c['location'] in CUT_END)} section cuts)"))
    return summary


def selftest():
    """A synthetic Passed run of the PF selftest set with a hand-made dump: B >= |error| holds, then B < |error| fails."""
    import copy
    import m3_synth as S
    gens, _ = m3_json.read_jsonl(os.path.join(HERE, "..", "m3_judge", "selftest", "pf_set.jsonl"))
    g = next(x for x in gens if x["ref"]["expect"] == "rows" and x["meta"].get("entries") is None)
    r = S.synthetic_run("pf", g, entry="typed", mode="SparseInteractive", head="0" * 40)
    row = next(x for x in r["results"] if x["kind"] == "element_local_axial_force")
    row["value"] = row["value"] * (1 + 1e-12) + 1e-9            # a small error, below the judge's breach threshold
    models, probs = MV.reference_views([g], "pf", "own")
    assert not probs, probs
    m = models[g["tag"]]
    PA_OPERANDS[g["tag"]] = (g["meta"]["E"], g["meta"]["G"])
    v, probs = MV.run_view(r, "pf", m.facts)
    assert v is not None, probs
    ex = exact_groups(m, v)
    key = next(k for k in ex if k[1] == "axial" and k[2] == row["entity_ref"] and k[3] == row["location"])
    err = math.sqrt(MC.qf(ex[key][1]))
    ev = {"record": "evaluation", "tag": g["tag"], "entry": "typed", "mode": "SparseInteractive", "case": "case:1", "eval": 1, "lambda_sigma_bits": "0" * 16,
          "frames": [{"member": "m", "E": float(g["meta"]["E"]), "E_bits": bits(float(g["meta"]["E"])), "G": float(g["meta"]["G"]),
                      "G_bits": bits(float(g["meta"]["G"]))}]}

    def run_with(B, eval_id=1, case="case:1", ls="0" * 16):
        groups = [{"entity": key[2], "location": key[3], "kind": "axial", "B": B, "fires": False}]
        gl = {"record": "groups", "case": case, "case_id": "case:1", "eval": eval_id, "lambda_sigma_bits": ls, "groups": groups}
        return judge_model_bounds(m, [v], {(g["tag"], "typed", "SparseInteractive"): ([ev], {"case:1": gl})})

    ok = []
    c1, _, p1, _ = run_with(err * 4)
    ok.append(("B = 4 |error| holds", [c for c in c1 if c["entity"] == key[2] and c["location"] == key[3]] and all(
        c["held"] for c in c1 if c["entity"] == key[2] and c["location"] == key[3])))
    c2, _, _, _ = run_with(err / 4)
    ok.append(("B = |error|/4 fails", any(not c["held"] for c in c2 if c["entity"] == key[2] and c["location"] == key[3])))
    c3, _, _, _ = run_with(math.nextafter(err, 0.0) if err else 0.0)
    ok.append(("B one ulp below |error| fails", any(not c["held"] for c in c3 if c["entity"] == key[2] and c["location"] == key[3])))
    # the intended system: frames from the binary64 rounding of the engine's own sections
    ref = pft_engine.Reference(PG.pa_request(g))
    frames = {}
    for mbr in ref.members:
        f = {"member": f"member:{len(mbr.id)}:{mbr.id}:", "theory": "timoshenko" if mbr.theory == pft_engine.TIMOSHENKO else "euler_bernoulli",
             "E": float(g["meta"]["E"]), "G": float(g["meta"]["G"]), "A": float(mbr.sec["A"]), "Iy": float(mbr.sec["I"]),
             "Iz": float(mbr.sec["I"]), "J": float(mbr.sec["J"]), "A_s": float(mbr.As_shear) if mbr.theory == pft_engine.TIMOSHENKO else None}
        f.update({k + "_bits": bits(f[k]) for k in ("E", "G", "A", "Iy", "Iz", "J")}, A_s_bits=bits(f["A_s"]) if f["A_s"] is not None else None)
        frames[mbr.id] = f
    ev["frames"] = list(frames.values())
    dumps1 = {(g["tag"], "typed", "SparseInteractive"): ([ev], {})}
    fr, p4 = model_frames(m, [v], dumps1)
    ok.append(("the probe's member labels resolve to the request's pipes", not p4 and set(fr) == set(m.facts.pipes)))
    rows_i, p5 = intended_rows(g, m, {k: dict(f) for k, f in fr.items()})
    phys = {k: v for k, v in m.expected.cases[0].rows}
    scale = max(abs(x) for k, x in phys.items() if k[0].startswith("element_local_bending_moment"))
    dev = max(abs(rows_i["case:1"][k] - x) for k, x in phys.items() if k[0].startswith("element_local_bending_moment"))
    ok.append(("the intended system of the binary64 sections is within 1e-14 of the physical rows", not p5 and dev <= scale / 10 ** 14))
    bad = copy.deepcopy(ev)
    bad["frames"][0]["G_bits"] = bits(math.nextafter(float(g["meta"]["G"]), math.inf))
    bad["frames"][0]["G"] = math.nextafter(float(g["meta"]["G"]), math.inf)
    _, p6 = model_frames(m, [v], {(g["tag"], "typed", "SparseInteractive"): ([bad], {})})
    ok.append(("a resolved G one ulp off the reference's operand is a problem", any("Pa operands" in p for p in p6)))
    ok.append(("the judged groups without a probe group are named", any("no probe group" in p for p in p1)))
    bind = lambda ps: any("does not name one evaluation line" in p for p in ps)
    ok.append(("a groups line naming its evaluation, case and lambda_sigma binds", not bind(p1)))
    for name, kw in (("a wrong evaluation ID", {"eval_id": 999999999}), ("a wrong case label", {"case": "case:2"}),
                     ("a lambda_sigma of another evaluation", {"ls": "1" * 16})):
        ok.append((f"{name} on the groups line is a problem", bind(run_with(err * 4, **kw)[2])))
    for name, good in ok:
        print(("PASS " if good else "FAIL ") + name)
    print(f"bound census self-test: {sum(bool(x) for _, x in ok)} of {len(ok)} checks pass")
    if not all(x for _, x in ok):
        raise SystemExit(1)


def bound_main(args):
    if args == ["--selftest"]:
        selftest()
        return
    opts = {"--raw": [], "--dump": [], "--tags": None, "--workers": "6", "--hager": "typed", "--out": None}
    set_path, cur = None, None
    for a in args:
        if a in opts:
            cur = a
        elif cur in ("--raw", "--dump"):
            opts[cur].append(a)
        elif cur is not None:
            opts[cur] = a
            cur = None
        else:
            set_path = a
    tags = set(x.strip() for x in open(opts["--tags"]) if x.strip()) if opts["--tags"] else None
    s = bound(set_path, opts["--raw"], opts["--dump"], tags, int(opts["--workers"]), opts["--hager"])
    if opts["--out"]:
        with open(opts["--out"], "x") as f:
            json.dump(s, f, indent=1, sort_keys=True, default=str)
    for k in ("verdict", "n_failed", "models", "runs", "counts", "error_zero", "margin", "worst", "physical_information", "stress_rule", "stress_beyond_rule", "stress_information", "stress_beyond_4_ulps", "groups_lines_superseded", "evaluations",
              "eps_go", "eps_no_go", "eps_over_norm_max", "A1_held", "A1_failed", "A1_margin_min", "n_problems", "problems"):
        print(f"{k}: {s[k]}")
    if s["n_failed"] or s["n_problems"] or s["stress_beyond_rule"]:
        raise SystemExit(1)


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "bound":
        bound_main(sys.argv[2:])
    else:
        KCE.main()
