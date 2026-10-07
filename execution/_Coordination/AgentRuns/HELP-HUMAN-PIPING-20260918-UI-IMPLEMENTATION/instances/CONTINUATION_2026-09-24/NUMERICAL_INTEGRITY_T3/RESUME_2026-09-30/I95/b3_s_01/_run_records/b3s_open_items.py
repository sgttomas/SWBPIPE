"""I95 B3-S: the open items of the exact-route TEXT runs (stdlib only): every run has no unmapped loop
header, and its only unclassified entries are id-unaudited (identifier placeholders on newly reached sites
that I65's audit table has no entry for; each is priced at its argument rule's class value).
Usage: python3 b3s_open_items.py <runs dir> <out json>"""
import json, os, sys
runs, out = sys.argv[1:3]
res = {}
for ch in ("ur", "urc", "er", "erc"):
    for c in (1, 2, 3):
        for v, n in (("", "full"), ("_W", "W"), ("_X", "X"), ("_env", "env")):
            d = json.load(open(os.path.join(runs, ch, f"c{c}", "work", f"text_budget{v}.caps.out.json")))
            assert not d["unmapped_loop_headers"], (ch, c, n)
            assert all(kind == "id-unaudited" for (kind, _), _n in d["unclassified_args"]), (ch, c, n)
            res[f"{ch}/c{c}/{n}"] = {"complete": d["complete"], "unmapped_loop_headers": 0,
                                     "id_unaudited": [[text.split("/src/", 1)[-1], k] for (kind, text), k in d["unclassified_args"]]}
json.dump(res, open(out, "w"), indent=1)
print({k: len(v["id_unaudited"]) for k, v in res.items() if k.endswith("c3/full") or k.endswith("c3/W")})
for t, k in res["erc/c3/full"]["id_unaudited"]:
    print(" ", k, t)
