"""DEL-04-11: the
independent Python verifier (`tests/verifiers/hanger_sizing_evidence.py`) over the committed
corpus (`fixtures/results/hanger_sizing_*.json`), the closed schema
(`schemas/hanger_sizing_result.schema.json`), the headless `size-hangers` verb's documents, and
the analysis-run builder's actual handling of the document.

Verdict parity: the corpus states the expected standings once; the Rust reader is held to all
five (`core/analysis/hanger_sizing/tests/publication.rs`), this verifier to `closure`, `binding`
and the arithmetic. All values are invented.
"""
from __future__ import annotations

from copy import deepcopy
import json
import os
from pathlib import Path
import subprocess

import pytest

from tests.schema_validation import validate_instance
from tests.verifiers import hanger_sizing_evidence as hs

ROOT = Path(__file__).resolve().parents[1]
RESULTS = ROOT / "fixtures/results"
SCHEMA = json.loads((ROOT / "schemas/hanger_sizing_result.schema.json").read_text(encoding="utf-8"))
CORPUS = json.loads((RESULTS / "hanger_sizing_corpus.json").read_text(encoding="utf-8"))["entries"]
DOCUMENTS = ("two_span", "four_span", "refused", "forgery")
PYTHON_STANDINGS = ("closure", "binding", "arithmetic_consistent_with_published_positions")


def load(name):
    return json.loads((RESULTS / name).read_text(encoding="utf-8"))


def mutate(doc, operations):
    """The corpus mutations: replace (an existing member or element) and add (a new object member)."""
    for operation in operations:
        parts = [part.replace("~1", "/").replace("~0", "~") for part in operation["path"].split("/")[1:]]
        parent = doc
        for part in parts[:-1]:
            parent = parent[int(part)] if isinstance(parent, list) else parent[part]
        last = parts[-1]
        if operation["op"] == "replace":
            if isinstance(parent, list):
                parent[int(last)] = operation["value"]
            else:
                assert last in parent, operation
                parent[last] = operation["value"]
        elif operation["op"] == "add":
            assert last not in parent, operation
            parent[last] = operation["value"]
        else:
            raise AssertionError(operation)
    return doc


def supplied(entry):
    bound = load(entry["input"])
    return {"solve_request": bound["request"] if "solve_request" in entry["supply"] else None,
            "library": bound["library"] if "library" in entry["supply"] else None}


@pytest.mark.parametrize("entry", CORPUS, ids=[entry["id"] for entry in CORPUS])
def test_the_verifier_returns_the_corpus_standings(entry):
    doc = mutate(load(entry["document"]), entry["mutations"])
    verdict = hs.verify(json.dumps(doc), **supplied(entry))
    for standing in PYTHON_STANDINGS:
        assert verdict[standing]["status"] == entry["expected"][standing], (entry["id"], standing, verdict[standing])
    assert verdict["binding"]["not_checked"] == entry["expected"]["binding_not_checked"], entry["id"]


def test_the_corpus_holds_the_documents_and_mutations_the_design_requires():
    expected = {entry["id"]: entry["expected"] for entry in CORPUS}
    outcomes = {c["outcome"] for name in ("two_span", "four_span")
                for c in load(f"hanger_sizing_result_{name}.json")["result"]["candidates"]}
    families = {c["selected"]["family"] for name in ("two_span", "four_span")
                for c in load(f"hanger_sizing_result_{name}.json")["result"]["candidates"] if c["selected"]}
    standings = {load(f"hanger_sizing_result_{name}.json")["result"]["verification_standing"] for name in ("two_span", "four_span")}
    assert outcomes == {"selected", "selected_by_fallback", "no_record_fits"}
    assert families == {"variable_spring", "constant_effort"}
    assert standings == {"unconditional", "conditional"}
    assert load("hanger_sizing_result_refused.json")["status"] == "refused"
    for required in ("mutation-outcome", "mutation-record-id", "mutation-count", "mutation-closed-member",
                     "mutation-fixed-text", "mutation-request-hash"):
        assert required in expected, required
    forgery = expected["self-consistent-forgery"]
    assert forgery["arithmetic_consistent_with_published_positions"] == "passed" and forgery["reproduction"] == "failed"


@pytest.mark.parametrize("name", DOCUMENTS)
def test_the_committed_documents_match_the_closed_schema(name):
    validate_instance(SCHEMA, load(f"hanger_sizing_result_{name}.json"), schema_label="hanger_sizing_result",
                      instance_label=name)


@pytest.mark.parametrize("entry", [e for e in CORPUS if e["expected"]["closure"] == "failed"
                                   and e["id"] not in ("mutation-standing",)], ids=lambda e: e["id"])
def test_the_schema_refuses_the_structural_closure_mutations(entry):
    """Each closure mutation the schema can state is refused by it too (the standing against outcomes is
    a reader rule JSON Schema cannot state)."""
    doc = mutate(load(entry["document"]), entry["mutations"])
    with pytest.raises(AssertionError, match="failed JSON Schema validation"):
        validate_instance(SCHEMA, doc, schema_label="hanger_sizing_result", instance_label=entry["id"])


def test_admission_refuses_duplicate_keys_and_numbers_outside_the_profile():
    text = (RESULTS / "hanger_sizing_result_two_span.json").read_text(encoding="utf-8")
    bound = load("hanger_sizing_input_two_span.json")
    assert hs.verify(text, solve_request=bound["request"], library=bound["library"])["closure"]["status"] == "passed"
    w = json.dumps(load("hanger_sizing_result_two_span.json")["result"]["candidates"][0]["w_n"])
    for bad, reason in ((text.replace('"status": "sized"', '"status": "sized", "status": "sized"', 1), "duplicate_key"),
                        (text.replace(f'"w_n": {w}', '"w_n": 1e20', 1), "number_outside_profile"),
                        (text.replace(f'"w_n": {w}', '"w_n": 9007199254740992', 1), "number_outside_profile"),
                        (text.replace(f'"w_n": {w}', '"w_n": 1e400', 1), "number_outside_profile"),
                        (text.replace(f'"w_n": {w}', '"w_n": ' + "9" * 5000, 1), "number_outside_profile"),
                        (text.replace(f'"w_n": {w}', '"w_n": NaN', 1), "invalid_json"),
                        (text + " x", "invalid_json")):
        assert bad != text
        verdict = hs.verify(bad, solve_request=bound["request"], library=bound["library"])
        assert verdict["closure"]["status"] == "failed" and reason in verdict["closure"]["difference"], verdict
        assert verdict["binding"]["status"] == "not_checked"
    # Inside the profile: 2^53 - 1 and -0 are admitted (the document then differs, which is binding's or arithmetic's).
    assert hs.admit('{"a": 9007199254740991, "b": -0.0}') == {"a": 9007199254740991, "b": 0.0}


def test_the_verifiers_fixed_lists_are_the_schemas():
    defs = SCHEMA["$defs"]
    assert tuple(item["const"] for item in defs["Limitations"]["prefixItems"]) == hs.LIMITATIONS
    assert tuple(item["const"] for item in defs["BasisStatement"]["prefixItems"]) == hs.BASIS_STATEMENT
    assert set(defs["RefusalCode"]["enum"]) == hs.REFUSAL_CODES
    assert set(defs["LoadCategory"]["enum"]) == hs.LOAD_CATEGORIES
    assert set(defs["StateCode"]["enum"]) == hs.STATE_CODES


def runner():
    path = os.environ.get("OPENPIPESTRESS_RUNNER_BIN")
    if not path or not Path(path).is_file():
        pytest.skip("the headless runner (OPENPIPESTRESS_RUNNER_BIN) is not available in this session")
    return path


@pytest.mark.parametrize("name, code", [("two_span", 0), ("four_span", 0), ("refused", 1)])
def test_the_runners_documents_read_with_the_python_standings(name, code):
    bound_text = (RESULTS / f"hanger_sizing_input_{name}.json").read_text(encoding="utf-8")
    completed = subprocess.run([runner(), "size-hangers", "--explicit-local-private-intent"], input=bound_text,
                               capture_output=True, text=True, check=False)
    assert completed.returncode == code, completed.stderr
    document = json.loads(completed.stdout)["payload"]
    bound = json.loads(bound_text)
    verdict = hs.verify(json.dumps(document), solve_request=bound["request"], library=bound["library"])
    assert verdict["closure"]["status"] == "passed" and verdict["binding"]["status"] == "passed", verdict
    expected = "passed" if code == 0 else "not_checked"
    assert verdict["arithmetic_consistent_with_published_positions"]["status"] == expected, verdict
    validate_instance(SCHEMA, document, schema_label="hanger_sizing_result", instance_label=name)


def test_the_analysis_run_builder_does_not_accept_the_document():
    """the analysis-run builder routes a source through the Rust reader's header dispatch, which
    refuses the hanger-sizing document by name; no record is built."""
    runner()
    from core.analysis_runs import build_analysis_run
    for name in ("two_span", "four_span", "refused"):
        with pytest.raises(ValueError, match="SOURCE_SCHEMA_VERSION_UNSUPPORTED"):
            build_analysis_run(load(f"hanger_sizing_result_{name}.json"),
                               input_manifest_ref={"ref_type": "audit_manifest", "ref_id": "invented-manifest"},
                               input_manifest_hash="0" * 64)
