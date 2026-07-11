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
    path.write_text(
        json.dumps(
            {
                "git": {"commit_hash": commit, "working_tree_dirty": dirty},
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
    'OpenPipeStress Technical Preview.app')."""
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
            "product_name": "OpenPipeStress Technical Preview",
            "version": "0.1.0",
            "identifier": "org.openpipestress.technical-preview",
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
    )
    monkeypatch.setattr(packaging, "_sweep_module", lambda: stub)
    monkeypatch.setattr(packaging, "commit_timestamp", lambda root: 1783987200)

    out_dir = tmp_path / "out"
    records_dir = tmp_path / "records"
    exit_code = packaging.main(
        [
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
    )
    assert exit_code == 0

    zips = list(out_dir.glob("*.zip"))
    checksums = list(out_dir.glob("*.zip.sha256"))
    records = list(records_dir.glob("RELEASE_ARTIFACT_*.json"))
    assert len(zips) == len(checksums) == len(records) == 1
    assert zips[0].name == (
        "OpenPipeStress-Technical-Preview_0.1.0_aarch64-apple-darwin.app.zip"
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
    assert f"_{commit[:12]}.json" in records[0].name


def test_execute_fails_loud_on_missing_bundle_or_wrong_arch(tmp_path, monkeypatch):
    packaging = load_module()
    stub = SimpleNamespace(
        collect_git_state=lambda root: clean_git_state(),
        collect_runtime_versions=lambda root: {"platform": "test"},
        git_state_unverified=lambda git: not git.get("commit_hash"),
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
