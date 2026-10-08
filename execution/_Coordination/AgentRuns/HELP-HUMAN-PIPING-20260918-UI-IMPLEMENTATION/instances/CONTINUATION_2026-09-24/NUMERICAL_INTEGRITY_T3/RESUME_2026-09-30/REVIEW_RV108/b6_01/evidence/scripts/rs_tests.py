import re, sys
def tests(path):
    out, binary = {}, None
    for line in open(path, errors="replace"):
        m = re.match(r"\s+Running (\S+)", line)
        if m: binary = m.group(1); continue
        if line.startswith("   Doc-tests"): binary = "doc:" + line.split()[-1]; continue
        m = re.match(r"test (\S+) \.\.\. (\w+)", line)
        if m: out[f"{binary}::{m.group(1)}"] = m.group(2)
    return out
b, c = tests(sys.argv[1]), tests(sys.argv[2])
print("base", len(b), {k: list(b.values()).count(k) for k in set(b.values())}, "| head", len(c), {k: list(c.values()).count(k) for k in set(c.values())})
print("removed:", sorted(set(b) - set(c)))
print("added:", sorted(set(c) - set(b)))
print("changed outcome:", sorted(k for k in set(b) & set(c) if b[k] != c[k]))
