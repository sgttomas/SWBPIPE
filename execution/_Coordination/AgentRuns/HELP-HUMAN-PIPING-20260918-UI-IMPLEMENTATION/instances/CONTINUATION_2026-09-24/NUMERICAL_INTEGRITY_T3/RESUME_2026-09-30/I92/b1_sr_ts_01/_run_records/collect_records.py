"""I92: copy run records into R/I92/b1_sr_ts_01/_run_records with placeholder paths only, then screen them.
Placeholders: WT (the T3 worktree root), NMS (the shared node_modules), VENV, P inside paths stays relative."""
import os, re, shutil, sys, hashlib

WT = "WT"
ROOTWT = "<the main checkout that holds NMS and VENV; its absolute path is not recorded>"
S = WT + "/scratch/i92_b1_sr_ts"
R = WT + "/numerics/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30"
OUT = R + "/I92/b1_sr_ts_01/_run_records"
SUBS = [
    (ROOTWT + "/projects/chirality-piping/node_modules", "NMS"),
    (ROOTWT + "/projects/chirality-piping/.venv", "VENV"),
    (WT + "/scratch/i92_b1_sr_ts", "S"),
    (WT, "WT"),
]
SCREEN = re.compile(SCREEN_PATTERN)  # recorded copy: the pattern matches a home path, the system temp path, a home-relative form, the worktrees directory and the worktree names; its literal text is not recorded

def sanitize(text):
    for a, b in SUBS: text = text.replace(a, b)
    return text

def put(src, rel):
    dst = os.path.join(OUT, rel); os.makedirs(os.path.dirname(dst), exist_ok=True)
    text = sanitize(open(src, encoding="utf-8").read())
    open(dst, "w", encoding="utf-8").write(text)

def put_text(rel, text):
    dst = os.path.join(OUT, rel); os.makedirs(os.path.dirname(dst), exist_ok=True)
    open(dst, "w", encoding="utf-8").write(sanitize(text))

if __name__ == "__main__":
    for src, rel in [(a, b) for a, b in (line.split("\t") for line in sys.stdin.read().strip().splitlines())]:
        put(src, rel)
    bad = []
    for root, _, files in os.walk(os.path.dirname(OUT)):
        for f in files:
            p = os.path.join(root, f)
            for i, line in enumerate(open(p, encoding="utf-8", errors="replace"), 1):
                if SCREEN.search(line): bad.append(f"{p.replace(R, 'R')}:{i}")
    print("screen hits", len(bad)); [print(b) for b in bad[:20]]
