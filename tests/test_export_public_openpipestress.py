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
import shutil
import sys
from pathlib import Path

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


# ---------------------------------------------------------------------------
# Profile: fail-toward-exclusion allowlist shape.
# ---------------------------------------------------------------------------


def test_private_coordination_surfaces_are_excluded_top_level():
    for name in ("execution", "plans", "loop", "init", "_harness", "AGENTS.md"):
        assert name in PIPE.EXCLUDED_TOP_LEVEL
        assert name not in PIPE.ROOT_DIRS
        assert name not in PIPE.ROOT_FILES


def test_local_evidence_and_coordination_subtrees_are_excluded():
    assert "validation/evidence" in PIPE.EXCLUDED_SUBTREE_PREFIXES
    assert "tools/coordination" in PIPE.EXCLUDED_SUBTREE_PREFIXES


def test_no_ci_workflow_ships_in_the_export():
    # DEC-059 hard fence: CI activation is a separate owner-gated act.
    assert ".github" in PIPE.SKIP_DIR_NAMES


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


def make_source_tree(base: Path) -> Path:
    """Minimal invented tree carrying every allowlisted root."""
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


def test_selection_fails_closed_when_allowlisted_root_is_missing(tmp_path):
    source = make_source_tree(tmp_path)
    shutil.rmtree(source / "schemas")
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
    assert "<openpipestress-project-root>/docs/PLAN.md" in text
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
    assert ("non_allowlisted_binary", "docs/blob.bin") in kinds
    assert ("private_path_residue", "docs/leak.md") in kinds
    assert ("private_key_block", "docs/key.md") in kinds
    assert ("non_allowlisted_top_level", "not_allowlisted/x.md") in kinds
    assert not any(path == "docs/ok.md" for _, path in kinds)


def test_allowed_binary_suffix_is_not_flagged(tmp_path):
    staging = tmp_path / "staging"
    (staging / "apps").mkdir(parents=True)
    (staging / "apps" / "icon.png").write_bytes(b"\x89PNG\r\n\x1a\n\x00")
    assert PIPE.boundary_findings(staging) == []


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


@needs_cargo
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
def test_execute_fails_on_unexpected_blocking_lint_finding(tmp_path):
    source = make_source_tree(tmp_path)
    (source / "docs" / "planted.md").write_text(
        "OPS_SYNTHETIC" + "_PROTECTED_TABLE\n", encoding="utf-8"
    )
    target = tmp_path / "out"
    assert PIPE.main(
        ["--source-root", str(source), "--execute", "--target", str(target)]
    ) == 1
    record = json.loads((target / PIPE.RECORD_NAME).read_text(encoding="utf-8"))
    assert record["gate_result"] == "fail"
    assert record["lint_gate"]["unexpected_blocking_count"] == 1
    assert record["lint_gate"]["unexpected_blocking"][0]["path"] == "docs/planted.md"


@needs_cargo
def test_execute_treats_reviewed_fixture_findings_as_expected(tmp_path):
    source = make_source_tree(tmp_path)
    fixture_dir = source / "fixtures" / "report_lint" / "invented"
    fixture_dir.mkdir(parents=True)
    shutil.copy2(
        ROOT / "fixtures" / "report_lint" / "invented" / "synthetic_risk_template.txt",
        fixture_dir / "synthetic_risk_template.txt",
    )
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
def test_execute_fails_on_unexpected_boundary_finding(tmp_path):
    source = make_source_tree(tmp_path)
    (source / "docs" / "server.pem").write_text("not a real key\n", encoding="utf-8")
    target = tmp_path / "out"
    assert PIPE.main(
        ["--source-root", str(source), "--execute", "--target", str(target)]
    ) == 1
    record = json.loads((target / PIPE.RECORD_NAME).read_text(encoding="utf-8"))
    assert record["gate_result"] == "fail"
    assert record["boundary_check"]["unexpected_finding_count"] == 1


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
