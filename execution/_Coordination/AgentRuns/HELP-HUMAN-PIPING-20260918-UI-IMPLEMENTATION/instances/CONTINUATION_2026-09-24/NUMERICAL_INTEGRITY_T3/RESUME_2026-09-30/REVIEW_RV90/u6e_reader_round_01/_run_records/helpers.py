"""RV90 own outcome dumper: own entry application and rehash (07e format rule), any reader root, any corpus."""
import sys, json, math, hashlib
from copy import deepcopy
root = __import__("os").environ["RV90_READER_ROOT"]
sys.path.insert(0, root)
from core.analysis_runs import retained_precision as rp
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1 as H
def dh(domain, payload): return H({"domain": domain, "payload": payload})
def index(v):
    if type(v) not in (int, float): return None
    if not math.isfinite(v) or v < 0 or v != int(v) or (v == 0 and math.copysign(1, v) < 0): return None
    return int(v)
def edit(doc, e):
    path = e["path"]; parent = doc
    for p in path[:-1]:
        i = index(p); parent = parent[i] if i is not None else parent[p]
    last = path[-1]; i = index(last)
    if e["op"] == "remove":
        if i is not None: del parent[i]
        else: del parent[last]
    else:
        assert e["op"] == "set", e
        if i is not None: parent[i] = deepcopy(e["value"])
        else: parent[last] = deepcopy(e["value"])
def rehash(src):
    rpv = src.get("retained_precision")
    if not isinstance(rpv, dict) or not isinstance(rpv.get("body"), dict): return
    b = rpv["body"]; attempts = b["product_attempts"]
    for s in b["sources"]:
        ai = index(s["preparation"]["attempt_ref"]) if isinstance(s.get("preparation"), dict) else None
        if ai is None or ai >= len(attempts) or not isinstance(attempts[ai], dict): continue
        a = attempts[ai]
        if all(m["result"]["kind"] == "prepared" for m in a["preparation"]["members"]):
            members = [{"member": m["member"], "old_source": m["old_source"], "old_facts": m["old_facts"], "section": m["result"]["section"]} for m in a["preparation"]["members"]]
            s["preparation"]["sha256"] = dh("retained_precision_preparation_v1", {"definition_id": a["definition_id"], "definition_sha256": rp.DEFINITION_HASH, "owner_ref": a["owner_ref"], "ordinary_attempt_ref": a["ordinary_attempt_ref"], "material_basis_ref": a["material_basis_ref"], "members": members})
    for c in b["cases"]:
        if c["status"] == "selected":
            si = index(c.get("source_ref"))
            if si is None or si >= len(b["sources"]) or not isinstance(b["sources"][si], dict): continue
            s = deepcopy(b["sources"][si]); s.pop("index", None)
            c["source_identity_sha256"] = dh("retained_precision_source_mp_v2", s)
    public = {k: v for k, v in src.items() if k != "retained_precision"}
    b["publication_sha256"] = dh("retained_precision_publication_mp_v2", public)
    rpv["receipt_sha256"] = dh("retained_precision_receipt_mp_v2", b)
def apply(case, entry):
    src = deepcopy(case["source"]); inv = deepcopy(case["invocation"])
    for e in entry["edits"]: edit(src, e)
    ie = entry.get("invocation_edits") or []
    for e in ie: edit(inv, e)
    if ie: src["retained_precision"]["body"]["invocation"]["value"] = dh("source_blocks_invocation_v1", inv)
    assert entry["rehash"] == "all"
    rehash(src)
    for e in entry.get("after_rehash") or []: edit(src, e)
    return src, inv
def outcome(src, inv):
    try:
        r = rp.validate_retained_precision(src, inv)
        return {"pass": True, "eligible": r["numerical_eligible"], "standing": r["standing"], "classes_sha256": hashlib.sha256(json.dumps(r["classifications"], sort_keys=True).encode()).hexdigest()}
    except rp.RetainedPrecisionError as e:
        return [e.gate, e.code]
