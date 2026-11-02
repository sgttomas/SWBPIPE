"""Reproduce the independent hanger-sizing references (DEL-04-11 R-3) with their maintained generator."""
from pathlib import Path
import subprocess
import sys

PROJECT = Path(__file__).resolve().parents[1]
REFERENCES = PROJECT / "validation/references/hanger_sizing"


def test_hanger_sizing_reference_generator_reproduces_committed_data():
    subprocess.run([sys.executable, str(REFERENCES / "generate_r3.py"), "--check"], cwd=PROJECT, check=True)
