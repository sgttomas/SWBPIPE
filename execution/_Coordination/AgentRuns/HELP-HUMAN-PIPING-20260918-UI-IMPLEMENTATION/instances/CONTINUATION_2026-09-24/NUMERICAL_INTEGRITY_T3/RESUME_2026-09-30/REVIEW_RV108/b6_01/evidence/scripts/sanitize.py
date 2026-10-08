"""Replace machine paths with the dispatch's placeholders; refuse if any remain."""
import re, sys
from pathlib import Path
ROOTW = "<the worktree that contains WT as .claude/t3>"  # recorded with a placeholder
SUBS = [
    (ROOTW + "/.claude/t3/scratch/rv108_b6_01", "S"),
    (ROOTW + "/.claude/t3/numerics", "NUM"),
    (ROOTW + "/.claude/t3", "WT"),
    (ROOTW + "/projects/chirality-piping/.venv", "VENV"),
    (ROOTW + "/projects/chirality-piping/node_modules", "NMS"),
]
bad = []
for p in map(Path, sys.argv[1:]):
    t = p.read_text(errors="strict")
    for a, b in SUBS: t = t.replace(a, b)
    t = re.sub(r"/" + "Users/[^/\s]+/\.local/share/mise/installs/[^\s'\"]*", "<toolchain>", t)
    p.write_text(t)
    if re.search("/" + "Users/|/" + "private/|/" + "var/folders/", t): bad.append(str(p))
if bad: print("UNSANITIZED:", bad); sys.exit(1)
print("sanitized", len(sys.argv) - 1, "files")
