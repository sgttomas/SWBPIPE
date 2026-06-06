#!/usr/bin/env python3
"""Focused tests for DEL-07-04 warning/blocking UX contracts."""

from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.gui.warnings import build_warning_ux_contract, canonical_json  # noqa: E402


def main():
    record = build_warning_ux_contract(
        warning_set_id="invented-warning-set",
        conditions=[
            {
                "warning_id": "warn-missing-support",
                "warning_class": "incomplete_data",
                "target_ref": {"ref_type": "support", "ref_id": "support-TBD"},
                "message": "Invented support has unresolved stiffness.",
                "source_status": "missing",
            },
            {
                "warning_id": "warn-assumption",
                "warning_class": "assumption",
                "target_ref": {"ref_type": "load_case", "ref_id": "load-1"},
                "source_status": "provided",
                "assumption_ref": {"ref_type": "assumption", "ref_id": "assumption-1"},
            },
        ],
    )
    assert record["deliverable_id"] == "DEL-07-04"
    assert record["auto_fill_missing_data"] is False
    assert record["blocking_summary"]["has_blocking_items"] is True
    assert record["blocking_summary"]["blocking_warning_ids"] == ["warn-missing-support"]
    assert record["analysis_boundary_contract"]["contract_ref"] == "DEL-02-03"
    assert "USER_RULE_FAILED" in record["analysis_boundary_contract"]["local_warning_class_status_map"]["diagnostic"]
    assert record["warnings"][0]["analysis_boundary_mapping"]["human_acceptance_record_policy"] == (
        "external_hash_bound_human_record_only"
    )
    assert record["warnings"][0]["professional_boundary_preserved"] is True
    assert any(item["code"] == "WARNING_SOURCE_STATUS_UNRESOLVED" for item in record["diagnostics"])
    assert "professional acceptance" not in canonical_json(record).lower()


def test_missing_data_warning_ux_main():
    main()


if __name__ == "__main__":
    main()
