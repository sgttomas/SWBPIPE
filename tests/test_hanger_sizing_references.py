"""Reproduce the independent hanger-sizing references (DEL-04-11 R-1 to R-5) with their maintained generators."""
from pathlib import Path
import subprocess
import sys

import pytest

PROJECT = Path(__file__).resolve().parents[1]
REFERENCES = PROJECT / "validation/references/hanger_sizing"


@pytest.mark.parametrize("generator", ["generate_r3.py", "generate_r125.py"])
def test_hanger_sizing_reference_generator_reproduces_committed_data(generator):
    subprocess.run([sys.executable, str(REFERENCES / generator), "--check"], cwd=PROJECT, check=True)
