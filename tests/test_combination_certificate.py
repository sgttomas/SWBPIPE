"""RT-A3 §4 combination certificate (DEL-05-02 Work ``combination-certificate.md`` §3): the Python consumer.

Over the shared corpus (``fixtures/results/combination_certificate_corpus.json``, written by PP
``tests/combination_certificate.rs``):

- the Python mirror (``tests/verifiers/combination_certificate.py``) admits every producer document with and without
  its model and refuses every forgery of ``fixtures/results/combination_certificate_forgeries.json`` with the code the
  Rust reader gives (RE ``tests/combination_certificate_contract.rs`` applies the same list);
- every document validates against its result schema, and not without the new member's definition;
- exact enclosure against the independent references: every derived component satisfies
  | |v_D,c| - |D_c| | <= B_D, and every derived support magnitude (v_M - B_D)^2 <= sum D_c^2 <= (v_M + B_D)^2, in
  ``fractions.Fraction`` with pi enclosed: the cantilever's subtractions and mechanics sum on EX and PF
  (``validation/references/combination_certificate``), op1a V-2, and the W-16b line's S1 and K1
  (``validation/references/w16_exact_combinations``).
"""
from __future__ import annotations

import copy
from fractions import Fraction
import json
import math
import os
from pathlib import Path
import re

import pytest

from schema_validation import schema_for_definition, validate_instance
from tests.verifiers import combination_certificate as cc, reader_verdict

PROJECT = Path(__file__).resolve().parents[1]
CORPUS = json.loads((PROJECT / "fixtures/results/combination_certificate_corpus.json").read_text(encoding="utf-8"))
FORGERIES_PATH = PROJECT / "fixtures/results/combination_certificate_forgeries.json"
REFERENCES = json.loads((PROJECT / "validation/references/combination_certificate/references.json").read_text(encoding="utf-8"))
W16 = json.loads((PROJECT / "validation/references/w16_exact_combinations/references.json").read_text(encoding="utf-8"))
PREFIX = "SOURCE_COMBINATION_CERTIFICATE_"
MEMBER = ("contract_evidence", "combination_certificate")


def entry(name: str) -> dict:
    return copy.deepcopy(next(e for e in CORPUS["documents"] if e["name"] == name))


def certificate(document: dict) -> dict:
    return document["contract_evidence"]["combination_certificate"]


@pytest.mark.parametrize("name", [e["name"] for e in CORPUS["documents"]])
def test_producer_documents_are_admitted_with_and_without_their_models(name):
    e = entry(name)
    cc.check(e["document"])
    assert reader_verdict.validate(e["reader"], e["document"]) == {"ok": True}
    assert reader_verdict.validate(e["reader"], e["document"], e["model"]) == {"ok": True}


def test_a_non_ascii_combination_identity_is_length_prefixed_in_utf8_bytes():
    """Regression: the diagnostic identity's length prefix counts UTF-8 bytes, as the producer and the Rust reader
    count them (`14:combination:Σ`, 13 characters); the unicode-ids capture is admitted."""
    document = json.loads((PROJECT / "fixtures/results/preview_physics_unicode_ids_sparse.json").read_text(encoding="utf-8"))
    stated = [d["id"] for d in document["diagnostics"] if d["id"].startswith("diagnostic:combination-certificate:")]
    assert stated == ["diagnostic:combination-certificate:14:combination:Σ"]
    cc.check(document)
    assert reader_verdict.validate("preview-physics", document) == {"ok": True}


def test_transport_reads_the_member_shape_only():
    """Transport is metadata-only: the member is admitted on its closed shape, also without results, diagnostics or
    status, and a malformed member is refused by name (Rust `transport_reads_the_member_shape_only` alike)."""
    document = entry("pf_cantilever")["document"]
    assert reader_verdict.transport("preview-physics", document) == {"ok": True}
    metadata = {k: v for k, v in document.items() if k not in {"results", "diagnostics", "status"}}
    assert reader_verdict.transport("preview-physics", metadata) == {"ok": True}
    for forged in ([], {"combinations": []}, {"combinations": [], "operand_cases": {}}):
        changed = copy.deepcopy(document)
        changed["contract_evidence"]["combination_certificate"] = forged
        verdict = reader_verdict.transport("preview-physics", changed)
        assert verdict["ok"] is False and "transport combination certificate shape" in json.dumps(verdict), verdict


# ----------------------------------------------------------------------------- forgeries (shared with the Rust reader)

def _pointer(path: list) -> str:
    return "/" + "/".join(str(p).replace("~", "~0").replace("/", "~1") for p in path)


def _record_index(document: dict, combination: str) -> int:
    return next(i for i, r in enumerate(certificate(document)["combinations"]) if r["combination_id"] == combination)


def _diagnostic_index(document: dict, predicate) -> int:
    return next(i for i, d in enumerate(document["diagnostics"]) if predicate(d))


def build_forgeries() -> list[dict]:
    """Every forgery as pointer edits (`set` or `remove`) with the code both readers give (None: admitted)."""
    out = []

    def edit(path, value=None, op="set"):
        item = {"op": op, "pointer": _pointer(path)}
        if op in ("set", "add"):
            item["value"] = value
        return item

    def add(name, path, code, what, value=None, op="set", model_code=None):
        item = {"document": name, "edits": [edit(path, value, op)], "code": code, "what": what}
        if model_code is not None:
            item["model_code"] = model_code
        out.append(item)

    pf, ex, w16 = entry("pf_cantilever")["document"], entry("ex_cantilever")["document"], entry("w16_line")["document"]
    cert = ["contract_evidence", "combination_certificate"]
    double = _record_index(pf, "comb:double")
    g = 0
    group = certificate(pf)["combinations"][double]["groups"][g]
    add("pf_cantilever", cert + ["combinations", double, "groups", g, "bound"], "RECORD", "forged B_D", math.nextafter(group["bound"], math.inf))
    add("pf_cantilever", cert + ["combinations", double, "groups", g, "q"], "RECORD", "forged q", math.nextafter(group["q"], math.inf))
    add("pf_cantilever", cert + ["combinations", double, "groups", g, "floor_scale"], "RECORD", "forged floor scale", 0.0)
    cancel = _record_index(pf, "comb:cancel")
    k = next(i for i, x in enumerate(certificate(pf)["combinations"][cancel]["groups"]) if x["verdict"] == "not_established")
    add("pf_cantilever", cert + ["combinations", cancel, "groups", k, "verdict"], "RECORD", "forged verdict", "established")
    add("pf_cantilever", cert + ["combinations", double, "uncovered_row_count"], "RECORD", "forged uncovered count",
        certificate(pf)["combinations"][double]["uncovered_row_count"] + 1)
    groups = certificate(pf)["operand_cases"][0]["groups"]
    covered = [i for i, x in enumerate(groups) if any(r is not None for r in x["rows"])]
    a, b = covered[0], covered[1]
    add("pf_cantilever", cert + ["operand_cases", 0, "groups", a, "bound"], "RECORD", "forged operand bound",
        math.nextafter(groups[a]["bound"], math.inf))
    add("pf_cantilever", cert + ["operand_cases", 0, "groups", a, "rows"], "OPERAND_ROWS", "forged operand row binding", groups[b]["rows"])
    add("pf_cantilever", cert + ["operand_cases", 0, "groups", a, "q"], "OPERAND_ROWS", "forged operand q", math.nextafter(groups[a]["q"], math.inf))
    add("pf_cantilever", cert + ["operand_cases", 0, "groups", a, "moment"], "OPERAND_GROUP_CLASS", "forged group class", not groups[a]["moment"])
    add("op1a_v1", cert + ["operand_cases"], "OPERAND_ELIGIBILITY", "a gap-bearing operand made eligible", [{"load_case_id": "case:S", "groups": []}])
    add("op1a_v1", cert + ["combinations", 0, "assessment"], "RECORD", "a gap-operand combination made assessed", "assessed")
    add("op1a_v2", cert + ["operand_cases", 0], "OPERAND_CASE_MISSING", "an operand statement dropped", op="remove")
    within = _diagnostic_index(ex, lambda d: d["code"] == cc.WITHIN)
    add("ex_cantilever", ["diagnostics", within, "message"], "DIAGNOSTICS", "forged notice", "combination comb:double: certified")
    add("ex_cantilever", ["diagnostics", within], "DIAGNOSTICS", "notice dropped", op="remove")
    add("ex_cantilever", cert, "PRESENCE", "member dropped, its notices kept", op="remove")
    # Absence is no certificate claim, whatever the document's age: member and notices absent are admitted, claiming nothing.
    notices = [i for i, d in enumerate(ex["diagnostics"]) if d["id"].startswith(cc.DIAGNOSTIC_PREFIX)]
    out.append({"document": "ex_cantilever", "what": "member and its notices absent (no certificate claim)",
                      "code": None, "edits": [edit(cert, op="remove"), *(edit(["diagnostics", i], op="remove") for i in reversed(notices))]})
    pf_notices = [i for i, d in enumerate(pf["diagnostics"]) if d["id"].startswith(cc.DIAGNOSTIC_PREFIX)]
    out.append({"document": "pf_cantilever", "what": "member and its notices absent (no certificate claim)",
                      "code": None, "edits": [edit(cert, op="remove"), *(edit(["diagnostics", i], op="remove") for i in reversed(pf_notices))]})
    add("ex_cantilever", cert + ["certified"], "SHAPE", "extra member key", True)
    axial = _record_index(ex, "comb:axial")
    wall = next(i for i, x in enumerate(certificate(ex)["combinations"][axial]["groups"]) if x["equal_pressure"] is not None)
    add("ex_cantilever", cert + ["combinations", axial, "groups", wall, "equal_pressure"], "RECORD", "D-6 statement dropped", None)
    ex_double = _record_index(ex, "comb:double")
    add("ex_cantilever", cert + ["combinations", ex_double, "expression"], "EXPRESSION", "swapped operands",
        {"minuend_id": "case:S", "subtrahend_id": "case:O2"}, model_code="MODEL_BINDING")
    integrity = _diagnostic_index(ex, lambda d: d["id"] == "diagnostic:numerical-integrity:case:S")
    add("ex_cantilever", ["diagnostics", integrity, "code"], "OPERAND_ELIGIBILITY", "operand standing without a member bound",
        "NUMERICAL_INTEGRITY_SENSITIVE")
    # Premise: the operand bounds are each operand case's own; a held subtrahend is not assessed (fail closed).
    add("pf_cantilever", cert + ["combinations", double, "subtrahend_support_state"], "RECORD", "an assessed record named held", "minuend_final",
        model_code="MODEL_BINDING")
    add("pf_cantilever", cert + ["combinations", double, "subtrahend_support_state"], "RECORD", "an unrecognized support state fails closed", "frozen",
        model_code="MODEL_BINDING")
    add("pf_cantilever", cert + ["combinations", double, "subtrahend_support_state"], "RECORD_SHAPE", "a non-text support state", 1,
        model_code="MODEL_BINDING")
    # Synthetic envelopes (this base withholds or refuses the member upstream): a held subtraction stated not assessed,
    # with no group, its subtrahend's other uses kept and its notice the not-assessed one, is admitted by both readers.
    reason = cc.held_reason("minuend_final")
    for name, combination, subtrahend_only_here in (("ex_cantilever", "comb:double", "case:O2"), ("pf_cantilever", "comb:double", "case:O2")):
        doc = entry(name)["document"]
        index = _record_index(doc, combination)
        record = certificate(doc)["combinations"][index]
        rows_of = [r for r in doc["results"] if r["basis_ref"] == {"ref_type": "combination", "ref_id": combination}
                   and r["kind"] != "combination_modulus_basis_record"]
        notice = _diagnostic_index(doc, lambda d: d["code"] == cc.WITHIN and d["affected_refs"] == [combination])
        o2 = next(i for i, c in enumerate(certificate(doc)["operand_cases"]) if c["load_case_id"] == subtrahend_only_here)
        synthetic = [
            edit(cert + ["combinations", index], dict(record, subtrahend_support_state="minuend_final", assessment="not_assessed",
                                                       reason=reason, groups=[], uncovered_row_count=len(rows_of))),
            edit(["diagnostics", notice], cc._diagnostic(combination, cc.NOT_ASSESSED, cc.not_assessed_message(combination, reason))),
            edit(cert + ["operand_cases", o2], op="remove"),
        ]
        # Their model solves the subtrahend on its own support state, so with it the held record is the model's
        # contradiction (MODEL_BINDING) before the certificate's own reading.
        held = {"model_code": "MODEL_BINDING"}
        out.append({"document": name, "edits": synthetic, "code": None, "what": f"{name}: a held subtraction stated not assessed is admitted", **held})
        out.append({"document": name, "edits": synthetic[:1] + synthetic[2:], "code": "DIAGNOSTICS", "what": f"{name}: a held subtraction keeps its assessed notice", **held})
        out.append({"document": name, "edits": synthetic[:2], "code": "OPERAND_CASES", "what": f"{name}: a held subtraction keeps its subtrahend's operand statement", **held})
    # The absence matrix (review of the certificate, threat cases (b)-(e)) on both actual envelopes. Absence of the
    # member is no certificate claim; a certificate notice is recognized by its reserved id namespace, code family or
    # canonical text, never by its id spelling alone.
    for name, doc in (("ex_cantilever", ex), ("pf_cantilever", pf)):
        ids = [i for i, d in enumerate(doc["diagnostics"]) if d["id"].startswith(cc.DIAGNOSTIC_PREFIX)]
        cancel_id = "comb:cancel"
        within_text = next(d["message"] for d in doc["diagnostics"] if d["code"] == cc.WITHIN and d["affected_refs"] == [cancel_id])
        drop_notices = [edit(["diagnostics", i], op="remove") for i in reversed(ids)]
        renamed = [edit(["diagnostics", i, "id"], doc["diagnostics"][i]["id"].replace("combination-certificate:", "renamed-certificate:"))
                   for i in ids]
        invented = {"id": "diagnostic:review:bound", "code": cc.WITHIN, "severity": "info", "source": cc.SOURCE,
                    "affected_refs": [cancel_id], "message": f"combination {cancel_id}: all derived row groups are established within their stated bound."}
        canonical_elsewhere = {"id": "diagnostic:review:note", "code": "REVIEW_NOTE", "severity": "info", "source": cc.SOURCE,
                               "affected_refs": [cancel_id], "message": within_text}
        if name == "pf_cantilever":
            add(name, cert, "PRESENCE", f"{name}: member dropped, its notices kept", op="remove")
        out.append({"document": name, "what": f"{name}: notices dropped, the member kept", "code": "DIAGNOSTICS", "edits": drop_notices})
        out.append({"document": name, "what": f"{name}: member dropped, its notices kept under renamed ids", "code": "PRESENCE",
                    "edits": [edit(cert, op="remove"), *renamed]})
        out.append({"document": name, "what": f"{name}: member and notices dropped, an establishment sentence under the reserved code",
                    "code": "PRESENCE", "edits": [edit(cert, op="remove"), *drop_notices, edit(["diagnostics", "-"], invented, op="add")]})
        out.append({"document": name, "what": f"{name}: member and notices dropped, the canonical sentence under another code and id",
                    "code": "PRESENCE", "edits": [edit(cert, op="remove"), *drop_notices, edit(["diagnostics", "-"], canonical_elsewhere, op="add")]})
        out.append({"document": name, "what": f"{name}: member kept, an extra establishment sentence under another id", "code": "DIAGNOSTICS",
                    "edits": [edit(["diagnostics", "-"], invented, op="add")]})
        out.append({"document": name, "what": f"{name}: member kept, its notices under renamed ids", "code": "DIAGNOSTICS", "edits": renamed})
        add(name, cert, "SHAPE", f"{name}: a null member", None)
        add(name, cert + ["operand_cases"], "SHAPE", f"{name}: operand_cases missing", op="remove")
        add(name, cert + ["operand_cases"], "SHAPE", f"{name}: operand_cases null", None)
        add(name, cert + ["combinations"], "SHAPE", f"{name}: combinations an object", {})
        add(name, cert + ["operand_cases", 0, "groups"], "SHAPE", f"{name}: an operand case's groups an object", {})
        # Review cert2 finding 2: a nested record that is not an object is refused by its shape, with the same code
        # with and without the model (the model binding leaves it to the shape check), and never raises.
        for value, what in ((None, "null"), (7, "a scalar"), ([], "an array")):
            add(name, cert + ["combinations", 0], "RECORD_SHAPE", f"{name}: a combination record {what}", value)
            add(name, cert + ["operand_cases", 0], "SHAPE", f"{name}: an operand case {what}", value)
            add(name, cert + ["operand_cases", 0, "groups", 0], "SHAPE", f"{name}: an operand group {what}", value)
    # A stated member is always fully checked: wrong-type arrays on the gap document (no assessed combination) are
    # refused by their shape, never read as empty.
    for value, what in (({}, "an object"), (None, "null"), ("not an array", "a string")):
        add("op1a_v1", cert + ["operand_cases"], "SHAPE", f"op1a_v1: operand_cases {what}", value)
    add("op1a_v1", cert + ["combinations"], "SHAPE", "op1a_v1: combinations a string", "not an array")
    k1 = _record_index(w16, "comb:K1")
    add("w16_line", cert + ["combinations", k1, "groups", 0, "factors_bound"], "RECORD", "forged factor bound",
        math.nextafter(certificate(w16)["combinations"][k1]["groups"][0]["factors_bound"], math.inf))
    return out


def test_forgeries_file_is_current():
    built = build_forgeries()
    if os.environ.get("CERT_WRITE_FORGERIES"):
        FORGERIES_PATH.write_text(json.dumps({"description": "RT-A3 §4 combination-certificate forgeries over combination_certificate_corpus.json: "
                                              "JSON-pointer edits each and the code the Rust reader and the Python mirror both give (null: admitted). "
                                              "Written by tests/test_combination_certificate.py (CERT_WRITE_FORGERIES=1).", "forgeries": built},
                                             indent=1, ensure_ascii=True) + "\n", encoding="utf-8")
    assert json.loads(FORGERIES_PATH.read_text(encoding="utf-8"))["forgeries"] == built


def _apply(document: dict, forgery: dict) -> dict:
    for edit in forgery["edits"]:
        _apply_one(document, edit)
    return document


def _apply_one(document: dict, forgery: dict) -> dict:
    parts = [p.replace("~1", "/").replace("~0", "~") for p in forgery["pointer"].split("/")[1:]]
    target = document
    for p in parts[:-1]:
        target = target[int(p)] if isinstance(target, list) else target[p]
    last = parts[-1]
    if forgery["op"] == "add":
        if last == "-":
            target.append(forgery["value"])
        else:
            target.insert(int(last), forgery["value"])
    elif forgery["op"] == "remove":
        if isinstance(target, list):
            target.pop(int(last))
        else:
            target.pop(last)
    else:
        original = target[int(last)] if isinstance(target, list) else target.get(last, "<absent>")
        assert original != forgery["value"], f"stale forgery: {forgery['pointer']}"
        if isinstance(target, list):
            target[int(last)] = forgery["value"]
        else:
            target[last] = forgery["value"]
    return document


@pytest.mark.parametrize("with_model", [False, True], ids=["without-model", "with-model"])
@pytest.mark.parametrize("forgery", json.loads(FORGERIES_PATH.read_text(encoding="utf-8"))["forgeries"], ids=lambda f: f["what"])
def test_every_forgery_is_refused_with_the_rust_code(forgery, with_model):
    """Each forgery, without and with its model; `model_code` is the code with the model where the model contradicts
    the forged record first (Rust `every_forgery_is_refused_with_the_shared_code` alike)."""
    e = entry(forgery["document"])
    forged = _apply(e["document"], forgery)
    code = forgery.get("model_code", forgery["code"]) if with_model else forgery["code"]
    outcome = reader_verdict.validate(e["reader"], forged, e["model"] if with_model else None)
    if code is None:
        assert outcome == {"ok": True}, (forgery["what"], outcome)
    else:
        assert outcome["ok"] is False and PREFIX + code in json.dumps(outcome), (forgery["what"], outcome)


# ----------------------------------------------------------------------------- the absence rule (review matrix)

def _forgery(document: str, what: str) -> dict:
    return next(f for f in json.loads(FORGERIES_PATH.read_text(encoding="utf-8"))["forgeries"] if (f["document"], f["what"]) == (document, what))


def _claims_survive(document: dict) -> list:
    """Any §4 certificate claim the document still makes: the member (its bounds and verdicts), or a diagnostic in the
    certificate's vocabulary, restated here independently of the readers (id namespace, the `COMBINATION_ROWS_` code
    family, or a sentence placing rows within, or assessing them against, a stated bound). W-16b's derived-rows
    notice ("only the derived row groups that contract_evidence.combination_certificate establishes carry a stated
    bound") is conditional on the member and claims nothing without it."""
    found = [] if "combination_certificate" not in document["contract_evidence"] else ["member"]
    for d in document["diagnostics"]:
        message = d.get("message", "")
        if (d["id"].startswith("diagnostic:combination-certificate:") or d["code"].startswith("COMBINATION_ROWS_")
                or "established within" in message or "within their stated bound" in message
                or "within its stated bound" in message or "against a stated bound" in message):
            found.append(d["id"])
    return found


@pytest.mark.parametrize("with_model", [False, True], ids=["without-model", "with-model"])
@pytest.mark.parametrize("name", ["ex_cantilever", "pf_cantilever"])
def test_absence_is_inspection_only_with_no_certificate_claim(name, with_model):
    """Absence of the member (member and notices both dropped, the current identity kept) is admitted as no §4
    certificate claim: no member, no certificate notice under any id, and no eligibility effect, the document's
    numerical_quality and its desktop standing being those of the certified document (decision (b))."""
    e = entry(name)
    model = e["model"] if with_model else None
    stripped = _apply(copy.deepcopy(e["document"]), _forgery(name, "member and its notices absent (no certificate claim)"))
    assert reader_verdict.validate(e["reader"], stripped, model) == {"ok": True}
    assert _claims_survive(stripped) == []
    assert _claims_survive(e["document"]) != []
    assert stripped["numerical_quality"] == e["document"]["numerical_quality"]
    assert stripped["formulation_basis"] == e["document"]["formulation_basis"]
    assert reader_verdict.standing(stripped, model) == reader_verdict.standing(e["document"], model)


W_CB3 = {
    "sparse_interactive": "7a1b99f7ac24e2990472830054310ec1276d9804a86dd0111c290749a134ddd6",
    "dense_scrutiny": "3f38c729bbed2d484459c13b21c2b8bc08fb9d06f691b90740286f492b87824e",
}


@pytest.mark.parametrize("mode", sorted(W_CB3))
def test_legacy_w_cb3_successors_are_accepted_with_their_own_receipt_and_no_certificate_claim(mode):
    """The registered W-CB3 retained combination successors (recorded before the certificate, bytes untouched) are
    accepted with and without their invocation; their retained-precision standing is their own receipt's, and they
    make no §4 certificate claim (Rust `legacy_w_cb3_successors_are_accepted` alike)."""
    import hashlib

    from tests.verifiers import retained_precision

    raw = (PROJECT / f"fixtures/results/retained_precision_combination_successor_{mode}.json").read_bytes()
    assert hashlib.sha256(raw).hexdigest() == W_CB3[mode]
    w = json.loads(raw)
    assert _claims_survive(w["source"]) == []
    bound = retained_precision.validate_retained_precision(w["source"], w["invocation"])
    assert (bound["invocation_bound"], bound["numerical_eligible"], bound["standing"]) == (True, True, "eligible"), bound
    unbound = retained_precision.validate_retained_precision(w["source"])
    assert (unbound["invocation_bound"], unbound["standing"]) == (False, "needs_recompute"), unbound


def test_model_binding_refuses_a_held_subtrahend_missing_from_the_record():
    e = entry("pf_cantilever")
    model = copy.deepcopy(e["model"])
    for c in model["combinations"]:
        if c["id"] == "comb:double":
            c["subtrahend_support_state"] = "minuend_final"
    with pytest.raises(ValueError, match=PREFIX + "MODEL_BINDING"):
        cc.model_checks(e["document"], model)
    cc.model_checks(e["document"], e["model"])


def test_model_binding_refuses_a_forged_expression():
    e = entry("pf_cantilever")
    model = copy.deepcopy(e["model"])
    for c in model["combinations"]:
        if c["id"] == "comb:double":
            c["minuend_id"], c["subtrahend_id"] = c["subtrahend_id"], c["minuend_id"]
    with pytest.raises(ValueError, match=PREFIX + "MODEL_BINDING"):
        cc.model_checks(e["document"], model)
    cc.model_checks(e["document"], e["model"])


# ----------------------------------------------------------------------------- schemas

def _validate_schema(document: dict, reader: str) -> None:
    """An EX producer envelope against pressure-2's raw envelope definition; a PF envelope's contract evidence against
    results.v0.3's PreviewPhysicsContractEvidence (the raw PF envelope's member, carried verbatim)."""
    import yaml

    schemas = PROJECT / "schemas"
    if reader == "pressure":
        pressure = json.loads((schemas / "results_pressure_2.v0.3.schema.json").read_text(encoding="utf-8"))
        root = schema_for_definition(pressure, "PressureResultEnvelope")
        root["$id"] = pressure["$id"]
        validate_instance(root, document, schema_label="results_pressure_2.v0.3", instance_label="certificate corpus")
    else:
        results = yaml.safe_load((schemas / "results.v0.3.schema.yaml").read_text(encoding="utf-8"))
        root = schema_for_definition(results, "PreviewPhysicsContractEvidence")
        root["$id"] = results["$id"]
        validate_instance(root, document["contract_evidence"], schema_label="results.v0.3", instance_label="certificate corpus")


@pytest.mark.parametrize("name", [e["name"] for e in CORPUS["documents"]])
def test_documents_validate_against_their_result_schema(name):
    e = entry(name)
    _validate_schema(e["document"], e["reader"])
    forged = copy.deepcopy(e["document"])
    forged["contract_evidence"]["combination_certificate"]["unexpected"] = 1
    with pytest.raises(Exception):
        _validate_schema(forged, e["reader"])


# ----------------------------------------------------------------------------- exact enclosure

PI_DIGITS = 130


def _pi_bounds() -> tuple[Fraction, Fraction]:
    def arctan_inverse(x, scale):
        total, term, n, sign, x2 = 0, scale // x, 1, 1, x * x
        while term:
            total += sign * (term // n)
            term //= x2
            n += 2
            sign = -sign
        return total

    scale = 10 ** PI_DIGITS
    pi = 4 * (4 * arctan_inverse(5, scale) - arctan_inverse(239, scale))
    return Fraction(pi, scale) - Fraction(1, 10 ** 120), Fraction(pi, scale) + Fraction(1, 10 ** 120)


PI = _pi_bounds()


def _interval(value: dict) -> tuple[Fraction, Fraction]:
    q0, q1 = Fraction(value["q0"]), Fraction(value["q1_pi"])
    a, b = q0 + q1 * PI[0], q0 + q1 * PI[1]
    return min(a, b), max(a, b)


def _encloses(published: float, exact: tuple[Fraction, Fraction], bound: float) -> bool:
    """| |v| - |D| | <= B for some |D| in the (nonnegative) enclosure."""
    v, b = abs(Fraction(published)), Fraction(bound)
    return v - b <= exact[1] and exact[0] <= v + b


def _rows(document: dict) -> dict:
    return {r["id"]: r for r in document["results"]}


def _derived(document: dict, combination: str, entity: str, location: str, kind: str) -> list[dict]:
    record = next(r for r in certificate(document)["combinations"] if r["combination_id"] == combination)
    return [g for g in record["groups"] if (g["entity"], g["location"], g["group"]) == (entity, location, kind)]


@pytest.mark.parametrize("name,route", [("ex_cantilever", "cantilever_exact"), ("pf_cantilever", "cantilever_preview")])
def test_cantilever_rows_lie_within_their_bound_of_the_exact_differences(name, route):
    document = entry(name)["document"]
    rows = _rows(document)
    checked = 0
    for combination in REFERENCES[route]["combinations"]:
        for expected in combination["groups"]:
            exact = [_interval(c) for c in expected["components"]]
            for g in _derived(document, combination["combination_id"], expected["entity"], expected["location"], expected["group"]):
                values = [rows[i]["value"] for i in g["rows"]]
                assert len(values) == len(exact)
                for v, d in zip(values, exact):
                    assert _encloses(v, d, g["bound"]), (name, combination["combination_id"], expected, v, g["bound"])
                checked += 1
                if expected["group"] in ("support_force", "support_moment"):
                    magnitude = _derived(document, combination["combination_id"], expected["entity"], "node", expected["group"] + "_magnitude")
                    assert len(magnitude) == 1
                    m = magnitude[0]
                    vm, bm = Fraction(rows[m["rows"][0]]["value"]), Fraction(m["bound"])
                    low = sum(d[0] ** 2 for d in exact)
                    high = sum(d[1] ** 2 for d in exact)
                    assert (max(vm - bm, 0)) ** 2 <= high and low <= (vm + bm) ** 2, (name, m)
    assert checked > 100


def test_v2_confirmed_nonlinear_rows_lie_within_their_bound():
    document = entry("op1a_v2")["document"]
    rows = _rows(document)
    reference = REFERENCES["op1a"]["v2"]
    for expected in reference["groups"]:
        found = _derived(document, reference["combination_id"], expected["entity"], expected["location"], expected["group"])
        assert len(found) == 1
        for v, d in zip([rows[i]["value"] for i in found[0]["rows"]], [_interval(c) for c in expected["components"]]):
            assert _encloses(v, d, found[0]["bound"])


_TERM = re.compile(r"^\(?(-?\d+(?:/\d+)?)\)?(\*pi)?$")


def _w16_value(text: str) -> tuple[Fraction, Fraction] | None:
    """q0 + q1*pi from the W-16b references' exact text; None for a 1/pi term."""
    q0, q1 = Fraction(0), Fraction(0)
    for term in text.replace(" + ", "\n").split("\n"):
        term = term.strip()
        if "/pi" in term:
            return None
        m = _TERM.match(term)
        if not m:
            return None
        if m.group(2):
            q1 += Fraction(m.group(1))
        else:
            q0 += Fraction(m.group(1))
    a, b = q0 + q1 * PI[0], q0 + q1 * PI[1]
    return min(a, b), max(a, b)


@pytest.mark.parametrize("combination,least", [("comb:S1", 10), ("comb:K1", 1)])
def test_w16_line_rows_lie_within_their_bound_of_the_frozen_exact_rows(combination, least):
    """K1 = A + 1.3B: each member is pressurized in one operand only, so its axial rows in the other operand are
    pressure_free_operand_recovery values, not certified rows; those derived rows stay uncovered and only the support
    groups (and the zero shear, torsion and bending groups) are formed."""
    document = entry("w16_line")["document"]
    rows = _rows(document)
    reference = next(c for c in W16["combinations"] if c["id"] == combination)
    exact = {}
    for r in reference["rows"]:
        exact[(r["kind"], r["entity"], r["location"], r.get("component"), r.get("section_basis"))] = r
    record = next(r for r in certificate(document)["combinations"] if r["combination_id"] == combination)
    checked = 0
    for g in record["groups"]:
        for row_id in g["rows"]:
            row = rows[row_id]
            key = (row["kind"], row["entity_ref"], row["metadata"]["location"], row["metadata"]["component"], None)
            r = exact.get(key)
            if r is None:
                continue
            value = _w16_value(r["exact"])
            if value is None:
                continue
            magnitude = (min(abs(value[0]), abs(value[1])) if value[0] * value[1] > 0 else Fraction(0), max(abs(value[0]), abs(value[1])))
            assert _encloses(row["value"], magnitude, g["bound"]), (combination, key, row["value"], r["exact"], g["bound"])
            checked += 1
    assert checked >= least, checked
