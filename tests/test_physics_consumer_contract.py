"""Reader checks use preserved outputs from actual joined sparse/dense invocations."""
from copy import deepcopy
import hashlib
import json
from pathlib import Path

import pytest
from core.analysis_runs.compatibility import (
    PHYSICS_CONTRACT_ID, PHYSICS_CONTRACT_SHA256, PRECISION_CONTRACT_ID,
    build_analysis_run, numerical_use_standing, validate_analysis_run_v0_3,
    verify_analysis_run_record,
)
from core.analysis_runs.physics_evidence import validate_physics_evidence
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1
from schema_validation import validate_instance

PROJECT = Path(__file__).resolve().parents[1]


def actual(mode="sparse"):
    return json.loads((PROJECT / f"fixtures/results/physics_connected_mechanics_{mode}.json").read_text())


def build(source):
    return build_analysis_run(source, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:reader-test"}, input_manifest_hash="1" * 64)


def bases(source):
    return [case["basis_ref"] for case in source["numerical_quality"]["cases"]]


@pytest.mark.parametrize("mode", ["sparse", "dense"])
def test_actual_physics_source_analysis_round_trip(mode):
    source = actual(mode)
    original = deepcopy(source)
    record = build(source)
    assert source == original
    assert verify_analysis_run_record(record) == "match"
    validate_analysis_run_v0_3(record, source)
    validate_instance(json.loads((PROJECT / "schemas/analysis_run.schema.json").read_text()), record, instance_label="actual physics analysis")
    assert record["analysis_run"]["reproducibility"]["semantic_contract"] == {"id": PHYSICS_CONTRACT_ID, "sha256": PHYSICS_CONTRACT_SHA256}
    assert numerical_use_standing(source, bases(source)) == "numerically_eligible"
    assert len(record["analysis_run"]["result_refs"]) == len(source["results"])
    received = next(item for item in record["analysis_run"]["hashes"] if item["payload_scope"] == "received_result")
    assert received["value"] == canonical_sha256_checked_v1(source)
    assert all(item["interpretation"]["status"] != "unavailable" for item in record["analysis_run"]["result_refs"])
    # Explicitly unpressurized case retains all member evidence, without a fake region.
    assert len(source["contract_evidence"]["exact_cases"]) == 2
    assert len(source["contract_evidence"]["pressure"]) == 1


def test_physics_table_identity_and_frozen_precision_table():
    assert hashlib.sha256((PROJECT / "fixtures/results/semantic_contract_v0_3_physics_1.json").read_bytes()).hexdigest() == PHYSICS_CONTRACT_SHA256
    assert hashlib.sha256((PROJECT / "fixtures/results/semantic_contract_v0_3_precision_1.json").read_bytes()).hexdigest() == "d75aacee175e178dbdeb256d89a65f4b375265f7da077725ee635af33df51d7e"


@pytest.mark.parametrize("change", [
    lambda s: s.pop("contract_evidence"),
    lambda s: s["contract_evidence"]["exact_cases"].pop(),
    lambda s: s["contract_evidence"]["pressure"].append(deepcopy(s["contract_evidence"]["pressure"][0])),
    lambda s: s["contract_evidence"]["pressure"][0]["result_ids"].pop(),
    lambda s: s["contract_evidence"]["pressure"][0].update(load_case_id="case:six-component-load"),
    lambda s: s["contract_evidence"]["pressure"][0]["geometry"][0].update(As_m2=1),
    lambda s: s["contract_evidence"]["pressure"][0]["materials"][0].update(nu=0.2),
    lambda s: s["contract_evidence"]["exact_cases"][1]["pipe_materials"].clear(),
    lambda s: s["contract_evidence"]["exact_cases"][0]["pipe_materials"][0].update(alpha_per_kelvin=0),
    lambda s: s["contract_evidence"]["exact_cases"][0]["pipe_sections"][0].update(As_m2=float("inf")),
    lambda s: s["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0].update(result_id="missing"),
    lambda s: s["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"].update(method="source_block_recovery_v1"),
    lambda s: s["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"][0]["terms"][0].update(region_id="missing"),
    lambda s: s["contract_evidence"].update(source_block_recovery={}),
    lambda s: s["summary"]["max_open_formula_stress"].update(value=1),
    lambda s: s["numerical_quality"]["cases"][0].update(evidence_refs=s["numerical_quality"]["cases"][1]["evidence_refs"]),
    lambda s: s["contract_evidence"]["pressure"][0].update(p_pa=1),
    lambda s: s.update(source_block_recovery={}),
])
def test_physics_source_corruption_cannot_gain_semantic_or_numerical_admission(change):
    source = actual()
    change(source)
    with pytest.raises(ValueError, match="SOURCE_"):
        build(source)
    assert numerical_use_standing(source, bases(source)) == "unsupported"


@pytest.mark.parametrize("identity", [PRECISION_CONTRACT_ID, "openpipestress.result_semantics/0.3.0/source-blocks-1", "openpipestress.result_semantics/0.3.0/precision-2"])
def test_physics_never_falls_back_to_other_contract(identity):
    source = actual()
    source["producer"]["semantic_contract_id"] = identity
    with pytest.raises(ValueError, match="SOURCE_"):
        build(source)


@pytest.mark.parametrize("status", ["sensitive", "not_assessed", "unresolved", "failed"])
def test_actual_physics_carrier_remains_inspectable_without_numerical_promotion(status):
    source = actual()
    source["numerical_quality"]["status"] = status
    for case in source["numerical_quality"]["cases"]:
        case["solve_quality"] = status
        case["accuracy_evidence"] = "reference_verified"
    assert verify_analysis_run_record(build(source)) == "match"
    assert numerical_use_standing(source, bases(source)) == "needs_recompute"


def test_physics_evidence_schema_is_closed_and_case_scoped():
    import jsonschema
    schema = json.loads((PROJECT / "schemas/results.v0.3.schema.yaml").read_text())
    validator = jsonschema.Draft202012Validator({"$ref": "#/$defs/PhysicsContractEvidence", "$defs": schema["$defs"]})
    source = actual()
    validator.validate(source["contract_evidence"])
    validate_physics_evidence(source)
    source["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]["extra"] = "unreviewed"
    with pytest.raises(jsonschema.ValidationError):
        validator.validate(source["contract_evidence"])


def test_unavailable_member_maximum_withholds_headline_but_keeps_signed_rows():
    source = actual()
    case = source["contract_evidence"]["exact_cases"][0]
    extremum = case["pipe_stress_extrema"].pop()
    source["results"] = [row for row in source["results"] if row["id"] != extremum["result_id"]]
    region = source["contract_evidence"]["pressure"][0]
    region["result_ids"].remove(extremum["result_id"])
    case["stress_maximum_coverage"] = {"complete": False, "unavailable_pipe_ids": [extremum["pipe_id"]]}
    headline = source["summary"].pop("max_open_formula_stress")
    validate_physics_evidence(source)
    assert verify_analysis_run_record(build(source)) == "match"
    source["summary"]["max_open_formula_stress"] = headline
    with pytest.raises(ValueError, match="headline"):
        validate_physics_evidence(source)


def test_actual_headless_physics_derivative_and_analysis_share_the_full_source():
    import os
    folder = os.environ.get("HEADLESS_PHYSICS_OUTPUT_DIR")
    if not folder:
        pytest.skip("Requires actual headless physics artifacts from the focused Rust lane")
    paths = list(Path(folder).glob("*.raw.json"))
    assert len(paths) == 2, "Both actual sparse/dense outputs are required"
    schema = json.loads((PROJECT / "schemas/results.v0.3.schema.yaml").read_text())
    for path in paths:
        source = json.loads(path.read_text())
        document = json.loads(path.with_name(path.name.replace(".raw.json", ".document.json")).read_text())
        validate_instance(schema, document, instance_label=f"actual headless physics {path.stem}")
        record = build(source)
        validate_analysis_run_v0_3(record, source)
        envelope = document["result_envelope"]
        for key in ["producer", "numerical_quality", "formulation_basis", "contract_evidence"]:
            assert envelope[key] == source[key]
        source_hash = canonical_sha256_checked_v1(source)
        received = next(item for item in record["analysis_run"]["hashes"] if item["payload_scope"] == "received_result")
        assert envelope["reproducibility"]["source_origin_bindings"][0]["received_carrier_checksum"]["value"] == received["value"] == source_hash
        assert envelope["semantic_contract_ref"]["ref_id"] == PHYSICS_CONTRACT_ID
        assert len(envelope["row_accounting"]) == len(source["results"])


def _inject_unbound_maximum(source):
    case_id = source["contract_evidence"]["exact_cases"][1]["load_case_id"]
    row = deepcopy(next(row for row in source["results"] if row["kind"] == "pipe_elastic_normal_stress_maximum_v2" and row["basis_ref"]["ref_id"] == case_id))
    row.update(id="unbound:maximum", value=100000000.25)
    source["results"].append(row)
    source["summary"]["max_open_formula_stress"] = {"value": row["value"], "unit": row["unit"], "location_ref": row["entity_ref"], "result_ref": row["id"]}


def _cyclic_nonphysical_operand(source):
    row = next(row for row in source["results"] if not row["kind"].endswith("_v2"))
    row["source_result_refs"] = [{"ref_type": "result_value", "ref_id": row["id"]}]


def _duplicate_rhs_group(source):
    groups = source["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"]
    groups.append(deepcopy(groups[0]))


def _remote_closure_contradiction(source):
    terminal = source["contract_evidence"]["pressure"][0]["terminals"][0]
    terminal.update(closure_transfer="separately_supported_or_compensated", remote_closure_excluded_from_pipe_solve=True, pipe_cap_transfer_global_n=[0, 0, 0], remote_closure_support_reaction_global_n=[999, 0, 0])


@pytest.mark.parametrize("mode", ["sparse", "dense"])
@pytest.mark.parametrize("change", [
    pytest.param(_inject_unbound_maximum, id="unbound-maximum-headline"),
    pytest.param(lambda s: s["contract_evidence"]["exact_cases"][1]["pipe_stress_extrema"][0].update(certified_gap_pa=0), id="false-zero-gap"),
    pytest.param(lambda s: s["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0].update(coefficient_basis="source_blocks_recovery_v1"), id="foreign-coefficient-basis"),
    pytest.param(lambda s: s["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0].update(approximation="unknown"), id="foreign-extrema-approximation"),
    pytest.param(lambda s: s["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"][0].update(enclosure_scope="unknown"), id="foreign-enclosure-scope"),
    pytest.param(lambda s: s["contract_evidence"]["pressure"][0]["terminals"][0].update(pipe_cap_transfer_global_n=[999, 0, 0]), id="wall-closure-transfer"),
    pytest.param(lambda s: s["contract_evidence"]["pressure"][0]["terminals"].__setitem__(1, deepcopy(s["contract_evidence"]["pressure"][0]["terminals"][0])), id="duplicate-terminals"),
    pytest.param(_remote_closure_contradiction, id="remote-closure-reaction"),
    pytest.param(lambda s: s["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"][0]["terms"][0].update(coefficient=999), id="poisson-coefficient"),
    pytest.param(lambda s: next(t for t in s["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"][0]["terms"] if t["kind"] == "terminal_cap").update(coefficient=999), id="cap-coefficient"),
    pytest.param(_duplicate_rhs_group, id="duplicate-rhs-group"),
    pytest.param(_cyclic_nonphysical_operand, id="cyclic-generic-row"),
    pytest.param(lambda s: s["results"][0].update(source_result_refs={}), id="malformed-generic-operands"),
    pytest.param(lambda s: s["contract_evidence"]["pressure"][0]["materials"][0].update(temperature_basis={"selection": "exact_point", "point_id": "missing"}), id="contradictory-temperature-selection"),
    pytest.param(lambda s: s["contract_evidence"]["pressure"][0]["applied_loads"][0].update(thermal_included=True), id="thermal-in-pressure-only-ledger"),
])
def test_review_corruptions_refused_before_record_or_numerical_admission(mode, change):
    source = actual(mode)
    change(source)
    with pytest.raises(ValueError, match="SOURCE_PHYSICS_EVIDENCE_INVALID"):
        validate_physics_evidence(source)
    with pytest.raises(ValueError, match="SOURCE_PHYSICS_EVIDENCE_INVALID"):
        build(source)
    assert numerical_use_standing(source, bases(source)) == "unsupported"


@pytest.mark.parametrize("field", ["assembled_pressure_rhs_global", "rounded_cap_rhs_global", "rounded_poisson_rhs_global"])
@pytest.mark.parametrize("slot", [3, 4, 5, 9, 10, 11])
def test_pressure_rhs_force_only_vectors_reject_moments(field, slot):
    source = actual()
    source["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"][field][slot] = 999
    with pytest.raises(ValueError, match="pressure moment"):
        validate_physics_evidence(source)


@pytest.mark.parametrize("mode", ["sparse", "dense"])
def test_actual_pressure_and_thermal_source_preserves_pressure_only_ledger(mode):
    source = json.loads((PROJECT / f"fixtures/results/physics_thermal_ui_mechanics_{mode}.json").read_text())
    original = deepcopy(source)
    assert source["status"]["mechanics"] == "MECHANICS_SOLVED"
    region = source["contract_evidence"]["pressure"][0]
    assert region["materials"][0]["thermal_consumed"] is True
    assert region["applied_loads"][0]["thermal_included"] is False
    validate_physics_evidence(source)
    assert verify_analysis_run_record(build(source)) == "match"
    assert numerical_use_standing(source, bases(source)) == "numerically_eligible"
    assert source == original


@pytest.mark.parametrize("name", ["missing_nu", "missing_closure"])
def test_actual_blocked_empty_evidence_remains_inspectable(name):
    import os
    folder = os.environ.get("PHYSICS_BLOCKED_OUTPUT_DIR")
    if not folder:
        pytest.skip("Requires actual blocked producer controls from the focused product lane")
    source = json.loads((Path(folder) / f"{name}.raw.json").read_text())
    assert source["status"]["mechanics"] != "MECHANICS_SOLVED"
    assert source["contract_evidence"] == {"pressure": [], "connector": [], "exact_cases": []}
    validate_physics_evidence(source)
    assert verify_analysis_run_record(build(source)) == "match"
    assert numerical_use_standing(source, bases(source)) != "numerically_eligible"
