#!/usr/bin/env python3
"""I85 B1-SP I3: every 64-hex literal (a pinned sha256) in PP's src and in RE's and the runner's
src/tests, at the base (2ba2f81863) and at the head (03f55e7178), by file. Removed literals would be changed
pins; added ones are new pins. Run in WT/b1 (read-only git: GIT_OPTIONAL_LOCKS=0).
Usage: pin_literals.py <base> <head>"""
import os, re, subprocess, sys
base, head = sys.argv[1:3]
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
roots = ["projects/chirality-piping/core/product_physics/src", "projects/chirality-piping/core/product_physics/tests",
         "projects/chirality-piping/core/reporting/result_export/src", "projects/chirality-piping/core/reporting/result_export/tests",
         "projects/chirality-piping/core/runner/headless/src", "projects/chirality-piping/core/runner/headless/tests"]
def files(rev):
    out = subprocess.run(["git", "ls-tree", "-r", "--name-only", rev, "--"] + roots, capture_output=True, text=True, env=env).stdout.split()
    return [f for f in out if f.endswith(".rs")]
def lits(rev, f):
    text = subprocess.run(["git", "show", f"{rev}:{f}"], capture_output=True, text=True, env=env).stdout
    return re.findall(r"\b[0-9a-f]{64}\b", text)
fb, fh = files(base), files(head)
total_b = total_h = 0
removed_all, added_all = [], []
for f in sorted(set(fb) | set(fh)):
    b = lits(base, f) if f in fb else []
    h = lits(head, f) if f in fh else []
    total_b += len(b); total_h += len(h)
    rem = sorted(set(b) - set(h)); add = sorted(set(h) - set(b))
    removed_all += [(f, x) for x in rem]; added_all += [(f, x) for x in add]
print(f"files scanned: base {len(fb)}, head {len(fh)}; 64-hex literals: base {total_b}, head {total_h}")
print(f"removed at head (changed pins): {len(removed_all)}")
for f, x in removed_all: print(f"  - {os.path.basename(f)} {x}")
print(f"added at head (new pins): {len(added_all)}")
for f, x in added_all: print(f"  + {os.path.basename(f)} {x}")
