"""I91 repair 02: for each mutant's junit file, each failing test and the kind of its failure.

Usage: mutant_kinds.py <junit dir> <jsonl>... > out.txt
A kill counts as an assertion when every failing test fails by a `failure` element (no `error`) whose
report ends at a line of the test file with AssertionError, or with pytest's own `pytest.raises` failure
("DID NOT RAISE"). The kind printed is that last line (`file:line: exception type`).
"""
import json, sys, xml.etree.ElementTree as ET
from pathlib import Path

D = Path(sys.argv[1])
rows = [json.loads(l) for p in sys.argv[2:] for l in open(p) if l.strip()]
bad = 0
for r in rows:
    kinds = []
    for tc in ET.parse(D / f"{r['id']}.xml").getroot().iter("testcase"):
        for tag in ("failure", "error"):
            el = tc.find(tag)
            if el is not None:
                body = (el.text or "").strip().splitlines()
                last = body[-1] if body else ""
                msg = last + " | " + (el.get("message") or "").splitlines()[0][:120]
                ok = tag == "failure" and last.startswith("tests/") and (last.endswith(": AssertionError") or "DID NOT RAISE" in (el.get("message") or ""))
                kinds.append((tc.get("name"), tag, ok, msg))
    allok = all(k[2] for k in kinds)
    if r["id"] == "PRE":  # Not a mutant: the head tests on R01's reader (which tests are new or moved).
        print(f"PRE (baseline, not a mutant): {len(r['failed'])} of {r['tests']} tests fail on R01's reader")
    else:
        if r["outcome"] == "killed" and not allok:
            bad += 1
        print(f"{r['id']}: {r['outcome']}, {len(r['failed'])} of {r['tests']} tests fail" + ("" if r["outcome"] != "killed" else f"; by assertion: {'yes' if allok else 'NO'}"))
    for name, tag, ok, msg in kinds:
        print(f"  {name}: {tag}: {msg}")
print(f"mutant kills not by an assertion: {bad}")
