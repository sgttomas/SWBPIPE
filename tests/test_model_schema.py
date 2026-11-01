#!/usr/bin/env python3
"""Contract checks for the model document schema (schemas/model.schema.yaml).

The schema describes the desktop's model document, `openpipestress.product_preview.model`
0.9.0 (owner OD-4, 2026-10-23). Its root is the format the Rust reader decodes; its
`$defs/DesktopDocument` is the desktop's open and save profile. Committed documents are
checked against it in test_model_document_schema.py. The transform's older project/model
shape is schemas/physical_transform_model.schema.yaml (test_physical_transform_model_schema.py).
"""

import json
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
TESTS_DIR = Path(__file__).resolve().parent
if str(TESTS_DIR) not in sys.path:
    sys.path.insert(0, str(TESTS_DIR))

from copy import deepcopy  # noqa: E402

import pytest  # noqa: E402

from schema_validation import load_schema, validate_instance, validate_schema_document, walk_keys, walk_strings  # noqa: E402

SCHEMA_PATH = ROOT / "schemas" / "model.schema.yaml"
WIRE_SOURCE = ROOT / "core" / "product_physics" / "src" / "lib.rs"
DESKTOP_SHAPE = ROOT / "apps" / "desktop" / "src" / "features" / "model-file" / "modelDocumentShape.ts"
LRS = "load_reference_state.schema.json#/$defs/"
DEMO = ROOT / "fixtures" / "product_preview" / "invented_demo_model.json"
DESKTOP_PROFILE = {"$schema": "https://json-schema.org/draft/2020-12/schema",
                   "$ref": "https://openpipestress.org/schemas/model.schema.yaml#/$defs/DesktopDocument"}

FORBIDDEN_SCHEMA_TEXT = {
    "CODE_COMPLIANT",
    "ASME",
    "B31",
    "certified",
    "sealed",
    "automatic compliance",
    "professional approval by the software",
}


def schema():
    return load_schema(SCHEMA_PATH)


def defs():
    return schema()["$defs"]


def object_schemas(value, path="$"):
    """Every subschema that declares `properties` or `type: object`, with its JSON path."""
    if not isinstance(value, dict):
        return
    if "properties" in value or value.get("type") == "object" or "object" in (value.get("type") or []):
        yield path, value
    for keyword in ("properties", "$defs"):
        for name, item in value.get(keyword, {}).items():
            yield from object_schemas(item, f"{path}.{keyword}.{name}")
    for keyword in ("items", "additionalProperties"):  # `not` holds negated conditions, not records
        yield from object_schemas(value.get(keyword), f"{path}.{keyword}")
    for keyword in ("allOf", "oneOf", "anyOf"):
        for index, item in enumerate(value.get(keyword, [])):
            yield from object_schemas(item, f"{path}.{keyword}[{index}]")


def wire_required_members():
    """Members `PreviewModelWire` (the reader's wire form) decodes without a default."""
    source = WIRE_SOURCE.read_text(encoding="utf-8")
    body = re.search(r"struct PreviewModelWire \{(.*?)\n\}", source, re.S).group(1)
    required, defaulted = [], False
    for line in body.splitlines():
        line = line.strip()
        if line.startswith("#[serde(") and "default" in line:
            defaulted = True
            continue
        field = re.match(r"(\w+):", line)
        if field:
            if not defaulted:
                required.append(field.group(1))
            defaulted = False
    return required


def desktop_entity_requirements():
    """modelDocumentShape.ts ENTITY_STRINGS: collection -> (required strings, optional collection)."""
    source = DESKTOP_SHAPE.read_text(encoding="utf-8")
    rows = re.findall(r'\["(\w+)", \[([^\]]*)\](, true)?\]', source)
    return {name: (re.findall(r'"(\w+)"', fields), bool(optional)) for name, fields, optional in rows}


def test_schema_is_a_draft_2020_12_document_with_stable_identity():
    document = schema()
    assert document["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert document["$id"] == "https://openpipestress.org/schemas/model.schema.yaml"
    assert validate_schema_document(document, schema_label=str(SCHEMA_PATH))
    assert "default" not in set(walk_keys(document))


def test_root_is_the_current_model_document_format():
    document = schema()
    assert document["additionalProperties"] is False
    assert document["properties"]["schema_version"]["enum"] == ["0.9.0"]
    assert document["properties"]["document_kind"]["const"] == "openpipestress.product_preview.model"
    # The root requires exactly what the Rust reader's wire form requires.
    assert sorted(document["required"]) == sorted(wire_required_members())
    assert set(document["required"]) == {
        "schema_version", "document_kind", "project", "analysis_status", "nodes", "pipe_segments", "supports",
    }
    # The rejection-only `shear_model` presence is never a document member.
    assert "shear_model" not in document["properties"]
    assert "implicit global" in document["description"]


def test_every_record_is_closed():
    open_records = []
    for path, record in object_schemas(schema()):
        if path.startswith("$.$defs.DesktopDocument"):
            continue  # the profile overlay adds requirements to the closed format records
        if record.get("additionalProperties") is not False:
            open_records.append(path)
    # Section.properties is a map of quantities, not a record. Section provenance
    # is a string or an object whose members no reader reads and no writer emits
    # (the section reader takes any JSON value).
    # data_boundary is the desktop's Record<string, string>.
    # The open maps are typed by the desktop as Record<string, ...>; the two
    # provenance unions take an object whose members nothing reads.
    assert open_records == [
        "$.$defs.DataBoundary",
        "$.$defs.Diagnostic.properties.provenance",
        "$.$defs.ProjectUnits",
        "$.$defs.Section.properties.properties",
        "$.$defs.Section.properties.provenance",
        "$.$defs.Support.properties.properties",
    ], open_records
    d = defs()
    assert d["Section"]["properties"]["properties"]["additionalProperties"] == {"$ref": "#/$defs/Quantity"}
    assert d["Support"]["properties"]["properties"]["additionalProperties"] == {"$ref": "#/$defs/Quantity"}
    assert d["DataBoundary"]["additionalProperties"] == {"type": "string"}
    assert d["ProjectUnits"]["additionalProperties"] == {"type": "string"}


def test_required_record_families_are_named():
    """REQ-02-01-02: Project, Model, Node, Element, Material, Component, Load/LoadCase, Result, Report."""
    d = defs()
    assert {"Project", "Model", "Node", "Element", "Material", "Component", "LoadCase", "LoadRecord",
            "Result", "Report"} <= set(d)
    assert d["Model"]["$ref"] == "#"
    assert d["Element"]["$ref"] == "#/$defs/PipeSegment"


def test_quantities_and_identities():
    quantity = defs()["Quantity"]
    assert quantity["required"] == ["value", "unit"]
    assert set(quantity["properties"]) == {"value", "unit"}
    # Readers refuse only a blank identity (EMPTY_ID); Unicode identities are admitted.
    pattern = re.compile(defs()["Id"]["pattern"])
    for valid in ("node:N-100", "support:锚", "load:𝔫", "invented:o4|zero"):
        assert pattern.search(valid)
    for blank in ("", " ", "\t"):
        assert not pattern.search(blank)


def test_load_reference_state_group_is_referenced_unchanged():
    d = defs()
    assert schema()["properties"]["reference_configurations"]["items"] == {"$ref": LRS + "ReferenceConfiguration"}
    assert d["Material"]["properties"]["expansion_laws"] == {"$ref": LRS + "MaterialExpansionLaws"}
    assert d["LoadCase"]["properties"]["analysis_state"] == {"$ref": LRS + "AnalysisState"}


def test_load_case_and_load_record_are_the_desktop_records_bound_by_the_load_crates():
    d = defs()
    load_case = d["LoadCase"]
    assert load_case["properties"]["primitive_loads"]["items"] == {"$ref": "#/$defs/LoadRecord"}
    assert {"label", "kind", "status", "pressure_regions", "equivalent_static", "modulus_basis_ref",
            "modulus_basis_temperature", "analysis_state"} <= set(load_case["properties"])
    assert load_case["not"]["required"] == ["modulus_basis_ref", "modulus_basis_temperature"]
    assert set(d["LoadRecord"]["required"]) == {"id", "category", "target", "direction", "magnitude", "dimension"}
    for source in ("core/loads/primitive_loads/src/lib.rs", "core/loads/user_loads/src/lib.rs"):
        text = (ROOT / source).read_text(encoding="utf-8")
        assert "schemas/model.schema.yaml#/$defs/LoadRecord" in text


def test_known_vocabularies():
    d = defs()
    assert set(d["SupportFamily"]["enum"]) == {
        "anchor", "guide", "line_stop", "vertical_support", "spring", "variable_spring_hanger",
        "spring_hanger", "constant_effort_support", "nonlinear",
        # Model document 0.9.0 (DEL-02-01 Design model-document-0.9.0.md §2.4): the dynamics lane's snubber.
        "snubber",
    }
    assert {"UX", "UY", "UZ", "RX", "RY", "RZ"} <= set(d["Dof"]["enum"])
    assert {"weight", "distributed_force", "thermal", "concentrated_force", "concentrated_moment",
            "imposed_displacement", "pressure"} <= set(d["LoadCategory"]["enum"])
    assert {"global_x", "global_y", "global_z", "rotation_x", "rotation_y", "rotation_z"} <= set(d["LoadDirection"]["enum"])
    assert d["Combination"]["properties"]["basis"]["enum"] == ["mechanics", "result_state_subtraction", "range_envelope",
                                                                # Model document 0.9.0 (§2.4; RT-A4).
                                                                "absolute_sum", "srss"]
    assert d["Combination"]["properties"]["mode"]["enum"] == ["min", "max", "min_abs", "max_abs"]
    assert {"one_way", "gap", "lift_off", "friction"} <= set(d["NonlinearSupport"]["properties"]["behavior"]["enum"])
    contract = d["PressureContract"]
    assert contract["required"] == ["version", "mode", "shear_model"]
    assert contract["properties"]["version"]["const"] == "3.0.0"
    assert contract["properties"]["mode"]["const"] == "exact_pressure_v3"
    assert contract["properties"]["shear_model"]["enum"] == ["euler_bernoulli", "timoshenko_annulus_energy_v1"]
    assert d["ObjectiveConnector"]["properties"]["version"]["const"] == "1.0.0"
    targets = [branch["properties"]["type"]["const"] for branch in d["LoadTarget"]["oneOf"]]
    assert targets == ["node", "element", "support"]


def test_desktop_document_is_the_format_plus_the_desktop_shape_check():
    profile = defs()["DesktopDocument"]
    format_ref, overlay = profile["allOf"]
    assert format_ref == {"$ref": "model.schema.yaml"}
    assert set(overlay["properties"]["project"]["required"]) == {"id", "name", "description", "units"}
    always = {"data_boundary", "project", "analysis_status"}
    required_collections = set()
    for collection, (fields, optional) in desktop_entity_requirements().items():
        if not optional:
            required_collections.add(collection)
        if collection == "diagnostics":
            assert defs()["Diagnostic"]["required"] == fields
            continue
        required = set(overlay["properties"][collection]["items"]["required"])
        expected = set(fields) | {"provenance"} if collection == "sections" else set(fields)
        assert required == expected, collection
    # `terms` is required except on the bases the shape check exempts (`TERMLESS_BASES`, model document 0.9.0).
    termless = re.search(r"TERMLESS_BASES[^=]*= new Set\(\[([^\]]*)\]\)", DESKTOP_SHAPE.read_text(encoding="utf-8"))
    combination = overlay["properties"]["combinations"]["items"]
    assert combination["if"] == {"not": {"properties": {"basis": {"enum": re.findall(r'"(\w+)"', termless.group(1))}}, "required": ["basis"]}}
    assert combination["then"] == {"required": ["terms"]}
    assert required_collections == {"nodes", "pipe_segments", "supports", "components", "load_cases", "diagnostics"}
    root_required = set(schema()["required"])
    assert set(overlay["required"]) | root_required >= required_collections | always
    assert set(overlay["required"]) == (required_collections | always) - root_required


def test_schema_text_makes_no_claim():
    text = "\n".join(walk_strings(schema()))
    for forbidden in FORBIDDEN_SCHEMA_TEXT:
        assert forbidden not in text


def demo():
    return json.loads(DEMO.read_text(encoding="utf-8"))


def accepted(document, schema=None):
    validate_instance(schema or SCHEMA_PATH_SCHEMA(), document, schema_label=str(SCHEMA_PATH))


def rejected(document, schema=None):
    with pytest.raises(AssertionError):
        validate_instance(schema or SCHEMA_PATH_SCHEMA(), document, schema_label=str(SCHEMA_PATH))


def SCHEMA_PATH_SCHEMA():  # noqa: N802 - the format schema itself
    return schema()


def with_change(change):
    document = demo()
    change(document)
    return document


def section_of(document, index=0):
    return document["pipe_segments"][index]["section"]


# null where a writer emits it or a reader gives it a meaning (see each slot's description).
NULL_ACCEPTED = {
    "support.family": lambda d: d["supports"][0].__setitem__("family", None),
    "support.nonlinear": lambda d: d["supports"][0].__setitem__("nonlinear", None),
    "node.label": lambda d: d["nodes"][0].__setitem__("label", None),
    "node.provenance": lambda d: d["nodes"][0].__setitem__("provenance", None),
    "pipe.label": lambda d: d["pipe_segments"][0].__setitem__("label", None),
    "pipe.provenance": lambda d: d["pipe_segments"][0].__setitem__("provenance", None),
    "section.mill_tolerance": lambda d: section_of(d).__setitem__("mill_tolerance", None),
    "section.material_density": lambda d: section_of(d).__setitem__("material_density", None),
    "section.contents_density": lambda d: section_of(d).__setitem__("contents_density", None),
    "section.insulation_thickness": lambda d: section_of(d).__setitem__("insulation_thickness", None),
    "section.insulation_density": lambda d: section_of(d).__setitem__("insulation_density", None),
    "load_case.modulus_basis_ref": lambda d: d["load_cases"][0].__setitem__("modulus_basis_ref", None),
    "load_case.modulus_basis_temperature": lambda d: d["load_cases"][0].__setitem__("modulus_basis_temperature", None),
}
# null that the readers refuse or that nothing writes stays invalid.
NULL_REJECTED = {
    "section.corrosion_allowance": lambda d: section_of(d).__setitem__("corrosion_allowance", None),
    "pipe.section_ref": lambda d: d["pipe_segments"][0].__setitem__("section_ref", None),
    "pressure_contract": lambda d: d.__setitem__("pressure_contract", None),
    "support.stiffness": lambda d: d["supports"][0].__setitem__("stiffness", None),
    "support.boundary_association": lambda d: d["supports"][0].__setitem__("boundary_association", None),
    "load_case.pressure_regions": lambda d: d["load_cases"][0].__setitem__("pressure_regions", None),
    "reference_configurations": lambda d: d.__setitem__("reference_configurations", None),
    "material.label": lambda d: d["materials"][0].__setitem__("label", None),
    "component.label": lambda d: d["components"][0].__setitem__("label", None),
}


@pytest.mark.parametrize("slot", sorted(NULL_ACCEPTED))
def test_null_is_accepted_where_writers_or_readers_use_it(slot):
    accepted(with_change(NULL_ACCEPTED[slot]))


@pytest.mark.parametrize("slot", sorted(NULL_REJECTED))
def test_null_is_refused_elsewhere(slot):
    rejected(with_change(NULL_REJECTED[slot]))


def test_desktop_profile_still_requires_string_labels_and_provenance():
    for slot in ("node.label", "node.provenance", "pipe.label", "pipe.provenance"):
        rejected(with_change(NULL_ACCEPTED[slot]), DESKTOP_PROFILE)


def test_desktop_profile_requires_terms_except_on_a_subtraction_or_envelope():
    """Model document 0.9.0 (section 2.4): a subtraction or envelope may omit `terms`; another basis may not."""
    def combination(basis, **members):
        def change(document):
            first, second = (case["id"] for case in document["load_cases"][:2])
            operands = {"result_state_subtraction": {"minuend_id": first, "subtrahend_id": second},
                        "range_envelope": {"operand_ids": [first, second], "mode": "max"}}.get(basis, {})
            document["combinations"] = [{"id": "combination:C-NEW", "label": "New", "basis": basis, "provenance": "invented",
                                         **operands, **members}]
        return with_change(change)
    for basis in ("result_state_subtraction", "range_envelope"):
        accepted(combination(basis), DESKTOP_PROFILE)
    for basis in ("mechanics", "absolute_sum", "srss"):
        accepted(combination(basis))
        rejected(combination(basis), DESKTOP_PROFILE)
        accepted(combination(basis, terms=[]), DESKTOP_PROFILE)


SECTION_NULL_SLOTS = ("mill_tolerance", "material_density", "contents_density", "insulation_thickness",
                      "insulation_density")


@pytest.mark.parametrize("slot", SECTION_NULL_SLOTS)
def test_desktop_profile_refuses_a_null_section_quantity_the_format_accepts(slot):
    """The desktop shape check reads every section member as a {value, unit} quantity."""
    document = with_change(NULL_ACCEPTED[f"section.{slot}"])
    accepted(document)
    rejected(document, DESKTOP_PROFILE)
    present = with_change(lambda d: section_of(d).__setitem__(slot, {"value": 1.0, "unit": "m"}))
    accepted(present, DESKTOP_PROFILE)


def test_conflicting_modulus_bases_are_refused_but_a_null_partner_is_not_a_conflict():
    def both(document, temperature):
        case = document["load_cases"][0]
        case["modulus_basis_ref"] = "temperature-point:invented"
        case["modulus_basis_temperature"] = temperature
    rejected(with_change(lambda d: both(d, {"value": 400.0, "unit": "K"})))
    accepted(with_change(lambda d: both(d, None)))


def test_diagnostic_provenance_is_a_string_or_an_object():
    accepted(with_change(lambda d: d["diagnostics"][0].__setitem__("provenance", "invented_example")))
    accepted(with_change(lambda d: d["diagnostics"][0].__setitem__(
        "provenance", {"source_name": "invented producer", "note": "shown as JSON"})))
    rejected(with_change(lambda d: d["diagnostics"][0].__setitem__("provenance", 7)))


def test_desktop_typed_open_maps_take_any_key_with_the_typed_value():
    accepted(with_change(lambda d: d["project"]["units"].__setitem__("mass", "kg")))
    rejected(with_change(lambda d: d["project"]["units"].__setitem__("mass", 1)))
    accepted(with_change(lambda d: d["data_boundary"].__setitem__("source", "invented_example")))
    rejected(with_change(lambda d: d["data_boundary"].__setitem__("source", None)))
    properties = {"linear_stiffness": {"value": 1.0e6, "unit": "N/m"}, "invented_rate": {"value": 2.0, "unit": "N/m"}}
    accepted(with_change(lambda d: d["supports"][0].__setitem__("properties", deepcopy(properties))))
    rejected(with_change(lambda d: d["supports"][0].__setitem__("properties", {"linear_stiffness": 1.0e6})))

