#!/usr/bin/env python3
"""Canonicalize OpenPipeStress dependency registers and build DAG-007.

This tool performs deterministic migration work only. It does not decide graph
topology, promote candidate edges, or update the approved DAG pointer.
"""

from __future__ import annotations

import argparse
import csv
import json
import subprocess
from collections import Counter, defaultdict, deque
from datetime import date
from pathlib import Path
from typing import Iterable


REQUIRED_COLUMNS = [
    "RegisterSchemaVersion",
    "DependencyID",
    "FromPackageID",
    "FromDeliverableID",
    "FromDeliverableName",
    "DependencyClass",
    "AnchorType",
    "Direction",
    "DependencyType",
    "TargetType",
    "TargetPackageID",
    "TargetDeliverableID",
    "TargetRefID",
    "TargetName",
    "TargetLocation",
    "Statement",
    "EvidenceFile",
    "SourceRef",
    "EvidenceQuote",
    "Explicitness",
    "RequiredMaturity",
    "ProposedMaturity",
    "SatisfactionStatus",
    "Confidence",
    "Origin",
    "FirstSeen",
    "LastSeen",
    "Status",
    "Notes",
]

CANONICAL_DEPENDENCY_TYPES = {
    "PREREQUISITE",
    "INTERFACE",
    "HANDOVER",
    "CONSTRAINT",
    "ENABLES",
    "OTHER",
}

BASIS_TYPES = {
    "ARCHITECTURE_BASIS",
    "SOURCE_BASIS",
    "IDENTITY_BASIS",
    "EXPORT_SOURCE_BASIS",
}

PREREQUISITE_TYPES = {
    "DOMAIN_MODEL",
    "RESULT_RECORD_MODEL",
    "SOLVER_PREDECESSOR",
    "GOVERNANCE_PREDECESSOR",
    "GUI_PREDECESSOR",
    "INTEROP_PREDECESSOR",
    "LOAD_STRESS_PREDECESSOR",
    "REPORTING_PREDECESSOR",
    "RULE_PACK_PREDECESSOR",
    "SECURITY_PREDECESSOR",
    "VALIDATION_PREDECESSOR",
    "DOCS_PREDECESSOR",
}

INTERFACE_TYPES = {
    "ADAPTER_FRAMEWORK",
    "COMPARISON_EXPORT_CONTRACT",
    "COMPONENT_LIBRARY_SCHEMA",
    "DIAGNOSTICS_CONTRACT",
    "EXPORT_CONTRACT",
    "EXPORT_PROFILE",
    "HANDOFF_PACKAGE_CONTRACT",
    "PERSISTENCE_CONTRACT",
    "PHYSICAL_TRANSFORM_CONTRACT",
    "PLUGIN_CONTRACT",
    "RESULT_EXPORT_CONTRACT",
    "RUNNER_CONTRACT",
    "SCHEMA_CONTRACT",
    "SERVICE_API",
    "TARGET_MAPPING_CONTRACT",
    "UNIT_CONTRACT",
}

CONSTRAINT_TYPES = {
    "API_PLUGIN_BOUNDARY",
    "BOUNDARY",
    "DATA_BOUNDARY",
    "GUI_GEOMETRY_CONTEXT",
}

HANDOVER_TYPES = {
    "CONSUMED_BY",
    "REPORT_INTEGRATION",
}

TARGET_TYPE_MAP = {
    "REFERENCE": "DOCUMENT",
    "SCOPE_ITEM": "REQUIREMENT",
}


def clean(value: object) -> str:
    return str(value or "").strip()


def repo_root(start: Path) -> Path:
    result = subprocess.run(
        ["git", "rev-parse", "--show-toplevel"],
        cwd=start,
        text=True,
        capture_output=True,
        check=True,
    )
    return Path(result.stdout.strip())


def read_csv(path: Path) -> tuple[list[str], list[dict[str, str]]]:
    with path.open(newline="", encoding="utf-8-sig") as handle:
        reader = csv.DictReader(handle)
        return list(reader.fieldnames or []), list(reader)


def write_csv(path: Path, fieldnames: list[str], rows: Iterable[dict[str, str]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames, extrasaction="ignore", lineterminator="\n")
        writer.writeheader()
        for row in rows:
            writer.writerow({field: row.get(field, "") for field in fieldnames})


def append_note(notes: str, *parts: str) -> str:
    existing = clean(notes)
    additions = [part for part in parts if part and part not in existing]
    if not additions:
        return existing
    return "; ".join([item for item in [existing, *additions] if item])


def canonical_dependency_type(row: dict[str, str]) -> str:
    value = clean(row.get("DependencyType"))
    if clean(row.get("DependencyClass")) == "ANCHOR":
        return "OTHER"
    if value in CANONICAL_DEPENDENCY_TYPES:
        return value
    if value in BASIS_TYPES or value in PREREQUISITE_TYPES or value.endswith("_PREDECESSOR"):
        return "PREREQUISITE"
    if value in INTERFACE_TYPES or value.endswith("_CONTRACT") or value.endswith("_FRAMEWORK") or value.endswith("_PROFILE"):
        return "INTERFACE"
    if value in CONSTRAINT_TYPES:
        return "CONSTRAINT"
    if value in HANDOVER_TYPES:
        return "HANDOVER"
    if value == "TRACEABILITY":
        return "OTHER"
    return "OTHER"


def canonicalize_row(
    row: dict[str, str],
    *,
    source_file: Path,
    today: str,
    candidate_rows: list[dict[str, str]],
) -> dict[str, str]:
    original = {column: clean(row.get(column)) for column in row}
    output = {column: row.get(column, "") for column in row}

    for column in REQUIRED_COLUMNS:
        output.setdefault(column, "")
    output["RegisterSchemaVersion"] = "v3.1"

    legacy_notes: list[str] = []
    for field in [
        "AnchorType",
        "Direction",
        "DependencyType",
        "TargetType",
        "Explicitness",
        "SatisfactionStatus",
        "Origin",
        "Status",
    ]:
        value = clean(original.get(field))
        if value:
            legacy_notes.append(f"legacy_{field.lower()}={value}")

    dependency_class = clean(output.get("DependencyClass"))
    if dependency_class not in {"ANCHOR", "EXECUTION"}:
        dependency_class = "EXECUTION"
        output["DependencyClass"] = dependency_class

    if dependency_class == "ANCHOR":
        output["AnchorType"] = "TRACES_TO_REQUIREMENT" if clean(output.get("AnchorType")) not in {"IMPLEMENTS_NODE", "TRACES_TO_REQUIREMENT"} else clean(output.get("AnchorType"))
        output["DependencyType"] = "OTHER"
        output["Direction"] = "UPSTREAM"
    else:
        output["AnchorType"] = "NOT_APPLICABLE"
        output["DependencyType"] = canonical_dependency_type(output)
        if clean(output.get("Direction")) not in {"UPSTREAM", "DOWNSTREAM"}:
            output["Direction"] = "UPSTREAM"

    output["TargetType"] = TARGET_TYPE_MAP.get(clean(output.get("TargetType")), clean(output.get("TargetType")))
    if clean(output.get("TargetType")) not in {"DELIVERABLE", "PACKAGE", "WBS_NODE", "REQUIREMENT", "DOCUMENT", "EQUIPMENT", "EXTERNAL", "UNKNOWN"}:
        output["TargetType"] = "UNKNOWN"

    if output["TargetType"] != "DELIVERABLE":
        if clean(output.get("TargetDeliverableID")) and not clean(output.get("TargetRefID")):
            output["TargetRefID"] = clean(output.get("TargetDeliverableID"))
        output["TargetDeliverableID"] = ""
    elif not clean(output.get("TargetDeliverableID")) and clean(output.get("TargetRefID")).startswith("DEL-"):
        output["TargetDeliverableID"] = clean(output.get("TargetRefID"))

    explicitness = clean(output.get("Explicitness"))
    output["Explicitness"] = explicitness if explicitness in {"EXPLICIT", "IMPLICIT"} else "IMPLICIT"

    satisfaction = clean(output.get("SatisfactionStatus"))
    output["SatisfactionStatus"] = satisfaction if satisfaction in {"TBD", "PENDING", "IN_PROGRESS", "SATISFIED", "WAIVED", "NOT_APPLICABLE"} else "TBD"

    confidence = clean(output.get("Confidence"))
    output["Confidence"] = confidence if confidence in {"HIGH", "MEDIUM", "LOW"} else "LOW"

    output["Origin"] = clean(output.get("Origin")) if clean(output.get("Origin")) == "DECLARED" else "EXTRACTED"

    if clean(output.get("Status")) == "CANDIDATE":
        candidate_rows.append(candidate_record(source_file, original, output))
        output["Status"] = "RETIRED"
        legacy_notes.append("candidate_disposition=moved_to_non_authoritative_worklist")
    elif clean(output.get("Status")) not in {"ACTIVE", "RETIRED"}:
        output["Status"] = "ACTIVE"

    if not clean(output.get("FirstSeen")):
        output["FirstSeen"] = today
    output["LastSeen"] = today
    output["Notes"] = append_note(clean(output.get("Notes")), *legacy_notes)
    return output


def candidate_record(source_file: Path, original: dict[str, str], canonical: dict[str, str]) -> dict[str, str]:
    return {
        "SourceFile": str(source_file),
        "DependencyID": clean(original.get("DependencyID")),
        "FromPackageID": clean(original.get("FromPackageID")),
        "FromDeliverableID": clean(original.get("FromDeliverableID")),
        "OriginalDependencyType": clean(original.get("DependencyType")),
        "CanonicalDependencyType": clean(canonical.get("DependencyType")),
        "TargetPackageID": clean(original.get("TargetPackageID")),
        "TargetDeliverableID": clean(original.get("TargetDeliverableID")),
        "Statement": clean(original.get("Statement")),
        "Disposition": "NON_AUTHORITATIVE_CANDIDATE_WORKLIST",
        "Notes": clean(original.get("Notes")),
    }


def declared_section(existing: str, heading: str) -> list[str]:
    lines = existing.splitlines()
    start = None
    for index, line in enumerate(lines):
        if line.strip() == heading:
            start = index
            break
    if start is None:
        return ["- None recorded."]
    collected: list[str] = []
    for line in lines[start + 1 :]:
        if line.startswith("## "):
            break
        if line.strip():
            collected.append(line)
    return collected or ["- None recorded."]


def render_dependencies_md(path: Path, rows: list[dict[str, str]], candidate_count: int, today: str) -> str:
    existing = path.read_text(encoding="utf-8") if path.exists() else ""
    deliverable_id = clean(rows[0].get("FromDeliverableID")) if rows else path.parent.name.split("_", 1)[0]
    deliverable_name = clean(rows[0].get("FromDeliverableName")) if rows else path.parent.name
    upstream = declared_section(existing, "## Declared Upstream Dependencies")
    downstream = declared_section(existing, "## Declared Downstream Dependencies")
    status_counts = Counter(clean(row.get("Status")) or "BLANK" for row in rows)
    class_counts = Counter(clean(row.get("DependencyClass")) or "BLANK" for row in rows)
    type_counts = Counter(clean(row.get("DependencyType")) or "BLANK" for row in rows)

    type_lines = [f"- `{key}`: {type_counts[key]}" for key in sorted(type_counts)]
    return "\n".join(
        [
            f"# Dependencies: {deliverable_id} {deliverable_name}",
            "",
            "## Coordination Mode",
            "- **Mode:** FULL_GRAPH",
            "- **Graph Authority:** `execution/_DAG/DAG-007/` is the current approved canonical graph authority.",
            "- **Authority Boundary:** Candidate/non-gating edges are not represented through `Status=CANDIDATE` in current canonical registers.",
            "",
            "## Declared Upstream Dependencies",
            *upstream,
            "",
            "## Declared Downstream Dependencies",
            *downstream,
            "",
            "## Extracted Dependency Register",
            "- **Local Register:** `Dependencies.csv`",
            "- **Register schema version:** `v3.1`",
            f"- **Canonicalized:** {today}",
            f"- **Rows:** {len(rows)} total; {status_counts.get('ACTIVE', 0)} ACTIVE; {status_counts.get('RETIRED', 0)} RETIRED.",
            f"- **Classes:** ANCHOR={class_counts.get('ANCHOR', 0)}; EXECUTION={class_counts.get('EXECUTION', 0)}.",
            f"- **Candidate rows moved to worklist:** {candidate_count}.",
            "",
            "## Canonical Dependency Types",
            *(type_lines or ["- None"]),
            "",
            "## Run Notes",
            "- Core enum fields conform to the canonical Chirality dependency model.",
            "- Legacy project-specific labels are preserved in `Notes` as `legacy_*` fields.",
            "- Candidate rows remain non-gating in the candidate worklist and require explicit human approval plus graph revalidation before promotion.",
            "- This dependency refresh does not authorize implementation, lifecycle promotion, release claims, professional approval, certification, sealing, authentication, or code-compliance claims.",
            "",
        ]
    )


def local_dependency_paths(working_root: Path) -> list[Path]:
    return sorted(working_root.glob("execution/PKG-*/1_Working/DEL-*/Dependencies.csv"))


def canonicalize_local(working_root: Path, today: str) -> dict[str, object]:
    candidate_rows: list[dict[str, str]] = []
    touched: list[dict[str, object]] = []
    worklist_path = working_root / "execution/_Reconciliation/DependencyTypeSystem/TYPE_RECTIFICATION_2026-06-16/CandidateDependencyEdges.csv"

    for csv_path in local_dependency_paths(working_root):
        header, rows = read_csv(csv_path)
        fieldnames = REQUIRED_COLUMNS + [column for column in header if column not in REQUIRED_COLUMNS]
        before = [dict(row) for row in rows]
        local_candidates_before = len(candidate_rows)
        canonical_rows = [
            canonicalize_row(row, source_file=csv_path.relative_to(working_root), today=today, candidate_rows=candidate_rows)
            for row in rows
        ]
        write_csv(csv_path, fieldnames, canonical_rows)
        md_path = csv_path.with_name("_DEPENDENCIES.md")
        md_path.write_text(
            render_dependencies_md(
                md_path,
                canonical_rows,
                len(candidate_rows) - local_candidates_before,
                today,
            ),
            encoding="utf-8",
        )
        touched.append({
            "path": str(csv_path.relative_to(working_root)),
            "rows": len(canonical_rows),
            "changed": before != canonical_rows,
            "candidate_rows_moved": len(candidate_rows) - local_candidates_before,
        })

    write_csv(
        worklist_path,
        [
            "SourceFile",
            "DependencyID",
            "FromPackageID",
            "FromDeliverableID",
            "OriginalDependencyType",
            "CanonicalDependencyType",
            "TargetPackageID",
            "TargetDeliverableID",
            "Statement",
            "Disposition",
            "Notes",
        ],
        candidate_rows,
    )
    return {"touched": touched, "candidate_worklist": str(worklist_path), "candidate_rows": len(candidate_rows)}


def normalized_edge(row: dict[str, str]) -> tuple[str, str] | None:
    from_id = clean(row.get("FromDeliverableID"))
    target_id = clean(row.get("TargetDeliverableID"))
    if not from_id or not target_id:
        return None
    if clean(row.get("Direction")) == "DOWNSTREAM":
        return target_id, from_id
    return from_id, target_id


def active_execution_rows(rows: Iterable[dict[str, str]]) -> list[dict[str, str]]:
    return [
        row
        for row in rows
        if clean(row.get("Status")) == "ACTIVE"
        and clean(row.get("DependencyClass")) == "EXECUTION"
        and clean(row.get("TargetType")) == "DELIVERABLE"
        and normalized_edge(row) is not None
    ]


def topological_waves(nodes: list[str], rows: list[dict[str, str]]) -> list[list[str]]:
    graph: dict[str, set[str]] = defaultdict(set)
    indegree: dict[str, int] = {node: 0 for node in nodes}
    for row in rows:
        edge = normalized_edge(row)
        if edge is None:
            continue
        source, target = edge
        if target not in graph[source]:
            graph[source].add(target)
            indegree[target] = indegree.get(target, 0) + 1
            indegree.setdefault(source, 0)

    queue = deque(sorted(node for node, degree in indegree.items() if degree == 0))
    waves: list[list[str]] = []
    while queue:
        wave = list(queue)
        waves.append(wave)
        queue.clear()
        next_nodes: list[str] = []
        for node in wave:
            for target in sorted(graph.get(node, set())):
                indegree[target] -= 1
                if indegree[target] == 0:
                    next_nodes.append(target)
        queue.extend(sorted(next_nodes))
    return waves


def render_topological_waves(waves: list[list[str]], today: str, source_basis: str) -> str:
    lines = [
        "---",
        "doc_id: DAG-007-TOPOLOGICAL-WAVES",
        "doc_kind: coordination.topological_waves",
        "status: proposed_pending_approval",
        f"created: {today}",
        "---",
        "",
        "# DAG-007 Topological Waves",
        "",
        f"These waves are computed from {source_basis}.",
        "Candidate rows are excluded from the active graph and remain in the non-authoritative candidate worklist.",
        "",
        "| Wave | Node count | Deliverables |",
        "|---|---:|---|",
    ]
    for index, wave in enumerate(waves, start=1):
        nodes = ", ".join(f"`{node}`" for node in wave)
        lines.append(f"| {index} | {len(wave)} | {nodes} |")
    lines.append("")
    return "\n".join(lines)


def render_cycle_report(active_count: int, node_count: int, wave_count: int, active_scc_count: int, today: str, source_basis: str) -> str:
    status = "ACYCLIC" if active_scc_count == 0 else "CYCLES_REQUIRING_RECONCILIATION"
    return "\n".join(
        [
            "# DAG-007 Cycle Report",
            "",
            f"- Active graph status: {status}",
            f"- Node count: {node_count}",
            f"- Active edge count: {active_count}",
            f"- Active SCCs with more than one node: {active_scc_count}",
            "- Candidate rows: excluded from canonical active edge register",
            f"- Topological waves: {wave_count}",
            f"- Generated: {today}",
            "",
            f"DAG-007 is proposed from {source_basis}.",
            "Candidate SCCs remain non-gating worklist items pending explicit human resolution.",
            "",
        ]
    )


def render_review_packet(summary: dict[str, object], today: str) -> str:
    return "\n".join(
        [
            "---",
            "doc_id: DAG-007-APPROVAL-REVIEW-PACKET",
            "doc_kind: coordination.approval_review_packet",
            "status: proposed_pending_human_approval",
            f"created: {today}",
            "---",
            "",
            "# DAG-007 Approval Review Packet",
            "",
            "## Proposal Summary",
            "",
            "`DAG-007` is a proposed canonical successor built from refreshed deliverable-local dependency registers.",
            "It removes candidate rows from the v3.1 edge register and normalizes core dependency enum fields",
            "to the canonical Chirality model.",
            "",
            "## Graph Facts",
            "",
            "| Fact | Value |",
            "|---|---:|",
            f"| Deliverable nodes | {summary['node_count']} |",
            f"| Edge rows | {summary['edge_row_count']} |",
            f"| Active edges | {summary['active_edge_count']} |",
            f"| Retired rows | {summary['retired_edge_count']} |",
            f"| Candidate worklist rows | {summary['candidate_edge_count']} |",
            f"| Active SCCs | {summary['active_scc_count']} |",
            f"| Topological waves | {summary['wave_count']} |",
            "",
            "## Approval Boundary",
            "",
            "Human approval, if granted, approves only the canonical graph-authority successor.",
            "It does not promote candidate rows, dispatch Type 2 work, change lifecycle states,",
            "make release-readiness claims, or create professional/code-compliance acceptance.",
            "",
        ]
    )


def render_approval_record_template(summary: dict[str, object], today: str) -> str:
    return "\n".join(
        [
            "---",
            "doc_id: DAG-007-APPROVAL-RECORD",
            "doc_kind: coordination.approval_record",
            "status: proposed_pending_human_approval",
            f"created: {today}",
            "approved: TBD",
            "approved_by: TBD",
            "approved_decomposition: execution/_Decomposition/SOFTWARE_DECOMP.md",
            'approved_revision: "0.7"',
            "dag_path: execution/_DAG/DAG-007/",
            "approval_scope: canonical_dependency_type_system_rectification",
            "candidate_treatment: separate_non_authoritative_worklist",
            "topology_basis: refreshed_deliverable_local_dependency_registers",
            "type2_dispatch: not_authorized_by_approval_record",
            "lifecycle_changes: not_authorized",
            "---",
            "",
            "# DAG-007 Approval Record Template",
            "",
            "`DAG-007` is not approved at creation time. Replace this template only after explicit human approval.",
            "",
            "## Proposed Decision",
            "",
            "Approve `DAG-007` as the canonical dependency successor built from refreshed local registers,",
            "with candidate rows held outside the canonical active edge register.",
            "",
            "## Proposed Graph Facts",
            "",
            f"- Nodes: {summary['node_count']}",
            f"- Edge rows in canonical register: {summary['edge_row_count']}",
            f"- Active edges: {summary['active_edge_count']}",
            f"- Candidate worklist rows: {summary['candidate_edge_count']}",
            "",
            "## Required Human Approval",
            "",
            "Approval text must explicitly approve this `DAG-007` package before `_DAG/_LATEST.md` is updated.",
            "",
        ]
    )


def build_dag_json(nodes: list[dict[str, str]], rows: list[dict[str, str]], summary: dict[str, object], today: str) -> dict[str, object]:
    active_rows = active_execution_rows(rows)
    return {
        "metadata": {
            "dag_id": "DAG-007",
            "created": today,
            "decomposition_revision": "0.7",
            "node_count": summary["node_count"],
            "package_count": summary["package_count"],
            "edge_row_count": summary["edge_row_count"],
            "active_edge_count": summary["active_edge_count"],
            "candidate_edge_count": summary["candidate_edge_count"],
            "retired_edge_count": summary["retired_edge_count"],
            "approval_status": "PROPOSED_PENDING_HUMAN_APPROVAL",
            "node_baseline_graph": "execution/_DAG/DAG-006/",
            "edge_basis": "refreshed deliverable-local Dependencies.csv registers",
            "candidate_treatment": "SEPARATE_NON_AUTHORITATIVE_WORKLIST",
            "active_cycle_status": "ACYCLIC" if summary["active_scc_count"] == 0 else "CYCLES_REQUIRING_RECONCILIATION",
            "edge_direction": "FromDeliverableID depends on TargetDeliverableID for UPSTREAM rows; DOWNSTREAM rows normalize inverse",
            "approval_record": "execution/_DAG/DAG-007/APPROVAL_RECORD.md",
        },
        "nodes": [
            {
                "id": clean(node.get("DeliverableID")),
                "package_id": clean(node.get("PackageID")),
                "name": clean(node.get("DeliverableName")),
                "type": clean(node.get("DeliverableType")),
                "path": clean(node.get("ExecutionPath")),
            }
            for node in nodes
        ],
        "edges": [
            {
                "id": clean(row.get("DependencyID")),
                "from": clean(row.get("FromDeliverableID")),
                "to": clean(row.get("TargetDeliverableID")),
                "direction": clean(row.get("Direction")),
                "dependency_type": clean(row.get("DependencyType")),
                "status": clean(row.get("Status")),
                "source": normalized_edge(row)[0] if normalized_edge(row) else "",
                "target": normalized_edge(row)[1] if normalized_edge(row) else "",
            }
            for row in active_rows
        ],
    }


def local_dependency_rows(working_root: Path, today: str) -> tuple[list[str], list[dict[str, str]], list[dict[str, str]]]:
    fieldnames = list(REQUIRED_COLUMNS)
    rows: list[dict[str, str]] = []
    candidate_rows: list[dict[str, str]] = []
    for csv_path in local_dependency_paths(working_root):
        header, source_rows = read_csv(csv_path)
        for column in header:
            if column not in fieldnames:
                fieldnames.append(column)
        for row in source_rows:
            normalized = canonicalize_row(
                row,
                source_file=csv_path.relative_to(working_root),
                today=today,
                candidate_rows=candidate_rows,
            )
            if clean(row.get("Status")) == "CANDIDATE":
                continue
            rows.append(normalized)
    return fieldnames, rows, candidate_rows


def active_sccs(rows: list[dict[str, str]]) -> list[list[str]]:
    graph: dict[str, set[str]] = defaultdict(set)
    nodes: set[str] = set()
    for row in active_execution_rows(rows):
        edge = normalized_edge(row)
        if edge is None:
            continue
        source, target = edge
        graph[source].add(target)
        nodes.add(source)
        nodes.add(target)

    index = 0
    stack: list[str] = []
    indices: dict[str, int] = {}
    lowlink: dict[str, int] = {}
    on_stack: set[str] = set()
    components: list[list[str]] = []

    def visit(node: str) -> None:
        nonlocal index
        indices[node] = index
        lowlink[node] = index
        index += 1
        stack.append(node)
        on_stack.add(node)

        for target in sorted(graph.get(node, set())):
            if target not in indices:
                visit(target)
                lowlink[node] = min(lowlink[node], lowlink[target])
            elif target in on_stack:
                lowlink[node] = min(lowlink[node], indices[target])

        if lowlink[node] == indices[node]:
            component: list[str] = []
            while True:
                item = stack.pop()
                on_stack.remove(item)
                component.append(item)
                if item == node:
                    break
            if len(component) > 1:
                components.append(sorted(component))

    for node in sorted(nodes):
        if node not in indices:
            visit(node)
    return sorted(components, key=lambda item: (len(item), item))


def duplicate_retention_key(row: dict[str, str]) -> tuple[int, str, str]:
    origin_rank = 0 if clean(row.get("Origin")) == "DECLARED" else 1
    return (origin_rank, clean(row.get("FirstSeen")), clean(row.get("DependencyID")))


def retire_duplicate_active_edges(rows: list[dict[str, str]], today: str) -> tuple[list[dict[str, str]], list[dict[str, str]]]:
    by_edge: dict[tuple[str, str], list[dict[str, str]]] = defaultdict(list)
    for row in rows:
        if clean(row.get("Status")) != "ACTIVE":
            continue
        if clean(row.get("DependencyClass")) != "EXECUTION":
            continue
        if clean(row.get("TargetType")) != "DELIVERABLE":
            continue
        edge = normalized_edge(row)
        if edge is not None:
            by_edge[edge].append(row)

    keep_ids: set[int] = set()
    duplicate_records: list[dict[str, str]] = []
    for (source, target), edge_rows in sorted(by_edge.items()):
        if len(edge_rows) <= 1:
            continue
        ranked = sorted(edge_rows, key=duplicate_retention_key)
        kept = ranked[0]
        keep_ids.add(id(kept))
        kept_id = clean(kept.get("DependencyID"))
        for duplicate in ranked[1:]:
            duplicate_id = clean(duplicate.get("DependencyID"))
            duplicate_records.append(
                {
                    "Source": source,
                    "Target": target,
                    "KeptDependencyID": kept_id,
                    "RetiredDependencyID": duplicate_id,
                    "FromDeliverableID": clean(duplicate.get("FromDeliverableID")),
                    "TargetDeliverableID": clean(duplicate.get("TargetDeliverableID")),
                    "DependencyType": clean(duplicate.get("DependencyType")),
                    "Statement": clean(duplicate.get("Statement")),
                    "Disposition": "RETIRED_IN_DAG007_AGGREGATE_ONLY_DUPLICATE_DIRECTED_EDGE",
                }
            )

    if not duplicate_records:
        return rows, []

    duplicate_ids = {record["RetiredDependencyID"] for record in duplicate_records}
    output: list[dict[str, str]] = []
    for row in rows:
        if clean(row.get("DependencyID")) in duplicate_ids:
            updated = dict(row)
            updated["Status"] = "RETIRED"
            updated["LastSeen"] = today
            kept_id = next(record["KeptDependencyID"] for record in duplicate_records if record["RetiredDependencyID"] == clean(row.get("DependencyID")))
            updated["Notes"] = append_note(
                clean(updated.get("Notes")),
                "dag007_duplicate_directed_edge_retired=true",
                f"duplicate_kept_id={kept_id}",
                "aggregate_only_local_register_unchanged=true",
            )
            output.append(updated)
        else:
            output.append(row)
    return output, duplicate_records


def build_dag007(working_root: Path, repo: Path, today: str) -> dict[str, object]:
    dag006 = working_root / "execution/_DAG/DAG-006"
    dag007 = working_root / "execution/_DAG/DAG-007"
    dag007.mkdir(parents=True, exist_ok=True)

    node_header, node_rows = read_csv(dag006 / "DeliverableNodes.csv")
    fieldnames, canonical_edge_rows, candidate_rows = local_dependency_rows(working_root, today)
    canonical_edge_rows, duplicate_rows = retire_duplicate_active_edges(canonical_edge_rows, today)
    write_csv(dag007 / "DependencyEdges.csv", fieldnames, canonical_edge_rows)
    write_csv(dag007 / "DeliverableNodes.csv", node_header, node_rows)
    write_csv(
        dag007 / "DAG-007_CandidateEdgeWorklist.csv",
        [
            "SourceFile",
            "DependencyID",
            "FromPackageID",
            "FromDeliverableID",
            "OriginalDependencyType",
            "CanonicalDependencyType",
            "TargetPackageID",
            "TargetDeliverableID",
            "Statement",
            "Disposition",
            "Notes",
        ],
        candidate_rows,
    )
    write_csv(
        dag007 / "DAG-007_DuplicateEdgeWorklist.csv",
        [
            "Source",
            "Target",
            "KeptDependencyID",
            "RetiredDependencyID",
            "FromDeliverableID",
            "TargetDeliverableID",
            "DependencyType",
            "Statement",
            "Disposition",
        ],
        duplicate_rows,
    )

    active_rows = active_execution_rows(canonical_edge_rows)
    node_ids = [clean(row.get("DeliverableID")) for row in node_rows if clean(row.get("DeliverableID"))]
    waves = topological_waves(node_ids, active_rows)
    sccs = active_sccs(canonical_edge_rows)
    summary = {
        "node_count": len(node_ids),
        "package_count": len({clean(row.get("PackageID")) for row in node_rows if clean(row.get("PackageID"))}),
        "edge_row_count": len(canonical_edge_rows),
        "active_edge_count": len(active_rows),
        "retired_edge_count": sum(1 for row in canonical_edge_rows if clean(row.get("Status")) == "RETIRED"),
        "candidate_edge_count": len(candidate_rows),
        "duplicate_edge_rows_retired": len(duplicate_rows),
        "active_scc_count": len(sccs),
        "wave_count": len(waves),
    }
    (dag007 / "dag.json").write_text(json.dumps(build_dag_json(node_rows, canonical_edge_rows, summary, today), indent=2) + "\n", encoding="utf-8")
    source_basis = "the refreshed deliverable-local `Dependencies.csv` registers; `DAG-006` supplies the node/path baseline only"
    (dag007 / "TopologicalWaves.md").write_text(render_topological_waves(waves, today, source_basis), encoding="utf-8")
    (dag007 / "Cycle_Report.md").write_text(render_cycle_report(len(active_rows), len(node_ids), len(waves), len(sccs), today, source_basis), encoding="utf-8")
    (dag007 / "DAG-007_APPROVAL_REVIEW_PACKET.md").write_text(render_review_packet(summary, today), encoding="utf-8")
    (dag007 / "APPROVAL_RECORD.md").write_text(render_approval_record_template(summary, today), encoding="utf-8")
    (dag007 / "PROPOSAL_RECORD.md").write_text(
        "\n".join(
            [
                "---",
                "doc_id: DAG-007-PROPOSAL-RECORD",
                "doc_kind: coordination.proposal_record",
                "status: proposed_pending_human_approval",
                f"created: {today}",
                "---",
                "",
                "# DAG-007 Proposal Record",
                "",
                "DAG-007 is proposed after the all-register dependency semantic refresh.",
                "It is built from refreshed deliverable-local dependency registers and holds candidate rows outside the canonical edge register.",
                "`DAG-006` supplies the node/path baseline only; root `_DAG/_LATEST.md` remains unchanged pending human approval.",
                "",
            ]
        ),
        encoding="utf-8",
    )
    (dag007 / "_LATEST.md").write_text(
        "\n".join(
            [
                "# DAG-007 Local Pointer",
                "",
                "- DAG artifact: `DAG-007`",
                "- Status: proposed_pending_human_approval",
                "- Approved graph authority: not yet; root `_DAG/_LATEST.md` remains on `DAG-006` until approval.",
                "",
            ]
        ),
        encoding="utf-8",
    )

    audit_json = dag007 / "DAG_Audit.json"
    audit_md = dag007 / "DAG_Audit.md"
    subprocess.run(
        [
            "python3",
            str(repo / "tools/coordination/audit_dag.py"),
            "--dag-dir",
            str(dag007.relative_to(working_root)),
            "--canonical",
            "--json-out",
            str(audit_json.relative_to(working_root)),
            "--markdown-out",
            str(audit_md.relative_to(working_root)),
        ],
        cwd=working_root,
        check=True,
    )
    return {"dag007": str(dag007), **summary}


def drift_summary(working_root: Path, output_path: Path) -> None:
    fields = [
        "DependencyClass",
        "AnchorType",
        "Direction",
        "DependencyType",
        "TargetType",
        "Explicitness",
        "SatisfactionStatus",
        "Confidence",
        "Origin",
        "Status",
    ]
    allowed = {
        "DependencyClass": {"ANCHOR", "EXECUTION"},
        "AnchorType": {"IMPLEMENTS_NODE", "TRACES_TO_REQUIREMENT", "NOT_APPLICABLE"},
        "Direction": {"UPSTREAM", "DOWNSTREAM"},
        "DependencyType": {"PREREQUISITE", "INTERFACE", "HANDOVER", "CONSTRAINT", "ENABLES", "OTHER"},
        "TargetType": {"DELIVERABLE", "PACKAGE", "WBS_NODE", "REQUIREMENT", "DOCUMENT", "EQUIPMENT", "EXTERNAL", "UNKNOWN"},
        "Explicitness": {"EXPLICIT", "IMPLICIT"},
        "SatisfactionStatus": {"TBD", "PENDING", "IN_PROGRESS", "SATISFIED", "WAIVED", "NOT_APPLICABLE"},
        "Confidence": {"HIGH", "MEDIUM", "LOW"},
        "Origin": {"DECLARED", "EXTRACTED"},
        "Status": {"ACTIVE", "RETIRED"},
    }
    rows: list[dict[str, str]] = []
    paths = local_dependency_paths(working_root) + [working_root / "execution/_DAG/DAG-007/DependencyEdges.csv"]
    for path in paths:
        _header, data = read_csv(path)
        for line, row in enumerate(data, start=2):
            for field in fields:
                value = clean(row.get(field))
                if value not in allowed[field]:
                    rows.append({"File": str(path.relative_to(working_root)), "Line": str(line), "DependencyID": clean(row.get("DependencyID")), "Field": field, "Value": value})
    write_csv(output_path, ["File", "Line", "DependencyID", "Field", "Value"], rows)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--working-root", type=Path, default=Path.cwd())
    parser.add_argument("--today", default=date.today().isoformat())
    parser.add_argument("--local", action="store_true", help="Canonicalize local deliverable registers.")
    parser.add_argument("--dag007", action="store_true", help="Build DAG-007 from refreshed local dependency registers.")
    parser.add_argument("--after-drift", action="store_true", help="Write after_enum_drift.csv evidence.")
    args = parser.parse_args()

    working_root = args.working_root.resolve()
    repo = repo_root(working_root)
    result: dict[str, object] = {}
    if args.local:
        result["local"] = canonicalize_local(working_root, args.today)
    if args.dag007:
        result["dag007"] = build_dag007(working_root, repo, args.today)
    if args.after_drift:
        evidence = working_root / "execution/_Reconciliation/DependencyTypeSystem/TYPE_RECTIFICATION_2026-06-16/Evidence/after_enum_drift.csv"
        drift_summary(working_root, evidence)
        result["after_drift"] = str(evidence)

    print(json.dumps(result, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
