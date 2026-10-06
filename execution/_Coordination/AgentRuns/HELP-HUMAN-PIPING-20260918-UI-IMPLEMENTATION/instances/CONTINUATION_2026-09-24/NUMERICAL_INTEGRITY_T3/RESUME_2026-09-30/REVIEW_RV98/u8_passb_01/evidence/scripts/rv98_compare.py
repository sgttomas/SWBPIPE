"""RV98: byte-for-byte comparison of two Pass B output folders.
Usage: rv98_compare.py <label A> <dir A> <label B> <dir B> [skip,dirs]
Skips the top-level folders named in the skip list (default work,tmp,rr,logs). Prints identical /
differing / only-in-A / only-in-B, with sha256 prefixes for the differing files."""
import hashlib, os, sys
la, a, lb, b = sys.argv[1:5]
skip = set((sys.argv[5] if len(sys.argv) > 5 else "work,tmp,rr,logs").split(","))
def files(root):
    out = {}
    for d, ds, fs in os.walk(root):
        rel = os.path.relpath(d, root)
        if rel == "." : ds[:] = [x for x in ds if x not in skip]
        for f in fs:
            p = os.path.normpath(os.path.join(rel, f))
            out[p] = os.path.join(d, f)
    return out
h = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest()
A, B = files(a), files(b)
same = sorted(k for k in A if k in B and h(A[k]) == h(B[k]))
diff = sorted(k for k in A if k in B and h(A[k]) != h(B[k]))
oa = sorted(k for k in A if k not in B); ob = sorted(k for k in B if k not in A)
print(f"{la} vs {lb} (skipping top-level {sorted(skip)}): common {len(same)+len(diff)}, identical {len(same)}, differ {len(diff)}, only in {la} {len(oa)}, only in {lb} {len(ob)}")
for k in diff: print(f"  DIFFER   {k}  {h(A[k])[:12]} {h(B[k])[:12]}")
for k in oa: print(f"  ONLY_{la}  {k}")
for k in ob: print(f"  ONLY_{lb}  {k}")
print("  IDENTICAL:", " ".join(same))
