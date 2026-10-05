"""I65 U4 G4: the sensitivity grid (one cap at a time, plus epsilon 6), each point a full sens.py chain.
Usage: python3 sens_grid.py <scratch dir> <snapshot P dir> <edges json> <lexicon json> > sensitivity.out.jsonl"""
import json, os, subprocess, sys
H = os.path.dirname(os.path.abspath(__file__))
S, SP, EDGES, LEX = sys.argv[1:5]
grid = [({}, 2), ({}, 6)]
for k, vals in [("n", [30, 28, 24, 16]), ("m", [30, 28, 24, 16]), ("g", [28, 24, 16]), ("s", [16, 0]), ("r", [144, 96, 48]),
                ("l", [160, 128, 96, 48]), ("ident", [112, 96, 64])]:
    for v in vals:
        grid.append(({k: v}, 2))
grid.append(({"raw_values": 8192, "raw_string_bytes": 32768, "raw_key_bytes": 32768}, 2))
grid.append(({"raw_values": 4096, "raw_string_bytes": 16384, "raw_key_bytes": 16384}, 2))
for i, (c, e) in enumerate(grid):
    out = subprocess.run(["python3", os.path.join(H, "sens.py"), os.path.join(S, f"sens_{i}"), SP, EDGES, LEX, str(e), json.dumps(c)],
                         capture_output=True, text=True)
    print(out.stdout.strip() if out.returncode == 0 else json.dumps({"caps": c, "eps": e, "error": out.stderr[-600:]}))
