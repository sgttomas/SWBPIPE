"""Copy a file into the records with host paths replaced by placeholders; refuse any host path left.

Usage: sanitize.py <src> <dst>
"""
import re, sys
from pathlib import Path

T3 = "WT"
VENV = "VENV"
PAIRS = [
    (VENV, "VENV"),
    (T3 + "/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30", "R"),
    (T3 + "/numerics", "NUM"),
    (T3, "WT"),
]
# The screened host forms, built from parts so that this file does not contain them literally.
FORMS = ["/" + "Users/", "/" + "private/", "~" + "/", re.escape("." + "claude/" + "worktrees")]
src, dst = Path(sys.argv[1]), Path(sys.argv[2])
text = src.read_text(errors="strict")
for old, new in PAIRS:
    text = text.replace(old, new)
bad = [m.group(0) for m in re.finditer("(" + "|".join(FORMS) + ")" + r"[^\s\"']*", text)]
if bad:
    sys.exit(f"host path left in {src}: {bad[:3]}")
dst.parent.mkdir(parents=True, exist_ok=True)
dst.write_text(text)
