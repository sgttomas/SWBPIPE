"""RV99 dense pass: every decided case of pass 1 (T, F, or a finite quantity
enclosure; runner CHECKED/FAILED with b > 0) is re-sampled densely over its
input enclosure (evenly spaced binary64 values by bit order, plus every value
within 32 ulps of each end, of q, and of each literal and table argument)."""
from __future__ import annotations

import json
import math
import struct
import sys

sys.path.insert(0, __import__("os").path.dirname(__file__))
from gen_rv99 import okey, ofromkey  # noqa: E402
from rv99_oracle import my_enclosure  # noqa: E402

cases_path, out_path, dense_path = sys.argv[1:4]


def f(h):
    return struct.unpack(">d", int(h, 16).to_bytes(8, "big"))[0]


def hx(x):
    return "0x" + struct.pack(">d", x).hex()


def nums(node, out):
    if isinstance(node, dict):
        for k, v in node.items():
            if k in ("value", "argument", "result") and isinstance(v, str) and v.startswith("0x"):
                out.append(f(v))
            else:
                nums(v, out)
    elif isinstance(node, list):
        for v in node:
            nums(v, out)
    return out


def dense(enc, anchors, n):
    lo, hi = enc
    klo, khi = okey(lo), okey(hi)
    keys = set()
    span = khi - klo
    if span + 1 <= n:
        keys.update(range(klo, khi + 1))
    else:
        for i in range(n):
            keys.add(klo + (span * i) // (n - 1))
    for a in [lo, hi] + anchors:
        if math.isfinite(a) and lo <= a <= hi:
            ka = okey(a)
            keys.update(k for k in range(ka - 32, ka + 33) if klo <= k <= khi)
    return [ofromkey(k) for k in sorted(keys)]


cases = json.load(open(cases_path))
out = json.load(open(out_path))
ev, rn = [], []
for case, res in zip(cases["eval"], out["eval"]):
    iv = res["interval"]
    if iv.get("panic") or iv["findings"] or iv["value"] is None:
        continue
    v = iv["value"]
    decided = (v["kind"] == "truth" and v["truth"] in "TF") or \
              (v["kind"] == "quantity" and v["enclosure"] is not None)
    if not decided:
        continue
    anchors = nums(case["formula"], [])
    encs = []
    for inp in case["inputs"]:
        q, b = f(inp["value"]), f(inp["bound"])
        encs.append(my_enclosure(q, b) or (q, q))
    k = len(encs)
    per = {1: 4096, 2: 64, 3: 16}.get(k, 8)
    lists = [dense(e, anchors + [f(i["value"])], per) for e, i in zip(encs, case["inputs"])]
    tuples = [[]]
    for lst in lists:
        tuples = [t + [x] for t in tuples for x in lst]
    if len(tuples) > 20000:
        step = len(tuples) // 20000 + 1
        tuples = tuples[::step]
    c = dict(case)
    c["id"] = case["id"]
    c["samples"] = [[hx(x) for x in t] for t in tuples]
    c["bisect"] = False
    ev.append(c)
for case, res in zip(cases["runner"], out["runner"]):
    bd = res["bounded"]
    if bd.get("panic") or bd["status"] == "RULE_INPUTS_INCOMPLETE" or f(case["bound"]) <= 0:
        continue
    q, b = f(case["actual"]), f(case["bound"])
    enc = my_enclosure(q, b)
    if enc is None:
        continue
    c = dict(case)
    c["samples"] = [hx(x) for x in dense(enc, [q], 4096)]
    rn.append(c)
json.dump({"eval": ev, "runner": rn}, open(dense_path, "w"))
print(len(ev), "dense eval cases;", sum(len(c["samples"]) for c in ev), "samples")
print(len(rn), "dense runner cases;", sum(len(c["samples"]) for c in rn), "samples")
