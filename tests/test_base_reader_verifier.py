"""b7-plan R2: the Python verifier and the product's Rust-routed reading over the shared base corpus.

The corpus is `base_reader_outcomes.rs`'s (`tests/base_reader_corpus.py` builds the same 558 statements in
Python); `fixtures/results/base_reader_outcomes.json` pins the native outcome of each.

- The verifier (`tests/verifiers/reader_verdict.py`) gives every committed outcome: the dispatch entry (route,
  table binding, header refusal, standing with its G1 findings), each reader's validation (with G2's model
  checks) and transport checks, and each row's semantics (signature ids and the outcomes' digest), except the
  declared class below, where both refuse with a different code or detail.
- Product Python (`core/analysis_runs/compatibility.py`) reads every statement as Rust does: its
  `_source_contract` gives the dispatch entry's header refusal or its route reader's outcome.
- G1 and G2 off the corpus: statements built to raise each model check and each standing finding read the
  same through the verifier and through the Rust reader.
"""
from __future__ import annotations

from copy import deepcopy
import hashlib
import json
import math

import pytest

from core.analysis_runs import compatibility as product, rust_reader
from tests import base_reader_corpus as corpus
from tests.verifiers import reader_verdict as rv
from tests.verifiers.source_blocks import domain_hash

# The declared class: both readers refuse, with a different code or detail. Each reader keeps its own words
# for the same refusal: the Python readers name a generic code with a detail (for example
# `SOURCE_PHYSICS_EVIDENCE_INVALID: namespace`) where Rust names a specific code
# (`SOURCE_PHYSICS_EVIDENCE_SHAPE`), and the verifier's canonical JSON names the unsafe-integer refusal
# without its value. The standing's findings then differ only in that reader's refusal text.
DECLARED_VALIDATE = {
    "preview-physics": {f"tamper/{name}" for name in (
        "V1-station-fraction-above-1", "V2-subdivisions-above-cap", "V3-negative-span-index", "V4-preview-cases-reordered",
        "V5-combination-support-row-not-global", "V6-null-affected-refs", "V7-blocked-modifier-count-nonzero",
        "V8-empty-sif-source-reference", "V9-missing-entity-ref", "V10-negative-intensified-value",
        "V11-support-both-attributed-and-withheld", "A4-N2-basis-ref-extra-key", "A4-N3-diagnostic-without-id",
        "A4-N4-combination-source-result-refs-null", "A4-N5-duplicate-combination-force-magnitude",
        "A4-N6-support-row-without-basis-ref-element-local", "A4-N8-blocked-envelope-without-summary",
        # T4-U4b: the verifier names the new kinds' refusals in its own words.
        "U4B-sigma-plus-changed", "U4B-sigma-minus-missing", "U4B-transverse-shear-negative", "U4B-arc-outside-domain-revived",
        "U4B-arc-station-cover",
        # Codex round 2: the malformed directional records (their transport outcomes agree).
        "U4B-transport-directional-bare", "U4B-transport-station-factor-negative", "U4B-transport-station-location",
        "U4B-transport-station-extra-key", "U4B-transport-maximum-approximation", "U4B-transport-maximum-span-index",
        "U4B-transport-maximum-subdivisions", "U4B-transport-maximum-gap-missing", "U4B-transport-maximum-pipe-empty")},
    # B7b: the load/reference-state statements are read by the pressure-2 reader. Its inherited checks keep their
    # own words; the three cases the ordinary pressure reader now reads (no or a broken group, a closed connector)
    # and the moved connector case join them (as test_load_reference_readers declares).
    "pressure": {f"lr/{name}" for name in (
        "UNKNOWN-case-section-inherited", "UNKNOWN-maximum-coverage-inherited", "SWAP-numerical-evidence-inherited",
        "PROFILE-exact-case-mode-inherited", "INHERITED-maximum-unit", "INHERITED-extrema-gap", "INHERITED-rhs-method",
        "INHERITED-headline", "INHERITED-section-diameter", "INHERITED-region-pressure-rhs",
        "MISSING-load-reference-states", "MISSING-contract-evidence", "CLOSED-connector",
        "U4B-sigma-plus-changed", "U4B-transverse-shear-changed", "U4B-sigma-minus-missing", "U4B-twin-order-swapped",
        "U4B-shear-coverage", "U4B-hoop-changed", "U4B-toroidal-coverage", "U4B-arc-tokens-on-straight", "U4B-arc-record-15th-key",
        "U4B-forged-poisson-warning")} | {"lr-reader/CONNECTOR_NOT_ARRAY"},
    # B7b: one source reader (pressure-source-1) reads the composite ledger cases and the joined statements.
    "pressure-source": {"lrs/NUM-unsafe-integer-in-publication"} | {f"ledger/{name}" for name in (
        "EVIDENCE_IDS", "VALUE_NONFINITE", "ROW_FIELDS", "ORDINARY_REPORT_KIND", "EXACT_SOURCE_REQUIRED",
        "SUPPORT_IDENTITY", "PROJECTION_IDS", "FUNCTIONAL_IDS", "PROJECTION_RESULTS", "ROW_INPUTS", "PROJECTION_ROW",
        "PROJECTION_RELATIVE_BOUND", "DERIVED_SIGNATURE", "SUPPORT_TERM_DUPLICATE", "SUPPORT_RESULT_BIJECTION",
        "SUPPORT_MAGNITUDE_COVERAGE", "SOURCE_FALLBACK_TRIGGER", "EVIDENCE_IDS#empty-id", "PHYSICAL_OBSERVATION_ESCAPE")},
}
# Standing findings that differ only in the route reader's refusal text (the reader above). A statement whose
# evidence object is missing is refused at the route before any reader text reaches its standing.
DECLARED_STANDING = (DECLARED_VALIDATE["preview-physics"] | DECLARED_VALIDATE["pressure"]) - {"lr/MISSING-contract-evidence"}
STANDING_CODES = {"LEGACY_ABSOLUTE_ROUNDING_INTEGRITY_NOT_ASSESSED", rv.SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED, "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN",
                  rv.SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED, rv.LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE,
                  rv.RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE, "NUMERICAL_INTEGRITY_NOT_QUALIFIED",
                  "NUMERICAL_EVIDENCE_ID_AMBIGUOUS", "REQUESTED_NUMERICAL_BASIS_UNAVAILABLE", "NUMERICAL_CASE_COVERAGE_INCOMPLETE",
                  "NUMERICAL_CASE_EVIDENCE_INCOMPLETE", "NUMERICAL_AGGREGATE_CONTRADICTION", "MECHANICS_NOT_SOLVED",
                  "SOURCE_BLOCKS_CASE_UNQUALIFIED", "SOURCE_BLOCKS_PHYSICAL_ROW_INSPECTION_ONLY", "SOURCE_BLOCKS_ENVELOPE_UNQUALIFIED"}


def _route_of(source):
    try:
        return rv.route(source)
    except rv.Refused:
        return None


def inputs():
    return corpus.corpus_inputs(_route_of, domain_hash)


def _strict_json(text):
    """JSON as the Rust readers parse it: no NaN/Infinity tokens and no number beyond binary64 range."""
    def refuse(token):
        raise ValueError(f"non-finite token {token}")
    def finite(token):
        value = float(token)
        if not math.isfinite(value):
            raise ValueError("number out of range")
        return value
    def integer(token):
        if len(token.lstrip("-")) > 309 or not math.isfinite(float(int(token))):
            raise ValueError("number out of range")
        return int(token)
    return json.loads(text, parse_constant=refuse, parse_float=finite, parse_int=integer)


def test_the_corpus_is_the_committed_corpus():
    built, committed = inputs(), corpus.committed()
    # B7b: 543 less the 14 load-reference transport cases retired with the v2 transport reader.
    # T4-U4b (§5.1): seven tampers of the new kinds (529 -> 536); review 2: the twin-order swap (537);
    # review 6: the preview arc withholding revived (538); review 3 and 5: the two arc envelopes and seven tampers (547);
    # Codex round 2: nine malformed directional records (556); W-16 slice b: the four combination
    # corpus entries with their models (560); review r1: the Celsius selection-temperature entry (561).
    assert [item.id for item in built] == list(committed) and len(built) == 561
    for item in built:
        entry = committed[item.id]
        if item.source is None:
            assert set(entry) == {"id", "parse"}, item.id
            continue
        assert set(entry.get("validate", {})) == set(item.readers), item.id
        assert set(entry.get("transport", {})) == set(item.transports), item.id
        assert ("rows" in entry) == item.rows, item.id


def _outcome(item):
    """The verifier's outcome of one corpus statement, in the committed entry's shape."""
    if item.source is None:
        try:
            _strict_json(item.text)
        except (ValueError, OverflowError):
            return {"id": item.id, "parse": "JSON_PARSE_REJECTED"}
        return {"id": item.id, "parse": "ACCEPTED"}
    entry = {"id": item.id, "dispatch": rv.dispatch(item.source, item.model, item.invocation)}
    if item.readers:
        entry["validate"] = {reader: rv.validate(reader, item.source, item.model, item.invocation) for reader in item.readers}
    if item.transports:
        entry["transport"] = {reader: rv.transport(reader, item.source) for reader in item.transports}
    if item.rows:
        rows = [rv.row_semantics(row, item.source) for row in item.source.get("results", [])]
        # The committed digest is the sha256 of the outcomes' JSON text with sorted keys (serde_json's Map);
        # table rows and metadata hold only strings, booleans and null.
        digest = hashlib.sha256(json.dumps(rows, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")).hexdigest()
        entry["rows"] = {"sha256": digest, "signature_ids": [(None if row["signature"] is None else row["signature"]["signature_id"]) if row["ok"] else row["error"]["code"] for row in rows]}
    return entry


def test_the_verifier_gives_every_committed_outcome_but_the_declared_class():
    committed = corpus.committed()
    validate_differences = {reader: set() for reader in DECLARED_VALIDATE}
    standing_differences = set()
    compared = 0
    for item in inputs():
        ours, theirs = _outcome(item), committed[item.id]
        if "parse" in theirs:
            assert ours == theirs, item.id
            continue
        mine, rust = ours["dispatch"], theirs["dispatch"]
        assert {k: mine[k] for k in ("ok", "route", "binding", "error")} == {k: rust[k] for k in ("ok", "route", "binding", "error")}, item.id
        assert {k: v for k, v in mine["standing"].items() if k != "findings"} == {k: v for k, v in rust["standing"].items() if k != "findings"}, item.id
        if mine["standing"]["findings"] != rust["standing"]["findings"]:
            standing_differences.add(item.id)
            a, b = mine["standing"]["findings"], rust["standing"]["findings"]
            assert len(a) == len(b), item.id
            for x, y in zip(a, b):
                assert x == y or (x not in STANDING_CODES and y not in STANDING_CODES), (item.id, x, y)
        for reader in item.readers:
            compared += 1
            if ours["validate"][reader] != theirs["validate"][reader]:
                validate_differences.setdefault(reader, set()).add(item.id)
                assert ours["validate"][reader]["ok"] is False and theirs["validate"][reader]["ok"] is False, (item.id, reader)
        for reader in item.transports:
            compared += 1
            assert ours["transport"][reader] == theirs["transport"][reader], (item.id, reader)
        if item.rows:
            assert ours["rows"] == theirs["rows"], item.id
    # B7b: 580 reader and transport readings of the parsed statements (fewer than before: one source reader);
    # T4-U4b adds the seven tampers' readings (587); review 2, the twin-order swap (588); review 6, the preview
    # arc withholding revived (589); review 3 and 5, the two arc envelopes (validate and transport) and seven
    # tampers (600); Codex round 2, nine malformed directional records read and transported and the revival
    # transported (619); W-16 slice b, the four combination entries read and transported (627);
    # review r1, the Celsius entry (629).
    assert compared == 629
    assert validate_differences == DECLARED_VALIDATE
    assert standing_differences == DECLARED_STANDING


@pytest.mark.parametrize("group", ["results/", "pressure_source/physics_source/", "pressure/load_reference/", "pressure_source/load_reference_source/",
                                   "load_reference_fallback_uz", "tamper/", "lr/", "lrs/", "lr-", "ledger/"])
def test_product_reads_each_statement_as_the_rust_reader(group):
    """Product `_source_contract` (header dispatch, then the route's reader, through the runner) gives the
    committed native reading of every corpus statement: the header refusal or the route reader's outcome
    (`corpus.product_reading`), and its table binding when admitted."""
    committed = corpus.committed()
    checked = 0
    for item in inputs():
        if not item.id.startswith(group):
            continue
        expected = corpus.product_reading(committed[item.id])
        if expected is None:
            continue
        if item.source is None:
            # Python reads the literal as a non-finite float or an unbounded int, which product code cannot hand on.
            source = json.loads(item.text)
        else:
            source = deepcopy(item.source)
        try:
            contract = product._source_contract(source)
            got = "accept"
        except ValueError as error:
            got = corpus.product_refusal_text(error)
        assert got == expected, item.id
        if got == "accept" and committed[item.id]["dispatch"]["binding"] is not None:
            assert {"id": contract[0], "sha256": contract[1]} == committed[item.id]["dispatch"]["binding"], item.id
        checked += 1
    assert checked > 0


# G1 and G2 off the corpus: built statements read through the verifier and through the Rust reader.
def _physics():
    source = corpus.read("fixtures/results/pressure_connected_mechanics_sparse.json")
    model = corpus.model_at("core/product_physics/tests/fixtures/exact_pressure_connected_request.json")
    return source, model


def _cases():
    source, model = _physics()
    lr_source = corpus.read("fixtures/product_preview/pressure/load_reference/connected-sparse_interactive.raw.json")
    lr_model = corpus.read("fixtures/product_preview/pressure/load_reference/connected.request.json")["model"]
    fewer_cases = deepcopy(model)
    fewer_cases["load_cases"] = fewer_cases["load_cases"][1:] or []
    extra_pipe = deepcopy(model)
    extra_pipe["pipe_segments"].append(dict(extra_pipe["pipe_segments"][0], id="pipe:not-solved"))
    foreign_support = deepcopy(model)
    for support in foreign_support["supports"]:
        support["id"] = support["id"] + ":renamed"
    lr_fewer = deepcopy(lr_model)
    lr_fewer["load_cases"] = lr_fewer["load_cases"][1:]
    contradiction = deepcopy(source)
    contradiction["numerical_quality"]["status"] = "sensitive"
    unsolved = deepcopy(source)
    unsolved["status"]["mechanics"] = "MODEL_INCOMPLETE"
    # B4/D1 (b), B7b: the composite pressure-source-1 statement is the source route here.
    blocks = corpus.read("fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json")
    blocks_request = corpus.read("fixtures/product_preview/pressure_source/physics_source/n05.request.json")
    combinations = deepcopy(blocks_request)
    combinations["model"]["combinations"] = [{"id": "combination:invented", "terms": []}]
    # T4-U6 (FINAL-B): G2 binds each case section's D, t, c_m and c_a to the model's section, bit for bit; each one
    # ulp off in the model.
    section = source["contract_evidence"]["exact_cases"][0]["pipe_sections"][0]
    off_by_one_ulp = []
    for key, leaf in (("outside_diameter", "outside_diameter_m"), ("wall_thickness", "wall_thickness_m"),
                      ("mill_tolerance", "mill_tolerance_m"), ("corrosion_allowance", "corrosion_allowance_m")):
        changed = deepcopy(model)
        changed["pipe_segments"][0]["section"][key] = {"value": math.nextafter(section[leaf], math.inf), "unit": "m"}
        off_by_one_ulp.append((f"physics-model-section-{key}", source, changed, None, "SOURCE_PHYSICS_MODEL_SECTION_GEOMETRY"))
    return off_by_one_ulp + [
        ("physics-model-case-coverage", source, fewer_cases, None, "SOURCE_PHYSICS_MODEL_CASE_COVERAGE"),
        ("physics-model-member-coverage", source, extra_pipe, None, "SOURCE_PHYSICS_MODEL_MEMBER_COVERAGE"),
        ("physics-support-binding", source, foreign_support, None, "SOURCE_PHYSICS_SUPPORT_BINDING"),
        # B7b: the load/reference-state statement is read by the pressure reader, whose G2 model checks follow its
        # evidence checks unwrapped (Rust `reader_verdict::validate`'s "pressure" arm), as on the ordinary statement.
        ("pressure-load-state-model-case-coverage", lr_source, lr_fewer, None, "SOURCE_PHYSICS_MODEL_CASE_COVERAGE"),
        ("aggregate-contradiction", contradiction, model, None, None),
        ("unsolved", unsolved, model, None, None),
        ("source-blocks-without-invocation", blocks, blocks_request["model"], None, None),
        ("source-blocks-with-combinations", blocks, combinations["model"], {"request": combinations, "solver_mode": "sparse_interactive"}, None),
    ]


@pytest.mark.parametrize("case", _cases(), ids=lambda case: case[0])
def test_g1_and_g2_read_the_same_through_the_verifier_and_rust(case):
    name, source, model, invocation, model_code = case
    verifier = rv.dispatch(source, model, invocation)
    rust = rust_reader.dispatch(source, model, invocation)
    assert verifier == rust, name
    if model_code is not None:
        reader = rust_reader.ROUTE_READERS[verifier["route"]]
        assert rv.validate(reader, source, model) == {"ok": False, "error": rv.error_json(model_code)}, name
        with pytest.raises(rust_reader.ReaderRefusal) as refused:
            rust_reader.validate(reader, source, model=model)
        assert str(refused.value) == model_code
        assert model_code in rust["standing"]["findings"] and not rust["standing"]["eligible"]
    if name == "aggregate-contradiction":
        assert "NUMERICAL_AGGREGATE_CONTRADICTION" in rust["standing"]["findings"]
    if name == "unsolved":
        assert rust["standing"]["findings"][-1] == "MECHANICS_NOT_SOLVED"
    if name == "source-blocks-without-invocation":
        assert rust["standing"]["findings"] == [rv.SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED]
    if name == "source-blocks-with-combinations":
        assert not rust["standing"]["eligible"]
