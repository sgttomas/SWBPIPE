#!/usr/bin/env python3
"""RV111: validate every runner line against rule_check_run_result.schema.json (VENV jsonschema).
Usage: rv111_schema.py <schema> <run.tsv> <tree label> <report.json>
Classifies each failure by its JSON path (array indices replaced by '*') and validator."""
import json
import sys
from collections import Counter

import jsonschema

schema = json.load(open(sys.argv[1], encoding="utf-8"))
validator = jsonschema.Draft202012Validator(schema)
lines = failing = 0
classes = Counter()
samples = {}
with open(sys.argv[2], encoding="utf-8") as fh:
    for line in fh:
        label, js = line.rstrip("\n").split("\t")[:2]
        if js == "PANIC":
            classes["PANIC"] += 1
            continue
        lines += 1
        errors = list(validator.iter_errors(json.loads(js)))
        if errors:
            failing += 1
        for e in errors:
            path = "/".join("*" if isinstance(p, int) else str(p) for p in e.absolute_path)
            key = f"{path} [{e.validator}: {json.dumps(e.instance)[:40]}]"
            classes[key] += 1
            samples.setdefault(key, label)
report = {"tree": sys.argv[3], "lines": lines, "failing_lines": failing,
          "failure_classes": dict(classes), "samples": samples}
json.dump(report, open(sys.argv[4], "w"), indent=1, sort_keys=True)
print(json.dumps(report, indent=1))
