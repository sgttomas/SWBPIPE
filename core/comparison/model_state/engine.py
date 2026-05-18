"""Deterministic model-state comparison for DEL-14-03.

The engine is intentionally provider-neutral: callers supply immutable model
state records or references plus explicit entity payloads. Analysis-run result
deltas are out of scope for this module.
"""

from __future__ import annotations

from copy import deepcopy
from datetime import UTC, datetime
import json
from typing import Any


METADATA_FIELDS = (
    "state_id",
    "state_name",
    "state_kind",
    "created_at",
    "model_ref",
    "parent_state_refs",
    "tags",
    "notes",
    "external_references",
    "unresolved_assumptions",
    "warnings",
    "analysis_status",
    "hashes",
    "immutability_policy",
    "professional_boundary",
    "provenance",
)

PROFESSIONAL_BOUNDARY = {
    "human_review_required": True,
    "software_makes_compliance_claim": False,
    "software_makes_certification_claim": False,
    "software_makes_sealing_claim": False,
    "software_makes_approval_claim": False,
    "software_makes_authentication_claim": False,
    "notice": (
        "Model-state comparison is deterministic decision-support output and "
        "requires external human review."
    ),
}

ENGINE_PROVENANCE = {
    "source_name": "OpenPipeStress DEL-14-03 model-state comparison engine",
    "source_location": "core/comparison/model_state/engine.py",
    "source_license": "project-governed",
    "contributor": "OpenPipeStress Type 2 worker",
    "contributor_certification": "implementation-only-no-professional-claim",
    "redistribution_status": "public_permissive",
    "review_status": "pending",
    "privacy_classification": "public_metadata",
}


class ModelStateComparisonError(ValueError):
    """Raised when comparison input cannot be interpreted deterministically."""


def compare_model_states(
    left_state: dict[str, Any],
    right_state: dict[str, Any],
    *,
    mappings: list[dict[str, Any]] | None = None,
    settings: dict[str, Any] | None = None,
) -> dict[str, Any]:
    """Compare two model states and return a deterministic diff envelope.

    Expected inputs may be either a raw DEL-14-01 ``model_state`` object or a
    wrapper containing ``model_state`` plus an ``entities`` list. Each entity
    must expose a stable identifier via ``stable_id``, ``entity_id``, ``id``,
    ``ref``, or ``reference.ref``.
    """

    settings = settings or {}
    mappings = mappings or []
    left = _state_view(left_state)
    right = _state_view(right_state)

    left_entities = _entity_index(left["entities"], side="left")
    right_entities = _entity_index(right["entities"], side="right")

    diagnostics: list[dict[str, Any]] = []
    diagnostics.extend(_state_metadata_diagnostics(left["record"], "left"))
    diagnostics.extend(_state_metadata_diagnostics(right["record"], "right"))

    mapping_pairs, mapped_left_ids, mapped_right_ids, unresolved_rows = _mapping_pairs(
        mappings, left_entities, right_entities, diagnostics
    )

    direct_ids = sorted(set(left_entities) & set(right_entities))
    rows: list[dict[str, Any]] = []
    rows.extend(unresolved_rows)
    processed_left: set[str] = set()
    processed_right: set[str] = set()

    for stable_id in direct_ids:
        rows.append(
            _compare_pair(
                left_entities[stable_id],
                right_entities[stable_id],
                match_basis="stable_id",
                mapping_id=None,
                settings=settings,
                diagnostics=diagnostics,
            )
        )
        processed_left.add(stable_id)
        processed_right.add(stable_id)

    for mapping_id, left_id, right_id in mapping_pairs:
        if left_id in processed_left or right_id in processed_right:
            diagnostics.append(
                _diagnostic(
                    "DUPLICATE_MAPPING_BASIS",
                    "MAPPING_UNRESOLVED",
                    "warning",
                    "A mapping overlaps an already matched stable identifier.",
                    "Review mapping records and retain one deterministic counterpart.",
                    [
                        _affected("left_entity", _entity_ref(left_entities.get(left_id), left_id)),
                        _affected("right_entity", _entity_ref(right_entities.get(right_id), right_id)),
                    ],
                )
            )
            continue
        rows.append(
            _compare_pair(
                left_entities[left_id],
                right_entities[right_id],
                match_basis="explicit_mapping",
                mapping_id=mapping_id,
                settings=settings,
                diagnostics=diagnostics,
            )
        )
        processed_left.add(left_id)
        processed_right.add(right_id)

    for stable_id in sorted(set(left_entities) - processed_left - mapped_left_ids):
        rows.append(_single_side("removed", "left", left_entities[stable_id]))
        diagnostics.append(
            _diagnostic(
                "UNMATCHED_LEFT_ENTITY",
                "UNMATCHED_ENTITY",
                "warning",
                "A left-side entity has no stable-ID or explicit mapped counterpart.",
                "Add a mapping record or confirm the entity was intentionally removed.",
                [_affected("left_entity", _entity_ref(left_entities[stable_id], stable_id))],
            )
        )

    for stable_id in sorted(set(right_entities) - processed_right - mapped_right_ids):
        rows.append(_single_side("added", "right", right_entities[stable_id]))
        diagnostics.append(
            _diagnostic(
                "UNMATCHED_RIGHT_ENTITY",
                "UNMATCHED_ENTITY",
                "warning",
                "A right-side entity has no stable-ID or explicit mapped counterpart.",
                "Add a mapping record or confirm the entity was intentionally added.",
                [_affected("right_entity", _entity_ref(right_entities[stable_id], stable_id))],
            )
        )

    rows.sort(key=_row_sort_key)
    diagnostics.sort(key=lambda item: _canonical(item))

    return {
        "comparison_kind": "model_state_to_model_state",
        "engine": "DEL-14-03_model_state_comparison_v1",
        "created_at": settings.get("created_at", "TBD"),
        "left_state_ref": _state_ref(left["record"], "left"),
        "right_state_ref": _state_ref(right["record"], "right"),
        "summary": _summary(rows),
        "entities": rows,
        "metadata": {
            "left": _metadata(left["record"]),
            "right": _metadata(right["record"]),
        },
        "diagnostics": diagnostics,
        "professional_boundary": deepcopy(PROFESSIONAL_BOUNDARY),
        "provenance": deepcopy(ENGINE_PROVENANCE),
    }


def canonical_json(value: Any) -> str:
    """Serialize comparison output with stable ordering for repeat checks."""

    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def _state_view(state: dict[str, Any]) -> dict[str, Any]:
    if not isinstance(state, dict):
        raise ModelStateComparisonError("model state input must be a dictionary")

    record = state.get("model_state", state)
    if not isinstance(record, dict):
        raise ModelStateComparisonError("model_state must be a dictionary")

    entities = state.get("entities", record.get("entities", []))
    if not isinstance(entities, list):
        raise ModelStateComparisonError("entities must be a list")

    return {"record": record, "entities": entities}


def _entity_index(entities: list[dict[str, Any]], *, side: str) -> dict[str, dict[str, Any]]:
    indexed: dict[str, dict[str, Any]] = {}
    for position, entity in enumerate(entities):
        if not isinstance(entity, dict):
            raise ModelStateComparisonError(f"{side} entity at position {position} is not an object")
        stable_id = _stable_id(entity)
        if not stable_id:
            raise ModelStateComparisonError(
                f"{side} entity at position {position} has no stable identifier"
            )
        if stable_id in indexed:
            raise ModelStateComparisonError(f"{side} entity stable identifier is duplicated: {stable_id}")
        indexed[stable_id] = deepcopy(entity)
    return indexed


def _mapping_pairs(
    mappings: list[dict[str, Any]],
    left_entities: dict[str, dict[str, Any]],
    right_entities: dict[str, dict[str, Any]],
    diagnostics: list[dict[str, Any]],
) -> tuple[list[tuple[str, str, str]], set[str], set[str], list[dict[str, Any]]]:
    pairs: list[tuple[str, str, str]] = []
    mapped_left: set[str] = set()
    mapped_right: set[str] = set()
    unresolved_rows: list[dict[str, Any]] = []

    for index, mapping in enumerate(mappings):
        if mapping.get("mapping_kind", "entity") != "entity":
            diagnostics.append(
                _diagnostic(
                    "UNSUPPORTED_MAPPING_CATEGORY",
                    "MAPPING_UNRESOLVED",
                    "warning",
                    "Only entity mapping records are supported by DEL-14-03.",
                    "Route analysis-run or result mapping behavior to the owning deliverable.",
                    [_affected("mapping_record", _mapping_ref(mapping, index))],
                )
            )
            continue

        status = mapping.get("mapping_status")
        left_id = _ref_id(mapping.get("left_ref"))
        right_id = _ref_id(mapping.get("right_ref"))
        mapping_id = str(mapping.get("mapping_id", f"mapping:{index:04d}"))

        if status == "unresolved_mapping" or not left_id or not right_id:
            diagnostics.append(
                _diagnostic(
                    "MAPPING_UNRESOLVED",
                    "MAPPING_UNRESOLVED",
                    "warning",
                    "A mapping record does not identify both entity counterparts.",
                    "Resolve the mapping before treating the entities as comparable.",
                    [_affected("mapping_record", _mapping_ref(mapping, index))],
                )
            )
            if left_id:
                mapped_left.add(left_id)
            if right_id:
                mapped_right.add(right_id)
            unresolved_rows.append(_unresolved_mapping_row(mapping_id, left_entities.get(left_id), right_entities.get(right_id), left_id, right_id))
            continue

        if left_id not in left_entities or right_id not in right_entities:
            diagnostics.append(
                _diagnostic(
                    "MAPPING_TARGET_MISSING",
                    "MAPPING_UNRESOLVED",
                    "warning",
                    "A mapping record references an entity absent from one comparison side.",
                    "Correct the mapping record or comparison participants.",
                    [
                        _affected("left_entity", _entity_ref(left_entities.get(left_id), left_id)),
                        _affected("right_entity", _entity_ref(right_entities.get(right_id), right_id)),
                    ],
                )
            )
            mapped_left.add(left_id)
            mapped_right.add(right_id)
            unresolved_rows.append(_unresolved_mapping_row(mapping_id, left_entities.get(left_id), right_entities.get(right_id), left_id, right_id))
            continue

        if status not in {"manual_match", "automatic_match"}:
            diagnostics.append(
                _diagnostic(
                    "UNSUPPORTED_MAPPING_STATUS",
                    "MAPPING_UNRESOLVED",
                    "warning",
                    "A mapping record has a status that is not comparable by this engine.",
                    "Use manual_match or automatic_match for explicit entity counterparts.",
                    [_affected("mapping_record", _mapping_ref(mapping, index))],
                )
            )
            mapped_left.add(left_id)
            mapped_right.add(right_id)
            unresolved_rows.append(_unresolved_mapping_row(mapping_id, left_entities.get(left_id), right_entities.get(right_id), left_id, right_id))
            continue

        pairs.append((mapping_id, left_id, right_id))
        mapped_left.add(left_id)
        mapped_right.add(right_id)

    pairs.sort()
    unresolved_rows.sort(key=_row_sort_key)
    return pairs, mapped_left, mapped_right, unresolved_rows


def _compare_pair(
    left: dict[str, Any],
    right: dict[str, Any],
    *,
    match_basis: str,
    mapping_id: str | None,
    settings: dict[str, Any],
    diagnostics: list[dict[str, Any]],
) -> dict[str, Any]:
    left_id = _stable_id(left)
    right_id = _stable_id(right)
    category_issue = _category_diagnostic(left, right, settings)
    unit_issue = _unit_diagnostics(left, right, settings)

    if category_issue:
        diagnostics.append(category_issue)
    diagnostics.extend(unit_issue)

    if category_issue or any(item["severity"] == "blocking" for item in unit_issue):
        classification = "unresolved"
        changes: list[dict[str, Any]] = []
    else:
        changes = _changes(left, right)
        classification = "unchanged" if not changes else "changed"
        if match_basis == "explicit_mapping":
            classification = "mapped_unchanged" if not changes else "mapped_changed"

    return {
        "row_id": _row_id(left_id, right_id, mapping_id),
        "classification": classification,
        "match_basis": match_basis,
        "mapping_id": mapping_id,
        "left_ref": _entity_ref(left, left_id),
        "right_ref": _entity_ref(right, right_id),
        "changes": changes,
    }


def _changes(left: dict[str, Any], right: dict[str, Any]) -> list[dict[str, Any]]:
    keys = sorted(set(left) | set(right))
    changes = []
    for key in keys:
        if key in {"stable_id", "entity_id", "id", "ref", "reference"}:
            continue
        left_value = left.get(key)
        right_value = right.get(key)
        if _canonical(left_value) != _canonical(right_value):
            changes.append(
                {
                    "field": key,
                    "left": deepcopy(left_value),
                    "right": deepcopy(right_value),
                }
            )
    return changes


def _unit_diagnostics(
    left: dict[str, Any], right: dict[str, Any], settings: dict[str, Any]
) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    unit_fields = set(settings.get("unit_bearing_fields", []))

    for field in sorted(unit_fields):
        if field not in left and field not in right:
            continue
        left_value = left.get(field)
        right_value = right.get(field)
        if _canonical(left_value) == _canonical(right_value):
            continue
        if not (_has_unit_metadata(left_value) and _has_unit_metadata(right_value)):
            diagnostics.append(
                _diagnostic(
                    "MISSING_UNIT_METADATA",
                    "UNIT_WARNING",
                    "blocking",
                    "A changed unit-bearing field is missing explicit unit or dimension metadata.",
                    "Provide unit and dimension metadata or a governed normalization contract.",
                    [
                        _affected("left_entity", _entity_ref(left, _stable_id(left))),
                        _affected("right_entity", _entity_ref(right, _stable_id(right))),
                    ],
                )
            )
            continue
        if left_value.get("dimension") != right_value.get("dimension"):
            diagnostics.append(
                _diagnostic(
                    "INCOMPATIBLE_DIMENSION",
                    "UNIT_WARNING",
                    "blocking",
                    "A changed unit-bearing field has incompatible dimensions.",
                    "Compare only dimensionally compatible values.",
                    [
                        _affected("left_entity", _entity_ref(left, _stable_id(left))),
                        _affected("right_entity", _entity_ref(right, _stable_id(right))),
                    ],
                )
            )
        elif left_value.get("unit") != right_value.get("unit"):
            diagnostics.append(
                _diagnostic(
                    "INCOMPATIBLE_UNIT",
                    "UNIT_WARNING",
                    "blocking",
                    "A changed unit-bearing field has different units and no normalization contract.",
                    "Normalize units through a governed unit contract before numeric comparison.",
                    [
                        _affected("left_entity", _entity_ref(left, _stable_id(left))),
                        _affected("right_entity", _entity_ref(right, _stable_id(right))),
                    ],
                )
            )

    return diagnostics


def _category_diagnostic(
    left: dict[str, Any], right: dict[str, Any], settings: dict[str, Any]
) -> dict[str, Any] | None:
    allowed = settings.get("comparable_entity_categories")
    if not allowed:
        return None
    left_category = _category(left)
    right_category = _category(right)
    if left_category in allowed and right_category in allowed and left_category == right_category:
        return None
    return _diagnostic(
        "UNSUPPORTED_CATEGORY",
        "MAPPING_UNRESOLVED",
        "warning",
        "Compared entities have unsupported or incompatible categories.",
        "Provide a supported category pair or mark the comparison as not comparable.",
        [
            _affected("left_entity", _entity_ref(left, _stable_id(left))),
            _affected("right_entity", _entity_ref(right, _stable_id(right))),
        ],
    )


def _state_metadata_diagnostics(record: dict[str, Any], side: str) -> list[dict[str, Any]]:
    diagnostics: list[dict[str, Any]] = []
    state_ref = _state_ref(record, side)

    for assumption in record.get("unresolved_assumptions", []):
        diagnostics.append(
            _diagnostic(
                "UNRESOLVED_STATE_ASSUMPTION",
                "ASSUMPTION_WARNING",
                "warning",
                "A compared model state carries an unresolved assumption.",
                "Resolve or review the assumption before relying on comparison output.",
                [_affected("assumption", _reference("Assumption", assumption.get("assumption_id", "TBD")))],
                provenance=assumption.get("provenance"),
            )
        )

    for warning in record.get("warnings", []):
        diagnostics.append(
            _diagnostic(
                "STATE_WARNING_PRESERVED",
                "PROVENANCE_WARNING",
                "info",
                "A compared model state carries a warning diagnostic.",
                "Review the source state warning together with comparison output.",
                [_affected("diagnostic", _reference("Diagnostic", warning.get("code", state_ref["ref"])))],
                provenance=warning.get("provenance"),
            )
        )

    diagnostics.append(
        _diagnostic(
            "PROFESSIONAL_BOUNDARY_NOTICE",
            "PROVENANCE_WARNING",
            "info",
            "Comparison output is bounded decision-support data and requires external human review.",
            "Use an external review workflow for project decisions.",
            [_affected(f"{side}_entity", state_ref)],
        )
    )

    return diagnostics


def _single_side(classification: str, side: str, entity: dict[str, Any]) -> dict[str, Any]:
    stable_id = _stable_id(entity)
    ref_key = "left_ref" if side == "left" else "right_ref"
    missing_key = "right_ref" if side == "left" else "left_ref"
    return {
        "row_id": _row_id(stable_id if side == "left" else None, stable_id if side == "right" else None, None),
        "classification": classification,
        "match_basis": "unmatched",
        "mapping_id": None,
        ref_key: _entity_ref(entity, stable_id),
        missing_key: None,
        "changes": [],
    }


def _unresolved_mapping_row(
    mapping_id: str,
    left: dict[str, Any] | None,
    right: dict[str, Any] | None,
    left_id: str | None,
    right_id: str | None,
) -> dict[str, Any]:
    return {
        "row_id": _row_id(left_id, right_id, mapping_id),
        "classification": "unresolved",
        "match_basis": "explicit_mapping",
        "mapping_id": mapping_id,
        "left_ref": _entity_ref(left, left_id) if left_id else None,
        "right_ref": _entity_ref(right, right_id) if right_id else None,
        "changes": [],
    }


def _summary(rows: list[dict[str, Any]]) -> dict[str, int]:
    keys = [
        "added",
        "removed",
        "changed",
        "unchanged",
        "mapped_changed",
        "mapped_unchanged",
        "unresolved",
    ]
    summary = {key: 0 for key in keys}
    for row in rows:
        summary[row["classification"]] += 1
    summary["total"] = len(rows)
    return summary


def _metadata(record: dict[str, Any]) -> dict[str, Any]:
    return {field: deepcopy(record[field]) for field in METADATA_FIELDS if field in record}


def _stable_id(entity: dict[str, Any] | None) -> str | None:
    if not entity:
        return None
    reference = entity.get("reference")
    if isinstance(reference, dict) and reference.get("ref"):
        return str(reference["ref"])
    for key in ("stable_id", "entity_id", "id", "ref"):
        if entity.get(key):
            return str(entity[key])
    return None


def _category(entity: dict[str, Any]) -> str:
    return str(entity.get("category") or entity.get("entity_type") or entity.get("object_type") or "TBD")


def _ref_id(reference: Any) -> str | None:
    if isinstance(reference, dict):
        return str(reference["ref"]) if reference.get("ref") else None
    if isinstance(reference, str) and reference:
        return reference
    return None


def _has_unit_metadata(value: Any) -> bool:
    return (
        isinstance(value, dict)
        and value.get("unit") not in (None, "")
        and value.get("dimension") not in (None, "")
    )


def _entity_ref(entity: dict[str, Any] | None, fallback_ref: str | None) -> dict[str, str]:
    if entity and isinstance(entity.get("reference"), dict):
        ref = entity["reference"]
        return {
            "object_type": str(ref.get("object_type", "Entity")),
            "ref": str(ref.get("ref", fallback_ref or "TBD")),
            **({"label": str(ref["label"])} if ref.get("label") else {}),
        }
    return _reference("Entity", fallback_ref or "TBD", label=_category(entity or {}))


def _mapping_ref(mapping: dict[str, Any], index: int) -> dict[str, str]:
    return _reference("ComparisonMapping", str(mapping.get("mapping_id", f"mapping:{index:04d}")))


def _state_ref(record: dict[str, Any], side: str) -> dict[str, str]:
    return _reference("ModelState", str(record.get("state_id", f"{side}:TBD")))


def _reference(object_type: str, ref: str, *, label: str | None = None) -> dict[str, str]:
    value = {"object_type": object_type, "ref": ref}
    if label:
        value["label"] = label
    return value


def _affected(role: str, ref: dict[str, Any]) -> dict[str, Any]:
    return {"affected_role": role, "affected_ref": deepcopy(ref)}


def _diagnostic(
    code: str,
    diagnostic_class: str,
    severity: str,
    message: str,
    remediation: str,
    affected_refs: list[dict[str, Any]],
    *,
    provenance: dict[str, Any] | None = None,
) -> dict[str, Any]:
    return {
        "code": code,
        "class": diagnostic_class,
        "severity": severity,
        "source": _reference("ModelState", "DEL-14-03"),
        "affected_refs": affected_refs,
        "message": message,
        "remediation": remediation,
        "provenance": deepcopy(provenance or ENGINE_PROVENANCE),
    }


def _row_id(left_id: str | None, right_id: str | None, mapping_id: str | None) -> str:
    basis = mapping_id or f"{left_id or 'none'}:{right_id or 'none'}"
    safe = "".join(character if character.isalnum() else "_" for character in basis)
    return f"row:{safe}"


def _row_sort_key(row: dict[str, Any]) -> tuple[str, str, str, str]:
    left = row.get("left_ref") or {}
    right = row.get("right_ref") or {}
    return (
        row["classification"],
        str(left.get("ref", "")),
        str(right.get("ref", "")),
        row["row_id"],
    )


def _canonical(value: Any) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def utc_timestamp() -> str:
    """Return an explicit UTC timestamp for callers that do not require replay hashes."""

    return datetime.now(UTC).replace(microsecond=0).isoformat()
