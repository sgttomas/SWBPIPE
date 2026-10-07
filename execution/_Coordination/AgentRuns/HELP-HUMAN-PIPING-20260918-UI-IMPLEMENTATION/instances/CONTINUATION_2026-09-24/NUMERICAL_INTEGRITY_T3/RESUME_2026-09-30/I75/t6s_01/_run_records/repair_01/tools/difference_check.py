"""I75 REPAIR_01: every pre-repair difference (V8 shortest form vs Rust {:e}) checked to be an exact tie:
the exact value lies exactly halfway between the two printed n-digit decimals, V8's is the even, lower one,
and Rust's is the larger. Exact arithmetic (fractions); reads <list>.txt and <list>.txt.rust.txt; V8's string
is recomputed by node (forms.mjs's pre-repair printer) and passed in as <list>.v8.txt."""
import json, struct, sys, pathlib
from fractions import Fraction
d = pathlib.Path(sys.argv[1]); out = {}
for name in ["random", "ties", "rv101_differences", "edges", "ties16", "pow2"]:
    W = (d / f"{name}.txt").read_text().split(); R = (d / f"{name}.txt.rust.txt").read_text().split(); V = (d / f"{name}.v8.txt").read_text().split()
    n = bad = 0; by = {}
    for w, r, v in zip(W, R, V):
        if r == v: continue
        n += 1; x = Fraction(struct.unpack(">d", bytes.fromhex(w))[0]); ax = abs(x)
        fr, fv = abs(Fraction(r)), abs(Fraction(v))
        dr, dv = r.lstrip("-").split("e")[0].replace(".", ""), v.lstrip("-").split("e")[0].replace(".", "")
        ok = len(dr) == len(dv) and fr > fv and ax - fv == fr - ax and int(dv[-1]) % 2 == 0 and (x < 0) == r.startswith("-") == v.startswith("-")
        by[len(dr)] = by.get(len(dr), 0) + 1
        if not ok: bad += 1
    out[name] = {"differences": n, "not_a_tie_rounded_up": bad, "by_digits": by}
print(json.dumps(out, indent=1))
