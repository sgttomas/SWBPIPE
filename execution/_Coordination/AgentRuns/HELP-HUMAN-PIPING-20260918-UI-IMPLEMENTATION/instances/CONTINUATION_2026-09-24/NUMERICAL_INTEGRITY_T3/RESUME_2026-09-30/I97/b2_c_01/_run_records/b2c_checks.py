"""I97 B2-C: read-only checks of the draft statics (records only).

Reads committed bytes and the generator's outputs; writes nothing but <out_json>.
  1. Draft 2020-12 meta-validation (jsonschema) of main's SCHEMA, B2's and the merged J1 text.
  2. The keyword vocabulary of the J1 text is main's (the three readers' walkers know no other keyword).
  3. Backward compatibility: every committed successor receipt, every CORPUS base, and every CORPUS mutation and
     must-pass entry (edits applied, as the corpus states them) has the same G1 shape verdict under main's SCHEMA
     and under the J1 text, by jsonschema and by PY's own G1 walker (`_shape`, loaded from the committed reader
     file, with its schema loader pointed at each text).
  4. The new $defs on hand-built instances: positive and negative cases, both validators.
  5. R-7: the published row set of a mechanics combination in the committed preview-physics-1 fixture.

Usage: python b2c_checks.py <P> <PY reader file> <J1 schema> <out_json>
"""
import copy
import importlib.util
import json
import sys
from collections import Counter, defaultdict
from pathlib import Path

import jsonschema


def load_reader(path):
    spec = importlib.util.spec_from_file_location("i97_py_reader", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def keywords(schema):
    found = set()

    def walk(v):
        if isinstance(v, dict):
            for k, x in v.items():
                found.add(k)
                if k in ("properties", "$defs"):
                    for y in x.values():
                        walk(y)
                elif k not in ("const", "enum"):
                    walk(x)
        elif isinstance(v, list):
            for x in v:
                walk(x)
    walk(schema)
    return found


def edit(doc, edits):
    for e in edits:
        target = doc
        for key in e["path"][:-1]:
            target = target[key]
        last = e["path"][-1]
        if e["op"] == "set":
            if isinstance(target, list) and last == len(target):
                target.append(copy.deepcopy(e["value"]))
            else:
                target[last] = copy.deepcopy(e["value"])
        elif e["op"] == "remove":
            del target[last]
        else:
            raise ValueError(e["op"])
    return doc


class Judge:
    def __init__(self, schema, reader):
        self.schema = schema
        self.validator = jsonschema.Draft202012Validator(schema)
        self.reader = reader

    def js(self, value, ref=None):
        if ref is None:
            return self.validator.is_valid(value)
        sub = {"$schema": self.schema["$schema"], "$defs": self.schema["$defs"], "$ref": f"#/$defs/{ref}"}
        return jsonschema.Draft202012Validator(sub).is_valid(value)

    def py(self, value, ref=None):
        self.reader._schema = lambda: self.schema
        spec = self.schema if ref is None else {"$ref": f"#/$defs/{ref}"}
        try:
            return bool(self.reader._shape(value, spec))
        except (KeyError, TypeError, AttributeError, IndexError):
            return False


def main():
    p, py_path, j1_path, out_json = (Path(a) for a in sys.argv[1:5])
    main_raw = (p / "schemas/retained_precision_mp_v2.schema.json").read_bytes()
    main_schema = json.loads(main_raw)
    j1 = json.loads(j1_path.read_bytes())
    reader = load_reader(py_path)
    report = {}

    # 1. Meta-validation.
    meta = {}
    for name, s in (("main", main_schema), ("j1", j1)):
        try:
            jsonschema.Draft202012Validator.check_schema(s)
            meta[name] = "valid"
        except jsonschema.SchemaError as exc:
            meta[name] = f"invalid: {exc.message}"
    report["meta_validation"] = meta

    # 2. Vocabulary.
    report["vocabulary"] = {"main": sorted(keywords(main_schema)), "j1": sorted(keywords(j1)),
                            "j1_subset_of_main": keywords(j1) <= keywords(main_schema)}

    # 2a. Existing tests that read SCHEMA's content, emulated on the J1 text:
    #  - PY test_all_object_definitions_are_closed (every object closed, required within properties);
    #  - PP retained_memory_law_tests.rs refusal_kinds_are_the_schema_preconditions (the 400-byte window after
    #    `"const": "unavailable_precondition"` names caller, source_family and resource_admission).
    def closed(v, bad):
        if isinstance(v, dict):
            if v.get("type") == "object" and not (v.get("additionalProperties") is False
                                                  and set(v["required"]) <= v["properties"].keys()):
                bad.append(sorted(v.get("properties", {}))[:4])
            for x in v.values():
                closed(x, bad)
        elif isinstance(v, list):
            for x in v:
                closed(x, bad)
        return bad
    j1_text = j1_path.read_bytes().decode()
    window_results = {}
    for name, text in (("main", main_raw.decode()), ("j1", j1_text)):
        start = text.find('"const": "unavailable_precondition"')
        block = text[start:start + 400]
        window_results[name] = all(f'"{k}"' in block for k in ("caller", "source_family", "resource_admission"))
    report["existing_schema_tests_emulated"] = {
        "all_objects_closed_j1": closed(j1, []) == [],
        "unavailable_precondition_window": window_results,
    }

    judges = {"main": Judge(main_schema, reader), "j1": Judge(j1, reader)}

    # 3. Backward compatibility.
    res = p / "fixtures/results"
    fixtures = sorted(res.glob("retained_precision_*successor*.json"))
    fx = {}
    for f in fixtures:
        doc = json.loads(f.read_bytes())
        envelope = doc.get("source") or doc.get("result_envelope") or doc
        receipt = envelope.get("retained_precision")
        if receipt is None:
            fx[f.name] = "no receipt"
            continue
        fx[f.name] = {k: {"jsonschema": j.js(receipt), "py_walker": j.py(receipt)} for k, j in judges.items()}
    report["committed_successor_fixtures"] = fx
    corpus = json.loads((res / "retained_precision_cases.json").read_bytes())
    bases = {c["id"]: c["source"] for c in corpus["cases"]}
    verdicts = Counter()
    differences = []
    base_rows = {}
    for cid, src in bases.items():
        r = src["retained_precision"]
        v = tuple((k, j.js(r), j.py(r)) for k, j in judges.items())
        base_rows[cid] = v
        verdicts[("base",) + v] += 1
    entries = [("mutation", m) for m in corpus["mutations"]] + [("must_pass", m) for m in corpus["must_pass"]]
    for kind, m in entries:
        doc = edit(copy.deepcopy(bases[m["base"]]), m["edits"])
        r = doc.get("retained_precision")
        if not isinstance(r, dict):
            verdicts[(kind, "no receipt object")] += 1
            continue
        v = {k: (j.js(r), j.py(r)) for k, j in judges.items()}
        verdicts[(kind, "main js/py", v["main"], "j1 js/py", v["j1"])] += 1
        if v["main"] != v["j1"]:
            differences.append({"id": m["id"], "expected": m["expected"], "main": v["main"], "j1": v["j1"]})
    report["corpus"] = {
        "cases": len(corpus["cases"]), "mutations": len(corpus["mutations"]), "must_pass": len(corpus["must_pass"]),
        "verdict_counts": {json.dumps(list(k)): n for k, n in sorted(verdicts.items(), key=str)},
        "main_vs_j1_shape_differences": differences,
    }

    # 4. The new $defs on hand-built instances (both validators, J1 text).
    base = bases["two_case_synthetic"]["retained_precision"]["body"]
    attempt = base["product_attempts"][0]
    case_source = base["sources"][0]
    run = next(c["run"] for c in base["cases"] if c.get("run"))
    selection = next(c["selection"] for c in base["cases"] if c["status"] == "selected")
    H = "0" * 64
    B = "3ff0000000000000"
    combination_attempt = {k: copy.deepcopy(attempt[k]) for k in
                           ("id", "result", "proof", "adapter", "overlay_work", "g5a_work")}
    combination_attempt.update({"definition_id": "RP-PREPARED-COMBINATION-DUAL-v1",
                                "owner_ref": {"kind": "combination", "index": 0}, "material_basis_ref": 0,
                                "source_ref": 2, "run_ref": 2,
                                "stages": {k: attempt["stages"][k] for k in
                                           ("native", "proof_start", "projection", "maxima", "values", "aliases",
                                            "certificate", "observables", "g5a")}})
    operand_preparation = {"id": 0, "definition_id": "RP-PREPARED-ORDINARY-DUAL-v1",
                           "owner_ref": {"kind": "case", "index": 1}, "ordinary_attempt_ref": 1,
                           "material_basis_ref": 0, "purpose": "combination_operand", "requested_by": [0],
                           "source_ref": 2, "result": {"kind": "prepared"}, "stage": "completed",
                           "preparation": copy.deepcopy(attempt["preparation"]),
                           "adapter": copy.deepcopy(attempt["adapter"]),
                           "operational": copy.deepcopy(attempt["operational"])}
    expr = {"kind": "mechanics", "terms": [{"case_id": "A", "factor": B}, {"case_id": "B", "factor": B}]}
    common = {"basis_ref": {"ref_type": "combination", "ref_id": "C"}, "expression": expr,
              "result_ids": ["r1"], "diagnostic_refs": []}
    selected = dict(common, disposition="retained_selected", method="contribution_preserving_multiprecision_v1",
                    call_ref=1, product_attempt_ref=2, run=copy.deepcopy(run), source_ref=3,
                    source_identity_sha256=H, selection=copy.deepcopy(selection))
    unavailable_no_call = dict(common, disposition="retained_unavailable", call_ref=None, product_attempt_ref=None,
                               reason={"code": "combination_unresolved", "phase": "preparation",
                                       "cause": {"kind": "operand_preparation_failure",
                                                 "operand_preparation_ref": 0}},
                               diagnostic_ref="d", run=None, source_ref=None)
    unavailable_presource = copy.deepcopy(unavailable_no_call)
    unavailable_presource.update(call_ref=1, reason={"code": "combination_unresolved", "phase": "preparation",
                                                     "cause": {"space": "combination", "tag": "operands_differ"}})
    unavailable_source = copy.deepcopy(unavailable_no_call)
    unavailable_source["reason"]["cause"] = {"kind": "operand_source_unavailable", "operand_index": 1}
    ordinary = dict(common, disposition="ordinary", reason="no_retained_mechanics",
                    expression={"kind": "range_envelope", "operand_ids": ["A", "B"], "mode": "max_abs"})
    withheld = dict(common, disposition="base_withheld", reason="COMBINATION_MODULUS_BASIS_MIXED", result_ids=[])
    combination_source = {"index": 3, "owner": {"kind": "combination", "combination_index": 0,
                                                "combination_id": "C"},
                          "kernel_source_sha256": H, "ledger_sha256": H, "stiffness_sha256": H,
                          "representative_source_ref": 0,
                          "operands": [{"case_index": 0, "factor": B, "source_ref": 0, "source_identity_sha256": H},
                                       {"case_index": 1, "factor": B, "source_ref": 2, "source_identity_sha256": H}]}
    call_runs = {"id": 1, "kind": "mechanics_combination", "owner_refs": [{"kind": "combination", "index": 0}],
                 "requested_operands": [{"source_ref": 0, "factor": B}, {"source_ref": 2, "factor": B}],
                 "source_refs": [3], "run_refs": [2], "invocation_before": 5, "invocation_after": 9,
                 "result": {"kind": "runs"}}
    call_refused = dict(call_runs, source_refs=[], run_refs=[], invocation_after=5,
                        result={"kind": "pre_source_refusal", "stage": "operand_validation",
                                "reason": {"space": "combination", "tag": "no_selected_operand"}})
    group = copy.deepcopy(base["groups"][0])
    combination_group = dict(copy.deepcopy(group), imports=[{"operand_index": 0, "selected_run": 0, "slot": "s128",
                                                              "build": 0}])
    operand_source = copy.deepcopy(case_source)
    operand_source["preparation"] = {"operand_preparation_ref": 0, "sha256": H}
    exact_operand_preparation = dict(copy.deepcopy(operand_preparation), definition_id="RP-PREPARED-EXACT-DUAL-v1")
    instances = [
        ("Combination", "retained_selected", selected, True),
        ("Combination", "retained_unavailable, operand_preparation_failure", unavailable_no_call, True),
        ("Combination", "retained_unavailable, pre_source_refusal", unavailable_presource, True),
        ("Combination", "retained_unavailable, operand_source_unavailable", unavailable_source, True),
        ("Combination", "ordinary (range)", ordinary, True),
        ("Combination", "base_withheld", withheld, True),
        ("Combination", "base_withheld with a row id (maxItems 0)", dict(withheld, result_ids=["r1"]), False),
        ("Combination", "ordinary with a run member (closed)", dict(ordinary, run=None), False),
        ("Combination", "ordinary with another reason", dict(ordinary, reason="combination_unresolved"), False),
        ("Combination", "base_withheld with an unknown gate code", dict(withheld, reason="X"), False),
        ("Combination", "selected without a call_ref", {k: v for k, v in selected.items() if k != "call_ref"}, False),
        ("Combination", "basis ref_type load_case",
         dict(ordinary, basis_ref={"ref_type": "load_case", "ref_id": "C"}), False),
        ("CombinationSource", "two operands", combination_source, True),
        ("CombinationSource", "with a case member (nodal_terms)", dict(combination_source, nodal_terms=[]), False),
        ("MechanicsCombinationCall", "runs", call_runs, True),
        ("MechanicsCombinationCall", "pre_source_refusal (no_selected_operand)", call_refused, True),
        ("MechanicsCombinationCall", "two owners",
         dict(call_runs, owner_refs=[{"kind": "combination", "index": 0}] * 2), False),
        ("MechanicsCombinationCall", "a case owner",
         dict(call_runs, owner_refs=[{"kind": "case", "index": 0}]), False),
        ("Call", "a mechanics call as a case batch call", call_runs, False),
        ("CombinationGroup", "with imports", combination_group, True),
        ("Group", "a combination group with imports, as Group (closed)", combination_group, False),
        ("CombinationAttempt", "copied proof and trace", combination_attempt, True),
        ("CombinationAttempt", "with the ordinary definition id",
         dict(combination_attempt, definition_id="RP-PREPARED-ORDINARY-DUAL-v1"), False),
        ("CombinationAttempt", "with a preparation stage",
         dict(combination_attempt, stages=dict(combination_attempt["stages"], preparation="not_entered")), False),
        ("ProductAttempt", "a case attempt with the combination id",
         dict(copy.deepcopy(attempt), definition_id="RP-PREPARED-COMBINATION-DUAL-v1"), False),
        ("OperandPreparation", "prepared", operand_preparation, True),
        ("OperandPreparation", "with the exact id (RV116 N-8)", exact_operand_preparation, False),
        ("OperandPreparation", "empty requested_by", dict(operand_preparation, requested_by=[]), False),
        ("CaseSource", "an operand-prepared source", operand_source, True),
    ]
    rows = []
    for ref, label, value, expect in instances:
        a, b = judges["j1"].js(value, ref), judges["j1"].py(value, ref)
        rows.append({"def": ref, "case": label, "expected_valid": expect, "jsonschema": a, "py_walker": b,
                     "agrees": a == b == expect})
    # Body-level: operand_preparations [] is refused (present only when non-empty); absent is accepted.
    body_cases = []
    for label, mutate, expect in (
            ("B1 body unchanged", lambda body: None, True),
            ("operand_preparations: []", lambda body: body.__setitem__("operand_preparations", []), False),
            ("operand_preparations: [one]",
             lambda body: body.__setitem__("operand_preparations", [copy.deepcopy(operand_preparation)]), True),
            ("combinations: [retained_selected]",
             lambda body: body.__setitem__("combinations", [copy.deepcopy(selected)]), True)):
        receipt = copy.deepcopy(bases["two_case_synthetic"]["retained_precision"])
        mutate(receipt["body"])
        body_cases.append({"case": label, "expected_valid": expect, "jsonschema": judges["j1"].js(receipt),
                           "py_walker": judges["j1"].py(receipt),
                           "main_jsonschema": judges["main"].js(receipt)})
    report["new_defs_instances"] = rows
    report["body_instances"] = body_cases
    report["new_defs_all_agree"] = all(r["agrees"] for r in rows) and all(
        c["jsonschema"] == c["py_walker"] == c["expected_valid"] for c in body_cases)

    # 5. R-7: the published row set of a mechanics combination (committed preview-physics-1 fixture).
    fixture = "preview_physics_unicode_ids_sparse.json"
    env = json.loads((res / fixture).read_bytes())
    by_basis = defaultdict(list)
    for row in env["results"]:
        b = row.get("basis_ref") or {}
        by_basis[(b.get("ref_type"), b.get("ref_id"))].append(row)
    cases = [k for k in by_basis if k[0] == "load_case"]
    combos = [k for k in by_basis if k[0] == "combination"]
    case_rows = by_basis[cases[0]]
    n = sum(r["kind"] == "displacement_magnitude" for r in case_rows)
    m = sum(r["kind"] == "element_local_axial_force" for r in case_rows) // 5
    g = sum(r["kind"] == "support_reaction_force_magnitude_v2" for r in case_rows)
    rowset = {"fixture": fixture, "contract": env["producer"]["semantic_contract_id"], "n": n, "m": m, "g": g,
              "case_row_kinds": dict(Counter(r["kind"] for r in case_rows))}
    for key in combos:
        rws = by_basis[key]
        kinds = Counter(r["kind"] for r in rws)
        stress = [r for r in rws if r["kind"].startswith("element_local_") and "_stress" in r["kind"]]
        actions = [r for r in rws if r["kind"].startswith("element_local_") and "_stress" not in r["kind"]]
        per_member_sites = defaultdict(set)
        for r in stress:
            per_member_sites[r["entity_ref"]].add((r["kind"], r["metadata"]["location"]))
        rowset[key[1]] = {
            "rows": len(rws), "expected_7n_50m_8g": 7 * n + 50 * m + 8 * g,
            "kinds": dict(kinds),
            "stress_rows_per_member": {e: len(s) for e, s in per_member_sites.items()},
            "stress_sites": sorted({loc for s in per_member_sites.values() for _, loc in s}),
            "action_rows": len(actions),
            "absent_kinds": [k for k in ("pipe_elastic_normal_stress_maximum_v2",
                                         "component_equal_factor_intensified_bending_stress_v1",
                                         "linear_solver_mode_basis", "sparse_live_path_dense_parity_relative_delta",
                                         "modulus_basis_record", "combination_modulus_basis_record")
                             if k not in kinds],
            "contiguous_after_case_rows": [i for i, r in enumerate(env["results"])
                                           if (r.get("basis_ref") or {}).get("ref_id") == key[1]] == list(
                range(min(i for i, r in enumerate(env["results"]) if (r.get("basis_ref") or {}).get("ref_id") == key[1]),
                      min(i for i, r in enumerate(env["results"]) if (r.get("basis_ref") or {}).get("ref_id") == key[1])
                      + len(rws))),
        }
    rowset["combination_gates"] = env["contract_evidence"]["combination_gates"]
    report["r7_row_set"] = rowset
    Path(out_json).write_text(json.dumps(report, indent=2, sort_keys=True, ensure_ascii=True) + "\n")
    print(json.dumps({"meta": meta, "vocabulary_subset": report["vocabulary"]["j1_subset_of_main"],
                      "corpus_differences": len(differences), "new_defs_all_agree": report["new_defs_all_agree"],
                      "r7": {k: (v if not isinstance(v, dict) else {kk: vv for kk, vv in v.items()
                                                                    if kk in ("rows", "expected_7n_50m_8g")})
                             for k, v in rowset.items() if k not in ("case_row_kinds", "combination_gates")}},
                     indent=1, sort_keys=True, ensure_ascii=True))


if __name__ == "__main__":
    main()
