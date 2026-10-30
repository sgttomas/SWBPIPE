"""W16T reference: every serialized interval encloses its exact value.

The generator rebuilds `references.json` in memory and compares it byte for
byte with the committed file. While rebuilding it asserts, in exact rational
arithmetic, that each serialized interval's lower endpoint (rounded toward
-inf) and upper endpoint (rounded toward +inf) enclose the internal rational
interval, which itself encloses the exact value. A round-to-nearest
serialization, which collapsed the six irrational intervals to points that
excluded their values, fails this test.
"""
import json
import subprocess
import sys
from decimal import Decimal
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
REFERENCE = PROJECT / "validation" / "references" / "w16_timoshenko_partial_wind"
GENERATOR = REFERENCE / "generators" / "w16t_reference.py"


def test_generator_check_rebuilds_the_committed_reference_with_enclosing_intervals():
    result = subprocess.run(
        [sys.executable, "-I", str(GENERATOR)],
        cwd=GENERATOR.parent,
        capture_output=True,
        text=True,
        timeout=600,
    )
    assert result.returncode == 0, result.stdout + result.stderr


def _records(node):
    if isinstance(node, dict):
        if "interval" in node:
            yield node
        for value in node.values():
            yield from _records(value)
    elif isinstance(node, list):
        for value in node:
            yield from _records(value)


def test_every_serialized_interval_is_a_proper_interval():
    references = json.loads((REFERENCE / "references.json").read_text())
    intervals = [record["interval"] for record in _records(references)]
    assert len(intervals) == 6
    for lower, upper in intervals:
        assert Decimal(lower) < Decimal(upper), (lower, upper)
