"""I97 B2-C revision 01: read-only checks (records only). Writes nothing but <out_json>.

  1. N-12 and N-5 on hand-built instances of the (unchanged) J1 SCHEMA, by jsonschema and by PY's own G1 walker:
     an OperandPreparation carrying DEF-C's id is refused (and DEF-C's id is in PTABLE r1, so G0's membership passes:
     the first failure is G1); a CaptureError {kind: origin} inside a CombinationAttempt or an OperandPreparation is
     admitted by SCHEMA, so G5 must refuse it.
  2. S-4 (a): the 64-epsilon guard (RE `guarded`, the base G7 `combination_magnitudes` predicate) holds when the
     published magnitude is binary64 hypot(hypot(x,y),z) of the published components, whatever hypot the reader
     uses within 1 ulp per call: every pair of 1-ulp-perturbed evaluations passes the guard, on adversarial and random
     triples; and math.hypot's nested error against the exact norm (Decimal, 120 digits).
  3. S-2: T-6's row layout on a model of the producer's append order (lib.rs: case rows; per combination in authored
     order its rows; then per subtraction or range combination in authored order one record per operand), over every
     in-domain shape (c <= 2 with z >= 1, C_eq <= 3): revision 01's rule holds for every one, v0's fails where a
     subtraction or range precedes a combination with rows, including c = 1 [range(A), 2*A].
  4. S-6: 07o's rehash rule on a synthetic B2 body (from CORPUS two_case_synthetic): one pass of the stated order is
     a fixed point and leaves every hash relation true; 07e's rule, or a wrong order, leaves relations false.
     The hash here is the generator's canonical form (sha256 of sorted-key compact JSON), not the readers' JCS
     adapter; the check is about dependency order only.

Usage: python b2c_checks_r1.py <P> <PY reader file> <J1 schema> <PTABLE r1> <v0 b2c_checks.py> <out_json>
"""
import copy
import hashlib
import importlib.util
import itertools
import json
import math
import random
import sys
from decimal import Decimal, getcontext
from fractions import Fraction
from pathlib import Path

DEF_O = "RP-PREPARED-ORDINARY-DUAL-v1"
DEF_C = "RP-PREPARED-COMBINATION-DUAL-v1"
EPS = 2.0 ** -52
MIN_POSITIVE = 2.0 ** -1022


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


# ------------------------------------------------------------------------------------------------- 1. N-12, N-5
def schema_instances(p, reader, j1, ptable, v0):
    corpus = json.loads((p / "fixtures/results/retained_precision_cases.json").read_bytes())
    base = next(c for c in corpus["cases"] if c["id"] == "two_case_synthetic")["source"]["retained_precision"]
    attempt = base["body"]["product_attempts"][0]
    judge = v0.Judge(j1, reader)
    op = {"id": 0, "definition_id": DEF_O, "owner_ref": {"kind": "case", "index": 1}, "ordinary_attempt_ref": 1,
          "material_basis_ref": 0, "purpose": "combination_operand", "requested_by": [0], "source_ref": 2,
          "result": {"kind": "prepared"}, "stage": "completed", "preparation": copy.deepcopy(attempt["preparation"]),
          "adapter": copy.deepcopy(attempt["adapter"]), "operational": copy.deepcopy(attempt["operational"])}
    origin = {"kind": "origin", "cause": {"kind": "missing_selected_origin", "operand": 0}}
    op_refused_origin = dict(copy.deepcopy(op), source_ref=None, stage="failed",
                             result={"kind": "refused", "error": {"kind": "preparation", "capture": origin,
                                                                  "section": None}})
    ca = {k: copy.deepcopy(attempt[k]) for k in ("id", "proof", "adapter", "overlay_work", "g5a_work")}
    ca.update({"definition_id": DEF_C, "owner_ref": {"kind": "combination", "index": 0}, "material_basis_ref": 0,
               "source_ref": 2, "run_ref": 2, "result": {"kind": "ready"},
               "stages": {k: attempt["stages"][k] for k in ("native", "proof_start", "projection", "maxima", "values",
                                                            "aliases", "certificate", "observables", "g5a")}})
    rows = []
    for ref, label, value, expect in (
            ("OperandPreparation", "DEF-O id (control)", op, True),
            ("OperandPreparation", "DEF-C id (N-12)", dict(op, definition_id=DEF_C), False),
            ("OperandPreparation", "refused, capture origin (N-5: admitted, so G5 refuses)", op_refused_origin, True),
            ("CombinationAttempt", "ready (control)", ca, True),
            ("CombinationAttempt", "unavailable, capture origin (N-5)",
             dict(ca, result={"kind": "unavailable", "error": {"kind": "capture", "cause": origin}}), True),
            ("CombinationAttempt", "unavailable, observable origin (N-5)",
             dict(ca, result={"kind": "unavailable", "error": {"kind": "observable", "cause": origin}}), True)):
        a, b = judge.js(value, ref), judge.py(value, ref)
        rows.append({"def": ref, "case": label, "expected_valid": expect, "jsonschema": a, "py_walker": b,
                     "agrees": a == b == expect})
    body = []
    for label, mutate, expect in (
            ("operand_preparations [DEF-O id]", lambda b: b.__setitem__("operand_preparations", [copy.deepcopy(op)]),
             True),
            ("operand_preparations [DEF-C id]",
             lambda b: b.__setitem__("operand_preparations", [dict(copy.deepcopy(op), definition_id=DEF_C)]), False)):
        receipt = copy.deepcopy(base)
        mutate(receipt["body"])
        a, b = judge.js(receipt), judge.py(receipt)
        body.append({"case": label, "expected_valid": expect, "jsonschema": a, "py_walker": b, "agrees": a == b == expect})
    ids = [d["id"] for d in ptable["product_formation_definitions"]]
    return {"instances": rows, "body": body,
            "ptable_r1_definition_ids": ids,
            "g0_row9_membership_passes_for_DEF_C": DEF_C in ids,
            "first_failure_for_DEF_C_on_an_operand_preparation": "G1 (schema const), not G0",
            "all_agree": all(r["agrees"] for r in rows + body)}


# ------------------------------------------------------------------------------------------------- 2. S-4 (a)
def guarded(published, recomputed):
    """RE preview_physics_evidence.rs `guarded`, in binary64."""
    return abs(published - recomputed) <= 64.0 * EPS * max(abs(published), MIN_POSITIVE)


def ratio(published, recomputed):
    return abs(published - recomputed) / (64.0 * EPS * max(abs(published), MIN_POSITIVE))


def perturbed(v, k):
    """A faithful 1-ulp libm: an exact zero stays +0 (hypot(0,0) is exact); otherwise either neighbour."""
    return v if k == 0 or v == 0 else math.nextafter(v, math.inf if k > 0 else -math.inf)


def nested(x, y, z, k1=0, k2=0):
    """hypot(hypot(x,y),z) with each call's result moved by k ulps (a 1-ulp libm model)."""
    return perturbed(math.hypot(perturbed(math.hypot(x, y), k1), z), k2)


def exact_norm(x, y, z):
    getcontext().prec = 120
    s = sum(Fraction(v) ** 2 for v in (x, y, z))
    return (Decimal(s.numerator) / Decimal(s.denominator)).sqrt()


def guard_study():
    rng = random.Random(20261007)
    triples = [(0.0, 0.0, 0.0), (-0.0, 0.0, -0.0), (3.0, 4.0, 12.0), (1.0, 1.0, 1.0), (1e-3, 0.0, 0.0),
               (5e-324, 5e-324, 5e-324), (5e-324, 0.0, 0.0), (2.0 ** -1022, 2.0 ** -1023, 2.0 ** -1074),
               (1e300, 1e300, 1e300), (1.0, 2.0 ** -60, 2.0 ** -60), (2.0 ** 60, 1.0, -1.0),
               (0.1, 0.2, 0.3), (-7.25e-9, 3.5e-9, 1.0e-12)]
    for _ in range(4000):
        e = rng.choice([(-1074, -1022), (-1030, -900), (-60, 10), (-12, 3), (600, 1000)])
        t = []
        for _ in range(3):
            if rng.random() < 0.1:
                t.append(rng.choice([0.0, -0.0]))
            else:
                t.append(rng.choice([1, -1]) * rng.random() * 2.0 ** rng.randint(*e))
        triples.append(tuple(t))
    worst_ratio, worst_case, fails, nonfinite = 0.0, None, 0, 0
    worst_err_eps, worst_err_case = 0.0, None
    for x, y, z in triples:
        models = [nested(x, y, z, a, b) for a in (-1, 0, 1) for b in (-1, 0, 1)]
        if not all(math.isfinite(m) for m in models):
            nonfinite += 1
            continue
        for p_, r_ in itertools.product(models, models):
            if not guarded(p_, r_):
                fails += 1
            q = ratio(p_, r_)
            if q > worst_ratio:
                worst_ratio, worst_case = q, [x, y, z]
        n = exact_norm(x, y, z)
        h = models[4]
        if n >= Decimal(MIN_POSITIVE):
            err = abs(Decimal(h) - n) / n / Decimal(EPS)
            if err > worst_err_eps:
                worst_err_eps, worst_err_case = float(err), [x, y, z]
        elif n == 0 and h != 0:
            fails += 1
    return {"triples": len(triples), "pairs_per_triple": 81, "guard_failures": fails,
            "nonfinite_triples_skipped (refused by the recipe: finite only)": nonfinite,
            "max |p-r| / (64 eps max(|p|,MIN_POSITIVE)) over 1-ulp models": worst_ratio,
            "at": [x.hex() for x in worst_case],
            "max relative error of nested math.hypot vs the exact norm, in eps (normal results)": worst_err_eps,
            "at_err": [x.hex() for x in worst_err_case]}


# ------------------------------------------------------------------------------------------------- 3. S-2 layout
def layout(c, combos):
    rows = [("load_case", i) for i in range(c) for _ in range(3)]
    for j, k in enumerate(combos):
        if k["rows"]:
            rows += [("combination", j)] * 3
    for j, k in enumerate(combos):
        rows += [("combination", j)] * k["records"]
    return rows


def t6_r1(rows, combos):
    first = next((i for i, r in enumerate(rows) if r[0] == "combination"), len(rows))
    tail = rows[first:]
    if any(r[0] != "combination" or not 0 <= r[1] < len(combos) for r in tail):
        return False
    for j, k in enumerate(combos):
        if k["kind"] == "mechanics":
            idx = [i for i, r in enumerate(rows) if r == ("combination", j)]
            if idx and idx != list(range(idx[0], idx[0] + len(idx))):
                return False
    return True


def t6_v0(rows, combos):
    first = next((i for i, r in enumerate(rows) if r[0] == "combination"), len(rows))
    tail = [r[1] for r in rows[first:]]
    blocks = [j for j, _ in itertools.groupby(tail)]
    return all(r[0] == "combination" for r in rows[first:]) and blocks == sorted(blocks) and len(blocks) == len(set(blocks))


def layout_study():
    shapes = []
    for c in (1, 2):
        cases = "AB"[:c]
        kinds = [("mechanics distinct", {"kind": "mechanics", "rows": True, "records": 0})]
        kinds.append(("mechanics repeated (C-1, rowless)", {"kind": "mechanics", "rows": False, "records": 0}))
        kinds += [(f"range({','.join(ops)})", {"kind": "range", "rows": True, "records": len(ops)})
                  for n in range(1, c + 1) for ops in itertools.combinations(cases, n)]
        if c == 2:
            kinds += [("subtraction", {"kind": "subtraction", "rows": True, "records": 2})]
        for z in range(1, 4 - c):
            for seq in itertools.product(kinds, repeat=z):
                combos = [k for _, k in seq]
                rows = layout(c, combos)
                shapes.append({"c": c, "authored": [n for n, _ in seq], "r1": t6_r1(rows, combos),
                               "v0": t6_v0(rows, combos)})
    example = next(s for s in shapes if s["c"] == 1 and s["authored"] == ["range(A)", "mechanics distinct"])
    return {"in_domain_shapes": len(shapes), "r1_holds_for_all": all(s["r1"] for s in shapes),
            "v0_fails": [f"c={s['c']}: " + ", ".join(s["authored"]) for s in shapes if not s["v0"]],
            "c1_range_A_then_2A": example}


# ------------------------------------------------------------------------------------------------- 4. S-6 rehash
def H(domain, payload):
    return hashlib.sha256(json.dumps({"domain": domain, "payload": payload}, sort_keys=True, separators=(",", ":"),
                                     ensure_ascii=False).encode()).hexdigest()


DEF_O_H = "a7ed7ca0bf0bba6e8b821ca4befa00a0fa9541a83694be8b28ac63e39b1d0349"


def at(items, ref):
    return items[ref] if isinstance(ref, int) and not isinstance(ref, bool) and 0 <= ref < len(items) else None


def members(record):
    return [{"member": m["member"], "old_source": m["old_source"], "old_facts": m["old_facts"],
             "section": m["result"]["section"]} for m in record["preparation"]["members"]]


def all_prepared(record):
    return all(m["result"]["kind"] == "prepared" for m in record["preparation"]["members"])


def case_preparation(body, s):
    a = at(body["product_attempts"], (s.get("preparation") or {}).get("attempt_ref"))
    if a is not None and all_prepared(a):
        return H("retained_precision_preparation_v1", {
            "definition_id": a["definition_id"], "definition_sha256": DEF_O_H, "owner_ref": a["owner_ref"],
            "ordinary_attempt_ref": a["ordinary_attempt_ref"], "material_basis_ref": a["material_basis_ref"],
            "members": members(a)})
    return None


def operand_preparation(body, s):
    r = at(body.get("operand_preparations", []), (s.get("preparation") or {}).get("operand_preparation_ref"))
    if r is not None and r["result"]["kind"] == "prepared" and all_prepared(r):
        return H("retained_precision_operand_preparation_v1", {
            "definition_id": r["definition_id"], "definition_sha256": DEF_O_H, "owner_ref": r["owner_ref"],
            "ordinary_attempt_ref": r["ordinary_attempt_ref"], "material_basis_ref": r["material_basis_ref"],
            "purpose": r["purpose"], "members": members(r)})
    return None


def identity(s):
    return H("retained_precision_source_mp_v2", {k: v for k, v in s.items() if k != "index"})


def step(body, value, name):
    if name in ("1 case preparation", "2 operand preparation"):
        f = case_preparation if name.startswith("1") else operand_preparation
        for s in body["sources"]:
            v = f(body, s)
            if v is not None:
                s["preparation"]["sha256"] = v
    elif name == "3 selected case identity":
        for c in body["cases"]:
            s = at(body["sources"], c.get("source_ref")) if c["status"] == "selected" else None
            if s is not None:
                c["source_identity_sha256"] = identity(s)
    elif name == "4 operand identity":
        for s in body["sources"]:
            if s["owner"]["kind"] == "combination":
                for o in s["operands"]:
                    t = at(body["sources"], o["source_ref"])
                    if t is not None:
                        o["source_identity_sha256"] = identity(t)
    elif name == "5 combination identity":
        for e in body["combinations"]:
            s = at(body["sources"], e.get("source_ref")) if e["disposition"] == "retained_selected" else None
            if s is not None:
                e["source_identity_sha256"] = identity(s)
    elif name == "6 publication":
        body["publication_sha256"] = H("retained_precision_publication_mp_v2",
                                       {k: v for k, v in value.items() if k != "retained_precision"})
    elif name == "7 receipt":
        value["retained_precision"]["receipt_sha256"] = H("retained_precision_receipt_mp_v2", body)


ORDER = ["1 case preparation", "2 operand preparation", "3 selected case identity", "4 operand identity",
         "5 combination identity", "6 publication", "7 receipt"]


def rehash(value, order):
    value = copy.deepcopy(value)
    for name in order:
        step(value["retained_precision"]["body"], value, name)
    return value


def broken(value):
    """Every hash relation of a B2 body that G1 checks, recomputed; the names of those that do not hold."""
    v = copy.deepcopy(value)
    b = v["retained_precision"]["body"]
    out = []
    for s in b["sources"]:
        for f, name in ((case_preparation, "case preparation"), (operand_preparation, "operand preparation")):
            x = f(b, s)
            if x is not None and x != s["preparation"]["sha256"]:
                out.append(f"{name} of source {s['index']}")
    for c in b["cases"]:
        s = at(b["sources"], c.get("source_ref")) if c["status"] == "selected" else None
        if s is not None and identity(s) != c["source_identity_sha256"]:
            out.append("selected case identity")
    for s in b["sources"]:
        if s["owner"]["kind"] == "combination":
            for o in s["operands"]:
                if identity(b["sources"][o["source_ref"]]) != o["source_identity_sha256"]:
                    out.append(f"operand {o['case_index']} identity")
    for e in b["combinations"]:
        if e["disposition"] == "retained_selected" and identity(b["sources"][e["source_ref"]]) != e["source_identity_sha256"]:
            out.append("combination identity")
    if H("retained_precision_publication_mp_v2", {k: x for k, x in v.items() if k != "retained_precision"}) != b["publication_sha256"]:
        out.append("publication")
    if H("retained_precision_receipt_mp_v2", b) != v["retained_precision"]["receipt_sha256"]:
        out.append("receipt")
    return out


def rehash_study(p):
    corpus = json.loads((p / "fixtures/results/retained_precision_cases.json").read_bytes())
    value = copy.deepcopy(next(c for c in corpus["cases"] if c["id"] == "two_case_synthetic")["source"])
    b = value["retained_precision"]["body"]
    zero = "0" * 64
    a1 = b["product_attempts"][1]
    b["operand_preparations"] = [{
        "id": 0, "definition_id": DEF_O, "owner_ref": {"kind": "case", "index": 1}, "ordinary_attempt_ref": 1,
        "material_basis_ref": 0, "purpose": "combination_operand", "requested_by": [0], "source_ref": 2,
        "result": {"kind": "prepared"}, "stage": "completed", "preparation": copy.deepcopy(a1["preparation"])}]
    operand = copy.deepcopy(b["sources"][1])
    operand.update(index=2, preparation={"operand_preparation_ref": 0, "sha256": zero})
    b["sources"].append(operand)
    b["sources"].append({"index": 3, "owner": {"kind": "combination", "combination_index": 0, "combination_id": "C"},
                         "kernel_source_sha256": zero, "ledger_sha256": zero, "stiffness_sha256": zero,
                         "representative_source_ref": 0,
                         "operands": [{"case_index": 0, "factor": "3ff0000000000000", "source_ref": 0,
                                       "source_identity_sha256": zero},
                                      {"case_index": 1, "factor": "3fe0000000000000", "source_ref": 2,
                                       "source_identity_sha256": zero}]})
    b["combinations"] = [{"basis_ref": {"ref_type": "combination", "ref_id": "C"}, "disposition": "retained_selected",
                          "source_ref": 3, "source_identity_sha256": zero}]
    for s in b["sources"][:2]:
        s["preparation"]["sha256"] = zero
    for c in b["cases"]:
        c["source_identity_sha256"] = zero
    once = rehash(value, ORDER)
    twice = rehash(once, ORDER)
    # m27-like: the operand preparation's owner edited, then one pass of each rule.
    edited = copy.deepcopy(once)
    edited["retained_precision"]["body"]["operand_preparations"][0]["owner_ref"] = {"kind": "case", "index": 0}
    rule_07e = ["1 case preparation", "3 selected case identity", "6 publication", "7 receipt"]
    wrong = ["1 case preparation", "3 selected case identity", "5 combination identity", "4 operand identity",
             "2 operand preparation", "6 publication", "7 receipt"]
    return {"stated_order": ORDER,
            "one_pass_is_a_fixed_point": once == twice,
            "relations_broken_after_one_pass_of_the_stated_order": broken(once),
            "after_an_operand_preparation_edit": {
                "stated_order (07o)": broken(rehash(edited, ORDER)),
                "07e rule (today's harnesses)": broken(rehash(edited, rule_07e)),
                "wrong order (5 before 4 before 2)": broken(rehash(edited, wrong))}}


def main():
    p, py_path, j1_path, ptable_path, v0_checks, out_json = (Path(a) for a in sys.argv[1:7])
    v0 = load(v0_checks, "b2c_checks_v0")
    reader = load(py_path, "i97_py_reader")
    j1 = json.loads(j1_path.read_bytes())
    ptable = json.loads(ptable_path.read_bytes())
    report = {"n12_n5_schema": schema_instances(p, reader, j1, ptable, v0),
              "s4_guard": guard_study(), "s2_layout": layout_study(), "s6_rehash": rehash_study(p)}
    Path(out_json).write_text(json.dumps(report, indent=2, sort_keys=True, ensure_ascii=True) + "\n")
    print(json.dumps({"n12_n5_all_agree": report["n12_n5_schema"]["all_agree"],
                      "s4": {k: v for k, v in report["s4_guard"].items() if not k.startswith("at")},
                      "s2": {k: report["s2_layout"][k] for k in ("in_domain_shapes", "r1_holds_for_all", "v0_fails")},
                      "s6": report["s6_rehash"]}, indent=1, sort_keys=True))


if __name__ == "__main__":
    main()
