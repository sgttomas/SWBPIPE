#!/usr/bin/env python3
"""Focused tests for DEL-07-05 results viewer contracts."""

from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.gui.results_viewer import build_results_viewer_contract, canonical_json  # noqa: E402


def main():
    record = build_results_viewer_contract(
        result_set_id="invented-results",
        result_items=[
            {
                "result_id": "disp-node-1",
                "result_kind": "displacement",
                "analysis_status": ["MECHANICS_SOLVED", "HUMAN_REVIEW_REQUIRED"],
                "hashes": [
                    {
                        "algorithm": "sha256",
                        "canonicalization": "JCS",
                        "payload_ref": {"ref_type": "ResultEnvelope", "ref_id": "results:invented"},
                        "value": "sha256:invented-result",
                    }
                ],
                "provenance_refs": [{"ref_type": "AnalysisRun", "ref_id": "run:invented"}],
                "values": [
                    {
                        "value_id": "ux",
                        "component": "UX",
                        "numeric_value": 1.2,
                        "unit": "mm",
                        "dimension": "displacement",
                        "location_ref": {"ref_type": "node", "ref_id": "N1"},
                    }
                ],
                "overlay_enabled": True,
                "overlay_target_ref": {"ref_type": "node", "ref_id": "N1"},
            },
            {
                "result_id": "stress-tbd",
                "result_kind": "stress",
                "values": [{"value_id": "smax", "numeric_value": "TBD", "unit": "MPa", "dimension": "stress"}],
            },
        ],
    )
    assert record["deliverable_id"] == "DEL-07-05"
    assert record["solver_execution"] == "not_performed"
    assert record["views"][0]["availability"] == "available"
    assert record["views"][1]["availability"] == "unresolved_TBD"
    assert record["views"][0]["analysis_status"] == ["MECHANICS_SOLVED", "HUMAN_REVIEW_REQUIRED"]
    assert record["views"][0]["hash_boundary"]["hashes_present"] is True
    assert record["unit_contract"]["contract_ref"] == "DEL-02-02"
    assert record["views"][0]["table_columns"] == [
        "component",
        "dimension",
        "location_ref",
        "numeric_value",
        "unit",
        "value_id",
    ]
    assert any(item["code"] == "RESULT_VALUE_OR_UNIT_UNRESOLVED" for item in record["diagnostics"])
    assert record["software_makes_professional_acceptance_claim"] is False
    assert "code compliant" not in canonical_json(record).lower()


if __name__ == "__main__":
    main()
