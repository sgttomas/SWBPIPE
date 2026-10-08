"""I98 B2-W controls: this probe's lines against I81's B1-0 probe
(R/I81/b1_probe_01/_run_records/probe_run2.log) and I86's B1-SW probe
(R/I86/b1_w_probe_01/_run_records/round2_run1.log), on the same producer bytes (PP, the
solver and RE `src` unchanged from I81's d8c88774d0 and I86's 47a3bdfcf5 to main 0b6c5d7362).

Usage: python compare_controls.py <I81 probe_run2.log> <I86 round2_run1.log> <I98 log>...
Compared per input and mode: the input sha, plain sha and length, published verdicts, the
Direct W1 cause, notices, both byte checks, the successor flag, published sha and length,
and for successors the receipt sha256, the Rust reader verdict and the case status; for I86
also the witness twin's outcome. The seed and native lines are compared for case C.
"""
import re
import sys

PAIRS = [  # (source, their prefix, their label, our label)
    ("I81", "I81_", "case_c", "cb2_case_c"),
    ("I81", "I81_", "u8 l0_isolated_node", "cb3_a"),
    ("I81", "I81_", "milestone (W1, W4, W7, headroom; attempted 'milestone')", "milestone"),
    ("I86", "I86_", "i3_c1_case_a", "i86_case_a"),
]
KEYS = ["input_sha", "plain_sha", "plain_len", "published_verdicts", "cause", "notices", "bytes_eq_with_notice_plain_case_none",
        "bytes_eq_plain", "published_successor", "published_sha", "published_len", "receipt_sha256", "rust_reader", "case_status", "ran"]
MODES = ["sparse_interactive", "dense_scrutiny"]


def value(line, key):
    m = re.search(r"(?:^| )" + re.escape(key) + r"=(\[[^\]]*\]|\S+)", line)
    return m.group(1) if m else None


def lines(path, prefix):
    for line in open(path, encoding="utf-8", errors="replace"):
        i = line.find(prefix)
        if i < 0 or (i > 0 and not line[:i].startswith("test ")):
            continue
        yield line[i:].rstrip("\n")


def collect(path, prefix, label):
    out = {}
    seeds_native = []
    current = None
    for line in lines(path, prefix):
        kind, rest = line.split(" ", 1)
        if kind.endswith("_BEGIN"):
            # Seed and native lines belong to the Direct run between BEGIN and its W1 line.
            current = (label, rest[len(label) + 1:].split()[0]) if rest.startswith(label + " ") else None
        if rest.startswith(label + " "):
            mode = rest[len(label) + 1:].split()[0]
            if kind == prefix + "W1":
                current = None
            d = out.setdefault(mode, {})
            for k in KEYS:
                v = value(rest, k)
                if v is not None and k not in d:
                    d[k] = v
        elif current and current[0] == label and kind[len(prefix):] in ("SEEDS", "NATIVE_OUTCOME", "W1_START"):
            seeds_native.append((current[1], kind[len(prefix):], rest))
    return out, seeds_native


def main():
    i81, i86, ours = sys.argv[1], sys.argv[2], sys.argv[3:]
    bad = 0
    compared = 0
    for source, prefix, theirs, mine in PAIRS:
        a, a_extra = collect(i81 if source == "I81" else i86, prefix, theirs)
        b, b_extra = {}, []
        for path in ours:
            x, y = collect(path, "I98_", mine)
            for mode, d in x.items():
                b.setdefault(mode, {}).update({k: v for k, v in d.items() if k not in b.get(mode, {})})
            if not b_extra:
                b_extra = y
        for mode in MODES:
            for k in KEYS:
                x, y = a.get(mode, {}).get(k), b.get(mode, {}).get(k)
                if x is None:
                    continue
                compared += 1
                ok = x == y
                bad += not ok
                print(f"{source} {mine:12} {mode:18} {k:38} {'equal' if ok else 'DIFFERENT'} {x if ok else (x, y)}")
        if mine == "cb2_case_c":
            # The seed lines and native outcomes, in order, both modes (I81's own run order).
            xa = [(m, k, r) for (m, k, r) in a_extra]
            xb = [(m, k, r) for (m, k, r) in b_extra]
            same = xa == xb
            bad += not same
            compared += 1
            print(f"{source} {mine:12} {'both':18} {'seed/W1_START/native lines (' + str(len(xa)) + ')':38} {'equal' if same else 'DIFFERENT'}")
    print(f"CONTROLS fields_compared={compared} fields_different={bad}")


if __name__ == "__main__":
    main()
