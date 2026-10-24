"""Oracle parity for the report package's bounded JSON Schema evaluator.

The vectors in ``fixtures/results/schema_eval_vectors.json`` are evaluated here
with the independent Python Draft 2020-12 validator (``jsonschema`` through
``tests/schema_validation.py``). The Rust evaluator,
``core/reporting/report_package/src/schema_eval.rs``, is checked against the
same expected verdicts by ``core/reporting/report_package/tests/schema_eval.rs``.
See DEL-08-01 Design ``report-package-current-results.md`` §2.3 item 1.

This test does not skip when ``jsonschema`` is missing: in its routed lane the
dependency is required, and a missing one is a failure. Parity is regression
evidence, not a claim of general-validator equivalence.
"""

from __future__ import annotations

from copy import deepcopy
import json
import re
from pathlib import Path
import sys

import pytest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(Path(__file__).resolve().parent))

from schema_validation import (  # noqa: E402
    _draft202012_validator,
    validate_instance,
    validate_schema_document,
)

VECTORS = json.loads(
    (ROOT / "fixtures/results/schema_eval_vectors.json").read_text(encoding="utf-8")
)
VECTOR_ID = "https://openpipestress.org/schemas/schema-eval-vector.schema.json"


def _pointer(value, pointer):
    if pointer in ("", None):
        return value
    for part in pointer.lstrip("/").split("/"):
        part = part.replace("~1", "/").replace("~0", "~")
        value = value[int(part)] if isinstance(value, list) else value[part]
    return value


def _patch(document, operations):
    document = deepcopy(document)
    for operation in operations:
        *parents, last = operation["path"].lstrip("/").split("/")
        target = _pointer(document, "/" + "/".join(parents)) if parents else document
        if operation["op"] == "remove":
            del target[int(last) if isinstance(target, list) else last]
        elif isinstance(target, list) and last == "-":
            target.append(operation["value"])
        else:
            target[int(last) if isinstance(target, list) else last] = operation["value"]
    return document


def case_instance(case):
    if "instance_file" in case:
        instance = json.loads((ROOT / case["instance_file"]).read_text(encoding="utf-8"))
        instance = _pointer(instance, case.get("instance_pointer", ""))
    else:
        instance = case["instance"]
    return _patch(instance, case.get("patch", []))


def oracle_verdict(case):
    schema = {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": VECTOR_ID,
        **case["schema"],
    }
    validate_schema_document(schema, schema_label=case["id"])
    try:
        validate_instance(schema, case_instance(case), instance_label=case["id"])
    except AssertionError as error:
        if "failed JSON Schema validation" not in str(error):
            raise
        return False
    return True


def test_the_oracle_dependency_is_present():
    # Fails, rather than skips, without jsonschema.
    assert _draft202012_validator() is not None


# The one admitted disagreement class (DEL-08-01 Design §2.3 item 1): an
# ECMA-262 `$` does not match before a final line feed; Python's `re` does.
FINAL_LF_CLASS = "ecma_dollar_before_final_line_feed"
CLOSURE_TEXT = "".join(
    (ROOT / "schemas" / name).read_text(encoding="utf-8")
    for name in (
        "results.v0.3.schema.yaml",
        "results_pressure_2.v0.3.schema.json",
        "physics_source_recovery.schema.json",
        "retained_precision_mp_v2.schema.json",
        "load_reference_state.schema.json",
    )
)


def _without_final_line_feed(case):
    """The case with the line feed removed from its recorded string."""
    pointer = case["oracle_divergence"]["string_pointer"]
    twin = deepcopy(case)
    value = _pointer(case_instance(case), pointer)
    if pointer:
        twin.setdefault("patch", [])
        twin["patch"] = [op for op in twin["patch"] if op.get("path") != pointer]
        twin["patch"].append({"op": "replace", "path": pointer, "value": value[:-1]})
    else:
        twin.pop("patch", None)
        twin["instance"] = value[:-1]
    return twin


def check_final_line_feed_entry(case):
    """An enumerated class entry: the ECMA verdict is invalid, the oracle's is
    valid, and the final line feed is the whole difference."""
    divergence = case["oracle_divergence"]
    assert set(divergence) == {"class", "string_pointer", "pattern"}, case["id"]
    assert divergence["class"] == FINAL_LF_CLASS
    assert divergence["class"] in VECTORS["known_oracle_divergences"]
    assert "pattern" in case["covers"]
    pattern = divergence["pattern"]
    assert re.fullmatch(r"\^\[0-9a-f\]\{\d+\}\$", pattern), pattern
    assert json.dumps(pattern) in CLOSURE_TEXT, "the pattern is one of the closure's"
    value = _pointer(case_instance(case), divergence["string_pointer"])
    assert isinstance(value, str) and value.endswith("\n") and "\n" not in value[:-1]
    # ECMA-262 `$` without the m flag is end of input, Python's `\Z`.
    assert re.search(pattern, value) is not None
    assert re.search(pattern[:-1] + r"\Z", value) is None
    assert case["valid"] is False, "the expected (Rust) verdict is the ECMA-262 one"
    assert oracle_verdict(case) is True
    assert oracle_verdict(_without_final_line_feed(case)) is True


@pytest.mark.parametrize("case", VECTORS["cases"], ids=[case["id"] for case in VECTORS["cases"]])
def test_oracle_verdict_matches_the_expected_verdict(case):
    if "oracle_divergence" in case:
        check_final_line_feed_entry(case)
    else:
        assert oracle_verdict(case) is case["valid"]


def test_every_evaluated_keyword_has_a_valid_and_an_invalid_case():
    evaluated = {
        "$ref", "additionalProperties", "allOf", "anyOf", "const", "contains", "dependentRequired",
        "else", "enum", "exclusiveMinimum", "if", "items", "maxItems", "maximum", "minItems",
        "minLength", "minimum", "not", "oneOf", "pattern", "properties", "required", "then",
        "type", "uniqueItems",
    }
    seen = {(keyword, case["valid"]) for case in VECTORS["cases"] for keyword in case["covers"]}
    missing = sorted(f"{keyword}:{verdict}" for keyword in evaluated for verdict in (True, False)
                     if (keyword, verdict) not in seen)
    assert not missing, missing


def test_every_vector_has_the_same_verdict_outside_the_enumerated_class():
    """Parity: every disagreement is an enumerated final-line-feed entry, and
    that class has exactly the one recorded name (review round 1, S3)."""
    assert set(VECTORS["known_oracle_divergences"]) == {FINAL_LF_CLASS}
    mismatches = sorted(case["id"] for case in VECTORS["cases"] if oracle_verdict(case) is not case["valid"])
    enumerated = sorted(case["id"] for case in VECTORS["cases"] if "oracle_divergence" in case)
    assert mismatches == enumerated
    for case in VECTORS["cases"]:
        if case["id"] in enumerated:
            check_final_line_feed_entry(case)
