#!/usr/bin/env python3
"""Read-only comparison of Piping's local Dependencies.csv files with a
materializer rerun from the accepted DAG.

The accepted DAG version and every deliverable folder's Dependencies.csv and
_DEPENDENCIES.md are copied to a temporary directory; the Root
materializer runs there in default and --canonical-output modes, with
--refresh-pointers, and each rewritten Dependencies.csv is compared with the
committed file. The committed tree is never written. The script also compares
the local rows with the accepted DAG's edge rows directly.

Usage, from the repository root:
    python3 projects/chirality-piping/execution/_Coordination/WorkGraphs/PIPING_DEP_MATERIALIZATION_20260927/evidence/compare_materializer.py [--tool-revision REV]

With --tool-revision, the materializer and its audit_dag.py module are taken
from that Git revision instead of the working tree. The output is
byte-reproducible for the same inputs and tool.
"""

from __future__ import annotations

import argparse
import csv
import io
import json
import shutil
import subprocess
import sys
import tempfile
from collections import Counter
from pathlib import Path

REPO = Path.cwd()
PROJECT = REPO / "projects/chirality-piping"
EXECUTION = PROJECT / "execution"
TOOL = REPO / "tools/coordination/materialize_local_dependencies.py"
REGISTERS = "PKG-*/1_Working/DEL-*/Dependencies.csv"
EXTENSION = ["EstimateImpactClass", "ConsumerHint"]


def parse(data: bytes) -> tuple[list[str], list[dict[str, str]]]:
    reader = csv.DictReader(io.StringIO(data.decode("utf-8-sig"), newline=""))
    rows = list(reader)
    return list(reader.fieldnames or []), rows


def accepted_dag() -> Path:
    for line in (EXECUTION / "_DAG/_LATEST.md").read_text(encoding="utf-8").splitlines():
        if line.startswith("- Latest DAG artifact path:"):
            return PROJECT / line.split("`")[1]
    raise SystemExit("no accepted DAG path in _DAG/_LATEST.md")


def classify(scratch: Path) -> dict[str, object]:
    files: Counter[str] = Counter()
    rows: Counter[str] = Counter()
    examples: dict[str, list[str]] = {}
    unchanged: list[str] = []
    for local in sorted(EXECUTION.glob(REGISTERS)):
        if "/PKG-00_" in local.as_posix():
            continue
        out = scratch / local.relative_to(EXECUTION)
        deliverable = local.parent.name.split("_")[0]
        before, after = local.read_bytes(), out.read_bytes()
        if before == after:
            unchanged.append(deliverable)
            continue
        files["changed"] += 1
        old_header, old_rows = parse(before)
        new_header, new_rows = parse(after)
        kinds = set()
        if old_header != new_header:
            kinds.add("header_adds_extension_columns" if new_header == old_header + EXTENSION else "header_other")
        old_by_id = {row["DependencyID"]: row for row in old_rows}
        new_by_id = {row["DependencyID"]: row for row in new_rows}
        for key, row in old_by_id.items():
            if key not in new_by_id:
                kind = f"row_dropped_{row.get('Origin', '')}_{row.get('Status', '')}"
                rows[kind] += 1
                kinds.add(kind)
                examples.setdefault(kind, []).append(f"{deliverable}:{key}")
        for key in sorted(new_by_id.keys() - old_by_id.keys()):
            rows["row_added"] += 1
            kinds.add("row_added")
        for key in sorted(old_by_id.keys() & new_by_id.keys()):
            old, new = old_by_id[key], new_by_id[key]
            columns = sorted(c for c in set(old) | set(new) if c and (old.get(c) or "") != (new.get(c) or ""))
            if not columns:
                continue
            old_notes, new_notes = old.get("Notes") or "", new.get("Notes") or ""
            if columns == ["Notes"] and new_notes.startswith(old_notes) and new_notes[len(old_notes):].lstrip("; ").startswith("legacy_"):
                kind = "notes_gain_aggregate_legacy_suffix"
            else:
                kind = "fields:" + ",".join(sorted(columns))
            rows[kind] += 1
            kinds.add(kind)
            examples.setdefault(kind, []).append(f"{deliverable}:{key}")
        common_old = [k for k in old_by_id if k in new_by_id]
        common_new = [k for k in new_by_id if k in old_by_id]
        if common_old != common_new:
            kinds.add("row_order")
        for kind in kinds:
            files[kind] += 1
    return {
        "changed_files_by_kind": dict(sorted(files.items())),
        "rows_by_kind": dict(sorted(rows.items())),
        "unchanged_files": unchanged,
        "examples": {k: v[:3] for k, v in sorted(examples.items())},
    }


def direct_comparison(dag_dir: Path) -> dict[str, object]:
    _header, dag_rows = parse((dag_dir / "DependencyEdges.csv").read_bytes())
    dag = {row["DependencyID"]: row for row in dag_rows}
    local: dict[str, dict[str, str]] = {}
    widths: Counter[int] = Counter()
    for path in sorted(EXECUTION.glob(REGISTERS)):
        if "/PKG-00_" in path.as_posix():
            continue
        header, rows = parse(path.read_bytes())
        widths[len(header)] += 1
        for row in rows:
            local[row["DependencyID"]] = row
    kinds: Counter[str] = Counter()
    for key in local.keys() & dag.keys():
        a, b = local[key], dag[key]
        core = [c for c in a if c and c != "Notes" and (a.get(c) or "") != (b.get(c) or "")]
        if core:
            kinds["core_field_differs"] += 1
        elif (a.get("Notes") or "") == (b.get("Notes") or ""):
            kinds["identical"] += 1
        elif (b.get("Notes") or "").startswith(a.get("Notes") or ""):
            kinds["dag_notes_extend_local_notes"] += 1
        else:
            kinds["notes_other"] += 1
    return {
        "dag_rows": len(dag),
        "dag_status": dict(Counter(row["Status"] for row in dag_rows)),
        "local_registers": sum(widths.values()),
        "local_header_widths": {str(k): v for k, v in sorted(widths.items())},
        "local_rows": len(local),
        "local_origin_status": {f"{k[0]}/{k[1]}": v for k, v in sorted(Counter((r.get("Origin"), r.get("Status")) for r in local.values()).items())},
        "only_local": sorted(local.keys() - dag.keys()),
        "only_dag": sorted(dag.keys() - local.keys()),
        "shared_row_kinds": dict(sorted(kinds.items())),
        "dag_rows_with_extension_values": sorted(k for k, r in dag.items() if any(r.get(c) for c in EXTENSION)),
    }


def copy_inputs(dag_dir: Path, scratch: Path) -> None:
    """Copy only what the materializer reads and writes: the accepted DAG
    version and each deliverable folder's two dependency files."""
    shutil.copytree(dag_dir, scratch / dag_dir.relative_to(EXECUTION))
    for folder in EXECUTION.glob("PKG-*/1_Working/DEL-*"):
        if not folder.is_dir():
            continue
        target = scratch / folder.relative_to(EXECUTION)
        target.mkdir(parents=True, exist_ok=True)
        for name in ("Dependencies.csv", "_DEPENDENCIES.md"):
            if (folder / name).is_file():
                shutil.copy2(folder / name, target / name)


def tool_at(revision: str | None, directory: Path) -> Path:
    """The materializer to run: the working tree's, or one taken from a Git revision."""
    if revision is None:
        return TOOL
    directory.mkdir(parents=True, exist_ok=True)
    for name in ("materialize_local_dependencies.py", "audit_dag.py"):
        data = subprocess.run(
            ["git", "show", f"{revision}:tools/coordination/{name}"], cwd=REPO, capture_output=True, check=True
        ).stdout
        (directory / name).write_bytes(data)
    return directory / "materialize_local_dependencies.py"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--tool-revision", help="Git revision to take the materializer from (default: working tree).")
    args = parser.parse_args()
    dag_dir = accepted_dag()
    result: dict[str, object] = {"accepted_dag": dag_dir.relative_to(REPO).as_posix(), "tool_revision": args.tool_revision}
    for mode, extra in (("default", []), ("canonical_output", ["--canonical-output"])):
        with tempfile.TemporaryDirectory() as tmp:
            scratch = Path(tmp) / "execution"
            copy_inputs(dag_dir, scratch)
            tool = tool_at(args.tool_revision, Path(tmp) / "tool")
            run = subprocess.run(
                [sys.executable, str(tool), "--dag-dir", str(scratch / dag_dir.relative_to(EXECUTION)),
                 "--execution-root", str(scratch), "--refresh-pointers", "--json-out", str(Path(tmp) / "summary.json"), *extra],
                capture_output=True, text=True, check=False,
            )
            summary = json.loads((Path(tmp) / "summary.json").read_text(encoding="utf-8"))
            result[mode] = {
                "exit_code": run.returncode,
                "console": run.stdout.splitlines(),
                "written_count": summary["written_count"],
                "skipped_status_counts": summary["skipped_status_counts"],
                "declared_id_collisions": {w["DeliverableID"]: w["DeclaredIdCollisions"] for w in summary["written"] if w["DeclaredIdCollisions"]},
                "set_aside_declared_rows": {w["DeliverableID"]: w["SetAsideDeclaredRows"] for w in summary["written"] if w["SetAsideDeclaredRows"]},
                # Summary fields added by later tool revisions; absent from earlier ones.
                "retired_from_aggregate_rows": {
                    w["DeliverableID"]: w["RetiredFromAggregateRows"] for w in summary["written"] if w.get("RetiredFromAggregateRows")
                },
                "dropped_local_rows": {
                    w["DeliverableID"]: w["DroppedLocalRows"] for w in summary["written"] if w.get("DroppedLocalRows")
                },
                **classify(scratch),
            }
    result["direct_local_vs_accepted_dag"] = direct_comparison(dag_dir)
    print(json.dumps(result, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
