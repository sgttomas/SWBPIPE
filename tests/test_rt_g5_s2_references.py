"""RT-G5 S2 references (`validation/references/rt_g5_s2_conditions/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory and compares
them byte for byte with the committed files. While rebuilding, every E3 row is
computed twice, by exact `Fraction(repr(v))` arithmetic and by a transcription of
DEL-02-01 Work `model-document-0.9.0.md` section 1.5.1's fixed-integer algorithm,
which must agree bit for bit and keep the stated intermediate bounds.
"""
import json
import subprocess
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
REFERENCE = PROJECT / "validation" / "references" / "rt_g5_s2_conditions"
GENERATOR = REFERENCE / "generators" / "rt_g5_s2_reference.py"


def test_generator_check_rebuilds_the_committed_references():
    result = subprocess.run([sys.executable, "-I", str(GENERATOR)], cwd=GENERATOR.parent,
                            capture_output=True, text=True, timeout=600)
    assert result.returncode == 0, result.stdout + result.stderr


def test_the_design_examples_keep_their_bits():
    rows = {row["id"]: row for row in json.loads((REFERENCE / "references.json").read_text())["temperature_differences"]}
    assert rows["width_regression"]["expected"]["bits"] == "3af357c299a88ea7"
    assert rows["decimal_tie_regression"]["expected"]["bits"] == "3f4aafbe76c8b435"
    assert rows["decimal_tie_regression"]["formatter_route_bits"] == "3f4aafbe76c8b43e"
    assert rows["fahrenheit_77_70"]["expected"]["bits"] == "400f1c71c71c71c7"
    assert rows["fahrenheit_77_70"]["typed_interval"]["bits"] == "400f1c71c71c71c8"
    assert rows["celsius_typed_decimals"]["expected"]["value"] == 0.2
    assert rows["fahrenheit_212_70"]["expected"]["value"] == 78.88888888888889
    assert rows["equal_mixed_units"]["expected"]["bits"] == "0000000000000000"
