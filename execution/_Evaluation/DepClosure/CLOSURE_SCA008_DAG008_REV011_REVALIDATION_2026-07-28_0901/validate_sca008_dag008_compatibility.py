#!/usr/bin/env python3
"""Deterministically revalidate SCA-008 against immutable DAG-008."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import re
import subprocess
from collections import Counter
from pathlib import Path


SCA_BASIS = "7b0be4d8772a16e5a4774a17988479587d00acca"
SCA_APPLICATION = "9b52076701c218f69255afbedcfc52025bd47fa3"
SCA_MERGE = "380ea2a794588075b83fe8cc0108ab7ce74b6b33"
EXAMINED_BASIS = "404e47c16a88e7ffdc6d1fc5fac61ebb6864211e"


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_csv(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle))


def git(repo: Path, *args: str) -> str:
    return subprocess.check_output(
        ["git", *args], cwd=repo, text=True, encoding="utf-8"
    ).strip()


def manifest_check(dag_dir: Path) -> dict[str, object]:
    rows = []
    for line in (dag_dir / "MANIFEST.sha256").read_text(encoding="utf-8").splitlines():
        digest, relpath = line.split("  ", 1)
        actual = sha256(dag_dir / relpath)
        rows.append(
            {
                "path": relpath,
                "expected": digest,
                "actual": actual,
                "pass": digest == actual,
            }
        )
    return {
        "entries": len(rows),
        "all_pass": all(row["pass"] for row in rows),
        "rows": rows,
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()

    repo = args.repo_root.resolve()
    project = repo / "projects/chirality-piping"
    execution = project / "execution"
    dag_dir = execution / "_DAG/DAG-008"

    changed_at_sca = git(
        repo,
        "diff",
        "--name-only",
        SCA_BASIS,
        SCA_APPLICATION,
        "--",
        "projects/chirality-piping/execution",
    ).splitlines()
    changed_after_merge = git(
        repo,
        "diff",
        "--name-only",
        SCA_MERGE,
        EXAMINED_BASIS,
        "--",
        "projects/chirality-piping/execution",
    ).splitlines()

    def dependency_or_dag(path: str) -> bool:
        return path.endswith("/Dependencies.csv") or "/execution/_DAG/" in path

    local_paths = sorted(execution.glob("PKG-*/1_Working/DEL-*/Dependencies.csv"))
    local_rows = [row for path in local_paths for row in read_csv(path)]
    aggregate_rows = read_csv(dag_dir / "DependencyEdges.csv")
    node_rows = read_csv(dag_dir / "DeliverableNodes.csv")
    deliverable_rows = read_csv(project / "docs/_Registers/Deliverables.csv")
    duplicate_rows = read_csv(dag_dir / "DAG-008_DuplicateEdgeWorklist.csv")

    local_by_id = {row["DependencyID"]: row for row in local_rows}
    aggregate_by_id = {row["DependencyID"]: row for row in aggregate_rows}
    common_fields = [
        field
        for field in local_rows[0]
        if field not in {"Status", "Notes"}
    ]
    field_mismatches = []
    status_mismatches = []
    for dependency_id in sorted(set(local_by_id) & set(aggregate_by_id)):
        local = local_by_id[dependency_id]
        aggregate = aggregate_by_id[dependency_id]
        for field in common_fields:
            if local[field] != aggregate[field]:
                field_mismatches.append(
                    {
                        "DependencyID": dependency_id,
                        "Field": field,
                        "Local": local[field],
                        "Aggregate": aggregate[field],
                    }
                )
        if local["Status"] != aggregate["Status"]:
            status_mismatches.append(
                {
                    "DependencyID": dependency_id,
                    "Local": local["Status"],
                    "Aggregate": aggregate["Status"],
                }
            )

    expected_retired = {
        row["RetiredDependencyID"] for row in duplicate_rows
    }
    actual_status_mismatch_ids = {
        row["DependencyID"] for row in status_mismatches
    }

    def active_edge_set(rows: list[dict[str, str]]) -> set[tuple[str, str]]:
        edges: set[tuple[str, str]] = set()
        for row in rows:
            if (
                row["Status"] != "ACTIVE"
                or row["DependencyClass"] != "EXECUTION"
                or row["TargetType"] != "DELIVERABLE"
                or not row["FromDeliverableID"]
                or not row["TargetDeliverableID"]
            ):
                continue
            if row["Direction"] == "UPSTREAM":
                edges.add((row["FromDeliverableID"], row["TargetDeliverableID"]))
            elif row["Direction"] == "DOWNSTREAM":
                edges.add((row["TargetDeliverableID"], row["FromDeliverableID"]))
        return edges

    local_active = active_edge_set(local_rows)
    aggregate_active = active_edge_set(aggregate_rows)
    deliverable_ids = {row["DeliverableID"] for row in deliverable_rows}
    node_ids = {row["DeliverableID"] for row in node_rows}
    wave_count = len(
        re.findall(
            r"^\| \d+ \|",
            (dag_dir / "TopologicalWaves.md").read_text(encoding="utf-8"),
            flags=re.MULTILINE,
        )
    )
    dag_audit = json.loads(
        (args.output.parent / "dag008_audit.json").read_text(encoding="utf-8")
    )
    manifest = manifest_check(dag_dir)

    checks = {
        "examined_basis_matches": git(repo, "rev-parse", "HEAD") == EXAMINED_BASIS,
        "sca_changed_dependency_or_dag_paths": [
            path for path in changed_at_sca if dependency_or_dag(path)
        ],
        "post_merge_changed_dependency_or_dag_paths": [
            path for path in changed_after_merge if dependency_or_dag(path)
        ],
        "local_register_count": len(local_paths),
        "local_row_count": len(local_rows),
        "aggregate_row_count": len(aggregate_rows),
        "local_status_counts": dict(Counter(row["Status"] for row in local_rows)),
        "aggregate_status_counts": dict(
            Counter(row["Status"] for row in aggregate_rows)
        ),
        "dependency_id_sets_equal": set(local_by_id) == set(aggregate_by_id),
        "non_status_notes_field_mismatch_count": len(field_mismatches),
        "non_status_notes_field_mismatches": field_mismatches,
        "status_mismatch_count": len(status_mismatches),
        "status_mismatches": status_mismatches,
        "status_mismatches_equal_duplicate_retirements": (
            actual_status_mismatch_ids == expected_retired
            and all(
                row["Local"] == "ACTIVE" and row["Aggregate"] == "RETIRED"
                for row in status_mismatches
            )
        ),
        "expected_duplicate_retirement_count": len(expected_retired),
        "local_unique_active_directed_edges": len(local_active),
        "aggregate_unique_active_directed_edges": len(aggregate_active),
        "active_directed_edge_sets_equal": local_active == aggregate_active,
        "deliverable_register_count": len(deliverable_ids),
        "dag_node_count": len(node_ids),
        "deliverable_node_sets_equal": deliverable_ids == node_ids,
        "topological_wave_count": wave_count,
        "dag_manifest": manifest,
        "dag_audit": {
            "canonical_finding_count": dag_audit["canonical_finding_count"],
            "endpoint_issue_count": dag_audit["endpoint_issue_count"],
            "active_graph": dag_audit["active_graph"],
        },
        "dag_pointer_sha256": sha256(execution / "_DAG/_LATEST.md"),
        "dag_manifest_sha256": sha256(dag_dir / "MANIFEST.sha256"),
        "dependency_edges_sha256": sha256(dag_dir / "DependencyEdges.csv"),
        "deliverable_nodes_sha256": sha256(dag_dir / "DeliverableNodes.csv"),
        "dag_json_sha256": sha256(dag_dir / "dag.json"),
    }

    pass_conditions = [
        checks["examined_basis_matches"],
        not checks["sca_changed_dependency_or_dag_paths"],
        not checks["post_merge_changed_dependency_or_dag_paths"],
        checks["local_register_count"] == 93,
        checks["local_row_count"] == 1480,
        checks["aggregate_row_count"] == 1480,
        checks["dependency_id_sets_equal"],
        checks["non_status_notes_field_mismatch_count"] == 0,
        checks["status_mismatch_count"] == 30,
        checks["status_mismatches_equal_duplicate_retirements"],
        checks["local_unique_active_directed_edges"] == 972,
        checks["aggregate_unique_active_directed_edges"] == 972,
        checks["active_directed_edge_sets_equal"],
        checks["deliverable_register_count"] == 101,
        checks["dag_node_count"] == 101,
        checks["deliverable_node_sets_equal"],
        checks["topological_wave_count"] == 15,
        manifest["entries"] == 15,
        manifest["all_pass"],
        dag_audit["canonical_finding_count"] == 0,
        dag_audit["endpoint_issue_count"] == 0,
        dag_audit["active_graph"]["scc_count"] == 0,
        dag_audit["active_graph"]["duplicate_edge_count"] == 0,
        dag_audit["active_graph"]["bidirectional_pair_count"] == 0,
    ]

    result = {
        "schema": "piping.sca008_dag008_compatibility.v1",
        "run_status": "PASS" if all(pass_conditions) else "BLOCK",
        "disposition": (
            "CURRENT_BY_REVALIDATION" if all(pass_conditions) else "HOLD"
        ),
        "basis": {
            "sca_basis": SCA_BASIS,
            "sca_application": SCA_APPLICATION,
            "sca_merge": SCA_MERGE,
            "examined_basis": EXAMINED_BASIS,
        },
        "checks": checks,
    }
    args.output.write_text(
        json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    return 0 if all(pass_conditions) else 1


if __name__ == "__main__":
    raise SystemExit(main())
