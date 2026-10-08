"""RV113 (RV-R): the repair-01 probe table across PY (I1, head), RS (b5cb7faaeb) and TS (7e47e51b5d), bound and unbound.
Usage: fg_table.py <dir with probes_rp.json, py_i1_rp.jsonl, py_head_rp.jsonl, rs_rp.jsonl, ts_rp.jsonl> <out.json>"""
import json
import sys

d, out = sys.argv[1], sys.argv[2]
probes = json.load(open(f"{d}/probes_rp.json"))
L = {k: {json.loads(l)["id"]: json.loads(l) for l in open(f"{d}/{k}_rp.jsonl")} for k in ("py_i1", "py_head", "rs", "ts")}


def short(v):
    if v is None:
        return None
    if "ok" in v:
        return "admitted, eligible" if v["ok"]["numerical_eligible"] else "admitted, needs_recompute"
    if "err" in v:
        return f'{v["err"]["gate"]} {v["err"]["code"].replace("RETAINED_PRECISION_", "")}'
    return f"escape {v}"


rows = []
for p in probes:
    r = {"id": p["id"], "item": p["item"], "note": p["note"]}
    for k in L:
        line = L[k][p["id"]]
        r[k] = {m: (short(line.get(m)) if "materialize_error" not in line else "materialize: " + line["materialize_error"]) for m in ("bound", "unbound")}
        r[k]["input_sha256"] = line.get("input_sha256")
    # Each harness digests its own serialization, so input digests compare within a language only (PY at I1 and the
    # head). Across languages, an admitted row's publication digest is the shared identity of the materialized input.
    r["same_input"] = r["py_i1"]["input_sha256"] == r["py_head"]["input_sha256"]
    pubs = {k: L[k][p["id"]].get(m, {}).get("ok", {}).get("publication_sha256") for k in L for m in ("bound", "unbound")}
    r["publication_sha256_admitted"] = sorted({v for v in pubs.values() if v})
    rows.append(r)
json.dump(rows, open(out, "w"), indent=1)
for r in rows:
    print(f'{r["id"]:34} py_same_input={r["same_input"]!s:5} publication digests among admitted rows={len(r["publication_sha256_admitted"])}')
    for m in ("bound", "unbound"):
        print(f'   {m:8} PY@I1={r["py_i1"][m]!s:28} PY={r["py_head"][m]!s:28} RS={r["rs"][m]!s:28} TS={r["ts"][m]!s:28}')
