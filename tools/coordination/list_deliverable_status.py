#!/usr/bin/env python3
"""List deliverable-local lifecycle status with optional DAG node context.

This is a read-only coordination helper. It discovers deliverables from local
`_STATUS.md` files and joins to the selected DAG's `DeliverableNodes.csv` for
node presence and path context when available. Local `_STATUS.md` values are
the lifecycle source of truth for work selection. A `## Remaining` section in
`_STATUS.md`, when present, records the deliverable's open scope as top-level
bullets; its bullet count is reported as `RemainingItems` (absent section = 0).
"""

from __future__ import annotations

import argparse
import csv
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


ROOT = Path(__file__).resolve().parents[2]
CANONICAL_STATES = {"OPEN", "INITIALIZED", "IN_PROGRESS", "CHECKING", "ISSUED"}
TOLERATED_NONSTANDARD_STATES = {"SEMANTIC_READY"}
STATUS_RE = re.compile(r"^\*\*Current State:\*\*\s*(?P<status>\S+)\s*$")
UPDATED_RE = re.compile(r"^\*\*Last Updated:\*\*\s*(?P<date>.+?)\s*$")
PACKAGE_RE = re.compile(r"^(?P<package_id>PKG-\d+)")
DELIVERABLE_RE = re.compile(r"^(?P<deliverable_id>DEL-\d+-\d+)(?:_(?P<name>.*))?$")


HEADER = [
    "DeliverableID",
    "PackageID",
    "DeliverableName",
    "LocalStatus",
    "LastUpdated",
    "StatusVocabulary",
    "RemainingItems",
    "DeliverablePath",
    "StatusPath",
    "DAG",
    "DAGNodePresent",
    "DAGExecutionPath",
]


@dataclass(frozen=True)
class LocalStatus:
    deliverable_id: str
    package_id: str
    deliverable_name: str
    status: str
    last_updated: str
    remaining_items: int
    deliverable_path: Path
    status_path: Path


def latest_dag(root: Path) -> str:
    latest = root / "execution/_DAG/_LATEST.md"
    for line in latest.read_text(encoding="utf-8").splitlines():
        if line.startswith("- Approved graph authority:"):
            value = line.split(":", maxsplit=1)[1].strip().strip("`").strip("/")
            return Path(value).name
    raise ValueError(f"approved graph authority not found in {latest}")


def rel(path: Path, root: Path) -> str:
    try:
        return path.relative_to(root).as_posix()
    except ValueError:
        return path.as_posix()


def package_id_from_path(path: Path) -> str:
    for part in path.parts:
        match = PACKAGE_RE.match(part)
        if match:
            return match.group("package_id")
    return ""


def deliverable_parts_from_path(path: Path) -> tuple[str, str]:
    for part in path.parts:
        match = DELIVERABLE_RE.match(part)
        if match:
            name = (match.group("name") or "").replace("_", " ").strip()
            return match.group("deliverable_id"), name
    return "", ""


def parse_status_file(path: Path) -> LocalStatus:
    status = ""
    last_updated = ""
    remaining_items = 0
    in_remaining = False
    for line in path.read_text(encoding="utf-8").splitlines():
        status_match = STATUS_RE.match(line)
        if status_match:
            status = status_match.group("status").strip()
            continue
        updated_match = UPDATED_RE.match(line)
        if updated_match:
            last_updated = updated_match.group("date").strip()
            continue
        if line.startswith("## "):
            in_remaining = line.strip() == "## Remaining"
            continue
        if in_remaining and line.startswith("- "):
            remaining_items += 1

    deliverable_path = path.parent
    package_id = package_id_from_path(deliverable_path)
    deliverable_id, deliverable_name = deliverable_parts_from_path(deliverable_path)
    return LocalStatus(
        deliverable_id=deliverable_id,
        package_id=package_id,
        deliverable_name=deliverable_name,
        status=status,
        last_updated=last_updated,
        remaining_items=remaining_items,
        deliverable_path=deliverable_path,
        status_path=path,
    )


def discover_statuses(root: Path) -> list[LocalStatus]:
    paths = sorted(root.glob("execution/PKG-*/1_Working/DEL-*/_STATUS.md"))
    return [parse_status_file(path) for path in paths]


def read_dag_nodes(root: Path, dag: str) -> dict[str, dict[str, str]]:
    path = root / "execution" / "_DAG" / dag / "DeliverableNodes.csv"
    with path.open(newline="", encoding="utf-8") as handle:
        reader = csv.DictReader(handle)
        if reader.fieldnames is None:
            raise ValueError(f"empty DAG node CSV: {path}")
        return {row.get("DeliverableID", ""): row for row in reader}


def status_vocabulary(status: str) -> str:
    if not status:
        return "MISSING"
    if status in CANONICAL_STATES:
        return "CANONICAL"
    if status in TOLERATED_NONSTANDARD_STATES:
        return "NONSTANDARD_TOLERATED"
    return "NONSTANDARD"


def rows(root: Path, dag: str, statuses: Iterable[LocalStatus]) -> list[dict[str, str]]:
    dag_nodes = read_dag_nodes(root, dag)
    output_rows: list[dict[str, str]] = []
    for item in sorted(statuses, key=lambda row: (row.package_id, row.deliverable_id)):
        node = dag_nodes.get(item.deliverable_id, {})
        output_rows.append(
            {
                "DeliverableID": item.deliverable_id,
                "PackageID": item.package_id,
                "DeliverableName": item.deliverable_name
                or node.get("DeliverableName", ""),
                "LocalStatus": item.status,
                "LastUpdated": item.last_updated,
                "StatusVocabulary": status_vocabulary(item.status),
                "RemainingItems": str(item.remaining_items),
                "DeliverablePath": rel(item.deliverable_path, root),
                "StatusPath": rel(item.status_path, root),
                "DAG": dag,
                "DAGNodePresent": "TRUE" if node else "FALSE",
                "DAGExecutionPath": node.get("ExecutionPath", ""),
            }
        )
    return output_rows


def filter_rows(
    output_rows: Iterable[dict[str, str]],
    statuses: set[str],
    exclude_issued: bool,
) -> list[dict[str, str]]:
    filtered = []
    for row in output_rows:
        status = row["LocalStatus"]
        if statuses and status not in statuses:
            continue
        if exclude_issued and status == "ISSUED":
            continue
        filtered.append(row)
    return filtered


def write_csv(output_rows: list[dict[str, str]]) -> None:
    writer = csv.DictWriter(sys.stdout, fieldnames=HEADER, lineterminator="\n")
    writer.writeheader()
    writer.writerows(output_rows)


def write_table(output_rows: list[dict[str, str]]) -> None:
    columns = [
        "DeliverableID",
        "PackageID",
        "LocalStatus",
        "StatusVocabulary",
        "RemainingItems",
        "DAGNodePresent",
        "DeliverablePath",
    ]
    widths = {
        column: max(len(column), *(len(row[column]) for row in output_rows))
        if output_rows
        else len(column)
        for column in columns
    }
    print("  ".join(column.ljust(widths[column]) for column in columns))
    print("  ".join("-" * widths[column] for column in columns))
    for row in output_rows:
        print("  ".join(row[column].ljust(widths[column]) for column in columns))


def write_markdown(output_rows: list[dict[str, str]]) -> None:
    columns = [
        "DeliverableID",
        "PackageID",
        "LocalStatus",
        "StatusVocabulary",
        "RemainingItems",
        "DAGNodePresent",
        "DeliverablePath",
    ]
    print("| " + " | ".join(columns) + " |")
    print("| " + " | ".join("---" for _ in columns) + " |")
    for row in output_rows:
        print("| " + " | ".join(row[column] for column in columns) + " |")


def print_summary(output_rows: list[dict[str, str]]) -> None:
    counts: dict[str, int] = {}
    for row in output_rows:
        counts[row["LocalStatus"]] = counts.get(row["LocalStatus"], 0) + 1
    status_counts = ", ".join(f"{key}={counts[key]}" for key in sorted(counts))
    remaining_total = sum(int(row["RemainingItems"]) for row in output_rows)
    remaining_deliverables = sum(
        1 for row in output_rows if int(row["RemainingItems"]) > 0
    )
    print(f"Rows: {len(output_rows)}", file=sys.stderr)
    print(f"Status counts: {status_counts}", file=sys.stderr)
    print(
        f"Remaining items: {remaining_total} across "
        f"{remaining_deliverables} deliverables",
        file=sys.stderr,
    )


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="List deliverable-local _STATUS.md values with DAG context."
    )
    parser.add_argument("--repo-root", default=str(ROOT), help="Repository root.")
    parser.add_argument("--dag", default=None, help="DAG folder name, for example DAG-006.")
    parser.add_argument(
        "--format",
        choices=("table", "csv", "markdown"),
        default="table",
        help="Output format.",
    )
    parser.add_argument(
        "--status",
        action="append",
        default=[],
        help="Filter by local status. Repeatable.",
    )
    parser.add_argument(
        "--exclude-issued",
        action="store_true",
        help="Exclude ISSUED deliverables from the report.",
    )
    parser.add_argument(
        "--summary",
        action="store_true",
        help="Print row count and local status counts to stderr.",
    )
    return parser.parse_args(argv)


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    root = Path(args.repo_root).resolve()
    dag = args.dag or latest_dag(root)
    output_rows = rows(root, dag, discover_statuses(root))
    output_rows = filter_rows(output_rows, set(args.status), args.exclude_issued)

    if args.format == "csv":
        write_csv(output_rows)
    elif args.format == "markdown":
        write_markdown(output_rows)
    else:
        write_table(output_rows)

    if args.summary:
        print_summary(output_rows)
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
