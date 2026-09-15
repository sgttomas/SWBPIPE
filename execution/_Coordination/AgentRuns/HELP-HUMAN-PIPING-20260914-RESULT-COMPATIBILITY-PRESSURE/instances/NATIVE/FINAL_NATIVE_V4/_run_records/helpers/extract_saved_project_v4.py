#!/usr/bin/env python3
import argparse
import hashlib
import json
import pathlib
import sqlite3


FIELDS = [
    "model_json",
    "editor_intents_json",
    "proposal_json",
    "selected_review_target_json",
    "mechanics_result_json",
    "analysis_run_json",
    "model_hash_json",
    "project_envelope_hash_json",
    "model_migration_ledger_json",
]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--db", required=True)
    parser.add_argument("--project-id", required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()
    out = pathlib.Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    db_uri = pathlib.Path(args.db).resolve().as_uri() + "?mode=ro"
    con = sqlite3.connect(db_uri, uri=True)
    con.row_factory = sqlite3.Row
    cols = ",".join(["project_id", "project_name", "created_at_unix", "updated_at_unix", *FIELDS])
    row = con.execute(f"SELECT {cols} FROM local_projects WHERE project_id=?", (args.project_id,)).fetchone()
    if row is None:
        raise SystemExit("project row not found")
    summary = {
        "project_id": row["project_id"],
        "project_name": row["project_name"],
        "created_at_unix": row["created_at_unix"],
        "updated_at_unix": row["updated_at_unix"],
        "fields": {},
    }
    for field in FIELDS:
        raw = row[field].encode("utf-8")
        target = out / f"{field}.json"
        target.write_bytes(raw)
        summary["fields"][field] = {
            "file": target.name,
            "byte_count": len(raw),
            "sha256": hashlib.sha256(raw).hexdigest(),
        }
    mechanics = json.loads(row["mechanics_result_json"])
    results = mechanics.get("results", [])
    summary["mechanics"] = {
        "run_id": mechanics.get("run_id"),
        "model_ref": mechanics.get("model_ref"),
        "status": mechanics.get("status"),
        "result_count": len(results),
        "rows": [
            {
                "index": index,
                "id": item.get("id"),
                "kind": item.get("kind"),
                "value": item.get("value"),
                "unit": item.get("unit"),
                "basis_ref": item.get("basis_ref"),
                "component": item.get("component"),
                "location": item.get("location"),
            }
            for index, item in enumerate(results)
        ],
    }
    encoded = json.dumps(summary, indent=2, ensure_ascii=False).encode("utf-8") + b"\n"
    (out / "extraction_summary.json").write_bytes(encoded)


if __name__ == "__main__":
    main()
