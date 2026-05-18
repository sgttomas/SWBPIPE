#!/usr/bin/env python3
"""Focused checks for DEL-14-03 model-state comparison."""

from __future__ import annotations

import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.comparison.model_state.engine import canonical_json, compare_model_states  # noqa: E402


def provenance():
    return {
        "source_name": "invented public test fixture",
        "source_location": "tests/test_model_state_comparison.py",
        "source_license": "project-governed",
        "contributor": "OpenPipeStress test",
        "contributor_certification": "invented non-engineering fixture",
        "redistribution_status": "invented_non_engineering_example",
        "review_status": "pending",
        "privacy_classification": "invented_public_example",
    }


def professional_boundary():
    return {
        "human_review_required": True,
        "software_makes_compliance_claim": False,
        "software_makes_certification_claim": False,
        "software_makes_sealing_claim": False,
        "software_makes_approval_claim": False,
        "software_makes_authentication_claim": False,
    }


def state(state_id, entities):
    return {
        "model_state": {
            "state_id": state_id,
            "state_name": f"Invented {state_id}",
            "state_kind": "comparison_basis",
            "created_at": "2026-05-05T00:00:00Z",
            "model_ref": {"object_type": "Model", "ref": "model:invented"},
            "parent_state_refs": [],
            "tags": [{"tag": "invented", "tag_kind": "comparison_label", "provenance": provenance()}],
            "notes": [
                {
                    "note_id": f"note:{state_id}",
                    "note_type": "design_note",
                    "statement": "Invented public comparison fixture.",
                    "visibility": "public",
                    "provenance": provenance(),
                }
            ],
            "external_references": [
                {
                    "reference_id": f"ext:{state_id}",
                    "reference_type": "document",
                    "label": "Invented public source",
                    "target": "public-fixture",
                    "binding_hashes": [],
                    "privacy_classification": "invented_public_example",
                    "provenance": provenance(),
                }
            ],
            "unresolved_assumptions": [
                {
                    "assumption_id": f"assumption:{state_id}",
                    "statement": "Invented unresolved assumption.",
                    "status": "unresolved",
                    "affected_refs": [{"object_type": "ModelState", "ref": state_id}],
                    "provenance": provenance(),
                }
            ],
            "warnings": [],
            "analysis_status": ["MODEL_INCOMPLETE"],
            "hashes": [
                {
                    "algorithm": "sha256",
                    "canonicalization": "JCS",
                    "payload_ref": {"object_type": "ModelState", "ref": state_id},
                    "payload_scope": "model_state_record",
                    "value": f"hash-{state_id}",
                }
            ],
            "immutability_policy": {
                "snapshot_is_read_only": True,
                "mutation_policy": "changes_create_new_model_state",
                "new_state_required_for_change": True,
                "hash_invalidates_external_acceptance": True,
            },
            "professional_boundary": professional_boundary(),
            "provenance": provenance(),
        },
        "entities": entities,
    }


def entity(stable_id, *, category="Component", label=None, **fields):
    return {
        "stable_id": stable_id,
        "category": category,
        "reference": {"object_type": "Entity", "ref": stable_id, "label": label or stable_id},
        **fields,
    }


def by_ref(result):
    rows = {}
    for row in result["entities"]:
        left_ref = row["left_ref"]["ref"] if row.get("left_ref") else None
        right_ref = row["right_ref"]["ref"] if row.get("right_ref") else None
        rows[(left_ref, right_ref)] = row
    return rows


def test_stable_id_matching_is_order_independent_and_preserves_metadata():
    left = state(
        "state:left",
        [
            entity("entity:changed", nominal_size={"value": 12, "unit": "in", "dimension": "length"}),
            entity("entity:unchanged", nominal_size={"value": 10, "unit": "in", "dimension": "length"}),
        ],
    )
    right_a = state(
        "state:right",
        [
            entity("entity:unchanged", nominal_size={"value": 10, "unit": "in", "dimension": "length"}),
            entity("entity:changed", nominal_size={"value": 14, "unit": "in", "dimension": "length"}),
        ],
    )
    right_b = state("state:right", list(reversed(right_a["entities"])))

    settings = {"unit_bearing_fields": ["nominal_size"], "created_at": "TBD"}
    first = compare_model_states(left, right_a, settings=settings)
    second = compare_model_states(left, right_b, settings=settings)

    assert canonical_json(first) == canonical_json(second)
    assert first["summary"]["changed"] == 1
    assert first["summary"]["unchanged"] == 1
    assert first["metadata"]["left"]["notes"][0]["note_id"] == "note:state:left"
    assert first["metadata"]["right"]["external_references"][0]["reference_id"] == "ext:state:right"
    assert any(item["code"] == "UNRESOLVED_STATE_ASSUMPTION" for item in first["diagnostics"])


def test_added_removed_changed_unchanged_and_explicit_mapping_are_classified():
    left = state(
        "state:left",
        [
            entity("entity:unchanged", value="same"),
            entity("entity:changed", value="before"),
            entity("entity:removed", value="left-only"),
            entity("entity:left-renamed", value="mapped"),
        ],
    )
    right = state(
        "state:right",
        [
            entity("entity:added", value="right-only"),
            entity("entity:changed", value="after"),
            entity("entity:right-renamed", value="mapped"),
            entity("entity:unchanged", value="same"),
        ],
    )
    mappings = [
        {
            "mapping_id": "map:renamed",
            "mapping_kind": "entity",
            "mapping_status": "manual_match",
            "left_ref": {"object_type": "Entity", "ref": "entity:left-renamed"},
            "right_ref": {"object_type": "Entity", "ref": "entity:right-renamed"},
        }
    ]

    result = compare_model_states(left, right, mappings=mappings)
    assert result["summary"] == {
        "added": 1,
        "removed": 1,
        "changed": 1,
        "unchanged": 1,
        "mapped_changed": 0,
        "mapped_unchanged": 1,
        "unresolved": 0,
        "total": 5,
    }
    rows = by_ref(result)
    assert rows[("entity:left-renamed", "entity:right-renamed")]["match_basis"] == "explicit_mapping"
    assert rows[("entity:changed", "entity:changed")]["changes"][0]["field"] == "value"


def test_unresolved_mapping_and_unsupported_category_emit_diagnostics():
    left = state("state:left", [entity("entity:left-only", category="Support")])
    right = state("state:right", [entity("entity:right-only", category="Load")])
    mappings = [
        {
            "mapping_id": "map:unresolved",
            "mapping_kind": "entity",
            "mapping_status": "unresolved_mapping",
            "left_ref": {"object_type": "Entity", "ref": "entity:left-only"},
        }
    ]

    result = compare_model_states(
        left,
        right,
        mappings=mappings,
        settings={"comparable_entity_categories": ["Component"]},
    )

    assert result["summary"]["unresolved"] == 1
    assert result["summary"]["added"] == 1
    assert any(item["code"] == "MAPPING_UNRESOLVED" for item in result["diagnostics"])


def test_unit_bearing_changes_require_unit_and_dimension_metadata():
    left = state("state:left", [entity("entity:pipe", nominal_size=10)])
    right = state("state:right", [entity("entity:pipe", nominal_size=12)])

    result = compare_model_states(left, right, settings={"unit_bearing_fields": ["nominal_size"]})

    assert result["summary"]["unresolved"] == 1
    assert result["summary"]["changed"] == 0
    diagnostic = next(item for item in result["diagnostics"] if item["code"] == "MISSING_UNIT_METADATA")
    assert diagnostic["severity"] == "blocking"


def test_output_boundary_language_does_not_make_prohibited_claims():
    result = compare_model_states(
        state("state:left", [entity("entity:one")]),
        state("state:right", [entity("entity:one")]),
    )
    text = canonical_json(result).lower()

    for forbidden in [
        "code compliant",
        "certified",
        "sealed",
        "authenticated",
        "professional approval",
        "external validation",
    ]:
        assert forbidden not in text
    assert result["professional_boundary"]["software_makes_compliance_claim"] is False


def main():
    test_stable_id_matching_is_order_independent_and_preserves_metadata()
    test_added_removed_changed_unchanged_and_explicit_mapping_are_classified()
    test_unresolved_mapping_and_unsupported_category_emit_diagnostics()
    test_unit_bearing_changes_require_unit_and_dimension_metadata()
    test_output_boundary_language_does_not_make_prohibited_claims()


if __name__ == "__main__":
    main()
