#!/usr/bin/env python3
"""Checks for public/private library import provenance validation."""

import copy
import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from core.library_import.provenance_checker import validate_library_import  # noqa: E402


MATERIAL_FIXTURE = ROOT / "fixtures" / "material" / "invented_material_library_valid.json"
COMPONENT_FIXTURE = ROOT / "fixtures" / "component" / "invented_section_component_library_valid.json"


def load_json(path):
    with path.open(encoding="utf-8") as handle:
        return json.load(handle)


def accepted_public_component_payload():
    payload = copy.deepcopy(load_json(COMPONENT_FIXTURE))
    payload["component_library"]["library_scope"] = "public_permissive_reviewed"
    payload["component_library"]["privacy_class"] = "public_permissive_reviewed"
    payload["component_library"]["review_status"] = "accepted"
    payload["component_library"]["provenance"]["source_license"] = "public test license"
    payload["component_library"]["provenance"]["redistribution_status"] = "public_permissive"
    payload["component_library"]["provenance"]["review_status"] = "accepted"
    payload["component_records"] = [
        {
            "component_id": "comp.public.example",
            "name": "Public Reviewed Example",
            "component_type": "other",
            "privacy_class": "public_permissive_reviewed",
            "redistribution_status": "public_permissive",
            "fields": [
                {
                    "field_id": "comp.public.example.weight",
                    "field_kind": "weight",
                    "value_status": "public_permissive_reviewed",
                    "public_repository_value_policy": "public_permissive_reviewed_only",
                    "required_for": "mechanics_solve",
                    "value": {
                        "magnitude": 1.0,
                        "unit": "N",
                        "dimension": "force",
                        "value_status": "public_permissive_reviewed",
                        "provenance": {
                            "source_name": "Invented permissive test source",
                            "source_location": "tests/test_library_import_provenance.py",
                            "source_license": "public test license",
                            "contributor": "OpenPipeStress",
                            "contributor_certification": "invented non-engineering value",
                            "redistribution_status": "public_permissive",
                            "review_status": "accepted",
                        },
                    },
                    "provenance": {
                        "source_name": "Invented permissive test source",
                        "source_location": "tests/test_library_import_provenance.py",
                        "source_license": "public test license",
                        "contributor": "OpenPipeStress",
                        "contributor_certification": "invented non-engineering value",
                        "redistribution_status": "public_permissive",
                        "review_status": "accepted",
                    },
                    "review_status": "accepted",
                }
            ],
            "completeness": [],
            "provenance": {
                "source_name": "Invented permissive test source",
                "source_location": "tests/test_library_import_provenance.py",
                "source_license": "public test license",
                "contributor": "OpenPipeStress",
                "contributor_certification": "invented non-engineering value",
                "redistribution_status": "public_permissive",
                "review_status": "accepted",
            },
            "review_status": "accepted",
        }
    ]
    return payload


def codes(result):
    return {finding.code for finding in result.findings}


def test_public_component_import_requires_accepted_public_provenance():
    result = validate_library_import(
        accepted_public_component_payload(),
        library_kind="component",
        intended_visibility="public",
    )
    assert result.accepted is True
    assert result.outcome == "ACCEPTED_PUBLIC"
    assert result.findings == ()


def test_public_material_import_with_tbd_rights_is_rejected_for_review():
    result = validate_library_import(
        load_json(MATERIAL_FIXTURE),
        library_kind="material",
        intended_visibility="public",
    )
    assert result.accepted is False
    assert result.outcome == "REJECTED"
    assert "IMPORT_REDIS_RIGHTS_MISSING" in codes(result)
    assert "IMPORT_REVIEW_REQUIRED" in codes(result)


def test_private_material_import_can_remain_local_with_tbd_rights():
    result = validate_library_import(
        load_json(MATERIAL_FIXTURE),
        library_kind="material",
        intended_visibility="private",
    )
    assert result.accepted is True
    assert result.outcome == "PRIVATE_LOCAL_ONLY"
    assert "IMPORT_REDIS_RIGHTS_MISSING" not in codes(result)


def test_missing_provenance_blocks_import_without_defaults():
    payload = copy.deepcopy(load_json(MATERIAL_FIXTURE))
    del payload["material_records"][0]["provenance"]
    result = validate_library_import(
        payload,
        library_kind="material",
        intended_visibility="public",
    )
    assert result.accepted is False
    assert result.outcome == "REJECTED"
    assert "IMPORT_PROVENANCE_MISSING" in codes(result)


def test_protected_suspected_metadata_quarantines_import():
    payload = copy.deepcopy(load_json(MATERIAL_FIXTURE))
    payload["material_library"]["provenance"]["redistribution_status"] = "protected_suspected"
    result = validate_library_import(
        payload,
        library_kind="material",
        intended_visibility="private",
    )
    assert result.accepted is False
    assert result.outcome == "QUARANTINE"
    assert "IMPORT_PROTECTED_CONTENT_SUSPECTED" in codes(result)


def test_unit_metadata_is_preserved_for_imported_values():
    payload = accepted_public_component_payload()
    del payload["component_records"][0]["fields"][0]["value"]["unit"]
    result = validate_library_import(
        payload,
        library_kind="component",
        intended_visibility="public",
    )
    assert result.accepted is False
    assert result.outcome == "REJECTED"
    assert "IMPORT_UNIT_METADATA_MISSING" in codes(result)


def test_import_findings_map_to_pkg02_diagnostic_envelope():
    payload = accepted_public_component_payload()
    del payload["component_records"][0]["fields"][0]["value"]["unit"]
    result = validate_library_import(
        payload,
        library_kind="component",
        intended_visibility="public",
    )

    diagnostic = next(
        item
        for item in result.diagnostics
        if item["code"] == "IMPORT_UNIT_METADATA_MISSING"
    )
    assert diagnostic["class"] == "import_boundary"
    assert diagnostic["source"] == "core.library_import.provenance_checker"
    assert diagnostic["affected_object"].endswith(".fields[0].value")
    assert diagnostic["remediation"]
    assert diagnostic["provenance"]["source_name"] == "library_import_payload"


if __name__ == "__main__":
    test_public_component_import_requires_accepted_public_provenance()
    test_public_material_import_with_tbd_rights_is_rejected_for_review()
    test_private_material_import_can_remain_local_with_tbd_rights()
    test_missing_provenance_blocks_import_without_defaults()
    test_protected_suspected_metadata_quarantines_import()
    test_unit_metadata_is_preserved_for_imported_values()
    test_import_findings_map_to_pkg02_diagnostic_envelope()


def test_hanger_nested_provenance_quarantine_carries_exact_location():
    payload = load_json(ROOT / 'fixtures/hanger/invented_hanger_library_valid.json')
    payload['hanger_records'][0]['hanger']['stiffness']['value']['provenance']['review_status'] = 'quarantined'
    result = validate_library_import(payload, library_kind='hanger', intended_visibility='private')
    assert result.outcome == 'QUARANTINE'
    assert ('IMPORT_PROTECTED_CONTENT_SUSPECTED', '$.hanger_records[0].hanger.stiffness.value', 'quarantine') in [
        (f.code, f.path, f.severity) for f in result.findings
    ]
