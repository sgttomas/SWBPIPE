#!/usr/bin/env python3
"""DEL-12-02 input-contract sweep: the classifier never decides a case less strictly
than the pre-contract code did, and the three implementations agree case for case.

`fixtures/redaction_export_controls/input_contract_sweep.json` is expanded here, in
`apps/desktop/src/features/redaction-controls/redactionInputContractSweep.test.ts` and in
`core/runner/headless/src/redaction_binding.rs` (test module) by the same rule.
"""

import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))

from core.security.redaction import classify_export_item, control_route_export  # noqa: E402

FIXTURE_DIR = ROOT / "fixtures" / "redaction_export_controls"
SWEEP_PATH = FIXTURE_DIR / "input_contract_sweep.json"
DECISIONS_PATH = FIXTURE_DIR / "input_contract_sweep_decisions.json"
CASE_COUNT = 100_744
PROJECTOR = json.loads(SWEEP_PATH.read_text(encoding="utf-8"))["aliases"]["projector"]


def load_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def expand_sweep(spec):
    """Return ``(family, item, context, local_private_intent, release_intent)`` cases.

    Items come in family order labels, flags, claims, alias_items; each is decided in the
    spec's twelve contexts in order.  Then the alias_projector records, each decided in the
    projector's ten contexts in order; then the proto_items and the proto_projector records
    in the same way.  A value equal to ``spec["absent"]`` leaves its member out.
    """

    absent = spec["absent"]

    def put(target, key, value):
        if not (isinstance(value, str) and value == absent):
            target[key] = value

    items = []
    labels = spec["labels"]
    for privacy in labels["privacy"]:
        for redistribution in labels["redistribution"]:
            for review in labels["review"]:
                for placement in labels["placements"]:
                    item = {"value": "x"}
                    if placement == "item":
                        put(item, "privacy_classification", privacy)
                        put(item, "redistribution_status", redistribution)
                        put(item, "review_status", review)
                    elif placement == "provenance":
                        provenance = {}
                        put(provenance, "privacy_classification", privacy)
                        put(provenance, "redistribution_status", redistribution)
                        put(provenance, "review_status", review)
                        item["provenance"] = provenance
                    else:
                        # split: the privacy and review on the item, the redistribution on a
                        # public provenance whose review is quarantined only with the item's.
                        put(item, "privacy_classification", privacy)
                        provenance = {
                            "privacy_classification": "public",
                            "redistribution_status": "public_permissive",
                            "review_status": "quarantined" if review == "quarantined" else "accepted",
                        }
                        put(provenance, "redistribution_status", redistribution)
                        item["provenance"] = provenance
                        put(item, "review_status", review)
                    items.append(("labels", item))
    for key in spec["flags"]["keys"]:
        for value in spec["flags"]["values"]:
            for base in spec["bases"]:
                item = {"value": "x", **base}
                if key.startswith("export_policy."):
                    item["export_policy"] = {key.split(".", 1)[1]: value}
                else:
                    item[key] = value
                items.append(("flags", item))
    for value in spec["claims"]["values"]:
        for base in spec["bases"]:
            item = {"value": "x", **base}
            put(item, "professional_claim", value)
            items.append(("claims", item))
    aliases = spec["aliases"]

    def alias_labels(privacy, alias, redistribution):
        labels = {}
        put(labels, "privacy_classification", privacy)
        put(labels, "classification", alias)
        put(labels, "redistribution_status", redistribution)
        return labels

    alias_grid = [
        (privacy, alias, redistribution)
        for privacy in aliases["privacy_classification"]
        for alias in aliases["classification"]
        for redistribution in aliases["redistribution"]
    ]
    for privacy, alias, redistribution in alias_grid:
        for placement in aliases["item_placements"]:
            labels = alias_labels(privacy, alias, redistribution)
            if placement == "provenance":
                item = {"value": "x", "provenance": labels}
            else:
                item = {"value": "x", **labels}
                if placement == "item_private_provenance":
                    item["provenance"] = dict(aliases["private_provenance"])
            items.append(("alias_items", item))
    cases = [
        (family, item, context, local_intent, release_intent)
        for family, item in items
        for context, local_intent, release_intent in spec["contexts"]
    ]
    projector = aliases["projector"]
    for privacy, alias, redistribution in alias_grid:
        for placement in projector["placements"]:
            record = alias_labels(privacy, alias, redistribution)
            if placement == "record_public_provenance":
                record["provenance"] = dict(projector["public_provenance"])
            record[projector["leaf_key"]] = projector["leaf_value"]
            cases.extend(
                ("alias_projector", record, context, local_intent, False)
                for context, local_intent in projector["contexts"]
            )
    proto = spec["proto"]
    member = proto["member"]
    proto_grid = [
        (privacy, alias, redistribution, member_value)
        for privacy in proto["privacy_classification"]
        for alias in proto["classification"]
        for redistribution in proto["redistribution"]
        for member_value in proto["member_values"]
    ]
    # A `__proto__` key is an ordinary key of a Python dict, as of a JSON object.
    for privacy, alias, redistribution, member_value in proto_grid:
        for placement in proto["item_placements"]:
            labels = alias_labels(privacy, alias, redistribution)
            labels[member] = member_value
            item = {"value": "x", "provenance": labels} if placement == "provenance" else {"value": "x", **labels}
            cases.extend(
                ("proto_items", item, context, local_intent, release_intent)
                for context, local_intent, release_intent in spec["contexts"]
            )
    for privacy, alias, redistribution, member_value in proto_grid:
        for placement in proto["projector_placements"]:
            record = alias_labels(privacy, alias, redistribution)
            if placement == "provenance":
                record["provenance"] = {**projector["public_provenance"], member: member_value}
            else:
                record[member] = member_value
                if placement == "record_public_provenance":
                    record["provenance"] = dict(projector["public_provenance"])
            record[projector["leaf_key"]] = projector["leaf_value"]
            cases.extend(
                ("proto_projector", record, context, local_intent, False)
                for context, local_intent in projector["contexts"]
            )
    return cases


def decide(case):
    """The Python ``action/reason`` for one sweep case: the classifier's for an item, the
    route projector's for the one leaf of an ``alias_projector`` or ``proto_projector`` record."""

    family, item, context, local_intent, release_intent = case
    if family in PROJECTOR_FAMILIES:
        projector = PROJECTOR
        controlled = control_route_export(
            {projector["record_key"]: item},
            route_id=projector["route_id"],
            export_context=context,
            explicit_local_private_intent=local_intent,
        )
        # A projected decision's path carries the projector's key prefix on each member.
        leaf = f"$.{projector['record_key']}.{projector['leaf_key']}"
        decided = [
            f"{decision.action}/{decision.reason_code}"
            for decision in controlled.decisions
            if decision.path.replace("__route_key__", "") == leaf
        ]
        assert len(decided) == 1, (leaf, decided)
        return decided[0]
    try:
        decision = classify_export_item(
            item,
            export_context=context,
            explicit_local_private_intent=local_intent,
            explicit_user_release_intent=release_intent,
        )
    except TypeError:
        return "raised/TypeError"
    return f"{decision.action}/{decision.reason_code}"


def decoded_column(frozen, name):
    codes = frozen["codes"]
    alphabet = frozen["alphabet"]
    return [codes[alphabet.index(symbol)] for symbol in "".join(frozen["decisions"][name])]


# "Less strict" is this total order on the classifier's actions, least to most strict.
# `redact_value` sits above `warning_only` by the existing code: `warning_only` returns the
# item with its value (`_apply_action`) and is the unknown-metadata action only in
# `local_private`, while `redact_value` replaces `value` and `text` with `[REDACTED]`, drops
# the unsafe detail keys, and is that action in every public or shared context.  The
# classifier produces no `redact_field` or `omit_field`.
ACTION_ORDER = ("include", "warning_only", "redact_value", "block_export")
BEFORE_COLUMNS = ("python_before", "typescript_before", "rust_before")
RAISED = "raised/TypeError"
# The Rust mirror had no route projector before the alias decision, so its frozen column
# holds no decision for an alias_projector case.
NOT_MIRRORED = "not_mirrored/NO_ROUTE_PROJECTOR"
NO_DECISION = (RAISED, NOT_MIRRORED)
PROJECTOR_FAMILIES = ("alias_projector", "proto_projector")
# The TypeScript projector before the decision that no JSON member becomes a prototype
# (HELP_HUMAN, 2026-10-27: "Parse so that no JSON member can become a prototype") copied a
# record by assignment, so a `__proto__` member became the copy's prototype, or was dropped
# when it was not an object.  Its proto_projector decisions are of a different record than
# the input, so those cases are held to the Python column, the reference whose reading of the
# member as an ordinary own member all three now share (the Rust column is `not_mirrored`).
PROTOTYPE_COPY_COLUMN = ("proto_projector", "typescript_before")


def held_to(family, name, code):
    return code not in NO_DECISION and (family, name) != PROTOTYPE_COPY_COLUMN


def strictness(code):
    return ACTION_ORDER.index(code.split("/", 1)[0])


def test_sweep_expands_to_the_pinned_case_count():
    cases = expand_sweep(load_json(SWEEP_PATH))
    assert len(cases) == CASE_COUNT
    frozen = load_json(DECISIONS_PATH)
    assert frozen["case_count"] == CASE_COUNT
    assert tuple(frozen["action_order"]) == ACTION_ORDER
    for name in (*BEFORE_COLUMNS, "contract"):
        assert len(decoded_column(frozen, name)) == CASE_COUNT, name


def test_no_case_is_decided_less_strictly_than_before_in_any_implementation():
    """The ruling's proof obligation: every case's decision is at least as strict as each
    implementation's frozen pre-contract decision.  Python's former TypeError on an array or
    object label is no decision; the case is held to the TypeScript and Rust columns.  Nor is
    the Rust column's ``not_mirrored`` on an alias_projector or proto_projector case; it is
    held to the Python and TypeScript columns.  A proto_projector case is not held to the
    TypeScript column (``PROTOTYPE_COPY_COLUMN``)."""

    frozen = load_json(DECISIONS_PATH)
    cases = expand_sweep(load_json(SWEEP_PATH))
    before = {name: decoded_column(frozen, name) for name in BEFORE_COLUMNS}
    looser = []
    for index, case in enumerate(cases):
        decided = decide(case)
        assert decided not in NO_DECISION, index
        for name, column in before.items():
            if held_to(case[0], name, column[index]) and strictness(decided) < strictness(column[index]):
                looser.append((index, name, column[index], decided))
    assert looser == []


def test_the_prototype_copy_column_is_the_proto_projector_cases_only_and_pinned():
    """The TypeScript column is set aside only for the appended proto_projector cases, each
    of which is still held to a decision, and its frozen decisions there differ from the
    contract as pinned: 780 less strict (a `__proto__` member's public labels, inherited by
    the copy, lifted a sibling into a public basis) and 120 stricter (a member's restrictive
    privacy, inherited by the copy, hid the record's own `classification` alias)."""

    frozen = load_json(DECISIONS_PATH)
    cases = expand_sweep(load_json(SWEEP_PATH))
    contract = decoded_column(frozen, "contract")
    before = {name: decoded_column(frozen, name) for name in BEFORE_COLUMNS}
    set_aside = [index for index, case in enumerate(cases) if case[0] == "proto_projector"]
    assert len(set_aside) == 10_080 and set_aside[0] >= 82_600
    assert all(before["python_before"][index] not in NO_DECISION for index in set_aside)
    typescript = before["typescript_before"]
    assert sum(strictness(typescript[i]) < strictness(contract[i]) for i in set_aside) == 780
    assert sum(strictness(typescript[i]) > strictness(contract[i]) for i in set_aside) == 120


def test_python_gives_the_contract_decision_for_every_case():
    """With the vitest and cargo twins, the three implementations agree case for case."""

    frozen = load_json(DECISIONS_PATH)
    cases = expand_sweep(load_json(SWEEP_PATH))
    contract = decoded_column(frozen, "contract")
    mismatches = [index for index, case in enumerate(cases) if decide(case) != contract[index]]
    assert mismatches == []


# Named regressions (HELP_HUMAN ruling 2026-10-27).  The per-row cases of the (a'), (b')
# and (c) tables are the `input-contract-*` cases of the shared corpus `cases.json`,
# decided by all three suites.

INPUT_CONTRACT_CORPUS_CASES = 49


def test_input_contract_corpus_cases_are_present_and_outside_the_pinned_original_cases():
    corpus = load_json(FIXTURE_DIR / "cases.json")
    added = [case for case in corpus["cases"] if case["case_id"].startswith("input-contract-")]
    assert len(added) == INPUT_CONTRACT_CORPUS_CASES
    assert all("explicit_user_release_intent" in case for case in added)
    assert corpus["cases"][-INPUT_CONTRACT_CORPUS_CASES:] == added


def test_route_projector_reads_an_array_label_as_no_public_basis():
    """TypeScript's route projector read ``String(["public"])`` as a public basis and
    included the sibling value; Python raised TypeError.  Both now redact it."""

    payload = {
        "record": {
            "privacy_classification": ["public"],
            "redistribution_status": ["public_permissive"],
            "note": "Invented placeholder note",
        }
    }
    controlled = control_route_export(
        payload, route_id="DREP-JSON-002", export_context="public_report"
    )
    note = [decision for decision in controlled.decisions if decision.path.endswith("note")]
    assert [(decision.action, decision.reason_code) for decision in note] == [
        ("redact_value", "REDISTRIBUTION_STATUS_UNKNOWN")
    ]
    assert controlled.payload["record"]["note"] == "[REDACTED]"


def test_route_projector_reads_no_classification_alias_under_a_null_primary_label():
    """HELP_HUMAN decision, 2026-10-27: "A null primary label must not let an alias lift a
    sibling into ``public_report``."  TypeScript read the ``classification`` alias with
    ``??`` and so included ``note``; Python reads it only when ``privacy_classification`` is
    absent and redacts it, as all three now do."""

    record = {
        "privacy_classification": None,
        "classification": "public",
        "redistribution_status": "public_permissive",
        "note": "INVENTED",
    }
    controlled = control_route_export(
        {"record": record}, route_id="DREP-JSON-002", export_context="public_report"
    )
    note = [decision for decision in controlled.decisions if decision.path.endswith("note")]
    assert [(decision.action, decision.reason_code) for decision in note] == [
        ("redact_value", "REDISTRIBUTION_STATUS_UNKNOWN")
    ]
    assert controlled.payload["record"]["note"] == "[REDACTED]"
    # The alias is still the label when the primary member is absent.
    del record["privacy_classification"]
    controlled = control_route_export(
        {"record": record}, route_id="DREP-JSON-002", export_context="public_report"
    )
    note = [decision for decision in controlled.decisions if decision.path.endswith("note")]
    assert [(decision.action, decision.reason_code) for decision in note] == [
        ("include", "SAFE_PUBLIC_METADATA")
    ]
    assert controlled.payload["record"]["note"] == "INVENTED"


# A JSON `__proto__` member is an ordinary own member (HELP_HUMAN decision, 2026-10-27:
# "Parse so that no JSON member can become a prototype").  One class per kind of member
# label in the code review's 180 cases and their converse: the member's labels label only
# the member, never the record that holds it, on the record or in its provenance, in the
# five projector contexts with and without intent.
PROJECTOR_CONTEXTS = [
    (context, intent)
    for context in ("public_report", "public_example", "shared_model", "downstream_tool", "local_private")
    for intent in (False, True)
]
PROTO_MEMBER_CLASSES = [
    # (class, the member, the record's own labels, the record's labels are a public basis)
    ("member_unknown_privacy", {"privacy_classification": "unknown"},
     {"classification": "public", "redistribution_status": "public_permissive"}, True),
    ("member_private_privacy", {"privacy_classification": "private_project_data"},
     {"classification": "public", "redistribution_status": "public_permissive"}, True),
    ("member_private_labels", {"privacy_classification": "private_project_data", "redistribution_status": "private_only"},
     {"classification": "public", "redistribution_status": "public_permissive"}, True),
    ("member_private_redistribution", {"redistribution_status": "private_only"},
     {"privacy_classification": "public", "redistribution_status": "public_permissive"}, True),
    ("member_public_labels", {"privacy_classification": "public", "redistribution_status": "public_permissive"},
     {}, False),
    ("member_public_redistribution", {"redistribution_status": "public_permissive"},
     {"privacy_classification": "public"}, False),
    ("member_public_privacy", {"privacy_classification": "public"},
     {"redistribution_status": "public_permissive"}, False),
    ("member_private_privacy_public_redistribution", {"privacy_classification": "private_project_data", "redistribution_status": "public_permissive"},
     {"classification": "public"}, False),
]
PUBLIC_PROVENANCE = {"privacy_classification": "public", "redistribution_status": "public_permissive"}


def proto_records(member, labels, member_key="__proto__"):
    """The class's record with the member on it and with the member in its provenance."""

    yield "record", {**labels, member_key: member, "note": "INVENTED"}
    yield "provenance", {**labels, "provenance": {**PUBLIC_PROVENANCE, member_key: member}, "note": "INVENTED"}


def note_decision(record, context, intent):
    controlled = control_route_export(
        {"record": record}, route_id="DREP-JSON-002", export_context=context, explicit_local_private_intent=intent
    )
    note = [
        (decision.action, decision.reason_code)
        for decision in controlled.decisions
        if decision.path.replace("__route_key__", "") == "$.record.note"
    ]
    assert len(note) == 1
    return note[0], controlled.payload


def test_route_projector_reads_a_json_proto_member_as_an_ordinary_own_member():
    """The code review's repro, parsed from JSON text: the member labels only itself, so the
    record's own `classification` alias and redistribution are its public basis and `note` is
    included, as Python always decided; the member is kept and its leaf is redacted."""

    payload = json.loads(
        '{"record":{"__proto__":{"privacy_classification":"private_project_data"},'
        '"classification":"public","redistribution_status":"public_permissive","note":"INVENTED"}}'
    )
    decided, materialized = note_decision(payload["record"], "public_report", False)
    assert decided == ("include", "SAFE_PUBLIC_METADATA")
    assert materialized == {
        "record": {
            "__proto__": {"privacy_classification": "[REDACTED]"},
            "classification": "public",
            "redistribution_status": "public_permissive",
            "note": "INVENTED",
        }
    }


def test_route_projector_proto_member_classes_label_only_the_member():
    for name, member, labels, public_basis in PROTO_MEMBER_CLASSES:
        expected_public = ("include", "SAFE_PUBLIC_METADATA")
        for (placement, record), (_, renamed) in zip(
            proto_records(member, labels), proto_records(member, labels, member_key="proto_member")
        ):
            for context, intent in PROJECTOR_CONTEXTS:
                expected = (
                    expected_public
                    if public_basis
                    else ("warning_only" if context == "local_private" else "redact_value", "REDISTRIBUTION_STATUS_UNKNOWN")
                )
                decided, materialized = note_decision(record, context, intent)
                where = (name, placement, context, intent)
                assert decided == expected, where
                assert note_decision(renamed, context, intent)[0] == decided, where
                # The member is kept as an ordinary member wherever the payload is materialized.
                if context != "local_private" or intent:
                    record_out = materialized["record"]
                    assert "__proto__" in (record_out if placement == "record" else record_out["provenance"]), where


def test_classifier_proto_member_classes_label_only_the_member():
    """An item's `__proto__` member, on the item or in its provenance, decides nothing: the
    item is decided as with the member renamed, in all twelve sweep contexts."""

    contexts = load_json(SWEEP_PATH)["contexts"]
    for name, member, labels, _ in PROTO_MEMBER_CLASSES:
        for placement in ("item", "provenance"):
            def item(member_key):
                carried = {**labels, member_key: member}
                return {"value": "x", "provenance": carried} if placement == "provenance" else {"value": "x", **carried}

            for context, intent, release in contexts:
                decide_item = lambda candidate: classify_export_item(
                    candidate,
                    export_context=context,
                    explicit_local_private_intent=intent,
                    explicit_user_release_intent=release,
                )
                decided = decide_item(item("__proto__"))
                control = decide_item(item("proto_member"))
                assert (decided.action, decided.reason_code) == (control.action, control.reason_code), (
                    name, placement, context, intent, release
                )


def test_concrete_path_never_turns_a_protected_block_into_a_redaction():
    """Before the reorder, ``concrete_path_present: true`` returned ``redact_value`` /
    ``CONCRETE_PATH_REDACTED`` ahead of the protected-content block, in every context."""

    item = {
        "field_id": "invented.input_contract.path_protected",
        "concrete_path_present": True,
        "privacy_classification": "protected_suspected",
        "redistribution_status": "public_permissive",
        "value": "Invented placeholder value",
    }
    for context in (
        "public_report",
        "public_example",
        "shared_model",
        "downstream_tool",
        "local_private",
        "user_released_report",
    ):
        decision = classify_export_item(item, export_context=context)
        assert (decision.action, decision.reason_code) == (
            "block_export",
            "PROTECTED_CONTENT_BLOCKED",
        ), context
