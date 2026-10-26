#!/usr/bin/env python3
"""Schema check of the desktop's representative FR-REP-003 records (W-11 slice (c)).

DEL-08-01 Design `exact-route-report-sections.md` section 7: the desktop builder
writes `fixtures/reports/invented/report_sections_provenance_records.json`
(`apps/desktop/src/features/report/renderedReport.test.tsx`, stale fails) from an
invented model whose material, section, component and support each appear with
and without a `provenance_record`, with records marked accepted, rejected and
quarantined. Every `provenance_notes` entry is validated against
`$defs/Provenance` and every `user_supplied_values` entry against
`$defs/UserSuppliedValue` of `schemas/report_sections.schema.yaml`.

The pre-existing divergence (design section 9) is named and excluded by value-id
prefix, `component-provenance:` and `spring-hanger:`, for `value_category` and
`quantity` only; those entries' records are still validated. Any other error,
including a new divergence, fails. Invented data only.
"""

import json
from pathlib import Path

import pytest

jsonschema = pytest.importorskip("jsonschema")


ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "schemas" / "report_sections.schema.yaml"
FIXTURE_PATH = ROOT / "fixtures" / "reports" / "invented" / "report_sections_provenance_records.json"

DIVERGENT_PREFIXES = ("component-provenance:", "spring-hanger:")
DIVERGENT_MEMBERS = {"value_category", "quantity"}


def _validator(definition):
    schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
    subschema = {
        "$schema": schema["$schema"],
        "$id": schema["$id"],
        "$defs": schema["$defs"],
        "$ref": f"#/$defs/{definition}",
    }
    jsonschema.Draft202012Validator.check_schema(subschema)
    return jsonschema.Draft202012Validator(subschema)


def _sections():
    return json.loads(FIXTURE_PATH.read_text(encoding="utf-8"))["report_sections"]


def _errors(validator, instance, excluded=frozenset()):
    return [
        f"{list(error.absolute_path)}: {error.message}"
        for error in validator.iter_errors(instance)
        if not (error.absolute_path and error.absolute_path[0] in excluded)
    ]


def test_fixture_covers_each_entity_with_and_without_a_record_and_each_review():
    sections = _sections()
    values = {value["value_id"]: value for value in sections["user_supplied_values"]}
    kinds = {value_id.split(":", 1)[0] for value_id in values}
    assert {"component-provenance", "material-provenance", "section-provenance", "support-provenance"} <= kinds
    reviews = {note["review_status"] for note in sections["provenance_notes"]}
    assert {"accepted", "rejected", "quarantined", "pending"} <= reviews
    # With a record (copied names) and without one (the component pattern), per kind.
    names = [note["source_name"] for note in sections["provenance_notes"]]
    assert any(name.endswith(" material provenance") for name in names)
    assert any(name.endswith(" section provenance") for name in names)
    assert any(name.endswith(" component provenance") for name in names)
    assert "Invented accepted datasheet" in names and "Invented rejected table" in names
    assert "Invented quarantined component record" in names and "Invented support record" in names


def test_every_provenance_note_is_a_schema_provenance():
    validator = _validator("Provenance")
    failures = {
        index: errors
        for index, note in enumerate(_sections()["provenance_notes"])
        if (errors := _errors(validator, note))
    }
    assert failures == {}


def test_every_user_supplied_value_is_a_schema_value_apart_from_the_named_divergence():
    validator = _validator("UserSuppliedValue")
    provenance = _validator("Provenance")
    failures = {}
    for value in _sections()["user_supplied_values"]:
        excluded = DIVERGENT_MEMBERS if value["value_id"].startswith(DIVERGENT_PREFIXES) else frozenset()
        errors = _errors(validator, value, excluded) + _errors(provenance, value["provenance"])
        if errors:
            failures[value["value_id"]] = errors
    assert failures == {}


def test_the_named_divergence_is_only_what_section_9_names():
    """The excluded members really diverge only for the pre-existing entries, and
    the new entries use `report_metadata` and omit `quantity`."""
    validator = _validator("UserSuppliedValue")
    for value in _sections()["user_supplied_values"]:
        if value["value_id"].startswith(DIVERGENT_PREFIXES):
            members = {error.absolute_path[0] for error in validator.iter_errors(value) if error.absolute_path}
            assert members <= DIVERGENT_MEMBERS, (value["value_id"], members)
        else:
            assert value["value_category"] == "report_metadata"
            assert "quantity" not in value
