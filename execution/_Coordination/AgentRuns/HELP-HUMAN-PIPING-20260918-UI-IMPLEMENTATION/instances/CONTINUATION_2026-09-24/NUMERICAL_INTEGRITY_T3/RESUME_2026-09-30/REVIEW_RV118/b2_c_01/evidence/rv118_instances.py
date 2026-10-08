"""RV118 (RV-C): my own positive and negative instances for each new $def of the merged J1 SCHEMA.

Read-only. Both validators are judged: jsonschema (Draft 2020-12) and PY's own G1 walker `_shape`,
loaded from NUM's committed reader file with its schema loader pointed at the J1 text in memory
(as I97's checks do; nothing is written into the repository). Sub-objects that the new $defs reuse
(Run, Selection, PreparedMember, adapter, proof trace) are taken from CORPUS's committed base
`two_case_synthetic`; every new object is written here.

It also checks oneOf disjointness: each positive instance of a new branch must fail every sibling branch,
so that `oneOf` is never ambiguous for an emitted value.

Usage: python rv118_instances.py <P> <PY reader file> <J1 schema> <out_json>
"""
import copy
import importlib.util
import json
import sys
from pathlib import Path

import jsonschema

H = "a" * 64
ONE = "3ff0000000000000"
HALF = "3fe0000000000000"
NEG = "bff0000000000000"


def load_reader(path):
    spec = importlib.util.spec_from_file_location("rv118_py_reader", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class Judge:
    def __init__(self, schema, reader):
        self.schema = schema
        self.reader = reader

    def js(self, value, ref):
        sub = {"$schema": self.schema["$schema"], "$defs": self.schema["$defs"], "$ref": f"#/$defs/{ref}"}
        return jsonschema.Draft202012Validator(sub).is_valid(value)

    def js_spec(self, value, spec):
        sub = {"$schema": self.schema["$schema"], "$defs": self.schema["$defs"]}
        sub.update(spec)
        return jsonschema.Draft202012Validator(sub).is_valid(value)

    def py(self, value, ref):
        self.reader._schema = lambda: self.schema
        try:
            return bool(self.reader._shape(value, {"$ref": f"#/$defs/{ref}"}))
        except (KeyError, TypeError, AttributeError, IndexError):
            return False

    def py_spec(self, value, spec):
        self.reader._schema = lambda: self.schema
        try:
            return bool(self.reader._shape(value, spec))
        except (KeyError, TypeError, AttributeError, IndexError):
            return False


def main():
    p, py_path, j1_path, out = (Path(a) for a in sys.argv[1:5])
    schema = json.loads(j1_path.read_bytes())
    judge = Judge(schema, load_reader(py_path))
    corpus = json.loads((p / "fixtures/results/retained_precision_cases.json").read_bytes())
    base = next(c for c in corpus["cases"] if c["id"] == "two_case_synthetic")["source"]["retained_precision"]
    body = base["body"]
    case_attempt = body["product_attempts"][0]
    run = copy.deepcopy(next(c["run"] for c in body["cases"] if c.get("run")))
    selection = copy.deepcopy(next(c["selection"] for c in body["cases"] if c["status"] == "selected"))
    group = copy.deepcopy(body["groups"][0])
    case_source = copy.deepcopy(body["sources"][0])

    # ---- my own instances --------------------------------------------------------------------
    mech = {"kind": "mechanics", "terms": [{"case_id": "load:A", "factor": ONE}, {"case_id": "load:B", "factor": NEG}]}
    sub = {"kind": "result_state_subtraction", "minuend_id": "load:A", "subtrahend_id": "load:B"}
    rng = {"kind": "range_envelope", "operand_ids": ["load:A"], "mode": "min_abs"}

    def entry(disposition, expression=None, **extra):
        e = {"basis_ref": {"ref_type": "combination", "ref_id": "comb:1"}, "expression": expression or mech,
             "disposition": disposition, "result_ids": ["result:combination:x"], "diagnostic_refs": ["d:1", "d:2"]}
        e.update(extra)
        return e

    run_c = copy.deepcopy(run)
    run_c["origin"]["owner_ref"] = {"kind": "combination", "index": 1}
    sel_e = entry("retained_selected", method="contribution_preserving_multiprecision_v1", call_ref=1,
                  product_attempt_ref=3, run=run_c, source_ref=4, source_identity_sha256=H, selection=selection)
    un_run = entry("retained_unavailable", call_ref=1, product_attempt_ref=3,
                   reason={"code": "combination_unresolved", "phase": "kernel",
                           "cause": {"kind": "prepared_product_failure", "product_attempt_ref": 3}},
                   diagnostic_ref="diagnostic:retained-precision:comb:1:unavailable", run=run_c, source_ref=4)
    un_facade = copy.deepcopy(un_run)
    un_facade["reason"] = {"code": "facade_certificate", "phase": "facade",
                           "cause": {"kind": "prepared_product_failure", "product_attempt_ref": 3}}
    un_pre = entry("retained_unavailable", call_ref=1, product_attempt_ref=None,
                   reason={"code": "combination_unresolved", "phase": "preparation",
                           "cause": {"space": "combination", "tag": "count_range", "name": "combination encoding"}},
                   diagnostic_ref="d", run=None, source_ref=None)
    un_ledger = copy.deepcopy(un_pre)
    un_ledger["reason"]["cause"] = {"space": "combination", "tag": "ledger_unavailable",
                                    "error": {"tag": "accumulator", "error": {"tag": "accumulator_overflow"}}}
    un_osu = copy.deepcopy(un_pre)
    un_osu.update(call_ref=None)
    un_osu["reason"]["cause"] = {"kind": "operand_source_unavailable", "operand_index": 0}
    ordinary_sub = entry("ordinary", sub, reason="no_retained_mechanics")
    ordinary_rng = entry("ordinary", rng, reason="no_retained_mechanics")
    withheld = entry("base_withheld", reason="NONLINEAR_COMBINATION_REQUIRES_SOLVE", result_ids=[])

    cases = []

    def case(ref, label, value, expect):
        cases.append((ref, label, value, expect))

    # CombinationExpression
    case("CombinationExpression", "mechanics, two terms, negative factor", mech, True)
    case("CombinationExpression", "mechanics, repeated case (schema admits; C-1 is a G5 rule)",
         {"kind": "mechanics", "terms": [{"case_id": "load:A", "factor": ONE}] * 2}, True)
    case("CombinationExpression", "mechanics, no terms", {"kind": "mechanics", "terms": []}, False)
    case("CombinationExpression", "mechanics, term with load_case member (model spelling)",
         {"kind": "mechanics", "terms": [{"load_case": "load:A", "factor": ONE}]}, False)
    case("CombinationExpression", "subtraction", sub, True)
    case("CombinationExpression", "subtraction missing subtrahend", {"kind": "result_state_subtraction", "minuend_id": "A"}, False)
    case("CombinationExpression", "range, one operand", rng, True)
    case("CombinationExpression", "range, empty operands", dict(rng, operand_ids=[]), False)
    case("CombinationExpression", "range, unknown mode", dict(rng, mode="abs"), False)
    case("CombinationExpression", "range with terms member", dict(rng, terms=[]), False)
    # Combination
    case("Combination", "retained_selected", sel_e, True)
    case("Combination", "retained_selected with null call_ref", dict(sel_e, call_ref=None), False)
    case("Combination", "retained_selected with null run", dict(sel_e, run=None), False)
    case("Combination", "retained_selected with diagnostic_ref", dict(sel_e, diagnostic_ref="d"), False)
    case("Combination", "retained_unavailable, Run not selected", un_run, True)
    case("Combination", "retained_unavailable, facade", un_facade, True)
    case("Combination", "retained_unavailable, count_range pre-source", un_pre, True)
    case("Combination", "retained_unavailable, combined ledger refusal", un_ledger, True)
    case("Combination", "retained_unavailable, operand_source_unavailable", un_osu, True)
    case("Combination", "retained_unavailable without diagnostic_ref",
         {k: v for k, v in un_run.items() if k != "diagnostic_ref"}, False)
    case("Combination", "retained_unavailable with source_identity_sha256", dict(un_run, source_identity_sha256=H), False)
    case("Combination", "retained_unavailable, unknown phase",
         dict(un_run, reason=dict(un_run["reason"], phase="combination")), False)
    case("Combination", "ordinary subtraction", ordinary_sub, True)
    case("Combination", "ordinary range", ordinary_rng, True)
    case("Combination", "ordinary with call_ref", dict(ordinary_sub, call_ref=None), False)
    case("Combination", "ordinary with gate code reason", dict(ordinary_sub, reason="COMBINATION_MODULUS_BASIS_MIXED"), False)
    case("Combination", "base_withheld", withheld, True)
    case("Combination", "base_withheld with rows", dict(withheld, result_ids=["r"]), False)
    case("Combination", "base_withheld with reason no_retained_mechanics", dict(withheld, reason="no_retained_mechanics"), False)
    case("Combination", "unknown disposition", dict(ordinary_sub, disposition="retained"), False)
    case("Combination", "basis ref_id missing", dict(ordinary_sub, basis_ref={"ref_type": "combination"}), False)
    # CombinationReason
    for tag in ("no_operands", "nested_combination", "operands_differ", "no_selected_operand"):
        case("CombinationReason", tag, {"space": "combination", "tag": tag}, True)
    case("CombinationReason", "combination_unresolved (C2 run tag, not used in B2)",
         {"space": "combination", "tag": "combination_unresolved"}, False)
    case("CombinationReason", "count_range without name", {"space": "combination", "tag": "count_range"}, False)
    case("CombinationReason", "ledger_unavailable without error", {"space": "combination", "tag": "ledger_unavailable"}, False)
    case("CombinationReason", "refusal space", {"space": "refusal", "tag": "no_operands"}, False)
    # CombinationSource
    cs = {"index": 4, "owner": {"kind": "combination", "combination_index": 1, "combination_id": "comb:1"},
          "kernel_source_sha256": H, "ledger_sha256": H, "stiffness_sha256": H, "representative_source_ref": 2,
          "operands": [{"case_index": 1, "factor": HALF, "source_ref": 2, "source_identity_sha256": H}]}
    case("CombinationSource", "one operand", cs, True)
    case("CombinationSource", "no operands", dict(cs, operands=[]), False)
    case("CombinationSource", "case owner", dict(cs, owner={"kind": "case", "case_index": 0, "case_id": "A"}), False)
    case("CombinationSource", "with preparation member", dict(cs, preparation=None), False)
    case("CombinationSource", "operand with case_id", dict(cs, operands=[dict(cs["operands"][0], case_id="A")]), False)
    # MechanicsCombinationCall
    call = {"id": 2, "kind": "mechanics_combination", "owner_refs": [{"kind": "combination", "index": 1}],
            "requested_operands": [{"source_ref": 2, "factor": HALF}], "source_refs": [4], "run_refs": [3],
            "invocation_before": 10, "invocation_after": 25, "result": {"kind": "runs"}}
    refused = dict(call, source_refs=[], run_refs=[], invocation_after=10,
                   result={"kind": "pre_source_refusal", "stage": "combined_preparation",
                           "reason": {"space": "combination", "tag": "ledger_unavailable",
                                      "error": {"tag": "accumulator", "error": {"tag": "non_representable"}}}})
    case("MechanicsCombinationCall", "runs", call, True)
    case("MechanicsCombinationCall", "pre_source_refusal at combined preparation", refused, True)
    case("MechanicsCombinationCall", "empty requested_operands (C2: representable)", dict(call, requested_operands=[]), True)
    case("MechanicsCombinationCall", "no owner", dict(call, owner_refs=[]), False)
    case("MechanicsCombinationCall", "two sources", dict(call, source_refs=[4, 5]), False)
    case("MechanicsCombinationCall", "unknown stage", dict(refused, result=dict(refused["result"], stage="custody")), False)
    case("MechanicsCombinationCall", "origin_refusal result (never serialized)",
         dict(call, result={"kind": "origin_refusal"}), False)
    case("MechanicsCombinationCall", "case_batch kind", dict(call, kind="case_batch"), False)
    # CombinationGroup
    cg = dict(copy.deepcopy(group), id=1, call=2, first_source_ref=4, source_refs=[4],
              imports=[{"operand_index": 1, "selected_run": 0, "slot": "v512", "build": 3}])
    case("CombinationGroup", "imports from operand 1", cg, True)
    case("CombinationGroup", "empty imports", dict(cg, imports=[]), True)
    case("CombinationGroup", "no imports member", {k: v for k, v in cg.items() if k != "imports"}, False)
    case("CombinationGroup", "unknown slot", dict(cg, imports=[dict(cg["imports"][0], slot="s2048")]), False)
    case("CombinationGroup", "import without build", dict(cg, imports=[{k: v for k, v in cg["imports"][0].items() if k != "build"}]), False)
    # CombinationAttempt
    ca = {k: copy.deepcopy(case_attempt[k]) for k in ("result", "proof", "adapter", "overlay_work", "g5a_work")}
    ca.update(id=3, definition_id="RP-PREPARED-COMBINATION-DUAL-v1", owner_ref={"kind": "combination", "index": 1},
              material_basis_ref=0, source_ref=4, run_ref=3,
              stages={k: "not_entered" for k in ("proof_start", "projection", "maxima", "values", "aliases",
                                                  "certificate", "observables", "g5a")} | {"native": "failed"})
    ca["result"] = {"kind": "unavailable", "error": {"kind": "native", "run_ref": 3}}
    ca["proof"] = None
    case("CombinationAttempt", "native failed, no proof", ca, True)
    case("CombinationAttempt", "null run_ref", dict(ca, run_ref=None), False)
    case("CombinationAttempt", "null source_ref", dict(ca, source_ref=None), False)
    case("CombinationAttempt", "with ordinary_attempt_ref", dict(ca, ordinary_attempt_ref=0), False)
    case("CombinationAttempt", "with operational", dict(ca, operational=copy.deepcopy(case_attempt["operational"])), False)
    case("CombinationAttempt", "case owner", dict(ca, owner_ref={"kind": "case", "index": 1}), False)
    case("CombinationAttempt", "exact definition id", dict(ca, definition_id="RP-PREPARED-EXACT-DUAL-v1"), False)
    case("CombinationAttempt", "stage value 'entered'", dict(ca, stages=dict(ca["stages"], native="entered")), False)
    # ProductAttempt (J1: B3b's enum)
    case("ProductAttempt", "exact definition id (B3b enum)", dict(copy.deepcopy(case_attempt), definition_id="RP-PREPARED-EXACT-DUAL-v1"), True)
    case("ProductAttempt", "combination owner", dict(copy.deepcopy(case_attempt), owner_ref={"kind": "combination", "index": 0}), False)
    # OperandPreparation
    op = {"id": 0, "definition_id": "RP-PREPARED-ORDINARY-DUAL-v1", "owner_ref": {"kind": "case", "index": 1},
          "ordinary_attempt_ref": 1, "material_basis_ref": 0, "purpose": "combination_operand", "requested_by": [0, 2],
          "source_ref": None, "stage": "failed",
          "result": {"kind": "refused", "error": {"kind": "preparation", "capture": {"kind": "native_unavailable"},
                                                  "section": None}},
          "preparation": {"members": []}, "adapter": copy.deepcopy(case_attempt["adapter"]),
          "operational": copy.deepcopy(case_attempt["operational"])}
    case("OperandPreparation", "refused, two requesters", op, True)
    case("OperandPreparation", "prepared with members", dict(op, source_ref=3, stage="completed", result={"kind": "prepared"},
                                                            preparation=copy.deepcopy(case_attempt["preparation"])), True)
    case("OperandPreparation", "refused with a native error", dict(op, result={"kind": "refused", "error": {"kind": "native", "run_ref": 0}}), False)
    case("OperandPreparation", "combination definition id", dict(op, definition_id="RP-PREPARED-COMBINATION-DUAL-v1"), False)
    case("OperandPreparation", "other purpose", dict(op, purpose="case"), False)
    case("OperandPreparation", "stages object instead of stage", {**{k: v for k, v in op.items() if k != "stage"}, "stages": {}}, False)
    case("OperandPreparation", "combination owner", dict(op, owner_ref={"kind": "combination", "index": 0}), False)
    case("OperandPreparation", "operational without old_coverage",
         dict(op, operational={k: v for k, v in op["operational"].items() if k != "old_coverage"}), False)
    # CaseSource third preparation branch
    s3 = dict(copy.deepcopy(case_source), preparation={"operand_preparation_ref": 0, "sha256": H})
    case("CaseSource", "operand-prepared", s3, True)
    case("CaseSource", "both refs", dict(s3, preparation={"operand_preparation_ref": 0, "attempt_ref": 0, "sha256": H}), False)
    # UnavailableCause additions
    case("UnavailableCause", "operand_preparation_failure", {"kind": "operand_preparation_failure", "operand_preparation_ref": 0}, True)
    case("UnavailableCause", "operand_source_unavailable", {"kind": "operand_source_unavailable", "operand_index": 2}, True)
    case("UnavailableCause", "CombinationReason", {"space": "combination", "tag": "operands_differ"}, True)
    case("UnavailableCause", "operand_source_unavailable with operand_preparation_ref",
         {"kind": "operand_source_unavailable", "operand_index": 2, "operand_preparation_ref": 0}, False)

    rows = []
    for ref, label, value, expect in cases:
        a, b = judge.js(value, ref), judge.py(value, ref)
        rows.append({"def": ref, "case": label, "expected_valid": expect, "jsonschema": a, "py_walker": b,
                     "agrees": a == b == expect})

    # oneOf disjointness: each positive instance of a branch fails its siblings.
    siblings = [
        ("CaseSource", "CombinationSource", s3, cs),
        ("Call", "MechanicsCombinationCall", body["calls"][0], call),
        ("Group", "CombinationGroup", group, cg),
        ("ProductAttempt", "CombinationAttempt", case_attempt, ca),
        ("Reason", "CombinationReason", {"space": "unresolved", "tag": "ceiling"}, {"space": "combination", "tag": "no_operands"}),
    ]
    disjoint = []
    for a_ref, b_ref, a_val, b_val in siblings:
        disjoint.append({"pair": f"{a_ref}/{b_ref}",
                         "a_valid_a": judge.js(a_val, a_ref), "a_valid_b": judge.js(a_val, b_ref),
                         "b_valid_b": judge.js(b_val, b_ref), "b_valid_a": judge.js(b_val, a_ref),
                         "py_a_valid_b": judge.py(a_val, b_ref), "py_b_valid_a": judge.py(b_val, a_ref)})
    comb_branches = schema["$defs"]["Combination"]["oneOf"]
    branch_hits = {}
    for label, value in (("retained_selected", sel_e), ("retained_unavailable", un_run), ("ordinary", ordinary_sub),
                         ("base_withheld", withheld)):
        branch_hits[label] = [judge.js_spec(value, b) for b in comb_branches]

    # Body-level: a whole W-CB1-like body built from two_case_synthetic plus one of each new member.
    whole = copy.deepcopy(base)
    b = whole["body"]
    b["combinations"] = [sel_e, ordinary_sub]
    b["sources"].append(cs)
    b["calls"].append(call)
    b["groups"].append(cg)
    b["product_attempts"].append(ca)
    b["operand_preparations"] = [op]
    b["work"]["execution_order"].append({"kind": "combination", "index": 1})
    body_level = {"jsonschema": jsonschema.Draft202012Validator(schema).is_valid(whole),
                  "py_walker": judge.py_spec(whole, schema)}
    # The same body with an empty operand_preparations member, and with combinations a list of strings.
    w2 = copy.deepcopy(whole)
    w2["body"]["operand_preparations"] = []
    w3 = copy.deepcopy(whole)
    w3["body"]["combinations"] = ["comb:1"]
    body_level["empty_operand_preparations"] = {"jsonschema": jsonschema.Draft202012Validator(schema).is_valid(w2),
                                                "py_walker": judge.py_spec(w2, schema)}
    body_level["combinations_as_ids"] = {"jsonschema": jsonschema.Draft202012Validator(schema).is_valid(w3),
                                         "py_walker": judge.py_spec(w3, schema)}

    report = {"instances": rows, "all_agree": all(r["agrees"] for r in rows), "counts": {
        "positive": sum(1 for r in rows if r["expected_valid"]), "negative": sum(1 for r in rows if not r["expected_valid"])},
        "disjointness": disjoint, "combination_branch_hits": branch_hits, "body_level": body_level}
    out.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"all_agree": report["all_agree"], "counts": report["counts"],
                      "disagreements": [r for r in rows if not r["agrees"]], "disjointness": disjoint,
                      "combination_branch_hits": branch_hits, "body_level": body_level}, indent=1))


if __name__ == "__main__":
    main()
