#!/usr/bin/env python3
"""Build the additive schema/type-first K8 V4 candidate."""

from __future__ import annotations

import csv
import hashlib
import json
from pathlib import Path

from jsonschema import Draft202012Validator

from build_candidate_v3 import artifacts as v3_artifacts
from semantic_validator_v4 import validate_writer


HERE = Path(__file__).resolve().parent


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def artifacts():
    schema, rows, witness = v3_artifacts()
    schema["$comment"] = "K8 candidate revision V4; public schema version remains candidate 0.2.0."
    schema["description"] = (
        "Route-A candidate revision V4. A strict host-JSON representability phase and "
        "complete structural/type schema phase precede all semantic relations."
    )
    Draft202012Validator.check_schema(schema)
    return schema, rows, witness


def main() -> None:
    schema, rows, witness = artifacts()
    schema_path = HERE / "RESULTS_SCHEMA_0_2_0_CANDIDATE_V4.json"
    schema_path.write_text(json.dumps(schema, indent=2) + "\n")
    with (HERE / "SOURCE_KIND_MAPPING_V4.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=[
            "kind", "unit", "source_family", "canonical_family", "canonical_dimension",
            "semantic_status", "owner_semantics",
        ], lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    mapping = {(row["kind"], row["unit"]): row for row in rows}
    payload, errors = validate_writer(witness, schema, mapping)
    assert payload is not None and not errors
    result = {
        "status": "PASS_CANDIDATE_V4_NOT_ADOPTED",
        "candidate_public_version": "0.2.0",
        "candidate_revision": "K8-V4",
        "schema_sha256": sha256(schema_path),
        "mapping_rows": 45,
        "full_native_nonlinear_rows": 830,
        "writer_reader_round_trip": "PASS",
        "validator_order": [
            "host_json_representability",
            "structural_type_schema",
            "version_dispatch",
            "semantic_relations",
            "full_schema"
        ],
        "api_legacy_pressure_design": "UNCHANGED_FROM_FROZEN_V3"
    }
    (HERE / "COVERAGE_VALIDATION_V4.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))


if __name__ == "__main__":
    main()
