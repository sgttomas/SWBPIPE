#!/usr/bin/env python3
"""DEC-081 claims-language surface check (D-48 O-A).

Thin shim: imports the repo-root deterministic validator
`tools/validation/validate_claims_language.py` and runs it in-process
against the live tree, requiring a clean result. This is the single
repo-wide claims-language enforcement run for the suite; the validator
itself is unit-tested in
`tools/validation/test_validate_claims_language.py`.
"""

import importlib.util
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


def _load_validator(root: Path):
    module_path = (
        root / "tools" / "validation" / "validate_claims_language.py"
    )
    spec = importlib.util.spec_from_file_location(
        "validate_claims_language", module_path
    )
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def test_live_tree_has_no_claims_language_findings():
    root = _repo_root()
    validator = _load_validator(root)

    scanned = validator.iter_scanned_files(root)
    assert scanned, "validator scanned no files; surface wiring is broken"

    findings = validator.validate_claims_language(root)
    formatted = "\n".join(
        f"{finding.code} {finding.path}"
        f"{':' + str(finding.line) if finding.line is not None else ''}: "
        f"{finding.message}"
        for finding in findings[:40]
    )
    assert not findings, (
        "DEC-081 claims-language validator reported findings:\n" + formatted
    )


if __name__ == "__main__":
    test_live_tree_has_no_claims_language_findings()
