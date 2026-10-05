"""Mechanical check that an edit is comment/doc-only and line-neutral: same line count, and every
line that differs is a `//`, `///` or `//!` comment line on both sides (stdlib only).
Usage: python3 comment_only_check.py <base root> <edited root> <file> [<file> ...]"""
import json, sys
base, edit, files = sys.argv[1], sys.argv[2], sys.argv[3:]
out, ok = [], True
for f in files:
    a = open(f"{base}/{f}").read().split("\n"); b = open(f"{edit}/{f}").read().split("\n")
    changed = [i + 1 for i, (x, y) in enumerate(zip(a, b)) if x != y]
    bad = [i for i in changed if not (a[i - 1].lstrip().startswith("//") and b[i - 1].lstrip().startswith("//"))]
    r = {"file": f, "lines": [len(a), len(b)], "changed_lines": changed, "non_comment_changes": bad}
    ok &= len(a) == len(b) and not bad and bool(changed)
    out.append(r)
print(json.dumps({"comment_only_and_line_neutral": ok, "files": out}, indent=1))
sys.exit(0 if ok else 1)
