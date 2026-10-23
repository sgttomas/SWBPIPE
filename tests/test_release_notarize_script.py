"""Focused tests for the D-06b notarize-and-staple helper (no Apple service is contacted)."""

from __future__ import annotations

import importlib.util
import sys
from pathlib import Path
from types import SimpleNamespace


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "release" / "notarize_app.py"


def load_module():
    spec = importlib.util.spec_from_file_location("notarize_app", MODULE_PATH)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def test_submission_outcome_reads_notarytool_json():
    notarize = load_module()
    good = "2efe2717-52ef-43a5-96dc-0797e4ca1041"
    assert notarize.submission_outcome(f'{{"id": "{good}", "status": "Accepted", "message": "ok"}}') == (good, "Accepted")
    assert notarize.submission_outcome("not json") == (None, None)
    assert notarize.submission_outcome("[1, 2]") == (None, None)
    # Anything but a UUID id or a known status is discarded, not echoed.
    assert notarize.submission_outcome('{"id": "Example Person ABCDE12345", "status": "Example Person"}') == (None, "unrecognized")


def test_execute_requires_a_bundle_and_a_profile_name(tmp_path, monkeypatch):
    notarize = load_module()
    monkeypatch.delenv(notarize.PROFILE_ENV, raising=False)
    assert notarize.main(["--execute", "--app-path", str(tmp_path / "Nope.app"), "--keychain-profile", "p"]) == 2
    app = tmp_path / "SWBPIPE.app"
    app.mkdir()
    assert notarize.main(["--execute", "--app-path", str(app)]) == 2
    # Dry run touches nothing.
    assert notarize.main(["--app-path", str(app)]) == 0


def test_staples_only_an_accepted_submission(tmp_path, monkeypatch, capsys):
    notarize = load_module()
    app = tmp_path / "SWBPIPE.app"
    app.mkdir()
    calls: list[list[str]] = []

    def fake(status):
        def run(argv):
            calls.append(argv)
            if argv[:3] == ["xcrun", "notarytool", "submit"]:
                return SimpleNamespace(returncode=0 if status == "Accepted" else 1, stdout=f'{{"id": "2efe2717-52ef-43a5-96dc-0797e4ca1041", "status": "{status}"}}', stderr="Error: account Example Person (ABCDE12345) secret-ish detail")
            return SimpleNamespace(returncode=0, stdout="", stderr="")
        return run

    monkeypatch.setattr(notarize, "run", fake("Invalid"))
    assert notarize.main(["--execute", "--app-path", str(app), "--keychain-profile", "swbpipe-notary"]) == 1
    assert not any(argv[:3] == ["xcrun", "stapler", "staple"] for argv in calls)
    # Tool stderr can name the account or team: it is never echoed.
    printed = capsys.readouterr()
    assert "Example Person" not in printed.out + printed.err
    assert "ABCDE12345" not in printed.out + printed.err

    calls.clear()
    monkeypatch.setattr(notarize, "run", fake("Accepted"))
    assert notarize.main(["--execute", "--app-path", str(app), "--keychain-profile", "swbpipe-notary"]) == 0
    submit = next(argv for argv in calls if argv[:3] == ["xcrun", "notarytool", "submit"])
    # Credentials come from the keychain profile only.
    assert submit[submit.index("--keychain-profile") + 1] == "swbpipe-notary"
    assert not {"--password", "--apple-id", "--key", "--team-id"} & set(submit)
    assert ["xcrun", "stapler", "staple", str(app.resolve())] in calls


def test_missing_tools_are_an_exit_code_not_a_traceback(tmp_path, monkeypatch):
    notarize = load_module()
    app = tmp_path / "SWBPIPE.app"
    app.mkdir()
    monkeypatch.setenv("PATH", str(tmp_path / "empty"))
    assert notarize.main(["--execute", "--app-path", str(app), "--keychain-profile", "p"]) == 1
