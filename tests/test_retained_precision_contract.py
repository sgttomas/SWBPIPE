"""Synthetic reader controls plus listed producer-solved bases (07l); no native Current evidence."""
import json
import math
import struct
from copy import deepcopy
from fractions import Fraction
from pathlib import Path

import pytest
from core.analysis_runs import retained_precision as rp

ROOT = Path(__file__).resolve().parents[1]


def exact(word):
    word = int(word, 16)
    exponent, significand = (word >> 52) & 2047, word & ((1 << 52) - 1)
    assert exponent != 2047
    if exponent:
        significand |= 1 << 52
    power = exponent - 1075 if exponent else -1074
    q = Fraction(significand) * (Fraction(2) ** power)
    return -q if word >> 63 else q


def assert_least_upper(result, target):
    assert math.isfinite(result) and result >= 0
    value = exact(rp.bits(result))
    assert value >= target
    if result:
        predecessor = struct.unpack(">d", (int(rp.bits(result), 16) - 1).to_bytes(8, "big"))[0]
        assert exact(rp.bits(predecessor)) < target


def corpus():
    return json.loads((ROOT / "fixtures/results/retained_precision_cases.json").read_text())


def test_small_bounds_against_independent_fraction_oracle():
    for row in corpus()["arithmetic"]["small_bounds"]:
        n, s = rp.from_bits(row["value"]), rp.from_bits(row["scale"])
        result = rp.absolute_bound(n, s)
        assert rp.bits(result) == row["expected"], row["id"]
        target = exact(row["b0"]) + exact(row["rounding"]) + Fraction(1, 1 << 1074)
        assert_least_upper(result, target)


def test_products_against_exact_oracle():
    for row in corpus()["arithmetic"]["products"]:
        a, b = rp.from_bits(row["a"]), rp.from_bits(row["b"])
        target = exact(row["a"]) * exact(row["b"])
        if row["expected"] is None:
            with pytest.raises(ValueError): rp.upward_product(a, b)
        else:
            result = rp.upward_product(a, b)
            assert rp.bits(result) == row["expected"]
            assert_least_upper(result, target)


def test_helper_input_rejection_and_far_separated_tail():
    for value in [math.nan, math.inf, -math.inf, -1.0]:
        with pytest.raises(ValueError): rp.upward_product(value, 1.0)
    assert rp.bits(rp.upward_product(-0.0, 1.0)) == "0000000000000000"
    result = rp.upward_small_sum(1.0, math.ldexp(1.0, -1074))
    assert rp.bits(result) == "3ff0000000000001"
    assert_least_upper(result, Fraction(1) + Fraction(2, 1 << 1074))


def test_all_bound_entrypoints_canonicalize_accepted_zero():
    for value in (0.0, -0.0, 1.0, -1.0):
        for scale in (0.0, -0.0):
            assert rp.bits(rp.absolute_bound(value, scale)) == "0000000000000000"
    for power in (53, 64):
        assert rp.bits(rp._scaled_component(-0.0, power)) == "0000000000000000"
    assert rp.bits(rp.upward_small_sum(-0.0, -0.0)) == "0000000000000001"
    for value in (math.nan, math.inf, -math.inf):
        with pytest.raises(ValueError): rp.absolute_bound(value, 0.0)
        with pytest.raises(ValueError): rp.absolute_bound(0.0, value)

def _apply_edits(value, edits):
    for edit in edits:
        parent = value
        for part in edit["path"][:-1]:
            parent = parent[part]
        key = edit["path"][-1]
        if edit["op"] == "remove":
            del parent[key]
        else:
            parent[key] = deepcopy(edit["value"])


def _rehash_ref(items, ref):
    """07e format rule (RV78-N1): a rehash index is a strict integral value: a JSON number, never a
    boolean, finite, integral, >= 0 and not -0 (0.0 is index 0; 0.5, true and -0.0 are not). A
    reference that is not an index, or does not resolve, is skipped (the reader reports it)."""
    if type(ref) not in (int, float) or not math.isfinite(ref) or ref != int(ref) or ref < 0 or (ref == 0 and math.copysign(1.0, ref) < 0):
        return None
    return items[int(ref)] if int(ref) < len(items) else None


def apply_mutation(base, mutation):
    value = deepcopy(base)
    _apply_edits(value, mutation["edits"])
    if mutation.get("_invocation_digest") is not None:
        value["retained_precision"]["body"]["invocation"]["value"] = mutation["_invocation_digest"]
    # D11: the shared format admits only rehash "all". Snapshot 07: when an entry removes the
    # receipt or its body (settled reading 2, a G0 pin), there is nothing to rehash.
    assert mutation["rehash"] == "all", "the shared format admits only rehash: all"
    receipt = value.get("retained_precision")
    if isinstance(receipt, dict) and isinstance(receipt.get("body"), dict):
        body = receipt["body"]
        for source in body["sources"]:
            preparation = source["preparation"]
            attempt = _rehash_ref(body["product_attempts"], preparation["attempt_ref"]) if preparation is not None else None
            if attempt is not None and all(m["result"]["kind"] == "prepared" for m in attempt["preparation"]["members"]):
                # The shared corpus is on the preview route: DEF-O's H (S-1; the reader's payload has no default).
                preparation["sha256"] = rp._hash("retained_precision_preparation_v1", rp._preparation_payload(attempt, rp.DEFINITION_HASH))
        for case in body["cases"]:
            source = _rehash_ref(body["sources"], case.get("source_ref")) if case["status"] == "selected" else None
            if source is not None:
                case["source_identity_sha256"] = rp._source_hash(source)
        body["publication_sha256"] = rp._hash("retained_precision_publication_mp_v2", {k:v for k,v in value.items() if k != "retained_precision"})
        receipt["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body)
    # D24 (snapshot 07b): optional after_rehash edits are applied literally after rehash "all",
    # with no further hashing (the G1 hash-integrity pins).
    _apply_edits(value, mutation.get("after_rehash") or [])
    return value


def apply_entry(fixture, entry):
    """Shared-corpus entry semantics (SHARED_SNAPSHOT_06C format): (1) apply `edits` to a copy
    of the base source; (2) apply `invocation_edits` (same edit grammar; absent means none) to a
    copy of the base invocation; (3) when any invocation edit exists, set
    retained_precision.body.invocation.value = H(source_blocks_invocation_v1, edited invocation);
    (4) rehash per `rehash`; then validate the edited source against the edited invocation."""
    invocation = deepcopy(fixture["invocation"])
    invocation_edits = entry.get("invocation_edits") or []
    _apply_edits(invocation, invocation_edits)
    digest = rp._hash("source_blocks_invocation_v1", invocation) if invocation_edits else None
    return apply_mutation(fixture["source"], dict(entry, _invocation_digest=digest)), invocation


def test_complete_synthetic_draft_control_is_not_qualification():
    from copy import deepcopy
    for fixture in corpus()["cases"]:
        source, invocation = deepcopy(fixture["source"]), deepcopy(fixture["invocation"])
        result = rp._validate_draft(source, invocation)
        assert result["classifications"] == fixture["expected_classifications"]
        assert result["invocation_bound"]
        # U7 (07i, D-U7-2): the shared expectation; 15 bases are eligible (07l: 13 synthetic plus the two
        # producer-solved L = 0 bases), the two with an unavailable case are not.
        assert result["numerical_eligible"] is fixture["expected"]["numerical_eligible"]
        assert result["standing"] == fixture["expected"]["standing"]
        assert source == fixture["source"] and invocation == fixture["invocation"]
        assert rp._validate_draft(source)["numerical_eligible"] is False
        # D-U6-1: the public entry runs every gate; the flag gates eligibility only.
        assert rp.validate_retained_precision(source, invocation) == result
        assert rp.validate_retained_precision(source)["numerical_eligible"] is False


def _outcome(entry, source, invocation):
    try:
        return ("pass", entry(source, invocation))
    except rp.RetainedPrecisionError as error:
        return ("refuse", error.gate, error.code, error.detail)


NOT_ELIGIBLE = {"invocation_bound": False, "numerical_eligible": False, "standing": "needs_recompute"}


def _corpus_entries():
    """(label, source, invocation, expected eligibility if it passes): a base's
    `expected`, a must-pass entry's `expected_eligibility` (07i), or not eligible
    without an invocation. Mutations all refuse, so they carry None."""
    data = corpus()
    for fixture in data["cases"]:
        yield fixture["id"], deepcopy(fixture["source"]), deepcopy(fixture["invocation"]), fixture["expected"]
        yield fixture["id"] + ":no-invocation", deepcopy(fixture["source"]), None, NOT_ELIGIBLE
    for kind in ("mutations", "must_pass"):
        for entry in data.get(kind, []):
            fixture = next(f for f in data["cases"] if f["id"] == entry["base"])
            source, invocation = apply_entry(fixture, entry)
            yield entry["id"], source, invocation, entry.get("expected_eligibility")


MILESTONE_PINS = {
    "sparse_interactive": ("ac6986b0680e0df9d88c33a5bf4635372fc3b83cbb9080da44e6d32dbdca59dc", [25, 69, 3, 1]),
    "dense_scrutiny": ("6cd1d249e5352aaffbd2b7d7349c74a1d0e0572df35500f66be49c3cad95c9b5", [25, 69, 3, 2]),
}


def _milestone(mode):
    """D-U6-5: byte-identical copies of PP's pinned successor files."""
    import hashlib
    raw = (ROOT / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_bytes()
    assert hashlib.sha256(raw).hexdigest() == MILESTONE_PINS[mode][0]
    return json.loads(raw)


def test_public_entry_equals_the_draft_on_every_corpus_entry():
    """D-U6-1: no input changes its accept/refuse outcome except by reaching the
    gates the public entry previously short-circuited at G0. U7: a passing entry's
    eligibility is its shared expectation (07i)."""
    count = 0
    for label, source, invocation, eligibility in _corpus_entries():
        public = _outcome(rp.validate_retained_precision, deepcopy(source), deepcopy(invocation))
        draft = _outcome(rp._validate_draft, deepcopy(source), deepcopy(invocation))
        assert public == draft, label
        if public[0] == "pass":
            got = {key: public[1][key] for key in ("invocation_bound", "numerical_eligible", "standing")}
            assert got == eligibility, label
        count += 1
    assert count == 2 * len(corpus()["cases"]) + len(corpus()["mutations"]) + len(corpus().get("must_pass", []))


@pytest.mark.parametrize("mode", sorted(MILESTONE_PINS))
def test_public_entry_on_the_real_milestone_receipts(mode):
    doc = _milestone(mode)
    for invocation in (doc["invocation"], None):
        public = rp.validate_retained_precision(deepcopy(doc["source"]), deepcopy(invocation))
        assert public == rp._validate_draft(deepcopy(doc["source"]), deepcopy(invocation))
        # U7: with its actual invocation the solved one-case milestone is eligible; without, not.
        assert public["numerical_eligible"] is (invocation is not None)
        assert public["standing"] == ("eligible" if invocation is not None else "needs_recompute")
        assert public["invocation_bound"] is (invocation is not None)
        counts = [sum(1 for c in public["classifications"] if c["class"] == k) for k in ("relative_verified", "absolute_verified", "input_derived", "non_quantity")]
        assert counts == MILESTONE_PINS[mode][1], (mode, counts)
    edited = deepcopy(doc["source"])
    edited["results"][0]["value"] = 12345.0
    with pytest.raises(rp.RetainedPrecisionError) as error:
        rp.validate_retained_precision(edited, doc["invocation"])
    assert (error.value.gate, error.value.code) == ("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH")



def _expected(mutation):
    """This reader's own expectation: `expected_by_reader.python` when present, else the shared
    `expected` (the 06b format, as Rust and TS read theirs; RV94 N-5, B6)."""
    return mutation.get("expected_by_reader", {}).get("python", mutation["expected"])


@pytest.mark.parametrize("mutation", corpus()["mutations"], ids=lambda x:x["id"])
def test_shared_draft_first_failure_controls(mutation):
    fixture = next(f for f in corpus()["cases"] if f["id"] == mutation["base"])
    source, invocation = apply_entry(fixture, mutation)
    with pytest.raises(rp.RetainedPrecisionError) as error:
        rp._validate_draft(source, invocation)
    assert {"gate":error.value.gate,"code":error.value.code} == _expected(mutation)


def test_old_operational_error_is_retained_independently_of_new_ready():
    """Reader logic only: a receipt shape whose old operational result is refused while
    the new result is ready must still validate with unchanged classifications. No native
    trigger is established for this coefficient_range refusal on ordinary-magnitude
    operands; this is not a native-faithful producer control (retired from the shared
    corpus as 05c)."""
    fixture = corpus()["cases"][0]
    source = apply_mutation(fixture["source"], {"edits":[{
        "path":["retained_precision","body","product_attempts",0,"operational","old",0,"result"],
        "op":"set","value":{"kind":"refused","error":{"kind":"coefficient_range","coefficient":"torsional_stiffness","operation":"mul"}}
    }], "rehash":"all"})
    result = rp._validate_draft(source, fixture["invocation"])
    assert result["classifications"] == fixture["expected_classifications"]
    # U7: the edit touches product_attempts only, so the base's eligibility holds.
    assert fixture["expected"]["numerical_eligible"] is True
    assert result["numerical_eligible"] is True


def test_native_source_encoding_domain_and_load_separation():
    from copy import deepcopy
    source = deepcopy(corpus()["cases"][0]["source"]["retained_precision"]["body"]["sources"][0])
    raw, stiffness = rp._native_source_encoding(source, True), rp._native_source_encoding(source, False)
    assert raw[:10] == b"K4SRC\x01\x02\x00\x00\x00"
    assert stiffness[:10] == b"K4STF\x01\x02\x00\x00\x00"
    assert (len(raw), len(stiffness)) == (552, 188)
    source["nodal_terms"][0]["source_id"] = "load:\u03b1"
    assert rp._native_source_encoding(source, True) != raw
    assert rp._native_source_encoding(source, False) == stiffness


COVERAGE = ["retained_precision", "body", "product_attempts", 0, "proof", "summary_coverage"]
SELECTION = ["retained_precision", "body", "cases", 0, "selection"]
VERIFICATION = ["retained_precision", "body", "cases", 0, "run", "records", 1, "verification"]
ZERO = "0000000000000000"


def _set(path, value):
    return {"path": path, "op": "set", "value": value}


@pytest.mark.parametrize("entry", corpus().get("must_pass", []), ids=lambda x: x["id"])
def test_shared_publicly_consistent_attestations_must_pass(entry):
    """I57 s4/s5: public coverage rules are necessary conditions only. These shared
    rewrites keep every public relation, so readers must accept them; only producer
    custody/replay can catch such attested private flags."""
    fixture = next(f for f in corpus()["cases"] if f["id"] == entry["base"])
    assert entry["expected"] == "pass"
    source, invocation = apply_entry(fixture, entry)
    result = rp._validate_draft(source, invocation)
    # 07n (B1 SC): an admitted rewrite that changes the classes (a case no longer selected, a row added or removed)
    # states its own `expected_classifications`, which the three readers agree on; otherwise the base's.
    assert result["classifications"] == entry.get("expected_classifications", fixture["expected_classifications"])
    # U7 (07i): 18 entries are eligible (07l: 14 plus the four L = 0 entries); the 10 with an unavailable case are not.
    got = {key: result[key] for key in ("invocation_bound", "numerical_eligible", "standing")}
    assert got == entry["expected_eligibility"]


def _layout_index(source, predicate):
    layout = source["retained_precision"]["body"]["sources"][0]["layout"]
    return next(i for i, row in enumerate(layout) if predicate(row))


@pytest.mark.parametrize("change", ["force_row_input_derived", "constrained_displacement_not_input_derived", "nonzero_prescription"])
def test_g5a_rederives_canonical_layout_and_zero_prescription(change):
    """Python-only (not shared corpus): D is rederived from source maps, never trusted."""
    fixture = corpus()["cases"][0]
    source = fixture["source"]
    layout = ["retained_precision", "body", "sources", 0, "layout"]
    if change == "force_row_input_derived":
        index = _layout_index(source, lambda r: r["kind"] == "force" and r["quantity"]["tag"] == "reaction")
        edits = [_set(layout + [index, "input_derived"], True)]
    elif change == "constrained_displacement_not_input_derived":
        index = _layout_index(source, lambda r: r["input_derived"])
        edits = [_set(layout + [index, "input_derived"], False)]
    else:
        edits = [_set(["retained_precision", "body", "sources", 0, "constraints", 0, "value"], "3ff0000000000000")]
    with pytest.raises(rp.RetainedPrecisionError) as error:
        rp._validate_draft(apply_mutation(source, {"edits": edits, "rehash": "all"}), fixture["invocation"])
    assert (error.value.gate, error.value.code) == ("G5a", "RETAINED_PRECISION_SCALE_MISMATCH")


def test_p512_floor_phi_follows_native_rounding():
    """C1 G5b 'same E/e-hat/Phi at p512' (verify.rs `e_hat` through `phi_512`). Python-only until the
    C1b p512 ladder base exists: Phi = fl-up(2^-438 * e-hat), e-hat uncoupled at L=0."""
    assert rp._phi_512(0.0) == 0.0
    assert rp._phi_512(1.0) == math.ldexp(1.0, -438)
    # 2^-1038 * (1 + 2^-52) rounds to 2^-1038 in the subnormal range; nearest * 2^438 is
    # below e-hat, so the native next-up applies.
    assert rp._phi_512(math.ldexp(1.0 + 2.0 ** -52, -600)) == math.ldexp(1.0, -1038) + math.ldexp(1.0, -1074)
    assert rp._e_hat([3.0, 0.0], 0.0) == [3.0, 0.0]
    assert rp._e_hat([3.0, 0.0], 2.0) == [3.0, 6.0]
    assert rp._e_hat([0.0, 8.0], 2.0) == [4.0, 8.0]


def _raises(fn, gate, code):
    with pytest.raises(rp.RetainedPrecisionError) as error:
        fn()
    assert (error.value.gate, error.value.code) == (gate, "RETAINED_PRECISION_" + code)


def _fail_g5(ok, code="ATTEMPT_MISMATCH"):
    rp._need(ok, "G5", code)


def test_schedule_replay_terminal_branches_reader_logic():
    """Python-only reader-logic controls for checklist N8-N11 branches that have no native-faithful
    shared base yet (Ceiling, idle/pre-schedule runs, verification-pass terminal)."""
    selected = deepcopy(corpus()["cases"][0]["source"]["retained_precision"]["body"]["cases"][0]["run"])
    idle = dict(selected, records=[], attempts=[], case_charge=0, invocation_increment=0,
                kernel_terminal={"kind": "unresolved", "reason": {"space": "unresolved", "tag": "budget", "scope": "invocation"}})
    rp._g5_schedule(idle, [], [], _fail_g5)
    _raises(lambda: rp._g5_schedule(dict(idle, kernel_terminal={"kind": "selected", "reason": None}), [], [], _fail_g5), "G5", "ATTEMPT_MISMATCH")
    # N10 (adaptive.rs `solve_cases_projected`): exhaustion gives Budget(invocation) and requires
    # invocation_before >= the invocation limit; a WorkAccounting idle run is never emitted
    # (C1:66-68) and is rejected.
    body = deepcopy(corpus()["cases"][0]["source"]["retained_precision"]["body"])
    entry = dict(idle, origin=dict(idle["origin"], group=None))
    fault = {"kind": "unresolved", "reason": {"space": "unresolved", "tag": "work_accounting", "fault": "overflow"}}
    _raises(lambda: rp._g5_schedule(dict(entry, kernel_terminal=fault), [], [], _fail_g5, body), "G5", "ATTEMPT_MISMATCH")
    _raises(lambda: rp._g5_schedule(entry, [], [], _fail_g5, body), "G5", "ATTEMPT_MISMATCH")
    rp._g5_schedule(dict(entry, invocation_before=body["work"]["invocation_limit"]), [], [], _fail_g5, body)
    # A rejected candidate at p128 must hand its verification to a reused p256 candidate.
    rejected = deepcopy(selected)
    reason = {"space": "attempt", "tag": "stop_rule", "quantity": {"tag": "displacement", "dof": {"node": 1, "component": "UX"}}, "body": 0, "kind": "translation"}
    rejected["attempts"][0]["outcome"] = rejected["records"][0]["outcome"] = {"kind": "rejected", "reason": reason}
    rejected["records"][1]["outcome"] = {"kind": "solved"}
    rejected["kernel_terminal"] = {"kind": "unresolved", "reason": {"space": "unresolved", "tag": "ceiling"}}
    _raises(lambda: rp._g5_schedule(rejected, rejected["records"], rejected["attempts"], _fail_g5), "G5", "ATTEMPT_MISMATCH")
    # A non-escalating verification-pass failure is terminal (no next attempt, non-selected terminal).
    vfail = deepcopy(selected)
    stop = {"space": "attempt", "tag": "stop", "stop": {"space": "stop", "tag": "structure"}}
    vfail["attempts"][0]["outcome"] = vfail["records"][0]["outcome"] = {"kind": "rejected", "reason": {"space": "attempt", "tag": "verification_failed"}}
    vfail["attempts"][0]["verification"] = {"record": 1, "precision": 256, "phase": "failed", "reason": stop}
    vfail["records"][1]["outcome"] = {"kind": "failed", "reason": stop}
    vfail["kernel_terminal"] = {"kind": "refused", "reason": {"space": "refusal", "tag": "structure"}}
    rp._g5_schedule(vfail, vfail["records"], vfail["attempts"], _fail_g5)
    _raises(lambda: rp._g5_schedule(dict(vfail, kernel_terminal={"kind": "selected", "reason": None}), vfail["records"], vfail["attempts"], _fail_g5), "G5", "ATTEMPT_MISMATCH")
    # Ceiling (N8): the reused p512 candidate is rejected and its p1024 verification only solved.
    ladder = deepcopy(next(f for f in corpus()["cases"] if f["id"] == "p512_ladder_synthetic")["source"]["retained_precision"]["body"]["cases"][0]["run"])
    ladder["attempts"][2]["outcome"] = ladder["records"][2]["outcome"] = {"kind": "rejected", "reason": reason}
    ladder["records"][3]["outcome"] = {"kind": "solved"}
    ladder["kernel_terminal"] = {"kind": "unresolved", "reason": {"space": "unresolved", "tag": "ceiling"}}
    rp._g5_schedule(ladder, ladder["records"], ladder["attempts"], _fail_g5)
    _raises(lambda: rp._g5_schedule(dict(ladder, kernel_terminal={"kind": "refused", "reason": {"space": "refusal", "tag": "structure"}}), ladder["records"], ladder["attempts"], _fail_g5), "G5", "ATTEMPT_MISMATCH")


def test_source_decline_relation_reader_logic():
    """Python-only reader-logic control for checklist O5 (no native-faithful source_decline base yet)."""
    fixture = next(f for f in corpus()["cases"] if f["id"] == "two_case_preparation_failure_synthetic")
    body = deepcopy(fixture["source"]["retained_precision"]["body"])
    diags = fixture["source"]["diagnostics"]
    decline = {"input_owner": {"case_index": 1, "case_id": "case:unavailable-row", "material_basis_ref": 0},
               "constructor_counts": {"nodes": 2, "members": 1, "springs": 0, "constraints": 6, "nodal_terms": 6, "stations": 3, "supports": 1, "id_utf8_bytes": 0, "directional_springs": 0},
               "error": {"tag": "no_nodes"}}
    body["cases"][1]["source_decline"] = decline
    quality = fixture["source"]["numerical_quality"]["cases"]
    rp._g5_ordinary(body, body["cases"], diags, quality)
    body["cases"][1]["source_decline"] = dict(decline, input_owner=dict(decline["input_owner"], case_index=0))
    _raises(lambda: rp._g5_ordinary(body, body["cases"], diags, quality), "G5", "ATTEMPT_MISMATCH")


# RV78 PROBES.json (review evidence, record e2f7fe8b34): exact edits on 06d bases; `expected` is RV78's
# contract reading, each confirmed against rulings D1-D16 and checkpoint A.
RV78_PROBES_JSON = r'''[{"id":"R1a_execution_order_swapped","base":"two_case_synthetic","edits":[{"path":["retained_precision","body","work","execution_order"],"op":"set","value":[{"kind":"case","index":1},{"kind":"case","index":0}]}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G3","code":"RETAINED_PRECISION_COVERAGE_MISMATCH"}},{"id":"R1b_run_id_not_position","base":"two_case_synthetic","edits":[{"path":["retained_precision","body","cases",1,"run","id"],"op":"set","value":5},{"path":["retained_precision","body","calls",0,"run_refs"],"op":"set","value":[0,5]}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G3","code":"RETAINED_PRECISION_COVERAGE_MISMATCH"}},{"id":"R2_old_members_reordered","base":"two_body_synthetic","edits":[{"path":["retained_precision","body","product_attempts",0,"operational","old"],"op":"set","value":[{"member":1,"inputs":["0000000000000000","4014000000000000","0000000000000000","3ff0000000000000","4014000000000000","0000000000000000","42474876e8000000","4231ed8ec2000000","3f6c4f3caf32fd23","3ee61aa4872838c0"],"result":{"kind":"ready","length":"3ff0000000000000","axial_stiffness":"41c4990f17e516ad","torsional_stiffness":"4128c47ead23fa80","normalization":["3ff0000000000000","0000000000000000","0000000000000000"]},"work":{"entered":0,"checks":0,"lost":false}},{"member":0,"inputs":["0000000000000000","0000000000000000","0000000000000000","3ff0000000000000","0000000000000000","0000000000000000","42474876e8000000","4231ed8ec2000000","3f6c4f3caf32fd23","3ee61aa4872838c0"],"result":{"kind":"ready","length":"3ff0000000000000","axial_stiffness":"41c4990f17e516ad","torsional_stiffness":"4128c47ead23fa80","normalization":["3ff0000000000000","0000000000000000","0000000000000000"]},"work":{"entered":0,"checks":0,"lost":false}}]}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G3","code":"RETAINED_PRECISION_COVERAGE_MISMATCH"}},{"id":"R3_complete_old_short_of_source","base":"two_body_synthetic","edits":[{"path":["retained_precision","body","product_attempts",0,"operational","old"],"op":"set","value":[{"member":0,"inputs":["0000000000000000","0000000000000000","0000000000000000","3ff0000000000000","0000000000000000","0000000000000000","42474876e8000000","4231ed8ec2000000","3f6c4f3caf32fd23","3ee61aa4872838c0"],"result":{"kind":"ready","length":"3ff0000000000000","axial_stiffness":"41c4990f17e516ad","torsional_stiffness":"4128c47ead23fa80","normalization":["3ff0000000000000","0000000000000000","0000000000000000"]},"work":{"entered":0,"checks":0,"lost":false}}]}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G3","code":"RETAINED_PRECISION_COVERAGE_MISMATCH"}},{"id":"R4_unavailable_source_backref_foreign","base":"two_case_facade_after_certificate_synthetic","edits":[{"path":["retained_precision","body","sources",1,"preparation","attempt_ref"],"op":"set","value":0}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5","code":"RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH"}},{"id":"R5_attempt_and_source_basis_not_ordinary","base":"two_case_two_groups_synthetic","edits":[{"path":["retained_precision","body","product_attempts",1,"material_basis_ref"],"op":"set","value":0},{"path":["retained_precision","body","sources",1,"material_basis_ref"],"op":"set","value":0}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5","code":"RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH"}},{"id":"R6a_native_error_with_selected_run","base":"two_case_facade_after_certificate_synthetic","edits":[{"path":["retained_precision","body","product_attempts",1,"result","error"],"op":"set","value":{"kind":"native","run_ref":1}}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5","code":"RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH"}},{"id":"R8_group_call_out_of_range","base":"two_case_synthetic","edits":[{"path":["retained_precision","body","groups",0,"call"],"op":"set","value":3}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}},{"id":"T1_escalating_failed_verification_pass_entered","base":"verification_failure_skip_synthetic","edits":[{"path":["retained_precision","body","cases",0,"run","records",1],"op":"set","value":{"index":1,"precision":256,"role":"verification","outcome":{"kind":"failed","reason":{"space":"attempt","tag":"stop","stop":{"space":"stop","tag":"condition"}}},"residual_basis":320,"corrections":0,"pivot_margin_min":null,"rcond":null,"residual_worst":null,"gate":null,"work":{"wide_lme":1,"exact_sum_lme":0,"own_lme":1,"shared_lme":4,"stop_rule_lme":0,"verification_lme":1,"verification_shared_lme":0,"own_stages":{"formation":0,"assembly":0,"residual_formation":0,"factor":0,"condition":0,"rhs":0,"solve":0,"refinement":0,"recovery":0,"stop_rule":0,"bounded_gate":0,"scale":0,"estimate":0,"charge":0,"bound":1,"shift":0,"bounded_formation":0,"wide_formation":0,"uc":0},"shared_stages":{"formation":4,"assembly":0,"residual_formation":0,"factor":0,"condition":0,"rhs":0,"solve":0,"refinement":0,"recovery":0,"stop_rule":0,"bounded_gate":0,"scale":0,"estimate":0,"charge":0,"bound":0,"shift":0,"bounded_formation":0,"wide_formation":0,"uc":0},"shared_built_here":true,"verification_shared_built_here":false},"storage":{"pattern_entries":144,"profile_entries":21,"limbs_per_entry":4},"verification":null,"bound_refusals":[],"shared_build_ref":1,"verification_shared_build_ref":null}},{"path":["retained_precision","body","cases",0,"run","attempts",0],"op":"set","value":{"precision":128,"candidate_record":0,"origin":{"kind":"fresh"},"verification":{"record":1,"precision":256,"phase":"failed","reason":{"space":"attempt","tag":"stop","stop":{"space":"stop","tag":"condition"}}},"outcome":{"kind":"rejected","reason":{"space":"attempt","tag":"verification_failed"}},"charges":[{"record":0,"part":"solve_and_verification"},{"record":0,"part":"candidate_stop"},{"record":1,"part":"solve_and_verification"}],"case_charge":9,"invocation_increment":9}},{"path":["retained_precision","body","cases",0,"run","case_charge"],"op":"set","value":37},{"path":["retained_precision","body","cases",0,"run","invocation_increment"],"op":"set","value":37},{"path":["retained_precision","body","cases",0,"run","invocation_after"],"op":"set","value":37},{"path":["retained_precision","body","calls",0,"invocation_after"],"op":"set","value":37},{"path":["retained_precision","body","work","charged"],"op":"set","value":37}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}},{"id":"T2_stop_rule_quantity_other_body","base":"p512_ladder_synthetic","edits":[{"path":["retained_precision","body","cases",0,"run","records",0,"outcome","reason","quantity"],"op":"set","value":{"tag":"displacement","dof":{"node":3,"component":"UX"}}},{"path":["retained_precision","body","cases",0,"run","attempts",0,"outcome","reason","quantity"],"op":"set","value":{"tag":"displacement","dof":{"node":3,"component":"UX"}}}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}},{"id":"T3_candidate_record_with_verification","base":"ordinary_prepared_synthetic","edits":[{"path":["retained_precision","body","cases",0,"run","records",0,"verification"],"op":"set","value":{"resolution":[{"body":0,"force":"426d1a94a2000000","moment":"426d1a94a2000000"}],"theta":[{"body":0,"value":"0000000000000000"}],"bound":[{"body":0,"value":"3ff0000000000000"}],"data_blocks":1,"shift_factorizations":0,"g_max":0,"uc_missing":null,"g_violation":null}}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}},{"id":"T4a_ordinary_diagnostic_ref_duplicate","base":"ordinary_prepared_synthetic","edits":[{"path":["retained_precision","body","ordinary_attempts",0,"diagnostic_refs"],"op":"set","value":["diagnostic:numerical-integrity:case:six-component-load","diagnostic:numerical-integrity:case:six-component-load"]}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}},{"id":"T4b_ordinary_diagnostic_ref_dangling","base":"ordinary_prepared_synthetic","edits":[{"path":["retained_precision","body","ordinary_attempts",0,"diagnostic_refs"],"op":"set","value":["diagnostic:numerical-integrity:case:six-component-load","diagnostic:rv78:absent"]}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}},{"id":"T4c_source_identity_stale_receipt_rehashed","base":"ordinary_prepared_synthetic","edits":[],"invocation_edits":[],"post_rehash_edits":[{"path":["retained_precision","body","cases",0,"source_identity_sha256"],"op":"set","value":"0000000000000000000000000000000000000000000000000000000000000000"}],"expected":{"gate":"G1","code":"RETAINED_PRECISION_RECEIPT_MISMATCH"}},{"id":"T4d_ordinary_dangling_plus_adapter_fault","base":"two_case_facade_after_certificate_synthetic","edits":[{"path":["retained_precision","body","ordinary_attempts",1,"diagnostic_refs"],"op":"set","value":["diagnostic:numerical-integrity:case:unavailable-row","diagnostic:rv78:absent"]},{"path":["retained_precision","body","product_attempts",1,"adapter","fault"],"op":"set","value":{"kind":"overflow","event":"map_write"}}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}},{"id":"R2b_unsourced_old_member_noncontiguous","base":"two_case_preparation_failure_synthetic","edits":[{"path":["retained_precision","body","product_attempts",1,"operational","old",0],"op":"set","value":{"member":1,"inputs":["0000000000000000","0000000000000000","0000000000000000","3ff0000000000000","0000000000000000","0000000000000000","42474876e8000000","4231ed8ec2000000","3f6c4f3caf32fd23","3ee61aa4872838c0"],"result":{"kind":"ready","length":"3ff0000000000000","axial_stiffness":"41c4990f17e516ad","torsional_stiffness":"4128c47ead23fa80","normalization":["3ff0000000000000","0000000000000000","0000000000000000"]},"work":{"entered":0,"checks":0,"lost":false}}}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G3","code":"RETAINED_PRECISION_COVERAGE_MISMATCH"}},{"id":"R3b_complete_old_longer_than_source","base":"two_body_synthetic","edits":[{"path":["retained_precision","body","product_attempts",0,"operational","old"],"op":"set","value":[{"member":0,"inputs":["0000000000000000","0000000000000000","0000000000000000","3ff0000000000000","0000000000000000","0000000000000000","42474876e8000000","4231ed8ec2000000","3f6c4f3caf32fd23","3ee61aa4872838c0"],"result":{"kind":"ready","length":"3ff0000000000000","axial_stiffness":"41c4990f17e516ad","torsional_stiffness":"4128c47ead23fa80","normalization":["3ff0000000000000","0000000000000000","0000000000000000"]},"work":{"entered":0,"checks":0,"lost":false}},{"member":1,"inputs":["0000000000000000","4014000000000000","0000000000000000","3ff0000000000000","4014000000000000","0000000000000000","42474876e8000000","4231ed8ec2000000","3f6c4f3caf32fd23","3ee61aa4872838c0"],"result":{"kind":"ready","length":"3ff0000000000000","axial_stiffness":"41c4990f17e516ad","torsional_stiffness":"4128c47ead23fa80","normalization":["3ff0000000000000","0000000000000000","0000000000000000"]},"work":{"entered":0,"checks":0,"lost":false}},{"member":2,"inputs":["0000000000000000","4014000000000000","0000000000000000","3ff0000000000000","4014000000000000","0000000000000000","42474876e8000000","4231ed8ec2000000","3f6c4f3caf32fd23","3ee61aa4872838c0"],"result":{"kind":"ready","length":"3ff0000000000000","axial_stiffness":"41c4990f17e516ad","torsional_stiffness":"4128c47ead23fa80","normalization":["3ff0000000000000","0000000000000000","0000000000000000"]},"work":{"entered":0,"checks":0,"lost":false}}]}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G3","code":"RETAINED_PRECISION_COVERAGE_MISMATCH"}},{"id":"B_interpolation_target_at_lower_point_equal_point_E","base":"ordinary_prepared_interpolated_material_synthetic","edits":[{"path":["retained_precision","body","material_bases",0,"selector","kelvin"],"op":"set","value":"4072c00000000000"},{"path":["retained_precision","body","material_bases",0,"materials",0,"selection","target_kelvin"],"op":"set","value":"4072c00000000000"}],"invocation_edits":[{"path":["request","model","materials",0,"temperature_points",0,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","materials",0,"temperature_points",1,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","load_cases",0,"modulus_basis_temperature"],"op":"set","value":{"value":300,"unit":"K"}}],"post_rehash_edits":[],"expected":{"gate":"G8","code":"RETAINED_PRECISION_PREPARATION_MISMATCH"}},{"id":"B_interpolation_target_below_range_equal_point_E","base":"ordinary_prepared_interpolated_material_synthetic","edits":[{"path":["retained_precision","body","material_bases",0,"selector","kelvin"],"op":"set","value":"4072b00000000000"},{"path":["retained_precision","body","material_bases",0,"materials",0,"selection","target_kelvin"],"op":"set","value":"4072b00000000000"}],"invocation_edits":[{"path":["request","model","materials",0,"temperature_points",0,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","materials",0,"temperature_points",1,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","load_cases",0,"modulus_basis_temperature"],"op":"set","value":{"value":299,"unit":"K"}}],"post_rehash_edits":[],"expected":{"gate":"G8","code":"RETAINED_PRECISION_PREPARATION_MISMATCH"}},{"id":"B_interpolation_target_at_upper_point_equal_point_E","base":"ordinary_prepared_interpolated_material_synthetic","edits":[{"path":["retained_precision","body","material_bases",0,"selector","kelvin"],"op":"set","value":"4073600000000000"},{"path":["retained_precision","body","material_bases",0,"materials",0,"selection","target_kelvin"],"op":"set","value":"4073600000000000"}],"invocation_edits":[{"path":["request","model","materials",0,"temperature_points",0,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","materials",0,"temperature_points",1,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","load_cases",0,"modulus_basis_temperature"],"op":"set","value":{"value":310,"unit":"K"}}],"post_rehash_edits":[],"expected":{"gate":"G8","code":"RETAINED_PRECISION_PREPARATION_MISMATCH"}},{"id":"B_interpolation_target_above_range_equal_point_E","base":"ordinary_prepared_interpolated_material_synthetic","edits":[{"path":["retained_precision","body","material_bases",0,"selector","kelvin"],"op":"set","value":"4073700000000000"},{"path":["retained_precision","body","material_bases",0,"materials",0,"selection","target_kelvin"],"op":"set","value":"4073700000000000"}],"invocation_edits":[{"path":["request","model","materials",0,"temperature_points",0,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","materials",0,"temperature_points",1,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","load_cases",0,"modulus_basis_temperature"],"op":"set","value":{"value":311,"unit":"K"}}],"post_rehash_edits":[],"expected":{"gate":"G8","code":"RETAINED_PRECISION_PREPARATION_MISMATCH"}},{"id":"B_section_accounting_exact_status","base":"two_case_preparation_failure_synthetic","edits":[{"path":["retained_precision","body","product_attempts",1,"preparation","members"],"op":"set","value":[{"member":0,"old_source":["42474876e8000000","4231ed8ec2000000","3f6c4f3caf32fd23","3ed61aa4872838bf","3ed61aa4872838bf","3ee61aa4872838c0"],"old_facts":["3fbeb851eb851eb8","3f847ae147ae147b","3f6c4f3caf32fd23","3ed61aa4872838bf","3ee61aa4872838c0","3f17066b621f3b1c","3faeb851eb851eb8"],"result":{"kind":"refused","error":{"kind":"accounting"}},"work":{"numeric":{"wide_lme":{"kind":"exact","value":0},"exact_sum_lme":{"kind":"exact","value":0},"entries":[{"kind":"exact","value":2},{"kind":"exact","value":4},{"kind":"exact","value":11},{"kind":"exact","value":2},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0}],"f64_arithmetic":{"kind":"exact","value":0},"sticky_status":"exact"},"initialized_endpoints":{"kind":"exact","value":27},"conversions":{"kind":"exact","value":0},"checks":{"kind":"exact","value":0},"endpoint_assignments":{"kind":"exact","value":0},"layout_bytes":[0,0,0,0,0,0,0,0]},"conversions":[]}]},{"path":["retained_precision","body","product_attempts",1,"result"],"op":"set","value":{"kind":"unavailable","error":{"kind":"preparation","capture":{"kind":"association","detail":"annulus preparation refused"},"section":{"kind":"accounting"}}}},{"path":["retained_precision","body","product_attempts",1,"adapter","fault"],"op":"set","value":null}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5","code":"RETAINED_PRECISION_WORK_MISMATCH"}},{"id":"B_control_equal_point_E_bracketed","base":"ordinary_prepared_interpolated_material_synthetic","edits":[],"invocation_edits":[{"path":["request","model","materials",0,"temperature_points",0,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}},{"path":["request","model","materials",0,"temperature_points",1,"elastic_modulus"],"op":"set","value":{"value":200000000000.0,"unit":"Pa"}}],"post_rehash_edits":[],"expected":"pass"},{"id":"Q1_nested_stop_work_accounting_exact_status","base":"two_case_facade_after_certificate_synthetic","edits":[{"path":["retained_precision","body","product_attempts",1,"result"],"op":"set","value":{"kind":"unavailable","error":{"kind":"proof","cause":{"kind":"numeric","cause":{"kind":"arithmetic","cause":{"space":"stop","tag":"work_accounting","fault":"overflow"}}}}}},{"path":["retained_precision","body","product_attempts",1,"adapter","fault"],"op":"set","value":null},{"path":["retained_precision","body","product_attempts",1,"proof","summary_coverage"],"op":"set","value":null},{"path":["retained_precision","body","product_attempts",1,"stages"],"op":"set","value":{"preparation":"completed","native":"completed","proof_start":"completed","projection":"completed","maxima":"completed","values":"completed","aliases":"completed","certificate":"failed","observables":"not_entered","g5a":"not_entered"}},{"path":["retained_precision","body","product_attempts",1,"proof","checks"],"op":"set","value":{"certificate":{"kind":"failed","error":{"kind":"proof","cause":{"kind":"numeric","cause":{"kind":"arithmetic","cause":{"space":"stop","tag":"work_accounting","fault":"overflow"}}}}},"observables":{"kind":"not_entered"},"g5a":{"kind":"not_entered"}}}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5","code":"RETAINED_PRECISION_WORK_MISMATCH"}},{"id":"Q2_view_work_fault_exact_status","base":"two_case_facade_after_certificate_synthetic","edits":[{"path":["retained_precision","body","product_attempts",1,"result"],"op":"set","value":{"kind":"unavailable","error":{"kind":"proof","cause":{"kind":"native_source","cause":{"kind":"view","issue":{"kind":"work","fault":"overflow"}}}}}},{"path":["retained_precision","body","product_attempts",1,"adapter","fault"],"op":"set","value":null},{"path":["retained_precision","body","product_attempts",1,"proof","summary_coverage"],"op":"set","value":null},{"path":["retained_precision","body","product_attempts",1,"proof","lanes"],"op":"set","value":[{"law":"admitted_k","state":"completed","error":null,"work":{"numeric":{"wide_lme":{"kind":"exact","value":0},"exact_sum_lme":{"kind":"exact","value":0},"entries":[{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0}],"f64_arithmetic":{"kind":"exact","value":0},"sticky_status":"exact"},"point_lme":{"kind":"exact","value":0},"view":{"visits":{"kind":"exact","value":0},"f64_operations":{"kind":"exact","value":0},"prescribed_capacity":0,"data_capacity":0},"correction":{"cast_lme":{"kind":"exact","value":0},"factor_lme":{"kind":"exact","value":0},"visits":{"kind":"exact","value":0},"calls":{"kind":"exact","value":0},"rhs_capacity":0,"output_capacity":0,"converted_capacity":0},"visits":{"kind":"exact","value":0},"member_builds":{"kind":"exact","value":0},"frame_builds":{"kind":"exact","value":0},"b_products":{"kind":"exact","value":0},"d_products":{"kind":"exact","value":0},"h_products":{"kind":"exact","value":0},"capacities":[],"data_capacity":0}},{"law":"annular_source","state":"failed","error":{"kind":"view","issue":{"kind":"work","fault":"overflow"}},"work":{"numeric":{"wide_lme":{"kind":"exact","value":0},"exact_sum_lme":{"kind":"exact","value":0},"entries":[{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0}],"f64_arithmetic":{"kind":"exact","value":0},"sticky_status":"exact"},"point_lme":{"kind":"exact","value":0},"view":{"visits":{"kind":"exact","value":0},"f64_operations":{"kind":"exact","value":0},"prescribed_capacity":0,"data_capacity":0},"correction":{"cast_lme":{"kind":"exact","value":0},"factor_lme":{"kind":"exact","value":0},"visits":{"kind":"exact","value":0},"calls":{"kind":"exact","value":0},"rhs_capacity":0,"output_capacity":0,"converted_capacity":0},"visits":{"kind":"exact","value":0},"member_builds":{"kind":"exact","value":0},"frame_builds":{"kind":"exact","value":0},"b_products":{"kind":"exact","value":0},"d_products":{"kind":"exact","value":0},"h_products":{"kind":"exact","value":0},"capacities":[],"data_capacity":0}}]},{"path":["retained_precision","body","product_attempts",1,"proof","projection_outcomes"],"op":"set","value":[]},{"path":["retained_precision","body","product_attempts",1,"proof","projection_conversions"],"op":"set","value":{"kind":"exact","value":0}},{"path":["retained_precision","body","product_attempts",1,"proof","completion"],"op":"set","value":{"kind":"not_entered"}},{"path":["retained_precision","body","product_attempts",1,"stages"],"op":"set","value":{"preparation":"completed","native":"completed","proof_start":"failed","projection":"not_entered","maxima":"not_entered","values":"not_entered","aliases":"not_entered","certificate":"not_entered","observables":"not_entered","g5a":"not_entered"}},{"path":["retained_precision","body","product_attempts",1,"proof","checks"],"op":"set","value":{"certificate":{"kind":"not_entered"},"observables":{"kind":"not_entered"},"g5a":{"kind":"not_entered"}}}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5","code":"RETAINED_PRECISION_WORK_MISMATCH"}},{"id":"Q4_old_operational_accounting_not_lost","base":"two_case_preparation_failure_synthetic","edits":[{"path":["retained_precision","body","product_attempts",1,"operational","old",0,"result"],"op":"set","value":{"kind":"refused","error":{"kind":"accounting"}}}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5","code":"RETAINED_PRECISION_WORK_MISMATCH"}},{"id":"Q0_control_lane_source_failed_storage","base":"two_case_facade_after_certificate_synthetic","edits":[{"path":["retained_precision","body","product_attempts",1,"result"],"op":"set","value":{"kind":"unavailable","error":{"kind":"proof","cause":{"kind":"native_source","cause":{"kind":"storage"}}}}},{"path":["retained_precision","body","product_attempts",1,"adapter","fault"],"op":"set","value":null},{"path":["retained_precision","body","product_attempts",1,"proof","summary_coverage"],"op":"set","value":null},{"path":["retained_precision","body","product_attempts",1,"proof","lanes"],"op":"set","value":[{"law":"admitted_k","state":"completed","error":null,"work":{"numeric":{"wide_lme":{"kind":"exact","value":0},"exact_sum_lme":{"kind":"exact","value":0},"entries":[{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0}],"f64_arithmetic":{"kind":"exact","value":0},"sticky_status":"exact"},"point_lme":{"kind":"exact","value":0},"view":{"visits":{"kind":"exact","value":0},"f64_operations":{"kind":"exact","value":0},"prescribed_capacity":0,"data_capacity":0},"correction":{"cast_lme":{"kind":"exact","value":0},"factor_lme":{"kind":"exact","value":0},"visits":{"kind":"exact","value":0},"calls":{"kind":"exact","value":0},"rhs_capacity":0,"output_capacity":0,"converted_capacity":0},"visits":{"kind":"exact","value":0},"member_builds":{"kind":"exact","value":0},"frame_builds":{"kind":"exact","value":0},"b_products":{"kind":"exact","value":0},"d_products":{"kind":"exact","value":0},"h_products":{"kind":"exact","value":0},"capacities":[],"data_capacity":0}},{"law":"annular_source","state":"failed","error":{"kind":"storage"},"work":{"numeric":{"wide_lme":{"kind":"exact","value":0},"exact_sum_lme":{"kind":"exact","value":0},"entries":[{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0},{"kind":"exact","value":0}],"f64_arithmetic":{"kind":"exact","value":0},"sticky_status":"exact"},"point_lme":{"kind":"exact","value":0},"view":{"visits":{"kind":"exact","value":0},"f64_operations":{"kind":"exact","value":0},"prescribed_capacity":0,"data_capacity":0},"correction":{"cast_lme":{"kind":"exact","value":0},"factor_lme":{"kind":"exact","value":0},"visits":{"kind":"exact","value":0},"calls":{"kind":"exact","value":0},"rhs_capacity":0,"output_capacity":0,"converted_capacity":0},"visits":{"kind":"exact","value":0},"member_builds":{"kind":"exact","value":0},"frame_builds":{"kind":"exact","value":0},"b_products":{"kind":"exact","value":0},"d_products":{"kind":"exact","value":0},"h_products":{"kind":"exact","value":0},"capacities":[],"data_capacity":0}}]},{"path":["retained_precision","body","product_attempts",1,"proof","projection_outcomes"],"op":"set","value":[]},{"path":["retained_precision","body","product_attempts",1,"proof","projection_conversions"],"op":"set","value":{"kind":"exact","value":0}},{"path":["retained_precision","body","product_attempts",1,"proof","completion"],"op":"set","value":{"kind":"not_entered"}},{"path":["retained_precision","body","product_attempts",1,"stages"],"op":"set","value":{"preparation":"completed","native":"completed","proof_start":"failed","projection":"not_entered","maxima":"not_entered","values":"not_entered","aliases":"not_entered","certificate":"not_entered","observables":"not_entered","g5a":"not_entered"}},{"path":["retained_precision","body","product_attempts",1,"proof","checks"],"op":"set","value":{"certificate":{"kind":"not_entered"},"observables":{"kind":"not_entered"},"g5a":{"kind":"not_entered"}}}],"invocation_edits":[],"post_rehash_edits":[],"expected":"pass"},{"id":"S2_unavailable_source_ref_null_instead_of_absent","base":"two_case_preparation_failure_synthetic","edits":[{"path":["retained_precision","body","cases",1,"source_ref"],"op":"set","value":null}],"invocation_edits":[],"post_rehash_edits":[],"expected":"pass"},{"id":"T4e_selected_case_ordinary_checks_passed","base":"ordinary_prepared_synthetic","edits":[{"path":["numerical_quality","cases",0,"solve_quality"],"op":"set","value":"checks_passed"},{"path":["retained_precision","body","ordinary_attempts",0,"initial","outcome"],"op":"set","value":"checks_passed"}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5","code":"RETAINED_PRECISION_ATTEMPT_MISMATCH"}},{"id":"layout_nonzero_prescription__k4","base":"ordinary_prepared_synthetic","edits":[{"path":["retained_precision","body","sources",0,"constraints",0,"value"],"op":"set","value":"3ff0000000000000"},{"path":["retained_precision","body","sources",0,"kernel_source_sha256"],"op":"set","value":"8cc6c03a650cfb2b94589eaf94440f41bef2a7746d1756e243fcc52d7519ef6b"}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G5a","code":"RETAINED_PRECISION_SCALE_MISMATCH"}},{"id":"maps_member_ends_swapped__k4","base":"ordinary_prepared_synthetic","edits":[{"path":["retained_precision","body","sources",0,"id_maps","members",0,"node_i"],"op":"set","value":1},{"path":["retained_precision","body","sources",0,"id_maps","members",0,"node_j"],"op":"set","value":0},{"path":["retained_precision","body","sources",0,"kernel_source_sha256"],"op":"set","value":"2e7709028c73c501c79beda97411cdf8fb92da167e8d7bfb0afd6f059bf02115"},{"path":["retained_precision","body","sources",0,"stiffness_sha256"],"op":"set","value":"389b7d25360fc930fd1521150be02208d8aae85f8750b836305f998d4691c535"},{"path":["retained_precision","body","groups",0,"stiffness_sha256"],"op":"set","value":"389b7d25360fc930fd1521150be02208d8aae85f8750b836305f998d4691c535"}],"invocation_edits":[],"post_rehash_edits":[],"expected":{"gate":"G8","code":"RETAINED_PRECISION_PREPARATION_MISMATCH"}}]'''


# ---------------------------------------------------------------------------------------------
# Review repair 07, phase B1 (rulings D1-D16, checkpoint A, settled readings): reader-local pins.
# RV78's PROBES.json edits (06d bases) with the ruled expectation; then constructed relations.
# ---------------------------------------------------------------------------------------------
RV78_PROBES = json.loads(RV78_PROBES_JSON)
B = ["retained_precision", "body"]
A1 = B + ["product_attempts", 1]
F_BASE, P_BASE, O_BASE = "two_case_facade_after_certificate_synthetic", "two_case_preparation_failure_synthetic", "ordinary_prepared_synthetic"


def _cases():
    return {f["id"]: f for f in corpus()["cases"]}


def _validate_entry(base, edits, invocation_edits=None, post=None):
    fixture = _cases()[base]
    source, invocation = apply_entry(fixture, {"edits": edits, "invocation_edits": invocation_edits or [], "rehash": "all"})
    if post:
        _apply_edits(source, post)
        body = source["retained_precision"]["body"]
        body["publication_sha256"] = rp._hash("retained_precision_publication_mp_v2", {k: v for k, v in source.items() if k != "retained_precision"})
        source["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body)
    return rp._validate_draft(source, invocation)


def _expect(expected, fn):
    if expected == "pass":
        fn()
    else:
        _raises(fn, expected[0], expected[1])


@pytest.mark.parametrize("probe", RV78_PROBES, ids=lambda p: p["id"])
def test_rv78_probe_relations(probe):
    """D1, D3-D6, D8, D16 and the strict bracket: each RV78 probe at its ruled first failure."""
    exp = probe["expected"]
    _expect(exp if exp == "pass" else (exp["gate"], exp["code"][len("RETAINED_PRECISION_"):]),
            lambda: _validate_entry(probe["base"], probe["edits"], probe["invocation_edits"], probe["post_rehash_edits"]))


def test_g0_union_d2():
    """D2 + settled readings 1-2: thresholds, canonicalization, receipt_version 1 and an absent
    receipt or body are G0 fields; a mistyped producer is a typed G0 refusal (RV79-N2)."""
    def g0(fn):
        with pytest.raises(rp.RetainedPrecisionError) as error:
            fn()
        assert (error.value.gate, error.value.code) == ("G0", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
    g0(lambda: _validate_entry(O_BASE, [_set(B + ["work", "case_limit"], 20_000_000_001)]))
    g0(lambda: _validate_entry(O_BASE, [_set(B + ["work", "invocation_limit"], 59_999_999_999)]))
    g0(lambda: _validate_entry(O_BASE, [_set(B + ["canonicalization"], "rfc8785")]))
    g0(lambda: _validate_entry(O_BASE, [_set(B + ["receipt_version"], 2)]))
    g0(lambda: _validate_entry(O_BASE, [_set(B + ["receipt_version"], True)]))
    fixture = _cases()[O_BASE]
    for drop in (["retained_precision"], B):
        source = deepcopy(fixture["source"])
        parent = source
        for part in drop[:-1]: parent = parent[part]
        del parent[drop[-1]]
        g0(lambda: rp._validate_draft(source, fixture["invocation"]))
    for key, value in (("producer", []), ("formulation_basis", "x")):
        source = deepcopy(fixture["source"]); source[key] = value
        g0(lambda: rp._validate_draft(source, fixture["invocation"]))


def test_numbers_are_values_d25():
    """D25 (D10's integral-float clause corrected): 17.0 and 17 are the same parsed value under
    I-JSON/JCS, so an integral float counter is accepted with the same canonical receipt hash; a
    non-integral one still fails G2."""
    charge = _cases()[O_BASE]["source"]["retained_precision"]["body"]["work"]["charged"]
    expected = _cases()[O_BASE]["expected_classifications"]
    assert _validate_entry(O_BASE, [_set(B + ["work", "charged"], float(charge))])["classifications"] == expected
    _raises(lambda: _validate_entry(O_BASE, [_set(B + ["work", "charged"], charge + 0.5)]), "G2", "ENCODING_MISMATCH")


def test_g5b_section_echo_terms_positive_d18():
    """D18 (D10 corrected): an echoed section term equal to the source's but not positive fails
    G5b SECTION_MISMATCH explicitly, never through the arithmetic fallback."""
    for key in ("area", "section_modulus", "length", "axial_stiffness", "torsional_stiffness"):
        for value in ("0000000000000000", "8000000000000000", "bff0000000000000"):
            _raises(lambda: _validate_entry(O_BASE, [_set(B + ["sources", 0, "section_terms", 0, key], value), _set(B + ["cases", 0, "selection", "section_terms", 0, key], value)]), "G5b", "SECTION_MISMATCH")


def test_class1_attempt_defect_wins_over_native_work_d3():
    """D3 class-1 convention: a native WORK defect found first still yields to an ATTEMPT defect."""
    base = _cases()[O_BASE]["source"]["retained_precision"]["body"]
    _raises(lambda: _validate_entry(O_BASE, [_set(B + ["calls", 0, "invocation_before"], 1), _set(B + ["cases", 0, "run", "records", 0, "corrections"], 4)]), "G5", "ATTEMPT_MISMATCH")
    _raises(lambda: _validate_entry(O_BASE, [_set(B + ["calls", 0, "invocation_before"], 1)]), "G5", "WORK_MISMATCH")
    # D16: a dangling build reference is a WORK defect, still deferred behind an ATTEMPT defect.
    _raises(lambda: _validate_entry(O_BASE, [_set(B + ["cases", 0, "run", "records", 0, "shared_build_ref"], 99)]), "G5", "WORK_MISMATCH")
    _raises(lambda: _validate_entry(O_BASE, [_set(B + ["cases", 0, "run", "records", 0, "shared_build_ref"], 99), _set(B + ["cases", 0, "run", "records", 0, "corrections"], 4)]), "G5", "ATTEMPT_MISMATCH")


def test_class2_order_and_dangling_references_d3_d16():
    """Ordinary checks (class 2, ATTEMPT) precede the C3 WORK list; dangling references take
    their own check's code (class 1 ATTEMPT, class 2 ordinary ATTEMPT, class 2 C3 PRODUCT_ATTEMPT)."""
    fault = {"kind": "overflow", "event": "map_write"}
    _raises(lambda: _validate_entry(F_BASE, [_set(A1 + ["adapter", "fault"], fault), _set(B + ["ordinary_attempts", 1, "diagnostic_refs"], ["diagnostic:missing"])]), "G5", "ATTEMPT_MISMATCH")
    _raises(lambda: _validate_entry(O_BASE, [_set(B + ["cases", 0, "run", "attempts", 0, "candidate_record"], 9)]), "G5", "ATTEMPT_MISMATCH")
    _raises(lambda: _validate_entry(O_BASE, [_set(B + ["product_attempts", 0, "ordinary_attempt_ref"], 7)]), "G5", "PRODUCT_ATTEMPT_MISMATCH")


def test_association_d4():
    """D4a-e (C3:146-148, :165, :167; S06 s1), G5 PRODUCT_ATTEMPT."""
    pa = lambda edits, base=F_BASE: _raises(lambda: _validate_entry(base, edits), "G5", "PRODUCT_ATTEMPT_MISMATCH")
    pa([_set(B + ["sources", 1, "preparation"], None)])                                      # a
    pa([_set(B + ["cases", 1, "reason", "cause", "product_attempt_ref"], 0)])               # c
    run_id = _cases()[F_BASE]["source"]["retained_precision"]["body"]["cases"][1]["run"]["id"]
    pa([_set(A1 + ["run_ref"], None), _set(A1 + ["stages", "native"], "not_entered")])      # e
    pa([_set(A1 + ["result"], {"kind": "unavailable", "error": {"kind": "preparation", "capture": {"kind": "storage", "detail": "prepared vector"}, "section": None}}),
        _set(B + ["cases", 1, "reason", "code"], "source_unavailable"), _set(B + ["cases", 1, "reason", "phase"], "preparation")])  # d: preparation with a Run
    pa([_set(A1 + ["result"], {"kind": "unavailable", "error": {"kind": "native", "run_ref": run_id}})])  # d: native with a selected Run


def test_native_records_d5():
    """D5c (rejected verification_failed needs a failed phase) and D5e (a phantom group)."""
    vf = {"kind": "rejected", "reason": {"space": "attempt", "tag": "verification_failed"}}
    _raises(lambda: _validate_entry("p512_ladder_synthetic", [_set(B + ["cases", 0, "run", "records", 0, "outcome"], vf), _set(B + ["cases", 0, "run", "attempts", 0, "outcome"], vf)]), "G5", "ATTEMPT_MISMATCH")
    groups = deepcopy(_cases()[O_BASE]["source"]["retained_precision"]["body"]["groups"])
    groups.append(dict(groups[0], id=1, call=7))
    _raises(lambda: _validate_entry(O_BASE, [_set(B + ["groups"], groups)]), "G5", "ATTEMPT_MISMATCH")


def test_ordinary_pass_d6():
    """D6b (selected quality domain), D6c (W2 trigger and nonzero exponent), D6d (legacy work_ref)."""
    quality = ["numerical_quality", "cases", 0, "solve_quality"]
    _raises(lambda: _validate_entry(O_BASE, [_set(quality, "not_assessed")]), "G5", "ATTEMPT_MISMATCH")
    fixture = _cases()[O_BASE]
    body = deepcopy(fixture["source"]["retained_precision"]["body"]); diags = fixture["source"]["diagnostics"]
    q = [dict(x, solve_quality="failed") for x in fixture["source"]["numerical_quality"]["cases"]]
    o = body["ordinary_attempts"][0]
    report = o["initial"]["report_diagnostic_ref"]
    o["initial"] = {"kind": "formation_failure", "error": {"tag": "numerical_range", "name": "x"}, "basis_index": 0}
    o["w2"] = {"kind": "published", "trigger": {"tag": "formation", "error": {"tag": "numerical_range", "name": "x"}}, "force_scale_exponent": 3, "report_diagnostic_ref": report}
    rp._g5_ordinary(body, body["cases"], diags, q)
    for change in ({"force_scale_exponent": 0}, {"trigger": {"tag": "formation", "error": {"tag": "numerical_range", "name": "y"}}},
                   {"trigger": {"tag": "evaluation", "error": {"tag": "range", "detail": "x"}}}):
        bad = deepcopy(body); bad["ordinary_attempts"][0]["w2"].update(change)
        _raises(lambda: rp._g5_ordinary(bad, bad["cases"], diags, q), "G5", "ATTEMPT_MISMATCH")
    for work_ref, rows in ((0, []), (0, [{"case_index": 1}])):
        bad = deepcopy(body); bad["ordinary_attempts"][0]["legacy_source"]["work_ref"] = work_ref; bad["legacy_source_work"] = rows
        _raises(lambda: rp._g5_ordinary(bad, bad["cases"], diags, q), "G5", "ATTEMPT_MISMATCH")


def test_g4_retained_diagnostic_names_one_requested_case_d7():
    fixture = _cases()[O_BASE]
    extra = deepcopy(next(d for d in fixture["source"]["diagnostics"] if d["code"] == "RETAINED_PRECISION_SELECTED"))
    extra.update(id="diagnostic:retained:orphan", affected_refs=["case:not-requested"])
    diags = fixture["source"]["diagnostics"] + [extra]
    _raises(lambda: _validate_entry(O_BASE, [_set(["diagnostics"], diags)]), "G4", "DIAGNOSTIC_MISMATCH")


def test_accounting_rules_d8_and_r3_both():
    """D8 R1'-R4 reader logic, including R3' with fault `both` (D13)."""
    attempt = deepcopy(_cases()[F_BASE]["source"]["retained_precision"]["body"]["product_attempts"][1])
    assert rp._accounting_rules(attempt) == (True, True, True, True)
    both = deepcopy(attempt)
    both["result"] = {"kind": "unavailable", "error": {"kind": "proof", "cause": {"kind": "work_accounting", "fault": "both"}}}
    assert rp._accounting_rules(both)[2] is False
    both["proof"]["numeric"]["wide_lme"] = {"kind": "unavailable", "fault": "overflow"}
    assert rp._accounting_rules(both)[2] is False
    both["proof"]["numeric"]["sticky_status"] = "inconsistent"
    assert rp._accounting_rules(both)[2] is True
    lane = deepcopy(attempt)  # a lane cause is bound to its own lane's work, not the proof's
    lane["proof"]["numeric"]["sticky_status"] = "overflow"
    lane["proof"]["lanes"][1]["state"] = "failed"; lane["proof"]["lanes"][1]["error"] = {"kind": "numeric", "cause": {"space": "stop", "tag": "work_accounting", "fault": "overflow"}}
    assert rp._accounting_rules(lane)[2] is False
    lane["proof"]["lanes"][1]["work"]["numeric"]["sticky_status"] = "overflow"
    assert rp._accounting_rules(lane)[2] is True
    op = deepcopy(attempt); op["operational"]["old"][0]["result"] = {"kind": "refused", "error": {"kind": "accounting"}}
    assert rp._accounting_rules(op)[1] is False


def test_theta_zero_on_no_data_body_d13():
    quarter = "3fd0000000000000"
    _raises(lambda: _validate_entry("ordinary_prepared_no_data_synthetic", [_set(B + ["cases", 0, "selection", "theta", 0, "value"], quarter),
            _set(B + ["cases", 0, "run", "records", 1, "verification", "theta", 0, "value"], quarter)]), "G5a", "SCALE_MISMATCH")


def test_ceiling_after_p128_verification_solve_failure_d13():
    """D13: an escalating p128 verification-solve failure skips to p512; a rejected p512 ends on the Ceiling."""
    pivot = {"space": "stop", "tag": "pivot", "global_dof": 6}
    rule = {"space": "attempt", "tag": "stop_rule", "quantity": {"tag": "displacement", "dof": {"node": 1, "component": "UX"}}, "body": 0, "kind": "translation"}
    vf = {"kind": "rejected", "reason": {"space": "attempt", "tag": "verification_failed"}}
    rec = lambda i, p, role, outcome: {"index": i, "precision": p, "role": role, "outcome": outcome, "corrections": 0}
    records = [rec(0, 128, "candidate", vf), rec(1, 256, "verification", {"kind": "failed", "reason": {"space": "attempt", "tag": "stop", "stop": pivot}}),
               rec(2, 512, "candidate", {"kind": "rejected", "reason": rule}), rec(3, 1024, "verification", {"kind": "solved"})]
    attempts = [{"precision": 128, "candidate_record": 0, "origin": {"kind": "fresh"}, "outcome": vf,
                 "verification": {"record": 1, "precision": 256, "phase": "failed", "reason": {"space": "attempt", "tag": "stop", "stop": pivot}}},
                {"precision": 512, "candidate_record": 2, "origin": {"kind": "fresh"}, "outcome": {"kind": "rejected", "reason": rule},
                 "verification": {"record": 3, "precision": 1024, "phase": "completed", "reason": None}}]
    run = {"kernel_terminal": {"kind": "unresolved", "reason": {"space": "unresolved", "tag": "ceiling"}}, "case_charge": 1, "invocation_increment": 1, "origin": {"group": 0}}
    rp._g5_schedule(run, records, attempts, _fail_g5)
    _raises(lambda: rp._g5_schedule(dict(run, kernel_terminal={"kind": "unresolved", "reason": {"space": "unresolved", "tag": "exact_sum_span"}}), records, attempts, _fail_g5), "G5", "ATTEMPT_MISMATCH")
    skip_one = deepcopy(attempts); skip_one[1]["precision"] = 256
    _raises(lambda: rp._g5_schedule(run, records, skip_one, _fail_g5), "G5", "ATTEMPT_MISMATCH")


def test_absolute_bound_small_scale_switch_both_sides_d13():
    """C1:158: S < 2^-988 uses the small-scale sum; S = 2^-988 is RU64(2^-64 S) (kills RV79 M11)."""
    s = 2.0 ** -988
    assert rp.bits(rp.absolute_bound(1.0, s)) == rp.bits(2.0 ** -1052)
    below = math.nextafter(s, 0.0)
    result = rp.absolute_bound(1.0, below)
    assert_least_upper(result, exact(rp.bits(rp.upward_product(below, 2.0 ** -64))) + Fraction(1, 1 << 53) + Fraction(1, 1 << 1074))
    assert result > 2.0 ** -54


def test_body_extent_summation_order():
    """verify.rs extent order ((dx^2 + dy^2) + dz^2) (kills RV79 M22)."""
    d = [float.fromhex("0x1.0000000000000p-15"), float.fromhex("0x1.6666666666666p-15"), float.fromhex("0x1.6666666666666p-29")]
    assert rp._extent([(0.0, 0.0, 0.0), tuple(d)]).hex() == "0x1.b87065db6a113p-15"


def test_rv79_surviving_mutants_m06_m09_m14():
    """M06: a passed G5a requires coverage; M09: a values failure is separate_failure; M14: the
    result error matches the first failed stage (P8/P9). Each on F' with a storage cause."""
    pa = lambda edits: _raises(lambda: _validate_entry(F_BASE, edits), "G5", "PRODUCT_ATTEMPT_MISMATCH")
    stages = dict(_cases()[F_BASE]["source"]["retained_precision"]["body"]["product_attempts"][1]["stages"])
    proof_storage = {"kind": "proof", "cause": {"kind": "storage"}}
    pa([_set(A1 + ["stages"], dict(stages, certificate="failed", observables="completed", g5a="completed")),
        _set(A1 + ["proof", "checks"], {"certificate": {"kind": "failed", "error": proof_storage}, "observables": {"kind": "passed"}, "g5a": {"kind": "passed"}}),
        _set(A1 + ["result"], {"kind": "unavailable", "error": proof_storage}), _set(A1 + ["proof", "summary_coverage"], None)])
    nv = dict(stages, values="failed", aliases="not_entered", certificate="not_entered")
    checks = {"certificate": {"kind": "not_entered"}, "observables": {"kind": "not_entered"}, "g5a": {"kind": "not_entered"}}
    values_failed = {"kind": "unavailable", "error": {"kind": "values", "cause": {"kind": "storage"}, "proof": {"kind": "association", "detail": "PP abandoned prepared draft"}}}
    pa([_set(A1 + ["stages"], nv), _set(A1 + ["proof", "checks"], checks), _set(A1 + ["proof", "summary_coverage"], None),
        _set(A1 + ["proof", "completion"], {"kind": "merged"}), _set(A1 + ["result"], values_failed)])
    pa([_set(A1 + ["stages"], dict(stages, maxima="failed", values="not_entered", aliases="not_entered", certificate="not_entered")),
        _set(A1 + ["proof", "checks"], checks), _set(A1 + ["proof", "summary_coverage"], None), _set(A1 + ["result"], values_failed)])


def test_snapshot_07_counts_and_entry_format():
    """Snapshot 07n (B1 SC) appends to 07m only (`test_snapshot_07n_appends_only_and_states_every_read` pins its
    part); 07m's own slices below are unchanged. Snapshot 07m: 07l plus B6's eight G7 mutations, appended so no existing slice moves, with the N-3
    probe's TS expectation aligned (B6; PLAN decision 11). 07l: 07k (07j plus RV94 N-3's G7 probe) plus
    U8's two producer-solved L = 0 bases and their 8 mutations and 4 must-pass entries (U8-2; 07k: U7
    repair; 07j: U7 slice L, C04; 07i: D-U7-2; 07h: RV90 S1, N1, N2, N4):
    17 cases, 294 mutations, 28 must-pass and the D37 table; only rehash "all" (D11); one expectation
    per entry except the two per-reader G7 entries; each must-pass entry also states its eligibility."""
    c = corpus()
    assert (len(c["cases"]), len(c["mutations"]), len(c["must_pass"])) == (26, 534, 78)
    assert set(c) == {"version", "provenance", "arithmetic", "cases", "mutations", "must_pass", "d37"}
    entries = c["mutations"][:294] + c["must_pass"][:28]
    assert all(e["rehash"] == "all" for e in entries)
    assert all(set(e) <= {"id", "base", "edits", "invocation_edits", "after_rehash", "rehash", "expected", "expected_by_reader", "expected_eligibility"} for e in entries)
    assert all(("expected_eligibility" in e) == (e in c["must_pass"]) for e in entries)
    eligible = lambda items, key: sum(item[key]["numerical_eligible"] for item in items)
    assert (eligible(c["cases"][:17], "expected"), eligible(c["must_pass"][:28], "expected_eligibility")) == (15, 18)
    # 07l (U8-2): the appended slices, on the two producer-solved L = 0 bases, each mode in turn.
    l0 = lambda names: [f"{name}_{mode}" for mode in L0_PINS for name in names]
    assert [e["id"] for e in c["mutations"][278:286]] == l0(["isolated_rotation_stop", "isolated_has_data", "isolated_estimate_coupled"]) + l0(["isolated_translation_rotation_stop"])
    assert [e["id"] for e in c["must_pass"][24:28]] == l0(["isolated_estimate_uncoupled"]) + l0(["isolated_translation_stop"])
    assert {e["base"] for e in c["mutations"][278:286] + c["must_pass"][24:28]} == {f"u8_l0_isolated_node_{mode}" for mode in L0_PINS}
    # 07m (B6): the appended G7 slice, on two synthetic bases.
    assert [e["id"] for e in c["mutations"][286:294]] == B6_07M_IDS
    assert {e["base"] for e in c["mutations"][286:294]} == {O_BASE, P_BASE}
    # 07j (C04): one must-pass entry has a not_required case that passes every gate, eligible.
    statuses = lambda e: [x["status"] for x in apply_entry(next(f for f in c["cases"] if f["id"] == e["base"]), e)[0]["retained_precision"]["body"]["cases"]]
    assert [e["id"] for e in c["must_pass"][:28] if "not_required" in statuses(e)] == ["not_required_second_case_checks_passed"]
    assert [e["id"] for e in entries if "expected_by_reader" in e] == ["g7_maximum_off_enclosure", "g7_not_required_quality_enum_invalid"]
    # RV94 N-5 (B6): the per-reader entries' whole expectation, pinned literally, so a changed shared or
    # per-language value fails here as well as in the reader that reads it (R34, R35).
    g7 = lambda code: {"gate": "G7", "code": code}
    assert {e["id"]: (e["expected"], e["expected_by_reader"]) for e in entries if "expected_by_reader" in e} == {
        "g7_maximum_off_enclosure": (g7("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID"), {"python": g7("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID"),
                                     "typescript": g7("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID"), "rust": g7("SOURCE_PREVIEW_PHYSICS_EXTREMA_BOUNDS")}),
        # PLAN decision 11 (B6): TS aligned, so the three readers share the base readers' code.
        "g7_not_required_quality_enum_invalid": (g7("SOURCE_NUMERICAL_CASE_INVALID"), {"python": g7("SOURCE_NUMERICAL_CASE_INVALID"),
                                                 "typescript": g7("SOURCE_NUMERICAL_CASE_INVALID"), "rust": g7("SOURCE_NUMERICAL_CASE_INVALID")}),
    }
    assert len({e["id"] for e in entries}) == len(entries)


# 07m (B6): the N-3 class at its full width (any case status, any part of the base case rule), then
# the sibling base header classes, each refused at G7 with the base readers' own code (PLAN decision 11).
B6_07M_IDS = ["g7_selected_quality_enum_invalid", "g7_unavailable_quality_enum_invalid", "g7_quality_case_evidence_ref_empty",
              "g7_quality_case_extra_member", "g7_quality_status_invalid", "g7_formulation_limitations_empty",
              "g7_contract_evidence_null", "g7_source_block_recovery_present"]


def _slice_outcomes(start, stop, want):
    """One slice of the shared mutations against this reader's own first failure: each observed
    (gate, code) equals the entry's expectation for Python, and the slice's tally of expectations
    equals the literal `want`, so a dropped, moved or re-expected entry fails here even though the
    per-entry test follows the corpus's own expectation (as Rust's slice tallies)."""
    from collections import Counter
    c = corpus()
    tally = Counter()
    for mutation in c["mutations"][start:stop]:
        fixture = next(f for f in c["cases"] if f["id"] == mutation["base"])
        source, invocation = apply_entry(fixture, mutation)
        with pytest.raises(rp.RetainedPrecisionError) as error:
            rp._validate_draft(source, invocation)
        observed = {"gate": error.value.gate, "code": error.value.code}
        assert observed == _expected(mutation), mutation["id"]
        tally[f'{observed["gate"]} {observed["code"]}'] += 1
    assert dict(tally) == want


def test_snapshot_07k_g7_probe_slice():
    """Mutation 277, RV94 N-3's G7 probe (07k), as its own one-entry slice (B6; I70's item 2)."""
    assert [m["id"] for m in corpus()["mutations"][277:278]] == ["g7_not_required_quality_enum_invalid"]
    _slice_outcomes(277, 278, {"G7 SOURCE_NUMERICAL_CASE_INVALID": 1})


def test_snapshot_07m_g7_header_slice():
    """07m (B6): the eight appended G7 mutations, by this reader's own first failure."""
    assert [m["id"] for m in corpus()["mutations"][286:294]] == B6_07M_IDS
    _slice_outcomes(286, 294, {"G7 SOURCE_NUMERICAL_CASE_INVALID": 4, "G7 SOURCE_NUMERICAL_QUALITY_INVALID": 1,
                               "G7 SOURCE_FORMULATION_BASIS_UNSUPPORTED": 1, "G7 SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED": 1,
                               "G7 SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN": 1})


def test_class2_ordinary_before_association_d17():
    """D17: inside class 2 an ordinary reference (ATTEMPT) precedes C3 association (PRODUCT_ATTEMPT)."""
    dangling = _set(B + ["ordinary_attempts", 1, "diagnostic_refs"], ["diagnostic:missing"])
    no_prep = _set(B + ["sources", 1, "preparation"], None)
    _raises(lambda: _validate_entry(F_BASE, [dangling]), "G5", "ATTEMPT_MISMATCH")
    _raises(lambda: _validate_entry(F_BASE, [no_prep]), "G5", "PRODUCT_ATTEMPT_MISMATCH")
    _raises(lambda: _validate_entry(F_BASE, [dangling, no_prep]), "G5", "ATTEMPT_MISMATCH")


def test_d19_unavailable_attempt_needs_its_cause_and_ready_needs_receipt_failure():
    """D19 (S06 s1), G5 PRODUCT_ATTEMPT class 2: both directions, reader logic beside the shared pins."""
    receipt = {"kind": "receipt_failure", "check": "encoding", "field_path": "x"}
    _raises(lambda: _validate_entry(F_BASE, [_set(B + ["cases", 1, "reason", "cause"], receipt), _set(B + ["cases", 1, "reason", "code"], "receipt_encoding"),
                                             _set(B + ["cases", 1, "reason", "phase"], "receipt")]), "G5", "PRODUCT_ATTEMPT_MISMATCH")
    fixture = _cases()[O_BASE]
    body = deepcopy(fixture["source"]["retained_precision"]["body"])
    rows = {c["basis_ref"]["ref_id"]: [r for r in fixture["source"]["results"] if r["basis_ref"]["ref_id"] == c["basis_ref"]["ref_id"]] for c in body["cases"]}
    case = body["cases"][0]
    for cause, ok in ((receipt, True), ({"kind": "unavailable_precondition", "precondition": "caller", "affected_refs": []}, False)):
        bad = deepcopy(body); c = bad["cases"][0]
        for k in ("method", "selection", "source_identity_sha256"): c.pop(k, None)
        c.update(status="unavailable", reason={"code": "receipt_encoding", "phase": "receipt", "cause": cause}, diagnostic_ref="diagnostic:x")
        if ok:
            rp._g5_products(bad, rows)
        else:
            _raises(lambda: rp._g5_products(bad, rows), "G5", "PRODUCT_ATTEMPT_MISMATCH")


def test_native_run_ref_on_nonselected_run_d30():
    """D30: a native error names the case's own nonselected Run (S06:38); kills the M13-type mutant."""
    fixture = _cases()[F_BASE]
    body = deepcopy(fixture["source"]["retained_precision"]["body"])
    rows = {c["basis_ref"]["ref_id"]: [r for r in fixture["source"]["results"] if r["basis_ref"]["ref_id"] == c["basis_ref"]["ref_id"]] for c in body["cases"]}
    case, a = body["cases"][1], body["product_attempts"][1]
    run_id = case["run"]["id"]
    case["run"]["kernel_terminal"] = {"kind": "unresolved", "reason": {"space": "unresolved", "tag": "ceiling"}}
    case["reason"].update(code="kernel_unresolved", phase="kernel")
    a["proof"] = None
    a["stages"] = {k: ("completed" if k == "preparation" else "failed" if k == "native" else "not_entered") for k in a["stages"]}
    a["result"] = {"kind": "unavailable", "error": {"kind": "native", "run_ref": run_id}}
    rp._g5_products(body, rows)
    for wrong in (run_id + 1, 0 if run_id else 1):
        bad = deepcopy(body); bad["product_attempts"][1]["result"]["error"]["run_ref"] = wrong
        _raises(lambda: rp._g5_products(bad, rows), "G5", "PRODUCT_ATTEMPT_MISMATCH")


def test_integers_by_value_at_every_site_d32():
    """D32 (D25 made uniform): each former host-integer site behaves for an integral float exactly as for int."""
    expected = _cases()[O_BASE]["expected_classifications"]
    zero64 = "0" * 64
    # G0 (receipt_version and the 20B/60B limits)
    assert _validate_entry(O_BASE, [_set(B + ["receipt_version"], 1.0), _set(B + ["work", "case_limit"], 20000000000.0),
                                    _set(B + ["work", "invocation_limit"], 60000000000.0)])["classifications"] == expected
    with pytest.raises(rp.RetainedPrecisionError) as error:
        _validate_entry(O_BASE, [_set(B + ["receipt_version"], 2.0)])
    assert (error.value.gate, error.value.code) == ("G0", "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED")
    # G1 source identity with source_ref 0.0 (RV79-E1) and the preparation hash with attempt_ref 0.0
    _raises(lambda: _validate_entry(O_BASE, [_set(B + ["cases", 0, "source_ref"], 0.0)], post=[_set(B + ["cases", 0, "source_identity_sha256"], zero64)]), "G1", "RECEIPT_MISMATCH")
    def forged_preparation():
        value, invocation = apply_entry(_cases()[O_BASE], {"edits": [_set(B + ["sources", 0, "preparation", "attempt_ref"], 0.0)], "rehash": "all"})
        body = value["retained_precision"]["body"]
        body["sources"][0]["preparation"]["sha256"] = zero64
        body["cases"][0]["source_identity_sha256"] = rp._source_hash(body["sources"][0])
        value["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body)
        return rp._validate_draft(value, invocation)
    _raises(forged_preparation, "G1", "RECEIPT_MISMATCH")
    # G3 complete old ids against the source map, and the coverage roster, with source_ref 0.0
    _raises(lambda: _validate_entry("two_body_synthetic", [_set(B + ["sources", 0, "id_maps", "members", 1, "kernel_member"], 5),
                                                         _set(B + ["product_attempts", 0, "source_ref"], 0.0)]), "G3", "COVERAGE_MISMATCH")
    _raises(lambda: _validate_entry(O_BASE, [_set(B + ["product_attempts", 0, "proof", "summary_coverage"], [{"body": 1, "stop": [True, True, True, True], "has_data": True}]),
                                             _set(B + ["product_attempts", 0, "source_ref"], 0.0)]), "G3", "COVERAGE_MISMATCH")


def test_model_schema_versions_d31():
    """D31: G8 admits model schema_version 0.1.0 and 0.2.0; 0.4.0 stays excluded. B3D-10, as ruled: 0.3.0 without a
    contract is refused. B3a is dropped, so 0.3.0 with the retired legacy_pressure_v1 contract is refused too."""
    expected = _cases()[O_BASE]["expected_classifications"]
    version_edit = lambda version: {"path": ["request", "model", "schema_version"], "op": "set", "value": version}
    legacy = {"path": ["request", "model", "pressure_contract"], "op": "set", "value": {"version": "1.0.0", "mode": "legacy_pressure_v1"}}
    for version in ("0.1.0", "0.2.0"):
        assert _validate_entry(O_BASE, [], [version_edit(version)])["classifications"] == expected
    _raises(lambda: _validate_entry(O_BASE, [], [version_edit("0.3.0"), legacy]), "G8", "INVOCATION_MISMATCH")
    _raises(lambda: _validate_entry(O_BASE, [], [version_edit("0.3.0")]), "G8", "INVOCATION_MISMATCH")
    _raises(lambda: _validate_entry(O_BASE, [], [version_edit("0.4.0")]), "G8", "INVOCATION_MISMATCH")


def test_verification_estimate_names_force_or_moment_d33():
    """D33 (verify.rs `verify_state`): an estimate rejection naming a translation or rotation row fails G5 ATTEMPT."""
    reason = {"space": "attempt", "tag": "verification_estimate", "quantity": {"tag": "displacement", "dof": {"node": 1, "component": "UX"}}, "body": 0, "kind": "translation"}
    run = B + ["cases", 0, "run"]
    _raises(lambda: _validate_entry("p512_ladder_synthetic", [_set(run + ["records", 0, "outcome", "reason"], reason), _set(run + ["attempts", 0, "outcome", "reason"], reason)]), "G5", "ATTEMPT_MISMATCH")


def test_negative_zero_anywhere_in_the_receipt_d34():
    """D34 (C1 s4; C1 G2 row): -0 fails G2 ENCODING anywhere in the receipt, including the enum/const
    integer fields G5aError.quantity_kind and source_decline.constructor_counts.directional_springs."""
    sanity = {"kind": "unavailable", "error": {"kind": "g5a", "cause": {"kind": "sanity", "body": 0, "quantity_kind": -0.0}}}
    _raises(lambda: _validate_entry(F_BASE, [_set(A1 + ["result"], sanity)]), "G2", "ENCODING_MISMATCH")
    decline = {"input_owner": {"case_index": 1, "case_id": "case:unavailable-row", "material_basis_ref": 0},
               "constructor_counts": {"nodes": 0, "members": 1, "springs": 0, "constraints": 6, "nodal_terms": 6, "stations": 3, "supports": 1, "id_utf8_bytes": 0, "directional_springs": -0.0},
               "error": {"tag": "no_nodes"}}
    _raises(lambda: _validate_entry(P_BASE, [_set(B + ["cases", 1, "source_decline"], decline)]), "G2", "ENCODING_MISMATCH")
    _raises(lambda: _validate_entry(O_BASE, [_set(B + ["cases", 0, "run", "records", 0, "corrections"], -0.0)]), "G2", "ENCODING_MISMATCH")


def test_rehash_index_rule_07e():
    """RV78-N1: the harness indexes only strict integral values and skips everything else."""
    items = ["a", "b"]
    assert [_rehash_ref(items, r) for r in (0, 1, 1.0, 0.0)] == ["a", "b", "b", "a"]
    assert [_rehash_ref(items, r) for r in (True, False, 0.5, -0.0, -1, 2, float("nan"), None, "0")] == [None] * 9


def test_error_kind_agrees_with_stage_record_d37():
    """D37 (D35 widened), G5 PRODUCT_ATTEMPT class 3: every error kind against every well-formed stage
    record, in both directions. RV79-N1 (snapshot 07g): the expected table is the corpus's `d37`,
    derived from the native sequence and C3, never from this reader's own table."""
    table = corpus()["d37"]
    marks = table["marks"]
    expected = {kind: {tuple(marks[m] for m in r) for r in records} for kind, records in table["kinds"].items()}
    universe = [tuple(marks[m] for m in r) for r in table["records"]]
    assert table["stage_order"] == list(rp.STAGE_ORDER) and len(universe) == len(set(universe)) == 25
    assert all(record in universe for allowed in expected.values() for record in allowed)
    fail_pa = lambda ok: rp._need(ok, "G5", "PRODUCT_ATTEMPT_MISMATCH")
    base = deepcopy(_cases()[F_BASE]["source"]["retained_precision"]["body"]["product_attempts"][1])
    misses = []
    for kind in list(expected) + table["unknown_kinds"]:
        for record in universe:
            a = deepcopy(base); a["proof"] = None
            a["stages"] = dict(zip(rp.STAGE_ORDER, record)); a["result"] = {"kind": "unavailable", "error": {"kind": kind}}
            try:
                rp._g5_typed(a, fail_pa); accepted = True
            except rp.RetainedPrecisionError as error:
                assert (error.gate, error.code) == ("G5", "RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH"); accepted = False
            if accepted != (record in expected.get(kind, set())):
                misses.append((kind, record, accepted))
    assert not misses, misses


def test_error_kind_agrees_with_stage_record_d37_reader_table_pins():
    """07f's direct pins on the reader's own table, kept beside the corpus comparison."""
    C, F, N = "completed", "failed", "not_entered"
    records = rp.ERROR_STAGE_RECORDS
    assert records["g5a"] == {tuple([C] * 9 + [F])}
    assert records["observable"] == {tuple([C] * 8 + [F, C]), tuple([C] * 8 + [F, F])}
    assert records["numeric"] == {tuple([C] * 10)} and records["preparation"] == {tuple([F] + [N] * 9)}
    assert tuple([C] * 8 + [N, N]) in records["capture"] and tuple([C] * 8 + [N, N]) not in records["proof"]
    fail_pa = lambda ok: rp._need(ok, "G5", "PRODUCT_ATTEMPT_MISMATCH")
    base = deepcopy(_cases()[F_BASE]["source"]["retained_precision"]["body"]["product_attempts"][1])
    every = set().union(*records.values())
    for kind, allowed in records.items():
        for record in every:
            a = deepcopy(base); a["proof"] = None
            a["stages"] = dict(zip(rp.STAGE_ORDER, record)); a["result"] = {"kind": "unavailable", "error": {"kind": kind}}
            if record in allowed:
                rp._g5_typed(a, fail_pa)
            else:
                _raises(lambda: rp._g5_typed(a, fail_pa), "G5", "PRODUCT_ATTEMPT_MISMATCH")


def _resealed(doc, edits):
    """A real milestone receipt edited and resealed like a corpus entry (rehash "all")."""
    return apply_entry({"source": doc["source"], "invocation": doc["invocation"]}, {"id": "u6e", "edits": edits, "rehash": "all"})


@pytest.mark.parametrize("mode", sorted(MILESTONE_PINS))
def test_f5_kills_u1_m09_m10_m20_on_the_real_milestone_receipts(mode):
    """F5 (D-U6-7; A2): the readers enforce the exact ordinary list, so U1's producer mutants M09
    (RETAINED_PRECISION_* listed), M10 (diagnostics that do not name the case listed) and M20
    (another row method token) are refused on the real receipt, not only by PP's committed bytes."""
    doc = _milestone(mode)
    source = doc["source"]
    case = source["retained_precision"]["body"]["cases"][0]["basis_ref"]["ref_id"]
    refs = source["retained_precision"]["body"]["ordinary_attempts"][0]["diagnostic_refs"]
    names = lambda d: isinstance(d.get("affected_refs"), list) and case in d["affected_refs"]
    assert refs == [d["id"] for d in source["diagnostics"] if names(d) and not d["code"].startswith("RETAINED_PRECISION_")]
    path = ["retained_precision", "body", "ordinary_attempts", 0, "diagnostic_refs"]
    m09 = [d["id"] for d in source["diagnostics"] if names(d)]
    m10 = [d["id"] for d in source["diagnostics"] if not d["code"].startswith("RETAINED_PRECISION_")]
    assert m09 != refs and m10 != refs, "the mutants differ from the exact list on this receipt"
    for edits, want in [([{"path": path, "op": "set", "value": m09}], ("G5", "RETAINED_PRECISION_ATTEMPT_MISMATCH")),
                        ([{"path": path, "op": "set", "value": m10}], ("G5", "RETAINED_PRECISION_ATTEMPT_MISMATCH")),
                        ([{"path": ["results", 0, "recovery_method"], "op": "set", "value": "other"}], ("G6", "RETAINED_PRECISION_ROW_METHOD_MISMATCH"))]:
        edited, invocation = _resealed(doc, edits)
        with pytest.raises(rp.RetainedPrecisionError) as error:
            rp._validate_draft(edited, invocation)
        assert (error.value.gate, error.value.code) == want, edits
    resealed, invocation = _resealed(doc, [])
    assert rp._validate_draft(resealed, invocation)["classifications"] == rp._validate_draft(deepcopy(source), deepcopy(doc["invocation"]))["classifications"]


@pytest.mark.parametrize("value", ["case:six-component-load", 5, {"case:six-component-load": 1}], ids=["string", "number", "object"])
def test_f5_non_array_affected_refs_names_no_case_s1(value):
    """RV90 S1 (07h): a non-array `affected_refs` names no case, as Rust's `list()` and TypeScript's
    `Array.isArray` read it (never a substring or key test, never an exception). Still listed, the
    diagnostic fails F5 (G5 ATTEMPT, the shared `f5_affected_refs_string_names_no_case`); unlisted,
    F5 passes and G7 refuses the malformed diagnostic, as every reader did before F5."""
    refs = _cases()[O_BASE]["source"]["retained_precision"]["body"]["ordinary_attempts"][0]["diagnostic_refs"]
    malformed = _set(["diagnostics", 0, "affected_refs"], value)
    _raises(lambda: _validate_entry(O_BASE, [malformed]), "G5", "ATTEMPT_MISMATCH")
    unlisted = _set(B + ["ordinary_attempts", 0, "diagnostic_refs"], refs[1:])
    with pytest.raises(rp.RetainedPrecisionError) as error:
        _validate_entry(O_BASE, [malformed, unlisted])
    assert (error.value.gate, error.value.code) == ("G7", "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID")

def test_rv80_n2_integral_normalization_touches_only_the_receipt(monkeypatch):
    """RV80-N2 (07g): D32's normalization of integral floats runs on the receipt only, never on the
    statement's rows, diagnostics or quality (Rust pins the same with `integral_receipt`)."""
    calls = []
    original = rp._normalize_integrals
    def recording(value):
        calls.append(value)
        return original(value)
    monkeypatch.setattr(rp, "_normalize_integrals", recording)
    fixture = _cases()[O_BASE]
    source = deepcopy(fixture["source"])
    source["retained_precision"]["body"]["work"]["charged"] = float(source["retained_precision"]["body"]["work"]["charged"])
    source["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", source["retained_precision"]["body"])
    rp._validate_draft(source, deepcopy(fixture["invocation"]))
    assert calls and set(calls[0]) == {"body", "receipt_sha256"}, "the outermost call is the receipt"
    assert not any(isinstance(v, dict) and "results" in v for v in calls), "the statement is never normalized"


# ---------------------------------------------------------------------------------------------
# Snapshot 07l (U8-2): the listed producer-solved bases (RR "I61's U8 plan ruled ...", decision 6).
# ---------------------------------------------------------------------------------------------
# PP's pinned L = 0 successor files (U8-1, `u8_l0_isolated_node_publishes_pinned_successor` at the U8
# head): (file sha256, receipt sha256, class counts), the counts as all three readers observed them (I68).
L0_PINS = {
    "sparse_interactive": ("93c6c86548b9d263cba9d9869010043d23ed1c9f2f304dd9f82eb705eb350876",
                           "c00cbe76954e5188c63b0ef69738cd40a8db15d303b1113a86524e6c3119dd72", [25, 78, 9, 1]),
    "dense_scrutiny": ("dbb3d477364248fb9ae15f7b9cff44410c2bffe45f783dd02d96eca663b0ac88",
                       "0b4250c8139ba25ab9d35fc2d443a01d85de943a8e3d0eb9193f5dd5061ce994", [25, 78, 9, 2]),
}
# The registered dev/test build that published them (PP `REGISTERED_PROFILES` at the U8 head).
L0_BUILD_IDENTITY = ("v1;rustc.release=1.97.1;rustc.commit=8bab26f4f68e0e26f0bb7960be334d5b520ea452;rustc.host=aarch64-apple-darwin;"
                     "rustc.llvm=22.1.6;target=aarch64-apple-darwin;target.arch=aarch64;target.pointer_width=64;target.endian=little;"
                     "target.os=macos;target.env=;panic=unwind;profile=debug;opt_level=0;debug_assertions=true;rustflags=;"
                     "pkg=open_pipe_stress_product_physics@0.2.0")
SYNTHETIC_PROVENANCE = "synthetic_reader_control_not_producer_execution_or_native_current"


def test_producer_solved_bases_are_the_pinned_live_successors_d_u6_5():
    """Decision 6 and D-U6-5: the corpus lists exactly the two producer-solved L = 0 bases. Each base's
    source and invocation are exactly the JSON values of PP's sha256-pinned live successor file (key
    order, types, signs and float bits), and its case-level provenance names the producer, the Direct
    entry, the registered build identity and the U8 head. With its invocation each is eligible, with
    the class counts all three readers observed; without, it is not."""
    import hashlib
    c = corpus()
    assert c["provenance"] == {"kind": "synthetic_control", "claim": "synthetic controls plus listed producer-solved bases; no native Current evidence"}
    assert [f["id"] for f in c["cases"][:17] if f["provenance"] != SYNTHETIC_PROVENANCE] == [f"u8_l0_isolated_node_{mode}" for mode in L0_PINS]
    for mode, (file_sha, receipt_sha, counts) in L0_PINS.items():
        base = _cases()[f"u8_l0_isolated_node_{mode}"]
        p = base["provenance"]
        assert p["fixture"] == f"fixtures/results/retained_precision_l0_successor_{mode}.json"
        raw = (ROOT / p["fixture"]).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == p["fixture_sha256"] == file_sha
        doc = json.loads(raw)
        assert set(doc) == {"id", "source", "invocation"} and doc["id"] == base["id"]
        for key in ("source", "invocation"):
            assert json.dumps(base[key]) == json.dumps(doc[key]), (mode, key)
        assert base["source"]["retained_precision"]["receipt_sha256"] == p["receipt_sha256"] == receipt_sha
        assert p["kind"] == "producer_solved" and p["producer"] == base["source"]["producer"]
        assert p["entry"] == "run_linear_static_preview_value_with_retained_direct"
        assert p["build_identity"] == L0_BUILD_IDENTITY
        assert p["u8_head"] == "d44909708529c6277fc1fd3b22997218296c8dd3"
        assert p["solver_mode"] == base["invocation"]["solver_mode"] == mode
        eligibility = lambda result: {key: result[key] for key in ("invocation_bound", "numerical_eligible", "standing")}
        assert eligibility(rp.validate_retained_precision(deepcopy(base["source"]))) == NOT_ELIGIBLE
        public = rp.validate_retained_precision(deepcopy(base["source"]), deepcopy(base["invocation"]))
        assert eligibility(public) == base["expected"] == {"invocation_bound": True, "numerical_eligible": True, "standing": "eligible"}
        assert public["classifications"] == base["expected_classifications"]
        assert [sum(1 for x in public["classifications"] if x["class"] == k) for k in ("relative_verified", "absolute_verified", "input_derived", "non_quantity")] == counts


# ---------------------------------------------------------------------------------------------
# B1 SR-PY (I91; PLAN_v2 §2.4; DESIGN_v2 §2 and §3.2-§3.3; RV108 N2): reader-local tests on
# synthetic n-case receipts, derived from the shared two-case bases and 07j's must-pass entry. They
# are not shared corpus entries: SC's 07n pins the shared ones (W-C2, `d38_beside_selected` with
# m1-m8, F-1's five and `not_required`'s three).
# ---------------------------------------------------------------------------------------------
NOT_REQUIRED = "not_required_second_case_checks_passed"
UNAVAILABLE_ROW = "case:unavailable-row"
PREP = ("G8", "RETAINED_PRECISION_PREPARATION_MISMATCH")
PRODUCT, ATTEMPT, WORK = (("G5", "RETAINED_PRECISION_" + x) for x in ("PRODUCT_ATTEMPT_MISMATCH", "ATTEMPT_MISMATCH", "WORK_MISMATCH"))


def _b1_verdict(base, edits, invocation_edits=None, must=None):
    """The reader's verdict on `base` after a must-pass entry's edits (when named) and `edits`,
    rehashed as a shared entry: ("admitted", eligible, standing), or the first failure (gate, code)."""
    prefix = []
    if must is not None:
        entry = next(e for e in corpus()["must_pass"] if e["id"] == must)
        assert entry["base"] == base
        prefix = deepcopy(entry["edits"])
    source, invocation = apply_entry(_cases()[base], {"edits": prefix + edits, "invocation_edits": invocation_edits or [], "rehash": "all"})
    try:
        result = rp.validate_retained_precision(source, invocation)
    except rp.RetainedPrecisionError as error:
        return (error.gate, error.code)
    return ("admitted", result["numerical_eligible"], result["standing"])


def _b1_row(base, kind, case):
    """The index of a base's row of `kind` for load case `case`."""
    rows = _cases()[base]["source"]["results"]
    return next(i for i, r in enumerate(rows) if r["kind"] == kind and r["basis_ref"]["ref_id"] == case)


def _b1_parity_row(case, row_id):
    """The dense base's parity row, moved to `case` with a fresh id; a non-selected case's row
    carries no recovery method (G6)."""
    rows = _cases()["ordinary_prepared_dense_synthetic"]["source"]["results"]
    row = deepcopy(next(r for r in rows if r["kind"] == "sparse_live_path_dense_parity_relative_delta"))
    row.pop("recovery_method", None)
    return dict(row, id=row_id, basis_ref=dict(row["basis_ref"], ref_id=case))


def _d38_edits():
    """R-D38 (4b) (DESIGN_v2 §2) on F_BASE, as SC's `d38_beside_selected` rewrites W-C2's case C:
    case 1 (unavailable, with its own registered CaseSource and a prepared attempt) failed its native
    stage before any Run, beside selected case 0. Its Run, `execution_order` entry and Call and Group
    entries go, the call's after-value and `charged` are recomputed (case 1's Run built nothing: it
    reused case 0's builds), its cause is a typed CaptureError::Origin, and every hash is resealed."""
    body = _cases()[F_BASE]["source"]["retained_precision"]["body"]
    assert all(b["origin"]["run"] == 0 for b in body["builds"])
    after = body["cases"][0]["run"]["invocation_after"]
    stages = dict.fromkeys(rp.STAGE_ORDER, "not_entered")
    stages.update(preparation="completed", native="failed")
    return [
        _set(B + ["cases", 1, "run"], None),
        _set(B + ["cases", 1, "reason"], {"code": "source_unavailable", "phase": "preparation", "cause": {"kind": "prepared_product_failure", "product_attempt_ref": 1}}),
        _set(A1 + ["run_ref"], None), _set(A1 + ["proof"], None), _set(A1 + ["stages"], stages),
        _set(A1 + ["result"], {"kind": "unavailable", "error": {"kind": "capture", "cause": {"kind": "origin", "cause": {"kind": "capacity"}}}}),
        _set(B + ["calls", 0, "owner_refs"], [{"kind": "case", "index": 0}]), _set(B + ["calls", 0, "source_refs"], [0]),
        _set(B + ["calls", 0, "run_refs"], [0]), _set(B + ["calls", 0, "invocation_after"], after),
        _set(B + ["groups", 0, "source_refs"], [0]), _set(B + ["work", "charged"], after),
        _set(B + ["work", "execution_order"], [{"kind": "case", "index": 0}]),
    ]


def test_b1_d38_capture_before_any_run_beside_a_selected_case():
    """R-D38 (4b) beside a selected case is admitted: G0-G8 pass, the standing is needs_recompute and
    case 0's classifications are the base's. Before B1, Python refused it at G5 PRODUCT_ATTEMPT (an
    entered native stage needed a Run). m1-m8 (DESIGN_v2 §2) and the other (4b) conjuncts are refused,
    each at this reader's first failure."""
    d38 = _d38_edits()
    assert _b1_verdict(F_BASE, d38) == ("admitted", False, "needs_recompute")
    source, invocation = apply_entry(_cases()[F_BASE], {"edits": d38, "rehash": "all"})
    assert rp.validate_retained_precision(source, invocation)["classifications"] == _cases()[F_BASE]["expected_classifications"]
    stages = dict.fromkeys(rp.STAGE_ORDER, "not_entered")
    stages.update(preparation="completed", native="failed")
    orphan = deepcopy(_cases()[F_BASE]["source"]["retained_precision"]["body"]["builds"])
    orphan.append(dict(deepcopy(orphan[0]), id=len(orphan), origin={"call": 0, "run": 1, "physical_record": 0, "phase": "shared"}))
    refused = {
        "m1 error kind native": ([_set(A1 + ["result", "error"], {"kind": "native", "run_ref": 1})], PRODUCT),
        "m2 native completed": ([_set(A1 + ["stages", "native"], "completed")], PRODUCT),
        "m3 run_ref while the case has no Run": ([_set(A1 + ["run_ref"], 1)], PRODUCT),
        "m4 execution_order still lists the case": ([_set(B + ["work", "execution_order"], [{"kind": "case", "index": 0}, {"kind": "case", "index": 1}])], ("G3", "RETAINED_PRECISION_COVERAGE_MISMATCH")),
        "m5 proof_start completed": ([_set(A1 + ["stages", "proof_start"], "completed")], PRODUCT),
        "m6 source_ref null with preparation completed": ([_set(A1 + ["source_ref"], None)], PRODUCT),
        "m7 the case's source in the call's source_refs": ([_set(B + ["calls", 0, "source_refs"], [0, 1])], ATTEMPT),
        "m7 the case's source in the group's source_refs": ([_set(B + ["groups", 0, "source_refs"], [0, 1])], ATTEMPT),
        "m8 the case's source_ref differs from the attempt's": ([_set(B + ["cases", 1, "source_ref"], 0)], PRODUCT),
        "a Build kept from the case's removed Run": ([_set(B + ["builds"], orphan)], WORK),
        "both source references null": ([_set(A1 + ["source_ref"], None), _set(B + ["cases", 1, "source_ref"], None)], PRODUCT),
        "result ready": ([_set(A1 + ["result"], {"kind": "ready"})], PRODUCT),
        "preparation failed": ([_set(A1 + ["stages"], dict(stages, preparation="failed"))], PRODUCT),
        "observables and G5a entered": ([_set(A1 + ["stages"], dict(stages, observables="failed", g5a="failed"))], PRODUCT),
        "reason code kernel_unresolved": ([_set(B + ["cases", 1, "reason", "code"], "kernel_unresolved")], PRODUCT),
        "reason phase kernel": ([_set(B + ["cases", 1, "reason", "phase"], "kernel")], PRODUCT),
        "cause names the other attempt": ([_set(B + ["cases", 1, "reason", "cause", "product_attempt_ref"], 0)], PRODUCT),
        # C2's cause table (repair 02, item 3): a receipt failure needs phase receipt; G5 ATTEMPT, as TS.
        "cause a receipt_failure in the preparation phase": ([_set(B + ["cases", 1, "reason", "cause"], {"kind": "receipt_failure", "check": "association", "field_path": "b1"})], ATTEMPT),
    }
    got = {name: _b1_verdict(F_BASE, d38 + edits) for name, (edits, _) in refused.items()}
    assert got == {name: want for name, (_, want) in refused.items()}


def test_b1_d38_reader_logic_names_every_conjunct():
    """The (4b) predicate itself, conjunct by conjunct, on the admitted attempt and case."""
    source, _ = apply_entry(_cases()[F_BASE], {"edits": _d38_edits(), "rehash": "all"})
    body = source["retained_precision"]["body"]
    a, case = body["product_attempts"][1], body["cases"][1]
    assert rp._d38_capture_before_run(a, case, 1)
    assert not rp._d38_capture_before_run(a, case, 0)  # the cause names this attempt
    breaks = [
        lambda a, c: c.__setitem__("run", body["cases"][0]["run"]),
        lambda a, c: a.__setitem__("proof", {}),
        lambda a, c: a["result"].__setitem__("kind", "ready"),
        lambda a, c: a["result"]["error"].__setitem__("kind", "native"),
        lambda a, c: a["stages"].__setitem__("preparation", "failed"),
        lambda a, c: a["stages"].__setitem__("native", "completed"),
        lambda a, c: a["stages"].__setitem__("certificate", "failed"),
        lambda a, c: c.__setitem__("status", "selected"),
        lambda a, c: c["reason"]["cause"].__setitem__("kind", "receipt_failure"),
        lambda a, c: c["reason"].__setitem__("code", "kernel_unresolved"),
        lambda a, c: c["reason"].__setitem__("phase", "kernel"),
        lambda a, c: (a.__setitem__("source_ref", None), c.__setitem__("source_ref", None)),
        lambda a, c: c.__setitem__("source_ref", 0),
    ]
    for k, edit in enumerate(breaks):
        a2, c2 = deepcopy(a), deepcopy(case)
        edit(a2, c2)
        assert not rp._d38_capture_before_run(a2, c2, 1), k


def test_b1_d38_stage_rule_refuses_each_conjunct_held_elsewhere():
    """RV113 N-2 (repair 02, item 6): the (4b) conjuncts that other checks also hold (D4d's capture-without-
    a-Run mapping, D19's cause, D4e's Run) are pinned where the reader applies them: `_g5_stages` itself
    refuses each break, G5 PRODUCT_ATTEMPT, whatever the other checks do."""
    source, _ = apply_entry(_cases()[F_BASE], {"edits": _d38_edits(), "rehash": "all"})
    body = source["retained_precision"]["body"]
    a, case = body["product_attempts"][1], body["cases"][1]
    fail = lambda ok: rp._need(ok, "G5", "PRODUCT_ATTEMPT_MISMATCH")
    rp._g5_stages(deepcopy(a), deepcopy(case), fail, 1)
    breaks = {
        "the case has a Run": lambda a, c: c.__setitem__("run", body["cases"][0]["run"]),
        "a native error, not a capture": lambda a, c: a["result"].__setitem__("error", {"kind": "native", "run_ref": 0}),
        "reason code kernel_unresolved": lambda a, c: c["reason"].__setitem__("code", "kernel_unresolved"),
        "reason phase kernel": lambda a, c: c["reason"].__setitem__("phase", "kernel"),
        "the case selected": lambda a, c: c.__setitem__("status", "selected"),
        "the cause a receipt_failure": lambda a, c: c["reason"].__setitem__("cause", {"kind": "receipt_failure", "check": "association", "field_path": "b1"}),
    }
    for name, edit in breaks.items():
        a2, c2 = deepcopy(a), deepcopy(case)
        edit(a2, c2)
        _raises(lambda: rp._g5_stages(a2, c2, fail, 1), "G5", "PRODUCT_ATTEMPT_MISMATCH")
    _raises(lambda: rp._g5_stages(deepcopy(a), deepcopy(case), fail, 0), "G5", "PRODUCT_ATTEMPT_MISMATCH")  # names another attempt


def _w2_published(index, tag="evaluation"):
    """Ordinary attempt `index` made W2-published (T-4's case B): an initial failure, then a W2
    publication triggered by it, keeping the attempt's own report diagnostic."""
    report = _cases()[P_BASE]["source"]["retained_precision"]["body"]["ordinary_attempts"][index]["initial"]["report_diagnostic_ref"]
    if tag == "evaluation":
        initial = {"kind": "structural_failure", "error": {"tag": "range", "detail": "b1"}, "diagnostic_ref": None}
        trigger = {"tag": "evaluation", "error": {"tag": "range", "detail": "b1"}}
    else:
        initial = {"kind": "formation_failure", "error": {"tag": "numerical_range", "name": "b1"}, "basis_index": 0}
        trigger = {"tag": "formation", "error": {"tag": "numerical_range", "name": "b1"}}
    w2 = {"kind": "published", "trigger": trigger, "force_scale_exponent": 3, "report_diagnostic_ref": report}
    return [_set(B + ["ordinary_attempts", index, "initial"], initial), _set(B + ["ordinary_attempts", index, "w2"], w2)]


def test_b1_g5_not_required_admits_a_w2_published_case():
    """G5's `not_required` rule (DESIGN_v2 §3.3, decision 9), on 07j's two-case statement (selected,
    then not_required): a W2-published case with the verdict checks_passed (T-4's case B), by an
    evaluation or a formation trigger, is admitted and the statement is eligible. Rust's three extra
    conjuncts (a Passed report, W2 untriggered) are not Python's rule. A non-null product attempt,
    `initial` not_attempted and another verdict stay refused, and a report keeps its outcome equality."""
    for tag in ("evaluation", "formation"):
        assert _b1_verdict(P_BASE, _w2_published(1, tag), must=NOT_REQUIRED) == ("admitted", True, "eligible"), tag
    refused = {
        "W2-published, verdict sensitive": (_w2_published(1) + [_set(["numerical_quality", "cases", 1, "solve_quality"], "sensitive")], ATTEMPT),
        "initial not_attempted": ([_set(B + ["ordinary_attempts", 1, "initial"], {"kind": "not_attempted", "cause": "ineligible"})], ATTEMPT),
        "report outcome differs from the verdict": ([_set(B + ["ordinary_attempts", 1, "initial", "outcome"], "sensitive")], ATTEMPT),
        "product_attempt_ref non-null (G3 first)": ([_set(B + ["cases", 1, "product_attempt_ref"], 0)], ("G3", "RETAINED_PRECISION_COVERAGE_MISMATCH")),
    }
    got = {name: _b1_verdict(P_BASE, edits, must=NOT_REQUIRED) for name, (edits, _) in refused.items()}
    assert got == {name: want for name, (_, want) in refused.items()}
    # RV113 S-4 (repair 02, item 5): the case names its own product attempt, which exists and names it, so
    # G3 passes and the rule's null-attempt conjunct is the first failure (G5 ATTEMPT, as Rust and TS;
    # RV113's probe `nr_product_attempt_ref_with_attempt`). 07j's edits, keeping attempt 1.
    own = [e for e in next(m for m in corpus()["must_pass"] if m["id"] == NOT_REQUIRED)["edits"]
           if e["path"] != B + ["product_attempts", 1]]
    assert len(own) == 6
    assert _b1_verdict(P_BASE, own + [_set(B + ["cases", 1, "product_attempt_ref"], 1)]) == ATTEMPT


def test_b1_g8_mode_row_and_requested_mode_for_every_case():
    """F-1 text B's P1 and the requested mode (DESIGN_v2 §3.2-§3.3), in G8's per-case loop: every case
    is checked, here the second one, unavailable (F_BASE) or not_required (07j). Before B1 Python
    checked neither, on any case."""
    fm = _b1_row(F_BASE, "linear_solver_mode_basis", UNAVAILABLE_ROW)
    pm = _b1_row(P_BASE, "linear_solver_mode_basis", UNAVAILABLE_ROW)
    rows = _cases()[F_BASE]["source"]["results"]
    duplicate = dict(deepcopy(rows[fm]), id="result:b1:duplicate-mode")
    # The not_required case has no proof, so dropping its row moves no projection index (G3).
    without = [r for i, r in enumerate(_cases()[P_BASE]["source"]["results"]) if i != pm]
    cases = {
        "unavailable case: dense code in sparse": (F_BASE, [_set(["results", fm, "value"], 2.0)], None),
        "unavailable case: mode code 3": (F_BASE, [_set(["results", fm, "value"], 3.0)], None),
        "unavailable case: two mode rows": (F_BASE, [_set(["results"], deepcopy(rows) + [duplicate])], None),
        "unavailable case: requested mode flipped": (F_BASE, [_set(B + ["ordinary_attempts", 1, "requested_mode"], "dense_scrutiny")], None),
        "not_required case: dense code in sparse": (P_BASE, [_set(["results", pm, "value"], 2.0)], NOT_REQUIRED),
        "not_required case: requested mode flipped": (P_BASE, [_set(B + ["ordinary_attempts", 1, "requested_mode"], "dense_scrutiny")], NOT_REQUIRED),
        "not_required case: no mode row": (P_BASE, [_set(["results"], deepcopy(without))], NOT_REQUIRED),
    }
    got = {name: _b1_verdict(base, edits, must=must) for name, (base, edits, must) in cases.items()}
    assert got == dict.fromkeys(cases, PREP)
    # The invocation's own mode is what each case's requested mode and mode code must match.
    assert _b1_verdict(F_BASE, [], [_set(["solver_mode"], "dense_scrutiny")]) == PREP


def test_b1_g8_parity_rows_p2_to_p4_for_every_case():
    """F-1 text B's P2-P4 (DESIGN_v2 §3.2) for every case. 07j's two-case statement is made dense (the
    invocation's mode, both requested modes and both mode rows; it has no parity row): the selected dense
    case at b = 0 without a parity row is admitted, and so is the not_required case's single parity row
    at b = 0. Two parity rows (P2), a parity row in sparse_interactive (P3) and a parity row on a
    W2-published case (P4) are refused at G8."""
    p = _cases()[P_BASE]["source"]
    to_dense = [_set(["solver_mode"], "dense_scrutiny")]
    dense = [_set(B + ["ordinary_attempts", k, "requested_mode"], "dense_scrutiny") for k in (0, 1)]
    dense_rows = deepcopy(p["results"])
    for case in ("case:six-component-load", UNAVAILABLE_ROW):
        dense_rows[_b1_row(P_BASE, "linear_solver_mode_basis", case)]["value"] = 2.0
    one, two = (_b1_parity_row(UNAVAILABLE_ROW, f"result:b1:parity-{k}") for k in (1, 2))
    on_dense = lambda extra_rows, extra=(): _b1_verdict(P_BASE, dense + [_set(["results"], dense_rows + extra_rows)] + list(extra), to_dense, NOT_REQUIRED)
    d = _cases()["ordinary_prepared_dense_synthetic"]["source"]
    parity = next(i for i, r in enumerate(d["results"]) if r["kind"] == "sparse_live_path_dense_parity_relative_delta")
    twice = dict(deepcopy(d["results"][parity]), id="result:b1:parity-twice")
    d_report = _cases()["ordinary_prepared_dense_synthetic"]["source"]["retained_precision"]["body"]["ordinary_attempts"][0]["initial"]["report_diagnostic_ref"]
    d_w2 = [_set(B + ["ordinary_attempts", 0, "initial"], {"kind": "structural_failure", "error": {"tag": "range", "detail": "b1"}, "diagnostic_ref": None}),
            _set(B + ["ordinary_attempts", 0, "w2"], {"kind": "published", "trigger": {"tag": "evaluation", "error": {"tag": "range", "detail": "b1"}}, "force_scale_exponent": 3, "report_diagnostic_ref": d_report})]
    f_rows = _cases()[F_BASE]["source"]["results"]
    got = {
        "dense b = 0, no parity row on either case": on_dense([]),
        "dense b = 0, one parity row on the not_required case": on_dense([one]),
        "dense, a W2-published not_required case without a parity row": on_dense([], _w2_published(1)),
        "P2: two parity rows on the not_required case": on_dense([one, two]),
        "P4: a parity row on the W2-published not_required case": on_dense([one], _w2_published(1)),
        "P2: two parity rows on the dense selected case": _b1_verdict("ordinary_prepared_dense_synthetic", [_set(["results"], deepcopy(d["results"]) + [twice])]),
        "P4: a parity row on a W2-published selected case": _b1_verdict("ordinary_prepared_dense_synthetic", d_w2),
        "P3: a parity row on the sparse unavailable case": _b1_verdict(F_BASE, [_set(["results"], deepcopy(f_rows) + [_b1_parity_row(UNAVAILABLE_ROW, "result:b1:sparse-parity")])]),
    }
    assert got == {
        "dense b = 0, no parity row on either case": ("admitted", True, "eligible"),
        "dense b = 0, one parity row on the not_required case": ("admitted", True, "eligible"),
        "dense, a W2-published not_required case without a parity row": ("admitted", True, "eligible"),
        "P2: two parity rows on the not_required case": PREP,
        "P4: a parity row on the W2-published not_required case": PREP,
        "P2: two parity rows on the dense selected case": PREP,
        "P4: a parity row on a W2-published selected case": PREP,
        "P3: a parity row on the sparse unavailable case": PREP,
    }


def test_b1_rv108_n2_a_missing_verdict_is_g5_attempt():
    """RV108 N2: a numerical_quality case without `solve_quality`, hash-consistent, fails G5 with
    RETAINED_PRECISION_ATTEMPT_MISMATCH wherever an ordinary rule reads the verdict (a report's
    outcome, not_required's checks_passed, selected's trigger verdicts), as Rust and TypeScript do;
    before, a KeyError reached the fail-closed fallback (PRODUCT_ATTEMPT). A case for which no rule
    reads the verdict still reaches G7's base header (SOURCE_NUMERICAL_CASE_INVALID)."""
    drop = lambda i: _set(["numerical_quality", "cases", i, "solve_quality"], None) | {"op": "remove"}
    got = {
        "selected case, report": _b1_verdict(O_BASE, [drop(0)]),
        "unavailable case, report": _b1_verdict(F_BASE, [drop(1)]),
        "not_required case, report": _b1_verdict(P_BASE, [drop(1)], must=NOT_REQUIRED),
        "not_required case, W2-published": _b1_verdict(P_BASE, _w2_published(1) + [drop(1)], must=NOT_REQUIRED),
    }
    assert got == dict.fromkeys(got, ATTEMPT)
    # An unavailable case whose initial is a structural failure (W2 not triggered): no G5 rule reads its
    # verdict. Present, the statement is admitted; absent, G7's base header refuses the case.
    structural = [_set(B + ["ordinary_attempts", 1, "initial"], {"kind": "structural_failure", "error": {"tag": "range", "detail": "b1"}, "diagnostic_ref": None})]
    assert _b1_verdict(F_BASE, structural) == ("admitted", False, "needs_recompute")
    assert _b1_verdict(F_BASE, structural + [drop(1)]) == ("G7", "SOURCE_NUMERICAL_CASE_INVALID")


# I91 repair 01 (RR "I91's SR-PY verified; the Build check accepted; PY's four false accepts repaired
# before RV-R"): Python admitted four statements that Rust and TypeScript refuse at G8. Each is now
# refused with their gate and code. Rust: RE `src/retained_precision.rs` `g8`; TS:
# `apps/desktop/src/features/results/retainedPrecision.ts` `invocationBinding`.
INVOCATION = ("G8", "RETAINED_PRECISION_INVOCATION_MISMATCH")


def test_b1_repair01_g8_invocation_members_and_solver_mode():
    """(d1) An invocation is exactly {request, solver_mode}: Rust's first `need` in `g8`
    (`o.len() == 2 && o.contains_key("request") && o.contains_key("solver_mode")`, with the digest) and
    TS's first `fail` in `invocationBinding` (`same(Object.keys(invocation).sort(), ['request',
    'solver_mode'])`), both INVOCATION_MISMATCH. (d2) Its solver mode is one of the two: Rust's second
    `need` (`matches!(mode, "sparse_interactive" | "dense_scrutiny")`) and TS's same first `fail`, both
    INVOCATION_MISMATCH; before, Python admitted an unknown mode (requested modes unedited) or refused it
    at PREPARATION through the requested-mode check. Each invocation edit rebinds the receipt's digest."""
    got = {
        "an extra member": _b1_verdict(O_BASE, [], [_set(["extra"], 1)]),
        "an extra member, 07j's two-case statement": _b1_verdict(P_BASE, [], [_set(["extra"], None)], NOT_REQUIRED),
        "solver_mode removed": _b1_verdict(O_BASE, [], [_set(["solver_mode"], None) | {"op": "remove"}]),
        "solver_mode unknown": _b1_verdict(O_BASE, [], [_set(["solver_mode"], "foo")]),
        "solver_mode another spelling": _b1_verdict(O_BASE, [], [_set(["solver_mode"], "SPARSE_INTERACTIVE")]),
        "solver_mode null": _b1_verdict(O_BASE, [], [_set(["solver_mode"], None)]),
        "solver_mode a list": _b1_verdict(O_BASE, [], [_set(["solver_mode"], ["sparse_interactive"])]),
    }
    assert got == dict.fromkeys(got, INVOCATION)
    # The other known mode stays a per-case PREPARATION refusal (the requested modes disagree).
    assert _b1_verdict(O_BASE, [], [_set(["solver_mode"], "dense_scrutiny")]) == PREP


def test_b1_repair01_g8_material_basis_of_every_case_and_exact_case_indices():
    """(b) DESIGN_v2 §3.3's G8 step 2 for every case: an ordinary attempt's material basis is its case's
    selector in first-seen order. Rust: `fail(o["requested_mode"] == mode && u(&o["material_basis_ref"])
    == index)` in `g8`'s case loop; TS: `fail(ordinary.material_basis_ref === bi && ...)`; both
    PREPARATION_MISMATCH. On 07j's not_required case (no product attempt, no source) Python admitted a
    dangling or foreign reference as eligible. (c) The material bases are exactly one per selector, in
    first-seen order, each with exactly its cases. Rust: `fail(list(&b["material_bases"]).len() ==
    expected_selectors.len())` and each basis's `case_indices == …`; TS: `fail(b.material_bases.length ===
    knownSelectors.length)` and `same(mb.case_indices, …)`; all PREPARATION_MISMATCH.
    Repair 02 (the alignment set, item 1): an ordinary attempt whose basis does not resolve to a basis listing
    its case fails first at G5 ATTEMPT_MISMATCH (TS `ordinaryAttempts`: `material_bases[a.material_basis_ref]
    ?.case_indices.includes(ci)`), so three of these inputs read G5 ATTEMPT in every reader once aligned (TS
    already). D16: the G8 refusals are the checks' own, never the fail-closed fallback's."""
    basis = deepcopy(_cases()[P_BASE]["source"]["retained_precision"]["body"]["material_bases"])
    assert basis[0]["case_indices"] == [0, 1]
    two_groups = "two_case_two_groups_synthetic"
    tg = deepcopy(_cases()[two_groups]["source"]["retained_precision"]["body"]["material_bases"])
    got = {
        "(b) not_required case: material_basis_ref 7": _b1_verdict(P_BASE, [_set(B + ["ordinary_attempts", 1, "material_basis_ref"], 7)], must=NOT_REQUIRED),
        "(b) not_required case: material_basis_ref 1 beside a second basis listing it": _b1_verdict(P_BASE, [
            _set(B + ["ordinary_attempts", 1, "material_basis_ref"], 1),
            _set(B + ["material_bases"], [dict(basis[0], case_indices=[0]), dict(deepcopy(basis[0]), index=1, case_indices=[1])])], must=NOT_REQUIRED),
        "(c) the basis omits the not_required case": _b1_verdict(P_BASE, [_set(B + ["material_bases", 0, "case_indices"], [0])], must=NOT_REQUIRED),
        "(c) the basis lists its cases out of order": _b1_verdict(P_BASE, [_set(B + ["material_bases", 0, "case_indices"], [1, 0])], must=NOT_REQUIRED),
        "(c) an extra basis listing no case": _b1_verdict(P_BASE, [_set(B + ["material_bases"], basis + [dict(deepcopy(basis[0]), index=1, case_indices=[])])], must=NOT_REQUIRED),
        "(c) two selectors, the second basis's cases swapped into the first": _b1_verdict(two_groups, [
            _set(B + ["material_bases", 0, "case_indices"], [0, 1]), _set(B + ["material_bases", 1, "case_indices"], [])]),
    }
    assert got == {
        "(b) not_required case: material_basis_ref 7": ATTEMPT,
        "(b) not_required case: material_basis_ref 1 beside a second basis listing it": PREP,
        "(c) the basis omits the not_required case": ATTEMPT,
        "(c) the basis lists its cases out of order": PREP,
        "(c) an extra basis listing no case": PREP,
        "(c) two selectors, the second basis's cases swapped into the first": ATTEMPT,
    }
    # (c)'s count refuses an extra basis by its own check (D16), not through the fail-closed fallback that a
    # missing count would reach (the extra basis has no selector). Bound statement; the refusal has no cause.
    fixture = _cases()[P_BASE]
    edits = deepcopy(next(m for m in corpus()["must_pass"] if m["id"] == NOT_REQUIRED)["edits"])
    source, invocation = apply_entry(fixture, {"edits": edits + [_set(B + ["material_bases"], basis + [dict(deepcopy(basis[0]), index=1, case_indices=[])])], "rehash": "all"})
    with pytest.raises(rp.RetainedPrecisionError) as error:
        rp.validate_retained_precision(source, invocation)
    assert (error.value.gate, error.value.code, error.value.__cause__) == ("G8", "RETAINED_PRECISION_PREPARATION_MISMATCH", None)
    # Controls: the unedited statements are admitted, and so is each base's own basis list.
    assert _b1_verdict(P_BASE, [], must=NOT_REQUIRED) == ("admitted", True, "eligible")
    assert _b1_verdict(two_groups, [_set(B + ["material_bases"], tg)]) == ("admitted", True, "eligible")


def test_b1_repair01_g8_every_material_basis_has_its_materials_checked():
    """(e) A material basis used only by cases without a CaseSource has its materials checked like any
    other: the used materials in input order, each selected for the basis's cases (id, selection,
    explicit G, E and G). Rust: `g8`'s material bases loop (`list(&mb["materials"])…eq(expected_material_
    indices…)` and, per material, `selected_material(raw, &cases[ci])`); TS: `b.material_bases.forEach`
    (`same(mb.materials.map(…input_index), …)` and `selectedMaterial(raw, cases[ci])` for each case);
    all PREPARATION_MISMATCH. Python checked a basis only through a CaseSource, so 07j's not_required case
    on its own named basis was admitted with any material list."""
    base_mb = deepcopy(_cases()[P_BASE]["source"]["retained_precision"]["body"]["material_bases"][0])
    point = {"id": "tp:b1", "temperature": {"value": 400, "unit": "K"}, "elastic_modulus": {"value": 400000000000.0, "unit": "Pa"},
             "shear_modulus": {"value": 154000000000.0, "unit": "Pa"}, "thermal_expansion_coefficient": {"value": 1e-05, "unit": "1/K"}}
    named = {"index": 1, "selector": {"kind": "named", "id": "tp:b1"}, "case_indices": [1], "materials": [dict(
        deepcopy(base_mb["materials"][0]), elastic_modulus=rp.bits(4e11), shear_modulus=rp.bits(1.54e11), selection={"kind": "named_point", "point_id": "tp:b1"})]}
    invocation = [_set(["request", "model", "materials", 0, "temperature_points"], [point]), _set(["request", "model", "load_cases", 1, "modulus_basis_ref"], "tp:b1")]

    def verdict(second):
        return _b1_verdict(P_BASE, [_set(B + ["ordinary_attempts", 1, "material_basis_ref"], 1),
                                    _set(B + ["material_bases"], [dict(base_mb, case_indices=[0]), second])], invocation, NOT_REQUIRED)

    def edited(change):
        second = deepcopy(named)
        change(second)
        return verdict(second)

    assert verdict(named) == ("admitted", True, "eligible")
    # Without its second basis, the sourceless case's basis reference does not resolve: G5 ATTEMPT since
    # repair 02 (the ordinary class's reference rule; TS `ordinaryAttempts`), before G8's count.
    assert _b1_verdict(P_BASE, [_set(B + ["ordinary_attempts", 1, "material_basis_ref"], 1),
                                _set(B + ["material_bases"], [dict(base_mb, case_indices=[0])])], invocation, NOT_REQUIRED) == ATTEMPT
    got = {
        "its elastic modulus wrong": edited(lambda m: m["materials"][0].__setitem__("elastic_modulus", rp.bits(3e11))),
        "its shear modulus wrong": edited(lambda m: m["materials"][0].__setitem__("shear_modulus", rp.bits(1e11))),
        "its material selection the base's": edited(lambda m: m["materials"][0].__setitem__("selection", {"kind": "base"})),
        "its material another id": edited(lambda m: m["materials"][0].__setitem__("id", "material:other")),
        "its shear origin derived": edited(lambda m: m["materials"][0].__setitem__("shear_origin", {"kind": "derived_e_nu", "poisson_ratio": rp.bits(0.3), "constitutive_basis": "homogeneous_isotropic_E_nu_v1"})),
        "no material listed": edited(lambda m: m.__setitem__("materials", [])),
    }
    assert got == dict.fromkeys(got, PREP)


# I91 repair 02 (RR "RV113's three returns verified; I3 made at `2ba2f81863`; the three-reader alignment set
# ruled", items 1-3): the receipt's own references at G3, the ordinary attempt's basis at G5, the model scope
# at G8 INVOCATION, and C2's cause table at G5. Inputs as RV113's probes (`evidence/fg/`, `evidence/probes/`).
COVERAGE = ("G3", "RETAINED_PRECISION_COVERAGE_MISMATCH")


def _b1_both(base, edits, must=None):
    """(bound verdict, unbound verdict) of `_b1_verdict`'s statement."""
    prefix = deepcopy(next(e for e in corpus()["must_pass"] if e["id"] == must)["edits"]) if must else []
    source, invocation = apply_entry(_cases()[base], {"edits": prefix + edits, "rehash": "all"})
    out = []
    for inv in (invocation, None):
        try:
            result = rp.validate_retained_precision(deepcopy(source), deepcopy(inv))
            out.append(("admitted", result["numerical_eligible"], result["standing"]))
        except rp.RetainedPrecisionError as error:
            out.append((error.gate, error.code))
    return tuple(out)


def test_b1_repair02_receipt_references_at_g3_and_the_ordinary_basis_at_g5():
    """Item 1, bound and unbound: each material basis and each source at its index, a source's owner its
    own case, and a basis's case list unique and in range, at G3 COVERAGE (TS `coverage`: `m.index === i &&
    unique(m.case_indices) && …`, `s.index === i && s.owner.kind === 'case' && ids[s.owner.case_index] ===
    s.owner.case_id`). Python admitted a wrong basis index even bound, and the rest unbound. An ordinary
    attempt whose basis does not resolve to a basis listing its case fails at G5 ATTEMPT_MISMATCH (TS
    `ordinaryAttempts`), bound and unbound. 07m's `integral_float_integers_and_references` stays a
    must-pass (the census)."""
    two_case, two_groups = "two_case_synthetic", "two_case_two_groups_synthetic"
    got = {
        "material_bases[0].index 1": _b1_both(O_BASE, [_set(B + ["material_bases", 0, "index"], 1)]),
        "two bases' indices swapped": _b1_both(two_groups, [_set(B + ["material_bases", 0, "index"], 1), _set(B + ["material_bases", 1, "index"], 0)]),
        "sources[0].index 1": _b1_both(O_BASE, [_set(B + ["sources", 0, "index"], 1)]),
        "two sources' indices swapped": _b1_both(two_case, [_set(B + ["sources", 0, "index"], 1), _set(B + ["sources", 1, "index"], 0)]),
        "case_indices with a duplicate": _b1_both(two_case, [_set(B + ["material_bases", 0, "case_indices"], [0, 1, 1])]),
        "case_indices out of range": _b1_both(two_case, [_set(B + ["material_bases", 0, "case_indices"], [0, 1, 2])]),
        "source 1's owner names case 0's id": _b1_both(two_case, [_set(B + ["sources", 1, "owner", "case_id"], "case:six-component-load")]),
        "source 1's owner index out of range": _b1_both(two_case, [_set(B + ["sources", 1, "owner", "case_index"], 2)]),
    }
    assert got == dict.fromkeys(got, (COVERAGE, COVERAGE))
    # D16: an owner index out of range is refused by the check itself, not by the fallback an IndexError reaches.
    source, _ = apply_entry(_cases()[two_case], {"edits": [_set(B + ["sources", 1, "owner", "case_index"], 2)], "rehash": "all"})
    with pytest.raises(rp.RetainedPrecisionError) as error:
        rp.validate_retained_precision(source)
    assert (error.value.gate, error.value.code, error.value.__cause__) == (*COVERAGE, None)
    ordinary = {
        "the not_required case's basis 7": _b1_both(P_BASE, [_set(B + ["ordinary_attempts", 1, "material_basis_ref"], 7)], NOT_REQUIRED),
        "its basis omits it": _b1_both(P_BASE, [_set(B + ["material_bases", 0, "case_indices"], [0])], NOT_REQUIRED),
        "a selected case's basis omits it": _b1_both(two_groups, [_set(B + ["material_bases", 1, "case_indices"], [])]),
    }
    assert ordinary == dict.fromkeys(ordinary, (ATTEMPT, ATTEMPT))
    assert _b1_both(P_BASE, [], NOT_REQUIRED) == (("admitted", True, "eligible"), ("admitted", False, "needs_recompute"))


def test_b1_repair02_model_scope_at_g8_invocation():
    """Item 2: the invocation model's scope as PP accepts it, at G8 INVOCATION_MISMATCH, before any
    PREPARATION check: no `reference_configurations` member (null included; PP `validate_document`), a
    `pressure_contract` absent or null, and `combinations` and `components` absent or `[]` (PP's typed model).
    RV113 §10.2's probes. Unbound reads have no invocation, so they are unaffected."""
    model = ["request", "model"]
    refused = {
        "reference_configurations null": [_set(model + ["reference_configurations"], None)],
        "reference_configurations []": [_set(model + ["reference_configurations"], [])],
        "pressure_contract {}": [_set(model + ["pressure_contract"], {})],
        "pressure_contract false": [_set(model + ["pressure_contract"], False)],
        "combinations null": [_set(model + ["combinations"], None)],
        "combinations {'x': 1}": [_set(model + ["combinations"], {"x": 1})],
        "combinations [{}]": [_set(model + ["combinations"], [{}])],
        "components 'x'": [_set(model + ["components"], "x")],
        "components null": [_set(model + ["components"], None)],
    }
    got = {name: _b1_verdict(O_BASE, [], edits) for name, edits in refused.items()}
    assert got == dict.fromkeys(refused, INVOCATION)
    admitted = {
        "pressure_contract null": [_set(model + ["pressure_contract"], None)],
        "combinations []": [_set(model + ["combinations"], [])],
        "components []": [_set(model + ["components"], [])],
    }
    got = {name: _b1_verdict(O_BASE, [], edits) for name, edits in admitted.items()}
    assert got == dict.fromkeys(admitted, ("admitted", True, "eligible"))
    assert "reference_configurations" not in _cases()[O_BASE]["invocation"]["request"]["model"]


def _c2_body(base=F_BASE):
    body = deepcopy(_cases()[base]["source"]["retained_precision"]["body"])
    source = _cases()[base]["source"]
    return body, source["diagnostics"], source["numerical_quality"]["cases"]


def test_b1_repair02_c2_cause_table_reader_logic():
    """Item 3: C2's cause table (CONTRACT_DELTA:72-74) at G5 ATTEMPT_MISMATCH in the ordinary class, for an
    unavailable case whose cause is not a prepared product failure: TS's form with the `precondition` keying.
    Each branch satisfied, then broken one conjunct at a time, on F_BASE's unavailable case 1 (`_g5_ordinary`
    alone, as the source-decline control does)."""
    body, diags, quality = _c2_body()
    selected_run = deepcopy(body["cases"][1]["run"])
    assert selected_run["kernel_terminal"]["kind"] == "selected"
    unresolved = dict(deepcopy(selected_run), kernel_terminal={"kind": "unresolved", "reason": {"space": "unresolved", "tag": "ceiling"}})
    decline = {"input_owner": {"case_index": 1, "case_id": "case:unavailable-row", "material_basis_ref": 0},
               "constructor_counts": {"nodes": 2, "members": 1, "springs": 0, "constraints": 6, "nodal_terms": 6, "stations": 3, "supports": 1, "id_utf8_bytes": 0, "directional_springs": 0},
               "error": {"tag": "no_nodes"}}

    def case(code, phase, cause, run=None, source_ref=None, source_decline=None):
        c = {k: deepcopy(body["cases"][1][k]) for k in ("basis_ref", "ordinary", "product_attempt_ref", "status", "diagnostic_ref")}
        c.update(reason={"code": code, "phase": phase, "cause": cause}, run=run, source_ref=source_ref)
        if source_decline is not None:
            c["source_decline"] = source_decline
        return c

    def verdict(c):
        b = deepcopy(body)
        b["cases"][1] = c
        try:
            rp._g5_ordinary(b, b["cases"], diags, quality)
            return "ok"
        except rp.RetainedPrecisionError as error:
            return (error.gate, error.code)

    receipt = {"kind": "receipt_failure", "check": "encoding", "field_path": "retained_precision.body"}
    facade = {"kind": "facade_failure", "owner_ref": {"kind": "case", "index": 1}, "row_id": None, "recipe": "identity", "operand_index": None, "check": "identity", "predicate": None}
    source_error = {"kind": "source_error", "error": {"tag": "no_nodes"}}
    precondition = lambda p: {"kind": "unavailable_precondition", "precondition": p, "affected_refs": ["case:unavailable-row"]}
    kernel = {"space": "unresolved", "tag": "ceiling"}
    satisfied = {
        **{f"receipt_failure, {code}": case(code, "receipt", receipt, selected_run, 1) for code in ("receipt_encoding", "publication_hash_range", "invocation_not_representable")},
        "facade_failure": case("facade_certificate", "facade", facade, selected_run, 1),
        "source_error": case("source_unavailable", "preparation", source_error, source_decline=decline),
        **{f"precondition {p}, phase {phase}": case(code, phase, precondition(p)) for p, code in rp.PRECONDITION_CODES.items() for phase in ("routing", "preparation")},
        "kernel reason": case("kernel_unresolved", "kernel", kernel, unresolved, 1),
    }
    assert {name: verdict(c) for name, c in satisfied.items()} == dict.fromkeys(satisfied, "ok")
    broken = {
        "receipt_failure, phase kernel": case("receipt_encoding", "kernel", receipt, selected_run, 1),
        "receipt_failure, code facade_certificate": case("facade_certificate", "receipt", receipt, selected_run, 1),
        "facade_failure, phase kernel": case("facade_certificate", "kernel", facade, selected_run, 1),
        "facade_failure, code kernel_unresolved": case("kernel_unresolved", "facade", facade, selected_run, 1),
        "facade_failure, no Run": case("facade_certificate", "facade", facade),
        "facade_failure, an unresolved Run": case("facade_certificate", "facade", facade, unresolved, 1),
        "facade_failure, another case's owner": case("facade_certificate", "facade", dict(facade, owner_ref={"kind": "case", "index": 0}), selected_run, 1),
        "source_error, phase routing": case("source_unavailable", "routing", source_error, source_decline=decline),
        "source_error, code caller_not_qualified": case("caller_not_qualified", "preparation", source_error, source_decline=decline),
        "source_error, no decline": case("source_unavailable", "preparation", source_error),
        "source_error, the decline's error another": case("source_unavailable", "preparation", dict(source_error, error={"tag": "no_members"}), source_decline=decline),
        "source_error, a Run": case("source_unavailable", "preparation", source_error, selected_run, 1),
        "precondition caller, code source_unavailable": case("source_unavailable", "routing", precondition("caller")),
        "precondition capture, code caller_not_qualified": case("caller_not_qualified", "routing", precondition("capture")),
        "precondition resource_admission, code upstream_no_wrap_not_established": case("upstream_no_wrap_not_established", "routing", precondition("resource_admission")),
        "precondition upstream_no_wrap, code resource_admission_not_available": case("resource_admission_not_available", "routing", precondition("upstream_no_wrap")),
        "precondition source_family, phase kernel": case("source_unavailable", "kernel", precondition("source_family")),
        "precondition caller, a Run": case("caller_not_qualified", "routing", precondition("caller"), selected_run, 1),
        "kernel reason, phase facade": case("kernel_unresolved", "facade", kernel, unresolved, 1),
        "kernel reason, code kernel_refused": case("kernel_refused", "kernel", kernel, unresolved, 1),
        "kernel reason, another reason": case("kernel_unresolved", "kernel", {"space": "unresolved", "tag": "budget", "scope": "case"}, unresolved, 1),
        "kernel reason, no Run": case("kernel_unresolved", "kernel", kernel),
    }
    assert {name: verdict(c) for name, c in broken.items()} == dict.fromkeys(broken, ("G5", "RETAINED_PRECISION_ATTEMPT_MISMATCH"))


def test_b1_repair02_c2_cause_table_in_the_reader():
    """Item 3 through the whole reader, RV113's C2 probes (`c2_*`): two_case_synthetic's case 1 made
    unavailable with its Ready attempt and selected Run. The receipt_failure branch satisfied is admitted
    (needs_recompute); broken, G5 ATTEMPT_MISMATCH (Python admitted these before, as Rust does today). The
    facade_failure branch satisfied reaches D19 (a Ready attempt needs receipt_failure, G5 PRODUCT_ATTEMPT);
    broken, G5 ATTEMPT_MISMATCH first (before: G5 PRODUCT_ATTEMPT). TS gives these verdicts already."""
    base = "two_case_synthetic"
    source = _cases()[base]["source"]
    old = source["retained_precision"]["body"]["cases"][1]
    cid = old["basis_ref"]["ref_id"]
    di = next(i for i, d in enumerate(source["diagnostics"]) if d["code"] == "RETAINED_PRECISION_SELECTED" and d.get("affected_refs") == [cid])
    rows = [{k: v for k, v in r.items() if not (k == "recovery_method" and r["basis_ref"]["ref_id"] == cid)} for r in source["results"]]

    def with_reason(code, phase, cause):
        case = {k: deepcopy(old[k]) for k in ("basis_ref", "ordinary", "product_attempt_ref", "run", "source_ref")}
        case.update(status="unavailable", reason={"code": code, "phase": phase, "cause": cause}, diagnostic_ref=source["diagnostics"][di]["id"])
        return _b1_verdict(base, [_set(B + ["cases", 1], case), _set(["diagnostics", di, "code"], "RETAINED_PRECISION_UNAVAILABLE"), _set(["results"], rows)])

    receipt = {"kind": "receipt_failure", "check": "encoding", "field_path": "retained_precision.body"}
    facade = {"kind": "facade_failure", "owner_ref": {"kind": "case", "index": 1}, "row_id": None, "recipe": "identity", "operand_index": None, "check": "identity", "predicate": None}
    got = {
        "c2_receipt_ok": with_reason("receipt_encoding", "receipt", receipt),
        "c2_receipt_phase_kernel": with_reason("kernel_unresolved", "kernel", receipt),
        "c2_receipt_code_facade": with_reason("facade_certificate", "receipt", receipt),
        "c2_receipt_phase_preparation": with_reason("source_unavailable", "preparation", receipt),
        "c2_facade_ok": with_reason("facade_certificate", "facade", facade),
        "c2_facade_phase_kernel": with_reason("facade_certificate", "kernel", facade),
    }
    assert got == {"c2_receipt_ok": ("admitted", False, "needs_recompute"), "c2_receipt_phase_kernel": ATTEMPT, "c2_receipt_code_facade": ATTEMPT,
                   "c2_receipt_phase_preparation": ATTEMPT, "c2_facade_ok": PRODUCT, "c2_facade_phase_kernel": ATTEMPT}


# ---------------------------------------------------------------------------------------------
# B1's reader follow-up toward I4′, PY's lane (I100; RR "I4 made at `30f3d1b24a`; RV113's items for ROOT
# ruled; …", rulings 1, 2 and 5). The inputs are RV113's probes (`probes_ts1.json`: the `h:`, `n6_` and
# `r2:` ids named below), each built from its corpus base and edits and rehashed as a shared entry.
# ---------------------------------------------------------------------------------------------
I4P_INVALID = ("G7", "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID")
I4P_NEEDS = ("admitted", False, "needs_recompute")
I4P_CARRIER = _set(["carrier_evidence"], {})
I4P_RECOVERY = _set(["source_block_recovery"], {})
I4P_EVIDENCE_NULL = _set(["contract_evidence"], None)


def _i4p_three(base, edits):
    """((bound, unbound, transport) verdicts, their details) of the rehashed entry. A verdict is ("admitted",
    eligible, standing) or the first failure (gate, code); a detail is the refusal's detail, or None."""
    source, invocation = apply_entry(_cases()[base], {"edits": edits, "rehash": "all"})
    verdicts, details = [], []
    for read in (lambda s: rp.validate_retained_precision(s, deepcopy(invocation)), rp.validate_retained_precision, rp.validate_retained_precision_transport):
        try:
            result = read(deepcopy(source))
            verdicts.append(("admitted", result["numerical_eligible"], result["standing"]))
            details.append(None)
        except rp.RetainedPrecisionError as error:
            verdicts.append((error.gate, error.code))
            details.append(error.detail)
    return tuple(verdicts), tuple(details)


def _i4p_projected(source):
    """The reader's projection: the same statement under the preview-physics-1 identity (no receipt)."""
    base = deepcopy(source)
    del base["retained_precision"]
    base["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/preview-physics-1"
    base["formulation_basis"]["profile_id"] = "product_preview_mechanics_v1"
    for row in base["results"]:
        row.pop("recovery_method", None)
    return base


def test_i4p_ruling1_transport_header_takes_rusts_order_and_codes():
    """Ruling 1 (RV113's SR-PY addendum S-1, option (a)): on transport, PY's base header takes Rust's order and
    codes, as TS's does. It has no carrier branch, so a `carrier_evidence` member alone is a metadata defect, G7
    SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID, and beside a header defect that defect's G2 code reads; and
    `source_block_recovery` is checked before `contract_evidence`. RV113's six transport probes, each expected
    with RS's and TS's gate and code at I4. Raw reads keep PY's own order at G7 (their codes are the declared
    per-reader raw class, B1_SC item 13), unchanged; so does the transport dispatch of a statement that is not
    a retained successor (ruling 1 is the retained reader's transport header)."""
    from core.analysis_runs import compatibility as c
    quality = _set(["numerical_quality", "status"], "bogus")
    case = _set(["numerical_quality", "cases", 0, "structural_status"], ["x"])
    probes = {
        "h_carrier_present": ([I4P_CARRIER], I4P_INVALID, "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"),
        "h_carrier_and_quality_defect": ([I4P_CARRIER, quality], ("G2", "SOURCE_NUMERICAL_QUALITY_INVALID"), "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"),
        "h_carrier_and_recovery": ([I4P_CARRIER, I4P_RECOVERY], ("G2", "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN"), "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"),
        "n6_carrier_evidence_with_case_defect": ([I4P_CARRIER, case], ("G2", "SOURCE_NUMERICAL_CASE_INVALID"), "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"),
        "h_recovery_and_evidence_null": ([I4P_RECOVERY, I4P_EVIDENCE_NULL], ("G2", "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN"), "SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED"),
        "n6_contract_evidence_null_and_source_block_recovery": ([I4P_EVIDENCE_NULL, I4P_RECOVERY], ("G2", "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN"), "SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED"),
    }
    got = {name: _i4p_three(O_BASE, edits)[0] for name, (edits, _, _) in probes.items()}
    assert got == {name: (("G7", raw), ("G7", raw), transport) for name, (_, transport, raw) in probes.items()}
    # The carrier member alone reaches the metadata check's namespace demand.
    assert _i4p_three(O_BASE, [I4P_CARRIER])[1][2] == "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: unsupported source namespace"
    # Each member alone keeps its own gate and code; the unedited base is admitted, never eligible.
    assert _i4p_three(O_BASE, [I4P_RECOVERY])[0][2] == ("G2", "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN")
    assert _i4p_three(O_BASE, [I4P_EVIDENCE_NULL])[0][2] == ("G2", "SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED")
    assert _i4p_three(O_BASE, [])[0][2] == I4P_NEEDS
    # The successor's transport dispatch is the reader's transport step: its text is the reader's detail.
    def dispatch(s):
        try:
            c._source_contract(deepcopy(s), check_receipt=False)
            return "ok"
        except ValueError as error:
            return str(error)
    carrier, _ = apply_entry(_cases()[O_BASE], {"edits": [I4P_CARRIER], "rehash": "all"})
    pair, _ = apply_entry(_cases()[O_BASE], {"edits": [I4P_RECOVERY, I4P_EVIDENCE_NULL], "rehash": "all"})
    assert dispatch(carrier) == "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: unsupported source namespace"
    assert dispatch(pair) == "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN"
    # A preview-physics-1 statement's own transport dispatch is unchanged (Python's order).
    assert dispatch(_i4p_projected(carrier)) == "SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"
    assert dispatch(_i4p_projected(pair)) == "SOURCE_PREVIEW_PHYSICS_EVIDENCE_REQUIRED"


def test_i4p_ruling2_transport_metadata_compares_withheld_records_as_multisets():
    """Ruling 2 (N-1 in RV113's three addenda): the transport metadata check's shared form is Rust's and TS's
    check plus PY's extrema-number demand. On transport, a withheld record whose multiplicity differs between
    cases (`r2:t_withheld_duplicate_multiset`) is refused at G7 SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: PY now
    compares the records as multisets (it compared sets and admitted it). Equal multisets, in any order, are
    admitted, as in RS and TS. The extrema-number demand stays (`r2:t_extrema_global_upper_string`,
    `r2:t_extrema_certified_gap_null`). The same check serves a preview-physics-1 statement's transport
    dispatch, with the same result. Raw reads are unchanged."""
    from core.analysis_runs import compatibility as c
    from core.analysis_runs.preview_physics_evidence import validate_transport_metadata
    two = "two_case_synthetic"
    path = lambda i: ["contract_evidence", "preview_cases", i, "support_attribution", "withheld"]
    s = {"support_id": "s", "reason": "CONSTANT_EFFORT_NOT_CONSUMED"}
    t = {"support_id": "t", "reason": "SUPPORT_ACTION_ATTRIBUTION_WITHHELD"}
    rows = {
        "t_withheld_duplicate_multiset": ([_set(path(0), [s, s]), _set(path(1), [s])], I4P_INVALID),
        "the multiplicities the other way": ([_set(path(0), [s]), _set(path(1), [s, s])], I4P_INVALID),
        "equal multisets, a duplicate in each": ([_set(path(0), [s, s]), _set(path(1), [s, s])], I4P_NEEDS),
        "equal multisets, in another order": ([_set(path(0), [s, t]), _set(path(1), [t, s])], I4P_NEEDS),
        "the sets differ": ([_set(path(0), [s]), _set(path(1), [t])], I4P_INVALID),
    }
    got = {name: _i4p_three(two, edits) for name, (edits, _) in rows.items()}
    assert {name: verdicts[2] for name, (verdicts, _) in got.items()} == {name: want for name, (_, want) in rows.items()}
    verdicts, details = got["t_withheld_duplicate_multiset"]
    assert details[2] == "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: support attribution differs between cases"
    # Raw reads are unchanged: the support-action step refuses a record repeated within a case.
    assert verdicts[:2] == (I4P_INVALID, I4P_INVALID)
    assert details[:2] == ("SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: withheld support record and diagnostic disagree",) * 2
    # The same transport metadata check, reached without a receipt, and the raw base check.
    source, _ = apply_entry(_cases()[two], {"edits": rows["t_withheld_duplicate_multiset"][0], "rehash": "all"})
    base = _i4p_projected(source)
    for check in (validate_transport_metadata, lambda s: c._source_contract(s, check_receipt=False)):
        with pytest.raises(ValueError) as error:
            check(deepcopy(base))
        assert str(error.value) == "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: support attribution differs between cases"
    with pytest.raises(ValueError) as error:
        c._source_contract(deepcopy(base))
    assert str(error.value) == "SOURCE_PREVIEW_PHYSICS_EVIDENCE_INVALID: withheld support record and diagnostic disagree"
    equal, _ = apply_entry(_cases()[two], {"edits": rows["equal multisets, in another order"][0], "rehash": "all"})
    validate_transport_metadata(_i4p_projected(equal))
    # The extrema-number demand, kept: refused bound, unbound and on transport.
    extremum = ["contract_evidence", "preview_cases", 0, "pipe_stress_extrema", 0]
    for key, value in (("global_upper_bound_pa", "x"), ("certified_gap_pa", None)):
        assert _i4p_three(O_BASE, [_set(extremum + [key], value)])[0] == (I4P_INVALID,) * 3, key


def test_i4p_ruling5_kernel_reason_without_a_run_is_g5_attempt_in_the_reader():
    """Ruling 5 (RV113's SR-PY addendum N-2): C2's kernel branch through the whole reader. A kernel reason on a
    case with no Run (`r2:cb_kernel_no_run`) is G5 ATTEMPT_MISMATCH, bound and unbound, refused by the check
    itself with no cause (D16), never by the fail-closed fallback's G5 PRODUCT_ATTEMPT."""
    case = {"basis_ref": {"ref_type": "load_case", "ref_id": UNAVAILABLE_ROW}, "ordinary": {"attempt_ref": 1, "quality_binding": {"kind": "present", "index": 1}},
            "product_attempt_ref": None, "status": "unavailable",
            "reason": {"code": "kernel_refused", "phase": "kernel", "cause": {"space": "refusal", "tag": "structure"}},
            "diagnostic_ref": "diagnostic:retained:unavailable-row", "run": None, "source_ref": None}
    source, invocation = apply_entry(_cases()[P_BASE], {"edits": [_set(B + ["cases", 1], case), {"path": B + ["product_attempts", 1], "op": "remove"}], "rehash": "all"})
    for inv in (invocation, None):
        with pytest.raises(rp.RetainedPrecisionError) as error:
            rp.validate_retained_precision(deepcopy(source), deepcopy(inv))
        assert (error.value.gate, error.value.code, error.value.__cause__) == (*ATTEMPT, None)


# ---------------------------------------------------------------------------------------------
# Snapshot 07n (B1 SC, I100; R/BRIEFS/B1_SC.md; PLAN_v2 §2.5), appended to 07m: W-C2's two producer-solved
# bases, `d38_beside_selected`, the out-of-order-authored and SF-2 successors, and 290 entries. Every new
# entry states its bound expectation (per reader only in a declared class: Rust's own raw G7 code where
# Python and TS share one; B1_SC items 12-14), and its unbound and transport reads: `expected_unbound`
# (or, for a per-reader entry, `expected_unbound_by_reader`) and `expected_transport`, each "pass"
# (admitted, not eligible) or a gate and code. An admitted rewrite that changes the classes states its own
# `expected_classifications`. Detail texts are not pinned.
# ---------------------------------------------------------------------------------------------
N07_W_C2 = {"sparse_interactive": ("7922e3e5278d0d87dc5faf79dfbc1f2a384899e97df306cc742355cdacdb6269", "cccb9664e1c58f0582348df3348d8b6e0b0941bcb0294a5b351a4d092ed18886"),
            "dense_scrutiny": ("c11f7566f1f0c469bc0a2808466dd9dd137ea64abed327fb9e4d35ff92ded22f", "ca6a62a6187a08d7b2e2643911fd232b02076b9032be1455754ff780540995f2")}
# PR-N (I109): the dense W-C2 and (C, B, A) successors re-pinned at the correctly rounded `rigid:N0`
# support force magnitude (one ulp above macOS libm's hypot chain), one pin for every platform.
# PP `retained_facade_tests.rs` REVERSED_PINNED, CBA_PINNED and AA2_PINNED: (receipt sha256, successor bytes sha256).
N07_PP_PINNED = {
    "cause_milestone_reversed_sparse_interactive": ("b79f691a4a899e3ed4a7d13957f74d4c4ee7430ee3062396214261144cbabd82", "93aa043538bbc01e0a7bcef4e381f88468edfd6792769b7041d7f6483ad959b1"),
    "cause_milestone_reversed_dense_scrutiny": ("c6b03683c8a2591f4d59429407608a6aafd0abf25663c7d771de60f221135832", "b70edc6d002c92692f00a247702353f79e6410acf17eb385b3d2c2500ca76d8e"),
    "sf2_c_b_a_sparse_interactive": ("863d692fa90d450240cbfacec1628937b8ccc9af2396637d4f913cc0bc416e80", "ea9a484657ca3de9a831a737b6a682966cc4b1a8783114b26298f34571cc7ebb"),
    "sf2_c_b_a_dense_scrutiny": ("255785d20cf0aa9f497ea324d744eb3e946871d5aac863ed8d0081d0521e8c92", "a320a5d33707c1fc8c12a35de624720dddbc35084979522c96ab1906e17c708f"),
    "sf2_a_a2_sparse_interactive": ("41f330856d4c6e94e2e1308fcd467818604c8f7e999ce499c8f3b49e26ef0d56", "529233eb989aca3553bdedfc9d1813ab3287675748ff076c27cb024c2b7fef63"),
    "sf2_a_a2_dense_scrutiny": ("30001ccf42ad12acd9dbb458392fee514946c1a52aa0d4e5f0b20bde5b09ea71", "f4075cdc80eff27099a28f787cec07080b745eca2d598eb3381a84ec03964176"),
}
N07_BASES = ["w_c2_sparse_interactive", "w_c2_dense_scrutiny", "d38_beside_selected"] + list(N07_PP_PINNED)
N07_KEYS = {"id", "base", "edits", "invocation_edits", "after_rehash", "rehash", "expected", "expected_by_reader", "expected_eligibility",
            "expected_classifications", "expected_unbound", "expected_unbound_by_reader", "expected_transport"}


def _n07_entries():
    c = corpus()
    return c["mutations"][294:] + c["must_pass"][28:]


def test_snapshot_07n_appends_only_and_states_every_read():
    c = corpus()
    assert [x["id"] for x in c["cases"][17:]] == N07_BASES
    assert (len(c["mutations"]) - 294, len(c["must_pass"]) - 28) == (240, 50)
    new = _n07_entries()
    assert all(set(e) <= N07_KEYS and e["rehash"] == "all" and "after_rehash" not in e for e in new)
    assert all(("expected_eligibility" in e) == (e in c["must_pass"]) and (e["expected"] == "pass") == (e in c["must_pass"]) for e in new)
    assert all("expected_transport" in e and ("expected_unbound" in e) != ("expected_unbound_by_reader" in e) for e in new)
    per = [e for e in new if "expected_by_reader" in e]
    assert len(per) == 45 and all("expected_unbound_by_reader" in e for e in per)
    # The declared class only: Python and TS share the expectation, Rust's raw code differs, all at G7, and the
    # unbound read is the same raw read per reader.
    assert all(e["expected_by_reader"]["python"] == e["expected_by_reader"]["typescript"] == e["expected"] != e["expected_by_reader"]["rust"]
               and e["expected"]["gate"] == e["expected_by_reader"]["rust"]["gate"] == "G7" and e["expected_unbound_by_reader"] == e["expected_by_reader"] for e in per)
    assert sum("expected_classifications" in e for e in c["must_pass"][28:]) == 16
    eligible = lambda items, key: sum(item[key]["numerical_eligible"] for item in items)
    assert (eligible(c["cases"], "expected"), eligible(c["must_pass"], "expected_eligibility")) == (19, 46)
    entries = c["mutations"] + c["must_pass"]
    assert len({e["id"] for e in entries}) == len(entries) == 612


def _n07_read(read, source):
    try:
        result = read(deepcopy(source))
    except rp.RetainedPrecisionError as error:
        return {"gate": error.gate, "code": error.code}
    assert not result["invocation_bound"] and not result["numerical_eligible"]
    return "pass"


@pytest.mark.parametrize("entry", corpus()["mutations"][294:] + corpus()["must_pass"][28:], ids=lambda x: x["id"])
def test_snapshot_07n_unbound_and_transport_reads(entry):
    """07n: each entry's unbound read (no invocation) and transport read, as the corpus states them for Python."""
    fixture = next(f for f in corpus()["cases"] if f["id"] == entry["base"])
    source, _ = apply_entry(fixture, entry)
    unbound = entry["expected_unbound"] if "expected_unbound" in entry else entry["expected_unbound_by_reader"]["python"]
    assert _n07_read(rp.validate_retained_precision, source) == unbound
    assert _n07_read(rp.validate_retained_precision_transport, source) == entry["expected_transport"]


def test_snapshot_07n_bases_are_the_pinned_successors_and_the_d38_derivation():
    """07n's producer-solved bases are D-U6-5 copies: W-C2's committed fixtures (I85, I3), and the successors PP's
    own tests pin (receipt and bytes sha256); `d38_beside_selected` is W-C2 sparse rewritten by DESIGN_v2 §2's
    derivation (the records script's steps), resealed. Each passes with its invocation as stated, and is never
    eligible without it."""
    import hashlib
    c = corpus()
    bases = {x["id"]: x for x in c["cases"][17:]}
    for mode, (file_sha, receipt_sha) in N07_W_C2.items():
        base = bases[f"w_c2_{mode}"]
        raw = (ROOT / base["provenance"]["fixture"]).read_bytes()
        assert hashlib.sha256(raw).hexdigest() == base["provenance"]["fixture_sha256"] == file_sha
        doc = json.loads(raw)
        assert doc["id"] == base["id"] and all(json.dumps(base[k]) == json.dumps(doc[k]) for k in ("source", "invocation"))
        assert receipt_sha is None or base["source"]["retained_precision"]["receipt_sha256"] == receipt_sha
    for bid, (receipt_sha, bytes_sha) in N07_PP_PINNED.items():
        p = bases[bid]["provenance"]
        assert (bases[bid]["source"]["retained_precision"]["receipt_sha256"], p["receipt_sha256"], p["successor_bytes_sha256"]) == (receipt_sha, receipt_sha, bytes_sha)
        assert p["kind"] == "producer_solved" and p["solver_mode"] == bases[bid]["invocation"]["solver_mode"]
    w = bases["w_c2_sparse_interactive"]
    body = w["source"]["retained_precision"]["body"]
    stages = dict.fromkeys(rp.STAGE_ORDER, "not_entered")
    stages.update(preparation="completed", native="failed")
    after = body["cases"][0]["run"]["invocation_after"]
    a1 = B + ["product_attempts", 1]
    derivation = [
        _set(B + ["cases", 2, "run"], None),
        _set(B + ["cases", 2, "reason"], {"code": "source_unavailable", "phase": "preparation", "cause": {"kind": "prepared_product_failure", "product_attempt_ref": 1}}),
        _set(a1 + ["run_ref"], None), _set(a1 + ["proof"], None), _set(a1 + ["stages"], stages),
        _set(a1 + ["result"], {"kind": "unavailable", "error": {"kind": "capture", "cause": {"kind": "origin", "cause": {"kind": "capacity"}}}}),
        _set(B + ["builds"], [b for b in body["builds"] if b["origin"]["run"] != 1]),
        _set(B + ["calls", 0, "owner_refs"], [{"kind": "case", "index": 0}]), _set(B + ["calls", 0, "run_refs"], [0]),
        _set(B + ["calls", 0, "source_refs"], [0]), _set(B + ["calls", 0, "invocation_after"], after),
        _set(B + ["groups", 0, "source_refs"], [0]), _set(B + ["work", "charged"], after),
        _set(B + ["work", "execution_order"], [{"kind": "case", "index": 0}]),
    ]
    derived = apply_mutation(w["source"], {"edits": derivation, "rehash": "all"})
    assert json.dumps(derived, sort_keys=True) == json.dumps(bases["d38_beside_selected"]["source"], sort_keys=True)
    assert bases["d38_beside_selected"]["provenance"] == SYNTHETIC_PROVENANCE
    for base in bases.values():
        result = rp.validate_retained_precision(deepcopy(base["source"]), deepcopy(base["invocation"]))
        assert {k: result[k] for k in ("invocation_bound", "numerical_eligible", "standing")} == base["expected"], base["id"]
        assert result["classifications"] == base["expected_classifications"], base["id"]
        assert rp.validate_retained_precision(deepcopy(base["source"]))["numerical_eligible"] is False
