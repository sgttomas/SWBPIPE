"""I65 U4 G3: recursion depth of the Rust reader's schema walkers (stdlib only, read-only).

Mirrors result_export/src/retained_precision.rs `shape` (:360) and `encoding` (:400):
from a schema node they recurse on the SAME value through `$ref` and each `oneOf`
branch, and on a CHILD value through `properties/<k>` and `items`.
Builds that graph over schema nodes (JSON pointers), reports any cycle, the longest
same-value chain, and the longest total path from the root (an upper bound on the
walkers' nested recursion depth when every descending edge consumes one value level).
Usage: python3 schema_depth.py <schema.json>
"""
import json, sys
sys.setrecursionlimit(100000)
schema = json.load(open(sys.argv[1]))

def node(ptr):
    cur = schema
    for part in [p for p in ptr.split("/") if p][0:]:
        part = part.replace("~1", "/").replace("~0", "~")
        cur = cur[int(part)] if isinstance(cur, list) else cur[part]
    return cur

same, desc = {}, {}
def walk(ptr):
    if ptr in same:
        return
    s = node(ptr)
    same[ptr], desc[ptr] = [], []
    if not isinstance(s, dict):
        return
    if isinstance(s.get("$ref"), str):
        same[ptr].append(s["$ref"].lstrip("#"))
    if isinstance(s.get("oneOf"), list):
        same[ptr].extend(f"{ptr}/oneOf/{i}" for i in range(len(s["oneOf"])))
    if isinstance(s.get("properties"), dict):
        desc[ptr].extend(f"{ptr}/properties/{k.replace('~','~0').replace('/','~1')}" for k in s["properties"])
    if isinstance(s.get("items"), dict):
        desc[ptr].append(f"{ptr}/items")
    for t in same[ptr] + desc[ptr]:
        walk(t)
walk("")

# cycle detection and longest paths (memoized DFS with colouring)
color, best_total, best_same = {}, {}, {}
cycles = []
def dfs(p, stack):
    color[p] = 1
    bt, bs = 0, 0
    for t in same[p]:
        if color.get(t) == 1:
            cycles.append(stack + [p, t]); continue
        if t not in best_total:
            dfs(t, stack + [p])
        bt = max(bt, 1 + best_total.get(t, 0)); bs = max(bs, 1 + best_same.get(t, 0))
    for t in desc[p]:
        if color.get(t) == 1:
            cycles.append(stack + [p, t]); continue
        if t not in best_total:
            dfs(t, stack + [p])
        bt = max(bt, 1 + best_total.get(t, 0))
    color[p] = 2
    best_total[p], best_same[p] = bt, bs
dfs("", [])
desc_only = {}
def dd(p):
    if p in desc_only: return desc_only[p]
    desc_only[p] = 0
    v = max([dd(t) for t in same[p]] + [1 + dd(t) for t in desc[p]] + [0])
    desc_only[p] = v
    return v
print(json.dumps({"schema_nodes_visited": len(same), "cycles": cycles[:20], "cycle_count": len(cycles),
                  "max_same_value_chain": max(best_same.values()),
                  "max_total_recursion_depth_from_root": best_total[""],
                  "max_value_depth_reached_by_schema": dd("")}, indent=1))
