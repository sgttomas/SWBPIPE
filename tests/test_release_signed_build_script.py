"""Focused tests for the identity-safe signed-build wrapper (nothing is built or signed)."""

from __future__ import annotations

import importlib.util
import io
import json
from types import SimpleNamespace
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
        assert build.main(["--execute", "--supplier-binary", "/explicit/supplier"]) == 2
    assert "Example Person" not in capsys.readouterr().err


def test_build_environment_names_the_kind_and_drops_tauri_notarization():
    build = load_module()
    env = build.build_environment({"PATH": "/usr/bin", "APPLE_ID": "x", "APPLE_PASSWORD": "y", "APPLE_API_KEY": "z"})
    assert env["APPLE_SIGNING_IDENTITY"] == "Developer ID Application"
    assert not {"APPLE_ID", "APPLE_PASSWORD", "APPLE_API_KEY"} & set(env)
    assert env["PATH"] == "/usr/bin"


def test_dry_run_and_missing_source_have_no_keychain_network_or_build_effects(monkeypatch):
    build = load_module()
    def forbidden(*args, **kwargs):
        raise AssertionError("side effect")
    monkeypatch.setattr(build, "keychain_identity_count", forbidden)
    monkeypatch.setattr(build.subprocess, "Popen", forbidden)
    monkeypatch.setattr(build.supplier, "prepare", forbidden)
    assert build.main([]) == 0
    assert build.main(["--supplier-download"]) == 0
    assert build.main(["--execute"]) == 2


def test_explicit_target_and_native_target_directory():
    build = load_module()
    assert build.BUILD_COMMAND[-2:] == ("--target", "aarch64-apple-darwin")
    suffix = Path("aarch64-apple-darwin/release/bundle/macos/SWBPIPE.app/Contents/Resources")
    assert build.built_resources({}) == build.DESKTOP / "src-tauri/target" / suffix
    assert build.built_resources({"CARGO_TARGET_DIR": "custom"}) == build.DESKTOP / "src-tauri/custom" / suffix
    assert build.built_resources({"CARGO_TARGET_DIR": "/explicit/target"}) == Path("/explicit/target") / suffix


def mock_pipeline(build, monkeypatch, *, build_status=0, verify_failure=False, prepare_failure=False):
    calls = []
    stages = []
    monkeypatch.setattr(build, "keychain_identity_count", lambda: 1)
    def prepare(output, target, **kwargs):
        calls.append(("prepare", target, kwargs))
        if prepare_failure:
            raise OSError("private-source signed-url?secret")
        output.mkdir()
        overlay = output / "tauri-overlay.json"
        overlay.write_text(json.dumps({"bundle": {"resources": {}}}))
        stages.append(overlay)
        return overlay
    def popen(command, **kwargs):
        assert stages[0].exists()
        assert command[-2:] == ("--config", str(stages[0]))
        assert command[-4:-2] == ("--target", "aarch64-apple-darwin")
        assert kwargs["env"]["APPLE_SIGNING_IDENTITY"] == build.IDENTITY_KIND
        assert not set(build.TAURI_NOTARIZATION_VARIABLES) & set(kwargs["env"])
        calls.append(("build", command))
        def wait():
            assert stages[0].exists()
            return build_status
        return SimpleNamespace(stdout=io.StringIO(
            f"{stages[0]} /explicit/private-source Developer ID Application: Example Person (ABCDE12345)\n"), wait=wait)
    def verify(resources, target):
        assert stages[0].exists()
        calls.append(("verify", resources, target))
        if verify_failure:
            raise OSError("private-source signed-url?secret")
    monkeypatch.setattr(build.supplier, "prepare", prepare)
    monkeypatch.setattr(build.supplier, "verify", verify)
    monkeypatch.setattr(build.subprocess, "Popen", popen)
    return calls, stages


def test_build_lifetime_verification_and_redaction(monkeypatch, capsys):
    build = load_module()
    monkeypatch.setenv("APPLE_PASSWORD", "private")
    calls, stages = mock_pipeline(build, monkeypatch)
    assert build.main(["--execute", "--supplier-binary", "/explicit/private-source"]) == 0
    assert [call[0] for call in calls] == ["prepare", "build", "verify"]
    assert calls[0][2] == {"archive": None, "binary": Path("/explicit/private-source"), "download": False}
    assert calls[-1][1] == build.built_resources(build.build_environment(dict(build.os.environ)))
    assert all(not path.exists() for path in stages)
    output = capsys.readouterr()
    for private in ("Example Person", "ABCDE12345", "/explicit/private-source", str(stages[0])):
        assert private not in output.out + output.err
    assert "bundled supplier and notice bytes verified" in output.out


def test_failed_build_never_verifies_or_claims_success(monkeypatch, capsys):
    build = load_module()
    calls, stages = mock_pipeline(build, monkeypatch, build_status=7)
    assert build.main(["--execute", "--supplier-download"]) == 7
    assert [call[0] for call in calls] == ["prepare", "build"]
    assert all(not path.exists() for path in stages)
    assert "build completed" not in capsys.readouterr().out


def test_failed_prepare_or_verify_never_claims_success(monkeypatch, capsys):
    for prepare_failure in (True, False):
        build = load_module()
        calls, stages = mock_pipeline(build, monkeypatch, prepare_failure=prepare_failure, verify_failure=True)
        assert build.main(["--execute", "--supplier-archive", "/explicit/archive"]) == 2
        assert [call[0] for call in calls] == (["prepare"] if prepare_failure else ["prepare", "build", "verify"])
        assert all(not path.exists() for path in stages)
        output = capsys.readouterr()
        assert "build completed" not in output.out
        assert "signed-url" not in output.err


def test_identity_failure_never_prepares_or_builds(monkeypatch):
    build = load_module()
    monkeypatch.setattr(build, "keychain_identity_count", lambda: None)
    def forbidden(*args, **kwargs):
        raise AssertionError("build effect")
    monkeypatch.setattr(build.supplier, "prepare", forbidden)
    monkeypatch.setattr(build.subprocess, "Popen", forbidden)
    assert build.main(["--execute", "--supplier-download"]) == 2


def test_output_failure_waits_before_private_stage_cleanup(monkeypatch):
    build = load_module()
    calls, stages = mock_pipeline(build, monkeypatch)
    class FailedOutput:
        def __iter__(self):
            raise OSError("private output failure")
    def popen(*args, **kwargs):
        def wait():
            assert stages[0].exists()
            calls.append(("wait",))
            return 0
        return SimpleNamespace(stdout=FailedOutput(), wait=wait)
    monkeypatch.setattr(build.subprocess, "Popen", popen)
    assert build.main(["--execute", "--supplier-download"]) == 2
    assert [call[0] for call in calls] == ["prepare", "wait"]
    assert not stages[0].exists()
