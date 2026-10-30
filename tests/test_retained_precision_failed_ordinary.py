"""B4/D1 W1 extension (b7-plan "Failed-ordinary successor rulings"): the independent verifier's
requested-mode basis, admitted blocking diagnostic and closed W2 refusal.

The producer asserts that the fixture is its own published output. Since the table pins follow the
reviewed table bytes (ruling (6)) the full verifier, G0 first, reads it as eligible; the mutation tests
read the later gates directly.
Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081)."""
import json
import math
from copy import deepcopy
from pathlib import Path

import pytest

from verifiers import retained_precision as rp

FIXTURE = Path(__file__).resolve().parents[1] / "fixtures/results/retained_precision_failed_ordinary_cxf.json"
MODES = ("sparse_interactive", "dense_scrutiny")


N06_FIXTURE = FIXTURE.with_name("retained_precision_failed_ordinary_n06.json")


def _cases(path=FIXTURE):
    fixture = json.loads(path.read_text())
    return [(c["successor"], {"request": c["request"], "solver_mode": c["solver_mode"]}) for c in fixture["cases"]]


def _rehash(source):
    body = source["retained_precision"]["body"]
    body["publication_sha256"] = rp._hash("retained_precision_publication_mp_v2", {k: v for k, v in source.items() if k != "retained_precision"})
    source["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body)


def _below_g0(source, invocation):
    source, invocation = deepcopy(source), deepcopy(invocation)
    route = rp.ROUTES[source["producer"]["semantic_contract_id"]]
    return rp._gates_after_g0(source, source["retained_precision"], rp._schema(), invocation, route)


def _refused(source, invocation):
    try:
        _below_g0(source, invocation)
    except rp.RetainedPrecisionError as error:
        return error.gate, error.code
    return "accepted", None


def _mutated(base, invocation, mutate):
    source = deepcopy(base)
    mutate(source)
    _rehash(source)
    return _refused(source, invocation)


def _requested_index(source):
    return next(i for i, r in enumerate(source["results"]) if r["kind"] == rp.REQUESTED_MODE_KIND)


MISMATCH = ("G5", "RETAINED_PRECISION_REQUESTED_MODE_MISMATCH")


def test_fixture_has_both_modes():
    assert [inv["solver_mode"] for _, inv in _cases()] == list(MODES)


@pytest.mark.parametrize("index", range(2))
def test_failed_ordinary_successor_passes_g1_to_g8_and_is_eligible(index):
    source, invocation = _cases()[index]
    assert _below_g0(source, None)["numerical_eligible"] is False
    result = _below_g0(source, invocation)
    assert result["invocation_bound"] and result["numerical_eligible"]


@pytest.mark.parametrize("index", range(2))
def test_failed_ordinary_successor_passes_g0_with_the_current_table_pins(index):
    # Ruling (6): TABLE_HASH follows the reviewed current table bytes, so the full verifier
    # (G0 first) gives the same eligible reading as the gates below G0.
    source, invocation = _cases()[index]
    result = rp.validate_retained_precision(source, invocation)
    assert result["invocation_bound"] and result["numerical_eligible"]
    assert result == _below_g0(source, invocation)


@pytest.mark.parametrize("index", range(2))
def test_requested_mode_mutations_refuse_by_name(index):
    base, invocation = _cases()[index]
    at = _requested_index(base)

    def append_observation(kind, row_id):
        def mutate(s):
            row = deepcopy(s["results"][at]); row["kind"] = kind; row["id"] = row_id
            s["results"].append(row)
        return mutate

    def replace_with_mode_row(s):
        s["results"][at]["kind"] = "linear_solver_mode_basis"; s["results"][at]["id"] = "result:solver-mode:linear-solve-basis"

    def other_mode(s):
        s["results"][at]["value"] = 2.0 if s["results"][at]["value"] == 1 else 1.0

    def basis_text(old, new):
        def mutate(s):
            s["results"][at]["metadata"]["basis"] = s["results"][at]["metadata"]["basis"].replace(old, new)
        return mutate

    def quality(key, value):
        def mutate(s):
            s["numerical_quality"]["cases"][0][key] = value
        return mutate

    def unnamed(s):
        s["numerical_quality"]["cases"][0]["evidence_refs"].remove(rp.REQUESTED_MODE_ID)

    def successful_ordinary(s):
        o = s["retained_precision"]["body"]["ordinary_attempts"][0]
        o["initial"] = {"kind": "report", "report_diagnostic_ref": "diagnostic:numerical-integrity:case", "outcome": "sensitive"}
        o["w2"] = {"kind": "not_triggered"}
        s["numerical_quality"]["cases"][0]["solve_quality"] = "sensitive"

    for label, mutate in (
        ("coexistence", append_observation("linear_solver_mode_basis", "result:solver-mode:linear-solve-basis")),
        ("fabricated observation", replace_with_mode_row),
        ("other mode", other_mode),
        ("owner", basis_text("owner_ref=case:0", "owner_ref=case:1")),
        ("certificate", basis_text("cases[0].selection", "cases[1].selection")),
        ("sensitive token", quality("solve_quality", "sensitive")),
        ("passive token", quality("structural_status", "passive_model_basis")),
        ("basis unnamed", unnamed),
        ("successful ordinary", successful_ordinary),
    ):
        assert _mutated(base, invocation, mutate) == MISMATCH, label
    # E14: a row of the retired DEC-050/053 parity kind has no signature and is refused.
    parity = append_observation("sparse_live_path_dense_parity_relative_delta", "result:sparse-live:dense-parity-relative-delta")
    assert _mutated(base, invocation, parity)[0] != "accepted"
    assert _mutated(base, invocation, quality("solve_quality", "checks_passed"))[0] == "G5"
    other = deepcopy(invocation)
    other["solver_mode"] = "dense_scrutiny" if invocation["solver_mode"] == "sparse_interactive" else "sparse_interactive"
    assert _refused(base, other)[0] == "G8"
    assert _mutated(base, invocation, lambda s: s["results"].pop(at)) == ("G3", "RETAINED_PRECISION_COVERAGE_MISMATCH")


@pytest.mark.parametrize("index", range(2))
def test_blocking_diagnostic_and_w2_mutations_refuse_by_name(index):
    base, invocation = _cases()[index]
    unbound = ("G5", "RETAINED_PRECISION_BLOCKING_DIAGNOSTIC_UNBOUND")
    w2 = ("G5", "RETAINED_PRECISION_W2_FAILURE_MISMATCH")

    def extra_blocking(s):
        s["diagnostics"].append({"id": "diagnostic:invented:blocking", "code": "SOLVER_SYSTEM_BLOCKED", "severity": "blocking",
                                 "message": "invented", "source": "core/product_physics", "affected_refs": ["supports"]})

    def widened(s):
        for d in s["diagnostics"]:
            if d["severity"] == "blocking":
                d["affected_refs"] = ["case", "supports"]

    def untranslated(s):
        s["retained_precision"]["body"]["ordinary_attempts"][0]["w2"]["failure"] = {"tag": "not_engaged"}

    def inner_trigger(s):
        s["retained_precision"]["body"]["ordinary_attempts"][0]["w2"]["failure"]["refusal"]["trigger"]["error"]["detail"] = "another"

    assert _mutated(base, invocation, extra_blocking) == unbound
    # The reverse binding refuses a widened failure diagnostic first.
    assert _mutated(base, invocation, widened) == FAILURE_DIAGNOSTIC
    assert _mutated(base, invocation, untranslated) == w2
    assert _mutated(base, invocation, inner_trigger) == w2


# b7-plan "N06 absorbed-spring certificate extension": N06's successors, certified with the congruence radius
# and one further annular_source correction, read like CX-F's, through G0 with the current table pins.
@pytest.mark.parametrize("index", range(4))
def test_n06_congruence_successor_passes_g1_to_g8_and_is_eligible(index):
    source, invocation = _cases(N06_FIXTURE)[index]
    assert _below_g0(source, None)["numerical_eligible"] is False
    result = _below_g0(source, invocation)
    assert result["invocation_bound"] and result["numerical_eligible"]
    assert rp.validate_retained_precision(source, invocation) == result


@pytest.mark.parametrize("index", range(4))
def test_n06_lane_corrections_and_perturbed_values_refuse(index):
    base, invocation = _cases(N06_FIXTURE)[index]
    lanes = base["retained_precision"]["body"]["product_attempts"][0]["proof"]["lanes"]
    assert [lane["work"]["correction"]["calls"]["value"] for lane in lanes] == [1, 2]

    def calls(lane, value):
        def mutate(s):
            s["retained_precision"]["body"]["product_attempts"][0]["proof"]["lanes"][lane]["work"]["correction"]["calls"] = {"kind": "exact", "value": value}
        return mutate

    work = ("G5", "RETAINED_PRECISION_WORK_MISMATCH")
    assert _mutated(base, invocation, calls(1, 3)) == work
    assert _mutated(base, invocation, calls(0, 2)) == work
    # A certified torque or rotation moved by one ulp no longer matches its projection outcome.
    for kind in ("element_local_torsional_moment", "global_nodal_rotation_x", "global_nodal_rotation_y"):
        at = next((i for i, r in enumerate(base["results"]) if r["kind"] == kind and r["value"] not in (0, None)), None)
        if at is None:
            continue

        def ulp(s, at=at):
            v = s["results"][at]["value"]
            s["results"][at]["value"] = math.nextafter(v, math.copysign(math.inf, v))

        assert _mutated(base, invocation, ulp) == ("G5", "RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH"), kind


@pytest.mark.parametrize("index", range(4))
def test_exact_and_combination_lanes_keep_one_correction(index):
    """The lane-work check on an N06 attempt read as the exact route or as a combination: two annular_source
    calls refuse there, one passes, so nothing else in the check depends on the relabelling. The products pass
    (`_g5_products`) takes the dispatcher's route, as G5 does."""
    base, _ = _cases(N06_FIXTURE)[index]
    body = base["retained_precision"]["body"]
    rows = {c["basis_ref"]["ref_id"]: [r for r in base["results"] if r["basis_ref"]["ref_id"] == c["basis_ref"]["ref_id"]] for c in body["cases"]}

    def lane_work(attempt, route):
        checks = []
        rp._g5_proof_rows(attempt, rows["case"], lambda ok: None, checks.append, route)
        return False not in checks

    def with_calls(attempt, lane, n):
        attempt = deepcopy(attempt)
        attempt["proof"]["lanes"][lane]["work"]["correction"]["calls"] = {"kind": "exact", "value": n}
        return attempt

    a = body["product_attempts"][0]
    combination = deepcopy(a); combination["owner_ref"]["kind"] = "combination"
    assert lane_work(a, rp.PREVIEW_ROUTE)
    assert not lane_work(a, rp.EXACT_ROUTE) and lane_work(with_calls(a, 1, 1), rp.EXACT_ROUTE)
    assert not lane_work(combination, rp.PREVIEW_ROUTE) and lane_work(with_calls(combination, 1, 1), rp.PREVIEW_ROUTE)
    assert not lane_work(a, None)
    assert not lane_work(with_calls(a, 0, 2), rp.PREVIEW_ROUTE)
    # Through the products pass with the route G5 hands it.
    rp._g5_products(deepcopy(body), rows, rp.PREVIEW_ROUTE)
    with pytest.raises(rp.RetainedPrecisionError) as error:
        rp._g5_products(deepcopy(body), rows, rp.EXACT_ROUTE)
    assert (error.value.gate, error.value.code) == ("G5", "RETAINED_PRECISION_WORK_MISMATCH")
    # G5 hands the products pass the dispatched route object itself (not a constant or a flag).
    seen = []
    original = rp._g5_products
    route = rp.PREVIEW_ROUTE._replace()
    try:
        rp._g5_products = lambda b, r, route_arg=None: (seen.append(route_arg), original(b, r, route_arg))[1]
        source, invocation = _cases(N06_FIXTURE)[index]
        rp._gates_after_g0(deepcopy(source), source["retained_precision"], rp._schema(), deepcopy(invocation), route)
    finally:
        rp._g5_products = original
    assert len(seen) == 1 and seen[0] is route
    for route, owner, lane, limit in ((rp.PREVIEW_ROUTE, "case", 1, 2), (rp.PREVIEW_ROUTE, "case", 0, 1), (rp.EXACT_ROUTE, "case", 1, 1),
                                      (rp.PREVIEW_ROUTE, "combination", 1, 1), (None, "case", 1, 1)):
        assert rp._lane_correction_limit(route, {"owner_ref": {"kind": owner}}, lane) == limit


FAILURE_DIAGNOSTIC = ("G5", "RETAINED_PRECISION_FAILURE_DIAGNOSTIC_MISMATCH")
FAILURE_QUALITY = ("G5", "RETAINED_PRECISION_FAILURE_QUALITY_MISMATCH")


def _failure_diagnostic(s):
    ref = s["retained_precision"]["body"]["ordinary_attempts"][0]["w2"]["diagnostic_ref"]
    return next(d for d in s["diagnostics"] if d["id"] == ref)


def _set_diagnostic(**fields):
    def mutate(s):
        _failure_diagnostic(s).update(fields)
    return mutate


@pytest.mark.parametrize("index", range(2))
def test_reclassified_failure_diagnostic_refuses_by_name(index):
    # Ruling (3)'s reverse binding (g2 review F1): the failure diagnostic the attempt names stays blocking,
    # names its one case, keeps the code the failure determines and the producer's message template (the
    # receipt records no message, so no message text is bound beyond it).
    base, invocation = _cases()[index]
    tail = rp.INTEGRITY_MESSAGE_TAIL
    for label, mutate in (
        ("warning", _set_diagnostic(severity="warning")),
        ("info", _set_diagnostic(severity="info")),
        ("checks_passed code", _set_diagnostic(code="NUMERICAL_INTEGRITY_CHECKS_PASSED")),
        ("sensitive code", _set_diagnostic(code="NUMERICAL_INTEGRITY_SENSITIVE")),
        ("another failure code", _set_diagnostic(code="NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED")),
        ("downgraded and recoded", _set_diagnostic(severity="warning", code="NUMERICAL_INTEGRITY_CHECKS_PASSED")),
        ("edited message", _set_diagnostic(message="Load case case: numerical integrity checks passed")),
        ("another case's message", _set_diagnostic(message="Load case supports: range: scaled evaluation outside normal range; "
                                                           "range_scaling: attempted; step1_trigger=x" + tail)),
        ("W2 marker removed", _set_diagnostic(message="Load case case: range: scaled evaluation outside normal range" + tail)),
    ):
        assert _mutated(base, invocation, mutate) == FAILURE_DIAGNOSTIC, label

    def unnamed(s):
        s["retained_precision"]["body"]["ordinary_attempts"][0]["w2"]["diagnostic_ref"] = None
    # A failed W2 that names no failure diagnostic is already the receipt's (G1).
    assert _mutated(base, invocation, unnamed) == ("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH")


@pytest.mark.parametrize("index", range(2))
def test_success_quality_beside_a_failed_ordinary_case_refuses_by_name(index):
    # Rulings (2) and (3) (g2 review F2): the case keeps the bound failure code's outcome tokens and names its
    # diagnostic; the envelope status is never a success token beside it.
    base, invocation = _cases()[index]

    def envelope(status):
        def mutate(s):
            s["numerical_quality"]["status"] = status
        return mutate

    def case(field, token):
        def mutate(s):
            s["numerical_quality"]["cases"][0][field] = token
        return mutate

    def diagnostic_unnamed(s):
        s["numerical_quality"]["cases"][0]["evidence_refs"].remove("diagnostic:numerical-integrity:case")

    for label, mutate in (
        ("envelope checks_passed", envelope("checks_passed")),
        ("envelope sensitive", envelope("sensitive")),
        ("envelope not_assessed", envelope("not_assessed")),
        ("solve failed", case("solve_quality", "failed")),
        ("assembly loss", case("model_matrix_fidelity", "assembly_loss_detected")),
        ("equations retained", case("model_matrix_fidelity", "represented_equations_retained")),
        ("accuracy not claimed", case("accuracy_evidence", "not_claimed")),
        ("mechanism", case("structural_status", "physical_mechanism_witnessed")),
        ("diagnostic unnamed", diagnostic_unnamed),
    ):
        assert _mutated(base, invocation, mutate) == FAILURE_QUALITY, label


def test_initial_failure_code_follows_the_producer_mapping():
    code = rp._initial_failure_code
    assert code({"tag": "range", "detail": "division overflow or underflow"}) == "NUMERICAL_INTEGRITY_UNRESOLVED"
    assert code({"tag": "numerically_unresolved", "detail": "positive diagonal contribution absorbed by assembly"}) == "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED"
    assert code({"tag": "numerically_unresolved", "detail": "pivot"}) == "NUMERICAL_INTEGRITY_UNRESOLVED"
    assert code({"tag": "negative_energy"}) == "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY"
    assert code({"tag": "mechanism"}) == "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM"
    assert code({"tag": "asymmetric"}) == code({"tag": "invalid_input"}) == "NUMERICAL_INTEGRITY_FAILED"
