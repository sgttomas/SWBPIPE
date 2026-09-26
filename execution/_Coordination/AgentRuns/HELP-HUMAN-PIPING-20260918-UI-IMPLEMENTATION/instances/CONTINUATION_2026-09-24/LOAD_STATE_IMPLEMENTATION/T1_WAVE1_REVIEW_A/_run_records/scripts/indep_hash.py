"""Independent recomputation of the joined per-case physical-evidence hash.

Written by the reviewer from the producer source (source_receipt.rs `hash`,
composite.rs `case_physical_evidence`, source_receipt.rs row construction),
without importing any project reader code. Implements RFC 8785 JCS itself.
"""
import hashlib
import json
import sys
from decimal import Decimal


def es_number(x):
    x = float(x)
    if x != x or x in (float("inf"), float("-inf")):
        raise ValueError("nonfinite")
    if x == 0:
        return "0"
    sign = "-" if x < 0 else ""
    x = abs(x)
    d = Decimal(repr(x))
    t = d.as_tuple()
    digits = "".join(map(str, t.digits)).rstrip("0") or "0"
    # value = 0.digits * 10**n
    n = len(t.digits) + t.exponent
    k = len(digits)
    if k <= n <= 21:
        s = digits + "0" * (n - k)
    elif 0 < n <= 21:
        s = digits[:n] + "." + digits[n:]
    elif -6 < n <= 0:
        s = "0." + "0" * (-n) + digits
    else:
        e = n - 1
        m = digits[0] + ("." + digits[1:] if k > 1 else "")
        s = m + "e" + ("+" if e >= 0 else "-") + str(abs(e))
    return sign + s


def es_string(s):
    out = ['"']
    for ch in s:
        o = ord(ch)
        if ch == '"':
            out.append('\\"')
        elif ch == "\\":
            out.append("\\\\")
        elif ch == "\b":
            out.append("\\b")
        elif ch == "\f":
            out.append("\\f")
        elif ch == "\n":
            out.append("\\n")
        elif ch == "\r":
            out.append("\\r")
        elif ch == "\t":
            out.append("\\t")
        elif o < 0x20:
            out.append("\\u%04x" % o)
        else:
            out.append(ch)
    out.append('"')
    return "".join(out)


def jcs(v):
    if v is None:
        return "null"
    if v is True:
        return "true"
    if v is False:
        return "false"
    if isinstance(v, (int, float)):
        if isinstance(v, int) and abs(v) > 2**53 - 1:
            raise ValueError("integer magnitude")
        return es_number(v)
    if isinstance(v, str):
        return es_string(v)
    if isinstance(v, list):
        return "[" + ",".join(jcs(x) for x in v) + "]"
    if isinstance(v, dict):
        keys = sorted(v, key=lambda k: k.encode("utf-16-be"))
        return "{" + ",".join(es_string(k) + ":" + jcs(v[k]) for k in keys) + "}"
    raise TypeError(type(v))


def dhash(domain, payload):
    return hashlib.sha256(jcs({"domain": domain, "payload": payload}).encode("utf-8")).hexdigest()


def self_test(table_path):
    table = json.load(open(table_path))
    for vec in table["hash_vectors"]:
        c = jcs(vec["input"])
        assert c == vec["expected_canonical_json"], (c, vec["expected_canonical_json"])
        assert hashlib.sha256(c.encode()).hexdigest() == vec["expected_sha256"]
    print("JCS self-test: %d table vectors reproduced" % len(table["hash_vectors"]))


def check(path):
    src = json.load(open(path))
    body = src["source_block_recovery"]["body"]
    ev = src["contract_evidence"]
    n_ok = 0
    for case, exact, record in zip(body["cases"], ev["exact_cases"], ev["load_reference_states"]):
        # Producer: exact (selected) case proof uses pressure = [] (composite.rs
        # line ~937); ordinary case uses its own pressure slice.
        if case["selected_method"] == "retained_source_blocks_exact_v1":
            pressure = []
        else:
            pressure = [p for p in ev["pressure"] if p["load_case_id"] == exact["load_case_id"]]
        proof = {"exact_case": exact, "pressure": pressure, "load_reference_state": record}
        got = dhash("load_reference_source_case_evidence_v1", proof)
        want = case["physical_evidence_sha256"]
        # Controls: wrong domain, and record omitted, must NOT match.
        wrong_domain = dhash("physics_source_case_evidence_v1", proof)
        no_record = dhash("load_reference_source_case_evidence_v1", {"exact_case": exact, "pressure": pressure})
        assert got == want, (path, exact["load_case_id"], got, want)
        assert wrong_domain != want and no_record != want
        n_ok += 1
    # receipt and publication hashes too
    assert src["source_block_recovery"]["receipt_sha256"] == dhash("source_blocks_receipt_v1", body)
    pub = {k: v for k, v in src.items() if k != "source_block_recovery"}
    assert body["publication_sha256"] == dhash("source_blocks_publication_v1", pub)
    return n_ok


if __name__ == "__main__":
    root = sys.argv[1]
    self_test(root + "/fixtures/results/semantic_contract_v0_3_load_reference_source_1.json")
    import glob
    total = 0
    for p in sorted(glob.glob(root + "/fixtures/product_preview/load_reference_source/*.raw.json")):
        n = check(p)
        total += n
        print("OK %-45s cases=%d (physical, receipt, publication hashes reproduced)" % (p.split("/")[-1], n))
    print("total cases", total)
