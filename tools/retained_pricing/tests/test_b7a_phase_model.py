"""Current route/success bounds refuse drift and preserve fallback owners."""
from copy import deepcopy
import ast
import json
import os
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = Path(os.environ.get("B7A_PHASE_TEST_SOURCE", str(PRICING.parents[1])))
MODEL = runpy.run_path(str(PRICING / "base/b7a_phase_model.py"))


def tables(route="L", branch="ordinary"):
    loops = {"loops": [], "loop_total": [deepcopy(MODEL["LOOP_RULE"])]}
    text = {"site_total": {f"formation_guard.rs:{k}": {"bytes": v, "why": MODEL["TOTAL_WHY"]}
                           for k, v in MODEL["TOTALS"].items()}}
    loops, text, _ = MODEL["transform"](SOURCE, route, loops, text)
    if branch == "success":
        loops, text = MODEL["success_tables"](SOURCE, loops, text)
    return loops, text


def graph():
    fg, decide, solve = (MODEL[k] for k in ("FG", "DECIDE", "SOLVE"))
    return {"edges": {solve: [fg], fg: [decide], decide: []},
            "site_loops": [[solve, fg, [[]]], [fg, decide, [[MODEL["ROW_LOOP"]]]]]}


def active(route="L", branch="ordinary"):
    loops, text = tables(route, branch)
    return MODEL["state"](SOURCE, loops, text, graph(), deepcopy(MODEL["FORMAT_ROWS"]), [])


@pytest.mark.parametrize("route,actual,branch,f", [
    ("L", 3, "ordinary", 3), ("L", 2, "ordinary", 2), ("L", 1, "ordinary", 1),
    ("L", 3, "success", 3), ("L", 2, "success", 2), ("L", 1, "success", 1),
    ("E", 3, "ordinary", 0), ("E", 3, "success", 0),
])
def test_actual_cases_separate_from_storage_and_effective_finding_count(route, actual, branch, f):
    context = active(route, branch)
    caps = dict(c=3, lc=actual, n=32, l=128, ident=128)
    counts = {MODEL["FG"]: actual, MODEL["DECIDE"]: f * 192}
    assert MODEL["finding_loop_total"](context, MODEL["FG"], [MODEL["ROW_LOOP"]], caps, counts) == f * 192
    for line in MODEL["TOTALS"]:
        owner = MODEL["FG"] if line == 322 else MODEL["DECIDE"]
        loops = [MODEL["ROW_LOOP"]] if line == 322 else []
        required = MODEL["finding_requested"](context, MODEL["FG_FILE"], line, "format", caps,
                                              counts, f * 192, owner, loops)
        # Independent per-generation arithmetic: no averaged per-case q.
        g = 384 + 1284 * 1536
        per_case = {239: 192 * 59, 241: 87 * 192 + g, 251: 321 * 192 + g,
                    322: 192 * 923 + g}
        assert required == 2 * f * per_case[line]
    ordinary = MODEL["ordinary_counts"](context, actual, 3, 128)
    assert ordinary["authored_inputs"] == actual * 128
    assert ordinary["storage_cases"] == 3
    assert ordinary["states"] == (4 if route == "L" else 1)
    assert ordinary["force_per_case"] == (1536 if route == "L" else 128)


@pytest.mark.parametrize("family", ["site_zero", "site_from", "site_size", "site_aggregate", "lex_site_size", "site_total"])
@pytest.mark.parametrize("line", [239, 241, 251, 322])
def test_competing_full_kind_alias_refuses(family, line):
    loops, text = tables()
    text.setdefault(family, {})[f"{MODEL['FG_FILE']}:{line}:format"] = {"bytes": "0"}
    with pytest.raises(ValueError, match="competing formation"):
        MODEL["state"](SOURCE, loops, text)


@pytest.mark.parametrize("mutation", ["total", "extra_total", "parent", "duplicate_parent", "stack", "missing_format", "lex_alias"])
def test_actual_rule_and_caller_inventory_drift_refuses(mutation):
    loops, text = tables()
    cg, formats, lexical = graph(), deepcopy(MODEL["FORMAT_ROWS"]), []
    if mutation == "total": loops["loop_total"][0]["total"] = "0"
    if mutation == "extra_total": loops["loop_total"].append(dict(MODEL["LOOP_RULE"], total="0"))
    if mutation == "parent": cg["edges"]["unknown.rs:1:caller"] = [MODEL["FG"]]
    if mutation == "duplicate_parent": cg["edges"][MODEL["SOLVE"]].append(MODEL["FG"])
    if mutation == "stack": cg["site_loops"][1][2] = [[]]
    if mutation == "missing_format": formats.pop()
    if mutation == "lex_alias": lexical.append(dict(file=MODEL["FG_FILE"], line=239, kind="to_string"))
    with pytest.raises(ValueError): MODEL["state"](SOURCE, loops, text, cg, formats, lexical)


@pytest.mark.parametrize("mutation", ["missing", "zero", "wrong_cases", "wrong_rows", "zero_mult", "wrong_mult", "wrong_owner", "wrong_stack"])
def test_runtime_witness_refuses_unproved_count(mutation):
    context = active("L", "success")
    caps = dict(c=3, lc=3, n=32, l=128, ident=128)
    counts = {MODEL["FG"]: 3, MODEL["DECIDE"]: 576}
    mult, owner, stack = 576, MODEL["FG"], [MODEL["ROW_LOOP"]]
    if mutation == "missing": del counts[MODEL["FG"]]
    if mutation == "zero": counts[MODEL["FG"]] = 0
    if mutation == "wrong_cases": counts[MODEL["FG"]] = 2
    if mutation == "wrong_rows": counts[MODEL["DECIDE"]] = 192
    if mutation == "zero_mult": mult = 0
    if mutation == "wrong_mult": mult = 384
    if mutation == "wrong_owner": owner = MODEL["DECIDE"]
    if mutation == "wrong_stack": stack = []
    with pytest.raises(ValueError):
        MODEL["finding_requested"](context, MODEL["FG_FILE"], 322, "format", caps, counts, mult, owner, stack)


@pytest.mark.parametrize("route,actual", [("L", 1), ("L", 2), ("L", 3), ("E", 3)])
def test_six_closed_requested_factories_round_up_each_native_coefficient(monkeypatch, route, actual):
    monkeypatch.syspath_prepend(str(PRICING / "base"))
    context = active(route, "success")
    full = active("L", "ordinary")
    owners = actual if route == "L" else 0
    for owner in ("assembled_children", "typed_source_children", "scaled_force_children",
                  "recovery_source_children", "uniform_constructor", "element_application"):
        original = MODEL["formed_owner"](full, 3, 3, 128, 128, owner=owner, branch="ordinary")
        changed = MODEL["formed_owner"](context, actual, 3, 128, 128, owner=owner, branch="success")
        for key, coefficient in original.items():
            assert 3 * changed.get(key, 0) >= owners * coefficient
            assert 3 * changed.get(key, 0) < owners * coefficient + 3
    with pytest.raises(ValueError):
        MODEL["formed_owner"](context, actual, 3, 128, 128, owner="generic_case", branch="success")


def test_failed_preview_requires_its_actual_custody_and_route():
    custody = dict(route="Preview", requested_cases=1, cases_seen=1, attempted=[0],
                   combinations=0, basis_expected=False, bases=0, source_some=True)
    assert MODEL["failed_preview_counts"](active(), 1, 3, 128, custody=custody)["held"] == ["blocked_original", "failed_manifest"]
    for route, actual in [("E", 1), ("L", 3)]:
        with pytest.raises(ValueError):
            MODEL["failed_preview_counts"](active(route), actual, 3, 128, custody=custody)


def test_stage2_and_context_counterfactuals_fail_before_credit():
    loops, text = tables()
    for name, value in [("member_loads_preparable", False), ("route", "unknown"), ("branch", "fallback_credit")]:
        bad_loop, bad_text = deepcopy(loops), deepcopy(text)
        bad_loop[MODEL["MARKER"]][name] = bad_text[MODEL["MARKER"]][name] = value
        with pytest.raises(ValueError): MODEL["state"](SOURCE, bad_loop, bad_text)
    with pytest.raises(ValueError): active("unknown")


@pytest.mark.parametrize("file,fragment", [
    ("core/product_physics/src/retained_product.rs", b"if self.case_has_member_loads(index)!=member.is_some()"),
    ("core/product_physics/src/retained_wire.rs", b"if !status.contains(&CaseStatus::Selected)"),
    ("core/product_physics/src/retained_memory.rs", b"fn domain_clauses("),
    ("core/product_physics/src/formation_guard.rs", b"if !force.has_formation_records()"),
])
def test_actual_stage2_admission_and_frozen_source_drift_refuses(tmp_path, file, fragment):
    domain = runpy.run_path(str(PRICING / "base/b7a_domain_model.py"))
    formed = runpy.run_path(str(PRICING / "base/b7a_formation_model.py"))
    scoped = runpy.run_path(str(PRICING / "base/b7a_scoped_text_model.py"))
    member = runpy.run_path(str(PRICING / "base/w1_member_load_model.py"))
    owners = set(domain["SOURCE_FILES"]) | set(formed["SOURCE_FILES"]) | set(member["SOURCE_FILES"]) | {
        scoped["UNITS"], "core/product_physics/src/retained_memory.rs"}
    for owner in owners:
        destination = tmp_path / owner
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(SOURCE / owner, destination)
    target = tmp_path / file
    raw = target.read_bytes()
    assert fragment in raw
    target.write_bytes(raw.replace(fragment, fragment + b"/* changed premise */", 1))
    with pytest.raises(ValueError): MODEL["inspect"](tmp_path)


def test_driver_selects_only_success_phases_and_preserves_report_and_reserve(tmp_path):
    work, successor = tmp_path / "full", tmp_path / "success"
    work.mkdir(); successor.mkdir()
    keys = list(MODEL["phase_names"](SOURCE))   # PU-X3 (§8.14): X3 where the source republishes
    phase = {k: {"requested": ["ordinary"], "moving": ["moving"]} for k in keys}
    tree = dict(which="caps", EPS=10, M=100000, R=100, text_atoms={"Text(row)": 100},
                assumed={"s(String)": 24}, forms={"ordinary": {"1": 1000}, "moving": {"1": 50},
                 "TAV_W": {"1": 600}, "TAV_X": {"1": 800}}, exprs={},
                phases={"sparse": deepcopy(phase), "dense": deepcopy(phase)})
    new = deepcopy(tree); new["forms"]["ordinary"]["1"] = 700
    new["forms"]["TAV_W"]["1"] = 400
    (work / "profile_tree.json").write_text(json.dumps(tree))
    (successor / "profile_tree.json").write_text(json.dumps(new))
    report = {"modes": {m: {"phases": {}, "admission_max": {}} for m in ("sparse", "dense")}}
    (work / "g4_caps.caps.eps10.out.json").write_text(json.dumps(report))
    result = MODEL["merge_success_tree"](work, successor, SOURCE)
    assert result["text_atoms"] == tree["text_atoms"] and result["assumed"] == tree["assumed"]
    assert result["forms"]["TAV_W"] == {"1": 600} and result["forms"]["TAV_X"] == {"1": 800}
    # Existing runtime picks these exact full form names for upward 5%.
    assert (result["forms"]["TAV_W"]["1"] + 19) // 20 == 30
    for mode in result["phases"]:
        for key in keys:
            expected = "success__ordinary" if key[:2] in ("W3", "W4", "W5") else "ordinary"
            assert result["phases"][mode][key]["requested"] == [expected]
    output = json.loads((work / "g4_caps.caps.eps10.out.json").read_bytes())
    assert set(output["modes"]["dense"]["phases"][keys[0]]) == {"requested", "moving_extra", "E_mov", "E_mov_plus_R", "fraction_of_M"}
    # Renaming both branches together must not invent a new phase identity.
    bad_tree, bad_success = deepcopy(tree), deepcopy(new)
    for item in (bad_tree, bad_success):
        item["phases"]["dense"]["W3 renamed"] = item["phases"]["dense"].pop(keys[2])
    (work / "profile_tree.json").write_text(json.dumps(bad_tree))
    (successor / "profile_tree.json").write_text(json.dumps(bad_success))
    with pytest.raises(ValueError, match="phase identities"):
        MODEL["merge_success_tree"](work, successor, SOURCE)
    (work / "profile_tree.json").write_text(json.dumps(tree))
    bad_success = deepcopy(new); bad_success["L"] = {"row": 99}
    (successor / "profile_tree.json").write_text(json.dumps(bad_success))
    with pytest.raises(ValueError, match="spelling maxima"):
        MODEL["merge_success_tree"](work, successor, SOURCE)
    new["assumed"]["s(String)"] = 25
    (successor / "profile_tree.json").write_text(json.dumps(new))
    with pytest.raises(ValueError, match="illustrative atom"):
        MODEL["merge_success_tree"](work, successor, SOURCE)
