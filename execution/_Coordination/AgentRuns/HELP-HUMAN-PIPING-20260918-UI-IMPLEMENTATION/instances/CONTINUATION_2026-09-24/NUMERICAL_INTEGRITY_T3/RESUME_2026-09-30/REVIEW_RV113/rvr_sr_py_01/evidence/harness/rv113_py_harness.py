"""RV113 (RV-R) reviewer harness for the Python reader. Not part of any candidate; run against an archive copy.

It materializes every shared corpus entry (and, optionally, reviewer probes) from the snapshot format rule, written
here independently (SHARED_SNAPSHOT_06C `format_change`; SHARED_SNAPSHOT_07E `format_rule`), and records each reader
verdict as one JSON line: bound, unbound and transport.

Usage: python rv113_py_harness.py <P root> census <out.jsonl>
       python rv113_py_harness.py <P root> probes <probes.json> <out.jsonl>
The checked-JSON, binary64 and units authorities are taken from the environment
(OPENPIPESTRESS_CHECKED_JSON_BIN, OPENPIPESTRESS_BINARY64_JSON_BIN, OPENPIPESTRESS_UNITS_BIN).
"""
import copy
import hashlib
import json
import sys

root = sys.argv[1]
sys.path.insert(0, root)
from core.analysis_runs.retained_precision import (  # noqa: E402
    RetainedPrecisionError, validate_retained_precision, validate_retained_precision_transport)
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1  # noqa: E402

DEFINITION_SHA256 = "a7ed7ca0bf0bba6e8b821ca4befa00a0fa9541a83694be8b28ac63e39b1d0349"
corpus = json.load(open(f"{root}/fixtures/results/retained_precision_cases.json"))


def strict_index(v):
    if type(v) is bool or not isinstance(v, (int, float)):
        return None
    if isinstance(v, float) and (v != v or v in (float("inf"), float("-inf")) or not v.is_integer()):
        return None
    if v < 0 or (v == 0 and str(v).startswith("-")):
        return None
    return int(v)


def step(v, key):
    i = strict_index(key)
    if i is not None:
        if type(v) is not list or i >= len(v):
            raise ValueError(f"index {i}")
        return v[i]
    if type(key) is not str or type(v) is not dict:
        raise ValueError(f"key {key!r}")
    if key not in v:
        v[key] = {}
    return v[key]


def apply_edit(root_value, e):
    path = e["path"]
    at = root_value
    for p in path[:-1]:
        at = step(at, p)
    last = path[-1]
    i = strict_index(last)
    if e["op"] == "remove":
        if i is not None:
            if type(at) is not list or i >= len(at):
                raise ValueError("remove index")
            del at[i]
        else:
            if type(at) is not dict:
                raise ValueError("remove key")
            at.pop(last, None)
    elif e["op"] == "set":
        if i is not None:
            if type(at) is not list or i >= len(at):
                raise ValueError("set index")
            at[i] = copy.deepcopy(e["value"])
        else:
            if type(at) is not dict:
                raise ValueError("set key")
            at[last] = copy.deepcopy(e["value"])
    else:
        raise ValueError(f"op {e['op']}")


def h(domain, payload):
    return canonical_sha256_checked_v1({"domain": domain, "payload": payload})


def rehash_all(source):
    """07E order: preparation hashes; selected source identities; publication; receipt."""
    body = source.get("retained_precision", {}).get("body") if type(source.get("retained_precision")) is dict else None
    if type(body) is not dict:
        return
    attempts = body.get("product_attempts") if type(body.get("product_attempts")) is list else []
    for s in body.get("sources") if type(body.get("sources")) is list else []:
        ai = strict_index((s.get("preparation") or {}).get("attempt_ref")) if type(s) is dict and type(s.get("preparation")) is dict else None
        if ai is None or ai >= len(attempts) or type(attempts[ai]) is not dict:
            continue
        a = attempts[ai]
        members = a["preparation"]["members"]
        if not all(m["result"]["kind"] == "prepared" for m in members):
            continue
        s["preparation"]["sha256"] = h("retained_precision_preparation_v1", {
            "definition_id": a["definition_id"], "definition_sha256": DEFINITION_SHA256, "owner_ref": a["owner_ref"],
            "ordinary_attempt_ref": a["ordinary_attempt_ref"], "material_basis_ref": a["material_basis_ref"],
            "members": [{"member": m["member"], "old_source": m["old_source"], "old_facts": m["old_facts"], "section": m["result"]["section"]} for m in members]})
    sources = body.get("sources") if type(body.get("sources")) is list else []
    for c in body.get("cases") if type(body.get("cases")) is list else []:
        if c.get("status") != "selected":
            continue
        si = strict_index(c.get("source_ref"))
        if si is None or si >= len(sources) or type(sources[si]) is not dict:
            continue
        s = copy.deepcopy(sources[si]); s.pop("index", None)
        c["source_identity_sha256"] = h("retained_precision_source_mp_v2", s)
    public = {k: v for k, v in source.items() if k != "retained_precision"}
    body["publication_sha256"] = h("retained_precision_publication_mp_v2", public)
    source["retained_precision"]["receipt_sha256"] = h("retained_precision_receipt_mp_v2", body)


def materialize(entry):
    base = next(c for c in corpus["cases"] if c["id"] == entry["base"])
    source, invocation = copy.deepcopy(base["source"]), copy.deepcopy(base["invocation"])
    for e in entry.get("edits") or []:
        apply_edit(source, e)
    inv_edits = entry.get("invocation_edits") or []
    for e in inv_edits:
        apply_edit(invocation, e)
    if inv_edits:
        source["retained_precision"]["body"]["invocation"]["value"] = h("source_blocks_invocation_v1", invocation)
    if entry["rehash"] != "all":
        raise ValueError("rehash is not all")
    rehash_all(source)
    for e in entry.get("after_rehash") or []:
        apply_edit(source, e)
    return source, invocation


def verdict(run):
    try:
        v = run()
        cls = json.dumps(v["classifications"], sort_keys=True, default=str)
        return {"ok": {"invocation_bound": v["invocation_bound"], "numerical_eligible": v["numerical_eligible"], "standing": v["standing"],
                       "publication_sha256": v["publication_sha256"], "classifications": len(v["classifications"]),
                       "classifications_sha256": hashlib.sha256(cls.encode()).hexdigest()}}
    except RetainedPrecisionError as e:
        return {"err": {"gate": e.gate, "code": e.code, "detail": getattr(e, "detail", None)}}
    except Exception as e:  # noqa: BLE001 -- an escape from the reader is itself a verdict
        return {"escape": f"{type(e).__name__}: {e}"}


def evaluate(kind, i, entry, raw=False):
    try:
        source, invocation = (copy.deepcopy(entry["source"]), copy.deepcopy(entry["invocation"])) if raw else materialize(entry)
    except Exception as e:  # noqa: BLE001
        return {"set": kind, "i": i, "id": entry["id"], "materialize_error": f"{type(e).__name__}: {e}"}
    text = json.dumps([source, invocation], sort_keys=True, separators=(",", ":"))
    return {"set": kind, "i": i, "id": entry["id"], "input_sha256": hashlib.sha256(text.encode()).hexdigest(),
            "bound": verdict(lambda: validate_retained_precision(copy.deepcopy(source), copy.deepcopy(invocation))),
            "unbound": verdict(lambda: validate_retained_precision(copy.deepcopy(source))),
            "transport": verdict(lambda: validate_retained_precision_transport(copy.deepcopy(source)))}


def main():
    mode = sys.argv[2]
    lines = []
    if mode == "census":
        out = sys.argv[3]
        for i, c in enumerate(corpus["cases"]):
            lines.append(evaluate("base", i, c, raw=True))
        for i, m in enumerate(corpus["mutations"]):
            lines.append(evaluate("mutation", i, m))
        for i, m in enumerate(corpus["must_pass"]):
            lines.append(evaluate("must_pass", i, m))
    else:
        probes, out = json.load(open(sys.argv[3])), sys.argv[4]
        for i, p in enumerate(probes):
            lines.append(evaluate("probe", i, p))
    with open(out, "w") as f:
        for line in lines:
            f.write(json.dumps(line, sort_keys=True) + "\n")
    print(f"{len(lines)} lines -> {out}")


main()
