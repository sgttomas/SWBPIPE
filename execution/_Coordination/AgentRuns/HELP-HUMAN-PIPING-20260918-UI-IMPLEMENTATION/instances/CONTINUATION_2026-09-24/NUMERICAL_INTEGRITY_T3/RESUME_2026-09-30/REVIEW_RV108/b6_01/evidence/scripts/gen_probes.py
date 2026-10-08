"""RV108: own probe set (single-defect, hash-consistent G7 header probes; tampered transports; beyond-header probes).
Writes S/probes/probes.json: [{id, family, base, source, invocation}]. Rehash 'all' as the shared format defines it
(preparation, source identity, publication, receipt), implemented here, using the reader's hash primitive only."""
import json, math, sys
from copy import deepcopy
from pathlib import Path
PROOT = Path(sys.argv[1]); OUT = Path(sys.argv[2])
sys.path.insert(0, str(PROOT))
from core.analysis_runs import retained_precision as rp

corpus = json.loads((PROOT / "fixtures/results/retained_precision_cases.json").read_text())
cases = {c["id"]: c for c in corpus["cases"]}

def idx(items, ref):
    if type(ref) not in (int, float) or not math.isfinite(ref) or ref != int(ref) or ref < 0: return None
    return items[int(ref)] if int(ref) < len(items) else None

def rehash(v):
    r = v.get("retained_precision")
    if isinstance(r, dict) and isinstance(r.get("body"), dict):
        b = r["body"]
        for s in b["sources"]:
            p = s["preparation"]; a = idx(b["product_attempts"], p["attempt_ref"]) if p is not None else None
            if a is not None and all(m["result"]["kind"] == "prepared" for m in a["preparation"]["members"]):
                p["sha256"] = rp._hash("retained_precision_preparation_v1", rp._preparation_payload(a))
        for c in b["cases"]:
            s = idx(b["sources"], c.get("source_ref")) if c["status"] == "selected" else None
            if s is not None: c["source_identity_sha256"] = rp._source_hash(s)
        b["publication_sha256"] = rp._hash("retained_precision_publication_mp_v2", {k: x for k, x in v.items() if k != "retained_precision"})
        r["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", b)
    return v

def reseal_receipt(v):
    v["retained_precision"]["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", v["retained_precision"]["body"]); return v

def setp(v, path, val):
    for p in path[:-1]: v = v[p]
    v[path[-1]] = deepcopy(val)
def delp(v, path):
    for p in path[:-1]: v = v[p]
    del v[path[-1]]

bases = {}
for cid in ("ordinary_prepared_synthetic", "two_case_preparation_failure_synthetic", "u8_l0_isolated_node_sparse_interactive", "two_case_synthetic"):
    bases[cid] = (cases[cid]["source"], cases[cid]["invocation"])
nr = next(m for m in corpus["must_pass"] if m["id"] == "not_required_second_case_checks_passed")
s = deepcopy(cases[nr["base"]]["source"])
for e in nr["edits"]:
    (delp(s, e["path"]) if e["op"] == "remove" else setp(s, e["path"], e["value"]))
bases["NR:not_required_second_case_checks_passed"] = (rehash(s), cases[nr["base"]]["invocation"])
for mode in ("sparse_interactive", "dense_scrutiny"):
    doc = json.loads((PROOT / f"fixtures/results/retained_precision_milestone_successor_{mode}.json").read_text())
    bases[f"milestone_{mode}"] = (doc["source"], doc["invocation"])

probes = []
def add(pid, family, base, fn, mode="rehash"):
    src, inv = deepcopy(bases[base][0]), deepcopy(bases[base][1])
    if fn: fn(src)
    if mode == "rehash": rehash(src)
    elif mode == "reseal": reseal_receipt(src)
    probes.append({"id": f"{base}|{pid}", "family": family, "base": base, "source": src, "invocation": inv})

BAD = {"str": "mechanism_detected", "empty": "", "null": None, "int": 1, "bool": True, "list": [], "dict": {}, "list_valid": ["passive_model_basis"]}
for base in bases:
    add("control", "control", base, None)
    nq = len(bases[base][0]["numerical_quality"]["cases"])
    for i in range(nq):
        for field in ("structural_status", "model_matrix_fidelity", "accuracy_evidence", "solve_quality"):
            for tag, val in BAD.items():
                add(f"case{i}.{field}={tag}", "A_case_enum", base, lambda s, i=i, f=field, v=val: setp(s, ["numerical_quality", "cases", i, f], v))
        for tag, val in {"empty_str": [""], "int": [1], "null_item": [None], "str": "x", "null": None, "dict": {}, "nested": [[]], "empty_list": []}.items():
            add(f"case{i}.evidence_refs={tag}", "A_case_refs", base, lambda s, i=i, v=val: setp(s, ["numerical_quality", "cases", i, "evidence_refs"], v))
        add(f"case{i}.extra", "A_case_keys", base, lambda s, i=i: setp(s, ["numerical_quality", "cases", i, "zz"], 1))
        for k in ("structural_status", "model_matrix_fidelity", "accuracy_evidence", "evidence_refs", "solve_quality"):
            add(f"case{i}.remove_{k}", "A_case_keys", base, lambda s, i=i, k=k: delp(s, ["numerical_quality", "cases", i, k]))
        add(f"case{i}.basis_ref.extra", "A_case_basis", base, lambda s, i=i: setp(s, ["numerical_quality", "cases", i, "basis_ref", "zz"], 1))
        add(f"case{i}=null", "A_case_keys", base, lambda s, i=i: setp(s, ["numerical_quality", "cases", i], None))
    for tag, val in {"str": "estimated", "empty": "", "null": None, "int": 1, "list": [], "dict": {}}.items():
        add(f"quality.status={tag}", "A_quality", base, lambda s, v=val: setp(s, ["numerical_quality", "status"], v))
    for k, val in (("value_representation", "binary32"), ("publication_quantization", "rounded"), ("integrity_policy", "M03-INTEGRITY-v2"), ("value_representation", []), ("integrity_policy", {})):
        add(f"quality.{k}={json.dumps(val)}", "A_quality", base, lambda s, k=k, v=val: setp(s, ["numerical_quality", k], v))
    add("quality.extra", "A_quality", base, lambda s: setp(s, ["numerical_quality", "zz"], 1))
    for k in ("status", "value_representation", "publication_quantization", "integrity_policy"):
        add(f"quality.remove_{k}", "A_quality", base, lambda s, k=k: delp(s, ["numerical_quality", k]))
    for tag, val in {"empty": [], "empty_str": [""], "int": [1], "str": "x", "null": None, "dict": {}, "nested": [[]], "other_nonempty": ["rv108 other limitation"]}.items():
        add(f"formulation.limitations={tag}", "A_formulation", base, lambda s, v=val: setp(s, ["formulation_basis", "limitations"], v))
    add("formulation.extra", "A_formulation", base, lambda s: setp(s, ["formulation_basis", "zz"], 1))
    add("formulation.remove_limitations", "A_formulation", base, lambda s: delp(s, ["formulation_basis", "limitations"]))
    for tag, val in {"null": None, "list": [], "str": "x", "zero": 0, "true": True, "false": False}.items():
        add(f"contract_evidence={tag}", "A_evidence_required", base, lambda s, v=val: setp(s, ["contract_evidence"], v))
    add("contract_evidence.remove", "A_evidence_required", base, lambda s: delp(s, ["contract_evidence"]))
    for tag, val in {"null": None, "dict": {}, "list": [], "zero": 0, "str": "x"}.items():
        add(f"source_block_recovery={tag}", "A_source_blocks", base, lambda s, v=val: setp(s, ["source_block_recovery"], v))
    for tag, val in {"dict": {}, "null": None, "list": [], "zero": 0}.items():
        add(f"carrier_evidence={tag}", "A_carrier", base, lambda s, v=val: setp(s, ["carrier_evidence"], v))
    add("producer.extra", "A_producer", base, lambda s: setp(s, ["producer", "zz"], 1))
    # Beyond the header: top-level extra member; evidence content.
    add("toplevel.extra", "B_beyond", base, lambda s: setp(s, ["zz_rv108"], 1))
    add("contract_evidence.extra", "B_beyond", base, lambda s: setp(s, ["contract_evidence", "zz"], 1))
    add("contract_evidence.combination_gates=str", "B_beyond", base, lambda s: setp(s, ["contract_evidence", "combination_gates"], "x"))
    # Transport tampering (no publication rehash; the receipt resealed or not as named).
    add("T.receipt_sha_zero", "T_tamper", base, lambda s: setp(s, ["retained_precision", "receipt_sha256"], "0" * 64), mode="none")
    add("T.body_edit_unsealed", "T_tamper", base, lambda s: setp(s, ["retained_precision", "body", "work", "charged"], s["retained_precision"]["body"]["work"]["charged"] + 1), mode="none")
    add("T.body_edit_resealed", "T_tamper", base, lambda s: setp(s, ["retained_precision", "body", "work", "charged"], s["retained_precision"]["body"]["work"]["charged"] + 1), mode="reseal")
    add("T.receipt_empty", "T_tamper", base, lambda s: setp(s, ["retained_precision"], {}), mode="none")
    add("T.receipt_null", "T_tamper", base, lambda s: setp(s, ["retained_precision"], None), mode="none")
    add("T.receipt_body_empty", "T_tamper", base, lambda s: setp(s, ["retained_precision", "body"], {}), mode="reseal")
    add("T.no_results", "T_tamper", base, lambda s: delp(s, ["results"]), mode="none")
    add("T.no_results_sha_zero", "T_tamper", base, lambda s: (delp(s, ["results"]), setp(s, ["retained_precision", "receipt_sha256"], "0" * 64)), mode="none")
    add("T.results_row_edit_unsealed", "T_tamper", base, lambda s: setp(s, ["results", 0, "id"], "rv108-other"), mode="none")
    add("T.receipt_version_2_resealed", "T_tamper", base, lambda s: setp(s, ["retained_precision", "body", "receipt_version"], 2), mode="reseal")
    add("T.policy_wrong_resealed", "T_tamper", base, lambda s: setp(s, ["retained_precision", "body", "policy"], "M03-INTEGRITY-MP-v1"), mode="reseal")
    add("T.negzero_resealed", "T_tamper", base, lambda s: setp(s, ["retained_precision", "body", "work", "charged"], -0.0), mode="reseal")
    add("T.receipt_extra_member_resealed", "T_tamper", base, lambda s: setp(s, ["retained_precision", "body", "zz"], 1), mode="reseal")
    add("T.receipt_sha_upper", "T_tamper", base, lambda s: setp(s, ["retained_precision", "receipt_sha256"], s["retained_precision"]["receipt_sha256"].upper()), mode="none")
    add("T.quality_enum_unsealed_pub", "T_header", base, lambda s: setp(s, ["numerical_quality", "cases", 0, "accuracy_evidence"], "estimated"), mode="none")
    add("T.quality_status_list", "T_header", base, lambda s: setp(s, ["numerical_quality", "status"], []), mode="none")
    add("T.contract_evidence_extra", "T_header", base, lambda s: setp(s, ["contract_evidence", "zz"], 1), mode="none")
    add("T.carrier_evidence", "T_header", base, lambda s: setp(s, ["carrier_evidence"], {}), mode="none")
    add("T.source_block_recovery", "T_header", base, lambda s: setp(s, ["source_block_recovery"], None), mode="none")
    add("T.contract_evidence_null", "T_header", base, lambda s: setp(s, ["contract_evidence"], None), mode="none")
    add("T.limitations_empty", "T_header", base, lambda s: setp(s, ["formulation_basis", "limitations"], []), mode="none")
    add("T.limitations_other", "T_header", base, lambda s: setp(s, ["formulation_basis", "limitations"], ["rv108 other"]), mode="none")
    add("T.producer_extra", "T_header", base, lambda s: setp(s, ["producer", "zz"], 1), mode="none")
    add("T.component_version", "T_header", base, lambda s: setp(s, ["producer", "component_version"], "0.3.0"), mode="none")
    add("T.row_token_removed", "T_header", base, lambda s: s["results"][0].pop("recovery_method", None), mode="none")
OUT.write_text(json.dumps(probes))
from collections import Counter
print(len(probes), Counter(p["family"] for p in probes))
