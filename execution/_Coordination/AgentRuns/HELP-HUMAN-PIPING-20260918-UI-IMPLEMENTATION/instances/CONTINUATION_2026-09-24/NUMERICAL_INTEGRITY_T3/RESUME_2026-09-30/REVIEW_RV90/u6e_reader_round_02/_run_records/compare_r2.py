import json, sys
L = sys.argv[1]; langs = sys.argv[2].split(",")
O = lambda n: json.load(open(f"{L}/out_{n}.json"))
exp = {}
for c in ("07g", "07h"):
    for line in open(f"{L}/applied_{c}.jsonl"):
        d = json.loads(line); exp.setdefault(c, {})[d["kind"] + ":" + d["id"]] = (d["expected"], d.get("expected_by_reader"))
norm = lambda x: tuple(x) if isinstance(x, list) else ("pass", tuple(x["classes"]), x["eligible"])
lang = {"py": "python", "rs": "rust", "ts": "typescript"}
for r in langs:
    g_reader = "g" if r == "py" else "h"   # Rust and TS reader code is byte-identical between 5e1e2625ac and cc4dd61d67
    a, b = O(f"{r}_{g_reader}_07g"), O(f"{r}_h_07h")
    print(f"[{r}] 07g reader on 07g vs 07h reader on 07h: changed 07g entries:", [k for k in a if norm(a[k]) != norm(b[k])], "| new:", {k: b[k] for k in b if k not in a})
    if r == "py":
        a2, b2 = O("py_g_07g"), O("py_h_07g"); print(f"[{r}] both readers on 07g: diffs", [k for k in a2 if norm(a2[k]) != norm(b2[k])])
        a3, b3 = O("py_g_07h"), O("py_h_07h"); print(f"[{r}] both readers on 07h: diffs", {k: (a3[k], b3[k]) for k in a3 if norm(a3[k]) != norm(b3[k])})
    miss = []
    for k, v in b.items():
        e, ebr = exp["07h"][k]
        if ebr and lang[r] in ebr: e = ebr[lang[r]]
        if k.startswith("cases:") or e == "pass":
            if not (isinstance(v, dict) and v.get("pass")): miss.append((k, v))
        elif not (isinstance(v, list) and v == [e["gate"], e["code"]]): miss.append((k, v, e))
    print(f"[{r}] 07h reader on 07h: entries {len(b)}, misses vs expected:", miss, "| eligible any:", any(isinstance(v, dict) and v.get("eligible") for v in b.values()))
    mp = b.get("must_pass:f5_envelope_reordered_exact_list"); print(f"[{r}] must-pass control:", mp)
if len(langs) == 3:
    P = {r: O(f"{r}_h_07h") for r in langs}
    print("parity 07h:", {k: [P[r][k] for r in langs] for k in P["py"] if len({json.dumps(norm(P[r][k])) for r in langs}) > 1})
