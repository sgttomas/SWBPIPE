"""Production statics and embedded files in the D1 crates (stdlib only): every line with
include_str!/include_bytes!/OnceLock/LazyLock/lazy_static!/thread_local! in non-test sources,
skipping #[cfg(test)] items. Keyed by (file, stripped text) so line shifts do not matter.
Usage: python3 statics_list.py <tree projects/chirality-piping> <crate_dirs.txt> [<reference json>]"""
import json, os, re, sys, collections
root, crates = sys.argv[1:3]
TOK = re.compile(r"include_str!|include_bytes!|OnceLock|LazyLock|lazy_static!|thread_local!")
rows = []
def cfg_test_spans(L):
    out = []
    for i, l in enumerate(L):
        if re.match(r"\s*#\[cfg\((test|any\(test)", l):
            depth, started, j = 0, False, i
            while j < len(L):
                depth += L[j].count("{") - L[j].count("}")
                if "{" in L[j]: started = True
                if started and depth <= 0: break
                if not started and L[j].rstrip().endswith(";") and j > i: break
                j += 1
            out.append((i + 1, j + 1))
    return out
def test_module_file(p):
    """G7 Pass B: a child module file whose every `mod <stem>;` declaration in its crate lies in #[cfg(test)]"""
    stem = os.path.splitext(os.path.basename(p))[0]
    if stem in ("lib", "mod", "main"): return False
    src = p.split("/src/")[0] + "/src"; decls = []
    for d, _, fs in os.walk(src):
        for f in fs:
            if not f.endswith(".rs"): continue
            L = open(os.path.join(d, f), encoding="utf-8").read().split("\n")
            hits = [i + 1 for i, l in enumerate(L) if re.search(r"(^|[\s}])mod\s+" + re.escape(stem) + r"\s*;", l.split("//")[0])]
            if hits:
                sp = cfg_test_spans(L); decls += [any(a <= h <= b for a, b in sp) for h in hits]
    return bool(decls) and all(decls)
for c in open(crates).read().split():
    for d, _, fs in os.walk(os.path.join(root, c)):
        if "/tests" in d or "/bin" in d: continue
        for f in sorted(fs):
            if not f.endswith(".rs") or f.endswith("_tests.rs") or f == "tests.rs": continue
            p = os.path.join(d, f); rel = os.path.relpath(p, root)
            if test_module_file(p): continue
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
