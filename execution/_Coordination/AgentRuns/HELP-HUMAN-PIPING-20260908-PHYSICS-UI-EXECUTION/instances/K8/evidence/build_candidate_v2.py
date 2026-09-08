#!/usr/bin/env python3
"""Build the additive K8 V2 candidate and a full in-memory validation witness."""

from __future__ import annotations

import copy
import csv
import hashlib
import importlib.util
import json
import subprocess
from pathlib import Path

from jsonschema import Draft202012Validator

from semantic_validator_v2 import validate_writer


HERE = Path(__file__).resolve().parent


def load_v1():
    spec = importlib.util.spec_from_file_location("k8_v1_builder", HERE / "build_candidate.py")
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def roots() -> tuple[Path, Path]:
    repo = Path(subprocess.check_output(["git", "rev-parse", "--show-toplevel"], text=True).strip())
    return repo, repo / "projects/chirality-piping"


def load_mapping(v1) -> list[dict[str, str]]:
    rows = list(csv.DictReader((HERE / "SOURCE_KIND_MAPPING.csv").open(encoding="utf-8")))
    for row in rows:
        row["owner_semantics"] = v1.owner_for(row["semantic_status"])
        row["canonical_family"] = row.pop("family")
        row["canonical_dimension"] = row.pop("dimension")
    return rows


def build_schema(rows: list[dict[str, str]]) -> dict:
    schema = json.loads((HERE / "RESULTS_SCHEMA_0_2_0_CANDIDATE.json").read_text())
    schema["$comment"] = "K8 candidate revision V2; public schema version remains candidate 0.2.0."
    schema["description"] = (
        "Route-A candidate revision V2. Native source_record controls, runtime semantic "
        "validation is mandatory, and source-specific or pending rows are not standardized mechanics."
    )
    schema["$defs"]["QuantityResult"]["properties"]["source_result_refs"] = {
        "type": "array",
        "items": {"$ref": "#/$defs/Id"},
        "description": "Ordered compatibility projection equal in presence and value to source_record.source_result_refs; enforced by the mandatory runtime validator.",
    }
    branches = schema["$defs"]["QuantityResult"]["allOf"][-1]["oneOf"]
    by_pair = {(r["kind"], r["unit"]): r for r in rows}
    assert len(branches) == len(by_pair) == 45
    for branch in branches:
        source_props = branch["properties"]["source_record"]["properties"]
        pair = (source_props["kind"]["const"], source_props["unit"]["const"])
        row = by_pair[pair]
        branch["properties"]["classification"]["properties"]["owner_semantics"] = {
            "const": row["owner_semantics"]
        }
        branch["properties"]["classification"]["required"].append("owner_semantics")
    Draft202012Validator.check_schema(schema)
    return schema


def build_full_nonlinear(v1, rows: list[dict[str, str]]) -> dict:
    _, working = roots()
    prior = working / "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-PHYSICS-AUDIT/instances/P5/compatibility_v1/author/nonlinear_canonical_document.json"
    raw_path = working / "fixtures/product_preview/invented_mechanics_result.json"
    document = json.loads(prior.read_text())
    raw = json.loads(raw_path.read_text())
    envelope = document["result_envelope"]
    result_set = envelope["result_sets"][0]
    existing = {value["result_id"]: value for item in envelope["result_sets"] for value in item["values"]}
    by_pair = {(r["kind"], r["unit"]): r for r in rows}
    migrated = []
    for source in raw["results"]:
        mapped = by_pair[(source["kind"], source["unit"])]
        value = copy.deepcopy(existing.get(source["id"], {}))
        basis_ref = copy.deepcopy(source.get("basis_ref", result_set["basis_ref"]))
        if not value:
            value = {
                "result_id": source["id"],
                "object_ref": {"ref_type": "preview_entity", "ref_id": source["entity_ref"]},
                "basis_ref": basis_ref,
                "magnitude": source["value"],
                "unit": source["unit"],
                "provenance": copy.deepcopy(envelope["provenance"]),
            }
        value["result_id"] = source["id"]
        value["object_ref"] = {"ref_type": "preview_entity", "ref_id": source["entity_ref"]}
        value["basis_ref"] = basis_ref
        value["magnitude"] = source["value"]
        value["unit"] = source["unit"]
        value["family"] = mapped["canonical_family"]
        value["dimension"] = mapped["canonical_dimension"]
        value["source_record"] = copy.deepcopy(source)
        value["classification"] = {
            "source_family": mapped["source_family"],
            "semantic_status": mapped["semantic_status"],
            "canonical_family": mapped["canonical_family"],
            "canonical_dimension": mapped["canonical_dimension"],
            "owner_semantics": mapped["owner_semantics"],
            "notes": "Transport mapping only; row semantic_status controls interpretation regardless of result-set type.",
        }
        if "metadata" in source:
            value["metadata"] = copy.deepcopy(source["metadata"])
        else:
            value.pop("metadata", None)
        if "source_result_refs" in source:
            value["source_result_refs"] = copy.deepcopy(source["source_result_refs"])
        else:
            value.pop("source_result_refs", None)
        migrated.append(value)
    result_set["values"] = migrated
    document["schema_version"] = "0.2.0"
    envelope["schema_version"] = "0.2.0"
    envelope["version_contract"] = {
        "schema_id": "https://openpipestress.org/schemas/results/0.2.0/results.schema.json",
        "document_version": "0.2.0",
        "minimum_reader_version": "0.2.0",
        "unknown_version_policy": "reject_unsupported_schema_version",
    }
    for checksum in [envelope["reproducibility"]["model_hash"], *envelope["reproducibility"]["run_hashes"]]:
        checksum["canonicalization_claim"] = {
            "JCS": "rfc8785_conformant",
            "rfc8785_jcs": "implementation_label_only_conformance_not_established",
            "NONE": "no_canonicalization",
            "TBD": "undetermined",
        }[checksum["canonicalization"]]
    assert len(migrated) == len(raw["results"]) == 830
    assert all(value["source_record"] == source for value, source in zip(migrated, raw["results"]))
    return document


def artifacts() -> tuple[dict, list[dict[str, str]], dict]:
    v1 = load_v1()
    rows = load_mapping(v1)
    return build_schema(rows), rows, build_full_nonlinear(v1, rows)


def main() -> None:
    schema, rows, witness = artifacts()
    schema_path = HERE / "RESULTS_SCHEMA_0_2_0_CANDIDATE_V2.json"
    schema_path.write_text(json.dumps(schema, indent=2) + "\n")
    with (HERE / "SOURCE_KIND_MAPPING_V2.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=[
            "kind", "unit", "source_family", "canonical_family", "canonical_dimension",
            "semantic_status", "owner_semantics",
        ], lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    mapping = {(r["kind"], r["unit"]): r for r in rows}
    payload, errors = validate_writer(witness, schema, mapping)
    assert payload is not None and not errors
    result = {
        "status": "PASS_CANDIDATE_V2_NOT_ADOPTED",
        "candidate_public_version": "0.2.0",
        "candidate_revision": "K8-V2",
        "schema_sha256": sha256(schema_path),
        "mapping_rows": len(rows),
        "full_native_nonlinear_rows": 830,
        "mandatory_semantic_validator": "semantic_validator_v2.py",
        "writer_reader_round_trip": "PASS",
        "source_record_json_value_equality": "830/830",
        "pressure_pending_transport": "PERMITTED_CANDIDATE_PRESERVE_ONLY",
        "pressure_standardization": "PKG04_05_ACCEPTANCE_REQUIRED",
    }
    (HERE / "COVERAGE_VALIDATION_V2.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result))


if __name__ == "__main__":
    main()
