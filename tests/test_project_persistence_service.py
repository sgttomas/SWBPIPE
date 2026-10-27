#!/usr/bin/env python3
"""Focused tests for TP-PER-01 project persistence service behavior."""

import json
import re
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
from schema_validation import schema_for_definition, validate_instance  # noqa: E402


FIXTURE_PATH = ROOT / "fixtures" / "persistence" / "invented_persisted_preview_project.json"
MODEL_SCHEMA_PATH = ROOT / "schemas" / "model.schema.yaml"
LOAD_REFERENCE_STATE_SCHEMA_PATH = ROOT / "schemas" / "load_reference_state.schema.json"
PHYSICAL_TRANSFORM_SCHEMA_PATH = ROOT / "schemas" / "physical_transform_model.schema.yaml"
PERSISTENCE_SCHEMA_PATH = ROOT / "schemas" / "project_persistence.schema.yaml"
MODEL_STATE_SCHEMA_PATH = ROOT / "schemas" / "model_state.schema.json"
ANALYSIS_RUN_SCHEMA_PATH = ROOT / "schemas" / "analysis_run.schema.json"
PHYSICAL_SOURCE_FIXTURE = ROOT / "fixtures" / "domain" / "invented_physical_source_of_truth_model.json"
# The canonical payload of the persistence checks below: the bundled invented
# demo, a model document (schemas/model.schema.yaml) the desktop writes.
CANONICAL_MODEL_FIXTURE = ROOT / "fixtures" / "product_preview" / "invented_demo_model.json"


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
        "canonicalization": "SORTED_COMPACT_JSON",
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
    model_id = model_payload["project"]["id"]
    provenance = invented_public_provenance(
        "Invented DEL-14-01 model-state persistence record"
    )
    model_ref = state_ref("Model", model_id, "Invented canonical model document")
    return {
        "schema_version": "0.1.0",
        "deliverable_id": "DEL-14-01",
        "package_id": "PKG-14",
        "scope_item": "SOW-071",
        "objectives": ["OBJ-016"],
        "state_contract_status": {
            "record_contract": "schema_first_model_state_records",
            "persistence_binding": "schemas/project_persistence.schema.yaml",
            "canonicalization": "sorted_compact_json_payload_hashes",
            "physical_project_container": {
                "profile": "sqlite_local_project_store",
                "decision_ref": "SCA-003",
                "storage_role": "local_store_index_projection",
                "canonical_truth": "sorted_compact_json_payload",
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


def test_run_history_preserves_distinct_record_revisions_with_repeated_run_id():
    from core.analysis_runs.compatibility import build_analysis_run

    mechanics = json.loads((ROOT / "fixtures/product_preview/pressure_source/load_reference_source/n05-sparse_interactive.raw.json").read_text())
    manifest_ref = {"object_type": "InputManifest", "ref": "input-manifest:repeated"}
    first = build_analysis_run(mechanics, input_manifest_ref=manifest_ref, input_manifest_hash="1" * 64)
    revised = build_analysis_run(mechanics, input_manifest_ref=manifest_ref, input_manifest_hash="1" * 64, rule_check_status="USER_RULE_CHECKED")
    run_id = mechanics["run_id"]
    envelope = build_project_persistence_envelope(
        project_id="project:repeated",
        project_name="Repeated analysis revision fixture",
        model_payload={"project": {"id": "project:repeated"}},
        model_state_refs=[artifact_ref("model_state", "state:repeated")],
        analysis_run_records=[first, revised],
        result_envelope_refs=[artifact_ref("result_envelope", "result-envelope:repeated")],
        result_refs=[artifact_ref("result", "result:repeated")],
    )
    history = envelope["project"]["run_history"]
    assert history["analysis_run_records"] == [first, revised]
    assert [item["ref"] for item in history["analysis_run_refs"]] == [run_id, run_id]
    identities = [item["hash"]["value"] for item in history["analysis_run_refs"]]
    assert len(set(identities)) == 2
    assert identities == [item["value"] for item in history["hash_manifest"] if item["payload_scope"] == "analysis_run_record"]


def validate_persistence_instance(instance, instance_label):
    from jsonschema import Draft202012Validator
    from referencing import Registry, Resource
    from referencing.jsonschema import DRAFT202012

    persistence_schema = load_json(PERSISTENCE_SCHEMA_PATH)
    model_schema = load_json(MODEL_SCHEMA_PATH)
    load_reference_state_schema = load_json(LOAD_REFERENCE_STATE_SCHEMA_PATH)
    model_state_schema = load_json(MODEL_STATE_SCHEMA_PATH)
    analysis_run_schema = load_json(ANALYSIS_RUN_SCHEMA_PATH)
    registry = Registry().with_resources(
        [
            (uri, Resource.from_contents(schema, default_specification=DRAFT202012))
            for uri, schema in (
                (model_schema["$id"], model_schema),
                ("https://openpipestress.org/schemas/model.schema.yaml", model_schema),
                ("model.schema.yaml", model_schema),
                (load_reference_state_schema["$id"], load_reference_state_schema),
                ("load_reference_state.schema.json", load_reference_state_schema),
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


def transformed_physical_source_model():
    """The headless transform's analytical model. It is checked against the
    transform's own reference format; it is not a model document."""
    physical_model = load_json(PHYSICAL_SOURCE_FIXTURE)["model"]
    transform = transform_physical_to_analytical(physical_model)
    assert not transform.has_blocking_findings
    return transform.analytical_model


def validate_transform_model(analytical_model, instance_label):
    validate_instance(
        schema_for_definition(load_json(PHYSICAL_TRANSFORM_SCHEMA_PATH), "Model"),
        analytical_model,
        schema_label=f"{PHYSICAL_TRANSFORM_SCHEMA_PATH}#/$defs/Model",
        instance_label=instance_label,
    )


def canonical_model_payload():
    payload = load_json(CANONICAL_MODEL_FIXTURE)
    validate_instance(
        load_json(MODEL_SCHEMA_PATH),
        payload,
        schema_label=str(MODEL_SCHEMA_PATH),
        instance_label="invented canonical model payload",
    )
    return payload


def canonical_payload_envelope(model_payload, *, model_state_records=None):
    state_refs = (
        [
            artifact_ref("model_state", item["model_state"]["state_id"])
            for item in model_state_records
        ]
        if model_state_records
        else [artifact_ref("model_state", "model-state:tp-per-01-canonical")]
    )
    return build_project_persistence_envelope(
        project_id=model_payload["project"]["id"],
        project_name=model_payload["project"]["name"],
        model_payload=model_payload,
        model_state_refs=state_refs,
        model_state_records=model_state_records,
        analysis_run_refs=[
            artifact_ref("analysis_run", "analysis-run:tp-per-01-canonical")
        ],
        result_envelope_refs=[
            artifact_ref("result_envelope", "result-envelope:tp-per-01-canonical")
        ],
        result_refs=[
            artifact_ref("result", "result:tp-per-01-canonical-schema-probe")
        ],
        provenance_manifest=persisted_project()["project"]["provenance_manifest"],
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
    model = envelope["project"]["model_payload"]
    assert model["document_kind"] == "openpipestress.product_preview.model"
    # The envelope names the project the embedded document names, as the desktop's save does.
    assert envelope["project"]["project_id"] == model["project"]["id"]
    assert envelope["project"]["project_name"] == model["project"]["name"]
    assert "load:L-200" in {item["id"] for item in model["load_cases"]}
    assert "combination:C-OPER-ALT" in {item["id"] for item in model["combinations"]}
    validate_persistence_instance(envelope, "invented persisted preview project")


def test_canonical_hashes_are_stable_and_cover_project_payloads():
    envelope = persisted_project()
    cloned = json.loads(canonical_json(envelope))

    assert canonical_json(envelope) == canonical_json(cloned)
    assert project_hash_manifest(envelope) == envelope["hash"]["hash_manifest"]
    scopes = {item["payload_scope"] for item in envelope["hash"]["hash_manifest"]}
    assert {"project_payload", "model_payload", "project_envelope"} <= scopes


def test_sorted_compact_json_has_exact_bytes_hash_and_no_jcs_label():
    expected = b'{"a":[1,{"b":true}],"z":"caf\\u00e9"}'
    assert canonical_json({"z": "café", "a": [1, {"b": True}]}).encode("ascii") == expected
    assert sha256(expected).hexdigest() == (
        "9fe3dcafeafda5780fb9062a482c657fdd139cb98c1d4e4356e7123b64665753"
    )
    envelope = persisted_project()
    assert envelope["hash"]["canonicalization"] == "SORTED_COMPACT_JSON"
    assert {
        item["canonicalization"] for item in envelope["hash"]["hash_manifest"]
    } == {"SORTED_COMPACT_JSON"}
    assert envelope["physical_container"]["canonical_truth"] == (
        "sorted_compact_json_payload"
    )
    assert "JCS" not in canonical_json(envelope)


def test_round_trip_keeps_semantic_equality_and_hashes():
    round_trip = round_trip_project_envelope(persisted_project())

    assert round_trip["serialization"] == "sorted_compact_json"
    assert round_trip["semantic_equal"] is True
    assert round_trip["diagnostics"] == []
    assert round_trip["source_hash"]["value"] == round_trip["round_trip_hash"]["value"]


def test_model_state_record_embeds_in_run_history_with_deterministic_hash():
    model_payload = canonical_model_payload()
    state_record = invented_model_state_record(model_payload)
    validate_instance(
        load_json(MODEL_STATE_SCHEMA_PATH),
        state_record,
        schema_label=str(MODEL_STATE_SCHEMA_PATH),
        instance_label="DEL-14-01 invented model state record",
    )

    envelope = canonical_payload_envelope(
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
    model_payload = canonical_model_payload()
    state_record = invented_model_state_record(model_payload)
    envelope = canonical_payload_envelope(
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
    model_payload = canonical_model_payload()
    original = canonical_payload_envelope(
        model_payload,
        model_state_records=[invented_model_state_record(model_payload)],
    )
    changed = canonical_payload_envelope(
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


def test_transformed_physical_output_validates_against_its_reference_format():
    """TP-PHYS-012: the transform output is checked against the transform's reference
    format with its traceability; it is not wrapped in a model document."""
    model = transformed_physical_source_model()
    validate_transform_model(model, "TP-PHYS-012 transformed analytical model")

    assert model["model_role"] == "analytical_solver_model"
    assert model["source_model_ref"] == {"object_type": "Model", "id": "PHYS-1"}
    assert any(
        link["trace_type"] == "physical_to_analytical"
        and link["source_ref"] == {"object_type": "Element", "id": "E-1"}
        and link["target_ref"] == {"object_type": "Element", "id": "E-1"}
        for link in model["traceability_links"]
    )

    noncanonical = deepcopy(model)
    noncanonical["sections"][0]["properties"]["area"]["dimension"] = "area_moment"
    try:
        validate_transform_model(noncanonical, "TP-PHYS-012 noncanonical-dimension model")
    except AssertionError as exc:
        assert "area_moment" in str(exc)
    else:
        raise AssertionError("Noncanonical transformed model dimension was accepted.")


def test_canonical_payload_embeds_in_schema_valid_persistence_envelope():
    envelope = canonical_payload_envelope(canonical_model_payload())

    assert validate_project_persistence_envelope(envelope) == []
    validate_persistence_instance(
        envelope,
        "invented canonical payload persistence envelope",
    )
    assert project_hash_manifest(envelope) == envelope["hash"]["hash_manifest"]

    round_trip = round_trip_project_envelope(envelope)
    assert round_trip["semantic_equal"] is True
    assert round_trip["diagnostics"] == []


def test_persistence_schema_rejects_payload_outside_the_model_document_format():
    model_payload = canonical_model_payload()
    model_payload["pipe_segments"][0]["section"]["outside_diameter"]["dimension"] = "length"
    envelope = canonical_payload_envelope(model_payload)

    assert validate_project_persistence_envelope(envelope) == []
    try:
        validate_persistence_instance(
            envelope,
            "noncanonical-quantity persistence envelope",
        )
    except AssertionError as exc:
        assert "dimension" in str(exc)
    else:
        raise AssertionError("A model payload outside the model document format was accepted.")


# Model document 0.8.0 (DEL-02-01 Design model-document-0.8.0.md §3.1, §3.3): a project whose model payload is a
# retired model document is refused by name on validate, create, save and open, with the desktop's code and text
# (src-tauri refuse_model_document: the retired message, then the recorded-runs note). The model is the committed
# 0.7.0 handoff control's.
_RETIRED_MODEL = ROOT / "fixtures/handoff/retired_0_7_0_witness.swbhandoff/model/model.json"
_RETIRED_TEXT = ("MODEL_SCHEMA_VERSION_RETIRED: model document 0.7.0 is retired; re-author it as model document 0.8.0 "
                 "(a pressure-free model omits pressure_contract). Nothing is migrated. Recorded runs keep their own format "
                 "and existing read and compare behaviour; this change adds no model-document-version gate for recorded runs. "
                 "Existing integrity checks and retained-precision/source-reader restrictions still apply. Run history is "
                 "listed for the active project's id, so this project's runs are reached through a current project with the "
                 "same project id, and run comparison keeps its eligibility rules.")


def _retired_envelope(version="0.7.0"):
    model = json.loads(_RETIRED_MODEL.read_text())
    model["schema_version"] = version
    return build_project_persistence_envelope(
        project_id=model["project"]["id"], project_name="Invented retired project", model_payload=model,
        model_state_refs=[{"object_type": "model_state", "ref": "state:retired"}],
        analysis_run_refs=[{"object_type": "analysis_run", "ref": "run:retired"}],
        result_envelope_refs=[{"object_type": "result_envelope", "ref": "result:retired"}],
        result_refs=[{"object_type": "result", "ref": "rows:retired"}])


def _refusals(diagnostics):
    return [(d["code"], d["severity"], d["message"]) for d in diagnostics if d["code"] == "MODEL_SCHEMA_VERSION_RETIRED"]


def test_a_retired_model_payload_is_refused_by_name_on_validate():
    assert _refusals(validate_project_persistence_envelope(_retired_envelope())) == [("MODEL_SCHEMA_VERSION_RETIRED", "blocking", _RETIRED_TEXT)]
    assert _refusals(validate_project_persistence_envelope(_retired_envelope("0.8.0"))) == []


def test_a_retired_model_payload_is_refused_on_create_and_save_and_nothing_is_written(tmp_path):
    for action in (create_project_store, save_project_store):
        store = tmp_path / f"{action.__name__}.sqlite3"
        outcome = action(store, _retired_envelope())
        assert _refusals(outcome["diagnostics"]) == [("MODEL_SCHEMA_VERSION_RETIRED", "blocking", _RETIRED_TEXT)]
        assert "hash_manifest" not in outcome
        assert open_project_store(store)["envelope"] is None


def test_a_store_holding_a_retired_model_payload_is_refused_on_open(tmp_path, monkeypatch):
    from core.project_persistence import service
    store = tmp_path / "written-before.sqlite3"
    # A store written before the ripple: the save that wrote it had no model-version gate.
    with monkeypatch.context() as patched:
        patched.setattr(service, "model_payload_version_refusal", lambda model_payload: None)
        assert save_project_store(store, _retired_envelope())["diagnostics"] == []
    opened = open_project_store(store)
    assert _refusals(opened["diagnostics"]) == [("MODEL_SCHEMA_VERSION_RETIRED", "blocking", _RETIRED_TEXT)]


def test_the_persistence_refusal_text_is_the_desktop_s():
    from core.project_persistence import service
    rust_format = (ROOT / "core/product_physics/src/model_format.rs").read_text()
    desktop = (ROOT / "apps/desktop/src-tauri/src/model_document_version.rs").read_text()
    assert f'pub const MODEL_SCHEMA_VERSION: &str = "{service.MODEL_VERSION}";' in rust_format
    retired = re.findall(r'"(0\.\d+\.\d+)"', re.search(r"pub const RETIRED_MODEL_SCHEMA_VERSIONS[^=]*=\s*\[([^\]]*)\]", rust_format).group(1))
    assert list(service.RETIRED_MODEL_VERSIONS) == retired
    assert f'pub const RETIRED_PROJECT_RUNS_NOTE: &str = "{service.RETIRED_PROJECT_RUNS_NOTE}";' in desktop
