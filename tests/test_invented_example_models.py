#!/usr/bin/env python3
"""Stdlib smoke checks for invented educational model examples.

The examples are in the headless physical-to-analytical transform's reference
format (schemas/physical_transform_model.schema.yaml, FR-MOD-007, DEL-13-04), not
the model document (schemas/model.schema.yaml) that project persistence carries.
"""

import json
import sys
from copy import deepcopy
from hashlib import sha256
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
if str(ROOT / "tests") not in sys.path:
    sys.path.insert(0, str(ROOT / "tests"))

import yaml  # noqa: E402

from core.project_persistence import (  # noqa: E402
    build_project_persistence_envelope,
    canonical_json,
    project_hash_manifest,
    round_trip_project_envelope,
    validate_project_persistence_envelope,
)
from schema_validation import validate_instance, walk_strings  # noqa: E402

EXAMPLE_DIR = ROOT / "examples" / "models" / "invented"
EXAMPLE_FILES = {
    "mechanics_only_toy_span.json",
    "fake_rule_pack_toy_model.json",
}
MODEL_SCHEMA_PATH = ROOT / "schemas" / "model.schema.yaml"
LOAD_REFERENCE_STATE_SCHEMA_PATH = ROOT / "schemas" / "load_reference_state.schema.json"
TRANSFORM_SCHEMA_PATH = ROOT / "schemas" / "physical_transform_model.schema.yaml"
PERSISTENCE_SCHEMA_PATH = ROOT / "schemas" / "project_persistence.schema.yaml"
MODEL_STATE_SCHEMA_PATH = ROOT / "schemas" / "model_state.schema.json"
ANALYSIS_RUN_SCHEMA_PATH = ROOT / "schemas" / "analysis_run.schema.json"
RULE_PACK_PATH = ROOT / "examples" / "rule_packs" / "invented_demo.yaml"

REQUIRED_PROJECT_KEYS = {
    "id",
    "name",
    "unit_system",
    "privacy_class",
    "storage_policy",
    "models",
    "rule_pack_refs",
    "report_settings",
    "reports",
    "diagnostics",
    "hashes",
}

REQUIRED_MODEL_KEYS = {
    "id",
    "name",
    "model_role",
    "coordinate_system",
    "nodes",
    "elements",
    "components",
    "materials",
    "sections",
    "supports",
    "load_cases",
    "combinations",
    "results",
    "diagnostics",
    "unresolved_assumptions",
    "traceability_links",
    "design_knowledge_refs",
    "constraint_refs",
    "equipment_interface_refs",
    "operation_refs",
    "model_state_refs",
    "analysis_run_refs",
    "comparison_refs",
    "handoff_package_refs",
    "external_reference_refs",
    "provenance",
}

REQUIRED_NOTICE_TEXT = {
    "invented",
    "non-code",
    "non-project",
    "not suitable for engineering reliance",
}

FORBIDDEN_TEXT = {
    "A" + "SME",
    "B" + "31",
    "B" + "31J",
    "allowable " + "stress",
    "stress " + "intensification",
    "SIF " + "table",
    "vendor " + "catalog",
    "real " + "secret",
    "private " + "key",
    "BEGIN " + "RSA",
    "BEGIN " + "OPENSSH",
    "pass" + "word",
    "token" + "=",
    "CAE" + "SAR",
    "Auto" + "PIPE",
    "RO" + "HR2",
    "code " + "compliant",
    "certified by " + "openpipestress",
    "sealed by " + "openpipestress",
    "approved by " + "openpipestress",
    "professional " + "approval " + "by the software",
    "OPS_SYNTHETIC_" + "PROTECTED_TABLE",
    "OPS_SYNTHETIC_" + "CODE_FORMULA",
    "OPS_SYNTHETIC_" + "PRIVATE_RULE_PAYLOAD",
    "OPS_SYNTHETIC_" + "VENDOR_CATALOG",
}


def load_examples():
    assert EXAMPLE_DIR.is_dir()
    paths = sorted(EXAMPLE_DIR.glob("*.json"))
    assert {path.name for path in paths} == EXAMPLE_FILES
    return [(path, json.loads(path.read_text(encoding="utf-8"))) for path in paths]


def load_schema(path):
    return yaml.safe_load(path.read_text(encoding="utf-8"))


def validate_persistence_instance(schema, instance, instance_label):
    from jsonschema import Draft202012Validator
    from referencing import Registry, Resource
    from referencing.jsonschema import DRAFT202012

    model_schema = load_schema(MODEL_SCHEMA_PATH)
    load_reference_state_schema = load_schema(LOAD_REFERENCE_STATE_SCHEMA_PATH)
    model_state_schema = load_schema(MODEL_STATE_SCHEMA_PATH)
    analysis_run_schema = load_schema(ANALYSIS_RUN_SCHEMA_PATH)
    registry = Registry().with_resources(
        [
            (
                uri,
                Resource.from_contents(schema_doc, default_specification=DRAFT202012),
            )
            for uri, schema_doc in (
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
    validator = Draft202012Validator(schema, registry=registry)
    errors = sorted(validator.iter_errors(instance), key=lambda error: list(error.path))
    if errors:
        formatted = "\n".join(format_schema_error(error) for error in errors[:10])
        raise AssertionError(f"{instance_label} failed JSON Schema validation:\n{formatted}")


def format_schema_error(error):
    path = "$"
    for part in error.path:
        path += f"[{part}]" if isinstance(part, int) else f".{part}"
    return f"{path}: {error.message}"


def artifact_ref(ref_kind, ref):
    return {"ref_kind": ref_kind, "ref": ref}


def test_invented_examples_follow_model_contract_shape():
    for path, example in load_examples():
        assert example["schema_version"] == "0.6.0", path
        project = example["project"]
        assert REQUIRED_PROJECT_KEYS <= set(project), path
        assert project["privacy_class"] == "public", path
        assert project["storage_policy"] == "public_example", path
        assert project["models"], path

        model_roles = {model["model_role"] for model in project["models"]}
        assert {"physical_source_of_truth", "analytical_solver_model"} <= model_roles

        physical_models = {
            model["id"]: model
            for model in project["models"]
            if model["model_role"] == "physical_source_of_truth"
        }
        assert len(physical_models) == 1

        for model in project["models"]:
            assert REQUIRED_MODEL_KEYS <= set(model), (path, model["id"])
            assert model["nodes"]
            assert model["elements"]
            assert model["materials"]
            assert model["sections"]
            assert model["load_cases"]
            assert model["combinations"]
            if model["model_role"] == "physical_source_of_truth":
                assert model["results"] == []
                assert model["traceability_links"] == []
            else:
                assert model["model_role"] == "analytical_solver_model"
                assert model["source_model_ref"]["id"] in physical_models
                assert model["results"]
                assert any(
                    link["trace_type"] == "physical_to_analytical"
                    and link["source_ref"] == model["source_model_ref"]
                    and link["target_ref"] == {"object_type": "Model", "id": model["id"]}
                    for link in model["traceability_links"]
                ), (path, model["id"])


def test_invented_examples_validate_against_transform_reference_schema():
    schema = load_schema(TRANSFORM_SCHEMA_PATH)

    for path, example in load_examples():
        validate_instance(
            schema,
            example,
            schema_label=str(TRANSFORM_SCHEMA_PATH.relative_to(ROOT)),
            instance_label=str(path.relative_to(ROOT)),
        )


def test_invented_examples_carry_concrete_hashes_for_published_payloads():
    rule_pack_digest = sha256(RULE_PACK_PATH.read_bytes()).hexdigest()

    for path, example in load_examples():
        project = example["project"]
        hashes = project["hashes"]
        assert len(hashes) == 1, path
        project_hash = hashes[0]
        assert project_hash["algorithm"] == "sha256", path
        assert project_hash["canonicalization"] == "JCS", path
        assert project_hash["value"].startswith("sha256:"), path

        project_without_hashes = deepcopy(project)
        project_without_hashes["hashes"] = []
        expected = "sha256:" + sha256(
            canonical_json(project_without_hashes).encode("utf-8")
        ).hexdigest()
        assert project_hash["value"] == expected, path

        for ref in project["rule_pack_refs"]:
            checksum = ref["checksum"]
            assert checksum["algorithm"] == "sha256", path
            assert checksum["canonicalization"] == "NONE", path
            assert checksum["value"] == f"sha256:{rule_pack_digest}", path


def test_invented_examples_round_trip_and_are_not_persistable_model_documents():
    """The persistence service round-trips any payload byte-stably, but the
    persistence schema's ModelPayload is the model document: an example in the
    transform's reference format is refused there, and no adapter converts it.
    The service refuses it too: its `schema_version` (the transform format's
    0.6.0) is not the current model document's (DEL-02-01 Design
    `model-document-0.8.0.md` §3.1; the shared version-state table)."""
    persistence_schema = load_schema(PERSISTENCE_SCHEMA_PATH)

    for path, example in load_examples():
        project = example["project"]
        envelope = build_project_persistence_envelope(
            project_id=project["id"],
            project_name=project["name"],
            model_payload=example,
            model_state_refs=[
                artifact_ref("model_state", f"model-state:{project['id']}:invented-preview")
            ],
            analysis_run_refs=[
                artifact_ref("analysis_run", f"analysis-run:{project['id']}:invented-preview")
            ],
            result_envelope_refs=[
                artifact_ref("result_envelope", f"result-envelope:{project['id']}:invented-preview")
            ],
            result_refs=[
                artifact_ref("result", f"result:{model['id']}:{result['id']}")
                for model in project["models"]
                for result in model["results"]
            ],
            provenance_manifest=[
                model["provenance"]
                for model in project["models"]
                if model["model_role"] == "physical_source_of_truth"
            ],
        )

        try:
            validate_persistence_instance(
                persistence_schema,
                envelope,
                f"persistence envelope for {path.relative_to(ROOT)}",
            )
        except AssertionError as exc:
            assert "'document_kind' is a required property" in str(exc), path
        else:
            raise AssertionError(f"{path.name}: a transform-format payload passed as a model document")
        refusal = [(d["code"], d["severity"]) for d in validate_project_persistence_envelope(envelope)]
        assert refusal == [("MODEL_SCHEMA_VERSION_RETIRED", "blocking")], path
        assert project_hash_manifest(envelope) == envelope["hash"]["hash_manifest"]

        round_trip = round_trip_project_envelope(envelope)
        assert round_trip["serialization"] == "sorted_compact_json"
        assert round_trip["semantic_equal"] is True
        assert [(d["code"], d["severity"]) for d in round_trip["diagnostics"]] == refusal
        assert round_trip["source_hash"]["value"] == round_trip["round_trip_hash"]["value"]


def test_invented_examples_carry_non_reliance_notices():
    for path, example in load_examples():
        text = "\n".join(walk_strings(example)).lower()
        for required in REQUIRED_NOTICE_TEXT:
            assert required in text, (path, required)
        assert "HUMAN_" + "APPROVED_FOR_PROJECT" not in text


def test_invented_examples_do_not_contain_protected_or_private_markers():
    for path, example in load_examples():
        text = "\n".join(walk_strings(example))
        text_lower = text.lower()
        for forbidden in FORBIDDEN_TEXT:
            haystack = text if forbidden != forbidden.lower() else text_lower
            needle = forbidden if forbidden != forbidden.lower() else forbidden.lower()
            assert needle not in haystack, (path, forbidden)


def test_fake_rule_example_links_only_invented_public_rule_pack():
    _, fake_rule = next(
        (path, example)
        for path, example in load_examples()
        if path.name == "fake_rule_pack_toy_model.json"
    )
    refs = fake_rule["project"]["rule_pack_refs"]
    assert len(refs) == 1
    ref = refs[0]
    assert ref["id"] == "INV_FAKE_RULE_PACK_REF"
    assert ref["redistribution_status"] == "public_permissive"
    assert "invented" in ref["source_notice"].lower()
    assert "engineering acceptance basis" in ref["source_notice"].lower()
