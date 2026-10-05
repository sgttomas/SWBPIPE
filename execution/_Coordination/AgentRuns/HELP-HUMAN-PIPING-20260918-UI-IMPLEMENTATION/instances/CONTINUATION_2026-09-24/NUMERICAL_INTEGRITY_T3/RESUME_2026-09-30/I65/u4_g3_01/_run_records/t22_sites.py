"""I65 U4 G3 T22: reproducible scalar-site census (stdlib only, read-only).
Lists, in non-test source (text before the first `#[cfg(test)] mod`), every
`CountRange(` token (with its label when literal), every `checked_mul(64)`,
every `u32::try_from(`, `ceil_sqrt` use, and residual/fallback count-to-m sites.
Usage: python3 t22_sites.py <P root> <dir> [<dir> ...]"""
import json, os, re, sys
root = sys.argv[1]
rows = []
for d0 in sys.argv[2:]:
    for d, _, fs in os.walk(os.path.join(root, d0)):
        if "/tests" in d: continue
        for f in sorted(fs):
            if not f.endswith(".rs") or "test" in f: continue
            p = os.path.join(d, f); t = open(p, encoding="utf-8").read()
            m = re.search(r"#\[cfg\(test\)\]\s*\n\s*(pub(\(crate\))?\s+)?mod\s+\w+\s*\{", t)
            if m: t = t[:m.start()]
            for kind, rx in [("CountRange", r"CountRange\(\s*(\"[^\"]*\")?"), ("checked_mul(64)", r"checked_mul\(64\)"),
                             ("u32::try_from", r"u32::try_from\("), ("ceil_sqrt", r"ceil_sqrt\("),
                             ("checked_mul", r"checked_mul\(")]:
                for mm in re.finditer(rx, t):
                    line = t.count("\n", 0, mm.start()) + 1
                    rows.append({"file": os.path.relpath(p, root), "line": line, "kind": kind,
                                 "label": (mm.group(1) if kind == "CountRange" and mm.group(1) else None),
                                 "text": t.split("\n")[line - 1].strip()[:160]})
summ = {}
for r in rows: summ[r["kind"]] = summ.get(r["kind"], 0) + 1
lab = sum(1 for r in rows if r["kind"] == "CountRange" and r["label"])
print(json.dumps({"summary": summ, "CountRange_with_literal_label": lab, "rows": rows}, indent=1))
