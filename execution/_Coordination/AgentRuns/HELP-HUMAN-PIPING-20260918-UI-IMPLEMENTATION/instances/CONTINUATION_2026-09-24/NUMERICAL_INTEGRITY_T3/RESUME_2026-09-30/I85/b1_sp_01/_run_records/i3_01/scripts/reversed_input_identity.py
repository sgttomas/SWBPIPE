#!/usr/bin/env python3
"""I85 B1-SP I3: the test's `milestone_reversed()` is I98's `cause_milestone_reversed`.
`milestone_reversed()` parses the milestone request fixture and reverses load case 0's
`primitive_loads`; this does the same in Python and compares, as JSON values, with I98's input
(R/I98/b2_w_probe_01/_run_records/inputs/cause_milestone_reversed.json).
Usage: reversed_input_identity.py <milestone request fixture> <I98 input>"""
import hashlib, json, sys
fixture, i98 = sys.argv[1:3]
raw = json.load(open(fixture))
raw["model"]["load_cases"][0]["primitive_loads"].reverse()
theirs = json.load(open(i98))
print("fixture sha256", hashlib.sha256(open(fixture, "rb").read()).hexdigest())
print("I98 input sha256", hashlib.sha256(open(i98, "rb").read()).hexdigest())
print("authored order (reversed):", [(l["id"], l["direction"]) for l in raw["model"]["load_cases"][0]["primitive_loads"]])
print("equal as JSON values:", raw == theirs)
sys.exit(0 if raw == theirs else 1)
