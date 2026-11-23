"""The four-span hydrostatic test model a person opens (`fixtures/hydrostatic_test/`).

Its generator reads only the frozen hydrostatic test references (checked against their
SHA256SUMS) and rebuilds the committed model in memory; `--check` compares it byte for
byte, so the model cannot drift from the references it is authored from.
"""
import subprocess
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
GENERATOR = PROJECT / "fixtures" / "hydrostatic_test" / "generate_four_span_test_model.py"


def test_the_committed_model_is_what_its_generator_writes():
    result = subprocess.run([sys.executable, "-I", str(GENERATOR), "--check"], cwd=PROJECT,
                            capture_output=True, text=True, timeout=120)
    assert result.returncode == 0, result.stdout + result.stderr
