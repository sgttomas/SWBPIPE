"""Compare a directory with a git tree listing (blob<TAB>path), by git blob hash (stdlib only)."""
import hashlib, os, sys
listing, root = sys.argv[1:3]
want = {}
for l in open(listing):
    h, p = l.rstrip("\n").split("\t", 1)
    want[p] = h
def blob(path):
    if os.path.islink(path):
        data = os.readlink(path).encode()
    else:
        data = open(path, "rb").read()
    return hashlib.sha1(b"blob %d\0" % len(data) + data).hexdigest()
bad = [p for p, h in want.items() if not os.path.lexists(os.path.join(root, p)) or blob(os.path.join(root, p)) != h]
have = set()
for d, ds, fs in os.walk(os.path.join(root, "projects")):
    for f in fs:
        have.add(os.path.relpath(os.path.join(d, f), root))
extra = sorted(have - set(want))
print(f"listed {len(want)}, mismatched {len(bad)}, extra {len(extra)}")
for p in bad[:10]: print("  MISMATCH", p)
for p in extra[:10]: print("  EXTRA", p)
