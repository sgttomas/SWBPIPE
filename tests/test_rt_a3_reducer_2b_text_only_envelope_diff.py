"""Reducer 2b regenerates current producer fixtures, never numerical references.

The preserved endpoint is reducer 2a's, integrated main 2b743781a (2a with
S2); test_rt_a3_reducer_text_only_envelope_diff keeps the 2a stage's proof
frozen at that endpoint. Across this stage only formulation_basis.limitations[0]
(the pressure-2 identity, restated from 2a's text to 2b's) and output binding
hashes may change. Exact numeric tokens, member/array order, row sets,
diagnostics, selection and every input-scoped hash (including embedded JSON
and the compact retained corpus) remain invariant, as in the 2a stage.

Every file listed here carries the identity at 2a's endpoint, so each must
move it (a stale carrier fails). The retained carriers are produced only by a
registered build: until the reviewed profile rebinding the two table inputs is
adopted, the build is Stale and their writers cannot regenerate them, so their
cases stay failing rather than being exempted.
"""
from pathlib import Path
import json
import subprocess

import pytest

from tests.test_o4_text_only_envelope_diff import Obj, load
from tests.test_rt_a3_reducer_text_only_envelope_diff import (
    FILES as FILES_2A, NEW as OLD, check_j7_envelope_digests_only, check_rv120_output_digests, compare,
)

ROOT = Path(__file__).resolve().parents[1]
BASE = "2b743781a2fbe533cb317bbd1dce10a394ce53c3"
CLAUSE_2A = (
    "reducers when their mapped member lies outside every pressure region or forms a concentric pressure-bore step, "
    "the only member of its own region, collinear between single straight members of other regions with "
    "transfers_to_wall at both ends, whose junction caps carry the step's pressure load "
    "(EXACT_PRESSURE_REDUCER_IN_REGION otherwise)"
)
CLAUSE_2B = (
    "reducers when their mapped member lies outside every pressure region or forms a pressure step, the only member "
    "of its own region between single straight members of other regions with transfers_to_wall at both ends, either "
    "concentric and collinear or eccentric or slanted with W-15c's run evidence and any read offset accepted, whose "
    "junction caps at the actual nodes along each member's own axis carry the step's pressure force and moment "
    "(EXACT_PRESSURE_REDUCER_IN_REGION otherwise)"
)
assert OLD.count(CLAUSE_2A) == 1
NEW = OLD.replace(CLAUSE_2A, CLAUSE_2B)
assert NEW != OLD

RETAINED = tuple(dict.fromkeys((
    "fixtures/results/retained_precision_cases.json",
    "fixtures/results/retained_precision_rv120_b3_inputs.json",
    *(f"fixtures/results/retained_precision_exact_successor_{kind}{mode}.json"
      for kind in ("", "derivative_") for mode in ("sparse_interactive", "dense_scrutiny")),
    *(f"fixtures/results/retained_precision_o4_w1e_{kind}successor_{mode}.json"
      for kind, modes in (("", ("sparse_interactive", "dense_scrutiny")),
                          ("mixed_", ("sparse_interactive",)),
                          ("unavailable_", ("sparse_interactive",))) for mode in modes),
)))
ORDINARY = tuple(dict.fromkeys((
    *(name for name in FILES_2A if name not in RETAINED and name != "fixtures/results/retained_precision_reader_outcomes.json"),
    "fixtures/results/pressure_u4b_arc_sparse.json",
    "fixtures/results/pressure_v3_joint_reader_control.json",
    "fixtures/model_operations/pressure_v3_l_line_authoring.json",
    "fixtures/reports/exact_route_limitations.json",
    "fixtures/reports/invented/user_release_exact/pressure_2_straight.json",
    "fixtures/reports/invented/exact_report_sections_parity.json",
    *(f"fixtures/reports/invented/exact_report_package_requests/pressure_2_{kind}.json"
      for kind in ("connector", "load_reference_states", "realized_bend", "straight")),
    *(f"fixtures/product_preview/pressure/load_reference/{kind}-{mode}.raw.json"
      for kind in ("connected", "pressure") for mode in ("sparse_interactive", "dense_scrutiny")),
    *(f"core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-{mode}.raw.json"
      for mode in ("sparse_interactive", "dense_scrutiny")),
)))
assert not set(ORDINARY) & set(RETAINED)


def _at(rev, name):
    return subprocess.check_output(["git", "show", f"{rev}:{name}"], cwd=ROOT, text=True)


def _states_old(name):
    text = _at(BASE, name)
    return any(form in text for form in (json.dumps(OLD)[1:-1], json.dumps(OLD, ensure_ascii=False)[1:-1]))


# pressure-2 table digest at 2a's endpoint and now (the restated admitted-families limitation).
TABLE_SHA256 = ("032e861fc66bf5f34a081b104ec879a39831bfd6472b186954966ea57d65f025",
                "a2db16a515d8e4e1c1bcd6607861df0e7e58cbf1cf82a0b984e2f1c0f128382e")
# The report fixtures state the identity as a statement, not as a published
# formulation basis: only these exact paths (an int matches "#") may carry it.
REPORT_IDENTITY_PATHS = {
    "fixtures/reports/exact_route_limitations.json": (("lists", "#", "limitations", "#"), ("strings", "#")),
    "fixtures/reports/invented/exact_report_sections_parity.json": (
        ("vectors", "#", "expected", "limitations", "#", "statement"),),
}


def _matches(path, pattern):
    return len(path) == len(pattern) and all(
        (isinstance(part, int) if want == "#" else part == want) for part, want in zip(path, pattern))


def _restore_report_identity(value, patterns, path=()):
    """Reverse the authorized restatement at the named report paths only; return (value, count)."""
    if isinstance(value, Obj):
        out, count = [], 0
        for key, item in value:
            item, n = _restore_report_identity(item, patterns, path + (key,))
            out.append((key, item))
            count += n
        return Obj(out), count
    if isinstance(value, list):
        out, count = [], 0
        for i, item in enumerate(value):
            item, n = _restore_report_identity(item, patterns, path + (i,))
            out.append(item)
            count += n
        return out, count
    if type(value) is str and any(_matches(path, p) for p in patterns):
        if value == NEW:
            return OLD, 1
        # A report statement that quotes the pressure-2 table's digest: only that digest moves.
        if TABLE_SHA256[1] in value:
            return value.replace(TABLE_SHA256[1], TABLE_SHA256[0]), 1
    return value, 0


def check(name):
    before, after = _at(BASE, name), (ROOT / name).read_text()
    rv120 = name == "fixtures/results/retained_precision_rv120_b3_inputs.json"
    if rv120:
        check_rv120_output_digests(json.loads(before), json.loads(after), base=BASE)
    if name in REPORT_IDENTITY_PATHS:
        restored, moved = _restore_report_identity(load(after), REPORT_IDENTITY_PATHS[name])
        # With the identity reversed at its named paths, nothing but binding hashes may differ.
        assert compare(load(before), restored, old=OLD, new=NEW) == 0, name
        assert moved > 0, f"{name}: still states reducer 2a's identity"
        return moved
    moved = compare(load(before), load(after), rv120_output_hashes=rv120, old=OLD, new=NEW)
    # A carrier that stated 2a's identity must now state 2b's: a stale file fails here.
    assert moved > 0 or not _states_old(name), f"{name}: still states reducer 2a's identity"
    return moved


@pytest.mark.parametrize("name", ORDINARY)
def test_reducer_2b_ordinary_carriers_move_only_identity_and_binding_hashes(name):
    check(name)


@pytest.mark.parametrize("name", RETAINED)
def test_reducer_2b_retained_carriers_move_only_identity_and_binding_hashes(name):
    check(name)


def test_reducer_2b_j7_moves_only_whole_envelope_digests():
    name = "fixtures/results/j7_rf_census_expected.json"
    check_j7_envelope_digests_only(load(_at(BASE, name)), load((ROOT / name).read_text()))


def test_reducer_2b_guard_rejects_other_moves_and_the_2a_pair():
    before = Obj([("formulation_basis", Obj([("limitations", [OLD])])), ("results", [Obj([("value", 1.0)])])])
    assert compare(before, Obj([("formulation_basis", Obj([("limitations", [NEW])])),
                                ("results", [Obj([("value", 1.0)])])]), old=OLD, new=NEW) == 1
    for changed in (
        # A number token or a row moves beside the identity.
        Obj([("formulation_basis", Obj([("limitations", [NEW])])), ("results", [Obj([("value", 1)])])]),
        Obj([("formulation_basis", Obj([("limitations", [NEW])])), ("results", [])]),
        # 2a's own pair, or an identity planted elsewhere, is not this stage's restatement.
        Obj([("formulation_basis", Obj([("limitations", [NEW + " "])])), ("results", [Obj([("value", 1.0)])])]),
    ):
        with pytest.raises(AssertionError):
            compare(before, changed, old=OLD, new=NEW)
    with pytest.raises(AssertionError):
        compare(Obj([("description", OLD)]), Obj([("description", NEW)]), old=OLD, new=NEW)
    # Input-scoped hashes stay protected in this stage too.
    with pytest.raises(AssertionError):
        compare(Obj([("model_sha256", "1" * 64)]), Obj([("model_sha256", "2" * 64)]), old=OLD, new=NEW)


def test_reducer_2b_report_identity_paths_are_exact():
    name = "fixtures/reports/exact_route_limitations.json"
    good = Obj([("lists", [Obj([("limitations", [NEW, "x"])])]), ("strings", [NEW])])
    restored, moved = _restore_report_identity(good, REPORT_IDENTITY_PATHS[name])
    assert moved == 2 and restored == Obj([("lists", [Obj([("limitations", [OLD, "x"])])]), ("strings", [OLD])])
    # The identity under any other member is not reversed, so the comparison refuses it.
    planted = Obj([("lists", [Obj([("producer", NEW)])])])
    restored, moved = _restore_report_identity(planted, REPORT_IDENTITY_PATHS[name])
    assert moved == 0
    with pytest.raises(AssertionError):
        compare(Obj([("lists", [Obj([("producer", OLD)])])]), restored, old=OLD, new=NEW)


def test_reducer_2b_identity_is_the_producers():
    source = (ROOT / "core/product_physics/src/exact_admission.rs").read_text()
    assert source.count(json.dumps(NEW, ensure_ascii=False)) == 1
    assert json.dumps(OLD, ensure_ascii=False) not in source
