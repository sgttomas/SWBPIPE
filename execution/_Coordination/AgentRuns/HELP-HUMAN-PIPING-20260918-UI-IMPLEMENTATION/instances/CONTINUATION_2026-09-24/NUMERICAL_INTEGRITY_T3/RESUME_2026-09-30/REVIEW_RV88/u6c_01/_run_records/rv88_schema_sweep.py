"""RV88 (U6c review): every carrier document under P, plus the real U6a Rust
derivatives (successor and its projection, both modes), validated in one lane
by that lane's own schemas: the dispatchers AND the exact v0.3 schemas (the
results dispatcher refuses every v0.3 identity but precision-1, I66 F-U6c-2,
so a dispatcher-only sweep cannot see results.v0.3 branch changes). Then
injection probes on every document the exact schema accepts.
Usage: python rv88_schema_sweep.py <lane P> <u6a slice dir> <out.tsv>"""
import json
import sys
from copy import deepcopy
from pathlib import Path

P, SLICE, OUT = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
sys.path.insert(0, str(P))
from tests.schema_validation import validate_instance  # noqa: E402

SCHEMAS = {name: json.loads((P / "schemas" / name).read_text()) for name in (
    "results.schema.yaml", "results.v0.3.schema.yaml", "analysis_run.schema.json", "analysis_run.v0.3.schema.json",
    "stress_neutral_export.schema.json", "stress_neutral_export.v0.3.schema.json")}
EXACT = {"results": "results.v0.3.schema.yaml", "analysis_run": "analysis_run.v0.3.schema.json", "stress_neutral": "stress_neutral_export.v0.3.schema.json"}
DISPATCH = {"results": "results.schema.yaml", "analysis_run": "analysis_run.schema.json", "stress_neutral": "stress_neutral_export.schema.json"}
RECEIPT = json.loads((SLICE / "cand_sparse_interactive_receipt_from_derivative.json").read_text())
CODES = ("retained_precision_absolute_verified", "retained_precision_not_covered")


def kind(v):
    if not isinstance(v, dict) or not isinstance(v.get("schema_version"), str):
        return None
    if "result_envelope" in v:
        return "results"
    if "analysis_run" in v and "run_contract_status" in v:
        return "analysis_run"
    if "csv_text" in v and "manifest" in v:
        return "stress_neutral"
    return None


def walk(v, ptr, out, depth=0):
    if depth > 9:
        return
    k = kind(v)
    if k:
        out.append((ptr, k, v))
    if isinstance(v, dict):
        for key, child in v.items():
            walk(child, f"{ptr}/{key}", out, depth + 1)
    elif isinstance(v, list):
        for i, child in enumerate(v):
            walk(child, f"{ptr}/{i}", out, depth + 1)


def verdict(schema_name, doc):
    try:
        validate_instance(SCHEMAS[schema_name], doc, instance_label="rv88")
        return "valid"
    except AssertionError as e:
        return "invalid:" + " | ".join(str(e).splitlines()[1:2])[:160]
    except Exception as e:  # noqa: BLE001
        return f"error:{type(e).__name__}"


docs = []
for root in ("fixtures", "tests", "core", "apps", "validation", "schemas"):
    base = P / root
    if not base.exists():
        continue
    for path in sorted(base.rglob("*.json")):
        if "node_modules" in path.parts or "target" in path.parts:
            continue
        try:
            doc = json.loads(path.read_text())
        except Exception:  # noqa: BLE001
            continue
        found = []
        walk(doc, "", found)
        docs += [(f"{path.relative_to(P)}#{ptr}", k, v) for ptr, k, v in found]
for f in sorted(SLICE.glob("cand_*_successor.json")) + sorted(SLICE.glob("cand_*_projection.json")):
    docs.append((f"U6A_RUST_DERIVATIVE/{f.name}", "results", json.loads(f.read_text())))

rows = []
for key, k, doc in docs:
    v_exact = verdict(EXACT[k], doc)
    rows.append(f"{key}\t{k}\tdispatch\t{verdict(DISPATCH[k], doc)}")
    rows.append(f"{key}\t{k}\texact\t{v_exact}")
    if v_exact != "valid":
        continue
    probes = []
    if k == "results":
        env = doc["result_envelope"]
        successor = env.get("producer", {}).get("semantic_contract_id", "").endswith("preview-physics-retained-1")
        if successor:
            d = deepcopy(doc); del d["result_envelope"]["retained_precision"]; probes.append(("drop_receipt", d))
            d = deepcopy(doc); d["result_envelope"]["retained_precision"]["body"]["policy"] = "M03-INTEGRITY-MP-v1"; probes.append(("receipt_policy_v1", d))
            d = deepcopy(doc); d["result_envelope"]["retained_precision"]["rv88"] = 1; probes.append(("receipt_extra_member", d))
            dis = env.get("row_disclosures") or []
            for code in CODES + ("rv88_unknown_reason",):
                if dis:
                    d = deepcopy(doc); d["result_envelope"]["row_disclosures"][0]["reason_code"] = code; probes.append((f"disclosure_{code}", d))
        else:
            d = deepcopy(doc); d["result_envelope"]["retained_precision"] = deepcopy(RECEIPT); probes.append(("inject_receipt", d))
            d = deepcopy(doc); d["result_envelope"]["retained_precision"] = None; probes.append(("inject_null_receipt", d))
            dis = env.get("row_disclosures") or []
            for code in CODES:
                if dis:
                    d = deepcopy(doc); d["result_envelope"]["row_disclosures"][0]["reason_code"] = code; probes.append((f"disclosure_{code}", d))
    elif k == "analysis_run":
        d = deepcopy(doc); d["analysis_run"]["retained_precision"] = deepcopy(RECEIPT); probes.append(("inject_receipt", d))
    elif k == "stress_neutral":
        d = deepcopy(doc); d["retained_precision"] = deepcopy(RECEIPT); probes.append(("inject_receipt", d))
    for name, d in probes:
        rows.append(f"{key}!{name}\t{k}\texact\t{verdict(EXACT[k], d)}")
OUT.write_text("\n".join(rows) + "\n")
print(len(docs), "documents;", len(rows), "rows")
