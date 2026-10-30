"""T6S-1 (CQ-7 B; I74 decision 7) and b7-plan E2: the results dispatcher is a $ref
to results.v0.3.schema.yaml, the one result-export version, so it admits exactly
what the version file admits; the retired 0.1.0/0.2.0 arms are gone (F-U6c-2:
the old inline copy knew only precision-1, which B1 retired; both now refuse it).

Validation goes through the registry helper (tests/schema_validation.py
validate_instance), which resolves the version file and its own external
references locally. These are shape checks only: a schema verdict creates no
standing, eligibility or producer-origin claim. Synthetic documents are built
around verbatim raw producer metadata (test_load_reference_schema
`_results_document`) and are labelled as such."""
from copy import deepcopy
import json
from pathlib import Path

import pytest

from tests.schema_validation import validate_instance
from tests.test_load_reference_schema import _results_document


ROOT = Path(__file__).resolve().parents[1]
SCHEMAS = ROOT / "schemas"
RESULTS = ROOT / "fixtures/results"
VERSION_FILE = "results.v0.3.schema.yaml"
PREVIEW_ID = "openpipestress.result_semantics/0.3.0/preview-physics-2"
PRESSURE_ID = "openpipestress.result_semantics/0.3.0/pressure-2"
# B7b: the retired exact v2 identities have no branch in the version file.
RETIRED_V2_IDS = tuple(f"openpipestress.result_semantics/0.3.0/{name}" for name in ("physics-1", "physics-source-1", "load-reference-1", "load-reference-source-1"))
RETIRED_PRECISION_ID = "openpipestress.result_semantics/0.3.0/precision-1"
SUCCESSOR_ID = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
# T6S-2's Rust goldens: derive_document's successor form of both pinned milestones.
GOLDENS = ("retained_precision_successor_derivative_sparse_interactive.json",
           "retained_precision_successor_derivative_dense_scrutiny.json")


def schema(name):
    return json.loads((SCHEMAS / name).read_text(encoding="utf-8"))


def admits(schema_value, document):
    try:
        validate_instance(schema_value, document, instance_label="dispatcher test")
    except AssertionError:
        return False
    return True


def committed_documents():
    """Every committed result document (a JSON object with a string
    schema_version and an object result_envelope) under the project's fixture,
    test, core, example and validation trees, at any depth of its file."""
    found = []
    for root in ("fixtures", "tests", "core", "examples", "validation"):
        for path in sorted((ROOT / root).rglob("*.json")):
            if {"node_modules", "target"} & set(path.relative_to(ROOT).parts):
                continue
            try:
                value = json.loads(path.read_text(encoding="utf-8"))
            except (OSError, ValueError):
                continue
            stack = [(value, "")]
            while stack:
                item, pointer = stack.pop()
                if isinstance(item, dict):
                    if isinstance(item.get("schema_version"), str) and isinstance(item.get("result_envelope"), dict):
                        found.append((f"{path.relative_to(ROOT).as_posix()}#{pointer}", item))
                    stack.extend((child, f"{pointer}/{key}") for key, child in item.items())
                elif isinstance(item, list):
                    stack.extend((child, f"{pointer}/{index}") for index, child in enumerate(item))
    return sorted(found, key=lambda entry: entry[0])


def committed_v03():
    return [(name, document) for name, document in committed_documents() if document["schema_version"] == "0.3.0"]


def synthetic(name):
    """A results 0.3 scaffold around one committed raw producer envelope."""
    source = json.loads((RESULTS / name).read_text(encoding="utf-8"))
    identity = source["producer"]["semantic_contract_id"]
    if "contract_evidence" in source:
        return _results_document(source, identity)
    document = _results_document({**source, "contract_evidence": None}, identity)
    del document["result_envelope"]["contract_evidence"]
    return document


def coverage_documents():
    """(label, identity, document): preview-physics-2, pressure-2 (B7b: physics-1's
    current twin) and the successor at 0.3.0."""
    goldens = [(f"golden {name}", SUCCESSOR_ID, json.loads((RESULTS / name).read_text(encoding="utf-8"))) for name in GOLDENS]
    return goldens + [
        ("synthetic preview-physics-2", PREVIEW_ID, synthetic("preview_physics_connected_sparse.json")),
        ("synthetic pressure-2", PRESSURE_ID, synthetic("pressure_connected_mechanics_sparse.json")),
    ]


def refused_variants(document):
    """Instances the version file refuses, for verdict equivalence on refusal."""
    extra = deepcopy(document)
    extra["result_envelope"]["unexpected_member"] = True
    missing = deepcopy(document)
    del missing["result_envelope"]["row_accounting"]
    receipt = deepcopy(document)
    if "retained_precision" in receipt["result_envelope"]:
        del receipt["result_envelope"]["retained_precision"]  # the successor without its receipt
    else:
        golden = json.loads((RESULTS / GOLDENS[0]).read_text(encoding="utf-8"))
        receipt["result_envelope"]["retained_precision"] = golden["result_envelope"]["retained_precision"]
    return [("unexpected member", extra), ("row_accounting missing", missing), ("receipt mismatched to identity", receipt)]


def equivalence_failures(dispatcher, documents, *, first=False):
    """Documents on which the dispatcher's verdict differs from the version file's."""
    version = schema(VERSION_FILE)
    failures = []
    for name, document in documents:
        if admits(dispatcher, document) != admits(version, document):
            failures.append(name)
            if first:
                break
    return failures


def test_dispatcher_is_a_reference_to_the_version_file():
    dispatcher = schema("results.schema.yaml")
    assert dispatcher["$ref"] == VERSION_FILE
    assert set(dispatcher) == {"$schema", "$id", "title", "description", "$ref"}
    assert schema(VERSION_FILE)["$id"] == "https://openpipestress.org/schemas/results.v0.3.schema.yaml"
    assert schema(VERSION_FILE)["properties"]["schema_version"] == {"const": "0.3.0"}
    assert "results.v0.3.schema.yaml" in dispatcher["description"]


def test_committed_v03_documents_include_the_goldens():
    names = [name for name, _ in committed_v03()]
    for golden in GOLDENS:
        assert f"fixtures/results/{golden}#" in names
    # B7b: the load/reference-state documents are the pressure-2 and pressure-source-1 twins (4 + 10).
    assert sum(name.startswith(("fixtures/results/pressure_load_state", "fixtures/results/pressure_source_load_state")) for name in names) >= 14
    assert not any(name.startswith("fixtures/results/load_reference") for name in names)
    assert len(names) >= 16


def verdicts(documents):
    dispatcher, version = schema("results.schema.yaml"), schema(VERSION_FILE)
    return [(name, admits(dispatcher, document), admits(version, document)) for name, document in documents]


def test_dispatcher_and_version_file_agree_on_every_committed_v03_document():
    found = verdicts(committed_v03())
    assert [name for name, by_dispatcher, by_file in found if by_dispatcher != by_file] == []
    # Every committed 0.3.0 document is admitted, so the agreement is on admission.
    assert [name for name, _, by_file in found if not by_file] == []


def test_dispatcher_and_version_file_agree_on_refusals():
    picked = [entry for entry in committed_v03() if "retained_precision_successor" in entry[0]]
    picked += [entry for entry in committed_v03() if "pressure_source_load_state_n05_sparse" in entry[0]][:1]
    picked += [(label, document) for label, _, document in coverage_documents() if label.startswith("synthetic")]
    variants = [(f"{name}: {what}", variant) for name, document in picked for what, variant in refused_variants(document)]
    found = verdicts(variants)
    assert [name for name, by_dispatcher, by_file in found if by_dispatcher != by_file] == []
    # Every variant is refused, so the agreement is on refusal.
    assert [name for name, _, by_file in found if by_file] == []


COVERAGE_LABELS = ["golden sparse_interactive", "golden dense_scrutiny", "synthetic preview-physics-2",
                   "synthetic pressure-2"]


@pytest.mark.parametrize("index", range(len(COVERAGE_LABELS)), ids=COVERAGE_LABELS)
def test_dispatcher_admits_preview_physics_pressure_and_the_successor(index):
    label, identity, document = coverage_documents()[index]
    assert document["schema_version"] == document["result_envelope"]["schema_version"] == "0.3.0"
    assert document["result_envelope"]["semantic_contract_ref"]["ref_id"] == identity, label
    assert admits(schema("results.schema.yaml"), document), label
    assert admits(schema(VERSION_FILE), document), label


@pytest.mark.parametrize("outer,inner", [
    ("0.3.0", "0.2.0"), ("0.3.0", "0.1.0"), ("0.2.0", "0.3.0"), ("0.1.0", "0.3.0"),
    ("0.1.0", "0.1.0"), ("0.2.0", "0.2.0"),
    ("0.4.0", "0.3.0"), ("0.3.0", "0.4.0"), ("0.4.0", "0.4.0"),
])
def test_mixed_and_unknown_versions_are_refused(outer, inner):
    dispatcher = schema("results.schema.yaml")
    for name in GOLDENS[:1]:
        document = json.loads((RESULTS / name).read_text(encoding="utf-8"))
        document["schema_version"] = outer
        document["result_envelope"]["schema_version"] = inner
        assert not admits(dispatcher, document), (outer, inner)


def wrong_reference(target):
    def mutate(dispatcher):
        dispatcher["$ref"] = target
    return mutate


def dropped_reference(dispatcher):
    """The reference is replaced by a schema nothing satisfies."""
    del dispatcher["$ref"]
    dispatcher["not"] = {}


@pytest.mark.parametrize("mutate", [
    wrong_reference("rule_check_run_result.schema.json"),
    wrong_reference("analysis_run.v0.3.schema.json"),
    dropped_reference,
], ids=["ref-rule-check", "ref-analysis-run", "drop-ref"])
def test_dispatcher_mutants_are_killed(mutate):
    """Each mutant breaks the 0.3.0 equivalence or coverage above."""
    mutant = schema("results.schema.yaml")
    mutate(mutant)
    coverage = [(label, document) for label, _, document in coverage_documents()]
    assert equivalence_failures(mutant, coverage + committed_v03(), first=True)


def test_dispatcher_and_version_file_refuse_precision_1():
    """B1: a results 0.3 document under the retired precision-1 identity has no branch."""
    source = json.loads((RESULTS / "preview_physics_connected_sparse.json").read_text(encoding="utf-8"))
    source["producer"]["semantic_contract_id"] = RETIRED_PRECISION_ID
    source.pop("contract_evidence")
    document = _results_document({**source, "contract_evidence": None}, RETIRED_PRECISION_ID)
    del document["result_envelope"]["contract_evidence"]
    assert verdicts([("precision-1", document)]) == [("precision-1", False, False)]


@pytest.mark.parametrize("identity", RETIRED_V2_IDS)
def test_dispatcher_and_version_file_refuse_the_retired_v2_identities(identity):
    """B7b: a results 0.3 document under a retired exact v2 identity has no branch."""
    document = synthetic("pressure_connected_mechanics_sparse.json")
    document["result_envelope"]["semantic_contract_ref"]["ref_id"] = identity
    assert verdicts([(identity, document)]) == [(identity, False, False)]
