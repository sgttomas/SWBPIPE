#!/usr/bin/env python3
"""Contract checks for the model document schema (schemas/model.schema.yaml).

The schema describes the desktop's model document, `openpipestress.product_preview.model`
0.7.0 (owner OD-4, 2026-10-23). Its root is the format the Rust reader decodes; its
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

from schema_validation import load_schema, validate_schema_document, walk_keys, walk_strings  # noqa: E402

SCHEMA_PATH = ROOT / "schemas" / "model.schema.yaml"
WIRE_SOURCE = ROOT / "core" / "product_physics" / "src" / "lib.rs"
DESKTOP_SHAPE = ROOT / "apps" / "desktop" / "src" / "features" / "model-file" / "modelDocumentShape.ts"
LRS = "load_reference_state.schema.json#/$defs/"

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
    for keyword in ("items", "additionalProperties", "not"):
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
    assert document["properties"]["schema_version"]["enum"] == ["0.7.0"]
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
    assert open_records == [
        "$.$defs.DataBoundary",
        "$.$defs.Section.properties.properties",
        "$.$defs.Section.properties.provenance",
    ], open_records
    assert defs()["Section"]["properties"]["properties"]["additionalProperties"] == {"$ref": "#/$defs/Quantity"}
    assert defs()["DataBoundary"]["additionalProperties"] == {"type": "string"}


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
    assert load_case["not"] == {"required": ["modulus_basis_ref", "modulus_basis_temperature"]}
    assert set(d["LoadRecord"]["required"]) == {"id", "category", "target", "direction", "magnitude", "dimension"}
    for source in ("core/loads/primitive_loads/src/lib.rs", "core/loads/user_loads/src/lib.rs"):
        text = (ROOT / source).read_text(encoding="utf-8")
        assert "schemas/model.schema.yaml#/$defs/LoadRecord" in text


def test_known_vocabularies():
    d = defs()
    assert set(d["SupportFamily"]["enum"]) == {
        "anchor", "guide", "line_stop", "vertical_support", "spring", "variable_spring_hanger",
        "spring_hanger", "constant_effort_support", "nonlinear",
    }
    assert {"UX", "UY", "UZ", "RX", "RY", "RZ"} <= set(d["Dof"]["enum"])
    assert {"weight", "distributed_force", "thermal", "concentrated_force", "concentrated_moment",
            "imposed_displacement", "pressure"} <= set(d["LoadCategory"]["enum"])
    assert {"global_x", "global_y", "global_z", "rotation_x", "rotation_y", "rotation_z"} <= set(d["LoadDirection"]["enum"])
    assert d["Combination"]["properties"]["basis"]["enum"] == ["mechanics", "result_state_subtraction", "range_envelope"]
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
        if collection == "combinations":
            expected |= {"terms"}
        assert required == expected, collection
    assert required_collections == {"nodes", "pipe_segments", "supports", "components", "load_cases", "diagnostics"}
    root_required = set(schema()["required"])
    assert set(overlay["required"]) | root_required >= required_collections | always
    assert set(overlay["required"]) == (required_collections | always) - root_required


def test_schema_text_makes_no_claim():
    text = "\n".join(walk_strings(schema()))
    for forbidden in FORBIDDEN_SCHEMA_TEXT:
        assert forbidden not in text
