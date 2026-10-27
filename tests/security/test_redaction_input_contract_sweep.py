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

from core.security.redaction import classify_export_item  # noqa: E402

FIXTURE_DIR = ROOT / "fixtures" / "redaction_export_controls"
SWEEP_PATH = FIXTURE_DIR / "input_contract_sweep.json"
DECISIONS_PATH = FIXTURE_DIR / "input_contract_sweep_decisions.json"
CASE_COUNT = 74_760


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
    return [
        (family, item, context, local_intent, release_intent)
        for family, item in items
        for context, local_intent, release_intent in spec["contexts"]
    ]


def decide(case):
    """The Python classifier's ``action/reason`` for one sweep case."""

    _family, item, context, local_intent, release_intent = case
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


def test_sweep_expands_to_the_pinned_case_count():
    cases = expand_sweep(load_json(SWEEP_PATH))
    assert len(cases) == CASE_COUNT
    assert load_json(DECISIONS_PATH)["case_count"] == CASE_COUNT


def test_frozen_python_column_is_the_current_classifier():
    frozen = load_json(DECISIONS_PATH)
    cases = expand_sweep(load_json(SWEEP_PATH))
    expected = decoded_column(frozen, "python_before")
    assert len(expected) == len(cases)
    mismatches = [index for index, case in enumerate(cases) if decide(case) != expected[index]]
    assert mismatches == []
