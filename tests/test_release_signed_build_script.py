"""Focused tests for the identity-safe signed-build wrapper (nothing is built or signed)."""

from __future__ import annotations

import importlib.util
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "release" / "build_signed_app.py"


def load_module():
    spec = importlib.util.spec_from_file_location("build_signed_app", MODULE_PATH)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


FIND_IDENTITY = """  1) 0123456789ABCDEF0123456789ABCDEF01234567 "Apple Development: Example Person (ABCDE12345)"
  2) 89ABCDEF0123456789ABCDEF0123456789ABCDEF "Developer ID Application: Example Person (ABCDE12345)"
     2 valid identities found
"""


def test_redact_masks_identity_names_teams_and_hashes():
    build = load_module()
    for line in (
        'Signing with identity "Developer ID Application: Example Person (ABCDE12345)"',
        "error: Developer ID Application: Example Person (ABCDE12345): ambiguous (matches 89ABCDEF0123456789ABCDEF0123456789ABCDEF)",
        "TeamIdentifier=ABCDE12345",
        "Apple Distribution: Example Person (ABCDE12345)",
        'error: ambiguous (matches "Developer ID Application: Example "Research" Ltd (ABCDE12345)" and "Developer ID Application: Other Ltd (FGHIJ67890)")',
    ):
        masked = build.redact(line)
        assert "Example Person" not in masked and "ABCDE12345" not in masked
        assert "Research" not in masked and "Other Ltd" not in masked
        assert "89ABCDEF0123456789ABCDEF0123456789ABCDEF" not in masked
    assert build.redact('Signing with identity "Developer ID Application"') == 'Signing with identity "Developer ID Application"'


def test_identity_count_and_refusal(monkeypatch, capsys):
    build = load_module()
    assert build.count_identities(FIND_IDENTITY) == 1
    assert build.count_identities(FIND_IDENTITY + FIND_IDENTITY) == 2
    assert build.count_identities("     0 valid identities found\n") == 0
    for count in (0, 2, None):
        monkeypatch.setattr(build, "keychain_identity_count", lambda count=count: count)
        assert build.main(["--execute"]) == 2
    assert "Example Person" not in capsys.readouterr().err


def test_build_environment_names_the_kind_and_drops_tauri_notarization():
    build = load_module()
    env = build.build_environment({"PATH": "/usr/bin", "APPLE_ID": "x", "APPLE_PASSWORD": "y", "APPLE_API_KEY": "z"})
    assert env["APPLE_SIGNING_IDENTITY"] == "Developer ID Application"
    assert not {"APPLE_ID", "APPLE_PASSWORD", "APPLE_API_KEY"} & set(env)
    assert env["PATH"] == "/usr/bin"
