"""Source-block transport branches of the composite identities, using unchanged received producer data.

B4/D1 (b) retired the non-composite source-blocks-1 identity, its table and its receipt schema;
physics-source-1 (and its joins) keep their source-block receipts. These are schema/reader checks,
not a native invocation or engineering qualification. No test assembles a canonical document around
source rows or fabricates a receipt.
"""
from copy import deepcopy
from functools import lru_cache
import hashlib
import json
from pathlib import Path

import pytest

from core.analysis_runs.compatibility import (
    PHYSICS_CONTRACT_ID, PHYSICS_CONTRACT_SHA256,
    build_analysis_run_v0_3,
    numerical_use_standing, validate_analysis_run_v0_3, verify_analysis_run_record,
)
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1
from schema_validation import envelope_branches, validate_instance, validate_schema_document, schema_for_definition

PROJECT = Path(__file__).resolve().parents[1]
MODES = ("sparse_interactive", "dense_scrutiny")
RETIRED_SOURCE_BLOCKS_ID = "openpipestress.result_semantics/0.3.0/source-blocks-1"


def read_received(path):
    # Bare -0 is significant in the received source receipt. Never canonicalize
    # the incoming data before preserving its represented source value bits.
    return json.loads(path.read_text(), parse_int=lambda token: -0.0 if token == "-0" else int(token))


@lru_cache(maxsize=None)
def schema(name):
    return json.loads((PROJECT / "schemas" / name).read_text())


def require_schema(name, instance):
    validate_instance(schema(name), instance, instance_label="actual source-bound artifact")

COMPOSITE_ID = "openpipestress.result_semantics/0.3.0/physics-source-1"
COMPOSITE_SHA256 = "0309b16e2ccb4ec40eaa0c3e7d339ed0d41693ee85dc4fc736d1656cf2702f53"
COMPOSITE_RECEIPT = "physics_source_recovery.schema.json"
COMPOSITE_ROOT = PROJECT / "fixtures/product_preview/physics_source"
COMPOSITE_PAIRS = tuple(f"{case}-{mode}" for case in ("n05", "n06", "mixed", "fields") for mode in MODES)


def test_composite_schema_is_the_only_source_block_receipt_schema():
    composite = schema(COMPOSITE_RECEIPT)
    validate_schema_document(composite)
    assert composite["$id"] == "openpipestress.physics_source_receipt/1.0.0"
    assert composite["$defs"]["receipt_body"]["properties"]["policy"] == {"const": "PHYSICS-SOURCE-1"}
    assert not (PROJECT / "schemas/source_block_recovery.schema.json").exists()
    assert not (PROJECT / "fixtures/results/semantic_contract_v0_3_source_blocks_1.json").exists()
    for name in ("results.v0.3.schema.yaml", "analysis_run.v0.3.schema.json", "stress_neutral_export.v0.3.schema.json"):
        validate_schema_document(schema(name))
        text = (PROJECT / "schemas" / name).read_text()
        assert RETIRED_SOURCE_BLOCKS_ID not in text and "source_block_recovery.schema.json" not in text, name
    assert hashlib.sha256((PROJECT / "fixtures/results/semantic_contract_v0_3_physics_source_1.json").read_bytes()).hexdigest() == COMPOSITE_SHA256


@lru_cache(maxsize=None)
def composite_analysis(name):
    path = COMPOSITE_ROOT / f"{name}.raw.json"
    if not path.is_file():
        pytest.skip("Actual composite producer artifact not yet emitted; no schema or qualification pass claimed")
    case = next(case for case in ("n05", "n06", "fields", "mixed") if name.startswith(case + "-"))
    mode = name.removeprefix(case + "-")
    source = read_received(path)
    request = read_received(COMPOSITE_ROOT / f"{case}.request.json")
    assert source["producer"]["semantic_contract_id"] == COMPOSITE_ID
    # This actual raw source is consumed by the production builder and validator;
    # neither source headers nor receipt/physical evidence are composed in tests.
    record = build_analysis_run_v0_3(source, input_manifest_ref={"object_type": "InputManifest", "ref": f"reference:composite:{name}"},
                                   input_manifest_hash=canonical_sha256_checked_v1({"request": request, "solver_mode": mode}),
                                   expected_basis_refs=[{"object_type": "LoadCase", "ref": c["id"]} for c in request["model"]["load_cases"]])
    return source, request, record


@pytest.mark.parametrize("name", COMPOSITE_PAIRS)
def test_actual_composite_source_analysis_retains_both_namespaces_under_its_own_method(name):
    source, request, record = composite_analysis(name)
    original = deepcopy(source)
    require_schema(COMPOSITE_RECEIPT, source["source_block_recovery"])
    require_schema("analysis_run.v0.3.schema.json", record)
    require_schema("analysis_run.schema.json", record)
    run = record["analysis_run"]
    assert run["source_block_recovery"] == source["source_block_recovery"]
    assert run["contract_evidence"] == source["contract_evidence"]
    assert run["reproducibility"]["semantic_contract"] == {"id": COMPOSITE_ID, "sha256": COMPOSITE_SHA256}
    assert next(row for row in run["hashes"] if row["payload_scope"] == "received_result")["value"] == canonical_sha256_checked_v1(source)
    validate_analysis_run_v0_3(record, source, expected_basis_refs=run["load_basis_refs"])
    assert verify_analysis_run_record(record) == "match"
    assert numerical_use_standing(source, [{"ref_type": "load_case", "ref_id": c["id"]} for c in request["model"]["load_cases"]]) == "needs_recompute"
    assert source == original


@pytest.mark.parametrize("mutation", ("missing_physical", "missing_receipt", "source1_policy", "wrong_hash", "wrong_row_method", "missing_case_binding", "missing_sections", "unknown_norm", "ordinary_maximum_for_source", "foreign_carrier", "retired_identity"))
def test_composite_analysis_closed_wire_rejects_cross_method_or_incomplete_evidence(mutation):
    _, _, original = composite_analysis("n05-sparse_interactive")
    record = deepcopy(original); run = record["analysis_run"]
    if mutation == "missing_physical": del run["contract_evidence"]
    elif mutation == "missing_receipt": del run["source_block_recovery"]
    elif mutation == "source1_policy": run["source_block_recovery"]["body"]["policy"] = "SOURCE-BLOCKS-1"
    elif mutation == "wrong_hash": run["reproducibility"]["semantic_contract"]["sha256"] = PHYSICS_CONTRACT_SHA256
    elif mutation == "wrong_row_method": run["result_refs"][0]["semantic_contract"].update(id=PHYSICS_CONTRACT_ID, sha256=PHYSICS_CONTRACT_SHA256)
    elif mutation == "missing_case_binding": del run["source_block_recovery"]["body"]["cases"][0]["physical_evidence_sha256"]
    elif mutation == "missing_sections": del run["source_block_recovery"]["body"]["cases"][0]["source"]["endpoint_sections"]
    elif mutation == "unknown_norm": run["source_block_recovery"]["body"]["cases"][0]["derived_checks"][0]["recipe_id"] = "unregistered_norm"
    elif mutation == "ordinary_maximum_for_source": run["contract_evidence"]["exact_cases"][0]["recovery_method"] = "ordinary_sparse_structural_v1"
    elif mutation == "retired_identity": run["reproducibility"]["semantic_contract"]["id"] = RETIRED_SOURCE_BLOCKS_ID
    else: run["carrier_evidence"] = None
    with pytest.raises(AssertionError): require_schema("analysis_run.v0.3.schema.json", record)


@pytest.mark.parametrize("name", COMPOSITE_PAIRS)
def test_actual_composite_receipt_and_physical_shapes_without_reconstructing_source(name):
    path = COMPOSITE_ROOT / f"{name}.raw.json"
    if not path.is_file(): pytest.skip("Actual composite capture unavailable; no positive shape claim")
    source = read_received(path)
    assert source["producer"]["semantic_contract_id"] == COMPOSITE_ID
    require_schema(COMPOSITE_RECEIPT, source["source_block_recovery"])
    validate_instance(schema_for_definition(schema(COMPOSITE_RECEIPT), "physical_evidence"), source["contract_evidence"])
    for receipt_case, physical_case in zip(source["source_block_recovery"]["body"]["cases"], source["contract_evidence"]["exact_cases"]):
        assert receipt_case["basis_ref"]["ref_id"] == physical_case["load_case_id"]
        assert receipt_case["selected_method"] == physical_case["recovery_method"]


def test_composite_work_limit_is_its_policy_bound():
    composite = schema(COMPOSITE_RECEIPT)
    new_work = composite["$defs"]["work"]
    assert new_work["properties"]["limit"]["maximum"] == 8_000_000
    # Scalar schema boundaries, not fabricated producer receipts or work claims.
    validate_instance(new_work["properties"]["limit"], 8_000_000)
    with pytest.raises(AssertionError): validate_instance(new_work["properties"]["limit"], 8_000_001)
    for key in ("limit", "charged", "publication_charged"):
        assert composite["$defs"]["receipt_body"]["properties"]["invocation_work"]["properties"][key]["maximum"] == 64_000_000


def test_actual_composite_maximum_metadata_has_a_method_scoped_canonical_route():
    source = read_received(COMPOSITE_ROOT / "n05-sparse_interactive.raw.json")
    actual = next(row["metadata"] for row in source["results"]
                  if row.get("metadata", {}).get("basis") == "retained_source_endpoint_normal_max_v1")
    results = schema("results.v0.3.schema.yaml")
    validate_instance(schema_for_definition(results, "PhysicsSourceMaximumMetadata"), actual)
    with pytest.raises(AssertionError): validate_instance(schema_for_definition(results, "ResultMetadata"), actual)
    # Validate the exact branch wiring; canonical packet positives remain the
    # separately supplied actual headless producer artifacts, never constructed here.
    branches = envelope_branches(results)
    # T4-U2 added pressure-1 (its branch held in results_pressure_1.v0.3); the join added
    # pressure-source-1; B1 removed precision-1's branch and B4/D1 (b) source-blocks-1's.
    assert len(branches) == 9
    for branch in branches:
        # Keyed by the branch's pinned producer identity, not by position: T0R
        # appended preview-physics-1 and T1 appended load-reference-1 and
        # load-reference-source-1; the retained-source methods (with b7-plan's
        # pressure-source-1 join) and the preview method use their own result sets.
        identity = branch["properties"]["producer"]["properties"]["semantic_contract_id"]["const"]
        target = {"openpipestress.result_semantics/0.3.0/physics-source-1": "PhysicsSourceResultSet",
                  "openpipestress.result_semantics/0.3.0/pressure-source-1": "PhysicsSourceResultSet",
                  "openpipestress.result_semantics/0.3.0/load-reference-source-1": "PhysicsSourceResultSet",
                  "openpipestress.result_semantics/0.3.0/preview-physics-1": "PreviewPhysicsResultSet",
                  # T3's F2a successor inherits the preview result sets (reader fan-in);
                  # B3b's exact successor keeps physics-1's (the default).
                  "openpipestress.result_semantics/0.3.0/preview-physics-retained-1": "PreviewPhysicsResultSet",
                  # T4-U2: pressure-1's result sets add the connector vocabulary.
                  "openpipestress.result_semantics/0.3.0/pressure-1": "PressureResultSet"}.get(identity, "ResultSet")
        assert branch["properties"]["result_sets"]["items"] == {"$ref": f"#/$defs/{target}"}
    assert results["$defs"]["ResultSet"]["properties"]["values"]["items"] == {"$ref": "#/$defs/QuantityResult"}
    assert results["$defs"]["QuantityResult"]["properties"]["metadata"] == {"$ref": "#/$defs/ResultMetadata"}
    for field, value in (("component", "axial_normal_stress"), ("coordinate_system", "global"), ("location", "midspan")):
        incorrect = deepcopy(actual); incorrect[field] = value
        with pytest.raises(AssertionError): validate_instance(schema_for_definition(results, "PhysicsSourceMaximumMetadata"), incorrect)


@pytest.mark.parametrize("policy", ("SOURCE-BLOCKS-1", "PRECISION-1", "PHYSICS-1"))
def test_actual_composite_receipt_cannot_claim_an_older_work_policy(policy):
    received = read_received(COMPOSITE_ROOT / "n05-sparse_interactive.raw.json")["source_block_recovery"]
    incorrect = deepcopy(received); incorrect["body"]["policy"] = policy
    with pytest.raises(AssertionError): require_schema(COMPOSITE_RECEIPT, incorrect)
