"""Reproduce new independent member-load reference data with its maintained writers."""
from pathlib import Path
import subprocess
import sys

import pytest

PROJECT = Path(__file__).resolve().parents[1]
VECTORS = PROJECT / "core/solver/frame_kernel/tests/retained_k4"


@pytest.mark.parametrize(
    "writer,args",
    [
        ("gen_k4_vectors.py", ["--only=declared", "--check"]),
        ("gen_member_recovery_vectors.py", ["--check"]),
        ("gen_member_maximum_vectors.py", ["--check"]),
    ],
)
def test_member_load_reference_writer_reproduces_committed_data(writer, args):
    subprocess.run([sys.executable, str(VECTORS / writer), *args], cwd=PROJECT, check=True)
