"""Reducer 2a regenerates current producer fixtures, never numerical references.

The preserved increment-1 endpoint is integrated main 2656dcb33. Across this
stage, only formulation_basis.limitations[0] and binding hashes may change.
Exact numeric tokens, member/array order, row sets, diagnostics and selection
remain invariant. The old O4/W15/tee proofs keep their historical endpoints.
"""
from pathlib import Path
import subprocess

import pytest

from tests.test_o4_text_only_envelope_diff import (
    HASH, Obj, RT_A3_IDENTITY_NEW, RT_A3_LISTED, load, _embedded,
    _rt_a3_member_path,
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


def compare(before, after, path=()):
    """Return identity-field count; refuse every non-binding or numerical move."""
    assert type(before) is type(after), path
    if isinstance(before, Obj):
        assert [k for k, _ in before] == [k for k, _ in after], path
        return sum(compare(a, b, _rt_a3_member_path(before, key, path))
                   for (key, a), (_, b) in zip(before, after))
    if isinstance(before, list):
        assert len(before) == len(after), path
        return sum(compare(a, b, path + (i,)) for i, (a, b) in enumerate(zip(before, after)))
    if type(before) is str and before == OLD and path[-3:] == ("formulation_basis", "limitations", 0):
        assert after == NEW, path
        return 1
    if before == after:
        return 0
    if type(before) is str:
        if HASH.fullmatch(before) and HASH.fullmatch(after):
            return 0
        a, b = _embedded(before), _embedded(after)
        if a is not None and b is not None:
            return compare(a, b, path + ("<json>",))
    raise AssertionError(path)


@pytest.mark.parametrize("name", FILES)
def test_reducer_2a_moves_only_identity_and_binding_hashes(name):
    before = subprocess.check_output(["git", "show", f"{BASE}:{name}"], cwd=ROOT, text=True)
    compare(load(before), load((ROOT / name).read_text()))


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
