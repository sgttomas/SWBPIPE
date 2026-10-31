"""Actual composite producer records and rehashed adversarial copies; no native witness.

B7b (A4/A5): the composite is pressure-source-1 (`3.0.0/exact_pressure_v3`, policy PRESSURE-SOURCE-1, case domain
`pressure_source_case_evidence_v1`) over the v3 producer twins; the retired physics-source-1 is not read.
"""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import math
import struct
import sys
import pytest
from tests.verifiers import physics_source as composite
from tests.verifiers import section_basis as sb
from tests.verifiers.physics_evidence import validate_transport_metadata as physics_transport
from tests.verifiers.pressure_2 import validate_pressure_evidence
from tests.verifiers.source_blocks import domain_hash, _derived, _signature

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "fixtures/product_preview/pressure_source/physics_source"
CASE_DOMAIN = composite.SourceContract.PRESSURE_V3.case_domain


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
        record["physical_evidence_sha256"] = domain_hash(CASE_DOMAIN, {"exact_case":physical,"pressure":pressure})
    body["publication_sha256"] = domain_hash("source_blocks_publication_v1", {k:v for k,v in source.items() if k != "source_block_recovery"})
    receipt["receipt_sha256"] = domain_hash("source_blocks_receipt_v1",body)


@pytest.mark.parametrize("name", ["n05", "n06", "mixed", "fields", "n05_units", "mixed_units", "n05_unicode"])
@pytest.mark.parametrize("mode", ["sparse_interactive", "dense_scrutiny"])
def test_actual_producer_requires_original_full_invocation(name, mode):
    source, invocation = actual(name, mode)
    original = deepcopy(source)
    assert composite.validate_physics_source(source, invocation)
    assert composite.validate_physics_source(source) is False
    composite.validate_transport_metadata({key: source[key] for key in ("producer", "formulation_basis", "contract_evidence", "source_block_recovery")})
    assert source == original
    altered = deepcopy(invocation)
    altered["request"]["accepted_unknown_field"] = False
    with pytest.raises(ValueError, match="INVOCATION_HASH"):
        composite.validate_physics_source(source, altered)
    altered = deepcopy(invocation)
    altered["solver_mode"] = "dense_scrutiny" if mode == "sparse_interactive" else "sparse_interactive"
    with pytest.raises(ValueError, match="INVOCATION_HASH"):
        composite.validate_physics_source(source, altered)


def test_current_table_identities_remain_pinned():
    # B1 deleted precision-1's table, B4/D1 (b) source-blocks-1's and B7b the v2 tables; the composite's own
    # current table and the ordinary pressure-2 table stay pinned, each resident on its own.
    for retired in ("precision_1", "pressure_1", "preview_physics_1", "source_blocks_1", "physics_1", "physics_source_1", "load_reference_1", "load_reference_source_1"):
        assert not (ROOT/f"fixtures/results/semantic_contract_v0_3_{retired}.json").exists()
    from core.analysis_runs.compatibility import PRESSURE_CONTRACT_SHA256, PRESSURE_SOURCE_CONTRACT_SHA256
    expected={"pressure_source_1":PRESSURE_SOURCE_CONTRACT_SHA256, "pressure_2":PRESSURE_CONTRACT_SHA256}
    for name,digest in expected.items():
        assert hashlib.sha256((ROOT/f"fixtures/results/semantic_contract_v0_3_{name}.json").read_bytes()).hexdigest()==digest
    assert composite.PRESSURE_TABLE_SHA256==PRESSURE_SOURCE_CONTRACT_SHA256
    assert composite.pressure_source_table()["source_block_policy"]=="PRESSURE-SOURCE-1"


def test_maximum_basis_is_a_signature_discriminator_only():
    source,_=actual()
    row=deepcopy(next(r for r in source["results"] if r["kind"]=="pipe_elastic_normal_stress_maximum_v2"))
    exact=_signature(row,composite.SourceContract.PRESSURE_V3)
    # T4-U6 (D-5, §2.4): the source maximum names its section basis.
    token,section=sb.split_token(exact["source_basis"])
    assert token==composite.MAX_BASIS and row["metadata"]["basis"]==exact["source_basis"]
    row["metadata"]["basis"]=f"{token}{sb.SEPARATOR}{sb.other(section)}"
    twin=_signature(row,composite.SourceContract.PRESSURE_V3)
    assert twin["signature_id"]!=exact["signature_id"]
    row["metadata"]["basis"]=f"recovered_from_open_mechanics_stress_components{sb.SEPARATOR}{section}"
    ordinary=_signature(row,composite.SourceContract.PRESSURE_V3)
    assert ordinary["signature_id"]!=exact["signature_id"]
    # A basis-less token no longer names a signature of a stress kind.
    for basis in [None,"invented",composite.MAX_BASIS,"recovered_from_open_mechanics_stress_components"]:
        if basis is None:row["metadata"].pop("basis")
        else:row["metadata"]["basis"]=basis
        assert _signature(row,composite.SourceContract.PRESSURE_V3) is None


def test_other_entrypoints_refuse_composite_and_cross_namespace_sources():
    source,invocation=actual()
    # The ordinary pressure-2 reader refuses the composite's receipt namespace.
    with pytest.raises(ValueError):validate_pressure_evidence(source)
    retired=deepcopy(source);retired["producer"]["semantic_contract_id"]="openpipestress.result_semantics/0.3.0/source-blocks-1"
    v2=deepcopy(source);v2["producer"]["semantic_contract_id"]="openpipestress.result_semantics/0.3.0/physics-source-1";v2["formulation_basis"]["profile_id"]="exact_straight_pressure_v2"
    for old in [json.loads((ROOT/"fixtures/results/pressure_connected_mechanics_sparse.json").read_text()),retired,v2]:
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


def _zero_action_twins(source):
    """The first reduced section-stress check whose action is an exact positive zero, and its nominal twin before it."""
    checks=source["source_block_recovery"]["body"]["cases"][0]["section_stress_checks"]
    reduced=next(i for i,c in enumerate(checks) if c["result_id"].endswith(":reduced-section") and struct.pack(">d",c["action"]["value"])==bytes(8))
    nominal=reduced-1
    assert checks[reduced]["result_id"]==checks[nominal]["result_id"].replace(":nominal-section",":reduced-section")
    assert checks[reduced]["action"]==checks[nominal]["action"]=={"interval":[0.0,0.0],"value":0.0}
    return checks,nominal,reduced


@pytest.mark.parametrize("twin", ["nominal", "reduced"])
@pytest.mark.parametrize("member", ["interval0", "interval1", "value"])
def test_reduced_stress_check_refuses_a_signed_zero_twin(twin, member):
    # T4-U6 (D-5 §2.5, §5; b7-plan R2 A2): the reduced twin's check binds its nominal twin's action bit for bit;
    # -0.0 equals 0.0 by value, not by bits. Each twin's action is also its station's, bit for bit
    # (STRESS_FUNCTIONAL_LINK, read first), so a signed-zero action is refused there on either twin; written `0` it is
    # the same binary64 number and is accepted (Rust's twin: reduced_stress_check_binds_its_nominal_twin_bit_for_bit).
    for value, code in ((-0.0, "STRESS_FUNCTIONAL_LINK"), (0, None)):
        source,invocation=actual()
        checks,nominal,reduced=_zero_action_twins(source)
        action=checks[nominal if twin=="nominal" else reduced]["action"]
        if member=="value":action["value"]=value
        else:action["interval"][int(member[-1])]=value
        reseal_negative(source)
        if code is None:
            assert composite.validate_physics_source(source,invocation)
        else:
            with pytest.raises(ValueError,match=f"^PHYSICS_SOURCE_{code}$"):composite.validate_physics_source(source,invocation)


def _zero_links(source):
    """Rust `retained_source_links_compare_numbers_by_bits`'s sites: each an exact positive zero on one side of a link
    (path from the document root, its refusal code)."""
    record=source["source_block_recovery"]["body"]["cases"][0]
    zero=lambda v: type(v) in (int,float) and struct.pack(">d",v)==bytes(8)
    # A midspan station action of a stress component (axial 0, torsion 3, bending 4 and 5) with an exact positive zero:
    # stress checks link it, no endpoint section does.
    station=record["source"]["section_functionals"][0]["stations"][2]
    component=next(c for c in (0,3,4,5) if zero(station["actions"][c]["value"]))
    assert sum(c["functional_index"]==station["functional_indices"][component] for c in record["section_stress_checks"])==2
    extremum=next(i for i,e in enumerate(source["contract_evidence"]["exact_cases"][0]["pipe_stress_extrema"]) if e["section_basis"]==sb.NOMINAL)
    norm=next(i for i,c in enumerate(record["derived_checks"]) if all(zero(v) for v in c["intervals"][0]))
    receipt=("source_block_recovery","body","cases",0)
    sites=[(receipt+("source","section_functionals",0,"stations",2,"actions",component,"value"),"STRESS_FUNCTIONAL_LINK"),
           (receipt+("source","endpoint_sections",0,"endpoints",0,"actions",0,"value"),"ENDPOINT_SECTION_LINK"),
           (("contract_evidence","exact_cases",0,"pipe_stress_extrema",extremum,"endpoints",0,"actions",0,"value"),"MAXIMUM_ENDPOINT_SOURCE"),
           (receipt+("derived_checks",norm,"intervals",0,0),"NORM_PROJECTION_BINDING")]
    for path,_ in sites:
        target=source
        for key in path:target=target[key]
        assert zero(target),path
    return sites


@pytest.mark.parametrize("site", range(4))
@pytest.mark.parametrize("value", [-0.0, 0])
def test_retained_source_links_compare_numbers_by_bits(site, value):
    # T4-U6 (A2): every link of the retained-source receipt compares its numbers by their binary64 bits, as the Rust
    # reader does: an exact positive zero on one side of a link written -0.0 is refused by name; written `0` it is the
    # same binary64 number and is accepted.
    source,invocation=actual()
    path,code=_zero_links(source)[site]
    target=source
    for key in path[:-1]:target=target[key]
    target[path[-1]]=value
    reseal_negative(source)
    if value==0 and type(value) is int:
        assert composite.validate_physics_source(source,invocation)
    else:
        with pytest.raises(ValueError,match=f"^PHYSICS_SOURCE_{code}$"):composite.validate_physics_source(source,invocation)


@pytest.mark.parametrize("twin", ["nominal", "reduced"])
def test_reduced_stress_check_reads_its_twin_by_bits_not_form(twin):
    # The same binary64 number in another JSON form (JCS writes 1e6 as 1000000) on either twin alone.
    source,invocation=actual()
    checks,nominal,reduced=_zero_action_twins(source)
    parameters=checks[nominal if twin=="nominal" else reduced]["parameters"]
    assert type(parameters["pa_per_mpa"]) is float
    parameters["pa_per_mpa"]=1_000_000
    reseal_negative(source)
    assert composite.validate_physics_source(source,invocation)


def test_physics_transport_uses_received_evidence_without_raw_reconstruction():
    # The exact successor's G7 transport check (results 0.3 ExactRetainedContractEvidence) over pressure-2 evidence.
    for name in ["pressure_connected_mechanics_sparse","pressure_thermal_ui_mechanics_sparse"]:
        source=json.loads((ROOT/f"fixtures/results/{name}.json").read_text())
        physics_transport({"contract_evidence":source["contract_evidence"]})
        bad=deepcopy(source["contract_evidence"])
        bad["exact_cases"][0]["pipe_stress_extrema"][0]["coefficient_basis"]="invented"
        with pytest.raises(ValueError):physics_transport({"contract_evidence":bad})


@pytest.mark.parametrize("field,unit", [("elastic_modulus","N"),("elastic_modulus","gpa"),("shear_modulus","K"),("poisson_ratio","none"),("thermal_expansion_coefficient","K")])
def test_actual_units_authority_rejects_wrong_dimension_and_unregistered_spelling(field, unit):
    source,invocation=actual("n05_units")
    changed=deepcopy(invocation)
    material=changed["request"]["model"]["materials"][0]
    material[field]={"value":200 if field!="poisson_ratio" else .25,"unit":unit}
    reseal_negative(source,changed)
    with pytest.raises(ValueError):composite.validate_physics_source(source,changed)


def isolated_torsion_recipe(action, radius, constant):
    """Synthetic scalar helper inputs only; never reseal or claim a producer result."""
    source,_=actual()
    case=source["source_block_recovery"]["body"]["cases"][0]
    check=next(c for c in case["section_stress_checks"] if c["component"]=="torsional_shear_stress")
    row=next(r for r in source["results"] if r["id"]==check["result_id"])
    exact=source["contract_evidence"]["exact_cases"][0]
    section=next(s for s in exact["pipe_sections"] if s["pipe_id"]==row["entity_ref"])
    member=next(m for m in case["source"]["section_functionals"] if m["pipe_id"]==row["entity_ref"])
    station=member["stations"][["end_i","quarter_1","midspan","quarter_3","end_j"].index(row["metadata"]["location"])]
    station["actions"][3]={"value":action,"interval":[action,action]}
    check["action"]=station["actions"][3]
    section.update(ro_m=radius)
    for basis in sb.BASES:  # T4-U6 (§2.3): the one source section binds both bases
        section[basis]["J_m4"]=constant
    check["parameters"].update(torsion_radius_m=radius,torsion_constant_m4=constant)
    row["value"]=action*radius/constant/1e6
    return source,case,row


def test_torsion_recipe_rejects_lossy_subnormal_intermediate():
    from fractions import Fraction
    import sys
    action,radius,constant=1e-280,1e-40,1e-160
    source,case,row=isolated_torsion_recipe(action,radius,constant)
    expected=Fraction.from_float(action)*Fraction.from_float(radius)/Fraction.from_float(constant)/1_000_000
    error=float(abs(Fraction.from_float(row["value"])-expected)/abs(expected))
    assert 0 < abs(action*radius) < sys.float_info.min
    assert abs(row["value"]) >= sys.float_info.min and error > 1e-9
    with pytest.raises(ValueError,match="STRESS_TORSION_INTERMEDIATE_RANGE"):
        composite.validate_stress(source,case,row,[])


@pytest.mark.parametrize("action", [0.0,-0.0,1e-200,-1e-200])
def test_torsion_recipe_keeps_structural_zero_and_normal_intermediates(action):
    source,case,row=isolated_torsion_recipe(action,1e-40,1e-160)
    composite.validate_stress(source,case,row,[])


def test_material_selection_normalizes_named_points_and_strict_temperature_brackets():
    # Isolated selection check against the actual mixed-unit material inputs;
    # altered selection below is not presented as a new producer result.
    source,invocation=actual("mixed_units")
    model=invocation["request"]["model"]
    authored=model["materials"][0]
    actual_case=model["load_cases"][1]
    canonical=composite._canonical_inputs(invocation)
    expected=source["contract_evidence"]["exact_cases"][1]["pipe_materials"][0]
    e,nu,alpha=composite._selected_material(authored,actual_case,canonical)
    assert composite._same(e,expected["E_pa"]) and composite._same(nu,expected["nu"])
    assert alpha is None
    point_case={"modulus_basis_ref":"point:lower"}
    assert composite._selected_material(authored,point_case,canonical)==(1000.0,.25,None)
    actual_case["modulus_basis_temperature"]={"value":0,"unit":"degC"}
    canonical=composite._canonical_inputs(invocation)
    with pytest.raises(StopIteration):composite._selected_material(authored,actual_case,canonical)


def test_selected_alpha_uses_same_normalized_material_basis():
    # Quantity/selection unit test only, using actual request shapes. No fake solve.
    _,invocation=actual("mixed_units")
    model=invocation["request"]["model"]
    authored=model["materials"][0]
    actual_case=model["load_cases"][1]
    a,b=authored["temperature_points"]
    a["thermal_expansion_coefficient"]={"value":1e-5,"unit":"1/degC"}
    b["thermal_expansion_coefficient"]={"value":1e-5,"unit":"1/degF"}
    canonical=composite._canonical_inputs(invocation)
    e,nu,alpha=composite._selected_material(authored,actual_case,canonical)
    t=canonical(actual_case["modulus_basis_temperature"],"temperature")
    t0,t1=[canonical(p["temperature"],"temperature") for p in (a,b)]
    f=(t-t0)/(t1-t0)
    expected=(1-f)*canonical(a["thermal_expansion_coefficient"],"thermal_expansion_coefficient")+f*canonical(b["thermal_expansion_coefficient"],"thermal_expansion_coefficient")
    assert composite._same(alpha,expected)
    assert composite._selected_material(authored,{"modulus_basis_ref":"point:upper"},canonical)[2]==canonical(b["thermal_expansion_coefficient"],"thermal_expansion_coefficient")


@pytest.mark.parametrize("e,nu,expected", [
    (float.fromhex("0x0.0000000000001p-1022"), .25, None),
    (float.fromhex("0x0.0000000000001p-1022"), 0.0, None),
    (float.fromhex("0x0.0000000000001p-1022"), -2.0**-54, None),
    (float.fromhex("0x0.0000000000001p-1022"), float.fromhex("-0x1.0000000000001p-54"), float.fromhex("0x0.0000000000001p-1022")),
    (float.fromhex("0x0.0000000000002p-1022"), .25, float.fromhex("0x0.0000000000001p-1022")),
    (float.fromhex("0x0.0000000000001p-1022"), -.5, float.fromhex("0x0.0000000000001p-1022")),
    (float.fromhex("0x1.fffffffffffffp+1023"), -.5, float.fromhex("0x1.fffffffffffffp+1023")),
    (float.fromhex("0x1.fffffffffffffp+1023"), float.fromhex("-0x1.0000000000001p-1"), None),
    (float.fromhex("0x1.fffffffffffffp+1023"), float.fromhex("-0x1.fffffffffffffp-2"), float.fromhex("0x1.fffffffffffffp+1023")),
    (float.fromhex("0x1.fffffffffffffp+1023"), 0, float.fromhex("0x1.fffffffffffffp+1022")),
])
def test_material_pair_matches_producer_scaled_boundary_domain(e, nu, expected):
    # Scalar source-domain controls, not fabricated producer output. Boundaries
    # follow pressure_exact.rs Scaled::to_f64 and IsotropicENu::new exactly.
    if expected is None:
        with pytest.raises(ValueError,match="ACTUAL_DERIVED_SHEAR_RANGE:boundary"):
            composite._checked_material_pair(e,nu,"boundary")
    else:
        assert composite._same(composite._checked_material_pair(e,nu,"boundary"),expected)


@pytest.mark.parametrize("stage", ["base","lower_bracket","upper_bracket","selected"])
def test_every_consumed_material_pair_checks_source_domain(stage):
    _,invocation=actual("mixed_units")
    material=invocation["request"]["model"]["materials"][0]
    selected_case=invocation["request"]["model"]["load_cases"][1]
    minimum=float.fromhex("0x0.0000000000001p-1022")
    if stage=="base":material["elastic_modulus"]={"value":minimum,"unit":"Pa"}
    elif stage in {"lower_bracket","upper_bracket"}:
        material["temperature_points"][0 if stage=="lower_bracket" else 1]["elastic_modulus"]={"value":minimum,"unit":"Pa"}
    else:
        selected_case.pop("modulus_basis_temperature")
        selected_case["modulus_basis_ref"]="point:lower"
        material["temperature_points"][0]["elastic_modulus"]={"value":minimum,"unit":"Pa"}
    canonical=composite._canonical_inputs(invocation)
    with pytest.raises(ValueError,match=f"ACTUAL_DERIVED_SHEAR_RANGE:{stage}"):
        composite._selected_material(material,selected_case,canonical)


@pytest.mark.parametrize("bad_endpoint", [0,1])
def test_public_rehashed_copy_cannot_hide_invalid_bracket_pair_behind_finite_selection(bad_endpoint):
    source,invocation=actual("mixed_units")
    points=invocation["request"]["model"]["materials"][0]["temperature_points"]
    points[bad_endpoint]["elastic_modulus"]={"value":float.fromhex("0x0.0000000000001p-1022"),"unit":"Pa"}
    points[1-bad_endpoint]["elastic_modulus"]={"value":4000.0,"unit":"Pa"}
    # This is the reviewer's deliberately false, rehashed copy; it is never an
    # actual producer/native witness. Its rounded selected E still equals 2000.
    reseal_negative(source,invocation)
    with pytest.raises(ValueError,match="ACTUAL_DERIVED_SHEAR_RANGE:"+("lower_bracket" if bad_endpoint==0 else "upper_bracket")):
        composite.validate_physics_source(source,invocation)


def test_e15_composite_norms_are_the_correctly_rounded_value():
    assert composite._norm([3.0, 4.0, 0.0]) == 5.0
    assert composite._norm([-1.327, -8.603, -8.186]) == float.fromhex("0x1.7e5fc308f80edp+3")
    assert composite._norm([7.944, 6.99]) == float.fromhex("0x1.529b4c153125ep+3")
    with pytest.raises(ValueError):
        composite._norm([1.0])


def translation_norm(values, output):
    """The composites' shared translation-norm recipe (relocated from the retired
    source-blocks-1 tests when B4/D1 (b) deleted them)."""
    kinds = ("global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z")
    inputs = [{"kind": kinds[i], "unit": "mm", "entity_ref": "norm-owner", "value": v} for i, v in enumerate(values)]
    row = {"kind": "displacement_magnitude", "unit": "mm", "entity_ref": "norm-owner", "value": output}
    return _derived("translation_norm_scaled_v1", row, inputs, composite.SourceContract.PRESSURE_V3)


def test_translation_norm_admits_normal_output_and_small_components():
    translation_norm([3.0, 4.0, 0.0], 5.0)
    small = math.ldexp(1.0, -600)
    translation_norm([small, 0.0, 0.0], small)
    translation_norm([sys.float_info.min, 0.0, 0.0], sys.float_info.min)
    translation_norm([1.0, math.ulp(0.0), 0.0], 1.0)
    translation_norm([-0.0, 0.0, -0.0], 0.0)


def test_e15_translation_norm_binds_the_correctly_rounded_value():
    # The published (3,4,0) control: the recipe gives exactly 5, so the independently
    # single-rounded 5 + 1 ulp that G7 admits is refused by the exact value binding;
    # where the former max-scaled recipe differs, its value is refused.
    controls = json.loads((ROOT / "fixtures/results/preview_magnitude_controls.json").read_text())["controls"]
    control = next(c for c in controls if c["id"] == "independent_single_rounding_one_ulp")
    unpack = lambda bits: struct.unpack(">d", bytes.fromhex(bits))[0]
    components = [unpack(b) for b in control["component_bits"]]
    one_ulp = unpack(control["magnitude_bits"])
    assert (components, one_ulp) == ([3.0, 4.0, 0.0], math.nextafter(5.0, math.inf))
    differing = [-1.327, -8.603, -8.186]
    correct = float.fromhex("0x1.7e5fc308f80edp+3")
    scale = max(abs(v) for v in differing)
    n = [v / scale for v in differing]
    scaled = scale * math.sqrt((n[0] * n[0] + n[1] * n[1]) + n[2] * n[2])
    assert scaled == math.nextafter(correct, 0.0)
    translation_norm(components, 5.0)
    with pytest.raises(ValueError, match="DERIVED_NORM_VALUE"):
        translation_norm(components, one_ulp)
    translation_norm(differing, correct)
    with pytest.raises(ValueError, match="DERIVED_NORM_VALUE"):
        translation_norm(differing, scaled)


def test_translation_norm_range_and_bit_identity_are_enforced():
    with pytest.raises(ValueError, match="DERIVED_NORM_RANGE"):
        translation_norm([math.ulp(0.0), 0.0, 0.0], math.ulp(0.0))
    with pytest.raises(ValueError, match="DERIVED_NORM_RANGE"):
        translation_norm([sys.float_info.max, sys.float_info.max, 0.0], 1.0)
    with pytest.raises(ValueError, match="DERIVED_NORM_VALUE"):
        translation_norm([1.0, 0.0, 0.0], math.nextafter(1.0, math.inf))
    with pytest.raises(ValueError, match="DERIVED_NORM_VALUE"):
        translation_norm([-0.0, 0.0, -0.0], -0.0)
    with pytest.raises(ValueError):
        translation_norm([math.nan, 0.0, 0.0], 0.0)


@pytest.mark.parametrize("recipe", ["support_force_norm_scaled_v1", "straight_open_stress_v1",
                                    "reviewed_stress_summary_v1", "section_property_from_source_v1"])
def test_source_blocks_1_recipes_are_unknown(recipe):
    # B4/D1 (b): these recipes went with source-blocks-1's preview case rows.
    row = {"kind": "displacement_magnitude", "unit": "mm", "entity_ref": "norm-owner", "value": 0.0}
    with pytest.raises(ValueError, match="DERIVED_RECIPE"):
        _derived(recipe, row, [], composite.SourceContract.PRESSURE_V3)


# Model document 0.9.0 (DEL-02-01 Design model-document-0.9.0.md §3.8): condition_sets and spectra other than absent or
# [], and any pipe's condition_set_ref (an empty string or null included), are declined by name, as the Rust reader
# (`condition_sets_spectra_and_set_references_are_declined_by_name`) and the retained admission (D1.3) decline them.
_SET = {"id": "cs", "provenance": "user", "conditions": [{"key": "OPE"}]}
_SPECTRUM = {"id": "sp", "points": [[0.1, 1.0], [1.0, 2.0]]}


@pytest.mark.parametrize("value,carried", [("absent", False), ([], False), ("one", True), (None, True), ({}, True)])
def test_model_0_9_0_condition_set_and_spectrum_predicates(value, carried):
    for member, item, predicate in (("condition_sets", _SET, composite.model_carries_condition_sets),
                                    ("spectra", _SPECTRUM, composite.model_carries_spectra)):
        model = {"pipe_segments": [{"id": "p"}]}
        if value != "absent":
            model[member] = [item] if value == "one" else value
        assert predicate(model) is carried, (member, value)


@pytest.mark.parametrize("reference,carried", [("absent", False), ("cs", True), ("", True), (None, True)])
def test_model_0_9_0_condition_set_ref_predicate(reference, carried):
    model = {"pipe_segments": [{"id": "p"}, {"id": "q"}]}
    if reference != "absent":
        model["pipe_segments"][1]["condition_set_ref"] = reference
    assert composite.model_carries_condition_set_ref(model) is carried


@pytest.mark.parametrize("change,detail", [
    (lambda m: m.update(condition_sets=[], spectra=[]), "INVOCATION_HASH"),
    (lambda m: m.update(condition_sets=[_SET]), "ACTUAL_CONDITION_SETS"),
    (lambda m: m.update(spectra=[_SPECTRUM]), "ACTUAL_SPECTRA"),
    (lambda m: m["pipe_segments"][0].update(condition_set_ref="cs"), "ACTUAL_CONDITION_SET_REF"),
    (lambda m: m["pipe_segments"][0].update(condition_set_ref=""), "ACTUAL_CONDITION_SET_REF"),
])
def test_model_0_9_0_members_are_declined_by_name_through_the_reader(change, detail):
    source, invocation = actual()
    assert composite.validate_physics_source(source, invocation)
    change(invocation["request"]["model"])
    # `[]` passes the declines and stops later, at the receipt's invocation binding the change breaks.
    with pytest.raises(ValueError, match=f"_{detail}$"):
        composite.validate_physics_source(source, invocation)
