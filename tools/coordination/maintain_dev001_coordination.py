#!/usr/bin/env python3
"""Maintain DEV-001 implementation evidence and blocker queue derivatives.

Purpose:
  Validate DEV-001 implementation evidence and generate deterministic blocker
  queue CSV/Markdown mirrors from the approved DAG files.

Inputs:
  - execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv
  - execution/_DAG/<dag>/DeliverableNodes.csv
  - execution/_DAG/<dag>/DependencyEdges.csv

Outputs with --write:
  - execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv
  - execution/_Coordination/DEV-001_BLOCKER_QUEUE.md
  - execution/_DAG/<dag>/DEV-001_BLOCKER_QUEUE.csv
  - execution/_DAG/<dag>/DEV-001_BLOCKER_QUEUE.md

Examples:
  python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --check
  python3 tools/coordination/maintain_dev001_coordination.py --dag DAG-005 --write
"""

from __future__ import annotations

import argparse
import csv
import io
import subprocess
import sys
from collections import Counter, defaultdict
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


ROOT = Path(__file__).resolve().parents[2]
IMPLEMENTATION_EVIDENCE = Path("execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv")
COORD_QUEUE_CSV = Path("execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv")
COORD_QUEUE_MD = Path("execution/_Coordination/DEV-001_BLOCKER_QUEUE.md")
QUEUE_HEADER = [
    "DeliverableID",
    "PackageID",
    "DeliverableName",
    "LifecycleState",
    "ImplementationEvidenceState",
    "EvidenceCommit",
    "ActiveUpstreamCount",
    "SatisfiedUpstreamCount",
    "BlockingUpstreamCount",
    "BlockerState",
    "BlockingUpstreamDeliverables",
    "BlockingEdgeIDs",
]
EVIDENCE_HEADER = [
    "DeliverableID",
    "PackageID",
    "EvidenceState",
    "EvidenceKind",
    "Commit",
    "CommitSubject",
    "CommittedDate",
    "HandoffCommit",
    "Notes",
]


@dataclass(frozen=True)
class GeneratedQueues:
    csv_text: str
    md_text: str
    rows: list[dict[str, str]]
    active_edge_count: int
    candidate_edge_count: int
    evidence_records: int
    committed_evidence_records: int
    semantic_ready_count: int
    architecture_basis_edge_count: int


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    try:
        with path.open(newline="", encoding="utf-8") as handle:
            reader = csv.DictReader(handle)
            rows = list(reader)
    except FileNotFoundError as exc:
        raise ValueError(f"missing CSV file: {path}") from exc
    if reader.fieldnames is None:
        raise ValueError(f"empty CSV file: {path}")
    return list(reader.fieldnames), rows


def latest_dag(root: Path) -> str:
    latest = root / "execution/_DAG/_LATEST.md"
    for line in latest.read_text(encoding="utf-8").splitlines():
        if line.startswith("- Approved graph authority:"):
            value = line.split(":", maxsplit=1)[1].strip().strip("`").strip("/")
            return Path(value).name
    raise ValueError(f"approved graph authority not found in {latest}")


def dag_paths(root: Path, dag: str) -> tuple[Path, Path, Path, Path]:
    dag_root = root / "execution" / "_DAG" / dag
    return (
        dag_root / "DeliverableNodes.csv",
        dag_root / "DependencyEdges.csv",
        dag_root / "DEV-001_BLOCKER_QUEUE.csv",
        dag_root / "DEV-001_BLOCKER_QUEUE.md",
    )


def validate_headers(label: str, header: list[str], expected: list[str]) -> list[str]:
    if header != expected:
        return [f"{label} header mismatch: expected {expected}, found {header}"]
    return []


def git_commit_metadata(root: Path, commit: str) -> tuple[str, str] | None:
    completed = subprocess.run(
        ["git", "show", "-s", "--format=%cs|%s", commit],
        cwd=root,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    if completed.returncode != 0:
        return None
    date, subject = completed.stdout.rstrip("\n").split("|", maxsplit=1)
    return date, subject


def validate_evidence(
    *,
    root: Path,
    evidence_header: list[str],
    evidence_rows: list[dict[str, str]],
    node_ids: set[str],
    validate_git: bool,
) -> list[str]:
    findings = validate_headers("implementation evidence", evidence_header, EVIDENCE_HEADER)
    seen: set[str] = set()
    for row in evidence_rows:
        deliverable_id = row.get("DeliverableID", "")
        if deliverable_id in seen:
            findings.append(f"duplicate evidence DeliverableID: {deliverable_id}")
        seen.add(deliverable_id)
        if deliverable_id not in node_ids:
            findings.append(f"evidence deliverable not in DeliverableNodes.csv: {deliverable_id}")
        if row.get("PackageID") != "PKG-00":
            if row.get("EvidenceState") != "COMMITTED":
                findings.append(f"{deliverable_id} EvidenceState is not COMMITTED")
            if not row.get("EvidenceKind"):
                findings.append(f"{deliverable_id} missing EvidenceKind")
        commit = row.get("Commit", "")
        if validate_git and commit:
            metadata = git_commit_metadata(root, commit)
            if metadata is None:
                findings.append(f"{deliverable_id} commit not found in git: {commit}")
                continue
            date, subject = metadata
            if row.get("CommittedDate") != date:
                findings.append(
                    f"{deliverable_id} committed date mismatch: {row.get('CommittedDate')} != {date}"
                )
            if row.get("CommitSubject") != subject:
                findings.append(
                    f"{deliverable_id} commit subject mismatch: {row.get('CommitSubject')} != {subject}"
                )
    return findings


def queue_rows(
    *,
    nodes: list[dict[str, str]],
    edges: list[dict[str, str]],
    evidence_rows: list[dict[str, str]],
    lifecycle_overrides: dict[str, str] | None = None,
) -> tuple[list[dict[str, str]], int, int, int]:
    evidence = {row["DeliverableID"]: row for row in evidence_rows}
    active_edges = [row for row in edges if row.get("Status") == "ACTIVE"]
    candidate_edge_count = sum(1 for row in edges if row.get("Status") == "CANDIDATE")
    upstreams: dict[str, list[dict[str, str]]] = defaultdict(list)
    architecture_basis_edge_count = 0
    for edge in active_edges:
        upstreams[edge.get("FromDeliverableID", "")].append(edge)
        if edge.get("TargetPackageID", "") == "PKG-00":
            architecture_basis_edge_count += 1

    rows: list[dict[str, str]] = []
    lifecycle_overrides = lifecycle_overrides or {}
    for node in nodes:
        deliverable_id = node["DeliverableID"]
        package_id = node["PackageID"]
        active_upstreams = upstreams.get(deliverable_id, [])
        satisfied: list[dict[str, str]] = []
        blocking: list[dict[str, str]] = []
        for edge in active_upstreams:
            target_package = edge.get("TargetPackageID", "")
            target_deliverable = edge.get("TargetDeliverableID", "")
            if target_package == "PKG-00":
                satisfied.append(edge)
                continue
            upstream_evidence = evidence.get(target_deliverable)
            if upstream_evidence and upstream_evidence.get("EvidenceState") == "COMMITTED":
                satisfied.append(edge)
            else:
                blocking.append(edge)

        own_evidence = evidence.get(deliverable_id)
        if package_id == "PKG-00":
            evidence_state = "ARCHITECTURE_BASELINE"
            evidence_commit = ""
        elif own_evidence:
            evidence_state = own_evidence.get("EvidenceState", "")
            evidence_commit = own_evidence.get("Commit", "")
        else:
            evidence_state = "NONE"
            evidence_commit = ""

        rows.append(
            {
                "DeliverableID": deliverable_id,
                "PackageID": package_id,
                "DeliverableName": node["DeliverableName"],
                "LifecycleState": lifecycle_overrides.get(deliverable_id, node["LifecycleState"]),
                "ImplementationEvidenceState": evidence_state,
                "EvidenceCommit": evidence_commit,
                "ActiveUpstreamCount": str(len(active_upstreams)),
                "SatisfiedUpstreamCount": str(len(satisfied)),
                "BlockingUpstreamCount": str(len(blocking)),
                "BlockerState": "BLOCKED" if blocking else "UNBLOCKED",
                "BlockingUpstreamDeliverables": ";".join(
                    sorted({edge.get("TargetDeliverableID", "") for edge in blocking})
                ),
                "BlockingEdgeIDs": ";".join(edge.get("DependencyID", "") for edge in blocking),
            }
        )
    return rows, len(active_edges), candidate_edge_count, architecture_basis_edge_count


def csv_text(rows: list[dict[str, str]]) -> str:
    buffer = io.StringIO()
    writer = csv.DictWriter(buffer, fieldnames=QUEUE_HEADER, lineterminator="\n")
    writer.writeheader()
    writer.writerows(rows)
    return buffer.getvalue()


def existing_lifecycle_overrides(root: Path) -> dict[str, str]:
    try:
        header, rows = read_csv(root / COORD_QUEUE_CSV)
    except (ValueError, csv.Error):
        return {}
    if "DeliverableID" not in header or "LifecycleState" not in header:
        return {}
    return {
        row["DeliverableID"]: row["LifecycleState"]
        for row in rows
        if row.get("DeliverableID") and row.get("LifecycleState")
    }


def md_text(
    *,
    dag: str,
    rows: list[dict[str, str]],
    active_edge_count: int,
    candidate_edge_count: int,
    evidence_records: int,
    committed_evidence_records: int,
    semantic_ready_count: int,
    architecture_basis_edge_count: int,
    updated: str,
) -> str:
    package_counts = Counter((row["PackageID"], row["BlockerState"]) for row in rows)
    packages = sorted({row["PackageID"] for row in rows})
    unblocked = [row for row in rows if row["BlockerState"] == "UNBLOCKED"]
    blocked = [row for row in rows if row["BlockerState"] == "BLOCKED"]
    lines = [
        "---",
        "doc_id: DEV-001-BLOCKER-QUEUE",
        "doc_kind: coordination.blocker_queue",
        "status: computed_active_edges_only",
        "created: 2026-04-30",
        f"updated: {updated}",
        f"source_graph: execution/_DAG/{dag}/DependencyEdges.csv",
        "implementation_evidence_source: execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv",
        "implementation_threshold: COMMITTED",
        "architecture_basis: satisfied_by_existing_baseline",
        "candidate_edges: excluded",
        "---",
        "",
        "# DEV-001 Implementation-Readiness Blocker Queue",
        "",
        "This blocker queue is an advisory implementation-readiness view only. It is not a schedule, staffing plan, priority list, lifecycle approval, professional approval, or readiness-for-reliance claim.",
        "",
        "## Computation Rule",
        "",
        f"- Source graph: `execution/_DAG/{dag}/DependencyEdges.csv`.",
        "- Included edges: `Status=ACTIVE` only.",
        "- Excluded edges: all `Status=CANDIDATE` rows.",
        "- Direction convention: `FromDeliverableID` is the downstream consumer and is blocked by `TargetDeliverableID`, the upstream provider.",
        "- Satisfaction threshold: upstream provider has `COMMITTED` evidence in `execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv`.",
        "- `SEMANTIC_READY` remains decomposition/context readiness evidence; it does not satisfy implementation blockers by itself.",
        "- `PKG-00` provider edges are satisfied by the accepted architecture baseline, not by implementation evidence.",
        "- `UNBLOCKED` means all active upstream implementation dependencies satisfy the threshold or are satisfied architecture-basis edges.",
        "- `BLOCKED` means one or more active upstream providers lack committed implementation evidence.",
        "",
        "## Evidence Summary",
        "",
        "| Evidence | Count |",
        "|---|---:|",
        f"| Packages represented | {len(packages)} |",
        f"| Deliverable nodes represented | {len(rows)} |",
        f"| Active edges included | {active_edge_count} |",
        f"| Candidate edges excluded | {candidate_edge_count} |",
        f"| Implementation evidence records | {evidence_records} |",
        f"| Committed implementation evidence | {committed_evidence_records} |",
        f"| Filesystem lifecycle `SEMANTIC_READY` (display only) | {semantic_ready_count} |",
        f"| PKG-00 architecture-basis edges satisfied | {architecture_basis_edge_count} |",
        f"| Implementation `UNBLOCKED` deliverables | {len(unblocked)} |",
        f"| Implementation `BLOCKED` deliverables | {len(blocked)} |",
        "",
        "## Package Summary",
        "",
        "| PackageID | UNBLOCKED | BLOCKED |",
        "|---|---:|---:|",
    ]
    for package_id in packages:
        lines.append(
            f"| `{package_id}` | {package_counts[(package_id, 'UNBLOCKED')]} | {package_counts[(package_id, 'BLOCKED')]} |"
        )
    lines.extend(
        [
            "",
            "## Unblocked DAG-Ready Items",
            "",
            "These deliverables have no active upstream implementation dependency below the `COMMITTED` threshold. Items without their own committed evidence are DAG-ready candidates, not completed work.",
            "",
            "| DeliverableID | PackageID | Implementation evidence | Active upstream | Name |",
            "|---|---|---|---:|---|",
        ]
    )
    for row in unblocked:
        lines.append(
            f"| `{row['DeliverableID']}` | `{row['PackageID']}` | {evidence_label(row)} | {row['ActiveUpstreamCount']} | {row['DeliverableName']} |"
        )
    lines.extend(
        [
            "",
            "## Blocked Items Grouped By Missing Upstream",
            "",
        ]
    )
    if blocked:
        missing: dict[str, list[str]] = defaultdict(list)
        for row in blocked:
            for upstream in row["BlockingUpstreamDeliverables"].split(";"):
                if upstream:
                    missing[upstream].append(row["DeliverableID"])
        lines.extend(["| Missing upstream | Blocked deliverables |", "|---|---|"])
        for upstream in sorted(missing):
            lines.append(f"| `{upstream}` | {', '.join(f'`{item}`' for item in sorted(missing[upstream]))} |")
    else:
        lines.append("No blocked items were found under the implementation-readiness threshold.")
    lines.extend(
        [
            "",
            "## Per-Deliverable Blocked Items",
            "",
        ]
    )
    if blocked:
        lines.extend(
            [
                "| DeliverableID | PackageID | Blocking upstream deliverables | Blocking edge IDs |",
                "|---|---|---|---|",
            ]
        )
        for row in blocked:
            lines.append(
                f"| `{row['DeliverableID']}` | `{row['PackageID']}` | `{row['BlockingUpstreamDeliverables']}` | `{row['BlockingEdgeIDs']}` |"
            )
    else:
        lines.append("No per-deliverable implementation blockers were found.")
    lines.extend(
        [
            "",
            "## Candidate Edges Excluded",
            "",
            "Candidate edges remain non-gating pending later `RECONCILIATION` and `CHANGE`; they were not used in the blocker state calculation.",
            "",
            "## Machine-Readable Queue",
            "",
            "Full per-deliverable queue rows are recorded in `execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv`.",
            "",
        ]
    )
    return "\n".join(lines)


def evidence_label(row: dict[str, str]) -> str:
    state = row["ImplementationEvidenceState"]
    commit = row["EvidenceCommit"]
    if commit:
        return f"`{state}` `{commit}`"
    return f"`{state}`"


def generate(root: Path, dag: str, updated: str) -> tuple[GeneratedQueues, list[str]]:
    nodes_path, edges_path, _dag_csv, _dag_md = dag_paths(root, dag)
    evidence_header, evidence_rows = read_csv(root / IMPLEMENTATION_EVIDENCE)
    node_header, nodes = read_csv(nodes_path)
    edge_header, edges = read_csv(edges_path)

    findings: list[str] = []
    node_ids = {row["DeliverableID"] for row in nodes}
    findings.extend(validate_evidence(
        root=root,
        evidence_header=evidence_header,
        evidence_rows=evidence_rows,
        node_ids=node_ids,
        validate_git=(root / ".git").exists(),
    ))
    if "DeliverableID" not in node_header:
        findings.append("DeliverableNodes.csv missing DeliverableID")
    if "Status" not in edge_header:
        findings.append("DependencyEdges.csv missing Status")

    rows, active_edge_count, candidate_edge_count, architecture_basis_edge_count = queue_rows(
        nodes=nodes,
        edges=edges,
        evidence_rows=evidence_rows,
        lifecycle_overrides=existing_lifecycle_overrides(root),
    )
    generated = GeneratedQueues(
        csv_text=csv_text(rows),
        md_text=md_text(
            dag=dag,
            rows=rows,
            active_edge_count=active_edge_count,
            candidate_edge_count=candidate_edge_count,
            evidence_records=len(evidence_rows),
            committed_evidence_records=sum(1 for row in evidence_rows if row.get("EvidenceState") == "COMMITTED"),
            semantic_ready_count=sum(1 for row in rows if row.get("LifecycleState") == "SEMANTIC_READY"),
            architecture_basis_edge_count=architecture_basis_edge_count,
            updated=updated,
        ),
        rows=rows,
        active_edge_count=active_edge_count,
        candidate_edge_count=candidate_edge_count,
        evidence_records=len(evidence_rows),
        committed_evidence_records=sum(1 for row in evidence_rows if row.get("EvidenceState") == "COMMITTED"),
        semantic_ready_count=sum(1 for row in rows if row.get("LifecycleState") == "SEMANTIC_READY"),
        architecture_basis_edge_count=architecture_basis_edge_count,
    )
    return generated, findings


def compare_outputs(root: Path, dag: str, generated: GeneratedQueues) -> list[str]:
    _nodes, _edges, dag_csv, dag_md = dag_paths(root, dag)
    expected = {
        root / COORD_QUEUE_CSV: generated.csv_text,
        root / COORD_QUEUE_MD: generated.md_text,
        dag_csv: generated.csv_text,
        dag_md: generated.md_text,
    }
    findings = []
    for path, text in expected.items():
        try:
            existing = path.read_text(encoding="utf-8")
        except FileNotFoundError:
            findings.append(f"missing generated output: {path.relative_to(root)}")
            continue
        if existing != text:
            findings.append(f"stale generated output: {path.relative_to(root)}")
    return findings


def write_outputs(root: Path, dag: str, generated: GeneratedQueues) -> None:
    _nodes, _edges, dag_csv, dag_md = dag_paths(root, dag)
    for path, text in (
        (root / COORD_QUEUE_CSV, generated.csv_text),
        (root / COORD_QUEUE_MD, generated.md_text),
        (dag_csv, generated.csv_text),
        (dag_md, generated.md_text),
    ):
        path.write_text(text, encoding="utf-8")


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Maintain DEV-001 coordination queue derivatives.")
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check", action="store_true", help="Validate evidence and generated queue mirrors.")
    mode.add_argument("--write", action="store_true", help="Rewrite derivative blocker queue files.")
    parser.add_argument("--dag", help="DAG authority ID, e.g. DAG-005. Defaults to execution/_DAG/_LATEST.md.")
    parser.add_argument("--repo-root", default=str(ROOT), help="Repository root.")
    parser.add_argument("--updated", default="2026-05-28", help="Updated date to render in Markdown outputs.")
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    root = Path(args.repo_root).resolve()
    dag = args.dag or latest_dag(root)
    generated, findings = generate(root, dag, args.updated)
    if args.check:
        findings.extend(compare_outputs(root, dag, generated))
    if findings:
        for finding in findings:
            print(f"ERROR: {finding}", file=sys.stderr)
        return 1
    if args.write:
        write_outputs(root, dag, generated)
        print(f"WROTE: DEV-001 blocker queue derivatives for {dag}")
    else:
        print(f"VALID: DEV-001 coordination derivatives for {dag}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
