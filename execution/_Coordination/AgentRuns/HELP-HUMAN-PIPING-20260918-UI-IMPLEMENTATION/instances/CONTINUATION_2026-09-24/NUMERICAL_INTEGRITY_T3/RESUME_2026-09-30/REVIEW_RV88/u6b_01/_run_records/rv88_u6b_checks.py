"""RV88 (U6b review): independent checks of the Python successor carriers
(candidate lane). Oracles: the receipt's own selection lists, the shared case
file, Rust U6a's and TS U6d's recorded outcomes, and fail-closed properties.
Prints PASS/FAIL per check and writes facts to <out.tsv>.
Usage: python rv88_u6b_checks.py <lane P> <out.tsv>"""
import json
import sys
from copy import deepcopy
from pathlib import Path

P, OUT = Path(sys.argv[1]), Path(sys.argv[2])
sys.path.insert(0, str(P))
from core.analysis_runs import compatibility as c  # noqa: E402
from core.analysis_runs import records as r  # noqa: E402
from core.analysis_runs.retained_precision import validate_retained_precision  # noqa: E402

SUCC = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
PREVIEW = "openpipestress.result_semantics/0.3.0/preview-physics-1"
METHOD = "contribution_preserving_multiprecision_v1"
KW = {"input_manifest_ref": {"object_type": "InputManifest", "ref": "manifest:rv88"}, "input_manifest_hash": "9" * 64}
facts, results = [], []


def check(name, ok, detail=""):
    results.append((name, bool(ok)))
    if not ok:
        print("FAIL", name, str(detail)[:300])


def fact(k, v):
    facts.append(f"{k}\t{json.dumps(v, sort_keys=True, default=str)}")


def err(f):
    try:
        return ("ok", f())
    except Exception as e:  # noqa: BLE001
        return ("err", str(e))


def milestone(mode):
    d = json.loads((P / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())
    return d["source"], d["invocation"]


def projection(src):
    p = deepcopy(src)
    del p["retained_precision"]
    p["producer"]["semantic_contract_id"] = PREVIEW
    p["formulation_basis"]["profile_id"] = "product_preview_mechanics_v1"
    for row in p["results"]:
        row.pop("recovery_method", None)
    return p


def refs(inv):
    return [{"ref_type": "load_case", "ref_id": lc["id"]} for lc in inv["request"]["model"]["load_cases"]]


for mode in ("sparse_interactive", "dense_scrutiny"):
    src, inv = milestone(mode)
    other, _ = milestone("dense_scrutiny" if mode == "sparse_interactive" else "sparse_interactive")
    # Dispatch and standing.
    check(f"{mode}/dispatch", list(c._source_contract(deepcopy(src))[:2]) == [SUCC, "c74742ce6a936384e00986006e6a0b2e6bb11f190451e876eed9ffa11903c6a8"])
    for label, args in (("none", (refs(inv),)), ("inv", (refs(inv), inv)), ("empty_inv", ([], inv))):
        check(f"{mode}/standing_{label}", c.numerical_use_standing(deepcopy(src), *args) == "needs_recompute")
    edited = deepcopy(src); edited["results"][0]["value"] = 12345.0
    fact(f"{mode}/edited_row_standing_no_invocation", c.numerical_use_standing(edited, refs(inv)))
    check(f"{mode}/edited_row_unsupported", c.numerical_use_standing(edited, refs(inv)) == "unsupported" and c.numerical_use_standing(edited, refs(inv), inv) == "unsupported")
    foreign = deepcopy(inv); foreign["solver_mode"] = "dense_scrutiny" if mode == "sparse_interactive" else "sparse_interactive"
    check(f"{mode}/foreign_mode_unsupported", c.numerical_use_standing(deepcopy(src), refs(inv), foreign) == "unsupported")
    # The standing seam never reads numerical_quality; case order matters.
    v = dict(validate_retained_precision(deepcopy(src), inv)); v["numerical_eligible"] = True
    check(f"{mode}/seam_eligible", c._retained_standing_from(v, src, refs(inv)) == "numerically_eligible")
    for nq in ({}, {"status": "checks_passed", "cases": []}, {**src["numerical_quality"], "status": "failed"}):
        s = deepcopy(src); s["numerical_quality"] = nq
        check(f"{mode}/seam_ignores_quality", c._retained_standing_from(v, s, refs(inv)) == "numerically_eligible")
    two = deepcopy(src); case2 = deepcopy(two["retained_precision"]["body"]["cases"][0]); case2["basis_ref"] = {"ref_type": "load_case", "ref_id": "second"}
    two["retained_precision"]["body"]["cases"].append(case2)
    fwd = refs(inv) + [{"ref_type": "load_case", "ref_id": "second"}]
    check(f"{mode}/seam_two_case_order", c._retained_standing_from(v, two, fwd) == "numerically_eligible" and c._retained_standing_from(v, two, fwd[::-1]) == "needs_recompute")
    # Binding against the receipt's own list.
    listed = {x["result_id"] for x in src["retained_precision"]["body"]["cases"][0]["selection"]["absolute_verified"]}
    got = {row["id"]: c.rule_binding_refusal(src, row) for row in src["results"]}
    check(f"{mode}/binding_by_receipt_list", all((got[i] == "RULE_QUANTITY_BELOW_VERIFIED_FLOOR") == (i in listed) for i in got) and set(got.values()) <= {None, "RULE_QUANTITY_BELOW_VERIFIED_FLOOR"} and len(listed) == 69, {k: v for k, v in got.items() if (v is not None) != (k in listed)})
    check(f"{mode}/binding_refused_statement", all(c.rule_binding_refusal(edited, row) == "RULE_QUANTITY_NOT_COVERED" for row in edited["results"]))
    proj = projection(src)
    check(f"{mode}/binding_projection_none", all(c.rule_binding_refusal(proj, row) is None for row in proj["results"]))
    relab = deepcopy(src); relab["producer"]["semantic_contract_id"] = PREVIEW
    fact(f"{mode}/binding_relabelled", sorted({str(c.rule_binding_refusal(relab, row)) for row in relab["results"]}))
    # Summary.
    summ = c.classification_summary(deepcopy(src), inv)
    fact(f"{mode}/summary", summ)
    check(f"{mode}/summary", summ == [{"case_id": "case", "relative_verified": 25, "absolute_verified": 69, "interval_bindable": 0, "not_covered": 0, "input_derived": 3, "non_quantity": 1 if mode == "sparse_interactive" else 2, "withheld": 97}])
    # AnalysisRun.
    rec = c.build_analysis_run(deepcopy(src), **KW)
    check(f"{mode}/ar_copy_equal", rec["analysis_run"].get("retained_precision") == src["retained_precision"] and json.dumps(rec["analysis_run"]["retained_precision"], sort_keys=True) == json.dumps(src["retained_precision"], sort_keys=True))
    check(f"{mode}/ar_validates", err(lambda: c.validate_analysis_run_v0_3(rec, deepcopy(src)))[0] == "ok")
    fact(f"{mode}/ar_verify", err(lambda: c.verify_analysis_run_record(rec)))
    muts = {
        "dropped": lambda x: x["analysis_run"].pop("retained_precision"),
        "null": lambda x: x["analysis_run"].__setitem__("retained_precision", None),
        "sha": lambda x: x["analysis_run"]["retained_precision"].__setitem__("receipt_sha256", "0" * 64),
        "body": lambda x: x["analysis_run"]["retained_precision"]["body"]["work"].__setitem__("charged", 1),
        "extra": lambda x: x["analysis_run"]["retained_precision"].__setitem__("rv88", 1),
        "other_mode": lambda x: x["analysis_run"].__setitem__("retained_precision", deepcopy(other["retained_precision"])),
    }
    for name, m in muts.items():
        x = deepcopy(rec); m(x)
        out = err(lambda: c.validate_analysis_run_v0_3(x, deepcopy(src)))
        check(f"{mode}/ar_{name}_refused", out == ("err", "ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH"), out)
    prec = c.build_analysis_run(deepcopy(proj), **KW)
    for name, val in (("receipt", deepcopy(src["retained_precision"])), ("null", None)):
        x = deepcopy(prec); x["analysis_run"]["retained_precision"] = val
        out = err(lambda: c.validate_analysis_run_v0_3(x, deepcopy(proj)))
        check(f"{mode}/ar_projection_{name}_refused", out == ("err", "ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"), out)
    # D-U6-9 and F-U6b-3.
    check(f"{mode}/wrapper_refuses_successor", err(lambda: r.build_preview_analysis_run_envelope(deepcopy(src))) == ("err", "ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"))
    legacy = json.loads((P / "fixtures/product_preview/invented_mechanics_result.json").read_text())
    check("wrapper_accepts_plain_legacy", err(lambda: r.build_preview_analysis_run_envelope(deepcopy(legacy)))[0] == "ok")
    for name, val in (("receipt", deepcopy(src["retained_precision"])), ("null", None)):
        x = deepcopy(legacy); x["retained_precision"] = val
        check(f"wrapper_refuses_legacy_{name}", err(lambda: r.build_preview_analysis_run_envelope(x)) == ("err", "ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"))
        check(f"v0_2_refuses_legacy_{name}", err(lambda: c.build_analysis_run_v0_2(x, **KW)) == ("err", "ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN"), err(lambda: c.build_analysis_run_v0_2(x, **KW)))
    # Legacy 0.1.0 with a W1 token on a row (Rust and TS refuse it at dispatch).
    tok = deepcopy(legacy); tok["results"][0]["recovery_method"] = METHOD
    fact("legacy_token/raw_dispatch", err(lambda: list(c._source_contract(deepcopy(tok))[:2])))
    fact("legacy_token/standing", err(lambda: c.numerical_use_standing(deepcopy(tok), [])))
    fact("legacy_token/analysis_run", err(lambda: c.build_analysis_run(deepcopy(tok), **KW)["schema_version"]))
    fact("legacy_token/wrapper", err(lambda: r.build_preview_analysis_run_envelope(deepcopy(tok))["schema_version"]))
    # Relabel forms: 8 identities x 2 profiles x forms; raw dispatch must refuse.
    tally = {}
    for sid in (PREVIEW, "openpipestress.result_semantics/0.3.0/physics-1", "openpipestress.result_semantics/0.3.0/physics-source-1", "openpipestress.result_semantics/0.3.0/precision-1",
                "openpipestress.result_semantics/0.3.0/load-reference-1", "openpipestress.result_semantics/0.3.0/load-reference-source-1", "openpipestress.result_semantics/0.3.0/source-blocks-1",
                "openpipestress.result_semantics/0.3.0/unknown-rv88"):
        for keep in (False, True):
            base = deepcopy(src); base["producer"]["semantic_contract_id"] = sid
            if not keep:
                base["formulation_basis"]["profile_id"] = "product_preview_mechanics_v1"
            p2 = projection(src); p2["producer"]["semantic_contract_id"] = sid
            last = deepcopy(p2); last["results"][-1]["recovery_method"] = METHOD
            forms = {"receipt": base, "null": {**deepcopy(base), "retained_precision": None}, "empty": {**deepcopy(base), "retained_precision": {}},
                     "tokens_only": {k: v for k, v in deepcopy(base).items() if k != "retained_precision"}, "one_token_last": last}
            for form, s in forms.items():
                out = err(lambda: c._source_contract(deepcopy(s)))
                key = out[1] if out[0] == "err" else "ADMITTED"
                tally[key] = tally.get(key, 0) + 1
                check(f"{mode}/relabel_refused", out[0] == "err", (sid, keep, form))
                if "retained_precision" in s:
                    check(f"{mode}/relabel_member_code", out == ("err", "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"), (sid, keep, form, out))
                check(f"{mode}/relabel_standing", c.numerical_use_standing(deepcopy(s), refs(inv), inv) == "unsupported", (sid, keep, form))
    fact(f"{mode}/relabel_codes", tally)
    # R-2 noticed ordinary envelope: identical to the plain projection.
    noticed = projection(src)
    noticed["diagnostics"].append({"id": "diagnostic:retained-precision:case:unavailable", "code": "RETAINED_PRECISION_UNAVAILABLE", "severity": "info", "message": "Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics; the retained_precision receipt records the actual attempt and its typed cause.", "source": "core/product_physics", "affected_refs": ["case"]})
    check(f"{mode}/r2_dispatch", c._source_contract(deepcopy(noticed))[:2] == c._source_contract(deepcopy(proj))[:2])
    check(f"{mode}/r2_standing", c.numerical_use_standing(deepcopy(noticed), refs(inv)) == c.numerical_use_standing(deepcopy(proj), refs(inv)))

# The 14 shared cases, by RV88's own Python mapping, and the three-language F1 table.
cases = json.loads((P / "fixtures/results/retained_precision_carrier_cases.json").read_text())
n = 0
for case in cases["cases"]:
    mode = "sparse_interactive" if "sparse" in cases["fixtures"][case["fixture"]]["path"] else "dense_scrutiny"
    s, inv = milestone(mode)
    s, inv = deepcopy(s), deepcopy(inv)
    for e in case["edits"]:
        at = s if e["target"] == "source" else inv
        for k in e["path"][:-1]:
            at = at[k]
        at[e["path"][-1]] = e["value"]
    req = refs(milestone(mode)[1]) if case["requested"] == "invocation" else case["requested"]
    standing = c.numerical_use_standing(deepcopy(s), req, inv if case["invocation"] is not None else None)
    out = err(lambda: c._source_contract(deepcopy(s)))
    dispatch = "ok" if out[0] == "ok" else out[1]
    fact(f"case/{case['id']}", [standing, dispatch, case["expected_standing"], case["expected_dispatch"]])
    check(f"case/{case['id']}", [standing, dispatch] == [case["expected_standing"], case["expected_dispatch"]], [standing, dispatch])
    n += 1
check("cases_count_14", n == 14)
OUT.write_text("\n".join(facts) + "\n")
print(f"{sum(ok for _, ok in results)} of {len(results)} checks pass")
