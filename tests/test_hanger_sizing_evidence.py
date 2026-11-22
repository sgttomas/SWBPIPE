"""DEL-04-11 slice 1d-α (design §8.3): the
independent Python verifier (`tests/verifiers/hanger_sizing_evidence.py`) over the committed
corpus (`fixtures/results/hanger_sizing_*.json`), the closed schema
(`schemas/hanger_sizing_result.schema.json`), the headless `size-hangers` verb's documents, and
the analysis-run builder's actual handling of the document (§8.3.8).

Verdict parity: the corpus states the expected standings once; the Rust reader is held to all
five (`core/analysis/hanger_sizing/tests/publication.rs`), this verifier to `closure`, `binding`
and the arithmetic. All values are invented.
"""
from __future__ import annotations

from copy import deepcopy
import json
import os
from pathlib import Path
import re
import subprocess
import unicodedata

import pytest

from tests.schema_validation import validate_instance
from tests.verifiers import hanger_sizing_evidence as hs

ROOT = Path(__file__).resolve().parents[1]
RESULTS = ROOT / "fixtures/results"
SCHEMA = json.loads((ROOT / "schemas/hanger_sizing_result.schema.json").read_text(encoding="utf-8"))
CORPUS = json.loads((RESULTS / "hanger_sizing_corpus.json").read_text(encoding="utf-8"))["entries"]
DOCUMENTS = ("two_span", "four_span", "four_span_constant", "refused", "forgery", "ex_pressurised", "ex_riser")
PYTHON_STANDINGS = ("closure", "binding", "arithmetic_consistent_with_published_positions")


def load(name):
    return json.loads((RESULTS / name).read_text(encoding="utf-8"))


def mutate(doc, operations):
    """The corpus mutations: replace (an existing member or element), add (a new object member) and remove (an
    existing object member)."""
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
        elif operation["op"] == "remove":
            assert isinstance(parent, dict) and last in parent, operation
            del parent[last]
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
    assert verdict["integrity_bound"] == {"status": "not_verified", "reason": "inverse_norm_bound_not_verified"}


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
    routes = {load(f"hanger_sizing_result_{name}.json")["route"] for name in DOCUMENTS}
    assert routes == {"pressure_free", "exact_pressure"}
    for required in ("mutation-outcome", "mutation-record-id", "mutation-count", "mutation-closed-member",
                     "mutation-fixed-text", "mutation-request-hash", "mutation-route-other"):
        assert required in expected, required
    forgery = expected["self-consistent-forgery"]
    assert forgery["arithmetic_consistent_with_published_positions"] == "passed" and forgery["reproduction"] == "failed"


@pytest.mark.parametrize("name", DOCUMENTS)
def test_the_committed_documents_match_the_closed_schema(name):
    validate_instance(SCHEMA, load(f"hanger_sizing_result_{name}.json"), schema_label="hanger_sizing_result",
                      instance_label=name)


@pytest.mark.parametrize("entry", [e for e in CORPUS if e["expected"]["closure"] == "passed"], ids=lambda e: e["id"])
def test_the_schema_admits_every_closed_corpus_document(entry):
    """Every document the readers find closed is admitted by the schema too, a refused document with a null
    route among them."""
    validate_instance(SCHEMA, mutate(load(entry["document"]), entry["mutations"]), schema_label="hanger_sizing_result",
                      instance_label=entry["id"])


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
    assert tuple(defs["Route"]["enum"]) == hs.ROUTES
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


@pytest.mark.parametrize("name, code", [("two_span", 0), ("four_span", 0), ("refused", 1), ("ex_pressurised", 0),
                                        ("ex_riser", 0)])
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


# --- per-solve disclosures (REQ-003): HANGER_SIZING_DERIVED_SOLVE_INTEGRITY and DISPLACEMENT_CORRECTION_UNAVAILABLE ---
# The committed two-span document (labels S_W, S_C, S_O1, S_O2, V_C, V_O1, V_O2) with its disclosures replaced in
# memory. PRODUCT_WARNING is the product's own message on the forced-failure fixture
# rt_a3_s6_c1_correction_unavailable.json (test-only fault injection evidence).
PRODUCT_WARNING = json.loads((RESULTS / "rt_a3_s6_c1_correction_unavailable.json").read_text(encoding="utf-8"))
PRODUCT_WARNING = next(d["message"] for d in PRODUCT_WARNING["envelope"]["diagnostics"]
                       if d["code"] == "DISPLACEMENT_CORRECTION_UNAVAILABLE")


def _integrity(label, token):
    return {"code": "HANGER_SIZING_DERIVED_SOLVE_INTEGRITY", "names": [label], "message": hs.integrity_message(token)}


def _unavailable(label, message=PRODUCT_WARNING):
    return {"code": "DISPLACEMENT_CORRECTION_UNAVAILABLE", "names": [label], "message": message}


def _state(code):
    return {"code": code, "names": ["support:L"], "message": "invented"}


def _with_disclosures(disclosures):
    doc = load("hanger_sizing_result_two_span.json")
    assert doc["result"]["disclosures"] == []
    doc["result"]["disclosures"] = disclosures
    return doc


ACCEPTED_DISCLOSURES = [
    _state("HANGER_SIZING_SUPPORT_STATE_DIFFERS"),
    _state("HANGER_SIZING_COLD_REFERENCE_STATE_DIFFERS"),
    _integrity("S_W", "not_assessed"),
    _integrity("S_O1", "sensitive"),
    _unavailable("S_O1"),
    _integrity("V_O2", "unresolved"),
    _unavailable("V_O2"),
    _integrity("V_C", "failed"),
]

# The integrity message PR 272 first published, before the bound was stated as not verified.
LEGACY_INTEGRITY_MESSAGE = ("sensitive: the numerical-integrity standing of this derived solve is sensitive, "
                            "not checks_passed; every value sizing takes from it carries that standing")

# (id, disclosures, the verifier's difference, whether the schema can state it)
REFUSED_DISCLOSURES = [
    ("unknown-code", [{"code": "HANGER_SIZING_UNKNOWN", "names": ["S_W"], "message": "x"}], "code: not one of", True),
    ("product-integrity-code", [{"code": "NUMERICAL_INTEGRITY_SENSITIVE", "names": ["S_W"], "message": "x"}],
     "code: not one of", True),
    ("label-not-in-plan", [_integrity("S_O3", "sensitive")], "not a solve label of this document", False),
    ("label-not-in-plan-warning", [_unavailable("V_O3")], "not a solve label of this document", False),
    ("label-malformed", [_integrity("source", "sensitive")], "not a solve label", True),
    ("repeated-integrity-pair", [_integrity("S_O1", "sensitive"), _integrity("S_O1", "failed")],
     "repeated for solve S_O1", False),
    ("repeated-warning-pair", [_unavailable("V_C"), _unavailable("V_C")], "repeated for solve V_C", False),
    ("repeated-state-code", [_state("HANGER_SIZING_SUPPORT_STATE_DIFFERS")] * 2, "code: repeated", False),
    ("two-labels", [{**_integrity("S_W", "sensitive"), "names": ["S_W", "S_C"]}], "not exactly one solve label", True),
    ("no-label", [{**_unavailable("S_W"), "names": []}], "not exactly one solve label", True),
    ("label-not-string", [{**_integrity("S_W", "sensitive"), "names": [7]}], "not a string", True),
    ("checks-passed-token", [_integrity("S_W", "checks_passed")], "not the integrity message", True),
    ("solver-checks-passed-token", [_integrity("S_W", "solver_checks_passed")], "not the integrity message", True),
    ("legacy-checks-passed-message", [{**_integrity("S_W", "sensitive"), "message": LEGACY_INTEGRITY_MESSAGE}],
     "not the integrity message of a standing other than solver_checks_passed", True),
    ("token-case", [_integrity("S_W", "SENSITIVE")], "not the integrity message", True),
    ("token-only", [{**_integrity("S_W", "sensitive"), "message": "sensitive"}], "not the integrity message", True),
    ("warning-empty", [_unavailable("S_W", "")], "message: empty", True),
    ("warning-extra-member", [{**_unavailable("S_W"), "extra": 1}], "not a member of the closed object", True),
]


def test_the_product_warning_is_the_fixtures_and_names_its_component_and_failure_kind():
    assert "node:A:UX" in PRODUCT_WARNING and "could not be certified" in PRODUCT_WARNING


def _solve(doc, label):
    return next(solve for solve in doc["result"]["solves"] if solve["label"] == label)


def _with_standings(disclosures):
    """_with_disclosures, each integrity disclosure's solve standing set to its token."""
    doc = _with_disclosures(disclosures)
    for disclosure in disclosures:
        if disclosure["code"] == "HANGER_SIZING_DERIVED_SOLVE_INTEGRITY":
            _solve(doc, disclosure["names"][0])["solver_standing"] = disclosure["message"].split(":", 1)[0]
    return doc


def test_both_per_solve_codes_on_different_valid_labels_are_closed_and_admitted_by_the_schema():
    doc = _with_standings(deepcopy(ACCEPTED_DISCLOSURES))
    hs.closure(doc)
    validate_instance(SCHEMA, doc, schema_label="hanger_sizing_result", instance_label="per-solve-accepted")


@pytest.mark.parametrize("case", REFUSED_DISCLOSURES, ids=[case[0] for case in REFUSED_DISCLOSURES])
def test_the_verifier_refuses_bad_per_solve_disclosures(case):
    _, disclosures, reason, _ = case
    with pytest.raises(hs.Difference, match=reason.replace("[", r"\[")):
        hs.closure(_with_disclosures(deepcopy(disclosures)))


@pytest.mark.parametrize("case", [c for c in REFUSED_DISCLOSURES if c[3]], ids=lambda c: c[0])
def test_the_schema_refuses_what_it_can_state_of_bad_per_solve_disclosures(case):
    """Membership of a label in this document's solves and (code, label) uniqueness compare sibling members,
    which JSON Schema cannot state; those are the readers' closure (the cases marked False above)."""
    _, disclosures, _, _ = case
    with pytest.raises(AssertionError, match="failed JSON Schema validation"):
        validate_instance(SCHEMA, _with_disclosures(deepcopy(disclosures)), schema_label="hanger_sizing_result",
                          instance_label=case[0])


def test_the_schemas_disclosure_vocabulary_is_the_verifiers():
    defs = SCHEMA["$defs"]
    assert tuple(defs["StateDisclosure"]["properties"]["code"]["enum"]) == hs.DISCLOSURE_CODES
    assert (defs["SolveIntegrityDisclosure"]["properties"]["code"]["const"],
            defs["CorrectionUnavailableDisclosure"]["properties"]["code"]["const"]) == hs.SOLVE_DISCLOSURE_CODES
    assert defs["SolveIntegrityDisclosure"]["properties"]["message"]["enum"] == [
        hs.integrity_message(token) for token in hs.NONPASSING_STANDINGS]
    assert defs["IntegrityBound"] == {
        "type": "object", "description": defs["IntegrityBound"]["description"],
        "properties": {"status": {"const": hs.INTEGRITY_BOUND_STATUS}, "reason": {"const": hs.INTEGRITY_BOUND_REASON}},
        "required": ["status", "reason"], "additionalProperties": False}
    assert defs["Result"]["properties"]["integrity_bound"] == {"$ref": "#/$defs/IntegrityBound"}
    assert "integrity_bound" in defs["Result"]["required"]
    assert defs["SolverStanding"]["enum"] == list(hs.SOLVER_STANDINGS)
    assert defs["SolverStanding"]["enum"][0] == hs.SOLVER_CHECKS_PASSED
    confirmation = defs["SupportStateConfirmation"]
    assert confirmation["properties"]["outcome"]["enum"] == list(hs.CONFIRMATION_OUTCOMES)
    assert confirmation["required"] == ["outcome", "detail"] and confirmation["additionalProperties"] is False
    assert confirmation["if"] == {"properties": {"outcome": {"const": hs.DETAIL_QUOTED}}}
    solve = defs["Solve"]
    assert solve["properties"]["solver_standing"] == {"$ref": "#/$defs/SolverStanding"}
    assert solve["properties"]["support_state_confirmation"] == {"$ref": "#/$defs/SupportStateConfirmation"}
    assert tuple(solve["required"]) == hs.SOLVE_KEYS


# --- integrity_bound (REQ-003): missing, extra, other values and non-objects are refused by both -------------------
MALFORMED_BOUNDS = [
    ("missing", None, "$.result.integrity_bound: missing"),
    ("extra-member", {**hs.INTEGRITY_BOUND, "bound": 1.0}, "$.result.integrity_bound.bound: not a member"),
    ("status-verified", {**hs.INTEGRITY_BOUND, "status": "verified"}, "$.result.integrity_bound.status: not"),
    ("status-missing", {"reason": hs.INTEGRITY_BOUND_REASON}, "$.result.integrity_bound.status: missing"),
    ("reason-invented", {**hs.INTEGRITY_BOUND, "reason": "invented_reason"}, "$.result.integrity_bound.reason: not"),
    ("reason-not-string", {**hs.INTEGRITY_BOUND, "reason": 1}, "$.result.integrity_bound.reason: not"),
    ("string", "not_verified", "$.result.integrity_bound: not an object"),
    ("null", "null", "$.result.integrity_bound: not an object"),
    ("array", [hs.INTEGRITY_BOUND], "$.result.integrity_bound: not an object"),
]


@pytest.mark.parametrize("case", MALFORMED_BOUNDS, ids=[case[0] for case in MALFORMED_BOUNDS])
def test_the_verifier_and_schema_refuse_malformed_integrity_bound(case):
    name, bound, reason = case
    doc = load("hanger_sizing_result_two_span.json")
    assert doc["result"]["integrity_bound"] == hs.INTEGRITY_BOUND
    if name == "missing":
        del doc["result"]["integrity_bound"]
    else:
        doc["result"]["integrity_bound"] = None if bound == "null" else deepcopy(bound)
    with pytest.raises(hs.Difference, match=reason.replace("[", r"\[").replace("$", r"\$")):
        hs.closure(doc)
    assert hs.verify(json.dumps(doc))["closure"]["status"] == "failed"
    with pytest.raises(AssertionError, match="failed JSON Schema validation"):
        validate_instance(SCHEMA, doc, schema_label="hanger_sizing_result", instance_label=name)


# --- each derived solve's solver_standing and support_state_confirmation (REQ-003) --------------------------------
def _sensitive_s_o1(doc):
    doc["result"]["disclosures"] = [_integrity("S_O1", "sensitive")]


# (id, change on the two-span document (S_O1 is solves[2]), the verifier's difference, whether the schema can state it)
REFUSED_SOLVES = [
    ("standing-solver-wire-token", lambda d: _solve(d, "S_O1").update(solver_standing="checks_passed"),
     r"solves\[2\].solver_standing: not one of", True),
    ("standing-verified", lambda d: _solve(d, "S_O1").update(solver_standing="verified"),
     r"solves\[2\].solver_standing: not one of", True),
    ("standing-missing", lambda d: _solve(d, "S_O1").pop("solver_standing"), r"solves\[2\].solver_standing: missing", True),
    ("standing-without-disclosure", lambda d: _solve(d, "S_O1").update(solver_standing="sensitive"),
     "solver_standing: sensitive without its HANGER_SIZING_DERIVED_SOLVE_INTEGRITY disclosure", False),
    ("disclosure-beside-passing", _sensitive_s_o1,
     "the integrity disclosure of S_O1 states sensitive, not its solver_standing solver_checks_passed", False),
    ("disclosure-other-token", lambda d: (_sensitive_s_o1(d), _solve(d, "S_O1").update(solver_standing="failed")),
     "the integrity disclosure of S_O1 states sensitive, not its solver_standing failed", False),
    ("confirmation-missing", lambda d: _solve(d, "S_O1").pop("support_state_confirmation"),
     "support_state_confirmation: missing", True),
    ("confirmation-not-object", lambda d: _solve(d, "S_O1").update(support_state_confirmation="solver_reported_established"),
     "support_state_confirmation: not an object", True),
    ("confirmation-outcome-invented", lambda d: _solve(d, "S_O1")["support_state_confirmation"].update(outcome="verified"),
     "support_state_confirmation.outcome: not one of", True),
    ("confirmation-extra-member", lambda d: _solve(d, "S_O1")["support_state_confirmation"].update(bound=1),
     "support_state_confirmation.bound: not a member", True),
    ("not-applicable-with-states", lambda d: _solve(d, "S_O1")["support_state_confirmation"].update(outcome="not_applicable"),
     "not 'not_applicable' exactly when the solve publishes no nonlinear-support state", True),
    ("established-without-states", lambda d: _solve(d, "S_O1").update(states={}),
     "not 'not_applicable' exactly when the solve publishes no nonlinear-support state", True),
    ("detail-missing", lambda d: _solve(d, "S_O1")["support_state_confirmation"].pop("detail"),
     "support_state_confirmation.detail: missing", True),
    ("detail-spurious", lambda d: _solve(d, "S_O1")["support_state_confirmation"].update(detail="invented"),
     "support_state_confirmation.detail: not null", True),
    ("detail-quoted-null", lambda d: _solve(d, "S_O1").update(
        support_state_confirmation={"outcome": "not_established_detail_quoted", "detail": None}),
     "support_state_confirmation.detail: not a string", True),
    ("detail-quoted-blank", lambda d: _solve(d, "S_O1").update(
        support_state_confirmation={"outcome": "not_established_detail_quoted", "detail": " \t"}),
     "support_state_confirmation.detail: blank", True),
]


def test_every_committed_solve_states_its_standing_and_confirmation():
    for name, outcome in (("two_span", "solver_reported_established"), ("four_span", "not_applicable"),
                          ("four_span_constant", "not_applicable")):
        for solve in load(f"hanger_sizing_result_{name}.json")["result"]["solves"]:
            assert solve["solver_standing"] == hs.SOLVER_CHECKS_PASSED, (name, solve)
            assert solve["support_state_confirmation"] == {"outcome": outcome, "detail": None}, (name, solve)


@pytest.mark.parametrize("case", REFUSED_SOLVES, ids=[case[0] for case in REFUSED_SOLVES])
def test_the_verifier_refuses_a_tampered_solve_standing_or_confirmation(case):
    _, change, reason, _ = case
    doc = load("hanger_sizing_result_two_span.json")
    hs.closure(doc)
    change(doc)
    with pytest.raises(hs.Difference, match=reason):
        hs.closure(doc)


@pytest.mark.parametrize("case", [c for c in REFUSED_SOLVES if c[3]], ids=lambda c: c[0])
def test_the_schema_refuses_what_it_can_state_of_a_tampered_solve(case):
    """The pairing of a solve's standing with its integrity disclosure compares sibling members, which JSON Schema
    cannot state; that is the readers' closure (the cases marked False above)."""
    _, change, _, _ = case
    doc = load("hanger_sizing_result_two_span.json")
    change(doc)
    with pytest.raises(AssertionError, match="failed JSON Schema validation"):
        validate_instance(SCHEMA, doc, schema_label="hanger_sizing_result", instance_label=case[0])


def test_a_consistent_non_passing_solve_and_a_quoted_detail_are_closed_and_admitted():
    doc = load("hanger_sizing_result_two_span.json")
    _sensitive_s_o1(doc)
    _solve(doc, "S_O1")["solver_standing"] = "sensitive"
    _solve(doc, "V_C")["support_state_confirmation"] = {"outcome": "not_established_detail_quoted",
                                                        "detail": "support support:C-lift: margin below its bound"}
    _solve(doc, "V_O1")["states"] = {}
    _solve(doc, "V_O1")["support_state_confirmation"] = {"outcome": "not_applicable", "detail": None}
    hs.closure(doc)
    validate_instance(SCHEMA, doc, schema_label="hanger_sizing_result", instance_label="consistent-solves")


# --- one blank-detail rule in the schema, the verifier, the engine and the desktop --------------------------------
BLANK_DETAILS = ["", "  ", "\u0085", "\ufeff", "\u0085\ufeff"]


def _quoted(detail):
    doc = load("hanger_sizing_result_two_span.json")
    _solve(doc, "S_O1")["support_state_confirmation"] = {"outcome": "not_established_detail_quoted", "detail": detail}
    return doc


def test_the_blank_detail_set_is_the_union_of_the_three_languages_whitespace():
    python = {c for c in range(0x110000) if chr(c).isspace()}
    # ECMA-262 WhiteSpace and LineTerminator: Zs, TAB, VT, FF, ZWNBSP (U+FEFF), LF, CR, LS, PS.
    ecma = {c for c in range(0x110000) if unicodedata.category(chr(c)) == "Zs"} | {
        0x0009, 0x000B, 0x000C, 0xFEFF, 0x000A, 0x000D, 0x2028, 0x2029}
    # Rust char::is_whitespace: the Unicode White_Space property.
    rust = {*range(0x0009, 0x000E), 0x0020, 0x0085, 0x00A0, 0x1680, *range(0x2000, 0x200B), 0x2028, 0x2029, 0x202F,
            0x205F, 0x3000}
    assert hs.BLANK_DETAIL_CODEPOINTS == python | ecma | rust
    assert len(hs.BLANK_DETAIL_CODEPOINTS) == 30


def test_the_schema_detail_pattern_matches_exactly_a_character_outside_the_blank_set():
    pattern = re.compile(SCHEMA["$defs"]["SupportStateConfirmation"]["then"]["properties"]["detail"]["pattern"])
    for c in range(0x110000):
        if 0xD800 <= c <= 0xDFFF:
            continue
        assert (pattern.search(chr(c)) is None) == (c in hs.BLANK_DETAIL_CODEPOINTS), hex(c)


@pytest.mark.parametrize("detail", BLANK_DETAILS, ids=repr)
def test_the_verifier_and_schema_refuse_a_blank_detail(detail):
    assert hs._is_blank_detail(detail)
    with pytest.raises(hs.Difference, match=r"support_state_confirmation.detail: blank"):
        hs.closure(_quoted(detail))
    with pytest.raises(AssertionError, match="failed JSON Schema validation"):
        validate_instance(SCHEMA, _quoted(detail), schema_label="hanger_sizing_result", instance_label=repr(detail))


def test_the_verifier_and_schema_admit_a_non_ascii_graphic_detail():
    assert not hs._is_blank_detail("support:α")
    hs.closure(_quoted("support:α"))
    validate_instance(SCHEMA, _quoted("support:α"), schema_label="hanger_sizing_result", instance_label="non-ascii")
