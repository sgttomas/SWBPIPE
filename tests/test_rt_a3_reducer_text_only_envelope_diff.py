"""Reducer 2a regenerates current producer fixtures, never numerical references.

The preserved increment-1 endpoint is integrated main 2656dcb33. Across this
stage, only formulation_basis.limitations[0] and binding hashes may change.
Exact numeric tokens, member/array order, row sets, diagnostics and selection
remain invariant. The old O4/W15/tee proofs keep their historical endpoints.
"""
from pathlib import Path
from copy import deepcopy
import hashlib
import json
import subprocess

import pytest

from tests.test_o4_text_only_envelope_diff import (
    HASH, Obj, RT_A3_IDENTITY_NEW, RT_A3_LISTED, load, _embedded,
    _rt_a3_member_path, classify, Token,
)

ROOT = Path(__file__).resolve().parents[1]
BASE = "2656dcb33328b12c5572a30c10124a40f35adffc"
OLD = RT_A3_IDENTITY_NEW
CLAUSE = (
    "reducers when their mapped member lies outside every pressure region or forms a concentric pressure-bore step, "
    "the only member of its own region, collinear between single straight members of other regions with "
    "transfers_to_wall at both ends, whose junction caps carry the step's pressure load "
    "(EXACT_PRESSURE_REDUCER_IN_REGION otherwise)"
)
NEW = OLD.replace(
    "reducers only when their mapped member lies outside every pressure region "
    "(EXACT_PRESSURE_REDUCER_IN_REGION otherwise)", CLAUSE,
)
assert NEW != OLD
FILES = tuple(dict.fromkeys((*RT_A3_LISTED,
    "fixtures/results/retained_precision_cases.json",
    "fixtures/results/retained_precision_reader_outcomes.json",
    "fixtures/results/pressure_v3_combination_reader_corpus.json",
    "fixtures/results/pressure_arc_tangent_above_one_sparse.json",
    *(f"fixtures/results/retained_precision_o4_w1e_{kind}successor_{mode}.json"
      for kind, modes in (("", ("sparse_interactive", "dense_scrutiny")),
                          ("mixed_", ("sparse_interactive",)),
                          ("unavailable_", ("sparse_interactive",))) for mode in modes),
    *(f"fixtures/results/pressure_plain_connected_{mode}.{kind}.json"
      for mode in ("sparse", "dense") for kind in ("document", "analysis_run", "stress_neutral")),
    *(f"apps/desktop/src/test-support/desktop-shaped/desktop_shaped_pressure_plain_connected_{mode}.{kind}.json"
      for mode in ("sparse", "dense") for kind in ("analysis_run", "stress_neutral")),
    *(f"fixtures/reports/invented/exact_report_witness_inputs/pressure_2_{kind}-sparse_interactive.raw.json"
      for kind in ("connector", "realized_bend")),
)))

# These bind the authored input, which the regeneration does not change. A
# report-package `request` can also contain output carriers, so protect its
# named model/manifest children rather than that whole transport wrapper.
INPUT_BINDINGS = frozenset({
    "invocation", "model", "authored_model", "corner_document", "input_manifest",
    "model_hash", "model_sha256", "model_document_sha256", "document_model_sha256",
    "request_hash", "request_sha256", "input_hash", "input_sha256",
    "input_manifest_hash", "input_manifest_hashes", "input_manifest_sha256",
    "audit_input_manifest_sha256",
})


def compare(before, after, path=(), *, rv120_output_hashes=False):
    """Return identity-field count; refuse every non-binding or numerical move."""
    assert type(before) is type(after), path
    if isinstance(before, Obj):
        assert [k for k, _ in before] == [k for k, _ in after], path
        # AnalysisRun digest lists name their subject in the record, not in
        # the path to `value`. Input-scoped records must stay unchanged too.
        if dict(before).get("payload_scope") in {"model", "request", "input_manifest"}:
            assert before == after, path
        return sum(compare(a, b, _rt_a3_member_path(before, key, path), rv120_output_hashes=rv120_output_hashes)
                   for (key, a), (_, b) in zip(before, after))
    if isinstance(before, list):
        assert len(before) == len(after), path
        return sum(compare(a, b, path + (i,), rv120_output_hashes=rv120_output_hashes) for i, (a, b) in enumerate(zip(before, after)))
    if type(before) is str and before == OLD and path[-3:] == ("formulation_basis", "limitations", 0):
        assert after == NEW, path
        return 1
    if before == after:
        return 0
    if type(before) is str:
        if HASH.fullmatch(before) and HASH.fullmatch(after):
            # RV120's documented input_sha256 binds a materialized RESULT
            # statement plus invocation, not the authored model. Only its
            # exact fixture field may move, after independent re-materialization.
            rv120_digest = (rv120_output_hashes and len(path) == 3 and path[0] == "shapes"
                            and isinstance(path[1], int) and path[2] == "input_sha256")
            assert rv120_digest or not INPUT_BINDINGS.intersection(path), path
            assert before.startswith("sha256:") == after.startswith("sha256:"), path
            return 0
        a, b = _embedded(before), _embedded(after)
        if a is not None and b is not None:
            return compare(a, b, path + ("<json>",), rv120_output_hashes=rv120_output_hashes)
    raise AssertionError(path)


@pytest.mark.parametrize("name", FILES)
def test_reducer_2a_moves_only_identity_and_binding_hashes(name):
    before = subprocess.check_output(["git", "show", f"{BASE}:{name}"], cwd=ROOT, text=True)
    after = (ROOT / name).read_text()
    rv120 = name == "fixtures/results/retained_precision_rv120_b3_inputs.json"
    if rv120:
        check_rv120_output_digests(json.loads(before), json.loads(after))
    compare(load(before), load(after), rv120_output_hashes=rv120)


def check_rv120_output_digests(before, after):
    """Recompute both result-statement digests, preserving every authored invocation.

    The Rust materializer and Python writer separately check their first gates.
    Here the independent Python rehash rule checks the actual digest subject.
    """
    from tests import test_retained_precision_b3 as rv

    def historical(name):
        return json.loads(subprocess.check_output(["git", "show", f"{BASE}:{name}"], cwd=ROOT, text=True))

    from tests.retained_precision_corpus import _STORAGE
    old_cases = {c["id"]: c for c in _STORAGE.expand(historical("fixtures/results/retained_precision_cases.json"))["cases"]}
    old_docs = {m: historical(f"fixtures/results/retained_precision_exact_successor_{m}.json")
                for m in ("sparse_interactive", "dense_scrutiny")}
    assert len(before["shapes"]) == len(after["shapes"])
    for old, new in zip(before["shapes"], after["shapes"]):
        assert (old["name"], old["base"], old["want"], old["invocation_edits"]) == (
            new["name"], new["base"], new["want"], new["invocation_edits"])
        kind, key = old["base"]
        base = old_cases[key] if kind == "corpus" else old_docs[key]
        source, invocation = deepcopy(base["source"]), deepcopy(base["invocation"])
        for value, edits in ((source, old["edits"]), (invocation, old["invocation_edits"])):
            for edit in edits:
                at = value
                for part in edit["path"][:-1]:
                    at = at[part]
                if edit["op"] == "remove":
                    del at[edit["path"][-1]]
                else:
                    assert edit["op"] == "set"
                    at[edit["path"][-1]] = deepcopy(edit["value"])
        source = rv.reseal(source, invocation, rv.rp.EXACT_DEFINITION_HASH)
        digest = lambda s, i: hashlib.sha256(json.dumps([s, i], sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        assert digest(source, invocation) == old["input_sha256"], old["name"]
        new_source, new_invocation = rv.rv120_input(new)
        assert digest(new_source, new_invocation) == new["input_sha256"], new["name"]
        assert invocation == new_invocation, new["name"]


def test_preservation_guard_rejects_other_moves():
    before = load('{"formulation_basis":{"limitations":[]},"results":[{"value":1.0}]}')
    for changed in (
        '{"formulation_basis":{"limitations":[]},"results":[{"value":1}]}',
        '{"formulation_basis":{"limitations":[]},"results":[{"value":2.0}]}',
        '{"formulation_basis":{"limitations":[]},"results":[]}',
        '{"formulation_basis":{"limitations":["unexpected"]},"results":[{"value":1.0}]}',
    ):
        with pytest.raises(AssertionError):
            compare(before, load(changed))
    # A syntactically valid identity at a different field is still forbidden.
    with pytest.raises(AssertionError):
        compare(Obj([("description", OLD)]), Obj([("description", NEW)]))


def test_preservation_guard_keeps_input_hashes_and_hash_encoding():
    old, new = "1" * 64, "2" * 64
    for field in INPUT_BINDINGS:
        with pytest.raises(AssertionError):
            compare(Obj([(field, old)]), Obj([(field, new)]))
    for scope in ("model", "request", "input_manifest"):
        with pytest.raises(AssertionError):
            compare(Obj([("payload_scope", scope), ("value", old)]),
                    Obj([("payload_scope", scope), ("value", new)]))
    # Rebinding a received output is expected; changing its digest encoding is not.
    assert compare(Obj([("payload_scope", "received_result"), ("value", old)]),
                   Obj([("payload_scope", "received_result"), ("value", new)])) == 0
    with pytest.raises(AssertionError):
        compare(old, "sha256:" + new, ("checksum", "value"))
    # Even the explicit result-digest context cannot exempt other fields.
    for path in (("input_sha256",), ("shapes", 0, "model_sha256"), ("other", 0, "input_sha256")):
        with pytest.raises(AssertionError):
            compare(old, new, path, rv120_output_hashes=True)


def check_j7_envelope_digests_only(before, after):
    """AC-002: a product-text change cannot move any other census fact."""
    changes = classify(before, after)
    assert changes
    assert all(kind == "hash" and path == "/entries/*/public/envelope_sha256"
               for kind, path in changes), changes[:8]
    return len(changes)


def test_reducer_2a_j7_moves_only_whole_envelope_digests():
    name = "fixtures/results/j7_rf_census_expected.json"
    before = subprocess.check_output(["git", "show", f"{BASE}:{name}"], cwd=ROOT, text=True)
    check_j7_envelope_digests_only(load(before), load((ROOT / name).read_text()))


def test_j7_guard_preserves_results_summary_identity_and_work():
    name = "fixtures/results/j7_rf_census_expected.json"
    baseline = load(subprocess.check_output(["git", "show", f"{BASE}:{name}"], cwd=ROOT, text=True))

    def set_member(obj, key, value):
        index = next(i for i, (k, _) in enumerate(obj) if k == key)
        obj[index] = (key, value)

    for field, replacement in (("results_sha256", "0" * 64),
                               ("summary_sha256", "0" * 64),
                               ("identity", "invented-wrong-identity")):
        changed = deepcopy(baseline)
        public = dict(dict(changed)["entries"][0])["public"]
        set_member(public, field, replacement)
        with pytest.raises(AssertionError):
            check_j7_envelope_digests_only(baseline, changed)
    changed = deepcopy(baseline)
    entry = next(e for e in dict(changed)["entries"] if "work" in dict(dict(e)["first_cause"]))
    work = dict(dict(entry)["first_cause"])["work"]
    set_member(work, "charged", Token(str(int(dict(work)["charged"]) + 1)))
    with pytest.raises(AssertionError):
        check_j7_envelope_digests_only(baseline, changed)
