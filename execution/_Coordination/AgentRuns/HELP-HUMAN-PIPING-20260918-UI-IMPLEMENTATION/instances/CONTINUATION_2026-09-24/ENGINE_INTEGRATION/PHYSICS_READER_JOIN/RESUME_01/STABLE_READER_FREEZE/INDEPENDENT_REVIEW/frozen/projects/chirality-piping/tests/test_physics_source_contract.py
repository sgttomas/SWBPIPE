"""Actual composite producer records and rehashed adversarial copies; no native witness."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import pytest
from core.analysis_runs import physics_source as composite
from core.analysis_runs.physics_evidence import validate_physics_evidence, validate_transport_metadata as physics_transport
from core.analysis_runs.source_blocks import validate_source_blocks, domain_hash, _signature

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "fixtures/product_preview/physics_source"


def actual(name="n05", mode="sparse_interactive"):
    return (json.loads((FIXTURES / f"{name}-{mode}.raw.json").read_text()),
            {"request": json.loads((FIXTURES / f"{name}.request.json").read_text()), "solver_mode": mode})


def reseal_negative(source, invocation=None):
    """Recompute hashes of a deliberately false statement; never a positive fixture."""
    receipt = source["source_block_recovery"]
    body = receipt["body"]
    if invocation is not None:
        body["invocation"]["value"] = domain_hash("source_blocks_invocation_v1", invocation)
    for record in body["cases"]:
        physical = next(c for c in source["contract_evidence"]["exact_cases"] if c["load_case_id"] == record["basis_ref"]["ref_id"])
        pressure = [p for p in source["contract_evidence"]["pressure"] if p["load_case_id"] == physical["load_case_id"]]
        record["physical_evidence_sha256"] = domain_hash("physics_source_case_evidence_v1", {"exact_case":physical,"pressure":pressure})
    body["publication_sha256"] = domain_hash("source_blocks_publication_v1", {k:v for k,v in source.items() if k != "source_block_recovery"})
    receipt["receipt_sha256"] = domain_hash("source_blocks_receipt_v1",body)


@pytest.mark.parametrize("name", ["n05", "n06", "mixed", "fields"])
@pytest.mark.parametrize("mode", ["sparse_interactive", "dense_scrutiny"])
def test_actual_producer_requires_original_full_invocation(name, mode):
    source, invocation = actual(name, mode)
    original = deepcopy(source)
    assert composite.validate_physics_source(source, invocation)
    assert composite.validate_physics_source(source) is False
    composite.validate_transport_metadata({"contract_evidence":source["contract_evidence"], "source_block_recovery":source["source_block_recovery"]})
    assert source == original
    altered = deepcopy(invocation)
    altered["request"]["accepted_unknown_field"] = False
    with pytest.raises(ValueError, match="INVOCATION_HASH"):
        composite.validate_physics_source(source, altered)
    altered = deepcopy(invocation)
    altered["solver_mode"] = "dense_scrutiny" if mode == "sparse_interactive" else "sparse_interactive"
    with pytest.raises(ValueError, match="INVOCATION_HASH"):
        composite.validate_physics_source(source, altered)


def test_previous_three_table_identities_remain_frozen():
    expected={"precision_1":"d75aacee175e178dbdeb256d89a65f4b375265f7da077725ee635af33df51d7e", "source_blocks_1":"5f299065f15a157bbedf9467a598994ae684c4ecb3f851bbcb291981ec550a9f"}
    from core.analysis_runs.compatibility import PHYSICS_CONTRACT_SHA256
    expected["physics_1"]=PHYSICS_CONTRACT_SHA256
    for name,digest in expected.items():
        assert hashlib.sha256((ROOT/f"fixtures/results/semantic_contract_v0_3_{name}.json").read_bytes()).hexdigest()==digest


def test_maximum_basis_is_a_signature_discriminator_only():
    source,_=actual()
    row=deepcopy(next(r for r in source["results"] if r["kind"]=="pipe_elastic_normal_stress_maximum_v2"))
    exact=_signature(row,composite)
    assert exact["source_basis"]==composite.MAX_BASIS
    row["metadata"]["basis"]="recovered_from_open_mechanics_stress_components"
    ordinary=_signature(row,composite)
    assert ordinary["signature_id"]!=exact["signature_id"]
    for basis in [None,"invented"]:
        if basis is None:row["metadata"].pop("basis")
        else:row["metadata"]["basis"]=basis
        assert _signature(row,composite) is None


def test_old_method_entrypoints_refuse_composite_and_cross_namespace_sources():
    source,invocation=actual()
    with pytest.raises(ValueError):validate_source_blocks(source,invocation)
    with pytest.raises(ValueError):validate_physics_evidence(source)
    for path in [ROOT/"fixtures/results/physics_connected_mechanics_sparse.json",ROOT/"fixtures/product_preview/source_blocks/n05-sparse_interactive.raw.json"]:
        old=json.loads(path.read_text())
        with pytest.raises(ValueError):composite.validate_physics_source(old)


@pytest.mark.parametrize("mutation", [
    "maximum_basis", "maximum_action", "maximum_interval", "maximum_functional", "maximum_section", "maximum_source_identity", "maximum_location", "maximum_criterion", "stress_operand", "stress_recipe", "stress_section", "stress_row", "stress_missing", "norm_interval", "norm_functional", "norm_missing", "support_owner", "projection_bits", "source_member", "ordinary_quality", "case_hash", "foreign_namespace", "case_work", "invocation_work", "pressure_presence", "material_selection", "geometry_selection",
])
def test_rehashed_invalid_composite_claims_are_rejected(mutation):
    source,invocation=actual()
    source=deepcopy(source);invocation=deepcopy(invocation)
    case=source["source_block_recovery"]["body"]["cases"][0]
    physical=source["contract_evidence"]["exact_cases"][0]
    maximum=physical["pipe_stress_extrema"][0]
    stress=case["section_stress_checks"][0]
    norm=case["derived_checks"][0]
    if mutation=="maximum_basis":maximum["basis"]="recovered_from_open_mechanics_stress_components"
    elif mutation=="maximum_action":maximum["endpoints"][0]["actions"][0]["value"]=1
    elif mutation=="maximum_interval":maximum["value_upper_pa"]=1
    elif mutation=="maximum_functional":maximum["endpoints"][0]["functional_indices"][0]+=1
    elif mutation=="maximum_section":maximum["area_m2"]*=2
    elif mutation=="maximum_source_identity":maximum["source_identity_sha256"]="0"*64
    elif mutation=="maximum_location":maximum["locations"]={"kind":"strict_endpoint","endpoint":"i"}
    elif mutation=="maximum_criterion":maximum["relative_limit"]=1e-8
    elif mutation=="stress_operand":stress["action"]["value"]=1
    elif mutation=="stress_recipe":stress["recipe_id"]="straight_open_stress_v1"
    elif mutation=="stress_section":stress["parameters"]["torsion_constant_m4"]*=2
    elif mutation=="stress_row":next(r for r in source["results"] if r["id"]==stress["result_id"])["value"]=1
    elif mutation=="stress_missing":case["section_stress_checks"].pop()
    elif mutation=="norm_interval":norm["interval"][1]+=1
    elif mutation=="norm_functional":norm["functional_indices"][0]+=1
    elif mutation=="norm_missing":case["derived_checks"].pop()
    elif mutation=="support_owner":case["supports"][0]["components"][0]["action_terms"][0]["source_id"]="other"
    elif mutation=="projection_bits":case["projections"][0]["value_bits"]="8000000000000000"
    elif mutation=="source_member":case["source"]["member_ids"][0]="other"
    elif mutation=="ordinary_quality":source["numerical_quality"]["cases"][0]["solve_quality"]="checks_passed"
    elif mutation=="case_hash":case["physical_evidence_sha256"]="0"*64
    elif mutation=="foreign_namespace":source["carrier_evidence"]={}
    elif mutation=="case_work":case["work"]["limit"]=8_000_001
    elif mutation=="invocation_work":source["source_block_recovery"]["body"]["invocation_work"]["charged"]+=1
    elif mutation=="pressure_presence":invocation["request"]["model"]["load_cases"][0]["pressure_regions"]=[{"p_pa":0}]
    elif mutation=="material_selection":invocation["request"]["model"]["materials"][0]["poisson_ratio"]["value"] = .3
    elif mutation=="geometry_selection":invocation["request"]["model"]["pipe_segments"][0]["section"]["wall_thickness"]["value"]*=.5
    reseal_negative(source,invocation)
    if mutation=="case_hash":
        case["physical_evidence_sha256"]="0"*64
        source["source_block_recovery"]["receipt_sha256"]=domain_hash("source_blocks_receipt_v1",source["source_block_recovery"]["body"])
    with pytest.raises(ValueError):composite.validate_physics_source(source,invocation)


def test_physics_transport_uses_received_evidence_without_raw_reconstruction():
    for name in ["physics_connected_mechanics_sparse","physics_thermal_ui_mechanics_sparse"]:
        source=json.loads((ROOT/f"fixtures/results/{name}.json").read_text())
        physics_transport({"contract_evidence":source["contract_evidence"]})
        bad=deepcopy(source["contract_evidence"])
        bad["exact_cases"][0]["pipe_stress_extrema"][0]["coefficient_basis"]="invented"
        with pytest.raises(ValueError):physics_transport({"contract_evidence":bad})
