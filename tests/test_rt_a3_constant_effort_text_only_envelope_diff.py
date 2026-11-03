"""RT-A3 increment 3 regenerates current producer fixtures, never numerical references.

The preserved endpoint is reducer 2b's, integrated main 3983f51cf (#80 and #95;
no carrier differs from #80's merge 6757b2341);
test_rt_a3_reducer_2b_text_only_envelope_diff keeps the 2b stage's proof frozen
there. This stage restates two producer texts (DEL-04-07 `pressure-contract.md`,
"Constant-effort supports (RT-A3 increment 3)"):

- the pressure-2 identity, formulation_basis.limitations[0] of every unselected
  v3 envelope, gains the constant-effort clause after the tee clause;
- the load/reference-state support limitation (S13), a later limitation of every
  load/reference-state envelope, pressure-2 and pressure-source-1 alike, gains
  D-2's sentence before "Spring base motion".

Across this stage only those two fields and output binding hashes may change.
Exact numeric tokens, member/array order, row sets, diagnostics, selection and
every input-scoped hash (including embedded JSON and the compact retained
corpus) remain invariant, as in the 2a and 2b stages. Every file listed here
states one of the two texts at the endpoint, so each must move (a stale carrier
fails), and every JSON file that stated one there is listed or named below.
"""
from pathlib import Path
import json
import subprocess

import pytest

from tests.test_o4_text_only_envelope_diff import HASH, Obj, load, _embedded, _rt_a3_member_path
from tests.test_rt_a3_reducer_text_only_envelope_diff import (
    INPUT_BINDINGS, check_j7_envelope_digests_only, check_rv120_output_digests,
)
from tests.test_rt_a3_reducer_2b_text_only_envelope_diff import (
    NEW as OLD_ID, ORDINARY as ORDINARY_2B, RETAINED, REPORT_IDENTITY_PATHS, _matches,
)

ROOT = Path(__file__).resolve().parents[1]
BASE = "3983f51cfcef6af26a7846bd63f267425379cfd3"

TEE_TAIL = "and a realized arc it names carries none"
CE_CLAUSE = (
    "; a constant-effort support (DEC-049) declaring exactly one translational DOF and a finite positive constant "
    "load is consumed: it has zero stiffness and applies that load as a constant nodal force along the DOF's "
    "positive global axis in every solved case, and a mechanics combination over a model with one is withheld "
    "(CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE) when a factor is not finite or the exact factor sum is not "
    "within 64 epsilon of 1; a record that is not consumed applies nothing and warns "
    "(SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED)"
)
assert OLD_ID.count(TEE_TAIL + ". ") == 1
NEW_ID = OLD_ID.replace(TEE_TAIL + ". ", TEE_TAIL + CE_CLAUSE + ". ")
OLD_LS = (
    "Global rigid support translations/rotations are prescribed absolute boundary values through the partitioned "
    "solve; reactions come from the unreduced equations. Spring base motion, device preload/reference, inactive or "
    "locked supports are not provided."
)
D2 = ("A consumed constant-effort support's constant force is a device action of the active support in every "
      "state. ")
NEW_LS = OLD_LS.replace("equations. Spring", "equations. " + D2 + "Spring")
assert NEW_ID != OLD_ID and NEW_LS != OLD_LS
# The restated fields, old -> new; the identity only at limitations[0].
PAIRS = {OLD_ID: NEW_ID, OLD_LS: NEW_LS}
# ASCII needles (no escape differs between JSON spellings) that only the old texts contain.
NEEDLES = ("a realized arc it names carries none. Pressure regions meeting",
           "reactions come from the unreduced equations. Spring base motion")
# pressure-2 table digest at the endpoint and now (admitted families, combinations limitation, gate 4).
TABLE_SHA256 = ("a2db16a515d8e4e1c1bcd6607861df0e7e58cbf1cf82a0b984e2f1c0f128382e",
                "be29fabcfacb1070a51bfb56dcfe61b4e4125638e249c3c0e96d092cc8aba43f")

# The pressure-source-1 load/reference-state carriers state S13 (not the pressure-2 identity).
SOURCE_LOAD_STATE = tuple(
    [f"fixtures/product_preview/pressure_source/load_reference_source/{name}-{mode}.raw.json"
     for name in ("n05", "n06", "eigen_motion", "fields", "mixed")
     for mode in ("sparse_interactive", "dense_scrutiny")]
    + [f"fixtures/results/pressure_source_load_state_{name}_{mode}.{kind}.json"
       for name in ("eigen_motion", "fields", "mixed", "n05", "n06")
       for mode in ("dense", "sparse")
       for kind in ("document", "analysis_run", "stress_neutral")]
    + ["fixtures/results/pressure_source_reader_corpus.json"]
)
ORDINARY = tuple(dict.fromkeys((*ORDINARY_2B, *SOURCE_LOAD_STATE)))
assert not set(ORDINARY) & set(RETAINED)
J7 = "fixtures/results/j7_rf_census_expected.json"
# JSON files that state an old text at the endpoint but are not compared field by field here.
NOT_COMPARED = {
    # The report statement table is keyed and sorted by each producer string's SHA-256, so S1 and S13 move
    # with their digests; its writer (exact_route_limitations.rs) and the desktop registry check bind it.
    "fixtures/reports/exact_report_statements.json",
}


def _at(rev, name):
    return subprocess.check_output(["git", "show", f"{rev}:{name}"], cwd=ROOT, text=True)


def _states_old(text):
    return any(needle in text for needle in NEEDLES)


def compare(before, after, path=(), *, rv120_output_hashes=False):
    """Return the restated-field count; refuse every non-binding or numerical move (2a's rule, two fields)."""
    assert type(before) is type(after), path
    if isinstance(before, Obj):
        assert [k for k, _ in before] == [k for k, _ in after], path
        if dict(before).get("payload_scope") in {"model", "request", "input_manifest"}:
            assert before == after, path
        return sum(compare(a, b, _rt_a3_member_path(before, key, path), rv120_output_hashes=rv120_output_hashes)
                   for (key, a), (_, b) in zip(before, after))
    if isinstance(before, list):
        assert len(before) == len(after), path
        return sum(compare(a, b, path + (i,), rv120_output_hashes=rv120_output_hashes)
                   for i, (a, b) in enumerate(zip(before, after)))
    if (type(before) is str and before in PAIRS and len(path) >= 3
            and path[-3:-1] == ("formulation_basis", "limitations") and isinstance(path[-1], int)
            and (before != OLD_ID or path[-1] == 0)):
        assert after == PAIRS[before], path
        return 1
    if before == after:
        return 0
    if type(before) is str:
        if HASH.fullmatch(before) and HASH.fullmatch(after):
            rv120_digest = (rv120_output_hashes and len(path) == 3 and path[0] == "shapes"
                            and isinstance(path[1], int) and path[2] == "input_sha256")
            assert rv120_digest or not INPUT_BINDINGS.intersection(path), path
            assert before.startswith("sha256:") == after.startswith("sha256:"), path
            return 0
        a, b = _embedded(before), _embedded(after)
        if a is not None and b is not None:
            return compare(a, b, path + ("<json>",), rv120_output_hashes=rv120_output_hashes)
    raise AssertionError(path)


def _restore_report_statements(value, patterns, path=()):
    """Reverse the authorized restatements at the named report paths only; return (value, count)."""
    if isinstance(value, Obj):
        out, count = [], 0
        for key, item in value:
            item, n = _restore_report_statements(item, patterns, path + (key,))
            out.append((key, item))
            count += n
        return Obj(out), count
    if isinstance(value, list):
        out, count = [], 0
        for i, item in enumerate(value):
            item, n = _restore_report_statements(item, patterns, path + (i,))
            out.append(item)
            count += n
        return out, count
    if type(value) is str and any(_matches(path, p) for p in patterns):
        for old, new in PAIRS.items():
            if value == new:
                return old, 1
        # A report statement that quotes the pressure-2 table's digest: only that digest moves.
        if TABLE_SHA256[1] in value:
            return value.replace(TABLE_SHA256[1], TABLE_SHA256[0]), 1
    return value, 0


def check(name, after=None):
    before = _at(BASE, name)
    after = (ROOT / name).read_text() if after is None else after
    if name == "fixtures/results/retained_precision_reader_outcomes.json":
        # Input hashes there bind result statements plus invocations; keys, readings and verdict summaries
        # are compared token for token (r1_native_reader_outcomes checks the materialization).
        compare(load(before), load(after))
        assert before != after, "reader outcomes still state the predecessor bindings"
        return 0
    rv120 = name == "fixtures/results/retained_precision_rv120_b3_inputs.json"
    if rv120:
        check_rv120_output_digests(json.loads(before), json.loads(after), base=BASE)
    if name in REPORT_IDENTITY_PATHS:
        restored, moved = _restore_report_statements(load(after), REPORT_IDENTITY_PATHS[name])
        moved += compare(load(before), restored)
        assert moved > 0, f"{name}: still states the endpoint's texts"
        return moved
    moved = compare(load(before), load(after), rv120_output_hashes=rv120)
    # A carrier that stated an old text must now state the new one: a stale file fails here.
    assert moved > 0 or not _states_old(before), f"{name}: still states the endpoint's texts"
    return moved


@pytest.mark.parametrize("name", ORDINARY)
def test_constant_effort_ordinary_carriers_move_only_the_two_texts_and_binding_hashes(name):
    check(name)


@pytest.mark.parametrize("name", RETAINED)
def test_constant_effort_retained_carriers_move_only_the_identity_and_binding_hashes(name):
    check(name)


def test_constant_effort_j7_moves_only_whole_envelope_digests():
    check_j7_envelope_digests_only(load(_at(BASE, J7)), load((ROOT / J7).read_text()))


def test_every_json_file_stating_an_old_text_is_compared_or_named():
    """Completeness: the listed carriers are every JSON file that stated either text at the endpoint, outside
    frozen references, design records and recorded runs; none states it now."""
    listed = set(ORDINARY) | set(RETAINED) | {J7} | NOT_COMPARED

    def stating(rev):
        found = set()
        for needle in NEEDLES:
            args = ["git", "grep", "-l", "-F", "-e", needle] + ([rev] if rev else []) + [
                "--", "*.json", ":!validation/", ":!execution/", ":!fixtures/persistence/recorded_model_*"]
            out = subprocess.run(args, cwd=ROOT, text=True, capture_output=True).stdout
            found |= {line.split(":", 1)[1] if rev else line for line in out.splitlines()}
        return found

    at_base = stating(BASE)
    assert at_base - listed == set()
    assert stating(None) == set(), "a JSON file still states an endpoint text"
    # Each listed carrier stated an old text at the endpoint (a listed file that did not is a list error).
    assert {n for n in listed if n not in at_base and n != J7} <= {
        n for n in listed if not _states_old(_at(BASE, n))}


def test_constant_effort_guard_rejects_other_moves():
    before = Obj([("formulation_basis", Obj([("limitations", [OLD_ID, "x", OLD_LS])])),
                  ("results", [Obj([("value", 1.0)])])])
    after = Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "x", NEW_LS])])),
                 ("results", [Obj([("value", 1.0)])])])
    assert compare(before, after) == 2
    for changed in (
        # A number token or a row moves beside the texts.
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "x", NEW_LS])])), ("results", [Obj([("value", 1)])])]),
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "x", NEW_LS])])), ("results", [])]),
        # Another limitation moves, or a text is restated otherwise.
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "y", NEW_LS])])), ("results", [Obj([("value", 1.0)])])]),
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID + " ", "x", NEW_LS])])),
             ("results", [Obj([("value", 1.0)])])]),
        Obj([("formulation_basis", Obj([("limitations", [NEW_ID, "x", OLD_LS + " "])])),
             ("results", [Obj([("value", 1.0)])])]),
    ):
        with pytest.raises(AssertionError):
            compare(before, changed)
    # The identity is restated only as limitation 0; neither text elsewhere.
    with pytest.raises(AssertionError):
        compare(Obj([("formulation_basis", Obj([("limitations", ["x", OLD_ID])]))]),
                Obj([("formulation_basis", Obj([("limitations", ["x", NEW_ID])]))]))
    for old, new in PAIRS.items():
        with pytest.raises(AssertionError):
            compare(Obj([("description", old)]), Obj([("description", new)]))
    # Input-scoped hashes stay protected in this stage too.
    with pytest.raises(AssertionError):
        compare(Obj([("model_sha256", "1" * 64)]), Obj([("model_sha256", "2" * 64)]))


def test_constant_effort_report_paths_are_exact():
    name = "fixtures/reports/exact_route_limitations.json"
    good = Obj([("lists", [Obj([("limitations", [NEW_ID, "x", NEW_LS])])]), ("strings", [NEW_ID, NEW_LS])])
    restored, moved = _restore_report_statements(good, REPORT_IDENTITY_PATHS[name])
    assert moved == 4 and restored == Obj([("lists", [Obj([("limitations", [OLD_ID, "x", OLD_LS])])]),
                                           ("strings", [OLD_ID, OLD_LS])])
    planted = Obj([("lists", [Obj([("producer", NEW_LS)])])])
    restored, moved = _restore_report_statements(planted, REPORT_IDENTITY_PATHS[name])
    assert moved == 0
    with pytest.raises(AssertionError):
        compare(Obj([("lists", [Obj([("producer", OLD_LS)])])]), restored)


def test_constant_effort_texts_are_the_producers():
    source = (ROOT / "core/product_physics/src/exact_admission.rs").read_text()
    for old, new in PAIRS.items():
        assert source.count(json.dumps(new, ensure_ascii=False)) == 1
        assert json.dumps(old, ensure_ascii=False) not in source
    assert _at(BASE, "core/product_physics/src/exact_admission.rs").count(json.dumps(OLD_LS)) == 1
