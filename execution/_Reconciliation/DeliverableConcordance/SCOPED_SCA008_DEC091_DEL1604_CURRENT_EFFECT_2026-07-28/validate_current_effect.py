#!/usr/bin/env python3
"""Validate the bounded D-60 current-effect reconciliation derivative."""

from __future__ import annotations

import csv
import hashlib
import json
import subprocess
from pathlib import Path


SOURCE_BASIS = "21e8e54e1f5648b7d3db29228271aaa8c7d8904f"
ACTIVATION_APPLICATION = "eba66bfe6ef00aa07d9cfac75eee79965218fa43"
ACTIVATION_SHA = "8728813764376e7c19c0760178b3b052eed2c819b5f7ccd24dea2873dddccad9"
HISTORICAL_TREE = "79914bd8b4a9bc183994e5a63746ef0f316ed3a2"
HISTORICAL_TREE_LIST_SHA = (
    "c547becd5eecae05cd6a2a5c64708223c136aa3c147ea55399f6b8cb8ce64f02"
)
STATUS_SHA = "39af5d4b4ebfcf0c0c46b122f29d33f33b0774094be317201ac8a8269572b366"


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    digest.update(path.read_bytes())
    return digest.hexdigest()


def csv_rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


def git(repo: Path, *args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=repo, text=True).strip()


def main() -> int:
    run_root = Path(__file__).resolve().parent
    repo = run_root.parents[5]
    project = repo / "projects/chirality-piping"
    historical_rel = (
        "projects/chirality-piping/execution/_Reconciliation/"
        "DeliverableConcordance/DELIVERABLE_CONCORDANCE_2026-07-11_1305"
    )
    historical = repo / historical_rel
    status = (
        project
        / "execution/PKG-16_Model Operation and Agent Proposal Framework/"
        "1_Working/DEL-16-04_Agent rationale and professional-boundary controls/"
        "_STATUS.md"
    )
    activation = (
        project
        / "execution/_Coordination/_DECISIONS/"
        "D-60_sca008_current_effect_reconciliation_activation.md"
    )
    decision_register = (
        project / "execution/_Coordination/_DECISIONS/_REGISTER.md"
    )

    authority = csv_rows(run_root / "AUTHORITY_CURRENT_EFFECT.csv")
    claims = csv_rows(
        run_root
        / "WAVES/SCA008/CLAIM_CONCORDANCE_DEL-16-04_CURRENT_EFFECT.csv"
    )
    remaining = csv_rows(
        run_root
        / "BACKCHECK/SCA008_CURRENT_EFFECT_2026-07-28/"
        "REMAINING_WORK_CENSUS.csv"
    )
    evidence = csv_rows(
        run_root
        / "BACKCHECK/SCA008_CURRENT_EFFECT_2026-07-28/DETAILED_EVIDENCE.csv"
    )

    tree = git(repo, "rev-parse", f"HEAD:{historical_rel}")
    tree_rows_bytes = subprocess.check_output(
        ["git", "ls-tree", "-r", "HEAD", "--", historical_rel], cwd=repo
    )
    tree_list_sha = hashlib.sha256(tree_rows_bytes).hexdigest()
    tracked_count = len(tree_rows_bytes.splitlines())
    status_text = status.read_text(encoding="utf-8")
    remaining_block = status_text.split("## Remaining\n", 1)[1].split(
        "\n## History", 1
    )[0]
    status_bullets = [
        line[2:] for line in remaining_block.splitlines() if line.startswith("- ")
    ]

    changed = git(repo, "status", "--short").splitlines()
    allowed_prefix = (
        "?? projects/chirality-piping/execution/_Reconciliation/"
        "DeliverableConcordance/"
        "SCOPED_SCA008_DEC091_DEL1604_CURRENT_EFFECT_2026-07-28/"
    )
    forbidden_changes = [line for line in changed if not line.startswith(allowed_prefix)]

    claim_by_id = {row["ClaimID"]: row for row in claims}
    authority_by_id = {row["AuthorityID"]: row for row in authority}
    checks = {
        "head_matches_source_basis": git(repo, "rev-parse", "HEAD") == SOURCE_BASIS,
        "activation_application_is_ancestor": subprocess.call(
            [
                "git",
                "merge-base",
                "--is-ancestor",
                ACTIVATION_APPLICATION,
                "HEAD",
            ],
            cwd=repo,
        )
        == 0,
        "activation_sha256": sha256(activation),
        "activation_register_row_count": sum(
            1
            for line in decision_register.read_text(encoding="utf-8").splitlines()
            if line.startswith("| D-60 |")
            and "DEC-063/DEC-091" in line
            and "REQ-009 and DECL-005" in line
        ),
        "authority_count": len(authority),
        "authority_ids": sorted(authority_by_id),
        "dec063_disposition": authority_by_id.get("DEC-063", {}).get(
            "CurrentRelianceStatus"
        ),
        "dec091_disposition": authority_by_id.get("DEC-091", {}).get(
            "CurrentRelianceStatus"
        ),
        "claim_count": len(claims),
        "claim_ids": sorted(claim_by_id),
        "req009_tuple": [
            claim_by_id.get("DEL-16-04-REQ-009", {}).get("Disposition"),
            claim_by_id.get("DEL-16-04-REQ-009", {}).get("Confidence"),
            claim_by_id.get("DEL-16-04-REQ-009", {}).get("AuthorityNeeded"),
            claim_by_id.get("DEL-16-04-REQ-009", {}).get(
                "SelectableUnderCurrentLoop"
            ),
        ],
        "decl005_tuple": [
            claim_by_id.get("DEL-16-04-DECL-005", {}).get("Disposition"),
            claim_by_id.get("DEL-16-04-DECL-005", {}).get("Confidence"),
            claim_by_id.get("DEL-16-04-DECL-005", {}).get("AuthorityNeeded"),
            claim_by_id.get("DEL-16-04-DECL-005", {}).get(
                "SelectableUnderCurrentLoop"
            ),
        ],
        "source_states": sorted({row["SourceState"] for row in claims}),
        "remaining_row_count": len(remaining),
        "remaining_ordinals": [row["RemainingOrdinal"] for row in remaining],
        "remaining_all_nonselectable": all(
            row["SelectableUnderCurrentLoop"] == "NO" for row in remaining
        ),
        "remaining_text_matches_status": [row["RemainingText"] for row in remaining]
        == status_bullets,
        "status_sha256": sha256(status),
        "status_lifecycle_in_progress": (
            "**Current State:** IN_PROGRESS" in status_text
        ),
        "historical_tree": tree,
        "historical_tracked_count": tracked_count,
        "historical_tree_list_sha256": tree_list_sha,
        "evidence_count": len(evidence),
        "evidence_all_pass": all(row["Result"] == "PASS" for row in evidence),
        "forbidden_worktree_changes": forbidden_changes,
        "reconciliation_pointer_sha256": sha256(
            project / "execution/_Reconciliation/_LATEST.md"
        ),
    }

    expected = [
        checks["head_matches_source_basis"],
        checks["activation_application_is_ancestor"],
        checks["activation_sha256"] == ACTIVATION_SHA,
        checks["activation_register_row_count"] == 1,
        checks["authority_count"] == 2,
        checks["authority_ids"] == ["DEC-063", "DEC-091"],
        checks["dec063_disposition"] == "NOT_CURRENT_RELIANCE",
        checks["dec091_disposition"] == "GOVERNING_CURRENT_EFFECT",
        checks["claim_count"] == 2,
        checks["claim_ids"]
        == ["DEL-16-04-DECL-005", "DEL-16-04-REQ-009"],
        checks["req009_tuple"]
        == ["PARTIALLY_IMPLEMENTED", "MEDIUM", "OWNER", "NO"],
        checks["decl005_tuple"] == ["ALIGNED", "HIGH", "NO", "NO"],
        checks["source_states"] == [SOURCE_BASIS],
        checks["remaining_row_count"] == 3,
        checks["remaining_ordinals"] == ["1", "2", "3"],
        checks["remaining_all_nonselectable"],
        checks["remaining_text_matches_status"],
        checks["status_sha256"] == STATUS_SHA,
        checks["status_lifecycle_in_progress"],
        checks["historical_tree"] == HISTORICAL_TREE,
        checks["historical_tracked_count"] == 294,
        checks["historical_tree_list_sha256"] == HISTORICAL_TREE_LIST_SHA,
        checks["evidence_count"] == 6,
        checks["evidence_all_pass"],
        not checks["forbidden_worktree_changes"],
        checks["reconciliation_pointer_sha256"]
        == "145edf07b189002df4b78a7e62e5df96c3eb392723c64f8d49fda16e726b5ea7",
    ]
    result = {
        "schema": "piping.d60.current_effect_reconciliation.validation.v1",
        "source_basis": SOURCE_BASIS,
        "status": "PASS" if all(expected) else "BLOCK",
        "closure": (
            "CURRENT_EFFECT_RECONCILED / CLOSED_WITH_RELIANCE_HOLD"
            if all(expected)
            else "HOLD"
        ),
        "checks": checks,
    }
    output = run_root / "VALIDATION_RESULT.json"
    output.write_text(
        json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    return 0 if all(expected) else 1


if __name__ == "__main__":
    raise SystemExit(main())
