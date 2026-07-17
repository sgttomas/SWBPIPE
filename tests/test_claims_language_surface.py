#!/usr/bin/env python3
"""DEC-081 claims-language surface check (D-48 O-A).

Thin shim: runs the repo-root deterministic validator
`tools/validation/validate_claims_language.py` against the live tree and
requires a clean result. The validator itself is unit-tested in
`tools/validation/test_validate_claims_language.py`.
"""

import subprocess
import sys
from pathlib import Path


def _repo_root() -> Path:
    for parent in Path(__file__).resolve().parents:
        if (parent / "tools" / "validation" /
                "validate_claims_language.py").is_file():
            return parent
    raise RuntimeError(
        "repo root with tools/validation/validate_claims_language.py "
        "not found above " + str(Path(__file__).resolve())
    )


def test_live_tree_has_no_claims_language_findings():
    root = _repo_root()
    completed = subprocess.run(
        [
            sys.executable,
            str(root / "tools" / "validation" /
                "validate_claims_language.py"),
            "--repo-root",
            str(root),
        ],
        capture_output=True,
        text=True,
        check=False,
    )
    output = (completed.stdout + completed.stderr).strip()
    if len(output) > 4000:
        output = output[:4000] + " …[truncated]"
    assert completed.returncode == 0, (
        "DEC-081 claims-language validator reported findings:\n" + output
    )
    assert output.startswith("VALID ")


if __name__ == "__main__":
    test_live_tree_has_no_claims_language_findings()
