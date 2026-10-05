"""Production statics and embedded files in the D1 crates (stdlib only): every line with
include_str!/include_bytes!/OnceLock/LazyLock/lazy_static!/thread_local! in non-test sources,
skipping #[cfg(test)] items. Keyed by (file, stripped text) so line shifts do not matter.
Usage: python3 statics_list.py <tree projects/chirality-piping> <crate_dirs.txt> [<reference json>]"""
import json, os, re, sys, collections
root, crates = sys.argv[1:3]
TOK = re.compile(r"include_str!|include_bytes!|OnceLock|LazyLock|lazy_static!|thread_local!")
rows = []
for c in open(crates).read().split():
    for d, _, fs in os.walk(os.path.join(root, c)):
        if "/tests" in d or "/bin" in d: continue
        for f in sorted(fs):
            if not f.endswith(".rs") or f.endswith("_tests.rs") or f == "tests.rs": continue
            p = os.path.join(d, f); rel = os.path.relpath(p, root)
            L = open(p, encoding="utf-8").read().split("\n")
            skip = set(); i = 0
            while i < len(L):
                if re.match(r"\s*#\[cfg\((test|any\(test)", L[i]):
                    depth, started, j = 0, False, i
                    while j < len(L):
                        depth += L[j].count("{") - L[j].count("}")
                        if "{" in L[j]: started = True
                        skip.add(j)
                        if started and depth <= 0: break
                        if not started and L[j].rstrip().endswith(";") and j > i: break
                        j += 1
                    i = j
                i += 1
            for n, l in enumerate(L):
                if n not in skip and TOK.search(l) and not l.strip().startswith("//"):
                    rows.append({"file": rel, "line": n + 1, "text": l.strip()})
out = {"rows": rows}
if len(sys.argv) > 3:
    ref = json.load(open(sys.argv[3]))["rows"]
    a = collections.Counter((r["file"], r["text"]) for r in ref); b = collections.Counter((r["file"], r["text"]) for r in rows)
    out["added"] = [list(k) for k in (b - a)]; out["removed"] = [list(k) for k in (a - b)]
print(json.dumps(out, indent=1))
