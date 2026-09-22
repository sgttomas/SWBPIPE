#!/usr/bin/env python3
"""Verify the bounded D-74 integration overlay against its sealed manifest."""

from __future__ import annotations

import csv
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path


HERE = Path(__file__).resolve().parent
REPO = HERE.parents[7]
MANIFEST_PATH = HERE / "MANIFEST.json"
PIPING = "projects/chirality-piping/"
R5 = (
    PIPING
    + "execution/_Reconciliation/DeliverableConcordance/"
    + "RECON_2026-09-21_WHOLE_CORPUS/R5_RECORD_CONTINUATION_2026-09-22/"
)
BASELINE = "1b5adbf50142a4c01c454c62a31dfcdc60da1894"
CANDIDATE = "a89b5ddecfb6d1ea8cca1b68d4895ab511e5c370"
D74 = "67e4738b2f276e3623cb16be6f8c2d7803f51521"
MERGE_BASE = "0fb42b36df5c93c34c02e209670f3cede937ce84"

OVERLAPS = {
    PIPING + "CONTRIBUTING.md",
    PIPING + "docs/IP_AND_DATA_BOUNDARY.md",
    PIPING + "docs/PROFESSIONAL_BOUNDARY.md",
    PIPING + "docs/README.md",
    PIPING + "docs/contributor_guide/index.md",
    PIPING + "docs/report_notice_template.md",
    PIPING + "docs/user_guide/index.md",
    PIPING + "governance/CONTRIBUTION_REVIEW_CHECKLIST.md",
    PIPING + "governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md",
}
AMENDED_AUTHORITY_CARRIERS = {
    PIPING + "docs/DIRECTIVE.md",
    PIPING + "docs/AGENTIC_DEVELOPMENT_WORKFLOW.md",
    PIPING + "governance/MAINTAINERS.md",
}
CURRENT_CARRIER_CORRECTIONS = {PIPING + "docs/BUILD_AND_RELEASE.md"}
OVERLAY_PREFIX = R5 + "D74_INTEGRATION_2026-09-22/"
OVERLAY_FILES = {
    OVERLAY_PREFIX + "MANIFEST.json",
    OVERLAY_PREFIX + "HANDOFF.md",
    OVERLAY_PREFIX + "verify_integration.py",
}
MIT_CARRIERS = {
    PIPING + "CONTRIBUTING.md",
    PIPING + "docs/README.md",
    PIPING + "docs/DIRECTIVE.md",
    PIPING + "docs/CONTRACT.md",
    PIPING + "docs/PROFESSIONAL_BOUNDARY.md",
    PIPING + "docs/IP_AND_DATA_BOUNDARY.md",
    PIPING + "docs/AGENTIC_DEVELOPMENT_WORKFLOW.md",
    PIPING + "docs/report_notice_template.md",
    PIPING + "docs/user_guide/index.md",
    PIPING + "docs/contributor_guide/index.md",
    PIPING + "governance/CONTRIBUTION_REVIEW_CHECKLIST.md",
    PIPING + "governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md",
    PIPING + "governance/MAINTAINERS.md",
    PIPING + "apps/desktop/src/features/report-lint/ReportLintPanel.tsx",
    PIPING + "apps/desktop/src/features/stress-neutral/StressNeutralExportPanel.tsx",
}
CONFLICT_MARKER = re.compile(rb"^(?:<<<<<<< |=======|>>>>>>> )(?:.*)?$", re.MULTILINE)


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def git_bytes(spec: str) -> bytes:
    return subprocess.run(
        ["git", "show", spec], cwd=REPO, check=True, stdout=subprocess.PIPE
    ).stdout


def fail(message: str) -> None:
    raise AssertionError(message)


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def working_bytes(path: str) -> bytes:
    target = REPO / path
    require(target.is_file(), f"missing worktree file: {path}")
    return target.read_bytes()


def manifest() -> dict:
    return json.loads(MANIFEST_PATH.read_text(encoding="utf-8"))


def accepts_bound_postimage(expected_hash: str, data: bytes) -> bool:
    """Single byte acceptance predicate shared by normal and negative checks."""
    return sha256(data) == expected_hash


def check() -> list[str]:
    record = manifest()
    require(record["schema"] == "d74-integration-manifest-v1", "manifest schema")
    require(record["baseline_commit"] == BASELINE, "wrong reconciliation baseline")
    require(record["candidate_commit"] == CANDIDATE, "wrong pre-D-74 candidate")
    require(record["d74_commit"] == D74, "wrong D-74 source commit")
    require(record["merge_base"] == MERGE_BASE, "wrong merge base")
    require(
        subprocess.check_output(
            ["git", "merge-base", CANDIDATE, D74], cwd=REPO, text=True
        ).strip()
        == MERGE_BASE,
        "candidate/D-74 ancestry changed",
    )

    rows = record["files"]
    paths = set(rows)
    overlap_paths = set(record["overlap_paths"])
    incoming_paths = set(record["incoming_only_paths"])
    require(overlap_paths == OVERLAPS, "overlap path set drift")
    require(incoming_paths == paths - OVERLAPS, "incoming-only path set drift")
    require(not overlap_paths & incoming_paths, "overlap/incoming path collision")
    require(set(record["amended_authority_carriers"]) == AMENDED_AUTHORITY_CARRIERS,
            "amended authority carrier set drift")
    carrier_repairs = record["current_carrier_corrections"]
    require(set(carrier_repairs) == CURRENT_CARRIER_CORRECTIONS,
            "additional current-carrier correction set drift")
    source_diff = set(
        subprocess.check_output(
            ["git", "diff", "--name-only", MERGE_BASE, D74, "--", PIPING],
            cwd=REPO,
            text=True,
        ).splitlines()
    )
    require(paths == source_diff, "D-74 affected path set differs from sealed manifest")
    require(len(paths) == 20 and len(overlap_paths) == 9 and len(incoming_paths) == 11,
            "unexpected D-74 path counts")

    original_r5_paths = set(
        subprocess.check_output(
            ["git", "ls-tree", "-r", "--name-only", CANDIDATE, "--", R5],
            cwd=REPO,
            text=True,
        ).splitlines()
    )
    preserved_r5 = record["preserved_r5_files"]
    require(set(preserved_r5) == original_r5_paths,
            "manifest does not preserve the complete a89 R5 subtree")
    current_r5_paths = {
        path.relative_to(REPO).as_posix()
        for path in (REPO / R5).rglob("*")
        if path.is_file()
    }
    require(current_r5_paths == original_r5_paths | OVERLAY_FILES,
            "R5 subtree is not append-only relative to a89")

    bindings_path = REPO / (R5 + "SOURCE_BINDINGS.csv")
    with bindings_path.open(encoding="utf-8-sig", newline="") as stream:
        bindings = {row["SourcePath"]: row for row in csv.DictReader(stream)}
    for path, item in rows.items():
        current = working_bytes(path)
        require(accepts_bound_postimage(item["postimage_sha256"], current),
                f"postimage hash mismatch: {path}")
        require(
            sha256(git_bytes(f"{D74}:{path}")) == item["d74_source_sha256"],
            f"D-74 source hash mismatch: {path}",
        )
        require(not CONFLICT_MARKER.search(current), f"merge marker found: {path}")

        if path in OVERLAPS:
            binding = bindings.get(path)
            require(binding is not None, f"overlap missing from original SOURCE_BINDINGS: {path}")
            require(binding["Baseline"] == BASELINE, f"unexpected binding baseline: {path}")
            source_hash = sha256(git_bytes(f"{CANDIDATE}:{path}"))
            require(source_hash == binding["AfterSHA256"], f"a89/source binding mismatch: {path}")
            require(source_hash == item["candidate_after_sha256"], f"a89 manifest mismatch: {path}")
        else:
            require(item["candidate_after_sha256"] is None, f"unexpected candidate binding: {path}")
            if path not in AMENDED_AUTHORITY_CARRIERS:
                require(current == git_bytes(f"{D74}:{path}"), f"D-74 source not preserved: {path}")

    for path, expected in preserved_r5.items():
        current = working_bytes(path)
        require(sha256(current) == expected, f"historical R5 artifact hash mismatch: {path}")
        require(current == git_bytes(f"{CANDIDATE}:{path}"), f"historical R5 artifact changed since a89: {path}")
    result = json.loads(working_bytes(R5 + "CHECK_RESULT.json"))
    require(result["status"] == "PASS", "historical R5 result was not PASS")
    require(result["baseline"] == BASELINE, "historical R5 result baseline mismatch")

    for path, item in carrier_repairs.items():
        current = working_bytes(path)
        require(path not in paths, f"current-carrier correction misclassified as D-74 source change: {path}")
        require(accepts_bound_postimage(item["postimage_sha256"], current),
                f"current-carrier postimage hash mismatch: {path}")
        require(not CONFLICT_MARKER.search(current), f"merge marker found in current-carrier correction: {path}")
        candidate_bytes = git_bytes(f"{CANDIDATE}:{path}")
        require(sha256(candidate_bytes) == item["candidate_preimage_sha256"],
                f"current-carrier a89 preimage hash mismatch: {path}")
        main_bytes = git_bytes(f"{D74}:{path}")
        require(sha256(main_bytes) == item["main_source_sha256"],
                f"current-carrier main source hash mismatch: {path}")
        require(main_bytes == git_bytes(f"{MERGE_BASE}:{path}"),
                f"current-carrier correction was attributed to D-74 source: {path}")
        require(path not in bindings,
                f"current-carrier correction unexpectedly appears in original R5 SOURCE_BINDINGS: {path}")

    # Negative probe uses a mutated in-memory copy only; no repository file is written.
    probe_path = sorted(OVERLAPS)[0]
    pristine = working_bytes(probe_path)
    tampered = bytes([pristine[0] ^ 1]) + pristine[1:]
    rejected = not accepts_bound_postimage(rows[probe_path]["postimage_sha256"], tampered)
    require(rejected, "negative probe failed to reject tampered overlap bytes")

    project_license = working_bytes(PIPING + "LICENSE.md")
    root_license = working_bytes("LICENSE.md")
    require(project_license == root_license, "Piping MIT license differs from repository MIT license")
    ruling = working_bytes(PIPING + "execution/_Coordination/_DECISIONS/D-74_RULING_2026-09-22.md").decode()
    require('selected_option: "Project license MIT"' in ruling, "D-74 ruling does not select MIT")
    require("We're going MIT, you can make the necessary changes now." in ruling,
            "D-74 owner's recorded direction missing")
    for path in sorted(MIT_CARRIERS):
        require("MIT" in working_bytes(path).decode("utf-8"), f"MIT absent from live carrier: {path}")

    ledger_path = REPO / (PIPING + "docs/_Registers/ScopeLedger.csv")
    with ledger_path.open(encoding="utf-8-sig", newline="") as stream:
        scope_rows = {row["ScopeItemID"]: row for row in csv.DictReader(stream)}
    for scope_id in ("SOW-001", "SOW-048"):
        row = scope_rows[scope_id]
        require("MIT" in row["Notes"] and "D-74" in row["Notes"], f"{scope_id} lacks D-74/MIT note")
    decomp = working_bytes(PIPING + "execution/_Decomposition/SOFTWARE_DECOMP.md").decode()
    require("|DEC-112|" in decomp and "D-74" in decomp and "`MIT`" in decomp,
            "decomposition lacks DEC-112/D-74/MIT")
    decision_register = working_bytes(PIPING + "execution/_Coordination/_DECISIONS/_REGISTER.md").decode()
    require("| D-74 |" in decision_register and "RULED (2026-09-22): MIT" in decision_register,
            "D-74 decision register row missing or not ruled")

    directive = working_bytes(PIPING + "docs/DIRECTIVE.md").decode()
    maintainers = working_bytes(PIPING + "governance/MAINTAINERS.md").decode()
    workflow = working_bytes(PIPING + "docs/AGENTIC_DEVELOPMENT_WORKFLOW.md").decode()
    authority_text = (directive + "\n" + maintainers + "\n" + workflow).lower()
    for phrase in (
        "quorum one",
        "sole developer, maintainer, and release authority",
        "external contribution intake remains closed",
        "dec-027",
        "dec-079",
    ):
        require(phrase in authority_text, f"DEC-027/079 authority carrier missing: {phrase}")
    require("legal advice" in authority_text, "owner/legal-advice activation condition missing")
    require("dec-025/059/093" in workflow.lower(),
            "agentic workflow misses its DEC-025/059/093 CI basis")
    require("dec-060" in workflow.lower(), "agentic workflow misses DEC-060 basis")
    workflow_normalized = " ".join(workflow.lower().split())
    require("no numeric floor adopted" in workflow_normalized,
            "agentic workflow does not preserve DEC-060 floor boundary")
    authority = decomp.lower()
    require("|dec-027|" in authority and "|dec-079|" in authority,
            "accepted DEC-027/079 source rows missing")
    require("external contributions are not accepted at this time" in authority,
            "DEC-027 closed intake basis missing")
    require("legal advice obtained by the owner" in authority,
            "DEC-079 legal-instrument activation condition missing")

    build_release = working_bytes(PIPING + "docs/BUILD_AND_RELEASE.md").decode().lower()
    build_release_normalized = " ".join(build_release.split())
    for phrase in (
        "dec-027 records the sole human project authority as sole maintainer and",
        "release authority with quorum one",
        "dec-057",
        "d-06b",
        "dec-059",
        "dec-093",
        "coverage, performance, tolerance, and permitted-variance thresholds",
    ):
        require(phrase in build_release_normalized, f"BUILD_AND_RELEASE authority correction missing: {phrase}")
    require("hosted ci is deferred (dec-025; re-decided at `d-05b` with d-06)" not in build_release_normalized,
            "BUILD_AND_RELEASE retains superseded D-05b-only CI statement")
    require("dec-059 conditionally selects public-export ci" in build_release_normalized,
            "BUILD_AND_RELEASE misses DEC-059 conditional public-export CI")
    require("dec-093 provides a bounded surface-4 ci-evidence alternative" in build_release_normalized,
            "BUILD_AND_RELEASE misses DEC-093 surface-4 evidence alternative")

    latest = working_bytes(PIPING + "execution/_DAG/_LATEST.md").decode()
    approval = working_bytes(PIPING + "execution/_DAG/DAG-011/APPROVAL_RECORD.md").decode()
    require("DAG-011" in latest and "approved_active_graph_authority" in latest,
            "current DAG pointer is not DAG-011 authority")
    require("Group-3" in approval and "DAG-011" in approval,
            "DAG-011 approval record does not bind owner adoption")

    census = REPO / (R5 + "REMAINING_WORK_CENSUS.csv")
    with census.open(encoding="utf-8-sig", newline="") as stream:
        residuals = list(csv.DictReader(stream))
    del01 = [row for row in residuals if row["DeliverableID"] == "DEL-01-01"]
    del03 = [row for row in residuals if row["DeliverableID"] == "DEL-01-03"]
    require(len(del01) == 1 and del01[0]["Lifecycle"] == "ISSUED" and del01[0]["Item"] == "NONE",
            "DEL-01-01 historical issued state/residual census changed")
    require(len(del03) == 3 and {row["Item"] for row in del03} == {"1", "2", "3"},
            "DEL-01-03 formal downstream residual census changed")
    item1 = next(row["RecordedText"].lower() for row in del03 if row["Item"] == "1")
    require("intake remains closed" in item1 and "dec-027" in item1 and "dec-079" in item1,
            "DEL-01-03 closed-intake residual missing")
    require(any("pre-release legal review" in row["RecordedText"].lower() for row in del03),
            "DEL-01-03 legal review residual missing")
    require(any("issue_template" in row["RecordedText"].lower() for row in del03),
            "DEL-01-03 public issue-template residual missing")

    return [
        f"PASS: {len(OVERLAPS)} overlap postimages and their a89 SOURCE_BINDINGS verified",
        f"PASS: {len(incoming_paths) - len(AMENDED_AUTHORITY_CARRIERS)} D-74-only postimages preserved exactly; three authority carriers amended and hash-bound",
        "PASS: BUILD_AND_RELEASE.md correction is bound from the a89 preimage; DEC-027/057, DEC-059/093, D-06b, and open thresholds checked",
        f"PASS: {len(paths)} affected paths contain no merge-conflict markers",
        f"PASS: {len(original_r5_paths)} original R5 files match a89 and are byte-preserved; historical CHECK_RESULT remains PASS",
        "PASS: MIT ruling, current license carriers, DEC-027/079, and DAG-011 pointers verified",
        "PASS: DEL-01-01 issued state and DEL-01-03 open downstream scope remain recorded",
        f"PASS: in-memory tamper of {probe_path} was rejected",
    ]


if __name__ == "__main__":
    try:
        for line in check():
            print(line)
    except Exception as error:  # surfaced as a concise failing verifier result
        print(f"FAIL: {error}", file=sys.stderr)
        raise
