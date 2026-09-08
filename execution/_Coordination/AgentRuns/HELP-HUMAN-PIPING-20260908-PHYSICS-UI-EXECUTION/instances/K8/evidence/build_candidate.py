#!/usr/bin/env python3
"""Build and validate the K8 route-A schema candidate without product writes."""

from __future__ import annotations

import copy
import csv
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

from jsonschema import Draft202012Validator


VERSION = "0.2.0"
DISCLOSURE = "HEADLESS_RUNNER_ENVELOPE_VOCABULARY_BOUNDARY_ROW"


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def repo_root() -> Path:
    return Path(
        subprocess.check_output(
            ["git", "rev-parse", "--show-toplevel"], text=True
        ).strip()
    )


def mapping_for(kind: str, unit: str) -> dict[str, str]:
    element = {
        "element_local_axial_force": ("force", "force"),
        "element_local_shear_force_y": ("force", "force"),
        "element_local_shear_force_z": ("force", "force"),
        "element_local_bending_moment_y": ("moment", "moment"),
        "element_local_bending_moment_z": ("moment", "moment"),
        "element_local_torsional_moment": ("moment", "moment"),
        "element_local_axial_normal_stress": ("stress", "stress"),
        "element_local_bending_normal_stress_y": ("stress", "stress"),
        "element_local_bending_normal_stress_z": ("stress", "stress"),
        "element_local_torsional_shear_stress": ("stress", "stress"),
        "open_formula_stress_summary": ("stress", "stress"),
    }
    if kind in element:
        family, dimension = element[kind]
        return dict(source_family="element_response", family=family, dimension=dimension,
                    semantic_status="existing_canonical_category")
    if kind == "displacement_magnitude":
        return dict(source_family="nodal_kinematic", family="displacement", dimension="length",
                    semantic_status="existing_canonical_category")
    if kind.startswith("global_nodal_displacement_"):
        return dict(source_family="nodal_kinematic", family="displacement", dimension="length",
                    semantic_status="source_specific_metadata_standardized")
    if kind.startswith("global_nodal_rotation_"):
        return dict(source_family="nodal_kinematic", family="rotation", dimension="angle",
                    semantic_status="source_specific_metadata_standardized")
    if kind == "reaction_resultant":
        return dict(source_family="support_response", family="reaction", dimension="force",
                    semantic_status="existing_canonical_category")
    if kind == "pipe_section_pressure_hoop_stress":
        return dict(source_family="pressure_response", family="stress", dimension="stress",
                    semantic_status="existing_pressure_source_label_preserved_pending_P5")
    if kind == "pipe_section_pressure_longitudinal_stress":
        return dict(source_family="pressure_response", family="stress", dimension="stress",
                    semantic_status="source_specific_unstandardized_pending_P5")
    if kind == "expansion_joint_pressure_thrust_load_review":
        return dict(source_family="pressure_response", family="force", dimension="force",
                    semantic_status="source_specific_unstandardized_pending_P5")
    if kind == "component_user_stress_multiplier_review":
        return dict(source_family="user_review", family="stress", dimension="stress",
                    semantic_status="source_specific_unstandardized")
    if kind == "component_user_stiffness_macro_element_review":
        dim = "linear_stiffness" if unit == "N/m" else "rotational_stiffness"
        return dict(source_family="user_review", family="stiffness", dimension=dim,
                    semantic_status="source_specific_unstandardized")
    if kind in {"spring_hanger_user_input_review", "constant_effort_user_input_review"}:
        if unit == "N":
            family, dimension = "force", "force"
        elif unit == "N/m":
            family, dimension = "stiffness", "linear_stiffness"
        else:
            family, dimension = "displacement", "length"
        return dict(source_family="user_review", family=family, dimension=dimension,
                    semantic_status="source_specific_unstandardized")
    if kind == "nonlinear_support_final_displacement":
        return dict(source_family="support_response", family="displacement", dimension="length",
                    semantic_status="source_specific_unstandardized")
    if kind in {"nonlinear_support_final_reaction", "nonlinear_support_friction_normal_reaction_derived"}:
        return dict(source_family="support_response", family="reaction", dimension="force",
                    semantic_status="source_specific_unstandardized")
    observations = {
        "linear_solver_mode_basis": "mode_code",
        "nonlinear_support_active_set_converged_flag": "boolean",
        "nonlinear_support_active_set_final_residual_count": "count",
        "nonlinear_support_active_set_iteration_count": "count",
        "nonlinear_support_active_set_state_code": "state_code",
        "nonlinear_support_observed_max_translation_delta": "length",
        "nonlinear_support_observed_max_rotation_delta": "angle",
        "nonlinear_support_observed_max_force_reaction_delta": "force",
        "nonlinear_support_observed_max_moment_reaction_delta": "moment",
        "nonlinear_support_observed_free_dof_force_residual": "force",
        "nonlinear_support_observed_free_dof_moment_residual": "moment",
        "nonlinear_support_free_dof_work_residual": "energy",
    }
    if kind in observations:
        return dict(source_family="solver_observation", family="solver_observation",
                    dimension=observations[kind], semantic_status="transport_observation_only")
    raise KeyError((kind, unit))


def extract_disclosed_pairs(document: dict) -> set[tuple[str, str]]:
    pairs = set()
    for diagnostic in document["result_envelope"]["diagnostics"]:
        if diagnostic["code"] != DISCLOSURE:
            continue
        match = re.search(r" kind=(.*?) unit=(.*?) ", diagnostic["message"])
        if not match:
            raise AssertionError(diagnostic["message"])
        pairs.add(match.groups())
    return pairs


def build_schema(base: dict, mappings: list[dict[str, str]]) -> dict:
    schema = copy.deepcopy(base)
    schema["$id"] = "https://openpipestress.org/schemas/results/0.2.0/results.schema.json"
    schema["title"] = "OpenPipeStress Result Export Schema 0.2.0 — CANDIDATE"
    schema["description"] = (
        "Route-A candidate. It preserves each native ResultItem verbatim in source_record and "
        "keeps source-specific or transport-only classifications explicit. It is not adopted."
    )
    schema["properties"]["schema_version"] = {"const": VERSION}
    defs = schema["$defs"]
    defs["ResultEnvelope"]["properties"]["schema_version"] = {"const": VERSION}
    defs["ResultEnvelope"]["required"].append("version_contract")
    defs["ResultEnvelope"]["properties"]["version_contract"] = {
        "$ref": "#/$defs/VersionContract"
    }
    defs["VersionContract"] = {
        "type": "object",
        "additionalProperties": False,
        "required": ["schema_id", "document_version", "minimum_reader_version", "unknown_version_policy"],
        "properties": {
            "schema_id": {"const": "https://openpipestress.org/schemas/results/0.2.0/results.schema.json"},
            "document_version": {"const": VERSION},
            "minimum_reader_version": {"const": VERSION},
            "unknown_version_policy": {"const": "reject_unsupported_schema_version"},
        },
    }
    dims = defs["DimensionId"]["enum"]
    defs["DimensionId"]["enum"] = list(dict.fromkeys(dims + ["energy", "count", "boolean", "state_code", "mode_code"]))
    defs["ResultFamily"]["enum"] += ["stiffness", "solver_observation"]
    native_metadata = {
        "type": "object",
        "additionalProperties": False,
        "description": "Exact five-string metadata from the native ResultItem. These strings are preserved, not normalized semantics.",
        "required": ["component", "coordinate_system", "location", "basis", "sign_convention"],
        "properties": {name: {"type": "string", "minLength": 1} for name in ["component", "coordinate_system", "location", "basis", "sign_convention"]},
    }
    defs["NativeResultMetadata"] = native_metadata
    defs["ResultMetadata"] = copy.deepcopy(native_metadata)
    defs["ResultMetadata"]["description"] = "Compatibility projection equal to source_record.metadata when metadata exists; equality is an adapter acceptance check."

    pair_branches = []
    quantity_branches = []
    for item in mappings:
        pair = {
            "type": "object",
            "properties": {"kind": {"const": item["kind"]}, "unit": {"const": item["unit"]}},
            "required": ["kind", "unit"],
        }
        pair_branches.append(pair)
        quantity_branches.append({
            "type": "object",
            "properties": {
                "source_record": pair,
                "family": {"const": item["family"]},
                "dimension": {"const": item["dimension"]},
                "classification": {
                    "type": "object",
                    "properties": {
                        "source_family": {"const": item["source_family"]},
                        "semantic_status": {"const": item["semantic_status"]},
                        "canonical_family": {"const": item["family"]},
                        "canonical_dimension": {"const": item["dimension"]},
                    },
                    "required": ["source_family", "semantic_status", "canonical_family", "canonical_dimension"],
                },
            },
            "required": ["source_record", "family", "dimension", "classification"],
        })
    defs["NativeResultRecord"] = {
        "allOf": [
            {
                "type": "object",
                "additionalProperties": False,
                "required": ["id", "kind", "value", "unit", "entity_ref"],
                "properties": {
                    "id": {"$ref": "#/$defs/Id"},
                    "kind": {"type": "string"},
                    "value": {"type": "number"},
                    "unit": {"type": "string", "minLength": 1},
                    "entity_ref": {"type": "string", "minLength": 1},
                    "basis_ref": {"$ref": "#/$defs/Reference"},
                    "source_result_refs": {"type": "array", "items": {"$ref": "#/$defs/Id"}},
                    "metadata": {"$ref": "#/$defs/NativeResultMetadata"},
                },
            },
            {"oneOf": pair_branches},
        ]
    }
    defs["ResultClassification"] = {
        "type": "object",
        "additionalProperties": False,
        "required": ["source_family", "semantic_status", "canonical_family", "canonical_dimension", "owner_semantics"],
        "properties": {
            "source_family": {"type": "string", "enum": sorted({m["source_family"] for m in mappings})},
            "semantic_status": {"type": "string", "enum": sorted({m["semantic_status"] for m in mappings})},
            "canonical_family": {"$ref": "#/$defs/ResultFamily"},
            "canonical_dimension": {"$ref": "#/$defs/DimensionId"},
            "owner_semantics": {
                "type": "string",
                "enum": ["PKG04_05_MECHANICS", "PKG13_MODEL_BRIDGE", "PKG08_TRANSPORT_ONLY"]
            },
            "notes": {"type": "string"},
        },
    }
    quantity = defs["QuantityResult"]
    quantity["required"] += ["source_record", "classification"]
    quantity["properties"]["source_record"] = {"$ref": "#/$defs/NativeResultRecord"}
    quantity["properties"]["classification"] = {"$ref": "#/$defs/ResultClassification"}
    quantity["allOf"].append({"oneOf": quantity_branches})

    checksum = defs["Checksum"]
    checksum["required"].append("canonicalization_claim")
    checksum["properties"]["canonicalization"]["enum"] = ["JCS", "rfc8785_jcs", "NONE", "TBD"]
    checksum["properties"]["canonicalization_claim"] = {
        "type": "string",
        "enum": ["rfc8785_conformant", "implementation_label_only_conformance_not_established", "no_canonicalization", "undetermined"],
    }
    checksum["allOf"] = [
        {"if": {"properties": {"canonicalization": {"const": "JCS"}}, "required": ["canonicalization"]}, "then": {"properties": {"canonicalization_claim": {"const": "rfc8785_conformant"}}}},
        {"if": {"properties": {"canonicalization": {"const": "rfc8785_jcs"}}, "required": ["canonicalization"]}, "then": {"properties": {"canonicalization_claim": {"const": "implementation_label_only_conformance_not_established"}}}},
        {"if": {"properties": {"canonicalization": {"const": "NONE"}}, "required": ["canonicalization"]}, "then": {"properties": {"canonicalization_claim": {"const": "no_canonicalization"}}}},
        {"if": {"properties": {"canonicalization": {"const": "TBD"}}, "required": ["canonicalization"]}, "then": {"properties": {"canonicalization_claim": {"const": "undetermined"}}}},
    ]
    return schema


def owner_for(status: str) -> str:
    if status == "transport_observation_only":
        return "PKG08_TRANSPORT_ONLY"
    if "pending_P5" in status:
        return "PKG04_05_MECHANICS"
    if status == "source_specific_unstandardized":
        return "PKG04_05_MECHANICS"
    return "PKG08_TRANSPORT_ONLY"


def classification(item: dict[str, str]) -> dict[str, str]:
    return {
        "source_family": item["source_family"],
        "semantic_status": item["semantic_status"],
        "canonical_family": item["family"],
        "canonical_dimension": item["dimension"],
        "owner_semantics": owner_for(item["semantic_status"]),
        "notes": "Transport mapping only; source_record is controlling on semantic disagreement.",
    }


def main() -> None:
    root = repo_root()
    working = root / "projects/chirality-piping"
    here = Path(__file__).resolve().parent
    prep = working / "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260907-PHYSICS-UI-PREPARATION/instances/W10"
    prior = working / "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-PHYSICS-AUDIT/instances/P5/compatibility_v1/author"
    raw_path = working / "fixtures/product_preview/invented_mechanics_result.json"
    base_schema_path = working / "schemas/results.schema.yaml"

    documents = {name: json.loads((prior / f"{name}_canonical_document.json").read_text()) for name in ["linear", "nonlinear", "zero_pressure"]}
    raw = json.loads(raw_path.read_text())
    observed_pairs = {(row["kind"], row["unit"]) for row in raw["results"]}
    for document in documents.values():
        observed_pairs |= extract_disclosed_pairs(document)
    mappings = []
    for kind, unit in sorted(observed_pairs):
        item = {"kind": kind, "unit": unit, **mapping_for(kind, unit)}
        mappings.append(item)
    assert len(mappings) == 45

    schema = build_schema(json.loads(base_schema_path.read_text()), mappings)
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema)

    nonlinear = copy.deepcopy(documents["nonlinear"])
    envelope = nonlinear["result_envelope"]
    existing = {v["result_id"]: v for s in envelope["result_sets"] for v in s["values"]}
    map_by_pair = {(m["kind"], m["unit"]): m for m in mappings}
    migrated = []
    for source in raw["results"]:
        item = map_by_pair[(source["kind"], source["unit"])]
        if source["id"] in existing:
            value = copy.deepcopy(existing[source["id"]])
            assert value["magnitude"] == source["value"]
            assert value["unit"] == source["unit"]
            if source.get("metadata") is not None:
                assert value.get("metadata") == source["metadata"]
        else:
            basis_ref = source.get("basis_ref") or envelope["run_ref"]
            value = {
                "result_id": source["id"],
                "family": item["family"],
                "object_ref": {"ref_type": source["entity_ref"].split(":", 1)[0], "ref_id": source["entity_ref"]},
                "basis_ref": basis_ref,
                "magnitude": source["value"],
                "unit": source["unit"],
                "dimension": item["dimension"],
                "provenance": copy.deepcopy(envelope["provenance"]),
            }
            if source.get("metadata") is not None:
                value["metadata"] = copy.deepcopy(source["metadata"])
        value["family"] = item["family"]
        value["dimension"] = item["dimension"]
        value["source_record"] = copy.deepcopy(source)
        value["classification"] = classification(item)
        migrated.append(value)
    envelope["result_sets"][0]["values"] = migrated
    nonlinear["schema_version"] = VERSION
    envelope["schema_version"] = VERSION
    envelope["version_contract"] = {
        "schema_id": schema["$id"],
        "document_version": VERSION,
        "minimum_reader_version": VERSION,
        "unknown_version_policy": "reject_unsupported_schema_version",
    }
    checksum_before = []
    for checksum in [envelope["reproducibility"]["model_hash"], *envelope["reproducibility"]["run_hashes"]]:
        checksum_before.append({k: copy.deepcopy(checksum[k]) for k in ["algorithm", "payload_ref", "value"]})
        label = checksum["canonicalization"]
        checksum["canonicalization_claim"] = {
            "JCS": "rfc8785_conformant",
            "rfc8785_jcs": "implementation_label_only_conformance_not_established",
            "NONE": "no_canonicalization",
            "TBD": "undetermined",
        }[label]
    errors = list(validator.iter_errors(nonlinear))
    if errors:
        for error in errors[:20]:
            print("/" + "/".join(map(str, error.absolute_path)), error.message, file=sys.stderr)
        raise SystemExit(f"candidate validation failed: {len(errors)} errors")
    assert len({value["result_id"] for value in migrated}) == len(migrated)
    unknown_version = copy.deepcopy(nonlinear)
    unknown_version["schema_version"] = "9.9.9"
    assert list(validator.iter_errors(unknown_version))
    unknown_pair = copy.deepcopy(nonlinear)
    unknown_pair["result_envelope"]["result_sets"][0]["values"][0]["source_record"]["kind"] = "future_unknown_kind"
    assert list(validator.iter_errors(unknown_pair))
    checksum_after = [{k: copy.deepcopy(c[k]) for k in ["algorithm", "payload_ref", "value"]} for c in [envelope["reproducibility"]["model_hash"], *envelope["reproducibility"]["run_hashes"]]]
    assert checksum_before == checksum_after
    for source, value in zip(raw["results"], migrated):
        assert value["source_record"] == source
        assert value["result_id"] == source["id"]
        assert value["magnitude"] == source["value"]
        assert value["unit"] == source["unit"]

    omitted = {}
    for name, document in documents.items():
        diagnostics = [d for d in document["result_envelope"]["diagnostics"] if d["code"] == DISCLOSURE]
        ids = [d["affected_object"]["ref_id"] for d in diagnostics]
        values = [v for s in document["result_envelope"]["result_sets"] for v in s["values"]]
        omitted[name] = {
            "emitted_rows": len(values),
            "omitted_rows": len(ids),
            "native_rows": len(values) + len(ids),
            "omitted_result_ids": ids,
        }
    assert {k: v["omitted_rows"] for k, v in omitted.items()} == {"linear": 9, "nonlinear": 33, "zero_pressure": 39}
    assert {k: v["native_rows"] for k, v in omitted.items()} == {"linear": 786, "nonlinear": 830, "zero_pressure": 813}

    schema_path = here / "RESULTS_SCHEMA_0_2_0_CANDIDATE.json"
    schema_path.write_text(json.dumps(schema, indent=2) + "\n")
    with (here / "SOURCE_KIND_MAPPING.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(
            stream,
            fieldnames=["kind", "unit", "source_family", "family", "dimension", "semantic_status"],
            lineterminator="\n",
        )
        writer.writeheader()
        writer.writerows(mappings)
    field_mapping = list(csv.DictReader((prep / "FIELD_MAPPING.csv").open(encoding="utf-8")))
    w10_rows = json.loads((prep / "ROW_INVENTORY.json").read_text())
    coverage = {
        "status": "PASS_CANDIDATE_DESIGN_NOT_ADOPTED",
        "source_sha": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip(),
        "candidate_version": VERSION,
        "candidate_schema_draft": schema["$schema"],
        "candidate_schema_sha256": sha256(schema_path),
        "observed_native_kind_unit_pairs": len(mappings),
        "full_native_nonlinear_rows_validated": len(migrated),
        "full_native_nonlinear_source_sha256": sha256(raw_path),
        "source_record_byte_semantics": "JSON value equality for every native ResultItem; no field dropped or rewritten",
        "w10_field_mapping_sha256": sha256(prep / "FIELD_MAPPING.csv"),
        "w10_field_inventory_sha256": sha256(prep / "FIELD_INVENTORY.json"),
        "w10_row_inventory_sha256": sha256(prep / "ROW_INVENTORY.json"),
        "w10_distinct_field_mapping_rows": len(field_mapping),
        "w10_invalid_metadata_occurrences": sum(int(r["occurrences"]) for r in field_mapping if r["schema_valid"] == "False"),
        "w10_canonical_rows": len(w10_rows),
        "w10_invalid_metadata_rows": sum(row["status"] == "UNSUPPORTED_MAPPING_REQUIRED" for row in w10_rows),
        "actual_documents": {name: {"sha256": sha256(prior / f"{name}_canonical_document.json"), **row} for name, row in omitted.items()},
        "referenced_checksum_algorithm_payload_ref_value_preserved": checksum_before == checksum_after,
        "canonicalization_label_policy": "legacy rfc8785_jcs remains an implementation label with no RFC8785 conformance assertion; JCS is reserved for evidence-backed conformance",
        "pressure_semantics_policy": "preserve actual source_record; no new wall/effective-force or closure-transfer meaning; P5/PKG04-05 remains semantic owner",
        "unknown_version_behavior_in_candidate": "reject_unsupported_schema_version",
        "unknown_version_schema_rejection_check": "PASS",
        "unknown_kind_unit_schema_rejection_check": "PASS",
        "unique_native_result_id_check": "PASS",
    }
    (here / "COVERAGE_VALIDATION.json").write_text(json.dumps(coverage, indent=2) + "\n")
    print(json.dumps({k: coverage[k] for k in ["status", "observed_native_kind_unit_pairs", "full_native_nonlinear_rows_validated", "w10_invalid_metadata_occurrences"]}))


if __name__ == "__main__":
    main()
