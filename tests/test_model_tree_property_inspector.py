#!/usr/bin/env python3
"""Focused tests for DEL-07-02 model tree/property inspector contracts."""

from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.gui.model_tree import build_model_tree_property_inspector, canonical_json  # noqa: E402


FORBIDDEN = ("certified", "sealed", "code compliant", "professional acceptance")


def invented_entities():
    return [
        {
            "entity_id": "node-N1",
            "entity_type": "node",
            "label": "Invented node N1",
            "sort_key": "001",
            "properties": [
                {
                    "field_id": "elevation",
                    "label": "Elevation",
                    "value": 12.0,
                    "unit": "m",
                    "editable": True,
                    "validation_state": "ready_for_service_validation",
                }
            ],
            "provenance_state": "invented_public_example",
        },
        {
            "entity_id": "pipe-P1",
            "entity_type": "pipe_run",
            "label": "Invented pipe P1",
            "sort_key": "002",
            "properties": [
                {
                    "field_id": "nominal_size",
                    "label": "Nominal size",
                    "value": "TBD",
                    "unit": "mm",
                    "editable": True,
                }
            ],
        },
    ]


def main():
    record = build_model_tree_property_inspector(
        project_id="invented-project",
        entities=invented_entities(),
        selected_ref={"ref_type": "pipe_run", "ref_id": "pipe-P1"},
    )
    again = build_model_tree_property_inspector(
        project_id="invented-project",
        entities=list(reversed(invented_entities())),
        selected_ref={"ref_type": "pipe_run", "ref_id": "pipe-P1"},
    )
    assert canonical_json(record) == canonical_json(again)
    assert record["deliverable_id"] == "DEL-07-02"
    assert record["selection"]["selected"] is True
    assert record["property_inspector"]["status"] == "selection_resolved"
    assert record["property_inspector"]["fields"][0]["validation_state"] == "unresolved_TBD"
    assert record["mutation_policy"] == "application_service_command_intents_only"
    assert record["professional_boundary"]["software_makes_compliance_claim"] is False
    assert any(item["code"] == "PROPERTY_VALUE_UNRESOLVED" for item in record["diagnostics"])
    text = canonical_json(record).lower()
    for term in FORBIDDEN:
        assert term not in text


if __name__ == "__main__":
    main()
