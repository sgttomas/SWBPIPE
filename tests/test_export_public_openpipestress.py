#!/usr/bin/env python3
"""Self-tests for the DEC-059 sanitized public-export pipeline.

Covers the D-05b packet G1-G7 build surface: fail-toward-exclusion
selection, sanitize pass, manifest determinism, mechanical boundary check,
the DEL-08-05 protected-content lint gate (engine CLI consumed as-is), the
export record/report boundary language, and --verify drift detection.
Integration tests exercise the real engine CLI via cargo against synthetic
invented trees staged into pytest temp directories; nothing is exported,
published, or cleared.
"""

from __future__ import annotations

import csv
import importlib.util
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path, PureWindowsPath

import pytest


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "release" / "export_public_openpipestress.py"

CARGO_AVAILABLE = shutil.which("cargo") is not None


def load_module():
    spec = importlib.util.spec_from_file_location(
        "export_public_openpipestress", MODULE_PATH
    )
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


PIPE = load_module()


def png_chunk(kind: bytes, body: bytes) -> bytes:
    import struct
    import zlib

    return (struct.pack(">I", len(body)) + kind + body
            + struct.pack(">I", zlib.crc32(kind + body) & 0xFFFFFFFF))


def make_png(*extra_chunks: bytes) -> bytes:
    """A well-formed 1x1 greyscale PNG, with `extra_chunks` before IDAT."""
    import struct
    import zlib

    header = png_chunk(b"IHDR", struct.pack(">IIBBBBB", 1, 1, 8, 0, 0, 0, 0))
    pixels = png_chunk(b"IDAT", zlib.compress(b"\x00\x00"))
    return (b"\x89PNG\r\n\x1a\n" + header + b"".join(extra_chunks) + pixels
            + png_chunk(b"IEND", b""))


# ---------------------------------------------------------------------------
# Profile: fail-toward-exclusion allowlist shape.
# ---------------------------------------------------------------------------


def test_private_coordination_surfaces_are_excluded_top_level():
    for name in ("execution", "plans", "loop", "init", "_harness", "AGENTS.md"):
        assert name in PIPE.EXCLUDED_TOP_LEVEL
        assert name not in PIPE.ROOT_DIRS
        assert name not in PIPE.ROOT_FILES


def test_monorepo_bound_root_files_are_documented_exclusions():
    # RT-V8 P3: both were out only by construction before.
    for name in (".gitattributes", "software-workflow.json"):
        assert name in PIPE.EXCLUDED_TOP_LEVEL
        assert name not in PIPE.ROOT_FILES


def test_export_detection_matches_what_the_export_leaves_out():
    # tests/public_export.py and the readiness tool tell the export by these two
    # excluded directories; an export that kept either would run monorepo-only tests.
    assert {"loop", "execution"} <= set(PIPE.EXCLUDED_TOP_LEVEL)
    spec = importlib.util.spec_from_file_location("public_export_probe", ROOT / "tests" / "public_export.py")
    probe = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(probe)
    assert probe.PROJECT == ROOT
    # The monorepo and hosted CI's checkout keep `loop/`; only the export lacks it.
    assert probe.PUBLIC_EXPORT == (not (ROOT / "loop").is_dir())


def test_report_states_how_the_staged_tree_verifies_itself():
    text = "\n".join(PIPE.SELF_VERIFICATION_LINES)
    assert "python3 -m pytest -q tests" in text
    assert "check_release_readiness.py --profile skeleton --execute" in text
    assert "tests/public_export.py" in text
    # The staged dry run failed 38 commit-binding tests outside a git checkout,
    # and a plain `git add -A` drops the force-added Cargo.lock files.
    assert "git init -q && git add -A -f && git commit" in text


def test_local_evidence_and_coordination_subtrees_are_excluded():
    assert "validation/evidence" in PIPE.EXCLUDED_SUBTREE_PREFIXES
    assert "tools/coordination" in PIPE.EXCLUDED_SUBTREE_PREFIXES


def test_no_ci_workflow_ships_in_the_export():
    # DEC-059 hard fence: CI activation is a separate owner-gated act.
    assert ".github" in PIPE.SKIP_DIR_NAMES


def test_lint_selection_is_derived_shared_and_case_insensitive(tmp_path):
    # Review finding 5: duplicated hand-kept suffix lists omitted shipped text
    # (.rs, .py, .ts, .tsx, .lock), and uppercase extensions bypassed them.
    spec = importlib.util.spec_from_file_location(
        "scan_member_probe", ROOT / "tools" / "release" / "run_release_candidate_scan.py"
    )
    scan = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(scan)
    assert scan.is_text_member is PIPE.is_text_member

    staging = tmp_path / "staging"
    (staging / "docs").mkdir(parents=True)
    text = ["lib.rs", "tool.py", "view.ts", "panel.tsx", "Cargo.lock", "NOTES.MD",
            "DATA.JSONL", "SHA256SUMS", "model.MBF"]
    for name in text:
        (staging / "docs" / name).write_text("invented\n", encoding="utf-8")
    (staging / "docs" / "icon.PNG").write_bytes(make_png())
    (staging / "docs" / "server.PEM").write_text("not a real key\n", encoding="utf-8")
    (staging / "docs" / "run.LOG").write_text("x\n", encoding="utf-8")

    linted = {p.name for p in PIPE.lintable_staged_files(staging)}
    assert set(text) <= linted
    assert "icon.PNG" not in linted

    findings = {(f["path"].rsplit("/", 1)[-1], f["kind"]) for f in PIPE.boundary_findings(staging)}
    assert ("server.PEM", "secret_material_filename") in findings
    assert ("run.LOG", "forbidden_suffix") in findings
    assert ("icon.PNG", "unreviewed_binary") in findings

    inventory = {entry["path"]: entry["lintable"] for entry in scan.inventory_class(staging)}
    assert inventory["docs/lib.rs"] is True and inventory["docs/Cargo.lock"] is True
    assert inventory["docs/icon.PNG"] is False


def test_skip_reason_rules():
    assert PIPE.skip_reason(Path("apps/node_modules/x.js")) == "skip_dir_name"
    assert PIPE.skip_reason(Path("core/target/debug/lib.rlib")) == "skip_dir_name"
    assert PIPE.skip_reason(Path("docs/.env.local")) == "env_file"
    assert PIPE.skip_reason(Path("docs/.DS_Store")) == "skip_file_name"
    assert PIPE.skip_reason(Path("tests/__pycache__/t.pyc")) == "skip_dir_name"
    assert PIPE.skip_reason(Path("docs/notes.pyc")) == "skip_suffix"
    assert PIPE.skip_reason(Path("validation/evidence/s.json")) == "excluded_subtree"
    assert PIPE.skip_reason(Path("tools/coordination/x.py")) == "excluded_subtree"
    assert (
        PIPE.skip_reason(Path("apps/desktop/public/wasm-engine.tmp-1/x.js"))
        == "excluded_subtree"
    )
    assert PIPE.skip_reason(Path("fixtures/quarantine/bad.txt")) == "quarantine_path"
    assert PIPE.skip_reason(Path("docs/README.md")) is None
    assert PIPE.skip_reason(Path("core/solver/src/lib.rs")) is None


# ---------------------------------------------------------------------------
# Synthetic source trees.
# ---------------------------------------------------------------------------


# The export is cut only from a git commit, so each synthetic tree is a git
# repository of its own, isolated from the caller's git configuration.
GIT_ENV = {
    **os.environ,
    "GIT_CONFIG_GLOBAL": os.devnull,
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_AUTHOR_NAME": "invented",
    "GIT_AUTHOR_EMAIL": "invented@example.invalid",
    "GIT_COMMITTER_NAME": "invented",
    "GIT_COMMITTER_EMAIL": "invented@example.invalid",
}


def git(source: Path, *args: str) -> None:
    subprocess.run(["git", *args], cwd=source, env=GIT_ENV, check=True,
                   capture_output=True)


def commit_all(source: Path, message: str = "invented") -> None:
    """Commit the synthetic tree as it stands (ignored files stay out)."""
    git(source, "add", "-A")
    git(source, "commit", "-q", "--allow-empty", "-m", message)


def make_source_tree(base: Path, *, commit: bool = True) -> Path:
    """Minimal invented tree carrying every allowlisted root, committed to a
    git repository of its own unless `commit` is False."""
    source = base / "source"
    for name in PIPE.ROOT_FILES:
        source.mkdir(parents=True, exist_ok=True)
        (source / name).write_text(f"invented {name}\n", encoding="utf-8")
    for name in PIPE.ROOT_DIRS:
        (source / name).mkdir(parents=True, exist_ok=True)
        (source / name / "README.md").write_text(
            f"Invented {name} placeholder.\n", encoding="utf-8"
        )
    # Private surfaces that must never be staged.
    (source / "execution").mkdir()
    (source / "execution" / "secret_plan.md").write_text(
        "private coordination\n", encoding="utf-8"
    )
    (source / "plans").mkdir()
    (source / "plans" / "PLAN.md").write_text("private plan\n", encoding="utf-8")
    (source / "validation" / "evidence").mkdir(parents=True)
    (source / "validation" / "evidence" / "SWEEP_X.json").write_text(
        "{}\n", encoding="utf-8"
    )
    (source / "apps" / "node_modules").mkdir(parents=True)
    (source / "apps" / "node_modules" / "pkg.js").write_text(
        "junk\n", encoding="utf-8"
    )
    (source / "docs" / ".env.local").write_text("SECRET=1\n", encoding="utf-8")
    if commit:
        git(source, "init", "-q")
        commit_all(source)
    return source


def test_selection_stages_allowlist_and_drops_private_surfaces(tmp_path):
    source = make_source_tree(tmp_path)
    selected, exclusions, unlisted = PIPE.select_files(source)
    posix = {rel.as_posix() for rel in selected}

    assert "docs/README.md" in posix
    assert ".gitignore" in posix
    assert not any(p.startswith("execution/") for p in posix)
    assert not any(p.startswith("plans/") for p in posix)
    assert not any(p.startswith("validation/evidence/") for p in posix)
    assert not any("node_modules" in p for p in posix)
    assert not any(".env" in p for p in posix)

    assert exclusions.get("excluded_subtree", 0) >= 1
    assert exclusions.get("skip_dir_name", 0) >= 1
    assert exclusions.get("env_file", 0) >= 1
    assert "execution" in unlisted
    assert "plans" in unlisted


@pytest.mark.skipif(shutil.which("git") is None, reason="git is not installed")
def test_selection_in_a_git_checkout_stages_tracked_files_only(tmp_path):
    # G3: the record binds the export to a commit, and `git status --porcelain`
    # does not show ignored files, so ignored or untracked local files must not
    # be staged under an allowlisted root.
    source = make_source_tree(tmp_path, commit=False)
    (source / ".gitignore").write_text("*.local.json\n", encoding="utf-8")
    git(source, "init", "-q")
    commit_all(source)
    (source / "fixtures" / "scratch_result.json").write_text("{}\n", encoding="utf-8")
    (source / "docs" / "notes.local.json").write_text("{}\n", encoding="utf-8")

    selected, _, _ = PIPE.select_files(source)
    posix = {rel.as_posix() for rel in selected}

    assert "docs/README.md" in posix
    assert "fixtures/scratch_result.json" not in posix
    assert "docs/notes.local.json" not in posix


def test_selection_reads_head_for_a_project_nested_in_its_repository(tmp_path):
    # The real source root is projects/<name> inside the monorepo; run from a
    # subdirectory, `git ls-tree HEAD:<prefix>` listed nothing without
    # --full-tree, so every allowlisted root read as missing.
    repository = tmp_path / "repository"
    source = make_source_tree(repository / "projects", commit=False)
    git(repository, "init", "-q")
    commit_all(repository)
    selected, _, unlisted = PIPE.select_files(source)
    posix = {rel.as_posix() for rel in selected}
    assert {"README.md", "docs/README.md"} <= posix
    assert "execution" in unlisted

    staging = tmp_path / "staging"
    PIPE.stage_files(source, staging, selected)
    assert (staging / "docs" / "README.md").read_text(encoding="utf-8") == (
        "Invented docs placeholder.\n"
    )


def test_export_stages_head_blobs_not_working_tree_state(tmp_path):
    # Review finding 1: the export copied working-tree bytes, so dirty tracked
    # edits and newly indexed files entered an export bound to HEAD.
    source = make_source_tree(tmp_path)
    committed = (source / "docs" / "README.md").read_bytes()
    (source / "docs" / "README.md").write_text("uncommitted edit\n", encoding="utf-8")
    (source / "fixtures" / "indexed_only.json").write_text("{}\n", encoding="utf-8")
    git(source, "add", "fixtures/indexed_only.json")
    (source / "api" / "README.md").unlink()
    (source / "private_local_dir").mkdir()
    (source / "private_local_dir" / "x.txt").write_text("x\n", encoding="utf-8")

    selected, _, unlisted = PIPE.select_files(source)
    posix = {rel.as_posix() for rel in selected}
    assert "fixtures/indexed_only.json" not in posix
    assert "api/README.md" in posix
    assert "private_local_dir" not in unlisted

    staging = tmp_path / "staging"
    PIPE.stage_files(source, staging, selected)
    assert (staging / "docs" / "README.md").read_bytes() == committed
    assert (staging / "api" / "README.md").read_text(encoding="utf-8") == (
        "Invented api placeholder.\n"
    )


def test_selection_refuses_a_source_outside_git(tmp_path):
    # Review finding 3: without a git probe the export fell back to a
    # filesystem walk that admits ignored and untracked material. It refuses.
    source = make_source_tree(tmp_path, commit=False)
    assert PIPE.tracked_files(source) is None
    with pytest.raises(SystemExit) as excinfo:
        PIPE.select_files(source)
    assert "GIT_PROBE_FAILED" in str(excinfo.value)


def test_execute_refuses_a_source_outside_git(tmp_path):
    source = make_source_tree(tmp_path, commit=False)
    with pytest.raises(SystemExit) as excinfo:
        PIPE.main(["--source-root", str(source), "--execute", "--target", str(tmp_path / "out")])
    assert "GIT_PROBE_FAILED" in str(excinfo.value)
    assert not (tmp_path / "out" / PIPE.STAGING_DIR_NAME).exists()


def test_no_symlink_is_exported_at_any_level(tmp_path):
    # Review finding 2: a root-file symlink bypassed the subtree exclusion and
    # copied bytes from outside the tree.
    outside = tmp_path / "outside.txt"
    outside.write_text("external bytes\n", encoding="utf-8")

    source = make_source_tree(tmp_path / "root_case")
    (source / "README.md").unlink()
    (source / "README.md").symlink_to(outside)
    commit_all(source)
    with pytest.raises(SystemExit) as excinfo:
        PIPE.select_files(source)
    assert "SYMLINKED_ROOT" in str(excinfo.value)

    source = make_source_tree(tmp_path / "subtree_case")
    (source / "docs" / "linked.md").symlink_to(outside)
    commit_all(source)
    selected, exclusions, _ = PIPE.select_files(source)
    assert "docs/linked.md" not in {rel.as_posix() for rel in selected}
    assert exclusions.get("symlink") == 1

    staging = tmp_path / "staging"
    (staging / "docs").mkdir(parents=True)
    (staging / "docs" / "linked.md").symlink_to(outside)
    kinds = {f["kind"] for f in PIPE.boundary_findings(staging)}
    assert "symlink" in kinds


def test_selection_fails_closed_when_allowlisted_root_is_missing(tmp_path):
    source = make_source_tree(tmp_path)
    shutil.rmtree(source / "schemas")
    commit_all(source)
    with pytest.raises(SystemExit) as excinfo:
        PIPE.select_files(source)
    assert "schemas" in str(excinfo.value)


def test_sanitize_replaces_private_absolute_paths(tmp_path):
    staging = tmp_path / "staging"
    staging.mkdir()
    private = PIPE.PUBLIC_REPLACEMENTS[0][0]
    (staging / "note.md").write_text(
        f"path {private}/docs/PLAN.md here\n", encoding="utf-8"
    )
    changed = PIPE.sanitize_text_files(staging)
    assert changed == 1
    text = (staging / "note.md").read_text(encoding="utf-8")
    assert "<swbpipe-project-root>/docs/PLAN.md" in text
    assert PIPE.PRIVATE_PATH_RESIDUE_MARKER not in text


# ---------------------------------------------------------------------------
# Boundary check (G4 mechanical pass).
# ---------------------------------------------------------------------------


def test_boundary_findings_flag_planted_risks(tmp_path):
    staging = tmp_path / "staging"
    (staging / "docs").mkdir(parents=True)
    (staging / "docs" / "ok.md").write_text("clean\n", encoding="utf-8")
    (staging / "docs" / "server.pem").write_text("x\n", encoding="utf-8")
    (staging / "docs" / "blob.bin").write_bytes(b"\x00\x01\x02\xff")
    (staging / "docs" / "leak.md").write_text(
        "see /Users" + "/someone/private\n", encoding="utf-8"
    )
    (staging / "docs" / "key.md").write_text(
        "-----" + "BEGIN RSA PRIVATE" + " KEY-----\n", encoding="utf-8"
    )
    (staging / "not_allowlisted").mkdir()
    (staging / "not_allowlisted" / "x.md").write_text("x\n", encoding="utf-8")

    kinds = {(f["kind"], f["path"]) for f in PIPE.boundary_findings(staging)}
    assert ("secret_material_filename", "docs/server.pem") in kinds
    assert ("unreviewed_binary", "docs/blob.bin") in kinds
    assert ("private_path_residue", "docs/leak.md") in kinds
    assert ("private_key_block", "docs/key.md") in kinds
    assert ("non_allowlisted_top_level", "not_allowlisted/x.md") in kinds
    assert not any(path == "docs/ok.md" for _, path in kinds)


def reviewed_entry(path: str, data: bytes) -> dict:
    import hashlib

    return {"path": path, "sha256": hashlib.sha256(data).hexdigest(),
            "size_bytes": len(data), "reason": "invented test entry"}


def test_binary_ships_only_when_its_exact_hash_is_reviewed(tmp_path):
    # Review r3: content parsing let CRC-correct PNGs carry text (IEND body,
    # oversized gAMA, invalid IDAT, bytes after the zlib stream), and r2: a
    # non-PNG named .png. No format is parsed now: a binary ships only when
    # its exact path, size and SHA-256 are on the reviewed list.
    import zlib

    phrase = ("OPS_SYNTHETIC" + "_PROTECTED_TABLE").encode("ascii")
    base = make_png()
    head, tail = base[: base.index(b"IDAT") - 4], base[base.index(b"IEND") - 4 :]
    adversarial = {
        "iend_text.png": base[: base.index(b"IEND") - 4] + png_chunk(b"IEND", phrase),
        "oversized_gama.png": make_png(png_chunk(b"gAMA", b"\x00" * 64 + phrase)),
        "invalid_idat.png": head + png_chunk(b"IDAT", phrase) + tail,
        "trailing_zlib.png": head + png_chunk(b"IDAT", zlib.compress(b"\x00\x00") + phrase) + tail,
        "not_a.png": b"\x00\xff invented bytes \xfe",
        "blob.bin": b"\x00\xff\xfe",
        "icon.icns": b"icns\x00\x00\x00\x09\xff",
        "no_suffix": b"\xff\xfe\x00",
        "valid.png": base,
    }
    staging = tmp_path / "staging"
    (staging / "apps").mkdir(parents=True)
    for name, data in adversarial.items():
        (staging / "apps" / name).write_bytes(data)
    (staging / "apps" / "text_named.png").write_text("invented text\n", encoding="utf-8")

    findings = {(f["path"], f["kind"]) for f in PIPE.boundary_findings(staging, reviewed={})}
    for name in adversarial:
        assert ("apps/" + name, "unreviewed_binary") in findings, name
    assert not any(path == "apps/text_named.png" for path, _ in findings)
    assert {p.name for p in PIPE.lintable_staged_files(staging)} == {"text_named.png"}

    reviewed = {
        "apps/valid.png": reviewed_entry("apps/valid.png", base),
        "apps/not_a.png": reviewed_entry("apps/not_a.png", b"other reviewed bytes"),
    }
    findings = {(f["path"], f["kind"]) for f in PIPE.boundary_findings(staging, reviewed=reviewed)}
    assert not any(path == "apps/valid.png" for path, _ in findings)
    assert ("apps/not_a.png", "reviewed_binary_hash_mismatch") in findings
    assert ("apps/iend_text.png", "unreviewed_binary") in findings

    # By default the list comes from the staged tree itself (so from HEAD).
    listing = staging / "tools" / "release" / "reviewed_binaries.json"
    listing.parent.mkdir(parents=True)
    listing.write_text(json.dumps({"binaries": [reviewed["apps/valid.png"]]}), encoding="utf-8")
    assert not any(f["path"] == "apps/valid.png" for f in PIPE.boundary_findings(staging))


def test_committed_reviewed_binary_list_matches_its_files():
    # Each entry is one exact reviewed file; the list cannot drift from it.
    import hashlib

    import release_members

    listing = ROOT / release_members.REVIEWED_BINARIES_PATH
    document = json.loads(listing.read_text(encoding="utf-8"))
    entries = release_members.load_reviewed_binaries(listing)
    assert len(entries) == len(document["binaries"])
    for path, entry in entries.items():
        data = (ROOT / path).read_bytes()
        assert entry["sha256"] == hashlib.sha256(data).hexdigest(), path
        assert entry["size_bytes"] == len(data) and entry["reason"].strip()


def test_reviewed_invented_home_paths_still_catch_a_real_one(tmp_path):
    # RT-V8 P2: the eight reviewed test files hold only invented placeholder
    # home paths; the reviewed kind applies only while that stays true.
    home = "/Users" + "/"
    staging = tmp_path / "staging"
    (staging / "core").mkdir(parents=True)
    (staging / "core" / "invented_only.rs").write_text(
        f'"{home}invented/Documents" "{home}invented-user/x.pdf"\n', encoding="utf-8")
    (staging / "core" / "mixed.rs").write_text(
        f'"{home}invented/Documents" "{home}someone-real/x"\n', encoding="utf-8")
    kinds = {(f["path"], f["kind"]) for f in PIPE.boundary_findings(staging, reviewed={})}
    assert ("core/invented_only.rs", "private_path_residue_invented") in kinds
    assert ("core/mixed.rs", "private_path_residue") in kinds
    reviewed = [path for (path, kind) in PIPE.EXPECTED_BOUNDARY_FINDINGS
                if kind == "private_path_residue_invented"]
    assert len(reviewed) == 8


def test_classify_boundary_findings_splits_expected_and_unexpected():
    expected_key = next(iter(PIPE.EXPECTED_BOUNDARY_FINDINGS))
    findings = [
        {"kind": expected_key[1], "path": expected_key[0]},
        {"kind": "private_key_block", "path": "docs/planted.md"},
    ]
    result = PIPE.classify_boundary_findings(findings)
    assert result["gate_pass"] is False
    assert len(result["expected"]) == 1
    assert result["expected"][0]["expected_justification"]
    assert len(result["unexpected"]) == 1
    assert result["unexpected"][0]["path"] == "docs/planted.md"

    clean = PIPE.classify_boundary_findings([])
    assert clean["gate_pass"] is True


# ---------------------------------------------------------------------------
# Lint gate classification (engine payload handling; no cargo needed).
# ---------------------------------------------------------------------------


def stub_lint_result(findings: list[dict]) -> dict:
    return {
        "status": "findings_present" if findings else "pass",
        "command": "cargo run ...",
        "summary": {
            "target_count": 1,
            "scanned_target_count": 1,
            "finding_count": len(findings),
            "blocking_finding_count": sum(
                1 for f in findings if f["severity"] == "BLOCKING"
            ),
            "clean_scan_is_clearance": False,
        },
        "findings": findings,
        "note": None,
    }


def test_classify_lint_gate_expected_unexpected_and_warnings(tmp_path):
    staging = tmp_path / "staging"
    staging.mkdir()
    expected_path, expected_code = next(iter(PIPE.EXPECTED_BLOCKING_LINT_FINDINGS))
    findings = [
        {
            "path": str(staging / expected_path),
            "code": expected_code,
            "severity": "BLOCKING",
        },
        {
            "path": str(staging / "docs" / "planted.md"),
            "code": "PROTECTED_CONTENT_SYNTHETIC_MARKER",
            "severity": "BLOCKING",
        },
        {
            "path": str(staging / "docs" / "grid.md"),
            "code": "UNKNOWN_PROVENANCE_REVIEW_REQUIRED",
            "severity": "WARNING",
        },
    ]
    gate = PIPE.classify_lint_gate(stub_lint_result(findings), staging)
    assert gate["gate_pass"] is False
    assert [f["path"] for f in gate["expected_blocking"]] == [expected_path]
    assert [f["path"] for f in gate["unexpected_blocking"]] == ["docs/planted.md"]
    assert gate["warning_count"] == 1
    # Unmatched expectation entries are reported for pruning, never a pass.
    assert any(expected_code not in item for item in gate["stale_expectations"]) or (
        len(gate["stale_expectations"])
        == len(PIPE.EXPECTED_BLOCKING_LINT_FINDINGS) - 1
    )


def test_classify_lint_gate_matches_only_exact_unc_staging_root():
    class UncStaging:
        def resolve(self):
            return PureWindowsPath(r"\\?\UNC\server\share\export\staging")

    expected_path, expected_code = next(iter(PIPE.EXPECTED_BLOCKING_LINT_FINDINGS))

    def classify(finding_path):
        finding = {
            "path": finding_path,
            "code": expected_code,
            "severity": "BLOCKING",
        }
        return PIPE.classify_lint_gate(stub_lint_result([finding]), UncStaging())

    exact = classify(f"unc://server/share/export/staging/{expected_path}")
    assert exact["gate_pass"] is True
    assert [finding["path"] for finding in exact["expected_blocking"]] == [expected_path]

    for path in (
        f"unc://server/share/export/staging-sibling/{expected_path}",
        f"unc://server/share/export/staging/../staging/{expected_path}",
    ):
        outside = classify(path)
        assert outside["gate_pass"] is False
        assert outside["expected_blocking"] == []
        assert outside["unexpected_blocking"][0]["path"] == path


def test_unavailable_engine_fails_the_gate(tmp_path):
    result = {
        "status": "engine_unavailable",
        "command": "cargo run ...",
        "summary": None,
        "findings": [],
        "note": "Engine CLI unavailable: boom. Fails toward review.",
    }
    gate = PIPE.classify_lint_gate(result, tmp_path)
    assert gate["gate_pass"] is False
    assert "unavailable" in gate["note"]


def test_expected_lint_findings_are_exact_paths_with_justifications():
    for (path, code), justification in PIPE.EXPECTED_BLOCKING_LINT_FINDINGS.items():
        assert "*" not in path and "*" not in code
        assert justification.strip()


# ---------------------------------------------------------------------------
# Plan mode (default: no writes).
# ---------------------------------------------------------------------------


def test_plan_mode_runs_clean_on_synthetic_tree(tmp_path, capsys):
    source = make_source_tree(tmp_path)
    before = {p for p in tmp_path.rglob("*")}
    assert PIPE.main(["--source-root", str(source)]) == 0
    assert {p for p in tmp_path.rglob("*")} == before
    out = capsys.readouterr().out
    assert "plan only (no writes)" in out
    assert "clears nothing" in out


def test_target_and_force_require_execute(tmp_path):
    assert PIPE.main(["--target", str(tmp_path / "out")]) == 2
    assert PIPE.main(["--execute"]) == 2


# ---------------------------------------------------------------------------
# Integration: execute/verify against the real DEL-08-05 engine CLI.
# ---------------------------------------------------------------------------

needs_cargo = pytest.mark.skipif(
    not CARGO_AVAILABLE, reason="cargo unavailable; lint-gate integration skipped"
)
runs_lint_engine = pytest.mark.runs_cargo("cargo run of the protected-content lint engine")


@needs_cargo
@runs_lint_engine
def test_execute_green_run_emits_bound_deterministic_export(tmp_path):
    source = make_source_tree(tmp_path)
    target_one = tmp_path / "out-one"
    target_two = tmp_path / "out-two"

    assert PIPE.main(
        ["--source-root", str(source), "--execute", "--target", str(target_one)]
    ) == 0
    for name in (
        PIPE.STAGING_DIR_NAME,
        PIPE.MANIFEST_NAME,
        PIPE.REPORT_NAME,
        PIPE.RECORD_NAME,
        PIPE.LINT_RUN_NAME,
    ):
        assert (target_one / name).exists()

    record = json.loads((target_one / PIPE.RECORD_NAME).read_text(encoding="utf-8"))
    assert record["decision_basis"] == "DEC-059"
    assert record["gate_result"] == "pass"
    assert record["boundary_check"]["unexpected_finding_count"] == 0
    assert record["lint_gate"]["clean_scan_is_clearance"] is False
    assert sorted(record["guarantees"]) == ["G1", "G2", "G3", "G4", "G5", "G6", "G7"]
    assert "not a" in record["boundary_note"]
    assert record["source"]["git"]["commit_hash"] is None or isinstance(
        record["source"]["git"]["commit_hash"], str
    )

    with (target_one / PIPE.MANIFEST_NAME).open(encoding="utf-8", newline="") as fh:
        rows = list(csv.DictReader(fh))
    assert rows and set(rows[0]) == {"path", "size_bytes", "sha256"}
    staged = {p.relative_to(target_one / "staging").as_posix()
              for p in (target_one / "staging").rglob("*") if p.is_file()}
    assert staged == {row["path"] for row in rows}
    assert not any(p.startswith("execution/") for p in staged)

    # Determinism (G3): a second run over the same tree is byte-identical.
    assert PIPE.main(
        ["--source-root", str(source), "--execute", "--target", str(target_two)]
    ) == 0
    assert (
        (target_one / PIPE.MANIFEST_NAME).read_bytes()
        == (target_two / PIPE.MANIFEST_NAME).read_bytes()
    )

    # Verify (G7): clean, then drift after tampering.
    assert PIPE.main(["--verify", str(target_one)]) == 0
    (target_one / "staging" / "docs" / "README.md").write_text(
        "tampered\n", encoding="utf-8"
    )
    assert PIPE.main(["--verify", str(target_one)]) == 1


@needs_cargo
@runs_lint_engine
def test_execute_fails_on_unexpected_blocking_lint_finding(tmp_path):
    source = make_source_tree(tmp_path)
    (source / "docs" / "planted.md").write_text(
        "OPS_SYNTHETIC" + "_PROTECTED_TABLE\n", encoding="utf-8"
    )
    commit_all(source)
    target = tmp_path / "out"
    assert PIPE.main(
        ["--source-root", str(source), "--execute", "--target", str(target)]
    ) == 1
    record = json.loads((target / PIPE.RECORD_NAME).read_text(encoding="utf-8"))
    assert record["gate_result"] == "fail"
    assert record["lint_gate"]["unexpected_blocking_count"] == 1
    assert record["lint_gate"]["unexpected_blocking"][0]["path"] == "docs/planted.md"


@needs_cargo
@runs_lint_engine
def test_execute_treats_reviewed_fixture_findings_as_expected(tmp_path):
    source = make_source_tree(tmp_path)
    fixture_dir = source / "fixtures" / "report_lint" / "invented"
    fixture_dir.mkdir(parents=True)
    shutil.copy2(
        ROOT / "fixtures" / "report_lint" / "invented" / "synthetic_risk_template.txt",
        fixture_dir / "synthetic_risk_template.txt",
    )
    commit_all(source)
    target = tmp_path / "out"
    assert PIPE.main(
        ["--source-root", str(source), "--execute", "--target", str(target)]
    ) == 0
    record = json.loads((target / PIPE.RECORD_NAME).read_text(encoding="utf-8"))
    assert record["gate_result"] == "pass"
    assert record["lint_gate"]["expected_blocking_count"] == 2
    assert record["lint_gate"]["unexpected_blocking_count"] == 0
    # Expected findings stay visible in the report for the owner's D-20 scan.
    report = (target / PIPE.REPORT_NAME).read_text(encoding="utf-8")
    assert "Reviewed expected blocking findings" in report


@needs_cargo
@runs_lint_engine
def test_execute_fails_on_unexpected_boundary_finding(tmp_path):
    source = make_source_tree(tmp_path)
    (source / "docs" / "server.pem").write_text("not a real key\n", encoding="utf-8")
    commit_all(source)
    target = tmp_path / "out"
    assert PIPE.main(
        ["--source-root", str(source), "--execute", "--target", str(target)]
    ) == 1
    record = json.loads((target / PIPE.RECORD_NAME).read_text(encoding="utf-8"))
    assert record["gate_result"] == "fail"
    assert record["boundary_check"]["unexpected_finding_count"] == 1


def test_execute_outputs_carry_no_host_paths_or_untracked_names(tmp_path, monkeypatch):
    # Review leak surfaces: absolute-path metadata, names of local artifacts,
    # raw lint paths and engine logs must not reach the record, report or
    # lint run. The engine is stubbed so no cargo runs.
    source = make_source_tree(tmp_path)
    (source / "invented-private-client-notes.txt").write_text("x\n", encoding="utf-8")
    target = tmp_path / "out"

    def fake_lint(paths, engine_root):
        return {
            "status": "findings_present",
            "command": "cargo run --quiet ...",
            "summary": {"target_count": len(paths), "finding_count": 1,
                        "blocking_finding_count": 0},
            "findings": [{"path": str(paths[0]), "severity": "WARNING",
                          "code": "UNKNOWN_PROVENANCE_REVIEW_REQUIRED", "line": 1}],
            "note": None,
        }

    monkeypatch.setattr(PIPE, "run_engine_lint", fake_lint)
    PIPE.main(["--source-root", str(source), "--execute", "--target", str(target)])
    for name in (PIPE.RECORD_NAME, PIPE.REPORT_NAME, PIPE.LINT_RUN_NAME):
        text = (target / name).read_text(encoding="utf-8")
        assert str(tmp_path) not in text and str(tmp_path.resolve()) not in text, name
        assert "invented-private-client-notes" not in text, name
    record = json.loads((target / PIPE.RECORD_NAME).read_text(encoding="utf-8"))
    assert record["source"]["source_root"] == "<repository>/"
    assert record["source"]["git"]["dirty_path_count"] == 1

    monkeypatch.undo()
    host_text = str(tmp_path / "engine-log-path")
    monkeypatch.setattr(
        PIPE, "run_command",
        lambda command, cwd: PIPE.subprocess.CompletedProcess(command, 1, "", host_text),
    )
    note = PIPE.run_engine_lint([source / "docs" / "README.md"], source)["note"]
    assert host_text not in note and "exited 1" in note


def test_execute_refuses_unsafe_targets(tmp_path):
    source = make_source_tree(tmp_path)
    inside = source / "export-out"
    assert PIPE.main(
        ["--source-root", str(source), "--execute", "--target", str(inside)]
    ) == 2

    occupied = tmp_path / "occupied"
    occupied.mkdir()
    (occupied / "keep.txt").write_text("x\n", encoding="utf-8")
    assert PIPE.main(
        ["--source-root", str(source), "--execute", "--target", str(occupied)]
    ) == 2
    assert (occupied / "keep.txt").exists()
