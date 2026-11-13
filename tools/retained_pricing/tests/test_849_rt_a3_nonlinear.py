"""RT-A3 increment 4 (§8.49): nonlinear supports on the exact route.

RTA3NL-S6 corrects W16C-ES's whole-edge zero of validate_profile -> carries_fields. Increment 4 forms the S6 context
once per validate_profile call, before the support loop, so an admitted plain EX D1 model (no component, no nonlinear
support) evaluates carries_fields: the edge is charged at one call per validate_profile call (the case loop's
equivalent-static call stays 0 by D1.5). RTA3NL-RD bounds the nonlinear reader's two D1-empty loops at 0 (D1.6 refuses
every nonlinear support, so no `nonlinear_support_*` row is published), RTA3NL-ID bounds a support state at 8 B, and
RTA3NL-PP re-keys `problem`'s parents after validate_profile by +11. RT-A3 S6 (§8.59) deletes the S6 context, so
on the current source the edge is zero again (RTA3S6-VP) and the parents move -10; the increment-4 charge is history.
Each mutation below stops the run or breaks the premise. These are source warrants, not runtime allocation
observations.
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
EDGE = ("pressure_runtime.rs:233:validate_profile", "case_state/mod.rs:36:carries_fields")   # RT-L2: 211 -> 233, unchanged text

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
def test_the_s6_context_is_gone_and_the_edge_is_zero_again(monkeypatch, route):
    """RT-A3 S6 (§8.59, RTA3S6-VP) deletes the S6 context: validate_profile's only carries_fields call is the case
    loop's, inside `case.equivalent_static.is_some()` (0 on D1, W16C-ES), so RTA3NL-S6's per-call charge no longer
    applies and the edge is zero again."""
    d = decisions(monkeypatch, route=route)
    assert EDGE in edges(d, "O10_EDGE_ZERO")
    assert not [e for e in d["EDGE_PER_CALL"] if (str(e[0]).split("/src/")[-1], str(e[1]).split("/src/")[-1]) == EDGE]


def test_validate_profile_calls_carries_fields_only_behind_equivalent_static():
    text = (SOURCE / PR).read_text(encoding="utf-8")
    body = text[text.index("pub(crate) fn validate_profile("):]
    body = body[:body.index("\n}\n")]
    assert S6 not in body and "nonlinear_support_admission" not in body
    assert body.count("carries_fields(") == 1
    assert body.index("        if case.equivalent_static.is_some() {\n") < body.index("carries_fields(")


@pytest.mark.parametrize("old, new", [
    # The S6 context restored verbatim before the support loop: an unconditional call again.
    ("        for support in &model.supports {\n            if let Some(family) = support_family(support) {\n",
     S6 + "        for support in &model.supports {\n            if let Some(family) = support_family(support) {\n"),
    # Another unconditional call of carries_fields in validate_profile.
    ("        for support in &model.supports {\n            if let Some(family) = support_family(support) {\n",
     "        let _again = super::case_state::carries_fields(model);\n"
     "        for support in &model.supports {\n            if let Some(family) = support_family(support) {\n"),
])
def test_a_restored_or_added_carries_fields_call_stops(tmp_path, monkeypatch, old, new):
    snap = copy_core(tmp_path)
    edit(snap, PR, old, new)
    with pytest.raises(SystemExit, match="RTA3NL-S6|W16C-ES|RTA3S6-VP"):
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
    assert keys[:2] == ["209:exact_member_recovery", "233:validate_profile"]   # RT-L2: +22, unchanged text
    # RT-A3 S6 (§8.59): -10 after validate_profile, whose S6 context is gone.
    assert "544:refuse_compensated_shared_terminals" in keys and "2588:collinear_run" in keys   # RT-L2: +22/+381; final source: +126; third scrutiny: +119; fourth: +2
    for key in keys:
        line, name = key.split(":")
        assert text[int(line) - 1].lstrip().startswith(("fn " + name, "pub(crate) fn " + name)), key
