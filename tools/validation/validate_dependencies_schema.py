#!/usr/bin/env python3
"""Validate a dependency CSV file against the OpenPipeStress v3.1 schema."""

from __future__ import annotations

import csv
import sys


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


def validate(csv_path: str) -> tuple[bool, list[str], list[str], int, list[str]]:
    try:
        with open(csv_path, "r", newline="") as handle:
            reader = csv.reader(handle)
            header = next(reader)
            raw_rows = list(reader)
    except FileNotFoundError:
        return False, [f"ERROR: File not found: {csv_path}"], [], 0, []
    except StopIteration:
        return False, [f"ERROR: Empty file: {csv_path}"], [], 0, []
    except csv.Error as exc:
        return False, [f"ERROR: CSV parse error in {csv_path}: {exc}"], [], 0, []

    header = [col.strip().lstrip("\ufeff") for col in header]
    missing = [col for col in REQUIRED_COLUMNS if col not in header]
    extensions = [col for col in header if col not in REQUIRED_COLUMNS]
    row_count = len(raw_rows)
    findings: list[str] = []

    expected_width = len(header)
    for index, row in enumerate(raw_rows, start=2):
        if len(row) != expected_width:
            dependency_id = row[1] if len(row) > 1 else "<missing>"
            findings.append(
                f"Row {index} field count mismatch: expected {expected_width}, "
                f"found {len(row)} (DependencyID={dependency_id})"
            )

    if "RegisterSchemaVersion" in header:
        version_index = header.index("RegisterSchemaVersion")
        for index, row in enumerate(raw_rows, start=2):
            if len(row) <= version_index:
                continue
            if row[version_index].strip() != "v3.1":
                dependency_id = row[1] if len(row) > 1 else "<missing>"
                findings.append(
                    f"Row {index} invalid RegisterSchemaVersion: "
                    f"{row[version_index]!r} (DependencyID={dependency_id})"
                )

    if missing:
        findings.append(f"Missing columns ({len(missing)}): {', '.join(missing)}")

    return not findings, findings, extensions, row_count, header


def main(argv: list[str] | None = None) -> int:
    args = sys.argv[1:] if argv is None else argv
    if len(args) < 1:
        print(f"Usage: {sys.argv[0]} <csv_path>", file=sys.stderr)
        return 1

    csv_path = args[0]
    valid, findings, extensions, row_count, header = validate(csv_path)

    if not valid:
        print(f"INVALID: {csv_path}")
        for finding in findings:
            print(f"  {finding}")
        print(f"  Data rows: {row_count}")
        return 1

    print(f"VALID: {csv_path}")
    print(
        f"  Columns: {len(header)} "
        f"({len(REQUIRED_COLUMNS)} required + {len(extensions)} extension)"
    )
    print(f"  Data rows: {row_count}")
    if extensions:
        print(f"  Extensions: {', '.join(extensions)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
