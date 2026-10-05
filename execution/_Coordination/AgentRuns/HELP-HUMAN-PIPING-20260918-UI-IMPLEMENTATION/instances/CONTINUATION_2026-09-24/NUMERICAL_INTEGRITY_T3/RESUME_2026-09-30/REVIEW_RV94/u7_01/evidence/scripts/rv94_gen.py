"""RV94 input generator (scratch only). Builds concrete statement bytes once, so that
all three languages read the same bytes. Mechanical corpus application (edits,
rehash "all") uses the committed Python test helper `apply_entry`/`apply_mutation`
and the reader's hash helpers; NO expectation is taken from any reader here.

Usage: python rv94_gen.py <P root> <out.jsonl>
Each record: {id, group, source, invocation|null, requested:[refs], g: "pass"|"fail"|"observe", note}
"""
import json
import sys
from copy import deepcopy
from pathlib import Path

P = Path(sys.argv[1]).resolve()
OUT = Path(sys.argv[2])
sys.path.insert(0, str(P))
sys.path.insert(0, str(P / "tests"))
import test_retained_precision_contract as T  # noqa: E402  (mechanical helpers only)

corpus = json.loads((P / "fixtures/results/retained_precision_cases.json").read_text())
carriers = json.loads((P / "fixtures/results/retained_precision_carrier_cases.json").read_text())
records = []


def refs(inv):
    if not isinstance(inv, dict):
        return []
    cases = inv.get("request", {}).get("model", {}).get("load_cases")
    return [{"ref_type": "load_case", "ref_id": c.get("id")} for c in cases] if isinstance(cases, list) else []


def add(id_, group, source, invocation, requested, g, note=""):
    records.append({"id": id_, "group": group, "source": source, "invocation": invocation,
                    "requested": requested, "g": g, "note": note})


bases = {c["id"]: c for c in corpus["cases"]}
# 1. corpus bases, with and without their invocation
for c in corpus["cases"]:
    add(f"base:{c['id']}", "base", deepcopy(c["source"]), deepcopy(c["invocation"]), refs(c["invocation"]), "pass")
    add(f"base:{c['id']}:noinv", "base_noinv", deepcopy(c["source"]), None, refs(c["invocation"]), "pass")
# 2. must-pass entries, with and without their invocation
for e in corpus["must_pass"]:
    src, inv = T.apply_entry(bases[e["base"]], e)
    add(f"mp:{e['id']}", "must_pass", src, inv, refs(inv), "pass")
    add(f"mp:{e['id']}:noinv", "must_pass_noinv", deepcopy(src), None, refs(inv), "pass")
# 3. mutations (all expected to refuse)
for e in corpus["mutations"]:
    src, inv = T.apply_entry(bases[e["base"]], e)
    add(f"mut:{e['id']}", "mutation", src, inv, refs(inv), "fail", json.dumps(e["expected"]))

# 4. carrier cases (literal edits, no rehash)
def fixture(fid):
    meta = carriers["fixtures"][fid]
    doc = json.loads((P / meta["path"]).read_text())
    if meta["shape"] == "milestone":
        return doc["source"], doc["invocation"]
    return doc, None


def set_path(target, path, value):
    at = target
    for k in path[:-1]:
        at = at[k]
    at[path[-1]] = deepcopy(value)


for c in carriers["cases"]:
    src, fx_inv = fixture(c["fixture"])
    src, fx_inv = deepcopy(src), deepcopy(fx_inv)
    inv = fx_inv if c["invocation"] == "fixture" else deepcopy(c["invocation"])
    for ed in c["edits"]:
        set_path(src if ed["target"] == "source" else inv, ed["path"], ed["value"])
    req = refs(fx_inv) if c["requested"] == "invocation" else c["requested"]
    g = "fail" if c["expected_standing"] == "unsupported" else "pass"
    add(f"carrier:{c['id']}", "carrier", src, inv, req, g, c["expected_standing"])

# 5. declared-difference standing forms
for d in carriers["declared_differences"]:
    for f in d["forms"]:
        if f["subject"] != "standing":
            continue
        for fid in f["fixtures"]:
            src, fx_inv = fixture(fid)
            src, fx_inv = deepcopy(src), deepcopy(fx_inv)
            inv = fx_inv if f["invocation"] == "fixture" else deepcopy(f["invocation"])
            for ed in f["edits"]:
                set_path(src if ed["target"] == "source" else inv, ed["path"], ed["value"])
            req = refs(fx_inv) if f["requested"] == "invocation" else f["requested"]
            exp = f["expected"]["python"]["standing"]
            add(f"declared:{d['id']}:{f['label']}:{fid}", "declared", src, inv, req,
                "fail" if exp == "unsupported" else "pass", json.dumps(f.get("capture")) + " " + json.dumps(f.get("current_model_edits")))

# 6. live milestone successors (byte-identical to PP's pinned output, D-U6-5)
for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = json.loads((P / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())
    src, inv = doc["source"], doc["invocation"]
    r = refs(inv)
    add(f"ms:{mode}:inv", "milestone", deepcopy(src), deepcopy(inv), r, "pass")
    add(f"ms:{mode}:noinv", "milestone", deepcopy(src), None, r, "pass")
    add(f"ms:{mode}:inv:req_empty", "milestone", deepcopy(src), deepcopy(inv), [], "pass")
    add(f"ms:{mode}:inv:req_other", "milestone", deepcopy(src), deepcopy(inv), [{"ref_type": "load_case", "ref_id": "other"}], "pass")
    add(f"ms:{mode}:inv:req_dup", "milestone", deepcopy(src), deepcopy(inv), r + r, "pass")
    add(f"ms:{mode}:inv:req_wrong_type", "milestone", deepcopy(src), deepcopy(inv), [{"ref_type": "load_combination", "ref_id": r[0]["ref_id"]}], "pass")
    # hostile invocations: mismatched mode, moved node, materials dropped, an empty object
    other = "dense_scrutiny" if mode == "sparse_interactive" else "sparse_interactive"
    i2 = deepcopy(inv); i2["solver_mode"] = other
    add(f"ms:{mode}:inv_mode_swapped", "hostile", deepcopy(src), i2, r, "fail", "G8 by contract (C1 G8: actual raw invocation+mode hash)")
    i3 = deepcopy(inv); i3["request"]["model"]["nodes"][0]["position"]["x"] = 1.0
    add(f"ms:{mode}:inv_node_moved", "hostile", deepcopy(src), i3, r, "fail", "G8 by contract")
    i4 = deepcopy(inv); del i4["request"]["materials"]
    add(f"ms:{mode}:inv_materials_dropped", "hostile", deepcopy(src), i4, r, "fail", "G8 by contract")
    add(f"ms:{mode}:inv_empty_object", "hostile", deepcopy(src), {}, r, "fail", "G8 by contract")
    # hostile statement: blocked envelope made hash-consistent (rehash all)
    for status in ("MODEL_INCOMPLETE", "MECHANICS_FAILED", "NOT_RUN"):
        s = T.apply_mutation(src, {"edits": [{"path": ["status", "mechanics"], "op": "set", "value": status}], "rehash": "all"})
        add(f"ms:{mode}:blocked:{status}", "hostile", s, deepcopy(inv), r, "fail", "G7 blocked envelope (scope clause)")
    # hostile statement: unrehashed row edit
    s = deepcopy(src); s["results"][0]["value"] = 12345.0
    add(f"ms:{mode}:edited_row_unrehashed", "hostile", s, deepcopy(inv), r, "fail", "G1")

# 7. hostile not_required variants on the 07j entry (gate outcome observed, not assumed)
nr = next(e for e in corpus["must_pass"] if e["id"] == "not_required_second_case_checks_passed")
nr_base = bases[nr["base"]]


def nr_variant(tag, extra_edits, invocation_edits=None, g="observe", note=""):
    e = deepcopy(nr)
    e["edits"] = e["edits"] + extra_edits
    if invocation_edits:
        e["invocation_edits"] = invocation_edits
    src, inv = T.apply_entry(nr_base, e)
    add(f"nr:{tag}", "hostile_nr", src, inv, refs(inv), g, note)
    return src, inv


src_nr, inv_nr = T.apply_entry(nr_base, nr)
r_nr = refs(inv_nr)
add("nr:req_selected_only", "hostile_nr", deepcopy(src_nr), deepcopy(inv_nr), r_nr[:1], "pass", "requested omits the not_required case")
add("nr:req_reversed", "hostile_nr", deepcopy(src_nr), deepcopy(inv_nr), list(reversed(r_nr)), "pass", "requested order reversed")
nr_variant("q_not_assessed", [{"path": ["numerical_quality", "cases", 1, "solve_quality"], "op": "set", "value": "not_assessed"}])
nr_variant("q_evidence_empty", [{"path": ["numerical_quality", "cases", 1, "evidence_refs"], "op": "set", "value": []}])
nr_variant("q_evidence_unresolved", [{"path": ["numerical_quality", "cases", 1, "evidence_refs"], "op": "set", "value": ["result:does-not-exist"]}])
nr_variant("q_accuracy_claimed", [{"path": ["numerical_quality", "cases", 1, "accuracy_evidence"], "op": "set", "value": "estimated"}])
nr_variant("q_structural_other", [{"path": ["numerical_quality", "cases", 1, "structural_status"], "op": "set", "value": "mechanism_detected"}])
nr_variant("q_fidelity_other", [{"path": ["numerical_quality", "cases", 1, "model_matrix_fidelity"], "op": "set", "value": "reduced"}])
nr_variant("q_basis_ref_other", [{"path": ["numerical_quality", "cases", 1, "basis_ref", "ref_id"], "op": "set", "value": "case:other"}])
nr_variant("desktop_shaped_invocation", [], [{"path": ["request", "materials"], "op": "set", "value": []}], note="07j bound to the desktop-captured request shape {model, materials: []}")
add("nr:noinv", "hostile_nr", deepcopy(src_nr), None, r_nr, "pass")

OUT.write_text("".join(json.dumps(r) + "\n" for r in records))
print(len(records), "records")
