"""RT-A3 increment 4 (retained-pricing-rules.md §8.49): nonlinear supports on the exact route.

RTA3NL-S6 corrects W16C-ES's whole-edge zero of validate_profile -> carries_fields. Increment 4 forms the S6 context
once per validate_profile call, before the support loop, so an admitted plain EX D1 model (no component, no nonlinear
support) evaluates carries_fields: the edge is charged at one call per validate_profile call (the case loop's
equivalent-static call stays 0 by D1.5). RTA3NL-RD bounds the nonlinear reader's two D1-empty loops at 0 (D1.6 refuses
every nonlinear support, so no `nonlinear_support_*` row is published), RTA3NL-ID bounds a support state at 8 B, and
RTA3NL-PP re-keys `problem`'s parents after validate_profile by +11. Each mutation below stops the run or breaks the
premise. These are source warrants, not runtime allocation observations.
"""
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
PR = "core/product_physics/src/pressure_runtime.rs"
CS = "core/product_physics/src/case_state/mod.rs"
RM = "core/product_physics/src/retained_memory.rs"
EV = "core/reporting/result_export/src/nonlinear_support_evidence.rs"
EDGE = ("pressure_runtime.rs:211:validate_profile", "case_state/mod.rs:36:carries_fields")

S6 = ("        let s6_context = (\n            super::case_state::carries_fields(model),\n"
      "            model.components.iter().any(|component| component.objective_connector.is_some()),\n        );\n")


def decisions(monkeypatch, snapshot=SOURCE, route="E"):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))


def copy_core(tmp_path):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    return tmp_path


def edit(snap, path, old, new):
    target = snap / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1, (path, old)
    target.write_text(text.replace(old, new), encoding="utf-8")


def edges(d, family):
    return [(str(e[0]).split("/src/")[-1], str(e[1]).split("/src/")[-1]) for e in d[family]]


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_s6_call_is_charged_once_per_call_not_zeroed(monkeypatch, route):
    """The review's counterexample: carries_fields runs for every admitted exact D1 model."""
    d = decisions(monkeypatch, route=route)
    assert EDGE not in edges(d, "O10_EDGE_ZERO")
    per_call = [e for e in d["EDGE_PER_CALL"] if (str(e[0]).split("/src/")[-1], str(e[1]).split("/src/")[-1]) == EDGE]
    assert len(per_call) == 1 and per_call[0][2] == "1" and "RTA3NL-S6" in per_call[0][3]


def test_the_s6_statement_is_outside_every_loop_of_validate_profile():
    """The premise of the count 1: the statement sits in the exact-contract arm (every exact document takes it), after
    the reducer-region refusals and before the support loop; carries_fields is the first of two calls."""
    text = (SOURCE / PR).read_text(encoding="utf-8")
    body = text[text.index("pub(crate) fn validate_profile("):]
    body = body[:body.index("\n}\n")]
    arm = body.index("    if let Some(contract) = contract {\n")
    at = body.index(S6 + "        for support in &model.supports {\n")
    between = body[arm:at]
    assert between.count("{") - between.count("}") == 1   # only the arm itself is open
    assert body.count("carries_fields(") == 2 and arm < at < body.index("carries_fields(") < body.rindex(
        "        if case.equivalent_static.is_some() {\n") < body.rindex("carries_fields(")


@pytest.mark.parametrize("old, new", [
    # The S6 context formed per support (a loop around the call): the count 1 no longer holds.
    (S6 + "        for support in &model.supports {\n",
     "        for support in &model.supports {\n" + S6.replace("        ", "            ", 1)),
    # A third call of carries_fields in validate_profile.
    ("        for support in &model.supports {\n            if let Some(family) = support_family(support) {\n"
     "                let verdict",
     "        let _again = super::case_state::carries_fields(model);\n"
     "        for support in &model.supports {\n            if let Some(family) = support_family(support) {\n"
     "                let verdict"),
])
def test_a_moved_or_repeated_s6_call_stops(tmp_path, monkeypatch, old, new):
    snap = copy_core(tmp_path)
    edit(snap, PR, old, new)
    with pytest.raises(SystemExit, match="RTA3NL-S6|W16C-ES"):
        decisions(monkeypatch, snapshot=snap)


def test_a_carries_fields_with_an_owner_stops(tmp_path, monkeypatch):
    """The charged closure is allocation-free only while carries_fields is the four-test predicate."""
    snap = copy_core(tmp_path)
    edit(snap, CS, "        || !model.request_material_expansion_laws.is_empty()\n",
         "        || !format!(\"{}\", model.request_material_expansion_laws.len()).is_empty()\n")
    with pytest.raises(SystemExit, match="RTA3NL-S6"):
        decisions(monkeypatch, snapshot=snap)


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_reader_loops_and_state_class(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    loops = {h: (b, fns) for h, b, why, fns in d["O10_LOOPS_LAST"] if "RTA3NL-RD" in why}
    assert loops == {
        'for row in rows.iter().filter(|r| kind(r).starts_with(" ") && r[" "][" "] != " ")':
            ("0", ["nonlinear_support_evidence.rs:case_ids"]),
        "iter:case_ids(rows)?.into_iter().map": ("0", ["nonlinear_support_evidence.rs:read"]),
    }
    audits = [v for k, v in d["O10_ID_AUDIT"] if str(k).split("/src/")[-1].startswith("nonlinear_support_evidence.rs:")]
    assert [a["s.state"]["bytes"] for a in audits] == [8]
    states = ["inactive", "active", "sticking", "sliding"]
    assert max(len(s.encode()) for s in states) == 8
    assert 'const STATES: [&str; 4] = ["inactive", "active", "sticking", "sliding"];' in (SOURCE / EV).read_text(encoding="utf-8")


@pytest.mark.parametrize("path, old, new", [
    # D1.6 no longer refuses a nonlinear support record: the reader's zeros lose their premise.
    (RM, "        if support.nonlinear.is_some() {\n", "        if support.nonlinear.is_some() && false {\n"),
    # A longer state token.
    (EV, '"sticking", "sliding"];', '"sticking", "sliding_long"];'),
    # A state set from somewhere else.
    (EV, "        let state = STATES[code as usize];\n", "        let state = if code > 3 { \"unrecognised\" } else { STATES[code as usize] };\n"),
    # The reader's case scan widened to every row.
    (EV, '    for row in rows.iter().filter(|r| kind(r).starts_with("nonlinear_support_") && r["basis_ref"]["ref_type"] != "combination") {\n',
     "    for row in rows.iter() {\n"),
])
def test_a_changed_reader_premise_stops(tmp_path, monkeypatch, path, old, new):
    snap = copy_core(tmp_path)
    edit(snap, path, old, new)
    with pytest.raises(SystemExit, match="RTA3NL-RD/ID"):
        decisions(monkeypatch, snapshot=snap)


def test_the_scoped_problem_parents_follow_their_functions():
    """RTA3NL-PP: every parent key names its function's definition line in the current source."""
    sys.path.insert(0, str(PRICING / "base"))
    import b7a_scoped_text_model as scoped
    text = (SOURCE / PR).read_text(encoding="utf-8").splitlines()
    keys = [p.split("pressure_runtime.rs:")[-1] for p in scoped.PROBLEM_PARENTS]
    assert keys[:2] == ["187:exact_member_recovery", "211:validate_profile"]
    assert "532:refuse_compensated_shared_terminals" in keys and "1970:collinear_run" in keys
    for key in keys:
        line, name = key.split(":")
        assert text[int(line) - 1].lstrip().startswith(("fn " + name, "pub(crate) fn " + name)), key
