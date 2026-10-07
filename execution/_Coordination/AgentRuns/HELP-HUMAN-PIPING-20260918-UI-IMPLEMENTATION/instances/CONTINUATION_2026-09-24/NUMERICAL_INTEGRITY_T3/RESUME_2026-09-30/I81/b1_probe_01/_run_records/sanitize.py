# I81: copy a log into the records with machine paths replaced by placeholders, and every line
# longer than 4,000 bytes (other than this probe's own I81_ lines) cut to its first 1,000 bytes plus its full length and sha256 (the long
# lines are the producer's committed cfg(test) prints I51_DUAL_LANES / I51_FROZEN_*).
import hashlib, os, sys
src, dst, wt = sys.argv[1], sys.argv[2], sys.argv[3]
cut = 0
with open(src, encoding="utf-8", errors="replace") as f, open(dst, "w") as g:
    for line in f:
        line = line.rstrip("\n")
        line = line.replace(wt + "/scratch/i81_b1_probe/arch", "ARCH").replace(wt, "WT").replace(os.path.expanduser("~"), "~")
        if len(line.encode()) > 4000 and "I81_" not in line[:120]:
            b = line.encode()
            line = b[:1000].decode("utf-8", "ignore") + "...[cut by sanitize.py: %d bytes, sha256 %s]" % (len(b), hashlib.sha256(b).hexdigest())
            cut += 1
        g.write(line + "\n")
print(dst.split("/")[-1], "lines cut:", cut)
