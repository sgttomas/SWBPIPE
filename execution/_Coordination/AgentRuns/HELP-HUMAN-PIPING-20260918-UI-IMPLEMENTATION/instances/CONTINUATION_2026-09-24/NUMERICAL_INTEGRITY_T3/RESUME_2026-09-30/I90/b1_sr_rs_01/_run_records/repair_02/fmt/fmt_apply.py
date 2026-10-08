"""I90 SR-RS repair 2: apply only the rustfmt --check blocks that touch this round's lines.
Usage: fmt_apply.py <file> <git diff -U0 output> <rustfmt --check output>"""
import re, sys
path, mine_path, fmt_path = sys.argv[1:]
mine = set()
for line in open(mine_path):
    m = re.match(r"^@@ -\d+(?:,\d+)? \+(\d+)(?:,(\d+))? @@", line)
    if m:
        start, count = int(m.group(1)), int(m.group(2) or 1)
        mine.update(range(start, start + count))
blocks, cur = [], None
for line in open(fmt_path):
    m = re.match(r"^Diff in .*:(\d+):$", line.rstrip("\n"))
    if m:
        cur = {"start": int(m.group(1)), "lines": []}; blocks.append(cur); continue
    if cur is not None and line[:1] in "+- ":
        cur["lines"].append(line.rstrip("\n"))
text = open(path).read().split("\n")
applied = 0
for b in sorted(blocks, key=lambda b: -b["start"]):
    old = [l[1:] for l in b["lines"] if l[:1] in " -"]
    new = [l[1:] for l in b["lines"] if l[:1] in " +"]
    span = set(range(b["start"], b["start"] + len(old)))
    if not span & mine:
        continue
    i = b["start"] - 1
    assert text[i:i + len(old)] == old, (b["start"], old[:3], text[i:i+3])
    text[i:i + len(old)] = new
    applied += 1
open(path, "w").write("\n".join(text))
print(f"applied {applied} of {len(blocks)} blocks")
