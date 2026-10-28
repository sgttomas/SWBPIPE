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
import shutil
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
RETAINED = "retained_source_endpoint_normal_max_v1"
# A committed pressure-source-1 record of an ordinary model whose maxima are retained-source endpoint maxima
RECOVERY_TEMPLATE = "pressure_source_plain_n05_sparse.stress_neutral.json"
READ_EXTREMUM = ("pipe_id", "result_id", "section_basis", "value_lower_pa", "value_upper_pa")


def mutate_both(runs, *fs):
    xs = copy.deepcopy(runs)
    for r in xs:
        for f in fs:
            f(r)
    return xs


def source_recovery(r):
    """A synthetic exact record as pressure-source-1 publishes it: retained-source maximum rows and the recovery evidence
    of an ordinary model (physics_source_recovery.schema.json). The members the judge reads (READ_EXTREMUM, and each case's
    load_case_id and coverage) come from the record; every other member (value_pa, endpoints, the error bounds, the
    materials, sections and recovery method) from the committed template."""
    fx, probs = m3_json.read_json(os.path.join(MV.FIXTURES, RECOVERY_TEMPLATE))
    assert not probs, probs
    tc = fx["contract_evidence"]["exact_cases"][0]
    te = next(e for e in tc["pipe_stress_extrema"] if e.get("basis") == RETAINED)
    for x in r["results"]:
        if x["kind"] == MV.MAXIMUM:
            x["metadata"]["basis"] = f"{RETAINED}; section_basis={MV._section_basis(x['metadata']['basis'])}"
    cases = []
    for c in r["contract_evidence"]["exact_cases"]:
        case = {k: copy.deepcopy(tc[k]) for k in ("material_basis", "pipe_materials", "pipe_sections", "pressure_rhs_assembly", "profile_mode",
                                                  "recovery_method")}
        case.update(load_case_id=c["load_case_id"], stress_maximum_coverage=copy.deepcopy(c["stress_maximum_coverage"]), pipe_stress_extrema=[])
        for e in c["pipe_stress_extrema"]:
            x = copy.deepcopy(te)
            x.update({k: e[k] for k in READ_EXTREMUM}, load_case_id=c["load_case_id"])
            case["pipe_stress_extrema"].append(x)
        cases.append(case)
    r["contract_evidence"] = {"connector": [], "exact_cases": cases, "pressure": []}
    return r


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

        def pins(route, st, controls=None):
            """The test's own synthetic set (and controls) declared as committed references: the judging library's
            `references` argument. The CLI and run.sh always read the committed REFERENCE_SHA256SUMS."""
            out = {(route, "set"): {MV.file_sha256(st): "synthetic"}}
            if controls:
                out[(route, "controls")] = {MV.file_sha256(controls): "synthetic"}
            return out

        def judge(rnd, attack, case, st, files, want, why=lambda j: True, label="", route="pf", **kw):
            refs = kw.pop("references", "test")
            args = dict(quiet=True, workers=kw.pop("workers", 1), runner_sha=kw.pop("runner_sha", R), head=kw.pop("head", head),
                        checkout=kw.pop("checkout", checkout), **kw)
            args["references"] = pins(route, st, args.get("controls_path")) if refs == "test" else refs
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
                  eval_out=T("small.eval.jsonl"), references=pins("pf", small))
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
            j = MR.report("pf", ref_p, [raw_p], quiet=True, workers=1, runner_sha=R, head=head, checkout=checkout, references=pins("pf", ref_p), **kw)
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
        # ---- the final round (Codex at cd94a8389b): the reference side bound (Agent 0), null first, exact types
        for route, models, runs, bad_runs in (("pf", [codex, ordinary], eight, [bad] + eight[1:]), ("exact", [pres, plain], xe, [xbad] + xe[1:])):
            P = route + "_"

            def edited(f, models=models):
                gs = copy.deepcopy(models)
                f(gs[0])
                return gs

            def far_node(g):
                g["ref"]["nodes"][1] = [g["ref"]["nodes"][1][0], ["1e20", "1e20", "1e20"]]

            def dup_node(g):
                g["ref"]["nodes"].append([g["ref"]["nodes"][0][0], ["1e20", "1e20", "1e20"]])
            for case, f in (("reference_node_1e20", far_node), ("reference_duplicate_node", dup_node)):
                st = write(T(P + case + ".set.jsonl"), edited(f))
                judge(6, f"{route}: a reference node moved to 1e20 or duplicated there (it set L_b), with a C3 defect", P + case, st,
                      [write(T(P + case + ".raw.jsonl"), bad_runs)], False, m3, "M3: C3; ref.nodes are not read, L_b is the request's", route=route)
                judge(6, f"{route}: the same reference, not committed", P + case + "_not_committed", st, [T(P + case + ".raw.jsonl")], False,
                      lambda j: prov(j) and any("is not a committed" in p for p in j["provenance"]), "provenance: reference file not committed",
                      route=route, references=None)

            def req_far(g):
                g["request"]["model"]["nodes"][1]["position"] = {"x": 1e20, "y": 1e20, "z": 1e20}

            def req_dup(g):
                g["request"]["model"]["nodes"].append(copy.deepcopy(g["request"]["model"]["nodes"][0]))

            def req_ghost_end(g):
                g["request"]["model"]["pipe_segments"][0]["to"] = "node:ghost"

            def req_ghost_support(g):
                g["request"]["model"]["supports"][0]["node"] = "node:ghost"

            def req_shared_id(g):
                g["request"]["model"]["supports"][0]["id"] = g["request"]["model"]["nodes"][0]["id"]

            def ref_ghost_row(g):
                row = next(w for w in g["ref"]["cases"]["case:1"]["rows"] if w[0] == "element_local_axial_force" or w[0] == "element_local_shear_force_y")
                row[1] = "pipe:ghost"

            def ref_bad_section(g):
                pid = next(iter(g["ref"]["sections"]))
                g["ref"]["sections"][pid]["ro"] = 0.1
            judge(6, f"{route}: a request node moved to 1e20 (the runs solved the original)", P + "request_node_1e20",
                  write(T(P + "rq-far.set.jsonl"), edited(req_far)), [write(T(P + "rq.raw.jsonl"), runs)], False,
                  lambda j: prov(j) and any("request sha256 differs" in p for p in j["provenance"]), "provenance: request sha256", route=route)
            for case, f, why in (("request_duplicate_node_id", req_dup, "duplicate nodes ids"), ("request_undefined_pipe_end", req_ghost_end, "ends of"),
                                 ("request_undefined_support_node", req_ghost_support, "node of"),
                                 ("request_id_naming_two_entities", req_shared_id, "names two entities"),
                                 ("reference_row_of_no_request_entity", ref_ghost_row, "a reference row"),
                                 ("reference_section_value_not_text", ref_bad_section, "ref section")):
                judge(6, f"{route}: reference admission ({case.replace('_', ' ')})", P + case, write(T(P + case + ".set.jsonl"), edited(f)),
                      [T(P + "rq.raw.jsonl")], False, lambda j, why=why: prov(j) and any(why in p for p in j["provenance"]),
                      "provenance: reference refused", route=route)

            def null_global_end(r, route=route):
                ends = [x for x in r["results"] if x["kind"] == "element_global_end_force_v1"]
                if not ends:
                    pid = models[0]["request"]["model"]["pipe_segments"][0]["id"] if r["tag"] == models[0]["tag"] else \
                        models[1]["request"]["model"]["pipe_segments"][0]["id"]
                    x = S.row(route, "case:1", "element_global_end_force_v1", pid, "end_i", "Fx", None, 0.0, "result:global-end")
                    r["results"].append(x)
                    r["results_count"] = len(r["results"])
                    ends = [x]
                ends[0]["value"] = None
            judge_runs(6, f"{route}: a contract-valid derived row published null on Passed runs", P + "null_derived_row",
                       write(T(P + "null.set.jsonl"), models), mut(runs, null_global_end), False,
                       lambda j: m3(j) and j.get("schema_invalid", 0) == 0, "M3: C1 (null first, before any kind-based skip)", route=route)
        # the controls file (exact route): every entry admitted
        x_twin_ref = next(g for g in ex_gens if g["tag"] == plain["meta"]["twin"])["ref"]
        for case, ctl_lines, why in (("control_unknown_entry", [{"tag": plain["tag"], "pertE": x_twin_ref}, {"tag": "absent-model", "pertE": x_twin_ref}],
                                      "not a model of the set"),
                                     ("control_malformed_pertE", [{"tag": plain["tag"], "pertE": {"malformed": True}}], "control reference"),
                                     ("control_unknown_malformed", [{"tag": plain["tag"], "pertE": x_twin_ref}, {"tag": "absent-model", "pertE": {"malformed": True}}],
                                      "not a model of the set"),
                                     ("control_duplicate_tag", [{"tag": plain["tag"], "pertE": x_twin_ref}] * 2, "duplicate tag"),
                                     ("control_extra_key", [{"tag": plain["tag"], "pertE": x_twin_ref, "note": "x"}], "keys")):
            judge(6, "exact: the controls file", case, ex_own, [T("exact-ctl.raw.jsonl")], False,
                  lambda j, why=why: prov(j) and any(why in p for p in j["provenance"]), "provenance: control refused", route="exact", variant="control",
                  controls_path=write(T(case + ".controls.jsonl"), ctl_lines))
        judge(6, "exact: a controls file not committed", "control_file_not_committed", ex_own, [T("exact-ctl.raw.jsonl")], False,
              lambda j: prov(j) and any("controls file" in p for p in j["provenance"]), "provenance: controls file not committed", route="exact",
              variant="control", controls_path=ctl, references={("exact", "set"): {MV.file_sha256(ex_own): "synthetic"}})
        # the committed digests themselves: tracked and unchanged in git
        repo = T("pins-repo")
        os.makedirs(repo)
        pin = os.path.join(repo, "REFERENCE_SHA256SUMS")
        open(pin, "w").write("0" * 64 + " pf set synthetic\n")
        gp = lambda *a: subprocess.run(["git", "-C", repo, *a], capture_output=True, text=True)
        gp("init", "-q")
        _, e1 = MV.pinned_references(pin)
        gp("add", "-A")
        gp("-c", "user.name=selftest", "-c", "user.email=selftest@invalid", "commit", "-q", "-m", "x")
        p2, e2 = MV.pinned_references(pin)
        open(pin, "a").write("1" * 64 + " pf set added later\n")
        _, e3 = MV.pinned_references(pin)
        table(6, "the committed digests: untracked, committed, then edited", "reference_sums_committed_and_unchanged",
              "untracked refused; committed read; edited refused",
              any("not in HEAD" in x for x in e1) and not e2 and p2 == {("pf", "set"): {"0" * 64: "synthetic"}} and any("differs" in x for x in e3))
        # ---- round 8 (Agent 0, accepted with conditions): the exact view admits load_reference_states (shape-checked from
        # the schema, never read) and retained_source_endpoint_normal_max_v1 only under pressure-source-1
        lrs_fx, _ = m3_json.read_json(os.path.join(MV.FIXTURES, "pressure_load_state_pressure_sparse.stress_neutral.json"))
        lrs_one = copy.deepcopy(lrs_fx["contract_evidence"]["load_reference_states"][0])
        lrs_one["load_case_id"] = "case:1"

        def scaled(x):
            """Every number of a record times 1e9 (each would move a verdict if it were read; the schema still holds)."""
            if isinstance(x, dict):
                return {k: scaled(v) for k, v in x.items()}
            if isinstance(x, list):
                return [scaled(v) for v in x]
            return x * 1e9 if type(x) in (int, float) else x
        with_lrs = lambda rec: (lambda r: r["contract_evidence"].update(load_reference_states=[copy.deepcopy(rec)]))
        check("round 8: the committed fixture's load_reference_states record is schema-valid and its scaled copy too",
              not MV._schema_checked_members({"load_reference_states": [lrs_one]}, MV.Facts([], [], [], ["case:1"]))
              and not MV._schema_checked_members({"load_reference_states": [scaled(lrs_one)]}, MV.Facts([], [], [], ["case:1"])))
        base_ok = MR.report("exact", ex_own, [write(T("r8-base.raw.jsonl"), xe)], quiet=True, workers=1, runner_sha=R, head=head, checkout=checkout,
                            references=pins("exact", ex_own), eval_out=T("r8-base.eval.jsonl"))
        base_bad = MR.report("exact", ex_own, [write(T("r8-bad.raw.jsonl"), [xbad] + xe[1:])], quiet=True, workers=1, runner_sha=R, head=head,
                             checkout=checkout, references=pins("exact", ex_own), eval_out=T("r8-bad.eval.jsonl"))
        verdicts = lambda path: {(r["tag"], r["entry"], r["mode"]): (r.get("verdict_failures"), r.get("info")) for r in
                                 (m3_json.strict_loads(line) for line in open(path))}
        for case, runs_, base_j, base_eval, want in (("exact_lrs_scaled_unread", xe, base_ok, T("r8-base.eval.jsonl"), True),
                                                     ("exact_lrs_scaled_unread_bad_rows", [xbad] + xe[1:], base_bad, T("r8-bad.eval.jsonl"), False)):
            j = judge(8, "exact view: an admitted load_reference_states whose every number is x1e9 (it would move the verdict if read)", case,
                      ex_own, [write(T(case + ".raw.jsonl"), mut(runs_, with_lrs(scaled(lrs_one))))], want,
                      lambda j: j.get("schema_invalid", 0) == 0, "verdict unchanged: never read", route="exact", eval_out=T(case + ".eval.jsonl"))
            table(8, "the same: every record's clauses and information equal those without it", case + "_records_equal",
                  "records equal" if verdicts(T(case + ".eval.jsonl")) == verdicts(base_eval) else "records differ",
                  verdicts(T(case + ".eval.jsonl")) == verdicts(base_eval) and j["pass"] == base_j["pass"])
        unknown = copy.deepcopy(lrs_one)
        unknown["unexpected"] = 1
        other_case = copy.deepcopy(lrs_one)
        other_case["load_case_id"] = "case:ghost"
        for case, f in (("exact_lrs_wrong_type", lambda r: r["contract_evidence"].update(load_reference_states={})),
                        ("exact_lrs_record_wrong_type", lambda r: r["contract_evidence"].update(load_reference_states=["record"])),
                        ("exact_lrs_unknown_key", with_lrs(unknown)),
                        ("exact_lrs_duplicate_case", lambda r: r["contract_evidence"].update(load_reference_states=[copy.deepcopy(lrs_one), copy.deepcopy(lrs_one)])),
                        ("exact_lrs_other_case", with_lrs(other_case)),
                        ("exact_evidence_unknown_member", lambda r: r["contract_evidence"].update(load_reference_state=[copy.deepcopy(lrs_one)]))):
            judge_runs(8, "exact view: a malformed load_reference_states or an evidence member the schema does not admit", case, ex_own, mut(xe, f),
                       False, schema, "refused by the view", route="exact")
        token = "retained_source_endpoint_normal_max_v1; section_basis="
        p2_id, ps1_id = MV.CONTRACTS["exact"][0][0], MV.CONTRACTS["exact"][1][0]
        u2 = MV.match_contract("exact", MV.MAXIMUM, "maximum_absolute_normal_stress", "Pa", token + "nominal_section", p2_id)
        u1 = MV.match_contract("exact", MV.MAXIMUM, "maximum_absolute_normal_stress", "Pa", token + "nominal_section", ps1_id)
        table(8, "the retained-source token row by row", "exact_token_contract_tables", f"pressure-2: refused; pressure-source-1: basis {u1[0]}",
              u2[1] is not None and u1 == ("nominal_section", None))

        def retained(r):
            for x in r["results"]:
                if x["kind"] == MV.MAXIMUM:
                    x["metadata"]["basis"] = token + MV._section_basis(x["metadata"]["basis"])

        def global_end(r):
            pid = next(x["entity_ref"] for x in r["results"] if x["kind"] == MV.MAXIMUM)
            r["results"].append(S.row("exact", "case:1", "element_global_end_force_v1", pid, "end_i", "Fx", None, 1.0, "result:global-end-p2"))
            r["results_count"] = len(r["results"])
        judge_runs(8, "exact view: the token beside a row only pressure-2 declares (the token under pressure-2)", "exact_token_under_pressure_2", ex_own,
                   mut(xe, lambda r: (retained(r), global_end(r))), False, schema, "refused by the view: no single contract", route="exact")
        # under pressure-source-1 the evidence is the recovery evidence (round 9), so these controls publish it
        jt = judge_runs(8, "exact view: the token in a record wholly of pressure-source-1 (control)", "exact_token_under_pressure_source_1", ex_own,
                        mut(xe, source_recovery), True, lambda j: j.get("contracts") == {ps1_id: 8}, "admitted under pressure-source-1", route="exact")
        judge_runs(8, "exact view: the token in a record wholly of pressure-source-1, a C3 defect (control)", "exact_token_under_pressure_source_1_bad_rows",
                   ex_own, mut([xbad] + xe[1:], source_recovery), False, m3, "M3: C3 (judged as before)", route="exact")
        # ---- round 9 (Agent 0, the third shape): pressure-source-1's recovery evidence, validated against
        # physics_source_recovery.schema.json at judge time and admitted only under pressure-source-1; its extrema are
        # read as kd5's prior judge read them (pipe_id, section_basis, value_upper_pa for C2, value_lower_pa for C7)
        rx = mut(xe, source_recovery)
        rxe = lambda r: r["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"]
        base9 = judge_runs(9, "exact view: pressure-source-1 recovery evidence on Passed records (control)", "exact_recovery_positive", ex_own, rx,
                           True, lambda j: j.get("contracts") == {ps1_id: 8} and j.get("schema_invalid", 0) == 0, "admitted under pressure-source-1",
                           route="exact", eval_out=T("r9-base.eval.jsonl"))

        def unread_scaled(r):
            for c in r["contract_evidence"]["exact_cases"]:
                for e in c["pipe_stress_extrema"]:
                    e["value_pa"] *= 1e9
                    e["absolute_error_bound_pa"] = e["absolute_error_bound_pa"] * 1e9 + 1e9
                    for p in e["endpoints"]:
                        p["value_pa"] *= 1e9
                        p["interval_pa"] = [x * 1e9 for x in p["interval_pa"]]
        j9 = judge_runs(9, "exact view: value_pa, the endpoints and the error bound x1e9 (each would move a verdict if it were read)",
                        "exact_recovery_unread_scaled", ex_own, mut(rx, unread_scaled), True, lambda j: j.get("schema_invalid", 0) == 0,
                        "verdict unchanged: never read", route="exact", eval_out=T("r9-scaled.eval.jsonl"))
        table(9, "the same: every record's clauses and information equal those without it", "exact_recovery_unread_scaled_records_equal",
              "records equal" if verdicts(T("r9-scaled.eval.jsonl")) == verdicts(T("r9-base.eval.jsonl")) else "records differ",
              verdicts(T("r9-scaled.eval.jsonl")) == verdicts(T("r9-base.eval.jsonl")) and j9["pass"] == base9["pass"])

        def c2_defect(r):
            for e in rxe(r)[:1]:
                e["value_lower_pa"] = e["value_upper_pa"] = 0.0
        c2_runs = mut(rx[:1], c2_defect) + rx[1:]
        judge_runs(9, "exact view: a Passed pressure-source-1 record whose retained maximum's upper bound is 0 (C2 coverage defect)",
                   "exact_recovery_c2_defect", ex_own, c2_runs, False,
                   lambda j: c2(j) and any("C2" in (v or {}) for (t, e, m), (v, _) in verdicts(T("r9-c2.eval.jsonl")).items()
                                           if (t, e, m) == (rx[0]["tag"], rx[0]["entry"], rx[0]["mode"])),
                   "M3: C2 on that record", route="exact", eval_out=T("r9-c2.eval.jsonl"))

        def drop(where, key):
            return lambda r: where(r).pop(key)
        dup = lambda r: rxe(r).append(copy.deepcopy(rxe(r)[0]))
        for case, f in (("exact_recovery_upper_wrong_type", lambda r: rxe(r)[0].update(value_upper_pa="1")),
                        ("exact_recovery_endpoints_wrong_type", lambda r: rxe(r)[0].update(endpoints={})),
                        ("exact_recovery_extremum_unknown_key", lambda r: rxe(r)[0].update(unexpected=1)),
                        ("exact_recovery_case_unknown_key", lambda r: r["contract_evidence"]["exact_cases"][0].update(unexpected=1)),
                        ("exact_recovery_unknown_member", lambda r: r["contract_evidence"].update(unexpected=[])),
                        ("exact_recovery_duplicate_extremum", dup),
                        ("exact_recovery_missing_value_pa", drop(lambda r: rxe(r)[0], "value_pa")),
                        ("exact_recovery_missing_recovery_method", drop(lambda r: r["contract_evidence"]["exact_cases"][0], "recovery_method")),
                        ("exact_recovery_missing_connector", drop(lambda r: r["contract_evidence"], "connector")),
                        ("exact_recovery_extremum_of_another_case", lambda r: rxe(r)[0].update(load_case_id="case:ghost"))):
            judge_runs(9, "exact view: malformed pressure-source-1 recovery evidence", case, ex_own, mut(rx, f), False, schema, "refused by the view",
                       route="exact")

        judge_runs(9, "exact view: the recovery evidence on rows of pressure-2 (the recovery shape under pressure-2)",
                   "exact_recovery_under_pressure_2", ex_own,
                   [dict(copy.deepcopy(p2), contract_evidence=copy.deepcopy(ps1["contract_evidence"])) for p2, ps1 in zip(xe, rx)], False,
                   lambda j: schema(j) and not j.get("contracts"), "refused by the view", route="exact")
        judge_runs(9, "exact view: pressure-2's evidence on rows of pressure-source-1", "exact_pressure_2_evidence_under_pressure_source_1", ex_own,
                   mut(xe, retained), False, lambda j: schema(j) and not j.get("contracts"), "refused by the view", route="exact")
        # ---- round 10 (Agent 0, a criterion change after the 0.8.0 re-judge): C4 within one contract, C4x (information)
        # across two; each entry still judged against its own reference
        def rec_of(path, tag, entry, mode):
            return next(r for r in (m3_json.strict_loads(x) for x in open(path)) if (r["tag"], r["entry"], r["mode"]) == (tag, entry, mode))

        def half_top(r):
            top = max((x for x in r["results"] if x["kind"].startswith("global_nodal_displacement_")), key=lambda x: abs(x["value"]))
            top["value"] *= 0.5
        sens_at = lambda tag, entry: (lambda r: r["tag"] == tag and r["entry"] == entry and r["mode"] == MV.MODES[0] and sens(r))
        for case, st, runs_, route, tag in (("pf_same_contract_c4", one, four, "pf", codex["tag"]),
                                            ("exact_same_contract_pressure_2_c4", ex_own, xe, "exact", pres["tag"]),
                                            ("exact_same_contract_pressure_source_1_c4", ex_own, rx, "exact", pres["tag"])):
            ev10 = T(case + ".eval.jsonl")
            judge_runs(10, "a same-contract typed/captured disagreement (typed Passed, captured Sensitive)", case, st, mut(runs_, sens_at(tag, "captured")),
                       False,
                       lambda j, ev10=ev10, tag=tag: m3(j) and j["c4_failures"] == 1 and not j["c4x_runs"]
                       and rec_of(ev10, tag, "captured", MV.MODES[0])["verdict_failures"] == {"C4": 1},
                       "M3: C4", route=route, eval_out=ev10)
        # pres: typed under pressure-2, captured under pressure-source-1 (the product's split, CLM-003); plain unchanged
        npx = sum(1 for r in xe if r["tag"] == pres["tag"])
        mixed = [r if r["entry"] == "typed" or r["tag"] != pres["tag"] else s_ for r, s_ in zip(xe, rx)]
        assert npx == 4 and sum(r["tag"] == pres["tag"] and r["entry"] == "captured" for r in mixed) == 2
        cap_sens, typ_sens = sens_at(pres["tag"], "captured"), sens_at(pres["tag"], "typed")
        cross_ok = lambda j: (j["c4_failures"] == 0 and j["c4x_runs"] == [[pres["tag"], MV.MODES[0]]]
                                 and j.get("contracts") == {p2_id: 6, ps1_id: 2} and j.get("schema_invalid", 0) == 0)
        judge_runs(10, "exact view: a cross-contract pair, its captured Sensitive (C4x information)", "exact_cross_contract_c4x", ex_own,
                   mut(mixed, cap_sens), True, cross_ok, "C4x listed; no C4", route="exact")
        judge_runs(10, "exact view: a cross-contract pair, its typed Sensitive (C4x information)", "exact_cross_contract_c4x_typed_sensitive",
                   ex_own, mut(mixed, typ_sens), True, cross_ok, "C4x listed; no C4", route="exact")
        refuse_typed = lambda r: r["tag"] == pres["tag"] and r["entry"] == "typed" and r["mode"] == MV.MODES[0] and r.update(
            diagnostics=[S.diag(sorted(MV.REFUSALS["exact"])[0], "blocking")], results=[], results_count=0, summary=None, contract_evidence=None)
        judge_runs(10, "exact view: a refused typed entry beside a pressure-source-1 captured one (a refusal has no contract)",
                   "exact_refusal_beside_pressure_source_1_c4", ex_own, mut(mixed, refuse_typed), False,
                   lambda j: m3(j) and j["c4_failures"] == 1 and not j["c4x_runs"], "M3: C4, not C4x", route="exact")
        for case, standing, defect_entry in (("exact_cross_contract_c3_on_typed", cap_sens, "typed"),
                                             ("exact_cross_contract_c3_on_captured", typ_sens, "captured")):
            ev10 = T(case + ".eval.jsonl")
            hit = lambda r, e=defect_entry: r["tag"] == pres["tag"] and r["entry"] == e and r["mode"] == MV.MODES[0] and half_top(r)
            judge_runs(10, f"exact view: a C4x pair whose Passed {defect_entry} entry halves its largest displacement", case, ex_own,
                       mutate_both(mixed, standing, hit), False,
                       lambda j, ev10=ev10, e=defect_entry: m3(j) and j["c4_failures"] == 0 and j["c4x_runs"] == [[pres["tag"], MV.MODES[0]]]
                       and "C3" in rec_of(ev10, pres["tag"], e, MV.MODES[0])["verdict_failures"],
                       f"M3: C3 on the {defect_entry} entry, judged against its own reference; C4x still listed", route="exact", eval_out=ev10)
        # ---- round 11 (Codex at c5d1bfecf1): JSON Schema equivalence of the evidence checker. An integer is a number with
        # a zero fractional part; uniqueItems compares JSON values; an unsupported keyword anywhere refuses the record
        def raises(node, value=1):
            try:
                MV.schema_problems(value, node, MV.SOURCE_RECOVERY_SCHEMA)
            except MV.SchemaUnsupported:
                return True
            return False
        sp = lambda v, node: MV.schema_problems(v, node, MV.SOURCE_RECOVERY_SCHEMA)
        integer = {"type": "integer", "minimum": 0}
        table(11, "the checker on integers", "schema_integer_values", "0, 0.0, 12.0 admitted; 0.5, true, '1' refused",
              not sp(0, integer) and not sp(0.0, integer) and not sp(12.0, integer) and sp(0.5, integer) and sp(True, integer) and sp("1", integer))
        uniq = {"type": "array", "uniqueItems": True}
        table(11, "the checker on uniqueItems", "schema_unique_items_by_value", "[0, 0.0] and [{a: 1}, {a: 1.0}] repeated; [0, 1], [0, false] unique",
              bool(sp([0, 0.0], uniq)) and bool(sp([{"a": 1}, {"a": 1.0}], uniq)) and not sp([0, 1], uniq) and not sp([0, False], uniq))
        table(11, "an unsupported keyword nested in a schema", "schema_unsupported_keyword_nested",
              "raised in if, in an untaken else, in oneOf, under properties; a supported schema does not raise",
              raises({"if": {"not": {}}, "then": {}}) and raises({"if": {"type": "integer"}, "then": {}, "else": {"allOf": []}})
              and raises({"oneOf": [{"type": "integer"}, {"anyOf": []}]}) and raises({"type": "object", "properties": {"x": {"not": {}}}}, {})
              and not raises({"if": {"type": "integer"}, "then": {"minimum": 0}, "else": {}, "oneOf": [{"type": "integer"}, {"type": "string"}]}))
        sc = {"support_id": pres["request"]["model"]["supports"][0]["id"], "node_id": pres["request"]["model"]["supports"][0]["node"], "dof": "UX",
              "global_dof": 0, "law_kind": "rigid_prescribed", "prescribed_value": 0, "unit": "m", "meaning": "absolute_reference_displacement",
              "physical_state_source": "support_state.boundary_motion"}

        def lrs_dof(dof):
            rec = copy.deepcopy(lrs_one)
            rec["support_components"] = [dict(sc, global_dof=dof)]
            return with_lrs(rec)

        def indices(f):
            def g(r):
                for c in r["contract_evidence"]["exact_cases"]:
                    for e in c["pipe_stress_extrema"]:
                        for p in e["endpoints"]:
                            p["functional_indices"] = [f(x) for x in p["functional_indices"]]
            return g
        for case, runs_, want, why, label in (
                ("exact_lrs_global_dof_0", mut(xe, lrs_dof(0)), True, lambda j: j.get("schema_invalid", 0) == 0, "admitted"),
                ("exact_lrs_global_dof_0_0", mut(xe, lrs_dof(0.0)), True, lambda j: j.get("schema_invalid", 0) == 0, "admitted: 0.0 is an integer"),
                ("exact_lrs_global_dof_0_5", mut(xe, lrs_dof(0.5)), False, schema, "refused by the view: not an integer"),
                ("exact_recovery_endpoint_indices_as_12_0", mut(rx, indices(float)), True, lambda j: j.get("schema_invalid", 0) == 0,
                 "admitted: 12.0 is an integer"),
                ("exact_recovery_endpoint_index_0_5", mut(rx, indices(lambda x: x + 0.5)), False, schema, "refused by the view: not an integer")):
            judge_runs(11, "exact view: integers written as JSON numbers in shape-checked evidence", case, ex_own, runs_, want, why, label, route="exact")
        saved_docs = dict(MV._SCHEMA_DOCS)
        try:
            for case, plant in (("exact_recovery_schema_unsupported_in_oneOf",
                                 lambda d: d["$defs"]["pressure_source_ordinary_evidence"]["properties"]["exact_cases"]["items"]["properties"]
                                 ["pipe_stress_extrema"]["items"]["oneOf"][0].update(anyOf=[])),
                                ("exact_recovery_schema_unsupported_in_else",
                                 lambda d: d["$defs"]["retained_endpoint_maximum"].update({"if": {"type": "object"}, "then": {}, "else": {"not": {}}}))):
                MV._SCHEMA_DOCS.clear()
                MV._SCHEMA_DOCS.update(saved_docs)
                doc = copy.deepcopy(MV._schema_doc(MV.SOURCE_RECOVERY_SCHEMA))
                plant(doc)
                MV._SCHEMA_DOCS[MV.SOURCE_RECOVERY_SCHEMA] = doc
                judge_runs(11, "exact view: the recovery schema with an unsupported keyword in a branch the record does not take", case, ex_own, rx,
                           False, lambda j: schema(j) and any("SchemaUnsupported" in p for p in j["schema_problems"]),
                           "refused by the view: SchemaUnsupported", route="exact")
        finally:
            MV._SCHEMA_DOCS.clear()
            MV._SCHEMA_DOCS.update(saved_docs)
        # ---- round 7 (Codex at cb60446a01): index flags hid an edited manifest; it is now read from HEAD's blob (Agent 0)
        jr = T("r7-judge-repo")
        rel_j = os.path.join("projects", "chirality-piping", "validation", "references", "m3_judge")
        rel_c = os.path.join("projects", "chirality-piping", "fixtures", "results")
        shutil.copytree(HERE, os.path.join(jr, rel_j), ignore=shutil.ignore_patterns("__pycache__"))
        os.makedirs(os.path.join(jr, rel_c))
        for f in MV.CONTRACT_FILES.values():
            shutil.copyfile(os.path.join(MV.FIXTURES, f), os.path.join(jr, rel_c, f))
        rel_s = os.path.join("projects", "chirality-piping", "schemas")
        os.makedirs(os.path.join(jr, rel_s))
        for f in MV.SCHEMA_FILES:
            shutil.copyfile(os.path.join(MV.SCHEMAS, f), os.path.join(jr, rel_s, f))
        gj = lambda *a: subprocess.run(["git", "-C", jr, *a], capture_output=True, text=True)
        gj("init", "-q")
        gj("add", "-A")
        commit = lambda msg: gj("-c", "user.name=selftest", "-c", "user.email=selftest@invalid", "commit", "-q", "-m", msg)
        commit("judge copy")
        jpin = os.path.join(jr, rel_j, "REFERENCE_SHA256SUMS")
        jpin_rel = os.path.join(rel_j, "REFERENCE_SHA256SUMS")
        original = open(jpin, "rb").read()
        r7_raw = write(T("r7-complete.raw.jsonl"), complete)
        r7_set = os.path.join(jr, rel_j, "selftest", "pf_set.jsonl")
        r7_uncommitted = write(T("r7-uncommitted.set.jsonl"), open(PF_SET).read().replace("}\n", "} \n", 1), raw=True)
        added = (MV.file_sha256(r7_uncommitted) + " pf set an uncommitted reference\n").encode()

        def r7(case, attack, st, want):
            pins_, probs_ = MV.pinned_references(jpin)
            lib_ok = (not probs_ and not MV.reference_file_problems(st, "pf", "set", pins_)) == want
            c = subprocess.run(py + [os.path.join(jr, rel_j, "m3_report.py"), "--route", "pf", st, r7_raw, "--workers", "2", "--runner-sha256", R,
                                     "--product-head", head, "--checkout", checkout], capture_output=True, text=True)
            last = c.stdout.strip().splitlines()[-1] if c.stdout.strip() else ""
            prov_line = next((x for x in c.stdout.splitlines() if x.startswith("PROVENANCE")), "")
            cli_ok = ((c.returncode == 0 and last == "OVERALL: PASS") if want else
                      (c.returncode == 1 and last == "OVERALL: FAIL" and prov_line.endswith("FAIL") and "REFERENCE_SHA256SUMS" in c.stdout))
            why = "" if not probs_ else ("not in HEAD" if "not in HEAD" in probs_[0] else "differs from HEAD's blob" if "differs" in probs_[0] else probs_[0][-60:])
            outcome = f"library {'admitted' if not probs_ else 'refused: ' + why}; CLI exit {c.returncode}, '{last}'"
            table(7, attack, case, outcome, lib_ok and cli_ok and "Traceback" not in c.stderr)

        r7("pins_clean_checkout", "a clean checkout of the judge (control)", r7_set, True)
        open(jpin, "wb").write(original + added)
        r7("pins_unstaged_edit", "an unstaged edit adding an uncommitted reference's digest", r7_uncommitted, False)
        gj("add", "--", jpin_rel)
        r7("pins_staged_edit", "a staged edit adding it", r7_uncommitted, False)
        gj("reset", "-q", "HEAD", "--", jpin_rel)
        open(jpin, "wb").write(original)
        for flag in ("assume-unchanged", "skip-worktree"):
            gj("update-index", "--" + flag, "--", jpin_rel)
            open(jpin, "wb").write(original + added)
            hidden = not gj("status", "--porcelain", "--", jpin_rel).stdout.strip() and gj("diff", "--quiet", "HEAD", "--", jpin_rel).returncode == 0
            r7(f"pins_{flag.replace('-', '_')}_edit", f"--{flag} plus an edit (git status and diff show nothing: {hidden})", r7_uncommitted, False)
            open(jpin, "wb").write(original)
            gj("update-index", "--no-" + flag, "--", jpin_rel)
        open(jpin, "wb").write(original + added)
        gj("add", "--", jpin_rel)
        commit("pin the reference")
        r7("pins_committed_edit", "the same digest committed (the accepted path; control)", r7_uncommitted, True)
        gj("rm", "-q", "--cached", "--", jpin_rel)
        commit("drop the manifest from HEAD")
        r7("pins_absent_from_head", "the manifest absent from HEAD (the working file kept)", r7_uncommitted, False)
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
        # run.sh judges through the CLI, which reads the committed REFERENCE_SHA256SUMS: the driver sets are byte copies
        # of committed reference files (the PF selftest set, the kd5 frozen subset)
        others = [g for g in pf_gens if g["tag"] != codex["tag"]]
        complete_bad = [bad if (r["tag"], r["entry"], r["mode"]) == (bad["tag"], bad["entry"], bad["mode"]) else r for r in complete]
        good, badr, exit7 = runner("good", complete), runner("bad", complete_bad), runner("exit7", complete, 7)
        good4 = good
        sd, sdx = T("driver-set"), T("driver-setx")
        os.makedirs(sd)
        os.makedirs(sdx)
        shutil.copyfile(PF_SET, os.path.join(sd, "set.jsonl"))
        shutil.copyfile(EXACT_SET, os.path.join(sdx, "set.jsonl"))
        sd4 = sd
        goodx = runner("goodx", [run(g, e, m, "exact") for g in ex_gens for e, m in EM])
        failing_twin_py = T("py-failing-twin.sh")
        open(failing_twin_py, "w").write(f'#!/bin/bash\ncase " $* " in *" --ref twin "*) echo "OVERALL: FAIL (synthetic: the twin pass did not complete)"; exit 1 ;; esac\n'
                                         f'exec {sys.executable} "$@"\n')
        os.chmod(failing_twin_py, 0o755)

        def driver(rnd, attack, case, runner_bin, rundir, chunks, variants, want, setdir=sd, why=lambda d: True, route="pf", python=sys.executable):
            for i, tags in enumerate(chunks):
                write(os.path.join(rundir, "chunks", f"c.{i:03d}.in.jsonl"), MV.request_lines(tags), raw=True)
            env = dict(os.environ, M3J_PY=python, M3J_WORKERS="2", M3J_BUILD=builder, M3J_FAKE_RUNNER=runner_bin)
            d = subprocess.run(["bash", os.path.join(HERE, "run.sh"), route, checkout, setdir, rundir, *variants], capture_output=True, text=True, env=env)
            last = d.stdout.strip().splitlines()[-1] if d.stdout.strip() else ""
            table(rnd, attack, case, f"run.sh exit {d.returncode}, final line '{last}'",
                  (d.returncode == 0) == want and last == ("OVERALL: PASS" if want else "OVERALL: FAIL") and why(rundir))

        driver(0, "positive control", "driver_positive", good, T("d/positive"), [[codex], others], ["own"], True)
        driver(0, "positive control", "driver_positive_with_twin_control", good4, T("d/twin-ok"), [pf_gens[:10], pf_gens[10:]], ["own", "twin"], True, sd4)
        driver(0, "exact view: positive control", "driver_exact_positive", goodx, T("d/exact"), [ex_gens[:32], ex_gens[32:]], ["own"], True, sdx,
               route="exact")
        driver(1, "a model missing from the chunks", "driver_missing_model", good, T("d/missing"), [others], ["own"], False)
        driver(1, "a chunk given twice", "driver_duplicate_across_chunks", good, T("d/dup"), [[codex], [codex], others], ["own"], False)
        driver(3, "a changed request under the same tag", "driver_changed_request", good, T("d/changed"), [[changed] + others], ["own"], False)
        driver(3, "a replaced runner", "driver_resume_good_then_bad_runner (good first)", good, T("d/resume"), [[codex], others], ["own"], True)
        driver(3, "a replaced runner", "driver_resume_bad_new_runner", badr, T("d/resume"), [[codex], others], ["own"], False)
        write(os.path.join(T("d/partial"), "out", "c.000.out.jsonl"), [bad])          # a partial earlier output of the same chunk
        driver(4, "a partial earlier output", "driver_recover_good_clear_partial", good, T("d/partial"), [[codex], others], ["own"], True,
               why=lambda d: sorted(os.listdir(os.path.join(d, "out"))) == ["c.000.out.jsonl", "c.001.out.jsonl"])
        write(os.path.join(T("d/partial2"), "out", "c.007.out.jsonl"), [bad])
        driver(4, "a stale output from an earlier chunking", "driver_stale_out_removed", good, T("d/partial2"), [[codex], others], ["own"], True,
               why=lambda d: not os.path.exists(os.path.join(d, "out", "c.007.out.jsonl")))
        driver(4, "a runner that exits 7 after valid records", "driver_valid_records_exit_nonzero", exit7, T("d/exit7"), [[codex], others], ["own"], False)
        driver(4, "a twin control that cannot complete", "driver_twin_runtime_error_accepted", good, T("d/twin-bad"), [[codex], others], ["own", "twin"], False,
               python=failing_twin_py)
        driver(4, "twin only, no own judgement", "driver_twin_only_no_own", good, T("d/twin-only"), [[codex], others], ["twin"], False)
        driver(5, "a control variant on the PF route", "driver_pf_control_refused", good, T("d/pf-ctl"), [[codex], others], ["own", "control"], False)
        driver(5, "an unknown route", "driver_unknown_route", good, T("d/route"), [[codex], others], ["own"], False, route="preview")
        tmpset = T("driver-set-unpinned")
        write(os.path.join(tmpset, "set.jsonl"), pf_gens)
        driver(6, "run.sh on a reference set whose sha256 is not committed", "driver_reference_not_committed", good, T("d/unpinned"), [[codex], others],
               ["own"], False, tmpset)
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
        full_raw = write(T("cli-complete.raw.jsonl"), complete)
        bad_raw = write(T("cli-bad.raw.jsonl"), complete_bad)
        sens_raw = write(T("cli-sensitive.raw.jsonl"), mut(complete_bad, sens))
        ex_raw = write(T("cli-exact.raw.jsonl"), [run(g, e, m, "exact") for g in ex_gens for e, m in EM])
        # the same JSON meaning, other bytes: one space after the first line's object
        modified = write(T("pf-set-modified.jsonl"), open(PF_SET).read().replace("}\n", "} \n", 1), raw=True)
        ex_modified = write(T("kd5-modified.jsonl"), open(EXACT_SET).read().replace("}\n", "} \n", 1), raw=True)
        check("the modified reference copies differ in bytes from the committed files",
              MV.file_sha256(modified) != MV.file_sha256(PF_SET) and MV.file_sha256(ex_modified) != MV.file_sha256(EXACT_SET))
        for case, argv, rc, route in (("cli_passing", [PF_SET, full_raw], 0, "pf"),
                                      ("cli_failing", [PF_SET, bad_raw], 1, "pf"),
                                      ("cli_bad_rows_sensitive_only", [PF_SET, sens_raw], 1, "pf"),
                                      ("cli_exact_passing", [EXACT_SET, ex_raw], 0, "exact"),
                                      ("cli_missing_mandatory_flags", [small, T("small_positive.raw.jsonl"), "--no-provenance"], 2, "pf"),
                                      ("cli_missing_route", [small, T("small_positive.raw.jsonl")], 2, None),
                                      ("cli_reference_not_committed", [small, T("small_positive.raw.jsonl")], 1, "pf"),
                                      ("cli_reference_hash_mismatch", [modified, full_raw], 1, "pf"),
                                      ("cli_exact_reference_hash_mismatch", [ex_modified, ex_raw], 1, "exact"),
                                      ("cli_reference_of_the_other_route", [EXACT_SET, ex_raw], 1, "pf")):
            flags = ["--runner-sha256", R, "--product-head", head, "--checkout", checkout] if case != "cli_missing_mandatory_flags" else []
            c = subprocess.run(py + [os.path.join(HERE, "m3_report.py"), *(["--route", route] if route else []), *argv, "--workers", "1", *flags],
                               capture_output=True, text=True)
            last = c.stdout.strip().splitlines()[-1] if c.stdout.strip() else ""
            rnd = 6 if "reference" in case else (5 if ("sensitive" in case or "route" in case) else (4 if rc == 2 else 0))
            table(rnd, "the CLI's final line and exit", case, f"exit {c.returncode}, final line '{last}'",
                  c.returncode == rc and last.startswith("OVERALL: PASS" if rc == 0 else "OVERALL: FAIL") and "Traceback" not in c.stderr
                  and ("reference" not in case or "is not a committed" in c.stdout))
    by_round = {}
    for r in rows:
        by_round[r[0]] = by_round.get(r[0], 0) + 1
    print("TABLE counts by round:", dict(sorted(by_round.items())))
    check(f"{len(rows)} regression cases across rounds 1-5 and the positive controls ({dict(sorted(by_round.items()))})", len(rows) >= 150)
