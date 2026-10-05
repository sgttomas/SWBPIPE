"""I65 U4 G2: lead scan for directly self-recursive functions (stdlib only, read-only).

This is a text heuristic, not a call graph: it finds `fn name` bodies (by brace
matching) that contain a call to the same name. It does not see mutual recursion,
trait-dispatched recursion or derive-generated recursion (serde, Drop, Clone,
Debug). STACK_PLAN.md lists those separately; G3 completes the call-graph
derivation. Test-only code is skipped: files whose name contains "test", and
everything after the first `#[cfg(test)]` line in a file.
Usage: python3 recursion_scan.py <repo-relative root> <crate dir> [<crate dir> ...]
"""
import json, os, re, sys

root = sys.argv[1]
results = []
FN = re.compile(r'\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(<[^>{]*>)?\s*\(')
for crate in sys.argv[2:]:
    base = os.path.join(root, crate)
    for dirpath, _, files in os.walk(base):
        if "/target" in dirpath or "/tests" in dirpath:
            continue
        for f in sorted(files):
            if not f.endswith(".rs") or "test" in f:
                continue
            path = os.path.join(dirpath, f)
            text = open(path, encoding="utf-8").read()
            cut = text.find("#[cfg(test)]")
            if cut >= 0:
                text = text[:cut]
            for m in FN.finditer(text):
                name = m.group(1)
                brace = text.find("{", m.end())
                semi = text.find(";", m.end())
                if brace < 0 or (0 <= semi < brace):
                    continue
                depth, i = 0, brace
                while i < len(text):
                    if text[i] == "{":
                        depth += 1
                    elif text[i] == "}":
                        depth -= 1
                        if depth == 0:
                            break
                    i += 1
                body = text[brace + 1:i]
                if re.search(r'(?<![A-Za-z0-9_])' + re.escape(name) + r'\s*(::<[^>]*>)?\s*\(', body):
                    line = text.count("\n", 0, m.start()) + 1
                    results.append({"file": os.path.relpath(path, root), "line": line, "fn": name})
print(json.dumps({"crates": sys.argv[2:], "self_recursive_candidates": results}, indent=1))
