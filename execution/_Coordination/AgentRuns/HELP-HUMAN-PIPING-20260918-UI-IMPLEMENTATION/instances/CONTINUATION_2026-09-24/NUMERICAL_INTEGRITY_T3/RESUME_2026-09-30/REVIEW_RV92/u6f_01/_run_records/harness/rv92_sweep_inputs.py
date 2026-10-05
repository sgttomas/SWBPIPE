"""RV92 (U6f): the existing-behaviour sweep inputs. Walks the BASE tree P (argv[1])
for every committed raw envelope (an identified producer with a results array,
or a legacy 0.1.0 envelope with status and results), every canonical results
document, AnalysisRun document and stress-neutral 0.3 package, and adds RV92's
injected downgrade forms on each raw envelope. Writes <out>/index.json and one
file per input (the same layout as the parity probes)."""
import copy, json, os, sys

P, OUT = sys.argv[1:3]
os.makedirs(OUT, exist_ok=True)
SKIP = {"execution", "node_modules", "target", ".venv", ".git", "dist", "public", "__pycache__"}
METHOD = "contribution_preserving_multiprecision_v1"
items = []


def add(kind, key, doc, invocation=None):
    items.append({"kind": kind, "id": key, "source": doc, "invocation": invocation})


def walk(x, ptr, depth, rel, parent):
    if depth > 9:
        return
    if isinstance(x, dict):
        prod = x.get("producer")
        identified = isinstance(prod, dict) and isinstance(prod.get("semantic_contract_id"), str) and isinstance(x.get("results"), list)
        legacy = x.get("schema_version") == "0.1.0" and isinstance(x.get("results"), list) and "status" in x
        if identified or legacy:
            inv = parent.get("invocation") if isinstance(parent, dict) and ptr.endswith("/source") else None
            add("raw", f"{rel}#{ptr}", x, inv if isinstance(inv, dict) else None)
        if isinstance(x.get("result_envelope"), dict) and isinstance(x.get("schema_version"), str):
            add("results_doc", f"{rel}#{ptr}", x)
        if isinstance(x.get("analysis_run"), dict) and isinstance(x.get("schema_version"), str):
            add("analysis_run", f"{rel}#{ptr}", x)
        if x.get("deliverable_id") == "DEL-17-06" and x.get("schema_version") == "0.3.0":
            add("stress_neutral", f"{rel}#{ptr}", x)
        for k, y in x.items():
            walk(y, f"{ptr}/{k}", depth + 1, rel, x)
    elif isinstance(x, list):
        for i, y in enumerate(x):
            walk(y, f"{ptr}/{i}", depth + 1, rel, x)


for root, dirs, files in os.walk(P):
    dirs[:] = sorted(d for d in dirs if d not in SKIP)
    for f in sorted(files):
        if not f.endswith(".json"):
            continue
        path = os.path.join(root, f)
        if os.path.getsize(path) > 40_000_000:
            continue
        try:
            v = json.load(open(path))
        except Exception:
            continue
        walk(v, "", 0, os.path.relpath(path, P), None)

raw = [i for i in items if i["kind"] == "raw"]
receipt = {"receipt_sha256": "0" * 64, "body": {}}
for i in raw:
    s = i["source"]
    rows = s["results"]
    forms = {"!receipt_obj": lambda d: d.__setitem__("retained_precision", copy.deepcopy(receipt)),
             "!receipt_null": lambda d: d.__setitem__("retained_precision", None)}
    if rows and all(isinstance(r, dict) for r in rows):
        forms["!token_first"] = lambda d: d["results"][0].__setitem__("recovery_method", METHOD)
        forms["!token_last"] = lambda d: d["results"][-1].__setitem__("recovery_method", METHOD)
        forms["!token_other"] = lambda d: d["results"][0].__setitem__("recovery_method", "other_method")
    for name, fn in forms.items():
        d = copy.deepcopy(s)
        fn(d)
        items.append({"kind": "raw_injected", "id": i["id"] + name, "source": d, "invocation": i["invocation"]})

index = []
for n, it in enumerate(items):
    src = it["source"]
    nq = (src.get("numerical_quality") or {}).get("cases") if isinstance(src.get("numerical_quality"), dict) else None
    req = [c.get("basis_ref") for c in nq if isinstance(c, dict)] if isinstance(nq, list) else []
    probe = {"id": it["id"], "group": it["kind"], "source": src, "invocation": it["invocation"], "requested": req,
             "load_cases": [r.get("ref_id") for r in req if isinstance(r, dict)]}
    fn = f"s{n:05d}.json"
    json.dump(probe, open(os.path.join(OUT, fn), "w"), separators=(",", ":"))
    index.append({"file": fn, "id": it["id"], "group": it["kind"]})
json.dump(index, open(os.path.join(OUT, "index.json"), "w"), indent=0)
import collections
print(collections.Counter(i["kind"] for i in items))
