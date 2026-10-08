"""I90 SR-RS repair 2: which rustfmt --check blocks would change a line this round added or changed.
Usage: fmt_hunks.py <git diff -U0 output> <rustfmt --check output>"""
import re, sys
mine = set()
for line in open(sys.argv[1]):
    m = re.match(r"^@@ -\d+(?:,\d+)? \+(\d+)(?:,(\d+))? @@", line)
    if m:
        start, count = int(m.group(1)), int(m.group(2) or 1)
        mine.update(range(start, start + count))
blocks, cur = [], None
for line in open(sys.argv[2]):
    m = re.match(r"^Diff in .*:(\d+):$", line.rstrip("\n"))
    if m:
        cur = {"start": int(m.group(1)), "lines": []}; blocks.append(cur); continue
    if cur is not None and line[:1] in "+- ":
        cur["lines"].append(line.rstrip("\n"))
touching = []
for b in blocks:
    n = b["start"]; span = set()
    for l in b["lines"]:
        if l.startswith("+"): continue
        if l.startswith("-"): span.add(n)
        n += 1
    if span & mine:
        touching.append(b)
print(f"rustfmt blocks: {len(blocks)}; touching this round's lines: {len(touching)}")
for b in touching:
    print(f"--- block at line {b['start']}"); print("\n".join(b["lines"][:60]))
