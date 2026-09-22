#!/usr/bin/env python3
"""Check that the R4 decision packets claim every OWNER_DECISION class row exactly once.

Each packet states its portion as a filter over CLASS_ASSIGNMENTS.csv or as a
key list. PORTIONS below transcribes those statements (packet section 5). Where
a packet gives a key list (T6-C04 in B10), the keys are read from the packet
text. Writes R4/PACKET_CLAIMS.csv; --check compares instead of writing.
"""
import csv, io, re, sys
from pathlib import Path

RUN = Path(__file__).resolve().parent.parent
PKT = RUN / "R4" / "DECISION_PACKETS"
OUT = RUN / "R4" / "PACKET_CLAIMS.csv"


def packet_text(pid):
    return next(PKT.glob(f"P*/{pid}_*.md")).read_text()


def mentioned(pid, key):
    return re.search(re.escape(key) + r"(?![\w.#/-])", packet_text(pid)) is not None


def whole(r):
    return True


# (ClassID, packet, predicate on the row); first match wins, order matters.
PORTIONS = [
    ("T4A-C06", "A1", lambda r: mentioned("A1", r["ClaimKey"])),
    ("T4A-C06", "A7", lambda r: mentioned("A7", r["ClaimKey"])),
    ("T4A-C06", "A10", lambda r: mentioned("A10", r["ClaimKey"])),
    ("T4A-C08", "A6", whole),
    ("T4B-C01", "A6", lambda r: r["ClaimKey"] == "DEL-01-01:SOW"),
    ("T4B-C01", "A4", whole),
    ("T4B-C02", "A6", lambda r: r["DeliverableID"] == "DEL-01-01"),
    ("T4B-C02", "A2", lambda r: r["DeliverableID"] == "DEL-17-03"),
    ("T4B-C03", "A5", whole),
    ("T4B-C04", "A7", whole),
    ("T5A-C05", "B6", lambda r: r["DeliverableID"] == "DEL-17-04"),
    ("T5A-C05", "C1", whole),
    ("T5B-C04", "A5", whole),
    ("T5B-C07", "A6", lambda r: r["DeliverableID"] == "DEL-01-01"),
    ("T5B-C07", "A10", whole),
    ("T5B-C09", "B4", lambda r: r["ClaimKey"] == "DEL-07-02:SOW#CLM-034"),
    ("T5B-C09", "C4", lambda r: r["DeliverableID"] == "DEL-11-01"),
    ("T6-C04", "B10", lambda r: mentioned("B10", r["ClaimKey"])),
    ("T6-C04", "B12", whole),
    ("T7-C01", "A3", whole),
    ("T7-C02", "A2", whole),
    ("T7-C03", "A1", whole),
    ("T7-C04", "A8", whole),
    ("T7-C05", "C1", whole),
]

# Portion counts as each packet states them.
STATED = {("T4A-C06", "A1"): 7, ("T4A-C06", "A7"): 5, ("T4A-C06", "A10"): 2,
          ("T4A-C08", "A6"): 9, ("T4B-C01", "A6"): 1, ("T4B-C01", "A4"): 85,
          ("T4B-C02", "A6"): 4, ("T4B-C02", "A2"): 1, ("T4B-C03", "A5"): 42,
          ("T4B-C04", "A7"): 17, ("T5A-C05", "B6"): 3, ("T5A-C05", "C1"): 14,
          ("T5B-C04", "A5"): 40, ("T5B-C07", "A6"): 16, ("T5B-C07", "A10"): 6,
          ("T5B-C09", "B4"): 1, ("T5B-C09", "C4"): 4, ("T6-C04", "B10"): 20,
          ("T6-C04", "B12"): 67, ("T7-C01", "A3"): 50, ("T7-C02", "A2"): 11,
          ("T7-C03", "A1"): 7, ("T7-C04", "A8"): 5, ("T7-C05", "C1"): 27}


def build():
    rows = [r for r in csv.DictReader(open(RUN / "R3" / "CLASS_ASSIGNMENTS.csv"))
            if r["Route"] == "OWNER_DECISION"]
    out, errors, got = [], [], {}
    for r in rows:
        hit = next((p for c, p, f in PORTIONS if c == r["ClassID"] and f(r)), None)
        if hit is None:
            errors.append(f"unclaimed {r['ClaimKey']} {r['ClassID']}")
            continue
        got[(r["ClassID"], hit)] = got.get((r["ClassID"], hit), 0) + 1
        out.append((r["ClaimKey"], r["DeliverableID"], r["ClassID"], hit))
    for k, n in STATED.items():
        if got.get(k, 0) != n:
            errors.append(f"portion {k} stated {n} got {got.get(k, 0)}")
    buf = io.StringIO()
    w = csv.writer(buf, lineterminator="\n")
    w.writerow(["ClaimKey", "DeliverableID", "ClassID", "Packet"])
    w.writerows(sorted(out))
    return buf.getvalue(), len(rows), errors


def main():
    text, n, errors = build()
    for e in errors:
        print("ERROR", e)
    if "--check" in sys.argv:
        same = OUT.exists() and OUT.read_text() == text
        print("PACKET_CLAIMS", "OK" if same else "DIFFERS")
        sys.exit(0 if same and not errors else 1)
    OUT.write_text(text)
    print(f"{n} owner-route rows; {len(errors)} errors")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
