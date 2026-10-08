import sys, xml.etree.ElementTree as ET
def tests(p):
    out = {}
    for tc in ET.parse(p).getroot().iter("testcase"):
        k = f'{tc.get("classname")}::{tc.get("name")}'
        out[k] = "failed" if (tc.find("failure") is not None or tc.find("error") is not None) else "skipped" if tc.find("skipped") is not None else "passed"
    return out
b, c = tests(sys.argv[1]), tests(sys.argv[2])
from collections import Counter
print("base", len(b), dict(Counter(b.values())), "| head", len(c), dict(Counter(c.values())))
rem = sorted(set(b) - set(c)); add = sorted(set(c) - set(b))
print("removed (%d):" % len(rem)); [print("  -", x) for x in rem]
print("added (%d):" % len(add)); [print("  +", x) for x in add]
print("changed outcome:", sorted(k for k in set(b) & set(c) if b[k] != c[k]))
