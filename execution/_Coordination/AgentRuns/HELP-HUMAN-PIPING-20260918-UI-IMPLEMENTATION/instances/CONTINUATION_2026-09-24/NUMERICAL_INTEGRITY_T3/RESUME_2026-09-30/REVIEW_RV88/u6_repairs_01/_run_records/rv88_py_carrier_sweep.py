"""RV88 (U6b review): independent sweep of the Python carriers, one lane at a time
(base cb03315779, candidate c89a7a986c), base APIs only. Walks every JSON file
under P (not execution), recursing to depth 9 into objects that carry a producer
id, plus legacy 0.1.0 raw envelopes. Per envelope, and per injected downgrade
form: raw and transport dispatch, standing (quality refs, empty, with the
sibling invocation), standing reason, every row's binding refusal, the 0.3 and
0.2 AnalysisRun builds and the 0.3 validation, and the 0.1.0 wrapper.
Usage: python rv88_py_carrier_sweep.py <lane P> <sparse fixture> <out.tsv>"""
import hashlib
import json
import sys
from copy import deepcopy
from pathlib import Path

P, SPARSE, OUT = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
sys.path.insert(0, str(P))
from core.analysis_runs import compatibility as c  # noqa: E402
from core.analysis_runs import records as r  # noqa: E402

SUCC = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
METHOD = "contribution_preserving_multiprecision_v1"
RECEIPT = json.loads(SPARSE.read_text())["source"]["retained_precision"]
NOTICE = "Retained-precision recovery is unavailable for this load case. Its published rows keep their ordinary values, standing and diagnostics; the retained_precision receipt records the actual attempt and its typed cause."
KW = {"input_manifest_ref": {"object_type": "InputManifest", "ref": "manifest:rv88"}, "input_manifest_hash": "8" * 64}


def h(v):
    return hashlib.sha256(json.dumps(v, sort_keys=True, separators=(",", ":")).encode()).hexdigest()[:16]


def safe(f):
    try:
        return "ok:" + json.dumps(f(), sort_keys=True, default=str)[:400]
    except Exception as e:  # noqa: BLE001
        return f"err:{type(e).__name__}:{str(e)[:160]}"


def find(v, ptr, out, depth=0):
    if depth > 9 or not isinstance(v, (dict, list)):
        return
    if isinstance(v, dict):
        ident = isinstance(v.get("producer"), dict) and isinstance(v["producer"].get("semantic_contract_id"), str)
        legacy = v.get("schema_version") == "0.1.0" and isinstance(v.get("results"), list) and isinstance(v.get("status"), dict)
        if (ident or legacy) and isinstance(v.get("results"), list):
            out.append((ptr, v, None))
        for k, x in v.items():
            before = len(out)
            find(x, f"{ptr}/{k}", out, depth + 1)
            if k == "source" and len(out) > before and out[before][0] == f"{ptr}/source":
                out[before] = (out[before][0], out[before][1], v.get("invocation"))
    else:
        for i, x in enumerate(v):
            find(x, f"{ptr}/{i}", out, depth + 1)


def probe(key, s, inv, rows):
    put = lambda f, v: rows.append(f"{key}\t{f}\t{v}")  # noqa: E731
    nq = [case.get("basis_ref") for case in (s.get("numerical_quality") or {}).get("cases", []) if isinstance(case, dict)] if isinstance(s.get("numerical_quality"), dict) else []
    put("id", (s.get("producer") or {}).get("semantic_contract_id", "") if isinstance(s.get("producer"), dict) else "")
    put("raw", safe(lambda: list(c._source_contract(deepcopy(s)))[:2]))
    put("transport", safe(lambda: list(c._source_contract(deepcopy(s), check_receipt=False))[:2]))
    put("standing_nq", safe(lambda: c.numerical_use_standing(deepcopy(s), nq)))
    put("standing_empty", safe(lambda: c.numerical_use_standing(deepcopy(s), [])))
    if inv is not None:
        put("standing_inv", safe(lambda: c.numerical_use_standing(deepcopy(s), nq, inv)))
    put("standing_reason", safe(lambda: c.standing_reason(deepcopy(s))))
    put("fresh", safe(lambda: c.is_fresh_contract_id((s.get("producer") or {}).get("semantic_contract_id", "")) if isinstance(s.get("producer"), dict) else None))
    put("binding", safe(lambda: [(row.get("id"), c.rule_binding_refusal(s, row)) for row in s.get("results", []) if isinstance(row, dict)]))
    def v03():
        rec = c.build_analysis_run(deepcopy(s), **KW)
        c.validate_analysis_run_v0_3(rec, deepcopy(s)) if rec.get("schema_version") == "0.3.0" else None
        return [rec.get("schema_version"), h(rec)]
    put("analysis_run", safe(v03))
    put("analysis_run_v0_2", safe(lambda: h(c.build_analysis_run_v0_2(deepcopy(s), **KW))))
    put("wrapper_0_1_0", safe(lambda: h(r.build_preview_analysis_run_envelope(deepcopy(s)))))


rows, n = [], 0
for path in sorted(P.rglob("*.json")):
    if any(part in ("execution", "node_modules", "target", ".venv") for part in path.parts):
        continue
    try:
        doc = json.loads(path.read_text())
    except Exception:  # noqa: BLE001
        continue
    found = []
    find(doc, "", found)
    for ptr, env, inv in found:
        n += 1
        key = f"{path.relative_to(P)}#{ptr}"
        probe(key, env, inv, rows)
        sid = (env.get("producer") or {}).get("semantic_contract_id") if isinstance(env.get("producer"), dict) else None
        if sid == SUCC:
            continue
        forms = {
            "receipt": lambda s: s.__setitem__("retained_precision", deepcopy(RECEIPT)),
            "null": lambda s: s.__setitem__("retained_precision", None),
            "token0": lambda s: s["results"][0].__setitem__("recovery_method", METHOD) if s["results"] else None,
            "tokenlast": lambda s: s["results"][-1].__setitem__("recovery_method", METHOD) if s["results"] else None,
            "othertoken": lambda s: s["results"][0].__setitem__("recovery_method", "rv88_other_method") if s["results"] else None,
            "r2notice": lambda s: s.setdefault("diagnostics", []).append({"id": "diagnostic:retained-precision:case:unavailable", "code": "RETAINED_PRECISION_UNAVAILABLE", "severity": "info", "message": NOTICE, "source": "core/product_physics", "affected_refs": ["case"]}) if isinstance(s.get("diagnostics"), list) else None,
        }
        for form, edit in forms.items():
            s = deepcopy(env)
            edit(s)
            probe(f"{key}!{form}", s, inv, rows)
rows.append(f"#envelopes\t{n}")
OUT.write_text("\n".join(rows) + "\n")
print(n, "envelopes")
