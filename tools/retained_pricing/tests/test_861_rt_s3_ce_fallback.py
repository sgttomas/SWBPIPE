"""RT-S3 CE fallback publication (§8.61): CEF-PJ and CEF-D1.

The CE correction adds one branch to solve_prepared_load_case's support loop and two lib.rs tests. CEF-PJ compares
every existing full-file and case-solve body pin without exactly that delta, so the pins keep S6's reviewed bytes.
CEF-D1 binds the branch, its receiver, predicate, classifier, force assignment and D1.6, and zeroes the one new call
edge on D1. Threat model: a changed CE receiver, admission, classifier, support loop or branch-owned allocation must
refuse; the unchanged S6 and CE cohorts price. Source hashes alone are not the proof. These are source warrants, not
runtime allocation observations.
"""
from pathlib import Path
import hashlib
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
BASE = PRICING / "base"
SOURCE = PRICING.parents[1]
LIB = "core/product_physics/src/lib.rs"
RM = "core/product_physics/src/retained_memory.rs"
CE = runpy.run_path(str(BASE / "ce_fallback_model.py"))

# label -> (base module, entry point)
GUARDS = {
    "B7a": ("b7a_domain_model.py", "inspect"),
    "KD5C4": ("kd5_text_model.py", "primitive_lib_minmax_callers"),
    "KD5T2": ("kd5_text_model.py", "inspect"),
    "B5O1": ("batch5_model.py", "active"),
    "KD5C6": ("kd5_first_error_model.py", "inspect"),
    "O4": ("o4_owners_model.py", "inspect"),
    "W1": ("w1_reductions_model.py", "inspect"),
    "CEF": ("ce_fallback_model.py", "inspect"),
}
FULL_FILE = {"B7a", "KD5C4", "W1", "CEF"}
EVERY = set(GUARDS)


def outcomes(snapshot, monkeypatch):
    monkeypatch.syspath_prepend(str(BASE))
    result = {}
    for label, (module, entry) in GUARDS.items():
        try:
            result[label] = runpy.run_path(str(BASE / module))[entry](snapshot)
        except (ValueError, SystemExit) as stop:
            result[label] = stop
    return result


def stopped(result):
    return {label for label, value in result.items() if isinstance(value, (ValueError, SystemExit))}


def decisions(monkeypatch, snapshot=SOURCE, route="L"):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))


def copy_core(tmp_path):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    return tmp_path


def edit(snap, path, old, new):
    target = snap / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1, (path, old)
    target.write_text(text.replace(old, new, 1), encoding="utf-8")
    return snap


def s6_snapshot(tmp_path):
    snap = copy_core(tmp_path)
    text = (SOURCE / LIB).read_text(encoding="utf-8")
    projected = CE["s6_text"](text)
    assert projected is not text
    (snap / LIB).write_text(projected, encoding="utf-8")
    return snap


def test_the_ce_cohort_passes_every_guard_through_the_projection(monkeypatch):
    result = outcomes(SOURCE, monkeypatch)
    assert stopped(result) == set(), result
    assert result["B7a"]["cef_projected"] == [LIB]
    # The projection is S6's reviewed full-file pin, read from the guards, not restated here.
    raw = (SOURCE / LIB).read_bytes()
    pin = runpy.run_path(str(BASE / "b7a_domain_model.py"))["SOURCE_FILES"][LIB]
    assert hashlib.sha256(raw).hexdigest() != pin
    assert hashlib.sha256(CE["s6_bytes"](raw)).hexdigest() == pin
    assert runpy.run_path(str(BASE / "kd5_text_model.py"))["B7A_PRIMITIVE_LIB_SOURCE_PIN"] == pin
    assert runpy.run_path(str(BASE / "w1_reductions_model.py"))["REDUCED_PINS"][LIB] == pin
    assert result["CEF"]["branch_lines"] == 6


def test_the_s6_cohort_is_unchanged_and_adds_no_cef_rule(tmp_path, monkeypatch):
    snap = s6_snapshot(tmp_path)
    result = outcomes(snap, monkeypatch)
    assert stopped(result) == set(), result
    assert result["B7a"]["cef_projected"] == []
    assert result["CEF"] is None
    for route in ("L", "E"):
        assert not [e for e in decisions(monkeypatch, snap, route)["O10_EDGE_ZERO"] if "CEF-D1" in e[2]]


@pytest.mark.parametrize("route", ["L", "E"])
def test_cef_d1_zeroes_exactly_the_one_classifier_call_edge(monkeypatch, route):
    zero = [(str(a), str(b)) for a, b, why in decisions(monkeypatch, route=route)["O10_EDGE_ZERO"] if "CEF-D1" in why]
    assert len(zero) == 1
    caller, callee = zero[0]
    assert caller.startswith("lib.rs:") and caller.endswith(":solve_prepared_load_case")
    assert callee.startswith("lib.rs:") and callee.endswith(":classify_constant_effort_consumption")
    # The predicate stays reached on D1; no other CE, parse or index edge is zeroed by this rule.
    assert not any("is_constant_effort_support" in str(b) or "parse_dof" in str(b) or "dof_index" in str(b)
                   for a, b, why in decisions(monkeypatch, route=route)["O10_EDGE_ZERO"] if "CEF-D1" in why)


def test_the_projection_needs_the_whole_delta_at_its_anchors():
    text = (SOURCE / LIB).read_text(encoding="utf-8")
    start = text.index(CE["TEST_START"])
    end = text.index(CE["TEST_NEXT"])
    only_branch = text[:start] + text[end:]
    only_tests = text.replace(CE["BRANCH"], "", 1)
    for partial in (only_branch, only_tests, text.replace("\n", "\r\n"),
                    text.replace(CE["BRANCH"], CE["BRANCH"] + CE["BRANCH"], 1)):
        assert CE["s6_text"](partial) is partial
    body = runpy.run_path(str(BASE / "kd5_text_model.py"))["function_body"](text, "solve_prepared_load_case")
    assert CE["s6_body"](only_branch, body) is body


BRANCH_LINE = "            } else if support.nonlinear.is_none() && is_constant_effort_support(support) {\n"
CALL_LINE = "                if let Ok(application) = classify_constant_effort_consumption(model, support) {\n"
FORCE_LINE = "                    vector[dof_index(application.dof)] = application.force_newtons;\n"


@pytest.mark.parametrize("path, old, new, expected", [
    # The receiver's nonlinear exclusion dropped: every guard on the case solve and the full file.
    (LIB, BRANCH_LINE, "            } else if is_constant_effort_support(support) {\n", EVERY),
    # The branch moved off its anchor.
    (LIB, BRANCH_LINE, "\n" + BRANCH_LINE, EVERY),
    # The force law changed.
    (LIB, FORCE_LINE, FORCE_LINE.replace("= application", "= 2.0 * application"), EVERY),
    # A branch-owned allocation added.
    (LIB, CALL_LINE, "                let _id = support.id.clone();\n" + CALL_LINE, EVERY),
    # The classifier's force or the hanger-type receiver changed: the full-file pins and CEF-D1.
    (LIB, "                force_newtons: constant_load.value,\n",
     "                force_newtons: constant_load.value * 1000.0,\n", FULL_FILE),
    (LIB, "        .or_else(|| support.family.as_deref().map(str::trim))\n",
     "        .or_else(|| support.family.as_deref())\n", FULL_FILE),
    # The reviewed tests edited: no projection, so every pin compares the changed bytes.
    (LIB, 'vec!["UY".to_string(); 64],', 'vec!["UY".to_string(); 65],', EVERY),
    # D1.6 admits a constant-effort family or stops refusing hangers.
    (RM, '"anchor" | "guide" | "line_stop" | "vertical_support" | "spring")',
     '"anchor" | "guide" | "line_stop" | "vertical_support" | "spring" | "constant_effort_support")',
     {"B7a", "B5O1", "CEF"}),
    (RM, "        if support.hanger.is_some() {\n            return refuse(C::Supports, F::Hanger);\n        }\n", "",
     {"B7a", "B5O1", "CEF"}),
])
def test_an_unreviewed_ce_change_refuses(tmp_path, monkeypatch, path, old, new, expected):
    snap = edit(copy_core(tmp_path), path, old, new)
    result = outcomes(snap, monkeypatch)
    assert expected <= stopped(result), result
    assert "CEF" in stopped(result)


def test_the_reviewed_tests_removed_refuses(tmp_path, monkeypatch):
    text = (SOURCE / LIB).read_text(encoding="utf-8")
    block = text[text.index(CE["TEST_START"]):text.index(CE["TEST_NEXT"])]
    snap = edit(copy_core(tmp_path), LIB, block, "")
    assert stopped(outcomes(snap, monkeypatch)) == EVERY


def test_the_d1_premise_is_read_from_the_admission_source():
    family = runpy.run_path(str(BASE / "kd5_text_model.py"))["_indented_function"](
        (SOURCE / RM).read_text(encoding="utf-8"), "family_clauses")[0]
    assert family.count(CE["D16"]) == 1 and family.count(CE["D16_FAMILIES"]) == 1
    assert family.index(CE["D16"]) < family.index(CE["D16_FAMILIES"])
    listed = [name.strip('"') for name in
              CE["D16_FAMILIES"].split("matches!(family.as_str(), ", 1)[1].split(")", 1)[0].split(" | ")]
    assert listed == list(CE["ADMITTED_FAMILIES"])
    assert "constant_effort_support" not in listed and all(name == name.strip() for name in listed)
