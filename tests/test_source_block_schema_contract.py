"""Explicit source-block transport branches using unchanged received producer data.

These are schema/reader checks, not a native invocation or engineering qualification.
Canonical positives require artifacts emitted by the actual headless derivative path;
no test assembles a canonical document around source rows or fabricates a receipt.
"""
from copy import deepcopy
from functools import lru_cache
import hashlib
import json
import os
from pathlib import Path

import pytest

from core.analysis_runs.compatibility import (
    PHYSICS_CONTRACT_ID, PHYSICS_CONTRACT_SHA256,
    PRECISION_CONTRACT_ID, PRECISION_CONTRACT_SHA256,
    SOURCE_BLOCKS_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_SHA256,
    analysis_record_projection, build_analysis_run_v0_3,
    numerical_use_standing, validate_analysis_run_v0_3, verify_analysis_run_record,
)
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1
from schema_validation import validate_instance, validate_schema_document, schema_for_definition

PROJECT = Path(__file__).resolve().parents[1]
RECEIVED = PROJECT / "fixtures/product_preview/source_blocks"
MODES = ("sparse_interactive", "dense_scrutiny")
SOURCE_PAIRS = tuple(f"{prefix}{case}-{mode}" for prefix in ("", "ui/")
                     for case in ("n05", "n06", "multicase") for mode in MODES)
METHODS = {
    "precision-1": (PRECISION_CONTRACT_ID, PRECISION_CONTRACT_SHA256),
    "physics-1": (PHYSICS_CONTRACT_ID, PHYSICS_CONTRACT_SHA256),
    "source-blocks-1": (SOURCE_BLOCKS_CONTRACT_ID, SOURCE_BLOCKS_CONTRACT_SHA256),
}
RECEIPT_REF = "source_block_recovery.schema.json"


def read_received(path):
    # Bare -0 is significant in the received source receipt. Never canonicalize
    # the incoming data before preserving its represented source value bits.
    return json.loads(path.read_text(), parse_int=lambda token: -0.0 if token == "-0" else int(token))


@lru_cache(maxsize=None)
def schema(name):
    return json.loads((PROJECT / "schemas" / name).read_text())


def require_schema(name, instance):
    validate_instance(schema(name), instance, instance_label="actual source-bound artifact")


@lru_cache(maxsize=None)
def source_analysis(name):
    source = read_received(RECEIVED / f"{name}.raw.json")
    request = read_received(RECEIVED / f"{name}.request.json")
    # A low-level reference manifest binds the exact captured input. It is not
    # an app Current manifest and does not authenticate a native invocation.
    manifest = {"model_basis": {"model_ref": source["model_ref"], "model_payload": request["model"]},
                "solver_basis": {"solver_name": source["producer"]["component_name"],
                                 "solver_version": source["producer"]["component_version"],
                                 "solver_build_ref": "reference:retained-source-block-producer-capture"},
                "received_request": request}
    record = build_analysis_run_v0_3(
        source, input_manifest_ref={"object_type": "InputManifest", "ref": f"reference:{name}"},
        input_manifest_hash=canonical_sha256_checked_v1(manifest), input_manifest=manifest,
        expected_basis_refs=[{"object_type": "LoadCase", "ref": case["id"]} for case in request["model"]["load_cases"]],
    )
    return source, request, record


@lru_cache(maxsize=None)
def older_analysis(method):
    family = "precision_connected_ui" if method == "precision-1" else "physics_connected_ui"
    source = read_received(PROJECT / f"fixtures/results/{family}_mechanics_sparse.json")
    record = build_analysis_run_v0_3(source, input_manifest_ref={"object_type": "InputManifest", "ref": "reference:older-method"},
                                   input_manifest_hash=canonical_sha256_checked_v1({"received_source_ref": source["model_ref"]}))
    return source, record


@pytest.mark.parametrize("name", SOURCE_PAIRS)
def test_actual_source_block_analysis_has_closed_receipt_and_exact_method_binding(name):
    source, request, record = source_analysis(name)
    generation = json.loads((RECEIVED / "generation.json").read_text())
    hashes = {row["path"]: row["sha256"] for row in generation["files"]}
    for suffix in ("raw", "request"):
        path = RECEIVED / f"{name}.{suffix}.json"
        assert hashlib.sha256(path.read_bytes()).hexdigest() == hashes[f"{name}.{suffix}.json"]
    before = deepcopy(source)
    require_schema("analysis_run.v0.3.schema.json", record)
    require_schema("analysis_run.schema.json", record)
    run = record["analysis_run"]
    assert run["source_block_recovery"] == source["source_block_recovery"]
    assert run["reproducibility"]["semantic_contract"] == dict(zip(("id", "sha256"), METHODS["source-blocks-1"]))
    assert all(row["semantic_contract"]["id"] == SOURCE_BLOCKS_CONTRACT_ID
               and row["semantic_contract"]["sha256"] == SOURCE_BLOCKS_CONTRACT_SHA256 for row in run["result_refs"])
    assert next(item for item in run["hashes"] if item["payload_scope"] == "received_result")["value"] == canonical_sha256_checked_v1(source)
    assert verify_analysis_run_record(record) == "match"
    validate_analysis_run_v0_3(record, source, expected_basis_refs=run["load_basis_refs"])
    # Schema and serialized receipt cannot recreate independent source context.
    bases = [{"ref_type": "load_case", "ref_id": case["id"]} for case in request["model"]["load_cases"]]
    assert numerical_use_standing(source, bases) == "needs_recompute"
    assert source == before


@pytest.mark.parametrize("method", ("precision-1", "physics-1"))
def test_actual_older_analysis_branches_keep_their_original_id_hash_and_no_receipt(method):
    source, record = older_analysis(method)
    require_schema("analysis_run.v0.3.schema.json", record)
    require_schema("analysis_run.schema.json", record)
    assert record["analysis_run"]["reproducibility"]["semantic_contract"] == dict(zip(("id", "sha256"), METHODS[method]))
    assert "source_block_recovery" not in record["analysis_run"]
    validate_analysis_run_v0_3(record, source)


@pytest.mark.parametrize("method", tuple(METHODS))
@pytest.mark.parametrize("value", (None, False, 0, "", [], {}))
def test_receipt_presence_is_method_specific_even_for_falsy_values(method, value):
    record = deepcopy(source_analysis("n05-sparse_interactive")[2] if method == "source-blocks-1" else older_analysis(method)[1])
    record["analysis_run"]["source_block_recovery"] = value
    with pytest.raises(AssertionError):
        require_schema("analysis_run.v0.3.schema.json", record)


@pytest.mark.parametrize("mutation", ("absent", "extra_outer", "extra_body", "extra_case", "wrong_version", "wrong_policy", "wrong_method"))
def test_closed_receipt_rejects_missing_unknown_and_foreign_method_fields(mutation):
    record = deepcopy(source_analysis("n05-sparse_interactive")[2])
    receipt = record["analysis_run"]["source_block_recovery"]
    if mutation == "absent": del record["analysis_run"]["source_block_recovery"]
    elif mutation == "extra_outer": receipt["unregistered"] = None
    elif mutation == "extra_body": receipt["body"]["contract_evidence"] = {}
    elif mutation == "extra_case": receipt["body"]["cases"][0]["unregistered"] = False
    elif mutation == "wrong_version": receipt["body"]["receipt_version"] = "2.0.0"
    elif mutation == "wrong_policy": receipt["body"]["policy"] = "PHYSICS-SOURCE-1"
    else: receipt["body"]["cases"][0]["selected_method"] = "retained_source_endpoint_normal_max_v1"
    with pytest.raises(AssertionError): require_schema("analysis_run.v0.3.schema.json", record)


@pytest.mark.parametrize("method", ("precision-1", "physics-1"))
def test_receipt_cannot_be_downgraded_to_an_older_analysis_id_hash(method):
    record = deepcopy(source_analysis("n05-sparse_interactive")[2])
    identity = dict(zip(("id", "sha256"), METHODS[method]))
    record["analysis_run"]["reproducibility"]["semantic_contract"] = identity
    for row in record["analysis_run"]["result_refs"]: row["semantic_contract"].update(identity)
    with pytest.raises(AssertionError): require_schema("analysis_run.v0.3.schema.json", record)


@pytest.mark.parametrize("mutation", ("wrong_hash", "row_other_method", "row_wrong_hash", "reserved_composite"))
def test_analysis_method_hash_and_each_row_contract_must_agree(mutation):
    record = deepcopy(source_analysis("n05-sparse_interactive")[2])
    run = record["analysis_run"]
    if mutation == "wrong_hash": run["reproducibility"]["semantic_contract"]["sha256"] = PRECISION_CONTRACT_SHA256
    elif mutation == "row_other_method": run["result_refs"][0]["semantic_contract"].update(id=PRECISION_CONTRACT_ID, sha256=PRECISION_CONTRACT_SHA256)
    elif mutation == "row_wrong_hash": run["result_refs"][0]["semantic_contract"]["sha256"] = PHYSICS_CONTRACT_SHA256
    else: run["reproducibility"]["semantic_contract"]["id"] = "openpipestress.result_semantics/0.3.0/physics-source-1"
    with pytest.raises(AssertionError): require_schema("analysis_run.v0.3.schema.json", record)


@pytest.mark.parametrize("method", tuple(METHODS))
@pytest.mark.parametrize("field", ("carrier_evidence", "contract_evidence"))
@pytest.mark.parametrize("value", (None, False, {}))
def test_analysis_forbids_foreign_namespaces_by_presence(method, field, value):
    record = deepcopy(source_analysis("n05-sparse_interactive")[2] if method == "source-blocks-1" else older_analysis(method)[1])
    record["analysis_run"][field] = value
    with pytest.raises(AssertionError): require_schema("analysis_run.v0.3.schema.json", record)


def test_shape_validation_does_not_replace_receipt_source_hash_or_case_binding():
    source, _, original = source_analysis("n05-sparse_interactive")
    record = deepcopy(original)
    record["analysis_run"]["source_block_recovery"]["receipt_sha256"] = "0" * 64
    # This is well-formed syntax, but not the receipt received from the source.
    require_schema("analysis_run.v0.3.schema.json", record)
    record["analysis_run"]["hashes"][0]["value"] = canonical_sha256_checked_v1(analysis_record_projection(record))
    assert verify_analysis_run_record(record) == "match"
    with pytest.raises(ValueError, match="ANALYSIS_SOURCE_BLOCK_RECEIPT_MISMATCH"):
        validate_analysis_run_v0_3(record, source, expected_basis_refs=original["analysis_run"]["load_basis_refs"])


def test_schema_and_table_identities_preserve_older_contracts_and_reference_receipt_file():
    for name in ("results.v0.3.schema.yaml", "analysis_run.v0.3.schema.json"):
        validate_schema_document(schema(name))
    for method, (_, expected) in METHODS.items():
        suffix = method.replace("-1", "_1").replace("source-blocks", "source_blocks")
        table = PROJECT / f"fixtures/results/semantic_contract_v0_3_{suffix}.json"
        assert hashlib.sha256(table.read_bytes()).hexdigest() == expected
    for name, definition in (("results.v0.3.schema.yaml", "ResultEnvelope"), ("analysis_run.v0.3.schema.json", "AnalysisRun")):
        branches = schema(name)["$defs"][definition]["oneOf"]
        assert branches[2]["properties"]["source_block_recovery"] == {"$ref": RECEIPT_REF}
        assert schema(name)["$defs"][definition]["properties"]["source_block_recovery"]["oneOf"][0] == {"$ref": RECEIPT_REF}
    assert schema(RECEIPT_REF)["$id"] == "openpipestress.source_block_receipt/1.0.0"


@pytest.fixture(scope="module")
def actual_derivatives():
    folder = os.environ.get("HEADLESS_SOURCE_BLOCK_OUTPUT_DIR")
    if not folder:
        pytest.skip("Actual source-block headless derivative artifacts unavailable; this is not a canonical schema pass")
    root = Path(folder)
    paths = sorted(root.glob("*.raw.json"))
    assert len(paths) >= 6, "Need actual n05/n06/multicase in both modes, not assembled metadata or an empty artifact directory"
    pairs = []
    for path in paths:
        document_path = path.with_name(path.name.replace(".raw.json", ".document.json"))
        assert document_path.is_file(), f"Missing real derivative for {path.name}"
        source, document = read_received(path), read_received(document_path)
        request = read_received(path.with_name(path.name.replace(".raw.json", ".request.json")))
        invocation = read_received(path.with_name(path.name.replace(".raw.json", ".invocation.json")))
        manifest = read_received(path.with_name(path.name.replace(".raw.json", ".manifest.json")))
        assert invocation == {"request": request, "solver_mode": manifest["mode"]}
        for field, value in (("request_digest", request), ("invocation_digest", invocation),
                             ("raw_digest", source), ("document_digest", document)):
            assert manifest[field] == canonical_sha256_checked_v1(value)
        assert source["producer"]["semantic_contract_id"] == SOURCE_BLOCKS_CONTRACT_ID
        pairs.append((source, document))
    return pairs


def test_actual_headless_derivatives_retain_receipt_and_match_analysis_source_hash(actual_derivatives):
    for source, document in actual_derivatives:
        require_schema("results.v0.3.schema.yaml", document)
        record = build_analysis_run_v0_3(source, input_manifest_ref={"object_type": "InputManifest", "ref": "reference:headless-schema"},
                                       input_manifest_hash=canonical_sha256_checked_v1({"received_source_ref": source["model_ref"]}))
        require_schema("analysis_run.v0.3.schema.json", record)
        envelope = document["result_envelope"]
        assert envelope["source_block_recovery"] == source["source_block_recovery"] == record["analysis_run"]["source_block_recovery"]
        assert envelope["producer"] == source["producer"]
        assert envelope["formulation_basis"] == source["formulation_basis"]
        assert envelope["semantic_contract_ref"]["ref_id"] == SOURCE_BLOCKS_CONTRACT_ID
        digest = canonical_sha256_checked_v1(source)
        assert envelope["reproducibility"]["source_origin_bindings"][0]["received_carrier_checksum"]["value"] == digest
        assert next(row for row in record["analysis_run"]["hashes"] if row["payload_scope"] == "received_result")["value"] == digest
        assert len(envelope["row_accounting"]) == len(source["results"])


@pytest.mark.parametrize("mutation", ("absent", "null", "false", "empty", "foreign_physics", "wrong_profile", "wrong_ref", "downgrade", "carrier_null", "reserved_composite"))
def test_actual_derivative_rejects_missing_mismatched_or_foreign_receipt_method(actual_derivatives, mutation):
    document = deepcopy(actual_derivatives[0][1])
    envelope = document["result_envelope"]
    if mutation == "absent": del envelope["source_block_recovery"]
    elif mutation in ("null", "false", "empty"): envelope["source_block_recovery"] = {"null": None, "false": False, "empty": {}}[mutation]
    elif mutation == "foreign_physics": envelope["contract_evidence"] = None
    elif mutation == "wrong_profile": envelope["formulation_basis"]["profile_id"] = "exact_straight_pressure_v2"
    elif mutation == "wrong_ref": envelope["semantic_contract_ref"]["ref_id"] = PRECISION_CONTRACT_ID
    elif mutation == "downgrade":
        envelope["producer"]["semantic_contract_id"] = PRECISION_CONTRACT_ID
        envelope["semantic_contract_ref"]["ref_id"] = PRECISION_CONTRACT_ID
    elif mutation == "carrier_null": envelope["carrier_evidence"] = None
    else:
        envelope["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/physics-source-1"
        envelope["semantic_contract_ref"]["ref_id"] = "openpipestress.result_semantics/0.3.0/physics-source-1"
    with pytest.raises(AssertionError): require_schema("results.v0.3.schema.yaml", document)


COMPOSITE_ID = "openpipestress.result_semantics/0.3.0/physics-source-1"
COMPOSITE_SHA256 = "ba13f2aefd7a38bd725e5f111e6ec30144bc8776aa957c6278ee7b1178298ba1"
COMPOSITE_RECEIPT = "physics_source_recovery.schema.json"
COMPOSITE_ROOT = PROJECT / "fixtures/product_preview/physics_source"
COMPOSITE_PAIRS = tuple(f"{case}-{mode}" for case in ("n05", "n06", "mixed", "fields") for mode in MODES)


def test_composite_schema_is_distinct_and_preserves_original_closed_receipt_shapes():
    original, composite = schema(RECEIPT_REF), schema(COMPOSITE_RECEIPT)
    validate_schema_document(composite)
    assert original["$id"] == "openpipestress.source_block_receipt/1.0.0"
    assert composite["$id"] == "openpipestress.physics_source_receipt/1.0.0"
    assert original["$defs"]["receipt_body"]["properties"]["policy"] == {"const": "SOURCE-BLOCKS-1"}
    assert composite["$defs"]["receipt_body"]["properties"]["policy"] == {"const": "PHYSICS-SOURCE-1"}
    for name in ("basis", "failure", "ordinary", "projection", "support_component", "support", "invocation"):
        assert composite["$defs"][name] == original["$defs"][name]
    for name in ("results.v0.3.schema.yaml", "analysis_run.v0.3.schema.json"):
        validate_schema_document(schema(name))
    assert hashlib.sha256((PROJECT / "fixtures/results/semantic_contract_v0_3_physics_source_1.json").read_bytes()).hexdigest() == COMPOSITE_SHA256


@pytest.mark.parametrize("name", ("n05-sparse_interactive", "ui/multicase-dense_scrutiny"))
def test_original_receipt_never_matches_composite_schema_without_relabeling(name):
    receipt = source_analysis(name)[0]["source_block_recovery"]
    require_schema(RECEIPT_REF, receipt)
    with pytest.raises(AssertionError): require_schema(COMPOSITE_RECEIPT, receipt)


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


@pytest.mark.parametrize("mutation", ("missing_physical", "missing_receipt", "source1_policy", "wrong_hash", "wrong_row_method", "missing_case_binding", "missing_sections", "unknown_norm", "ordinary_maximum_for_source", "foreign_carrier"))
def test_composite_analysis_closed_wire_rejects_cross_method_or_incomplete_evidence(mutation):
    _, _, original = composite_analysis("n05-sparse_interactive")
    record = deepcopy(original); run = record["analysis_run"]
    if mutation == "missing_physical": del run["contract_evidence"]
    elif mutation == "missing_receipt": del run["source_block_recovery"]
    elif mutation == "source1_policy": run["source_block_recovery"]["body"]["policy"] = "SOURCE-BLOCKS-1"
    elif mutation == "wrong_hash": run["reproducibility"]["semantic_contract"]["sha256"] = SOURCE_BLOCKS_CONTRACT_SHA256
    elif mutation == "wrong_row_method": run["result_refs"][0]["semantic_contract"].update(id=PHYSICS_CONTRACT_ID, sha256=PHYSICS_CONTRACT_SHA256)
    elif mutation == "missing_case_binding": del run["source_block_recovery"]["body"]["cases"][0]["physical_evidence_sha256"]
    elif mutation == "missing_sections": del run["source_block_recovery"]["body"]["cases"][0]["source"]["endpoint_sections"]
    elif mutation == "unknown_norm": run["source_block_recovery"]["body"]["cases"][0]["derived_checks"][0]["recipe_id"] = "unregistered_norm"
    elif mutation == "ordinary_maximum_for_source": run["contract_evidence"]["exact_cases"][0]["recovery_method"] = "ordinary_sparse_structural_v1"
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


def test_composite_work_limit_changes_only_its_policy_bound():
    original, composite = schema(RECEIPT_REF), schema(COMPOSITE_RECEIPT)
    old_work, new_work = original["$defs"]["work"], composite["$defs"]["work"]
    assert old_work["properties"]["limit"]["maximum"] == 4_000_000
    assert new_work["properties"]["limit"]["maximum"] == 8_000_000
    # Scalar schema boundaries, not fabricated producer receipts or work claims.
    validate_instance(new_work["properties"]["limit"], 8_000_000)
    with pytest.raises(AssertionError): validate_instance(old_work["properties"]["limit"], 8_000_000)
    with pytest.raises(AssertionError): validate_instance(new_work["properties"]["limit"], 8_000_001)
    for key in ("limit", "charged", "publication_charged"):
        assert composite["$defs"]["receipt_body"]["properties"]["invocation_work"]["properties"][key]["maximum"] == 64_000_000
    comparable = deepcopy(new_work)
    comparable["properties"]["limit"]["maximum"] = old_work["properties"]["limit"]["maximum"]
    comparable["description"] = old_work["description"]
    assert comparable == old_work


def test_actual_composite_maximum_metadata_has_a_method_scoped_canonical_route():
    source = read_received(COMPOSITE_ROOT / "n05-sparse_interactive.raw.json")
    actual = next(row["metadata"] for row in source["results"]
                  if row.get("metadata", {}).get("basis") == "retained_source_endpoint_normal_max_v1")
    results = schema("results.v0.3.schema.yaml")
    validate_instance(schema_for_definition(results, "PhysicsSourceMaximumMetadata"), actual)
    with pytest.raises(AssertionError): validate_instance(schema_for_definition(results, "ResultMetadata"), actual)
    # Validate the exact branch wiring; canonical packet positives remain the
    # separately supplied actual headless producer artifacts, never constructed here.
    for index, branch in enumerate(results["$defs"]["ResultEnvelope"]["oneOf"]):
        target = "PhysicsSourceResultSet" if index in (3, 5) else "ResultSet"
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
    with pytest.raises(AssertionError): require_schema(RECEIPT_REF, incorrect)
