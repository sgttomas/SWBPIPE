"""I91 SR-PY cascade census (R5): every 07m entry through one Python reader.

Usage: census.py <P root> <corpus path> <out.json>
Entries: the 17 bases (with and without their invocation), the 294 mutations and the 28 must-pass
entries, each built with the contract test's own apply_entry semantics (rehash "all") using the
reader under test for hashing. Each is read through three entry points: raw with the entry's
invocation, raw without an invocation, and the transport validator. An outcome is
("pass", sha256 of the key-sorted result) or ("refuse", gate, code, detail).
"""
import hashlib, json, math, sys
from copy import deepcopy
from pathlib import Path

PROOT, CORPUS, OUT = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
sys.path.insert(0, str(PROOT))
from core.analysis_runs import retained_precision as rp  # noqa: E402


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


def outcome(fn, *args):
    try:
        result = fn(*deepcopy(args))
        return ["pass", hashlib.sha256(json.dumps(result, sort_keys=True).encode()).hexdigest()]
    except rp.RetainedPrecisionError as error:
        return ["refuse", error.gate, error.code, error.detail]
    except Exception as error:  # an escape is itself an outcome to compare
        return ["escape", type(error).__name__, str(error)[:200]]


def main():
    data = json.loads(CORPUS.read_text())
    bases = {f["id"]: f for f in data["cases"]}
    rows = {}
    for fixture in data["cases"]:
        entries = [("base:" + fixture["id"], deepcopy(fixture["source"]), deepcopy(fixture["invocation"]))]
        for label, source, invocation in entries:
            rows[label] = {"raw": outcome(rp.validate_retained_precision, source, invocation),
                           "raw_noinv": outcome(rp.validate_retained_precision, source, None),
                           "transport": outcome(rp.validate_retained_precision_transport, source)}
    for kind in ("mutations", "must_pass"):
        for entry in data.get(kind, []):
            source, invocation = apply_entry(bases[entry["base"]], entry)
            label = kind + ":" + entry["id"]
            assert label not in rows, label
            rows[label] = {"raw": outcome(rp.validate_retained_precision, source, invocation),
                           "raw_noinv": outcome(rp.validate_retained_precision, source, None),
                           "transport": outcome(rp.validate_retained_precision_transport, source)}
    counts = {"bases": len(data["cases"]), "mutations": len(data["mutations"]), "must_pass": len(data.get("must_pass", []))}
    OUT.write_text(json.dumps({"counts": counts, "corpus_sha256": hashlib.sha256(CORPUS.read_bytes()).hexdigest(), "rows": rows}, indent=1, sort_keys=True))
    print(counts, len(rows))


main()
