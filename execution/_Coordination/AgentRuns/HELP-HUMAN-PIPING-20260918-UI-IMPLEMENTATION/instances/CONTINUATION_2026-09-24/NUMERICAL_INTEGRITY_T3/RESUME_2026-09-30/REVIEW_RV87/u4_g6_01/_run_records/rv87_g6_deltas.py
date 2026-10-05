"""RV87 (u4_g6_01): items 1 and 4. (1) RV87's 18 SF-1 sites (and RV89's 5, a subset) in the G6 run:
priced at the bound RV87 stated. (4) Row-by-row deltas part 2 -> G6 for the whole, W and X runs,
split into the 18 sites and the rest; and the in-build maximum recomputed from the profile trees and
the pinned records (forms x in-build atom values).
Usage: python3 rv87_g6_deltas.py <G6 _run_records> <part2 _run_records> <RV87 u4_g4_02 rv87_s2_confirm.out.json>
Stdlib only."""
import json, os, sys, collections, re
G, P2, S2 = sys.argv[1], sys.argv[2], sys.argv[3]
def runs(base, d):
    return {v: json.load(open(os.path.join(base, d, f"text_budget{v}.caps.out.json"))) for v in ("", "_W", "_X")}
g6 = runs(G, "text_g6")
p2 = {"": json.load(open(os.path.join(P2, "text_p2", "text_budget.caps.out.json")))}
s2 = json.load(open(S2))
def idx(rows):
    d = collections.defaultdict(list)
    for r in rows:
        d[(r["file"], r["line"], r["kind"])].append(r)
    return d
A, B = idx(p2[""]["rows"]), idx(g6[""]["rows"])
# item 1
item1 = []
for r in s2["remaining"]:
    f, line = r["site"].rsplit(":", 1)
    rowsB = B.get((f, int(line), r["kind"]), [])
    rowsA = A.get((f, int(line), r["kind"]), [])
    # the G6 row that changed from the part-2 row priced at r['priced']
    pairs = [(a, b) for a, b in zip(rowsA, rowsB)]
    cand = [b for a, b in pairs if a["bytes"] == r["priced"]] or rowsB
    got = max(b["bytes"] for b in cand)
    item1.append({"site": r["site"].split("core/")[-1], "class": r["class"], "rv87_bound": r["bound"], "g6_bytes": got,
                  "g6_req": max(b["req"] for b in cand), "rv87_req_at_bound": r["req_at_bound"], "ok": got >= r["bound"]})
# item 4: deltas
def diff(a, b):
    A_, B_ = idx(a["rows"]), idx(b["rows"])
    out = []
    for k in sorted(set(A_) | set(B_)):
        for x, y in zip(A_.get(k, []), B_.get(k, [])):
            if (x["mult"], x["bytes"], x["req"]) != (y["mult"], y["bytes"], y["req"]):
                out.append({"site": f"{k[0].split('core/')[-1]}:{k[1]}", "kind": k[2], "p2": (x["mult"], x["bytes"]), "g6": (y["mult"], y["bytes"]), "delta": y["req"] - x["req"]})
    return out
d_whole = diff(p2[""], g6[""])
mine = {(x["site"], ) for x in item1}
s18 = {x["site"] for x in item1}
in18 = [d for d in d_whole if d["site"] in s18]
rest = [d for d in d_whole if d["site"] not in s18]
res = {"item1": item1, "item1_all_ok": all(x["ok"] for x in item1),
       "whole": {"p2": p2[""]["total_text_requested_bytes"], "g6": g6[""]["total_text_requested_bytes"],
                 "delta": g6[""]["total_text_requested_bytes"] - p2[""]["total_text_requested_bytes"],
                 "rows_changed": len(d_whole), "lowered": [d for d in d_whole if d["delta"] < 0],
                 "delta_18": sum(d["delta"] for d in in18), "delta_rest": sum(d["delta"] for d in rest)},
       "rest_rows": rest,
       "W": g6["_W"]["total_text_requested_bytes"], "X": g6["_X"]["total_text_requested_bytes"]}
# in-build maximum from tree forms x record atoms
def record(path):
    atoms, phases = {}, {}
    for line in open(path):
        if line.startswith("I65_G5_ATOM"):
            head, name, val, ill = line.rstrip("\n").split("\t")
            atoms[name] = int(val)
        m = re.match(r"I65_G5_PHASE mode=(\w+) phase=(\w+) requested=(\d+) moving=(\d+) E_mov_plus_R=(\d+)", line)
        if m:
            phases[(m.group(1), m.group(2))] = (int(m.group(3)), int(m.group(4)), int(m.group(5)))
    return atoms, phases
def evaluate(tree, atoms):
    forms, exprs = tree["forms"], tree["exprs"]
    def ev_form(name):
        tot = 0
        for k, v in forms[name].items():
            if k == "1":
                tot += v
            elif k in tree["text_atoms"]:
                tot += v * tree["text_atoms"][k]
            else:
                tot += v * atoms[k]
        return tot
    def evn(node):
        if isinstance(node, str):
            return evn(exprs[node]) if node in exprs else ev_form(node)
        if "sum" in node:
            return sum(evn(c) for c in node["sum"])
        return max(evn(c) for c in node["max"])
    out = {}
    for mode, ph in tree["phases"].items():
        for name, spec in ph.items():
            rq = sum(evn(c) for c in spec["requested"]); mv = max(evn(c) for c in spec["moving"])
            out[(mode, name.split(" ")[0])] = (rq, mv, rq + mv + tree["R"])
    return out
for tag, tree_p, rec_p in (("part2", os.path.join(P2, "text_p2", "profile_tree.json"), os.path.join(P2, "profile_record_p2.txt")),
                           ("g6", os.path.join(G, "text_g6", "profile_tree.json"), os.path.join(G, "per_identity", "profile_record.test.txt"))):
    tree = json.load(open(tree_p)); atoms, phases = record(rec_p)
    ev = evaluate(tree, atoms)
    res[f"inbuild_{tag}"] = {f"{m}/{p}": {"rv87": v[2], "record": phases.get((m, p), (0, 0, 0))[2], "equal": v[2] == phases.get((m, p), (0, 0, 0))[2]}
                             for (m, p), v in ev.items()}
    res[f"atoms_{tag}"] = atoms
    res[f"tree_{tag}"] = tree
# decomposition of the W3 dense change: (a) the tree with part-2 atoms vs part-2 tree with part-2 atoms; (b) atoms
t2 = res.pop("tree_part2"); t6 = res.pop("tree_g6"); a2 = res.pop("atoms_part2"); a6 = res.pop("atoms_g6")
def w3(tree, atoms, mode="dense"):
    return evaluate(tree, atoms)[(mode, "W3")][2]
try:
    res["W3_dense_decomposition"] = {"part2": w3(t2, a2), "g6_tree_part2_atoms": w3(t6, a2), "g6": w3(t6, a6),
                                     "text_and_forms": w3(t6, a2) - w3(t2, a2), "atoms": w3(t6, a6) - w3(t6, a2)}
except KeyError as e:
    res["W3_dense_decomposition"] = {"missing_atom": str(e)}
res["forms_changed"] = sorted(k for k in set(t2["forms"]) | set(t6["forms"]) if t2["forms"].get(k) != t6["forms"].get(k))
res["atoms_changed"] = {k: [a2.get(k), a6.get(k)] for k in sorted(set(a2) | set(a6)) if a2.get(k) != a6.get(k)}
print(json.dumps(res, indent=1, default=str))
