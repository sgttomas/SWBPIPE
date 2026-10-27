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
CASE_COUNT = 82_600
PROJECTOR = json.loads(SWEEP_PATH.read_text(encoding="utf-8"))["aliases"]["projector"]


def load_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def expand_sweep(spec):
    """Return ``(family, item, context, local_private_intent, release_intent)`` cases.

    Items come in family order labels, flags, claims; each is decided in the spec's
    twelve contexts in order.  A value equal to ``spec["absent"]`` leaves its member out.
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
    return cases


def decide(case):
    """The Python ``action/reason`` for one sweep case: the classifier's for an item, the
    route projector's for the one leaf of an ``alias_projector`` record."""

    family, item, context, local_intent, release_intent = case
    if family == "alias_projector":
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
    the Rust column's ``not_mirrored`` on an alias_projector case; it is held to the Python
    and TypeScript columns."""

    frozen = load_json(DECISIONS_PATH)
    cases = expand_sweep(load_json(SWEEP_PATH))
    before = {name: decoded_column(frozen, name) for name in BEFORE_COLUMNS}
    looser = []
    for index, case in enumerate(cases):
        decided = decide(case)
        assert decided not in NO_DECISION, index
        for name, column in before.items():
            if column[index] not in NO_DECISION and strictness(decided) < strictness(column[index]):
                looser.append((index, name, column[index], decided))
    assert looser == []


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
