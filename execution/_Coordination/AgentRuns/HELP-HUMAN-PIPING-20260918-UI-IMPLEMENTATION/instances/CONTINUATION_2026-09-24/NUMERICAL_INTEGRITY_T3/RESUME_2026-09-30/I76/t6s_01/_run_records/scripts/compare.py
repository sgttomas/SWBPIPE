"""I76: test-by-test comparison of two runs. Usage:
compare.py junit <base.xml> <cand.xml>   (pytest junit XML)
compare.py cargo <base.log> <cand.log>   (cargo test output; tests keyed by binary and name)"""
import re, sys, xml.etree.ElementTree as ET
from collections import Counter


def junit(path):
    out = {}
    for case in ET.parse(path).getroot().iter("testcase"):
        key = f"{case.get('classname')}::{case.get('name')}"
        tags = {child.tag for child in case}
        outcome = "failed" if "failure" in tags else "error" if "error" in tags else "skipped" if "skipped" in tags else "passed"
        assert key not in out or out[key] == outcome, key
        out[key] = outcome
    return out


def cargo(path):
    out, binary = {}, None
    for line in open(path, encoding="utf-8", errors="replace"):
        m = re.match(r"\s+Running (?:unittests )?(\S+)", line)
        if m:
            binary = re.sub(r"-[0-9a-f]{16}\)?$", "", m.group(1).split("/")[-1]).rstrip(")")
            if m.group(1).startswith("src/") or m.group(1).startswith("tests/"):
                binary = m.group(1)
            continue
        m = re.match(r"test (\S+) \.\.\. (ok|FAILED|ignored)", line)
        if m:
            out[f"{binary}::{m.group(1)}"] = {"ok": "passed", "FAILED": "failed", "ignored": "skipped"}[m.group(2)]
    return out


def main():
    kind, base, cand = sys.argv[1:4]
    read = junit if kind == "junit" else cargo
    b, c = read(base), read(cand)
    print("base:", dict(Counter(b.values())), "total", len(b))
    print("candidate:", dict(Counter(c.values())), "total", len(c))
    added = sorted(set(c) - set(b))
    removed = sorted(set(b) - set(c))
    changed = sorted((k, b[k], c[k]) for k in set(b) & set(c) if b[k] != c[k])
    print(f"added ({len(added)}):", *[f"  {k} [{c[k]}]" for k in added], sep="\n")
    print(f"removed ({len(removed)}):", *[f"  {k}" for k in removed], sep="\n")
    print(f"outcome changed ({len(changed)}):", *[f"  {k}: {x} -> {y}" for k, x, y in changed], sep="\n")
    fail_b = sorted(k for k, v in b.items() if v in ("failed", "error"))
    fail_c = sorted(k for k, v in c.items() if v in ("failed", "error"))
    print(f"base failures/errors ({len(fail_b)}); candidate failures/errors ({len(fail_c)}); identical sets: {fail_b == fail_c}")


if __name__ == "__main__":
    main()
