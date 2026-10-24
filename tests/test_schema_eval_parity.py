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


@pytest.mark.parametrize("case", VECTORS["cases"], ids=[case["id"] for case in VECTORS["cases"]])
def test_oracle_verdict_matches_the_expected_verdict(case):
    expected = case["valid"]
    divergence = case.get("oracle_divergence")
    if divergence is not None:
        assert divergence in VECTORS["known_oracle_divergences"]
        # The recorded divergence: the oracle accepts what ECMA-262 refuses.
        assert expected is False
        assert oracle_verdict(case) is True
    else:
        assert oracle_verdict(case) is expected


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
