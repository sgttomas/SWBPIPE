"""DEL-04-09 I3a (`execution/PKG-04/DEL-04-09/Work/modal-analysis.md` §5.3): the
independent Python modal reader (`tests/verifiers/modal_evidence.py`) over the
committed corpus (`fixtures/results/modal_corpus.json`) and fixtures
(`fixtures/results/modal_*.json`).

Verdict parity: the corpus states each entry's verdict once; the Rust reader is held
to it in `core/reporting/modal_result/tests/corpus.rs`, this reader here, so
the two agree on every closure, binding, relation, health, refusal and decision. All
values are invented.
"""
from __future__ import annotations

import ast
import json
from pathlib import Path

import pytest

from tests.verifiers import canonical_json
from tests.verifiers import modal_evidence as me

ROOT = Path(__file__).resolve().parents[1]
RESULTS = ROOT / "fixtures/results"
CORPUS = json.loads((RESULTS / "modal_corpus.json").read_text(encoding="utf-8"))["entries"]


def load(name):
    return json.loads((RESULTS / name).read_text(encoding="utf-8"))


def _parts(path):
    return [part.replace("~1", "/").replace("~0", "~") for part in path.split("/")[1:]]


def apply(doc, operations):
    """The corpus operations: replace, add, scale, scale_rows and withhold."""
    for op in operations:
        kind = op["op"]
        if kind in ("replace", "add", "scale"):
            parts = _parts(op["path"])
            parent = doc
            for part in parts[:-1]:
                parent = parent[int(part)] if isinstance(parent, list) else parent[part]
            last = parts[-1]
            key = int(last) if isinstance(parent, list) else last
            if kind == "add":
                assert last not in parent, op
            elif not isinstance(parent, list):
                assert last in parent, op
            parent[key] = parent[key] * op["factor"] if kind == "scale" else op["value"]
        elif kind == "scale_rows":
            for row in doc["tables"][op["table"]]["rows"]:
                if all(row.get(k) == v for k, v in op["where"].items()):
                    row[op["field"]]["value"] = row[op["field"]]["value"] * op["factor"]
        elif kind == "withhold":
            doc["tables"][op["table"]] = {"status": "withheld", "reason": "corpus: payload withheld"}
        else:
            raise AssertionError(op)
    return doc


def _reseal_table(table):
    if table.get("status") != "published":
        return
    checksums = []
    for row in table["rows"]:
        body = {k: v for k, v in row.items() if k != "checksum"}
        row["checksum"] = canonical_json.canonical_sha256(body)
        checksums.append(row["checksum"])
    table["row_count"] = len(checksums)
    table["digest"] = canonical_json.canonical_sha256(checksums)


def reseal(doc):
    for table in (doc.get("tables") or {}).values():
        _reseal_table(table)
    if "refusals" in doc:
        _reseal_table(doc["refusals"])


def statuses(verdict):
    return {"closure": verdict["closure"]["status"], "binding": verdict["binding"]["status"],
            "relations": {k: v["status"] for k, v in verdict["relations"].items()}, "health": verdict["health"],
            "decision": verdict["decision"], "refusal_codes": verdict["refusal_codes"]}


def run(entry):
    doc = apply(load(entry["document"]), entry["operations"])
    if entry["reseal"]:
        reseal(doc)
    request = None
    if entry["request"] is not None:
        request = apply(load(entry["request"]), entry["request_operations"])
    verdict = me.read(doc, request)
    return statuses(verdict), verdict


@pytest.mark.parametrize("entry", CORPUS, ids=[entry["id"] for entry in CORPUS])
def test_the_python_reader_returns_every_corpus_verdict(entry):
    got, verdict = run(entry)
    intent = entry["intent"]
    if intent == "none":
        assert got["decision"] in ("consistent_with_scoped_unsupported", "refusal_consistent"), verdict
    elif intent == "scoped":
        assert got["decision"] == "consistent_with_scoped_unsupported", verdict
        assert all(status != "failed" for status in got["relations"].values()), verdict
    elif intent in ("closure", "binding"):
        assert got[intent] == "failed" and got["decision"] == "rejected", verdict
    else:
        assert got["relations"][intent] == "failed" and got["decision"] == "rejected", verdict
    assert got == entry["expected"], verdict


def test_the_corpus_targets_every_relation():
    intents = {entry["intent"] for entry in CORPUS}
    assert {"none", "scoped", "closure", "binding", *me.RELATIONS} <= intents
    assert len(CORPUS) >= 50


def test_the_readers_semantic_table_is_the_committed_table():
    committed = load("semantic_contract_modal_1.json")
    assert me.semantic_table() == committed
    assert canonical_json.canonical_sha256(committed) == me.SEMANTICS_SHA256
    assert me.catalog_differences() == []


def test_every_solved_fixture_is_consistent_only_up_to_its_stated_scope():
    for path in sorted(RESULTS.glob("modal_*.request.json")):
        name = path.name.replace(".request.json", ".json")
        verdict = me.read(load(name), load(path.name))
        if verdict["decision"] == "refusal_consistent":
            assert verdict["refusal_codes"] and verdict["health"] == "not_assessed"
            continue
        assert verdict["decision"] == "consistent_with_scoped_unsupported", (name, verdict)
        units = verdict["relations"]["units"]
        assert units["status"] == "scoped_unsupported"
        assert any(scope.startswith("catalog_family_absent") for scope in units["scope"]), units


def test_the_reader_imports_only_the_standard_library_and_its_siblings():
    tree = ast.parse((ROOT / "tests/verifiers/modal_evidence.py").read_text(encoding="utf-8"))
    imported = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            imported |= {alias.name for alias in node.names}
        elif isinstance(node, ast.ImportFrom):
            imported.add(node.module or "")
    assert imported <= {"__future__", "json", "math", "typing", "tests.verifiers"}, imported
