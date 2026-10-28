"""Every refutation round's attack (1-5) as a committed regression case of the M3 judge: the judging pass
(m3_report.py), run.sh and runner/build.sh, on synthetic records only. run.sh is driven with a synthetic builder
whose runners are Python scripts, so no product can run. Each case prints a TABLE line: round | attack | case |
outcome. Rounds 1-4 were found on the PF judge (pf_timoshenko_m3t/ at #1663) and are replayed on the shared judge's PF
view; round 5's attacks are replayed on the PF view and, with the exact route's required analogues, on the exact
view. Called from m3_selftest.py."""

from __future__ import annotations

import copy
import json
import os
import re
import subprocess
import sys
import tempfile

import m3_json
import m3_report as MR
import m3_synth as S
import m3_view as MV

HERE = os.path.dirname(os.path.abspath(__file__))
PF_SET = os.path.join(HERE, "selftest", "pf_set.jsonl")
EXACT_SET = os.path.normpath(os.path.join(HERE, "..", "kd5_timoshenko", "frozen", "kd5t_frozen_subset.jsonl"))
PASSED, SENSITIVE = "NUMERICAL_INTEGRITY_CHECKS_PASSED", "NUMERICAL_INTEGRITY_SENSITIVE"


def collection_scenarios(check):
    checkout = subprocess.run(["git", "-C", HERE, "rev-parse", "--show-toplevel"], capture_output=True, text=True).stdout.strip()
    head = subprocess.run(["git", "-C", HERE, "rev-parse", "HEAD"], capture_output=True, text=True).stdout.strip()
    check(f"the self-test's checkout {checkout} resolves HEAD to a commit", bool(re.fullmatch(r"[0-9a-f]{40}", head)))
    py = [sys.executable, "-I", "-B"]
    R = S.SYNTH_RUNNER
    pf_gens, _ = m3_json.read_jsonl(PF_SET)
    ex_gens, _ = m3_json.read_jsonl(EXACT_SET)
    by_tag = {x["tag"]: x for x in pf_gens}
    pick = lambda d, th="ts": next(g for g in pf_gens if g["meta"]["design"] == d and g["meta"]["placement"] == "o" and g["tag"].endswith("|" + th))
    codex, ordinary = pick("NCcodexS8"), pick("CANT03")
    refused_g = next(g for g in pf_gens if g["ref"]["expect"] == "refused")
    run = lambda g, e, m, route="pf", **kw: S.synthetic_run(route, g, entry=e, mode=m, head=head, **kw)
    EM = [(e, m) for e in MV.ENTRIES for m in MV.MODES]
    four = [run(codex, e, m) for e, m in EM]
    eight = four + [run(ordinary, e, m) for e, m in EM]
    bad = copy.deepcopy(four[0])
    for r in bad["results"]:
        if r["kind"] == "global_nodal_displacement_y" and r["entity_ref"] == "node:N1":
            r["value"] *= 0.444
    complete = [run(x, e, m) for x in pf_gens for e, m in EM]
    rows = []

    def table(rnd, attack, case, outcome, ok):
        rows.append((rnd, attack, case, outcome))
        print(f"TABLE {rnd} | {attack} | {case} | {outcome}", flush=True)
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
        withref = write(T("refused-set.jsonl"), [refused_g, codex])   # one family: the refused model beside an admitted one
        pf_set = write(T("pf-set.jsonl"), pf_gens)

        def judge(rnd, attack, case, st, files, want, why=lambda j: True, label="", route="pf", **kw):
            args = dict(quiet=True, workers=kw.pop("workers", 1), runner_sha=kw.pop("runner_sha", R), head=kw.pop("head", head),
                        checkout=kw.pop("checkout", checkout), **kw)
            j = MR.report(route, st, files, **args)
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
        c2 = lambda j: m3(j) and j.get("schema_invalid", 0) == 0
        design = lambda j: j["provenance_pass"] and j["inventory"]["pass"] and j["m3_pass"] and not j["design_pass"]
        vacuous = lambda j: j["provenance_pass"] and j["inventory"]["pass"] and j["m3_pass"] and not j["nonvacuity"]["pass"]
        sens = lambda r: S.set_standing(r, SENSITIVE)
        err_only = lambda r: [r.pop(k) for k in ("diagnostics", "results", "results_count", "numerical_quality", "summary", "contract_evidence",
                                                  "publication")] and r.update(error="synthetic runner failure")
        ev = lambda r: r["contract_evidence"]["preview_cases"][0]
        # ---- positive controls
        judge(0, "positive control", "small_positive", small, [write(T("small_positive.raw.jsonl"), eight)], True)
        judge(0, "positive control", "pf_selftest_set_positive_80", pf_set, [write(T("set.raw.jsonl"), complete)], True,
              lambda j: j["inventory"]["expected"] == 80, workers=4)
        judge(0, "positive control", "chunks_complete_positive (two raw files)", small, [write(T("a.raw.jsonl"), four), write(T("b.raw.jsonl"), eight[4:])], True)
        judge(4, "key order is not meaning", "key_order_positive", small,
              [write(T("ko.raw.jsonl"), "".join(json.dumps(dict(reversed(list(r.items())))) + "\n" for r in eight), raw=True)], True)
        judge(4, "an escaped key is the same key", "escaped_key_positive", small,
              [write(T("esc.raw.jsonl"), open(T("small_positive.raw.jsonl")).read().replace('"tag":', '"\\u0074ag":'), raw=True)], True)
        judge(4, "a twin control judged against the twins", "set_twin_control", pf_set, [T("set.raw.jsonl")], False, m3,
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
        judge(2, "a sidecar (mismatched) given as a raw input", "r2_sidecar_as_input", one, [s0, s1, side], False, schema, "refused by the view")
        judge(2, "split C4 with missing sidecars", "r2_split_c4_no_sidecars", one, [s0, write(T("s1b.raw.jsonl"), split[1])], False,
              lambda j: j["c4_failures"] == 2, "C4 = 2")
        judge_runs(2, "split C4 on the whole selftest set", "r2_set_c4", pf_set, [r for r in complete if r["tag"] != codex["tag"]] + split[0] + split[1],
                   False, lambda j: j["c4_failures"] == 2, "C4 = 2", workers=4)
        MR.report("pf", small, [T("small_positive.raw.jsonl")], quiet=True, workers=1, runner_sha=R, head=head, checkout=checkout,
                  eval_out=T("small.eval.jsonl"))
        evs = [m3_json.strict_loads(line) for line in open(T("small.eval.jsonl"))]
        pre_repair = mut(evs, lambda r: (r.pop("design_failures", None), r.update(info=dict(r.get("info") or {}, C10=1))))
        judge_runs(2, "a pre-repair evaluation cache given as input", "r2_pre_repair_cache_as_input", small, pre_repair, False, schema, "refused by the view")
        # ---- round 3
        rnull = lambda r: r.update(error=None, results=[], results_count=0)
        judge_runs(3, "null error, no rows", "r3_error_null_only", small, mut(eight[:1], rnull) + eight[1:], False, schema, "refused by the view")
        judge_runs(3, "null error on the 55.6%-low Passed record", "r3_error_null_bad_rows", small, [dict(bad, error=None)] + eight[1:], False, schema,
                   "refused by the view")
        judge_runs(3, "an error string beside results", "r3_error_string_plus_results", small, [dict(four[0], error="x")] + eight[1:], False, schema,
                   "refused by the view")
        judge_runs(3, "a runner error", "r3_error_string_only", small, mut(eight[:1], err_only) + eight[1:], False,
                   lambda j: j["runner_errors"] == 1 and inv(j), "C0, inventory")
        judge_runs(3, "a run with no outcome", "r3_no_outcome", small, mut(eight, lambda r: r.update(results=[], results_count=0, diagnostics=[])), False,
                   schema, "refused: no outcome")
        judge_runs(3, "every run a runner error", "r3_set_all_errors", pf_set, mut(complete, err_only), False,
                   lambda j: j["runner_errors"] == 80 and len(j["inventory"]["unsuccessful"]) == 80 and j["m3_failing_records"] == 80, "C0 x80", workers=4)
        judge_runs(3, "a forged pairing", "r3_unknown_pairing", small, mut(eight[:1], lambda r: r.update(pairing={"integrity": ["x"]})) + eight[1:],
                   False, schema, "refused by the view")
        for case, f in (("r3_replay_pairing_lie_rehashed", lambda r: r.update(pairing={"integrity": r.get("integrity"), "row_digest": "0" * 64})),
                        ("r3_replay_missing_verdict_failures", lambda r: r.pop("verdict_failures", None)),
                        ("r3_replay_null_design_failures", lambda r: r.update(design_failures=None)),
                        ("r3_replay_legacy_c4_currentstamp", lambda r: r.update(stamp={"format": "pft-m3t-judging/2"}))):
            judge_runs(3, "an evaluation record given as input", case, small, mut(evs, f), False, schema, "refused by the view")
        for case, f in (("r3_missing_clause_list", lambda r: r.pop("results")), ("r3_null_clause_list", lambda r: r.update(results=None))):
            judge_runs(3, "a missing or null clause list", case, small, mut(eight[:1], f) + eight[1:], False, schema, "refused by the view")
        changed = copy.deepcopy(codex)
        changed["request"]["model"]["nodes"][1]["position"]["x"] = 2.0
        judge_runs(3, "a changed request (L = 2 m) under the same tag", "r3_changed_L_hash", small,
                   [dict(r, request_sha256=MV.request_sha256(changed["tag"], changed["request"])) for r in four] + eight[4:], False, prov, "provenance")
        judge(3, "mixed runner", "r3_mixed_runner", small, [T("a.raw.jsonl"), write(T("b6.raw.jsonl"), [dict(r, runner_sha256="6" * 64) for r in eight[4:]])],
              False, prov, "provenance")
        judge(3, "mixed head", "r3_mixed_head", small, [T("a.raw.jsonl"), write(T("b7.raw.jsonl"), [dict(r, product_head="1" * 40) for r in eight[4:]])],
              False, prov, "provenance")
        judge(3, "--runner-sha256 differs", "r3_expect_runner_mismatch", small, [T("small_positive.raw.jsonl")], False, prov, "provenance", runner_sha="7" * 64)
        judge(3, "--product-head differs", "r3_expect_head_mismatch", small, [T("small_positive.raw.jsonl")], False, prov, "provenance", head="7" * 40)
        for case, h in (("r3_dirty", head + "-dirty"), ("r3_unknown", "unknown")):
            judge_runs(3, "a head that is not a commit", case, small, [dict(r, product_head=h) for r in eight], False, schema, "refused by the view")
        # ---- round 4
        for case, h in (("arbitrary_noncommit_head", "this-is-not-a-commit"), ("unicode_head_suffix", head + "-dirty\u200b")):
            judge_runs(4, "a head that is not 40 hex", case, small, [dict(r, product_head=h) for r in eight], False, lambda j: schema(j) and prov(j),
                       "refused and provenance", head=h)
        judge_runs(4, "a 40-hex head that is not a commit", "nonexistent_hex_head", small, [dict(r, product_head="0" * 40) for r in eight], False, prov,
                   "provenance: not a commit", head="0" * 40)
        judge(4, "a head without a checkout", "head_without_checkout", small, [T("small_positive.raw.jsonl")], False, prov, "provenance",
              checkout=T("no-such-dir"))
        judge(4, "no --runner-sha256", "missing_runner_sha", small, [T("small_positive.raw.jsonl")], False, prov, "provenance", runner_sha=None)
        judge_runs(4, "identical forged runner sha256 at the real head", "forged_identical_provenance", small, [dict(r, runner_sha256="f" * 64) for r in eight],
                   True, lambda j: True, "outside the model: assertions", runner_sha="f" * 64)
        refusal = lambda code: (lambda r: r.update(diagnostics=[S.diag(code, "blocking")], results=[], results_count=0, summary=None, contract_evidence=None))
        for case, f in (("unknown_integrity_code_bad_rows", lambda r: S.set_standing(r, "NUMERICAL_INTEGRITY_NOT_A_STANDING")),
                        ("sensitive_before_passed_bad_rows", lambda r: r["diagnostics"].insert(0, S.diag(SENSITIVE, "warning", n=1))),
                        ("passed_before_sensitive_bad_rows", lambda r: r["diagnostics"].append(S.diag(SENSITIVE, "warning", n=1))),
                        ("standing_naming_another_case", lambda r: r.update(diagnostics=[S.diag(PASSED, "info", case="case:2")])),
                        ("refusal_plus_standing", lambda r: r["diagnostics"].append(S.diag(MV.PF_REFUSAL_CODE, "blocking"))),
                        ("unrecognised_refusal_code", refusal("SOMETHING_BLOCKING")),
                        ("two_refusal_codes", lambda r: r.update(diagnostics=[S.diag(MV.PF_REFUSAL_CODE, "blocking"),
                                                                              S.diag("PF_TIMOSHENKO_PARTIAL_SPAN_UNSUPPORTED", "blocking")],
                                                                 results=[], results_count=0, summary=None, contract_evidence=None))):
            judge_runs(4, "contradictory or unknown standing or refusal", case, small, mut([bad] + eight[1:], f), False, schema, "refused by the view")
        judge_runs(4, "every run Sensitive, bad rows", "bad_rows_sensitive_only", small, mut([bad] + eight[1:], sens), False, vacuous,
                   "nonvacuity: 0 judged Passed groups (round 5)")
        judge_runs(4, "a standing with no rows", "sensitive_no_results", small, mut(eight, lambda r: (sens(r), r.update(results=[], results_count=0))), False,
                   schema, "refused by the view")

        def swap(xs):
            for i in range(4):
                for k in MV.RAW_SUCCESS - MV.RAW_COMMON:
                    xs[i][k], xs[i + 4][k] = copy.deepcopy(xs[i + 4][k]), copy.deepcopy(xs[i][k])
            return xs
        judge_runs(4, "another model's content under a matching hash", "different_model_content_matching_hash", small, swap(copy.deepcopy(eight)), False,
                   schema, "refused: misbound rows")
        judge_runs(4, "another model's content, Sensitive", "different_model_content_sensitive", small, swap(mut(eight, sens)), False, schema,
                   "refused: misbound rows")
        twin_content = []
        for r in eight:
            t = run(by_tag[by_tag[r["tag"]]["meta"]["twin"]], r["entry"], r["mode"])
            twin_content.append(dict(r, **{k: t[k] for k in MV.RAW_SUCCESS - MV.RAW_COMMON}))
        judge_runs(4, "the twin's rows under the own hash", "mixed_theory_content_own_hash", small, twin_content, False, m3, "M3")
        for case, f in (("unicode_confusable_key", lambda r: r.update({"t\u0430g": r.pop("tag")})), ("bool_ns", lambda r: r.update(ns=True)),
                        ("bool_row_value", lambda r: r["results"][0].update(value=True)), ("raw_with_ref_variant", lambda r: r.update(ref_variant="twin")),
                        ("nested_summary_arbitrary", lambda r: r.update(summary={"arbitrary": True})),
                        ("nested_headline_wrong_type", lambda r: r["summary"].update(max_open_formula_stress="high")),
                        ("nested_evidence_wrong_type", lambda r: r.update(contract_evidence={"combination_gates": [], "preview_cases": "not-an-array"})),
                        ("nested_extrema_wrong_type", lambda r: ev(r)["pipe_stress_extrema"][0].update(value_upper_pa="1")),
                        ("evidence_of_another_pipe", lambda r: ev(r)["pipe_stress_extrema"][0].update(pipe_id="pipe:ZZ"))):
            judge_runs(4, "a malformed nested field", case, small, mut(eight[:1], f) + eight[1:], False, schema, "refused by the view")
        add_solver = lambda **kw: (lambda r: (r["results"].append(S.solver_row(**kw)), r.update(results_count=len(r["results"]))))
        judge_runs(0, "positive control: the product's solver-mode row", "solver_row_genuine", small, mut(eight, add_solver()), True)
        judge_runs(4, "a solver-mode row naming another entity", "solver_row_other_entity", small, mut(eight[:1], add_solver(entity="solver:other")) + eight[1:],
                   False, schema, "refused: misbound row")
        nan = write(T("nan.raw.jsonl"), json.dumps(dict(four[0], results=[dict(four[0]["results"][0], value=float("nan"))] + four[0]["results"][1:])) + "\n"
                    + "".join(json.dumps(r) + "\n" for r in eight[1:]), raw=True)
        judge(4, "a NaN token", "nan_row_value", small, [nan], False, prov, "strict JSON: refused")
        dd = write(T("dd.raw.jsonl"), "".join('{"diagnostics":' + json.dumps([S.diag(PASSED, "info")]) + ","
                                              + json.dumps(dict(r, diagnostics=[S.diag(SENSITIVE, "warning")]))[1:] + "\n" for r in [bad] + eight[1:]), raw=True)
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
            j = MR.report("pf", ref_p, [raw_p], quiet=True, workers=1, runner_sha=R, head=head, checkout=checkout, **kw)
            kept = all(open(p, "rb").read() == b for p, b in before.items()) and (target != "existing" or open(alias).read() == "keep")
            table(4, "an output aliasing an input or an existing file", case, "OVERALL: FAIL (outputs refused, inputs intact)",
                  not j["pass"] and bool(j.get("outputs_refused")) and kept)
        # ---- round 5 (Codex at decb1dbb61): the validator and the comparison read different data; and its earlier-kind variants
        def low_upper(r):
            e = ev(r)["pipe_stress_extrema"][0]
            e["value_upper_pa"] = 0.0
            e["value_lower_pa"] = 0.0
        judge_runs(5, "an upper bound of 0 on its own", "low_upper_alone", small, mut(eight, low_upper), False, c2, "M3: C2")
        for field in ("intensified_measures", "stress_maximum_coverage", "support_attribution"):
            def shadow(r, field=field):
                good = copy.deepcopy(ev(r)["pipe_stress_extrema"][0])
                low_upper(r)
                ev(r)[field] = {"nested": [good]}
            judge_runs(5, "a good extremum shadowed in a nested member, the read one 0", "shadow_evidence_" + field, small, mut(eight, shadow), False, schema,
                       "refused by the view: member shape")

        def shadow_unread(r):
            good = copy.deepcopy(ev(r)["pipe_stress_extrema"][0])
            low_upper(r)
            ev(r)["support_attribution"]["withheld"] = [good]
        judge_runs(5, "a good extremum in a well-typed member the view does not read", "shadow_evidence_unread_member", small, mut(eight, shadow_unread), False,
                   c2, "M3: C2; the comparison never sees it")

        def zero_max(r):
            ev(r)["pipe_stress_extrema"] = []
            for x in r["results"]:
                if x["kind"] == MV.MAXIMUM:
                    x["value"] = 0.0
            S.refresh_headlines(r, "pf")
        judge_runs(5, "no evidence, maxima published as 0", "missing_evidence_low_max", small, mut(eight, zero_max), False, c2, "M3: C2")

        def unique_shadow(r, where):
            good = copy.deepcopy(ev(r)["pipe_stress_extrema"][0])
            zero_max(r)
            if where == "nested":
                ev(r)["support_attribution"] = {"nested": [good]}
            else:
                ev(r)["support_attribution"]["withheld"] = [good]
        judge_runs(5, "the only good extremum shadowed (no duplicate identity at all)", "unique_shadow", small, mut(eight, lambda r: unique_shadow(r, "nested")),
                   False, schema, "refused by the view: member shape")
        judge_runs(5, "the only good extremum in an unread member", "unique_shadow_unread", small, mut(eight, lambda r: unique_shadow(r, "withheld")),
                   False, c2, "M3: C2")

        def duplicate_evidence(r, reverse=False):
            es = ev(r)["pipe_stress_extrema"]
            good = copy.deepcopy(es[0])
            low_upper(r)
            es.append(good)
            if reverse:
                es.reverse()
        judge_runs(5, "a duplicate extremum identity, 0 then good", "duplicate_evidence_bad_good", small, mut(eight, duplicate_evidence), False, schema,
                   "refused by the view: duplicate extremum")
        judge_runs(5, "a duplicate extremum identity, good then 0", "duplicate_evidence_good_bad", small, mut(eight, lambda r: duplicate_evidence(r, True)),
                   False, schema, "refused by the view: duplicate extremum")

        def duplicate_case(r):
            cs = r["contract_evidence"]["preview_cases"]
            good = copy.deepcopy(cs[0])
            low_upper(r)
            cs.append(good)
        judge_runs(5, "a duplicate evidence case, 0 then good", "duplicate_case_evidence_bad_good", small, mut(eight, duplicate_case), False, schema,
                   "refused by the view: two case objects")

        def uy_unit(u):
            def f(r):
                for x in r["results"]:
                    if x["kind"] == "global_nodal_displacement_y" and x["entity_ref"] == "node:N1":
                        x["unit"] = u
            return f
        judge_runs(5, "UY published in m (the contract's unit is mm)", "dimensionally_wrong_uy_unit", small, mut(eight, uy_unit("m")), False, schema,
                   "refused by the view: unit")
        judge_runs(5, "a unit that is an object", "public_units_object", small, mut(eight, uy_unit({})), False, schema, "refused by the view")
        judge_runs(5, "a headline unit that is an object", "headline_unit_object", small,
                   mut(eight, lambda r: r["summary"]["max_open_formula_stress"].update(unit={})), False, schema, "refused by the view")
        judge_runs(5, "a headline in MPa", "headline_unit_mpa", small, mut(eight, lambda r: r["summary"]["max_open_formula_stress"].update(unit="MPa")), False,
                   schema, "refused by the view")

        def inverted(r):
            for e in ev(r)["pipe_stress_extrema"]:
                e["value_lower_pa"] = 2 * e["value_upper_pa"] + 1.0
        judge_runs(5, "an inverted enclosure (lower > upper)", "inverted_extrema_interval", small, mut(eight, inverted), False, schema,
                   "refused by the view: lo > hi")

        def contradiction(r):
            x = next(x for x in r["results"] if x["kind"] == "global_nodal_displacement_y")
            x["metadata"] = dict(x["metadata"], location="different-location", component="UZ", coordinate_system="element_local")
        judge_runs(5, "row metadata contradicting the row", "row_metadata_contradiction", small, mut(eight, contradiction), False, schema,
                   "refused by the view")
        judge_runs(5, "a row repeated whole (same id, same value)", "identical_row_repeated", small,
                   mut(eight, lambda r: (r["results"].append(copy.deepcopy(r["results"][0])), r.update(results_count=len(r["results"])))), False, schema,
                   "refused by the view: duplicate")

        def other_frame(r):
            x = next(x for x in r["results"] if x["kind"] == "element_local_bending_moment_z")
            x["metadata"]["coordinate_system"] = "global"
        judge_runs(5, "a judged row in another frame", "row_frame_contradiction", small, mut(eight, other_frame), False, schema, "refused by the view")
        for code in sorted(MV.STANDINGS):
            short = code.replace("NUMERICAL_INTEGRITY_", "")
            judge_runs(5, "every run of one standing", "standing_" + short, small, mut(eight, lambda r, code=code: S.set_standing(r, code)), code == PASSED,
                       (lambda j: j["pass"]) if code == PASSED else vacuous, "" if code == PASSED else "nonvacuity: 0 judged Passed groups")
            judge_runs(5, "a standing given twice", "double_standing_" + short, small,
                       mut(eight, lambda r, code=code: r["diagnostics"].append(S.diag(code, "warning", n=9))), False, schema, "refused by the view")
        ref_runs = [run(refused_g, e, m) for e, m in EM] + four
        for code in sorted(MV.REFUSALS["pf"]):
            ok = code == MV.PF_REFUSAL_CODE
            judge_runs(5, "an expected refusal under each recognised code", "refusal_" + code, withref,
                       mut(ref_runs[:4], refusal(code)) + ref_runs[4:], ok, (lambda j: j["pass"]) if ok else design,
                       "" if ok else "design conformance: C10")
            judge_runs(5, "the same refusal given twice (distinct ids)", "same_refusal_code_twice_" + code, withref,
                       mut(ref_runs[:4], lambda r, code=code: r.update(diagnostics=[S.diag(code, "blocking"), S.diag(code, "blocking", n=1)])) + ref_runs[4:],
                       False, schema, "refused by the view")
        for loc in ("row", "evidence", "headline", "unread"):
            text = []
            for r in eight:
                x = copy.deepcopy(r)
                if loc == "row":
                    x["results"][0]["value"] = "__DUP__"
                elif loc == "evidence":
                    ev(x)["pipe_stress_extrema"][0]["value_lower_pa"] = "__DUP__"
                elif loc == "headline":
                    x["summary"]["max_open_formula_stress"]["value"] = "__DUP__"
                else:
                    x["numerical_quality"] = {"outer": {"value": "__DUP__"}}
                text.append(json.dumps(x).replace('"value": "__DUP__"', '"value": 0, "value": 1')
                            .replace('"value_lower_pa": "__DUP__"', '"value_lower_pa": 0, "value_lower_pa": 1') + "\n")
            judge(5, "a duplicate JSON key", "duplicate_json_" + loc, small, [write(T("dj-" + loc + ".raw.jsonl"), "".join(text), raw=True)], False, prov,
                  "strict JSON: refused")
        for field, value in (("kind", "global_nodal_displacement_y"), ("entity_ref", "solver:other"), ("case", "case:other")):
            def malformed(r, field=field, value=value):
                add_solver()(r)
                r["results"][-1][field] = value
            judge_runs(5, "a malformed solver-mode row", "solver_wrong_" + field, small, mut(eight[:1], malformed) + eight[1:], False, schema,
                       "refused by the view")
        for case, f, why, label in (
                ("summary_null", lambda r: r.update(summary=None), schema, "refused by the view"),
                ("headline_list", lambda r: r["summary"].update(max_open_formula_stress=[]), schema, "refused by the view"),
                ("headline_bool", lambda r: r["summary"]["max_open_formula_stress"].update(value=True), schema, "refused by the view"),
                ("headline_other_row", lambda r: r["summary"]["max_open_formula_stress"].update(result_ref=r["results"][0]["id"]), schema,
                 "refused by the view: result_ref"),
                ("evidence_upper_bool", lambda r: ev(r)["pipe_stress_extrema"][0].update(value_upper_pa=True), schema, "refused by the view"),
                ("evidence_section_basis_list", lambda r: ev(r)["pipe_stress_extrema"][0].update(section_basis=[]), schema, "refused by the view"),
                ("evidence_upper_null", lambda r: ev(r)["pipe_stress_extrema"][0].update(value_upper_pa=None), c2, "M3: C2"),
                ("evidence_lower_string", lambda r: ev(r)["pipe_stress_extrema"][0].update(value_lower_pa="0"), schema, "refused by the view"),
                ("evidence_result_id_other_row", lambda r: ev(r)["pipe_stress_extrema"][0].update(result_id=r["results"][0]["id"]), schema,
                 "refused by the view: result_id"),
                ("unknown_standing_prefix", lambda r: S.set_standing(r, "NUMERICAL_INTEGRITY"), schema, "refused by the view")):
            judge_runs(5, "a malformed or unbound field", case, small, mut(eight, f), False, why, label)
        judge_runs(5, "one family all Sensitive while another passes", "nonvacuity_one_family_sensitive", small, eight[:4] + mut(eight[4:], sens), False,
                   lambda j: vacuous(j) and j["nonvacuity"]["vacuous_families"] == ["cantilever"], "nonvacuity: cantilever has 0 judged Passed groups")
        saved = dict(MR.EXEMPT["pf"])
        MR.EXEMPT["pf"]["cantilever"] = "self-test of the exemption mechanism"
        try:
            judge_runs(5, "the same with that family named exempt (mechanism only)", "nonvacuity_exempt_family_mechanism", small,
                       eight[:4] + mut(eight[4:], sens), True, lambda j: j["nonvacuity"]["pass"], "the exemption applies only to the named family")
        finally:
            MR.EXEMPT["pf"] = saved
        # ---- round 5 on the exact view: Agent 0's required three and round 5's four
        pres = next(g for g in ex_gens if g["meta"]["family"] == "pressure" and g["meta"]["section_variant"] == "both")
        plain = next(g for g in ex_gens if g["meta"]["family"] == "cantilever" and g["meta"]["section_variant"] == "zero")
        ex_set = write(T("exact-set.jsonl"), [pres, plain] + [next(g for g in ex_gens if g["tag"] == x["meta"]["twin"]) for x in (pres, plain)])
        ex_own = write(T("exact-own.jsonl"), [pres, plain])
        xe = [run(g, e, m, "exact") for g in (pres, plain) for e, m in EM]
        xev = lambda r: r["contract_evidence"]["exact_cases"][0]
        jx = lambda *a, **kw: judge_runs(*a, route="exact", **kw)
        jx(0, "exact view: positive control", "exact_positive", ex_own, xe, True)
        jx(0, "exact view: positive control with the solver-mode rows", "exact_solver_rows", ex_own, mut(xe, add_solver()), True)
        jx(5, "exact view: a null error key", "exact_error_null", ex_own, [dict(xe[0], error=None)] + xe[1:], False, schema, "refused by the view")
        jx(5, "exact view: a missing run", "exact_missing_run", ex_own, xe[1:], False, inv, "inventory")
        xbad = copy.deepcopy(xe[0])
        top = max((r for r in xbad["results"] if r["kind"].startswith("global_nodal_displacement_")), key=lambda r: abs(r["value"]))
        top["value"] *= 0.5
        jx(5, "exact view: a duplicate run (bad then good)", "exact_duplicate_run", ex_own, [xbad] + xe, False, inv, "inventory: duplicate")
        jx(5, "exact view: the largest displacement halved on a Passed run", "exact_bad_displacement", ex_own, [xbad] + xe[1:], False, m3, "M3: C3")

        def x_low(r):
            e = xev(r)["pipe_stress_extrema"][0]
            e["value_upper_pa"] = e["value_lower_pa"] = 0.0

        def x_shadow(r):
            good = copy.deepcopy(xev(r)["pipe_stress_extrema"][0])
            x_low(r)
            r["contract_evidence"]["pressure"].append({"nested": [good]})
        jx(5, "exact view: a good extremum in an unread member, the read one 0", "exact_shadow_evidence_unread", ex_own, mut(xe, x_shadow), False, c2,
           "M3: C2; the comparison never sees it")

        def x_dup(r):
            es = xev(r)["pipe_stress_extrema"]
            good = copy.deepcopy(es[0])
            x_low(r)
            es.append(good)
        jx(5, "exact view: a duplicate extremum identity", "exact_duplicate_evidence", ex_own, mut(xe, x_dup), False, schema,
           "refused by the view: duplicate extremum")

        def x_unit(r):
            for x in r["results"]:
                if x["kind"] == "global_nodal_displacement_y":
                    x["unit"] = "m"
        jx(5, "exact view: UY in m", "exact_unit_m", ex_own, mut(xe, x_unit), False, schema, "refused by the view: unit")

        def x_stress_unit(r):
            for x in r["results"]:
                if x["kind"] == "pipe_axial_membrane_stress_v2":
                    x["unit"] = "MPa"
        jx(5, "exact view: wall membrane stress in MPa (the contract's unit is Pa)", "exact_membrane_unit_mpa", ex_own, mut(xe, x_stress_unit), False, schema,
           "refused by the view: unit")

        def x_inv(r):
            for e in xev(r)["pipe_stress_extrema"]:
                e["value_lower_pa"] = 2 * e["value_upper_pa"] + 1.0
        jx(5, "exact view: an inverted enclosure", "exact_inverted_interval", ex_own, mut(xe, x_inv), False, schema, "refused by the view: lo > hi")
        jx(5, "exact view: every run Sensitive", "exact_sensitive_only", ex_own, mut(xe, sens), False, vacuous, "nonvacuity")
        judge(5, "exact view: the twin control", "exact_twin_control", ex_set,
              [write(T("exact-twin.raw.jsonl"), xe + [run(next(g for g in ex_gens if g["tag"] == x["meta"]["twin"]), e, m, "exact")
                                                       for x in (pres, plain) for e, m in EM])], False, m3,
              "M3 fails, provenance and inventory pass: the control behaves", route="exact", variant="twin")
        ctl = write(T("exact-controls.jsonl"), [{"tag": plain["tag"], "pertE": next(g for g in ex_gens if g["tag"] == plain["meta"]["twin"])["ref"]}])
        judge(5, "exact view: a control reference (one model has a control)", "exact_control_variant", ex_own, [write(T("exact-ctl.raw.jsonl"), xe)], False,
              lambda j: m3(j) and j["set_aside"] == 4 and j["inventory"]["expected"] == 4, "M3 fails; the model without a control is set aside",
              route="exact", variant="control", controls_path=ctl)
        jx(5, "exact view: the PF route's evidence shape", "exact_with_pf_evidence", ex_own,
           mut(xe, lambda r: r.update(contract_evidence={"combination_gates": [], "preview_cases": r["contract_evidence"]["exact_cases"]})), False, schema,
           "refused by the view")
        judge_runs(5, "PF view: exact-route records", "pf_with_exact_records", small,
                   [dict(x, tag=t["tag"], request_sha256=MV.request_sha256(t["tag"], t["request"])) for x, t in zip(xe, [codex] * 4 + [ordinary] * 4)],
                   False, schema, "refused by the view")
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
        open(builder, "w").write('#!/bin/bash\nset -eu\ncp "$M3J_FAKE_RUNNER" "$3"\nchmod +x "$3"\n')
        os.chmod(builder, 0o755)
        good, badr, exit7 = runner("good", eight), runner("bad", [bad] + eight[1:]), runner("exit7", eight, 7)
        twins = [by_tag[codex["meta"]["twin"]], by_tag[ordinary["meta"]["twin"]]]
        good4 = runner("good4", eight + [run(t, e, m) for t in twins for e, m in EM])
        sd, sd4, sdx = T("driver-set"), T("driver-set4"), T("driver-setx")
        write(os.path.join(sd, "set.jsonl"), [codex, ordinary])
        write(os.path.join(sd4, "set.jsonl"), [codex, ordinary] + twins)
        write(os.path.join(sdx, "set.jsonl"), [pres, plain])
        goodx = runner("goodx", xe)

        def driver(rnd, attack, case, runner_bin, rundir, chunks, variants, want, setdir=sd, why=lambda d: True, route="pf"):
            for i, tags in enumerate(chunks):
                write(os.path.join(rundir, "chunks", f"c.{i:03d}.in.jsonl"), MV.request_lines(tags), raw=True)
            env = dict(os.environ, M3J_PY=sys.executable, M3J_WORKERS="1", M3J_BUILD=builder, M3J_FAKE_RUNNER=runner_bin)
            d = subprocess.run(["bash", os.path.join(HERE, "run.sh"), route, checkout, setdir, rundir, *variants], capture_output=True, text=True, env=env)
            last = d.stdout.strip().splitlines()[-1] if d.stdout.strip() else ""
            table(rnd, attack, case, f"run.sh exit {d.returncode}, final line '{last}'",
                  (d.returncode == 0) == want and last == ("OVERALL: PASS" if want else "OVERALL: FAIL") and why(rundir))

        driver(0, "positive control", "driver_positive", good, T("d/positive"), [[codex], [ordinary]], ["own"], True)
        driver(0, "positive control", "driver_positive_with_twin_control", good4, T("d/twin-ok"), [[codex, ordinary], twins], ["own", "twin"], True, sd4)
        driver(0, "exact view: positive control", "driver_exact_positive", goodx, T("d/exact"), [[pres], [plain]], ["own"], True, sdx, route="exact")
        driver(1, "a model missing from the chunks", "driver_missing_model", good, T("d/missing"), [[codex]], ["own"], False)
        driver(1, "a chunk given twice", "driver_duplicate_across_chunks", good, T("d/dup"), [[codex], [codex], [ordinary]], ["own"], False)
        driver(3, "a changed request under the same tag", "driver_changed_request", good, T("d/changed"), [[changed, ordinary]], ["own"], False)
        driver(3, "a replaced runner", "driver_resume_good_then_bad_runner (good first)", good, T("d/resume"), [[codex], [ordinary]], ["own"], True)
        driver(3, "a replaced runner", "driver_resume_bad_new_runner", badr, T("d/resume"), [[codex], [ordinary]], ["own"], False)
        write(os.path.join(T("d/partial"), "out", "c.000.out.jsonl"), [bad])
        driver(4, "a partial earlier output", "driver_recover_good_clear_partial", good, T("d/partial"), [[codex], [ordinary]], ["own"], True,
               why=lambda d: sorted(os.listdir(os.path.join(d, "out"))) == ["c.000.out.jsonl", "c.001.out.jsonl"])
        write(os.path.join(T("d/partial2"), "out", "c.007.out.jsonl"), [bad])
        driver(4, "a stale output from an earlier chunking", "driver_stale_out_removed", good, T("d/partial2"), [[codex], [ordinary]], ["own"], True,
               why=lambda d: not os.path.exists(os.path.join(d, "out", "c.007.out.jsonl")))
        driver(4, "a runner that exits 7 after valid records", "driver_valid_records_exit_nonzero", exit7, T("d/exit7"), [[codex], [ordinary]], ["own"], False)
        driver(4, "a twin control that cannot complete", "driver_twin_runtime_error_accepted", good, T("d/twin-bad"), [[codex], [ordinary]], ["own", "twin"], False)
        driver(4, "twin only, no own judgement", "driver_twin_only_no_own", good, T("d/twin-only"), [[codex], [ordinary]], ["twin"], False)
        driver(5, "a control variant on the PF route", "driver_pf_control_refused", good, T("d/pf-ctl"), [[codex], [ordinary]], ["own", "control"], False)
        driver(5, "an unknown route", "driver_unknown_route", good, T("d/route"), [[codex], [ordinary]], ["own"], False, route="preview")
        # build.sh: a dirty build input refuses the build before cargo runs (a throwaway repository; cargo is `false`)
        repo = T("build-repo")
        rel = os.path.join("projects", "chirality-piping")
        rr = os.path.join(rel, "validation", "references", "m3_judge", "runner")
        for f in (os.path.join(rel, "core", "product_physics", "Cargo.toml"), os.path.join(rel, "fixtures", "x.json"), os.path.join(rel, "schemas", "x.json"),
                  os.path.join(rr, "Cargo.toml"), os.path.join(rr, "Cargo.lock"), os.path.join(rr, "build.rs"), os.path.join(rel, "apps", "x.txt")):
            write(os.path.join(repo, f), "x\n", raw=True)
        gg = lambda *a: subprocess.run(["git", "-C", repo, *a], capture_output=True, text=True)
        gg("init", "-q")
        gg("add", "-A")
        gg("-c", "user.name=selftest", "-c", "user.email=selftest@invalid", "commit", "-q", "-m", "x")
        benv = dict(os.environ, M3J_CARGO="false")
        for case, path, want in (("build_clean (cargo then refuses: false)", None, 1), ("build_dirty_core", os.path.join(rel, "core", "new.rs"), 3),
                                 ("build_dirty_runner_lock", os.path.join(rr, "Cargo.lock"), 3),
                                 ("build_dirty_fixtures", os.path.join(rel, "fixtures", "x.json"), 3), ("build_apps_not_an_input", os.path.join(rel, "apps", "x.txt"), 1)):
            gg("checkout", "-q", "--", ".")
            gg("clean", "-qfd")
            if path:
                write(os.path.join(repo, path), "changed\n", raw=True)
            b = subprocess.run(["bash", os.path.join(HERE, "runner", "build.sh"), repo, T("bt"), T("bt.bin")], capture_output=True, text=True, env=benv)
            table(4, "the runner built from a dirty input", case, f"build.sh exit {b.returncode}" + (" (refused: dirty input)" if b.returncode == 3 else ""),
                  b.returncode == want and not os.path.exists(T("bt.bin")))
        # the CLI: one final OVERALL line, exit 0 only on PASS; usage and internal errors fail by name
        for case, argv, rc, route in (("cli_passing", [small, T("small_positive.raw.jsonl")], 0, "pf"),
                                      ("cli_failing", [small, T("bad_uy_55_6_percent_low.raw.jsonl")], 1, "pf"),
                                      ("cli_bad_rows_sensitive_only", [small, T("bad_rows_sensitive_only.raw.jsonl")], 1, "pf"),
                                      ("cli_exact_passing", [ex_own, T("exact_positive.raw.jsonl")], 0, "exact"),
                                      ("cli_missing_mandatory_flags", [small, T("small_positive.raw.jsonl"), "--no-provenance"], 2, "pf"),
                                      ("cli_missing_route", [small, T("small_positive.raw.jsonl")], 2, None)):
            flags = ["--runner-sha256", R, "--product-head", head, "--checkout", checkout] if case != "cli_missing_mandatory_flags" else []
            c = subprocess.run(py + [os.path.join(HERE, "m3_report.py"), *(["--route", route] if route else []), *argv, "--workers", "1", *flags],
                               capture_output=True, text=True)
            last = c.stdout.strip().splitlines()[-1] if c.stdout.strip() else ""
            rnd = 5 if ("sensitive" in case or "route" in case) else (4 if rc == 2 else 0)
            table(rnd, "the CLI's final line and exit", case, f"exit {c.returncode}, final line '{last}'",
                  c.returncode == rc and last.startswith("OVERALL: PASS" if rc == 0 else "OVERALL: FAIL") and "Traceback" not in c.stderr)
    by_round = {}
    for r in rows:
        by_round[r[0]] = by_round.get(r[0], 0) + 1
    print("TABLE counts by round:", dict(sorted(by_round.items())))
    check(f"{len(rows)} regression cases across rounds 1-5 and the positive controls ({dict(sorted(by_round.items()))})", len(rows) >= 150)
