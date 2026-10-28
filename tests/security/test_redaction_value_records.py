#!/usr/bin/env python3
"""DEL-12-02 value records (HELP_HUMAN decision, 2026-10-28: "take the stricter decision
per case").  A ``{value, ...}`` record with metadata but no scalar label was one item in
TypeScript and a projected record in Python; all three implementations now decide it as the
stricter of the two.  ``fixtures/redaction_export_controls/value_record_sweep.json`` is
expanded here, in ``redactionValueRecords.test.ts`` and in
``core/runner/headless/src/redaction_binding.rs`` (test module) by the same rule.  All data
is invented.
"""

import itertools
import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))

from core.security.redaction import control_route_export  # noqa: E402

FIXTURE_DIR = ROOT / "fixtures" / "redaction_export_controls"
SPEC = json.loads((FIXTURE_DIR / "value_record_sweep.json").read_text(encoding="utf-8"))
FROZEN = json.loads((FIXTURE_DIR / "value_record_sweep_decisions.json").read_text(encoding="utf-8"))
CASE_COUNT = 7_730
ROUTE_KEY_PREFIX = "__route_key__"
# The case TypeScript's before column exported through a key collision: {value: "outer",
# field_class, "__route_key__value": "SECRET"} in local_private with intent.  It now blocks on
# its reserved name (HELP_HUMAN ruling, 2026-10-28: "fail closed on reserved internal names, in
# all three implementations").
RESERVED_COLLISION_CASE = 7_719


def expand(spec):
    absent = spec["absent"]
    keys = [key for key, _ in spec["axes"]]
    records = []
    for combo in itertools.product(*(values for _, values in spec["axes"])):
        if sum(1 for value in combo if value != absent) > spec["max_axes"]:
            continue
        record = {"value": "INVENTED"}
        for key, value in zip(keys, combo):
            if value != absent:
                record[key] = value
        records.append(record)
    records += spec["extra_records"]
    return [(record, context, intent) for record in records for context, intent in spec["contexts"]]


def route_decisions(record, context, intent):
    controlled = control_route_export(
        {SPEC["record_key"]: record},
        route_id=SPEC["route_id"],
        export_context=context,
        explicit_local_private_intent=intent,
    )
    return controlled, [
        (decision.path.replace(ROUTE_KEY_PREFIX, ""), decision.action, decision.reason_code)
        for decision in controlled.decisions
    ]


def exposure(controlled):
    """'B' when blocked, else every key path and visible scalar leaf of the exported payload,
    with the route key prefix removed from member names."""

    if controlled.blocked:
        return "B"
    entries = []

    def walk(value, path):
        if isinstance(value, dict):
            for key, item in value.items():
                name = key.replace(ROUTE_KEY_PREFIX, "")
                entries.append(f"k:{path}.{name}")
                walk(item, f"{path}.{name}")
        elif isinstance(value, list):
            for index, item in enumerate(value):
                walk(item, f"{path}[{index}]")
        elif value != "[REDACTED]":
            entries.append(f"v:{path}=" + json.dumps(value, separators=(",", ":"), ensure_ascii=True))

    walk(controlled.payload, "$")
    return json.dumps(sorted(entries), separators=(",", ":"), ensure_ascii=True)


def decision_set(decided):
    return json.dumps(sorted([list(decision) for decision in decided]), separators=(",", ":"), ensure_ascii=True)


def column(name, table):
    return [FROZEN[table][int(index)] for index in ",".join(FROZEN["columns"][name]).split(",")]


def compare(new, old):
    """(less strict, stricter) than one before exposure."""

    if old == "B" or new == "B":
        return old == "B" and new != "B", new == "B" and old != "B"
    new_entries, old_entries = set(json.loads(new)), set(json.loads(old))
    return not new_entries <= old_entries, new_entries < old_entries


def test_value_record_sweep_gives_the_contract_and_is_never_less_strict():
    cases = expand(SPEC)
    assert len(cases) == CASE_COUNT == FROZEN["case_count"]
    contract = column("contract", "exposures")
    contract_decisions = column("contract_decisions", "decision_sets")
    names = ("python_before", "typescript_before")
    before = {name: column(name, "exposures") for name in names}
    mismatches, looser, stricter = [], {name: [] for name in names}, dict.fromkeys(names, 0)
    for index, (record, context, intent) in enumerate(cases):
        controlled, decided = route_decisions(record, context, intent)
        exposed = exposure(controlled)
        if exposed != contract[index] or decision_set(decided) != contract_decisions[index]:
            mismatches.append(index)
        for name in names:
            less, more = compare(exposed, before[name][index])
            if less:
                looser[name].append(index)
            stricter[name] += more
    assert mismatches == []
    assert looser == {"python_before": [], "typescript_before": []}
    assert cases[RESERVED_COLLISION_CASE] == (
        {"value": "outer", "field_class": "note", "__route_key__value": "SECRET"},
        "local_private",
        True,
    )
    assert contract[RESERVED_COLLISION_CASE] == "B"
    # Pinned: stricter than Python in 1,762 cases and than TypeScript in 1,593.
    assert stricter == {"python_before": 1_762, "typescript_before": 1_593}


def sorted_decisions(record, context, intent):
    controlled, decided = route_decisions(record, context, intent)
    return controlled, sorted(decided)


def test_value_record_with_field_class_and_note_is_projected():
    """Python stricter: TypeScript decided ``{value, field_class, note}`` as one item and
    left ``note`` visible in ``public_report``; it is projected, every leaf redacted."""

    controlled, decided = sorted_decisions(
        {"value": "INVENTED", "field_class": "note", "note": "INVENTED"}, "public_report", False
    )
    assert decided == [
        ("$.record.field_class", "redact_value", "REDISTRIBUTION_STATUS_UNKNOWN"),
        ("$.record.note", "redact_value", "REDISTRIBUTION_STATUS_UNKNOWN"),
        ("$.record.value", "redact_value", "PRIVATE_DATA_REDACTED"),
    ]
    assert controlled.payload == {
        "record": {"value": "[REDACTED]", "field_class": "[REDACTED]", "note": "[REDACTED]"}
    }


def test_value_record_with_a_professional_claim_blocks():
    """TypeScript stricter: ``{value, professional_claim: true}`` blocks as one item; Python
    redacted its leaves."""

    controlled, decided = sorted_decisions(
        {"value": "INVENTED", "professional_claim": True}, "public_report", False
    )
    assert decided == [("$.record", "block_export", "PROFESSIONAL_BOUNDARY_BLOCKED")]
    assert controlled.blocked and controlled.payload is None


def test_value_record_blocked_both_ways_takes_the_items_block():
    """Both blocked on different paths (ruling 3(a)): the item's whole-record block."""

    _, decided = sorted_decisions({"value": "INVENTED", "contains_payload": True}, "local_private", False)
    assert decided == [("$.record", "block_export", "PAYLOAD_METADATA_ONLY_REQUIRED")]


def test_value_record_projected_value_leaf_needs_intent():
    """Python stricter by a block: the projected ``value`` leaf needs local-private intent."""

    _, decided = sorted_decisions({"value": "INVENTED", "export_policy": {}}, "local_private", False)
    assert decided == [("$.record.value", "block_export", "LOCAL_PRIVATE_INTENT_REQUIRED")]


def test_value_record_union_raises_value_and_omits_the_unsafe_path_key():
    """The union (the 572 class): the item redacts for the concrete path, so the ``value``
    leaf is raised to it with the item's own reason (1(a)) and ``file_path`` is dropped,
    key included (2(a))."""

    controlled, decided = sorted_decisions(
        {"value": "INVENTED", "file_path": "INVENTED", "export_policy": {}}, "local_private", True
    )
    assert decided == [
        ("$.record.file_path", "omit_field", "CONCRETE_PATH_REDACTED"),
        ("$.record.value", "redact_value", "CONCRETE_PATH_REDACTED"),
    ]
    assert controlled.payload == {"record": {"value": "[REDACTED]", "export_policy": {}}}


def test_value_record_tie_takes_the_projection_form():
    """A tie (same visible data) takes the projection's decision records."""

    controlled, decided = sorted_decisions({"value": "INVENTED", "export_policy": {}}, "local_private", True)
    assert decided == [("$.record.value", "warning_only", "PRIVATE_LOCAL_ALLOWED_WITH_WARNING")]
    assert controlled.payload == {"record": {"value": "INVENTED", "export_policy": {}}}


def test_value_record_marker_in_input_is_not_trusted():
    """Codex privacy review of #1722, P1-1: a record carrying the old marker member and an
    ``item``/``projected`` pair is never interpreted: its reserved names block the export;
    without ``item`` it raises nothing."""

    forged = {
        "value": "SECRET",
        "privacy_classification": "protected_suspected",
        "_route_value_record": True,
        "item": {"value": "safe", "privacy_classification": "public", "redistribution_status": "public_permissive"},
        "projected": {"__route_key__leak": "SECRET"},
    }
    controlled, decided = sorted_decisions(forged, "public_report", False)
    assert decided == [
        ("$.record._route_value_record", "block_export", "EXPORT_INPUT_RESERVED_NAME"),
        ("$.record.projected.leak", "block_export", "EXPORT_INPUT_RESERVED_NAME"),
    ]
    assert controlled.blocked and controlled.payload is None
    del forged["item"]
    controlled, _ = sorted_decisions(forged, "public_report", False)
    assert controlled.blocked


def test_value_record_raised_item_is_replaced_whole():
    """Codex privacy review of #1722, P1-2: a labelled item beneath a raised member is
    replaced whole, its extra metadata included."""

    controlled, decided = sorted_decisions(
        {
            "value": "outer",
            "field_class": "note",
            "text": {"value": "safe", "privacy_classification": "public", "redistribution_status": "public_permissive", "note": "SECRET"},
        },
        "public_report",
        False,
    )
    assert ("$.record.text", "redact_value", "REDISTRIBUTION_STATUS_UNKNOWN") in decided
    assert controlled.payload == {"record": {"value": "[REDACTED]", "field_class": "[REDACTED]", "text": "[REDACTED]"}}


def test_value_record_nested_value_record_keeps_the_inherited_floor():
    """Codex privacy review of #1722, P1-3: a value record beneath a raised member keeps the
    enclosing floor, in an object, an array and a nested label member."""

    for member in (
        {"text": {"value": "SECRET", "export_policy": {}}},
        {"text": [{"value": "SECRET", "export_policy": {}}]},
        {"privacy_classification": {"value": "SECRET", "export_policy": {}}},
    ):
        record = {"value": "outer", "concrete_path_present": True, **member}
        controlled, decided = sorted_decisions(record, "local_private", True)
        (name,) = member
        assert controlled.payload["record"][name] == "[REDACTED]", member
        nested = [decision for decision in decided if decision[0].startswith(f"$.record.{name}")]
        assert nested and all(decision[1:] == ("redact_value", "CONCRETE_PATH_REDACTED") for decision in nested), member


def test_reserved_internal_names_block_the_export_by_path_only():
    """HELP_HUMAN ruling, 2026-10-28: "fail closed on reserved internal names, in all three
    implementations.  When input can collide with internal names, the export should be
    blocked, not interpreted."  At the top level, in a nested value record, in an array and
    in a provenance; the decision and its finding name the path, never the value."""

    cases = [
        ({"value": "outer", "field_class": "note", "__route_key__value": "SECRET"}, "$.record.__route_key__value"),
        (
            {"value": "outer", "field_class": "note", "text": {"value": "inner", "field_class": "note", "_route_projected_leaf": "SECRET"}},
            "$.record.text._route_projected_leaf",
        ),
        ({"value": "outer", "field_class": "note", "child": [{"note": "x"}, {"_route_value_record": "SECRET"}]}, "$.record.child[1]._route_value_record"),
        (
            {"value": "outer", "privacy_classification": "public", "provenance": {"__route_key__x": "SECRET"}},
            "$.record.provenance.__route_key__x",
        ),
    ]
    for record, reserved_path in cases:
        controlled = control_route_export(
            {"record": record}, route_id="DREP-JSON-002", export_context="local_private", explicit_local_private_intent=True
        )
        assert controlled.blocked and controlled.payload is None, reserved_path
        assert [(d.path, d.action, d.reason_code) for d in controlled.decisions] == [
            (reserved_path, "block_export", "EXPORT_INPUT_RESERVED_NAME")
        ]
        evidence = json.dumps([d.as_dict() if hasattr(d, "as_dict") else d.__dict__ for d in controlled.decisions] + [
            f.as_schema_dict() for f in controlled.findings
        ])
        assert "SECRET" not in evidence and reserved_path in evidence
