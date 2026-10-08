# I86 (after I81's sanitize.py): copy a log into the records with machine paths replaced by
# placeholders (ARCH for the archive, WT, ~), and every line longer than 4,000 bytes that is
# not one of this probe's own I86_ lines cut to its first 1,000 bytes plus its full length and
# sha256 (the long lines are the producer's committed cfg(test) prints I51_DUAL_LANES /
# I51_FROZEN_*). A libtest "test <name> ... " prefix does not hide an I86_ line.
import hashlib, os, sys
src, dst, wt = sys.argv[1], sys.argv[2], sys.argv[3]
cut = 0
with open(src, encoding="utf-8", errors="replace") as f, open(dst, "w") as g:
    for line in f:
        line = line.rstrip("\n")
        line = line.replace(wt + "/scratch/i86_b1_w/arch", "ARCH").replace(wt, "WT").replace(os.path.expanduser("~"), "~")
        head = line[:200]
        own = head.startswith("I86_") or (head.startswith("test ") and " I86_" in head)
        if len(line.encode()) > 4000 and not own:
            b = line.encode()
            line = b[:1000].decode("utf-8", "ignore") + "...[cut by sanitize.py: %d bytes, sha256 %s]" % (len(b), hashlib.sha256(b).hexdigest())
            cut += 1
        g.write(line + "\n")
print(dst.split("/")[-1], "lines cut:", cut)
