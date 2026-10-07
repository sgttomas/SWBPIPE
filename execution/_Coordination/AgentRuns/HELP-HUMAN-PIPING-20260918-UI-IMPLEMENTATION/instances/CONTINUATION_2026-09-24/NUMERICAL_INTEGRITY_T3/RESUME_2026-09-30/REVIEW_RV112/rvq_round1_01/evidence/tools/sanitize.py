#!/usr/bin/env python3
"""RV112: copy a text file into the records with placeholder paths only.

Usage: sanitize.py <in> <out>
Longest prefixes first: R, T, NUM, VENV, WT (WT is this file's grand-grand-parent: WT/scratch/rv112_rvq_01/tools).
The toolchain and cargo homes become RUSTUP and CARGO_HOME. Refuses (exit 1) if any absolute
home or system-temp path, or a home-relative form, is left.
"""
import os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
WT = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
assert os.path.isdir(os.path.join(WT, "tools")) and os.path.isdir(os.path.join(WT, "guard")), WT
NUM = os.path.join(WT, "numerics")
T = os.path.join(NUM, "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3")
R = os.path.join(T, "RESUME_2026-09-30")
VENV = os.path.abspath(os.path.join(WT, "..", "..", "projects/chirality-piping/.venv"))
assert os.path.isdir(VENV), VENV
HOME = os.path.expanduser("~")
PAIRS = [(R, "R"), (T, "T"), (NUM, "NUM"), (VENV, "VENV"), (WT, "WT"),
         (os.path.join(HOME, ".rustup"), "RUSTUP"), (os.path.join(HOME, ".cargo"), "CARGO_HOME")]


def clean(text):
    for prefix, name in PAIRS:
        text = text.replace(prefix, name)
    roots = ["/" + "Users/", "/" + "private/", "~" + "/", "/" + "var/folders/"]
    bad = re.findall("(" + "|".join(re.escape(x) + r"[^\s'\"]*" for x in roots) + ")", text)
    if bad:
        raise SystemExit(f"unsanitized paths left: {sorted(set(bad))[:5]}")
    return text


if __name__ == "__main__":
    src, dst = sys.argv[1], sys.argv[2]
    text = open(src, encoding="utf-8", errors="replace").read()
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    open(dst, "w", encoding="utf-8").write(clean(text))
