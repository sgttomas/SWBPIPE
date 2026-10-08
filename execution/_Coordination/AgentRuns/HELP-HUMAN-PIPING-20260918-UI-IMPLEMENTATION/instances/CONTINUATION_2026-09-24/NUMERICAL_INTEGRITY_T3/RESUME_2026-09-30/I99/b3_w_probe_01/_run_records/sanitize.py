# I99 (after I81's and I86's sanitize.py): copy a log into the records with machine paths
# replaced by placeholders (ARCH for the archive, WT, ~) and the machine's host name by
# <host>; every line longer than 4,000 bytes that is not one of this probe's own I99_ lines
# is cut to its first 1,000 bytes plus its full length and sha256 (the long lines are the
# producer's committed cfg(test) prints). A libtest "test <name> ... " prefix does not hide
# an I99_ line. usage: sanitize.py <src> <dst> <WT> <host name>
import hashlib, os, sys
src, dst, wt, host = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
cut = 0
with open(src, encoding="utf-8", errors="replace") as f, open(dst, "w") as g:
    for line in f:
        line = line.rstrip("\n")
        line = line.replace(wt + "/scratch/i99_b3w/arch", "ARCH").replace(wt, "WT").replace(os.path.expanduser("~"), "~")
        line = line.replace(host, "<host>")
        head = line[:200]
        own = head.startswith("I99_") or (head.startswith("test ") and " I99_" in head)
        if len(line.encode()) > 4000 and not own:
            b = line.encode()
            line = b[:1000].decode("utf-8", "ignore") + "...[cut by sanitize.py: %d bytes, sha256 %s]" % (len(b), hashlib.sha256(b).hexdigest())
            cut += 1
        g.write(line + "\n")
print(dst.split("/")[-1], "lines cut:", cut)
