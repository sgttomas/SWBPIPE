"""Focused tests for the DEC-057 release-artifact packaging entrypoint."""

from __future__ import annotations

import importlib.util
import json
import os
import plistlib
import stat
import struct
import sys
import zipfile
from pathlib import Path
from types import SimpleNamespace


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "release" / "package_release_artifact.py"


def load_module():
    spec = importlib.util.spec_from_file_location(
        "package_release_artifact", MODULE_PATH
    )
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def make_fake_app(base: Path, name: str = "Fake App.app") -> Path:
    """A minimal .app tree: arm64 Mach-O stub, resources, and a symlink."""
    app = base / name
    macos_dir = app / "Contents" / "MacOS"
    resources = app / "Contents" / "Resources"
    macos_dir.mkdir(parents=True)
    resources.mkdir(parents=True)
    with (app / "Contents" / "Info.plist").open("wb") as handle:
        plistlib.dump({"CFBundleExecutable": app.stem}, handle)
    executable = macos_dir / app.stem
    executable.write_bytes(b"\xcf\xfa\xed\xfe" + struct.pack("<I", 0x0100000C) + b"\0" * 8)
    executable.chmod(0o755)
    (resources / "data.bin").write_bytes(b"payload" * 64)
    os.symlink("data.bin", resources / "alias.bin")
    return app


def clean_git_state(commit: str = "a" * 40) -> dict:
    return {
        "commit_hash": commit,
        "branch": "main",
        "status_capture_failed": False,
        "working_tree_dirty": False,
        "dirty_paths": [],
    }


def sweep_stub(tmp_path: Path, commit: str, status: str = "pass", dirty: bool = False) -> Path:
    path = tmp_path / f"SWEEP_stub_{commit[:8]}_{status}_{dirty}.json"
    surface_ids = [
        "cargo_crate_sweep",
        "python_pytest",
        "desktop_vitest",
        "desktop_playwright_e2e",
        "desktop_production_build",
    ]
    surfaces = []
    failed = status == "fail"
    for order, surface_id in enumerate(surface_ids, start=1):
        surface_status = "fail" if failed and order == 1 else (
            "not_run" if failed else "pass"
        )
        surfaces.append(
            {
                "order": order,
                "surface_id": surface_id,
                "description": f"fixture {surface_id}",
                "execution_capability": (
                    "host" if surface_id == "desktop_playwright_e2e" else "sandboxed"
                ),
                "commands": (
                    []
                    if surface_status == "not_run"
                    else [
                        {
                            "argv": ["fixture"],
                            "exit_code": 1 if surface_status == "fail" else 0,
                            "duration_seconds": 0.0,
                        }
                    ]
                ),
                "status": surface_status,
            }
        )
    path.write_text(
        json.dumps(
            {
                "artifact": "openpipestress.evidence_sweep_summary",
                "schema_version": 2,
                "decision_basis": "DEC-025",
                "git": {
                    "commit_hash": commit,
                    "branch": "fixture",
                    "status_capture_failed": False,
                    "working_tree_dirty": dirty,
                    "dirty_paths": ["fixture"] if dirty else [],
                },
                "started_utc": "2026-08-19T12:00:00+00:00",
                "surfaces": surfaces,
                "overall_status": status,
            }
        ),
        encoding="utf-8",
    )
    return path


def test_dry_run_prints_plan_and_exits_zero(capsys):
    packaging = load_module()
    assert packaging.main([]) == 0
    out = capsys.readouterr().out
    assert "dry-run" in out
    assert "aarch64-apple-darwin" in out
    assert "DEC-057" in out


def test_ruled_shape_constants():
    packaging = load_module()
    assert packaging.RELEASE_TARGET_TRIPLE == "aarch64-apple-darwin"
    assert packaging.INSTALLER_FORMAT == "tauri_app_bundle_zip"
    assert "not code-signed or notarized" in packaging.UNSIGNED_INSTALL_CAVEAT
    assert "DEC-057" in packaging.UNSIGNED_INSTALL_CAVEAT
    assert "D-06b" in packaging.UNSIGNED_INSTALL_CAVEAT
    assert "SHA-256" in packaging.UNSIGNED_INSTALL_CAVEAT
    assert "Not a release act" in packaging.BOUNDARY_NOTE
    assert "Developer ID" in packaging.SIGNED_INSTALL_NOTE
    assert "notarized" in packaging.SIGNED_INSTALL_NOTE
    assert "SHA-256" in packaging.SIGNED_INSTALL_NOTE
    assert "Proceed with signing and notarizing." in packaging.SIGNING_DECISION


def test_bundle_config_matches_ruled_shape():
    """tauri.conf.json carries the DEC-057 E5 selections."""
    conf = json.loads(
        (ROOT / "apps" / "desktop" / "src-tauri" / "tauri.conf.json").read_text(
            encoding="utf-8"
        )
    )
    assert conf["bundle"]["active"] is True
    assert conf["bundle"]["targets"] == ["app"]
    assert "icons/icon.icns" in conf["bundle"]["icon"]
    for icon in conf["bundle"]["icon"]:
        assert (ROOT / "apps" / "desktop" / "src-tauri" / icon).is_file()
    # D-06b (2026-10-23): signed with the hardened runtime. The identity comes
    # from the build environment (APPLE_SIGNING_IDENTITY), never this file.
    macos = conf["bundle"]["macOS"]
    assert macos["hardenedRuntime"] is True
    assert not {"signingIdentity", "providerShortName"} & set(macos)


def test_icns_asset_is_multi_resolution_png_member_icns():
    icns = (ROOT / "apps" / "desktop" / "src-tauri" / "icons" / "icon.icns").read_bytes()
    assert icns[:4] == b"icns"
    assert struct.unpack(">I", icns[4:8])[0] == len(icns)
    kinds = []
    offset = 8
    while offset < len(icns):
        kind = icns[offset : offset + 4]
        length = struct.unpack(">I", icns[offset + 4 : offset + 8])[0]
        assert icns[offset + 8 : offset + 16] == b"\x89PNG\r\n\x1a\n"
        kinds.append(kind)
        offset += length
    # 1024px (ic10) plus standard/@2x members down to 16px.
    for expected in (b"ic10", b"ic09", b"ic07", b"icp4"):
        assert expected in kinds


def test_deterministic_zip_is_byte_identical_and_preserves_modes(tmp_path):
    packaging = load_module()
    app = make_fake_app(tmp_path)
    date_time = (2026, 7, 10, 0, 0, 0)
    zip_a = tmp_path / "a.zip"
    zip_b = tmp_path / "b.zip"
    packaging.build_deterministic_zip(app, zip_a, date_time)
    packaging.build_deterministic_zip(app, zip_b, date_time)
    assert zip_a.read_bytes() == zip_b.read_bytes()

    with zipfile.ZipFile(zip_a) as archive:
        names = archive.namelist()
        assert names == sorted(names)
        assert names[0] == "Fake App.app/"
        infos = {info.filename: info for info in archive.infolist()}
        exe = infos["Fake App.app/Contents/MacOS/Fake App"]
        assert exe.create_system == 3
        assert exe.date_time == date_time
        assert stat.S_IMODE(exe.external_attr >> 16) == 0o755
        link = infos["Fake App.app/Contents/Resources/alias.bin"]
        assert stat.S_ISLNK(link.external_attr >> 16)
        assert archive.read(link.filename) == b"data.bin"


def test_zip_date_time_clamps_to_zip_epoch():
    packaging = load_module()
    assert packaging.zip_date_time(None) == (1980, 1, 1, 0, 0, 0)
    assert packaging.zip_date_time(0) == (1980, 1, 1, 0, 0, 0)
    assert packaging.zip_date_time(1783987200)[0] == 2026


def test_checksum_line_is_shasum_compatible():
    packaging = load_module()
    line = packaging.checksum_line("ab" * 32, "artifact.app.zip")
    assert line == ("ab" * 32) + "  artifact.app.zip\n"


def test_app_main_executable_honors_info_plist(tmp_path):
    """The Tauri bundle names its binary via CFBundleExecutable, not the
    product name (observed: openpipestress-desktop inside
    'SWBPIPE.app')."""
    packaging = load_module()
    app = tmp_path / "Pretty Name.app"
    (app / "Contents" / "MacOS").mkdir(parents=True)
    with (app / "Contents" / "Info.plist").open("wb") as handle:
        plistlib.dump({"CFBundleExecutable": "actual-binary"}, handle)
    assert packaging.app_main_executable(app) == (
        app / "Contents" / "MacOS" / "actual-binary"
    )
    # Missing/unreadable plist falls back to the bundle stem.
    bare = tmp_path / "Bare.app"
    (bare / "Contents" / "MacOS").mkdir(parents=True)
    assert packaging.app_main_executable(bare) == (
        bare / "Contents" / "MacOS" / "Bare"
    )


def test_macho_arm64_check(tmp_path):
    packaging = load_module()
    good = tmp_path / "good"
    good.write_bytes(b"\xcf\xfa\xed\xfe" + struct.pack("<I", 0x0100000C))
    bad_arch = tmp_path / "bad_arch"
    bad_arch.write_bytes(b"\xcf\xfa\xed\xfe" + struct.pack("<I", 0x01000007))
    fat = tmp_path / "fat"
    fat.write_bytes(b"\xca\xfe\xba\xbe" + b"\0" * 4)
    assert packaging.macho_is_arm64(good)
    assert not packaging.macho_is_arm64(bad_arch)
    assert not packaging.macho_is_arm64(fat)
    assert not packaging.macho_is_arm64(tmp_path / "missing")


def test_authenticity_chain_states(tmp_path):
    packaging = load_module()
    commit = "b" * 40
    clean = clean_git_state(commit)

    matching = sweep_stub(tmp_path, commit)
    chain = packaging.evaluate_chain(commit, clean, matching, ROOT)
    assert chain["status"] == packaging.CHAIN_VERIFIED
    assert chain["sweep_commit"] == commit

    assert (
        packaging.evaluate_chain(commit, clean, None, ROOT)["status"]
        == packaging.CHAIN_NO_SWEEP
    )
    mismatched = sweep_stub(tmp_path, "c" * 40)
    assert (
        packaging.evaluate_chain(commit, clean, mismatched, ROOT)["status"]
        == packaging.CHAIN_SWEEP_COMMIT_MISMATCH
    )
    failing = sweep_stub(tmp_path, commit, status="fail")
    assert (
        packaging.evaluate_chain(commit, clean, failing, ROOT)["status"]
        == packaging.CHAIN_SWEEP_NOT_PASS
    )
    dirty_sweep = sweep_stub(tmp_path, commit, dirty=True)
    assert (
        packaging.evaluate_chain(commit, clean, dirty_sweep, ROOT)["status"]
        == packaging.CHAIN_SWEEP_DIRTY
    )
    unverified_sweep = sweep_stub(tmp_path, commit)
    unverified_summary = json.loads(unverified_sweep.read_text(encoding="utf-8"))
    unverified_summary["git"].update(
        status_capture_failed=True,
        working_tree_dirty=None,
        dirty_paths=[],
    )
    unverified_sweep.write_text(json.dumps(unverified_summary), encoding="utf-8")
    assert (
        packaging.evaluate_chain(commit, clean, unverified_sweep, ROOT)["status"]
        == packaging.CHAIN_SWEEP_GIT_UNVERIFIED
    )
    dirty_tree = dict(clean, working_tree_dirty=True, dirty_paths=["x"])
    assert (
        packaging.evaluate_chain(commit, dirty_tree, matching, ROOT)["status"]
        == packaging.CHAIN_DIRTY
    )
    unverified = dict(clean, commit_hash=None)
    assert (
        packaging.evaluate_chain(None, unverified, matching, ROOT)["status"]
        == packaging.CHAIN_GIT_UNVERIFIED
    )
    unreadable = tmp_path / "not_json.json"
    unreadable.write_text("{", encoding="utf-8")
    assert (
        packaging.evaluate_chain(commit, clean, unreadable, ROOT)["status"]
        == packaging.CHAIN_SWEEP_UNREADABLE
    )


def test_chain_rejects_mismatched_ci_binding_and_partial_summary(tmp_path):
    packaging = load_module()
    commit = "b" * 40
    clean = clean_git_state(commit)

    ci_path = sweep_stub(tmp_path, commit)
    ci_summary = json.loads(ci_path.read_text(encoding="utf-8"))
    ci_summary["schema_version"] = 3
    ci_summary["decision_basis"] = ["DEC-025", "DEC-093"]
    surface4 = ci_summary["surfaces"][3]
    surface4["execution_capability"] = "ci"
    surface4["commands"] = []
    surface4["ci_binding"] = {
        "workflow_path": ".github/workflows/piping-desktop-e2e.yml",
        "run_id": 1234,
        "run_attempt": 1,
        "head_sha": "c" * 40,
        "conclusion": "success",
        "registered_e2e_specs_executed": True,
        "viewport_projects": ["chromium-desktop", "chromium-compact"],
    }
    ci_path.write_text(json.dumps(ci_summary), encoding="utf-8")

    assert (
        packaging.evaluate_chain(commit, clean, ci_path, ROOT)["status"]
        == packaging.CHAIN_SWEEP_INVALID
    )

    partial_path = sweep_stub(tmp_path, commit)
    partial_summary = json.loads(partial_path.read_text(encoding="utf-8"))
    partial_summary["surfaces"] = [
        surface
        for surface in partial_summary["surfaces"]
        if surface["execution_capability"] == "sandboxed"
    ]
    for order, surface in enumerate(partial_summary["surfaces"], start=1):
        surface["order"] = order
    partial_summary["surface_selection"] = {
        "only_capability": "sandboxed",
        "complete_dec025_sweep": False,
    }
    partial_path.write_text(json.dumps(partial_summary), encoding="utf-8")

    assert (
        packaging.evaluate_chain(commit, clean, partial_path, ROOT)["status"]
        == packaging.CHAIN_SWEEP_INVALID
    )

    contradictory_path = sweep_stub(tmp_path, commit)
    contradictory = json.loads(contradictory_path.read_text(encoding="utf-8"))
    contradictory["git"]["dirty_paths"] = ["modified-file"]
    contradictory_path.write_text(json.dumps(contradictory), encoding="utf-8")

    assert (
        packaging.evaluate_chain(commit, clean, contradictory_path, ROOT)["status"]
        == packaging.CHAIN_SWEEP_INVALID
    )


def test_chain_accepts_evidence_only_closeout_commits(tmp_path):
    """DEC-025 pattern: sweep summaries land as evidence-only commits, so
    the record HEAD may trail the sweep commit by evidence files only."""
    import subprocess

    packaging = load_module()
    repo = tmp_path / "repo"
    repo.mkdir()

    def git(*args: str) -> str:
        completed = subprocess.run(
            ("git", *args),
            cwd=repo,
            capture_output=True,
            text=True,
            check=True,
            env={
                **os.environ,
                "GIT_AUTHOR_NAME": "t",
                "GIT_AUTHOR_EMAIL": "t@t",
                "GIT_COMMITTER_NAME": "t",
                "GIT_COMMITTER_EMAIL": "t@t",
            },
        )
        return completed.stdout.strip()

    git("init", "-q")
    (repo / "code.txt").write_text("code\n", encoding="utf-8")
    git("add", "code.txt")
    git("commit", "-qm", "code")
    sweep_commit = git("rev-parse", "HEAD")
    evidence = repo / "validation" / "evidence" / "sweeps"
    evidence.mkdir(parents=True)
    (evidence / "SWEEP_x.json").write_text("{}\n", encoding="utf-8")
    git("add", "validation")
    git("commit", "-qm", "evidence closeout")
    record_commit = git("rev-parse", "HEAD")

    sweep = sweep_stub(tmp_path, sweep_commit)
    chain = packaging.evaluate_chain(
        record_commit, clean_git_state(record_commit), sweep, repo
    )
    assert chain["status"] == packaging.CHAIN_VERIFIED
    assert chain["evidence_only_delta_paths"] == [
        "validation/evidence/sweeps/SWEEP_x.json"
    ]

    # A code-path delta between the commits is a real mismatch.
    (repo / "code.txt").write_text("changed\n", encoding="utf-8")
    git("add", "code.txt")
    git("commit", "-qm", "code change")
    later_commit = git("rev-parse", "HEAD")
    chain = packaging.evaluate_chain(
        later_commit, clean_git_state(later_commit), sweep, repo
    )
    assert chain["status"] == packaging.CHAIN_SWEEP_COMMIT_MISMATCH


def test_record_carries_section8_fields_and_caveat():
    packaging = load_module()
    commit = "d" * 40
    chain = {
        "sweep_artifact": "validation/evidence/sweeps/SWEEP_x.json",
        "sweep_commit": commit,
        "sweep_overall_status": "pass",
        "record_commit": commit,
        "status": packaging.CHAIN_VERIFIED,
        "checksum_sha256": "ef" * 32,
    }
    record = packaging.build_record(
        identity={
            "product_name": "SWBPIPE",
            "version": "0.1.0",
            "identifier": "com.chirality.swbpipe",
        },
        git_state=clean_git_state(commit),
        runtime={"platform": "test"},
        zip_relpath="dist/release/x.app.zip",
        checksum_relpath="dist/release/x.app.zip.sha256",
        digest="ef" * 32,
        zip_size=123,
        chain=chain,
        started_utc="2026-07-10T00:00:00+00:00",
    )
    # docs/BUILD_AND_RELEASE.md §8: source revision, evidence profile,
    # checks, artifact paths, checksums, validation status, limitations,
    # boundary notices, human review record.
    assert record["git"]["commit_hash"] == commit
    assert record["evidence_profile"].startswith("DEC-025")
    assert record["artifacts"][0]["sha256"] == "ef" * 32
    assert record["artifacts"][0]["checksum_file"].endswith(".sha256")
    assert record["validation_status"] == "pass"
    assert record["known_limitations"]
    assert record["boundary_note"] == packaging.BOUNDARY_NOTE
    assert record["human_review"]["state"] == "none_recorded"
    # DEC-057 ruled shape + unsigned caveat + authenticity chain.
    assert record["decision_basis"] == "DEC-057"
    assert record["release_matrix"]["target_triple"] == "aarch64-apple-darwin"
    assert record["installer_format"] == "tauri_app_bundle_zip"
    assert record["signing"]["state"] == "unsigned"
    assert record["signing"]["re_decision"] == "D-06b"
    assert record["unsigned_install_caveat"] == packaging.UNSIGNED_INSTALL_CAVEAT
    assert record["authenticity_chain"]["status"] == packaging.CHAIN_VERIFIED


def test_record_filename_binds_commit_and_dirty_state():
    packaging = load_module()
    record = {
        "git": clean_git_state("e" * 40),
        "recorded_utc": "2026-07-10T01:02:03+00:00",
    }
    assert (
        packaging.record_filename(record)
        == f"RELEASE_ARTIFACT_20260710T010203Z_{'e' * 12}.json"
    )
    record["git"] = dict(record["git"], working_tree_dirty=True)
    assert packaging.record_filename(record).endswith("-dirty.json")


def test_execute_end_to_end_with_fake_app(tmp_path, monkeypatch, capsys):
    packaging = load_module()
    commit = "f" * 40
    app = make_fake_app(tmp_path)
    sweep = sweep_stub(tmp_path, commit)
    stub = SimpleNamespace(
        collect_git_state=lambda root: clean_git_state(commit),
        collect_runtime_versions=lambda root: {"platform": "test"},
        git_state_unverified=lambda git: not git.get("commit_hash"),
        is_complete_sweep_summary=lambda summary: True,
    )
    monkeypatch.setattr(packaging, "_sweep_module", lambda: stub)
    monkeypatch.setattr(packaging, "commit_timestamp", lambda root: 1783987200)
    monkeypatch.setattr(packaging, "inspect_signing", lambda app: unsigned_inspection(packaging))
    monkeypatch.setattr(
        packaging,
        "inspect_zip_roundtrip",
        lambda zip_path, name: {**unsigned_inspection(packaging), "extracted": True},
    )

    out_dir = tmp_path / "out"
    records_dir = tmp_path / "records"
    argv = [
        "--execute",
        "--app-path",
        str(app),
        "--sweep-artifact",
        str(sweep),
        "--output-dir",
        str(out_dir),
        "--records-dir",
        str(records_dir),
    ]
    # D-06b (2026-10-23): an unsigned bundle is refused before anything is written.
    assert packaging.main(argv) == 2
    assert not out_dir.exists() and not records_dir.exists()
    exit_code = packaging.main(argv + ["--allow-unsigned"])
    assert exit_code == 0

    zips = list(out_dir.glob("*.zip"))
    checksums = list(out_dir.glob("*.zip.sha256"))
    records = list(records_dir.glob("RELEASE_ARTIFACT_*.json"))
    assert len(zips) == len(checksums) == len(records) == 1
    assert zips[0].name == (
        "SWBPIPE_0.1.0_aarch64-apple-darwin.app.zip"
    )
    digest = packaging.sha256_file(zips[0])
    assert checksums[0].read_text(encoding="utf-8") == packaging.checksum_line(
        digest, zips[0].name
    )
    record = json.loads(records[0].read_text(encoding="utf-8"))
    assert record["authenticity_chain"]["status"] == packaging.CHAIN_VERIFIED
    assert record["authenticity_chain"]["checksum_sha256"] == digest
    assert record["artifacts"][0]["sha256"] == digest
    assert record["unsigned_install_caveat"] == packaging.UNSIGNED_INSTALL_CAVEAT
    assert "install_note" not in record
    assert record["signing"]["state"] == packaging.SIGNING_UNSIGNED
    assert f"_{commit[:12]}.json" in records[0].name


def unsigned_inspection(packaging) -> dict:
    return {
        "state": packaging.SIGNING_UNSIGNED,
        "notarization": packaging.NOTARIZATION_NONE,
        "signature_valid": False,
        "hardened_runtime": False,
        "adhoc": False,
        "secure_timestamp": False,
        "ticket_stapled": False,
        "gatekeeper_accepted": False,
        "gatekeeper_source": None,
        "checks": [],
    }


def notarized_inspection(packaging) -> dict:
    return {
        **unsigned_inspection(packaging),
        "state": packaging.SIGNING_DEVELOPER_ID,
        "notarization": packaging.NOTARIZATION_STAPLED,
        "signature_valid": True,
        "hardened_runtime": True,
        "secure_timestamp": True,
        "ticket_stapled": True,
        "gatekeeper_accepted": True,
        "gatekeeper_source": "notarized_developer_id",
    }


IDENTITY_NAME = "Developer ID Application: Example Person (ABCDE12345)"


def fake_signing_tools(*, signed=True, stapled=True, notarized=True, runtime=True,
                       apple_developer_id=True, adhoc=False, source="Notarized Developer ID"):
    """A stand-in for codesign/stapler/spctl output, carrying identity strings."""

    def run(argv):
        tool = argv[:2]
        if argv[0] == "codesign" and "--verify" in argv and any(a.startswith("-R=") for a in argv):
            ok = signed and apple_developer_id and not adhoc
            return SimpleNamespace(returncode=0 if ok else 3, stdout="", stderr="" if ok else "test-requirement: code failed to satisfy specified code requirement(s)")
        if argv[0] == "codesign" and "--verify" in argv:
            return SimpleNamespace(returncode=0 if signed else 1, stdout="", stderr="" if signed else "code object is not signed at all")
        if argv[0] == "codesign":
            if not signed:
                return SimpleNamespace(returncode=1, stdout="", stderr="code object is not signed at all")
            flags = "0x10002(adhoc,runtime)" if adhoc else "0x10000(runtime)" if runtime else "0x0(none)"
            return SimpleNamespace(returncode=0, stdout="", stderr="\n".join([
                "Executable=/tmp/SWBPIPE.app/Contents/MacOS/swbpipe",
                f"CodeDirectory v=20500 size=1 flags={flags} hashes=1+7 location=embedded",
                f"Authority={IDENTITY_NAME}",
                "Authority=Developer ID Certification Authority",
                "Authority=Apple Root CA",
                "Timestamp=23 Oct 2026 at 10:00:00",
                "TeamIdentifier=ABCDE12345",
            ]))
        if tool == ["xcrun", "stapler"]:
            return SimpleNamespace(returncode=0 if stapled else 65, stdout="The validate action worked!" if stapled else "", stderr="")
        if argv[0] == "spctl":
            if not notarized:
                return SimpleNamespace(returncode=3, stdout="", stderr="/tmp/SWBPIPE.app: rejected\nsource=Unnotarized Developer ID")
            return SimpleNamespace(returncode=0, stdout="", stderr="\n".join([
                "/tmp/SWBPIPE.app: accepted",
                f"source={source}",
                f"origin={IDENTITY_NAME}",
            ]))
        raise AssertionError(argv)

    return run


def test_inspect_signing_classifies_and_drops_identity_strings(tmp_path):
    packaging = load_module()
    app = tmp_path / "SWBPIPE.app"
    inspection = packaging.inspect_signing(app, run=fake_signing_tools())
    assert inspection["state"] == packaging.SIGNING_DEVELOPER_ID
    assert inspection["notarization"] == packaging.NOTARIZATION_STAPLED
    assert inspection["hardened_runtime"] and inspection["secure_timestamp"]
    assert inspection["gatekeeper_source"] == "notarized_developer_id"
    assert packaging.is_signed_and_notarized(inspection)
    # The record is public: no authority name, team identifier or origin line.
    text = json.dumps(inspection)
    assert "Example Person" not in text and "ABCDE12345" not in text
    assert "origin" not in text and "Authority" not in text

    not_stapled = packaging.inspect_signing(app, run=fake_signing_tools(stapled=False))
    assert not_stapled["notarization"] == packaging.NOTARIZATION_NOT_STAPLED
    assert not packaging.is_signed_and_notarized(not_stapled)
    no_runtime = packaging.inspect_signing(app, run=fake_signing_tools(runtime=False))
    assert not packaging.is_signed_and_notarized(no_runtime)
    unnotarized = packaging.inspect_signing(app, run=fake_signing_tools(notarized=False))
    assert unnotarized["notarization"] == packaging.NOTARIZATION_NONE
    unsigned = packaging.inspect_signing(app, run=fake_signing_tools(signed=False, stapled=False, notarized=False))
    assert unsigned["state"] == packaging.SIGNING_UNSIGNED
    assert unnotarized["gatekeeper_source"] == "unnotarized_developer_id"
    # A look-alike name on a certificate Apple did not issue as Developer ID.
    deceptive = packaging.inspect_signing(app, run=fake_signing_tools(apple_developer_id=False))
    assert deceptive["state"] == packaging.SIGNING_OTHER
    assert not packaging.is_signed_and_notarized(deceptive)
    adhoc = packaging.inspect_signing(app, run=fake_signing_tools(adhoc=True))
    assert adhoc["state"] == packaging.SIGNING_OTHER and adhoc["adhoc"]
    # A local Gatekeeper rule label is never recorded verbatim, and never counts as notarized.
    labelled = packaging.inspect_signing(app, run=fake_signing_tools(source="Example Person's local rule"))
    assert labelled["gatekeeper_source"] == "other"
    assert labelled["notarization"] == packaging.NOTARIZATION_NONE
    assert "Example Person" not in json.dumps(labelled)
    # Tools absent (a non-macOS host): unsigned, never an exception.
    absent = packaging.inspect_signing(app, run=lambda argv: None)
    assert absent["state"] == packaging.SIGNING_UNSIGNED
    assert absent["notarization"] == packaging.NOTARIZATION_NONE


def test_execute_records_signed_notarized_bundle(tmp_path, monkeypatch):
    packaging = load_module()
    commit = "a" * 40
    app = make_fake_app(tmp_path)
    sweep = sweep_stub(tmp_path, commit)
    stub = SimpleNamespace(
        collect_git_state=lambda root: clean_git_state(commit),
        collect_runtime_versions=lambda root: {"platform": "test"},
        git_state_unverified=lambda git: not git.get("commit_hash"),
        is_complete_sweep_summary=lambda summary: True,
    )
    monkeypatch.setattr(packaging, "_sweep_module", lambda: stub)
    monkeypatch.setattr(packaging, "commit_timestamp", lambda root: 1783987200)
    monkeypatch.setattr(packaging, "inspect_signing", lambda app: notarized_inspection(packaging))
    roundtrip = {"value": {**notarized_inspection(packaging), "extracted": True}}
    monkeypatch.setattr(packaging, "inspect_zip_roundtrip", lambda zip_path, name: roundtrip["value"])
    argv = [
        "--execute", "--app-path", str(app), "--sweep-artifact", str(sweep),
        "--output-dir", str(tmp_path / "out"), "--records-dir", str(tmp_path / "records"),
    ]
    assert packaging.main(argv) == 0
    record = json.loads(next((tmp_path / "records").glob("*.json")).read_text(encoding="utf-8"))
    assert record["signing"]["state"] == packaging.SIGNING_DEVELOPER_ID
    assert record["signing"]["notarization"] == packaging.NOTARIZATION_STAPLED
    assert record["signing"]["zip_roundtrip"]["notarization"] == packaging.NOTARIZATION_STAPLED
    assert record["signing"]["basis"] == packaging.SIGNING_DECISION
    assert record["install_note"] == packaging.SIGNED_INSTALL_NOTE
    assert "unsigned_install_caveat" not in record

    # A zip whose extracted bundle no longer verifies fails loud: the zip and
    # checksum are removed and no record is written.
    for path in (tmp_path / "out").iterdir():
        path.unlink()
    for path in (tmp_path / "records").iterdir():
        path.unlink()
    roundtrip["value"] = {**unsigned_inspection(packaging), "extracted": True}
    assert packaging.main(argv) == 1
    assert not list((tmp_path / "out").iterdir())
    assert not list((tmp_path / "records").iterdir())


def test_execute_fails_loud_on_missing_bundle_or_wrong_arch(tmp_path, monkeypatch):
    packaging = load_module()
    stub = SimpleNamespace(
        collect_git_state=lambda root: clean_git_state(),
        collect_runtime_versions=lambda root: {"platform": "test"},
        git_state_unverified=lambda git: not git.get("commit_hash"),
        is_complete_sweep_summary=lambda summary: True,
    )
    monkeypatch.setattr(packaging, "_sweep_module", lambda: stub)

    missing = packaging.main(
        ["--execute", "--app-path", str(tmp_path / "Nope.app")]
    )
    assert missing == 2

    app = make_fake_app(tmp_path)
    executable = packaging.app_main_executable(app)
    executable.write_bytes(b"\xcf\xfa\xed\xfe" + struct.pack("<I", 0x01000007))
    wrong_arch = packaging.main(["--execute", "--app-path", str(app)])
    assert wrong_arch == 2


def test_committed_icons_reproduce_from_the_generator(tmp_path):
    """The committed icon assets are exactly the generator's output."""
    spec = importlib.util.spec_from_file_location(
        "generate_app_icon", ROOT / "tools" / "release" / "generate_app_icon.py"
    )
    assert spec is not None and spec.loader is not None
    generator = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = generator
    spec.loader.exec_module(generator)
    generator.generate(tmp_path)
    icons = ROOT / "apps" / "desktop" / "src-tauri" / "icons"
    assert (tmp_path / "icon.png").read_bytes() == (icons / "icon.png").read_bytes()
    assert (tmp_path / "icon.icns").read_bytes() == (icons / "icon.icns").read_bytes()


def test_dec025_sweep_surface_plan_is_unmodified():
    """This tranche sits beside the DEC-025 gate; the sweep plan is untouched."""
    spec = importlib.util.spec_from_file_location(
        "run_evidence_sweep_pkgtest",
        ROOT / "tools" / "release" / "run_evidence_sweep.py",
    )
    assert spec is not None and spec.loader is not None
    sweep = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = sweep
    spec.loader.exec_module(sweep)
    assert [surface.surface_id for surface in sweep.build_sweep_plan()] == [
        "cargo_crate_sweep",
        "python_pytest",
        "desktop_vitest",
        "desktop_playwright_e2e",
        "desktop_production_build",
    ]
