"""§8.43 MODAL-SEAM's standing tuple (SEAM-ST) and §8.49 RTA3NL's reader (NL-RD, NL-RQ), re-bound at the DEL-04-07
publication-basis decision (base/nonlinear_seams.py).

Each guard holds at the current source, and each mutant fails it by name. lib.rs is pinned whole by earlier models,
so in the decision chain a lib.rs edit stops there first; the SEAM-ST tests run the guard itself. The nonlinear reader
has no earlier whole-file pin, so its chain test edits the file and expects RTA3NL-RD's stop.

These are source warrants, not runtime allocation observations.
"""
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
LIB = SOURCE / "core/product_physics/src/lib.rs"
NL = "core/reporting/result_export/src/nonlinear_support_evidence.rs"
SEAMS = runpy.run_path(str(PRICING / "base/nonlinear_seams.py"))
FN = runpy.run_path(str(PRICING / "base/rta5_held_role.py"))["_fn"]


def _case(lib=None):
    return FN(lib if lib is not None else LIB.read_text(encoding="utf-8"), "solve_prepared_load_case")


def _mutant(text, old, new):
    assert text.count(old) == 1, old
    return text.replace(old, new)


def _nl():
    return (SOURCE / NL).read_text(encoding="utf-8")


def _reader(text):
    production = text.split("#[cfg(test)]")[0]
    return SEAMS["reader_failures"](production, FN(text, "read"))


# ---- SEAM-ST

def test_the_standing_tuple_holds_at_the_current_source():
    assert SEAMS["standing_failures"](_case()) == []


@pytest.mark.parametrize("old,new", [
    # the None arm builds something
    ("            None => (None, None, None, None),", "            None => (None, None, None, Some(Default::default())),"),
    # nonlinear_staged re-assigned
    ("    let released = confirmed.is_some() && nonlinear_staged.is_some();\n",
     "    let nonlinear_staged = nonlinear_staged.or(None);\n    let released = confirmed.is_some() && nonlinear_staged.is_some();\n"),
    # the release no longer waits for the decision
    ("    if let Some(staged) = nonlinear_staged.filter(|_| released) {\n",
     "    if let Some(staged) = nonlinear_staged {\n"),
    # a further read of the staged rows
    ("    let released = confirmed.is_some() && nonlinear_staged.is_some();\n",
     "    let released = confirmed.is_some() && nonlinear_staged.is_some();\n    let _rows = nonlinear_staged.as_ref();\n"),
], ids=["none-arm-builds", "re-assigned", "release-unconditional", "further-read"])
def test_a_changed_standing_tuple_stops(old, new):
    lib = _mutant(LIB.read_text(encoding="utf-8"), old, new)
    failed = SEAMS["standing_failures"](_case(lib))
    assert any(f.startswith("SEAM-ST ") for f in failed), failed


# ---- NL-RD / NL-RQ

READ_REQUIRE = ('    require(cases.is_empty() || source["status"]["mechanics"] == "MECHANICS_SOLVED", '
                '"ROWS_IN_UNSOLVED_DOCUMENT")?;\n')
CASE_REQUIRE = ('    require(code != crate::semantic_contract::INSPECTION_ONLY_CODE, '
                '"INSPECTION_ONLY_IN_SOLVED_DOCUMENT")?;\n')


def test_the_reader_holds_at_the_current_source():
    failed, loop = _reader(_nl())
    assert failed == [] and loop == "iter:cases.into_iter().map"


@pytest.mark.parametrize("old,new,names", [
    # the message is built before the call: an allocation whether or not the condition holds
    (READ_REQUIRE,
     '    let message = format!("ROWS_IN_UNSOLVED_DOCUMENT");\n'
     '    require(cases.is_empty() || source["status"]["mechanics"] == "MECHANICS_SOLVED", &message)?;\n',
     ("NL-RD ", "NL-RQ ")),
    # require takes an owned message
    ("fn require(ok: bool, code: &str) -> Check {\n", "fn require(ok: bool, code: String) -> Check {\n", ("NL-RQ ",)),
    # the empty-case short circuit removed
    (READ_REQUIRE,
     '    require(source["status"]["mechanics"] == "MECHANICS_SOLVED", "ROWS_IN_UNSOLVED_DOCUMENT")?;\n', ("NL-RD ",)),
    # read_case's message formatted at the call
    (CASE_REQUIRE,
     '    require(code != crate::semantic_contract::INSPECTION_ONLY_CODE, &format!("INSPECTION_ONLY_{code}"))?;\n',
     ("NL-RD ", "NL-RQ ")),
], ids=["message-built-before-call", "owned-message", "short-circuit-removed", "read-case-message-formatted"])
def test_a_changed_reader_stops(old, new, names):
    failed, _ = _reader(_mutant(_nl(), old, new))
    for name in names:
        assert any(f.startswith(name) for f in failed), (name, failed)


def test_the_chain_stops_by_name_on_a_changed_reader(tmp_path, monkeypatch):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    target = tmp_path / NL
    target.write_text(_mutant(target.read_text(encoding="utf-8"), READ_REQUIRE,
                              '    require(source["status"]["mechanics"] == "MECHANICS_SOLVED", "ROWS_IN_UNSOLVED_DOCUMENT")?;\n'),
                      encoding="utf-8")
    monkeypatch.setenv("SQ2_SNAP", str(tmp_path))
    with pytest.raises(SystemExit, match=r"RTA3NL-RD: .*NL-RD "):
        runpy.run_path(str(PRICING / "rules/sq2_decisions_L.py"))


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_reader_loop_is_keyed_at_the_current_header(monkeypatch, route):
    monkeypatch.setenv("SQ2_SNAP", str(SOURCE))
    d = runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))
    loops = [r for r in d["O10_LOOPS_LAST"] if r[3] == ["nonlinear_support_evidence.rs:read"]]
    assert [(r[0], r[1]) for r in loops] == [("iter:cases.into_iter().map", "0")]
