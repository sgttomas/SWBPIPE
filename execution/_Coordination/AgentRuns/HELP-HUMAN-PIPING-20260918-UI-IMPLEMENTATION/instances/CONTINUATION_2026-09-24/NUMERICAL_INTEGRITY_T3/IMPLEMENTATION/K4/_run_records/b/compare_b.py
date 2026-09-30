"""Compare K4's 39-manifest run with the Mac baseline of main 7ac7b1c37's piping
source (K6's final-head run at cd325c1fe, the same piping tree; operation_applier
from its fresh-target re-run)."""
import re, sys, glob, os
base_log, base_vs, cand_log, cand_dir = sys.argv[1:5]
def parse(path):
    out = {}
    for l in open(path):
        m = re.match(r'^(\d+) passed=(\d+) failed=(\d+) ignored=(\d+) (\S+)$', l.strip())
        if m:
            out[m.group(5)] = tuple(int(m.group(i)) for i in (1, 2, 3, 4))
    return out
base = parse(base_log); cand = parse(cand_log)
base["core/model_operations/operation_applier/Cargo.toml"] = (0, 194, 0, 0)  # fresh-target re-run
txt = open(base_vs).read()
bfail = eval(txt.split("candidate failing: ")[1].split("\n")[0])
base_fail = {k: v for k, v in bfail}
cand_fail = {}
for f in sorted(glob.glob(os.path.join(cand_dir, "*.log"))):
    names = sorted(set(re.findall(r'^test (\S+) \.\.\. FAILED', open(f, errors="replace").read(), re.M)))
    if names:
        cand_fail[os.path.basename(f)] = names
print("manifests: base %d cand %d" % (len(base), len(cand)))
for mf in sorted(set(base) | set(cand)):
    b, c = base.get(mf), cand.get(mf)
    if b is None or c is None or b[1:] != c[1:] or (b[0] != 0) != (c[0] != 0):
        print("CHANGED %s: base %s -> cand %s" % (mf, b, c))
print("candidate failing:", sorted(cand_fail.items()))
print("baseline failing:", sorted(base_fail.items()))
print("failing sets identical:", sorted(cand_fail.items()) == sorted(base_fail.items()))
