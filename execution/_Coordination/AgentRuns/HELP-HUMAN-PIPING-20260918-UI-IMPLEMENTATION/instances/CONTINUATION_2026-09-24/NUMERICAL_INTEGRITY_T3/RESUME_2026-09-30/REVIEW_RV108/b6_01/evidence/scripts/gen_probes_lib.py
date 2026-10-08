import math
from copy import deepcopy
from core.analysis_runs import retained_precision as rp
def idx(items, ref):
    if type(ref) not in (int, float) or not math.isfinite(ref) or ref != int(ref) or ref < 0 or (ref == 0 and math.copysign(1.0, ref) < 0): return None
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
def setp(v, path, val):
    for p in path[:-1]: v = v[p]
    v[path[-1]] = deepcopy(val)
def delp(v, path):
    for p in path[:-1]: v = v[p]
    del v[path[-1]]
