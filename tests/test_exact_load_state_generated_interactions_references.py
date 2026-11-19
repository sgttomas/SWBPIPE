"""Generated loads beside the rest of a load and reference-state document on the exact route
(`validation/references/exact_load_state_generated_interactions/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory, compares them byte for byte with the
committed files and fails if any of its cross-checks disagrees. The independence scan checks the generator's
direct import roots against the standard library, whitespace-free string literals for a product-root path segment,
and calls to named dynamic import and evaluation functions; computed paths, indirect calls and filesystem
containment are outside it. No product test reads the package yet.
"""
import ast
import hashlib
import json
import re
import subprocess
import sys
from fractions import Fraction
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
REFERENCE = PROJECT / "validation" / "references" / "exact_load_state_generated_interactions"
GENERATOR = REFERENCE / "generators" / "generated_interactions_reference.py"


def test_generator_check_rebuilds_the_committed_references():
    result = subprocess.run([sys.executable, "-I", "-B", str(GENERATOR)], cwd=GENERATOR.parent,
                            capture_output=True, text=True, timeout=900)
    assert result.returncode == 0, result.stdout + result.stderr


def test_sha256sums_verify():
    lines = (REFERENCE / "SHA256SUMS").read_text().splitlines()
    assert lines
    listed = set()
    for line in lines:
        digest, path = line.split("  ", 1)
        listed.add(path)
        assert hashlib.sha256((REFERENCE / path).read_bytes()).hexdigest() == digest, path
    assert listed == {"README.md", "references.json", "generators/generated_interactions_reference.py"}


PRODUCT_ROOTS = {"core", "apps", "tests", "tools", "fixtures", "schemas"}
DYNAMIC_CALLS = {"__import__", "import_module", "exec", "eval", "spec_from_file_location"}


def names_product_path(text):
    """A literal names a product path when it is non-empty, has no whitespace and a segment is a product root."""
    return bool(text) and not re.search(r"\s", text) and any(
        segment in PRODUCT_ROOTS for segment in re.split(r"[\\/]+", text))


def test_generator_is_independent_of_the_product():
    tree = ast.parse(GENERATOR.read_text())
    modules = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            modules |= {alias.name.split(".")[0] for alias in node.names}
        elif isinstance(node, ast.ImportFrom):
            assert node.level == 0, "relative import"
            modules.add(node.module.split(".")[0])
        elif isinstance(node, ast.Constant) and isinstance(node.value, str):
            assert not names_product_path(node.value), f"line {node.lineno}: {node.value!r}"
        elif isinstance(node, ast.Call):
            func = node.func
            name = func.id if isinstance(func, ast.Name) else func.attr if isinstance(func, ast.Attribute) else None
            assert name not in DYNAMIC_CALLS, f"line {node.lineno}: call to {name}"
    assert modules, "no imports found"
    assert modules <= set(sys.stdlib_module_names), sorted(modules - set(sys.stdlib_module_names))


def _doc():
    return json.loads((REFERENCE / "references.json").read_text())


def _rows(item):
    return {r["id"]: r for r in item.get("rows", [])}


def test_every_cross_check_agrees():
    doc = _doc()
    assert doc["crosscheck"] and all(c["agrees"] for c in doc["crosscheck"])


def test_the_constant_effort_force_acts_in_every_case_and_cancels_in_every_term():
    doc = _doc()
    for case in doc["cases"]:
        for rid, row in _rows(case).items():
            if rid.startswith("constant_effort/"):
                assert row["expected"]["value"] > 0, (case["id"], rid)
    for term in doc["terms"]:
        ce = [r for rid, r in _rows(term).items() if rid.startswith("constant_effort/")]
        assert ce and all(r["expected"]["exact"] == "0" for r in ce), term["id"]


def test_srss_and_absolute_sum_are_formed_on_every_signed_row_of_the_three_terms():
    doc = _doc()
    terms = {t["id"]: _rows(t) for t in doc["terms"]}
    combos = {c["id"]: c for c in doc["combinations"]}
    srss, absum = _rows(combos["CEF-EQ-SRSS"]), _rows(combos["CEF-EQ-ABS"])
    operands = [terms[o] for o in combos["CEF-EQ-SRSS"]["operands"]]
    assert list(srss) == list(absum) == list(operands[0])
    for rid in srss:
        values = [t[rid]["expected"]["value"] for t in operands]
        assert srss[rid]["expected"]["value"] <= absum[rid]["expected"]["value"], rid
        assert abs(srss[rid]["expected"]["value"] - sum(v * v for v in values) ** 0.5) <= 1e-12 * (
            1 + abs(srss[rid]["expected"]["value"])), rid
        assert abs(absum[rid]["expected"]["value"] - sum(abs(v) for v in values)) <= 1e-12 * (
            1 + absum[rid]["expected"]["value"]), rid
    assert combos["CEF-EQ-SRSS-OF-CASES"]["rows"] == []


def test_each_term_reaction_resultant_balances_the_generated_resultant():
    for term in _doc()["terms"]:
        if "reaction_resultant" in term:
            assert term["reaction_resultant"]["equals_minus_generated_resultant"], term["id"]


def test_the_lift_off_support_lifts_only_above_the_threshold():
    doc = _doc()
    status = {c["id"]: c["expected_support_status"]["support:lo-b"] for c in doc["cases"] if c["model"] == "LO"}
    assert status == {"LO-SUS": "in contact (active)", "LO-EQZ-SEATED": "in contact (active)",
                      "LO-EQZ-LIFTED": "lifted (inactive)"}
    k_star = doc["lift_off_threshold"]["g_factor_z"]["value"]
    k = {c["id"]: float(Fraction(c["seismic"]["g_factors"][2])) for c in doc["cases"]
         if c["model"] == "LO" and c["seismic"]}
    assert k["LO-EQZ-SEATED"] < k_star < k["LO-EQZ-LIFTED"]


def test_the_replaced_span_carries_no_generated_load_and_a_wind_mark_on_it_is_refused():
    doc = _doc()
    for case in doc["cases"]:
        if case["model"] == "JR":
            assert all(g.get("member") != "pipe:b-c" for g in case.get("generated_loads", [])), case["id"]
            assert not any("/pipe:b-c/" in rid for rid in _rows(case)), case["id"]
    refusals = {r["id"]: r for r in doc["refusals"]}
    assert set(refusals) == {"JR-REFUSE-WIND-ON-REPLACED-SPAN", "JR-REFUSE-PART-SPAN-WIND-ON-REPLACED-SPAN",
                             "JR-REFUSE-GENERATED-IDENTITY-COLLISION"}
    assert all(r["code"] is None for r in refusals.values())
    seismic = next(c for c in doc["cases"] if c["id"] == "JR-EQ")
    assert seismic["generated_loads"] and all(any(x["value"] for x in g["intensity_N_per_m"])
                                              for g in seismic["generated_loads"])


def test_whole_and_part_span_wind_marks_have_their_identities():
    doc = _doc()
    part = next(c for c in doc["cases"] if c["id"] == "JR-WIND-PART")
    marks = sorted((g["identity"]["mark"], g["identity"]["member"]) for g in part["generated_loads"])
    assert marks == [("part span", "pipe:c-n"), ("whole span", "pipe:a-m"), ("whole span", "pipe:m-b"),
                     ("whole span", "pipe:n-d")]
    whole = next(c for c in doc["cases"] if c["id"] == "JR-WIND")
    assert [r["expected"] for r in whole["rows"]] != [r["expected"] for r in part["rows"]]


def test_a_generation_only_case_has_a_result_and_an_unlisted_primitive_is_excluded():
    doc = _doc()
    cases = {c["id"]: c for c in doc["cases"]}
    for cid in ("JR-EQ", "JR-WIND", "JR-WIND-PART"):
        assert cases[cid]["load_sources"]["declared_ledger"] == [] and cases[cid]["rows"], cid
    assert cases["JR-EQ-UNLISTED-PRIMITIVE"]["rows_equal"] == "JR-EQ"
    applied = next(c for c in doc["controls"] if c["id"] == "JR-CTRL-UNLISTED-PRIMITIVE-APPLIED")
    assert any(r["expected"]["value"] for r in applied["rows"])


def test_removing_the_device_moves_single_case_rows_and_no_term_row():
    doc = _doc()
    twin = next(c for c in doc["controls"] if c["id"] == "CEF-CTRL-DEVICE-REMOVED-FROM-EVERY-CASE")
    for cid in ("CEF-SUS", "CEF-EQX", "CEF-EQY", "CEF-EQZ"):
        moved = twin["rows_that_move"][cid]
        assert any(r.startswith("reaction/") for r in moved), cid
        with_device = _rows(next(c for c in doc["cases"] if c["id"] == cid))
        without = _rows({"rows": twin["twin_case_rows"][cid]})
        assert all(with_device[r]["expected"] != without[r]["expected"] for r in moved), cid
        assert all(with_device[r]["expected"] == without[r]["expected"] for r in twin["rows_that_do_not_move"][cid])


def test_the_directional_row_separates_srss_absolute_sum_and_signed_sum():
    row = _doc()["directional_row"]
    terms = [t["value"] for t in row["terms"].values()]
    assert min(terms) < 0 < max(terms)
    values = {row[k]["value"] for k in ("srss", "absolute_sum", "signed_sum")}
    assert len(values) == 3
    assert abs(row["signed_sum"]["value"] - sum(terms)) <= 1e-12 * abs(row["signed_sum"]["value"])


def test_axial_member_end_rows_say_which_axial_force_they_are():
    doc = _doc()
    cef = next(c for c in doc["cases"] if c["id"] == "CEF-SUS")
    axial = [r for r in cef["rows"] if "axial_quantity" in r]
    assert len(axial) == 8 and all(r["axial_quantity"].startswith("effective (elastic) member force") for r in axial)


def test_the_refusals_state_their_grounds_and_observable_outcome():
    refusals = {r["id"]: r for r in _doc()["refusals"]}
    for rid in ("JR-REFUSE-WIND-ON-REPLACED-SPAN", "JR-REFUSE-PART-SPAN-WIND-ON-REPLACED-SPAN"):
        assert refusals[rid]["observable_outcome"], rid
        assert "DEL-03-06 AC-004" in refusals[rid]["rests_on"] and "8.1" in refusals[rid]["rests_on"], rid
        assert "pipe:b-c" in refusals[rid]["expected"], rid


def test_the_identity_collision_is_refused_on_agent_0s_decision_for_an_unlisted_primitive():
    doc = _doc()
    collision = next(r for r in doc["refusals"] if r["id"] == "JR-REFUSE-GENERATED-IDENTITY-COLLISION")
    assert collision["colliding_identities"] == ("an unlisted stored primitive of the same case whose identity equals "
                                                 "a generated load's identity")
    assert collision["expected"] == "Refuse the case before assembly."
    assert "Agent 0, 18 November 2026" in collision["rests_on"] and collision["code"] is None
    assert "listed or not" not in json.dumps(doc)
    assert [o["id"] for o in doc["open_readings"]] == ["GENERATED-IDENTITY-EQUALS-LISTED-OR-SYNTHESIZED-IDENTITY"]


def test_every_compared_item_states_its_comparison_floors():
    doc = _doc()
    rule = doc["conventions"]["comparison"]
    assert "nonzero passes when abs(observed - expected) <= 1e-9 * abs(expected)" in rule
    assert "exactly zero passes when abs(observed) <= floor" in rule
    assert "max(" not in rule
    assert "euler_bernoulli" in doc["binding"]
    for item in doc["cases"] + doc["terms"] + doc["combinations"] + doc["controls"]:
        for row in item.get("rows", []):
            assert not any("floor" in k or "tolerance" in k for k in row), (item["id"], row["id"])
            assert not any("floor" in k or "tolerance" in k for k in row["expected"]), (item["id"], row["id"])
    for item in doc["cases"] + doc["terms"] + doc["combinations"]:
        if item.get("rows"):
            floor = item["comparison_floor"]
            assert set(floor) == {"force", "moment", "translation", "rotation"}, item["id"]
            assert all(v > 0 for v in floor.values()), item["id"]
            for family, value in floor.items():
                largest = max(abs(r["expected"]["value"]) for r in item["rows"] if r["family"] == family)
                assert largest > 0 and value == 1e-9 * largest, (item["id"], family)
    assert doc["lift_off_threshold"]["comparison_floor"] == {"dimensionless": 1e-12}


def test_aggregated_axial_rows_are_labelled_as_magnitudes():
    combos = {c["id"]: c for c in _doc()["combinations"]}
    for cid in ("CEF-EQ-SRSS", "CEF-EQ-ABS"):
        axial = [r for r in combos[cid]["rows"] if "axial_quantity" in r]
        assert len(axial) == 8 and all(r["axial_quantity"].startswith("magnitude aggregated") for r in axial), cid
