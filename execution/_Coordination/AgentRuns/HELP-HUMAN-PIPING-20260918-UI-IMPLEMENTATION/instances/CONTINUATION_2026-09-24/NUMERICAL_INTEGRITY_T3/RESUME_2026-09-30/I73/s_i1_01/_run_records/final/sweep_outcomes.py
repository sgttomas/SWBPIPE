"""Parse a pytest junit XML into sorted 'outcome<TAB>nodeid' lines."""
import sys, xml.etree.ElementTree as ET
root = ET.parse(sys.argv[1]).getroot()
lines = []
for case in root.iter("testcase"):
    node = f"{case.get('classname')}::{case.get('name')}"
    outcome = "passed"
    for child in case:
        if child.tag in ("failure", "error", "skipped"):
            outcome = {"failure": "failed", "error": "error", "skipped": "skipped"}[child.tag]
            if child.tag == "error" and outcome == "error":
                pass
    lines.append(f"{outcome}\t{node}")
for line in sorted(lines):
    print(line)

# Recorded copies (sweep_base.outcomes, sweep_cand.outcomes) shorten node ids
# longer than 200 characters to their first 160 characters plus "...#" and the
# first 16 hex digits of the full id's sha256; the comparison was made on the
# full ids.
