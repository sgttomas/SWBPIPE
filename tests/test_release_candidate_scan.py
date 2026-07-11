#!/usr/bin/env python3
"""Tests for the DEC-058 release-candidate scan runner and record emitter.

All fixture content is invented at test time inside pytest tmp directories:
lookalike shapes only, never protected standards data. No test writes to
`validation/evidence/releases/` — real scan records are emitted only by the
owner under DEC-058; here the output directory is always a temp directory.
"""

from __future__ import annotations

import importlib.util
import json
import shutil
import subprocess
import sys
from pathlib import Path

import pytest


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "release" / "run_release_candidate_scan.py"

FIXED_NOW = "2026-07-10T12:00:00+00:00"
FIXED_COMMIT = "abcdef0123456789abcdef0123456789abcdef01"

ACCEPTED_PROVENANCE = {
    "source_name": "OpenPipeStress invented example",
    "source_location": "invented/inline",
    "source_license": "invented-public",
    "contributor": "OpenPipeStress",
    "contributor_certification": (
        "Invented non-engineering example; no protected source copied."
    ),
    "redistribution_status": "public_permissive",
    "review_status": "accepted",
}


def load_module():
    spec = importlib.util.spec_from_file_location(
        "run_release_candidate_scan", MODULE_PATH
    )
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def make_candidate_tree(tmp_path: Path) -> Path:
    candidate = tmp_path / "candidate"
    candidate.mkdir()
    (candidate / "README.md").write_text(
        "Invented release-candidate stub for scan tooling self-test.\n",
        encoding="utf-8",
    )
    (candidate / "example.json").write_text(
        json.dumps(
            {"note": "invented example", "provenance": dict(ACCEPTED_PROVENANCE)}
        ),
        encoding="utf-8",
    )
    return candidate


def run_scan(module, tmp_path: Path, candidate_root: Path, extra=None, **kwargs):
    output_dir = tmp_path / "out"
    argv = [
        "--candidate",
        kwargs.get("candidate", "rc-test"),
        "--artifact-class",
        f"AC-4={candidate_root}",
        "--not-applicable",
        "AC-1=no native package produced by this self-test",
        "--not-applicable",
        "AC-2=no report artifact produced by this self-test",
        "--not-applicable",
        "AC-3=no installers produced (D-06 pending)",
        "--not-applicable",
        "AC-5=no separate example corpus in this self-test",
        "--not-applicable",
        "AC-6=no release notes produced by this self-test",
        "--security-profile",
        "skip",
        "--lint-mode",
        kwargs.get("lint_mode", "skip"),
        "--output-dir",
        str(output_dir),
        "--now-utc",
        FIXED_NOW,
        "--commit",
        FIXED_COMMIT,
        "--execute",
    ]
    if extra:
        argv.extend(extra)
    exit_code = module.main(argv)
    return exit_code, output_dir


def read_single_record(output_dir: Path) -> tuple[Path, dict]:
    records = sorted(output_dir.glob("SCAN_*.json"))
    assert len(records) == 1
    return records[0], json.loads(records[0].read_text(encoding="utf-8"))


def test_plan_mode_writes_nothing(tmp_path, capsys):
    module = load_module()
    exit_code = module.main(
        ["--candidate", "rc-plan", "--output-dir", str(tmp_path / "out")]
    )
    captured = capsys.readouterr()
    assert exit_code == 0
    assert "plan only" in captured.out
    assert "UNSIGNED" in captured.out
    assert not (tmp_path / "out").exists()


def test_record_filename_and_core_fields(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    exit_code, output_dir = run_scan(module, tmp_path, candidate_root)
    assert exit_code == 0

    record_path, record = read_single_record(output_dir)
    assert record_path.name == "SCAN_rc-test_20260710T120000Z_abcdef012345.json"
    assert record["artifact"] == "openpipestress.release_protected_content_scan_record"
    assert record["decision_basis"] == "DEC-058"
    assert record["candidate"] == "rc-test"
    assert record["git"]["commit_hash"] == FIXED_COMMIT
    assert record["clean_scan_is_clearance"] is False
    assert record["publication_gate"] == "blocked_pending_owner_sign_off"
    assert set(record["checks"]) == {
        "artifact_inventory_walk",
        "protected_content_lint",
        "provenance_manifest_check",
        "security_profile",
        "quarantine_hygiene",
    }
    assert record["known_limitations"]
    assert "not a legal clearance" in record["boundary_note"].lower()


def test_all_six_classes_recorded_never_silently_omitted(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    _, output_dir = run_scan(module, tmp_path, candidate_root)
    _, record = read_single_record(output_dir)

    assert sorted(record["artifact_classes"]) == [
        "AC-1",
        "AC-2",
        "AC-3",
        "AC-4",
        "AC-5",
        "AC-6",
    ]
    assert record["classes_scanned"] == ["AC-4"]
    assert record["classes_not_applicable"] == [
        "AC-1",
        "AC-2",
        "AC-3",
        "AC-5",
        "AC-6",
    ]
    for class_id in record["classes_not_applicable"]:
        entry = record["artifact_classes"][class_id]
        assert entry["status"] == "not_applicable"
        assert entry["reason"].strip()


def test_missing_class_specification_is_an_error(tmp_path, capsys):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    exit_code = module.main(
        [
            "--candidate",
            "rc-missing",
            "--artifact-class",
            f"AC-4={candidate_root}",
            "--output-dir",
            str(tmp_path / "out"),
            "--now-utc",
            FIXED_NOW,
            "--commit",
            FIXED_COMMIT,
            "--execute",
        ]
    )
    captured = capsys.readouterr()
    assert exit_code == 2
    assert "never silently omitted" in captured.err
    assert not (tmp_path / "out").exists()


def test_inventory_walk_records_checksums(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    _, output_dir = run_scan(module, tmp_path, candidate_root)
    _, record = read_single_record(output_dir)

    artifacts = record["artifact_classes"]["AC-4"]["artifacts"]
    assert [artifact["path"] for artifact in artifacts] == [
        "README.md",
        "example.json",
    ]
    for artifact in artifacts:
        assert len(artifact["sha256"]) == 64
        assert artifact["size_bytes"] > 0
        assert artifact["lintable"] is True
    assert record["checks"]["artifact_inventory_walk"]["artifact_count"] == 2


def test_record_is_deterministic_given_fixed_inputs(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    _, first_dir = run_scan(module, tmp_path / "first", candidate_root)
    _, second_dir = run_scan(module, tmp_path / "second", candidate_root)
    first_path, _ = read_single_record(first_dir)
    second_path, _ = read_single_record(second_dir)
    assert first_path.name == second_path.name
    assert first_path.read_bytes() == second_path.read_bytes()


def test_sign_off_block_is_explicitly_unsigned_and_pending(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    _, output_dir = run_scan(module, tmp_path, candidate_root)
    _, record = read_single_record(output_dir)

    sign_off = record["owner_sign_off"]
    assert sign_off["signed"] is False
    assert sign_off["status"] == "pending_owner_review"
    assert sign_off["signatory"] is None
    assert sign_off["signed_utc"] is None
    assert sign_off["acceptance_statement"] is None
    assert sign_off["per_finding_dispositions_recorded"] is False
    assert "DEC-027" in sign_off["signatory_role"]
    assert "Tooling never signs" in sign_off["note"]


def test_skipped_checks_fail_toward_owner_review(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    _, output_dir = run_scan(module, tmp_path, candidate_root)
    _, record = read_single_record(output_dir)

    assert record["checks"]["security_profile"]["status"] == "not_run"
    assert record["checks"]["protected_content_lint"]["status"] == "not_run"
    assert record["scan_disposition"] == "incomplete_checks_require_owner_review"


def test_provenance_violations_are_blocking_findings(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    bad_provenance = dict(ACCEPTED_PROVENANCE)
    bad_provenance["redistribution_status"] = "unknown"
    bad_provenance["source_license"] = "TBD"
    (candidate_root / "unreviewed.json").write_text(
        json.dumps({"provenance": bad_provenance}), encoding="utf-8"
    )

    _, output_dir = run_scan(module, tmp_path, candidate_root)
    _, record = read_single_record(output_dir)

    provenance_check = record["checks"]["provenance_manifest_check"]
    assert provenance_check["status"] == "findings_present"
    assert provenance_check["provenance_records_checked"] == 2
    assert any("TBD" in violation for violation in provenance_check["violations"])

    blocking = [
        finding
        for finding in record["findings"]
        if finding["check"] == "provenance_manifest_check"
    ]
    assert blocking
    assert all(finding["severity"] == "BLOCKING" for finding in blocking)
    assert all(
        finding["disposition"] == "pending_owner_review" for finding in blocking
    )
    assert record["scan_disposition"] == "findings_require_owner_review"
    assert record["blocking_finding_count"] >= 1


def test_quarantine_hygiene_flags_quarantine_paths(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    quarantined = candidate_root / "quarantine" / "protected-content"
    quarantined.mkdir(parents=True)
    (quarantined / "held.txt").write_text(
        "invented quarantined placeholder\n", encoding="utf-8"
    )

    _, output_dir = run_scan(module, tmp_path, candidate_root)
    _, record = read_single_record(output_dir)

    hygiene = record["checks"]["quarantine_hygiene"]
    assert hygiene["status"] == "findings_present"
    assert any(
        "quarantine/protected-content" in offender
        for offender in hygiene["artifacts_under_quarantine_path"]
    )
    assert record["scan_disposition"] == "findings_require_owner_review"


def test_lint_findings_are_normalized_with_pending_disposition(tmp_path, monkeypatch):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)

    def fake_lint(paths, root):
        assert paths, "lintable members should be passed to the engine"
        return {
            "status": "findings_present",
            "command": "cargo run --quiet ...",
            "engine_summary": {"finding_count": 1},
            "finding_count": 1,
            "findings": [
                {
                    "finding_id": "RPLC-F0001",
                    "code": "UNKNOWN_PROVENANCE_REVIEW_REQUIRED",
                    "class": "PROVENANCE_WARNING",
                    "severity": "WARNING",
                    "path": str(paths[0]),
                    "line": 2,
                    "column": 1,
                    "matched_policy": "OPS-K-IP-3",
                    "excerpt": "allowable stress",
                    "message": "Standards-table signature (stubbed).",
                    "remediation": "Route to human IP review.",
                    "review_route": "HUMAN_IP_REVIEW",
                    "disposition": "PENDING",
                }
            ],
            "note": None,
        }

    monkeypatch.setattr(module, "run_engine_lint", fake_lint)
    _, output_dir = run_scan(module, tmp_path, candidate_root, lint_mode="engine")
    _, record = read_single_record(output_dir)

    lint_findings = [
        finding
        for finding in record["findings"]
        if finding["check"] == "protected_content_lint"
    ]
    assert len(lint_findings) == 1
    finding = lint_findings[0]
    assert finding["code"] == "UNKNOWN_PROVENANCE_REVIEW_REQUIRED"
    assert finding["review_route"] == "HUMAN_IP_REVIEW"
    assert finding["matched_policy"] == "OPS-K-IP-3"
    assert finding["disposition"] == "pending_owner_review"
    assert record["scan_disposition"] == "findings_require_owner_review"


def test_record_makes_no_release_readiness_claim(tmp_path):
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    _, output_dir = run_scan(module, tmp_path, candidate_root)
    record_path, record = read_single_record(output_dir)

    serialized = record_path.read_text(encoding="utf-8").lower()
    assert "release ready" not in serialized
    assert "release-ready" not in serialized
    assert "certified" not in serialized
    assert record["scan_disposition"] != "pass"
    assert "clearance" not in record["scan_disposition"]


@pytest.mark.skipif(
    shutil.which("cargo") is None, reason="cargo unavailable; engine CLI not built"
)
def test_engine_lint_integration_flags_invented_lookalike(tmp_path):
    """End-to-end check (b): the DEL-08-05 engine CLI flags an INVENTED
    standards-table lookalike (designator name plus synthetic values only)
    and the record fails toward owner review."""
    module = load_module()
    candidate_root = make_candidate_tree(tmp_path)
    (candidate_root / "lookalike.txt").write_text(
        "Invented lookalike fixture: no standards values copied.\n"
        "ASME code table heading (invented placeholder)\n"
        "101.1 202.2 303.3 404.4\n"
        "111.1 222.2 333.3 444.4\n"
        "121.1 242.2 363.3 484.4\n",
        encoding="utf-8",
    )

    exit_code, output_dir = run_scan(
        module, tmp_path, candidate_root, lint_mode="engine"
    )
    assert exit_code == 0
    _, record = read_single_record(output_dir)

    lint_check = record["checks"]["protected_content_lint"]
    assert lint_check["status"] == "findings_present"
    signature_findings = [
        finding
        for finding in lint_check["findings"]
        if finding["matched_policy"] == "OPS-K-IP-3"
    ]
    assert len(signature_findings) == 1
    assert signature_findings[0]["code"] == "UNKNOWN_PROVENANCE_REVIEW_REQUIRED"
    assert signature_findings[0]["review_route"] == "HUMAN_IP_REVIEW"
    assert signature_findings[0]["path"].endswith("lookalike.txt")
    assert record["scan_disposition"] == "findings_require_owner_review"


@pytest.mark.skipif(
    shutil.which("cargo") is None, reason="cargo unavailable; engine CLI not built"
)
def test_engine_cli_reports_clean_content_without_findings(tmp_path):
    """Negative path for check (b): legitimate invented content with no
    standards-table signature produces no engine findings."""
    clean = tmp_path / "clean.txt"
    clean.write_text(
        "Rule pack id: invented-demo; version: 0.1.0; human review required.\n",
        encoding="utf-8",
    )
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "--quiet",
            "--manifest-path",
            "core/reporting/protected_content_linter/Cargo.toml",
            "--bin",
            "protected_content_lint_cli",
            "--",
            "--provenance-mode",
            "external",
            str(clean),
        ],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    assert completed.returncode == 0
    payload = json.loads(completed.stdout)
    assert payload["summary"]["finding_count"] == 0
    assert payload["summary"]["clean_scan_is_clearance"] is False
