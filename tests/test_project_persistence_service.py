#!/usr/bin/env python3
"""Focused tests for TP-PER-01 project persistence service behavior."""

import json
import sys
from copy import deepcopy
from hashlib import sha256
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TESTS_DIR = Path(__file__).resolve().parent
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
if str(TESTS_DIR) not in sys.path:
    sys.path.insert(0, str(TESTS_DIR))

from core.model_transform.physical_to_analytical.contract import (  # noqa: E402
    transform_physical_to_analytical,
)
from core.project_persistence import (
    build_project_persistence_envelope,
    canonical_json,
    create_project_store,
    external_artifact_reference,
    open_project_store,
    physical_container_profile,
    project_hash_manifest,
    rebuild_retrieval_sidecars,
    round_trip_project_envelope,
    save_project_store,
    validate_external_artifact_references,
    validate_project_persistence_envelope,
    version_check_project_store,
)
from schema_validation import validate_instance  # noqa: E402


FIXTURE_PATH = ROOT / "fixtures" / "persistence" / "invented_persisted_preview_project.json"
MODEL_SCHEMA_PATH = ROOT / "schemas" / "model.schema.yaml"
PERSISTENCE_SCHEMA_PATH = ROOT / "schemas" / "project_persistence.schema.yaml"
MODEL_STATE_SCHEMA_PATH = ROOT / "schemas" / "model_state.schema.json"
ANALYSIS_RUN_SCHEMA_PATH = ROOT / "schemas" / "analysis_run.schema.json"
PHYSICAL_SOURCE_FIXTURE = ROOT / "fixtures" / "domain" / "invented_physical_source_of_truth_model.json"


def persisted_project():
    return json.loads(FIXTURE_PATH.read_text(encoding="utf-8"))


def load_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def artifact_ref(ref_kind, ref):
    return {"ref_kind": ref_kind, "ref": ref}


def state_ref(object_type, ref, label=None):
    value = {"object_type": object_type, "ref": ref}
    if label is not None:
        value["label"] = label
    return value


def invented_public_provenance(source_name):
    return {
        "source_name": source_name,
        "source_location": "tests/test_project_persistence_service.py",
        "source_license": "project-governed-invented-fixture",
        "contributor": "OpenPipeStress Type 2 worker",
        "contributor_certification": "invented non-engineering schema fixture",
        "redistribution_status": "invented_non_engineering_example",
        "review_status": "accepted",
        "privacy_classification": "invented_public_example",
    }


def state_checksum(payload_ref, payload_scope, value):
    return {
        "algorithm": "sha256",
        "canonicalization": "JCS",
        "payload_ref": payload_ref,
        "payload_scope": payload_scope,
        "value": value,
    }


def professional_boundary():
    return {
        "human_review_required": True,
        "software_makes_compliance_claim": False,
        "software_makes_certification_claim": False,
        "software_makes_sealing_claim": False,
        "software_makes_approval_claim": False,
        "software_makes_authentication_claim": False,
    }


def invented_model_state_record(model_payload, *, note_statement=None):
    state_id = "state:tp-per-01-invented-preview"
    model_id = model_payload["project"]["models"][0]["id"]
    provenance = invented_public_provenance(
        "Invented DEL-14-01 model-state persistence record"
    )
    model_ref = state_ref("Model", model_id, "Invented transformed analytical model")
    return {
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-14-01",
        "package_id": "PKG-14",
        "scope_item": "SOW-071",
        "objectives": ["OBJ-016"],
        "state_contract_status": {
            "record_contract": "schema_first_model_state_records",
            "persistence_binding": "schemas/project_persistence.schema.yaml",
            "canonicalization": "JCS_compatible_json_payload_hashes",
            "physical_project_container": {
                "profile": "sqlite_local_project_store",
                "decision_ref": "SCA-003",
                "storage_role": "local_store_index_projection",
                "canonical_truth": "canonical_json_jcs_payload",
                "sql_public_contract": False,
                "direct_sql_access_allowed": False,
                "hosted_db_allowed": False,
                "network_required": False,
                "sidecars_rebuildable": True,
            },
            "external_human_acceptance": "hash_bound_external_record_only",
        },
        "model_state": {
            "state_id": state_id,
            "state_name": "TP-PER-01 invented preview state",
            "state_kind": "design_snapshot",
            "created_at": "2026-06-06T00:00:00Z",
            "model_ref": model_ref,
            "parent_state_refs": [],
            "tags": [
                {
                    "tag": "invented-preview",
                    "tag_kind": "workflow_label",
                    "provenance": provenance,
                }
            ],
            "notes": [
                {
                    "note_id": "note:tp-per-01-preview",
                    "note_type": "design_note",
                    "statement": note_statement
                    or "Invented model state for persistence round-trip evidence.",
                    "visibility": "public",
                    "provenance": provenance,
                }
            ],
            "external_references": [
                {
                    "reference_id": "external:tp-per-01-audit-manifest",
                    "reference_type": "audit_manifest",
                    "label": "Invented audit manifest reference",
                    "target": "audit-manifest:tp-per-01-invented-preview",
                    "binding_hashes": [
                        state_checksum(
                            state_ref(
                                "AuditManifest",
                                "audit-manifest:tp-per-01-invented-preview",
                            ),
                            "audit_manifest",
                            "invented-audit-manifest-placeholder-hash",
                        )
                    ],
                    "privacy_classification": "invented_public_example",
                    "provenance": provenance,
                }
            ],
            "unresolved_assumptions": [
                {
                    "assumption_id": "assumption:tp-per-01-review-needed",
                    "statement": (
                        "Invented fixture keeps professional review as an "
                        "explicit unresolved assumption."
                    ),
                    "status": "unresolved",
                    "affected_refs": [model_ref],
                    "provenance": provenance,
                }
            ],
            "warnings": [
                {
                    "code": "STATE_FIXTURE_REVIEW_ONLY",
                    "class": "ASSUMPTION_WARNING",
                    "severity": "warning",
                    "source": state_ref("ModelState", state_id),
                    "affected_object": model_ref,
                    "message": "Invented fixture is review evidence only.",
                    "remediation": "Use user-supplied project data and human review before reliance.",
                    "provenance": provenance,
                }
            ],
            "analysis_status": ["MODEL_INCOMPLETE", "HUMAN_REVIEW_REQUIRED"],
            "hashes": [
                state_checksum(
                    model_ref,
                    "model_payload",
                    sha256(canonical_json(model_payload).encode("utf-8")).hexdigest(),
                )
            ],
            "immutability_policy": {
                "snapshot_is_read_only": True,
                "mutation_policy": "changes_create_new_model_state",
                "new_state_required_for_change": True,
                "hash_invalidates_external_acceptance": True,
            },
            "professional_boundary": professional_boundary(),
            "provenance": provenance,
        },
    }


def state_record_hash_from(envelope):
    for item in envelope["project"]["run_history"]["hash_manifest"]:
        if item["payload_scope"] == "model_state_record":
            return item
    raise AssertionError("No model_state_record hash found in run history.")


def validate_persistence_instance(instance, instance_label):
    from jsonschema import Draft202012Validator
    from referencing import Registry, Resource
    from referencing.jsonschema import DRAFT202012

    persistence_schema = load_json(PERSISTENCE_SCHEMA_PATH)
    model_schema = load_json(MODEL_SCHEMA_PATH)
    model_state_schema = load_json(MODEL_STATE_SCHEMA_PATH)
    analysis_run_schema = load_json(ANALYSIS_RUN_SCHEMA_PATH)
    registry = Registry().with_resources(
        [
            (uri, Resource.from_contents(schema, default_specification=DRAFT202012))
            for uri, schema in (
                (model_schema["$id"], model_schema),
                ("https://openpipestress.org/schemas/model.schema.yaml", model_schema),
                ("model.schema.yaml", model_schema),
                (model_state_schema["$id"], model_state_schema),
                ("https://openpipestress.org/schemas/model_state.schema.json", model_state_schema),
                ("model_state.schema.json", model_state_schema),
                (analysis_run_schema["$id"], analysis_run_schema),
                ("https://openpipestress.org/schemas/analysis_run.schema.json", analysis_run_schema),
                ("analysis_run.schema.json", analysis_run_schema),
            )
        ]
    )
    validator = Draft202012Validator(persistence_schema, registry=registry)
    errors = sorted(validator.iter_errors(instance), key=lambda error: list(error.path))
    if errors:
        formatted = "\n".join(format_schema_error(error) for error in errors[:10])
        raise AssertionError(f"{instance_label} failed JSON Schema validation:\n{formatted}")


def format_schema_error(error):
    path = "$"
    for part in error.path:
        path += f"[{part}]" if isinstance(part, int) else f".{part}"
    return f"{path}: {error.message}"


def transformed_physical_source_payload():
    physical_model = load_json(PHYSICAL_SOURCE_FIXTURE)["model"]
    transform = transform_physical_to_analytical(physical_model)
    assert not transform.has_blocking_findings
    analytical_model = transform.analytical_model
    payload = {
        "schema_version": "0.1.0",
        "project": {
            "id": "project:tp-phys-012-derived",
            "name": "TP-PHYS-012 invented derived analytical project",
            "description": "Invented project payload derived from the canonical physical source fixture.",
            "unit_system": {
                "id": "unit-system:tp-phys-012-si",
                "description": "Invented SI-like unit metadata for transform persistence checks.",
                "base_units": {
                    "length": "m",
                    "mass": "kg",
                    "force": "N",
                    "temperature": "degC",
                    "angle": "rad",
                },
            },
            "privacy_class": "public",
            "storage_policy": "public_example",
            "models": [analytical_model],
            "rule_pack_refs": [],
            "report_settings": {
                "id": "report-settings:tp-phys-012-derived",
                "include_input_manifest": True,
                "include_provenance_summary": True,
                "include_professional_boundary_notice": True,
                "result_refs": [],
                "rule_pack_refs": [],
                "provenance": analytical_model["provenance"],
            },
            "reports": [],
            "diagnostics": [],
            "hashes": [
                {
                    "algorithm": "sha256",
                    "canonicalization": "JCS",
                    "payload_ref": {
                        "object_type": "Project",
                        "id": "project:tp-phys-012-derived",
                    },
                    "value": "tp-phys-012-invented-placeholder-hash",
                }
            ],
        },
    }
    validate_instance(
        load_json(MODEL_SCHEMA_PATH),
        payload,
        schema_label=str(MODEL_SCHEMA_PATH),
        instance_label="TP-PHYS-012 derived analytical payload",
    )
    return payload


def derived_transform_envelope(model_payload, *, model_state_records=None):
    state_refs = (
        [
            artifact_ref("model_state", item["model_state"]["state_id"])
            for item in model_state_records
        ]
        if model_state_records
        else [artifact_ref("model_state", "model-state:tp-phys-012-derived")]
    )
    return build_project_persistence_envelope(
        project_id="project:tp-phys-012-derived",
        project_name="TP-PHYS-012 invented derived analytical project",
        model_payload=model_payload,
        model_state_refs=state_refs,
        model_state_records=model_state_records,
        analysis_run_refs=[
            artifact_ref("analysis_run", "analysis-run:tp-phys-012-derived")
        ],
        result_envelope_refs=[
            artifact_ref("result_envelope", "result-envelope:tp-phys-012-derived")
        ],
        result_refs=[
            artifact_ref("result", "result:tp-phys-012-derived-schema-probe")
        ],
        provenance_manifest=[
            model_payload["project"]["models"][0]["provenance"],
        ],
    )


def test_invented_persistence_fixture_validates_with_run_history_refs():
    envelope = persisted_project()
    run_history = envelope["project"]["run_history"]

    assert envelope["document_kind"] == "openpipestress.project_persistence"
    assert envelope["physical_container"]["status"] == "accepted"
    assert envelope["physical_container"]["profile"] == "sqlite_local_project_store"
    assert envelope["physical_container"]["decision_ref"] == "SCA-003"
    assert envelope["physical_container"]["direct_sql_access_allowed"] is False
    assert envelope["validation_profile"]["telemetry_default"] == "off"
    assert envelope["project"]["private_data"]["default_transmission_allowed"] is False
    assert validate_project_persistence_envelope(envelope) == []
    assert {item["ref"] for item in run_history["analysis_run_refs"]} == {
        "run:preview-linear-static-001"
    }
    assert "result-envelope:run:preview-linear-static-001" in {
        item["ref"] for item in run_history["result_envelope_refs"]
    }
    assert "result:stress:pipe-P-120:end-j:torsional-shear" in {
        item["ref"] for item in run_history["result_refs"]
    }
    assert "result:force:pipe-P-120:midspan:axial" in {
        item["ref"] for item in run_history["result_refs"]
    }
    assert "result:force:pipe-P-120:quarter-1:shear-y" in {
        item["ref"] for item in run_history["result_refs"]
    }
    assert "result:combination:combination-C-OPER-ALT:force:pipe-P-120:axial" in {
        item["ref"] for item in run_history["result_refs"]
    }
    assert "result:combination:combination-C-OPER-ALT:force:pipe-P-120:quarter-1:shear-y" in {
        item["ref"] for item in run_history["result_refs"]
    }
    model = envelope["project"]["model_payload"]["project"]["models"][0]
    assert "loadcase:L200" in {item["id"] for item in model["load_cases"]}
    assert "combination:C-OPER-ALT" in {item["id"] for item in model["combinations"]}


def test_canonical_hashes_are_stable_and_cover_project_payloads():
    envelope = persisted_project()
    cloned = json.loads(canonical_json(envelope))

    assert canonical_json(envelope) == canonical_json(cloned)
    assert project_hash_manifest(envelope) == envelope["hash"]["hash_manifest"]
    scopes = {item["payload_scope"] for item in envelope["hash"]["hash_manifest"]}
    assert {"project_payload", "model_payload", "project_envelope"} <= scopes


def test_round_trip_keeps_semantic_equality_and_hashes():
    round_trip = round_trip_project_envelope(persisted_project())

    assert round_trip["serialization"] == "canonical_json_jcs"
    assert round_trip["semantic_equal"] is True
    assert round_trip["diagnostics"] == []
    assert round_trip["source_hash"]["value"] == round_trip["round_trip_hash"]["value"]


def test_model_state_record_embeds_in_run_history_with_deterministic_hash():
    model_payload = transformed_physical_source_payload()
    state_record = invented_model_state_record(model_payload)
    validate_instance(
        load_json(MODEL_STATE_SCHEMA_PATH),
        state_record,
        schema_label=str(MODEL_STATE_SCHEMA_PATH),
        instance_label="DEL-14-01 invented model state record",
    )

    envelope = derived_transform_envelope(
        model_payload,
        model_state_records=[state_record],
    )
    run_history = envelope["project"]["run_history"]
    state_hash = state_record_hash_from(envelope)
    expected_hash = sha256(canonical_json(state_record).encode("utf-8")).hexdigest()

    assert run_history["model_state_refs"] == [
        artifact_ref("model_state", state_record["model_state"]["state_id"])
    ]
    assert run_history["model_state_records"] == [state_record]
    assert state_hash["payload_ref"] == run_history["model_state_refs"][0]
    assert state_hash["payload_scope"] == "model_state_record"
    assert state_hash["value"] == expected_hash
    assert state_hash in envelope["hash"]["hash_manifest"]
    assert validate_project_persistence_envelope(envelope) == []
    validate_persistence_instance(
        envelope,
        "DEL-14-01 model-state-record persistence envelope",
    )


def test_model_state_record_round_trips_through_persistence_store(tmp_path):
    model_payload = transformed_physical_source_payload()
    state_record = invented_model_state_record(model_payload)
    envelope = derived_transform_envelope(
        model_payload,
        model_state_records=[state_record],
    )
    state_hash = state_record_hash_from(envelope)

    round_trip = round_trip_project_envelope(envelope)
    assert round_trip["semantic_equal"] is True
    assert round_trip["diagnostics"] == []
    assert round_trip["envelope"]["project"]["run_history"]["model_state_records"] == [
        state_record
    ]
    assert state_record_hash_from(round_trip["envelope"]) == state_hash

    store_path = tmp_path / "state-record.opsdb"
    saved = save_project_store(store_path, envelope)
    opened = open_project_store(store_path)

    assert saved["diagnostics"] == []
    assert opened["diagnostics"] == []
    assert opened["envelope"]["project"]["run_history"]["model_state_records"] == [
        state_record
    ]
    assert state_record_hash_from(opened["envelope"]) == state_hash
    assert canonical_json(opened["envelope"]) == canonical_json(saved["envelope"])


def test_model_state_record_payload_change_changes_persistence_hash():
    model_payload = transformed_physical_source_payload()
    original = derived_transform_envelope(
        model_payload,
        model_state_records=[invented_model_state_record(model_payload)],
    )
    changed = derived_transform_envelope(
        model_payload,
        model_state_records=[
            invented_model_state_record(
                model_payload,
                note_statement="Invented model state note changed for hash evidence.",
            )
        ],
    )

    assert state_record_hash_from(original)["value"] != state_record_hash_from(changed)[
        "value"
    ]
    assert project_hash_manifest(original) != project_hash_manifest(changed)


def test_sqlite_store_round_trip_preserves_canonical_hashes(tmp_path):
    envelope = persisted_project()
    store_path = tmp_path / "project.opsdb"

    saved = create_project_store(store_path, envelope)
    opened = open_project_store(store_path)
    version = version_check_project_store(store_path)

    assert saved["diagnostics"] == []
    assert opened["diagnostics"] == []
    assert opened["envelope"]["hash"]["project_payload_hash"]["value"] == (
        saved["envelope"]["hash"]["project_payload_hash"]["value"]
    )
    assert canonical_json(opened["envelope"]) == canonical_json(saved["envelope"])
    assert version["db_migration_status"] == "current"
    assert version["product_schema_migration_status"] == "current"


def test_retrieval_sidecar_rebuild_is_hash_neutral(tmp_path):
    store_path = tmp_path / "project.opsdb"
    saved = save_project_store(store_path, persisted_project())

    rebuild = rebuild_retrieval_sidecars(store_path)

    assert saved["diagnostics"] == []
    assert rebuild["sidecar_kind"] == "sqlite_fts5_bm25"
    assert rebuild["hash_neutral"] is True
    assert rebuild["before_hash"] == rebuild["after_hash"]


def test_external_artifact_references_report_missing_and_private_without_copying(tmp_path):
    missing_path = tmp_path / "missing-large-file.dat"
    artifact = external_artifact_reference(
        artifact_id="external:large-file",
        uri_or_path=str(missing_path),
        size_bytes=1024,
        checksum_value="0" * 64,
        classification="private_project",
        verification_status="missing",
    )
    envelope = persisted_project()
    envelope["external_artifacts"] = [artifact]
    envelope["hash"] = {
        **envelope["hash"],
        "project_payload_hash": project_hash_manifest(envelope)[0],
        "hash_manifest": project_hash_manifest(envelope),
    }

    codes = {item["code"] for item in validate_external_artifact_references(envelope)}

    assert "PERSISTENCE_EXTERNAL_ARTIFACT_MISSING" in codes
    assert not missing_path.exists()


def test_physical_container_profile_blocks_hosted_db_and_direct_sql():
    profile = physical_container_profile()

    assert profile["profile"] == "sqlite_local_project_store"
    assert profile["sql_public_contract"] is False
    assert profile["direct_sql_access_allowed"] is False
    assert profile["hosted_db_allowed"] is False
    assert profile["network_required"] is False
    assert profile["sidecars_rebuildable"] is True


def test_mutation_changes_hash_and_validation_reports_mismatch():
    envelope = persisted_project()
    mutated = deepcopy(envelope)
    mutated["project"]["model_payload"]["project"]["name"] = "Changed invented project"

    assert project_hash_manifest(envelope) != project_hash_manifest(mutated)
    codes = {item["code"] for item in validate_project_persistence_envelope(mutated)}
    assert "PERSISTENCE_PROJECT_HASH_MISMATCH" in codes
    assert "PERSISTENCE_HASH_MANIFEST_MISMATCH" in codes


def test_missing_boundary_fields_return_structured_diagnostics():
    envelope = persisted_project()
    del envelope["project"]["provenance_manifest"]
    del envelope["project"]["private_data"]
    del envelope["professional_boundary"]
    envelope["project"]["run_history"]["result_refs"] = []

    codes = {item["code"] for item in validate_project_persistence_envelope(envelope)}

    assert "PERSISTENCE_PROVENANCE_MISSING" in codes
    assert "PERSISTENCE_PRIVATE_DATA_BOUNDARY_INVALID" in codes
    assert "PERSISTENCE_PROFESSIONAL_BOUNDARY_VIOLATION" in codes
    assert "PERSISTENCE_RUN_HISTORY_REFS_MISSING" in codes


def test_fixture_preserves_private_data_provenance_and_professional_boundaries():
    envelope = persisted_project()

    assert envelope["project"]["private_data"]["classification"] == "public_permissive"
    assert envelope["project"]["private_data"]["default_transmission_allowed"] is False
    assert envelope["project"]["provenance_manifest"]
    for item in envelope["project"]["provenance_manifest"]:
        assert item["redistribution_status"] == "public_permissive"
        assert item["review_status"] == "accepted"

    boundary = envelope["professional_boundary"]
    assert boundary["human_review_required"] is True
    assert not boundary["software_makes_compliance_claim"]
    assert not boundary["software_makes_certification_claim"]
    assert not boundary["software_makes_sealing_claim"]
    assert not boundary["software_makes_approval_claim"]
    assert not boundary["software_makes_authentication_claim"]


def test_transformed_physical_fixture_embeds_in_schema_valid_persistence_envelope():
    model_payload = transformed_physical_source_payload()
    envelope = derived_transform_envelope(model_payload)
    model = envelope["project"]["model_payload"]["project"]["models"][0]

    assert model["model_role"] == "analytical_solver_model"
    assert model["source_model_ref"] == {"object_type": "Model", "id": "PHYS-1"}
    assert any(
        link["trace_type"] == "physical_to_analytical"
        and link["source_ref"] == {"object_type": "Element", "id": "E-1"}
        and link["target_ref"] == {"object_type": "Element", "id": "E-1"}
        for link in model["traceability_links"]
    )
    assert validate_project_persistence_envelope(envelope) == []
    validate_persistence_instance(
        envelope,
        "TP-PHYS-012 transformed physical fixture persistence envelope",
    )
    assert project_hash_manifest(envelope) == envelope["hash"]["hash_manifest"]

    round_trip = round_trip_project_envelope(envelope)
    assert round_trip["semantic_equal"] is True
    assert round_trip["diagnostics"] == []


def test_persistence_schema_rejects_transformed_payload_with_noncanonical_dimension():
    model_payload = transformed_physical_source_payload()
    model_payload["project"]["models"][0]["sections"][0]["properties"]["area"][
        "dimension"
    ] = "area_moment"
    envelope = derived_transform_envelope(model_payload)

    assert validate_project_persistence_envelope(envelope) == []
    try:
        validate_persistence_instance(
            envelope,
            "TP-PHYS-012 noncanonical-dimension persistence envelope",
        )
    except AssertionError as exc:
        assert "area_moment" in str(exc)
    else:
        raise AssertionError("Noncanonical transformed payload dimension was accepted.")
