import math
from copy import deepcopy
from core.analysis_runs import retained_precision as rp
def _apply_edits(value, edits):
    for edit in edits:
        parent = value
        for part in edit["path"][:-1]:
            parent = parent[part]
        key = edit["path"][-1]
        if edit["op"] == "remove":
            del parent[key]
        else:
            parent[key] = deepcopy(edit["value"])


def _rehash_ref(items, ref):
    if type(ref) not in (int, float) or not math.isfinite(ref) or ref != int(ref) or ref < 0 or (ref == 0 and math.copysign(1.0, ref) < 0):
        return None
    return items[int(ref)] if int(ref) < len(items) else None


def apply_mutation(base, mutation):
    value = deepcopy(base)
    _apply_edits(value, mutation["edits"])
    if mutation.get("_invocation_digest") is not None:
        value["retained_precision"]["body"]["invocation"]["value"] = mutation["_invocation_digest"]
    assert mutation["rehash"] == "all"
    receipt = value.get("retained_precision")
    if isinstance(receipt, dict) and isinstance(receipt.get("body"), dict):
        body = receipt["body"]
        for source in body["sources"]:
            preparation = source["preparation"]
            attempt = _rehash_ref(body["product_attempts"], preparation["attempt_ref"]) if preparation is not None else None
            if attempt is not None and all(m["result"]["kind"] == "prepared" for m in attempt["preparation"]["members"]):
                preparation["sha256"] = rp._hash("retained_precision_preparation_v1", rp._preparation_payload(attempt))
        for case in body["cases"]:
            source = _rehash_ref(body["sources"], case.get("source_ref")) if case["status"] == "selected" else None
            if source is not None:
                case["source_identity_sha256"] = rp._source_hash(source)
        body["publication_sha256"] = rp._hash("retained_precision_publication_mp_v2", {k: v for k, v in value.items() if k != "retained_precision"})
        receipt["receipt_sha256"] = rp._hash("retained_precision_receipt_mp_v2", body)
    _apply_edits(value, mutation.get("after_rehash") or [])
    return value


def apply_entry(fixture, entry):
    invocation = deepcopy(fixture["invocation"])
    invocation_edits = entry.get("invocation_edits") or []
    _apply_edits(invocation, invocation_edits)
    digest = rp._hash("source_blocks_invocation_v1", invocation) if invocation_edits else None
    return apply_mutation(fixture["source"], dict(entry, _invocation_digest=digest)), invocation


