"""W-15 slice d (DEL-03-05 Design ``rigid-component-stiffness-scaling.md`` §5), test-only: the reference-section
identification of a component body's results, checked independently of the Rust reader
(``core/reporting/result_export/src/component_body.rs``). Standard library only.

A member formed with a rigid-family component's DEC-045 stiffness scale is a component body. Each of its stress rows
(every case, section basis and combination) and its stress extrema carry the closed record ``component_body`` =
{``component_ref``, ``stiffness_scale``, ``section_role``}; each stress-maximum coverage entry lists the bodies in
``component_body_pipe_ids`` when there are any; the case maxima and headlines range over the other members.
"""
from __future__ import annotations

import math
import struct
from typing import Any, Mapping

RECORD = "component_body"
COVERAGE_KEY = "component_body_pipe_ids"
SECTION_ROLE = "reference_pipe_section"
BODY_REFERENCE_SECTION = "COMPONENT_BODY_REFERENCE_SECTION"
RIGID_FAMILY = frozenset({"valve", "flange", "reducer", "rigid", "specialty"})


def _require(condition: Any, detail: str) -> None:
    if not condition:
        raise ValueError(f"component body: {detail}")


def stress_kinds(table: Mapping[str, Any]) -> frozenset[str]:
    """The table's row kinds whose physical dimension is stress."""
    return frozenset(row.get("kind") for row in table.get("rows", []) if row.get("source_physical_semantic_dimension") == "stress")


def _bits(value: Any) -> Any:
    return struct.pack("<d", value) if isinstance(value, float) else value


def record_shape(record: Any) -> bool:
    return (isinstance(record, Mapping) and set(record) == {"component_ref", "stiffness_scale", "section_role"}
            and isinstance(record["component_ref"], str) and bool(record["component_ref"])
            and isinstance(record["stiffness_scale"], (int, float)) and not isinstance(record["stiffness_scale"], bool)
            and math.isfinite(record["stiffness_scale"]) and record["stiffness_scale"] > 0
            and record["section_role"] == SECTION_ROLE)


def _same(a: Mapping[str, Any], b: Mapping[str, Any]) -> bool:
    return (a["component_ref"] == b["component_ref"] and a["section_role"] == b["section_role"]
            and _bits(float(a["stiffness_scale"])) == _bits(float(b["stiffness_scale"])))


def without_record(value: Mapping[str, Any]) -> dict[str, Any]:
    """A copy without the slice-d members, for closed shape checks."""
    return {key: item for key, item in value.items() if key not in (RECORD, COVERAGE_KEY)}


def _cases(source: Mapping[str, Any]) -> list[Mapping[str, Any]]:
    evidence = source.get("contract_evidence") or {}
    return [case for key in ("preview_cases", "exact_cases") for case in (evidence.get(key) or [])]


def _coverage_entries(case: Mapping[str, Any]) -> list[Mapping[str, Any]]:
    coverage = case.get("stress_maximum_coverage")
    if isinstance(coverage, list):
        return [entry for entry in coverage if isinstance(entry, Mapping)]
    return [coverage] if isinstance(coverage, Mapping) else []


def source_bodies(source: Mapping[str, Any], table: Mapping[str, Any]) -> dict[str, Mapping[str, Any]]:
    """The bodies a received source declares, each with its record (the Rust ``source_bodies``)."""
    listed: list[str] | None = None
    for case in _cases(source):
        for entry in _coverage_entries(case):
            if COVERAGE_KEY in entry:
                ids = entry[COVERAGE_KEY]
                _require(isinstance(ids, list) and ids and all(isinstance(i, str) and i for i in ids) and len(set(ids)) == len(ids), "coverage shape")
            else:
                ids = []
            if listed is None:
                listed = list(ids)
            _require(listed == ids, "coverage disagreement")
    bodies = set(listed or [])
    kinds = stress_kinds(table)
    records: dict[str, Mapping[str, Any]] = {}
    for row in source.get("results") or []:
        entity = row.get("entity_ref")
        expected = entity in bodies and row.get("kind") in kinds
        if RECORD in row:
            record = row[RECORD]
            _require(expected, "forged record")
            _require(record_shape(record), "record shape")
            if entity in records:
                _require(_same(records[entity], record), "inconsistent record")
            else:
                records[entity] = record
        else:
            _require(not expected, "missing record")
    _require(len(records) == len(bodies), "missing record")
    _require(len({r["component_ref"] for r in records.values()}) == len(records), "inconsistent record")
    for case in _cases(source):
        for extremum in case.get("pipe_stress_extrema") or []:
            pipe = extremum.get("pipe_id")
            if RECORD in extremum:
                _require(pipe in records and record_shape(extremum[RECORD]) and _same(extremum[RECORD], records[pipe]), "extremum record")
            else:
                _require(pipe not in records, "missing extremum record")
    return records


def headline_not_body(headline: Any, bodies: Mapping[str, Any]) -> None:
    _require(headline is None or (isinstance(headline, Mapping) and headline.get("location_ref") not in bodies), "headline names a body")


def model_bodies(model: Mapping[str, Any]) -> dict[str, tuple[str, float]]:
    """The model's expected bodies by the producer's eligibility predicate (DEL-03-05 Design §2.3)."""
    pipes = model.get("pipe_segments") or []
    bodies: dict[str, tuple[str, float]] = {}
    for component in model.get("components") or []:
        if component.get("kind") not in RIGID_FAMILY:
            continue
        if (component.get("mechanics_interface") or {}).get("solver_consumption") != "mechanics_geometry_only":
            continue
        node = component.get("node")
        pipe_ref = (component.get("geometry") or {}).get("rigid_pipe_ref")
        if not isinstance(pipe_ref, str) or not pipe_ref.strip():
            continue
        if not any(p.get("id") == pipe_ref and node in (p.get("from"), p.get("to")) for p in pipes):
            continue
        modifiers = component.get("modifiers") or {}
        scale = (modifiers.get("stiffness_scaling_user_value") or {}).get("value")
        source_reference = modifiers.get("source_reference")
        if not isinstance(scale, (int, float)) or isinstance(scale, bool) or not isinstance(source_reference, str) or not source_reference.strip():
            continue
        if not (math.isfinite(scale) and scale > 0):
            continue
        optional = [modifiers.get(key) for key in ("linear_stiffness_user_value", "rotational_stiffness_user_value") if modifiers.get(key) is not None]
        if any(not (isinstance(q.get("value"), (int, float)) and math.isfinite(q["value"]) and q["value"] > 0) for q in optional):
            continue
        _require(pipe_ref not in bodies, "model conflict")
        bodies[pipe_ref] = (component.get("id"), float(scale))
    return bodies


def model_checks(source: Mapping[str, Any], model: Mapping[str, Any], table: Mapping[str, Any]) -> None:
    """The source's bodies and records are exactly the model's (component and scale bit for bit)."""
    received = source_bodies(source, table)
    expected = model_bodies(model)
    _require(set(received) == set(expected), "model mismatch")
    for pipe, (component, scale) in expected.items():
        record = received[pipe]
        _require(record["component_ref"] == component and _bits(float(record["stiffness_scale"])) == _bits(scale), "model mismatch")


def rule_binding_refusal(row: Mapping[str, Any]) -> str | None:
    """A body row binds to no rule input: ``RULE_INPUTS_INCOMPLETE`` with this reason."""
    return BODY_REFERENCE_SECTION if RECORD in row else None
