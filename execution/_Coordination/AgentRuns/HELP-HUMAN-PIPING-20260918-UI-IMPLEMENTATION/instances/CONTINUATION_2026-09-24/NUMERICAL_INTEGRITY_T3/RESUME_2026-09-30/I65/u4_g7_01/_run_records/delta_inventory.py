"""I65 U4 G7 item 1: the production-code delta between two revisions, hunk by hunk, with each
hunk's enclosing item and its reachability on the D1 graph (stdlib only; Git reads only).
Reachability: the TEXT chain's lexical call graph from run_linear_static_preview_value_with_retained_direct
(edges json), and its multiplicity M (text_budget whole-run output, after edge_zero).
Usage: python3 delta_inventory.py <repo> <OLD> <NEW> <work tree projects/chirality-piping> <edges json> <text_budget out json>"""
import json, os, re, subprocess, sys
repo, old, new, W, edges_p, tb_p = sys.argv[1:7]
P = "projects/chirality-piping/"
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
git = lambda *a: subprocess.run(["git", "-C", repo, *a], capture_output=True, text=True, env=env, check=True).stdout
names = [f for f in git("diff", "--name-only", old, new, "--", P).split("\n") if f and "/execution/" not in f]
E = json.load(open(edges_p))["edges"]; TB = json.load(open(tb_p)); M = TB["function_multiplicity"]
root = [k for k in E if k.endswith(":run_linear_static_preview_value_with_retained_direct")]
reach, todo = set(root), list(root)
while todo:
    v = todo.pop()
    for w in E.get(v, []):
        if w not in reach: reach.add(w); todo.append(w)
LB = json.load(open(os.path.join(os.path.dirname(tb_p), "loop_bounds.g4.json")))
EZ = {(e["caller"], e["callee"]) for e in LB.get("edge_zero", [])}
sh = lambda k: k.split("/src/")[-1]
live, todo = set(root), list(root)
while todo:
    v = todo.pop()
    for w in E.get(v, []):
        if w not in live and (sh(v), sh(w)) not in EZ: live.add(w); todo.append(w)
FN = re.compile(r"^\s*(pub(\([^)]*\))?\s+)?(const\s+)?(async\s+)?(unsafe\s+)?fn\s+(\w+)")
rows, other = [], []
for f in names:
    rel = f[len(P):]
    if not rel.endswith(".rs") or not rel.startswith("core/") or "/tests/" in rel or rel.endswith("_tests.rs"):
        other.append(rel); continue
    L = open(os.path.join(W, rel), encoding="utf-8").read().split("\n")
    # #[cfg(test)] spans: from the attribute to the end of the item that follows (brace matched)
    test_lines = set()
    for i, l in enumerate(L):
        if re.match(r"\s*#\[cfg\((test|any\(test)", l):
            d, started, j = 0, False, i
            while j < len(L):
                d += L[j].count("{") - L[j].count("}")
                if "{" in L[j]: started = True
                test_lines.add(j + 1)
                if started and d <= 0: break
                j += 1
    for line in git("diff", "-U0", old, new, "--", f).split("\n"):
        m = re.match(r"@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@", line)
        if not m: continue
        n0, nc = int(m.group(3)), int(m.group(4) or 1)
        span = range(n0, n0 + max(nc, 1))
        if nc and all(x in test_lines for x in span):
            rows.append({"file": rel, "new_lines": f"{n0}-{n0 + nc - 1}", "item": "#[cfg(test)]", "class": "test (excluded)"}); continue
        # the enclosing (or, for a new item, the first defined) fn
        fns = set()
        for x in span:
            for j in range(min(x, len(L)) - 1, -1, -1):
                mm = FN.match(L[j])
                if mm:
                    fns.add((j + 1, mm.group(6))); break
        keys = [f"{rel}:{ln}:{nm}" for ln, nm in sorted(fns)]
        rows.append({"file": rel, "new_lines": f"{n0}-{n0 + nc - 1}" if nc else f"after {n0}", "removed": int(m.group(2) or 1) if m.group(2) != "0" else 0,
                     "fns": [{"fn": k.split("/src/")[-1], "reached": k in reach, "live": k in live, "M_text": M.get(k, 0)} for k in keys]})
json.dump({"old": old, "new": new, "rust_hunks": rows, "other_files": other}, sys.stdout, indent=1)
