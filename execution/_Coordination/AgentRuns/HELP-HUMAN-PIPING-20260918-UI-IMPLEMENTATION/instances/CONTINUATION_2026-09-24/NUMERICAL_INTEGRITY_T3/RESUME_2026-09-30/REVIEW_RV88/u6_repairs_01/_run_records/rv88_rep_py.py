"""RV88 (U6 repair confirmation), Python side: the 20 shared v2 cases and the 4
declared differences by RV88's own Python mapping, and the legacy-token paths
(U6b S-1) on every Python carrier. Oracle for binding: the receipt's own list.
Usage: python rv88_rep_py.py <lane P> <out.tsv>"""
import json, sys
from copy import deepcopy
from pathlib import Path

P, OUT = Path(sys.argv[1]), Path(sys.argv[2])
sys.path.insert(0, str(P))
from core.analysis_runs import compatibility as c  # noqa: E402
from core.analysis_runs import records as r  # noqa: E402

METHOD = "contribution_preserving_multiprecision_v1"
KW = {"input_manifest_ref": {"object_type": "InputManifest", "ref": "manifest:rv88"}, "input_manifest_hash": "a" * 64}
rows, fails = [], []


def err(f):
    try:
        return ("ok", f())
    except Exception as e:  # noqa: BLE001
        return ("err", str(e))


def setp(t, path, v):
    for k in path[:-1]:
        t = t[k]
    t[path[-1]] = v


def refs(inv):
    return [{"ref_type": "load_case", "ref_id": lc["id"]} for lc in inv["request"]["model"]["load_cases"]]


cf = json.loads((P / "fixtures/results/retained_precision_carrier_cases.json").read_text())
for case in cf["cases"]:
    fx = cf["fixtures"][case["fixture"]]
    doc = json.loads((P / fx["path"]).read_text())
    src, inv = (deepcopy(doc["source"]), deepcopy(doc["invocation"])) if fx["shape"] == "milestone" else (deepcopy(doc), None)
    for e in case["edits"]:
        setp(src if e["target"] == "source" else inv, e["path"], e["value"])
    req = refs(inv) if case["requested"] == "invocation" else case["requested"]
    st = c.numerical_use_standing(deepcopy(src), req, inv if case["invocation"] is not None else None)
    d = err(lambda: c._source_contract(deepcopy(src)))
    dispatch = "ok" if d[0] == "ok" else d[1]
    rows.append(f"case\t{case['id']}\t{st}\t{dispatch}\t{case['expected_standing']}\t{case['expected_dispatch']}")
    if [st, dispatch] != [case["expected_standing"], case["expected_dispatch"]]:
        fails.append(case["id"])
for dd in cf["declared_differences"]:
    for fxn in dd["fixtures"]:
        doc = json.loads((P / cf["fixtures"][fxn]["path"]).read_text())
        src, inv = deepcopy(doc["source"]), doc["invocation"]
        for e in dd["edits"]:
            setp(src, e["path"], e["value"])
        listed = {x["result_id"] for x in doc["source"]["retained_precision"]["body"]["cases"][0]["selection"]["absolute_verified"]}
        if dd["subject"] == "standing":
            got = c.numerical_use_standing(deepcopy(src), refs(inv), None)
        elif dd["subject"] == "transport":
            t = err(lambda: c._source_contract(deepcopy(src), check_receipt=False))
            got = "ok" if t[0] == "ok" else t[1]
        else:
            codes = [c.rule_binding_refusal(src, row) for row in src["results"]]
            if all(x == "RULE_QUANTITY_NOT_COVERED" for x in codes):
                got = "every_row:RULE_QUANTITY_NOT_COVERED"
            elif all((x == "RULE_QUANTITY_BELOW_VERIFIED_FLOOR") == (row["id"] in listed) and x in (None, "RULE_QUANTITY_BELOW_VERIFIED_FLOOR") for row, x in zip(src["results"], codes)):
                got = "by_validated_class"
            else:
                got = "OTHER"
        want = list(dd["expected"]["python"].values())[0]
        rows.append(f"dd\t{dd['id']}\t{fxn}\t{got}\t{want}")
        if got != want:
            fails.append(dd["id"])
# U6b S-1: a legacy 0.1.0 source with the token on one row, every Python path.
legacy = json.loads((P / "fixtures/product_preview/invented_mechanics_result.json").read_text())
for where in ("first", "middle", "last"):
    for method in (METHOD, "rv88_other_method"):
        s = deepcopy(legacy)
        i = {"first": 0, "middle": len(s["results"]) // 2, "last": -1}[where]
        s["results"][i]["recovery_method"] = method
        out = {
            "raw": err(lambda: list(c._source_contract(deepcopy(s))[:1])),
            "transport": err(lambda: list(c._source_contract(deepcopy(s), check_receipt=False)[:1])),
            "standing": c.numerical_use_standing(deepcopy(s), []),
            "build_analysis_run": err(lambda: c.build_analysis_run(deepcopy(s), **KW)["schema_version"]),
            "build_v0_2": err(lambda: c.build_analysis_run_v0_2(deepcopy(s), **KW)["schema_version"]),
            "wrapper_0_1_0": err(lambda: r.build_preview_analysis_run_envelope(deepcopy(s))["schema_version"]),
        }
        rows.append(f"legacy_token\t{where}\t{method}\t{json.dumps(out)}")
        refused = method == METHOD
        ok = (out["raw"] == ("err", "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN") and out["standing"] == "unsupported"
              and out["build_analysis_run"][0] == "err" and out["build_v0_2"] == ("err", "ANALYSIS_LEGACY_SOURCE_DOWNGRADE_FORBIDDEN")
              and out["wrapper_0_1_0"] == ("err", "ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN")) if refused else all(
              v[0] == "ok" if isinstance(v, tuple) else v == "needs_recompute" for k, v in out.items())
        if not ok:
            fails.append(f"legacy_token/{where}/{method}")
OUT.write_text("\n".join(rows) + "\n")
print(len(rows), "rows; failures:", fails)
