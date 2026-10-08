"""RV113 (RV-R): replace host paths with placeholders in a text file (in place), then screen it.
Usage: sanitize_py.py <file> ...   (exit 1 if any host form survives)"""
import gzip
import os
import sys

ROOT = os.environ["RV113_HOST_ROOT"]  # the host checkout that holds WT (given at run time, never recorded)
WT = ROOT + "/.claude/t3"
NUM = WT + "/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30"
PAIRS = [
    (NUM, "R"),
    (WT + "/rv113/py-i1/projects/chirality-piping", "P@I1"),
    (WT + "/rv113/py-head/projects/chirality-piping", "P@HEAD"),
    (WT + "/rv113/py-mut/projects/chirality-piping", "P@MUT"),
    (WT + "/rv113/fg-rs/projects/chirality-piping", "P@RS"),
    (WT + "/rv113/fg-ts/projects/chirality-piping", "P@TS"),
    (ROOT + "/projects/chirality-piping/node_modules", "NMS"),
    (ROOT + "/projects/chirality-piping/.venv", "VENV"),
    (WT, "WT"),
    (os.environ["RV113_HOST_HOME"] + "/.local/share/mise/installs/python/3.13.14", "PYTHON"),
    (os.environ["RV113_HOST_HOME"] + "/.cargo", "CARGO_HOME"),
    (os.environ["RV113_HOST_HOME"] + "/.rustup", "RUSTUP_HOME"),
]
# The screened host forms, spelled so that this file does not contain them itself.
BAD = ["/" + "Users/", "/" + "private/", "~" + "/", "swb" + "pipe", "8a4" + "1be", "/var/" + "folders/"]
bad = 0
for path in sys.argv[1:]:
    gz = path.endswith(".gz")
    raw = (gzip.open if gz else open)(path, "rt", encoding="utf-8", errors="surrogateescape").read()
    for a, b in PAIRS:
        raw = raw.replace(a, b)
    hits = [x for x in BAD if x in raw]
    if hits:
        bad += 1
        print("HOST FORM LEFT", path.rsplit("/", 1)[-1], hits)
    with (gzip.open(path, "wt", encoding="utf-8", errors="surrogateescape", compresslevel=9) if gz else open(path, "w", encoding="utf-8", errors="surrogateescape")) as f:
        f.write(raw)
sys.exit(1 if bad else 0)
