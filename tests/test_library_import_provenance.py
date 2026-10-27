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


# W-17 object form: a disposition member no verdict can read is refused by
# name (MODEL-DISPOSITION-RECORD-MALFORMED, blocking), the desktop export
# screen's rule; the Rust port is pinned to this by provenance_parity.rs.
MALFORMED = "MODEL-DISPOSITION-RECORD-MALFORMED"
HANGER_FIXTURE = ROOT / "fixtures" / "hanger" / "invented_hanger_library_valid.json"


def malformed(result):
    return [(f.path, f.message) for f in result.findings if f.code == MALFORMED]


def test_unreadable_record_dispositions_are_refused_by_name():
    cases = [
        (("material_records", 0, "review_status"), ["quarantined"], "material_records[0].review_status", "review_status", "an array"),
        (("material_records", 0, "review_status"), {"status": "rejected"}, "material_records[0].review_status", "review_status", "an object"),
        (("material_records", 0, "redistribution_status"), ["protected_suspected"], "material_records[0].redistribution_status", "redistribution_status", "an array"),
        (("material_records", 0, "redistribution_status"), {"status": "rejected"}, "material_records[0].redistribution_status", "redistribution_status", "an object"),
        (("material_records", 0, "provenance", "review_status"), ["quarantined"], "material_records[0].provenance.review_status", "provenance.review_status", "an array"),
        (("material_records", 0, "provenance", "redistribution_status"), {"status": "rejected"}, "material_records[0].provenance.redistribution_status", "provenance.redistribution_status", "an object"),
        (("material_records", 0, "properties", 0, "provenance", "review_status"), ["quarantined"], "material_records[0].properties[0].provenance.review_status", "properties[0].provenance.review_status", "an array"),
        (("material_records", 0, "properties", 0, "provenance"), ["Invented", "quarantined"], "material_records[0].properties[0].provenance", "properties[0].provenance", "an array"),
    ]
    for pointer, value, path, member, shape in cases:
        payload = load_json(MATERIAL_FIXTURE)
        target = payload
        for key in pointer[:-1]:
            target = target[key]
        target[pointer[-1]] = value
        for visibility in ("private", "public"):
            result = validate_library_import(payload, library_kind="material", intended_visibility=visibility)
            assert not result.accepted, (path, visibility)
            assert malformed(result) == [(
                path,
                f"Record `mat.invented.alpha` of the material library `matlib.invented.public_fixture` holds `{member}` as {shape}, so its disposition cannot be read.",
            )], (path, visibility, malformed(result))
        assert validate_library_import(payload, library_kind="material", intended_visibility="private").outcome in {"REJECTED", "QUARANTINE"}


def test_unreadable_library_metadata_and_positional_provenance_are_refused():
    payload = load_json(MATERIAL_FIXTURE)
    payload["material_library"]["review_status"] = ["quarantined"]
    result = validate_library_import(payload, library_kind="material", intended_visibility="private")
    assert result.outcome == "REJECTED"
    assert malformed(result) == [(
        "material_library.review_status",
        "The material library `matlib.invented.public_fixture` holds `review_status` as an array, so its disposition cannot be read.",
    )]
    payload = load_json(MATERIAL_FIXTURE)
    payload["material_records"][0]["provenance"] = ["Invented", "private_only", "pending"]
    result = validate_library_import(payload, library_kind="material", intended_visibility="private")
    assert result.outcome == "REJECTED"
    assert [f.code for f in result.findings if f.path.startswith("material_records[0].provenance")] == ["IMPORT_PROVENANCE_MISSING", MALFORMED]


def test_readable_and_unscreened_dispositions_still_import():
    # Controls: the fixture itself, an object-valued classification at the
    # record, library and nested levels, and non-text scalars, which state no
    # disposition the verdict reads, are not refused as malformed.
    payload = load_json(MATERIAL_FIXTURE)
    assert validate_library_import(payload, library_kind="material", intended_visibility="private").outcome == "PRIVATE_LOCAL_ONLY"
    payload["material_records"][0]["classification"] = {"value": "protected_suspected"}
    payload["material_library"]["classification"] = {"redistribution_status": "private_only"}
    payload["material_records"][0]["properties"][0]["classification"] = {"review_status": "accepted"}
    payload["material_records"][0]["privacy_class"] = {"value": "private_user_data"}
    payload["material_records"][0]["review_status"] = None
    result = validate_library_import(payload, library_kind="material", intended_visibility="private")
    assert result.outcome == "PRIVATE_LOCAL_ONLY", [(f.code, f.path) for f in result.findings]
    assert malformed(result) == []


def test_unreadable_hanger_dispositions_are_refused_by_name():
    payload = load_json(HANGER_FIXTURE)
    payload["hanger_records"][0]["provenance"]["review_status"] = ["quarantined"]
    result = validate_library_import(payload, library_kind="hanger", intended_visibility="private")
    assert result.outcome == "REJECTED"
    assert malformed(result) == [(
        "$.hanger_records[0].provenance.review_status",
        "Record `invented.variable` of the hanger library `invented.hangers` holds `provenance.review_status` as an array, so its disposition cannot be read.",
    )]

