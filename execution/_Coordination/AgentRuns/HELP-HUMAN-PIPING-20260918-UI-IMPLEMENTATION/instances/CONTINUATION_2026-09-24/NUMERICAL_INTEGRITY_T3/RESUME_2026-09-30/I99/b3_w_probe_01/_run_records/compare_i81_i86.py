"""I99 B3-W control: reproduce I81's and I86's recorded lines for the committed 0.1.0 milestone.
usage: compare_i81_i86.py <I81 probe_run2.log> <I86 round1_controls.log> <I99 run log>
Fields compared per mode: input sha256; ordinary results, plain sha256 and length, published
verdicts; Direct W1 cause, counts, notices, both byte checks, published sha256 and length;
successor receipt sha256, Rust reader verdict and case status; and (I86) the witness twin."""
import json, re, sys

def lines(path, prefix):
    out = []
    for line in open(path, encoding="utf-8", errors="replace"):
        line = line.rstrip("\n")
        if line.startswith("test ") and f" {prefix}_" in line:
            line = line[line.index(f" {prefix}_") + 1:]
        if line.startswith(prefix + "_"):
            out.append(line)
    return out

def kv(line, key):
    m = re.search(r"(?:^| )" + re.escape(key) + r"=(\S+)", line)
    return m.group(1) if m else None

def verdicts_from(line):
    m = re.search(r"published_verdicts=(\[.*?\])(?: |$)", line)
    return json.loads(m.group(1)) if m else None

def extract(path, prefix, label_pred):
    rec = {}
    for line in lines(path, prefix):
        tag, rest = line.split(" ", 1)
        kind = tag[len(prefix) + 1:]
        # label is everything up to the mode token
        m = re.match(r"(.*?) (sparse_interactive|dense_scrutiny)(?: |$)", rest)
        if not m or not label_pred(m.group(1)):
            continue
        mode = m.group(2)
        r = rec.setdefault(mode, {})
        if kind == "BEGIN" and "input_sha" not in r:
            r["input_sha"] = kv(line, "input_sha")
        elif kind == "ORDINARY" and "plain_sha" not in r:
            r["results"] = kv(line, "results"); r["plain_sha"] = kv(line, "plain_sha"); r["plain_len"] = kv(line, "plain_len")
            v = verdicts_from(line)
            if v is None:
                f = json.loads(line[line.index("facts=") + 6:])
                v = [{"ref": x["ref"], "solve_quality": x["solve_quality"]} for x in f["verdicts"]]
            r["verdicts"] = v
        elif kind == "W1" and "cause" not in r:
            for k in ["cause", "one_run_through_g_c", "notices", "bytes_eq_with_notice_plain_case_none", "bytes_eq_plain", "published_sha", "published_len", "published_successor"]:
                r[k] = kv(line, k)
        elif kind == "SUCCESSOR" and "receipt" not in r:
            r["receipt"] = kv(line, "receipt_sha256"); r["rust_reader"] = kv(line, "rust_reader"); r["case_status"] = kv(line, "case_status")
        elif kind == "WITNESS" and "witness" not in r:
            r["witness"] = kv(line, "ran")
    return rec

i81 = extract(sys.argv[1], "I81", lambda l: l.startswith("milestone (W1"))
i86 = extract(sys.argv[2], "I86", lambda l: l == "milestone")
i99 = extract(sys.argv[3], "I99", lambda l: l == "milestone")
bad = 0
for name, ref in [("I81", i81), ("I86", i86)]:
    for mode in ["sparse_interactive", "dense_scrutiny"]:
        a, b = ref.get(mode, {}), i99.get(mode, {})
        keys = sorted(a)
        diffs = [k for k in keys if a[k] != b.get(k)]
        bad += len(diffs)
        print(f"{name} milestone {mode}: fields compared={len(keys)} ({', '.join(keys)}) differences={len(diffs)}" + ("" if not diffs else " " + json.dumps({k: [a[k], b.get(k)] for k in diffs})))
print("TOTAL differences:", bad)
sys.exit(1 if bad else 0)
