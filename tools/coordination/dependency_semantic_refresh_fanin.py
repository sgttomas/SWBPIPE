#!/usr/bin/env python3
"""Fan-in helpers for the dependency semantic refresh tranche."""

from __future__ import annotations

import argparse
import csv
import json
import subprocess
from pathlib import Path


def dependency_registers(working_root: Path) -> list[Path]:
    return sorted(
        path
        for path in (working_root / "execution").rglob("Dependencies.csv")
        if "_run_records" not in path.parts
    )


def read_rows(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8-sig") as handle:
        return list(csv.DictReader(handle))


def write_csv(path: Path, rows: list[dict[str, object]], fields: list[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=fields)
        writer.writeheader()
        writer.writerows(rows)


def inventory(working_root: Path, output_dir: Path, label: str) -> dict[str, object]:
    register_rows: list[dict[str, object]] = []
    total_rows = 0
    active_rows = 0
    retired_rows = 0
    candidate_rows = 0
    active_del00_targets = 0

    for path in dependency_registers(working_root):
        rows = read_rows(path)
        counts = {
            "file": str(path.relative_to(working_root)),
            "rows": len(rows),
            "active": 0,
            "retired": 0,
            "candidate": 0,
            "active_del00_targets": 0,
        }
        for row in rows:
            total_rows += 1
            status = row.get("Status", "").strip()
            if status == "ACTIVE":
                active_rows += 1
                counts["active"] += 1
            if status == "RETIRED":
                retired_rows += 1
                counts["retired"] += 1
            if status == "CANDIDATE":
                candidate_rows += 1
                counts["candidate"] += 1
            if status == "ACTIVE" and row.get("TargetDeliverableID", "").startswith("DEL-00-"):
                active_del00_targets += 1
                counts["active_del00_targets"] += 1
        register_rows.append(counts)

    write_csv(
        output_dir / f"register_inventory_{label}.csv",
        register_rows,
        ["file", "rows", "active", "retired", "candidate", "active_del00_targets"],
    )

    summary = {
        "dependency_registers": len(register_rows),
        "rows": total_rows,
        "active": active_rows,
        "retired": retired_rows,
        "candidate_status_rows": candidate_rows,
        "active_del00_target_rows": active_del00_targets,
    }
    (output_dir / f"summary_{label}.json").write_text(json.dumps(summary, indent=2) + "\n")
    return summary


def validate_all(working_root: Path, output_dir: Path) -> int:
    rows: list[dict[str, object]] = []
    failures = 0
    validator = working_root / "tools/validation/validate_dependencies_schema.py"
    for path in dependency_registers(working_root):
        result = subprocess.run(
            ["python3", str(validator), str(path)],
            cwd=working_root,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            check=False,
        )
        ok = result.returncode == 0
        if not ok:
            failures += 1
        rows.append(
            {
                "file": str(path.relative_to(working_root)),
                "valid": str(ok),
                "returncode": result.returncode,
                "output": result.stdout.strip().replace("\n", "\\n"),
            }
        )
    write_csv(output_dir / "validation_results.csv", rows, ["file", "valid", "returncode", "output"])
    return failures


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--working-root", required=True)
    parser.add_argument("--output-dir", required=True)
    parser.add_argument("--label", default="after")
    parser.add_argument("--validate", action="store_true")
    args = parser.parse_args()

    working_root = Path(args.working_root).resolve()
    output_dir = Path(args.output_dir).resolve()
    output_dir.mkdir(parents=True, exist_ok=True)

    summary = inventory(working_root, output_dir, args.label)
    failures = validate_all(working_root, output_dir) if args.validate else 0
    summary["validation_failures"] = failures
    print(json.dumps(summary, indent=2))
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
