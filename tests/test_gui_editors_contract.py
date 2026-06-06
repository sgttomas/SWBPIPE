#!/usr/bin/env python3
"""Focused tests for DEL-07-03 GUI editor contracts."""

from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.gui.editors import build_editor_contract, canonical_json  # noqa: E402


def main():
    record = build_editor_contract(
        editor_set_id="invented-editor-set",
        editors=[
            {
                "editor_id": "material-editor",
                "editor_kind": "material",
                "target_ref": {"ref_type": "material", "ref_id": "mat-public-1"},
                "library_classification": "invented_public_example",
                "fields": [
                    {
                        "field_id": "density",
                        "value": 7850.0,
                        "unit": "kg/m^3",
                        "dimension": "density",
                        "source_ref": {"ref_type": "invented_source", "ref_id": "src-1"},
                    }
                ],
            },
            {
                "editor_id": "rule-pack-reference",
                "editor_kind": "rule_pack_reference",
                "target_ref": {"ref_type": "rule_pack", "ref_id": "rp-private-ref"},
                "library_classification": "private_reference_only",
                "fields": [{"field_id": "rule_pack_name", "value": "Invented local checks"}],
                "rule_pack_lifecycle": {"checksum": "sha256:invented", "state": "referenced"},
            },
        ],
    )
    assert record["deliverable_id"] == "DEL-07-03"
    assert record["private_payload_policy"] == "references_and_checksums_only_no_private_payload_copy"
    assert record["editors"][0]["save_intent"]["mutates_persistent_project"] is False
    assert record["editors"][0]["fields"][0]["unit_metadata"]["dimension_is_canonical"] is True
    assert record["unit_contract"]["contract_ref"] == "DEL-02-02"
    assert record["editors"][1]["rule_pack_lifecycle"]["checksum"] == "sha256:invented"
    assert not record["diagnostics"]
    assert record["professional_boundary"]["software_makes_approval_claim"] is False
    assert "private payload" not in canonical_json(record).lower()

    missing = build_editor_contract(
        editor_set_id="missing-checksum",
        editors=[
            {
                "editor_id": "rule-pack-reference",
                "editor_kind": "rule_pack_reference",
                "fields": [{"field_id": "name", "value": "TBD"}],
                "rule_pack_lifecycle": {"state": "referenced"},
            }
        ],
    )
    assert any(item["code"] == "RULE_PACK_CHECKSUM_MISSING" for item in missing["diagnostics"])
    assert any(item["code"] == "EDITOR_FIELD_VALUE_UNRESOLVED" for item in missing["diagnostics"])

    missing_unit = build_editor_contract(
        editor_set_id="missing-unit",
        editors=[
            {
                "editor_id": "component-editor",
                "editor_kind": "component",
                "fields": [{"field_id": "diameter", "value": 125.0, "dimension": "length"}],
            }
        ],
    )
    assert any(item["code"] == "EDITOR_UNIT_METADATA_MISSING" for item in missing_unit["diagnostics"])
    assert missing_unit["editors"][0]["validation_state"] == "blocked_by_contract_metadata"


def test_gui_editors_contract_main():
    main()


if __name__ == "__main__":
    main()
