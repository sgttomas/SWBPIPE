"""Load/reference-state schema contract: model 0.8.0's optional load/reference-state group (B7b).

Shape only. The authored namespace is checked against the current producer requests (model 0.8.0 with the exact
pressure contract); raw evidence against the current producer envelopes, which publish pressure-2
(`3.0.0/exact_pressure_v3`) with `contract_evidence.load_reference_states`: results_pressure_2.v0.3's
PressureContractEvidence, whose records are load_reference_state.schema.json's LoadReferenceStateRecord. Coverage,
unit dimension and parse-then-block rules are producer diagnostics and are not asserted here. Under the one shape
pressure-2 shares with and without the group, three facts the retired load-reference-1 evidence schema encoded are
reader checks (the group's presence, the exact cases' resolved material basis and the regions' resolved temperature
basis); they are checked here through the readers, the product's (Rust) and the verifier. Carrier instances are the
actual pressure_load_state_* AnalysisRun and stress-neutral fixtures the product's Python route builds, or a minimal
hand-built results 0.3 scaffold around verbatim raw metadata; they are not reader output and carry no hash or
cross-binding claim. B7b retired load-reference-1, its table and its definitions.
"""
from __future__ import annotations

from copy import deepcopy
import json
from pathlib import Path

import pytest

from schema_validation import envelope_branches, schema_for_definition, validate_instance, validate_schema_document

PROJECT = Path(__file__).resolve().parents[1]
SCHEMAS = PROJECT / "schemas"
LOAD_REFERENCE = PROJECT / "fixtures/product_preview/pressure/load_reference"
LR_SCHEMA_NAME = "load_reference_state.schema.json"
PRESSURE_SCHEMA_NAME = "results_pressure_2.v0.3.schema.json"
CARRIERS = ("results.v0.3.schema.yaml", PRESSURE_SCHEMA_NAME, "analysis_run.v0.3.schema.json", "stress_neutral_export.v0.3.schema.json")

CONTRACT = "openpipestress.load_reference_state/1.0.0"
RECORD_PROFILE = "resolved_straight_load_state_v1"
PRESSURE_ID = "openpipestress.result_semantics/0.3.0/pressure-2"
# T4-U6 (D-5, DEL-05-03 section-bases-and-timoshenko.md §2.4): the per-basis stress signatures (106).
PRESSURE_SHA = "be61c6ea828797ad5664d06a2c835b28deffaff07cc33fec6bb4d48a8fcb0919"
PRESSURE_PROFILE = "exact_pressure_v3"
PRESSURE_SOURCE_ID = "openpipestress.result_semantics/0.3.0/pressure-source-1"
PREVIEW_ID = "openpipestress.result_semantics/0.3.0/preview-physics-2"
PREVIEW_SHA = "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2"
RETIRED_LR_ID = "openpipestress.result_semantics/0.3.0/load-reference-1"
RETIRED_IDS = ("openpipestress.result_semantics/0.3.0/physics-1", "openpipestress.result_semantics/0.3.0/physics-source-1",
               RETIRED_LR_ID, "openpipestress.result_semantics/0.3.0/load-reference-source-1")
EXACT_CONTRACT = {"version": "3.0.0", "mode": PRESSURE_PROFILE, "shear_model": "euler_bernoulli"}
EVIDENCE_REF = {"$ref": f"{PRESSURE_SCHEMA_NAME}#/$defs/PressureContractEvidence"}
PRESSURE_RAW = PROJECT / "fixtures/results/pressure_connected_mechanics_sparse.json"
PREVIEW_RAW = PROJECT / "fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json"
# B4/D1 (b): the foreign receipt is the composite's (the only source-block receipt remaining).
SOURCE_BLOCKS_RAW = PROJECT / "fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json"
CARRIER_STEMS = [f"{stem}_{short}" for stem in ("connected", "pressure") for short in ("sparse", "dense")]

# Authored keys the wire (CP2_WIRE.md) and the typed DTO make optional.
OPTIONAL_AUTHORED = {"label", "operating_temperature", "analysis_basis_override", "mass_state_ref",
                     "boundary_motion", "base_motion", "device_reference"}


def _reject_constant(token):
    raise ValueError(f"non-JSON constant {token}")


def _unique_object(pairs):
    keys = [key for key, _ in pairs]
    assert len(keys) == len(set(keys)), f"duplicate keys {keys}"
    return dict(pairs)


def strict_json(path):
    return json.loads(path.read_text(encoding="utf-8"), parse_constant=_reject_constant,
                      object_pairs_hook=_unique_object)


def schema(name):
    return strict_json(SCHEMAS / name)


LRS = schema(LR_SCHEMA_NAME)
PRESSURE_SCHEMA = schema(PRESSURE_SCHEMA_NAME)


def definition(name):
    return schema_for_definition(LRS, name)


def accepted(name, instance):
    validate_instance(definition(name), instance, schema_label=LR_SCHEMA_NAME, instance_label=name)


def rejected(name, instance):
    with pytest.raises(AssertionError):
        validate_instance(definition(name), instance, schema_label=LR_SCHEMA_NAME, instance_label=name)


def evidence_definition():
    """results_pressure_2's PressureContractEvidence as a root of its own; its sibling references resolve locally."""
    root = schema_for_definition(PRESSURE_SCHEMA, "PressureContractEvidence")
    if "$id" in PRESSURE_SCHEMA:
        root["$id"] = PRESSURE_SCHEMA["$id"]
    return root


def evidence_accepted(instance):
    validate_instance(evidence_definition(), instance, schema_label=PRESSURE_SCHEMA_NAME, instance_label="PressureContractEvidence")


def evidence_rejected(instance):
    with pytest.raises(AssertionError):
        validate_instance(evidence_definition(), instance, schema_label=PRESSURE_SCHEMA_NAME, instance_label="PressureContractEvidence")


def request(stem):
    return strict_json(LOAD_REFERENCE / f"{stem}.request.json")["model"]


def raw_paths():
    paths = sorted(LOAD_REFERENCE.glob("*.raw.json"))
    assert len(paths) == 4, "the four actual producer envelopes (connected/pressure x sparse/dense) are required"
    return paths


def raw(stem="pressure-sparse_interactive"):
    return strict_json(LOAD_REFERENCE / f"{stem}.raw.json")


def reader_refusals(document):
    """The product's (Rust) and the verifier's raw dispatch of a statement: their refusal texts, or "accept"."""
    from core.analysis_runs.compatibility import _source_contract
    from tests.verifiers import compatibility as verifier

    def outcome(read):
        try:
            read(document)
        except ValueError as error:
            return str(error)
        return "accept"
    return outcome(_source_contract), outcome(verifier._source_contract)


def walk_objects(value):
    """Every JSON object with its path; one per shape signature.

    The signature is the index-free path plus the discriminants (kind,
    definition, owner_kind) of the node and all its ancestors, so every tagged
    variant, and every object nested in one, is visited once.
    """
    seen = set()

    def discriminants(node):
        return tuple(node.get(key) for key in ("kind", "definition", "owner_kind"))

    def visit(node, where, signature):
        if isinstance(node, dict):
            signature = signature + (discriminants(node),)
            if signature not in seen:
                seen.add(signature)
                yield where, node
            for name, item in node.items():
                yield from visit(item, where + (name,), signature + (name,))
        elif isinstance(node, list):
            for index, item in enumerate(node):
                yield from visit(item, where + (index,), signature + ("[]",))

    yield from visit(value, (), ())


def at(document, path):
    for part in path:
        document = document[part]
    return document


def label(path):
    return "/" + "/".join(str(part) for part in path)


# ---------------------------------------------------------------- fixtures of every authored variant
def q(value, unit):
    return {"value": value, "unit": unit}


def coefficient_points():
    return [{"temperature": q(20, "degC"), "coefficient": q(1.0e-5, "1/K")},
            {"temperature": q(200, "degC"), "coefficient": q(1.2e-5, "1/K")}]


ALL_REFERENCE_CONFIGURATION = {
    "id": "reference:all-variants", "label": None, "geometry_ref": {"kind": "authored_model_geometry"},
    "member_references": [
        {"pipe_ref": "pipe:a", "basis": {"kind": "temperature_reference", "installation_temperature": q(20, "degC")},
         "fit": {"kind": "none"}, "provenance": "invented schema variant"},
        {"pipe_ref": "pipe:b", "basis": {"kind": "direct_strain_reference"},
         "fit": {"kind": "natural_length_change", "length_change": q(-2, "mm")}, "provenance": "invented schema variant"},
        {"pipe_ref": "pipe:c", "basis": {"kind": "temperature_reference", "installation_temperature": q(68, "degF")},
         "fit": {"kind": "fit_strain", "strain": q(-0.0002, "1")}, "provenance": "invented schema variant"},
    ],
    "provenance": "invented schema variant",
}

ALL_EXPANSION_LAWS = [
    {"id": "law:secant-constant", "definition": "engineering_secant", "datum_temperature": q(20, "degC"),
     "data": {"kind": "constant", "coefficient": q(1.0e-5, "1/K")}, "provenance": "invented"},
    {"id": "law:secant-table", "definition": "engineering_secant", "datum_temperature": q(20, "degC"),
     "data": {"kind": "table", "interpolation": "linear_coefficient", "points": coefficient_points()},
     "provenance": "invented"},
    {"id": "law:dilation", "definition": "engineering_dilation", "datum_temperature": q(20, "degC"),
     "data": {"kind": "table", "interpolation": "linear_dilation",
              "points": [{"temperature": q(20, "degC"), "dilation": q(0.0, "1")},
                         {"temperature": q(200, "degC"), "dilation": q(0.002, "1")}]},
     "provenance": "invented"},
    {"id": "law:differential", "definition": "differential_per_datum_length", "datum_temperature": q(293.15, "K"),
     "data": {"kind": "table", "interpolation": "linear_coefficient", "points": coefficient_points()},
     "provenance": "invented"},
    {"id": "law:logarithmic", "definition": "logarithmic_per_current_length", "datum_temperature": q(527.67, "degR"),
     "data": {"kind": "table", "interpolation": "linear_coefficient", "points": coefficient_points()},
     "provenance": "invented"},
]

MOTION = {"dof": "UX", "value": q(0.1, "mm"), "meaning": "absolute_reference_displacement"}
ALL_ANALYSIS_STATE = {
    "contract": CONTRACT, "reference_configuration_ref": "reference:all-variants",
    "element_states": [
        {"pipe_ref": "pipe:a", "operating_temperature": q(100, "degC"),
         "material_selection": {"kind": "explicit_base_properties", "material_ref": "material:x",
                                "applicability_reference": "user basis"},
         "thermal_state": {"kind": "unchanged_reference", "provenance": "invented"},
         "analysis_basis_override": {"reason": "invented reason", "provenance": "invented"},
         "mass_state_ref": "mass:parsed-then-blocked"},
        {"pipe_ref": "pipe:b",
         "material_selection": {"kind": "exact_point", "material_ref": "material:x", "point_ref": "point:hot"},
         "thermal_state": {"kind": "explicit_interval_strain", "strain": q(0.001, "1"),
                           "interval_reference": "invented interval", "provenance": "invented"}},
        {"pipe_ref": "pipe:c", "operating_temperature": None, "analysis_basis_override": None, "mass_state_ref": None,
         "material_selection": {"kind": "temperature_interpolation", "material_ref": "material:x",
                                "temperature": q(100, "degC"), "interpolation": "piecewise_linear",
                                "extrapolation": "forbidden"},
         "thermal_state": {"kind": "constant_alpha_interval", "coefficient": q(1.0e-5, "1/K"),
                           "temperature_change": q(80, "K"), "coefficient_meaning": "engineering_interval",
                           "provenance": "invented"}},
        {"pipe_ref": "pipe:d", "operating_temperature": q(100, "degC"),
         "material_selection": {"kind": "exact_point", "material_ref": "material:x", "point_ref": "point:hot"},
         "thermal_state": {"kind": "free_length_state", "expansion_law_ref": "law:secant-constant"}},
    ],
    "support_states": [
        {"support_ref": "support:a", "participation": {"kind": "active_model_device"},
         "boundary_motion": [MOTION, {"dof": "RZ", "value": q(0.001, "rad"), "meaning": "absolute_reference_displacement"}]},
        {"support_ref": "support:b", "participation": {"kind": "inactive"}, "boundary_motion": None,
         "base_motion": [MOTION], "device_reference": {"kind": "unloaded_reference", "reference_position": q(0, "mm")}},
        {"support_ref": "support:c", "participation": {"kind": "locked_equivalent_support", "components": [
            {"dof": "UZ", "position_source": {"kind": "entered", "value": q(1, "mm")}},
            {"dof": "UY", "position_source": {"kind": "predecessor_value", "case_ref": "case:predecessor",
                                              "state_hash": "0" * 64, "support_ref": "support:c", "dof": "UY"}}]},
         "base_motion": None,
         "device_reference": {"kind": "force_at_reference", "reference_position": q(0, "mm"), "force": q(10, "N")}},
        {"support_ref": "support:d", "participation": {"kind": "active_model_device"}},
    ],
    "load_sources": [{"source_ref": "load:a", "factor": 1.0}, {"source_ref": "load:b", "factor": -0.5}],
    "history": {"kind": "independent_equilibrium"},
    "provenance": "invented schema variant",
}

AUTHORED = (("ReferenceConfiguration", ALL_REFERENCE_CONFIGURATION),
            ("MaterialExpansionLaws", ALL_EXPANSION_LAWS),
            ("AnalysisState", ALL_ANALYSIS_STATE))


# ---------------------------------------------------------------- schema documents
@pytest.mark.parametrize("name", (LR_SCHEMA_NAME,) + CARRIERS)
def test_schema_documents_parse_strictly_and_are_draft_2020_12(name):
    document = schema(name)
    assert document["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    validate_schema_document(document, schema_label=name)


def test_retired_load_reference_table_and_definitions_are_absent():
    """B7b retired load-reference-1 with `2.0.0/exact_straight_pressure_v2`: its table and its evidence
    definitions are gone; the group's records and resolved materials remain."""
    assert not (PROJECT / "fixtures/results/semantic_contract_v0_3_load_reference_1.json").exists()
    for name in ("LoadReferenceContractEvidence", "LoadReferenceExactCase", "LoadReferencePressureEvidence", "PhysicsGeometry"):
        assert name not in LRS["$defs"], name
    for name in ("LoadReferenceStateRecord", "LoadReferencePipeMaterial", "LoadReferenceRegionMaterial"):
        assert name in LRS["$defs"], name
    text = (SCHEMAS / LR_SCHEMA_NAME).read_text(encoding="utf-8")
    assert "exact_straight_pressure_v2" not in text and RETIRED_LR_ID not in text


def test_load_reference_schema_stays_within_the_closed_shape_checker_vocabulary():
    """Local one-level refs and the keyword set the repository's closed-shape checkers interpret."""
    allowed = {"$schema", "$id", "title", "description", "$defs", "type", "additionalProperties", "required",
               "properties", "items", "const", "enum", "oneOf", "anyOf", "allOf", "$ref", "minLength", "minItems",
               "maxItems", "minimum", "maximum", "pattern"}

    def check(node, path, keyword_level=True):
        if isinstance(node, list):
            for index, item in enumerate(node):
                check(item, f"{path}/{index}")
            return
        if not isinstance(node, dict):
            return
        for key, value in node.items():
            if keyword_level:
                assert key in allowed, f"{path}/{key}"
                if key == "$ref":
                    assert value.startswith("#/$defs/") and value[len("#/$defs/"):] in LRS["$defs"], value
                if key == "pattern":
                    assert value in {"^[0-9a-f]{64}$", "^[0-9a-f]{16}$"}, value
                if key == "additionalProperties":
                    assert value is False, path
                if key in {"const", "enum"}:
                    continue
                check(value, f"{path}/{key}", keyword_level=key not in {"properties", "$defs"})
            else:
                check(value, f"{path}/{key}")
        if keyword_level and node.get("type") == "object":
            assert "properties" in node and node.get("additionalProperties") is False, path
        if keyword_level and node.get("type") == "array":
            assert "items" in node, path

    check(LRS, "#")


# ---------------------------------------------------------------- resolved-material shapes
def _without(value, pointer_parts):
    value = deepcopy(value)
    parent = value
    for part in pointer_parts[:-1]:
        parent = parent[part]
    parent.pop(pointer_parts[-1])
    return value


def test_resolved_material_shapes_differ_only_at_the_documented_keys():
    results = schema("results.v0.3.schema.yaml")["$defs"]
    base = results["PhysicsMaterial"]
    defs = LRS["$defs"]
    # Pipe materials: PhysicsMaterial plus the two resolved-member keys; pressure-2 holds the same closed shape.
    material = _without(defs["LoadReferencePipeMaterial"], ["description"])
    assert material["required"] == base["required"] + ["material_selection_kind", "resolved_eigenstrain"]
    assert {k: v for k, v in material["properties"].items() if k in base["properties"]} == base["properties"]
    assert set(material["properties"]) - set(base["properties"]) == {"material_selection_kind", "resolved_eigenstrain"}
    assert {k: v for k, v in material.items() if k not in {"required", "properties"}} == {
        k: v for k, v in base.items() if k not in {"required", "properties"}}
    assert _without(PRESSURE_SCHEMA["$defs"]["PressureResolvedPipeMaterial"], ["description"]) == material
    # Region materials: PhysicsMaterial plus temperature_basis, the resolved per-member pair's constant.
    region = _without(defs["LoadReferenceRegionMaterial"], ["description"])
    assert region["required"] == base["required"] + ["temperature_basis"]
    assert region["properties"]["temperature_basis"] == {"const": "resolved_member_state"}
    assert {k: v for k, v in region["properties"].items() if k != "temperature_basis"} == base["properties"]


# ---------------------------------------------------------------- positive: current producer inputs and outputs
@pytest.mark.parametrize("stem", ("connected", "pressure"))
def test_current_request_authored_namespace_validates(stem):
    model = request(stem)
    assert model["schema_version"] == "0.8.0" and model["pressure_contract"] == EXACT_CONTRACT
    assert model["reference_configurations"]
    for index, configuration in enumerate(model["reference_configurations"]):
        validate_instance(definition("ReferenceConfiguration"), configuration, instance_label=f"{stem}:reference[{index}]")
    laws = [material["expansion_laws"] for material in model["materials"] if "expansion_laws" in material]
    assert laws
    for index, material_laws in enumerate(laws):
        validate_instance(definition("MaterialExpansionLaws"), material_laws, instance_label=f"{stem}:laws[{index}]")
    assert model["load_cases"]
    for case in model["load_cases"]:
        validate_instance(definition("AnalysisState"), case["analysis_state"], instance_label=f"{stem}:{case['id']}")


@pytest.mark.parametrize("path", raw_paths(), ids=lambda path: path.stem)
def test_every_raw_envelope_contract_evidence_validates(path):
    envelope = strict_json(path)
    assert envelope["producer"]["semantic_contract_id"] == PRESSURE_ID
    assert envelope["formulation_basis"]["profile_id"] == PRESSURE_PROFILE
    assert "source_block_recovery" not in envelope
    evidence = envelope["contract_evidence"]
    assert set(evidence) == {"pressure", "connector", "exact_cases", "load_reference_states"}
    evidence_accepted(evidence)
    for record in evidence["load_reference_states"]:
        assert record["contract"] == CONTRACT and record["profile"] == RECORD_PROFILE
        validate_instance(definition("LoadReferenceStateRecord"), record, instance_label=f"{path.stem}:{record['load_case_id']}")


def test_pressure_evidence_is_one_shape_and_the_retained_exact_shape_refuses_the_group():
    """pressure-2 publishes one evidence shape with or without the group; the exact successor's retained shape
    (results 0.3 ExactRetainedContractEvidence, straight members only) admits neither the group nor its
    resolved members."""
    evidence_accepted(strict_json(PRESSURE_RAW)["contract_evidence"])
    results = schema("results.v0.3.schema.yaml")
    for path in raw_paths():
        with pytest.raises(AssertionError):
            validate_instance(schema_for_definition(results, "ExactRetainedContractEvidence"), strict_json(path)["contract_evidence"])


@pytest.mark.parametrize("name,instance", AUTHORED, ids=[name for name, _ in AUTHORED])
def test_every_reviewed_authored_variant_has_a_shape(name, instance):
    accepted(name, instance)


def test_optional_authored_fields_accept_absence_and_null_but_expansion_laws_must_be_an_array():
    state = deepcopy(ALL_ANALYSIS_STATE)
    for element in state["element_states"]:
        for key in ("operating_temperature", "analysis_basis_override", "mass_state_ref"):
            element.pop(key, None)
    for support in state["support_states"]:
        for key in ("boundary_motion", "base_motion", "device_reference"):
            support.pop(key, None)
    accepted("AnalysisState", state)
    configuration = deepcopy(ALL_REFERENCE_CONFIGURATION)
    configuration.pop("label")
    accepted("ReferenceConfiguration", configuration)
    accepted("MaterialExpansionLaws", [])
    rejected("MaterialExpansionLaws", None)


def test_admission_rules_are_not_shape():
    """Unknown contract strings, empty text and non-catalog units parse; the producer blocks them."""
    state = deepcopy(ALL_ANALYSIS_STATE)
    state["contract"] = "openpipestress.load_reference_state/9.9.9"
    state["provenance"] = ""
    state["element_states"][0]["operating_temperature"] = q(100, "not-a-unit")
    state["load_sources"][0]["factor"] = 0
    state["support_states"][0]["boundary_motion"][0]["dof"] = "UQ"
    accepted("AnalysisState", state)


# ---------------------------------------------------------------- negative: authored namespace
AUTHORED_OBJECTS = [(name, path) for name, instance in AUTHORED for path, _ in walk_objects(instance)]


@pytest.mark.parametrize("name,path", AUTHORED_OBJECTS, ids=[f"{n}{label(p)}" for n, p in AUTHORED_OBJECTS])
def test_authored_unknown_key_is_rejected_at_every_object_level(name, path):
    instance = deepcopy(dict(AUTHORED)[name])
    at(instance, path)["unexpected_key"] = True
    rejected(name, instance)


AUTHORED_KEYS = [(name, path, key) for name, instance in AUTHORED for path, node in walk_objects(instance) for key in node]


@pytest.mark.parametrize("name,path,key", AUTHORED_KEYS, ids=[f"{n}{label(p)}/{k}" for n, p, k in AUTHORED_KEYS])
def test_authored_missing_key_is_rejected_unless_the_wire_makes_it_optional(name, path, key):
    instance = deepcopy(dict(AUTHORED)[name])
    del at(instance, path)[key]
    if key in OPTIONAL_AUTHORED:
        accepted(name, instance)
    else:
        rejected(name, instance)


DISCRIMINANTS = [
    ("ReferenceConfiguration", ("geometry_ref", "kind")),
    ("ReferenceConfiguration", ("member_references", 0, "basis", "kind")),
    ("ReferenceConfiguration", ("member_references", 1, "basis", "kind")),
    ("ReferenceConfiguration", ("member_references", 0, "fit", "kind")),
    ("ReferenceConfiguration", ("member_references", 1, "fit", "kind")),
    ("ReferenceConfiguration", ("member_references", 2, "fit", "kind")),
    ("MaterialExpansionLaws", (0, "definition")),
    ("MaterialExpansionLaws", (2, "definition")),
    ("MaterialExpansionLaws", (3, "definition")),
    ("MaterialExpansionLaws", (0, "data", "kind")),
    ("MaterialExpansionLaws", (1, "data", "kind")),
    ("MaterialExpansionLaws", (2, "data", "kind")),
    ("MaterialExpansionLaws", (1, "data", "interpolation")),
    ("MaterialExpansionLaws", (2, "data", "interpolation")),
    ("AnalysisState", ("element_states", 0, "material_selection", "kind")),
    ("AnalysisState", ("element_states", 2, "material_selection", "interpolation")),
    ("AnalysisState", ("element_states", 2, "material_selection", "extrapolation")),
    ("AnalysisState", ("element_states", 0, "thermal_state", "kind")),
    ("AnalysisState", ("element_states", 3, "thermal_state", "kind")),
    ("AnalysisState", ("element_states", 2, "thermal_state", "coefficient_meaning")),
    ("AnalysisState", ("support_states", 0, "participation", "kind")),
    ("AnalysisState", ("support_states", 2, "participation", "components", 0, "position_source", "kind")),
    ("AnalysisState", ("support_states", 1, "device_reference", "kind")),
    ("AnalysisState", ("support_states", 0, "boundary_motion", 0, "meaning")),
    ("AnalysisState", ("history", "kind")),
]


@pytest.mark.parametrize("name,path", DISCRIMINANTS, ids=[f"{n}{label(p)}" for n, p in DISCRIMINANTS])
def test_unknown_discriminant_is_rejected(name, path):
    instance = deepcopy(dict(AUTHORED)[name])
    at(instance, path[:-1])[path[-1]] = "unreviewed_value"
    rejected(name, instance)
    at(instance, path[:-1])[path[-1]] = None
    rejected(name, instance)


@pytest.mark.parametrize("kind", ("none", "natural_length_change", "fit_strain"))
def test_fit_with_both_length_change_and_strain_is_rejected(kind):
    configuration = deepcopy(ALL_REFERENCE_CONFIGURATION)
    configuration["member_references"][0]["fit"] = {"kind": kind, "length_change": q(-2, "mm"), "strain": q(-0.0002, "1")}
    rejected("ReferenceConfiguration", configuration)


def test_history_other_than_independent_equilibrium_is_rejected():
    state = deepcopy(ALL_ANALYSIS_STATE)
    state["history"] = {"kind": "continuation"}
    rejected("AnalysisState", state)
    state["history"] = {}
    rejected("AnalysisState", state)


# ---------------------------------------------------------------- negative: raw evidence
def _raw_variants():
    """Actual connected evidence with the nullable member objects populated (override, fit input, segments)."""
    evidence = deepcopy(raw("connected-sparse_interactive")["contract_evidence"])
    member = evidence["load_reference_states"][0]["members"][0]
    member.update(analysis_basis_override={"reason": "invented", "provenance": "invented"}, fit_kind="fit_strain",
                  fit_input={"strain": -0.0002}, consumed_law_point_indices=[0, 1], consulted_law_point_indices=[1],
                  consumed_law_segments=[{"use": "integration_interval", "lower_index": 0, "upper_index": 1,
                                          "start_k": 293.15, "end_k": 423.15}],
                  consulted_law_segments=[{"use": "interpolation_sample", "lower_index": 0, "upper_index": 1,
                                           "start_k": 293.15, "end_k": 293.15}])
    return evidence


# Both actual evidence families (connected: support motion, stored/excluded sources, fit input; pressure:
# pressure regions) plus the populated-variant instance. Walk paths start with the index in RAW_SET.
RAW_SET = [raw("connected-sparse_interactive")["contract_evidence"], raw("pressure-sparse_interactive")["contract_evidence"],
           _raw_variants()]
RAW_OBJECTS = [path for path, _ in walk_objects(RAW_SET)]


@pytest.mark.parametrize("index", range(len(RAW_SET)))
def test_raw_walk_starting_points_are_valid(index):
    evidence_accepted(RAW_SET[index])


@pytest.mark.parametrize("path", RAW_OBJECTS, ids=[label(p) for p in RAW_OBJECTS])
def test_raw_unknown_key_is_rejected_at_every_object_level(path):
    evidence = deepcopy(RAW_SET[path[0]])
    at(evidence, path[1:])["unexpected_key"] = True
    evidence_rejected(evidence)


# Missing keys: every key of the load/reference-state shapes (the evidence root, each load_reference_states
# record and its sub-objects, exact_cases pipe_materials). The group itself is optional in pressure-2's one shape
# and is selected by presence: its removal is refused by the readers (below), not by the shape.
RAW_KEYS = [(path, key) for path, node in walk_objects(RAW_SET)
            if path[1:] == () or path[1:2] == ("load_reference_states",) or "pipe_materials" in path
            for key in node if (path[1:], key) != ((), "load_reference_states")]


@pytest.mark.parametrize("path,key", RAW_KEYS, ids=[f"{label(p)}/{k}" for p, k in RAW_KEYS])
def test_raw_missing_key_is_rejected(path, key):
    evidence = deepcopy(RAW_SET[path[0]])
    del at(evidence, path[1:])[key]
    evidence_rejected(evidence)


def _first_state():
    """Actual pressure-envelope evidence (it carries pressure regions) and its first case record."""
    evidence = deepcopy(RAW_SET[1])
    return evidence, evidence["load_reference_states"][0]


@pytest.mark.parametrize("mutation", (
    "contract", "profile", "source_recovery_status", "source_recovery_code", "history", "geometry_kind",
    "projection_hash", "mode_method_mix", "segment_use", "fit_kind", "reference_basis", "thermal_definition",
    "selection_kind", "exact_selection_kind", "support_unit_mismatch", "support_law", "owner_kind", "classification",
    "pressure_factor", "excluded_owner", "connector", "string_number",
))
def test_raw_evidence_enum_and_constant_mutations_are_rejected(mutation):
    evidence, state = _first_state()
    member = state["members"][0]
    if mutation == "contract": state["contract"] = "openpipestress.load_reference_state/2.0.0"
    elif mutation == "profile": state["profile"] = "exact_straight_pressure_v2"
    elif mutation == "source_recovery_status": state["source_recovery"]["status"] = "joined"
    elif mutation == "source_recovery_code": state["source_recovery"]["code"] = "OTHER"
    elif mutation == "history": state["history"]["kind"] = "continuation"
    elif mutation == "geometry_kind": state["reference_geometry"]["kind"] = "deformed"
    elif mutation == "projection_hash": state["reference_geometry"]["projection_sha256"] = "Z" * 64
    elif mutation == "mode_method_mix": state["solve"]["recovery_method"] = (
        "ordinary_dense_structural_v1" if state["solve"]["requested_mode"] == "sparse_interactive" else "ordinary_sparse_structural_v1")
    elif mutation == "segment_use": member["consumed_law_segments"] = [
        {"use": "extrapolation", "lower_index": 0, "upper_index": 1, "start_k": 293.15, "end_k": 300.0}]
    elif mutation == "fit_kind": member["fit_kind"] = "fit"
    elif mutation == "reference_basis": member["reference_basis"] = "ambient"
    elif mutation == "thermal_definition": member["thermal_definition"] = "engineering_tangent"
    elif mutation == "selection_kind": member["material_selection_kind"] = "base_material"
    elif mutation == "exact_selection_kind": evidence["exact_cases"][0]["pipe_materials"][0]["material_selection_kind"] = "interpolation"
    elif mutation == "support_unit_mismatch":
        state["support_components"] = [{"support_id": "support:root", "node_id": "node:root", "dof": "RZ", "global_dof": 5,
                                        "law_kind": "rigid_prescribed", "prescribed_value": 0.001, "unit": "m",
                                        "meaning": "absolute_reference_displacement",
                                        "physical_state_source": "support_state.boundary_motion"}]
    elif mutation == "support_law":
        state["support_components"] = [{"support_id": "support:root", "node_id": "node:root", "dof": "UX", "global_dof": 0,
                                        "law_kind": "spring", "prescribed_value": 0.001, "unit": "m",
                                        "meaning": "absolute_reference_displacement",
                                        "physical_state_source": "support_state.boundary_motion"}]
    elif mutation == "owner_kind": state["contributions"][0]["owner_kind"] = "derived"
    elif mutation == "classification": state["contributions"][0]["classification"] = "ordinary_applied"
    elif mutation == "pressure_factor":
        region = next(item for item in state["contributions"] if item["owner_kind"] == "pressure_region")
        region["factor"] = 1.0
    elif mutation == "excluded_owner": state["excluded_sources"] = [
        {"source_id": "load:x", "owner_kind": "pressure_region", "classification": "excluded", "category": "concentrated_force",
         "reason": "not listed in analysis_state.load_sources"}]
    elif mutation == "connector": evidence["connector"] = [{}]
    elif mutation == "string_number": member["selected_E_pa"] = str(member["selected_E_pa"])
    evidence_rejected(evidence)


@pytest.mark.parametrize(("mutation", "rust", "verifier"), (
    ("group_removed", "SOURCE_PHYSICS_MATERIAL_SHAPE", "SOURCE_PHYSICS_EVIDENCE_INVALID: material shape"),
    ("exact_material_basis", "SOURCE_LOAD_REFERENCE_MATERIAL_BASIS", "SOURCE_LOAD_REFERENCE_MATERIAL_BASIS"),
    ("pressure_temperature_basis", "SOURCE_LOAD_REFERENCE_REGION_TEMPERATURE_BASIS", "SOURCE_LOAD_REFERENCE_REGION_TEMPERATURE_BASIS"),
))
def test_group_presence_and_resolved_bases_are_reader_checks_under_the_shared_shape(mutation, rust, verifier):
    """The retired load-reference-1 evidence schema encoded these as constants; pressure-2's one shape admits
    them, and both readers refuse them (the verifier's text is its own, as its other generic codes)."""
    document = raw()
    evidence = document["contract_evidence"]
    if mutation == "group_removed": del evidence["load_reference_states"]
    elif mutation == "exact_material_basis": evidence["exact_cases"][0]["material_basis"] = "case_wide_base_material"
    else: evidence["pressure"][0]["materials"][0]["temperature_basis"] = {"selection": "base_material"}
    evidence_accepted(evidence)
    assert reader_refusals(document) == (rust, verifier)


def test_raw_absent_optionals_are_null_and_admitted_values_are_accepted():
    evidence, state = _first_state()
    member = state["members"][0]
    for key in ("interpolation_fraction", "applicability_reference", "analysis_basis_override", "expansion_law_id",
                "operating_temperature_k", "material_selection_temperature_k", "installation_temperature_k",
                "coefficient_datum_k", "installation_datum_stretch", "operating_datum_stretch", "fit_input"):
        member[key] = None
    member["consumed_material_points"] = [dict(member["consumed_material_points"][0], temperature_k=None)] if member[
        "consumed_material_points"] else []
    evidence_accepted(evidence)
    member.update(interpolation_fraction=0.5, applicability_reference="user basis",
                  analysis_basis_override={"reason": "invented", "provenance": "invented"}, fit_kind="fit_strain",
                  fit_input={"strain": -0.0002}, thermal_definition="logarithmic_per_current_length",
                  material_selection_kind="temperature_interpolation", reference_basis="direct_strain_reference",
                  consumed_law_point_indices=[0, 1], consulted_law_point_indices=[1],
                  consumed_law_segments=[{"use": "integration_interval", "lower_index": 0, "upper_index": 1,
                                          "start_k": 293.15, "end_k": 423.15}],
                  consulted_law_segments=[{"use": "interpolation_sample", "lower_index": 0, "upper_index": 1,
                                           "start_k": 293.15, "end_k": 293.15}])
    evidence_accepted(evidence)
    for bad in ({"strain": -0.0002, "length_change_m": 0.001}, {"length_change_m": "1"}, {}):
        member["fit_input"] = bad
        evidence_rejected(evidence)


# ---------------------------------------------------------------- carriers
def test_carrier_branches_are_the_current_identities():
    """The results, AnalysisRun and stress-neutral carriers name the current identities only; a load/reference-state
    publication is carried on pressure-2's (or pressure-source-1's) branch, whose evidence is PressureContractEvidence."""
    ids = [branch["properties"]["producer"]["properties"]["semantic_contract_id"]["const"]
           for branch in envelope_branches(schema("results.v0.3.schema.yaml"))]
    # PF Timoshenko (DEL-05-03 pf-timoshenko.md §4): its branch follows preview-physics-2's.
    assert ids == [PRESSURE_SOURCE_ID, PREVIEW_ID, "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1", PRESSURE_ID,
                   "openpipestress.result_semantics/0.3.0/preview-physics-retained-1",
                   "openpipestress.result_semantics/0.3.0/physics-retained-1"]
    results = schema("results.v0.3.schema.yaml")["$defs"]["ResultEnvelope"]
    assert results["properties"]["contract_evidence"]["anyOf"].count(EVIDENCE_REF) == 1
    pressure_branch = envelope_branches(schema("results.v0.3.schema.yaml"))[ids.index(PRESSURE_ID)]
    assert pressure_branch["properties"]["contract_evidence"] == {"$ref": "#/$defs/PressureContractEvidence"}
    assert pressure_branch["properties"]["formulation_basis"]["properties"]["profile_id"] == {"const": PRESSURE_PROFILE}
    run = schema("analysis_run.v0.3.schema.json")["$defs"]
    contracts = [branch["properties"]["id"]["const"] for branch in run["SemanticContract"]["oneOf"]]
    assert sorted(contracts) == sorted(ids)
    assert run["SemanticContract"]["oneOf"][contracts.index(PRESSURE_ID)] == {"properties": {"id": {"const": PRESSURE_ID}, "sha256": {"const": PRESSURE_SHA}}}
    package = schema("stress_neutral_export.v0.3.schema.json")
    package_ids = [branch["properties"]["semantic_contract"]["properties"]["id"]["const"] for branch in package["oneOf"]]
    assert package_ids == ids
    branch = package["oneOf"][package_ids.index(PRESSURE_ID)]
    assert branch["properties"]["semantic_contract"]["properties"] == {"id": {"const": PRESSURE_ID}, "sha256": {"const": PRESSURE_SHA}}
    assert branch["properties"]["contract_evidence"] == EVIDENCE_REF
    for name in ("results.v0.3.schema.yaml", PRESSURE_SCHEMA_NAME, "analysis_run.v0.3.schema.json", "stress_neutral_export.v0.3.schema.json", LR_SCHEMA_NAME):
        text = (SCHEMAS / name).read_text(encoding="utf-8")
        assert not any(identity in text for identity in RETIRED_IDS), name


def _results_document(source, contract_id):
    """Minimal results 0.3 scaffold around verbatim raw producer metadata and evidence."""
    def ref(kind, identifier):
        return {"ref_type": kind, "ref_id": identifier}

    def scoped(scope, kind, identifier):
        return {"algorithm": "sha256", "canonicalization": "openpipestress_jcs_ijson_v1", "payload_scope": scope,
                "payload_ref": ref(kind, identifier), "value": "0" * 64}

    run_ref = ref("mechanics_run", source["run_id"])
    model_ref = ref("model", source["model_ref"])
    envelope = {
        "schema_version": "0.3.0", "envelope_id": "result-envelope:schema-test", "model_ref": model_ref,
        "run_ref": run_ref,
        "solver_version": {"solver_name": "schema-test", "solver_version": "0", "solver_build_ref": "unavailable:schema-test"},
        "unit_system_ref": ref("unit_system", "unit-system:schema-test"),
        "load_basis_refs": [deepcopy(case["basis_ref"]) for case in source["numerical_quality"]["cases"]],
        "result_sets": [{"set_id": "result-set:schema-test", "set_type": "mechanics",
                         "basis_ref": deepcopy(source["numerical_quality"]["cases"][0]["basis_ref"]), "values": []}],
        "diagnostics": [],
        "provenance": {"source_name": "schema test", "source_location": "tests/test_load_reference_schema.py",
                       "source_license": "invented", "contributor": "schema test",
                       "contributor_certification": "invented", "redistribution_status": "invented_non_engineering_example",
                       "review_status": "pending"},
        "reproducibility": {
            "model_hash": {"algorithm": "sha256", "canonicalization": "openpipestress_jcs_ijson_v1",
                           "payload_scope": "model_payload", "payload_ref": model_ref, "value": "0" * 64},
            "run_hashes": [], "audit_manifest_ref": ref("audit_manifest", "audit:schema-test"),
            "deterministic_ordering": True, "raw_source_hashes": [],
            "derivative_hash": scoped("derivative_document_excludes_own_hash", "result_envelope", "result-envelope:schema-test"),
            "derivative_hash_excludes": "result_envelope.reproducibility.derivative_hash",
            "source_origin_bindings": [{
                "origin_class": "attested_headless_producer", "qualification_ref": ref("qualification", "qualification:schema-test"),
                "authentic_producer_available": False,
                "received_carrier_checksum": scoped("attested_headless_producer_carrier", "mechanics_run", source["run_id"]),
                "original_producer_checksum": None, "origin_limit": "schema-level test instance only",
                "origin_id": "origin:schema-test", "actual_model_ref": model_ref, "mechanics_run_ref": run_ref,
                "request_model_ref": None, "request_run_ref": None, "request_alias_disclosure": None}],
            "request_hash": None},
        "analysis_status": ["HUMAN_REVIEW_REQUIRED"],
        "professional_boundary": {"human_review_required": True, "software_makes_compliance_claim": False,
                                  "software_makes_certification_claim": False, "software_makes_sealing_claim": False,
                                  "software_makes_approval_claim": False, "software_makes_authentication_claim": False},
        "downstream_use": {"review": True, "regression_comparison": True, "report_consumption": True,
                           "headless_automation": True, "governed_downstream_tooling": True, "additional_export_formats": "TBD"},
        "review_evidence": [], "row_disclosures": [], "row_accounting": [], "source_annotations": [],
        "producer": deepcopy(source["producer"]),
        "semantic_contract_ref": ref("semantic_contract", contract_id),
        "formulation_basis": deepcopy(source["formulation_basis"]),
        "numerical_quality": deepcopy(source["numerical_quality"]),
        "contract_evidence": deepcopy(source["contract_evidence"]),
    }
    return {"schema_version": "0.3.0", "deliverable_id": "DEL-08-04", "package_id": "PKG-08", "scope_item": "SOW-046",
            "objectives": ["OBJ-007", "OBJ-009"],
            "export_format_status": {"baseline_format": "schema_first_json_result_envelope", "additional_formats": "TBD",
                                     "public_transport_protocol": "TBD", "local_fea_package_format": "TBD",
                                     "external_adapter_formats": "TBD"},
            "result_envelope": envelope}


def results_valid(document):
    validate_instance(schema("results.v0.3.schema.yaml"), document, instance_label="results 0.3")


def results_invalid(document):
    with pytest.raises(AssertionError):
        validate_instance(schema("results.v0.3.schema.yaml"), document, instance_label="results 0.3")


def receipt():
    received = strict_json(SOURCE_BLOCKS_RAW)["source_block_recovery"]
    validate_instance(schema("physics_source_recovery.schema.json"), received, instance_label="actual receipt")
    return received


@pytest.mark.parametrize("path", raw_paths(), ids=lambda path: path.stem)
def test_results_pressure_branch_accepts_verbatim_load_state_raw_metadata(path):
    results_valid(_results_document(strict_json(path), PRESSURE_ID))


def test_results_pressure_branch_still_accepts_its_plain_scaffold():
    results_valid(_results_document(strict_json(PRESSURE_RAW), PRESSURE_ID))


@pytest.mark.parametrize("mutation", (
    "producer_id", "ref_id", "profile", "unknown_id", "source_block_recovery", "preview_evidence",
    "missing_evidence", "evidence_extra_key", "relabel_as_retired_load_reference", "records_extra_key",
))
def test_results_pressure_branch_rejects_contract_mixups(mutation):
    source = raw()
    document = _results_document(source, PRESSURE_ID)
    envelope = document["result_envelope"]
    if mutation == "producer_id": envelope["producer"]["semantic_contract_id"] = PREVIEW_ID
    elif mutation == "ref_id": envelope["semantic_contract_ref"]["ref_id"] = PREVIEW_ID
    elif mutation == "profile": envelope["formulation_basis"]["profile_id"] = RECORD_PROFILE
    elif mutation == "unknown_id":
        envelope["producer"]["semantic_contract_id"] = envelope["semantic_contract_ref"]["ref_id"] = PRESSURE_ID.replace("pressure-2", "pressure-3")
    elif mutation == "source_block_recovery": envelope["source_block_recovery"] = receipt()
    elif mutation == "preview_evidence": envelope["contract_evidence"] = strict_json(PREVIEW_RAW)["contract_evidence"]
    elif mutation == "missing_evidence": del envelope["contract_evidence"]
    elif mutation == "evidence_extra_key": envelope["contract_evidence"]["source_receipt"] = {}
    elif mutation == "relabel_as_retired_load_reference":
        envelope["producer"]["semantic_contract_id"] = envelope["semantic_contract_ref"]["ref_id"] = RETIRED_LR_ID
        envelope["formulation_basis"]["profile_id"] = RECORD_PROFILE
    elif mutation == "records_extra_key":
        envelope["contract_evidence"]["load_reference_states"][0]["unexpected_key"] = True
    results_invalid(document)


def carrier(stem, kind):
    return strict_json(PROJECT / f"fixtures/results/pressure_load_state_{stem}.{kind}.json")


def analysis_valid(record):
    for name in ("analysis_run.v0.3.schema.json", "analysis_run.schema.json"):
        validate_instance(schema(name), record, instance_label=f"AnalysisRun via {name}")


def analysis_invalid(record):
    with pytest.raises(AssertionError):
        validate_instance(schema("analysis_run.v0.3.schema.json"), record, instance_label="AnalysisRun")


def _rebound(record, contract_id=PRESSURE_ID, sha=PRESSURE_SHA, rows=None):
    record = deepcopy(record)
    run = record["analysis_run"]
    run["reproducibility"]["semantic_contract"].update(id=contract_id, sha256=sha)
    for item in run["result_refs"]:
        item["semantic_contract"].update(id=rows[0] if rows else contract_id, sha256=rows[1] if rows else sha)
    return record


@pytest.mark.parametrize("stem", CARRIER_STEMS)
def test_analysis_run_pressure_branch_accepts_the_actual_load_state_carriers(stem):
    record = carrier(stem, "analysis_run")
    assert record["analysis_run"]["reproducibility"]["semantic_contract"] == {"id": PRESSURE_ID, "sha256": PRESSURE_SHA}
    assert record["analysis_run"]["result_refs"], "the record binds rows"
    analysis_valid(record)


@pytest.mark.parametrize("mutation", (
    "preview_hash", "retired_identity", "unknown_id", "mixed_rows", "contract_evidence", "source_block_recovery",
))
def test_analysis_run_pressure_branch_rejects_contract_mixups(mutation):
    analysis = carrier("pressure_sparse", "analysis_run")
    evidence = raw()["contract_evidence"]
    if mutation == "preview_hash": record = _rebound(analysis, sha=PREVIEW_SHA)
    elif mutation == "retired_identity": record = _rebound(analysis, contract_id=RETIRED_LR_ID, sha="e41657242c65fe063b0cc3a82248191eb998dff9c80afdae03c8a7a419aa7f09")
    elif mutation == "unknown_id": record = _rebound(analysis, contract_id=PRESSURE_ID.replace("pressure-2", "pressure-3"))
    elif mutation == "mixed_rows": record = _rebound(analysis, rows=(PREVIEW_ID, PREVIEW_SHA))
    elif mutation == "contract_evidence":
        record = deepcopy(analysis)
        record["analysis_run"]["contract_evidence"] = evidence
    elif mutation == "source_block_recovery":
        record = deepcopy(analysis)
        record["analysis_run"]["source_block_recovery"] = receipt()
    analysis_invalid(record)


def package_valid(packet):
    validate_instance(schema("stress_neutral_export.v0.3.schema.json"), packet, instance_label="stress-neutral 0.3")


def package_invalid(packet):
    with pytest.raises(AssertionError):
        validate_instance(schema("stress_neutral_export.v0.3.schema.json"), packet, instance_label="stress-neutral 0.3")


@pytest.mark.parametrize("stem", CARRIER_STEMS)
def test_stress_neutral_pressure_branch_accepts_the_actual_load_state_packages(stem):
    packet = carrier(stem, "stress_neutral")
    assert packet["semantic_contract"] == {"id": PRESSURE_ID, "sha256": PRESSURE_SHA}
    assert "load_reference_states" in packet["contract_evidence"]
    package_valid(packet)


@pytest.mark.parametrize("mutation", (
    "sha", "producer_id", "ref_id", "contract_id", "profile", "source_block_recovery", "preview_evidence",
    "missing_evidence", "missing_source_annotations", "csv_policy", "records_extra_key",
))
def test_stress_neutral_pressure_branch_rejects_contract_mixups(mutation):
    packet = carrier("pressure_sparse", "stress_neutral")
    if mutation == "sha": packet["semantic_contract"]["sha256"] = PREVIEW_SHA
    elif mutation == "producer_id": packet["producer"]["semantic_contract_id"] = PREVIEW_ID
    elif mutation == "ref_id": packet["semantic_contract_ref"]["ref_id"] = PREVIEW_ID
    elif mutation == "contract_id": packet["semantic_contract"]["id"] = PREVIEW_ID
    elif mutation == "profile": packet["formulation_basis"]["profile_id"] = RECORD_PROFILE
    elif mutation == "source_block_recovery": packet["source_block_recovery"] = receipt()
    elif mutation == "preview_evidence": packet["contract_evidence"] = strict_json(PREVIEW_RAW)["contract_evidence"]
    elif mutation == "missing_evidence": del packet["contract_evidence"]
    elif mutation == "missing_source_annotations": del packet["source_annotations"]
    elif mutation == "csv_policy":
        for key in ("csv_encoding", "csv_row_order"):
            del packet["export_profile"][key]
    elif mutation == "records_extra_key": packet["contract_evidence"]["load_reference_states"][0]["unexpected_key"] = True
    package_invalid(packet)
