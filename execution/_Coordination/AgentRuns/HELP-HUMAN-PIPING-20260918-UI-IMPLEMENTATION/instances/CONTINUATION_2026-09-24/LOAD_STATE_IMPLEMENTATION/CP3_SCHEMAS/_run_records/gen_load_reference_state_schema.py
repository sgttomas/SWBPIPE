"""Generate schemas/load_reference_state.schema.json (CP3_SCHEMAS TASK).

Run from WORKING_ROOT:  <python> <this file>
The physics sub-shapes are deep-copied from results.v0.3 $defs so that the
load-reference copies differ only at the documented pointers. The file is
written atomically (temporary file in schemas/, then rename).
"""
from __future__ import annotations

import copy
import json
import os
import tempfile
from pathlib import Path

ROOT = Path.cwd()
RESULTS = json.loads((ROOT / "schemas/results.v0.3.schema.yaml").read_text(encoding="utf-8"))
PCE = RESULTS["$defs"]["PhysicsContractEvidence"]

CONTRACT = "openpipestress.load_reference_state/1.0.0"
PROFILE = "resolved_straight_load_state_v1"


def ref(name):
    return {"$ref": f"#/$defs/{name}"}


def nullable(schema):
    return {"oneOf": [{"type": "null"}, schema]}


def closed(required, properties, description=None, optional=()):
    body = {}
    if description:
        body["description"] = description
    body.update({"type": "object", "additionalProperties": False, "required": list(required), "properties": properties})
    missing = set(required) - set(properties)
    assert not missing, missing
    assert set(optional) <= set(properties)
    return body


def tagged(tag, value, extra_required=(), extra=None, description=None):
    properties = {tag: {"const": value}}
    properties.update(extra or {})
    return closed([tag, *extra_required], properties, description)


STRING = {"type": "string"}
TEXT = {"type": "string", "minLength": 1}
NUMBER = {"type": "number"}
INDEX = {"type": "integer", "minimum": 0}
BOOL = {"type": "boolean"}
Q = ref("Quantity")

defs = {}

# ---------------------------------------------------------------- authored
defs["Quantity"] = closed(
    ["value", "unit"], {"value": NUMBER, "unit": STRING},
    "Authored quantity {value, unit}. Shape only: unit-catalog membership, dimension and finiteness are "
    "producer admission rules that parse and then block (for example LOAD_STATE_QUANTITY_INVALID or "
    "LOAD_STATE_BOUNDARY_MOTION_UNIT_INVALID). Closed per CP2_WIRE; see RETURN.md for the typed-DTO note.")

defs["GeometryReference"] = tagged(
    "kind", "authored_model_geometry",
    description="The model's own normalized nodes, connectivity and pipe frames; the producer publishes its projection hash.")

defs["ReferenceBasis"] = {
    "description": "Member reference basis (tag `kind`). An unknown kind is a typed-boundary rejection.",
    "oneOf": [
        tagged("kind", "temperature_reference", ["installation_temperature"], {"installation_temperature": Q}),
        tagged("kind", "direct_strain_reference",
               description="Interval-only thermal routes; supplies no absolute temperature."),
    ],
}

defs["FitReference"] = {
    "description": "Explicit fit selection (tag `kind`); absence of fit is the explicit kind `none`. A fit carrying both "
                   "`length_change` and `strain`, or no `kind`, is rejected. The natural-length reference length is the "
                   "member's authored chord length (producer rule).",
    "oneOf": [
        tagged("kind", "none"),
        tagged("kind", "natural_length_change", ["length_change"], {"length_change": Q}),
        tagged("kind", "fit_strain", ["strain"], {"strain": Q}),
    ],
}

defs["MemberReference"] = closed(
    ["pipe_ref", "basis", "fit", "provenance"],
    {"pipe_ref": STRING, "basis": ref("ReferenceBasis"), "fit": ref("FitReference"), "provenance": STRING},
    "One member of a reference configuration. Coverage (every pipe exactly once), unknown/duplicate members and "
    "empty provenance are producer diagnostics, not shape.")

defs["ReferenceConfiguration"] = closed(
    ["id", "geometry_ref", "member_references", "provenance"],
    {"id": STRING, "label": nullable(STRING), "geometry_ref": ref("GeometryReference"),
     "member_references": {"type": "array", "items": ref("MemberReference")}, "provenance": STRING},
    "Model-level `reference_configurations[]` item: the stress-free installed state. `label` is optional; JSON null "
    "is accepted as absence, as at the typed boundary. Resolution and coverage are producer diagnostics "
    "(LOAD_STATE_REFERENCE_*).", optional=["label"])

defs["CoefficientPoint"] = closed(["temperature", "coefficient"], {"temperature": Q, "coefficient": Q})
defs["DilationPoint"] = closed(["temperature", "dilation"], {"temperature": Q, "dilation": Q})

defs["SecantData"] = {
    "description": "engineering_secant data (tag `kind`): a constant coefficient or a linear-coefficient table.",
    "oneOf": [
        tagged("kind", "constant", ["coefficient"], {"coefficient": Q}),
        tagged("kind", "table", ["interpolation", "points"], {
            "interpolation": {"const": "linear_coefficient"},
            "points": {"type": "array", "items": ref("CoefficientPoint")}}),
    ],
}
defs["DilationData"] = tagged("kind", "table", ["interpolation", "points"], {
    "interpolation": {"const": "linear_dilation"},
    "points": {"type": "array", "items": ref("DilationPoint")}},
    description="engineering_dilation data: a linear-dilation table only.")
defs["CoefficientTableData"] = tagged("kind", "table", ["interpolation", "points"], {
    "interpolation": {"const": "linear_coefficient"},
    "points": {"type": "array", "items": ref("CoefficientPoint")}},
    description="Coefficient-definition data (differential or logarithmic): a linear-coefficient table only.")


def law(definition, data):
    return tagged("definition", definition, ["id", "datum_temperature", "data", "provenance"], {
        "id": STRING, "datum_temperature": Q, "data": ref(data), "provenance": STRING})


defs["ExpansionLaw"] = {
    "description": "User-owned expansion definition carried by its material record (tag `definition`). Table "
                   "coverage, positivity, point order and unit dimension are producer diagnostics "
                   "(LOAD_STATE_EXPANSION_LAW_INVALID, LOAD_STATE_STRAIN_UNRESOLVED), not shape.",
    "oneOf": [
        law("engineering_secant", "SecantData"),
        law("engineering_dilation", "DilationData"),
        law("differential_per_datum_length", "CoefficientTableData"),
        law("logarithmic_per_current_length", "CoefficientTableData"),
    ],
}
defs["MaterialExpansionLaws"] = {
    "description": "The material-owned `expansion_laws` array. When the key is present it must be an array (JSON "
                   "null is rejected at the typed boundary); the material record is otherwise unchanged.",
    "type": "array", "items": ref("ExpansionLaw"),
}

defs["MaterialSelection"] = {
    "description": "Per-member material selection (tag `kind`). material_ref equality with the pipe's material, "
                   "E/nu basis, point/temperature resolution and override needs are producer diagnostics "
                   "(LOAD_STATE_MATERIAL_*).",
    "oneOf": [
        tagged("kind", "explicit_base_properties", ["material_ref", "applicability_reference"],
               {"material_ref": STRING, "applicability_reference": STRING}),
        tagged("kind", "exact_point", ["material_ref", "point_ref"], {"material_ref": STRING, "point_ref": STRING}),
        tagged("kind", "temperature_interpolation", ["material_ref", "temperature", "interpolation", "extrapolation"],
               {"material_ref": STRING, "temperature": Q, "interpolation": {"const": "piecewise_linear"},
                "extrapolation": {"const": "forbidden"}}),
    ],
}

defs["ThermalState"] = {
    "description": "Per-member thermal state (tag `kind`). free_length_state requires operating_temperature, a "
                   "temperature_reference basis and a law on the same material; those are producer diagnostics.",
    "oneOf": [
        tagged("kind", "unchanged_reference", ["provenance"], {"provenance": STRING}),
        tagged("kind", "explicit_interval_strain", ["strain", "interval_reference", "provenance"],
               {"strain": Q, "interval_reference": STRING, "provenance": STRING}),
        tagged("kind", "constant_alpha_interval", ["coefficient", "temperature_change", "coefficient_meaning", "provenance"],
               {"coefficient": Q, "temperature_change": Q, "coefficient_meaning": {"const": "engineering_interval"},
                "provenance": STRING}),
        tagged("kind", "free_length_state", ["expansion_law_ref"], {"expansion_law_ref": STRING}),
    ],
}

defs["AnalysisBasisOverride"] = closed(
    ["reason", "provenance"], {"reason": STRING, "provenance": STRING},
    "Explicit actual-versus-selected temperature basis override; empty text is a producer diagnostic.")

defs["ElementState"] = closed(
    ["pipe_ref", "material_selection", "thermal_state"],
    {"pipe_ref": STRING, "operating_temperature": nullable(Q), "material_selection": ref("MaterialSelection"),
     "thermal_state": ref("ThermalState"), "analysis_basis_override": nullable(ref("AnalysisBasisOverride")),
     "mass_state_ref": nullable(STRING)},
    "`analysis_state.element_states[]` item. operating_temperature, analysis_basis_override and mass_state_ref are "
    "optional (JSON null is accepted as absence, as at the typed boundary). mass_state_ref parses and then blocks "
    "(LOAD_STATE_MASS_STATE_UNSUPPORTED). Coverage is a producer diagnostic.",
    optional=["operating_temperature", "analysis_basis_override", "mass_state_ref"])

defs["PositionSource"] = {
    "description": "Locked-component position source (tag `kind`).",
    "oneOf": [
        tagged("kind", "entered", ["value"], {"value": Q}),
        tagged("kind", "predecessor_value", ["case_ref", "state_hash", "support_ref", "dof"],
               {"case_ref": STRING, "state_hash": STRING, "support_ref": STRING, "dof": STRING}),
    ],
}
defs["LockedComponent"] = closed(["dof", "position_source"], {"dof": STRING, "position_source": ref("PositionSource")})

defs["Participation"] = {
    "description": "Support participation (tag `kind`). `inactive` and `locked_equivalent_support` parse and then "
                   "block (LOAD_STATE_SUPPORT_PARTICIPATION_UNSUPPORTED) in this capability.",
    "oneOf": [
        tagged("kind", "active_model_device"),
        tagged("kind", "inactive"),
        tagged("kind", "locked_equivalent_support", ["components"],
               {"components": {"type": "array", "items": ref("LockedComponent")}}),
    ],
}

defs["Motion"] = closed(
    ["dof", "value", "meaning"],
    {"dof": STRING, "value": Q, "meaning": {"const": "absolute_reference_displacement"}},
    "Prescribed support motion. DOF validity, duplicates, rigid restraint and length/angle unit are producer "
    "diagnostics (LOAD_STATE_BOUNDARY_MOTION_*).")

defs["DeviceReference"] = {
    "description": "Device reference state (tag `kind`); parses and then blocks (LOAD_STATE_SUPPORT_REFERENCE_UNSUPPORTED).",
    "oneOf": [
        tagged("kind", "force_at_reference", ["reference_position", "force"], {"reference_position": Q, "force": Q}),
        tagged("kind", "unloaded_reference", ["reference_position"], {"reference_position": Q}),
    ],
}

defs["SupportState"] = closed(
    ["support_ref", "participation"],
    {"support_ref": STRING, "participation": ref("Participation"),
     "boundary_motion": nullable({"type": "array", "items": ref("Motion")}),
     "base_motion": nullable({"type": "array", "items": ref("Motion")}),
     "device_reference": nullable(ref("DeviceReference"))},
    "`analysis_state.support_states[]` item. boundary_motion, base_motion and device_reference are optional (JSON "
    "null is accepted as absence). base_motion and device_reference parse and then block. Coverage is a producer "
    "diagnostic.", optional=["boundary_motion", "base_motion", "device_reference"])

defs["LoadSource"] = closed(
    ["source_ref", "factor"], {"source_ref": STRING, "factor": NUMBER},
    "Complete inclusion entry for one stored primitive of the owning case. Resolution, duplicates and a finite "
    "nonzero factor are producer diagnostics (LOAD_STATE_SOURCE_*).")

defs["History"] = {
    "description": "Case history (tag `kind`); independent_equilibrium is the only kind.",
    "oneOf": [tagged("kind", "independent_equilibrium")],
}

defs["AnalysisState"] = closed(
    ["contract", "reference_configuration_ref", "element_states", "support_states", "load_sources", "history", "provenance"],
    {"contract": {"type": "string", "description": f"Supported value: {CONTRACT}. Any other string parses and then "
                  "blocks (LOAD_STATE_CONTRACT_UNSUPPORTED); it is an admission rule, not shape."},
     "reference_configuration_ref": STRING,
     "element_states": {"type": "array", "items": ref("ElementState")},
     "support_states": {"type": "array", "items": ref("SupportState")},
     "load_sources": {"type": "array", "items": ref("LoadSource")},
     "history": ref("History"), "provenance": STRING},
    "Case-owned `analysis_state` (model 0.4.0): the complete resolved-state and ordinary-source request.")

# ------------------------------------------------------------ raw evidence
defs["PhysicsGeometry"] = copy.deepcopy(RESULTS["$defs"]["PhysicsGeometry"])

pipe_material = copy.deepcopy(RESULTS["$defs"]["PhysicsMaterial"])
pipe_material["required"] = pipe_material["required"] + ["material_selection_kind", "resolved_eigenstrain"]
pipe_material["properties"]["material_selection_kind"] = {
    "enum": ["explicit_base_properties", "exact_point", "temperature_interpolation"]}
pipe_material["properties"]["resolved_eigenstrain"] = NUMBER
defs["LoadReferencePipeMaterial"] = {"description": "results.v0.3 PhysicsMaterial plus material_selection_kind and "
                                     "resolved_eigenstrain, from the same resolved members.", **pipe_material}

pressure = copy.deepcopy(PCE["properties"]["pressure"]["items"])
pressure["properties"]["materials"]["items"]["properties"]["temperature_basis"] = {"const": "resolved_member_state"}
defs["LoadReferencePressureEvidence"] = {
    "description": "results.v0.3 PhysicsContractEvidence pressure item, except materials[].temperature_basis, which "
                   "the producer publishes as the string resolved_member_state for resolved per-member pairs.",
    **pressure}

exact = copy.deepcopy(PCE["properties"]["exact_cases"]["items"])
exact["properties"]["material_basis"] = {"const": "resolved_per_member_load_reference_state_v1"}
exact["properties"]["pipe_materials"]["items"] = ref("LoadReferencePipeMaterial")
defs["LoadReferenceExactCase"] = {
    "description": "results.v0.3 PhysicsContractEvidence exact_cases item, except material_basis (const) and "
                   "pipe_materials items (LoadReferencePipeMaterial).", **exact}

defs["ConsumedMaterialPoint"] = closed(
    ["point_id", "temperature_k", "E_pa", "nu", "retained_G_ignored"],
    {"point_id": TEXT, "temperature_k": {"type": ["number", "null"]}, "E_pa": NUMBER, "nu": NUMBER,
     "retained_G_ignored": BOOL})

defs["LawSegment"] = closed(
    ["use", "lower_index", "upper_index", "start_k", "end_k"],
    {"use": {"enum": ["interpolation_sample", "integration_interval"]}, "lower_index": INDEX, "upper_index": INDEX,
     "start_k": NUMBER, "end_k": NUMBER},
    "Law-table segment. Consumed segments entered the value; consulted segments only established coverage or "
    "positivity admissibility.")

NUM_OR_NULL = {"type": ["number", "null"]}
defs["LoadReferenceMember"] = closed(
    ["pipe_id", "material_id", "material_selection_kind", "consumed_material_points", "interpolation_fraction",
     "applicability_reference", "analysis_basis_override", "selected_E_pa", "selected_nu", "derived_G_pa", "G_basis",
     "retained_G_ignored", "operating_temperature_k", "material_selection_temperature_k", "reference_basis",
     "installation_temperature_k", "thermal_definition", "expansion_law_id", "coefficient_datum_k",
     "consumed_law_point_indices", "consumed_law_segments", "consulted_law_point_indices", "consulted_law_segments",
     "installation_datum_stretch", "operating_datum_stretch", "thermal_strain", "thermal_stretch", "fit_strain",
     "fit_stretch", "total_eigenstrain", "eigenstrain_composition", "fit_kind", "fit_input", "reference_length_m"],
    {
        "pipe_id": TEXT, "material_id": TEXT,
        "material_selection_kind": {"enum": ["explicit_base_properties", "exact_point", "temperature_interpolation"]},
        "consumed_material_points": {"type": "array", "items": ref("ConsumedMaterialPoint")},
        "interpolation_fraction": NUM_OR_NULL,
        "applicability_reference": {"type": ["string", "null"], "minLength": 1},
        "analysis_basis_override": nullable(closed(["reason", "provenance"], {"reason": TEXT, "provenance": TEXT})),
        "selected_E_pa": NUMBER, "selected_nu": NUMBER, "derived_G_pa": NUMBER, "G_basis": TEXT,
        "retained_G_ignored": BOOL,
        "operating_temperature_k": NUM_OR_NULL, "material_selection_temperature_k": NUM_OR_NULL,
        "reference_basis": {"enum": ["temperature_reference", "direct_strain_reference"]},
        "installation_temperature_k": NUM_OR_NULL,
        "thermal_definition": {"enum": ["unchanged_reference", "explicit_interval_strain", "constant_alpha_interval",
                                        "engineering_secant", "engineering_dilation", "differential_per_datum_length",
                                        "logarithmic_per_current_length"]},
        "expansion_law_id": {"type": ["string", "null"], "minLength": 1},
        "coefficient_datum_k": NUM_OR_NULL,
        "consumed_law_point_indices": {"type": "array", "items": INDEX},
        "consumed_law_segments": {"type": "array", "items": ref("LawSegment")},
        "consulted_law_point_indices": {"type": "array", "items": INDEX},
        "consulted_law_segments": {"type": "array", "items": ref("LawSegment")},
        "installation_datum_stretch": NUM_OR_NULL, "operating_datum_stretch": NUM_OR_NULL,
        "thermal_strain": NUMBER, "thermal_stretch": NUMBER, "fit_strain": NUMBER, "fit_stretch": NUMBER,
        "total_eigenstrain": NUMBER,
        "eigenstrain_composition": {"const": "lambda_fit*lambda_thermal-1"},
        "fit_kind": {"enum": ["none", "natural_length_change", "fit_strain"]},
        "fit_input": {"oneOf": [{"type": "null"},
                                closed(["length_change_m"], {"length_change_m": NUMBER}),
                                closed(["strain"], {"strain": NUMBER})]},
        "reference_length_m": NUMBER,
    },
    "One resolved member (CP2_WIRE_ADDENDUM_1 section 3). Absent optional values are JSON null. Cross-field "
    "bindings (fit_kind with fit_input, E/nu/G with exact_cases, coverage) belong to the reader validators.")

defs["SupportComponent"] = {
    **closed(
        ["support_id", "node_id", "dof", "global_dof", "law_kind", "prescribed_value", "unit", "meaning",
         "physical_state_source"],
        {"support_id": TEXT, "node_id": TEXT, "dof": {"enum": ["UX", "UY", "UZ", "RX", "RY", "RZ"]},
         "global_dof": INDEX, "law_kind": {"const": "rigid_prescribed"}, "prescribed_value": NUMBER,
         "unit": {"enum": ["m", "rad"]}, "meaning": {"const": "absolute_reference_displacement"},
         "physical_state_source": {"const": "support_state.boundary_motion"}},
        "One entered boundary_motion DOF; every other restrained DOF is an explicit zero boundary value. "
        "Translations publish m and rotations rad."),
    "oneOf": [
        {"properties": {"dof": {"enum": ["UX", "UY", "UZ"]}, "unit": {"const": "m"}}},
        {"properties": {"dof": {"enum": ["RX", "RY", "RZ"]}, "unit": {"const": "rad"}}},
    ],
}

defs["Contribution"] = {
    "description": "Complete physical-source ledger entry (tag `owner_kind`, fixed classification per owner). The "
                   "support_state `value` and the pressure_region null `factor` are published by the producer but "
                   "not listed in CP2_WIRE_ADDENDUM_1 section 3 (see RETURN.md).",
    "oneOf": [
        tagged("owner_kind", "stored_primitive",
               ["source_id", "classification", "factor", "category", "dimension", "authored_normalized_magnitude",
                "applied_magnitude"],
               {"source_id": TEXT, "classification": {"const": "ordinary_applied"}, "factor": NUMBER,
                "category": TEXT, "dimension": TEXT, "authored_normalized_magnitude": NUMBER,
                "applied_magnitude": NUMBER}),
        tagged("owner_kind", "resolved_member_state", ["source_id", "classification", "consumed_input_refs", "value"],
               {"source_id": TEXT, "classification": {"const": "eigenstrain"},
                "consumed_input_refs": {"type": "array", "items": TEXT}, "value": NUMBER}),
        tagged("owner_kind", "support_state", ["source_id", "classification", "value"],
               {"source_id": TEXT, "classification": {"const": "prescribed_boundary"}, "value": NUMBER}),
        tagged("owner_kind", "pressure_region", ["source_id", "classification", "factor"],
               {"source_id": TEXT, "classification": {"const": "pressure_eigen_and_closure"}, "factor": {"type": "null"}}),
    ],
}

defs["ExcludedSource"] = closed(
    ["source_id", "owner_kind", "classification", "category", "reason"],
    {"source_id": TEXT, "owner_kind": {"const": "stored_primitive"}, "classification": {"const": "excluded"},
     "category": TEXT, "reason": TEXT},
    "A stored primitive of the case not listed in analysis_state.load_sources.")

defs["LoadReferenceStateRecord"] = closed(
    ["load_case_id", "contract", "profile", "reference_configuration_id", "provenance", "reference_geometry",
     "history", "solve", "source_recovery", "members", "support_components", "contributions", "excluded_sources"],
    {
        "load_case_id": TEXT,
        "contract": {"const": CONTRACT},
        "profile": {"const": PROFILE},
        "reference_configuration_id": TEXT,
        "provenance": TEXT,
        "reference_geometry": closed(["kind", "projection_sha256"], {
            "kind": {"const": "authored_model_geometry"},
            "projection_sha256": {"type": "string", "pattern": "^[0-9a-f]{64}$"}}),
        "history": closed(["kind"], {"kind": {"const": "independent_equilibrium"}}),
        "solve": {
            **closed(["requested_mode", "recovery_method", "boundary", "eigenload"], {
                "requested_mode": {"enum": ["sparse_interactive", "dense_scrutiny"]},
                "recovery_method": {"enum": ["ordinary_sparse_structural_v1", "ordinary_dense_structural_v1"]},
                "boundary": TEXT, "eigenload": TEXT}),
            "oneOf": [
                {"properties": {"requested_mode": {"const": "sparse_interactive"},
                                "recovery_method": {"const": "ordinary_sparse_structural_v1"}}},
                {"properties": {"requested_mode": {"const": "dense_scrutiny"},
                                "recovery_method": {"const": "ordinary_dense_structural_v1"}}},
            ],
        },
        "source_recovery": closed(["status", "code"], {
            "status": {"const": "not_joined"}, "code": {"const": "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"}}),
        "members": {"type": "array", "items": ref("LoadReferenceMember")},
        "support_components": {"type": "array", "items": ref("SupportComponent")},
        "contributions": {"type": "array", "items": ref("Contribution")},
        "excluded_sources": {"type": "array", "items": ref("ExcludedSource")},
    },
    "One `contract_evidence.load_reference_states[]` record per case, exactly the CP2_WIRE_ADDENDUM_1 section 3 "
    "keys. Case binding, uniqueness and member/pipe bijection belong to the reader validators.")

defs["LoadReferenceContractEvidence"] = closed(
    ["pressure", "connector", "exact_cases", "load_reference_states"],
    {"pressure": {"type": "array", "items": ref("LoadReferencePressureEvidence")},
     "connector": {"const": []},
     "exact_cases": {"type": "array", "items": ref("LoadReferenceExactCase")},
     "load_reference_states": {"type": "array", "items": ref("LoadReferenceStateRecord")}},
    "Raw producer `contract_evidence` for semantic contract openpipestress.result_semantics/0.3.0/load-reference-1 "
    f"(profile {PROFILE}). Carriers preserve it verbatim. Closed shape only; cross-binding is executable reader "
    "validation.")

schema = {
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "$id": "https://openpipestress.org/schemas/load_reference_state.schema.json",
    "title": "Load/reference-state authored namespace and load-reference-1 raw evidence",
    "description": (
        f"Definitions library for {CONTRACT} (model 0.4.0) and the load-reference-1 raw producer evidence. Validate a "
        "specific $defs entry; the root carries no assertions. Authored definitions describe typed-boundary shape "
        "only: an unknown field, a missing required field or an unknown discriminant is rejected before solve, while "
        "coverage, resolution, unit dimension, finiteness and the parse-then-block branches are producer diagnostics "
        "and are not encoded here. Every $ref is a local #/$defs/<name> reference so the repository's closed-shape "
        "checkers can use this file as their root."),
    "$defs": defs,
}


def check_vocabulary(node, path="#"):
    """Keep to the vocabulary interpreted by the repository's closed-shape checkers."""
    allowed = {"$schema", "$id", "title", "description", "$defs", "type", "additionalProperties", "required",
               "properties", "items", "const", "enum", "oneOf", "anyOf", "allOf", "$ref", "minLength", "minItems",
               "maxItems", "minimum", "maximum", "pattern"}
    if isinstance(node, dict):
        in_properties = path.endswith("/properties") or path.endswith("/$defs")
        for key, value in node.items():
            if not in_properties:
                assert key in allowed, f"{path}/{key}"
                if key == "$ref":
                    assert value.startswith("#/$defs/") and "/" not in value[len("#/$defs/"):], value
                if key == "pattern":
                    assert value in {"^[0-9a-f]{64}$", "^[0-9a-f]{16}$"}, value
                if key == "type" and (value == "object" or value == ["object"]):
                    assert "properties" in node, path
                if key == "type" and value == "array":
                    assert "items" in node, path
            if key in ("const", "enum"):
                continue
            check_vocabulary(value, f"{path}/{key}")
    elif isinstance(node, list):
        for index, value in enumerate(node):
            check_vocabulary(value, f"{path}/{index}")


check_vocabulary(schema)
text = json.dumps(schema, indent=2) + "\n"
json.loads(text)
target = ROOT / "schemas/load_reference_state.schema.json"
fd, temporary = tempfile.mkstemp(prefix=".load_reference_state.", suffix=".tmp", dir=target.parent)
with os.fdopen(fd, "w", encoding="utf-8") as handle:
    handle.write(text)
os.chmod(temporary, 0o644)
os.replace(temporary, target)
print(target.relative_to(ROOT))
