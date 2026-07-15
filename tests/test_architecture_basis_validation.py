from __future__ import annotations

import importlib.util
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "validation" / "validate_architecture_basis.py"


def load_module():
    spec = importlib.util.spec_from_file_location(
        "validate_architecture_basis", MODULE_PATH
    )
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def test_live_architecture_basis_contract_passes():
    completed = subprocess.run(
        [sys.executable, str(MODULE_PATH)],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )

    assert completed.returncode == 0, completed.stdout + completed.stderr
    assert "PASS (8 members)" in completed.stdout


def test_missing_project_fails_closed(tmp_path):
    validator = load_module()

    errors = validator.validate(tmp_path)

    assert errors
    assert any("member census mismatch" in error for error in errors)
