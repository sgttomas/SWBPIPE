"""The shared base-reader corpus (b7-plan R2) and its committed Rust outcomes, for Python tests.

`core/reporting/result_export/tests/base_reader_outcomes.rs` builds the corpus statements from the
corpus files and pins the native `reader_verdict` outcome of each in
`fixtures/results/base_reader_outcomes.json`. This module builds the same statements in Python, from
the same files with the same mutation grammar, so the Python verifier and the product's Rust-routed
paths can be compared with those outcomes entry by entry. Test support only.

B7b: the corpus is built from the current fixtures only (the Rust test's B7b construction): the exact-route
statements are the v3 twins read by the pressure-2 and pressure-source-1 readers, with and without the
load/reference-state group; the ledger cases apply to the received joined statements directly (the v2
projection is gone); the retired v2 identities appear only as named refusals.
"""
from __future__ import annotations

from copy import deepcopy
from functools import lru_cache
import json
from pathlib import Path
from typing import Any, Callable, NamedTuple

ROOT = Path(__file__).resolve().parents[1]
OUTCOMES = ROOT / "fixtures/results/base_reader_outcomes.json"
MARKER = "__BASE_READER_TEXT_LITERAL__"
MODES = ("sparse_interactive", "dense_scrutiny")
ROUTE_READERS = {"preview_physics": "preview-physics", "pressure": "pressure", "pressure_source": "pressure-source"}
PARSE_REJECTED = "JSON_PARSE_REJECTED"


class Input(NamedTuple):
    """One corpus statement and what is read of it (the Rust test's `Input`). `source` is None when its
    text is not JSON the Rust readers parse (a non-finite or out-of-range literal): `text` holds it."""
    id: str
    source: Any
    text: str | None
    model: Any
    invocation: Any
    readers: tuple[str, ...]
    transports: tuple[str, ...]
    rows: bool


def read(path: str) -> Any:
    return json.loads((ROOT / path).read_text(encoding="utf-8"))


def model_at(path: str) -> Any:
    value = read(path)
    return value["model"] if "model" in value else value


def _tokens(path: str) -> list[str]:
    return [token.replace("~1", "/").replace("~0", "~") for token in path[1:].split("/")]


def _parent(doc: Any, path: str) -> tuple[Any, str]:
    parent = doc
    for token in _tokens(path)[:-1]:
        parent = parent[int(token)] if isinstance(parent, list) else parent[token]
    return parent, _tokens(path)[-1]


def _pointer(doc: Any, path: str) -> Any:
    for token in _tokens(path) if path else []:
        doc = doc[int(token)] if isinstance(doc, list) else doc[token]
    return doc


def _set(doc: Any, path: str, value: Any) -> None:
    parent, last = _parent(doc, path)
    if isinstance(parent, list):
        parent[int(last)] = value
    else:
        parent[last] = value


def _matches(item: Any, pattern: dict[str, Any]) -> bool:
    return isinstance(item, dict) and all(key in item and item[key] == value for key, value in pattern.items())


def reseal(doc: dict[str, Any], op: dict[str, Any], domain_hash: Callable[[str, Any], str]) -> None:
    """The corpus's `reseal` op: recompute the named receipt hashes over the mutated statement (B7b: the current
    case-evidence domain, the record bound by its case id, as the reader binds it)."""
    evidence = doc.get("contract_evidence")
    if op.get("physical") is not None:
        exact = evidence["exact_cases"]
        for index, case in enumerate(doc["source_block_recovery"]["body"]["cases"]):
            pressure = [p for p in evidence["pressure"] if p["load_case_id"] == exact[index]["load_case_id"]]
            records = evidence.get("load_reference_states")
            record = next((r for r in records if r.get("load_case_id") == exact[index]["load_case_id"]), None) if isinstance(records, list) else None
            kind = op["physical"]
            if kind == "joined":
                domain, payload = "pressure_source_case_evidence_v1", {"exact_case": exact[index], "pressure": pressure, "load_reference_state": record}
            elif kind == "joined_without_record":
                domain, payload = "pressure_source_case_evidence_v1", {"exact_case": exact[index], "pressure": pressure}
            elif kind == "physics":
                domain, payload = "physics_source_case_evidence_v1", {"exact_case": exact[index], "pressure": pressure}
            else:
                raise AssertionError(f"unknown reseal {kind}")
            case["physical_evidence_sha256"] = domain_hash(domain, payload)
    if op.get("publication") is True:
        publication = {key: value for key, value in doc.items() if key != "source_block_recovery"}
        doc["source_block_recovery"]["body"]["publication_sha256"] = domain_hash("source_blocks_publication_v1", publication)
    if op.get("receipt") is True:
        doc["source_block_recovery"]["receipt_sha256"] = domain_hash("source_blocks_receipt_v1", doc["source_block_recovery"]["body"])


def apply(doc: Any, ops: list[dict[str, Any]], sources: Callable[[str], Any], domain_hash: Callable[[str, Any], str]) -> tuple[Any, str | None]:
    """The mutated document and, for a `nonfinite` or `json_text` op, the literal replacing the marker in its text."""
    doc = deepcopy(doc)
    literal = None
    for op in ops:
        path, kind = op.get("path", ""), op["op"]
        if kind in ("set", "replace"):
            _set(doc, path, deepcopy(op["value"]))
        elif kind == "remove":
            parent, last = _parent(doc, path)
            if isinstance(parent, list):
                parent.pop(int(last))
            else:
                del parent[last]
        elif kind == "append":
            _pointer(doc, path).append(deepcopy(op["value"]))
        elif kind == "add":
            parent, last = _parent(doc, path)
            if isinstance(parent, dict):
                parent[last] = deepcopy(op["value"])
            elif last == "-":
                parent.append(deepcopy(op["value"]))
            else:
                parent.insert(int(last), deepcopy(op["value"]))
        elif kind == "reverse":
            _pointer(doc, path).reverse()
        elif kind == "copy_append":
            _pointer(doc, path).append(deepcopy(_pointer(doc, op["from"])))
        elif kind == "swap":
            first, second = deepcopy(_pointer(doc, path)), deepcopy(_pointer(doc, op["with"]))
            _set(doc, path, second)
            _set(doc, op["with"], first)
        elif kind == "graft":
            _set(doc, path, deepcopy(_pointer(sources(op["source"]), op["from"])))
        elif kind == "remove_where":
            array = _pointer(doc, path)
            array[:] = [item for item in array if not _matches(item, op["match"])]
        elif kind == "set_where":
            for item in _pointer(doc, path):
                if _matches(item, op["match"]):
                    item[op["key"]] = deepcopy(op["value"])
        elif kind == "append_copy_where":
            array = _pointer(doc, path)
            copy = deepcopy(next(item for item in array if _matches(item, op["match"])))
            copy.update(deepcopy(op["set"]))
            array.append(copy)
        elif kind == "nonfinite":
            _set(doc, path, MARKER)
            literal = {"Infinity": "1e400", "-Infinity": "-1e400", "NaN": "NaN"}[op["value"]]
        elif kind == "json_text":
            _set(doc, path, MARKER)
            literal = op["value"]
        elif kind == "reseal":
            reseal(doc, op, domain_hash)
        else:
            raise AssertionError(f"unknown op {kind}")
    return doc, literal


def _transported(doc: dict[str, Any]) -> dict[str, Any]:
    meta = {"schema_version": doc["schema_version"]}
    for key in ("producer", "numerical_quality", "formulation_basis", "contract_evidence", "source_block_recovery", "carrier_evidence"):
        if key in doc:
            meta[key] = doc[key]
    return meta


def _materialized(doc: Any, literal: str | None) -> tuple[Any, str | None]:
    if literal is None:
        return doc, None
    return None, json.dumps(doc, ensure_ascii=False, separators=(",", ":")).replace(f'"{MARKER}"', literal)


@lru_cache(maxsize=1)
def corpus_inputs(route_of: Callable[[Any], str | None], domain_hash: Callable[[str, Any], str]) -> tuple[Input, ...]:
    """The corpus statements in the Rust test's order. `route_of(source)` names a base statement's route (or
    None when its header is refused), as the Rust test reads `rv::route`; the caller supplies its own reading."""
    inputs: list[Input] = []

    def base(id_: str, source: Any, model: Any, invocation: Any) -> None:
        reader = ROUTE_READERS.get(route_of(source) or "")
        inputs.append(Input(id_, source, None, model, invocation, (reader,) if reader else (),
                            (reader,) if reader else (), True))

    results = [
        ("preview_physics_connected_sparse", "fixtures/model_operations/precision_connected_ui_model.json"),
        ("preview_physics_connected_dense", "fixtures/model_operations/precision_connected_ui_model.json"),
        ("preview_physics_invented_sparse", "core/product_physics/tests/fixtures/preview_physics_invented_model.json"),
        ("preview_physics_invented_dense", "core/product_physics/tests/fixtures/preview_physics_invented_model.json"),
        ("preview_physics_unicode_ids_sparse", "core/product_physics/tests/fixtures/preview_physics_unicode_ids_model.json"),
        ("pressure_connected_mechanics_sparse", "core/product_physics/tests/fixtures/exact_pressure_connected_request.json"),
        ("pressure_connected_mechanics_dense", "core/product_physics/tests/fixtures/exact_pressure_connected_request.json"),
        ("pressure_connected_ui_mechanics_sparse", "fixtures/model_operations/exact_pressure_authoring_model.json"),
        ("pressure_connected_ui_mechanics_dense", "fixtures/model_operations/exact_pressure_authoring_model.json"),
        ("pressure_thermal_ui_mechanics_sparse", "fixtures/model_operations/physics_thermal_ui_model.json"),
        ("pressure_thermal_ui_mechanics_dense", "fixtures/model_operations/physics_thermal_ui_model.json"),
        # T4-U4b review item 5: a directional pressure-2 arc and a preview arc envelope.
        ("pressure_u4b_arc_sparse", "core/product_physics/tests/fixtures/u4b_l90_p_model.json"),
        ("preview_physics_u4b_arc_sparse", "core/product_physics/tests/fixtures/u4b_z_preview_rev_model.json"),
        ("../product_preview/invented_mechanics_result_preview_physics_1_sparse", None),
        ("../product_preview/invented_mechanics_result_preview_physics_1_dense", None),
        ("../product_preview/invented_demo_result_preview_physics_1_sparse", None),
        ("../product_preview/invented_demo_result_preview_physics_1_dense", None),
    ]
    for name, model in results:
        base(f"results/{name.removeprefix('../')}", read(f"fixtures/results/{name}.json"), model_at(model) if model else None, None)
    # The joined load/reference-state statements are read by the one source reader, which takes their invocation
    # (never numerically eligible).
    for folder, names, invoked in (("pressure_source/physics_source", ("n05", "n06", "fields", "mixed", "mixed_units", "n05_units", "n05_unicode"), True),
                                   ("pressure/load_reference", ("connected", "pressure"), False),
                                   ("pressure_source/load_reference_source", ("n05", "n06", "fields", "mixed", "eigen_motion"), True)):
        for name in names:
            request = read(f"fixtures/product_preview/{folder}/{name}.request.json")
            for mode in MODES:
                source = read(f"fixtures/product_preview/{folder}/{name}-{mode}.raw.json")
                invocation = {"request": request, "solver_mode": mode} if invoked else None
                base(f"{folder}/{name}-{mode}", source, deepcopy(request["model"]), invocation)
    # B4/D1 (b): the preview route's retired source-blocks-1 statements left the corpus.
    fallback = model_at("core/reporting/result_export/tests/fixtures/load_reference_fallback_uz.request.json")
    for mode in MODES:
        base(f"load_reference_fallback_uz-{mode}", read(f"core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-{mode}.raw.json"),
             deepcopy(fallback), None)
    # W-16 slice b: the shared pressure-2 combination corpus, each envelope with its model.
    for case in read("fixtures/results/pressure_v3_combination_reader_corpus.json")["cases"]:
        base(f"combination/{case['label']}", case["envelope"], deepcopy(case["model"]), None)

    def unreachable(_name: str) -> Any:
        raise AssertionError("graft outside a mutation file")

    vector = read("fixtures/results/preview_physics_tamper_vector.json")
    for variant in vector["variants"]:
        doc, literal = apply(read(vector["bases"][variant["base"]]), variant["ops"], unreachable, domain_hash)
        assert literal is None
        # Codex round 2: a variant with a transport expectation is also read by transport.
        transports = ("preview-physics",) if "transport" in variant else ()
        inputs.append(Input(f"tamper/{variant['id']}", doc, None, None, None, ("preview-physics",), transports, False))
    for path, reader, joined in (("core/reporting/result_export/tests/fixtures/load_reference_mutations.json", "pressure", False),
                                 ("core/reporting/result_export/tests/fixtures/load_reference_source_mutations.json", "pressure-source", True)):
        cases = read(path)
        sources = {name: read(file) for name, file in cases["sources"].items()}
        lookup = lambda name: sources[name]
        prefix = "lrs" if joined else "lr"
        for case in cases["cases"]:
            doc, literal = apply(sources[case["source"]], case["ops"], lookup, domain_hash)
            readers = [reader]
            if joined and "lr" in case:
                readers.append("pressure")
            assert "ps" not in case, "B7b folded 'ps' into 'joined'"
            source, text = _materialized(doc, literal)
            inputs.append(Input(f"{prefix}/{case['id']}", source, text, None, None, tuple(readers), (), False))
        for case in cases["transport_cases"]:
            doc, literal = apply(sources[case["source"]], case["ops"], lookup, domain_hash)
            assert literal is None
            inputs.append(Input(f"{prefix}-transport/{case['id']}", _transported(doc), None, None, None, (), (reader,), False))
    for case in read("fixtures/results/loadReferenceReaderCases.cases.json"):
        doc, literal = apply(read(case["source"]), case["ops"], unreachable, domain_hash)
        assert literal is None
        inputs.append(Input(f"lr-reader/{case['id']}", doc, None, None, None, ("pressure",), (), False))
    # The ledger cases: the source reader's ledger checks on the received joined statement (B7b: no projection).
    received: dict[str, Any] = {}
    for case in read("fixtures/results/loadReferenceSourceLedger.cases.json"):
        name = case["source"]
        if name not in received:
            received[name] = read(f"fixtures/product_preview/pressure_source/load_reference_source/{name}.raw.json")
        doc, literal = apply(received[name], case["ops"], unreachable, domain_hash)
        assert literal is None
        if isinstance(case.get("reseal_publication"), bool):
            reseal(doc, {"physical": None, "publication": case["reseal_publication"], "receipt": True}, domain_hash)
        inputs.append(Input(f"ledger/{case['id']}", doc, None, None, None, ("pressure-source",), (), False))
    return tuple(inputs)


@lru_cache(maxsize=1)
def committed() -> dict[str, dict[str, Any]]:
    """The committed native outcomes, by entry id."""
    entries = json.loads(OUTCOMES.read_text(encoding="utf-8"))["entries"]
    return {entry["id"]: entry for entry in entries}


def text(outcome: dict[str, Any]) -> str:
    """An outcome as the corpus files state reader outcomes: "accept" or the reader's whole error text."""
    if outcome["ok"] is True:
        return "accept"
    error = outcome["error"]
    return error["detail"] or error["code"]


def product_reading(entry: dict[str, Any]) -> str | None:
    """What the product's `_source_contract` gives for a committed entry, read from the native outcomes: the
    header dispatch's refusal, or its route's reader (`accept` for a route without one); None when the
    entry does not pin its route's reader."""
    if "parse" in entry:
        return PARSE_REJECTED
    dispatch = entry["dispatch"]
    if dispatch["ok"] is not True:
        return text(dispatch)
    reader = ROUTE_READERS.get(dispatch["route"])
    if reader is None:
        return "accept"
    outcome = entry.get("validate", {}).get(reader)
    return None if outcome is None else text(outcome)


def product_refusal_text(error: BaseException) -> str:
    """A product refusal's text; a statement refused at the JSON text boundary reads as the corpus states it."""
    message = str(error)
    if message.startswith("READER_INPUT_NOT_JSON"):
        return PARSE_REJECTED
    return message
