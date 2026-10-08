"""RV113 (RV-R): `_source_contract`'s differential (RV108 N1; the SR-PY rule "every input that did not raise there
reads exactly as before"). Run once per archive (I1, head); compare the two outputs with compare_py.py.

Inputs (built identically in each run, in memory, ids stable):
- every 07m entry (17 bases, 294 mutations, 28 must-pass), materialized by the RV113 harness's rule, read three ways:
  the raw successor, the reader's G7 projection (check_receipt on), and its transport projection (check_receipt off);
- enum variants: on four bases' projections, each of the five enum members (numerical_quality.status and case 0's
  solve_quality, structural_status, model_matrix_fidelity, accuracy_evidence) set to each of 19 values (valid and
  invalid strings, empty, null, numbers, booleans, lists, dicts), both check_receipt modes;
- every other envelope under fixtures/ (any JSON object with a `producer` member), raw, and with the same enum
  variants on its first quality case when it has one.
Each outcome: ["return", <json>] or [<exception type>, <message>].
Usage: python rv113_source_contract_diff.py <P root> <out.jsonl>
"""
import copy
import importlib.util
import json
import pathlib
import sys

root = sys.argv[1]
sys.path.insert(0, root)
from core.analysis_runs.compatibility import _source_contract  # noqa: E402

spec = importlib.util.spec_from_file_location("rv113h", pathlib.Path(__file__).with_name("rv113_py_harness_lib.py"))
lib = importlib.util.module_from_spec(spec); spec.loader.exec_module(lib)
lib.init(root)
corpus = lib.corpus

VALUES = ["checks_passed", "sensitive", "not_assessed", "passive_model_basis", "represented_equations_retained", "not_claimed",
          "bogus", "", None, 0, 1, 1.5, True, False, [], ["checks_passed"], ["passive_model_basis"], {}, {"value": "checks_passed"}]
FIELDS = [("status", None), ("solve_quality", 0), ("structural_status", 0), ("model_matrix_fidelity", 0), ("accuracy_evidence", 0)]


def projection(source, transport):
    p = copy.deepcopy(source); p.pop("retained_precision", None)
    if type(p.get("producer")) is dict:
        p["producer"]["semantic_contract_id"] = "openpipestress.result_semantics/0.3.0/preview-physics-1"
    if type(p.get("formulation_basis")) is dict:
        p["formulation_basis"]["profile_id"] = "product_preview_mechanics_v1"
    for row in p["results"] if type(p.get("results")) is list else []:
        if type(row) is dict:
            row.pop("recovery_method", None)
    return p


def outcome(src, check_receipt):
    try:
        r = _source_contract(copy.deepcopy(src), check_receipt=check_receipt)
        return ["return", json.loads(json.dumps(r, default=repr, sort_keys=True))]
    except Exception as e:  # noqa: BLE001
        return [type(e).__name__, str(e)]


def variants(src):
    q = src.get("numerical_quality")
    if type(q) is not dict:
        return
    for field, case in FIELDS:
        if case is not None and not (type(q.get("cases")) is list and q["cases"] and type(q["cases"][0]) is dict):
            continue
        for k, value in enumerate(VALUES):
            v = copy.deepcopy(src)
            target = v["numerical_quality"] if case is None else v["numerical_quality"]["cases"][case]
            target[field] = copy.deepcopy(value)
            yield f"{field}={k}", v


lines = []


def record(iid, src, modes=(True, False)):
    for m in modes:
        lines.append({"id": f"{iid}|check_receipt={m}", "outcome": outcome(src, m)})


for kind, entries in (("base", corpus["cases"]), ("mutation", corpus["mutations"]), ("must_pass", corpus["must_pass"])):
    for i, e in enumerate(entries):
        try:
            source = copy.deepcopy(e["source"]) if kind == "base" else lib.materialize(e)[0]
        except Exception as exc:  # noqa: BLE001
            lines.append({"id": f"{kind}:{i}:{e['id']}|materialize", "outcome": [type(exc).__name__, str(exc)]}); continue
        record(f"{kind}:{i}:{e['id']}|raw", source, (True,))
        record(f"{kind}:{i}:{e['id']}|g7", projection(source, False), (True,))
        record(f"{kind}:{i}:{e['id']}|transport", projection(source, True), (False,))
for bid in ["ordinary_prepared_synthetic", "two_case_preparation_failure_synthetic", "u8_l0_isolated_node_dense_scrutiny", "two_case_facade_after_certificate_synthetic"]:
    base = next(c for c in corpus["cases"] if c["id"] == bid)["source"]
    for tag, v in variants(projection(base, False)):
        record(f"variant:{bid}:{tag}", v)
fixtures = sorted(pathlib.Path(root, "fixtures").rglob("*.json"))
for f in fixtures:
    try:
        data = json.loads(f.read_text())
    except Exception:  # noqa: BLE001
        continue
    docs = [data] if type(data) is dict and "producer" in data else []
    if type(data) is dict and type(data.get("source")) is dict and "producer" in data["source"]:
        docs.append(data["source"])
    for k, d in enumerate(docs):
        rel = str(f.relative_to(root))
        record(f"fixture:{rel}:{k}", d)
        for tag, v in variants(d):
            record(f"fixture:{rel}:{k}:{tag}", v)
with open(sys.argv[2], "w") as out:
    for line in lines:
        out.write(json.dumps(line, sort_keys=True) + "\n")
print(len(lines), "outcomes ->", sys.argv[2])
