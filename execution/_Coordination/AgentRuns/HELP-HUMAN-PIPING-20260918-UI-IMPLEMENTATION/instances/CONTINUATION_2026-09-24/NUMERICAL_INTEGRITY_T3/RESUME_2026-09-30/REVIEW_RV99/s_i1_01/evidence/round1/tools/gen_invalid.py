"""RV99 round 1: invalid and explicit overlays on RV99's own pass-1 cases.
For 240 of the pass-1 random cases, input 0 gets one of: a NaN, negative,
-inf or +inf bound; an explicit valid enclosure (RV99's own outward box of
q +/- b, so the samples lie inside); an explicit null; an inverted pair; a
NaN end; an infinite end. Rust's evaluate_interval and the Python reference
must agree on all of them, and decided outcomes must stay sound."""
import json, math, random, struct, sys
sys.path.insert(0, sys.argv[3])
from rv99_oracle import my_enclosure
R = random.Random(4242)
hx = lambda x: "0x" + struct.pack(">d", x).hex()
f = lambda h: struct.unpack(">d", int(h, 16).to_bytes(8, "big"))[0]
cases = json.load(open(sys.argv[1]))["eval"]
rand = [c for c in cases if c["id"].startswith("rand_")][:240]
out = []
kinds = ["nan_bound", "neg_bound", "neginf_bound", "inf_bound", "explicit_valid", "explicit_null",
         "inverted", "nan_end", "inf_end", "explicit_valid"]
for k, c in enumerate(rand):
    c = json.loads(json.dumps(c))
    kind = kinds[k % len(kinds)]
    inp = c["inputs"][0]
    q, b = f(inp["value"]), f(inp["bound"])
    if kind.endswith("_bound"):
        inp["bound"] = hx({"nan_bound": float("nan"), "neg_bound": -(b or 1e-9), "neginf_bound": -math.inf,
                           "inf_bound": math.inf}[kind])
    else:
        e = my_enclosure(q, b if b > 0 else 1e-9) or (q, q)
        inp["enclosure"] = {"explicit_valid": [hx(e[0]), hx(e[1])], "explicit_null": None,
                            "inverted": [hx(e[1] + 1.0), hx(e[0] - 1.0)] if e[0] == e[1] else [hx(e[1]), hx(e[0])],
                            "nan_end": [hx(float("nan")), hx(e[1])], "inf_end": [hx(e[0]), hx(math.inf)]}[kind]
    c["id"] = f"inv_{kind}_{c['id']}"
    c["bisect"] = False
    out.append(c)
json.dump({"eval": out, "runner": []}, open(sys.argv[2], "w"))
print(len(out), "invalid/explicit overlay cases")
