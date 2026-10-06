"""I72 U8-4: compare a Pass B output folder with an earlier one, file by file, byte for byte (stdlib only).
Skips work/ (the source copy), tmp/, rr/ (copied rules, compared through rules/ and sens_pb/) and logs/.
Usage: python3 compare_outputs.py <this run's pass_<tag>> <earlier pass_<tag>>
Prints one line per file: SAME, DIFF, ONLY_THIS or ONLY_EARLIER, then a count line."""
import hashlib, os, sys
a, b = sys.argv[1:3]
SKIP = ("work", "tmp", "rr", "logs")
def files(root):
    out = {}
    for d, ds, fs in os.walk(root):
        rel = os.path.relpath(d, root)
        if rel.split(os.sep)[0] in SKIP: ds[:] = []; continue
        for f in fs:
            p = os.path.join(d, f); out[os.path.relpath(p, root)] = hashlib.sha256(open(p, "rb").read()).hexdigest()
    return out
A, B = files(a), files(b); n = {}
for k in sorted(set(A) | set(B)):
    s = "SAME" if A.get(k) == B.get(k) else "ONLY_THIS" if k not in B else "ONLY_EARLIER" if k not in A else "DIFF"
    n[s] = n.get(s, 0) + 1; print(s, k)
print("COUNTS", " ".join(f"{k}={v}" for k, v in sorted(n.items())))
