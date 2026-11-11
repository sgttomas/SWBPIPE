#!/usr/bin/env python3
"""Parity of the desktop's model-state comparison with the reference engine.

W-12 S3a (DEL-14-01 Design ``desktop-states-runs-comparison.md`` section 4.1):
``fixtures/comparison/model_state_comparison_golden.json`` holds invented state
pairs as the desktop flattened them, with the desktop's comparison of each. It
is written by ``apps/desktop/src/features/comparison/stateComparisonParity.test.ts``.
This test runs ``compare_model_states`` on the same flattened lists through an
adapter and requires equal summaries and per-row classifications on the shared
projection:

- ``family`` becomes ``category`` and the entity key becomes ``stable_id``;
- each entity's fields are projected to its leaf paths, as the desktop's field
  diff reads them, each under a ``field:`` key, so none collides with the ids the
  reference skips (``stable_id``, ``entity_id``, ``id``, ``ref``, ``reference``);
- the paths that hold a quantity on both sides of a mapped pair are declared as
  unit-bearing fields in ``settings``, each quantity carrying one declared
  dimension, so a different unit is the reference's ``INCOMPATIBLE_UNIT``;
- a desktop ``unit_changed`` row maps to the reference's ``unresolved``.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

import pytest


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.comparison.model_state.engine import ModelStateComparisonError, compare_model_states  # noqa: E402


GOLDEN = ROOT / "fixtures" / "comparison" / "model_state_comparison_golden.json"
SCHEMA = ROOT / "schemas" / "model.schema.yaml"
DECLARED_DIMENSION = "declared-by-parity-adapter"


def _golden() -> dict:
    return json.loads(GOLDEN.read_text(encoding="utf-8"))


def _is_scalar(value) -> bool:
    return not isinstance(value, (dict, list))


def _is_quantity(value) -> bool:
    if not isinstance(value, dict) or not isinstance(value.get("unit"), str):
        return False
    others = [item for key, item in value.items() if key != "unit"]
    return bool(others) and all(isinstance(item, (int, float)) and not isinstance(item, bool) for item in others)


def _is_leaf(value) -> bool:
    if _is_scalar(value) or _is_quantity(value):
        return True
    if isinstance(value, list):
        return all(_is_scalar(item) for item in value)
    return len(value) == 0


def _leaves(value, path, out) -> None:
    if path and _is_leaf(value):
        out["field:" + json.dumps(path)] = value
        return
    items = enumerate(value) if isinstance(value, list) else value.items()
    for key, item in items:
        _leaves(item, [*path, key], out)


def _projected(entity: dict) -> dict:
    leaves: dict = {}
    _leaves(entity["fields"], [], leaves)
    return leaves


def _stable_id(entity: dict) -> str:
    return json.dumps([entity["family"], *entity["key"]])


def _reference_entity(entity: dict, unit_fields: set[str]) -> dict:
    projected = {}
    for key, value in _projected(entity).items():
        if key in unit_fields and _is_quantity(value):
            value = {**value, "dimension": DECLARED_DIMENSION}
        projected[key] = value
    return {"stable_id": _stable_id(entity), "category": entity["family"], **projected}


def _unit_bearing_fields(left: list[dict], right: list[dict]) -> set[str]:
    """Paths that hold a quantity on both sides of some mapped pair and never on one side only."""

    right_by_id = {_stable_id(entity): entity for entity in right}
    both: set[str] = set()
    one_sided: set[str] = set()
    for entity in left:
        counterpart = right_by_id.get(_stable_id(entity))
        if counterpart is None:
            continue
        left_leaves, right_leaves = _projected(entity), _projected(counterpart)
        for key in set(left_leaves) | set(right_leaves):
            left_quantity = _is_quantity(left_leaves.get(key))
            right_quantity = _is_quantity(right_leaves.get(key))
            if left_quantity and right_quantity:
                both.add(key)
            elif left_quantity or right_quantity:
                one_sided.add(key)
    return both - one_sided


def _state(state_id: str, entities: list[dict]) -> dict:
    return {"model_state": {"state_id": state_id}, "entities": entities}


def _row_ref(row: dict) -> str:
    side = row.get("left_ref") or row.get("right_ref")
    return side["ref"]


def _expected_classification(row: dict) -> str:
    if row["classification"] == "changed" and any(change["kind"] == "unit_changed" for change in row["changes"]):
        return "unresolved"
    return row["classification"]


def test_the_golden_assigns_every_member_of_the_schema():
    schema = json.loads(SCHEMA.read_text(encoding="utf-8"))
    assert schema["additionalProperties"] is False
    assert sorted(_golden()["member_assignment"]) == sorted(schema["properties"])


PAIRS = _golden()["pairs"]


@pytest.mark.parametrize("pair", PAIRS, ids=[pair["name"] for pair in PAIRS])
def test_the_reference_engine_agrees_on_the_shared_projection(pair):
    left_entities, right_entities = pair["left_entities"], pair["right_entities"]
    if "refusal" in pair["desktop"]:
        assert pair["desktop"]["refusal"] == "COMPARISON-DUPLICATE-ID"
        with pytest.raises(ModelStateComparisonError, match="stable identifier is duplicated"):
            compare_model_states(
                _state("state:left", [_reference_entity(entity, set()) for entity in left_entities]),
                _state("state:right", [_reference_entity(entity, set()) for entity in right_entities]),
            )
        return

    unit_fields = _unit_bearing_fields(left_entities, right_entities)
    desktop_rows = {_stable_id(row): row for row in pair["desktop"]["rows"]}
    # Every unit change the desktop reports lies on a declared unit-bearing path,
    # so the pair is within the shared projection.
    for row in desktop_rows.values():
        for change in row["changes"]:
            if change["kind"] == "unit_changed":
                assert "field:" + json.dumps(change["path"]) in unit_fields, (row["id"], change["path"])

    result = compare_model_states(
        _state("state:left", [_reference_entity(entity, unit_fields) for entity in left_entities]),
        _state("state:right", [_reference_entity(entity, unit_fields) for entity in right_entities]),
        settings={"unit_bearing_fields": sorted(unit_fields), "created_at": "invented"},
    )

    # Per-row classifications.
    reference_rows = {_row_ref(row): row for row in result["entities"]}
    assert set(reference_rows) == set(desktop_rows)
    assert {key: row["classification"] for key, row in reference_rows.items()} == {
        key: _expected_classification(row) for key, row in desktop_rows.items()
    }

    # Summaries (the reference has no manual mappings here).
    desktop_summary = pair["desktop"]["summary"]
    unresolved = sum(1 for row in desktop_rows.values() if _expected_classification(row) == "unresolved")
    assert result["summary"] == {
        "added": desktop_summary["added"],
        "removed": desktop_summary["removed"],
        "changed": desktop_summary["changed"] - unresolved,
        "unchanged": desktop_summary["unchanged"],
        "mapped_changed": 0,
        "mapped_unchanged": 0,
        "unresolved": unresolved,
        "total": desktop_summary["total"],
    }

    # Changed rows name the same field paths; unresolved rows carry INCOMPATIBLE_UNIT.
    incompatible = {
        affected["affected_ref"]["ref"]
        for diagnostic in result["diagnostics"]
        if diagnostic["code"] == "INCOMPATIBLE_UNIT"
        for affected in diagnostic["affected_refs"]
    }
    for key, row in reference_rows.items():
        desktop_paths = {"field:" + json.dumps(change["path"]) for change in desktop_rows[key]["changes"]}
        if row["classification"] == "changed":
            assert {change["field"] for change in row["changes"]} == desktop_paths
        if row["classification"] == "unresolved":
            assert key in incompatible
    assert not any(diagnostic["code"] in {"MISSING_UNIT_METADATA", "INCOMPATIBLE_DIMENSION"} for diagnostic in result["diagnostics"])

    # The invariant, on the reference's side too: equal model hashes exactly when
    # every row is unchanged.
    assert pair["hashes_equal"] == (result["summary"]["unchanged"] == result["summary"]["total"])


def test_the_golden_covers_each_classification_and_a_unit_change():
    classifications = {
        _expected_classification(row)
        for pair in PAIRS
        if "rows" in pair["desktop"]
        for row in pair["desktop"]["rows"]
    }
    assert classifications == {"added", "removed", "changed", "unchanged", "unresolved"}
