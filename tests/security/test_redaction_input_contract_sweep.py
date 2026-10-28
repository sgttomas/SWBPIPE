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
    TypeScript column (``PROTOTYPE_COPY_COLUMN``).  Each column's count is asserted
    separately."""

    frozen = load_json(DECISIONS_PATH)
    cases = expand_sweep(load_json(SWEEP_PATH))
    before = {name: decoded_column(frozen, name) for name in BEFORE_COLUMNS}
    looser = {name: [] for name in BEFORE_COLUMNS}
    for index, case in enumerate(cases):
        decided = decide(case)
        assert decided not in NO_DECISION, index
        for name, column in before.items():
            if held_to(case[0], name, column[index]) and strictness(decided) < strictness(column[index]):
                looser[name].append((index, column[index], decided))
    assert looser["python_before"] == []
    assert looser["typescript_before"] == []
    assert looser["rust_before"] == []


def test_the_prototype_copy_column_is_the_proto_projector_cases_only_and_pinned():
    """The TypeScript column is set aside only for the appended proto_projector cases, each
    of which is still held to the Python column.  Under the fail-closed rule (HELP_HUMAN
    ruling, 2026-10-27, option (b)) the contract is no less strict than the set-aside column
    either: it is stricter in 1,920 cases (among them the 780 where a member's public labels,
    inherited by the copy, lifted a sibling into a public basis, and the 120 where a member's
    restrictive privacy, inherited by the copy, hid the record's own `classification` alias,
    which redact again) and less strict in none.  Against Python's frozen decisions the
    contract is stricter in 1,932 `__proto__` cases, by design."""

    frozen = load_json(DECISIONS_PATH)
    cases = expand_sweep(load_json(SWEEP_PATH))
    contract = decoded_column(frozen, "contract")
    before = {name: decoded_column(frozen, name) for name in BEFORE_COLUMNS}
    set_aside = [index for index, case in enumerate(cases) if case[0] == "proto_projector"]
    assert len(set_aside) == 10_080 and set_aside[0] >= 82_600
    assert all(before["python_before"][index] not in NO_DECISION for index in set_aside)
    typescript = before["typescript_before"]
    assert sum(strictness(typescript[i]) < strictness(contract[i]) for i in set_aside) == 1_920
    assert sum(strictness(typescript[i]) > strictness(contract[i]) for i in set_aside) == 0
    python = before["python_before"]
    proto_cases = [i for i, case in enumerate(cases) if case[0] in ("proto_items", "proto_projector")]
    assert len(proto_cases) == 18_144 and proto_cases[0] == 82_600
    assert sum(strictness(contract[i]) > strictness(python[i]) for i in proto_cases) == 1_932


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


# A JSON `__proto__` member is an own member (HELP_HUMAN decision, 2026-10-27: "Parse so
# that no JSON member can become a prototype"), and it fails closed on the key itself
# (HELP_HUMAN ruling, 2026-10-27: "option (b), fail closed on the key itself, in all three
# implementations. A record carrying an own `__proto__` member gets no direct public basis,
# and its siblings are labelled by path. The item classifier treats the member as an unknown
# label.").  One class per kind of member label in the code review's 180 cases and their
# converse, with the member on the record and in its provenance, in the five projector
# contexts with and without intent.
PROJECTOR_CONTEXTS = [
    (context, intent)
    for context in ("public_report", "public_example", "shared_model", "downstream_tool", "local_private")
    for intent in (False, True)
]
PROTO_MEMBER_CLASSES = [
    # (class, the member, the record's own labels)
    ("member_unknown_privacy", {"privacy_classification": "unknown"},
     {"classification": "public", "redistribution_status": "public_permissive"}),
    ("member_private_privacy", {"privacy_classification": "private_project_data"},
     {"classification": "public", "redistribution_status": "public_permissive"}),
    ("member_private_labels", {"privacy_classification": "private_project_data", "redistribution_status": "private_only"},
     {"classification": "public", "redistribution_status": "public_permissive"}),
    ("member_private_redistribution", {"redistribution_status": "private_only"},
     {"privacy_classification": "public", "redistribution_status": "public_permissive"}),
    ("member_public_labels", {"privacy_classification": "public", "redistribution_status": "public_permissive"},
     {}),
    ("member_public_redistribution", {"redistribution_status": "public_permissive"},
     {"privacy_classification": "public"}),
    ("member_public_privacy", {"privacy_classification": "public"},
     {"redistribution_status": "public_permissive"}),
    ("member_private_privacy_public_redistribution", {"privacy_classification": "private_project_data", "redistribution_status": "public_permissive"},
     {"classification": "public"}),
]
PUBLIC_LABELS = {"privacy_classification": "public", "redistribution_status": "public_permissive"}


def proto_records(member, labels, member_key="__proto__"):
    """The class's record with the member on it and with the member in its provenance."""

    yield "record", {**labels, member_key: member, "note": "INVENTED"}
    yield "provenance", {**labels, "provenance": {**PUBLIC_LABELS, member_key: member}, "note": "INVENTED"}


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


def unbased(context):
    """A path-labelled sibling of a record with no public basis."""

    return ("warning_only" if context == "local_private" else "redact_value", "REDISTRIBUTION_STATUS_UNKNOWN")


def barred(control, context):
    """The classifier's decision with the safe-and-public pairing barred: inclusion becomes
    the unknown-label decision; every other decision stands."""

    if control[0] != "include":
        return control
    if context == "user_released_report":
        return ("block_export", "USER_RELEASE_VALUE_UNBOUND")
    return ("warning_only" if context == "local_private" else "redact_value", "UNKNOWN_PROVENANCE_WARNING")


def test_route_projector_fails_closed_on_a_json_proto_member():
    """The code review's repro, parsed from JSON text.  The record carries an own
    `__proto__` member, so it has no direct public basis and `note` is redacted, as base
    TypeScript decided and Python did not; the member is kept and its leaf is redacted."""

    payload = json.loads(
        '{"record":{"__proto__":{"privacy_classification":"private_project_data"},'
        '"classification":"public","redistribution_status":"public_permissive","note":"INVENTED"}}'
    )
    decided, materialized = note_decision(payload["record"], "public_report", False)
    assert decided == ("redact_value", "REDISTRIBUTION_STATUS_UNKNOWN")
    assert materialized == {
        "record": {
            "__proto__": {"privacy_classification": "[REDACTED]"},
            "classification": "[REDACTED]",
            "redistribution_status": "[REDACTED]",
            "note": "[REDACTED]",
        }
    }


def test_route_projector_proto_member_classes_give_no_public_basis():
    for name, member, labels in PROTO_MEMBER_CLASSES:
        for placement, record in proto_records(member, labels):
            for context, intent in PROJECTOR_CONTEXTS:
                decided, materialized = note_decision(record, context, intent)
                where = (name, placement, context, intent)
                assert decided == unbased(context), where
                # The member is kept as an own member wherever the payload is materialized.
                if context != "local_private" or intent:
                    record_out = materialized["record"]
                    assert "__proto__" in (record_out if placement == "record" else record_out["provenance"]), where


def test_classifier_proto_member_classes_read_the_member_as_an_unknown_label():
    """An item's `__proto__` member, on the item or in its provenance, bars the
    safe-and-public pairing and nothing else, in all twelve sweep contexts: the item is
    decided as with the member renamed, except that inclusion becomes the unknown-label
    decision."""

    contexts = load_json(SWEEP_PATH)["contexts"]
    for name, member, labels in PROTO_MEMBER_CLASSES:
        for placement in ("item", "provenance"):
            def item(member_key):
                carried = {**labels, member_key: member}
                return {"value": "x", "provenance": carried} if placement == "provenance" else {"value": "x", **carried}

            for context, intent, release in contexts:
                def decide_item(candidate):
                    decision = classify_export_item(
                        candidate,
                        export_context=context,
                        explicit_local_private_intent=intent,
                        explicit_user_release_intent=release,
                    )
                    return decision.action, decision.reason_code

                assert decide_item(item("__proto__")) == barred(decide_item(item("proto_member")), context), (
                    name, placement, context, intent, release
                )


def test_proto_member_classes_are_stricter_than_python_read_them_before_the_ruling():
    """One case per member class, on a record and an item whose own labels are public:
    Python read the member as an ordinary member (as it reads the member renamed), so the
    record's sibling and the item were included; now both fail closed.  Deliberately
    stricter than Python's previous reading."""

    for name, member, _ in PROTO_MEMBER_CLASSES:
        record = {**PUBLIC_LABELS, "__proto__": member, "note": "INVENTED"}
        renamed = {**PUBLIC_LABELS, "proto_member": member, "note": "INVENTED"}
        assert note_decision(renamed, "public_report", False)[0] == ("include", "SAFE_PUBLIC_METADATA"), name
        assert note_decision(record, "public_report", False)[0] == unbased("public_report"), name
        item = classify_export_item({"value": "x", **PUBLIC_LABELS, "__proto__": member}, export_context="public_report")
        control = classify_export_item({"value": "x", **PUBLIC_LABELS, "proto_member": member}, export_context="public_report")
        assert (control.action, control.reason_code) == ("include", "SAFE_PUBLIC_METADATA"), name
        assert (item.action, item.reason_code) == ("redact_value", "UNKNOWN_PROVENANCE_WARNING"), name
        assert strictness(f"{item.action}/{item.reason_code}") > strictness(f"{control.action}/{control.reason_code}")


def test_proto_member_leaves_every_decision_but_a_final_inclusion_unchanged():
    """Codex r2 on #1692: the bar is the last step of the decision order.  Each labels,
    flags, claims and alias_items case, decided with an own `__proto__` member added on the
    item and, where it has one, in its provenance, gives its own decision with only an
    inclusion barred."""

    differ = []
    for index, (family, item, context, intent, release) in enumerate(expand_sweep(load_json(SWEEP_PATH))):
        if family not in ("labels", "flags", "claims", "alias_items"):
            continue

        def decide_item(candidate):
            try:
                decision = classify_export_item(
                    candidate,
                    export_context=context,
                    explicit_local_private_intent=intent,
                    explicit_user_release_intent=release,
                )
            except TypeError:
                return ("raised", "TypeError")
            return decision.action, decision.reason_code

        expected = barred(decide_item(item), context)
        if decide_item({**item, "__proto__": {}}) != expected:
            differ.append((index, "item"))
        provenance = item.get("provenance")
        if isinstance(provenance, dict):
            if decide_item({**item, "provenance": {**provenance, "__proto__": {}}}) != expected:
                differ.append((index, "provenance"))
    assert differ == []


def test_proto_member_keeps_the_concrete_path_redaction_in_every_context():
    """Codex r2 on #1692: in user_released_report with release intent this gave
    `USER_RELEASE_VALUE_UNBOUND`."""

    item = json.loads(
        '{"value":"x","privacy_classification":"public","redistribution_status":"public_permissive",'
        '"concrete_path_present":true,"__proto__":{}}'
    )
    renamed = {("proto_member" if key == "__proto__" else key): value for key, value in item.items()}
    for context, intent, release in load_json(SWEEP_PATH)["contexts"]:
        decided = [
            classify_export_item(
                candidate,
                export_context=context,
                explicit_local_private_intent=intent,
                explicit_user_release_intent=release,
            )
            for candidate in (item, renamed)
        ]
        assert [(d.action, d.reason_code) for d in decided] == [("redact_value", "CONCRETE_PATH_REDACTED")] * 2, context


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
