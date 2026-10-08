"""I86 B1-SW control: the milestone, W2 and W2b through this probe against I81's B1-0 probe
(R/I81/b1_probe_01/_run_records/probe_run2.log), same main producer bytes.

Usage: python3 compare_i81.py <i81 probe_run2.log> <i86 controls log>
Compared per input and mode: input sha, plain sha, plain length, published verdicts, Direct
W1 cause, notices, byte checks, published sha and length.
"""
import re
import sys

I81_LABELS = {
    "milestone (W1, W4, W7, headroom; attempted 'milestone')": "milestone",
    "W2 cap_maximal": "committed_w2",
    "W2b cap_maximal_solvable": "committed_w2b",
}
KEYS = ["input_sha", "plain_sha", "plain_len", "published_verdicts", "cause", "notices", "bytes_eq_with_notice_plain_case_none",
        "bytes_eq_plain", "published_successor", "published_sha", "published_len"]


def value(line, key):
    m = re.search(r"(?:^| )" + re.escape(key) + r"=(\[[^\]]*\]|\S+)", line)
    return m.group(1) if m else None


def collect(path, prefix, labels):
    out = {}
    for line in open(path, encoding="utf-8", errors="replace"):
        # A test's first print shares its line with libtest's "test <name> ... " prefix.
        i = line.find(prefix)
        if i < 0 or (i > 0 and not line[:i].startswith("test ")):
            continue
        line = line[i:]
        kind, rest = line.split(" ", 1)
        for long, short in labels.items():
            if rest.startswith(long + " "):
                mode = rest[len(long) + 1:].split()[0]
                d = out.setdefault((short, mode), {})
                for k in KEYS:
                    v = value(rest, k)
                    if v is not None and k not in d:
                        d[k] = v
    return out


a = collect(sys.argv[1], "I81_", I81_LABELS)
b = collect(sys.argv[2], "I86_", {v: v for v in I81_LABELS.values()})
bad = 0
for key in sorted(a):
    for k in KEYS:
        x, y = a[key].get(k), b.get(key, {}).get(k)
        ok = x == y
        bad += not ok
        print(f"{key[0]:14} {key[1]:18} {k:38} {'equal' if ok else 'DIFFERENT'} {x if ok else (x, y)}")
print(f"pairs={len(a)} fields_different={bad}")
