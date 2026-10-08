# I81: compare this probe's Direct-entry results with I68's U8-0 probe log on the shared inputs
# (same input sha256): plain sha, published sha, W1 cause, notices. Read-only over the two logs.
import re, sys
def rows(path, prefix):
    out, begin = {}, {}
    for line in open(path):
        m = re.search(prefix + r'_BEGIN (.*) (sparse_interactive|dense_scrutiny) registered=\w+ input_sha=(\w+)', line)
        if m: begin[(m.group(1), m.group(2))] = m.group(3); continue
        m = re.search(prefix + r'_ORDINARY (.*) (sparse_interactive|dense_scrutiny) status=.* plain_sha=(\w+)', line)
        if m: out.setdefault((begin[(m.group(1), m.group(2))], m.group(2)), {})["plain"] = m.group(3); continue
        m = re.search(prefix + r'_W1 (.*) (sparse_interactive|dense_scrutiny) cause=(.*?) counts=.* notices=(\d+) .* published_sha=(\w+)', line)
        if m: out.setdefault((begin[(m.group(1), m.group(2))], m.group(2)), {}).update(label=m.group(1), cause=m.group(3), notices=m.group(4), published=m.group(5))
    return out
a, b = rows(sys.argv[1], "I68"), rows(sys.argv[2], "I81")
for key in sorted(set(a) & set(b), key=lambda k: (a[k]["label"], k[1])):
    x, y = a[key], b[key]
    same = all(x[f] == y[f] for f in ("plain", "published", "cause", "notices"))
    print("%-26s %-18s input %s.. I68 %s | I81 %s | %s" % (x["label"], key[1], key[0][:12], x["cause"], y["cause"], "IDENTICAL plain+published+cause+notices" if same else "DIFFERS"))
print("shared inputs:", len(set(a) & set(b)))
