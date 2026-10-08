# I98 (after I81's and I86's sanitize.py): copy a log into the records with machine paths
# replaced by placeholders (ARCH for the archive, S for this probe's scratch, WT, VENV_ROOT, HOME), and
# every line longer than 4,000 bytes that is not one of this probe's own I98_ lines cut to its
# first 1,000 bytes plus its full length and sha256 (the long lines are the producer's committed
# cfg(test) prints I51_DUAL_LANES / I51_FROZEN_*). A libtest "test <name> ... " prefix does not
# hide an I98_ line. Usage: python sanitize.py <src> <dst> <WT> <VENV python>
import hashlib
import os
import sys

src, dst, wt, venv = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
venv_root = venv.rsplit("/bin/python", 1)[0]
cut = 0
with open(src, encoding="utf-8", errors="replace") as f, open(dst, "w", encoding="utf-8") as g:
    for line in f:
        line = line.rstrip("\n")
        line = (line.replace(wt + "/scratch/i98_b2w/arch", "ARCH").replace(wt + "/scratch/i98_b2w", "S")
                .replace(venv_root, "VENV_ROOT").replace(wt, "WT").replace(os.path.expanduser("~"), "HOME"))
        head = line[:200]
        own = head.startswith("I98_") or (head.startswith("test ") and " I98_" in head) or head.startswith("PY_READER") or head.startswith("  PY_READER")
        if len(line.encode()) > 4000 and not own:
            b = line.encode()
            line = b[:1000].decode("utf-8", "ignore") + "...[cut by sanitize.py: %d bytes, sha256 %s]" % (len(b), hashlib.sha256(b).hexdigest())
            cut += 1
        g.write(line + "\n")
print(dst.split("/")[-1], "lines cut:", cut)
